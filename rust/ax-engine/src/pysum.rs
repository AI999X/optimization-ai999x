//! Infinite series for the calculus package's series_sum, which calls sympy.summation(a_n, (n, k, oo)), and its
//! self-check _nsum (a numerical sum). Written by hand: the Rust runtime has no sympy.
//!
//! summation: the term is split into addends c * rho^n * R(n) (rho a rational ratio from powers b^(a n + c) with
//! rational b, R a rational function), grouped by rho, and each group summed:
//!   - rho = 1: partial fractions over the rational roots of the denominator. Poles of order j >= 2 give Hurwitz zeta
//!     values: at integer points zeta(j) minus a finite sum (zeta(2m) = rational * pi^(2m), exactly), at half-integer
//!     points through (2^j - 1) zeta(j), at other rational points numerically. Simple poles give digamma differences:
//!     within one class of points mod 1 a finite sum of rationals; when the simple poles of two classes do not cancel
//!     separately, sympy returns nan (it splits the sum into divergent parts), and so does this. A polynomial part
//!     or simple poles that do not cancel diverge, as does a pole on an index of the sum (sympy's zoo);
//!   - |rho| < 1: a polynomial part in the binomial basis (sum C(m, j) rho^m = rho^j / (1 - rho)^(j+1)), simple poles
//!     at integers through -log(1 - rho);
//!   - rho = -1: poles at integers through the alternating zeta values (eta(1) = log 2, eta(j) = (1 - 2^(1-j)) zeta(j)),
//!     at other rational points numerically (Hurwitz zeta and digamma); a polynomial part oscillates, which series_sum
//!     reports as divergent (sympy leaves such a Sum unevaluated, which holds oo);
//!   - rho > 1 or rho < -1: divergent;
//!   - a single power c n^(-p) with a rational p that is not an integer: zeta(p) less a finite sum, divergent for p <= 1.
//! A divergent series is returned as zoo (series_sum refuses "the series diverges"), sympy's nan as nan. Anything else
//! (a denominator with complex or irrational roots, polylog-type terms, exp or trigonometric terms) is an internal
//! error (Unsupported): never a guessed number. Values computed numerically are carried as the nearest double.
//!
//! Every size is bounded before work: degrees, pole orders, shifts, exponent coefficients and the bits of rho^k.

use crate::py::{Py, PyErr, R};
use crate::pysym::{Poly, Root};
use crate::sym::{self, Ex, Fun};
use num_bigint::BigInt;
use num_rational::BigRational as Q;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::rc::Rc;

const MAX_DEG: i64 = 24;            // degree of a numerator or denominator summed here
const MAX_SHIFT: i64 = 100_000;     // length of a finite correction sum (start index minus a pole)
const MAX_EXP_COEF: i64 = 64;       // |a| in b^(a n + c)
const MAX_EXP_CONST: i64 = 4096;    // |c| in b^(a n + c)
const MAX_BASE_BITS: u64 = 256;     // bits of b's numerator plus denominator
const MAX_POW_BITS: u64 = 4_000_000; // bits of rho^k (k the start index or a pole)
const MAX_ACC_BITS: u64 = 1_000_000; // bits of the exact running sum of a finite correction

/// the exact running sum stays within MAX_ACC_BITS (checked after every term, so the work is bounded)
fn acc_ok(q: &Q) -> Result<(), PyErr> {
    if bits(q) > MAX_ACC_BITS { Err(unsup("a finite correction too large to hold exactly")) } else { Ok(()) }
}

fn unsup(m: impl Into<String>) -> PyErr {
    PyErr::Unsupported(format!("summation: {}", m.into()))
}

fn qi(n: i64) -> Q {
    Q::from_integer(BigInt::from(n))
}

fn bits(q: &Q) -> u64 {
    q.numer().bits() + q.denom().bits()
}

fn qf(q: &Q) -> f64 {
    q.numer().to_f64().unwrap_or(f64::NAN) / q.denom().to_f64().unwrap_or(f64::NAN)
}

/// A double as the exact rational of its decimal repr (how a numerically computed part is carried)
fn fq(v: f64) -> Result<Q, PyErr> {
    match sym::rational_of_repr(v) {
        Some(Ex::Num(q)) => Ok(q),
        _ => Err(unsup("a value that is not finite")),
    }
}

/// q^e exactly (e may be negative; q nonzero then), the size checked first
fn qpow(q: &Q, e: i64) -> Result<Q, PyErr> {
    if e == 0 {
        return Ok(Q::one());
    }
    if q.is_zero() {
        return if e > 0 { Ok(Q::zero()) } else { Err(unsup("zero to a negative power")) };
    }
    if bits(q).saturating_mul(e.unsigned_abs()) > MAX_POW_BITS {
        return Err(unsup("a power too large to hold exactly"));
    }
    let mut base = if e < 0 { Q::one() / q } else { q.clone() };
    let mut n = e.unsigned_abs();
    let mut out = Q::one();
    while n > 0 {
        if n & 1 == 1 {
            out *= &base;
        }
        n >>= 1;
        if n > 0 {
            base = &base * &base;
        }
    }
    Ok(out)
}

