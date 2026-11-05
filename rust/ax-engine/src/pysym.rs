//! The symbolic layer of the calculus and differential-equations procedures in Rust: the package helpers that read,
//! evaluate, integrate and test expressions (parse_expr, _compile, at, quad, continuous_on, poly_of, real_roots_in,
//! show), the sympy calls the procedure bodies make (sp.diff, sp.Rational, sp.Poly, sp.N, ...) and the methods of a
//! sympy expression or polynomial they use. tools/py2rust.py routes those names here, so a procedure body translates
//! as written.
//!
//! What follows the package exactly: the refusals (word for word: an undefined point, a value that is not real, a
//! function that is not continuous on the interval, a polynomial that is required), exact polynomial arithmetic on
//! rationals and exact real roots (Sturm sequences; a rational root is returned exactly, an irrational one as the
//! nearest double), and sympy's continuity decisions (continuous_domain's rules, and its fallback to a 401-point grid
//! where sympy cannot decide, which it does for every expression holding exp). Values are computed in double
//! precision where the package uses 30-digit mpmath; the answers agree to the products' 6-digit bar, which
//! tools/rust_accuracy.py checks on every case. The self-checks that the package runs with mpmath's numerical
//! derivative use Taylor-mode automatic differentiation here (a different algorithm from the symbolic derivative,
//! accurate to rounding), and its Gauss-Legendre check a Gauss-Legendre rule.

#[allow(unused_imports)]
use crate::lm::{Lm, LmF64};
use crate::py::{self, Py, PyErr, R};
use crate::sym::{self, Ex, Fun};
use num_bigint::BigInt;
use num_rational::BigRational as Q;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::rc::Rc;

fn verr(m: impl Into<String>) -> PyErr {
    PyErr::Value(m.into())
}

fn terr(m: impl Into<String>) -> PyErr {
    PyErr::Type(m.into())
}

fn unsup(m: impl Into<String>) -> PyErr {
    PyErr::Unsupported(m.into())
}

pub fn ex(e: Ex) -> Py {
    Py::Sym(Rc::new(e))
}

pub fn symbol(name: &str) -> Py {
    ex(Ex::Sym(Rc::from(name)))
}

/// sp.pi, sp.E, sp.oo (a float infinity here: the procedures only compare with it or test `has`), sp.zoo, sp.nan
pub fn constant(name: &str) -> R {
    Ok(match name {
        "pi" => ex(Ex::Pi),
        "E" => ex(Ex::E),
        "oo" => Py::Float(f64::INFINITY),
        "zoo" => ex(Ex::Zoo),
        "nan" => ex(Ex::Nan),
        // unevaluated limits, sums and bounds: these expressions never hold one (e.has(sp.Limit) is False)
        "Limit" | "Sum" | "AccumBounds" | "Piecewise" => Py::None,
        _ => return Err(unsup(format!("sympy.{name}"))),
    })
}

/// A Python value as an expression (numbers exactly, a float by its decimal repr as sympy's Rational(repr(x)) would
/// not: a float stays its binary value, as sympify(float) keeps a Float's double)
fn ex_of(p: &Py) -> Result<Ex, PyErr> {
    if let Py::Poly(q) = p {
        return Ok(q.as_ex());
    }
    py::to_ex(p).ok_or_else(|| terr(format!("cannot use {} as an expression", py::type_name(p))))
}

/// A variable given as a name or a symbol
fn var_name(p: &Py) -> Result<Rc<str>, PyErr> {
    match p {
        Py::Str(s) => Ok(s.clone()),
        Py::Sym(e) => match &**e {
            Ex::Sym(s) => Ok(s.clone()),
            _ => Err(terr("a variable is expected")),
        },
        _ => Err(terr("a variable is expected")),
    }
}

fn names_of(p: &Py) -> Result<Vec<Rc<str>>, PyErr> {
    p.items()?.iter().map(var_name).collect()
}

// ================================================================================================= polynomials
/// A polynomial in one variable with rational coefficients (sympy's Poly over QQ): c[i] is the coefficient of
/// gen^i, without trailing zeros (the zero polynomial has none).
#[derive(Clone, Debug, PartialEq)]
pub struct Poly {
    pub gen: Rc<str>,
    pub c: Vec<Q>,
    /// coefficients that are not all rational (sympy's Poly over an extension, as Poly(pi*x, x)): only its degree
    /// and coefficients are read (the procedures then refuse it for its irrational coefficients)
    pub irr: Option<Rc<Vec<Ex>>>,
}

impl Poly {
    pub fn new(gen: Rc<str>, mut c: Vec<Q>) -> Poly {
        while c.last().is_some_and(|x| x.is_zero()) {
            c.pop();
        }
        Poly { gen, c, irr: None }
    }
    pub fn constant(gen: Rc<str>, v: Q) -> Poly {
        Poly::new(gen, vec![v])
    }
    pub fn is_zero(&self) -> bool {
        self.c.is_empty()
    }
    /// degree; -1 for the zero polynomial (sympy: -oo)
    pub fn degree(&self) -> i64 {
        self.c.len() as i64 - 1
    }
    pub fn add(&self, o: &Poly) -> Poly {
        let n = self.c.len().max(o.c.len());
        let c = (0..n).map(|i| self.c.get(i).cloned().unwrap_or_else(Q::zero) + o.c.get(i).cloned().unwrap_or_else(Q::zero)).collect();
        Poly::new(self.gen.clone(), c)
    }
    pub fn neg(&self) -> Poly {
        Poly::new(self.gen.clone(), self.c.iter().map(|x| -x).collect())
    }
    pub fn sub(&self, o: &Poly) -> Poly {
        self.add(&o.neg())
    }
    pub fn mul(&self, o: &Poly) -> Poly {
        if self.is_zero() || o.is_zero() {
            return Poly::new(self.gen.clone(), vec![]);
        }
        let mut c = vec![Q::zero(); self.c.len() + o.c.len() - 1];
        for (i, a) in self.c.iter().enumerate() {
            for (j, b) in o.c.iter().enumerate() {
                c[i + j] += a * b;
            }
        }
        Poly::new(self.gen.clone(), c)
    }
    pub fn scale(&self, k: &Q) -> Poly {
        Poly::new(self.gen.clone(), self.c.iter().map(|x| x * k).collect())
    }
    pub fn pow(&self, n: u32) -> Poly {
        let mut out = Poly::constant(self.gen.clone(), Q::one());
        for _ in 0..n {
            out = out.mul(self);
        }
        out
    }
    pub fn diff(&self) -> Poly {
        let c = self.c.iter().enumerate().skip(1).map(|(i, x)| x * Q::from_integer(BigInt::from(i))).collect();
        Poly::new(self.gen.clone(), c)
    }
    /// the antiderivative with no constant term (sympy Poly.integrate())
    pub fn integrate(&self) -> Poly {
        let mut c = vec![Q::zero()];
        c.extend(self.c.iter().enumerate().map(|(i, x)| x / Q::from_integer(BigInt::from(i + 1))));
        Poly::new(self.gen.clone(), c)
    }
    pub fn eval_q(&self, x: &Q) -> Q {
        self.c.iter().rev().fold(Q::zero(), |acc, k| acc * x + k)
    }
    pub fn eval_f(&self, x: f64) -> f64 {
        self.c.iter().rev().fold(0.0, |acc, k| acc * x + k.to_f64().unwrap_or(f64::NAN))
    }
    pub fn lc(&self) -> Q {
        self.c.last().cloned().unwrap_or_else(Q::zero)
    }
    /// coefficients highest degree first (sympy all_coeffs; [0] for the zero polynomial)
    pub fn all_coeffs(&self) -> Vec<Q> {
        if self.c.is_empty() {
            return vec![Q::zero()];
        }
        self.c.iter().rev().cloned().collect()
    }
    pub fn as_ex(&self) -> Ex {
        let x = Ex::Sym(self.gen.clone());
        sym::add_all(self.c.iter().enumerate().filter(|(_, k)| !k.is_zero())
            .map(|(i, k)| sym::mul(sym::num(k.clone()), sym::pow(x.clone(), sym::int(i as i64)))).collect())
    }
    /// (quotient, remainder) of division by a nonzero polynomial
    pub fn divmod(&self, d: &Poly) -> (Poly, Poly) {
        let mut r = self.c.clone();
        let dl = d.lc();
        let dd = d.c.len();
        if r.len() < dd {
            return (Poly::new(self.gen.clone(), vec![]), self.clone());
        }
        let mut qc = vec![Q::zero(); r.len() - dd + 1];
        for i in (0..qc.len()).rev() {
            let k = &r[i + dd - 1] / &dl;
            for j in 0..dd {
                r[i + j] -= &k * &d.c[j];
            }
            qc[i] = k;
        }
        (Poly::new(self.gen.clone(), qc), Poly::new(self.gen.clone(), r))
    }
    pub fn monic(&self) -> Poly {
        if self.is_zero() {
            return self.clone();
        }
        self.scale(&(Q::one() / self.lc()))
    }
    pub fn gcd(&self, o: &Poly) -> Poly {
        let (mut a, mut b) = (self.clone(), o.clone());
        while !b.is_zero() {
            let r = a.divmod(&b).1;
            a = b;
            b = r;
        }
        a.monic()
    }
    /// the square-free part (monic): the product of the distinct irreducible factors
    pub fn sqf_part(&self) -> Poly {
        if self.degree() < 1 {
            return self.monic();
        }
        let g = self.gcd(&self.diff());
        self.divmod(&g).0.monic()
    }
    /// Yun's square-free factorization: (content, [(factor, multiplicity)]) with monic factors
    pub fn sqf_list(&self) -> (Q, Vec<(Poly, u32)>) {
        let lc = self.lc();
        let mut out = vec![];
        if self.degree() < 1 {
            return (if self.is_zero() { Q::zero() } else { lc }, out);
        }
        let f = self.monic();
        let fp = f.diff();
        let mut a = f.gcd(&fp);
        let mut b = f.divmod(&a).0;
        let mut c = fp.divmod(&a).0;
        let mut d = c.sub(&b.diff());
        let mut i = 1;
        while b.degree() >= 1 {
            a = b.gcd(&d);
            if a.degree() >= 1 {
                out.push((a.clone(), i));
            }
            b = b.divmod(&a).0;
            c = d.divmod(&a).0;
            d = c.sub(&b.diff());
            i += 1;
        }
        (lc, out)
    }
}

/// An expression as a polynomial in `x` whose coefficients are expressions free of `x`; None if it is not one
fn ex_poly(e: &Ex, x: &str) -> Option<Vec<Ex>> {
    fn has_x(e: &Ex, x: &str) -> bool {
        let mut v = vec![];
        sym::free_symbols(e, &mut v);
        v.iter().any(|s| &**s == x)
    }
    fn padd(a: Vec<Ex>, b: Vec<Ex>) -> Vec<Ex> {
        let n = a.len().max(b.len());
        (0..n).map(|i| sym::add(a.get(i).cloned().unwrap_or_else(|| sym::int(0)), b.get(i).cloned().unwrap_or_else(|| sym::int(0)))).collect()
    }
    fn pmul(a: &[Ex], b: &[Ex]) -> Vec<Ex> {
        let mut c = vec![sym::int(0); a.len() + b.len() - 1];
        for (i, p) in a.iter().enumerate() {
            for (j, q) in b.iter().enumerate() {
                c[i + j] = sym::add(c[i + j].clone(), sym::mul(p.clone(), q.clone()));
            }
        }
        c
    }
    if !has_x(e, x) {
        return Some(vec![e.clone()]);
    }
    match e {
        Ex::Sym(_) => Some(vec![sym::int(0), sym::int(1)]),
        Ex::Add(ts) => {
            let mut acc = vec![sym::int(0)];
            for t in ts.iter() {
                acc = padd(acc, ex_poly(t, x)?);
            }
            Some(acc)
        }
        Ex::Mul(fs) => {
            let mut acc = vec![sym::int(1)];
            for f in fs.iter() {
                acc = pmul(&acc, &ex_poly(f, x)?);
            }
            Some(acc)
        }
        Ex::Pow(b, p) => {
            let Ex::Num(k) = &**p else { return None };
            if !k.is_integer() || k.is_negative() || has_x(p, x) {
                return None;
            }
            let n = k.to_integer().to_u32()?;
            if n > 512 {
                return None;
            }
            let base = ex_poly(b, x)?;
            let mut acc = vec![sym::int(1)];
            for _ in 0..n {
                acc = pmul(&acc, &base);
            }
            Some(acc)
        }
        _ => None,
    }
}

/// sympy's e.is_polynomial(x)
pub fn is_polynomial(e: &Ex, x: &str) -> bool {
    ex_poly(e, x).is_some()
}

/// Poly(e, x) over the rationals: Err(true) if e is not a polynomial in x, Err(false) if a coefficient is irrational
pub fn poly_from(e: &Ex, x: &Rc<str>) -> Result<Poly, bool> {
    let cs = ex_poly(e, x).ok_or(true)?;
    let mut c = vec![];
    for k in cs {
        match k {
            Ex::Num(v) => c.push(v),
            _ => return Err(false),
        }
    }
    Ok(Poly::new(x.clone(), c))
}

// ------------------------------------------------------------------------------------------- exact real roots
fn sign_q(v: &Q) -> i32 {
    if v.is_zero() { 0 } else if v.is_positive() { 1 } else { -1 }
}

/// The Sturm sequence of a square-free polynomial
fn sturm(p: &Poly) -> Vec<Poly> {
    let mut seq = vec![p.clone(), p.diff()];
    while !seq.last().unwrap().is_zero() {
        let n = seq.len();
        let r = seq[n - 2].divmod(&seq[n - 1]).1.neg();
        if r.is_zero() {
            break;
        }
        seq.push(r);
    }
    seq
}

fn sign_changes(seq: &[Poly], x: &Q) -> usize {
    let s: Vec<i32> = seq.iter().map(|p| sign_q(&p.eval_q(x))).filter(|s| *s != 0).collect();
    s.windows(2).filter(|w| w[0] != w[1]).count()
}

/// A real root: exact (rational) or inside an isolating interval (lo, hi) whose ends are not roots
#[derive(Clone, Debug)]
pub enum Root {
    Exact(Q),
    Iso(Q, Q),
}

impl Root {
    pub fn approx(&self, p: &Poly) -> f64 {
        match self {
            Root::Exact(q) => q.to_f64().unwrap_or(f64::NAN),
            Root::Iso(lo, hi) => {
                let (mut lo, mut hi) = (lo.clone(), hi.clone());
                let slo = sign_q(&p.eval_q(&lo));
                let two = Q::from_integer(BigInt::from(2));
                for _ in 0..200 {
                    let (a, b) = (lo.to_f64().unwrap_or(0.0), hi.to_f64().unwrap_or(0.0));
                    let m = (a + b) / 2.0;
                    if m <= a || m >= b {
                        break;
                    }
                    let mid = (&lo + &hi) / &two;
                    let s = sign_q(&p.eval_q(&mid));
                    if s == 0 {
                        return mid.to_f64().unwrap_or(f64::NAN);
                    }
                    if s == slo { lo = mid } else { hi = mid }
                }
                ((&lo + &hi) / two).to_f64().unwrap_or(f64::NAN)
            }
        }
    }
    pub fn as_py(&self, p: &Poly) -> Py {
        match self {
            Root::Exact(q) => ex(sym::num(q.clone())),
            Root::Iso(..) => Py::Float(self.approx(p)),
        }
    }
    /// compare with a rational: -1 below, 0 equal, 1 above (refines an isolating interval as needed)
    fn cmp_q(&mut self, p: &Poly, a: &Q) -> i32 {
        loop {
            match self {
                Root::Exact(q) => return sign_q(&(q.clone() - a)),
                Root::Iso(lo, hi) => {
                    if a <= lo {
                        return 1;
                    }
                    if a >= hi {
                        return -1;
                    }
                    if p.eval_q(a).is_zero() {
                        *self = Root::Exact(a.clone());
                        continue;
                    }
                    if sign_q(&p.eval_q(a)) == sign_q(&p.eval_q(lo)) { *lo = a.clone() } else { *hi = a.clone() }
                }
            }
        }
    }
}

