//! mpmath's extended precision for translated procedures: inside `with mp.workdps(n)` (precision above the 53 bits
//! of a double) mp.mpf values are multiple-precision binary floats (astro-float, MIT licence) and every operation on
//! them rounds to the working precision, as mpmath's do; float() of one rounds to the nearest double. At the default
//! precision (53 bits) mpmath's arithmetic is a double's, and values stay Py::Float exactly as before.
//!
//! The working precision is a thread-local value set by a guard (`workdps`), restored when the block ends.

#[allow(unused_imports)]
use crate::lm::{Lm, LmF64};
use crate::py::{Py, PyErr, R};
use astro_float::{BigFloat, Consts, RoundingMode};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

const RM: RoundingMode = RoundingMode::ToEven;

thread_local! {
    static PREC: Cell<usize> = const { Cell::new(53) };
    static CC: RefCell<Option<Consts>> = const { RefCell::new(None) };
}

/// The working precision in bits (mpmath's mp.prec)
pub fn prec() -> usize {
    PREC.with(|p| p.get())
}

/// Precision above a double's: mpf values are multiple-precision
pub fn extended() -> bool {
    prec() > 53
}

/// The precision a block set, restored when it ends (with mp.workdps(n): ...)
pub struct Guard(usize);

impl Drop for Guard {
    fn drop(&mut self) {
        PREC.with(|p| p.set(self.0));
    }
}

fn set(bits: usize) -> Guard {
    let old = prec();
    PREC.with(|p| p.set(bits.max(1)));
    Guard(old)
}

/// mpmath's dps_to_prec: max(1, round((dps + 1) * log2(10)))
pub fn dps_to_prec(dps: f64) -> usize {
    (((dps.trunc() + 1.0) * 3.321_928_094_887_362_6).round() as usize).max(1)
}

/// with mp.workdps / workprec / extradps / extraprec (n)
pub fn scope(kind: &str, n: &Py) -> Result<Guard, PyErr> {
    let v = n.f()?;
    Ok(match kind {
        "workdps" => set(dps_to_prec(v)),
        "workprec" => set(v as usize),
        "extradps" => set(prec() + dps_to_prec(v) - dps_to_prec(0.0)),
        _ => set(prec() + v as usize),
    })
}

fn with_cc<T>(f: impl FnOnce(&mut Consts) -> T) -> T {
    CC.with(|c| {
        let mut c = c.borrow_mut();
        if c.is_none() {
            *c = Some(Consts::new().expect("mpmath constants"));
        }
        f(c.as_mut().unwrap())
    })
}

/// A decimal text as a multiple-precision value at precision p
fn parse_dec(t: &str, p: usize) -> BigFloat {
    with_cc(|cc| BigFloat::parse(t, astro_float::Radix::Dec, p, RM, cc))
}

/// A multiple-precision value
#[derive(Clone, Debug)]
pub struct M(pub BigFloat);

/// The nearest double (round to nearest; an exponent beyond a double's gives infinity or zero)
pub fn to_f64(b: &BigFloat) -> f64 {
    if b.is_nan() {
        return f64::NAN;
    }
    if b.is_inf_pos() {
        return f64::INFINITY;
    }
    if b.is_inf_neg() {
        return f64::NEG_INFINITY;
    }
    let Some((m, _n, sign, e, _)) = b.as_raw_parts() else { return f64::NAN };
    if m.iter().all(|w| *w == 0) {
        return 0.0;
    }
    let wb = std::mem::size_of::<astro_float::Word>() * 8;
    // the top 64 bits of the normalized mantissa (value = 0.mmm... * 2^e), and whether any bit below them is set
    let mut top: u64 = 0;
    let mut taken = 0;
    let mut sticky = false;
    for w in m.iter().rev() {
        let w = *w as u64;
        if taken < 64 {
            top = if wb == 64 { w } else { (top << 32) | w };
            taken += wb;
        } else if w != 0 {
            sticky = true;
        }
    }
    // round the 64 bits to 53 with the sticky bit (ties to even), exactly
    let low = top & 0x7ff;
    let mut hi = top >> 11;
    let half = 0x400;
    if low > half || (low == half && (sticky || hi & 1 == 1)) || (low == half && sticky) {
        hi += 1;
    }
    let f = hi as f64;                                  // exact: hi < 2^54
    let v = libm::ldexp(f, e - 53);
    if matches!(sign, astro_float::Sign::Neg) { -v } else { v }
}

/// b rounded to exactly p significant bits, to nearest with ties to even. mpmath rounds every result to the working
/// precision in bits; astro-float keeps whole 64-bit words, so without this a 70-bit block computes at 128 bits and a
/// tanh-sinh estimate whose successive sums agree to 90 bits in mpmath (error 0) differs in the extra bits here.
/// The work is bounded by the number's own mantissa words (no allocation beyond one copy of them).
pub(crate) fn round_bits(b: BigFloat, p: usize) -> BigFloat {
    let Some((m, _n, sign, e, _)) = b.as_raw_parts() else { return b };
    let wb = std::mem::size_of::<astro_float::Word>() * 8;
    let total = m.len() * wb;
    if p == 0 || p >= total || m.iter().all(|w| *w == 0) {
        return b;
    }
    let mut w: Vec<astro_float::Word> = m.to_vec();       // least significant word first, normalized (top bit set)
    let drop = total - p;                                 // the low bits that go
    let bit = |w: &[astro_float::Word], i: usize| (w[i / wb] >> (i % wb)) & 1 == 1;
    let round = bit(&w, drop - 1);
    let below = drop - 1;                                 // bits under the rounding bit
    let sticky = w[..below / wb].iter().any(|x| *x != 0)
        || (below % wb != 0 && (w[below / wb] & ((1 as astro_float::Word).wrapping_shl((below % wb) as u32).wrapping_sub(1))) != 0);
    let lsb = bit(&w, drop);
    for x in w.iter_mut().take(drop / wb) {
        *x = 0;
    }
    if drop % wb != 0 {
        w[drop / wb] &= !((1 as astro_float::Word).wrapping_shl((drop % wb) as u32).wrapping_sub(1));
    }
    let mut e = e;
    if round && (sticky || lsb) {
        // add one unit at bit `drop`, carrying upward; a carry out of the top makes 0.1000... * 2^(e + 1)
        let mut i = drop / wb;
        let mut add = (1 as astro_float::Word).wrapping_shl((drop % wb) as u32);
        loop {
            let (s, c) = w[i].overflowing_add(add);
            w[i] = s;
            if !c {
                break;
            }
            add = 1;
            i += 1;
            if i == w.len() {
                let top = w.len() - 1;
                w[top] = (1 as astro_float::Word) << (wb - 1);
                if e == astro_float::EXPONENT_MAX {
                    return BigFloat::from_f64(if matches!(sign, astro_float::Sign::Neg) { f64::NEG_INFINITY } else { f64::INFINITY }, p);
                }
                e += 1;
                break;
            }
        }
    }
    BigFloat::from_words(&w, sign, e)
}

/// A Python number as a multiple-precision value at the working precision (exact for an int, a float or a
/// fraction's parts)
pub fn of(p: &Py) -> Option<BigFloat> {
    let pr = prec().max(64);
    Some(match p {
        Py::Mpf(m) => m.0.clone(),
        Py::Float(x) => BigFloat::from_f64(*x, pr.max(64)),
        Py::Int(i) => BigFloat::from_i64(*i, pr.max(64)),
        Py::Bool(b) => BigFloat::from_i64(*b as i64, pr),
        Py::Big(n) => parse_dec(&n.to_string(), pr),
        Py::Frac(q) => {
            let a = parse_dec(&q.numer().to_string(), pr);
            let b = parse_dec(&q.denom().to_string(), pr);
            a.div(&b, pr, RM)
        }
        _ => return None,
    })
}

/// of(p) without copying an mpf's digits
fn of_cow(p: &Py) -> Option<std::borrow::Cow<'_, BigFloat>> {
    match p {
        Py::Mpf(m) => Some(std::borrow::Cow::Borrowed(&m.0)),
        _ => of(p).map(std::borrow::Cow::Owned),
    }
}

/// A result: a multiple-precision value at an extended precision (rounded to exactly that many bits, as mpmath's),
/// else the double (mpmath at 53 bits)
pub fn wrap(b: BigFloat) -> Py {
    if extended() {
        Py::Mpf(Rc::new(M(round_bits(b, prec()))))
    } else {
        Py::Float(to_f64(&b))
    }
}

/// mp.mpf(x)
pub fn mpf(x: &Py) -> R {
    if !extended() {
        return Ok(Py::Float(x.f()?));
    }
    match x {
        Py::Str(s) => {
            let b = parse_dec(s.trim(), prec());
            if b.is_nan() {
                return Err(PyErr::Value(format!("cannot create mpf from {}", crate::py::repr(x))));
            }
            Ok(wrap(b))
        }
        _ => Ok(wrap(of(x).ok_or_else(|| PyErr::Type("cannot create mpf from this value".into()))?)),
    }
}