fn has_var(e: &Ex, x: &str) -> bool {
    let mut free = vec![];
    sym::free_symbols(e, &mut free);
    free.iter().any(|s| &**s == x)
}

/// A factor b^(a x + c) with rational b and integers a, c: (b^c, b^a)
fn geometric(f: &Ex, x: &Rc<str>, depth: usize) -> Result<Option<(Q, Q)>, PyErr> {
    if depth > 8 {
        return Ok(None);
    }
    let Ex::Pow(b, p) = f else { return Ok(None) };
    if !has_var(p, x) {
        // (b^(a x + c))^m for an integer m
        if let (Ex::Num(m), true) = (&**p, has_var(b, x)) {
            if m.is_integer() && m.abs() <= qi(MAX_EXP_COEF) {
                if let Some((c, r)) = geometric(b, x, depth + 1)? {
                    let m = m.to_integer().to_i64().unwrap_or(0);
                    return Ok(Some((qpow(&c, m)?, qpow(&r, m)?)));
                }
            }
        }
        return Ok(None);
    }
    let Ex::Num(base) = &**b else { return Err(unsup("a power of the index with a base that is not rational")) };
    if base.is_zero() || bits(base) > MAX_BASE_BITS {
        return Err(unsup("a power of the index with this base"));
    }
    let lin = match crate::pysym::poly_from(p, x) {
        Ok(l) if l.degree() == 1 && l.c.iter().all(|c| c.is_integer()) => l,
        _ => return Err(unsup("a power of the index whose exponent is not a n + c with integers a, c")),
    };
    let (c0, a1) = (lin.c[0].to_integer().to_i64(), lin.c[1].to_integer().to_i64());
    let (Some(c0), Some(a1)) = (c0, a1) else { return Err(unsup("an exponent coefficient too large")) };
    if a1.abs() > MAX_EXP_COEF || c0.abs() > MAX_EXP_CONST {
        return Err(unsup("an exponent coefficient too large"));
    }
    Ok(Some((qpow(base, c0)?, qpow(base, a1)?)))
}

/// The addends of a term grouped by their ratio rho: (rho, the rest of the group as one expression)
fn groups(e: &Ex, x: &Rc<str>) -> Result<Vec<(Q, Ex)>, PyErr> {
    let addends: Vec<Ex> = match e {
        Ex::Add(ts) => (**ts).clone(),
        other => vec![other.clone()],
    };
    if addends.len() > 64 {
        return Err(unsup("too many terms"));
    }
    let mut out: Vec<(Q, Ex)> = vec![];
    for t in addends {
        let factors: Vec<Ex> = match &t {
            Ex::Mul(fs) => (**fs).clone(),
            other => vec![other.clone()],
        };
        let (mut c, mut rho) = (Q::one(), Q::one());
        let mut rest = vec![];
        for f in factors {
            match geometric(&f, x, 0)? {
                Some((cf, rf)) => {
                    c *= cf;
                    rho *= rf;
                    if bits(&rho) > MAX_BASE_BITS * MAX_EXP_COEF as u64 || bits(&c) > MAX_POW_BITS {
                        return Err(unsup("a power too large to hold exactly"));
                    }
                }
                None => rest.push(f),
            }
        }
        let part = sym::mul(sym::num(c), sym::mul_all(rest));
        match out.iter_mut().find(|(r, _)| *r == rho) {
            Some(g) => g.1 = sym::add(g.1.clone(), part),
            None => out.push((rho, part)),
        }
    }
    Ok(out)
}

/// P(r + t) as a polynomial in t
fn shift(p: &Poly, r: &Q) -> Poly {
    let g = p.gen.clone();
    let lin = Poly::new(g.clone(), vec![r.clone(), Q::one()]);
    let mut out = Poly::new(g.clone(), vec![]);
    for c in p.c.iter().rev() {
        out = out.mul(&lin).add(&Poly::constant(g.clone(), c.clone()));
    }
    out
}

/// The Bernoulli numbers B_0 .. B_n (B_1 = -1/2)
fn bernoulli(n: usize) -> Vec<Q> {
    let mut b: Vec<Q> = vec![Q::one()];
    for m in 1..=n {
        // sum_{i=0}^{m} C(m+1, i) B_i = 0
        let mut s = Q::zero();
        let mut c = BigInt::one();                  // C(m+1, i)
        for (i, bi) in b.iter().enumerate() {
            s += Q::from_integer(c.clone()) * bi;
            c = c * BigInt::from(m + 1 - i) / BigInt::from(i + 1);
        }
        b.push(-s / Q::from_integer(BigInt::from(m + 1)));
    }
    b
}

