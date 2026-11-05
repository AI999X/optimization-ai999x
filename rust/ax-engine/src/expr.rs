//! Expression trees exported from sympy (tools/export_rust_pack.py), evaluated in complex double precision.

#[allow(unused_imports)]
use crate::lm::{Lm, LmF64};
use crate::num::C;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Node {
    Num(f64),
    /// an exact fraction p/q (a cube root's exponent 1/3 is computed with cbrt, not as x^0.333..)
    Frac(f64, f64),
    Cx(f64, f64),
    Sym(String),
    Add(Vec<Node>),
    Mul(Vec<Node>),
    Pow(Box<Node>, Box<Node>),
    Fun(Func, Vec<Node>),
}

#[derive(Clone, Copy, Debug)]
pub enum Func {
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Exp,
    Expm1,
    Log,
    Log1p,
    Abs,
    Sinh,
    Cosh,
    Tanh,
    /// the Lambert W function's principal real branch W0 (sympy's LambertW(x))
    LambertW0,
    /// its lower real branch W-1 (sympy's LambertW(x, -1))
    LambertWm1,
}

impl Node {
    pub fn from_json(v: &Value) -> Result<Node, String> {
        let o = v.as_object().ok_or("an expression node must be an object")?;
        if let Some(x) = o.get("n") {
            return Ok(Node::Num(x.as_f64().ok_or("bad number")?));
        }
        if let Some(k) = o.get("k").and_then(|k| k.as_str()) {
            return match k {
                "pi" => Ok(Node::Num(std::f64::consts::PI)),
                "e" => Ok(Node::Num(std::f64::consts::E)),
                other => Err(format!("unknown constant {other}")),
            };
        }
        if let Some(q) = o.get("q").and_then(|q| q.as_array()) {
            let (p, d) = (q[0].as_f64().unwrap_or(0.0), q[1].as_f64().unwrap_or(1.0));
            return Ok(Node::Frac(p, d));
        }
        if let Some(x) = o.get("c") {
            let a = x.as_array().ok_or("bad complex")?;
            return Ok(Node::Cx(a[0].as_f64().unwrap_or(0.0), a[1].as_f64().unwrap_or(0.0)));
        }
        if let Some(x) = o.get("s") {
            return Ok(Node::Sym(x.as_str().ok_or("bad symbol")?.to_string()));
        }
        let list = |x: &Value| -> Result<Vec<Node>, String> {
            x.as_array().ok_or("bad list")?.iter().map(Node::from_json).collect()
        };
        if let Some(x) = o.get("+") {
            return Ok(Node::Add(list(x)?));
        }
        if let Some(x) = o.get("*") {
            return Ok(Node::Mul(list(x)?));
        }
        if let Some(x) = o.get("^") {
            let mut l = list(x)?;
            let e = l.pop().ok_or("bad power")?;
            let b = l.pop().ok_or("bad power")?;
            return Ok(Node::Pow(Box::new(b), Box::new(e)));
        }
        if let (Some(f), Some(a)) = (o.get("f"), o.get("a")) {
            let func = match f.as_str().unwrap_or("") {
                "sin" => Func::Sin,
                "cos" => Func::Cos,
                "tan" => Func::Tan,
                "asin" => Func::Asin,
                "acos" => Func::Acos,
                "atan" => Func::Atan,
                "exp" => Func::Exp,
                "expm1" => Func::Expm1,
                "log" => Func::Log,
                "log1p" => Func::Log1p,
                "Abs" => Func::Abs,
                "sinh" => Func::Sinh,
                "cosh" => Func::Cosh,
                "tanh" => Func::Tanh,
                "LambertW" => {
                    let k = match a.as_array().map(|v| v.len()) {
                        Some(1) => 0.0,
                        Some(2) => a[1]["n"].as_f64().ok_or("bad LambertW branch")?,
                        _ => return Err("bad LambertW arguments".into()),
                    };
                    if k == 0.0 {
                        Func::LambertW0
                    } else if k == -1.0 {
                        Func::LambertWm1
                    } else {
                        return Err(format!("unsupported LambertW branch {k}"));
                    }
                }
                other => return Err(format!("unsupported function {other}")),
            };
            return Ok(Node::Fun(func, list(a)?));
        }
        Err("unknown expression node".into())
    }

