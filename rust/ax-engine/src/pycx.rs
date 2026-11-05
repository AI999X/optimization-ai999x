//! Python's complex numbers (CPython 3.13, Objects/complexobject.c and Modules/cmathmodule.c), on (re, im) pairs.
//! The algorithms are CPython's own, so a translated procedure rounds as the package does: division by Smith's
//! method (_Py_c_quot), integer powers up to 100 by repeated squaring (c_powi) and others through polar form
//! (_Py_c_pow), cmath.sqrt and cmath.log as cmathmodule.c computes them. A float or int meeting a complex is first made
//! complex (x + 0j), as Python 3.13 does (3.14 changed mixed arithmetic; the reference packages run 3.13).

#[allow(unused_imports)]
use crate::lm::{Lm, LmF64};
use crate::py::PyErr;

pub type C = (f64, f64);

pub fn add(a: C, b: C) -> C {
    (a.0 + b.0, a.1 + b.1)
}

pub fn sub(a: C, b: C) -> C {
    (a.0 - b.0, a.1 - b.1)
}

pub fn mul(a: C, b: C) -> C {
    (a.0 * b.0 - a.1 * b.1, a.0 * b.1 + a.1 * b.0)
}

pub fn neg(a: C) -> C {
    (-a.0, -a.1)
}

/// _Py_c_quot: Smith's method; ZeroDivisionError for a zero divisor.
pub fn quot(a: C, b: C) -> Result<C, PyErr> {
    let (abs_br, abs_bi) = (b.0.abs(), b.1.abs());
    if abs_br >= abs_bi {
        if abs_br == 0.0 {
            return Err(PyErr::ZeroDivision);
        }
        let ratio = b.1 / b.0;
        let denom = b.0 + b.1 * ratio;
        Ok(((a.0 + a.1 * ratio) / denom, (a.1 - a.0 * ratio) / denom))
    } else if abs_bi >= abs_br {
        let ratio = b.0 / b.1;
        let denom = b.0 * ratio + b.1;
        Ok(((a.0 * ratio + a.1) / denom, (a.1 * ratio - a.0) / denom))
    } else {
        Ok((f64::NAN, f64::NAN))                          // at least one of b's parts is a nan
    }
}

/// _Py_c_pow: polar form.
fn c_pow(a: C, b: C) -> Result<C, PyErr> {
    if b.0 == 0.0 && b.1 == 0.0 {
        return Ok((1.0, 0.0));
    }
    if a.0 == 0.0 && a.1 == 0.0 {
        if b.1 != 0.0 || b.0 < 0.0 {
            return Err(PyErr::ZeroDivision);
        }
        return Ok((0.0, 0.0));
    }
    let vabs = a.0.hypot_lm(a.1);
    let mut len = vabs.powf_lm(b.0);
    let at = a.1.atan2_lm(a.0);
    let mut phase = at * b.0;
    if b.1 != 0.0 {
        len /= (at * b.1).exp_lm();
        phase += b.1 * vabs.ln_lm();
    }
    Ok((len * phase.cos_lm(), len * phase.sin_lm()))
}

fn c_powu(x: C, n: i64) -> C {
    let (mut r, mut p, mut mask) = ((1.0, 0.0), x, 1i64);
    while mask > 0 && n >= mask {
        if n & mask != 0 {
            r = mul(r, p);
        }
        mask <<= 1;
        p = mul(p, p);
    }
    r
}

fn c_powi(x: C, n: i64) -> Result<C, PyErr> {
    if n > 0 {
        Ok(c_powu(x, n))
    } else {
        quot((1.0, 0.0), c_powu(x, -n))
    }
}

/// complex ** complex (complex_pow): an integral real exponent up to 100 by repeated squaring, else polar form;
/// OverflowError when a finite computation overflows.
pub fn pow(a: C, b: C) -> Result<C, PyErr> {
    let r = if b.1 == 0.0 && b.0 == b.0.floor() && b.0.abs() <= 100.0 {
        c_powi(a, b.0 as i64)?
    } else {
        c_pow(a, b)?
    };
    if !(r.0.is_finite() && r.1.is_finite()) && a.0.is_finite() && a.1.is_finite() && b.0.is_finite()
        && b.1.is_finite() {
        return Err(PyErr::Overflow);
    }
    Ok(r)
}