/// The distinct real roots of a polynomial, ascending (each rational root exactly)
pub fn real_roots(p: &Poly) -> Vec<Root> {
    if p.degree() < 1 {
        return vec![];
    }
    let s = p.sqf_part();
    let seq = sturm(&s);
    // Cauchy bound: every root lies in (-B, B)
    let lc = s.lc().abs();
    let mut b = Q::one();
    for k in &s.c[..s.c.len() - 1] {
        b += k.abs() / &lc;
    }
    let mut out = vec![];
    let two = Q::from_integer(BigInt::from(2));
    let mut stack = vec![(-b.clone(), b)];
    while let Some((lo, hi)) = stack.pop() {
        let n = sign_changes(&seq, &lo) as i64 - sign_changes(&seq, &hi) as i64;
        if n == 0 {
            continue;
        }
        let mid = (&lo + &hi) / &two;
        if n == 1 {
            out.push(Root::Iso(lo, hi));
            continue;
        }
        if s.eval_q(&mid).is_zero() {
            out.push(Root::Exact(mid.clone()));
            // the root at mid is counted in neither half below: shift the cut off it
            let eps = (&hi - &lo) / Q::from_integer(BigInt::from(1024));
            let (m1, m2) = (&mid - &eps, &mid + &eps);
            stack.push((m2, hi));
            stack.push((lo, m1));
            continue;
        }
        stack.push((mid.clone(), hi));
        stack.push((lo, mid));
    }
    // a root exactly at an interval end of an isolating interval: Sturm counts in (lo, hi]; settle ends
    let mut roots = vec![];
    for r in out {
        match r {
            Root::Iso(_, hi) if s.eval_q(&hi).is_zero() => roots.push(Root::Exact(hi)),
            Root::Iso(lo, hi) => roots.push(rational_or_iso(&s, lo, hi)),
            e => roots.push(e),
        }
    }
    roots.sort_by(|a, b| a.approx(&s).total_cmp(&b.approx(&s)));
    roots
}

/// An isolated root: exact if it is rational (its denominator divides the leading coefficient of the integer
/// polynomial), else the interval
fn rational_or_iso(s: &Poly, mut lo: Q, mut hi: Q) -> Root {
    // integer form: the lcm of denominators times the polynomial
    let mut den = BigInt::one();
    for k in &s.c {
        den = num_integer::Integer::lcm(&den, k.denom());
    }
    let lead = (s.lc() * Q::from_integer(den)).to_integer().abs();
    // refine until the interval is narrower than 1/(2 lead^2): two rationals with denominators <= lead differ by
    // at least 1/lead^2, so at most one candidate lies inside
    let target = Q::new(BigInt::one(), BigInt::from(2) * &lead * &lead);
    let two = Q::from_integer(BigInt::from(2));
    let slo = sign_q(&s.eval_q(&lo));
    let mut guard = 0;
    while &hi - &lo > target && guard < 4000 {
        let mid = (&lo + &hi) / &two;
        let sm = sign_q(&s.eval_q(&mid));
        if sm == 0 {
            return Root::Exact(mid);
        }
        if sm == slo { lo = mid } else { hi = mid }
        guard += 1;
    }
    let mid = (&lo + &hi) / &two;
    if let Ok(c) = crate::pyfrac::limit_denominator(&mid, &lead) {
        if c > lo && c < hi && s.eval_q(&c).is_zero() {
            return Root::Exact(c);
        }
    }
    Root::Iso(lo, hi)
}

/// The number of distinct real roots in the closed interval [a, b]
pub fn count_roots(p: &Poly, a: &Q, b: &Q) -> usize {
    real_roots(p).into_iter().filter_map(|mut r| {
        let s = p.sqf_part();
        (r.cmp_q(&s, a) >= 0 && r.cmp_q(&s, b) <= 0).then_some(())
    }).count()
}

// ================================================================================================= evaluation
/// Why an evaluation stopped, named as the package's evaluator names it
#[derive(Debug, Clone, PartialEq)]
pub enum Stop {
    ZeroDivision,
    Overflow,
    Value,
    NotReal,
}

impl Stop {
    fn name(&self) -> &'static str {
        match self {
            Stop::ZeroDivision => "ZeroDivisionError",
            Stop::Overflow => "OverflowError",
            Stop::Value | Stop::NotReal => "ValueError",
        }
    }
}

/// The value as the package's _compile evaluates it with mpmath (use_mp=True): infinities pass (log(0) = -inf),
/// division by an exact zero stops (ZeroDivisionError), and a value that turns complex is not real. Double precision.
pub fn eval_mp(e: &Ex, var: &dyn Fn(&str) -> Option<f64>) -> Result<f64, Stop> {
    use Fun::*;
    Ok(match e {
        Ex::Num(v) => v.to_f64().unwrap_or(f64::NAN),
        Ex::Pi => std::f64::consts::PI,
        Ex::E => std::f64::consts::E,
        Ex::Sym(s) => var(s).ok_or(Stop::Value)?,
        Ex::Zoo | Ex::Nan => return Err(Stop::Value),
        Ex::Add(ts) => {
            let mut s = eval_mp(&ts[0], var)?;
            for t in &ts[1..] {
                s += eval_mp(t, var)?;
            }
            s
        }
        Ex::Mul(fs) => {
            let mut p = eval_mp(&fs[0], var)?;
            for f in &fs[1..] {
                p *= eval_mp(f, var)?;
            }
            p
        }
        Ex::Pow(b, p) => {
            let bv = eval_mp(b, var)?;
            if let Ex::Num(k) = &**p {
                if k.is_integer() {
                    let k = k.to_integer().to_i32().unwrap_or(if k.is_negative() { i32::MIN } else { i32::MAX });
                    if bv == 0.0 && k < 0 {
                        return Err(Stop::ZeroDivision);
                    }
                    return Ok(bv.powi(k));
                }
                if *k == sym::q(1, 2) {
                    if bv < 0.0 {
                        return Err(Stop::NotReal);
                    }
                    return Ok(bv.sqrt());
                }
            }
            let pv = eval_mp(p, var)?;
            if bv == 0.0 && pv < 0.0 {
                return Err(Stop::ZeroDivision);
            }
            if bv < 0.0 && pv.is_finite() && pv != pv.trunc() {
                return Err(Stop::NotReal);
            }
            bv.powf_lm(pv)
        }
        Ex::Fun(f, a) => {
            let x = eval_mp(a, var)?;
            match f {
                Sin => x.sin_lm(),
                Cos => x.cos_lm(),
                Tan => x.tan_lm(),
                Sec => 1.0 / x.cos_lm(),
                Csc | Cot if x.sin_lm() == 0.0 => return Err(Stop::ZeroDivision),
                Csc => 1.0 / x.sin_lm(),
                Cot => x.cos_lm() / x.sin_lm(),
                Asin | Acos if x.abs() > 1.0 => return Err(Stop::NotReal),
                Asin => x.asin_lm(),
                Acos => x.acos_lm(),
                Atan => x.atan_lm(),
                Sinh => x.sinh_lm(),
                Cosh => x.cosh_lm(),
                Tanh => x.tanh_lm(),
                Exp => x.exp_lm(),
                Log if x < 0.0 => return Err(Stop::NotReal),
                Log if x == 0.0 => f64::NEG_INFINITY,
                Log => x.ln_lm(),
                Abs => x.abs(),
            }
        }
    })
}

/// The value as the package's _compile evaluates it with Python floats and the math module (use_mp=False):
/// math's domain errors (ValueError), overflow (OverflowError) and division by zero (ZeroDivisionError).
pub fn eval_float(e: &Ex, var: &dyn Fn(&str) -> Option<f64>) -> Result<f64, Stop> {
    eval_float_g(e, var, false)
}

/// eval_float, with the terms of a sum added exactly rounded (math.fsum) when `fsum` is set
pub fn eval_float_g(e: &Ex, var: &dyn Fn(&str) -> Option<f64>, fsum: bool) -> Result<f64, Stop> {
    use Fun::*;
    let eval_float = |e: &Ex, var: &dyn Fn(&str) -> Option<f64>| eval_float_g(e, var, fsum);
    let over = |v: f64, x: f64| if v.is_infinite() && x.is_finite() { Err(Stop::Overflow) } else { Ok(v) };
    Ok(match e {
        Ex::Num(v) => v.to_f64().unwrap_or(f64::NAN),
        Ex::Pi => std::f64::consts::PI,
        Ex::E => std::f64::consts::E,
        Ex::Sym(s) => var(s).ok_or(Stop::Value)?,
        Ex::Zoo | Ex::Nan => return Err(Stop::Value),
        Ex::Add(ts) if fsum => {
            let v: Vec<Py> = ts.iter().map(|t| eval_float(t, var).map(Py::Float)).collect::<Result<_, _>>()?;
            py::fsum(&Py::list(v)).and_then(|r| r.f()).map_err(|_| Stop::Overflow)?
        }
        Ex::Add(ts) => {
            let mut s = eval_float(&ts[0], var)?;
            for t in &ts[1..] {
                s += eval_float(t, var)?;
            }
            s
        }
        Ex::Mul(fs) => {
            let mut p = eval_float(&fs[0], var)?;
            for f in &fs[1..] {
                p *= eval_float(f, var)?;
            }
            p
        }
        Ex::Pow(b, p) => {
            let bv = eval_float(b, var)?;
            if let Ex::Num(k) = &**p {
                if k.is_integer() {
                    let kk = k.to_integer().to_i32().unwrap_or(if k.is_negative() { i32::MIN } else { i32::MAX });
                    if bv == 0.0 && kk < 0 {
                        return Err(Stop::ZeroDivision);
                    }
                    let r = bv.powf_lm(kk as f64);
                    return over(r, bv);
                }
                if *k == sym::q(1, 2) {
                    if bv < 0.0 {
                        return Err(Stop::Value);
                    }
                    return Ok(bv.sqrt());
                }
            }
            let pv = eval_float(p, var)?;
            if bv == 0.0 && pv < 0.0 {
                return Err(Stop::ZeroDivision);
            }
            if bv < 0.0 && pv.is_finite() && pv != pv.trunc() {
                return Err(Stop::Value);                            // a complex result: "not a real number"
            }
            let r = bv.powf_lm(pv);
            if r.is_infinite() && bv.is_finite() && pv.is_finite() {
                return Err(Stop::Overflow);
            }
            r
        }
        Ex::Fun(f, a) => {
            let x = eval_float(a, var)?;
            match f {
                Sin | Cos | Tan | Sec | Csc | Cot if x.is_infinite() => return Err(Stop::Value),
                Sin => x.sin_lm(),
                Cos => x.cos_lm(),
                Tan => x.tan_lm(),
                Sec => {
                    let c = x.cos_lm();
                    if c == 0.0 { return Err(Stop::ZeroDivision) }
                    1.0 / c
                }
                Csc | Cot if x.sin_lm() == 0.0 => return Err(Stop::ZeroDivision),
                Csc => 1.0 / x.sin_lm(),
                Cot => x.cos_lm() / x.sin_lm(),
                Asin | Acos if x.abs() > 1.0 => return Err(Stop::Value),
                Asin => x.asin_lm(),
                Acos => x.acos_lm(),
                Atan => x.atan_lm(),
                Sinh => over(x.sinh_lm(), x)?,
                Cosh => over(x.cosh_lm(), x)?,
                Tanh => x.tanh_lm(),
                Exp => over(x.exp_lm(), x)?,
                Log if x <= 0.0 => return Err(Stop::Value),
                Log => x.ln_lm(),
                Abs => x.abs(),
            }
        }
    })
}

fn has_undefined(e: &Ex) -> bool {
    sym::has_undefined(e)
}

/// _compile(e, names, use_mp): a function of the variables' values (ValueError as the package words it)
pub fn compile(e: &Ex, names: &[Rc<str>], use_mp: bool) -> R {
    let mut free = vec![];
    sym::free_symbols(e, &mut free);
    if let Some(s) = free.iter().find(|s| !names.contains(s)) {
        return Err(verr(format!("unexpected variable {s}")));
    }
    if let Some(n) = undefined_name(e) {
        return Err(verr(format!("cannot evaluate {n}")));
    }
    let (e, names) = (e.clone(), names.to_vec());
    let (e2, names2) = (e.clone(), names.clone());
    let cp = crate::pympf::prec();       // the rational constants take the precision of this moment (_compile)
    let f = py::func(move |a: &[Py]| -> R {
        if use_mp && crate::pympf::extended() {
            let vals: Vec<astro_float::BigFloat> = a.iter()
                .map(|v| crate::pympf::of(v).ok_or_else(|| terr("a number is required"))).collect::<Result<_, _>>()?;
            let var = |s: &str| names.iter().position(|n| &**n == s).and_then(|i| vals.get(i).cloned());
            return match crate::pympf::eval_ex_c(&e, &var, cp) {
                Ok(v) => Ok(crate::pympf::wrap(v)),
                Err(Stop::NotReal) => Err(verr("the expression is not real there")),
                Err(s) => Err(verr(format!("the expression is not defined there ({})", s.name()))),
            };
        }
        let vals: Vec<f64> = a.iter().map(|v| v.f()).collect::<Result<_, _>>()?;
        let var = |s: &str| names.iter().position(|n| &**n == s).and_then(|i| vals.get(i).copied());
        let r = if use_mp { eval_mp(&e, &var) } else { eval_float(&e, &var) };
        match r {
            Ok(v) => Ok(Py::Float(v)),
            Err(Stop::NotReal) if use_mp => Err(verr("the expression is not real there")),
            Err(s) => Err(verr(format!("the expression is not defined there ({})", s.name()))),
        }
    });
    register(&f, e2, names2);
    Ok(f)
}

// ------------------------------------------------------------------------------------------- compiled functions
type Compiled = (std::rc::Weak<dyn Fn(&[Py]) -> R>, Ex, Vec<Rc<str>>);

thread_local! {
    /// the expression behind each function _compile returned (while it lives): mp.diff and mp.taylor of such a
    /// function differentiate the expression by automatic differentiation
    static COMPILED: std::cell::RefCell<Vec<Compiled>> = const { std::cell::RefCell::new(Vec::new()) };
}

fn register(f: &Py, e: Ex, names: Vec<Rc<str>>) {
    if let Py::Func(rc) = f {
        COMPILED.with(|c| {
            let mut c = c.borrow_mut();
            c.retain(|(w, _, _)| w.strong_count() > 0);
            if c.len() < 4096 {
                c.push((Rc::downgrade(rc), e, names));
            }
        });
    }
}

/// The expression and variables of a function _compile made, if `f` is one
pub fn compiled(f: &Py) -> Option<(Ex, Vec<Rc<str>>)> {
    let Py::Func(rc) = f else { return None };
    let addr = Rc::as_ptr(rc) as *const ();
    COMPILED.with(|c| c.borrow().iter()
        .find(|(w, _, _)| w.as_ptr() as *const () == addr && w.strong_count() > 0)
        .map(|(_, e, n)| (e.clone(), n.clone())))
}

fn stop_err(s: Stop) -> PyErr {
    match s {
        Stop::ZeroDivision => PyErr::ZeroDivision,
        Stop::NotReal => verr("the expression is not real there"),
        other => verr(format!("the expression is not defined there ({})", other.name())),
    }
}

fn factorial(n: usize) -> f64 {
    (2..=n).fold(1.0, |a, k| a * k as f64)
}

/// The m-th derivative along the direction d of a compiled expression at p: m! times the t^m coefficient of
/// e(p + t d)
fn directional(e: &Ex, names: &[Rc<str>], p: &[f64], d: &[f64], m: usize) -> Result<f64, PyErr> {
    let seed = |s: &str| names.iter().position(|n| &**n == s).map(|i| (p[i], d[i]));
    let c = taylor(e, &seed, m).map_err(stop_err)?;
    Ok(c[m] * factorial(m))
}

/// All multi-indices of k entries summing to `left`
fn multi_indices(k: usize, left: usize, cur: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
    if k == 1 {
        cur.push(left);
        out.push(cur.clone());
        cur.pop();
        return;
    }
    for a in 0..=left {
        cur.push(a);
        multi_indices(k - 1, left - a, cur, out);
        cur.pop();
    }
}