    /// The value with every symbol substituted; Err when a symbol has no value.
    pub fn eval(&self, vars: &HashMap<String, f64>) -> Result<C, String> {
        Ok(match self {
            Node::Num(x) => C::r(*x),
            Node::Frac(p, q) => C::r(p / q),
            Node::Cx(a, b) => C::new(*a, *b),
            Node::Sym(s) => C::r(*vars.get(s).ok_or_else(|| format!("no value for {s}"))?),
            Node::Pow(b, e) if matches!(**e, Node::Frac(p, q) if q == 3.0 && (p == 1.0 || p == -1.0)) => {
                // a real cube root: cbrt is exact to the last bit (x^(1/3) as a power is not)
                let bv = b.eval(vars)?;
                let Node::Frac(p, _) = **e else { unreachable!() };
                if bv.im == 0.0 && bv.re >= 0.0 {
                    let r = bv.re.cbrt_lm();
                    C::r(if p > 0.0 { r } else { 1.0 / r })
                } else {
                    bv.powc(C::r(p / 3.0))
                }
            }
            Node::Add(xs) => {
                let mut acc = C::r(0.0);
                for x in xs {
                    acc = acc.add(x.eval(vars)?);
                }
                acc
            }
            Node::Mul(xs) => {
                let mut acc = C::r(1.0);
                for x in xs {
                    acc = acc.mul(x.eval(vars)?);
                }
                acc
            }
            Node::Pow(b, e) => b.eval(vars)?.powc(e.eval(vars)?),
            Node::Fun(Func::Log, a) if matches!(a[0], Node::Pow(..)) => {
                // log(b^e) = e*log(b) for a positive real base: sympy writes t = log((V0/V)**(C*R)), whose power
                // overflows a double (10^(10^7)) although the logarithm is ordinary; the reference evaluates it at
                // 17 digits with an unbounded exponent, this gives the same value without the overflow
                let Node::Pow(b, e) = &a[0] else { unreachable!() };
                let (b, e) = (b.eval(vars)?, e.eval(vars)?);
                if b.im == 0.0 && b.re > 0.0 && e.im == 0.0 {
                    C::r(e.re * b.re.ln_lm())
                } else {
                    b.powc(e).ln_lm()
                }
            }
            Node::Fun(f, a) => {
                let z = a[0].eval(vars)?;
                match f {
                    Func::Sin => z.sin_lm(),
                    Func::Cos => z.cos_lm(),
                    Func::Exp => z.exp_lm(),
                    // exp(x) - 1 without forming exp(x): exact for a tiny x
                    Func::Expm1 => {
                        if z.im == 0.0 {
                            C::r(z.re.exp_m1_lm())
                        } else {
                            let e = z.exp_lm();
                            C::new(e.re - 1.0, e.im)
                        }
                    }
                    Func::Log => z.ln_lm(),
                    // log(1 + w) without forming 1 + w (exact for a tiny w): log(V0/V) as log1p((V0 - V)/V)
                    Func::Log1p => {
                        if z.im == 0.0 && z.re > -1.0 {
                            C::r(z.re.ln_1p_lm())
                        } else {
                            C::new(1.0 + z.re, z.im).ln_lm()
                        }
                    }
                    Func::Asin => z.asin(),               // C's own (its float parts use libm)
                    Func::Acos => z.acos(),
                    Func::Abs => C::r(z.abs()),
                    Func::Tan => {
                        let (s, c) = (z.sin_lm(), z.cos_lm());
                        div(s, c)
                    }
                    Func::Atan => {
                        if z.im == 0.0 {
                            C::r(z.re.atan_lm())
                        } else {
                            return Err("complex atan".into());
                        }
                    }
                    Func::Sinh => C::r(z.re.sinh_lm()),
                    Func::Cosh => C::r(z.re.cosh_lm()),
                    Func::Tanh => C::r(z.re.tanh_lm()),
                    Func::LambertW0 | Func::LambertWm1 => {
                        // real arguments only: where W is complex the reference keeps no value either (not real)
                        if z.im.abs() > 1e-12 * 1.0f64.max(z.re.abs()) {
                            return Err("LambertW of a complex number".into());
                        }
                        C::r(lambert_w(z.re, matches!(f, Func::LambertWm1))?)
                    }
                }
            }
        })
    }
}

