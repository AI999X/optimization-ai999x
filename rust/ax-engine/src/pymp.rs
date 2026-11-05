//! The mpmath functions the procedures use, in double precision: the answers are checked to 6 significant digits
//! (tools/rust_accuracy.py), and these routines reach about 12 or more where mpmath works at 20-40 digits.
//! - quad: adaptive Gauss-Kronrod (7/15 points, QUADPACK's nodes) on each interval between the given points, an
//!   infinite end mapped to a finite one (x = a + (1 - t)/t); complex integrands integrated part by part;
//! - polyroots: mpmath's own Durand-Kerner iteration (same starting points, same update order), each root polished by
//!   Newton steps, then mpmath's cleanup of tiny parts and its sort by (|imaginary part|, real part);
//! - gammainc(z, a, b, regularized): from the regularized lower function P (series) and upper Q (continued fraction);
//! - besselj(n, x) for an integer order; the elementary functions with mpmath's complex results for a negative square
//!   root or logarithm; mpf, mpc, fsum, factorial, pi, e, inf.
//! with mp.workdps(n) changes nothing here: every value is a double.

#[allow(unused_imports)]
use crate::lm::{Lm, LmF64};
use crate::py::{self, Py, PyErr, R};

fn unsupported(what: &str) -> PyErr {
    PyErr::Unsupported(format!("mpmath.{what}"))
}

fn kw<'a>(kws: &'a [(String, Py)], name: &str) -> Option<&'a Py> {
    kws.iter().find(|(k, _)| k == name).map(|(_, v)| v)
}

/// An mpmath constant (mp.pi, mpmath.inf ...).
pub fn constant(name: &str) -> R {
    if let Some(v) = crate::pympf::constant(name) {
        return Ok(v);
    }
    Ok(Py::Float(match name {
        "pi" => std::f64::consts::PI,
        "e" => std::f64::consts::E,
        "inf" => f64::INFINITY,
        "ninf" => f64::NEG_INFINITY,
        "nan" => f64::NAN,
        "eps" => f64::EPSILON,
        _ => return Err(unsupported(name)),
    }))
}