/// abs(z): hypot, OverflowError when finite parts overflow.
pub fn abs(a: C) -> Result<f64, PyErr> {
    let r = crate::py::hypot2(a.0, a.1);
    if r.is_infinite() && a.0.is_finite() && a.1.is_finite() {
        return Err(PyErr::Overflow);
    }
    Ok(r)
}

pub fn phase(a: C) -> f64 {
    a.1.atan2_lm(a.0)
}

/// cmath.rect(r, phi) for finite arguments (an infinite one is outside what the procedures use).
pub fn rect(r: f64, phi: f64) -> Result<C, PyErr> {
    if !(r.is_finite() && phi.is_finite()) {
        return Err(PyErr::Unsupported("cmath.rect of an infinite or nan argument".into()));
    }
    if phi == 0.0 {
        return Ok((r, r * phi));                          // cmath keeps the sign of zero this way
    }
    Ok((r * phi.cos_lm(), r * phi.sin_lm()))
}

/// cmath.sqrt (cmathmodule.c cmath_sqrt_impl), finite arguments.
pub fn sqrt(z: C) -> Result<C, PyErr> {
    if !(z.0.is_finite() && z.1.is_finite()) {
        return Err(PyErr::Unsupported("cmath.sqrt of an infinite or nan argument".into()));
    }
    if z.0 == 0.0 && z.1 == 0.0 {
        return Ok((0.0, z.1));
    }
    let (ax, ay) = (z.0.abs(), z.1.abs());
    let s = if ax < f64::MIN_POSITIVE && ay < f64::MIN_POSITIVE {
        // hypot would be subnormal: scale up by 2^53, take the root, scale down by 2^-27 (CM_SCALE_UP/DOWN)
        let ax = ax * 2f64.powi(53);
        (ax + ax.hypot_lm(ay * 2f64.powi(53))).sqrt() * 2f64.powi(-27)
    } else {
        let ax = ax / 8.0;
        2.0 * (ax + ax.hypot_lm(ay / 8.0)).sqrt()
    };
    let d = ay / (2.0 * s);
    Ok(if z.0 >= 0.0 { (s, d.copysign(z.1)) } else { (d, s.copysign(z.1)) })
}

/// cmath.exp (cmathmodule.c cmath_exp_impl), finite arguments.
pub fn exp(z: C) -> Result<C, PyErr> {
    if !(z.0.is_finite() && z.1.is_finite()) {
        return Err(PyErr::Unsupported("cmath.exp of an infinite or nan argument".into()));
    }
    const CM_LOG_LARGE_DOUBLE: f64 = 708.3964185322641;  // log(DBL_MAX) rounded down, as cmathmodule.c
    let r = if z.0 > CM_LOG_LARGE_DOUBLE {
        let l = (z.0 - 1.0).exp_lm();
        (l * z.1.cos_lm() * std::f64::consts::E, l * z.1.sin_lm() * std::f64::consts::E)
    } else {
        let l = z.0.exp_lm();
        (l * z.1.cos_lm(), l * z.1.sin_lm())
    };
    if r.0.is_infinite() || r.1.is_infinite() {
        return Err(PyErr::Overflow);
    }
    Ok(r)
}