const EPS: f64 = f64::EPSILON; // 2.2e-16: one rounding

impl Node {
    /// log10 of the value's magnitude, computed without forming the value: V0*exp(-t/(R*C)) for t/(R*C) = 10^7 is
    /// 10^(-4.3 million), far outside double precision (1e-308 .. 1e308), yet its size is ordinary arithmetic here.
    /// None when the size cannot be known this way (a sum that does not evaluate).
    pub fn log10_abs(&self, vars: &HashMap<String, f64>) -> Option<f64> {
        let direct = |n: &Node| -> Option<f64> {
            let v = n.eval(vars).ok()?;
            (v.is_finite() && v.abs() > 0.0).then(|| v.abs().log10_lm())
        };
        match self {
            Node::Mul(xs) => xs.iter().map(|x| x.log10_abs(vars)).sum(),
            // a sum dominated by one term (T_inf + (T_i - T_inf)*exp(1.6e6)): the size of that term, when it exceeds
            // every other by more than 17 decades (the others cannot change its first digits)
            Node::Add(xs) => match direct(self) {
                Some(l) => Some(l),
                None => {
                    let ls: Vec<f64> = xs.iter().map(|x| x.log10_abs(vars)).collect::<Option<_>>()?;
                    let top = ls.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                    let rest = ls.iter().filter(|l| **l < top).cloned().fold(f64::NEG_INFINITY, f64::max);
                    (ls.iter().filter(|l| **l == top).count() == 1 && top - rest > 17.0).then_some(top)
                }
            },
            Node::Pow(b, e) => {
                let ev = e.eval(vars).ok()?;
                if ev.im != 0.0 || !ev.re.is_finite() {
                    return None;
                }
                Some(ev.re * b.log10_abs(vars)?)
            }
            Node::Fun(Func::Exp, a) => {
                let z = a[0].eval(vars).ok()?;
                (z.im == 0.0 && z.re.is_finite()).then(|| z.re / std::f64::consts::LN_10)
            }
            _ => direct(self),
        }
    }

    /// Whether `log10_abs` (and so a claim "about 1e1294", or "zero" from an underflow) can be trusted: every exponent
    /// it relies on known to within half a decade, every other factor to within half its size. A cancellation inside
    /// an exponent (log1p(x) for x = -1 + 1e-199, rounded to -1 or to -1 + 1e-16 depending on the last bit of pow)
    /// leaves the size unknown by hundreds of decades, and different maths libraries then disagree on it.
    /// The sign of a value too large or too small for a double, read the way `log10_abs` reads its size: a product
    /// multiplies its factors' signs, an exponential is positive, a sum takes the sign of its dominant term. None
    /// when unknown.
    pub fn sign_of(&self, vars: &HashMap<String, f64>) -> Option<f64> {
        let direct = |n: &Node| -> Option<f64> {
            let v = n.eval(vars).ok()?;
            (v.is_finite() && v.im == 0.0 && v.re != 0.0).then(|| v.re.signum())
        };
        match self {
            Node::Mul(xs) => xs.iter().map(|x| x.sign_of(vars)).product(),
            Node::Fun(Func::Exp, _) => Some(1.0),
            Node::Pow(b, e) => {
                let s = b.sign_of(vars)?;
                if s > 0.0 {
                    return Some(1.0);
                }
                let ev = e.eval(vars).ok()?;
                (ev.im == 0.0 && ev.re == ev.re.trunc()).then(|| if ev.re % 2.0 == 0.0 { 1.0 } else { -1.0 })
            }
            Node::Add(xs) => direct(self).or_else(|| {
                let top = self.log10_abs(vars)?;
                let i = xs.iter().position(|x| x.log10_abs(vars) == Some(top))?;
                xs[i].sign_of(vars)
            }),
            _ => direct(self),
        }
    }