/// mpmath.<name>(args, keywords)
pub fn call(name: &str, a: &[Py], kws: &[(String, Py)]) -> R {
    let arg = |k: usize| a.get(k).ok_or_else(|| PyErr::Type(format!("{name}() missing an argument")));
    if let (Some(x), true) = (a.first(), a.len() == 1) {
        if let Some(r) = crate::pympf::func(name, x) {
            return r;
        }
    }
    match name {
        "mpf" if crate::pympf::extended() => crate::pympf::mpf(arg(0)?),
        "mpf" => match arg(0)? {
            Py::Str(_) => py::float(arg(0)?),
            x => Ok(Py::Float(x.f()?)),
        },
        "mpc" => py::complex(a),
        "sqrt" | "log" | "ln" => {
            let x = arg(0)?;
            if let Py::Complex(..) = x {
                return py::cmath(if name == "sqrt" { "sqrt" } else { "log" }, &[x.clone()]);
            }
            let v = x.f()?;
            if v < 0.0 {
                return py::cmath(if name == "sqrt" { "sqrt" } else { "log" }, &[Py::Float(v)]);  // mpmath: complex
            }
            if name != "sqrt" && v == 0.0 {
                return Ok(Py::Float(f64::NEG_INFINITY));                          // mpmath.log(0) is -inf
            }
            py::math1(if name == "sqrt" { "sqrt" } else { "log" }, &Py::Float(v))
        }
        "exp" | "sin" | "cos" | "tan" | "sinh" | "cosh" | "tanh" | "atan" | "asin" | "acos" | "expm1" | "log1p"
        | "log10" | "fabs" | "floor" | "ceil" => {
            let x = arg(0)?;
            if let Py::Complex(..) = x {
                return py::cmath(name, &[x.clone()]);
            }
            match name {
                "fabs" => Ok(Py::Float(x.f()?.abs())),
                "floor" | "ceil" => Ok(Py::Float(if name == "floor" { x.f()?.floor() } else { x.f()?.ceil() })),
                "asin" | "acos" if x.f()?.abs() > 1.0 => py::cmath(name, &[Py::Float(x.f()?)]),
                _ => py::math1(name, &Py::Float(x.f()?)),
            }
        }
        "atan2" => py::atan2(arg(0)?, arg(1)?),
        "power" => py::pow(&Py::Float(arg(0)?.f()?), arg(1)?),
        "re" => py::real(arg(0)?),
        "im" => py::imag(arg(0)?),
        "fsum" => match crate::pympf::fsum(&arg(0)?.items()?) {
            Some(r) => r,
            None => py::fsum(arg(0)?),
        },
        "factorial" => {
            let n = arg(0)?.f()?;
            Ok(Py::Float(crate::pymath::tgamma(n + 1.0).map_err(|_| py::value_error("factorial"))?))
        }
        "gamma" => Ok(Py::Float(crate::pymath::tgamma(arg(0)?.f()?).map_err(|_| py::value_error("gamma"))?)),
        "quad" => quad_call(arg(0)?, arg(1)?, kws),
        // inside an extended workdps block: mpmath's own iteration at its precision (pympf::polyroots_ext)
        "polyroots" if crate::pympf::extended() && kws.iter().all(|(k, _)| ["maxsteps", "extraprec", "error"].contains(&k.as_str())) => {
            let maxsteps = kw(kws, "maxsteps").map(|m| m.i()).transpose()?.unwrap_or(50).max(0) as usize;
            let extraprec = kw(kws, "extraprec").map(|m| m.f()).transpose()?.unwrap_or(10.0).max(0.0) as usize;
            let error = kw(kws, "error").map(|e| e.truthy()).unwrap_or(false);
            match crate::pympf::polyroots_ext(&arg(0)?.items()?, maxsteps, extraprec, error) {
                Some(r) => r,
                None => polyroots(&arg(0)?.items()?, maxsteps as i64, error),
            }
        }
        "polyroots" => polyroots(&arg(0)?.items()?, kw(kws, "maxsteps").map(|m| m.i()).transpose()?.unwrap_or(50),
                                 kw(kws, "error").map(|e| e.truthy()).unwrap_or(false)),
        "gammainc" => {
            let z = arg(0)?.f()?;
            let lo = a.get(1).map(|x| x.f()).transpose()?.unwrap_or(0.0);
            let hi = a.get(2).map(|x| x.f()).transpose()?.unwrap_or(f64::INFINITY);
            let reg = kw(kws, "regularized").map(|r| r.truthy()).unwrap_or(false);
            Ok(Py::Float(gammainc(z, lo, hi, reg)?))
        }
        "besselj" => {
            let (n, x) = (arg(0)?.f()?, arg(1)?.f()?);
            if n != n.trunc() || n.abs() > 1e6 {
                return Err(unsupported("besselj of a non-integer order"));
            }
            Ok(Py::Float(libm::jn(n as i32, x)))
        }
        "diff" => crate::pysym::mp_diff(a, kws),
        "taylor" => crate::pysym::mp_taylor(a),
        "e1" => crate::pympf::e1_py(arg(0)?),
        "erf" => Ok(Py::Float(libm::erf(arg(0)?.f()?))),
        "erfc" => Ok(Py::Float(libm::erfc(arg(0)?.f()?))),
        "beta" => {
            let (x, y) = (arg(0)?.f()?, arg(1)?.f()?);
            let lg = |v: f64| crate::pymath::lgamma(v).map_err(|_| py::value_error("beta"));
            let sign = |v: f64| crate::pymath::tgamma(v).map(|g| g.signum()).unwrap_or(1.0);
            Ok(Py::Float(sign(x) * sign(y) * sign(x + y) * (lg(x)? + lg(y)? - lg(x + y)?).exp_lm()))
        }
        "legendre" | "chebyt" => {
            // the three-term recurrence in exact rational arithmetic on the double's exact value
            let n = arg(0)?.f()?;
            if n < 0.0 || n != n.trunc() || n > 5000.0 {
                return Err(unsupported("orthogonal polynomial of this degree"));
            }
            let x = crate::pyfrac::from_f64(arg(1)?.f()?)?;
            Ok(Py::Float(ortho_exact(name == "legendre", n as usize, &x)))
        }
        "zeta" => Ok(Py::Float(zeta(arg(0)?.f()?)?)),
        "polyval" => {
            let x = arg(1)?.f()?;
            let mut v = 0.0;
            for c in arg(0)?.items()? {
                v = v * x + c.f()?;
            }
            Ok(Py::Float(v))
        }
        "linspace" => {
            let (lo, hi, n) = (arg(0)?.f()?, arg(1)?.f()?, arg(2)?.f()? as usize);
            if n == 1 {
                return Ok(Py::list(vec![Py::Float(lo)]));
            }
            Ok(Py::list((0..n).map(|k| Py::Float(lo + (hi - lo) * k as f64 / (n - 1) as f64)).collect()))
        }
        "sec" | "csc" | "cot" => {
            let x = arg(0)?.f()?;
            let (s, c) = (x.sin_lm(), x.cos_lm());
            let (num, den) = match name {
                "sec" => (1.0, c),
                "csc" => (1.0, s),
                _ => (c, s),
            };
            if den == 0.0 {
                return Err(PyErr::ZeroDivision);
            }
            Ok(Py::Float(num / den))
        }
        "findroot" => findroot(arg(0)?, arg(1)?),
        _ => Err(unsupported(name)),
    }
}