fn factorial(n: usize) -> BigInt {
    (1..=n).fold(BigInt::one(), |a, k| a * BigInt::from(k))
}

/// B_2m / (2m)! for m = 1..=6 as doubles (Euler-Maclaurin)
fn em_coefficients() -> Vec<f64> {
    let b = bernoulli(12);
    (1..=6).map(|m| qf(&(&b[2 * m] / Q::from_integer(factorial(2 * m))))).collect()
}

/// The Hurwitz zeta function zeta(s, a) = sum_{i>=0} (a + i)^(-s) for s > 1, a > 0 (Euler-Maclaurin from a + i >= 20)
fn hurwitz(s: f64, a: f64) -> f64 {
    let n = if a < 20.0 { (20.0 - a).ceil() as usize } else { 0 };
    let mut sum = 0.0;
    for i in 0..n {
        sum += (a + i as f64).powf(-s);
    }
    let b = a + n as f64;
    sum += b.powf(1.0 - s) / (s - 1.0) + b.powf(-s) / 2.0;
    let mut rising = s;                      // s (s + 1) ... (s + 2m - 2)
    for (m1, c) in em_coefficients().into_iter().enumerate() {
        let m = m1 + 1;
        sum += c * rising * b.powf(-s - 2.0 * m as f64 + 1.0);
        rising *= (s + 2.0 * m as f64 - 1.0) * (s + 2.0 * m as f64);
    }
    sum
}

/// The digamma function for a > 0 (recurrence to a >= 20, then the asymptotic series)
fn digamma(a: f64) -> f64 {
    let n = if a < 20.0 { (20.0 - a).ceil() as usize } else { 0 };
    let mut s = 0.0;
    for i in 0..n {
        s -= 1.0 / (a + i as f64);
    }
    let b = a + n as f64;
    let bern = bernoulli(12);
    s += b.ln() - 1.0 / (2.0 * b);
    for m in 1..=6usize {
        s -= qf(&bern[2 * m]) / (2.0 * m as f64 * b.powi(2 * m as i32));
    }
    s
}

/// The value being built: a rational part, multiples of pi^j and of log(q)
#[derive(Default)]
struct Value {
    q: Q,
    pi: Vec<(usize, Q)>,
    logs: Vec<(Q, Q)>,           // (coefficient, argument)
}

impl Value {
    fn add_pi(&mut self, j: usize, c: Q) {
        if let Some(e) = self.pi.iter_mut().find(|(k, _)| *k == j) {
            e.1 += c;
        } else {
            self.pi.push((j, c));
        }
    }
    fn add_log(&mut self, arg: Q, c: Q) {
        if let Some(e) = self.logs.iter_mut().find(|(_, a)| *a == arg) {
            e.0 += c;
        } else {
            self.logs.push((c, arg));
        }
    }
    /// c * zeta(j), j >= 2: zeta(2m) = (-1)^(m+1) B_2m (2 pi)^(2m) / (2 (2m)!); an odd zeta value as the nearest double
    fn add_zeta(&mut self, j: usize, c: &Q) -> Result<(), PyErr> {
        if j % 2 == 0 {
            let b = bernoulli(j)[j].clone();
            let sign = if (j / 2) % 2 == 1 { Q::one() } else { -Q::one() };
            let k = sign * b * Q::from_integer(BigInt::from(2).pow(j as u32)) / (qi(2) * Q::from_integer(factorial(j)));
            self.add_pi(j, c * k);
        } else {
            let z = crate::pymp::call("zeta", &[Py::Float(j as f64)], &[])?.f()?;
            self.q += c * fq(z)?;
        }
        Ok(())
    }
    fn ex(&self) -> Ex {
        let mut terms = vec![sym::num(self.q.clone())];
        for (j, c) in &self.pi {
            if !c.is_zero() {
                terms.push(sym::mul(sym::num(c.clone()), sym::pow(Ex::Pi, sym::int(*j as i64))));
            }
        }
        for (c, a) in &self.logs {
            if !c.is_zero() {
                terms.push(sym::mul(sym::num(c.clone()), sym::func(Fun::Log, sym::num(a.clone()))));
            }
        }
        sym::add_all(terms)
    }
}

