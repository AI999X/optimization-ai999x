//! Python's fractions.Fraction (CPython 3.13, Lib/fractions.py) on unbounded integers (num-bigint): exact
//! arithmetic for the procedures that reduce matrices or sum series exactly. Conversions follow Python: Fraction(float)
//! is the float's exact binary value, Fraction("0.1") the decimal written, float(q) the correctly rounded quotient.

use crate::py::{value_error, PyErr};
use num_bigint::BigInt;
use num_integer::Integer;
use num_rational::BigRational;
use num_traits::{One, Signed, ToPrimitive, Zero};

pub type Q = BigRational;

pub fn int(n: i64) -> Q {
    Q::from_integer(BigInt::from(n))
}

/// Fraction(n, d) from integers: ZeroDivisionError for d = 0 (normalized, denominator positive).
pub fn ratio(n: BigInt, d: BigInt) -> Result<Q, PyErr> {
    if d.is_zero() {
        return Err(PyErr::ZeroDivision);
    }
    Ok(Q::new(n, d))
}

/// Fraction(x) for a float: its exact value. ValueError for nan, OverflowError for an infinity (as Python).
pub fn from_f64(x: f64) -> Result<Q, PyErr> {
    if x.is_nan() {
        return Err(value_error("cannot convert NaN to integer ratio"));
    }
    if x.is_infinite() {
        return Err(PyErr::Overflow);
    }
    Q::from_float(x).ok_or(PyErr::Overflow)
}

/// Fraction("3/4"), Fraction("-1.25e-3"), Fraction(" 7 "): Python's literal forms (no underscores).
pub fn from_str(s: &str) -> Result<Q, PyErr> {
    let bad = || value_error(format!("Invalid literal for Fraction: {}", crate::num::py_repr_str(s)));
    let t = s.trim();
    let (neg, body) = match t.as_bytes().first() {
        Some(b'-') => (true, &t[1..]),
        Some(b'+') => (false, &t[1..]),
        _ => (false, t),
    };
    let digits = |x: &str| !x.is_empty() && x.bytes().all(|c| c.is_ascii_digit());
    let q = if let Some((n, d)) = body.split_once('/') {
        if !digits(n) || !digits(d) {
            return Err(bad());
        }
        ratio(n.parse::<BigInt>().map_err(|_| bad())?, d.parse::<BigInt>().map_err(|_| bad())?)?
    } else {
        let (mant, exp) = match body.find(['e', 'E']) {
            Some(i) => (&body[..i], Some(&body[i + 1..])),
            None => (body, None),
        };
        let (ip, fp) = match mant.split_once('.') {
            Some((a, b)) => (a, b),
            None => (mant, ""),
        };
        if (ip.is_empty() && fp.is_empty()) || !(ip.is_empty() || digits(ip)) || !(fp.is_empty() || digits(fp)) {
            return Err(bad());
        }
        let mut e: i64 = 0;
        if let Some(x) = exp {
            let (sign, num) = match x.as_bytes().first() {
                Some(b'-') => (-1, &x[1..]),
                Some(b'+') => (1, &x[1..]),
                _ => (1, x),
            };
            if !digits(num) {
                return Err(bad());
            }
            e = sign * num.parse::<i64>().map_err(|_| bad())?;
        }
        let n: BigInt = format!("{ip}{fp}").trim_start_matches('0').parse().unwrap_or_else(|_| BigInt::zero());
        let e = e - fp.len() as i64;
        if e.unsigned_abs() > 100_000 {
            return Err(PyErr::Unsupported("a decimal exponent beyond 100000".into()));
        }
        let ten = BigInt::from(10);
        if e >= 0 {
            Q::from_integer(n * num_traits::pow(ten, e as usize))
        } else {
            Q::new(n, num_traits::pow(ten, (-e) as usize))
        }
    };
    Ok(if neg { -q } else { q })
}