/// a op b when either is a multiple-precision value: mpmath's arithmetic at the working precision; None otherwise
pub fn bin(a: &Py, b: &Py, op: &str) -> Option<R> {
    if !matches!(a, Py::Mpf(_)) && !matches!(b, Py::Mpf(_)) {
        return None;
    }
    // with a complex operand (mpmath's mpc), the double-precision complex arithmetic (mpc is not modelled here)
    if matches!(a, Py::Complex(..)) || matches!(b, Py::Complex(..)) {
        return None;
    }
    let (x, y) = match (of_cow(a), of_cow(b)) {
        (Some(x), Some(y)) => (x, y),
        _ => return Some(Err(PyErr::Type(format!("unsupported operand type(s) for {op}")))),
    };
    // computed 64 bits above the working precision, then rounded once to it (wrap): correctly rounded but in about
    // one case in 2^64, as mpmath's arithmetic is
    let p = prec().max(53) + 64;
    Some(Ok(wrap(match op {
        "+" => x.add(&y, p, RM),
        "-" => x.sub(&y, p, RM),
        "*" => x.mul(&y, p, RM),
        "/" => {
            if y.is_zero() {
                return Some(Err(PyErr::ZeroDivision));
            }
            x.div(&y, p, RM)
        }
        _ => {
            // ** : an integer power exactly by squaring; otherwise exp(y log x) for x > 0
            if y.is_int() && to_f64(&y).abs() < 1e9 {
                let k = to_f64(&y) as i64;
                if x.is_zero() && k < 0 {
                    return Some(Err(PyErr::ZeroDivision));
                }
                let r = x.powi(k.unsigned_abs() as usize, p, RM);
                if k < 0 { BigFloat::from_i64(1, p).div(&r, p, RM) } else { r }
            } else if x.is_negative() {
                return Some(Err(PyErr::Unsupported("a complex power (mpc) of a negative mpf".into())));
            } else if x.is_zero() {
                if y.is_negative() {
                    return Some(Err(PyErr::ZeroDivision));
                }
                BigFloat::from_i64(if y.is_zero() { 1 } else { 0 }, p)
            } else {
                with_cc(|cc| x.pow(&y, p, RM, cc))
            }
        }
    })))
}

/// -x, abs(x)
pub fn neg(m: &M) -> Py {
    wrap(m.0.neg())
}

pub fn abs(m: &M) -> Py {
    wrap(m.0.abs())
}

/// The order of two numbers when either is multiple-precision (None if one is not a number or is NaN)
pub fn cmp(a: &Py, b: &Py) -> Option<std::cmp::Ordering> {
    let (x, y) = (of(a)?, of(b)?);
    x.partial_cmp(&y)
}

/// An mpmath function of a multiple-precision value (or of any number at an extended precision): sqrt, exp, log,
/// trigonometric and hyperbolic functions, fabs; None when the precision is a double's and no argument is an mpf
pub fn func(name: &str, x: &Py) -> Option<R> {
    if !matches!(x, Py::Mpf(_)) && !extended() {
        return None;
    }
    if !extended() {
        return None;                                     // an mpf at 53 bits: the double functions are mpmath's
    }
    const NAMES: [&str; 19] = ["sqrt", "exp", "log", "ln", "sin", "cos", "tan", "asin", "acos", "atan", "sinh", "cosh",
                               "tanh", "fabs", "sec", "csc", "cot", "expm1", "log1p"];
    if !NAMES.contains(&name) {
        return None;
    }
    let v = of_cow(x)?;
    let p = prec();
    let r = with_cc(|cc| -> Result<BigFloat, PyErr> {
        Ok(match name {
            "sqrt" => {
                if v.is_negative() {
                    return Err(PyErr::Unsupported("mpc".into()));
                }
                v.sqrt(p, RM)
            }
            "exp" => mexp(&v, p, cc),
            "log" | "ln" => {
                if v.is_zero() {
                    return Ok(BigFloat::from_f64(f64::NEG_INFINITY, p));
                }
                if v.is_negative() {
                    return Err(PyErr::Unsupported("mpc".into()));
                }
                v.ln(p, RM, cc)
            }
            "sin" => msin(&v, p, cc),
            "cos" => mcos(&v, p, cc),
            "tan" => v.tan(p, RM, cc),
            "asin" | "acos" => {
                if to_f64(&v).abs() > 1.0 {
                    return Err(PyErr::Unsupported("mpc".into()));
                }
                if name == "asin" { v.asin(p, RM, cc) } else { v.acos(p, RM, cc) }
            }
            "atan" => v.atan(p, RM, cc),
            "sinh" => v.sinh(p, RM, cc),
            "cosh" => v.cosh(p, RM, cc),
            "tanh" => v.tanh(p, RM, cc),
            "fabs" => v.abs(),
            // exp(x) - 1 and log(1 + x) with guard bits for the cancellation near zero (mpmath's are accurate there too)
            "expm1" | "log1p" => {
                if v.is_zero() {
                    return Ok(v.clone().into_owned());
                }
                let a = to_f64(&v).abs();
                let extra = if a > 0.0 && a < 1.0 { (-a.log2_lm()).ceil() as usize + 16 } else { 16 };
                if extra > 4096 {
                    return Err(PyErr::Unsupported(format!("mpmath.{name} of a tiny argument")));
                }
                let wp = p + 64 + extra;
                let one = BigFloat::from_i64(1, wp);
                let mut r = if name == "expm1" {
                    mexp(&v, wp, cc).sub(&one, wp, RM)
                } else {
                    let s = v.add(&one, wp, RM);
                    if s.is_zero() {
                        BigFloat::from_f64(f64::NEG_INFINITY, p)
                    } else if s.is_negative() {
                        return Err(PyErr::Unsupported("mpc".into()));
                    } else {
                        s.ln(wp, RM, cc)
                    }
                };
                r.set_precision(p, RM).map_err(|_| PyErr::Overflow)?;
                r
            }
            "sec" | "csc" | "cot" => {
                let (s, c) = (msin(&v, p, cc), mcos(&v, p, cc));
                let (n, d) = match name {
                    "sec" => (BigFloat::from_i64(1, p), c),
                    "csc" => (BigFloat::from_i64(1, p), s),
                    _ => (c, s),
                };
                if d.is_zero() {
                    return Err(PyErr::ZeroDivision);
                }
                n.div(&d, p, RM)
            }
            _ => return Err(PyErr::Unsupported(format!("mpmath.{name} at extended precision"))),
        })
    });
    Some(r.map(wrap))
}

// ------------------------------------------------------------------------------- exp, sin and cos in fixed point
// astro-float's exp and sin take 42 and 24 us at 103 bits (a multiplication 0.2 us): an M-PSK curve made of 287
// Craig integrals took 23 s to redraw (2026-10-30). As mpmath does, these work on integers scaled by 2^w (w = the
// precision + 48 or 64 guard bits): range reduction by ln 2 or pi/2 (astro-float's own constants), a Taylor series,
// then one rounding to the precision. Correctly rounded except in about one case in 2^30; outside the range where
// that holds (huge arguments, results near zero relative to the scale) astro-float answers as before.

/// |x| * 2^w as an integer (truncated), with x's sign
fn to_fixed(b: &BigFloat, w: usize) -> Option<num_bigint::BigInt> {
    use num_bigint::{BigInt, BigUint, Sign as S};
    if b.is_zero() {
        return Some(BigInt::from(0));
    }
    let (m, _n, sign, e, _) = b.as_raw_parts()?;
    let wb = std::mem::size_of::<astro_float::Word>() * 8;
    let mut digits: Vec<u32> = Vec::with_capacity(m.len() * 2);
    for wd in m {
        let x = *wd as u64;
        digits.push(x as u32);
        if wb == 64 {
            digits.push((x >> 32) as u32);
        }
    }
    let mag = BigUint::new(digits);                    // value = mag * 2^(e - len*wb)
    let shift = e as i64 - (m.len() * wb) as i64 + w as i64;
    let mag = if shift >= 0 { mag << (shift as usize) } else { mag >> ((-shift) as usize) };
    Some(BigInt::from_biguint(if matches!(sign, astro_float::Sign::Neg) { S::Minus } else { S::Plus }, mag))
}

/// x / 2^w * 2^k rounded to p bits
fn from_fixed(x: &num_bigint::BigInt, w: usize, k: i64, p: usize) -> Option<BigFloat> {
    let mag = x.magnitude();
    if mag.bits() == 0 {
        return Some(BigFloat::from_i64(0, p));
    }
    let wb = std::mem::size_of::<astro_float::Word>() * 8;
    let d = mag.to_u32_digits();
    let words: Vec<astro_float::Word> = if wb == 64 {
        d.chunks(2).map(|c| (c[0] as u64 | ((c.get(1).copied().unwrap_or(0) as u64) << 32)) as astro_float::Word).collect()
    } else {
        d.iter().map(|&v| v as astro_float::Word).collect()
    };
    let e = (words.len() * wb) as i64 - w as i64 + k;   // 0.mmm * 2^e
    if e.abs() > (1 << 28) {
        return None;
    }
    let sign = if x.sign() == num_bigint::Sign::Minus { astro_float::Sign::Neg } else { astro_float::Sign::Pos };
    let mut b = BigFloat::from_words(&words, sign, e as astro_float::Exponent);
    b.set_precision(p, RM).ok()?;
    Some(b)
}