/// The partial fractions of a proper n/d (d splitting over the rationals): (root, [c_1 .. c_m]) for
/// c_j / (x - root)^j
fn partial_fractions(n: &Poly, d: &Poly) -> Result<Vec<(Q, Vec<Q>)>, PyErr> {
    let g = d.gen.clone();
    let (_, factors) = d.sqf_list();
    let mut out = vec![];
    for (f, m) in factors {
        let roots = crate::pysym::real_roots(&f);
        if roots.len() as i64 != f.degree() {
            return Err(unsup("a denominator with complex roots"));
        }
        for r in roots {
            let Root::Exact(r) = r else { return Err(unsup("a denominator with irrational roots")) };
            let m = m as usize;
            // B = d / (x - r)^m; the Taylor coefficients at r of n / B give c_m, c_(m-1), ..
            let mut b = d.clone();
            let lin = Poly::new(g.clone(), vec![-r.clone(), Q::one()]);
            for _ in 0..m {
                let (q, rem) = b.divmod(&lin);
                if !rem.is_zero() {
                    return Err(unsup("partial fractions"));
                }
                b = q;
            }
            let (a, b) = (shift(n, &r), shift(&b, &r));
            let at = |p: &Poly, i: usize| p.c.get(i).cloned().unwrap_or_else(Q::zero);
            let b0 = at(&b, 0);
            if b0.is_zero() {
                return Err(unsup("partial fractions"));
            }
            let mut s: Vec<Q> = vec![];
            for i in 0..m {
                let mut v = at(&a, i);
                for l in 1..=i {
                    v -= at(&b, l) * &s[i - l];
                }
                s.push(v / &b0);
            }
            // s[i] is the coefficient of 1/(x - r)^(m - i)
            let mut cs = vec![Q::zero(); m];
            for (i, v) in s.into_iter().enumerate() {
                cs[m - 1 - i] = v;
            }
            out.push((r, cs));
        }
    }
    Ok(out)
}

fn small_int(q: &Q, what: &str) -> Result<i64, PyErr> {
    if !q.is_integer() {
        return Err(unsup(what.to_string()));
    }
    q.to_integer().to_i64().filter(|v| v.abs() <= MAX_SHIFT).ok_or_else(|| unsup(format!("{what} too far")))
}

#[derive(PartialEq)]
enum Sum {
    Finite,
    Diverges,
    Nan,
}