/// P_n(x) (legendre) or T_n(x) (chebyt) by the three-term recurrence in exact rationals, rounded once
fn ortho_exact(legendre: bool, n: usize, x: &crate::pyfrac::Q) -> f64 {
    use num_traits::{One, ToPrimitive};
    type Q = crate::pyfrac::Q;
    let int = |k: usize| Q::from_integer(num_bigint::BigInt::from(k));
    let (mut p0, mut p1) = (Q::one(), x.clone());
    if n == 0 {
        return 1.0;
    }
    for k in 1..n {
        let p2 = if legendre {
            (int(2 * k + 1) * x * &p1 - int(k) * &p0) / int(k + 1)
        } else {
            int(2) * x * &p1 - &p0
        };
        p0 = p1;
        p1 = p2;
    }
    p1.to_f64().unwrap_or(f64::NAN)
}

/// The Riemann zeta function for real s > 1 by Euler-Maclaurin summation (N = 12 terms, 10 Bernoulli corrections)
fn zeta(s: f64) -> Result<f64, PyErr> {
    if s == 1.0 {
        return Err(py::value_error("zeta(1) is a pole"));
    }
    if s <= 1.0 {
        return Err(unsupported("zeta for s <= 1"));
    }
    if s > 60.0 {
        return Ok(1.0 + 2f64.powf_lm(-s) + 3f64.powf_lm(-s));
    }
    const B2: [f64; 10] = [1.0 / 6.0, -1.0 / 30.0, 1.0 / 42.0, -1.0 / 30.0, 5.0 / 66.0, -691.0 / 2730.0, 7.0 / 6.0,
                           -3617.0 / 510.0, 43867.0 / 798.0, -174611.0 / 330.0];
    let n = 12.0f64;
    let mut sum = 0.0;
    for k in (1..12).rev() {
        sum += (k as f64).powf_lm(-s);
    }
    sum += n.powf_lm(1.0 - s) / (s - 1.0) + 0.5 * n.powf_lm(-s);
    // + sum over j of B_2j/(2j)! s(s+1)...(s+2j-2) N^(-s-2j+1)
    let mut rising = s;
    let mut fact = 2.0;
    let mut npow = n.powf_lm(-s - 1.0);
    for (j, b) in B2.iter().enumerate() {
        sum += b / fact * rising * npow;
        let j1 = (j + 1) as f64;
        rising *= (s + 2.0 * j1 - 1.0) * (s + 2.0 * j1);
        fact *= (2.0 * j1 + 1.0) * (2.0 * j1 + 2.0);
        npow /= n * n;
    }
    Ok(sum)
}

/// mpmath.findroot(f, (a, b)) with a bracketing solver: the sign change in [a, b] bisected to full precision
fn findroot(f: &Py, x0: &Py) -> R {
    let ev = |t: f64| -> Result<f64, PyErr> { py::call(f, &[Py::Float(t)])?.f() };
    let pts = x0.items()?;
    if pts.len() != 2 {
        return Err(unsupported("findroot without a bracket"));
    }
    let (mut a, mut b) = (pts[0].f()?, pts[1].f()?);
    let (mut fa, fb) = (ev(a)?, ev(b)?);
    if fa == 0.0 {
        return Ok(Py::Float(a));
    }
    if fb == 0.0 {
        return Ok(Py::Float(b));
    }
    if fa.signum() == fb.signum() {
        return Err(unsupported("findroot: no sign change in the bracket"));
    }
    for _ in 0..200 {
        let m = 0.5 * (a + b);
        if m <= a.min(b) || m >= a.max(b) {
            break;
        }
        let fm = ev(m)?;
        if fm == 0.0 {
            return Ok(Py::Float(m));
        }
        if fm.signum() == fa.signum() {
            a = m;
            fa = fm;
        } else {
            b = m;
        }
    }
    Ok(Py::Float(0.5 * (a + b)))
}

// ------------------------------------------------------------------------------------------------- integration
/// QUADPACK qk15: the 15-point Kronrod rule and its embedded 7-point Gauss rule on [a, b].
const XGK: [f64; 8] = [0.991455371120812639206854697526329, 0.949107912342758524526189684047851,
    0.864864423359769072789712788640926, 0.741531185599394439863864773280788,
    0.586087235467691130294144845693013, 0.405845151377397166906606412076961,
    0.207784955007898467600689403773245, 0.000000000000000000000000000000000];
const WGK: [f64; 8] = [0.022935322010529224963732008058970, 0.063092092629978553290700663189204,
    0.104790010322250183839876322541518, 0.140653259715525918745189590510238,
    0.169004726639267902826583426598550, 0.190350578064785409913256402421014,
    0.204432940075298892414161999234649, 0.209482141084727828012999174891714];