// The series below run on fixed-size words when the scale w fits (Fx, up to 250 bits: a working precision up to
// 186 bits for sin and cos), with exactly the integer semantics of the arbitrary-size path they replace (a shift of a
// negative number rounds down, a division truncates toward zero), so every result is bit for bit the same; a
// num-bigint allocation per operation was most of the time of an integrand evaluation (2026-10-31: Fourier
// coefficients at 7 s). Wider scales keep the arbitrary-size integers.

const FXN: usize = 4;
const FX_MAX_W: usize = 250;

/// A signed fixed-point integer of at most 256 bits (sign and magnitude, least significant word first)
#[derive(Clone, Copy)]
struct Fx {
    neg: bool,
    m: [u64; FXN],
}

impl Fx {
    fn zero() -> Fx {
        Fx { neg: false, m: [0; FXN] }
    }
    fn is_zero(&self) -> bool {
        self.m.iter().all(|w| *w == 0)
    }
    fn pow2(w: usize) -> Fx {
        let mut m = [0; FXN];
        m[w / 64] = 1u64 << (w % 64);
        Fx { neg: false, m }
    }
    fn of(x: &num_bigint::BigInt) -> Option<Fx> {
        let d = x.magnitude().to_u64_digits();
        if d.len() > FXN {
            return None;
        }
        let mut m = [0; FXN];
        m[..d.len()].copy_from_slice(&d);
        let f = Fx { neg: x.sign() == num_bigint::Sign::Minus, m };
        Some(if f.is_zero() { Fx::zero() } else { f })
    }
    fn big(&self) -> num_bigint::BigInt {
        use num_bigint::{BigInt, BigUint, Sign as S};
        let mut d: Vec<u32> = Vec::with_capacity(FXN * 2);
        for w in self.m {
            d.push(w as u32);
            d.push((w >> 32) as u32);
        }
        let mag = BigUint::new(d);
        BigInt::from_biguint(if self.neg { S::Minus } else { S::Plus }, mag)
    }
    fn neg(mut self) -> Fx {
        if !self.is_zero() {
            self.neg = !self.neg;
        }
        self
    }
    /// self + o, exactly (the magnitudes stay below 2^255 here)
    fn add(&self, o: &Fx) -> Fx {
        let (a, b) = (&self.m, &o.m);
        if self.neg == o.neg {
            let mut m = [0; FXN];
            let mut carry = 0u64;
            for i in 0..FXN {
                let (s1, c1) = a[i].overflowing_add(b[i]);
                let (s2, c2) = s1.overflowing_add(carry);
                m[i] = s2;
                carry = (c1 as u64) + (c2 as u64);
            }
            return Fx { neg: self.neg, m };
        }
        let ge = (0..FXN).rev().find(|&i| a[i] != b[i]).map(|i| a[i] > b[i]).unwrap_or(true);
        let (big, small, neg) = if ge { (a, b, self.neg) } else { (b, a, o.neg) };
        let mut m = [0; FXN];
        let mut borrow = 0u64;
        for i in 0..FXN {
            let (d1, b1) = big[i].overflowing_sub(small[i]);
            let (d2, b2) = d1.overflowing_sub(borrow);
            m[i] = d2;
            borrow = (b1 as u64) + (b2 as u64);
        }
        let f = Fx { neg, m };
        if f.is_zero() { Fx::zero() } else { f }
    }
    /// floor(self * o / 2^w) (num-bigint's >> of a signed product); None if it does not fit
    fn mul_shr(&self, o: &Fx, w: usize) -> Option<Fx> {
        let mut prod = [0u64; 2 * FXN];
        for i in 0..FXN {
            if self.m[i] == 0 {
                continue;
            }
            let mut carry: u128 = 0;
            for j in 0..FXN {
                let t = (self.m[i] as u128) * (o.m[j] as u128) + (prod[i + j] as u128) + carry;
                prod[i + j] = t as u64;
                carry = t >> 64;
            }
            prod[i + FXN] = carry as u64;
        }
        let (q, r) = (w / 64, w % 64);
        let mut m = [0u64; FXN];
        for (i, slot) in m.iter_mut().enumerate() {
            let lo = prod.get(i + q).copied().unwrap_or(0);
            let hi = prod.get(i + q + 1).copied().unwrap_or(0);
            *slot = if r == 0 { lo } else { (lo >> r) | (hi << (64 - r)) };
        }
        // what lies above the four words must be zero
        if r == 0 {
            if prod.iter().skip(q + FXN).any(|x| *x != 0) {
                return None;
            }
        } else if prod.get(q + FXN).map(|x| x >> r != 0).unwrap_or(false) || prod.iter().skip(q + FXN + 1).any(|x| *x != 0) {
            return None;
        }
        let neg = self.neg != o.neg;
        let mut f = Fx { neg, m };
        if neg {
            let low = prod[..q].iter().any(|x| *x != 0) || (r != 0 && prod[q] & ((1u64 << r) - 1) != 0);
            if low && !(self.is_zero() || o.is_zero()) {
                // rounding down a negative quotient: one more unit of magnitude
                let one = Fx { neg: false, m: { let mut u = [0; FXN]; u[0] = 1; u } };
                let mag = Fx { neg: false, m: f.m }.add(&one);
                f = Fx { neg: true, m: mag.m };
            }
        }
        Some(if f.is_zero() { Fx::zero() } else { f })
    }
    /// self / n truncated toward zero
    fn div_small(&self, n: u64) -> Fx {
        let mut m = [0u64; FXN];
        let mut rem: u128 = 0;
        for i in (0..FXN).rev() {
            let cur = (rem << 64) | self.m[i] as u128;
            m[i] = (cur / n as u128) as u64;
            rem = cur % n as u128;
        }
        let f = Fx { neg: self.neg, m };
        if f.is_zero() { Fx::zero() } else { f }
    }
}

thread_local! {
    /// ln 2 and pi/2 as fixed-point integers per scale (a handful of scales occur; bounded)
    static FIXED_CONSTS: RefCell<Vec<(u8, usize, num_bigint::BigInt)>> = const { RefCell::new(Vec::new()) };
}

/// ln 2 * 2^(w + 32) (kind 0), pi/2 * 2^(w + 32) (kind 1) or pi/2 * 2^(w + 96) (kind 2), cached
fn fixed_const(kind: u8, w: usize, cc: &mut Consts) -> Option<num_bigint::BigInt> {
    if let Some(v) = FIXED_CONSTS.with(|c| c.borrow().iter().find(|(k, ww, _)| *k == kind && *ww == w).map(|e| e.2.clone())) {
        return Some(v);
    }
    let v = match kind {
        0 => to_fixed(&cc.ln_2(w + 32, RM), w + 32)?,
        1 => to_fixed(&cc.pi(w + 32, RM), w + 32)? >> 1usize,
        _ => to_fixed(&cc.pi(w + 96, RM), w + 96)? >> 1usize,
    };
    FIXED_CONSTS.with(|c| {
        let mut c = c.borrow_mut();
        if c.len() >= 64 {
            c.clear();
        }
        c.push((kind, w, v.clone()));
    });
    Some(v)
}

/// exp(v) at p bits in fixed point; None where astro-float should answer
fn fast_exp(v: &BigFloat, p: usize, cc: &mut Consts) -> Option<BigFloat> {
    let xf = to_f64(v);
    if p > 1024 || !(xf.abs() < 1e6) || v.is_zero() {
        return None;
    }
    let w = p + 48;
    let x = to_fixed(v, w)?;
    let k = (xf / std::f64::consts::LN_2).round() as i64;
    let ln2 = fixed_const(0, w, cc)?;
    const S: usize = 10;                                // exp(r) = exp(r / 2^S)^(2^S)
    let r = (x - ((ln2 * k) >> 32usize)) >> S;
    from_fixed(&exp_series(&r, w, S)?, w, k, p)
}

/// exp(r)^(2^squarings) at scale w (r already divided by 2^squarings): the Taylor series of exp(r), then the squarings
fn exp_series(r: &num_bigint::BigInt, w: usize, squarings: usize) -> Option<num_bigint::BigInt> {
    if w <= FX_MAX_W {
        if let Some(rf) = Fx::of(r) {
            let one = Fx::pow2(w);
            let (mut sum, mut term) = (one, one);
            for n in 1u64..400 {
                term = term.mul_shr(&rf, w)?.div_small(n);
                if term.is_zero() {
                    for _ in 0..squarings {
                        sum = sum.mul_shr(&sum, w)?;
                    }
                    return Some(sum.big());
                }
                sum = sum.add(&term);
            }
            return None;
        }
    }
    exp_series_big(r, w, squarings)
}