/// The partial derivative of the given orders at p. One variable: a directional derivative along its axis. Mixed:
/// from directional derivatives of the same total order m along integer directions (entries -2..=2), since
/// D_d^m f = sum over |a| = m of m!/a! d^a f_a; the partials of order m are solved by least squares.
fn partial(e: &Ex, names: &[Rc<str>], p: &[f64], orders: &[usize]) -> Result<f64, PyErr> {
    let nv = names.len();
    let m: usize = orders.iter().sum();
    let nz: Vec<usize> = (0..nv).filter(|&i| orders[i] > 0).collect();
    if m == 0 {
        let seed = |s: &str| names.iter().position(|n| &**n == s).map(|i| (p[i], 0.0));
        return Ok(taylor(e, &seed, 0).map_err(stop_err)?[0]);
    }
    if nz.len() == 1 {
        let mut d = vec![0.0; nv];
        d[nz[0]] = 1.0;
        return directional(e, names, p, &d, m);
    }
    let k = nz.len();
    let mut alphas = vec![];
    multi_indices(k, m, &mut vec![], &mut alphas);
    let want = alphas.iter().position(|a| a.iter().zip(&nz).all(|(x, &i)| *x == orders[i])).unwrap();
    let mut dirs: Vec<Vec<f64>> = vec![];
    let total = 5usize.pow(k as u32);
    for code in 0..total {
        let mut c = code;
        let d: Vec<f64> = (0..k).map(|_| { let v = (c % 5) as f64 - 2.0; c /= 5; v }).collect();
        if d[0] > 0.0 {
            dirs.push(d);
        }
    }
    let coef = |d: &[f64], a: &[usize]| -> f64 {
        let mut c = factorial(m);
        for (x, &ai) in d.iter().zip(a) {
            c *= x.powi(ai as i32) / factorial(ai);
        }
        c
    };
    let na = alphas.len();
    let mut ata = vec![vec![0.0; na]; na];
    let mut atb = vec![0.0; na];
    for d in &dirs {
        let mut full = vec![0.0; nv];
        for (t, &i) in nz.iter().enumerate() {
            full[i] = d[t];
        }
        let b = directional(e, names, p, &full, m)?;
        let row: Vec<f64> = alphas.iter().map(|a| coef(d, a)).collect();
        for i in 0..na {
            atb[i] += row[i] * b;
            for j in 0..na {
                ata[i][j] += row[i] * row[j];
            }
        }
    }
    for c in 0..na {
        let piv = (c..na).max_by(|&i, &j| ata[i][c].abs().total_cmp(&ata[j][c].abs())).unwrap();
        ata.swap(c, piv);
        atb.swap(c, piv);
        for r in c + 1..na {
            let f = ata[r][c] / ata[c][c];
            for j in c..na {
                ata[r][j] -= f * ata[c][j];
            }
            atb[r] -= f * atb[c];
        }
    }
    let mut x = vec![0.0; na];
    for r in (0..na).rev() {
        let s: f64 = (r + 1..na).map(|j| ata[r][j] * x[j]).sum();
        x[r] = (atb[r] - s) / ata[r][r];
    }
    Ok(x[want])
}

/// Ridders' extrapolated central difference of a first derivative (for a function with no expression behind it)
fn ridders(f: &Py, x: f64) -> Result<f64, PyErr> {
    let ev = |t: f64| -> Result<f64, PyErr> { py::call(f, &[Py::Float(t)])?.f() };
    let (con, con2, ntab) = (1.4, 1.96, 12);
    let mut h = 0.1 * x.abs().max(1.0);
    let mut a = vec![vec![0.0; ntab]; ntab];
    a[0][0] = (ev(x + h)? - ev(x - h)?) / (2.0 * h);
    let (mut best, mut err) = (a[0][0], f64::INFINITY);
    for i in 1..ntab {
        h /= con;
        a[0][i] = (ev(x + h)? - ev(x - h)?) / (2.0 * h);
        let mut fac = con2;
        for j in 1..=i {
            a[j][i] = (a[j - 1][i] * fac - a[j - 1][i - 1]) / (fac - 1.0);
            fac *= con2;
            let e = (a[j][i] - a[j - 1][i]).abs().max((a[j][i] - a[j - 1][i - 1]).abs());
            if e <= err {
                err = e;
                best = a[j][i];
            }
        }
        if (a[i][i] - a[i - 1][i - 1]).abs() >= 2.0 * err {
            break;
        }
    }
    Ok(best)
}

/// mpmath.diff(f, x, n=1) and diff(f, (x, y, ..), (i, j, ..)): automatic differentiation of a compiled
/// expression; for any other function, a first derivative by Ridders' extrapolation (higher orders not modelled)
pub fn mp_diff(a: &[Py], kws: &[(String, Py)]) -> R {
    let f = a.first().ok_or_else(|| terr("diff() needs a function"))?;
    let at = a.get(1).ok_or_else(|| terr("diff() needs a point"))?;
    let n = a.get(2).cloned().or_else(|| kws.iter().find(|(k, _)| k == "n").map(|(_, v)| v.clone()));
    let point: Vec<f64> = match at {
        Py::Tuple(_) | Py::List(_) => at.items()?.iter().map(|v| v.f()).collect::<Result<_, _>>()?,
        v => vec![v.f()?],
    };
    let orders: Vec<usize> = match &n {
        None => (0..point.len()).map(|i| usize::from(i == 0)).collect(),
        Some(o @ (Py::Tuple(_) | Py::List(_))) => {
            o.items()?.iter().map(|v| v.f().map(|x| x as usize)).collect::<Result<_, _>>()?
        }
        Some(v) => vec![v.f()? as usize],
    };
    if orders.len() != point.len() {
        return Err(terr("diff(): the orders do not match the point"));
    }
    if let Some((e, names)) = compiled(f) {
        if names.len() == point.len() {
            if crate::pympf::extended() {
                // inside a workdps block: the exact derivative evaluated at the working precision
                let mut d = e.clone();
                for (v, &k) in names.iter().zip(&orders) {
                    for _ in 0..k {
                        d = sym::diff(&d, v);
                    }
                }
                let pts: Vec<Py> = match at {
                    Py::Tuple(_) | Py::List(_) => at.items()?,
                    v => vec![v.clone()],
                };
                let g = compile(&d, &names, true)?;
                return py::call(&g, &pts);
            }
            return partial(&e, &names, &point, &orders).map(Py::Float);
        }
    }
    if point.len() == 1 && orders[0] == 1 && crate::pympf::extended() {
        // mpmath's own step method (differentiation.hsteps): (f(x + h) - f(x - h)) / 2h with h = 2^-(prec + 10),
        // the values taken at (prec + 20) * 2 bits so the difference keeps the working precision, rounded back
        let x = match at { Py::Tuple(_) | Py::List(_) => at.items()?[0].clone(), v => v.clone() };
        let prec = crate::pympf::prec();
        let v = {
            let _g = crate::pympf::scope("workprec", &Py::Int(((prec + 20) * 2) as i64))?;
            let hh = crate::pympf::mpf(&Py::Float(2f64.powi(-(prec as i32) - 10)))?;
            let up = py::call(f, &[py::add(&x, &hh)?])?;
            let dn = py::call(f, &[py::sub(&x, &hh)?])?;
            py::truediv(&py::sub(&up, &dn)?, &py::mul(&Py::Int(2), &hh)?)?
        };
        return py::add(&v, &crate::pympf::mpf(&Py::Int(0))?);
    }
    if point.len() == 1 && orders[0] == 1 {
        return ridders(f, point[0]).map(Py::Float);
    }
    if point.len() == 1 && orders[0] == 0 {
        return py::call(f, &[Py::Float(point[0])]);
    }
    Err(unsup("mpmath.diff of higher order of a function with no expression"))
}

/// mpmath.taylor(f, x, n): the Taylor coefficients f^(k)(x)/k!, k = 0..n, of a compiled expression
pub fn mp_taylor(a: &[Py]) -> R {
    let f = a.first().ok_or_else(|| terr("taylor() needs a function"))?;
    let x = a.get(1).ok_or_else(|| terr("taylor() needs a point"))?.f()?;
    let n = a.get(2).ok_or_else(|| terr("taylor() needs an order"))?.f()? as usize;
    let Some((e, names)) = compiled(f) else { return Err(unsup("mpmath.taylor of a function with no expression")) };
    if names.len() != 1 {
        return Err(unsup("mpmath.taylor of several variables"));
    }
    let c = taylor(&e, &|s: &str| (s == &*names[0]).then_some((x, 1.0)), n).map_err(stop_err)?;
    Ok(Py::list(c.into_iter().map(Py::Float).collect()))
}

fn undefined_name(e: &Ex) -> Option<&'static str> {
    match e {
        Ex::Zoo => Some("ComplexInfinity"),
        Ex::Nan => Some("NaN"),
        Ex::Add(v) | Ex::Mul(v) => v.iter().find_map(undefined_name),
        Ex::Pow(b, p) => undefined_name(b).or_else(|| undefined_name(p)),
        Ex::Fun(_, a) => undefined_name(a),
        _ => None,
    }
}

fn fin(v: f64, what: &str) -> R {
    if !v.is_finite() {
        return Err(verr(format!("{what} is not finite")));
    }
    Ok(Py::Float(v))
}

/// at(e, names, *vals): the value at a point (ValueError where undefined or not finite)
pub fn at(e: &Ex, names: &[Rc<str>], vals: &[Py]) -> R {
    if has_undefined(e) {
        return Err(verr("the expression is undefined there"));
    }
    let _g = crate::pympf::scope("workdps", &Py::Int(30))?;
    let f = compile(e, names, true)?;           // built inside the block, as the package's at() does
    let v = py::call(&f, vals)?.f()?;
    fin(v, "the result")
}

/// quad(f, a, b, points=None) of the calculus package: mpmath's tanh-sinh quadrature at 20 digits with its error
/// estimate; if that is above 1e-15 of the value, again on 16 equal pieces (the given points instead, if any) up to
/// degree 10; ValueError if the estimate is still above 1e-12 of the value. A finite interval runs mpmath's own
/// algorithm (ts_quad_mp), so the value carries the same digits, the same quadrature error where the integrand is
/// hard (a long oscillating interval) and an exact 0 for an integrand odd about the midpoint; an infinite end keeps
/// the double-precision adaptive rule.
pub fn quad(f: &Py, a: f64, b: f64, points: &Py) -> R {
    let mut cuts = vec![a];
    if !matches!(points, Py::None) {
        for p in points.items()? {
            cuts.push(p.f()?);
        }
    }
    cuts.push(b);
    if cuts.iter().all(|c| c.is_finite()) {
        let _g = crate::pympf::scope("workdps", &Py::Int(20))?;
        let pts: Vec<Py> = cuts.iter().map(|c| crate::pympf::mpf(&Py::Float(*c))).collect::<Result<_, _>>()?;
        let p = crate::pympf::prec();
        let degree = (4.0 + (p as f64 / 30.0).log2().max(0.0)) as usize + 2;     // mpmath's guess_degree
        let (mut v, mut err) = ts_quad_mp(f, &pts, degree)?;
        if err > 1e-15 * v.f()?.abs().max(1.0) {
            let pts2 = if matches!(points, Py::None) {
                // mp.linspace(a, b, 17)
                let step = py::truediv(&py::sub(&pts[pts.len() - 1], &pts[0])?, &crate::pympf::mpf(&Py::Int(16))?)?;
                let mut y = (0..17).map(|i| py::add(&py::mul(&Py::Int(i), &step)?, &pts[0])).collect::<Result<Vec<_>, _>>()?;
                y[16] = pts[pts.len() - 1].clone();
                y
            } else {
                pts.clone()
            };
            (v, err) = ts_quad_mp(f, &pts2, 10)?;
        }
        if err > 1e-12 * v.f()?.abs().max(1.0) {
            return Err(verr("the integral did not converge numerically"));
        }
        return fin(v.f()?, "the integral");
    }
    quad_f64(f, a, b, points)
}

/// mpmath's tanh-sinh nodes of a degree on [-1, 1] for a precision (TanhSinh.calc_nodes at prec + 40 bits): the
/// positive abscissas with their weights (the negative ones mirror them; degree 1 adds x = 0, weight pi/2)
fn ts_nodes_mp(degree: usize, prec: usize) -> Result<Rc<Vec<(Py, Py)>>, PyErr> {
    thread_local!(static CACHE: std::cell::RefCell<std::collections::HashMap<(usize, usize), Rc<Vec<(Py, Py)>>>> =
        std::cell::RefCell::new(std::collections::HashMap::new()));
    if let Some(v) = CACHE.with(|c| c.borrow().get(&(degree, prec)).cloned()) {
        return Ok(v);
    }
    let _g = crate::pympf::scope("workprec", &Py::Int((prec + 40) as i64))?;
    let exp = |x: &Py| -> R { crate::pympf::func("exp", x).unwrap_or_else(|| Err(unsup("mp exp"))) };
    let one = crate::pympf::mpf(&Py::Int(1))?;
    let two = crate::pympf::mpf(&Py::Int(2))?;
    let pi = crate::pympf::constant("pi").ok_or_else(|| unsup("mp pi"))?;
    let pi4 = py::truediv(&pi, &crate::pympf::mpf(&Py::Int(4))?)?;
    let tol = 2f64.powi(-(prec as i32) - 10);
    let t0 = crate::pympf::mpf(&Py::Float(2f64.powi(-(degree as i32))))?;
    let h = if degree == 1 { t0.clone() } else { py::mul(&t0, &two)? };
    let expt0 = exp(&t0)?;
    let mut a = py::mul(&pi4, &expt0)?;
    let mut b = py::truediv(&pi4, &expt0)?;
    let udelta = exp(&h)?;
    let urdelta = py::truediv(&one, &udelta)?;
    let mut out = vec![];
    for _ in 0..=(20usize << degree) {
        let c = exp(&py::sub(&a, &b)?)?;
        let d = py::truediv(&one, &c)?;
        let co = py::truediv(&py::add(&c, &d)?, &two)?;
        let si = py::truediv(&py::sub(&c, &d)?, &two)?;
        let x = py::truediv(&si, &co)?;
        let w = py::truediv(&py::add(&a, &b)?, &py::mul(&co, &co)?)?;
        if py::sub(&x, &one)?.f()?.abs() <= tol {
            break;
        }
        out.push((x, w));
        a = py::mul(&a, &udelta)?;
        b = py::mul(&b, &urdelta)?;
    }
    let v = Rc::new(out);
    CACHE.with(|c| c.borrow_mut().insert((degree, prec), v.clone()));
    Ok(v)
}

/// mpmath.quad(f, pts, error=True, maxdegree=m) with the tanh-sinh rule at the working precision: per piece, degrees
/// 1..m of the step sum (each reusing the previous one, the nodes mapped linearly, the step summed exactly at 20 bits
/// above the working precision) until mpmath's error extrapolation is below eps/8; (value, summed error estimate)
fn ts_quad_mp(f: &Py, pts: &[Py], max_degree: usize) -> Result<(Py, f64), PyErr> {
    let prec = crate::pympf::prec();
    let eps = 2f64.powi(1 - prec as i32) / 8.0;
    let _g = crate::pympf::scope("workprec", &Py::Int((prec + 20) as i64))?;
    let two = crate::pympf::mpf(&Py::Int(2))?;
    let pi2 = {
        // the degree-1 weight pi/2, as calc_nodes computes it (40 bits above the working precision)
        let _x = crate::pympf::scope("workprec", &Py::Int((prec + 40) as i64))?;
        let pi = crate::pympf::constant("pi").ok_or_else(|| unsup("mp pi"))?;
        py::truediv(&pi, &crate::pympf::mpf(&Py::Int(2))?)?
    };
    let call = |x: Py| -> R {
        let v = py::call(f, &[x])?;
        if let Py::Complex(..) = v {
            return Err(verr("the expression is not real there"));
        }
        Ok(v)
    };
    let mut total = crate::pympf::mpf(&Py::Int(0))?;
    let mut total_err = 0.0;
    for w in pts.windows(2) {
        let (a, b) = (&w[0], &w[1]);
        if py::sub(a, b)?.f()? == 0.0 {
            continue;
        }
        let c = py::truediv(&py::sub(b, a)?, &two)?;
        let d = py::truediv(&py::add(b, a)?, &two)?;
        let mut results: Vec<Py> = vec![];
        let mut err = 0.0;
        for degree in 1..=max_degree {
            let nodes = ts_nodes_mp(degree, prec)?;
            // (weight, value) pairs: nodes and weights mapped at the working precision, the integrand evaluated there
            let mut pairs = vec![];
            if degree == 1 {
                pairs.push((py::mul(&c, &pi2)?, call(d.clone())?));
            }
            for (x, wx) in nodes.iter() {
                let cx = py::mul(&c, x)?;
                let cw = py::mul(&c, wx)?;
                pairs.push((cw.clone(), call(py::add(&d, &cx)?)?));
                pairs.push((cw, call(py::sub(&d, &cx)?)?));
            }
            // mpmath's fdot: the products and their sum exact, rounded once
            let fdot = {
                let _x = crate::pympf::scope("workprec", &Py::Int((4 * prec + 600) as i64))?;
                let terms: Vec<Py> = pairs.iter().map(|(w, v)| py::mul(w, v)).collect::<Result<_, _>>()?;
                crate::pympf::fsum(&terms).unwrap_or_else(|| Err(unsup("mp fsum")))?
            };
            let fdot = py::add(&fdot, &crate::pympf::mpf(&Py::Int(0))?)?;
            let h =crate::pympf::mpf(&Py::Float(2f64.powi(-(degree as i32))))?;
            let mut s = fdot;
            if let Some(prev) = results.last() {
                s = py::add(&py::truediv(prev, &py::mul(&h, &two)?)?, &s)?;
            }
            results.push(py::mul(&h, &s)?);
            if degree > 1 {
                let n = results.len();
                let gap = |i: usize, j: usize| -> Result<f64, PyErr> { Ok(py::sub(&results[i], &results[j])?.f()?.abs()) };
                err = if n == 2 {
                    gap(1, 0)?
                } else if gap(n - 1, n - 2)? == 0.0 && gap(n - 2, n - 3)? == 0.0 {
                    0.0
                } else {
                    let d1 = gap(n - 1, n - 2)?.log10();
                    let d2 = gap(n - 1, n - 3)?.log10();
                    let d4 = 0f64.min((d1 * d1 / d2).max(2.0 * d1).max(-(prec as f64)));
                    10f64.powi(if d4.is_nan() { 0 } else { d4.trunc() as i32 })
                };
                if err <= eps {
                    break;
                }
            }
        }
        total = py::add(&total, results.last().unwrap())?;
        total_err += err;
    }
    Ok((total, total_err))
}