const WG: [f64; 4] = [0.129484966168869693270611432679082, 0.279705391489276667901467771423780,
    0.381830050505118944950369775488975, 0.417959183673469387755102040816327];

/// One rule on [a, b] of g (a function of x giving a pair (re, im)): (estimate, error estimate) for each part.
fn qk15(g: &mut dyn FnMut(f64) -> Result<(f64, f64), PyErr>, a: f64, b: f64) -> Result<((f64, f64), f64), PyErr> {
    let c = 0.5 * (a + b);
    let h = 0.5 * (b - a);
    let fc = g(c)?;
    let (mut rk, mut rg) = ((fc.0 * WGK[7], fc.1 * WGK[7]), (fc.0 * WG[3], fc.1 * WG[3]));
    for j in 0..7 {
        let x = h * XGK[j];
        let (f1, f2) = (g(c - x)?, g(c + x)?);
        rk.0 += WGK[j] * (f1.0 + f2.0);
        rk.1 += WGK[j] * (f1.1 + f2.1);
        if j % 2 == 1 {
            rg.0 += WG[j / 2] * (f1.0 + f2.0);
            rg.1 += WG[j / 2] * (f1.1 + f2.1);
        }
    }
    let est = (rk.0 * h, rk.1 * h);
    let err = ((rk.0 - rg.0) * h).abs() + ((rk.1 - rg.1) * h).abs();
    Ok((est, err))
}

/// Global adaptive bisection on [a, b] (finite) until the summed error is below rel * |integral| (or abs).
fn adapt(g: &mut dyn FnMut(f64) -> Result<(f64, f64), PyErr>, a: f64, b: f64) -> Result<((f64, f64), f64), PyErr> {
    let mut parts = vec![(a, b, qk15(g, a, b)?)];
    for _ in 0..4000 {
        let tot = parts.iter().fold((0.0, 0.0), |s, p| (s.0 + p.2 .0 .0, s.1 + p.2 .0 .1));
        let err: f64 = parts.iter().map(|p| p.2 .1).sum();
        let size = tot.0.hypot_lm(tot.1);
        if err <= (1e-14 * size).max(1e-300) {
            return Ok((tot, err));
        }
        let k = (0..parts.len()).max_by(|&i, &j| parts[i].2 .1.total_cmp(&parts[j].2 .1)).unwrap();
        let (lo, hi, _) = parts.swap_remove(k);
        let mid = 0.5 * (lo + hi);
        if !(mid > lo && mid < hi) {
            parts.push((lo, hi, qk15(g, lo, hi)?));            // cannot split further: keep what we have
            break;
        }
        parts.push((lo, mid, qk15(g, lo, mid)?));
        parts.push((mid, hi, qk15(g, mid, hi)?));
    }
    let tot = parts.iter().fold((0.0, 0.0), |s, p| (s.0 + p.2 .0 .0, s.1 + p.2 .0 .1));
    Ok((tot, parts.iter().map(|p| p.2 .1).sum()))
}

/// The integral of f over [a, b], either end possibly infinite.
pub(crate) fn integrate(f: &Py, a: f64, b: f64) -> Result<((f64, f64), f64), PyErr> {
    let mut val = |x: f64| -> Result<(f64, f64), PyErr> {
        match py::call(f, &[Py::Float(x)])? {
            Py::Complex(r, i) => Ok((r, i)),
            v => Ok((v.f()?, 0.0)),
        }
    };
    if a == b {
        return Ok(((0.0, 0.0), 0.0));
    }
    if a > b {
        let ((r, i), e) = integrate(f, b, a)?;
        return Ok(((-r, -i), e));
    }
    match (a.is_infinite(), b.is_infinite()) {
        (false, false) => adapt(&mut val, a, b),
        (false, true) => {
            // x = a + (1 - t)/t, t in (0, 1]
            let mut g = |t: f64| -> Result<(f64, f64), PyErr> {
                let v = val(a + (1.0 - t) / t)?;
                Ok((v.0 / (t * t), v.1 / (t * t)))
            };
            adapt(&mut g, 0.0, 1.0)
        }
        (true, false) => {
            let mut g = |t: f64| -> Result<(f64, f64), PyErr> {
                let v = val(b - (1.0 - t) / t)?;
                Ok((v.0 / (t * t), v.1 / (t * t)))
            };
            adapt(&mut g, 0.0, 1.0)
        }
        (true, true) => {
            let ((r1, i1), e1) = integrate(f, f64::NEG_INFINITY, 0.0)?;
            let ((r2, i2), e2) = integrate(f, 0.0, f64::INFINITY)?;
            Ok(((r1 + r2, i1 + i2), e1 + e2))
        }
    }
}

