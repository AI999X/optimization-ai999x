//! Differential equations, written by hand (see handwritten/mod.rs): `_expo_terms`, the Laplace-transform helper
//! that writes f(t) as a sum of c t^n e^{z t} with complex c and z. The package does it with sympy
//! (`sp.expand(e.rewrite(sp.exp))`, then reads each term); here the same expansion is built directly: sin, cos, sinh
//! and cosh become complex exponentials, products and integer powers are multiplied out, and like terms combine as
//! sympy combines them (same power of t, same exponent, same non-numeric factor), so the list of terms, its values
//! and the refusals match the package's.
//!
//! Also `inverse_laplace` (partial fractions at 40 digits with complex poles: mpmath's mpc, which the translated
//! runtime carries only in double precision) and `linear_system_solution` (mpmath.expm at 30 digits).

use super::arg;
use crate::py::{self, Py, PyErr, R};
use crate::pysym::Poly;
use crate::sym::{self, Ex, Fun};
use num_bigint::BigInt;
use num_rational::BigRational as Q;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::rc::Rc;

pub fn helper(name: &str, a: &[Py], kws: &[(String, Py)]) -> Option<R> {
    match name {
        "_expo_terms" => Some(expo_terms(a, kws)),
        "linear_system_solution" => Some(linear_system_solution(a)),
        "inverse_laplace" => Some(inverse_laplace(a)),
        _ => None,
    }
}

const SUP: &str = "supported: sums of c t^n e^{a t} sin/cos/sinh/cosh(b t)";
const NONLIN: &str = "supported: sums of c t^n e^{a t} sin/cos(b t) (an exponent not linear in t)";
const T: &str = "t";

fn expo_terms(a: &[Py], kws: &[(String, Py)]) -> R {
    let exact = match a.get(1).or_else(|| kws.iter().find(|(k, _)| k == "exact").map(|(_, v)| v)) {
        Some(v) => v.truthy(),
        None => false,
    };
    if exact {
        return Err(PyErr::Unsupported("_expo_terms(exact=True)".into()));
    }
    let Py::Str(text) = arg(a, 0) else { return Err(PyErr::Value("the expression is missing".into())) };
    let e = crate::symparse::parse_text_de(text, &[T]).map_err(PyErr::Value)?;
    let terms = terms(&e, 0).map_err(|m| PyErr::Value(m.into()))?;
    let mut out = vec![];
    for t in &terms {
        if t.n > 12 {
            return Err(PyErr::Value("powers of t up to 12 are supported".into()));
        }
        let (c, z) = t.values()?;
        out.push(Py::tuple(vec![Py::Complex(c.0, c.1), Py::Int(t.n as i64), Py::Complex(z.0, z.1)]));
    }
    if out.is_empty() {
        return Err(PyErr::Value("the function is identically zero".into()));
    }
    Ok(Py::list(out))
}

/// One term q * i^ip * o * t^n * exp((zr + i zi) t + rr + i ri): q a rational, o a constant non-numeric factor (1 if
/// none), the exponent's coefficient of t and constant part exact
#[derive(Clone)]
struct Term {
    q: Q,
    ip: u8,
    o: Ex,
    n: u32,
    zr: Ex,
    zi: Ex,
    rr: Ex,
    ri: Ex,
}

fn zero() -> Ex {
    sym::int(0)
}

fn one_term() -> Term {
    Term { q: Q::one(), ip: 0, o: sym::int(1), n: 0, zr: zero(), zi: zero(), rr: zero(), ri: zero() }
}

fn has_t(e: &Ex) -> bool {
    let mut v = vec![];
    sym::free_symbols(e, &mut v);
    v.iter().any(|s| &**s == T)
}

/// k/2 when the term is k pi / 2 (exp(i k pi / 2) is i^k, which sympy evaluates)
fn half_pi_multiple(e: &Ex) -> Option<i64> {
    let two = Q::from_integer(BigInt::from(2));
    let r = match e {
        Ex::Pi => Q::one(),
        Ex::Mul(v) if v.len() == 2 => match (&v[0], &v[1]) {
            (Ex::Num(r), Ex::Pi) | (Ex::Pi, Ex::Num(r)) => r.clone(),
            _ => return None,
        },
        _ => return None,
    };
    let k = r * two;
    if k.is_integer() { k.to_integer().to_i64() } else { None }
}

impl Term {
    fn key_eq(&self, b: &Term) -> bool {
        self.ip == b.ip && self.n == b.n && self.o == b.o && self.zr == b.zr && self.zi == b.zi && self.rr == b.rr
            && self.ri == b.ri
    }