fn quad_f64(f: &Py, a: f64, b: f64, points: &Py) -> R {
    let mut pts = vec![a];
    if !matches!(points, Py::None) {
        for p in points.items()? {
            pts.push(p.f()?);
        }
    }
    pts.push(b);
    let (mut v, mut err) = (0.0, 0.0);
    for w in pts.windows(2) {
        let ((r, im), e) = crate::pymp::integrate(f, w[0], w[1])?;
        if im != 0.0 {
            return Err(verr("the expression is not real there"));
        }
        v += r;
        err += e;
    }
    if !(err <= 1e-12 * v.abs().max(1.0)) {
        return Err(verr("the integral did not converge numerically"));
    }
    fin(v, "the integral")
}

// ------------------------------------------------------------------------------------------- continuity
/// A condition the variable must meet on the whole interval for the expression to be real and continuous there
enum Cond {
    NonZero(Ex),
    Pos(Ex),
    NonNeg(Ex),
    InUnit(Ex),
}

fn has_var(e: &Ex, x: &str) -> bool {
    let mut v = vec![];
    sym::free_symbols(e, &mut v);
    v.iter().any(|s| &**s == x)
}

/// a*x + b with constant a, b (sympy solves a trigonometric equation of such an argument)
fn is_linear(e: &Ex, x: &str) -> bool {
    match ex_poly(e, x) {
        Some(cs) => cs.len() <= 2 && cs.iter().all(|c| !has_var(c, x)),
        None => false,
    }
}

/// A rational function of x (numerator and denominator polynomials with constant coefficients)
fn is_rational_fn(e: &Ex, x: &str) -> bool {
    match e {
        Ex::Sym(_) | Ex::Num(_) | Ex::Pi | Ex::E => true,
        Ex::Add(v) | Ex::Mul(v) => v.iter().all(|t| is_rational_fn(t, x)),
        Ex::Pow(b, p) => matches!(&**p, Ex::Num(k) if k.is_integer()) && is_rational_fn(b, x),
        other => !has_var(other, x),
    }
}

/// sympy can solve g = 0 (or g >= 0) for x: a rational function, a trigonometric function of a linear argument, a
/// monotone function (exp, log, sinh, tanh, atan, roots) of such, each up to a constant factor and a constant term
fn solvable(g: &Ex, x: &str) -> bool {
    use Fun::*;
    if !has_var(g, x) || is_rational_fn(g, x) {
        return true;
    }
    match g {
        Ex::Add(v) => {
            let var: Vec<&Ex> = v.iter().filter(|t| has_var(t, x)).collect();
            var.len() == 1 && solvable(var[0], x)
        }
        Ex::Mul(v) => {
            let var: Vec<&Ex> = v.iter().filter(|t| has_var(t, x)).collect();
            var.len() == 1 && solvable(var[0], x)
        }
        Ex::Fun(Sin | Cos, a) => is_linear(a, x) || matches!(&**a, Ex::Fun(Abs, u) if is_linear(u, x)),
        Ex::Fun(Exp | Log | Sinh | Cosh | Tanh | Atan | Asin | Acos, a) => solvable(a, x),
        Ex::Pow(b, p) => matches!(&**p, Ex::Num(_)) && solvable(b, x),
        _ => false,
    }
}

/// What sympy's continuous_domain does with an expression: the conditions on x it imposes; Err(true) where sympy
/// raises (an inequality it cannot solve, a constant that is not real: the package then uses its 401-point grid),
/// Err(false) where the expression is certainly not continuous on any interval (a constant constraint that fails,
/// a singularity sympy cannot locate: a ConditionSet is removed from the interval)
fn conditions(e: &Ex, out: &mut Vec<Cond>, x: &str) -> Result<(), bool> {
    use Fun::*;
    // constraints (continuous_domain): on every atom, constants included
    fn constraints(e: &Ex, out: &mut Vec<Cond>, x: &str) -> Result<(), bool> {
        let check_const = |g: &Ex, ok: &dyn Fn(f64) -> bool| -> Result<(), bool> {
            match eval_mp(g, &|_| None) {
                Ok(v) if v.is_finite() => if ok(v) { Ok(()) } else { Err(false) },
                _ => Err(true),
            }
        };
        fn push(out: &mut Vec<Cond>, c: Cond, g: &Ex, x: &str) -> Result<(), bool> {
            if !solvable(g, x) {
                return Err(true);
            }
            out.push(c);
            Ok(())
        }
        match e {
            Ex::Add(v) | Ex::Mul(v) => {
                for t in v.iter() {
                    constraints(t, out, x)?;
                }
            }
            Ex::Pow(b, p) => {
                constraints(b, out, x)?;
                constraints(p, out, x)?;
                let odd_rational = matches!(&**p, Ex::Num(k) if (k.denom() % BigInt::from(2)) != BigInt::zero());
                if !odd_rational {
                    if has_var(b, x) {
                        push(out, Cond::NonNeg((**b).clone()), b, x)?;
                    } else {
                        check_const(b, &|v| v >= 0.0)?;
                    }
                }
            }
            Ex::Fun(f, a) => {
                // the function's own constraint first (log(asin(3)): sympy raises on the non-real argument)
                match f {
                    Log => if has_var(a, x) { push(out, Cond::Pos((**a).clone()), a, x)? } else { check_const(a, &|v| v > 0.0)? },
                    Asin | Acos => {
                        if has_var(a, x) {
                            push(out, Cond::InUnit((**a).clone()), a, x)?
                        } else {
                            check_const(a, &|v| (-1.0..=1.0).contains(&v))?
                        }
                    }
                    _ => {}
                }
                constraints(a, out, x)?;
            }
            _ => {}
        }
        Ok(())
    }
    // singularities: bases of negative powers after tan/sec/csc/cot are rewritten through cos (1/sec(u) = cos(u))
    fn singular(e: &Ex, out: &mut Vec<Ex>) {
        let neg = |p: &Ex| match p { Ex::Num(k) => k.is_negative(), other => eval_mp(other, &|_| None).map(|v| v < 0.0).unwrap_or(false) };
        match e {
            Ex::Add(v) | Ex::Mul(v) => v.iter().for_each(|t| singular(t, out)),
            Ex::Pow(b, p) if neg(p) => {
                singular(p, out);
                match &**b {
                    Ex::Fun(Sec | Csc, u) => singular(u, out),
                    Ex::Fun(Tan, u) => {
                        singular(u, out);
                        out.push(sym::func(Sin, (**u).clone()));
                    }
                    Ex::Fun(Cot, u) => {
                        singular(u, out);
                        out.push(sym::func(Cos, (**u).clone()));
                    }
                    other => {
                        singular(other, out);
                        out.push(other.clone());
                    }
                }
            }
            Ex::Pow(b, p) => {
                singular(b, out);
                singular(p, out);
            }
            Ex::Fun(f, u) => {
                singular(u, out);
                match f {
                    Tan | Sec => out.push(sym::func(Cos, (**u).clone())),
                    Csc | Cot => out.push(sym::func(Sin, (**u).clone())),
                    Log => out.push((**u).clone()),
                    _ => {}
                }
            }
            _ => {}
        }
    }
    constraints(e, out, x)?;
    let mut bases = vec![];
    singular(e, &mut bases);
    for b in bases {
        if !has_var(&b, x) {
            match eval_mp(&b, &|_| None) {
                Ok(v) if v == 0.0 => return Err(false),
                _ => continue,
            }
        }
        if !solvable(&b, x) {
            return Err(false);                       // sympy: a ConditionSet of singularities (not continuous)
        }
        out.push(Cond::NonZero(b));
    }
    Ok(())
}

/// The exact value of g at a rational point, when the expression folds to a rational there
fn exact_at(g: &Ex, x: &str, a: &Q) -> Option<Q> {
    match sym::subs(g, x, &sym::num(a.clone())) {
        Ex::Num(v) => Some(v),
        _ => None,
    }
}

/// Whether the condition holds on all of [lo, hi] (exact for a rational polynomial, else by dense sampling with
/// a refinement of every local minimum of |g| to find a touching zero)
fn holds(c: &Cond, x: &Rc<str>, lo: &Q, hi: &Q) -> bool {
    let (g, kind) = match c {
        Cond::NonZero(g) => (g, 0),
        Cond::Pos(g) => (g, 1),
        Cond::NonNeg(g) => (g, 2),
        Cond::InUnit(g) => (g, 3),
    };
    if kind == 3 {
        let up = sym::sub(sym::int(1), g.clone());
        let dn = sym::add(sym::int(1), g.clone());
        return holds(&Cond::NonNeg(up), x, lo, hi) && holds(&Cond::NonNeg(dn), x, lo, hi);
    }
    if let Ok(p) = poly_from(g, x) {
        if p.is_zero() {
            return kind == 2;
        }
        let mut roots = real_roots(&p);
        let s = p.sqf_part();
        let inside: Vec<Root> = roots.iter_mut().filter_map(|r| {
            let (a, b) = (r.cmp_q(&s, lo), r.cmp_q(&s, hi));
            (a >= 0 && b <= 0).then(|| r.clone())
        }).collect();
        let pos_at = |t: &Q| sign_q(&p.eval_q(t));
        return match kind {
            0 => inside.is_empty(),
            1 => inside.is_empty() && pos_at(lo) > 0,
            _ => {
                // nonnegative: between consecutive roots (and the ends) the sign is constant; test a point in each piece
                let mut cuts = vec![lo.clone()];
                for r in &inside {
                    if let Root::Exact(q) = r { cuts.push(q.clone()) } else if let Root::Iso(a, b) = r {
                        cuts.push((a + b) / Q::from_integer(BigInt::from(2)));
                    }
                }
                cuts.push(hi.clone());
                let two = Q::from_integer(BigInt::from(2));
                let mut ok = pos_at(lo) >= 0 && pos_at(hi) >= 0;
                for w in cuts.windows(2) {
                    if w[0] < w[1] {
                        let m = (&w[0] + &w[1]) / &two;
                        // a piece with a sign change inside an isolating interval: test its midpoint between roots
                        if pos_at(&m) < 0 {
                            ok = false;
                        }
                    }
                }
                ok
            }
        };
    }
    // sampled: values at the ends exactly when they fold to rationals
    let (l, h) = (lo.to_f64().unwrap_or(f64::NAN), hi.to_f64().unwrap_or(f64::NAN));
    let val = |t: f64| eval_mp(g, &|s: &str| (s == &**x).then_some(t)).ok();
    let end = |q: &Q, t: f64| exact_at(g, x, q).map(|v| v.to_f64().unwrap_or(f64::NAN)).or_else(|| val(t));
    let n = 4000;
    let mut ys = Vec::with_capacity(n + 1);
    for k in 0..=n {
        let t = l + (h - l) * k as f64 / n as f64;
        let y = if k == 0 { end(lo, l) } else if k == n { end(hi, h) } else { val(t) };
        match y {
            Some(y) if y.is_finite() => ys.push((t, y)),
            _ => return false,
        }
    }
    let bad = |y: f64| match kind { 0 => y == 0.0, 1 => y <= 0.0, _ => y < 0.0 };
    if ys.iter().any(|(_, y)| bad(*y)) {
        return false;
    }
    if ys.windows(2).any(|w| (w[0].1 > 0.0 && w[1].1 < 0.0) || (w[0].1 < 0.0 && w[1].1 > 0.0)) {
        return false;                                               // a sign change: a zero in between
    }
    // a touching zero between samples: refine each interior local minimum of |g|
    let scale = ys.iter().map(|(_, y)| y.abs()).fold(0.0, f64::max).max(1e-300);
    for k in 1..n {
        let (a, b, c) = (ys[k - 1].1.abs(), ys[k].1.abs(), ys[k + 1].1.abs());
        if b <= a && b <= c {
            let (mut p, mut q) = (ys[k - 1].0, ys[k + 1].0);
            for _ in 0..80 {
                let m1 = p + (q - p) * 0.381966;
                let m2 = p + (q - p) * 0.618034;
                match (val(m1), val(m2)) {
                    (Some(y1), Some(y2)) => if y1.abs() < y2.abs() { q = m2 } else { p = m1 },
                    _ => return false,
                }
            }
            match val((p + q) / 2.0) {
                Some(y) if (kind < 2 && y.abs() <= 1e-13 * scale) || bad(y) => return false,
                None => return false,
                _ => {}
            }
        }
    }
    true
}

/// Whether a condition holds on all of [lo, infinity), for a condition polynomial in x (exact, by its real roots);
/// None for any other condition (no sampling can cover an infinite interval)
fn holds_from(c: &Cond, x: &Rc<str>, lo: &Q) -> Option<bool> {
    let (g, kind) = match c {
        Cond::NonZero(g) => (g, 0),
        Cond::Pos(g) => (g, 1),
        Cond::NonNeg(g) => (g, 2),
        Cond::InUnit(g) => {
            let up = Cond::NonNeg(sym::sub(sym::int(1), g.clone()));
            let dn = Cond::NonNeg(sym::add(sym::int(1), g.clone()));
            return Some(holds_from(&up, x, lo)? && holds_from(&dn, x, lo)?);
        }
    };
    let p = poly_from(g, x).ok()?;
    if p.is_zero() {
        return Some(kind == 2);
    }
    let s = p.sqf_part();
    let mut inside = vec![];
    for mut r in real_roots(&p) {
        if r.cmp_q(&s, lo) >= 0 {
            inside.push(r);
        }
    }
    let pos_at = |t: &Q| sign_q(&p.eval_q(t));
    let lead = sign_q(&p.lc());
    Some(match kind {
        0 => inside.is_empty(),
        1 => inside.is_empty() && pos_at(lo) > 0,
        _ => {
            let mut cuts = vec![lo.clone()];
            for r in &inside {
                match r {
                    Root::Exact(q) => cuts.push(q.clone()),
                    Root::Iso(a, b) => cuts.push((a + b) / Q::from_integer(BigInt::from(2))),
                }
            }
            let two = Q::from_integer(BigInt::from(2));
            let mut ok = pos_at(lo) >= 0 && (p.degree() < 1 || lead > 0);
            for w in cuts.windows(2) {
                if w[0] < w[1] && pos_at(&((&w[0] + &w[1]) / &two)) < 0 {
                    ok = false;
                }
            }
            ok
        }
    })
}