/// mpmath's tanh-sinh rule (its default quad method) on a finite [a, b], in double precision: degrees 1..max_degree
/// of the step sum with mpmath's nodes (t = k 2^-m, x = tanh(pi/2 sinh t), cut where 1 - x <= 2^-(prec+10)), each
/// degree reusing the previous sum, stopped by mpmath's error extrapolation. Unlike an adaptive Gauss-Kronrod rule,
/// its nodes crowd the ends doubly exponentially, so an integrand concentrated near an end of a long interval (the
/// package's convolution of e^-u at t = 2e6) is seen as mpmath sees it. A degree whose sum agrees with the last two to
/// double precision is taken as converged (mpmath would go on to its 2^-prec epsilon).
pub(crate) fn tanh_sinh(f: &Py, a: f64, b: f64, max_degree: usize) -> Result<((f64, f64), f64), PyErr> {
    let prec = crate::pympf::prec() as i32;
    let eps = 2f64.powi(-prec - 2);                     // mpmath: ctx.eps / 8
    let tol = 2f64.powi(-prec - 10);
    let (c, d) = ((b - a) / 2.0, (b + a) / 2.0);
    let val = |x: f64| -> Result<(f64, f64), PyErr> {
        match py::call(f, &[Py::Float(x)])? {
            Py::Complex(r, i) => Ok((r, i)),
            v => Ok((v.f()?, 0.0)),
        }
    };
    // Neumaier-compensated sums (mpmath sums the step exactly)
    fn acc(s: &mut (f64, f64), v: f64) {
        let t = s.0 + v;
        s.1 += if s.0.abs() >= v.abs() { (s.0 - t) + v } else { (v - t) + s.0 };
        s.0 = t;
    }
    let pi2 = std::f64::consts::FRAC_PI_2;
    let mut results: Vec<(f64, f64)> = vec![];
    let mut abs_prev = 0.0;                             // the same step sums of |w f|: the size of the rounding noise
    let mut err = 0.0;
    for degree in 1..=max_degree.max(1) {
        let h = 2f64.powi(-(degree as i32));
        let (t0, step) = if degree == 1 { (h, h) } else { (h, 2.0 * h) };
        let (mut sr, mut si) = ((0.0, 0.0), (0.0, 0.0));
        let mut sa = 0.0;
        if degree == 1 {
            let v = val(d)?;
            acc(&mut sr, c * pi2 * v.0);
            acc(&mut si, c * pi2 * v.1);
            sa += (c * pi2).abs() * v.0.hypot_lm(v.1);
        }
        for k in 0..=(20usize << degree) {
            let t = t0 + k as f64 * step;
            // the lm trait (one maths library everywhere): with the platform's sinh/exp/cosh the native and browser
            // builds placed nodes a bit apart and could stop at different degrees (nuclear_engineering line source)
            let u = pi2 * t.sinh_lm();
            let e2 = (2.0 * u).exp_lm();
            let one_minus_x = 2.0 / (e2 + 1.0);           // 1 - tanh(u), without cancellation
            if one_minus_x <= tol {
                break;
            }
            let ch = u.cosh_lm();
            let w = c * pi2 * t.cosh_lm() / (ch * ch);
            let dx = c * one_minus_x;
            let (xp, xm) = (b - dx, a + dx);
            // a node that rounds onto an end in double precision is evaluated there (the package's integrands take
            // float(u), which rounds the same way: on a long interval such nodes still carry weight); where the
            // integrand is undefined at the end itself, mpmath's node lies just inside it and its term is negligible
            let inside = |x: f64| x > a.min(b) && x < a.max(b);
            let at = |x: f64| -> Result<(f64, f64), PyErr> {
                if inside(x) {
                    return val(x);
                }
                Ok(match val(x) { Ok(v) if v.0.is_finite() && v.1.is_finite() => v, _ => (0.0, 0.0) })
            };
            let (fp, fm) = (at(xp)?, at(xm)?);
            acc(&mut sr, w * (fp.0 + fm.0));
            acc(&mut si, w * (fp.1 + fm.1));
            sa += w.abs() * (fp.0.hypot_lm(fp.1) + fm.0.hypot_lm(fm.1));
        }
        let abs_now = h * (sa + if degree > 1 { abs_prev / (h * 2.0) } else { 0.0 });
        abs_prev = abs_now;
        let (mut sr, mut si) = (sr.0 + sr.1, si.0 + si.1);
        if let Some(p) = results.last() {
            sr += p.0 / (h * 2.0);
            si += p.1 / (h * 2.0);
        }
        results.push((h * sr, h * si));
        if degree > 1 {
            let n = results.len();
            let dist = |x: (f64, f64), y: (f64, f64)| (x.0 - y.0).hypot_lm(x.1 - y.1);
            let size = results[n - 1].0.hypot_lm(results[n - 1].1).max(abs_now).max(f64::MIN_POSITIVE);
            err = if n == 2 {
                dist(results[0], results[1])
            } else if results[n - 1] == results[n - 2] && results[n - 2] == results[n - 3] {
                0.0
            } else {
                let d1 = dist(results[n - 1], results[n - 2]).log10_lm();
                let d2 = dist(results[n - 1], results[n - 3]).log10_lm();
                let d4 = 0f64.min((d1 * d1 / d2).max(2.0 * d1).max(-(prec as f64)));
                let d4 = if d4.is_nan() { 0.0 } else { d4 };
                10f64.powi(d4.trunc() as i32)
            };
            // agreement down to the rounding noise of the sum (relative to the integral of |f|): converged in double
            // precision, where mpmath, carrying 20 more digits, goes on to its own epsilon
            if n >= 3 && dist(results[n - 1], results[n - 2]) <= 1e-14 * size
                && dist(results[n - 1], results[n - 3]) <= 1e-13 * size {
                err = err.min(eps);
            }
            if err <= eps {
                break;
            }
        }
    }
    Ok((*results.last().unwrap(), err))
}

