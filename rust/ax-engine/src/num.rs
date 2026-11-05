//! Numbers as the Python reference writes them: exact fractions for unit exponents, complex values for evaluating
//! solved forms, and Python's own text formats (`{:g}`, `{:.4g}`, `repr`) so messages are identical.

#[allow(unused_imports)]
use crate::lm::{Lm, LmF64};
use std::cmp::Ordering;

/// An exact fraction (unit exponents such as m^(1/3)).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Rat {
    pub num: i64,
    pub den: i64,
}

fn gcd(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a.max(1)
}

impl Rat {
    pub fn new(num: i64, den: i64) -> Rat {
        let (num, den) = if den < 0 { (-num, -den) } else { (num, den) };
        let g = gcd(num, den);
        Rat { num: num / g, den: den / g }
    }
    pub fn int(n: i64) -> Rat {
        Rat::new(n, 1)
    }
    pub fn zero() -> Rat {
        Rat::new(0, 1)
    }
    pub fn is_zero(&self) -> bool {
        self.num == 0
    }
    pub fn add(self, o: Rat) -> Rat {
        Rat::new(self.num * o.den + o.num * self.den, self.den * o.den)
    }
    pub fn sub(self, o: Rat) -> Rat {
        Rat::new(self.num * o.den - o.num * self.den, self.den * o.den)
    }
    pub fn mul(self, o: Rat) -> Rat {
        Rat::new(self.num * o.num, self.den * o.den)
    }
    pub fn to_f64(self) -> f64 {
        self.num as f64 / self.den as f64
    }
    /// "1", "-2", "1/3" (Python's str(Fraction)); also accepts "0.5" (a decimal written exactly).
    pub fn parse(s: &str) -> Option<Rat> {
        let s = s.trim();
        if let Some((a, b)) = s.split_once('/') {
            let (a, b) = (a.trim().parse::<i64>().ok()?, b.trim().parse::<i64>().ok()?);
            if b == 0 {
                return None;
            }
            return Some(Rat::new(a, b));
        }
        if let Some((ip, fp)) = s.split_once('.') {
            let neg = ip.starts_with('-');
            let digits = format!("{}{}", ip.trim_start_matches(['-', '+']), fp);
            let n: i64 = digits.parse().ok()?;
            let d = 10i64.checked_pow(fp.len() as u32)?;
            return Some(Rat::new(if neg { -n } else { n }, d));
        }
        s.parse::<i64>().ok().map(Rat::int)
    }
}

/// A complex number in double precision (the Python reference evaluates at 17 digits and keeps real values).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct C {
    pub re: f64,
    pub im: f64,
}

impl C {
    pub fn r(x: f64) -> C {
        C { re: x, im: 0.0 }
    }
    pub fn new(re: f64, im: f64) -> C {
        C { re, im }
    }
    pub fn add(self, o: C) -> C {
        C::new(self.re + o.re, self.im + o.im)
    }
    pub fn mul(self, o: C) -> C {
        if self.im == 0.0 && o.im == 0.0 {
            return C::r(self.re * o.re);
        }
        C::new(self.re * o.re - self.im * o.im, self.re * o.im + self.im * o.re)
    }
    pub fn abs(self) -> f64 {
        self.re.hypot_lm(self.im)
    }
    // (the Lm trait below lets one spelling, z.exp_lm(), serve a complex value and a float alike)
    pub fn is_finite(self) -> bool {
        self.re.is_finite() && self.im.is_finite()
    }
    pub fn exp(self) -> C {
        let e = self.re.exp_lm();
        if self.im == 0.0 {
            return C::r(e);
        }
        C::new(e * self.im.cos_lm(), e * self.im.sin_lm())
    }
    pub fn ln(self) -> C {
        if self.im == 0.0 && self.re > 0.0 {
            return C::r(self.re.ln_lm());
        }
        C::new(self.abs().ln_lm(), self.im.atan2_lm(self.re))
    }
    pub fn sin(self) -> C {
        if self.im == 0.0 {
            return C::r(self.re.sin_lm());
        }
        C::new(self.re.sin_lm() * self.im.cosh_lm(), self.re.cos_lm() * self.im.sinh_lm())
    }
    pub fn cos(self) -> C {
        if self.im == 0.0 {
            return C::r(self.re.cos_lm());
        }
        C::new(self.re.cos_lm() * self.im.cosh_lm(), -self.re.sin_lm() * self.im.sinh_lm())
    }
    /// Principal branch, as sympy: asin(z) = -i ln(iz + sqrt(1 - z^2)).
    pub fn asin(self) -> C {
        if self.im == 0.0 && self.re.abs() <= 1.0 {
            return C::r(self.re.asin_lm());
        }
        let iz = C::new(-self.im, self.re);
        let one_minus = C::new(1.0, 0.0).add(self.mul(self).mul(C::r(-1.0)));
        let w = iz.add(one_minus.powc(C::r(0.5))).ln_lm();
        C::new(w.im, -w.re)
    }
    pub fn acos(self) -> C {
        if self.im == 0.0 && self.re.abs() <= 1.0 {
            return C::r(self.re.acos_lm());
        }
        let a = self.asin();
        C::new(std::f64::consts::FRAC_PI_2 - a.re, -a.im)
    }
    /// The principal power, real arithmetic wherever the result is real (as an exact evaluation gives).
    pub fn powc(self, e: C) -> C {
        if self.im == 0.0 && e.im == 0.0 {
            let (b, x) = (self.re, e.re);
            if x == x.trunc() || b > 0.0 {
                return C::r(b.powf_lm(x));
            }
            if b == 0.0 {
                return C::r(if x > 0.0 { 0.0 } else { f64::INFINITY });
            }
        }
        if self.re == 0.0 && self.im == 0.0 {
            return if e.re > 0.0 { C::r(0.0) } else { C::r(f64::NAN) };
        }
        e.mul(self.ln_lm()).exp_lm()
    }
}