/// improper_integral(f, a) of the calculus package: continuity on [a, oo) (sympy's continuous_domain), convergence
/// by the limit of x^1.01 f(x) at infinity, then the quadrature on [a, oo)
fn improper_integral(f: &Py, a: f64) -> R {
    let e = ex_of(&parse_expr(f, None)?)?;
    let x: Rc<str> = Rc::from("x");
    let Some(Ex::Num(lo)) = sym::rational_of_repr(a) else { return Err(verr("not a finite number")) };
    let mut cs = vec![];
    let continuous = match conditions(&e, &mut cs, &x) {
        Ok(()) => {
            let mut ok = true;
            for c in &cs {
                match holds_from(c, &x, &lo) {
                    Some(h) => ok &= h,
                    None => return Err(unsup("continuous_domain on [a, oo) of a condition that is not polynomial")),
                }
            }
            ok
        }
        Err(false) => false,
        Err(true) => return Err(unsup("continuous_domain raises here")),
    };
    if !continuous {
        return Err(verr("the integrand is not continuous on the whole interval [a, infinity)"));
    }
    let g = sym::mul(sym::pow(Ex::Sym(x.clone()), sym::num(Q::new(BigInt::from(101), BigInt::from(100)))), e.clone());
    match crate::pylimit::at_infinity(&g, &x, 1) {
        Ok(crate::pylimit::Lim::Value(v)) if sym::is_number(&v) && !sym::has_undefined(&v) => {}
        Ok(_) => return Err(verr("convergence cannot be established (the integrand does not decay faster than 1/x^1.01)")),
        Err(w) => return Err(unsup(format!("limit at infinity: {w:?}"))),
    }
    let fc = compile(&e, &[x], true)?;
    let v = quad(&fc, a, f64::INFINITY, &Py::None)?;
    Ok(Py::dict(vec![(Py::str("integral"), v)]))
}

/// _verify_ratio(i, o) of the calculus package: L = |a_(N+1)/a_N| at N = 10^12 (mpmath at 40 digits there; here in
/// logarithms, crate::pympf::ratio_at, since 4^(10^12) is beyond a fixed-exponent number), then its residual
/// worst(rel_err(ratio_limit, L, 1e-10), _same(verdict, ...)); a term that cannot be evaluated is a ValueError, which
/// the package's _safe_verify turns into a rejection
fn verify_ratio(i: &Py, o: &Py) -> R {
    let text = py::getitem(i, &Py::str("a"))?;
    let e = ex_of(&parse_expr(&text, Some(&Py::tuple(vec![Py::str("n")])))?)?;
    let l = crate::pympf::ratio_at(&e, "n", 1_000_000_000_000)
        .map_err(|s| verr(format!("the expression is not defined there ({})", s.name())))?;
    let lim = py::getitem(o, &Py::str("ratio_limit"))?;
    let rel = match lim.f() {
        Ok(g) => {
            let d = (g - l).abs() / l.abs().max(1.0);
            if d.is_nan() { 1.0 } else { (d - 1e-10).max(0.0) }
        }
        Err(_) => 1.0,
    };
    let want = if py::lt(&lim, &Py::Int(1))? { "converges" } else { "diverges" };
    let verdict = py::getitem(o, &Py::str("verdict"))?;
    let same = if matches!(&verdict, Py::Str(s) if &**s == want) { 0.0 } else { 1.0 };
    Ok(Py::Float(rel.max(same)))
}

/// continuous_on(e, var, a, b): ValueError unless e is real and continuous on all of [a, b] (sympy's
/// continuous_domain on the interval of the decimals a and b; where sympy cannot decide, a 401-point grid)
pub fn continuous_on(e: &Ex, var: &Rc<str>, a: f64, b: f64) -> Result<(), PyErr> {
    let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
    let q = |v: f64| match sym::rational_of_repr(v) { Some(Ex::Num(q)) => Ok(q), _ => Err(verr("not a finite number")) };
    let (ql, qh) = (q(lo)?, q(hi)?);
    let mut cs = vec![];
    match conditions(e, &mut cs, var) {
        Ok(()) => {
            if cs.iter().all(|c| holds(c, var, &ql, &qh)) {
                return Ok(());
            }
        }
        Err(false) => {}
        Err(true) => {
            // sympy cannot decide: the package's grid of 401 float values (an error there is the refusal)
            let g = compile(e, std::slice::from_ref(var), false)?;
            for k in 0..401 {
                py::call(&g, &[Py::Float(lo + (hi - lo) * k as f64 / 400.0)])?;
            }
            return Ok(());
        }
    }
    Err(verr(format!("the function is not continuous (or not real) on the whole interval [{}, {}]",
                     crate::pyfmt::fmt(lo, "g"), crate::pyfmt::fmt(hi, "g"))))
}

// ------------------------------------------------------------------------------------------- Taylor-mode derivative
/// Taylor coefficients in t up to order n of e with each variable v = value + direction * t (seed(v) gives the
/// pair): automatic differentiation; Err where a function is not defined
fn taylor(e: &Ex, seed: &dyn Fn(&str) -> Option<(f64, f64)>, n: usize) -> Result<Vec<f64>, Stop> {
    use Fun::*;
    let cst = |v: f64| { let mut c = vec![0.0; n + 1]; c[0] = v; c };
    let mul = |u: &[f64], v: &[f64]| (0..=n).map(|k| (0..=k).map(|j| u[j] * v[k - j]).sum::<f64>()).collect::<Vec<f64>>();
    let recip = |u: &[f64]| -> Result<Vec<f64>, Stop> {
        if u[0] == 0.0 {
            return Err(Stop::ZeroDivision);
        }
        let mut w = vec![0.0; n + 1];
        w[0] = 1.0 / u[0];
        for k in 1..=n {
            w[k] = -(1..=k).map(|j| u[j] * w[k - j]).sum::<f64>() / u[0];
        }
        Ok(w)
    };
    // w = integral of u' v (w0 given): w_k = (1/k) sum j u_j v_{k-j}
    let integ = |u: &[f64], v: &[f64], w0: f64| {
        let mut w = vec![0.0; n + 1];
        w[0] = w0;
        for k in 1..=n {
            w[k] = (1..=k).map(|j| j as f64 * u[j] * v[k - j]).sum::<f64>() / k as f64;
        }
        w
    };
    let powr = |u: &[f64], p: f64| -> Result<Vec<f64>, Stop> {
        if u[0] == 0.0 {
            return Err(Stop::ZeroDivision);
        }
        let mut w = vec![0.0; n + 1];
        w[0] = u[0].powf_lm(p);
        for k in 1..=n {
            w[k] = (1..=k).map(|j| (p * j as f64 - (k - j) as f64) * u[j] * w[k - j]).sum::<f64>() / (k as f64 * u[0]);
        }
        Ok(w)
    };
    let sincos = |u: &[f64], hyper: bool| {
        let (mut s, mut c) = (vec![0.0; n + 1], vec![0.0; n + 1]);
        if hyper { s[0] = u[0].sinh_lm(); c[0] = u[0].cosh_lm(); } else { s[0] = u[0].sin_lm(); c[0] = u[0].cos_lm(); }
        for k in 1..=n {
            let ds: f64 = (1..=k).map(|j| j as f64 * u[j] * c[k - j]).sum::<f64>() / k as f64;
            let dc: f64 = (1..=k).map(|j| j as f64 * u[j] * s[k - j]).sum::<f64>() / k as f64;
            s[k] = ds;
            c[k] = if hyper { dc } else { -dc };
        }
        (s, c)
    };
    Ok(match e {
        Ex::Num(v) => cst(v.to_f64().unwrap_or(f64::NAN)),
        Ex::Pi => cst(std::f64::consts::PI),
        Ex::E => cst(std::f64::consts::E),
        Ex::Sym(s) => {
            let (v, d) = seed(s).ok_or(Stop::Value)?;
            let mut c = cst(v);
            if n >= 1 {
                c[1] = d;
            }
            c
        }
        Ex::Zoo | Ex::Nan => return Err(Stop::Value),
        Ex::Add(ts) => {
            let mut s = vec![0.0; n + 1];
            for t in ts.iter() {
                let c = taylor(t, seed, n)?;
                for k in 0..=n { s[k] += c[k]; }
            }
            s
        }
        Ex::Mul(fs) => {
            let mut p = cst(1.0);
            for f in fs.iter() {
                p = mul(&p, &taylor(f, seed, n)?);
            }
            p
        }
        Ex::Pow(b, p) => {
            let u = taylor(b, seed, n)?;
            match &**p {
                Ex::Num(k) if k.is_integer() && !k.is_negative() => {
                    let m = k.to_integer().to_u32().ok_or(Stop::Overflow)?;
                    let mut w = cst(1.0);
                    for _ in 0..m { w = mul(&w, &u); }
                    w
                }
                Ex::Num(k) if k.is_integer() => {
                    let m = (-k).to_integer().to_u32().ok_or(Stop::Overflow)?;
                    let mut w = cst(1.0);
                    for _ in 0..m { w = mul(&w, &u); }
                    recip(&w)?
                }
                Ex::Num(k) => {
                    if u[0] < 0.0 {
                        return Err(Stop::NotReal);
                    }
                    powr(&u, k.to_f64().unwrap_or(f64::NAN))?
                }
                _ => {
                    // b^p = exp(p log b)
                    let lg = taylor(&sym::func(Log, (**b).clone()), seed, n)?;
                    let pe = taylor(p, seed, n)?;
                    let z = mul(&pe, &lg);
                    integ(&z, &cst(0.0), 0.0); // (shape only)
                    let mut w = vec![0.0; n + 1];
                    w[0] = z[0].exp_lm();
                    for k in 1..=n {
                        w[k] = (1..=k).map(|j| j as f64 * z[j] * w[k - j]).sum::<f64>() / k as f64;
                    }
                    w
                }
            }
        }
        Ex::Fun(f, arg) => {
            let u = taylor(arg, seed, n)?;
            match f {
                Sin => sincos(&u, false).0,
                Cos => sincos(&u, false).1,
                Sinh => sincos(&u, true).0,
                Cosh => sincos(&u, true).1,
                Tan => { let (s, c) = sincos(&u, false); mul(&s, &recip(&c)?) }
                Sec => recip(&sincos(&u, false).1)?,
                Csc => recip(&sincos(&u, false).0)?,
                Cot => { let (s, c) = sincos(&u, false); mul(&c, &recip(&s)?) }
                Tanh => { let (s, c) = sincos(&u, true); mul(&s, &recip(&c)?) }
                Exp => {
                    let mut w = vec![0.0; n + 1];
                    w[0] = u[0].exp_lm();
                    for k in 1..=n {
                        w[k] = (1..=k).map(|j| j as f64 * u[j] * w[k - j]).sum::<f64>() / k as f64;
                    }
                    w
                }
                Log => {
                    if u[0] <= 0.0 {
                        return Err(if u[0] == 0.0 { Stop::ZeroDivision } else { Stop::NotReal });
                    }
                    integ(&u, &recip(&u)?, u[0].ln_lm())
                }
                Asin | Acos => {
                    if u[0].abs() >= 1.0 {
                        return Err(Stop::NotReal);
                    }
                    let one_m = { let sq = mul(&u, &u); let mut o = cst(1.0); for k in 0..=n { o[k] -= sq[k]; } o };
                    let v = powr(&one_m, -0.5)?;
                    let w = integ(&u, &v, u[0].asin_lm());
                    if *f == Asin { w } else {
                        let mut w = w.iter().map(|c| -c).collect::<Vec<f64>>();
                        w[0] = u[0].acos_lm();
                        w
                    }
                }
                Atan => {
                    let one_p = { let sq = mul(&u, &u); let mut o = cst(1.0); for k in 0..=n { o[k] += sq[k]; } o };
                    integ(&u, &recip(&one_p)?, u[0].atan_lm())
                }
                Abs => {
                    if u[0] == 0.0 {
                        return Err(Stop::Value);
                    }
                    let s = u[0].signum();
                    u.iter().map(|c| s * c).collect()
                }
            }
        }
    })
}

/// The n-th derivative at a point by Taylor-mode automatic differentiation
pub fn derivative_ad(e: &Ex, x: &str, a: f64, n: usize) -> Result<f64, Stop> {
    let c = taylor(e, &|s: &str| (s == x).then_some((a, 1.0)), n)?;
    let mut f = 1.0;
    for k in 2..=n {
        f *= k as f64;
    }
    Ok(c[n] * f)
}

// ------------------------------------------------------------------------------------------- Gauss-Legendre
/// The nodes and weights of the m-point Gauss-Legendre rule on [-1, 1]
pub fn gauss_legendre(m: usize) -> Vec<(f64, f64)> {
    let mut out = Vec::with_capacity(m);
    for i in 0..m {
        let mut z = (std::f64::consts::PI * (i as f64 + 0.75) / (m as f64 + 0.5)).cos();
        let mut dp = 0.0;
        for _ in 0..100 {
            let (mut p0, mut p1) = (1.0, z);
            for k in 2..=m {
                let p2 = ((2 * k - 1) as f64 * z * p1 - (k - 1) as f64 * p0) / k as f64;
                p0 = p1;
                p1 = p2;
            }
            dp = m as f64 * (z * p1 - p0) / (z * z - 1.0);
            let dz = p1 / dp;
            z -= dz;
            if dz.abs() < 1e-16 {
                break;
            }
        }
        out.push((z, 2.0 / ((1.0 - z * z) * dp * dp)));
    }
    out
}

/// Gauss-Legendre quadrature of a function on equal panels: 32 points per panel, panels doubled until two
/// successive sums agree (the package's verifiers' rule, mpmath gauss-legendre on knots)
pub fn gl_panels(f: &Py, a: f64, b: f64, panels: usize) -> Result<f64, PyErr> {
    let rule = gauss_legendre(32);
    // The terms are summed with Neumaier's compensation: a plain running sum of thousands of terms carries a rounding
    // noise above the 1e-15 settling test, so a converged rule kept doubling to 64 times the panels (0.7 s for one
    // Fourier coefficient's check, 2026-10-31); compensated, the noise is a few units of the last place.
    // Each sum also returns the sum of the terms' sizes: two sums that differ by no more than the rounding of their
    // terms (8 units of the last place of that size) have settled too, as no further doubling can show a difference
    // (an integral that cancels to nothing out of large terms, Fourier coefficients on a long interval).
    let sum = |np: usize| -> Result<(f64, f64), PyErr> {
        let h = (b - a) / np as f64;
        let (mut s, mut comp, mut size) = (0.0f64, 0.0f64, 0.0f64);
        for p in 0..np {
            let (lo, hi) = (a + h * p as f64, a + h * (p + 1) as f64);
            let (c, r) = ((lo + hi) / 2.0, (hi - lo) / 2.0);
            for (z, w) in &rule {
                let t = w * r * py::call(f, &[Py::Float(c + r * z)])?.f()?;
                let u = s + t;
                if u.is_finite() {
                    comp += if s.abs() >= t.abs() { (s - u) + t } else { (t - u) + s };
                }
                s = u;
                size += t.abs();
            }
        }
        Ok((if s.is_finite() { s + comp } else { s }, size))
    };
    let mut np = panels.max(1);
    let (mut prev, mut prev_size) = sum(np)?;
    for _ in 0..6 {
        np *= 2;
        let (cur, size) = sum(np)?;
        let d = (cur - prev).abs();
        if d <= 1e-15 * cur.abs().max(1.0) || d <= 8.0 * f64::EPSILON * size.max(prev_size) {
            return Ok(cur);
        }
        (prev, prev_size) = (cur, size);
    }
    Ok(prev)
}

// ================================================================================================= printing
/// show(e): sympy's text of an expression with ^ for powers
pub fn show(e: &Ex) -> String {
    sym::sstr(e).replace("**", "^")
}

// ================================================================================================= the helpers
fn arg(a: &[Py], kws: &[(String, Py)], i: usize, name: &str) -> Option<Py> {
    a.get(i).cloned().or_else(|| kws.iter().find(|(k, _)| k == name).map(|(_, v)| v.clone()))
}

fn need(a: &[Py], kws: &[(String, Py)], i: usize, name: &str, f: &str) -> R {
    arg(a, kws, i, name).ok_or_else(|| terr(format!("{f}() missing argument {name}")))
}

/// parse_expr(text, names=("x",)): the allowlisted reader (crate::symparse)
pub fn parse_expr(text: &Py, names: Option<&Py>) -> R {
    let Py::Str(t) = text else { return Err(verr("the expression must be text")) };
    let names: Vec<Rc<str>> = match names {
        Some(n) => names_of(n)?,
        None => vec![Rc::from("x")],
    };
    let refs: Vec<&str> = names.iter().map(|s| &**s).collect();
    crate::symparse::parse(t, &refs).map(ex).map_err(verr)
}

