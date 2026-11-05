//! Python's math.gamma and math.lgamma as CPython 3.13 computes them (Modules/mathmodule.c: m_tgamma, m_lgamma, a
//! 13-term Lanczos sum with its exact table of small factorials and m_sinpi), and math.erf / math.erfc, which CPython
//! takes from the C library: here the `libm` crate (a pure-Rust port of musl's fdlibm, accurate to an ulp). The
//! rough approximations these replace lost digits near lgamma's zeros (x = 1, 2) and in erf near |x| = 3.

#[allow(unused_imports)]
use crate::lm::{Lm, LmF64};
/// Why a value has no answer: Python's ValueError("math domain error") or OverflowError("math range error").
#[derive(Debug, PartialEq)]
pub enum MathErr {
    Domain,
    Range,
}

const LANCZOS_N: usize = 13;
const LANCZOS_G: f64 = 6.024680040776729583740234375;
const LANCZOS_G_MINUS_HALF: f64 = 5.524680040776729583740234375;
const LANCZOS_NUM: [f64; LANCZOS_N] = [
    23531376880.410759688572007674451636754734846804940,
    42919803642.649098768957899047001988850926355848959,
    35711959237.355668049440185451547166705960488635843,
    17921034426.037209699919755754458931112671403265390,
    6039542586.3520280050642916443072979210699388420708,
    1439720407.3117216736632230727949123939715485786772,
    248874557.86205415651146038641322942321632125127801,
    31426415.585400194380614231628318205362874684987640,
    2876370.6289353724412254090516208496135991145378768,
    186056.26539522349504029498971604569928220784236328,
    8071.6720023658162106380029022722506138218516325024,
    210.82427775157934587250973392071336271166969580291,
    2.5066282746310002701649081771338373386264310793408,
];
const LANCZOS_DEN: [f64; LANCZOS_N] = [
    0.0, 39916800.0, 120543840.0, 150917976.0, 105258076.0, 45995730.0, 13339535.0, 2637558.0, 357423.0, 32670.0,
    1925.0, 66.0, 1.0,
];
/// gamma(1) .. gamma(23): the factorials 0! .. 22!, exact in a double
const GAMMA_INTEGRAL: [f64; 23] = [
    1.0, 1.0, 2.0, 6.0, 24.0, 120.0, 720.0, 5040.0, 40320.0, 362880.0, 3628800.0, 39916800.0, 479001600.0,
    6227020800.0, 87178291200.0, 1307674368000.0, 20922789888000.0, 355687428096000.0, 6402373705728000.0,
    121645100408832000.0, 2432902008176640000.0, 51090942171709440000.0, 1124000727777607680000.0,
];

fn lanczos_sum(x: f64) -> f64 {
    let (mut num, mut den) = (0.0, 0.0);
    if x < 5.0 {
        for i in (0..LANCZOS_N).rev() {
            num = num * x + LANCZOS_NUM[i];
            den = den * x + LANCZOS_DEN[i];
        }
    } else {
        for i in 0..LANCZOS_N {
            num = num / x + LANCZOS_NUM[i];
            den = den / x + LANCZOS_DEN[i];
        }
    }
    num / den
}

/// sin(pi * x), exact at the integers and half-integers (m_sinpi)
fn sinpi(x: f64) -> f64 {
    use std::f64::consts::PI;
    let y = x.abs() % 2.0;
    let n = (2.0 * y).round() as i64;
    let r = match n {
        0 => (PI * y).sin_lm(),
        1 => (PI * (y - 0.5)).cos_lm(),
        2 => (PI * (1.0 - y)).sin_lm(),
        3 => -(PI * (y - 1.5)).cos_lm(),
        4 => (PI * (y - 2.0)).sin_lm(),
        _ => f64::NAN,
    };
    1.0f64.copysign(x) * r
}