    fn norm(mut self) -> Term {
        if let Ex::Num(v) = &self.o {
            self.q *= v.clone();
            self.o = sym::int(1);
        }
        // i pi k / 2 in the constant exponent: a power of i
        let parts: Vec<Ex> = match &self.ri { Ex::Add(v) => v.iter().cloned().collect(), other => vec![other.clone()] };
        let mut k = 0i64;
        let mut keep = vec![];
        for p in parts {
            match half_pi_multiple(&p) {
                Some(m) => k += m,
                None => keep.push(p),
            }
        }
        if k != 0 {
            self.ri = sym::add_all(keep);
        }
        let mut ip = (self.ip as i64 + k).rem_euclid(4);
        if ip >= 2 {
            self.q = -self.q;
            ip -= 2;
        }
        self.ip = ip as u8;
        self
    }

    fn mul(&self, b: &Term) -> Term {
        Term {
            q: &self.q * &b.q,
            ip: self.ip + b.ip,
            o: sym::mul(self.o.clone(), b.o.clone()),
            n: self.n + b.n,
            zr: sym::add(self.zr.clone(), b.zr.clone()),
            zi: sym::add(self.zi.clone(), b.zi.clone()),
            rr: sym::add(self.rr.clone(), b.rr.clone()),
            ri: sym::add(self.ri.clone(), b.ri.clone()),
        }
        .norm()
    }

    /// 1 / term (a term without a power of t)
    fn inv(&self) -> Term {
        let mut t = Term {
            q: Q::one() / &self.q,
            ip: self.ip,
            o: sym::pow(self.o.clone(), sym::int(-1)),
            n: 0,
            zr: sym::neg(self.zr.clone()),
            zi: sym::neg(self.zi.clone()),
            rr: sym::neg(self.rr.clone()),
            ri: sym::neg(self.ri.clone()),
        };
        if t.ip == 1 {
            t.q = -t.q;                                           // 1/i = -i
        }
        t.norm()
    }

    fn scaled(&self, q: Q, ip: u8) -> Term {
        let mut t = self.clone();
        t.q *= q;
        t.ip += ip;
        t.norm()
    }

    /// (c, z) as complex numbers
    fn values(&self) -> Result<((f64, f64), (f64, f64)), PyErr> {
        let ev = |e: &Ex| sym::eval(e, &|_| None).map_err(|_| PyErr::Type("can't convert expression to complex".into()));
        let mag = self.q.to_f64().unwrap_or(f64::NAN) * ev(&self.o)? * ev(&self.rr)?.exp();
        let th = ev(&self.ri)?;
        let (mut re, mut im) = (mag * th.cos(), mag * th.sin());
        if th == 0.0 {
            im = 0.0;
        }
        if self.ip == 1 {
            (re, im) = (-im, re);
        }
        Ok(((re + 0.0, im + 0.0), (ev(&self.zr)? + 0.0, ev(&self.zi)? + 0.0)))
    }
}

/// like terms combined (exact coefficients); a term whose coefficient cancels to zero dropped
fn merge(v: Vec<Term>) -> Vec<Term> {
    let mut out: Vec<Term> = vec![];
    for t in v {
        if let Some(u) = out.iter_mut().find(|u| u.key_eq(&t)) {
            u.q += t.q;
        } else {
            out.push(t);
        }
    }
    out.retain(|t| !t.q.is_zero());
    out
}

/// Bounds on the work an input can ask for: the terms of one expansion and the nesting of the expression (an input
/// beyond them is refused with the package's "supported" wording; the package itself would expand on, slowly)
const MAX_TERMS: usize = 4000;
const MAX_DEPTH: usize = 64;

fn product(a: &[Term], b: &[Term]) -> Result<Vec<Term>, &'static str> {
    if a.len().saturating_mul(b.len()) > 4 * MAX_TERMS {
        return Err(SUP);
    }
    let mut v = vec![];
    for x in a {
        for y in b {
            v.push(x.mul(y));
        }
    }
    let v = merge(v);
    if v.len() > MAX_TERMS {
        return Err(SUP);
    }
    Ok(v)
}