/// A package helper replaced by its Rust version (tools/py2rust.py REPLACE): "model:name", positional and keyword
/// arguments
pub fn helper(name: &str, a: &[Py], kws: &[(String, Py)]) -> R {
    let (model, name) = name.split_once(':').unwrap_or(("calculus", name));
    if model == "differential_equations" {
        return helper_de(name, a, kws);
    }
    if model != "calculus" {
        return crate::handwritten::helper(model, name, a, kws);     // written in Rust by hand (tools/py2rust.py)
    }
    match name {
        "parse_expr" | "_parse_expr" | "_parse_cached" => {
            parse_expr(&need(a, kws, 0, "text", name)?, arg(a, kws, 1, "names").as_ref())
        }
        "number" => {
            let t = need(a, kws, 0, "text", name)?;
            if let Py::Int(_) | Py::Float(_) | Py::Big(_) = t {
                return Ok(Py::Float(t.f()?));
            }
            let s = py::to_str(&t);
            let e = crate::symparse::parse(&s, &[]).map_err(verr)?;
            match eval_mp(&e, &|_| None) {
                Ok(v) if v.is_finite() => Ok(Py::Float(v)),
                _ => Err(verr(format!("{} is not a finite real number", py::repr(&Py::str(&s))))),
            }
        }
        "_compile" => {
            let e = ex_of(&need(a, kws, 0, "e", name)?)?;
            let names = names_of(&need(a, kws, 1, "names", name)?)?;
            let mp = arg(a, kws, 2, "use_mp").map(|v| v.truthy()).unwrap_or(true);
            compile(&e, &names, mp)
        }
        "at" => {
            // at(e, names, *vals, dps=30)
            let e = ex_of(a.first().ok_or_else(|| terr("at() missing e"))?)?;
            let names = names_of(a.get(1).ok_or_else(|| terr("at() missing names"))?)?;
            at(&e, &names, &a[2..])
        }
        "fx" => {
            let names = arg(a, kws, 1, "names");
            let e = parse_expr(&need(a, kws, 0, "text", name)?, names.as_ref())?;
            let nm = match &names { Some(n) => names_of(n)?, None => vec![Rc::from("x")] };
            let f = compile(&ex_of(&e)?, &nm, true)?;
            Ok(Py::tuple(vec![e, f]))
        }
        "quad" => {
            let f = need(a, kws, 0, "f", name)?;
            let lo = need(a, kws, 1, "a", name)?.f()?;
            let hi = need(a, kws, 2, "b", name)?.f()?;
            let pts = arg(a, kws, 4, "points").unwrap_or(Py::None);
            quad(&f, lo, hi, &pts)
        }
        "continuous_on" => {
            let e = ex_of(&need(a, kws, 0, "e", name)?)?;
            let v = var_name(&need(a, kws, 1, "var", name)?)?;
            continuous_on(&e, &v, need(a, kws, 2, "a", name)?.f()?, need(a, kws, 3, "b", name)?.f()?)?;
            Ok(Py::None)
        }
        "show" => Ok(Py::str(&show(&ex_of(&need(a, kws, 0, "e", name)?)?))),
        "poly_of" => {
            let v: Rc<str> = match arg(a, kws, 1, "var") { Some(v) => var_name(&v)?, None => Rc::from("x") };
            let e = ex_of(&parse_expr(&need(a, kws, 0, "text", name)?, Some(&Py::tuple(vec![Py::Str(v.clone())])))?)?;
            match poly_from(&e, &v) {
                Ok(p) => Ok(Py::Poly(Rc::new(p))),
                Err(true) => Err(verr("this capability needs a polynomial")),
                Err(false) => Err(verr("the polynomial needs rational coefficients")),
            }
        }
        "real_roots_in" => {
            let p = poly_arg(&need(a, kws, 0, "p", name)?)?;
            let open = arg(a, kws, 3, "open_").map(|v| v.truthy()).unwrap_or(true);
            real_roots_in(&p, need(a, kws, 1, "a", name)?.f()?, need(a, kws, 2, "b", name)?.f()?, open)
        }
        "_mp_diff" => {
            // _mp_diff(text, a, n=1, names=("x",)): the n-th derivative by automatic differentiation
            let names = arg(a, kws, 3, "names");
            let e = ex_of(&parse_expr(&need(a, kws, 0, "text", name)?, names.as_ref())?)?;
            let nm = match &names { Some(n) => names_of(n)?, None => vec![Rc::from("x")] };
            let n = arg(a, kws, 2, "n").map(|v| v.f()).transpose()?.unwrap_or(1.0) as usize;
            let x0 = need(a, kws, 1, "a", name)?.f()?;
            derivative_ad(&e, &nm[0], x0, n).map(Py::Float)
                .map_err(|s| verr(format!("the expression is not defined there ({})", s.name())))
        }
        "improper_integral" => improper_integral(&need(a, kws, 0, "f", name)?, need(a, kws, 1, "a", name)?.f()?),
        // _nsum(a, k, lam=False): series_sum's numerical check (crate::pysum::nsum)
        "_nsum" => crate::pysum::nsum(&need(a, kws, 0, "a", name)?, &need(a, kws, 1, "k", name)?),
        "_verify_ratio" => verify_ratio(&need(a, kws, 0, "i", name)?, &need(a, kws, 1, "o", name)?),
        "_gl" => {
            // _gl(F, a, b, panels=8, dps=20)
            let f = need(a, kws, 0, "F", name)?;
            let panels = arg(a, kws, 3, "panels").map(|v| v.f()).transpose()?.unwrap_or(8.0) as usize;
            gl_panels(&f, need(a, kws, 1, "a", name)?.f()?, need(a, kws, 2, "b", name)?.f()?, panels).map(Py::Float)
        }
        _ => Err(unsup(format!("helper {name}"))),
    }
}

/// The differential-equations package's helpers: its reader (parse_text, through the engine's parser) and its float
/// evaluator (compile_float: an exactly rounded sum, Python's float and math errors, a non-finite value refused)
fn helper_de(name: &str, a: &[Py], kws: &[(String, Py)]) -> R {
    let names_or = |p: Option<Py>| -> Result<Vec<Rc<str>>, PyErr> {
        match p { Some(n) => names_of(&n), None => Ok(vec![Rc::from("t"), Rc::from("y")]) }
    };
    let parse = |text: &Py, names: &[Rc<str>]| -> Result<Ex, PyErr> {
        let Py::Str(t) = text else { return Err(verr("the expression is missing")) };
        let refs: Vec<&str> = names.iter().map(|s| &**s).collect();
        crate::symparse::parse_text_de(t, &refs).map_err(verr)
    };
    match name {
        "parse_text" => {
            let names = names_or(arg(a, kws, 1, "names"))?;
            parse(&need(a, kws, 0, "text", name)?, &names).map(ex)
        }
        "compile_float" => {
            let e = ex_of(&need(a, kws, 0, "e", name)?)?;
            compile_de(&e, &names_or(arg(a, kws, 1, "names"))?)
        }
        "_fn" => {
            let names = names_or(arg(a, kws, 1, "names"))?;
            let e = parse(&need(a, kws, 0, "text", name)?, &names)?;
            compile_de(&e, &names)
        }
        _ => crate::handwritten::helper("differential_equations", name, a, kws),
    }
}

/// compile_float of the differential-equations package
fn compile_de(e: &Ex, names: &[Rc<str>]) -> R {
    let mut free = vec![];
    sym::free_symbols(e, &mut free);
    if let Some(s) = free.iter().find(|s| !names.contains(s)) {
        return Err(verr(format!("unexpected variable {s}")));
    }
    if let Some(n) = undefined_name(e) {
        return Err(verr(format!("cannot evaluate {n}")));
    }
    let (e, names) = (e.clone(), names.to_vec());
    Ok(py::func(move |a: &[Py]| -> R {
        let vals: Vec<f64> = a.iter().map(|v| v.f()).collect::<Result<_, _>>()?;
        let var = |s: &str| names.iter().position(|n| &**n == s).and_then(|i| vals.get(i).copied());
        match eval_float_g(&e, &var, true) {
            Ok(v) if v.is_finite() => Ok(Py::Float(v)),
            Ok(_) => Err(verr("the expression is not finite there")),
            Err(s) => Err(verr(format!("the expression is not defined there ({})", s.name()))),
        }
    }))
}

fn poly_arg(p: &Py) -> Result<Poly, PyErr> {
    match p {
        Py::Poly(q) => Ok((**q).clone()),
        _ => Err(terr("a polynomial is expected")),
    }
}

/// real_roots_in(p, a, b, open_=True): the distinct real roots in (a, b) (or [a, b]), ascending
pub fn real_roots_in(p: &Poly, a: f64, b: f64, open: bool) -> R {
    if p.is_zero() {
        return Err(verr("the polynomial is identically zero"));
    }
    if p.degree() < 1 {
        return Ok(Py::list(vec![]));
    }
    let q = |v: f64| match sym::rational_of_repr(v) { Some(Ex::Num(q)) => Ok(q), _ => Err(verr("not a finite number")) };
    let (qa, qb) = (q(a)?, q(b)?);
    let s = p.sqf_part();
    let mut out = vec![];
    for mut r in real_roots(p) {
        let (ca, cb) = (r.cmp_q(&s, &qa), r.cmp_q(&s, &qb));
        let inside = if open { ca > 0 && cb < 0 } else { ca >= 0 && cb <= 0 };
        if inside {
            out.push(r.as_py(&s));
        }
    }
    Ok(Py::list(out))
}

// ================================================================================================= sympy calls
/// sp.<name>(*args, **kws) as the procedures call it
pub fn call(name: &str, a: &[Py], kws: &[(String, Py)]) -> R {
    let a0 = || a.first().cloned().ok_or_else(|| terr(format!("sympy.{name}() missing an argument")));
    match name {
        "Rational" => {
            let x = a0()?;
            let r = match (&x, a.get(1)) {
                (Py::Str(s), None) => crate::pyfrac::from_str(s.trim())?,
                (_, None) => match py::to_ex(&x) {
                    Some(Ex::Num(q)) => q,
                    _ => return Err(terr("Rational() of this value")),
                },
                (p, Some(q)) => {
                    let (Some(Ex::Num(p)), Some(Ex::Num(q))) = (py::to_ex(p), py::to_ex(q)) else {
                        return Err(terr("Rational() of these values"));
                    };
                    if q.is_zero() {
                        return Ok(ex(Ex::Zoo));
                    }
                    p / q
                }
            };
            Ok(ex(sym::num(r)))
        }
        "summation" => {
            // summation(e, (k, start, oo)): the infinite series of crate::pysum
            let e = ex_of(&a0()?)?;
            let spec = a.get(1).ok_or_else(|| terr("summation() missing its limits"))?.items()?;
            if spec.len() != 3 || !matches!(&spec[2], Py::Float(v) if *v == f64::INFINITY) {
                return Err(unsup("summation other than up to infinity"));
            }
            let v = var_name(&spec[0])?;
            let Py::Int(k) = &spec[1] else { return Err(unsup("summation from a start that is not an int")) };
            Ok(ex(crate::pysum::summation(&e, &v, *k)?))
        }
        "Integer" => {
            let x = a0()?;
            match &x {
                Py::Float(v) => Ok(ex(sym::int_big(num_bigint::BigInt::from(v.trunc() as i64)))),
                _ => Ok(ex(ex_of(&x)?)),
            }
        }
        "sympify" | "S" => {
            let x = a0()?;
            match &x {
                Py::Str(s) => crate::symparse::parse(s, &["x", "y", "z", "t"]).map(ex).map_err(verr),
                Py::Poly(_) => Ok(x),
                _ => Ok(ex(ex_of(&x)?)),
            }
        }
        "nsimplify" => {
            let x = a0()?;
            match py::to_ex(&x) {
                Some(Ex::Num(q)) => Ok(ex(sym::num(q))),
                Some(e) if !matches!(x, Py::Float(_)) => Ok(ex(e)),
                _ => match &x {
                    // a float: the decimal it prints, as nsimplify finds for the procedures' inputs
                    Py::Float(v) => Ok(ex(sym::rational_of_repr(*v).ok_or_else(|| verr("not a finite number"))?)),
                    _ => Err(unsup("nsimplify")),
                },
            }
        }
        "sqrt" => Ok(ex(sym::sqrt(ex_of(&a0()?)?))),
        "Abs" => Ok(ex(sym::func(Fun::Abs, ex_of(&a0()?)?))),
        "sin" | "cos" | "tan" | "sec" | "csc" | "cot" | "asin" | "acos" | "atan" | "sinh" | "cosh" | "tanh" | "exp" | "log" => {
            let f = match name {
                "sin" => Fun::Sin, "cos" => Fun::Cos, "tan" => Fun::Tan, "sec" => Fun::Sec, "csc" => Fun::Csc,
                "cot" => Fun::Cot, "asin" => Fun::Asin, "acos" => Fun::Acos, "atan" => Fun::Atan, "sinh" => Fun::Sinh,
                "cosh" => Fun::Cosh, "tanh" => Fun::Tanh, "exp" => Fun::Exp, _ => Fun::Log,
            };
            Ok(ex(sym::func(f, ex_of(&a0()?)?)))
        }
        "diff" => {
            // diff(e, x[, n][, y[, m]] ...): each variable n times (once if no count follows), in order
            let mut e = ex_of(&a0()?)?;
            if a.len() < 2 {
                return Err(terr("diff() needs a variable"));
            }
            let mut i = 1;
            while i < a.len() {
                let x = var_name(&a[i])?;
                let n = match a.get(i + 1) {
                    Some(k @ (Py::Int(_) | Py::Float(_))) => { i += 1; k.f()? as usize }
                    _ => 1,
                };
                for _ in 0..n {
                    e = sym::diff(&e, &x);
                }
                i += 1;
            }
            Ok(ex(e))
        }
        "Add.make_args" | "Mul.make_args" => {
            let e = ex_of(&a0()?)?;
            let parts = match (&e, name) {
                (Ex::Add(v), "Add.make_args") | (Ex::Mul(v), "Mul.make_args") => v.iter().map(|t| ex(t.clone())).collect(),
                _ => vec![ex(e)],
            };
            Ok(Py::tuple(parts))
        }
        "N" => {
            let x = a0()?;
            if let Py::Float(_) | Py::Int(_) = x {
                return Ok(Py::Float(x.f()?));
            }
            let e = ex_of(&x)?;
            match eval_mp(&e, &|_| None) {
                Ok(v) => Ok(Py::Float(v)),
                Err(Stop::NotReal) => Ok(Py::Complex(f64::NAN, f64::NAN)),
                Err(_) => Ok(Py::Float(f64::NAN)),
            }
        }
        "expand" => {
            let x = a0()?;
            if let Py::Poly(_) = x {
                return Ok(x);
            }
            Ok(ex(expand(&ex_of(&x)?)))
        }
        "Poly" => {
            let x = a0()?;
            let g = var_name(a.get(1).ok_or_else(|| unsup("Poly without a generator"))?)?;
            if let Py::Poly(p) = &x {
                return Ok(Py::Poly(Rc::new(Poly::new(g, p.c.clone()))));
            }
            let e = ex_of(&x)?;
            match poly_from(&e, &g) {
                Ok(p) => Ok(Py::Poly(Rc::new(p))),
                Err(true) => Err(PyErr::Other("PolynomialError".into())),
                Err(false) => {
                    let mut cs = ex_poly(&e, &g).unwrap_or_default();
                    while cs.last().is_some_and(sym::is_zero) {
                        cs.pop();
                    }
                    let mut p = Poly::new(g, vec![]);
                    p.irr = Some(Rc::new(cs));
                    Ok(Py::Poly(Rc::new(p)))
                }
            }
        }
        "together" | "cancel" => Ok(ex(ex_of(&a0()?)?)),
        "fraction" => {
            // fraction(cancel(together(e))): numerator and denominator in lowest terms with integer coefficients and
            // a positive leading coefficient below (sympy's cancel); an expression that is not a rational function of
            // one variable is returned over 1
            let e = ex_of(&a0()?)?;
            let g = single_gen(&e);
            let mut free = vec![];
            sym::free_symbols(&e, &mut free);
            match (free.len() <= 1).then(|| ratfun(&e, &g)).flatten() {
                Some((n, d)) => Ok(Py::tuple(vec![ex(n.as_ex()), ex(d.as_ex())])),
                None => Ok(Py::tuple(vec![ex(e), ex(sym::int(1))])),
            }
        }
        "real_roots" => {
            // every real root as often as its multiplicity, ascending (sympy's real_roots)
            let p = poly_arg(&a0()?)?;
            let _ = kws;
            let mut all: Vec<(f64, Py)> = vec![];
            for (f, m) in p.sqf_list().1 {
                let s = f.sqf_part();
                for r in real_roots(&f) {
                    for _ in 0..m {
                        all.push((r.approx(&s), r.as_py(&s)));
                    }
                }
            }
            all.sort_by(|a, b| a.0.total_cmp(&b.0));
            Ok(Py::list(all.into_iter().map(|(_, v)| v).collect()))
        }
        "limit" => {
            let e = ex_of(&a0()?)?;
            let x = var_name(a.get(1).ok_or_else(|| terr("limit() needs a variable"))?)?;
            let pt = a.get(2).ok_or_else(|| terr("limit() needs a point"))?;
            let dir = match a.get(3) { Some(Py::Str(d)) => d.to_string(), Some(_) => return Err(terr("limit() direction")), None => "+".into() };
            limit(&e, &x, pt, &dir)
        }
        // sympy's simplify: the procedures only take limits or values of the result, which the expansion decides on
        // the unsimplified expression
        "simplify" => Ok(ex(ex_of(&a0()?)?)),
        "integrate" => integrate(&ex_of(&a0()?)?, &a[1..]),
        "sign" => {
            let v = a0()?;
            let s = match py::to_ex(&v) {
                Some(Ex::Num(q)) => sign_q(&q) as i64,
                Some(e) => match eval_mp(&e, &|_| None) { Ok(f) if f > 0.0 => 1, Ok(f) if f < 0.0 => -1, Ok(_) => 0, Err(_) => return Err(unsup("sign")) },
                None => return Err(terr("sign() of this value")),
            };
            Ok(ex(sym::int(s)))
        }
        "prod" => {
            let mut acc = ex(sym::int(1));
            for f in a0()?.items()? {
                acc = py::mul(&acc, &f)?;
            }
            Ok(acc)
        }
        "sqf_part" => {
            let x = a0()?;
            let e = ex_of(&x)?;
            let g = single_gen(&e);
            let p = poly_from(&e, &g).map_err(|_| unsup("sqf_part of a non-polynomial"))?;
            Ok(ex(p.sqf_part().as_ex()))
        }
        "sqf_list" => {
            let e = ex_of(&a0()?)?;
            let g = single_gen(&e);
            let p = poly_from(&e, &g).map_err(|_| unsup("sqf_list of a non-polynomial"))?;
            let (c, fs) = p.sqf_list();
            Ok(Py::tuple(vec![ex(sym::num(c)), Py::list(fs.into_iter()
                .map(|(f, m)| Py::tuple(vec![ex(f.as_ex()), Py::Int(m as i64)])).collect())]))
        }
        _ => Err(unsup(format!("sympy.{name}"))),
    }
}