/// exp_series on arbitrary-size integers
fn exp_series_big(r: &num_bigint::BigInt, w: usize, squarings: usize) -> Option<num_bigint::BigInt> {
    use num_bigint::BigInt;
    let one = BigInt::from(1) << w;
    let (mut sum, mut term) = (one.clone(), one);
    for n in 1u32..400 {
        term = ((&term * r) >> w) / n;
        if term.bits() == 0 {
            for _ in 0..squarings {
                sum = (&sum * &sum) >> w;
            }
            return Some(sum);
        }
        sum += &term;
    }
    None
}

/// The sine series (odd: r - r^3/3! + ...) or the cosine series (1 - r^2/2! + ...) of r at scale w, |r| <= pi/4
fn trig_series(r: &num_bigint::BigInt, w: usize, cosine: bool) -> Option<num_bigint::BigInt> {
    if w <= FX_MAX_W {
        if let Some(rf) = Fx::of(r) {
            let r2 = rf.mul_shr(&rf, w)?;
            let start = if cosine { Fx::pow2(w) } else { rf };
            let (mut s, mut term) = (start, start);
            for k in 1u64..400 {
                let d = if cosine { (2 * k - 1) * (2 * k) } else { (2 * k) * (2 * k + 1) };
                term = term.mul_shr(&r2, w)?.div_small(d).neg();
                if term.is_zero() {
                    return Some(s.big());
                }
                s = s.add(&term);
            }
            return None;
        }
    }
    trig_series_big(r, w, cosine)
}

/// trig_series on arbitrary-size integers
fn trig_series_big(r: &num_bigint::BigInt, w: usize, cosine: bool) -> Option<num_bigint::BigInt> {
    use num_bigint::BigInt;
    let r2 = (r * r) >> w;
    let start = if cosine { BigInt::from(1) << w } else { r.clone() };
    let (mut s, mut term) = (start.clone(), start);
    for k in 1u32..400 {
        let d = if cosine { (2 * k - 1) * (2 * k) } else { (2 * k) * (2 * k + 1) };
        term = -(((&term * &r2) >> w) / d);
        if term.bits() == 0 {
            return Some(s);
        }
        s += &term;
    }
    None
}

/// sin v (cosine false) or cos v at p bits in fixed point, computing only the series it needs; None where astro-float
/// should answer
fn fast_trig(v: &BigFloat, p: usize, cc: &mut Consts, cosine: bool) -> Option<BigFloat> {
    let xf = to_f64(v);
    if p > 1024 || !(xf.abs() < 1e15) || v.is_zero() {
        return None;
    }
    let q = (xf / std::f64::consts::FRAC_PI_2).round() as i64;
    // pi/2 carries 32 bits beyond the scale for |q| < 2^20 (|x| < 1.6e6), 96 above that: its error times q stays
    // 12 bits under the scale's last place
    let big_q = q.unsigned_abs() >= 1 << 20;
    let reduce = |w: usize, cc: &mut Consts| -> Option<num_bigint::BigInt> {
        let x = to_fixed(v, w)?;
        if big_q {
            let pio2 = fixed_const(2, w, cc)?;
            return Some(x - ((pio2 * q) >> 96usize));
        }
        let pio2 = fixed_const(1, w, cc)?;
        Some(x - ((pio2 * q) >> 32usize))               // |r| <= pi/4
    };
    let w0 = p + 64;
    let mut w = w0;
    let mut r = reduce(w, cc)?;
    if r.bits() + 24 < w0 as u64 {
        // |r| < 2^-24 (an argument near a multiple of pi/2, as tanh-sinh nodes next to a Fourier subinterval's end
        // are): the reduction again at a scale with at least as many more bits as r's leading zeros (rounded up to a
        // whole word, so few scales occur), so r and its series keep 64 bits beyond the precision relative to r
        // itself; astro-float answered these before, at 20 us each and not always correctly rounded
        let extra = w0 - r.bits() as usize + 8;
        if extra > 2048 {
            return None;
        }
        w = (w0 + extra).div_ceil(64) * 64;
        r = reduce(w, cc)?;
        if r.bits() + 24 < w0 as u64 {
            return None;                                 // still short of p + 40 significant bits: astro-float answers
        }
    }
    // sin: s, c, -s, -c by quadrant; cos: c, -s, -c, s
    let quad = (q.rem_euclid(4) + if cosine { 1 } else { 0 }) % 4;
    let use_cos = quad % 2 == 1;
    let t = trig_series(&r, w, use_cos)?;
    let t = if quad >= 2 { -t } else { t };
    from_fixed(&t, w, 0, p)
}

/// (sin v, cos v) at p bits in fixed point; None where astro-float should answer
#[cfg(test)]
fn fast_sincos(v: &BigFloat, p: usize, cc: &mut Consts) -> Option<(BigFloat, BigFloat)> {
    Some((fast_trig(v, p, cc, false)?, fast_trig(v, p, cc, true)?))
}

pub(crate) fn mexp(x: &BigFloat, p: usize, cc: &mut Consts) -> BigFloat {
    fast_exp(x, p, cc).unwrap_or_else(|| x.exp(p, RM, cc))
}

pub(crate) fn msin(x: &BigFloat, p: usize, cc: &mut Consts) -> BigFloat {
    fast_trig(x, p, cc, false).unwrap_or_else(|| x.sin(p, RM, cc))
}

pub(crate) fn mcos(x: &BigFloat, p: usize, cc: &mut Consts) -> BigFloat {
    fast_trig(x, p, cc, true).unwrap_or_else(|| x.cos(p, RM, cc))
}

/// Euler's constant to 100 digits (enough for a working precision up to 300 bits)
const EULER_GAMMA: &str = "0.5772156649015328606065120900824024310421593359399235988057672348848677267776646709369470632917467495";

/// The exponential integral E1(x) = integral from x to infinity of exp(-t)/t dt for x > 0, at p bits: the power series
/// -gamma - ln x - sum (-x)^k/(k k!) with guard bits for its cancellation (x < 20), else the continued fraction
/// exp(-x) / (x + 1 - 1/(x + 3 - 4/(x + 5 - ...))) by Lentz's method. E1(0) = +inf; a negative x (a complex value in
/// mpmath) is outside what this models.
pub fn e1(x: &BigFloat, p: usize) -> Result<BigFloat, PyErr> {
    if x.is_nan() {
        return Ok(x.clone());
    }
    if x.is_zero() {
        return Ok(BigFloat::from_f64(f64::INFINITY, p));
    }
    if x.is_negative() {
        return Err(PyErr::Unsupported("mpmath.e1 of a negative argument (complex)".into()));
    }
    if x.is_inf_pos() {
        return Ok(BigFloat::from_i64(0, p));
    }
    if p > 300 {
        return Err(PyErr::Unsupported("mpmath.e1 above 300 bits".into()));
    }
    let xf = to_f64(x);
    let wp0 = p + 64 + (3.0 * xf.min(20.0)) as usize;
    let g = parse_dec(EULER_GAMMA, wp0);                 // before with_cc below: parse_dec borrows the constants too
    with_cc(|cc| {
        if xf < 20.0 {
            let wp = wp0;
            let mx = x.neg();
            let mut t = BigFloat::from_i64(1, wp);           // (-x)^k / k!
            let mut s = BigFloat::from_i64(0, wp);
            let mut k: i64 = 1;
            loop {
                t = t.mul(&mx, wp, RM).div(&BigFloat::from_i64(k, wp), wp, RM);
                let term = t.div(&BigFloat::from_i64(k, wp), wp, RM);
                s = s.add(&term, wp, RM);
                let (tf, sf) = (to_f64(&term).abs(), to_f64(&s).abs());
                if (k as f64) > xf && (term.is_zero() || tf <= sf * 2f64.powi(-(wp as i32) - 4)) {
                    break;
                }
                k += 1;
                if k > 100_000 {
                    return Err(PyErr::Unsupported("mpmath.e1 series did not converge".into()));
                }
            }
            let r = g.neg().sub(&x.ln(wp, RM, cc), wp, RM).sub(&s, wp, RM);
            let mut r = r;
            r.set_precision(p, RM).map_err(|_| PyErr::Overflow)?;
            Ok(r)
        } else {
            let wp = p + 32;
            let one = BigFloat::from_i64(1, wp);
            let two = BigFloat::from_i64(2, wp);
            let mut b = x.add(&one, wp, RM);
            let mut c = BigFloat::from_f64(1e300, wp);
            let mut d = one.div(&b, wp, RM);
            let mut h = d.clone();
            let eps = 2f64.powi(-(wp as i32));
            for i in 1i64..100_000 {
                let an = BigFloat::from_i64(-i * i, wp);
                b = b.add(&two, wp, RM);
                d = one.div(&an.mul(&d, wp, RM).add(&b, wp, RM), wp, RM);
                c = b.add(&an.div(&c, wp, RM), wp, RM);
                let del = c.mul(&d, wp, RM);
                h = h.mul(&del, wp, RM);
                if to_f64(&del.sub(&one, wp, RM)).abs() < eps {
                    let mut r = h.mul(&mexp(&x.neg(), wp, cc), wp, RM);
                    r.set_precision(p, RM).map_err(|_| PyErr::Overflow)?;
                    return Ok(r);
                }
            }
            Err(PyErr::Unsupported("mpmath.e1 continued fraction did not converge".into()))
        }
    })
}