/// mpmath's default degree for a precision (QuadratureRule.guess_degree)
fn guess_degree(prec: usize) -> usize {
    (4.0 + (prec as f64 / 30.0).log2_lm().max(0.0)) as usize + 2
}

/// mpmath.quad(f, [a, b, c, ...], error=..): the sum over consecutive points (tanh-sinh on finite pieces, mpmath's
/// default; an infinite end and method="gauss-legendre" keep the adaptive Gauss-Kronrod emulation)
fn quad_call(f: &Py, pts: &Py, kws: &[(String, Py)]) -> R {
    let xs: Vec<f64> = pts.items()?.iter().map(|x| x.f()).collect::<Result<_, _>>()?;
    if xs.len() < 2 {
        return Err(PyErr::Type("quad needs at least two points".into()));
    }
    let ts = match kw(kws, "method") {
        None => true,
        Some(Py::Str(m)) => &**m == "tanh-sinh",
        Some(_) => false,
    };
    let degree = match kw(kws, "maxdegree") {
        Some(v) if v.truthy() => v.f()? as usize,
        _ => guess_degree(crate::pympf::prec()),
    };
    let (mut re, mut im, mut err) = (0.0, 0.0, 0.0);
    for w in xs.windows(2) {
        let ((r, i), e) = if w[0] == w[1] {
            ((0.0, 0.0), 0.0)
        } else if ts && w[0].is_finite() && w[1].is_finite() {
            tanh_sinh(f, w[0], w[1], degree)?
        } else {
            integrate(f, w[0], w[1])?
        };
        re += r;
        im += i;
        err += e;
    }
    let v = if im != 0.0 { Py::Complex(re, im) } else { Py::Float(re) };
    if kw(kws, "error").map(|e| e.truthy()).unwrap_or(false) {
        return Ok(Py::tuple(vec![v, Py::Float(err)]));
    }
    Ok(v)
}

/// scipy.integrate.quad(f, a, b, points=...): (value, error estimate); the interval split at the given points
/// that lie inside it, as SciPy does
pub fn scipy_quad(f: &Py, a: f64, b: f64, points: &Py) -> R {
    let (lo, hi, sign) = if a <= b { (a, b, 1.0) } else { (b, a, -1.0) };
    let mut cuts = vec![lo];
    if !matches!(points, Py::None) {
        let mut inner: Vec<f64> = points.items()?.iter().map(|x| x.f()).collect::<Result<_, _>>()?;
        inner.retain(|x| *x > lo && *x < hi);
        inner.sort_by(f64::total_cmp);
        cuts.extend(inner);
    }
    cuts.push(hi);
    let (mut r, mut e) = (0.0, 0.0);
    for w in cuts.windows(2) {
        let ((v, _), err) = integrate(f, w[0], w[1])?;
        r += v;
        e += err;
    }
    Ok(Py::tuple(vec![Py::Float(sign * r), Py::Float(e)]))
}

// ------------------------------------------------------------------------------------------------- polynomial roots
type C = (f64, f64);

fn cmul(a: C, b: C) -> C {
    (a.0 * b.0 - a.1 * b.1, a.0 * b.1 + a.1 * b.0)
}

fn cdiv(a: C, b: C) -> Option<C> {
    crate::pycx::quot(a, b).ok()
}

fn cabs(a: C) -> f64 {
    a.0.hypot_lm(a.1)
}

fn horner(c: &[C], x: C) -> C {
    c.iter().fold((0.0, 0.0), |acc, &k| { let m = cmul(acc, x); (m.0 + k.0, m.1 + k.1) })
}