/// An expression as a rational function p/q of x with rational coefficients, in lowest terms: integer coefficients
/// with no common factor, the leading coefficient of q positive (sympy's cancel); None if it is not one
pub fn ratfun(e: &Ex, x: &Rc<str>) -> Option<(Poly, Poly)> {
    fn go(e: &Ex, x: &Rc<str>) -> Option<(Poly, Poly)> {
        let one = || Poly::constant(x.clone(), Q::one());
        Some(match e {
            Ex::Num(v) => (Poly::constant(x.clone(), v.clone()), one()),
            Ex::Sym(s) if s == x => (Poly::new(x.clone(), vec![Q::zero(), Q::one()]), one()),
            Ex::Add(ts) => {
                let (mut n, mut d) = (Poly::new(x.clone(), vec![]), one());
                for t in ts.iter() {
                    let (a, b) = go(t, x)?;
                    n = n.mul(&b).add(&a.mul(&d));
                    d = d.mul(&b);
                }
                (n, d)
            }
            Ex::Mul(fs) => {
                let (mut n, mut d) = (one(), one());
                for f in fs.iter() {
                    let (a, b) = go(f, x)?;
                    n = n.mul(&a);
                    d = d.mul(&b);
                }
                (n, d)
            }
            Ex::Pow(b, p) => {
                let Ex::Num(k) = &**p else { return None };
                if !k.is_integer() || k.abs() > Q::from_integer(BigInt::from(64)) {
                    return None;
                }
                let m = k.to_integer().abs().to_u32()?;
                let (a, c) = go(b, x)?;
                if k.is_negative() {
                    if a.is_zero() {
                        return None;
                    }
                    (c.pow(m), a.pow(m))
                } else {
                    (a.pow(m), c.pow(m))
                }
            }
            _ => return None,
        })
    }
    let (n, d) = go(e, x)?;
    if d.is_zero() {
        return None;
    }
    let g = n.gcd(&d);
    let (mut n, mut d) = if g.degree() >= 1 { (n.divmod(&g).0, d.divmod(&g).0) } else { (n, d) };
    // integer coefficients without a common factor; the denominator's leading coefficient positive
    let mut den = BigInt::one();
    for k in n.c.iter().chain(d.c.iter()) {
        den = num_integer::Integer::lcm(&den, k.denom());
    }
    let scale = Q::from_integer(den);
    n = n.scale(&scale);
    d = d.scale(&scale);
    let mut g = BigInt::zero();
    for k in n.c.iter().chain(d.c.iter()) {
        g = num_integer::Integer::gcd(&g, &k.to_integer());
    }
    if !g.is_zero() {
        let inv = Q::new(BigInt::one(), g);
        n = n.scale(&inv);
        d = d.scale(&inv);
    }
    if d.lc().is_negative() {
        n = n.neg();
        d = d.neg();
    }
    Some((n, d))
}

/// e as numerator and denominator: the factors of a product with a negative integer power go below
fn numer_denom(e: &Ex) -> (Ex, Ex) {
    let fs: Vec<Ex> = match e { Ex::Mul(v) => (**v).clone(), other => vec![other.clone()] };
    let (mut n, mut d) = (vec![], vec![]);
    for f in fs {
        match &f {
            Ex::Pow(b, p) if matches!(&**p, Ex::Num(k) if k.is_negative()) => {
                d.push(sym::pow((**b).clone(), sym::neg((**p).clone())));
            }
            _ => n.push(f),
        }
    }
    (sym::mul_all(n), sym::mul_all(d))
}

/// The order of vanishing at x = a and the leading Taylor coefficient of an expression analytic there, exactly
/// (repeated differentiation, values substituted exactly); None where it is not analytic at a, every coefficient
/// up to order 12 vanishes, or a coefficient is not decidably zero or nonzero
fn order_at(e: &Ex, x: &str, a: &Ex) -> Option<(usize, Ex)> {
    let mut d = e.clone();
    let mut fact = sym::int(1);
    for k in 0..=12usize {
        if k > 0 {
            d = sym::diff(&d, x);
            fact = sym::mul(fact, sym::int(k as i64));
        }
        let v = sym::subs(&d, x, a);
        if sym::has_undefined(&v) {
            return None;
        }
        if sym::is_zero(&v) {
            continue;
        }
        let f = eval_mp(&v, &|_| None).ok()?;
        if !f.is_finite() || f.abs() < 1e-12 {
            return None;                       // not decidably nonzero (a cancellation the expression does not show)
        }
        return Some((k, sym::div(v, fact)));
    }
    None
}

/// sympy's limit(e, x, a, dir) at a finite point for an expression whose numerator and denominator are analytic
/// there: the exact value (a finite limit), or zoo for an infinite one (sympy gives oo, -oo or zoo, each refused as
/// infinite); anything else is not modelled
pub fn limit(e: &Ex, x: &Rc<str>, pt: &Py, dir: &str) -> R {
    use crate::pylimit::{self, Lim};
    let undetermined = || verr("the limit does not exist or could not be determined");
    let as_py = |l: Lim| -> R {
        match l {
            Lim::Value(v) => Ok(ex(v)),
            Lim::Infinite => Ok(ex(Ex::Zoo)),
            Lim::Oscillates => Err(undetermined()),
        }
    };
    if let Py::Float(v) = pt {
        if v.is_infinite() {
            let l = pylimit::at_infinity(e, x, if *v > 0.0 { 1 } else { -1 })
                .map_err(|w| unsup(format!("limit at infinity: {w:?}")))?;
            return as_py(l);
        }
    }
    let a = ex_of(pt)?;
    let side = |s: i32| -> Result<Lim, PyErr> {
        match pylimit::one_sided(e, x, &a, s) {
            Ok(l) => Ok(l),
            Err(w) => {
                // the exact orders of vanishing of numerator and denominator (analytic at the point)
                let (n, d) = numer_denom(e);
                match (order_at(&n, x, &a), order_at(&d, x, &a)) {
                    (Some((vn, cn)), Some((vd, cd))) => Ok(if vn >= vd {
                        Lim::Value(if vn == vd { sym::div(cn, cd) } else { sym::int(0) })
                    } else {
                        Lim::Infinite
                    }),
                    _ => Err(unsup(format!("limit at a point: {w:?}"))),
                }
            }
        }
    };
    match dir {
        "+" => as_py(side(1)?),
        "-" => as_py(side(-1)?),
        _ => {
            let (r, l) = (side(1)?, side(-1)?);
            match (&l, &r) {
                (Lim::Value(p), Lim::Value(q)) => {
                    let same = p == q || (eval_mp(&sym::sub(p.clone(), q.clone()), &|_| None).ok() == Some(0.0)
                        && sym::is_number(p) && sym::is_number(q));
                    if same {
                        return Ok(ex(q.clone()));
                    }
                    if is_decidably_different(p, q) {
                        return Err(verr(format!("The limit does not exist since left hand limit = {} and right hand limit = {}",
                                                sym::sstr(p), sym::sstr(q))));
                    }
                    Err(unsup("limit: one-sided values not decidably equal or different"))
                }
                (Lim::Infinite, _) | (_, Lim::Infinite) => Ok(ex(Ex::Zoo)),
                _ => Err(undetermined()),
            }
        }
    }
}

fn is_decidably_different(p: &Ex, q: &Ex) -> bool {
    matches!((eval_mp(p, &|_| None), eval_mp(q, &|_| None)), (Ok(a), Ok(b)) if (a - b).abs() > 1e-12 * a.abs().max(b.abs()).max(1.0))
}

/// The antiderivative in x (other symbols are constants), without a constant: of a polynomial in x, and term by term
/// of constants times sin, cos or exp of a linear argument (a divergence of a field with sin(y) or exp(z/2), calculus
/// 2026-10-30). Anything else is refused as before.
fn antiderivative(e: &Ex, x: &Rc<str>) -> Result<Ex, PyErr> {
    let ex_ = expand(e);
    if let Some(cs) = ex_poly(&ex_, x) {
        return Ok(poly_antiderivative(cs, x));
    }
    let terms: Vec<Ex> = match &ex_ { Ex::Add(t) => (**t).clone(), other => vec![other.clone()] };
    let mut poly_part = vec![];
    let mut out = vec![];
    for t in terms {
        if ex_poly(&t, x).is_some() {
            poly_part.push(t);
            continue;
        }
        // the term as (constant factors) * (one factor in x)
        let fs: Vec<Ex> = match &t { Ex::Mul(f) => (**f).clone(), other => vec![other.clone()] };
        let (dep, free): (Vec<Ex>, Vec<Ex>) = fs.into_iter().partition(|f| has_var(f, x));
        let [f] = dep.as_slice() else { return Err(unsup("integrate of a non-polynomial")) };
        let c = sym::mul_all(free);
        let (fun, arg) = match f {
            Ex::Fun(g @ (Fun::Sin | Fun::Cos | Fun::Exp), a) => (*g, (**a).clone()),
            Ex::Pow(b, a) if **b == Ex::E => (Fun::Exp, (**a).clone()),
            _ => return Err(unsup("integrate of a non-polynomial")),
        };
        // a linear argument a*x + b, a a nonzero constant
        let Some(lin) = ex_poly(&expand(&arg), x) else { return Err(unsup("integrate of a non-polynomial")) };
        if lin.len() != 2 || matches!(&lin[1], Ex::Num(q) if q.is_zero()) {
            return Err(unsup("integrate of a non-polynomial"));
        }
        let a = lin[1].clone();
        let anti = match fun {
            Fun::Sin => sym::neg(sym::func(Fun::Cos, arg)),
            Fun::Cos => sym::func(Fun::Sin, arg),
            _ => sym::func(Fun::Exp, arg),
        };
        out.push(sym::div(sym::mul(c, anti), a));
    }
    if !poly_part.is_empty() {
        let cs = ex_poly(&sym::add_all(poly_part), x).ok_or_else(|| unsup("integrate of a non-polynomial"))?;
        out.push(poly_antiderivative(cs, x));
    }
    Ok(sym::add_all(out))
}

fn poly_antiderivative(cs: Vec<Ex>, x: &Rc<str>) -> Ex {
    let xv = Ex::Sym(x.clone());
    sym::add_all(cs.into_iter().enumerate().map(|(k, c)| {
        sym::mul(sym::mul(c, sym::num(Q::new(BigInt::one(), BigInt::from(k + 1)))), sym::pow(xv.clone(), sym::int(k as i64 + 1)))
    }).collect())
}

/// sympy's integrate(e, x) and integrate(e, (x, a, b), (y, c, d), ...) for polynomial integrands: exact
pub fn integrate(e: &Ex, specs: &[Py]) -> R {
    if specs.is_empty() {
        return Err(unsup("integrate without a variable"));
    }
    let mut cur = e.clone();
    for sp in specs {
        match sp {
            Py::Tuple(t) if t.len() == 3 => {
                let x = var_name(&t[0])?;
                let (lo, hi) = (ex_of(&t[1])?, ex_of(&t[2])?);
                let f = antiderivative(&cur, &x)?;
                cur = sym::sub(subs_many(&f, &[(x.clone(), hi)]), subs_many(&f, &[(x, lo)]));
            }
            other => {
                let x = var_name(other)?;
                cur = antiderivative(&cur, &x)?;
            }
        }
    }
    Ok(ex(expand(&cur)))
}

/// The variable of a one-variable expression (x if it has none)
fn single_gen(e: &Ex) -> Rc<str> {
    let mut v = vec![];
    sym::free_symbols(e, &mut v);
    v.into_iter().next().unwrap_or_else(|| Rc::from("x"))
}

/// a * b with both multiplied out term by term (never rebuilding a product of sums, which the automatic
/// simplification would fold back into a power: (y - 1)(y - 1) = (y - 1)^2)
fn mul_terms(a: &Ex, b: &Ex) -> Ex {
    let ta: Vec<Ex> = match a { Ex::Add(t) => (**t).clone(), other => vec![other.clone()] };
    let tb: Vec<Ex> = match b { Ex::Add(t) => (**t).clone(), other => vec![other.clone()] };
    let mut out = vec![];
    for x in &ta {
        for y in &tb {
            out.push(sym::mul(x.clone(), y.clone()));
        }
    }
    sym::add_all(out)
}

/// sympy's expand: products and integer powers of sums multiplied out
pub fn expand(e: &Ex) -> Ex {
    match e {
        Ex::Add(ts) => sym::add_all(ts.iter().map(expand).collect()),
        Ex::Mul(fs) => {
            let mut acc = sym::int(1);
            for f in fs.iter() {
                acc = mul_terms(&acc, &expand(f));
            }
            acc
        }
        Ex::Pow(b, p) => {
            let b2 = expand(b);
            if let (Ex::Add(_), Ex::Num(k)) = (&b2, &**p) {
                if k.is_integer() && k.is_positive() && *k <= sym::q(64, 1) {
                    let n = k.to_integer().to_u32().unwrap_or(0);
                    let mut acc = sym::int(1);
                    for _ in 0..n {
                        acc = mul_terms(&acc, &b2);
                    }
                    return acc;
                }
            }
            sym::pow(b2, (**p).clone())
        }
        Ex::Fun(f, a) => sym::func(*f, expand(a)),
        other => other.clone(),
    }
}