/// exp of a sum of terms: the exponent must be linear in t
fn exp_of(ta: &[Term]) -> Result<Term, &'static str> {
    let mut t = one_term();
    for a in ta {
        let plain = sym::is_zero(&a.zr) && sym::is_zero(&a.zi) && sym::is_zero(&a.rr) && sym::is_zero(&a.ri);
        if !plain || a.n > 1 {
            return Err(NONLIN);
        }
        let val = sym::mul(sym::num(a.q.clone()), a.o.clone());
        let (re, im) = if a.n == 1 { (&mut t.zr, &mut t.zi) } else { (&mut t.rr, &mut t.ri) };
        if a.ip == 0 {
            *re = sym::add(re.clone(), val);
        } else {
            *im = sym::add(im.clone(), val);
        }
    }
    Ok(t.norm())
}

fn terms(e: &Ex, depth: usize) -> Result<Vec<Term>, &'static str> {
    if depth > MAX_DEPTH {
        return Err(SUP);
    }
    let terms = |x: &Ex| terms(x, depth + 1);
    let half = || Q::new(BigInt::from(1), BigInt::from(2));
    Ok(match e {
        Ex::Num(v) => {
            if v.is_zero() { vec![] } else { vec![Term { q: v.clone(), ..one_term() }] }
        }
        Ex::Pi => vec![Term { o: Ex::Pi, ..one_term() }],
        Ex::E => vec![Term { rr: sym::int(1), ..one_term() }],
        Ex::Sym(s) if &**s == T => vec![Term { n: 1, ..one_term() }],
        Ex::Add(v) => {
            let mut all = vec![];
            for x in v.iter() {
                all.extend(terms(x)?);
            }
            if all.len() > MAX_TERMS {
                return Err(SUP);
            }
            merge(all)
        }
        Ex::Mul(v) => {
            let mut acc = vec![one_term()];
            for x in v.iter() {
                acc = product(&acc, &terms(x)?)?;
            }
            acc
        }
        Ex::Pow(b, p) => {
            if !has_t(b) && !has_t(p) {
                match (&**b, &**p) {
                    (Ex::Add(_), Ex::Num(k)) if k.is_integer() && k.is_positive() && *k <= Q::from_integer(BigInt::from(64)) => {
                        power(&terms(b)?, k.to_integer().to_i64().unwrap_or(1))?
                    }
                    _ => vec![Term { o: e.clone(), ..one_term() }.norm()],
                }
            } else if !has_t(b) {
                // b^u = exp(u log b) for a positive constant base
                match sym::eval(b, &|_| None) {
                    Ok(v) if v > 0.0 => {
                        let u = sym::mul((**p).clone(), sym::func(Fun::Log, (**b).clone()));
                        vec![exp_of(&terms(&u)?)?]
                    }
                    _ => return Err(SUP),
                }
            } else {
                let Ex::Num(k) = &**p else { return Err(SUP) };
                if !k.is_integer() {
                    return Err(SUP);
                }
                let k = k.to_integer().to_i64().ok_or(SUP)?;
                let tb = terms(b)?;
                if k > 0 {
                    let top = tb.iter().map(|t| t.n).max().unwrap_or(0) as i64;
                    if top * k > 12 {
                        return Err("powers of t up to 12 are supported");
                    }
                    if k > 400 {
                        return Err(SUP);
                    }
                    power(&tb, k)?
                } else if tb.len() == 1 && tb[0].n == 0 {
                    let m = k.checked_neg().ok_or(SUP)?;
                    if m > 400 {
                        return Err(SUP);
                    }
                    let inv = tb[0].inv();
                    power(&[inv], m)?
                } else {
                    return Err(SUP);
                }
            }
        }
        Ex::Fun(f @ (Fun::Exp | Fun::Sin | Fun::Cos | Fun::Sinh | Fun::Cosh), a) => {
            let ta = terms(a)?;
            let neg: Vec<Term> = ta.iter().map(|t| t.scaled(-Q::one(), 0)).collect();
            match f {
                Fun::Exp => vec![exp_of(&ta)?],
                Fun::Sinh | Fun::Cosh => {
                    let (p, m) = (exp_of(&ta)?, exp_of(&neg)?);
                    let s = if *f == Fun::Sinh { -half() } else { half() };
                    merge(vec![p.scaled(half(), 0), m.scaled(s, 0)])
                }
                _ => {
                    let ia: Vec<Term> = ta.iter().map(|t| t.scaled(Q::one(), 1)).collect();
                    let ineg: Vec<Term> = ta.iter().map(|t| t.scaled(-Q::one(), 1)).collect();
                    let (p, m) = (exp_of(&ia)?, exp_of(&ineg)?);
                    if *f == Fun::Sin {
                        // sin u = -i/2 e^{iu} + i/2 e^{-iu}
                        merge(vec![p.scaled(-half(), 1), m.scaled(half(), 1)])
                    } else {
                        merge(vec![p.scaled(half(), 0), m.scaled(half(), 0)])
                    }
                }
            }
        }
        other if !has_t(other) && !matches!(other, Ex::Zoo | Ex::Nan) => vec![Term { o: other.clone(), ..one_term() }.norm()],
        _ => return Err(SUP),
    })
}