    /// Whether a log1p appears anywhere in this expression (the exporter's rewrite of log(a/b) that keeps digits when
    /// a ~ b, and loses the whole value when a/b is far below 1e-16).
    pub fn has_log1p(&self) -> bool {
        match self {
            Node::Fun(Func::Log1p, _) => true,
            Node::Fun(_, a) | Node::Add(a) | Node::Mul(a) => a.iter().any(|x| x.has_log1p()),
            Node::Pow(b, e) => b.has_log1p() || e.has_log1p(),
            _ => false,
        }
    }

    pub fn size_known(&self, vars: &HashMap<String, f64>) -> bool {
        let direct = |n: &Node| -> bool {
            match n.eval_bound(vars) {
                Ok((v, e)) => v.is_finite() && e.is_finite() && (e <= 0.5 * v.abs() || (v.abs() == 0.0 && e == 0.0)),
                Err(_) => false,
            }
        };
        match self {
            Node::Mul(xs) => xs.iter().all(|x| x.size_known(vars)),
            Node::Add(xs) if !direct(self) => {
                // known through its dominant term (log10_abs), every term's own size known
                self.log10_abs(vars).is_some() && xs.iter().all(|x| x.size_known(vars))
            }
            // a size is stated to half a decade, or to 1e-9 of its exponent when that is enormous (V1*exp(1.1e17):
            // the exponent itself is known to 1e-16 relative, about 40 decades, out of 4.9e16)
            Node::Pow(b, e) => match e.eval_bound(vars) {
                Ok((ev, ee)) if ev.im == 0.0 && ev.re.is_finite() => {
                    let lb = b.log10_abs(vars).unwrap_or(0.0);
                    b.size_known(vars) && ee.is_finite() && ee * lb.abs() <= 0.5f64.max(1e-9 * (ev.re * lb).abs())
                }
                _ => false,
            },
            Node::Fun(Func::Exp, a) => match a[0].eval_bound(vars) {
                Ok((z, e)) => z.im == 0.0 && z.re.is_finite() && e.is_finite()
                    && e / std::f64::consts::LN_10 <= 0.5f64.max(1e-9 * (z.re / std::f64::consts::LN_10).abs()),
                Err(_) => false,
            },
            _ => direct(self),
        }
    }
}