/// math.gamma (m_tgamma)
pub fn tgamma(x: f64) -> Result<f64, MathErr> {
    if !x.is_finite() {
        if x.is_nan() || x > 0.0 {
            return Ok(x);
        }
        return Err(MathErr::Domain);
    }
    if x == 0.0 {
        return Err(MathErr::Domain);
    }
    if x == x.floor() {
        if x < 0.0 {
            return Err(MathErr::Domain);
        }
        if x <= GAMMA_INTEGRAL.len() as f64 {
            return Ok(GAMMA_INTEGRAL[x as usize - 1]);
        }
    }
    let absx = x.abs();
    if absx < 1e-20 {
        let r = 1.0 / x;
        return if r.is_infinite() { Err(MathErr::Range) } else { Ok(r) };
    }
    if absx > 200.0 {
        return if x < 0.0 { Ok(0.0 / sinpi(x)) } else { Err(MathErr::Range) };
    }
    let y = absx + LANCZOS_G_MINUS_HALF;
    let z = if absx > LANCZOS_G_MINUS_HALF {
        let q = y - absx;
        q - LANCZOS_G_MINUS_HALF
    } else {
        let q = y - LANCZOS_G_MINUS_HALF;
        q - absx
    };
    let z = z * LANCZOS_G / y;
    let mut r;
    if x < 0.0 {
        r = -std::f64::consts::PI / sinpi(absx) / absx * y.exp_lm() / lanczos_sum(absx);
        r -= z * r;
        if absx < 140.0 {
            r /= y.powf_lm(absx - 0.5);
        } else {
            let sqrtpow = y.powf_lm(absx / 2.0 - 0.25);
            r /= sqrtpow;
            r /= sqrtpow;
        }
    } else {
        r = lanczos_sum(absx) / y.exp_lm();
        r += z * r;
        if absx < 140.0 {
            r *= y.powf_lm(absx - 0.5);
        } else {
            let sqrtpow = y.powf_lm(absx / 2.0 - 0.25);
            r *= sqrtpow;
            r *= sqrtpow;
        }
    }
    if r.is_infinite() {
        return Err(MathErr::Range);
    }
    Ok(r)
}

/// math.lgamma (m_lgamma)
pub fn lgamma(x: f64) -> Result<f64, MathErr> {
    if !x.is_finite() {
        return Ok(if x.is_nan() { x } else { f64::INFINITY });
    }
    if x == x.floor() && x <= 2.0 {
        return if x <= 0.0 { Err(MathErr::Domain) } else { Ok(0.0) };
    }
    let absx = x.abs();
    if absx < 1e-20 {
        return Ok(-absx.ln_lm());
    }
    let mut r = lanczos_sum(absx).ln_lm() - LANCZOS_G;
    r += (absx - 0.5) * ((absx + LANCZOS_G - 0.5).ln_lm() - 1.0);
    if x < 0.0 {
        r = std::f64::consts::PI.ln_lm() - sinpi(absx).abs().ln_lm() - absx.ln_lm() - r;
    }
    if r.is_infinite() {
        return Err(MathErr::Range);
    }
    Ok(r)
}

/// statistics._normal_dist_inv_cdf (Wichura's AS241, as CPython 3.13: coefficients copied from Lib/statistics.py)
pub fn normal_inv_cdf(p: f64, mu: f64, sigma: f64) -> f64 {
    let q = p - 0.5;
    if q.abs() <= 0.425 {
        let r = 0.180625 - q * q;
        let num = (((((((2.5090809287301226727e+3 * r + 3.3430575583588128105e+4) * r + 6.7265770927008700853e+4)
            * r + 4.5921953931549871457e+4) * r + 1.3731693765509461125e+4) * r + 1.9715909503065514427e+3) * r
            + 1.3314166789178437745e+2) * r + 3.3871328727963666080e+0) * q;
        let den = ((((((5.2264952788528545610e+3 * r + 2.8729085735721942674e+4) * r + 3.9307895800092710610e+4)
            * r + 2.1213794301586595867e+4) * r + 5.3941960214247511077e+3) * r + 6.8718700749205790830e+2) * r
            + 4.2313330701600911252e+1) * r + 1.0;
        let x = num / den;
        return mu + (x * sigma);
    }
    let r = if q <= 0.0 { p } else { 1.0 - p };
    let r = (-r.ln_lm()).sqrt();
    let (num, den) = if r <= 5.0 {
        let r = r - 1.6;
        ((((((((7.74545014278341407640e-4 * r + 2.27238449892691845833e-2) * r + 2.41780725177450611770e-1) * r
            + 1.27045825245236838258e+0) * r + 3.64784832476320460504e+0) * r + 5.76949722146069140550e+0) * r
            + 4.63033784615654529590e+0) * r + 1.42343711074968357734e+0),
         (((((((1.05075007164441684324e-9 * r + 5.47593808499534494600e-4) * r + 1.51986665636164571966e-2) * r
            + 1.48103976427480074590e-1) * r + 6.89767334985100004550e-1) * r + 1.67638483018380384940e+0) * r
            + 2.05319162663775882187e+0) * r + 1.0))
    } else {
        let r = r - 5.0;
        ((((((((2.01033439929228813265e-7 * r + 2.71155556874348757815e-5) * r + 1.24266094738807843860e-3) * r
            + 2.65321895265761230930e-2) * r + 2.96560571828504891230e-1) * r + 1.78482653991729133580e+0) * r
            + 5.46378491116411436990e+0) * r + 6.65790464350110377720e+0),
         (((((((2.04426310338993978564e-15 * r + 1.42151175831644588870e-7) * r + 1.84631831751005468180e-5) * r
            + 7.86869131145613259100e-4) * r + 1.48753612908506148525e-2) * r + 1.36929880922735805310e-1) * r
            + 5.99832206555887937690e-1) * r + 1.0))
    };
    let x = num / den;
    let x = if q < 0.0 { -x } else { x };
    mu + (x * sigma)
}