fn power(tb: &[Term], k: i64) -> Result<Vec<Term>, &'static str> {
    let mut acc = vec![one_term()];
    for _ in 0..k {
        acc = product(&acc, tb)?;
    }
    Ok(acc)
}

// ------------------------------------------------------------------------------------------------ inverse Laplace
/// A complex number of mpmath values (an mpc): real and imaginary parts at the working precision
#[derive(Clone)]
struct C {
    r: Py,
    i: Py,
}

fn mpz() -> R {
    crate::pympf::mpf(&Py::Int(0))
}

impl C {
    fn real(r: Py) -> Result<C, PyErr> {
        Ok(C { r, i: mpz()? })
    }
    fn add(&self, o: &C) -> Result<C, PyErr> {
        Ok(C { r: py::add(&self.r, &o.r)?, i: py::add(&self.i, &o.i)? })
    }
    fn sub(&self, o: &C) -> Result<C, PyErr> {
        Ok(C { r: py::sub(&self.r, &o.r)?, i: py::sub(&self.i, &o.i)? })
    }
    fn mul(&self, o: &C) -> Result<C, PyErr> {
        Ok(C {
            r: py::sub(&py::mul(&self.r, &o.r)?, &py::mul(&self.i, &o.i)?)?,
            i: py::add(&py::mul(&self.r, &o.i)?, &py::mul(&self.i, &o.r)?)?,
        })
    }
    /// None when o is zero (mpmath's ZeroDivisionError)
    fn div(&self, o: &C) -> Result<Option<C>, PyErr> {
        let d = py::add(&py::mul(&o.r, &o.r)?, &py::mul(&o.i, &o.i)?)?;
        if d.f()? == 0.0 && o.r.f()? == 0.0 && o.i.f()? == 0.0 {
            return Ok(None);
        }
        let nr = py::add(&py::mul(&self.r, &o.r)?, &py::mul(&self.i, &o.i)?)?;
        let ni = py::sub(&py::mul(&self.i, &o.r)?, &py::mul(&self.r, &o.i)?)?;
        Ok(Some(C { r: py::truediv(&nr, &d)?, i: py::truediv(&ni, &d)? }))
    }
    fn abs(&self) -> Result<f64, PyErr> {
        Ok(crate::py::hypot2(self.r.f()?, self.i.f()?))
    }
    fn exp(&self) -> Result<C, PyErr> {
        let f = |n: &str, x: &Py| -> R { crate::pympf::func(n, x).unwrap_or_else(|| Err(PyErr::Unsupported("mp function".into()))) };
        let m = f("exp", &self.r)?;
        Ok(C { r: py::mul(&m, &f("cos", &self.i)?)?, i: py::mul(&m, &f("sin", &self.i)?)? })
    }
    /// +z: rounded to the working precision
    fn round(&self) -> Result<C, PyErr> {
        let z = mpz()?;
        Ok(C { r: py::add(&self.r, &z)?, i: py::add(&self.i, &z)? })
    }
}

fn qmp(q: &Q) -> R {
    let int = |b: &BigInt| match b.to_i64() { Some(v) => Py::Int(v), None => Py::Big(Rc::new(b.clone())) };
    py::truediv(&crate::pympf::mpf(&int(q.numer()))?, &int(q.denom()))
}

/// Why a text is not taken as F(s): not a rational function of s, or one whose degrees (built before any
/// cancelling) would pass MAX_DEG, or nested beyond MAX_NEST
enum RatErr {
    NotRational,
    TooBig,
}

/// Bounds on the work an input can ask for, checked BEFORE each product or power is built: a numerator or
/// denominator degree above MAX_DEG at any step, an exponent above MAX_DEG, or nesting deeper than MAX_NEST is
/// refused with the package's degree wording (its own limit, denominators up to degree 10, is checked afterwards)
const MAX_DEG: i64 = 40;
const MAX_NEST: usize = 64;