// ------------------------------------------------------------------------------------------------ Python formats
fn sci_parts(x: f64, digits: usize) -> (String, i32) {
    // x rounded to `digits` significant digits: (mantissa digits without the dot, decimal exponent)
    let s = format!("{:.*e}", digits.saturating_sub(1), x.abs());
    let (m, e) = s.split_once('e').unwrap();
    (m.replace('.', ""), e.parse::<i32>().unwrap())
}

/// Python's format(x, ".{p}g") (and "g" = ".6g").
pub fn py_g(x: f64, p: usize) -> String {
    let p = if p == 0 { 1 } else { p };
    if x.is_nan() {
        return "nan".into();
    }
    if x.is_infinite() {
        return if x > 0.0 { "inf".into() } else { "-inf".into() };
    }
    let sign = if x.is_sign_negative() { "-" } else { "" };
    if x == 0.0 {
        return format!("{sign}0");
    }
    let (digits, exp) = sci_parts(x, p);
    let body = if exp < -4 || exp >= p as i32 {
        let mut m = digits.clone();
        let rest = m.split_off(1);
        let frac = rest.trim_end_matches('0');
        let mant = if frac.is_empty() { m } else { format!("{m}.{frac}") };
        format!("{mant}e{}{:02}", if exp < 0 { '-' } else { '+' }, exp.abs())
    } else if exp >= 0 {
        let (ip, fp) = digits.split_at((exp + 1) as usize);
        let fp = fp.trim_end_matches('0');
        if fp.is_empty() { ip.to_string() } else { format!("{ip}.{fp}") }
    } else {
        let zeros = "0".repeat((-exp - 1) as usize);
        let fp = format!("{zeros}{digits}");
        let fp = fp.trim_end_matches('0');
        format!("0.{fp}")
    };
    format!("{sign}{body}")
}

/// Python's repr(float): the shortest text that reads back as the same double.
pub fn py_repr_f64(x: f64) -> String {
    if x.is_nan() {
        return "nan".into();
    }
    if x.is_infinite() {
        return if x > 0.0 { "inf".into() } else { "-inf".into() };
    }
    if x == 0.0 {
        return if x.is_sign_negative() { "-0.0".into() } else { "0.0".into() };
    }
    let s = format!("{:e}", x.abs()); // shortest round-trip digits
    let (m, e) = s.split_once('e').unwrap();
    let exp: i32 = e.parse().unwrap();
    let digits = m.replace('.', "");
    let sign = if x < 0.0 { "-" } else { "" };
    if (-4..16).contains(&exp) {
        let body = if exp >= 0 {
            let n = digits.len() as i32;
            if n > exp + 1 {
                let (a, b) = digits.split_at((exp + 1) as usize);
                format!("{a}.{b}")
            } else {
                format!("{}{}.0", digits, "0".repeat((exp + 1 - n) as usize))
            }
        } else {
            format!("0.{}{}", "0".repeat((-exp - 1) as usize), digits)
        };
        format!("{sign}{body}")
    } else {
        let mant = if digits.len() > 1 { format!("{}.{}", &digits[..1], &digits[1..]) } else { digits.clone() };
        format!("{sign}{mant}e{}{:02}", if exp < 0 { '-' } else { '+' }, exp.abs())
    }
}

/// Python's repr(str).
pub fn py_repr_str(s: &str) -> String {
    let q = if s.contains('\'') && !s.contains('"') { '"' } else { '\'' };
    let mut out = String::new();
    out.push(q);
    for ch in s.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c == q => {
                out.push('\\');
                out.push(c);
            }
            c if (c as u32) < 0x20 || c as u32 == 0x7f => out.push_str(&format!("\\x{:02x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push(q);
    out
}

pub fn cmp_f64(a: f64, b: f64) -> Ordering {
    a.partial_cmp(&b).unwrap_or(Ordering::Equal)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn python_formats() {
        assert_eq!(py_g(-93.4, 4), "-93.4");
        assert_eq!(py_g(-0.000434294, 4), "-0.0004343");
        assert_eq!(py_g(1e-5, 6), "1e-05");
        assert_eq!(py_g(123456789.0, 6), "1.23457e+08");
        assert_eq!(py_g(100000.0, 6), "100000");
        assert_eq!(py_g(1000000.0, 6), "1e+06");
        assert_eq!(py_g(2.0, 6), "2");
        assert_eq!(py_repr_f64(3.0), "3.0");
        assert_eq!(py_repr_f64(1e-5), "1e-05");
        assert_eq!(py_repr_f64(0.1), "0.1");
        assert_eq!(py_repr_f64(1e16), "1e+16");
        assert_eq!(py_repr_f64(123.456), "123.456");
        assert_eq!(py_repr_str("3 kg"), "'3 kg'");
        assert_eq!(py_repr_str("it's"), "\"it's\"");
    }
}

/// The complex forms behind one spelling (crate::lm): z.exp_lm() is C::exp, whose float parts already use libm.
impl crate::lm::Lm for C {
    fn exp_lm(self) -> C { self.exp() }
    fn ln_lm(self) -> C { self.ln() }
    fn sin_lm(self) -> C { self.sin() }
    fn cos_lm(self) -> C { self.cos() }
}
