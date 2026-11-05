//! Materials science, written by hand (see handwritten/mod.rs): the Debye heat capacity, whose Python integrates
//! x^4 e^-x / (1 - e^-x)^2 with mpmath's tanh-sinh rule at 30 digits. In double precision 1 - e^-x is 0 at the rule's
//! nodes next to x = 0 (they reach 1e-34 of the interval), so the translated procedure divided by zero on every input;
//! here the integral is computed in the stable form x^4 e^-x / expm1(-x)^2 by Gauss-Legendre panels, and the one region
//! where the package itself divides by zero (theta_D/T below 3.3713e-5: there mpmath's 30-digit e^-x rounds to 1 at its
//! node nearest 0) gives the same ZeroDivisionError.

use super::arg;
use crate::lm::{Lm, LmF64};
use crate::py::{value_error, Py, PyErr, R};

pub fn helper(name: &str, a: &[Py], _kws: &[(String, Py)]) -> Option<R> {
    match name {
        "debye_heat_capacity" => Some(debye_heat_capacity(arg(a, 0), arg(a, 1))),
        _ => None,
    }
}

const NA: f64 = 6.02214076e23;
const KB: f64 = 1.380649e-23;
/// mpmath.quad at workdps(30) raises ZeroDivisionError for theta_D/T below this (bisected on the package to adjacent
/// doubles, and checked on 3,000 random points around it: no other failing point; every xD above it solves)
const PY_ZERO_DIVISION_BELOW: f64 = 3.371339845587423e-05;

/// _pos(name, *vals): each a finite positive int or float (a bool is an int in Python)
fn pos(name: &str, vals: &[&Py]) -> Result<Vec<f64>, PyErr> {
    let mut out = Vec::with_capacity(vals.len());
    for v in vals {
        let x = match v {
            Py::Bool(b) => *b as i64 as f64,
            Py::Int(_) | Py::Big(_) | Py::Float(_) => v.f()?,
            _ => f64::NAN,
        };
        if !(x.is_finite() && x > 0.0) {
            return Err(value_error(format!("{name} must be a positive number")));
        }
        out.push(x);
    }
    Ok(out)
}

/// The 20-point Gauss-Legendre rule on [-1, 1] (Newton on P_20 from the Chebyshev guesses; the same lm functions
/// on every platform, so the browser and native builds get the same nodes)
fn gauss_legendre_20() -> [(f64, f64); 20] {
    const N: usize = 20;
    let mut rule = [(0.0, 0.0); N];
    for i in 0..N / 2 {
        let mut x = (std::f64::consts::PI * (i as f64 + 0.75) / (N as f64 + 0.5)).cos_lm();
        let mut dp = 1.0;
        for _ in 0..100 {
            let (mut p0, mut p1) = (1.0, x);
            for k in 2..=N {
                let p2 = ((2 * k - 1) as f64 * x * p1 - (k - 1) as f64 * p0) / k as f64;
                p0 = p1;
                p1 = p2;
            }
            dp = N as f64 * (x * p1 - p0) / (x * x - 1.0);
            let dx = p1 / dp;
            x -= dx;
            if dx.abs() <= 1e-16 {
                break;
            }
        }
        let w = 2.0 / ((1.0 - x * x) * dp * dp);
        rule[i] = (x, w);
        rule[N - 1 - i] = (-x, w);
    }
    rule
}

/// int_0^xD x^4 e^x / (e^x - 1)^2 dx for 0 < xD <= 300: panels of width at most 1 (the integrand is analytic, its
/// nearest singularities at +-2 pi i), 20 Gauss-Legendre points each: about 1e-16 relative
fn debye_integral(xd: f64) -> f64 {
    let rule = gauss_legendre_20();
    let n = xd.ceil().max(1.0) as usize;                     // xD <= 300 is checked first: at most 300 panels
    let h = xd / n as f64;
    let g = |x: f64| {
        let e = (-x).exp_m1_lm();
        x.powf_lm(4.0) * (-x).exp_lm() / (e * e)
    };
    let mut total = 0.0;
    for p in (0..n).rev() {                                  // small terms (the far tail) first
        let (lo, hi) = (p as f64 * h, if p + 1 == n { xd } else { (p + 1) as f64 * h });
        let (c, d) = ((hi - lo) / 2.0, (hi + lo) / 2.0);
        let s: f64 = rule.iter().map(|&(x, w)| w * g(d + c * x)).sum();
        total += c * s;
    }
    total
}

fn debye_heat_capacity(theta_d: &Py, t: &Py) -> R {
    let v = pos("the Debye temperature and the temperature", &[theta_d, t])?;
    let (theta_d, t) = (v[0], v[1]);
    let xd = theta_d / t;
    if xd > 300.0 {
        return Err(value_error("T is below theta_D/300: use the low-temperature T^3 law"));
    }
    if xd < PY_ZERO_DIVISION_BELOW {
        return Err(PyErr::ZeroDivision);
    }
    let i = debye_integral(xd);
    let rgas = NA * KB;
    let cv = 9.0 * rgas * (t / theta_d).powf_lm(3.0) * i;
    let r12 = crate::pyfmt::r12;
    Ok(Py::dict(vec![(Py::str("Cv"), Py::Float(r12(cv))), (Py::str("Cv_over_3R"), Py::Float(r12(cv / (3.0 * rgas))))]))
}
