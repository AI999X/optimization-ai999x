//! Limits by asymptotic expansion, for sympy's limit() in the calculus procedures: at infinity (x = 1/t) and from
//! one side of a point (x = a + t or a - t), t -> 0+. An expression is expanded as a Puiseux series in t with exact
//! coefficients (rational exponents; roots, exp, log, the trigonometric and hyperbolic functions composed through their
//! exact Taylor series), or falls into a class beyond powers: exponentially small (exp(-1/t), smaller than any power),
//! exponentially large (exp(1/t), with its sign), or oscillating (sin(1/t)). The leading behaviour gives the limit:
//! a finite exact value, an infinity (sympy's oo, -oo or zoo, all refused as infinite by the package) or an
//! oscillation (sympy's AccumBounds, refused as undetermined). What it cannot decide exactly it does not answer.

use crate::sym::{self, Ex, Fun};
use num_bigint::BigInt;
use num_rational::BigRational as Q;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::BTreeMap;
use std::rc::Rc;

/// Why no limit is given
#[derive(Debug, Clone, PartialEq)]
pub enum Undecided {
    /// beyond what the expansion models (a log of t, a product of exponentially large and small parts, ...)
    NotModelled(&'static str),
}

type Res<T> = Result<T, Undecided>;

fn nm<T>(why: &'static str) -> Res<T> {
    Err(Undecided::NotModelled(why))
}

/// A Puiseux series in t: exact coefficients for every exponent below `top` (terms at or above `top` are unknown;
/// `top` = None means the sum is exact).
#[derive(Clone, Debug)]
struct Ser {
    terms: BTreeMap<Q, Ex>,
    top: Option<Q>,
}

/// The asymptotic class of an expression as t -> 0+
#[derive(Clone, Debug)]
enum A {
    S(Ser),
    /// tends to 0 faster than any power of t
    Small,
    /// tends to +inf (1) or -inf (-1) faster than any power of t
    Large(i32),
    /// oscillates (bounded or not) without a limit
    Osc,
}

fn qi(n: i64) -> Q {
    Q::from_integer(BigInt::from(n))
}

/// How many orders beyond the leading exponent a series keeps (enough for the cancellations of the procedures'
/// inputs, such as sqrt(x^2 + k x) - x)
const ORDER: i64 = 8;

fn is_zero_ex(c: &Ex) -> Res<bool> {
    if sym::is_zero(c) {
        return Ok(true);
    }
    if let Ex::Num(_) = c {
        return Ok(false);
    }
    match crate::pysym::eval_mp(c, &|_| None) {
        Ok(v) if v != 0.0 && v.abs() > 1e-25 => Ok(false),
        Ok(v) if v.is_nan() => nm("a coefficient is not a number"),
        _ => nm("a coefficient is not decidably zero or nonzero"),
    }
}

fn sign_ex(c: &Ex) -> Res<i32> {
    if let Ex::Num(q) = c {
        return Ok(if q.is_positive() { 1 } else if q.is_negative() { -1 } else { 0 });
    }
    match crate::pysym::eval_mp(c, &|_| None) {
        Ok(v) if v > 1e-25 => Ok(1),
        Ok(v) if v < -1e-25 => Ok(-1),
        _ => nm("the sign of a coefficient is not decidable"),
    }
}

impl Ser {
    fn exact(terms: BTreeMap<Q, Ex>) -> Ser {
        Ser { terms: terms.into_iter().filter(|(_, c)| !sym::is_zero(c)).collect(), top: None }
    }
    fn constant(c: Ex) -> Ser {
        let mut m = BTreeMap::new();
        m.insert(Q::zero(), c);
        Ser::exact(m)
    }
    fn monomial(e: Q, c: Ex) -> Ser {
        let mut m = BTreeMap::new();
        m.insert(e, c);
        Ser::exact(m)
    }
    /// drop exact zeros and terms at or above top
    fn clean(mut self) -> Res<Ser> {
        let mut out = BTreeMap::new();
        for (e, c) in self.terms {
            if let Some(t) = &self.top {
                if &e >= t {
                    continue;
                }
            }
            if !is_zero_ex(&c)? {
                out.insert(e, c);
            }
        }
        self.terms = out;
        Ok(self)
    }
    /// the leading exponent and coefficient; None for a series that is zero as far as it is known
    fn lead(&self) -> Option<(Q, Ex)> {
        self.terms.iter().next().map(|(e, c)| (e.clone(), c.clone()))
    }
    fn min_top(a: &Option<Q>, b: &Option<Q>) -> Option<Q> {
        match (a, b) {
            (None, x) | (x, None) => x.clone(),
            (Some(x), Some(y)) => Some(x.min(y).clone()),
        }
    }
    fn add(&self, o: &Ser) -> Res<Ser> {
        let mut m = self.terms.clone();
        for (e, c) in &o.terms {
            let v = match m.remove(e) {
                Some(x) => sym::add(x, c.clone()),
                None => c.clone(),
            };
            m.insert(e.clone(), v);
        }
        Ser { terms: m, top: Ser::min_top(&self.top, &o.top) }.clean()
    }
    fn neg(&self) -> Ser {
        Ser { terms: self.terms.iter().map(|(e, c)| (e.clone(), sym::neg(c.clone()))).collect(), top: self.top.clone() }
    }
    fn mul(&self, o: &Ser) -> Res<Ser> {
        let (Some((va, _)), Some((vb, _))) = (self.lead(), o.lead()) else {
            // a factor known to be zero below its top: the product is known to be zero below va + vb tops
            let top = match (self.lead(), o.lead()) {
                (None, Some((vb, _))) => self.top.as_ref().map(|t| t + vb),
                (Some((va, _)), None) => o.top.as_ref().map(|t| t + va),
                _ => match (&self.top, &o.top) { (Some(a), Some(b)) => Some(a + b), _ => None },
            };
            return Ok(Ser { terms: BTreeMap::new(), top: Some(top.unwrap_or_else(Q::zero)) });
        };
        let top = Ser::min_top(&self.top.as_ref().map(|t| t + &vb), &o.top.as_ref().map(|t| t + &va));
        let cap = &va + &vb + qi(ORDER);
        let top = Ser::min_top(&top, &Some(cap));
        let mut m: BTreeMap<Q, Ex> = BTreeMap::new();
        for (ea, ca) in &self.terms {
            for (eb, cb) in &o.terms {
                let e = ea + eb;
                if top.as_ref().is_some_and(|t| &e >= t) {
                    continue;
                }
                let p = sym::mul(ca.clone(), cb.clone());
                let v = match m.remove(&e) {
                    Some(x) => sym::add(x, p),
                    None => p,
                };
                m.insert(e, v);
            }
        }
        Ser { terms: m, top }.clean()
    }
    /// s = c t^v (1 + u): (c, v, u) with u of positive exponents
    fn split(&self) -> Res<(Ex, Q, Ser)> {
        let Some((v, c)) = self.lead() else { return nm("a leading term that cancels beyond the kept order") };
        let inv_c = sym::pow(c.clone(), sym::int(-1));
        let mut m = BTreeMap::new();
        for (e, k) in self.terms.iter().skip(1) {
            m.insert(e - &v, sym::mul(k.clone(), inv_c.clone()));
        }
        let top = self.top.as_ref().map(|t| t - &v);
        Ok((c, v, Ser { terms: m, top }))
    }
    /// sum over k of coef[k] u^k for u of positive exponents (a Taylor series in u), to the kept order
    fn compose(coef: &dyn Fn(usize) -> Ex, u: &Ser) -> Res<Ser> {
        let mut out = Ser::constant(coef(0));
        let Some((vu, _)) = u.lead() else {
            // u is zero below its top: the series is the constant term, known below that top
            out.top = u.top.clone();
            return Ok(out);
        };
        if !vu.is_positive() {
            return nm("a composition with a term that does not vanish");
        }
        let kmax = (qi(ORDER) / &vu).ceil().to_integer().to_usize().unwrap_or(64).min(64) + 1;
        let mut pw = Ser::constant(sym::int(1));
        for k in 1..=kmax {
            pw = pw.mul(u)?;
            let c = coef(k);
            if sym::is_zero(&c) {
                continue;
            }
            let term = pw.mul(&Ser::constant(c))?;
            out = out.add(&term)?;
        }
        // the first omitted term bounds what is known
        let cut = &vu * qi(kmax as i64 + 1);
        out.top = Ser::min_top(&out.top, &Some(cut));
        out.clean()
    }
    fn inv(&self) -> Res<Ser> {
        let (c, v, u) = self.split()?;
        // 1/(c t^v (1+u)) = c^-1 t^-v sum (-u)^k
        let s = Ser::compose(&|k| sym::int(if k % 2 == 0 { 1 } else { -1 }), &u)?;
        s.mul(&Ser::monomial(-v, sym::pow(c, sym::int(-1))))
    }
    fn powq(&self, r: &Q) -> Res<Ser> {
        let (c, v, u) = self.split()?;
        if !r.is_integer() && sign_ex(&c)? < 0 {
            return nm("a fractional power of a negative leading coefficient");
        }
        // (1+u)^r = sum binom(r, k) u^k
        let rr = r.clone();
        let s = Ser::compose(&move |k| {
            let mut b = Q::one();
            for j in 0..k {
                b = b * (&rr - qi(j as i64)) / qi(j as i64 + 1);
            }
            sym::num(b)
        }, &u)?;
        s.mul(&Ser::monomial(&v * r, sym::pow(c, sym::num(r.clone()))))
    }
}

/// f(c + w) as a series in w: f^(k)(c)/k! from exact derivatives (None where f is not analytic at c)
fn taylor_at(f: Fun, c: &Ex, w: &Ser) -> Res<Ser> {
    let z: Rc<str> = Rc::from("_w");
    let zx = Ex::Sym(z.clone());
    let mut d = sym::func(f, zx.clone());
    let mut coefs = vec![];
    let mut fact = Q::one();
    for k in 0..=64usize {
        if k > 0 {
            d = sym::diff(&d, &z);
            fact *= qi(k as i64);
        }
        let v = sym::subs(&d, &z, c);
        if sym::has_undefined(&v) {
            return nm("a function evaluated where it is not analytic");
        }
        coefs.push(sym::mul(v, sym::num(Q::one() / &fact)));
        if k as i64 > ORDER + 2 {
            break;
        }
    }
    let n = coefs.len();
    Ser::compose(&move |k| if k < n { coefs[k].clone() } else { sym::int(0) }, w)
}

fn is_const(e: &Ex, x: &str) -> bool {
    let mut v = vec![];
    sym::free_symbols(e, &mut v);
    !v.iter().any(|s| &**s == x)
}

/// The class of e as t -> 0+ where x = sub (a series in t)
fn expand(e: &Ex, x: &str, sub: &Ser) -> Res<A> {
    if is_const(e, x) {
        return Ok(A::S(Ser::constant(e.clone())));
    }
    match e {
        Ex::Sym(_) => Ok(A::S(sub.clone())),
        Ex::Add(ts) => {
            let mut acc = A::S(Ser::constant(sym::int(0)));
            for t in ts.iter() {
                acc = add(acc, expand(t, x, sub)?)?;
            }
            Ok(acc)
        }
        Ex::Mul(fs) => {
            let mut acc = A::S(Ser::constant(sym::int(1)));
            for f in fs.iter() {
                acc = mul(acc, expand(f, x, sub)?)?;
            }
            Ok(acc)
        }
        Ex::Pow(b, p) => {
            let base = expand(b, x, sub)?;
            if let Ex::Num(r) = &**p {
                return pow_q(base, r);
            }
            // b^p = exp(p log b)
            let lb = log(base)?;
            let pe = expand(p, x, sub)?;
            exp(mul(pe, lb)?)
        }
        Ex::Fun(f, a) => {
            let arg = expand(a, x, sub)?;
            fun(*f, arg)
        }
        _ => nm("an undefined value"),
    }
}


fn add(a: A, b: A) -> Res<A> {
    Ok(match (a, b) {
        (A::S(x), A::S(y)) => A::S(x.add(&y)?),
        (A::S(x), A::Small) | (A::Small, A::S(x)) => {
            if x.lead().is_none() && x.top.is_none() { A::Small } else { A::S(x) }
        }
        (A::Small, A::Small) => A::Small,
        (A::Large(s), A::Large(r)) => if s == r { A::Large(s) } else { return nm("exponentially large terms of opposite sign") },
        (A::Large(s), _) | (_, A::Large(s)) => A::Large(s),
        (A::Osc, A::S(x)) | (A::S(x), A::Osc) => {
            match x.lead() {
                Some((v, _)) if v.is_negative() => A::S(x),       // a growing power outweighs a bounded oscillation
                _ => A::Osc,
            }
        }
        (A::Osc, _) | (_, A::Osc) => A::Osc,
    })
}

fn mul(a: A, b: A) -> Res<A> {
    Ok(match (a, b) {
        (A::S(x), A::S(y)) => A::S(x.mul(&y)?),
        (A::S(x), A::Small) | (A::Small, A::S(x)) => {
            if x.lead().is_none() && x.top.is_none() { A::S(Ser::constant(sym::int(0))) } else { A::Small }
        }
        (A::Small, A::Small) => A::Small,
        (A::S(x), A::Large(s)) | (A::Large(s), A::S(x)) => match x.lead() {
            Some((_, c)) => A::Large(s * sign_ex(&c)?),
            None if x.top.is_none() => A::S(Ser::constant(sym::int(0))),
            None => return nm("an exponentially large factor times a vanishing series"),
        },
        (A::Large(s), A::Large(r)) => A::Large(s * r),
        (A::Small, A::Large(_)) | (A::Large(_), A::Small) => return nm("exponentially large times exponentially small"),
        (A::Osc, A::S(x)) | (A::S(x), A::Osc) => match x.lead() {
            Some((v, _)) if v.is_positive() => {
                // bounded oscillation times a vanishing power: tends to 0, its terms unknown from that order on
                A::S(Ser { terms: BTreeMap::new(), top: Some(v) })
            }
            None if x.top.is_none() => A::S(Ser::constant(sym::int(0))),
            _ => A::Osc,
        },
        (A::Osc, A::Small) | (A::Small, A::Osc) => A::Small,
        _ => A::Osc,
    })
}

fn pow_q(base: A, r: &Q) -> Res<A> {
    match base {
        A::S(s) => {
            if s.lead().is_none() {
                if r.is_positive() {
                    return Ok(A::S(Ser { terms: BTreeMap::new(), top: s.top.as_ref().map(|t| t * r) }));
                }
                return nm("a negative power of a vanishing series");
            }
            Ok(A::S(s.powq(r)?))
        }
        A::Small => if r.is_positive() { Ok(A::Small) } else { nm("a negative power of an exponentially small value") },
        A::Large(s) => {
            if r.is_negative() {
                Ok(A::Small)
            } else if r.is_integer() {
                let even = (r.to_integer() % BigInt::from(2)).is_zero();
                Ok(A::Large(if even { 1 } else { s }))
            } else if s > 0 {
                Ok(A::Large(1))
            } else {
                nm("a fractional power of a large negative value")
            }
        }
        A::Osc => nm("a power of an oscillating value"),
    }
}

fn log(a: A) -> Res<A> {
    match a {
        A::S(s) => {
            let (c, v, u) = s.split()?;
            if !v.is_zero() {
                return nm("a logarithm of a power of t");
            }
            if sign_ex(&c)? <= 0 {
                return nm("a logarithm of a non-positive value");
            }
            // log(c (1 + u)) = log c + sum (-1)^(k+1) u^k / k
            let s = Ser::compose(&|k| if k == 0 { sym::int(0) } else {
                sym::num(Q::new(BigInt::from(if k % 2 == 1 { 1 } else { -1 }), BigInt::from(k)))
            }, &u)?;
            Ok(A::S(s.add(&Ser::constant(sym::func(Fun::Log, c)))?))
        }
        A::Large(1) => nm("a logarithm of an exponentially large value"),
        _ => nm("a logarithm of this value"),
    }
}

fn exp(a: A) -> Res<A> {
    match a {
        A::S(s) => {
            let neg: Vec<(Q, Ex)> = s.terms.iter().filter(|(e, _)| e.is_negative()).map(|(e, c)| (e.clone(), c.clone())).collect();
            if let Some((_, c)) = neg.first() {
                return Ok(if sign_ex(c)? > 0 { A::Large(1) } else { A::Small });
            }
            if s.lead().is_none() && s.top.as_ref().is_some_and(|t| !t.is_positive()) {
                return nm("an exponent known only to a non-positive order");
            }
            let c0 = s.terms.get(&Q::zero()).cloned().unwrap_or_else(|| sym::int(0));
            let mut u = s.clone();
            u.terms.remove(&Q::zero());
            let w = Ser::compose(&|k| {
                let mut f = Q::one();
                for j in 1..=k {
                    f /= qi(j as i64);
                }
                sym::num(f)
            }, &u)?;
            Ok(A::S(w.mul(&Ser::constant(sym::func(Fun::Exp, c0)))?))
        }
        A::Small => Ok(A::S(Ser::constant(sym::int(1)))),
        A::Large(1) => nm("an exponential of an exponentially large value"),
        A::Large(_) => Ok(A::Small),
        A::Osc => Ok(A::Osc),
    }
}

fn fun(f: Fun, a: A) -> Res<A> {
    use Fun::*;
    match f {
        Exp => return exp(a),
        Log => return log(a),
        _ => {}
    }
    match a {
        A::S(s) => {
            match s.lead() {
                Some((v, c)) if v.is_negative() => {
                    // the argument grows without bound
                    let sg = sign_ex(&c)?;
                    match f {
                        Sin | Cos => Ok(A::Osc),
                        Atan => {
                            // atan(u) = sign(u) pi/2 - atan(1/u)
                            let inv = s.inv()?;
                            let t = taylor_at(Atan, &sym::int(0), &inv)?;
                            let half_pi = sym::mul(sym::num(Q::new(BigInt::from(sg), BigInt::from(2))), Ex::Pi);
                            Ok(A::S(Ser::constant(half_pi).add(&t.neg())?))
                        }
                        Tanh => Ok(A::S(Ser::constant(sym::int(sg as i64)))),
                        Sinh => Ok(A::Large(sg)),
                        Cosh => Ok(A::Large(1)),
                        Abs => Ok(A::S(if sg > 0 { s } else { s.neg() })),
                        _ => nm("this function of a growing argument"),
                    }
                }
                Some((_, _)) | None => {
                    if s.lead().is_none() && s.top.as_ref().is_some_and(|t| !t.is_positive()) {
                        return nm("an argument known only to a non-positive order");
                    }
                    let c0 = s.terms.get(&Q::zero()).cloned().unwrap_or_else(|| sym::int(0));
                    let mut w = s.clone();
                    w.terms.remove(&Q::zero());
                    if f == Abs {
                        let sg = match s.lead() { Some((_, c)) => sign_ex(&c)?, None => return nm("Abs of a vanishing value") };
                        return Ok(A::S(if sg > 0 { s } else { s.neg() }));
                    }
                    Ok(A::S(taylor_at(f, &c0, &w)?))
                }
            }
        }
        A::Small => Ok(A::S(taylor_at(f, &sym::int(0), &Ser::constant(sym::int(0)))?)),
        A::Large(sg) => match f {
            Sin | Cos => Ok(A::Osc),
            Atan => Ok(A::S(Ser::constant(sym::mul(sym::num(Q::new(BigInt::from(sg), BigInt::from(2))), Ex::Pi)))),
            Tanh => Ok(A::S(Ser::constant(sym::int(sg as i64)))),
            Sinh => Ok(A::Large(sg)),
            Cosh => Ok(A::Large(1)),
            Abs => Ok(A::Large(1)),
            _ => nm("this function of an exponentially large argument"),
        },
        A::Osc => match f {
            Abs => Ok(A::Osc),
            _ => nm("a function of an oscillating value"),
        },
    }
}

/// A limit's outcome as sympy gives it
#[derive(Debug, Clone)]
pub enum Lim {
    /// a finite exact value
    Value(Ex),
    /// oo, -oo or zoo
    Infinite,
    /// AccumBounds: no limit
    Oscillates,
}

/// The limit of e as t -> 0+ with x = sub(t)
fn limit_t(e: &Ex, x: &str, sub: &Ser) -> Res<Lim> {
    match expand(e, x, sub)? {
        A::S(s) => match s.lead() {
            Some((v, c)) => {
                if v.is_positive() {
                    Ok(Lim::Value(sym::int(0)))
                } else if v.is_zero() {
                    Ok(Lim::Value(c))
                } else {
                    Ok(Lim::Infinite)
                }
            }
            None => match &s.top {
                Some(t) if t.is_positive() => Ok(Lim::Value(sym::int(0))),
                None => Ok(Lim::Value(sym::int(0))),
                _ => nm("the leading term cancels beyond the kept order"),
            },
        },
        A::Small => Ok(Lim::Value(sym::int(0))),
        A::Large(_) => Ok(Lim::Infinite),
        A::Osc => Ok(Lim::Oscillates),
    }
}

/// The limit of e as x -> +inf (sign 1) or -inf (sign -1)
pub fn at_infinity(e: &Ex, x: &str, sign: i32) -> Res<Lim> {
    limit_t(e, x, &Ser::monomial(qi(-1), sym::int(sign as i64)))
}

/// The one-sided limit of e as x -> a from the right (side 1) or the left (side -1)
pub fn one_sided(e: &Ex, x: &str, a: &Ex, side: i32) -> Res<Lim> {
    let mut m = BTreeMap::new();
    m.insert(Q::zero(), a.clone());
    m.insert(Q::one(), sym::int(side as i64));
    limit_t(e, x, &Ser::exact(m))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lim_inf(s: &str) -> Lim {
        at_infinity(&crate::symparse::parse(s, &["x"]).unwrap(), "x", 1).unwrap()
    }

    fn val(l: Lim) -> f64 {
        match l {
            Lim::Value(e) => sym::eval(&e, &|_| None).unwrap(),
            other => panic!("not a value: {other:?}"),
        }
    }

    #[test]
    fn limits_at_infinity() {
        assert_eq!(val(lim_inf("(3x^2 + 2x - 1)/(4x^2 + 5)")), 0.75);
        assert_eq!(val(lim_inf("(2x^3 + 1)/(5x^3 + 3x)")), 0.4);
        assert!((val(lim_inf("(1 + 3/x)^x")) - 3f64.exp()).abs() < 1e-15);
        assert_eq!(val(lim_inf("sqrt(x^2 + 4x) - x")), 2.0);
        assert_eq!(val(lim_inf("x*sin(2/x)")), 2.0);
        assert!((val(lim_inf("(3x + 1)/sqrt(2x^2 + 1)")) - 3.0 / 2f64.sqrt()).abs() < 1e-15);
        assert!((val(lim_inf("atan(x) + 4/x")) - std::f64::consts::FRAC_PI_2).abs() < 1e-15);
        assert_eq!(val(lim_inf("x^2*exp(-x)")), 0.0);
        assert!(matches!(lim_inf("exp(x)/x^5"), Lim::Infinite));
        assert!(matches!(lim_inf("x^2 + 1"), Lim::Infinite));
        assert!(matches!(lim_inf("sin(x)"), Lim::Oscillates));
        assert_eq!(val(lim_inf("sin(x)/x")), 0.0);
    }

    #[test]
    fn one_sided_limits() {
        let p = |s: &str| crate::symparse::parse(s, &["x"]).unwrap();
        let v = |l: Lim| val(l);
        assert_eq!(v(one_sided(&p("sin(3x)/x"), "x", &sym::int(0), 1).unwrap()), 3.0);
        assert_eq!(v(one_sided(&p("(1 - cos(x))/x^2"), "x", &sym::int(0), -1).unwrap()), 0.5);
        assert_eq!(v(one_sided(&p("sqrt(x)"), "x", &sym::int(0), 1).unwrap()), 0.0);
        assert!(matches!(one_sided(&p("1/x"), "x", &sym::int(0), 1).unwrap(), Lim::Infinite));
    }
}