/// float(q): the correctly rounded quotient (round half to even), as Python's int / int.
pub fn to_f64(q: &Q) -> f64 {
    let (n, d) = (q.numer(), q.denom());
    if n.is_zero() {
        return 0.0;
    }
    let neg = n.is_negative();
    let a = n.abs();
    // a / d scaled by 2^shift so the integer quotient has 55 or 56 bits
    let shift: i64 = 55 + d.bits() as i64 - a.bits() as i64;
    let (num, den) = if shift >= 0 { (&a << shift as usize, d.clone()) } else { (a.clone(), d << (-shift) as usize) };
    let (qt, rem) = num.div_rem(&den);
    let sticky = !rem.is_zero();
    // the value is qt * 2^-shift; keep 53 bits, more when the result would be subnormal
    let bits = qt.bits() as i64;                                 // 55 or 56
    let exp = bits - 1 - shift;                                  // floor(log2(value))
    // the bits a double keeps: 53, fewer for a subnormal result (its last bit is worth 2^-1074)
    let keep = if exp < -1022 { 53 - (-1022 - exp) } else { 53 };
    let r = if keep < 0 {
        0.0                                                      // below half the smallest subnormal
    } else if keep == 0 {
        // in [2^-1075, 2^-1074): exactly half the smallest subnormal rounds to even (0), anything above to it
        let exact_half = !sticky && qt == (BigInt::one() << (bits - 1) as usize);
        if exact_half { 0.0 } else { f64::from_bits(1) }
    } else {
        let drop = (bits - keep) as usize;                       // at least 2
        let m = &qt >> drop;
        let rest = &qt - (&m << drop);
        let half = BigInt::one() << (drop - 1);
        let up = rest > half || (rest == half && (sticky || m.is_odd()));
        let m = if up { m + 1 } else { m };                      // may carry to 2^keep: still exact below
        ldexp(m.to_u64().unwrap_or(0) as f64, exp - keep + 1)
    };
    if neg { -r } else { r }
}

/// x * 2^e in exact steps (each factor a normal power of two), overflowing to infinity as a double does.
fn ldexp(mut x: f64, mut e: i64) -> f64 {
    while e > 1000 {
        x *= 2f64.powi(1000);
        e -= 1000;
    }
    while e < -1000 {
        x *= 2f64.powi(-1000);
        e += 1000;
    }
    x * 2f64.powi(e as i32)
}

/// q ** n for an integer n: ZeroDivisionError for 0 to a negative power.
pub fn powi(q: &Q, n: i64) -> Result<Q, PyErr> {
    if n.unsigned_abs() > 100_000 {
        return Err(PyErr::Unsupported("an exact power beyond 100000".into()));
    }
    if n >= 0 {
        return Ok(num_traits::pow(q.clone(), n as usize));
    }
    if q.is_zero() {
        return Err(PyErr::ZeroDivision);
    }
    Ok(num_traits::pow(q.recip(), (-n) as usize))
}

/// floor(q) as an integer.
pub fn floor(q: &Q) -> BigInt {
    q.numer().div_floor(q.denom())
}

/// round(q): to the nearest integer, ties to even (Fraction.__round__ without ndigits).
pub fn round_even(q: &Q) -> BigInt {
    let f = floor(q);
    let rest = q - Q::from_integer(f.clone());
    let half = Q::new(BigInt::one(), BigInt::from(2));
    if rest > half || (rest == half && f.is_odd()) {
        f + 1
    } else {
        f
    }
}

/// round(q, nd): Fraction(round(q * 10^nd), 10^nd) (or the mirror for a negative nd).
pub fn round_n(q: &Q, nd: i64) -> Result<Q, PyErr> {
    if nd.unsigned_abs() > 10_000 {
        return Err(PyErr::Unsupported("round to beyond 10000 digits".into()));
    }
    let shift = Q::from_integer(num_traits::pow(BigInt::from(10), nd.unsigned_abs() as usize));
    Ok(if nd >= 0 {
        Q::new(round_even(&(q * &shift)), shift.to_integer())
    } else {
        Q::from_integer(round_even(&(q / &shift))) * shift
    })
}

/// q.limit_denominator(max_den) (Lib/fractions.py, 3.13).
pub fn limit_denominator(q: &Q, max_den: &BigInt) -> Result<Q, PyErr> {
    if max_den < &BigInt::one() {
        return Err(value_error("max_denominator should be at least 1"));
    }
    if q.denom() <= max_den {
        return Ok(q.clone());
    }
    let (mut p0, mut q0, mut p1, mut q1) = (BigInt::zero(), BigInt::one(), BigInt::one(), BigInt::zero());
    let (mut n, mut d) = (q.numer().clone(), q.denom().clone());
    loop {
        let a = n.div_floor(&d);
        let q2 = &q0 + &a * &q1;
        if &q2 > max_den {
            break;
        }
        let p2 = &p0 + &a * &p1;
        p0 = std::mem::replace(&mut p1, p2);
        q0 = std::mem::replace(&mut q1, q2);
        let nd = &n - &a * &d;
        n = std::mem::replace(&mut d, nd);
    }
    let k = (max_den - &q0).div_floor(&q1);
    if BigInt::from(2) * &d * (&q0 + &k * &q1) <= *q.denom() {
        Ok(Q::new(p1, q1))
    } else {
        Ok(Q::new(&p0 + &k * &p1, &q0 + &k * &q1))
    }
}