/// numerator and denominator of a rational function of s (sympy's fraction(cancel(together(e)))), Err where it is not
/// one (or too big to build); *irr set where a constant is not rational
fn rational_fn(e: &Ex, irr: &mut bool, depth: usize) -> Result<(Poly, Poly), RatErr> {
    if depth > MAX_NEST {
        return Err(RatErr::TooBig);
    }
    let s: Rc<str> = Rc::from("s");
    let one = || Poly::constant(s.clone(), Q::one());
    let has_s = |x: &Ex| { let mut v = vec![]; sym::free_symbols(x, &mut v); !v.is_empty() };
    let deg = |p: &Poly| p.degree().max(0);
    let fits = |d: i64| if d > MAX_DEG { Err(RatErr::TooBig) } else { Ok(()) };
    Ok(match e {
        Ex::Num(v) => (Poly::constant(s.clone(), v.clone()), one()),
        Ex::Sym(n) if &**n == "s" => (Poly::new(s.clone(), vec![Q::zero(), Q::one()]), one()),
        Ex::Add(v) => {
            let (mut n, mut d) = (Poly::new(s.clone(), vec![]), one());
            for x in v.iter() {
                let (a, b) = rational_fn(x, irr, depth + 1)?;
                fits(deg(&d) + deg(&b))?;
                fits((deg(&n) + deg(&b)).max(deg(&a) + deg(&d)))?;
                n = n.mul(&b).add(&a.mul(&d));
                d = d.mul(&b);
            }
            (n, d)
        }
        Ex::Mul(v) => {
            let (mut n, mut d) = (one(), one());
            for x in v.iter() {
                let (a, b) = rational_fn(x, irr, depth + 1)?;
                fits(deg(&n) + deg(&a))?;
                fits(deg(&d) + deg(&b))?;
                n = n.mul(&a);
                d = d.mul(&b);
            }
            (n, d)
        }
        Ex::Pow(b, p) if matches!(&**p, Ex::Num(k) if k.is_integer()) && has_s(b) => {
            let Ex::Num(k) = &**p else { return Err(RatErr::NotRational) };
            let k = k.to_integer().to_i64().ok_or(RatErr::TooBig)?;
            if k.unsigned_abs() > MAX_DEG as u64 {
                return Err(RatErr::TooBig);
            }
            let (n, d) = rational_fn(b, irr, depth + 1)?;
            let m = k.unsigned_abs() as i64;
            fits(deg(&n) * m)?;
            fits(deg(&d) * m)?;
            let m = m as u32;
            if k >= 0 { (n.pow(m), d.pow(m)) } else { (d.pow(m), n.pow(m)) }
        }
        other if !has_s(other) => {
            *irr = true;
            (one(), one())
        }
        _ => return Err(RatErr::NotRational),
    })
}

/// mpmath.polyroots(co, maxsteps=500, extraprec=300) of a real polynomial (coefficients highest first, mpf)
fn polyroots(co: &[Py]) -> Result<Vec<C>, PyErr> {
    let orig = crate::pympf::prec();
    let tol = 2f64.powi(1 - orig as i32);
    let deg = co.len() - 1;
    let mut roots;
    {
        let _g = crate::pympf::scope("extraprec", &Py::Int(300))?;
        let lead = co[0].clone();
        let cs: Vec<C> = co.iter().map(|c| Ok(C::real(py::truediv(c, &lead)?)?)).collect::<Result<_, PyErr>>()?;
        let polyval = |x: &C| -> Result<C, PyErr> {
            let mut p = cs[0].clone();
            for c in &cs[1..] {
                p = p.mul(x)?.add(c)?;
            }
            Ok(p)
        };
        // (0.4 + 0.9j)**n in double precision (CPython's binary powering), then as mpc
        let cmul = |a: (f64, f64), b: (f64, f64)| (a.0 * b.0 - a.1 * b.1, a.0 * b.1 + a.1 * b.0);
        roots = vec![];
        for n in 0..deg {
            let (mut r, mut x, mut mask) = ((1.0, 0.0), (0.4, 0.9), 1usize);
            while mask > 0 && n >= mask {
                if n & mask != 0 {
                    r = cmul(r, x);
                }
                mask <<= 1;
                x = cmul(x, x);
            }
            roots.push(C { r: crate::pympf::mpf(&Py::Float(r.0))?, i: crate::pympf::mpf(&Py::Float(r.1))? });
        }
        let mut err = vec![1.0f64; deg];
        let mut converged = false;
        for _ in 0..500 {
            if err.iter().cloned().fold(f64::MIN, f64::max) < tol {
                converged = true;
                break;
            }
            for i in 0..deg {
                let p = roots[i].clone();
                let mut x = polyval(&p)?;
                for j in 0..deg {
                    if i != j {
                        if let Some(q) = x.div(&p.sub(&roots[j])?)? {
                            x = q;
                        }
                    }
                }
                roots[i] = p.sub(&x)?;
                err[i] = x.abs()?;
            }
        }
        if !converged && err.iter().cloned().fold(f64::MIN, f64::max) >= tol {
            return Err(PyErr::Other("NoConvergence".into()));
        }
        for z in roots.iter_mut() {
            if z.abs()? < tol {
                *z = C::real(mpz()?)?;
            } else if z.i.f()?.abs() < tol {
                z.i = mpz()?;
            } else if z.r.f()?.abs() < tol {
                z.r = mpz()?;
            }
        }
        let key = |z: &C| (z.i.f().unwrap_or(0.0).abs(), z.r.f().unwrap_or(0.0));
        roots.sort_by(|a, b| key(a).partial_cmp(&key(b)).unwrap_or(std::cmp::Ordering::Equal));
    }
    roots.iter().map(|z| z.round()).collect()
}