/// sum_{x=k}^oo of rho^x n(x)/d(x) added into v
fn sum_group(rho: &Q, n: &Poly, d: &Poly, k: i64, v: &mut Value) -> Result<Sum, PyErr> {
    if n.is_zero() {
        return Ok(Sum::Finite);
    }
    if n.degree() > MAX_DEG || d.degree() > MAX_DEG {
        return Err(unsup("a term of too high a degree"));
    }
    let one = Q::one();
    if rho > &one || rho < &(-one.clone()) {
        return Ok(Sum::Diverges);
    }
    let (p, rem) = n.divmod(d);
    let pf = partial_fractions(&rem, d)?;
    // a pole on an index of the sum: a term is undefined (sympy's zoo)
    if pf.iter().any(|(r, _)| r.is_integer() && r >= &qi(k)) {
        return Ok(Sum::Diverges);
    }
    let half = Q::new(BigInt::one(), BigInt::from(2));
    if *rho == one {
        if !p.is_zero() {
            return Ok(Sum::Diverges);
        }
        let s1: Q = pf.iter().map(|(_, cs)| cs[0].clone()).fold(Q::zero(), |a, b| a + b);
        if !s1.is_zero() {
            return Ok(Sum::Diverges);
        }
        // the sum runs over x - r = a, a + 1, ...; the terms with a <= 0 (a not an integer) are added as they are
        let mut poles: Vec<(Q, &Vec<Q>)> = vec![];
        for (r, cs) in &pf {
            let mut a = qi(k) - r;
            if !a.is_positive() {
                let m = small_int(&(-a.floor()), "a pole")?;
                for _ in 0..=m {
                    if a.is_positive() {
                        break;
                    }
                    for (j1, c) in cs.iter().enumerate() {
                        v.q += c / qpow(&a, j1 as i64 + 1)?;
                    }
                    a += Q::one();
                }
            }
            small_int(&a.floor(), "a pole")?;
            poles.push((a, cs));
        }
        // simple poles: -sum c_i psi(a_i); within a class of a mod 1 the digammas differ by finite sums, so the class
        // gives rationals when its coefficients cancel; classes that do not cancel separately are sympy's nan
        let mut classes: Vec<(Q, Q, Q)> = vec![];        // (a mod 1, smallest a, sum of c)
        for (a, cs) in &poles {
            let fr = a - a.floor();
            match classes.iter_mut().find(|(f, _, _)| *f == fr) {
                Some(cl) => {
                    if *a < cl.1 {
                        cl.1 = a.clone();
                    }
                    cl.2 += &cs[0];
                }
                None => classes.push((fr, a.clone(), cs[0].clone())),
            }
        }
        if classes.iter().any(|(_, _, s)| !s.is_zero()) {
            return Ok(Sum::Nan);
        }
        for (a, cs) in &poles {
            let c = &cs[0];
            if !c.is_zero() {
                let base = &classes.iter().find(|(f, _, _)| *f == a - a.floor()).unwrap().1;
                let steps = small_int(&(a - base), "a pole")?;
                // psi(a) = psi(base) + sum_{t<steps} 1/(base + t); psi(base) cancels in the class
                for t in 0..steps {
                    v.q -= c / (base + qi(t));
                    acc_ok(&v.q)?;
                }
            }
            for (j1, c) in cs.iter().enumerate().skip(1) {
                let j = j1 + 1;
                if c.is_zero() {
                    continue;
                }
                if a.is_integer() {
                    v.add_zeta(j, c)?;
                    for i in 1..small_int(a, "a pole")? {
                        v.q -= c / qpow(&qi(i), j as i64)?;
                        acc_ok(&v.q)?;
                    }
                } else if (a - &half).is_integer() {
                    // zeta(j, h + 1/2) = (2^j - 1) zeta(j) - sum_{i=0}^{h-1} (i + 1/2)^(-j)
                    let f = Q::from_integer(BigInt::from(2).pow(j as u32) - BigInt::one());
                    v.add_zeta(j, &(c * f))?;
                    for i in 0..small_int(&(a - &half), "a pole")? {
                        v.q -= c / qpow(&(qi(i) + &half), j as i64)?;
                        acc_ok(&v.q)?;
                    }
                } else {
                    v.q += c * fq(hurwitz(j as f64, qf(a)))?;
                }
            }
        }
        return Ok(Sum::Finite);
    }
    if *rho == -one.clone() {
        if !p.is_zero() {
            return Ok(Sum::Diverges);                    // oscillates: sympy's unevaluated Sum, "diverges"
        }
        for (r, cs) in &pf {
            if r.is_integer() {
                let ri = small_int(r, "a pole")?;
                let a = k - ri;                          // >= 1: a pole on an index was refused above
                // (-1)^x = (-1)^r (-1)^m, m = x - r = a, a + 1, ...; sum_{m>=a} (-1)^m/m^j = -eta(j) - sum_{m<a}
                let sr = if ri.rem_euclid(2) == 0 { Q::one() } else { -Q::one() };
                for (j1, c) in cs.iter().enumerate() {
                    let j = j1 + 1;
                    if c.is_zero() {
                        continue;
                    }
                    let c = &sr * c;
                    if j == 1 {
                        v.add_log(qi(2), -c.clone());
                    } else {
                        let f = Q::one() - qpow(&qi(2), 1 - j as i64)?;
                        v.add_zeta(j, &(-(&c * f)))?;
                    }
                    for m in 1..a {
                        let t = qpow(&qi(m), j as i64)?;
                        let sm = if m % 2 == 0 { Q::one() } else { -Q::one() };
                        v.q -= &c * sm / t;
                        acc_ok(&v.q)?;
                    }
                }
            } else {
                // sum_{x>=k} (-1)^x c/(x - r)^j: the terms with x - r <= 0 as they are, then from x0 with
                // a = x0 - r > 0, (-1)^x0 sum_{m>=0} (-1)^m/(m + a)^j = (-1)^x0 2^-j (zeta(j, a/2) - zeta(j, (a+1)/2)),
                // for j = 1 (psi((a+1)/2) - psi(a/2))/2, numerically
                let mut x0 = k;
                let mut a = qi(k) - r;
                let steps = small_int(&(-a.floor()).max(Q::zero()), "a pole")?;
                for _ in 0..=steps {
                    if a.is_positive() {
                        break;
                    }
                    let sx = if x0.rem_euclid(2) == 0 { Q::one() } else { -Q::one() };
                    for (j1, c) in cs.iter().enumerate() {
                        v.q += &sx * c / qpow(&a, j1 as i64 + 1)?;
                        acc_ok(&v.q)?;
                    }
                    a += Q::one();
                    x0 += 1;
                }
                let sx = if x0.rem_euclid(2) == 0 { 1.0 } else { -1.0 };
                let af = qf(&a);
                for (j1, c) in cs.iter().enumerate() {
                    let j = j1 + 1;
                    if c.is_zero() {
                        continue;
                    }
                    let s = if j == 1 {
                        (digamma((af + 1.0) / 2.0) - digamma(af / 2.0)) / 2.0
                    } else {
                        2f64.powi(-(j as i32)) * (hurwitz(j as f64, af / 2.0) - hurwitz(j as f64, (af + 1.0) / 2.0))
                    };
                    v.q += c * fq(sx * s)?;
                }
            }
        }
        return Ok(Sum::Finite);
    }
    // 0 < |rho| < 1
    let omr = &one - rho;
    for (r, cs) in &pf {
        if cs.iter().skip(1).any(|c| !c.is_zero()) {
            return Err(unsup("a geometric term over a squared factor (a polylogarithm)"));
        }
        let ri = small_int(r, "a pole that is not an integer, under a geometric term")?;
        let a = k - ri;                                  // >= 1: a pole on an index was refused above
        // sum_{x>=k} rho^x c/(x - r) = c rho^r (-log(1 - rho) - sum_{m=1}^{a-1} rho^m/m)
        let c = &cs[0] * qpow(rho, ri)?;
        v.add_log(omr.clone(), -c.clone());
        let mut rm = Q::one();
        for m in 1..a {
            rm *= rho;
            if bits(&rm) > MAX_POW_BITS {
                return Err(unsup("a power too large to hold exactly"));
            }
            v.q -= &c * &rm / qi(m);
            acc_ok(&v.q)?;
        }
    }
    if !p.is_zero() {
        // rho^k sum_{m>=0} P(m + k) rho^m, P(m + k) in the binomial basis (forward differences at 0)
        let pk = shift(&p, &qi(k));
        let deg = pk.degree().max(0) as usize;
        let mut vals: Vec<Q> = (0..=deg).map(|m| pk.eval_q(&qi(m as i64))).collect();
        let mut s = Q::zero();
        let mut rj = Q::one();
        let mut den = omr.clone();
        for _ in 0..=deg {
            s += &vals[0] * &rj / &den;
            vals = vals.windows(2).map(|w| &w[1] - &w[0]).collect();
            rj *= rho;
            den *= &omr;
            if vals.is_empty() {
                break;
            }
        }
        v.q += qpow(rho, k)? * s;
    }
    Ok(Sum::Finite)
}