/// cmath.log_lm(z) (cmathmodule.c c_log), finite arguments; ValueError("math domain error") at zero.
pub fn log(z: C) -> Result<C, PyErr> {
    if !(z.0.is_finite() && z.1.is_finite()) {
        return Err(PyErr::Unsupported("cmath.log of an infinite or nan argument".into()));
    }
    let (ax, ay) = (z.0.abs(), z.1.abs());
    let ln2 = std::f64::consts::LN_2;
    let re = if ax > f64::MAX / 4.0 || ay > f64::MAX / 4.0 {
        (ax / 2.0).hypot_lm(ay / 2.0).ln_lm() + ln2
    } else if ax < f64::MIN_POSITIVE && ay < f64::MIN_POSITIVE {
        if ax > 0.0 || ay > 0.0 {
            let k = 2f64.powi(53);
            (ax * k).hypot_lm(ay * k).ln_lm() - 53.0 * ln2
        } else {
            return Err(crate::py::value_error("math domain error"));
        }
    } else {
        let h = ax.hypot_lm(ay);
        if (0.71..=1.73).contains(&h) {
            let (am, an) = if ax > ay { (ax, ay) } else { (ay, ax) };
            ((am - 1.0) * (am + 1.0) + an * an).ln_1p_lm() / 2.0
        } else {
            h.ln_lm()
        }
    };
    Ok((re, z.1.atan2_lm(z.0)))
}

fn finite_or(z: C, name: &str) -> Result<(), PyErr> {
    if z.0.is_finite() && z.1.is_finite() {
        Ok(())
    } else {
        Err(PyErr::Unsupported(format!("cmath.{name} of an infinite or nan argument")))
    }
}

fn no_overflow(r: C) -> Result<C, PyErr> {
    if r.0.is_infinite() || r.1.is_infinite() { Err(PyErr::Overflow) } else { Ok(r) }
}

const CM_LOG_LARGE_DOUBLE: f64 = 708.3964185322641;
const CM_LARGE_DOUBLE: f64 = f64::MAX / 4.0;
const M_LN2: f64 = std::f64::consts::LN_2;

/// cmath.cosh (cmathmodule.c cmath_cosh_impl), finite arguments.
pub fn cosh(z: C) -> Result<C, PyErr> {
    finite_or(z, "cosh")?;
    let (x, y) = z;
    no_overflow(if x.abs() > CM_LOG_LARGE_DOUBLE {
        let xm = x - 1f64.copysign(x);
        (y.cos_lm() * xm.cosh_lm() * std::f64::consts::E, y.sin_lm() * xm.sinh_lm() * std::f64::consts::E)
    } else {
        (y.cos_lm() * x.cosh_lm(), y.sin_lm() * x.sinh_lm())
    })
}

/// cmath.sinh (cmathmodule.c cmath_sinh_impl), finite arguments.
pub fn sinh(z: C) -> Result<C, PyErr> {
    finite_or(z, "sinh")?;
    let (x, y) = z;
    no_overflow(if x.abs() > CM_LOG_LARGE_DOUBLE {
        let xm = x - 1f64.copysign(x);
        (y.cos_lm() * xm.sinh_lm() * std::f64::consts::E, y.sin_lm() * xm.cosh_lm() * std::f64::consts::E)
    } else {
        (y.cos_lm() * x.sinh_lm(), y.sin_lm() * x.cosh_lm())
    })
}

/// cmath.tanh (cmathmodule.c cmath_tanh_impl), finite arguments.
pub fn tanh(z: C) -> Result<C, PyErr> {
    finite_or(z, "tanh")?;
    let (x, y) = z;
    if x.abs() > CM_LOG_LARGE_DOUBLE {
        return Ok((1f64.copysign(x), 4.0 * y.sin_lm() * y.cos_lm() * (-2.0 * x.abs()).exp_lm()));
    }
    let tx = x.tanh_lm();
    let ty = y.tan_lm();
    let cx = 1.0 / x.cosh_lm();
    let txty = tx * ty;
    let denom = 1.0 + txty * txty;
    Ok((tx * (1.0 + ty * ty) / denom, ((ty / denom) * cx) * cx))
}

/// cmath.cos(z) = cosh(iz)
pub fn cos(z: C) -> Result<C, PyErr> {
    cosh((-z.1, z.0))
}