fn dhorner(c: &[C], x: C) -> C {
    let n = c.len() - 1;
    c[..n].iter().enumerate().fold((0.0, 0.0), |acc, (i, &k)| {
        let m = cmul(acc, x);
        let p = (n - i) as f64;
        (m.0 + p * k.0, m.1 + p * k.1)
    })
}

/// mpmath.polyroots(coeffs): highest degree first; the roots as mpmath orders them
pub fn polyroots(coeffs: &[Py], maxsteps: i64, error: bool) -> R {
    let mut c: Vec<C> = coeffs.iter().map(|x| match x {
        Py::Complex(r, i) => Ok((*r, *i)),
        v => v.f().map(|r| (r, 0.0)),
    }).collect::<Result<_, _>>()?;
    if c.is_empty() || c[0] == (0.0, 0.0) {
        if c.len() <= 1 {
            return Ok(Py::list(vec![]));
        }
        return Err(py::value_error("Input to polyroots must not be the zero polynomial"));
    }
    if c.len() == 1 {
        return Ok(Py::list(vec![]));
    }
    let lead = c[0];
    c = c.iter().map(|&k| cdiv(k, lead).unwrap_or(k)).collect();
    let deg = c.len() - 1;
    // mpmath's starting points (0.4 + 0.9j)^n and its Durand-Kerner sweep
    let mut roots: Vec<C> = (0..deg).map(|n| {
        let mut z = (1.0, 0.0);
        for _ in 0..n {
            z = cmul(z, (0.4, 0.9));
        }
        z
    }).collect();
    let scale = c.iter().map(|&k| cabs(k)).fold(1.0f64, f64::max);
    let mut err = vec![1.0f64; deg];
    let steps = maxsteps.max(50) as usize * 4;                       // double precision may need a few more sweeps
    for _ in 0..steps {
        let worst = err.iter().cloned().fold(0.0f64, f64::max);
        if worst < 1e-15 * scale {
            break;
        }
        for i in 0..deg {
            let p = roots[i];
            let mut x = horner(&c, p);
            for j in 0..deg {
                if i != j {
                    if let Some(q) = cdiv(x, (p.0 - roots[j].0, p.1 - roots[j].1)) {
                        x = q;
                    }
                }
            }
            roots[i] = (p.0 - x.0, p.1 - x.1);
            err[i] = cabs(x);
        }
    }
    // Newton polishing in double precision
    for r in roots.iter_mut() {
        for _ in 0..3 {
            let d = dhorner(&c, *r);
            if let Some(s) = cdiv(horner(&c, *r), d) {
                if !(s.0.is_finite() && s.1.is_finite()) {
                    break;
                }
                *r = (r.0 - s.0, r.1 - s.1);
            }
        }
    }
    if roots.iter().any(|r| !(r.0.is_finite() && r.1.is_finite())) {
        return Err(PyErr::Unsupported("polyroots did not converge".into()));
    }
    // mpmath's cleanup at its precision, here at what a double resolves for a root of this size
    let out: Vec<C> = roots.iter().map(|&r| {
        let tol = 1e-10 * cabs(r).max(1.0);
        if cabs(r) < 1e-14 {
            (0.0, 0.0)
        } else if r.1.abs() < tol {
            (r.0, 0.0)
        } else if r.0.abs() < tol {
            (0.0, r.1)
        } else {
            r
        }
    }).collect();
    let mut out = out;
    out.sort_by(|a, b| a.1.abs().total_cmp(&b.1.abs()).then(a.0.total_cmp(&b.0)));
    let vals: Vec<Py> = out.iter().map(|&(r, i)| if i == 0.0 { Py::Float(r) } else { Py::Complex(r, i) }).collect();
    if error {
        let e = err.iter().cloned().fold(f64::EPSILON, f64::max);
        return Ok(Py::tuple(vec![Py::list(vals), Py::Float(e)]));
    }
    Ok(Py::list(vals))
}

// ------------------------------------------------------------------------------------------------- incomplete gamma
/// The regularized lower function P(s, x) by its series (x < s + 1) and the upper Q(s, x) by Lentz's continued
/// fraction (otherwise): (P, Q), each computed directly so neither is a difference of nearly equal numbers.
fn pq(s: f64, x: f64) -> (f64, f64) {
    if x <= 0.0 {
        return (0.0, 1.0);
    }
    if x.is_infinite() {
        return (1.0, 0.0);
    }
    let lg = crate::pymath::lgamma(s).unwrap_or(f64::INFINITY);
    let pref = (-x + s * x.ln_lm() - lg).exp_lm();
    if x < s + 1.0 {
        let (mut ap, mut del, mut sum) = (s, 1.0 / s, 1.0 / s);
        for _ in 0..100_000 {
            ap += 1.0;
            del *= x / ap;
            sum += del;
            if del.abs() < sum.abs() * 1e-17 {
                break;
            }
        }
        let p = sum * pref;
        (p, 1.0 - p)
    } else {
        let tiny = 1e-300;
        let mut b = x + 1.0 - s;
        let mut cc = 1.0 / tiny;
        let mut d = 1.0 / b;
        let mut h = d;
        for i in 1..100_000 {
            let an = -(i as f64) * (i as f64 - s);
            b += 2.0;
            d = an * d + b;
            if d.abs() < tiny {
                d = tiny;
            }
            cc = b + an / cc;
            if cc.abs() < tiny {
                cc = tiny;
            }
            d = 1.0 / d;
            let del = d * cc;
            h *= del;
            if (del - 1.0).abs() < 1e-17 {
                break;
            }
        }
        let q = pref * h;
        (1.0 - q, q)
    }
}