impl Node {
    /// The value and a bound on its absolute rounding error (running error analysis: every input and every operation
    /// contributes its rounding, propagated by the operation's sensitivity). Used to know how many digits of an
    /// answer are guaranteed, and to judge a residual that cancellation makes impossible to measure in double
    /// precision. Err when not evaluable.
    pub fn eval_bound(&self, vars: &HashMap<String, f64>) -> Result<(C, f64), String> {
        Ok(match self {
            Node::Num(x) => (C::r(*x), EPS * x.abs()),
            Node::Frac(p, q) => (C::r(p / q), EPS * (p / q).abs()),
            Node::Cx(a, b) => (C::new(*a, *b), EPS * a.hypot_lm(*b)),
            Node::Sym(s) => {
                let v = *vars.get(s).ok_or_else(|| format!("no value for {s}"))?;
                (C::r(v), EPS * v.abs())
            }
            Node::Add(xs) => {
                let (mut acc, mut err, mut mag) = (C::r(0.0), 0.0, 0.0);
                for x in xs {
                    let (v, e) = x.eval_bound(vars)?;
                    acc = acc.add(v);
                    err += e;
                    mag += v.abs();
                }
                (acc, err + EPS * mag)
            }
            Node::Mul(xs) => {
                let (mut acc, mut rel, mut abs_terms) = (C::r(1.0), 0.0f64, 0.0f64);
                let vals: Vec<(C, f64)> = xs.iter().map(|x| x.eval_bound(vars)).collect::<Result<_, _>>()?;
                for (v, _) in &vals {
                    acc = acc.mul(*v);
                }
                for (i, (v, e)) in vals.iter().enumerate() {
                    if v.abs() > 0.0 {
                        rel += e / v.abs();
                    } else {
                        // a zero factor: its error times the other factors
                        let others: f64 = vals.iter().enumerate().filter(|(j, _)| *j != i).map(|(_, (w, _))| w.abs()).product();
                        abs_terms += e * others;
                    }
                }
                (acc, acc.abs() * (rel + EPS) + abs_terms)
            }
            Node::Pow(b, e) => {
                let ((bv, be), (ev, ee)) = (b.eval_bound(vars)?, e.eval_bound(vars)?);
                let v = bv.powc(ev);
                let mut err = EPS * v.abs();
                if ev.re < 0.0 && be >= bv.abs() {
                    // dividing by a quantity whose rounding may exceed it (sin(pi) is 1.2e-16 against an error of
                    // 7e-16): the result can be anything, a linear estimate would claim a bound it does not have
                    return Ok((v, f64::INFINITY));
                }
                if bv.abs() > 0.0 {
                    err += v.abs() * (ev.abs() * be / bv.abs() + bv.abs().ln_lm().abs() * ee);
                } else if be > 0.0 {
                    err += be.powf_lm(ev.re.max(0.0));
                }
                (v, err)
            }
            Node::Fun(f, a) => {
                let (z, e) = a[0].eval_bound(vars)?;
                let v = self.eval(vars)?;
                let slope = match f {
                    Func::Sin | Func::Cos => 1.0,
                    Func::Exp => v.abs(),
                    Func::Expm1 => (v.abs() + 1.0).max(z.re.exp_lm()),
                    Func::Log => 1.0 / z.abs(),
                    Func::Log1p => 1.0 / (1.0 + z.re).abs().max(f64::MIN_POSITIVE),
                    Func::Asin | Func::Acos => 1.0 / (1.0 - z.re * z.re).abs().sqrt().max(f64::MIN_POSITIVE),
                    Func::Atan => 1.0 / (1.0 + z.re * z.re),
                    Func::Abs => 1.0,
                    Func::Tan => 1.0 + v.abs() * v.abs(),
                    Func::Sinh | Func::Cosh => z.re.cosh_lm(),
                    Func::Tanh => 1.0,
                    // W'(z) = 1/(e^W (1 + W)), unbounded at the branch point z = -1/e
                    Func::LambertW0 | Func::LambertWm1 => 1.0 / (v.re.exp_lm() * (1.0 + v.re)).abs().max(f64::MIN_POSITIVE),
                };
                // asin/acos near +-1: the slope is unbounded but the change is not (acos(1 - d) ~ sqrt(2 d)); without
                // this cap a theta of 190 turns "satisfied" theta = acos(..) within its own error bound
                let spread = match f {
                    Func::Asin | Func::Acos => (slope * e).min((2.0 * e).sqrt()),
                    // near z = -1/e, W = -1 +- sqrt(2 (e z + 1)): a change d in z moves W by at most sqrt(2 e d)
                    Func::LambertW0 | Func::LambertWm1 => (slope * e).min((2.0 * std::f64::consts::E * e).sqrt()),
                    _ => slope * e,
                };
                (v, spread + 2.0 * EPS * v.abs())
            }
        })
    }
}