/// cmath.sin(z) = -i sinh(iz)
pub fn sin(z: C) -> Result<C, PyErr> {
    let s = sinh((-z.1, z.0))?;
    Ok((s.1, -s.0))
}

/// cmath.tan(z) = -i tanh(iz)
pub fn tan(z: C) -> Result<C, PyErr> {
    let s = tanh((-z.1, z.0))?;
    Ok((s.1, -s.0))
}

/// cmath.asinh (cmathmodule.c cmath_asinh_impl), finite arguments.
pub fn asinh(z: C) -> Result<C, PyErr> {
    finite_or(z, "asinh")?;
    let (x, y) = z;
    if x.abs() > CM_LARGE_DOUBLE || y.abs() > CM_LARGE_DOUBLE {
        let l = (x / 2.0).hypot_lm(y / 2.0).ln_lm() + M_LN2 * 2.0;
        let re = if y >= 0.0 { l.copysign(x) } else { -l.copysign(-x) };
        return Ok((re, y.atan2_lm(x.abs())));
    }
    let s1 = sqrt((1.0 + y, -x))?;
    let s2 = sqrt((1.0 - y, x))?;
    Ok(((s1.0 * s2.1 - s2.0 * s1.1).asinh_lm(), y.atan2_lm(s1.0 * s2.0 - s1.1 * s2.1)))
}

/// cmath.acosh (cmathmodule.c cmath_acosh_impl), finite arguments.
pub fn acosh(z: C) -> Result<C, PyErr> {
    finite_or(z, "acosh")?;
    let (x, y) = z;
    if x.abs() > CM_LARGE_DOUBLE || y.abs() > CM_LARGE_DOUBLE {
        return Ok(((x / 2.0).hypot_lm(y / 2.0).ln_lm() + M_LN2 * 2.0, y.atan2_lm(x)));
    }
    let s1 = sqrt((x - 1.0, y))?;
    let s2 = sqrt((x + 1.0, y))?;
    Ok(((s1.0 * s2.0 + s1.1 * s2.1).asinh_lm(), 2.0 * s1.1.atan2_lm(s2.0)))
}

/// cmath.acos (cmathmodule.c cmath_acos_impl), finite arguments.
pub fn acos(z: C) -> Result<C, PyErr> {
    finite_or(z, "acos")?;
    let (x, y) = z;
    if x.abs() > CM_LARGE_DOUBLE || y.abs() > CM_LARGE_DOUBLE {
        let l = (x / 2.0).hypot_lm(y / 2.0).ln_lm() + M_LN2 * 2.0;
        let im = if x < 0.0 { -l.copysign(y) } else { l.copysign(-y) };
        return Ok((y.abs().atan2_lm(x), im));
    }
    let s1 = sqrt((1.0 - x, -y))?;
    let s2 = sqrt((1.0 + x, y))?;
    Ok((2.0 * s1.0.atan2_lm(s2.0), (s2.0 * s1.1 - s2.1 * s1.0).asinh_lm()))
}

/// cmath.asin(z) = -i asinh(iz)
pub fn asin(z: C) -> Result<C, PyErr> {
    let s = asinh((-z.1, z.0))?;
    Ok((s.1, -s.0))
}

/// cmath.atanh (cmathmodule.c cmath_atanh_impl), finite arguments; ValueError("math domain error") at +-1.
pub fn atanh(z: C) -> Result<C, PyErr> {
    finite_or(z, "atanh")?;
    if z.0 < 0.0 {
        let r = atanh(neg(z))?;
        return Ok(neg(r));
    }
    let (x, y) = z;
    let ay = y.abs();
    let sqrt_large = CM_LARGE_DOUBLE.sqrt();
    let sqrt_min = f64::MIN_POSITIVE.sqrt();
    if x > sqrt_large || ay > sqrt_large {
        let h = (x / 2.0).hypot_lm(y / 2.0);
        return Ok((x / 4.0 / h / h, -(std::f64::consts::FRAC_PI_2).copysign(-y)));
    }
    if x == 1.0 && ay < sqrt_min {
        if ay == 0.0 {
            return Err(crate::py::value_error("math domain error"));
        }
        return Ok((-(ay.sqrt() / ay.hypot_lm(2.0).sqrt()).ln_lm(), (2f64.atan2_lm(-ay) / 2.0).copysign(y)));
    }
    Ok(((4.0 * x / ((1.0 - x) * (1.0 - x) + ay * ay)).ln_1p_lm() / 4.0,
        -(-2.0 * y).atan2_lm((1.0 - x) * (1.0 + x) - ay * ay) / 2.0))
}