/// c * x^(-p) for a rational p that is not an integer: Some((c, p))
fn power_term(e: &Ex, x: &str) -> Option<(Q, Q)> {
    let fs: Vec<Ex> = match e {
        Ex::Mul(v) => (**v).clone(),
        other => vec![other.clone()],
    };
    let (mut c, mut p) = (Q::one(), None);
    for f in fs {
        match &f {
            Ex::Num(q) => c *= q,
            Ex::Pow(b, k) => match (&**b, &**k) {
                (Ex::Sym(s), Ex::Num(q)) if &**s == x && !q.is_integer() && p.is_none() => p = Some(-q.clone()),
                _ => return None,
            },
            _ => return None,
        }
    }
    p.map(|p| (c, p))
}

/// sympy.summation(e, (x, k, oo)) for the series above; zoo when the series diverges, nan where sympy gives nan
pub fn summation(e: &Ex, x: &Rc<str>, k: i64) -> Result<Ex, PyErr> {
    if k.abs() > MAX_SHIFT {
        return Err(unsup("a starting index too large"));
    }
    let mut v = Value::default();
    let mut out = Sum::Finite;
    for (rho, part) in &groups(e, x)? {
        let s = match crate::pysym::ratfun(part, x) {
            Some((n, d)) => sum_group(rho, &n, &d, k, &mut v)?,
            None => match power_term(part, x) {
                Some((c, p)) if *rho == Q::one() => {
                    if p <= Q::one() {
                        Sum::Diverges
                    } else if k < 1 {
                        Sum::Diverges                    // 0^(-p) is undefined
                    } else {
                        let pf = qf(&p);
                        let z = crate::pymp::call("zeta", &[Py::Float(pf)], &[])?.f()?;
                        let mut s = z;
                        for i in 1..k {
                            s -= (i as f64).powf(-pf);
                        }
                        v.q += &c * fq(s)?;
                        Sum::Finite
                    }
                }
                _ => return Err(unsup("a term that is not a rational function times a geometric factor")),
            },
        };
        if s == Sum::Nan {
            return Ok(Ex::Nan);
        }
        if s == Sum::Diverges {
            out = Sum::Diverges;
        }
    }
    if out == Sum::Diverges {
        return Ok(Ex::Zoo);
    }
    Ok(v.ex())
}

// ------------------------------------------------------------------------------------------- the numerical check
/// Neumaier-compensated sum of f(k), f(k + 1), ... f(k + n - 1)
fn direct(f: &dyn Fn(f64) -> Result<f64, PyErr>, k: f64, n: usize) -> Result<f64, PyErr> {
    let (mut s, mut comp) = (0.0f64, 0.0f64);
    for j in 0..n {
        let term = f(k + j as f64)?;
        let y = s + term;
        comp += if s.abs() >= term.abs() { (s - y) + term } else { (term - y) + s };
        s = y;
    }
    Ok(s + comp)
}