/// mpmath.e1(x) for a real x: a multiple-precision value at an extended precision, else the double
pub fn e1_py(x: &Py) -> R {
    let p = prec().max(53);
    let v = of(x).ok_or_else(|| PyErr::Type("e1 of a non-number".into()))?;
    let r = e1(&v, p.max(64))?;
    if extended() {
        let mut r = r;
        r.set_precision(p, RM).map_err(|_| PyErr::Overflow)?;
        Ok(wrap(r))
    } else {
        Ok(Py::Float(to_f64(&r)))
    }
}

// ------------------------------------------------------------------- polyroots at an extended working precision
// mpmath.polyroots (calculus/polynomials.py) operation for operation on mpmath's complex arithmetic (libmp/libmpc.py):
// products exact, each sum rounded once to nearest at the working precision; mpc_div with its intermediate sums
// rounded down at prec + 10; abs as mpf_hypot (the sum of squares rounded down at prec + 4, then the root to nearest).
// A multiple root therefore stalls or converges exactly as it does in mpmath: NoConvergence (an internal error, not a
// refusal) or roots that the cleanup chops at the same tolerance.

type MC = (BigFloat, BigFloat);

fn exact_mul(a: &BigFloat, b: &BigFloat) -> BigFloat {
    let p = a.precision().unwrap_or(64) + b.precision().unwrap_or(64) + 64;
    a.mul(b, p, RM)
}

fn mc_sub(x: &MC, y: &MC, p: usize) -> MC {
    (x.0.sub(&y.0, p, RM), x.1.sub(&y.1, p, RM))
}

fn mc_add(x: &MC, y: &MC, p: usize) -> MC {
    (x.0.add(&y.0, p, RM), x.1.add(&y.1, p, RM))
}

fn mc_mul(x: &MC, y: &MC, p: usize) -> MC {
    let (ac, bd, ad, bc) = (exact_mul(&x.0, &y.0), exact_mul(&x.1, &y.1), exact_mul(&x.0, &y.1), exact_mul(&x.1, &y.0));
    (ac.sub(&bd, p, RM), ad.add(&bc, p, RM))
}

fn mc_div(x: &MC, y: &MC, p: usize) -> Result<MC, PyErr> {
    let wp = p + 10;
    let down = RoundingMode::ToZero;
    let mag = exact_mul(&y.0, &y.0).add(&exact_mul(&y.1, &y.1), wp, down);
    if mag.is_zero() {
        return Err(PyErr::ZeroDivision);
    }
    let t = exact_mul(&x.0, &y.0).add(&exact_mul(&x.1, &y.1), wp, down);
    let u = exact_mul(&x.1, &y.0).sub(&exact_mul(&x.0, &y.1), wp, down);
    Ok((t.div(&mag, p, RM), u.div(&mag, p, RM)))
}

fn mc_abs(x: &MC, p: usize) -> BigFloat {
    if x.1.is_zero() {
        let mut a = x.0.abs();
        let _ = a.set_precision(p, RM);
        return a;
    }
    if x.0.is_zero() {
        let mut a = x.1.abs();
        let _ = a.set_precision(p, RM);
        return a;
    }
    let h2 = exact_mul(&x.0, &x.0).add(&exact_mul(&x.1, &x.1), p + 4, RoundingMode::ToZero);
    h2.sqrt(p, RM)
}

fn rounded(b: &BigFloat, p: usize) -> BigFloat {
    let mut r = b.clone();
    let _ = r.set_precision(p, RM);
    r
}

/// mpmath.polyroots(coeffs, maxsteps, cleanup=True, extraprec, error) inside an extended workdps block; None when a
/// coefficient is not a real or complex number (the caller keeps its double-precision path).
pub fn polyroots_ext(coeffs: &[Py], maxsteps: usize, extraprec: usize, error: bool) -> Option<R> {
    // bounded work: a larger request goes to the double-precision path
    if extraprec > 4096 || maxsteps > 100_000 {
        return None;
    }
    let orig = prec();
    let wp = orig.checked_add(extraprec)?;
    let mut cs: Vec<(MC, bool)> = vec![];                    // (value, is complex)
    for c in coeffs {
        cs.push(match c {
            Py::Complex(r, i) => ((BigFloat::from_f64(*r, 64), BigFloat::from_f64(*i, 64)), true),
            Py::Bool(_) | Py::Int(_) | Py::Big(_) | Py::Float(_) | Py::Frac(_) | Py::Mpf(_) => {
                ((of(c)?, BigFloat::from_i64(0, 64)), false)
            }
            _ => return None,
        });
    }
    if cs.len() <= 1 {
        if cs.is_empty() || (cs[0].0 .0.is_zero() && cs[0].0 .1.is_zero()) {
            return Some(Err(crate::py::value_error("Input to polyroots must not be the zero polynomial")));
        }
        return Some(Ok(Py::list(vec![])));
    }
    if orig > 1000 {
        return None;
    }
    Some((|| -> R {
        let tol = BigFloat::from_f64(libm::ldexp(1.0, 1 - orig as i32), 64);   // ctx.eps = 2^(1 - prec)
        let deg = cs.len() - 1;
        let (lead, lead_cx) = cs[0].clone();
        let one = BigFloat::from_i64(1, 64);
        let co: Vec<MC> = if lead.0 == one && lead.1.is_zero() {
            cs.iter().map(|(v, _)| v.clone()).collect()
        } else if !lead_cx {
            if lead.0.is_zero() {
                return Err(PyErr::ZeroDivision);
            }
            cs.iter().map(|(v, cx)| if *cx {
                (v.0.div(&lead.0, wp, RM), v.1.div(&lead.0, wp, RM))            // mpc_div_mpf
            } else {
                (v.0.div(&lead.0, wp, RM), BigFloat::from_i64(0, 64))
            }).collect()
        } else {
            let mut out = vec![];
            for (v, _) in &cs {
                out.push(mc_div(v, &lead, wp)?);                               // mpc_div (a real c made complex)
            }
            out
        };
        let polyval = |x: &MC| -> MC {
            let mut p = co[0].clone();
            for c in &co[1..] {
                p = mc_add(c, &mc_mul(x, &p, wp), wp);
            }
            p
        };
        let mut roots: Vec<MC> = (0..deg).map(|n| {
            let z = crate::pycx::pow((0.4, 0.9), (n as f64, 0.0)).unwrap_or((1.0, 0.0));
            (BigFloat::from_f64(z.0, 64), BigFloat::from_f64(z.1, 64))
        }).collect();
        let mut err: Vec<BigFloat> = vec![BigFloat::from_i64(1, 64); deg];
        let max_err = |err: &Vec<BigFloat>| -> BigFloat {
            let mut m = err[0].clone();
            for e in &err[1..] {
                if e > &m {
                    m = e.clone();
                }
            }
            m
        };
        for _ in 0..maxsteps {
            if max_err(&err) < tol {
                break;
            }
            for i in 0..deg {
                let p = roots[i].clone();
                let mut x = polyval(&p);
                for j in 0..deg {
                    if i != j {
                        let d = mc_sub(&p, &roots[j], wp);
                        if let Ok(q) = mc_div(&x, &d, wp) {
                            x = q;
                        }
                    }
                }
                roots[i] = mc_sub(&p, &x, wp);
                err[i] = mc_abs(&x, wp);
            }
        }
        if max_err(&err) >= tol {
            return Err(PyErr::Other("NoConvergence".into()));
        }
        // cleanup: (value, is real)
        let zero = BigFloat::from_i64(0, 64);
        let mut out: Vec<(MC, bool)> = roots.iter().map(|r| {
            if mc_abs(r, wp) < tol {
                ((zero.clone(), zero.clone()), true)
            } else if r.1.abs() < tol {
                ((r.0.clone(), zero.clone()), true)
            } else if r.0.abs() < tol {
                ((zero.clone(), r.1.clone()), false)
            } else {
                (r.clone(), false)
            }
        }).collect();
        out.sort_by(|a, b| {
            let (ia, ib) = (a.0 .1.abs(), b.0 .1.abs());
            ia.partial_cmp(&ib).unwrap_or(std::cmp::Ordering::Equal)
                .then(a.0 .0.partial_cmp(&b.0 .0).unwrap_or(std::cmp::Ordering::Equal))
        });
        let vals: Vec<Py> = out.iter().map(|((re, im), real)| {
            if *real {
                wrap(rounded(re, orig))
            } else {
                Py::Complex(to_f64(&rounded(re, orig)), to_f64(&rounded(im, orig)))
            }
        }).collect();
        if error {
            let mut e = max_err(&err);
            let floor = BigFloat::from_f64(libm::ldexp(1.0, 1 - orig as i32), 64);
            if floor > e {
                e = floor;
            }
            return Ok(Py::tuple(vec![Py::list(vals), wrap(rounded(&e, orig))]));
        }
        Ok(Py::list(vals))
    })())
}

thread_local! {
    /// mp.pi and mp.e per precision, as built (a value is immutable; a few precisions occur; bounded)
    static CONSTANTS: RefCell<Vec<(bool, usize, Py)>> = const { RefCell::new(Vec::new()) };
}