/// mpmath.gammainc(z, a, b, regularized): the integral of t^(z-1) e^(-t) from a to b (divided by gamma(z) when
/// regularized), for z > 0
pub fn gammainc(z: f64, a: f64, b: f64, regularized: bool) -> Result<f64, PyErr> {
    if !(z > 0.0) {
        return Err(unsupported("gammainc of an order <= 0"));
    }
    let (pa, qa) = pq(z, a);
    let (pb, qb) = pq(z, b);
    // the smaller of the two equal differences: P(b) - P(a) = Q(a) - Q(b)
    let reg = if pa < 0.5 { pb - pa } else { qa - qb };
    if regularized {
        return Ok(reg);
    }
    Ok(reg * crate::pymath::tgamma(z).map_err(|_| PyErr::Overflow)?)
}

#[cfg(test)]
mod tests {
    //! mpmath's values at 30 digits, rounded to doubles (printed by mpmath 1.3); required to 1e-10 relative, well
    //! inside the 6-digit bar
    use super::*;

    fn close(got: f64, want: f64) {
        assert!((got - want).abs() <= 1e-10 * want.abs().max(1e-300), "{got} vs {want}");
    }

    fn func(f: fn(f64) -> f64) -> Py {
        py::func(move |a: &[Py]| Ok(Py::Float(f(a[0].f()?))))
    }

    #[test]
    fn quad_as_mpmath() {
        let q = |f: Py, a: f64, b: f64| match quad_call(&f, &Py::floats(&[a, b]), &[]).unwrap() {
            Py::Float(x) => x,
            other => panic!("{other:?}"),
        };
        close(q(func(|x| 1.0 / (0.3 * (1.0 + x).powi(3) + 0.7).sqrt()), 0.0, 2.0), 1.20947120427111);
        close(q(func(|x| 1.0 / ((1.0 + x) * (0.3 * (1.0 + x).powi(3) + 0.7).sqrt())), 1.0, f64::INFINITY),
              0.41175955086787136);
        close(q(func(|x| (-x * x).exp_lm()), f64::NEG_INFINITY, f64::INFINITY), 1.772453850905516);
        close(q(func(|x| 1.0 / x.sqrt()), 0.0, 1.0), 1.999999999469417);   // mpmath's own value
    }

    #[test]
    fn polyroots_as_mpmath() {
        let r = |c: &[f64]| py::repr(&polyroots(&c.iter().map(|x| Py::Float(*x)).collect::<Vec<_>>(), 200, false).unwrap());
        assert_eq!(r(&[1.0, -6.0, 11.0, -6.0]), "[1.0, 2.0, 3.0]");
        let Py::List(l) = polyroots(&[1.0, 0.0, 1.0, 1.0].map(Py::Float), 200, false).unwrap() else { panic!() };
        let l = l.borrow();
        close(l[0].f().unwrap(), -0.6823278038280193);
        match (&l[1], &l[2]) {
            (Py::Complex(a, b), Py::Complex(c, d)) => {
                close(*a, 0.34116390191400964);
                close(*b, -1.161541399997252);
                close(*c, 0.34116390191400964);
                close(*d, 1.161541399997252);
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn gammainc_and_besselj_as_mpmath() {
        close(gammainc(2.5, 0.0, 3.0, false).unwrap(), 0.922271212307834);
        close(gammainc(2.5, 3.0, f64::INFINITY, false).unwrap(), 0.407069175871303);
        close(gammainc(4.0, 1.0, 6.0, true).unwrap(), 0.8298079603471983);
        close(gammainc(0.5, 20.0, f64::INFINITY, false).unwrap(), 4.5013744732773784e-10);
        let j = |n: f64, x: f64| call("besselj", &[Py::Float(n), Py::Float(x)], &[]).unwrap().f().unwrap();
        close(j(0.0, 2.5), -0.048383776468198);
        close(j(3.0, 10.0), 0.058379379305186815);
        close(j(5.0, 0.5), 8.053627241357474e-06);
    }
}