/// str(q): "3/4", or "3" for a whole number.
pub fn to_str(q: &Q) -> String {
    if q.denom().is_one() { q.numer().to_string() } else { format!("{}/{}", q.numer(), q.denom()) }
}

/// repr(q): "Fraction(3, 4)".
pub fn repr(q: &Q) -> String {
    format!("Fraction({}, {})", q.numer(), q.denom())
}

/// format(q, ".Nf"): exact decimal rounding, ties to even (Fraction.__format__, Python 3.12+); None for another spec.
pub fn format_fixed(q: &Q, spec: &str) -> Option<String> {
    let p: usize = spec.strip_prefix('.')?.strip_suffix('f')?.parse().ok()?;
    let scaled = round_even(&(q * Q::from_integer(num_traits::pow(BigInt::from(10), p))));
    let neg = scaled.is_negative() || (scaled.is_zero() && q.is_negative());
    let digits = scaled.abs().to_string();
    let digits = if digits.len() <= p { format!("{}{}", "0".repeat(p + 1 - digits.len()), digits) } else { digits };
    let (ip, fp) = digits.split_at(digits.len() - p);
    let body = if p == 0 { ip.to_string() } else { format!("{ip}.{fp}") };
    Some(if neg { format!("-{body}") } else { body })
}

/// A big integer as Python's int here: within 64 bits, else unsupported (this runtime's ints are 64-bit).
pub fn small(n: &BigInt) -> Result<i64, PyErr> {
    n.to_i64().ok_or_else(|| PyErr::Unsupported("an integer beyond 64 bits".into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_python_converts() {
        // float(Fraction(1, 3)) == 0.3333333333333333; Fraction(0.1) is 3602879701896397/36028797018963968
        assert_eq!(to_f64(&Q::new(BigInt::from(1), BigInt::from(3))), 1.0 / 3.0);
        assert_eq!(from_f64(0.1).unwrap(), Q::new(BigInt::from(3602879701896397u64), BigInt::from(36028797018963968u64)));
        assert_eq!(from_str("1e-05").unwrap(), Q::new(BigInt::from(1), BigInt::from(100000)));
        assert_eq!(from_str("-2.50").unwrap(), Q::new(BigInt::from(-5), BigInt::from(2)));
        assert_eq!(from_str("3/6").unwrap(), Q::new(BigInt::from(1), BigInt::from(2)));
        assert!(from_str("abc").is_err());
        for x in [0.1, 2.0 / 3.0, 1e-300, 5e-324, 1.7976931348623157e308, -123.456, 6.02214076e23] {
            assert_eq!(to_f64(&from_f64(x).unwrap()), x);
        }
        // ties to even: 2^53 + 1 is halfway between two doubles
        let t = Q::from_integer((BigInt::one() << 53usize) + 1);
        assert_eq!(to_f64(&t), 9007199254740992.0);
        assert_eq!(to_f64(&from_str("0.1").unwrap()), 0.1);
    }

    #[test]
    fn as_python_rounds() {
        let pi = from_f64(std::f64::consts::PI).unwrap();
        assert_eq!(limit_denominator(&pi, &BigInt::from(1000)).unwrap(), Q::new(BigInt::from(355), BigInt::from(113)));
        assert_eq!(round_even(&Q::new(BigInt::from(5), BigInt::from(2))), BigInt::from(2));
        assert_eq!(round_even(&Q::new(BigInt::from(-7), BigInt::from(2))), BigInt::from(-4));
        assert_eq!(format_fixed(&Q::new(BigInt::from(1), BigInt::from(8)), ".2f").unwrap(), "0.12");
        assert_eq!(to_str(&Q::new(BigInt::from(-6), BigInt::from(4))), "-3/2");
    }
}