/// _poles(Q): [(pole, multiplicity)] at the working precision, multiplicities from the square-free factorisation
fn poles(q: &Poly) -> Result<Vec<(C, usize)>, PyErr> {
    let mut out = vec![];
    for (f, m) in q.sqf_list().1 {
        let co: Vec<Py> = f.all_coeffs().iter().map(qmp).collect::<Result<_, _>>()?;
        let rs = if f.degree() == 1 {
            vec![C::real(py::truediv(&py::neg(&co[1])?, &co[0])?)?]
        } else {
            polyroots(&co)?
        };
        out.extend(rs.into_iter().map(|r| (r, m as usize)));
    }
    Ok(out)
}

/// _taylor(co, p, m): coefficients of poly(p + u) for u^0 .. u^{m-1}, by repeated synthetic division
fn taylor(co: &[C], p: &C, m: usize) -> Result<Vec<C>, PyErr> {
    let mut co = co.to_vec();
    let mut res = vec![];
    for _ in 0..m {
        let mut q = vec![];
        let mut r = C::real(mpz()?)?;
        for c in &co {
            r = r.mul(p)?.add(c)?;
            q.push(r.clone());
        }
        res.push(q.pop().unwrap());
        co = q;
        if co.is_empty() {
            break;
        }
    }
    while res.len() < m {
        res.push(C::real(mpz()?)?);
    }
    Ok(res)
}