/// mp.pi and mp.e at an extended precision; None at a double's
pub fn constant(name: &str) -> Option<Py> {
    if !extended() {
        return None;
    }
    let p = prec();
    let is_pi = match name {
        "pi" => true,
        "e" => false,
        _ => return None,
    };
    if let Some(v) = CONSTANTS.with(|c| c.borrow().iter().find(|(k, q, _)| *k == is_pi && *q == p).map(|e| e.2.clone())) {
        return Some(v);
    }
    let v = constant1(name)?;
    CONSTANTS.with(|c| {
        let mut c = c.borrow_mut();
        if c.len() >= 32 {
            c.clear();
        }
        c.push((is_pi, p, v.clone()));
    });
    Some(v)
}

fn constant1(name: &str) -> Option<Py> {
    let p = prec();
    let v = with_cc(|cc| match name {
        "pi" => Some(cc.pi(p, RM)),
        "e" => Some(cc.e(p, RM)),
        _ => None,
    })?;
    Some(wrap(v))
}

/// mp.fsum of numbers at an extended precision
pub fn fsum(items: &[Py]) -> Option<R> {
    if !extended() {
        return None;
    }
    let p = prec() + 64;
    let mut s = BigFloat::from_i64(0, p);
    for x in items {
        match of(x) {
            Some(v) => s = s.add(&v, p, RM),
            None => return Some(Err(PyErr::Type("fsum of a non-number".into()))),
        }
    }
    Some(Ok(wrap(s.add(&BigFloat::from_i64(0, prec()), prec(), RM))))
}

/// The text mpmath shows for an mpf (repr): mpf('...') with the digits of the working precision
pub fn repr(m: &M) -> String {
    format!("mpf('{}')", crate::num::py_repr_f64(to_f64(&m.0)))
}

/// The multiple-precision value of a symbolic expression at the working precision (the package's _compile with
/// mpmath inside a workdps block): the same refusals as crate::pysym::eval_mp
pub fn eval_ex(e: &crate::sym::Ex, var: &dyn Fn(&str) -> Option<BigFloat>) -> Result<BigFloat, crate::pysym::Stop> {
    eval_ex_c(e, var, prec())
}

/// eval_ex with the rational constants at `cp` bits: the package's _compile turns a rational into mp.mpf(p)/q when
/// it builds the callable, at the precision of that moment (53 bits outside a workdps block), while pi and e and every
/// operation take the precision of the call. Every operation is rounded to exactly the working precision (round_bits).
pub fn eval_ex_c(e: &crate::sym::Ex, var: &dyn Fn(&str) -> Option<BigFloat>, cp: usize) -> Result<BigFloat, crate::pysym::Stop> {
    use crate::pysym::Stop;
    use crate::sym::{Ex, Fun::*};
    use num_traits::ToPrimitive;
    let p = prec();
    let rb = |b: BigFloat| round_bits(b, p);
    let int = |k: i64| BigFloat::from_i64(k, p);
    let big = |n: &num_bigint::BigInt| match n.to_i64() {
        Some(k) => BigFloat::from_i64(k, p),
        None => parse_dec(&n.to_string(), p),
    };
    Ok(match e {
        Ex::Num(q) => {
            // mp.mpf(q.p) / q.q at cp bits (each step rounded to cp), then used as that value
            let cpw = cp.max(1);
            let n = round_bits(big(q.numer()), cpw);
            if q.denom() == &num_bigint::BigInt::from(1) {
                n
            } else {
                round_bits(n.div(&big(q.denom()), cpw + 64, RM), cpw)
            }
        }
        Ex::Pi => rb(with_cc(|cc| cc.pi(p, RM))),
        Ex::E => rb(with_cc(|cc| cc.e(p, RM))),
        Ex::Sym(s) => var(s).ok_or(Stop::Value)?,
        Ex::Zoo | Ex::Nan => return Err(Stop::Value),
        Ex::Add(ts) => {
            let mut s = eval_ex_c(&ts[0], var, cp)?;
            for t in &ts[1..] {
                s = rb(s.add(&eval_ex_c(t, var, cp)?, p, RM));
            }
            s
        }
        Ex::Mul(fs) => {
            let mut s = eval_ex_c(&fs[0], var, cp)?;
            for f in &fs[1..] {
                s = rb(s.mul(&eval_ex_c(f, var, cp)?, p, RM));
            }
            s
        }
        Ex::Pow(b, k) => {
            let bv = eval_ex_c(b, var, cp)?;
            if let Ex::Num(q) = &**k {
                if q.is_integer() {
                    let n = q.to_integer().to_i64().ok_or(Stop::Overflow)?;
                    if bv.is_zero() && n < 0 {
                        return Err(Stop::ZeroDivision);
                    }
                    let r = rb(bv.powi(n.unsigned_abs() as usize, p, RM));
                    return Ok(if n < 0 { rb(int(1).div(&r, p, RM)) } else { r });
                }
                if *q == crate::sym::q(1, 2) {
                    if bv.is_negative() {
                        return Err(Stop::NotReal);
                    }
                    return Ok(rb(bv.sqrt(p, RM)));
                }
            }
            let kv = eval_ex_c(k, var, cp)?;
            if bv.is_zero() {
                if kv.is_negative() {
                    return Err(Stop::ZeroDivision);
                }
                return Ok(int(if kv.is_zero() { 1 } else { 0 }));
            }
            if bv.is_negative() {
                if kv.is_int() {
                    let n = to_f64(&kv) as i64;
                    let r = rb(bv.powi(n.unsigned_abs() as usize, p, RM));
                    return Ok(if n < 0 { rb(int(1).div(&r, p, RM)) } else { r });
                }
                return Err(Stop::NotReal);
            }
            rb(with_cc(|cc| bv.pow(&kv, p, RM, cc)))
        }
        Ex::Fun(f, a) => {
            let x = eval_ex_c(a, var, cp)?;
            with_cc(|cc| -> Result<BigFloat, Stop> {
                Ok(match f {
                    Sin => msin(&x, p, cc),
                    Cos => mcos(&x, p, cc),
                    Tan => x.tan(p, RM, cc),
                    Sec => int(1).div(&mcos(&x, p, cc), p, RM),
                    Csc | Cot => {
                        let s = msin(&x, p, cc);
                        if s.is_zero() {
                            return Err(Stop::ZeroDivision);
                        }
                        if *f == Csc { int(1).div(&s, p, RM) } else { mcos(&x, p, cc).div(&s, p, RM) }
                    }
                    Asin | Acos => {
                        if to_f64(&x).abs() > 1.0 {
                            return Err(Stop::NotReal);
                        }
                        if *f == Asin { x.asin(p, RM, cc) } else { x.acos(p, RM, cc) }
                    }
                    Atan => x.atan(p, RM, cc),
                    Sinh => x.sinh(p, RM, cc),
                    Cosh => x.cosh(p, RM, cc),
                    Tanh => x.tanh(p, RM, cc),
                    Exp => mexp(&x, p, cc),
                    Log => {
                        if x.is_negative() {
                            return Err(Stop::NotReal);
                        }
                        if x.is_zero() {
                            BigFloat::from_f64(f64::NEG_INFINITY, p)
                        } else {
                            x.ln(p, RM, cc)
                        }
                    }
                    Abs => x.abs(),
                })
            }).map(rb)?
        }
    })
}