pub fn erf(x: f64) -> f64 {
    libm::erf(x)
}

pub fn erfc(x: f64) -> f64 {
    libm::erfc(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// AX_MATH_SWEEP=<file> of lines "fn x expected" written by Python (repr floats, "err" for an exception): every
    /// value within 4 units in the last place of Python's (61,876 points, 2026-10-29: 42 differ, by 1-2 ulp), the
    /// same exceptions where Python raises.
    #[test]
    fn sweep_against_python() {
        let Ok(path) = std::env::var("AX_MATH_SWEEP") else { return };
        let text = std::fs::read_to_string(path).unwrap();
        let (mut n, mut bad, mut ulps_max, mut differ) = (0, vec![], 0u64, 0);
        for line in text.lines() {
            let p: Vec<&str> = line.split_whitespace().collect();
            let x: f64 = p[1].parse().unwrap();
            let got = match p[0] {
                "gamma" => tgamma(x).ok(),
                "lgamma" => lgamma(x).ok(),
                "erf" => Some(erf(x)),
                _ => Some(erfc(x)),
            };
            n += 1;
            let ok = match (got, p[2]) {
                (None, "err") => true,
                (Some(g), e) if e != "err" => {
                    let w: f64 = e.parse().unwrap();
                    if g == w || (g.is_nan() && w.is_nan()) {
                        true
                    } else {
                        // units in the last place between two finite doubles of the same sign
                        let u = if g.is_finite() && w.is_finite() && g.signum() == w.signum() {
                            (g.to_bits() as i64 - w.to_bits() as i64).unsigned_abs()
                        } else {
                            u64::MAX
                        };
                        differ += 1;
                        ulps_max = ulps_max.max(u);
                        u <= 4
                    }
                }
                _ => false,
            };
            if !ok {
                bad.push(format!("{} {} want {} got {:?}", p[0], p[1], p[2], got));
            }
        }
        println!("{n} points, {differ} differ from Python, by at most {ulps_max} ulp; {} beyond 4 ulp", bad.len());
        for b in bad.iter().take(15) {
            println!("  {b}");
        }
        assert!(bad.is_empty());
    }

    #[test]
    fn as_python_computes_them() {
        // values printed by Python 3.13: math.gamma(0.5), math.lgamma(0.5), math.gamma(5.5), math.lgamma(100.25),
        // math.lgamma(-2.5), math.gamma(-0.5), math.erf(0.5), math.erfc(3.0)
        assert_eq!(tgamma(0.5).unwrap(), 1.7724538509055159);
        assert_eq!(tgamma(5.5).unwrap(), 52.34277778455352);
        assert_eq!(tgamma(-0.5).unwrap(), -3.544907701811032);
        assert_eq!(tgamma(5.0).unwrap(), 24.0);
        assert_eq!(lgamma(1.0).unwrap(), 0.0);
        assert_eq!(lgamma(2.0).unwrap(), 0.0);
        assert!(lgamma(0.0).is_err() && tgamma(-3.0).is_err() && tgamma(172.0).is_err());
        assert_eq!(lgamma(0.5).unwrap(), 0.5723649429247004);
        assert_eq!(lgamma(100.25).unwrap(), 360.2845596377643);
        assert_eq!(lgamma(-2.5).unwrap(), -0.05624371649767457);
        let close = |a: f64, b: f64| ((a - b) / b).abs() < 2e-16;
        assert!(close(erf(0.5), 0.5204998778130465));
        assert!(close(erfc(3.0), 2.209049699858544e-05));
        // statistics.NormalDist(...).inv_cdf in Python 3.13: the central, tail and far-tail branches of AS241
        for (p, mu, s, want) in [(0.975, 0.0, 1.0, 1.9599639845400536), (0.5, 0.0, 1.0, 0.0),
                                 (0.3, 0.0, 1.0, -0.5244005127080407), (1e-10, 0.0, 1.0, -6.361340902404056),
                                 (1e-300, 0.0, 1.0, -37.0470962993612), (0.9999, 0.0, 1.0, 3.7190164854557084),
                                 (0.1, 3.0, 2.0, 0.4368968689107984)] {
            assert_eq!(normal_inv_cdf(p, mu, s), want, "inv_cdf({p})");
        }
    }
}