/// sum_{i>=0} (-1)^i b(i) by Cohen, Rodriguez Villegas and Zagier (algorithm 1, 30 terms)
fn cvz(b: &dyn Fn(usize) -> Result<f64, PyErr>) -> Result<f64, PyErr> {
    let n = 30usize;
    let mut d = (3.0 + 8f64.sqrt()).powi(n as i32);
    d = (d + 1.0 / d) / 2.0;
    let (mut bb, mut c, mut s) = (-1.0f64, -d, 0.0f64);
    for i in 0..n {
        c = bb - c;
        s += c * b(i)?;
        bb = (i as f64 + n as f64) * (i as f64 - n as f64) * bb / ((i as f64 + 0.5) * (i as f64 + 1.0));
    }
    Ok(s / d)
}

/// sum_{x>=k} e(x) by Euler-Maclaurin from N = k + 30: the first 30 terms, the tail integral by quadrature, f(N)/2
/// and five Bernoulli corrections from derivatives by automatic differentiation
fn euler_maclaurin(e: &Ex, f: &dyn Fn(f64) -> Result<f64, PyErr>, k: f64) -> Result<f64, PyErr> {
    let big_n = k + 30.0;
    let head = direct(f, k, 30)?;
    let fc = crate::pysym::compile(e, &[Rc::from("n")], false)?;
    let ((tail, im), err) = crate::pymp::integrate(&fc, big_n, f64::INFINITY)?;
    if im != 0.0 || !tail.is_finite() || !(err <= 1e-12 * tail.abs().max(1e-3)) {
        return Err(PyErr::Value("the tail integral did not converge".into()));
    }
    let mut corr = f(big_n)? / 2.0;
    for (i1, c) in em_coefficients().into_iter().enumerate().take(5) {
        let d = crate::pysym::derivative_ad(e, "n", big_n, 2 * i1 + 1)
            .map_err(|_| PyErr::Value("the expression is not defined there".into()))?;
        corr -= c * d;
    }
    Ok(head + tail + corr)
}

fn eval_at(e: &Ex, n: f64) -> Result<f64, PyErr> {
    crate::pysym::eval_float(e, &|s: &str| (s == "n").then_some(n))
        .map_err(|_| PyErr::Value("the expression is not defined there".into()))
        .and_then(|v| if v.is_finite() { Ok(v) } else { Err(PyErr::Value("not finite".into())) })
}

/// The package's own choice of method on one term: direct summation when the terms decay geometrically, an
/// acceleration when they alternate, Euler-Maclaurin otherwise
fn nsum_term(e: &Ex, k: f64) -> Result<f64, PyErr> {
    let f = |n: f64| eval_at(e, n);
    let t: Vec<f64> = (0..4).map(|j| f(k + j as f64)).collect::<Result<_, _>>()?;
    let far = (f(k + 60.0)?.abs(), f(k + 61.0)?.abs());
    if far.0 != 0.0 && far.1 / far.0 < 0.95 && t.iter().all(|v| *v != 0.0) {
        let (mut s, mut comp) = (0.0f64, 0.0f64);
        for j in 0..20000 {
            let term = f(k + j as f64)?;
            let y = s + term;
            comp += if s.abs() >= term.abs() { (s - y) + term } else { (term - y) + s };
            s = y;
            if j > 70 && term.abs() < 1e-18 * (s + comp).abs().max(1e-30) {
                return Ok(s + comp);
            }
        }
    }
    if (0..3).all(|j| t[j] * t[j + 1] < 0.0) {
        let v = cvz(&|i| Ok(f(k + i as f64)?.abs()))?;
        return Ok(if t[0] < 0.0 { -v } else { v });
    }
    euler_maclaurin(e, &f, k)
}