/// (sign, ln|value|) of an expression with one variable at x > 0, at p bits: values far beyond the exponent range of
/// a multiple-precision number (4^n at n = 10^12, which the ratio test's self-check evaluates; mpmath's exponents
/// are unbounded) are carried by their logarithms. Sign 0 is an exact zero. Recursion depth is bounded.
fn eval_log(e: &crate::sym::Ex, v: &str, x: &BigFloat, p: usize, depth: usize) -> Result<(i32, BigFloat), crate::pysym::Stop> {
    use crate::pysym::Stop;
    use crate::sym::{Ex, Fun};
    use num_traits::ToPrimitive;
    if depth > 64 {
        return Err(Stop::Overflow);
    }
    let zero = || (0, BigFloat::from_i64(0, p));
    let of_value = |b: BigFloat| -> (i32, BigFloat) {
        if b.is_zero() {
            return zero();
        }
        let s = if b.is_negative() { -1 } else { 1 };
        (s, with_cc(|cc| b.abs().ln(p, RM, cc)))
    };
    // the value back from its logarithm, where it is of a size a number holds
    let value = |(s, l): &(i32, BigFloat)| -> Result<BigFloat, Stop> {
        if *s == 0 {
            return Ok(BigFloat::from_i64(0, p));
        }
        if to_f64(l).abs() > 1.0e5 {
            return Err(Stop::Overflow);
        }
        let m = with_cc(|cc| mexp(l, p, cc));
        Ok(if *s < 0 { m.neg() } else { m })
    };
    let big = |n: &num_bigint::BigInt| match n.to_i64() {
        Some(k) => BigFloat::from_i64(k, p),
        None => parse_dec(&n.to_string(), p),
    };
    Ok(match e {
        Ex::Num(q) => of_value(big(q.numer()).div(&big(q.denom()), p, RM)),
        Ex::Pi => of_value(with_cc(|cc| cc.pi(p, RM))),
        Ex::E => (1, BigFloat::from_i64(1, p)),
        Ex::Sym(s) if &**s == v => of_value(x.clone()),
        Ex::Sym(_) | Ex::Zoo | Ex::Nan => return Err(Stop::Value),
        Ex::Mul(fs) => {
            let (mut s, mut l) = (1, BigFloat::from_i64(0, p));
            for f in fs.iter() {
                let (sf, lf) = eval_log(f, v, x, p, depth + 1)?;
                if sf == 0 {
                    return Ok(zero());
                }
                s *= sf;
                l = l.add(&lf, p, RM);
            }
            (s, l)
        }
        Ex::Add(ts) => {
            let parts: Vec<(i32, BigFloat)> = ts.iter().map(|t| eval_log(t, v, x, p, depth + 1)).collect::<Result<_, _>>()?;
            let nz: Vec<&(i32, BigFloat)> = parts.iter().filter(|(s, _)| *s != 0).collect();
            let Some(m) = nz.iter().map(|(_, l)| l.clone()).max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)) else {
                return Ok(zero());
            };
            let mut sum = BigFloat::from_i64(0, p);
            for (s, l) in nz {
                let d = l.sub(&m, p, RM);
                if to_f64(&d) < -(p as f64) - 64.0 {
                    continue;                                   // below the precision of the sum
                }
                let t = with_cc(|cc| mexp(&d, p, cc));
                sum = if *s < 0 { sum.sub(&t, p, RM) } else { sum.add(&t, p, RM) };
            }
            let (s, l) = of_value(sum);
            if s == 0 {
                return Ok(zero());
            }
            (s, l.add(&m, p, RM))
        }
        Ex::Pow(b, k) => {
            let kv = value(&eval_log(k, v, x, p, depth + 1)?)?;
            let (sb, lb) = eval_log(b, v, x, p, depth + 1)?;
            if sb == 0 {
                if kv.is_negative() || kv.is_zero() {
                    return if kv.is_zero() { Ok((1, BigFloat::from_i64(0, p))) } else { Err(Stop::ZeroDivision) };
                }
                return Ok(zero());
            }
            let mut s = 1;
            if sb < 0 {
                if !kv.is_int() {
                    return Err(Stop::NotReal);
                }
                let half = kv.div(&BigFloat::from_i64(2, p), p, RM);
                if !half.is_int() {
                    s = -1;
                }
            }
            (s, kv.mul(&lb, p, RM))
        }
        Ex::Fun(f, a) => {
            let la = eval_log(a, v, x, p, depth + 1)?;
            match f {
                Fun::Exp => (1, value(&la)?),
                Fun::Log => {
                    if la.0 <= 0 {
                        return Err(if la.0 == 0 { Stop::ZeroDivision } else { Stop::NotReal });
                    }
                    of_value(la.1)
                }
                Fun::Abs => (la.0.abs(), la.1),
                _ => {
                    let av = value(&la)?;
                    let t = Ex::Fun(*f, std::rc::Rc::new(Ex::Sym(std::rc::Rc::from("__a"))));
                    let _g = set(p);
                    of_value(eval_ex_c(&t, &|s: &str| (s == "__a").then(|| av.clone()), p)?)
                }
            }
        }
    })
}