/// inverse_laplace(F, t) of the package: f(t) from the partial fractions of a strictly proper rational F(s) at 40
/// digits, and the poles (real ones first, ascending; then complex ones by real part)
fn inverse_laplace(a: &[Py]) -> R {
    let (ftext, t) = (arg(a, 0), arg(a, 1));
    let tf = fin(t)?;
    if tf < 0.0 {
        return Err(PyErr::Value("t must be >= 0".into()));
    }
    // _ratfun
    let Py::Str(text) = ftext else { return Err(PyErr::Value("the expression is missing".into())) };
    let e = crate::symparse::parse_text_de(text, &["s"]).map_err(PyErr::Value)?;
    let mut irr = false;
    let (num, den) = rational_fn(&e, &mut irr, 0).map_err(|w| PyErr::Value(match w {
        RatErr::NotRational => "F(s) must be a rational function of s".into(),
        RatErr::TooBig => "denominators up to degree 10 are supported".into(),
    }))?;
    if irr {
        return Err(PyErr::Value("F(s) needs rational coefficients".into()));
    }
    let g = num.gcd(&den);
    let (p, q) = if g.is_zero() { (num, den) } else { (num.divmod(&g).0, den.divmod(&g).0) };
    if p.is_zero() {
        return Err(PyErr::Value("F(s) is zero".into()));
    }
    if p.degree() >= q.degree() {
        return Err(PyErr::Value("F(s) must be strictly proper (degree of numerator < degree of denominator)".into()));
    }
    if q.degree() > 10 {
        return Err(PyErr::Value("denominators up to degree 10 are supported".into()));
    }
    let _g = crate::pympf::scope("workdps", &Py::Int(40))?;
    let pc: Vec<C> = p.all_coeffs().iter().map(|c| C::real(qmp(c)?)).collect::<Result<_, _>>()?;
    let qc: Vec<C> = q.all_coeffs().iter().map(|c| C::real(qmp(c)?)).collect::<Result<_, _>>()?;
    let tm = C::real(crate::pympf::mpf(&Py::Float(tf))?)?;
    let mut total = C::real(mpz()?)?;
    let mut big = 0.0f64;
    let pl = poles(&q)?;
    for (pole, m) in &pl {
        let m = *m;
        let mut rr = qc.clone();
        for _ in 0..m {
            let mut nq = vec![];
            let mut r = C::real(mpz()?)?;
            for c in &rr {
                r = r.mul(pole)?.add(c)?;
                nq.push(r.clone());
            }
            nq.pop();
            rr = nq;
        }
        let (n_t, r_t) = (taylor(&pc, pole, m)?, taylor(&rr, pole, m)?);
        let mut g: Vec<C> = vec![];
        for k in 0..m {
            let mut s = C::real(mpz()?)?;
            for j in 1..=k {
                s = s.add(&r_t[j].mul(&g[k - j])?)?;
            }
            let v = n_t[k].sub(&s)?.div(&r_t[0])?.ok_or(PyErr::ZeroDivision)?;
            g.push(v);
        }
        let mut s = C::real(mpz()?)?;
        for k in 0..m {
            let e = m - 1 - k;
            let fact = (1..=e).map(|v| v as f64).product::<f64>().max(1.0);
            let mut tp = C::real(crate::pympf::mpf(&Py::Int(1))?)?;
            for _ in 0..e {
                tp = tp.mul(&tm)?;
            }
            let f = C::real(crate::pympf::mpf(&Py::Float(fact))?)?;
            s = s.add(&g[k].mul(&tp)?.div(&f)?.ok_or(PyErr::ZeroDivision)?)?;
        }
        let term = pole.mul(&tm)?.exp()?.mul(&s)?;
        big += term.abs()?;
        total = total.add(&term)?;
    }
    if total.i.f()?.abs() > 1e-25 * big.max(1.0) {
        return Err(PyErr::Value("internal: the inverse is not real".into()));
    }
    let val = total.r.f()?;
    if big > 1e20 * val.abs().max(1e-300) {
        return Err(PyErr::Value("the result is lost to cancellation".into()));
    }
    // _canon_c of the poles as Python complex numbers
    let zs: Vec<(f64, f64)> = pl.iter().map(|(z, _)| Ok((z.r.f()?, z.i.f()?))).collect::<Result<_, PyErr>>()?;
    let zs: Vec<(f64, f64)> = zs.into_iter()
        .map(|z| if z.1.abs() <= 1e-25 * crate::py::hypot2(z.0, z.1).max(1.0) { (z.0, 0.0) } else { z }).collect();
    let mut real: Vec<f64> = zs.iter().filter(|z| z.1 == 0.0).map(|z| z.0).collect();
    real.sort_by(f64::total_cmp);
    let mut cplx: Vec<(f64, f64)> = zs.iter().filter(|z| z.1 != 0.0).cloned().collect();
    let r9 = |x: f64| format!("{x:.9}").parse::<f64>().unwrap_or(x);
    cplx.sort_by(|a, b| (r9(a.0), -a.1).partial_cmp(&(r9(b.0), -b.1)).unwrap_or(std::cmp::Ordering::Equal));
    // the real poles are floats (z.real), the others complex numbers
    let mut out = vec![];
    for x in &real {
        out.push(Py::Float(r12(*x)?));
    }
    for z in cplx {
        out.push(Py::Complex(r12(z.0)?, r12(z.1)?));
    }
    Ok(Py::dict(vec![(Py::str("f"), Py::Float(r12(val)?)), (Py::str("poles"), Py::list(out))]))
}

// ------------------------------------------------------------------------------------------------ x' = A x
/// _fin of the package: every input a finite int or float (not a bool)
fn fin(v: &Py) -> Result<f64, PyErr> {
    match v {
        Py::Int(_) | Py::Float(_) | Py::Big(_) => {
            let x = v.f()?;
            if x.is_finite() { Ok(x) } else { Err(PyErr::Value("every input must be a finite number".into())) }
        }
        _ => Err(PyErr::Value("every input must be a finite number".into())),
    }
}

/// _r of the package: 12 significant digits, a non-finite value refused
fn r12(v: f64) -> Result<f64, PyErr> {
    if !v.is_finite() {
        return Err(PyErr::Value("the result is not finite (it overflows)".into()));
    }
    if v == 0.0 {
        return Ok(0.0);
    }
    Ok(format!("{v:.11e}").parse::<f64>().unwrap_or(v))
}