/// The Lambert W function of a real x (w*e^w = x): the principal branch W0 (x >= -1/e, w >= -1) or, with `lower`, the
/// branch W-1 (-1/e <= x < 0, w <= -1), as sympy's LambertW(x) and LambertW(x, -1). Err where the branch has no real
/// value (the reference's value is complex there and is not kept). A start from the branch-point series near -1/e,
/// from log(1 + x) or the asymptotic log x - log log x elsewhere, then at most 64 Halley steps.
pub fn lambert_w(x: f64, lower: bool) -> Result<f64, String> {
    const E_HI: f64 = std::f64::consts::E;
    const E_LO: f64 = 1.445_646_891_729_250_2e-16;         // e - E_HI
    if x.is_nan() {
        return Err("LambertW of NaN".into());
    }
    // e*x + 1, the distance from the branch point, without losing e's last digits
    let q = E_HI * x + 1.0 + E_LO * x;
    if q < 0.0 {
        // just below -1/e by rounding of the argument (x*e^x for x = -1 computed in double precision): the branch point
        if q >= -8.0 * f64::EPSILON {
            return Ok(-1.0);
        }
        return Err("LambertW below -1/e has no real value".into());
    }
    if lower && x >= 0.0 {
        return Err("the lower branch of LambertW is real only for -1/e <= x < 0".into());
    }
    if x == 0.0 {
        return Ok(0.0);
    }
    if x.is_infinite() {
        return Ok(f64::INFINITY);
    }
    let p = (2.0 * q).sqrt() * if lower { -1.0 } else { 1.0 };
    let series = |p: f64| -1.0 + p - p * p / 3.0 + 11.0 / 72.0 * p * p * p - 43.0 / 540.0 * p * p * p * p;
    if p.abs() < 1e-3 {
        return Ok(series(p));                             // the series is exact to double precision there
    }
    let mut w = if p.abs() < 0.5 {
        series(p)
    } else if !lower && x < 3.0 {
        (1.0 + x).ln_lm()
    } else if !lower {
        let l1 = x.ln_lm();
        let l2 = l1.ln_lm();
        l1 - l2 + l2 / l1
    } else {
        let l1 = (-x).ln_lm();
        let l2 = (-l1).ln_lm();
        l1 - l2 + l2 / l1
    };
    for _ in 0..64 {
        let ew = w.exp_lm();
        let f = w * ew - x;
        let wp1 = w + 1.0;
        let denom = ew * wp1 - (w + 2.0) * f / (2.0 * wp1);
        if !denom.is_finite() || denom == 0.0 {
            break;
        }
        let step = f / denom;
        w -= step;
        if step.abs() <= 4.0 * f64::EPSILON * (1.0 + w.abs()) {
            return Ok(w);
        }
    }
    // not settled: accept only a w that solves w*e^w = x to near double precision
    let r = w * w.exp_lm() - x;
    if w.is_finite() && r.abs() <= 1e-13 * x.abs().max(1e-300) {
        Ok(w)
    } else {
        Err("LambertW did not converge".into())
    }
}

fn div(a: C, b: C) -> C {
    let d = b.re * b.re + b.im * b.im;
    C::new((a.re * b.re + a.im * b.im) / d, (a.im * b.re - a.re * b.im) / d)
}

#[cfg(test)]
mod tests {
    use super::lambert_w;
    use crate::lm::Lm;

    #[test]
    fn lambert_w_branches() {
        // mpmath.lambertw at 30 digits
        let cases = [(1.0, false, 0.567_143_290_409_783_8), (10.0, false, 1.745_528_002_740_699_4),
                     (1e100, false, 224.843_106_445_118_5), (-0.2, false, -0.259_171_101_819_073_7),
                     (-0.3678, false, -0.979_360_714_957_830_5), (1e-10, false, 9.999_999_999e-11),
                     (-0.2, true, -2.542_641_357_773_526_4), (-0.3678, true, -1.020_927_239_409_425_5),
                     (-1e-10, true, -26.295_238_819_246_93)];
        for (x, lower, want) in cases {
            let w = lambert_w(x, lower).unwrap();
            assert!((w - want).abs() <= 1e-12 * want.abs().max(1.0), "W({x}, {lower}) = {w}, want {want}");
            assert!((w * w.exp_lm() - x).abs() <= 1e-13 * x.abs().max(1e-300) + 1e-15);
        }
        let b = -1.0 / std::f64::consts::E;
        assert!((lambert_w(b, false).unwrap() + 1.0).abs() < 1e-7);
        assert!((lambert_w(b, true).unwrap() + 1.0).abs() < 1e-7);
        assert!(lambert_w(-0.5, false).is_err());
        assert!(lambert_w(0.5, true).is_err());
        assert!(lambert_w(f64::NAN, false).is_err());
    }
}