/// _nsum(a, k): the series summed numerically, by another method than the closed form. The addends are grouped by
/// their ratio as for the closed form (an algebraic step only); then an alternating group (ratio -1) by the
/// Cohen-Villegas-Zagier acceleration of its rational part, a group with ratio 1 by Euler-Maclaurin, a geometric
/// group directly; a term that does not split so is summed as the package chooses (direct summation when the terms
/// decay geometrically, acceleration when they alternate, Euler-Maclaurin otherwise). Double precision: the check
/// compares to 1e-10.
pub fn nsum(a: &Py, k: &Py) -> R {
    let names = Py::tuple(vec![Py::str("n")]);
    let e = match crate::pysym::parse_expr(a, Some(&names))? {
        Py::Sym(e) => (*e).clone(),
        _ => return Err(PyErr::Value("the term must be an expression".into())),
    };
    let k = k.f()?;
    if !(k.is_finite() && k.abs() <= MAX_SHIFT as f64) {
        return Err(PyErr::Value("the starting index is out of range".into()));
    }
    let k = k.trunc();
    let x: Rc<str> = Rc::from("n");
    let gs = match groups(&e, &x) {
        Ok(g) if g.len() > 1 || g.iter().any(|(r, _)| *r == -Q::one()) => g,
        _ => return Ok(Py::Float(nsum_term(&e, k)?)),
    };
    let mut total = 0.0;
    for (rho, part) in &gs {
        let r = qf(rho);
        total += if *rho == -Q::one() {
            let s0 = if (k as i64).rem_euclid(2) == 0 { 1.0 } else { -1.0 };
            s0 * cvz(&|i| eval_at(part, k + i as f64))?
        } else if *rho == Q::one() {
            euler_maclaurin(part, &|n| eval_at(part, n), k)?
        } else if r.abs() < 1.0 {
            let mut s = 0.0;
            let mut j = 0usize;
            loop {
                let term = r.powf(k + j as f64) * eval_at(part, k + j as f64)?;
                s += term;
                if (j > 70 && term.abs() < 1e-18 * s.abs().max(1e-30)) || j > 20000 {
                    break;
                }
                j += 1;
            }
            s
        } else {
            return Err(PyErr::Value("the series diverges".into()));
        };
    }
    if !total.is_finite() {
        return Err(PyErr::Value("the sum is not finite".into()));
    }
    Ok(Py::Float(total))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn val(text: &str, k: i64) -> Option<f64> {
        let e = crate::symparse::parse(text, &["k"]).unwrap();
        match summation(&e, &Rc::from("k"), k) {
            Ok(Ex::Zoo) => Some(f64::INFINITY),
            Ok(Ex::Nan) => Some(f64::NAN),
            Ok(s) => Some(crate::pysym::eval_mp(&s, &|_| None).unwrap()),
            Err(_) => None,
        }
    }

    #[test]
    fn closed_forms() {
        let pi = std::f64::consts::PI;
        let close = |a: f64, b: f64| (a - b).abs() <= 1e-14 * b.abs().max(1.0);
        assert!(close(val("1/k^2", 1).unwrap(), pi * pi / 6.0));
        assert!(close(val("1/k^4", 1).unwrap(), pi.powi(4) / 90.0));
        assert!(close(val("1/(k*(k + 1))", 1).unwrap(), 1.0));
        assert!(close(val("-2/(k*(k + 2))", 1).unwrap(), -1.5));
        assert!(close(val("1/((2*k - 1)*(2*k + 1))", 1).unwrap(), 0.5));
        assert!(close(val("1/(4*k^2 - 1)", 0).unwrap(), -0.5));
        assert!(close(val("(-1)^(k + 1)/k", 1).unwrap(), 2f64.ln()));
        assert!(close(val("(-1)^k/(2*k + 1)", 0).unwrap(), pi / 4.0));
        assert!(close(val("(1/2)^k", 0).unwrap(), 2.0));
        assert!(close(val("2*(1/2)^k", 2).unwrap(), 1.0));
        assert!(close(val("k/4^k", 1).unwrap(), 4.0 / 9.0));
        assert!(close(val("k^2/2^k", 1).unwrap(), 6.0));
        assert!(close(val("1/2^k + 1/k^2", 1).unwrap(), 1.0 + pi * pi / 6.0));
        assert!(close(val("1/k^3", 1).unwrap(), 1.2020569031595942));
        assert!(close(val("(1/2)^k/k", 1).unwrap(), 2f64.ln()));
        assert!(close(val("1/(k - 1/2)^2", 0).unwrap(), 8.934802200544679));
        assert!(close(val("1/(k + 1/3)^2", 1).unwrap(), 1.095597125427094));
        assert_eq!(val("1/k", 1), Some(f64::INFINITY));
        assert_eq!(val("2^k", 0), Some(f64::INFINITY));
        assert_eq!(val("k", 1), Some(f64::INFINITY));
        assert_eq!(val("(-1)^k", 0), Some(f64::INFINITY));
        assert_eq!(val("1/(k^2 - 1)", 0), Some(f64::INFINITY));
        assert!(val("1/(k*(2*k + 1))", 1).unwrap().is_nan());     // sympy's nan
        assert_eq!(val("1/(k^2 + 1)", 0), None);                  // complex roots: not summed here
    }

    #[test]
    fn numerical_check_agrees() {
        for (t, k) in [("1/n^2", 1), ("1/(n*(n + 2))", 1), ("(-1)^(n + 1)/n", 1), ("n^2/3^n", 1), ("1/5^n + 1/n^2", 1),
                       ("1/n^2 + (-1)^n/n", 2), ("(-1)^n/(2*n + 1)", 0)] {
            let exact = val(&t.replace('n', "k"), k).unwrap();
            let num = nsum(&Py::str(t), &Py::Int(k)).unwrap().f().unwrap();
            assert!((exact - num).abs() < 1e-12 * exact.abs().max(1.0), "{t}: {exact} vs {num}");
        }
    }
}