/// |g(N + 1) / g(N)| at a large N for the ratio test's self-check (the package evaluates it with mpmath at 40 digits,
/// whose exponents are unbounded): in logarithms at 40 digits plus 64 guard bits, then the nearest double
pub fn ratio_at(e: &crate::sym::Ex, v: &str, n: i64) -> Result<f64, crate::pysym::Stop> {
    let p = dps_to_prec(40.0) + 64;
    let x0 = BigFloat::from_i64(n, p);
    let x1 = BigFloat::from_i64(n + 1, p);
    let (s0, l0) = eval_log(e, v, &x0, p, 0)?;
    let (s1, l1) = eval_log(e, v, &x1, p, 0)?;
    if s0 == 0 {
        return Err(crate::pysym::Stop::ZeroDivision);
    }
    if s1 == 0 {
        return Ok(0.0);
    }
    let d = l1.sub(&l0, p, RM);
    if to_f64(&d).abs() > 700.0 {
        return Ok(if to_f64(&d) > 0.0 { f64::INFINITY } else { 0.0 });
    }
    Ok(to_f64(&with_cc(|cc| mexp(&d, p, cc))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_bit_rounding() {
        // 1/3 at 70 bits: the low 58 bits of the two words are clear, and the value is 1/3 rounded at bit 70
        let x = BigFloat::from_i64(1, 192).div(&BigFloat::from_i64(3, 192), 192, RM);
        let r = round_bits(x.clone(), 70);
        let (m, _, _, _, _) = r.as_raw_parts().unwrap();
        let wb = std::mem::size_of::<astro_float::Word>() * 8;
        let drop = m.len() * wb - 70;
        assert!(m[..drop / wb].iter().all(|w| *w == 0));
        assert!(r.sub(&x, 192, RM).abs().div(&x, 192, RM) <= BigFloat::from_f64(2f64.powi(-70), 64));
        // a carry out of the top: 0.111...1 (all ones) rounds up to 1.0
        let ones = BigFloat::from_i64(1, 128).sub(&BigFloat::from_f64(2f64.powi(-100), 128), 128, RM);
        assert_eq!(to_f64(&round_bits(ones, 70)), 1.0);
        // already short: unchanged
        assert_eq!(to_f64(&round_bits(BigFloat::from_f64(0.1, 64), 70)), 0.1);
    }

    #[test]
    fn ratio_in_logarithms() {
        let e = crate::symparse::parse("4^n/(n + 1)", &["n"]).unwrap();
        let l = ratio_at(&e, "n", 1_000_000_000_000).unwrap();
        assert!((l - 4.0).abs() < 1e-11, "{l}");
        let e = crate::symparse::parse("n^3*(3/2)^n", &["n"]).unwrap();
        assert!((ratio_at(&e, "n", 1_000_000_000_000).unwrap() - 1.5).abs() < 1e-11);
        let e = crate::symparse::parse("(1/2)^n/(n^2 + 1)", &["n"]).unwrap();
        assert!((ratio_at(&e, "n", 1_000_000_000_000).unwrap() - 0.5).abs() < 1e-11);
    }

    #[test]
    fn thirty_digits_as_mpmath() {
        let _g = scope("workdps", &Py::Int(30)).unwrap();
        assert_eq!(prec(), 103);
        // (1 + 1e-20) - 1 keeps its 1e-20 at 30 digits (a double gives 0)
        let one = mpf(&Py::Int(1)).unwrap();
        let tiny = mpf(&Py::Str(Rc::from("1e-20"))).unwrap();
        let s = bin(&bin(&one, &tiny, "+").unwrap().unwrap(), &one, "-").unwrap().unwrap();
        // to 103 bits exactly, as mpmath: 9.99999999997452451411988619035502e-21 (1 + 1e-20 rounded at 2^-102)
        assert_eq!(s.f().unwrap(), 9.999999999974525e-21);
        // (1 + 1/x)^x at x = 1e20: mpmath's 2.7182818284521204 (1 + 1e-20 at 103 bits carries 1e-11 of the exponent)
        let x = mpf(&Py::Str(Rc::from("1e20"))).unwrap();
        let base = bin(&one, &bin(&one, &x, "/").unwrap().unwrap(), "+").unwrap().unwrap();
        let v = bin(&base, &x, "**").unwrap().unwrap().f().unwrap();
        assert_eq!(v, 2.7182818284521204);
        drop(_g);
        assert_eq!(prec(), 53);
        assert!(matches!(mpf(&Py::Int(1)).unwrap(), Py::Float(_)));
    }

    /// the fixed-point exp, sin and cos: within one unit in the last place of astro-float's, and correctly rounded
    /// (against astro-float with 64 more bits, rounded)
    #[test]
    fn fixed_point_functions_as_astro() {
        let mut cc = Consts::new().unwrap();
        let (mut n, mut same) = (0usize, 0usize);
        let (mut fast_true, mut astro_true) = (0usize, 0usize);
        let mut seed: u64 = 0x2545F4914F6CDD1D;
        let mut rnd = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            (seed >> 11) as f64 / (1u64 << 53) as f64
        };
        for &p in &[64usize, 103, 133, 200, 336] {
            for i in 0..1500 {
                let mag = 10f64.powf(rnd() * 9.0 - 4.0);          // 1e-4 .. 1e5
                let x0 = if i % 2 == 0 { mag } else { -mag };
                let x = BigFloat::from_f64(x0, 64).div(&BigFloat::from_i64(3, p), p, RM);   // not a double
                let checks: Vec<(&str, Option<BigFloat>, BigFloat)> = vec![
                    ("exp", if x0.abs() < 700.0 { fast_exp(&x, p, &mut cc) } else { None }, x.exp(p, RM, &mut cc)),
                    ("sin", fast_sincos(&x, p, &mut cc).map(|v| v.0), x.sin(p, RM, &mut cc)),
                    ("cos", fast_sincos(&x, p, &mut cc).map(|v| v.1), x.cos(p, RM, &mut cc)),
                ];
                for (name, fast, exact) in checks {
                    let Some(fast) = fast else { continue };
                    n += 1;
                    let mut truth = match name { "exp" => x.exp(p + 64, RM, &mut cc), "sin" => x.sin(p + 64, RM, &mut cc), _ => x.cos(p + 64, RM, &mut cc) };
                    truth.set_precision(p, RM).unwrap();
                    if fast.sub(&truth, p + 64, RM).is_zero() { fast_true += 1; }
                    if exact.sub(&truth, p + 64, RM).is_zero() { astro_true += 1; }
                    let d = fast.sub(&exact, p + 64, RM).abs();
                    if d.is_zero() {
                        same += 1;
                        continue;
                    }
                    // one unit in the last place of the exact value
                    let ulp = BigFloat::from_i64(1, p).mul(&BigFloat::from_f64(2f64.powi(exact.exponent().unwrap() - p as i32), 64), p, RM);
                    assert!(d.cmp(&ulp) <= Some(0), "{name}({x0}/3) at {p} bits: {fast} vs {exact}");
                }
            }
        }
        eprintln!("fast correctly rounded {fast_true} of {n}; astro {astro_true} of {n}");
        assert!(n > 15000, "{n} checked");
        // astro-float at the working precision is within one unit but correctly rounded only about one time in five
        // (2026-10-30: 4,135 of 20,715); the fixed-point values are the correctly rounded ones, as mpmath's are
        assert!(fast_true * 10000 >= n * 9999, "{fast_true} of {n} correctly rounded");
        eprintln!("fixed point: {same} of {n} equal to astro-float");
    }

    /// sin and cos next to multiples of pi/2 (the reduced argument below 2^-24): correctly rounded, as the reduction
    /// is redone at a wider scale
    #[test]
    fn trig_near_multiples_of_half_pi() {
        let mut cc = Consts::new().unwrap();
        let (mut n, mut right) = (0usize, 0usize);
        for &p in &[70usize, 90, 103, 133, 200] {
            for k in 1i64..40 {
                for d in (26..=150).step_by(7) {
                    for sgn in [1i64, -1] {
                        let wp = p + 200;
                        let base = cc.pi(wp, RM).mul(&BigFloat::from_i64(k, wp), wp, RM).div(&BigFloat::from_i64(2, wp), wp, RM);
                        let delta = BigFloat::from_f64(sgn as f64 * 2f64.powi(-(d as i32)) * 1.37, 64);
                        let mut x = base.add(&delta, wp, RM);
                        x.set_precision(p, RM).unwrap();
                        for cosine in [false, true] {
                            let Some(fast) = fast_trig(&x, p, &mut cc, cosine) else { continue };
                            let mut truth = if cosine { x.cos(p + 256, RM, &mut cc) } else { x.sin(p + 256, RM, &mut cc) };
                            truth.set_precision(p, RM).unwrap();
                            n += 1;
                            if fast.sub(&truth, p + 256, RM).is_zero() {
                                right += 1;
                            }
                        }
                    }
                }
            }
        }
        // large arguments (1e6 .. 1e14, where pi/2 takes 96 extra bits)
        let mut seed: u64 = 0x2545F4914F6CDD1D;
        for &p in &[70usize, 90, 133] {
            for i in 0..600 {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                let mag = 10f64.powf(6.0 + (seed >> 11) as f64 / (1u64 << 53) as f64 * 8.0);
                let x = BigFloat::from_f64(if i % 2 == 0 { mag } else { -mag }, 64).div(&BigFloat::from_i64(7, p), p, RM);
                for cosine in [false, true] {
                    let Some(fast) = fast_trig(&x, p, &mut cc, cosine) else { continue };
                    let mut truth = if cosine { x.cos(p + 256, RM, &mut cc) } else { x.sin(p + 256, RM, &mut cc) };
                    truth.set_precision(p, RM).unwrap();
                    n += 1;
                    if fast.sub(&truth, p + 256, RM).is_zero() {
                        right += 1;
                    }
                }
            }
        }
        assert!(n > 6000, "{n}");
        assert!(right * 1000 >= n * 999, "{right} of {n} correctly rounded");
    }

    /// the fixed-size word series give exactly the integers of the arbitrary-size ones (signs, floors, truncations)
    #[test]
    fn word_series_exactly_as_bigint() {
        use num_bigint::BigInt;
        let mut seed: u64 = 0x9E3779B97F4A7C15;
        let mut rnd = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        let mut n = 0;
        for &w in &[64usize, 117, 154, 167, 200, 250] {
            for i in 0..400 {
                // |r| up to pi/4 (trig) or ln2/2 / 2^10 (exp) at scale w, either sign
                let a = BigInt::from(rnd()) << (w.saturating_sub(64));
                let a: BigInt = a >> (i % 40usize);
                let r = if i % 2 == 0 { a.clone() } else { -a.clone() };
                let rt = (&r * BigInt::from(785)) / BigInt::from(1000);
                for cosine in [false, true] {
                    assert_eq!(trig_series(&rt, w, cosine), trig_series_big(&rt, w, cosine), "trig w={w} i={i}");
                }
                let re = &r >> 12usize;
                assert_eq!(exp_series(&re, w, 10), exp_series_big(&re, w, 10), "exp w={w} i={i}");
                n += 1;
            }
        }
        assert_eq!(n, 2400);
    }

    #[test]
    fn nearest_double() {
        // normal doubles (subnormals, below 2.2e-308, do not occur in these procedures)
        for x in [1.0, -2.5, 0.1, 1e-300, 6.02e23, std::f64::consts::PI, 2.2250738585072014e-308, 1.7976931348623157e308] {
            assert_eq!(to_f64(&BigFloat::from_f64(x, 128)), x);
        }
    }

    #[test]
    fn polyroots_as_mpmath() {
        // mpmath 1.3 at workdps(30): polyroots([mpf(str(c)) ...], maxsteps, extraprec), parts as doubles (2026-10-30)
        let run = |c: &[&str], ms: usize, ep: usize| -> Result<Vec<(f64, f64)>, PyErr> {
            let _g = scope("workdps", &Py::Float(30.0)).unwrap();
            let co: Vec<Py> = c.iter().map(|s| mpf(&Py::str(s)).unwrap()).collect();
            let r = polyroots_ext(&co, ms, ep, false).unwrap()?;
            Ok(r.items().unwrap().iter().map(|z| match z {
                Py::Complex(a, b) => (*a, *b),
                x => (x.f().unwrap(), 0.0),
            }).collect())
        };
        assert_eq!(run(&["1", "0", "0", "0", "0"], 200, 60).unwrap(),
                   vec![(-2.23107988278048e-31, 0.0), (2.6370946664450856e-31, 1.986339955098839e-31),
                        (0.0, -2.05109675469105e-31), (0.0, 2.426029394014883e-31)]);
        assert!(matches!(run(&["1.0", "3.0", "3.0", "1.0"], 400, 80), Err(PyErr::Other(_))));
        assert_eq!(run(&["2.5", "-1.25", "3.75", "0.5", "-7.0"], 200, 60).unwrap(),
                   vec![(-1.0, 0.0), (1.095182684698218, 0.0), (0.20240865765089097, 1.5860901049825114),
                        (0.20240865765089097, -1.5860901049825114)]);
        assert_eq!(run(&["1", "-6", "11", "-6.0"], 200, 60).unwrap(), vec![(1.0, 0.0), (2.0, 0.0), (3.0, 0.0)]);
        assert_eq!(run(&["1.0", "2.0", "1.0"], 200, 60).unwrap(), vec![(-1.0, 0.0), (-1.0, 0.0)]);
        assert_eq!(run(&["1.0", "0.3", "-2.0"], 50, 10).unwrap(),
                   vec![(-1.5721462653327891, 0.0), (1.272146265332789, 0.0)]);
    }

    #[test]
    fn e1_matches_mpmath() {
        // float(mpmath.e1(x)) (mpmath 1.3, 2026-10-30): both sides of the series / continued-fraction switch at 20
        let cases = [(1e-12, 27.053805451028015), (1e-6, 13.23829589306249), (0.01, 4.037929576538114),
                     (0.3, 0.9056766516758468), (1.0, 0.21938393439552029), (2.5, 0.024914917870269736),
                     (7.0, 0.00011548173161033822), (15.0, 1.918627892147867e-08), (19.9, 1.092215649960176e-10),
                     (20.1, 8.857204800579297e-11), (35.0, 1.752705938994737e-17), (80.0, 2.228543258688473e-37),
                     (300.0, 1.71038427680451e-133), (700.0, 1.406518766234033e-307)];
        for (x, want) in cases {
            for p in [64usize, 86, 120] {
                let got = to_f64(&e1(&BigFloat::from_f64(x, 128), p).unwrap());
                assert_eq!(got, want, "e1({x}) at {p} bits");
            }
        }
    }
}