/// linear_system_solution(A, x0, t): x(t) = e^{At} x0 with the matrix exponential at 30 digits (the package uses
/// mpmath.expm; here scaling and squaring of the Taylor series at 30 bits above mpmath's 30 digits), with the
/// package's checks and refusals
fn linear_system_solution(a: &[Py]) -> R {
    let (am, x0, t) = (arg(a, 0), arg(a, 1), arg(a, 2));
    let tf = fin(t)?;
    let rows = am.items()?;
    let n = rows.len();
    let rows: Vec<Vec<Py>> = rows.iter().map(|r| r.items()).collect::<Result<_, _>>()?;
    if !(1..=6).contains(&n) || rows.iter().any(|r| r.len() != n) {
        return Err(PyErr::Value("A must be square, of size 1 to 6".into()));
    }
    let x0 = x0.items()?;
    if x0.len() != n {
        return Err(PyErr::Value("x0 needs one entry per row of A".into()));
    }
    let _g = crate::pympf::scope("workprec", &Py::Int((crate::pympf::dps_to_prec(30.0) + 30) as i64))?;
    let mpf = |v: &Py| -> R { crate::pympf::mpf(&Py::Float(v.f()?)) };
    let zero = mpf(&Py::Int(0))?;
    let one = mpf(&Py::Int(1))?;
    let tm = mpf(&Py::Float(tf))?;
    // B = A t / 2^s, its max-row-sum norm at most 1/2
    let mut norm = 0.0f64;
    for r in &rows {
        let mut s = 0.0;
        for v in r {
            s += (v.f()? * tf).abs();
        }
        norm = norm.max(s);
    }
    if !norm.is_finite() {
        return Err(PyErr::Value("the result is not finite (it overflows)".into()));
    }
    let sq = if norm > 0.5 { (norm / 0.5).log2().ceil() as i32 } else { 0 };
    let scale = mpf(&Py::Float(2f64.powi(-sq)))?;
    let mut b = vec![vec![zero.clone(); n]; n];
    for i in 0..n {
        for j in 0..n {
            b[i][j] = py::mul(&py::mul(&mpf(&rows[i][j])?, &tm)?, &scale)?;
        }
    }
    let matmul = |x: &Vec<Vec<Py>>, y: &Vec<Vec<Py>>| -> Result<Vec<Vec<Py>>, PyErr> {
        let mut out = vec![vec![zero.clone(); n]; n];
        for i in 0..n {
            for j in 0..n {
                let mut s = zero.clone();
                for k in 0..n {
                    s = py::add(&s, &py::mul(&x[i][k], &y[k][j])?)?;
                }
                out[i][j] = s;
            }
        }
        Ok(out)
    };
    // the Taylor series: terms B^k / k! until they fall below the working precision
    let mut e = vec![vec![zero.clone(); n]; n];
    for (i, row) in e.iter_mut().enumerate() {
        row[i] = one.clone();
    }
    let mut term = e.clone();
    let tiny = 2f64.powi(-(crate::pympf::prec() as i32) - 4);
    for k in 1..400 {
        term = matmul(&term, &b)?;
        let kk = mpf(&Py::Int(k))?;
        let mut biggest = 0.0f64;
        for row in term.iter_mut() {
            for v in row.iter_mut() {
                *v = py::truediv(v, &kk)?;
                biggest = biggest.max(v.f()?.abs());
            }
        }
        for i in 0..n {
            for j in 0..n {
                e[i][j] = py::add(&e[i][j], &term[i][j])?;
            }
        }
        if biggest <= tiny {
            break;
        }
    }
    for _ in 0..sq {
        e = matmul(&e, &e)?;
    }
    let mut xs = vec![];
    let mut big = 0.0f64;
    for i in 0..n {
        let mut s = Py::Int(0);
        let mut size = 0.0f64;
        for j in 0..n {
            let p = py::mul(&e[i][j], &mpf(&x0[j])?)?;
            size += p.f()?.abs();
            s = py::add(&s, &p)?;
        }
        big = big.max(size);
        xs.push(s.f()?);
    }
    for v in &xs {
        if v.abs() < 1e-6 * big {
            return Err(PyErr::Value("a component is lost to cancellation (it is near zero)".into()));
        }
    }
    let out: Vec<f64> = xs.iter().map(|v| r12(*v)).collect::<Result<_, _>>()?;
    Ok(Py::dict(vec![
        (Py::str("x"), Py::list(out.iter().map(|v| Py::Float(*v)).collect())),
        (Py::str("x1"), Py::Float(out[0])),
    ]))
}