/// a op b with a polynomial operand: a polynomial where sympy keeps one (+, -, * with a polynomial or a number,
/// ** a non-negative integer), else the expression (sympy's Poly / anything is an expression)
pub fn poly_bin(a: &Py, b: &Py, op: &str) -> R {
    let gen = match (a, b) { (Py::Poly(p), _) | (_, Py::Poly(p)) => p.gen.clone(), _ => unreachable!() };
    let as_poly = |v: &Py| -> Option<Poly> {
        match v {
            Py::Poly(p) => Some((**p).clone()),
            other => poly_from(&py::to_ex(other)?, &gen).ok(),
        }
    };
    let pp = |q: Poly| Ok(Py::Poly(Rc::new(q)));
    match op {
        "+" | "-" | "*" => {
            if let (Some(x), Some(y)) = (as_poly(a), as_poly(b)) {
                return pp(match op { "+" => x.add(&y), "-" => x.sub(&y), _ => x.mul(&y) });
            }
        }
        "**" => {
            if let (Py::Poly(x), Some(Ex::Num(k))) = (a, py::to_ex(b)) {
                if k.is_integer() && !k.is_negative() && k <= sym::q(512, 1) {
                    return pp(x.pow(k.to_integer().to_u32().unwrap_or(0)));
                }
            }
        }
        _ => {}
    }
    let (x, y) = (ex_of(a)?, ex_of(b)?);
    Ok(ex(match op {
        "+" => sym::add(x, y),
        "-" => sym::sub(x, y),
        "*" => sym::mul(x, y),
        "/" => if sym::is_zero(&y) { Ex::Zoo } else { sym::div(x, y) },
        _ => sym::pow(x, y),
    }))
}

// ================================================================================================= methods
fn atoms_has(e: &Ex, what: &Py) -> bool {
    match what {
        Py::Sym(w) => contains(e, w),
        Py::Float(v) if v.is_infinite() => false,          // sp.oo / -sp.oo never occur in these expressions
        _ => false,
    }
}

fn contains(e: &Ex, w: &Ex) -> bool {
    if e == w {
        return true;
    }
    match e {
        Ex::Add(v) | Ex::Mul(v) => v.iter().any(|t| contains(t, w)),
        Ex::Pow(b, p) => contains(b, w) || contains(p, w),
        Ex::Fun(_, a) => contains(a, w),
        _ => false,
    }
}

/// e with several variables replaced at once (sympy's simultaneous subs)
pub fn subs_many(e: &Ex, pairs: &[(Rc<str>, Ex)]) -> Ex {
    match e {
        Ex::Sym(s) => pairs.iter().find(|(k, _)| k == s).map(|(_, v)| v.clone()).unwrap_or_else(|| e.clone()),
        Ex::Add(ts) => sym::add_all(ts.iter().map(|t| subs_many(t, pairs)).collect()),
        Ex::Mul(fs) => sym::mul_all(fs.iter().map(|f| subs_many(f, pairs)).collect()),
        Ex::Pow(b, p) => sym::pow(subs_many(b, pairs), subs_many(p, pairs)),
        Ex::Fun(f, a) => sym::func(*f, subs_many(a, pairs)),
        other => other.clone(),
    }
}

/// A method of a symbolic expression or a polynomial; None if the value is neither
pub fn method(o: &Py, name: &str, a: &[Py]) -> Option<R> {
    match o {
        Py::Sym(e) => Some(sym_method(e, name, a)),
        Py::Poly(p) => Some(poly_method(p, name, a)),
        _ => None,
    }
}

fn sym_method(e: &Ex, name: &str, a: &[Py]) -> R {
    let a0 = || a.first().cloned().ok_or_else(|| terr(format!(".{name}() missing an argument")));
    match name {
        "subs" => {
            // subs(x, v) or subs({x: v, y: w}) (simultaneous)
            let pairs: Vec<(Rc<str>, Ex)> = match (a0()?, a.get(1)) {
                (Py::Dict(d), None) => d.borrow().iter().map(|(k, v)| Ok((var_name(k)?, ex_of(v)?))).collect::<Result<_, PyErr>>()?,
                (k, Some(v)) => vec![(var_name(&k)?, ex_of(v)?)],
                _ => return Err(terr("subs() arguments")),
            };
            Ok(ex(subs_many(e, &pairs)))
        }
        "has" => Ok(Py::Bool(a.iter().any(|w| atoms_has(e, w)))),
        "is_polynomial" => {
            // sympy: a polynomial in every symbol given (all free symbols when none is)
            let vars: Vec<Rc<str>> = if a.is_empty() {
                let mut v = vec![];
                sym::free_symbols(e, &mut v);
                v
            } else {
                a.iter().map(var_name).collect::<Result<_, _>>()?
            };
            Ok(Py::Bool(vars.iter().all(|x| is_polynomial(e, x))))
        }
        "diff" => call("diff", &[&[ex(e.clone())][..], a].concat(), &[]),
        "expand" => Ok(ex(expand(e))),
        "integrate" => integrate(e, a),
        "as_expr" => Ok(ex(e.clone())),
        "evalf" => call("N", &[ex(e.clone())], &[]),
        _ => Err(unsup(format!("expression method .{name}"))),
    }
}

fn poly_method(p: &Poly, name: &str, a: &[Py]) -> R {
    let pp = |q: Poly| Py::Poly(Rc::new(q));
    if let Some(cs) = &p.irr {
        return match name {
            "degree" => Ok(Py::Int(cs.len() as i64 - 1)),
            "all_coeffs" => Ok(Py::list(cs.iter().rev().map(|c| ex(c.clone())).collect())),
            "as_expr" => {
                let x = Ex::Sym(p.gen.clone());
                Ok(ex(sym::add_all(cs.iter().enumerate()
                    .map(|(i, k)| sym::mul(k.clone(), sym::pow(x.clone(), sym::int(i as i64)))).collect())))
            }
            _ => Err(unsup(format!("method .{name} of a polynomial with irrational coefficients"))),
        };
    }
    match name {
        "degree" => Ok(if p.is_zero() { Py::Float(f64::NEG_INFINITY) } else { Py::Int(p.degree()) }),
        "all_coeffs" => Ok(Py::list(p.all_coeffs().into_iter().map(|c| ex(sym::num(c))).collect())),
        "as_expr" => Ok(ex(p.as_ex())),
        "diff" => Ok(pp(p.diff())),
        "integrate" => Ok(pp(p.integrate())),
        "eval" => {
            let x = a.first().ok_or_else(|| terr("eval() needs a point"))?;
            match py::to_ex(x) {
                Some(Ex::Num(q)) if !matches!(x, Py::Float(_)) => Ok(ex(sym::num(p.eval_q(&q)))),
                Some(_) | None => Ok(Py::Float(p.eval_f(x.f()?))),
            }
        }
        "count_roots" if a.is_empty() => Ok(Py::Int(real_roots(p).len() as i64)),
        "count_roots" => {
            let q = |v: &Py| match py::to_ex(v) { Some(Ex::Num(q)) => Ok(q), _ => Err(unsup("count_roots bounds")) };
            let (lo, hi) = (q(a.first().ok_or_else(|| terr("count_roots() bounds"))?)?, q(a.get(1).ok_or_else(|| terr("count_roots() bounds"))?)?);
            Ok(Py::Int(count_roots(p, &lo, &hi) as i64))
        }
        "LC" => Ok(ex(sym::num(p.lc()))),
        "monic" => Ok(pp(p.monic())),
        "sqf_part" => Ok(pp(p.sqf_part())),
        "sqf_list" => {
            let (c, fs) = p.sqf_list();
            Ok(Py::tuple(vec![ex(sym::num(c)), Py::list(fs.into_iter()
                .map(|(f, m)| Py::tuple(vec![pp(f), Py::Int(m as i64)])).collect())]))
        }
        _ => Err(unsup(format!("polynomial method .{name}"))),
    }
}

/// An attribute of a symbolic expression or a polynomial (e.is_number, p.gen, q.p ...); None if neither
pub fn attr(o: &Py, name: &str) -> Option<R> {
    match o {
        Py::Sym(e) => Some(sym_attr(e, name)),
        Py::Poly(p) => Some(match name {
            "is_zero" => Ok(Py::Bool(p.is_zero())),
            "gen" => Ok(symbol(&p.gen)),
            "free_symbols" => Ok(Py::list(if p.degree() >= 1 { vec![symbol(&p.gen)] } else { vec![] })),
            _ => Err(unsup(format!("polynomial attribute .{name}"))),
        }),
        _ => None,
    }
}

fn sym_attr(e: &Ex, name: &str) -> R {
    let int_of = |b: &BigInt| b.to_i64().map(Py::Int).unwrap_or_else(|| py::int_of(b.clone()));
    match name {
        "is_number" => Ok(Py::Bool(sym::is_number(e))),
        "is_Rational" | "is_rational" => Ok(Py::Bool(matches!(e, Ex::Num(_)))),
        "is_Integer" | "is_integer" => Ok(Py::Bool(matches!(e, Ex::Num(q) if q.is_integer()))),
        "is_Float" => Ok(Py::Bool(false)),
        "is_Symbol" => Ok(Py::Bool(matches!(e, Ex::Sym(_)))),
        "is_Add" => Ok(Py::Bool(matches!(e, Ex::Add(_)))),
        "is_Mul" => Ok(Py::Bool(matches!(e, Ex::Mul(_)))),
        "is_Pow" => Ok(Py::Bool(matches!(e, Ex::Pow(..)))),
        "is_zero" => Ok(match e {
            Ex::Num(q) => Py::Bool(q.is_zero()),
            _ if sym::is_number(e) => Py::Bool(eval_mp(e, &|_| None).map(|v| v == 0.0).unwrap_or(false)),
            _ => Py::None,
        }),
        "p" => match e { Ex::Num(q) => Ok(int_of(q.numer())), _ => Err(terr("no attribute p")) },
        "q" => match e { Ex::Num(q) => Ok(int_of(q.denom())), _ => Err(terr("no attribute q")) },
        "free_symbols" => {
            let mut v = vec![];
            sym::free_symbols(e, &mut v);
            Ok(Py::list(v.iter().map(|s| symbol(s)).collect()))
        }
        "args" => Ok(Py::tuple(match e {
            Ex::Add(v) | Ex::Mul(v) => v.iter().map(|t| ex(t.clone())).collect(),
            Ex::Pow(b, p) => vec![ex((**b).clone()), ex((**p).clone())],
            Ex::Fun(_, a) => vec![ex((**a).clone())],
            _ => vec![],
        })),
        "base" => match e { Ex::Pow(b, _) => Ok(ex((**b).clone())), _ => Ok(ex(e.clone())) },
        "exp" => match e { Ex::Pow(_, p) => Ok(ex((**p).clone())), _ => Ok(ex(sym::int(1))) },
        _ => Err(unsup(format!("expression attribute .{name}"))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(c: &[i64]) -> Poly {
        Poly::new(Rc::from("x"), c.iter().map(|&k| Q::from_integer(BigInt::from(k))).collect())
    }

    #[test]
    fn roots_exact_and_isolated() {
        // (x - 1)(2x + 1)(x^2 - 2) = 2x^4 - x^3 - 5x^2 + 2x + 2
        let q = p(&[2, 2, -5, -1, 2]);
        let r = real_roots(&q);
        let s = q.sqf_part();
        let v: Vec<f64> = r.iter().map(|x| x.approx(&s)).collect();
        assert_eq!(v.len(), 4);
        assert!((v[0] + 2f64.sqrt()).abs() < 1e-15 && (v[1] + 0.5).abs() < 1e-15 && (v[2] - 1.0).abs() < 1e-15);
        assert!(matches!(r[1], Root::Exact(_)) && matches!(r[2], Root::Exact(_)) && matches!(r[0], Root::Iso(..)));
        // a double root counted once
        assert_eq!(real_roots(&p(&[1, -2, 1])).len(), 1);
        assert_eq!(count_roots(&p(&[-1, 0, 1]), &Q::from_integer(BigInt::from(-1)), &Q::from_integer(BigInt::from(1))), 2);
    }

    #[test]
    fn continuity_as_sympy() {
        let x: Rc<str> = Rc::from("x");
        let e = |s: &str| crate::symparse::parse(s, &["x"]).unwrap();
        let ok = |s: &str, a: f64, b: f64| continuous_on(&e(s), &x, a, b).is_ok();
        for (s, a, b, want) in [("sqrt(x)", 0.0, 1.0, true), ("log(x)", 0.0, 1.0, false), ("1/x", -1.0, 1.0, false),
                                ("x^(1/3)", -1.0, 1.0, true), ("sqrt(1-x^2)", -1.0, 1.0, true), ("tan(x)", 0.0, 2.0, false),
                                ("tan(x)", 0.0, 1.0, true), ("asin(2x)", 0.0, 1.0, false), ("1/sqrt(x)", 0.0, 1.0, false),
                                ("x^x", 0.0, 1.0, true), ("x^sin(x)", -1.0, 1.0, false), ("1/(x^3-2)", 0.5, 2.0, false),
                                ("1/(x^3-2)", -1.0, 1.0, true), ("log(sin(x))", 0.1, 3.0, true), ("1/(x-0.5)", 0.0, 1.0, false),
                                ("cot(x)", 0.0, 1.0, false), ("1/(sin(x)+2)", -1.0, 1.0, true), ("sqrt(x^2-1)", 0.5, 2.0, false)] {
            assert_eq!(ok(s, a, b), want, "{s} on [{a}, {b}]");
        }
        // exp is decided like any other function (exp(1/x) is not continuous where 1/x is not)
        assert!(!ok("exp(1/x)", -1.0, 1.0) && ok("exp(1/x)", 0.5, 2.0) && ok("exp(-x^2)", -1.0, 1.0));
        assert!(matches!(continuous_on(&e("log(x)"), &x, 0.0, 1.0),
                         Err(PyErr::Value(m)) if m == "the function is not continuous (or not real) on the whole interval [0, 1]"));
    }

    #[test]
    fn taylor_derivatives() {
        let e = crate::symparse::parse("x^2*exp(x) + sin(3x)/(1+x^2) + sqrt(x+4) + atan(x)*log(x+2)", &["x"]).unwrap();
        for n in 1..=6 {
            let mut d = e.clone();
            for _ in 0..n {
                d = sym::diff(&d, "x");
            }
            let want = sym::eval(&d, &|_| Some(0.7)).unwrap();
            let got = derivative_ad(&e, "x", 0.7, n).unwrap();
            assert!((got - want).abs() <= 1e-11 * want.abs().max(1.0), "order {n}: {got} vs {want}");
        }
    }

    #[test]
    fn mixed_partials_by_directions() {
        let e = crate::symparse::parse("x^2*y^3 + sin(x*y) + exp(x - z)*y", &["x", "y", "z"]).unwrap();
        let names: Vec<Rc<str>> = ["x", "y", "z"].iter().map(|s| Rc::from(*s)).collect();
        let p = [0.7, 1.3, -0.4];
        for orders in [[1, 1, 0], [2, 1, 0], [1, 2, 0], [1, 1, 1], [0, 2, 1], [3, 0, 0]] {
            let mut d = e.clone();
            for (v, &k) in ["x", "y", "z"].iter().zip(&orders) {
                for _ in 0..k {
                    d = sym::diff(&d, v);
                }
            }
            let want = sym::eval(&d, &|s| ["x", "y", "z"].iter().position(|n| *n == s).map(|i| p[i])).unwrap();
            let got = partial(&e, &names, &p, &orders).unwrap();
            assert!((got - want).abs() <= 1e-11 * want.abs().max(1.0), "{orders:?}: {got} vs {want}");
        }
    }

    #[test]
    fn zeta_and_orthogonal_polynomials() {
        let z = |s: f64| crate::pymp::call("zeta", &[Py::Float(s)], &[]).unwrap().f().unwrap();
        assert!((z(2.0) - std::f64::consts::PI.powi(2) / 6.0).abs() < 1e-15);
        assert!((z(1.5) - 2.612_375_348_685_488).abs() < 1e-14);
        assert!((z(4.0) - std::f64::consts::PI.powi(4) / 90.0).abs() < 1e-15);
        let p = crate::pymp::call("legendre", &[Py::Int(3), Py::Float(0.5)], &[]).unwrap().f().unwrap();
        assert_eq!(p, -0.4375);
        let t = crate::pymp::call("chebyt", &[Py::Int(5), Py::Float(0.3)], &[]).unwrap().f().unwrap();
        assert!((t - (5.0 * 0.3f64.acos()).cos()).abs() < 1e-15);
    }

    #[test]
    fn gauss_legendre_integrates() {
        let f = py::func(|a: &[Py]| Ok(Py::Float(a[0].f()?.sin_lm())));
        let v = gl_panels(&f, 0.0, std::f64::consts::PI, 8).unwrap();
        assert!((v - 2.0).abs() < 1e-14);
    }
}