/// cmath.atan(z) = -i atanh(iz)
pub fn atan(z: C) -> Result<C, PyErr> {
    let s = atanh((-z.1, z.0))?;
    Ok((s.1, -s.0))
}

/// A float as complex repr writes a part: repr's digits without a trailing ".0" (CPython's 'r' format, no
/// Py_DTSF_ADD_DOT_0).
fn part(x: f64) -> String {
    if x.is_nan() {
        return "nan".into();
    }
    if x.is_infinite() {
        return if x > 0.0 { "inf".into() } else { "-inf".into() };
    }
    let s = crate::num::py_repr_f64(x);
    s.strip_suffix(".0").map(str::to_string).unwrap_or(s)
}

/// repr(z): "2j" for a zero real part with a positive sign, else "(re+imj)".
pub fn repr(z: C) -> String {
    let im = part(z.1);
    let im = if im.starts_with('-') { im } else { format!("+{im}") };
    if z.0 == 0.0 && z.0.is_sign_positive() {
        return format!("{}j", part(z.1));
    }
    format!("({}{}j)", part(z.0), im)
}

/// format(z, spec) for a non-empty float spec: both parts in that format, the imaginary one signed, then "j"
/// (format(1+2j, '.3f') is '1.000+2.000j').
pub fn format(z: C, spec: &str) -> String {
    let re = crate::pyfmt::fmt(z.0, spec);
    let im = crate::pyfmt::fmt(z.1, spec);
    let im = if im.starts_with('-') { im } else { format!("+{im}") };
    format!("{re}{im}j")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_python_writes_them() {
        assert_eq!(repr((1.0, 2.0)), "(1+2j)");
        assert_eq!(repr((-0.0, 1.0)), "(-0+1j)");
        assert_eq!(repr((0.0, 2.0)), "2j");
        assert_eq!(repr((1e16, 1.0)), "(1e+16+1j)");
        assert_eq!(repr((1.5, -2.0)), "(1.5-2j)");
        assert_eq!(format((1.0, 2.0), ".3f"), "1.000+2.000j");
    }

    #[test]
    fn complex_functions_as_cmath() {
        // cmath in CPython 3.13 on Linux (2026-10-30); the elementary functions underneath may differ in the last bit
        type F = fn(C) -> Result<C, PyErr>;
        let fs: [(&str, F); 12] = [("cosh", cosh), ("sinh", sinh), ("tanh", tanh), ("cos", cos), ("sin", sin),
            ("tan", tan), ("acosh", acosh), ("asinh", asinh), ("atanh", atanh), ("acos", acos), ("asin", asin),
            ("atan", atan)];
        let cases: [(&str, C, C); 36] = [
            ("cosh", (0.3, 1.7), (-0.13468611222056817, 0.30198205908019077)),
            ("cosh", (-2.5, 0.4), (5.648212643652469, -2.3560605996348833)),
            ("cosh", (1.2, -0.01), (1.8105650353004459, -0.015094361978487039)),
            ("sinh", (0.3, 1.7), (-0.039235763211921855, 1.0366254194722615)),
            ("sinh", (-2.5, 0.4), (-5.572607353227216, 2.3880260037274095)),
            ("sinh", (1.2, -0.01), (1.509385882973342, -0.018106253898824736)),
            ("tanh", (0.3, 1.7), (2.9115207481259047, -1.1686311699363665)),
            ("tanh", (-2.5, 0.4), (-0.9906090499682905, 0.009576666968472824)),
            ("tanh", (1.2, -0.01), (0.83368003507265, -0.003050089650209115)),
            ("cos", (0.3, 1.7), (2.701992959680282, -0.7818376958374182)),
            ("cos", (-2.5, 0.4), (-0.8660942286425589, 0.24582382511889792)),
            ("cos", (1.2, -0.01), (0.3623758725153804, 0.009320546200296626)),
            ("sin", (0.3, 1.7), (0.8358243686191043, 2.527468723190649)),
            ("sin", (-2.5, 0.4), (-0.64699170030571, -0.3290716033879797)),
            ("sin", (1.2, -0.01), (0.9320856883098756, -0.0036236379380277816)),
            ("tan", (0.3, 1.7), (0.035682500915865334, 0.9457340139741869)),
            ("tan", (-2.5, 0.4), (0.5915279571294256, 0.5478425473611187)),
            ("tan", (1.2, -0.01), (2.5701941063068605, -0.07610675252232865)),
            ("acosh", (0.3, 1.7), (1.3107496902488214, 1.4194108455323973)),
            ("acosh", (-2.5, 0.4), (1.583064277731504, 2.9692327019062206)),
            ("acosh", (1.2, -0.01), (0.6225679237417546, -0.01507053709412824)),
            ("asinh", (0.3, 1.7), (1.1511827117610314, 1.3583551589067286)),
            ("asinh", (-2.5, 0.4), (-1.6574026509870519, 0.14769524749264357)),
            ("asinh", (1.2, -0.01), (1.0159888764269558, -0.006401810303562233)),
            ("atanh", (0.3, 1.7), (0.07595582215779155, 1.049069489395666)),
            ("atanh", (-2.5, 0.4), (-0.40971896292848803, 1.4973911344930801)),
            ("atanh", (1.2, -0.01), (1.198328581585435, -1.5480898405544548)),
            ("acos", (0.3, 1.7), (1.4194108455323973, -1.3107496902488214)),
            ("acos", (-2.5, 0.4), (2.9692327019062206, -1.583064277731504)),
            ("acos", (1.2, -0.01), (0.01507053709412824, 0.6225679237417546)),
            ("asin", (0.3, 1.7), (0.1513854812624992, 1.3107496902488214)),
            ("asin", (-2.5, 0.4), (-1.3984363751113238, 1.583064277731504)),
            ("asin", (1.2, -0.01), (1.5557257897007684, -0.6225679237417546)),
            ("atan", (0.3, 1.7), (1.4236790442393028, 0.6358752035135133)),
            ("atan", (-2.5, 0.4), (-1.197779675476077, 0.054192317400185486)),
            ("atan", (1.2, -0.01), (0.8760782063212993, -0.004098284473917201)),
        ];
        for (name, z, want) in cases {
            let f = fs.iter().find(|(n, _)| *n == name).unwrap().1;
            let got = f(z).unwrap();
            for (g, w) in [(got.0, want.0), (got.1, want.1)] {
                assert!((g - w).abs() <= 4e-16 * w.abs().max(1e-300), "{name}{z:?}: {got:?} vs {want:?}");
            }
        }
    }

    #[test]
    fn as_python_computes_them() {
        // (-8.0) ** (1/3) in Python 3.13: (1.0000000000000002+1.7320508075688772j)
        assert_eq!(pow((-8.0, 0.0), (1.0 / 3.0, 0.0)).unwrap(), (1.0000000000000002, 1.7320508075688772));
        assert_eq!(quot((1.0, 2.0), (3.0, -4.0)).unwrap(), (-0.2, 0.4));
        assert_eq!(pow((1.0, 1.0), (3.0, 0.0)).unwrap(), (-2.0, 2.0));
        assert!(matches!(quot((1.0, 0.0), (0.0, 0.0)), Err(PyErr::ZeroDivision)));
        assert_eq!(sqrt((-4.0, 0.0)).unwrap(), (0.0, 2.0));
    }
}
