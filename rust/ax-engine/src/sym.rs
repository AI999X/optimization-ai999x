//! Exact symbolic expressions for the calculus and differential-equations procedures: rationals (a written decimal is
//! the exact rational it spells), pi and e, named real variables, sums, products, powers and the allowlisted functions.
//! Construction simplifies as sympy evaluates automatically where it matters to an answer: numbers fold exactly, like
//! terms and like powers combine, a number distributes over a sum, integer powers expand over products, perfect powers
//! come out of roots (sqrt(12) = 2 sqrt(3)), E**x is exp(x), and functions take their exact special values (sin(pi/6)
//! = 1/2, log(1) = 0, tan(pi/2) undefined). Undefined results are Zoo (sympy's complex infinity) or Nan; the reader
//! refuses an expression holding either. Derivatives are exact; values are computed in double precision.

#[allow(unused_imports)]
use crate::lm::{Lm, LmF64};
use num_bigint::BigInt;
use num_integer::Integer;
use num_rational::BigRational as Q;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Fun {
    Sin, Cos, Tan, Sec, Csc, Cot, Asin, Acos, Atan, Sinh, Cosh, Tanh, Exp, Log, Abs,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Ex {
    Num(Q),
    Pi,
    E,
    Sym(Rc<str>),
    Add(Rc<Vec<Ex>>),
    Mul(Rc<Vec<Ex>>),
    Pow(Rc<Ex>, Rc<Ex>),
    Fun(Fun, Rc<Ex>),
    /// sympy's zoo (complex infinity) and nan: an undefined value
    Zoo,
    Nan,
}

// ------------------------------------------------------------------------------------------------- numbers
pub fn q(n: i64, d: i64) -> Q {
    Q::new(BigInt::from(n), BigInt::from(d))
}

pub fn num(v: Q) -> Ex {
    Ex::Num(v)
}

pub fn int(n: i64) -> Ex {
    Ex::Num(Q::from_integer(BigInt::from(n)))
}

pub fn int_big(n: BigInt) -> Ex {
    Ex::Num(Q::from_integer(n))
}

/// sympy.Rational(repr(x)): the decimal that repr writes, exactly (0.1 -> 1/10, 1e+20 -> 10^20); None for inf/nan.
pub fn rational_of_repr(x: f64) -> Option<Ex> {
    if !x.is_finite() {
        return None;
    }
    crate::pyfrac::from_str(&crate::num::py_repr_f64(x)).ok().map(Ex::Num)
}

fn as_num(e: &Ex) -> Option<&Q> {
    if let Ex::Num(v) = e { Some(v) } else { None }
}

pub fn is_zero(e: &Ex) -> bool {
    matches!(e, Ex::Num(v) if v.is_zero())
}

fn is_one(e: &Ex) -> bool {
    matches!(e, Ex::Num(v) if v.is_one())
}

pub fn has_undefined(e: &Ex) -> bool {
    match e {
        Ex::Zoo | Ex::Nan => true,
        Ex::Add(v) | Ex::Mul(v) => v.iter().any(has_undefined),
        Ex::Pow(a, b) => has_undefined(a) || has_undefined(b),
        Ex::Fun(_, a) => has_undefined(a),
        _ => false,
    }
}

pub fn free_symbols(e: &Ex, out: &mut Vec<Rc<str>>) {
    match e {
        Ex::Sym(s) => {
            if !out.contains(s) {
                out.push(s.clone());
            }
        }
        Ex::Add(v) | Ex::Mul(v) => v.iter().for_each(|x| free_symbols(x, out)),
        Ex::Pow(a, b) => {
            free_symbols(a, out);
            free_symbols(b, out);
        }
        Ex::Fun(_, a) => free_symbols(a, out),
        _ => {}
    }
}

pub fn is_number(e: &Ex) -> bool {
    let mut s = vec![];
    free_symbols(e, &mut s);
    s.is_empty()
}

/// |e| as a double when e has no variables (sympy's abs(b) of a number), else None.
pub fn numeric_abs(e: &Ex) -> Option<f64> {
    if !is_number(e) {
        return None;
    }
    eval(e, &|_| None).ok().map(f64::abs)
}

// ------------------------------------------------------------------------------------------------- construction
/// (rational coefficient, the rest) of a term: 3*x^2 -> (3, x^2), x -> (1, x), 5 -> (5, 1)
fn split_coeff(t: &Ex) -> (Q, Ex) {
    match t {
        Ex::Num(v) => (v.clone(), int(1)),
        Ex::Mul(fs) => {
            if let Some(Ex::Num(v)) = fs.first() {
                let rest: Vec<Ex> = fs[1..].to_vec();
                let r = if rest.len() == 1 { rest[0].clone() } else { Ex::Mul(Rc::new(rest)) };
                (v.clone(), r)
            } else {
                (Q::one(), t.clone())
            }
        }
        _ => (Q::one(), t.clone()),
    }
}

fn with_coeff(c: Q, rest: Ex) -> Ex {
    if c.is_zero() {
        return int(0);
    }
    if is_one(&rest) {
        return Ex::Num(c);
    }
    if c.is_one() {
        return rest;
    }
    match rest {
        Ex::Mul(fs) => {
            let mut v = vec![Ex::Num(c)];
            v.extend(fs.iter().cloned());
            Ex::Mul(Rc::new(v))
        }
        Ex::Add(ts) => add_all(ts.iter().map(|t| mul(Ex::Num(c.clone()), t.clone())).collect()),
        r => Ex::Mul(Rc::new(vec![Ex::Num(c), r])),
    }
}

pub fn add_all(terms: Vec<Ex>) -> Ex {
    let mut flat = vec![];
    for t in terms {
        match t {
            Ex::Add(v) => flat.extend(v.iter().cloned()),
            t => flat.push(t),
        }
    }
    if flat.iter().any(|t| matches!(t, Ex::Nan)) {
        return Ex::Nan;
    }
    let zoos = flat.iter().filter(|t| matches!(t, Ex::Zoo)).count();
    if zoos > 1 {
        return Ex::Nan;
    }
    if zoos == 1 {
        return Ex::Zoo;
    }
    let mut constant = Q::zero();
    let mut groups: BTreeMap<Ex, Q> = BTreeMap::new();
    for t in flat {
        if let Ex::Num(v) = &t {
            constant += v;
            continue;
        }
        let (c, rest) = split_coeff(&t);
        *groups.entry(rest).or_insert_with(Q::zero) += c;
    }
    let mut out: Vec<Ex> = groups.into_iter().filter(|(_, c)| !c.is_zero()).map(|(r, c)| with_coeff(c, r)).collect();
    if !constant.is_zero() {
        out.insert(0, Ex::Num(constant));
    }
    match out.len() {
        0 => int(0),
        1 => out.pop().unwrap(),
        _ => {
            out.sort();
            Ex::Add(Rc::new(out))
        }
    }
}

pub fn add(a: Ex, b: Ex) -> Ex {
    add_all(vec![a, b])
}

pub fn neg(a: Ex) -> Ex {
    mul(int(-1), a)
}

pub fn sub(a: Ex, b: Ex) -> Ex {
    add(a, neg(b))
}

/// (base, exponent) of a factor: x^2 -> (x, 2), x -> (x, 1)
fn split_pow(f: &Ex) -> (Ex, Ex) {
    match f {
        Ex::Pow(b, e) => ((**b).clone(), (**e).clone()),
        Ex::Fun(Fun::Exp, a) => (Ex::E, (**a).clone()),
        f => (f.clone(), int(1)),
    }
}

pub fn mul_all(factors: Vec<Ex>) -> Ex {
    let mut flat = vec![];
    for f in factors {
        match f {
            Ex::Mul(v) => flat.extend(v.iter().cloned()),
            f => flat.push(f),
        }
    }
    if flat.iter().any(|t| matches!(t, Ex::Nan)) {
        return Ex::Nan;
    }
    let has_zero = flat.iter().any(is_zero);
    let has_zoo = flat.iter().any(|t| matches!(t, Ex::Zoo));
    if has_zero && has_zoo {
        return Ex::Nan;
    }
    if has_zoo {
        return Ex::Zoo;
    }
    if has_zero {
        return int(0);
    }
    let mut coeff = Q::one();
    let mut groups: BTreeMap<Ex, Vec<Ex>> = BTreeMap::new();
    for f in flat {
        if let Ex::Num(v) = &f {
            coeff *= v;
            continue;
        }
        let (b, e) = split_pow(&f);
        groups.entry(b).or_default().push(e);
    }
    let mut out = vec![];
    for (b, es) in groups {
        let p = pow(b, add_all(es));
        match p {
            Ex::Num(v) => coeff *= v,
            Ex::Zoo => return Ex::Zoo,
            Ex::Mul(fs) => {
                for f in fs.iter() {
                    if let Ex::Num(v) = f {
                        coeff *= v;
                    } else {
                        out.push(f.clone());
                    }
                }
            }
            p => {
                if !is_one(&p) {
                    out.push(p);
                }
            }
        }
    }
    if coeff.is_zero() {
        return int(0);
    }
    out.sort();
    // a number times a single sum distributes over it (sympy: 2*(x + 1) is 2*x + 2)
    if out.len() == 1 && !coeff.is_one() {
        if let Ex::Add(ts) = &out[0] {
            return add_all(ts.iter().map(|t| mul(Ex::Num(coeff.clone()), t.clone())).collect());
        }
    }
    match (out.len(), coeff.is_one()) {
        (0, _) => Ex::Num(coeff),
        (1, true) => out.pop().unwrap(),
        _ => {
            if !coeff.is_one() {
                out.insert(0, Ex::Num(coeff));
            }
            Ex::Mul(Rc::new(out))
        }
    }
}

pub fn mul(a: Ex, b: Ex) -> Ex {
    mul_all(vec![a, b])
}

pub fn div(a: Ex, b: Ex) -> Ex {
    mul(a, pow(b, int(-1)))
}

pub fn sqrt(a: Ex) -> Ex {
    pow(a, num(q(1, 2)))
}

/// n^(1/k) as (root, rest) with n = root^k * rest and rest free of k-th powers (n > 0)
fn extract_root(n: &BigInt, k: u32) -> (BigInt, BigInt) {
    let mut root = BigInt::one();
    let mut rest = n.clone();
    // trial division by small primes (numbers here are modest), then a whole k-th root check of what remains
    let mut p = BigInt::from(2);
    while &p * &p <= rest && p < BigInt::from(100_000) {
        let pk = num_traits::pow(p.clone(), k as usize);
        while (&rest % &pk).is_zero() {
            rest /= &pk;
            root *= &p;
        }
        p += 1;
    }
    let r = rest.nth_root(k);
    if num_traits::pow(r.clone(), k as usize) == rest {
        root *= r;
        rest = BigInt::one();
    }
    (root, rest)
}

/// A rational to a rational power, exactly where sympy evaluates it: integer powers; perfect powers out of roots.
fn rat_pow(b: &Q, e: &Q) -> Option<Ex> {
    if e.is_integer() {
        let k = e.to_integer().to_i64()?;
        if b.is_zero() {
            return Some(if k > 0 { int(0) } else { Ex::Zoo });
        }
        let r = if k >= 0 { num_traits::pow(b.clone(), k as usize) } else { num_traits::pow(b.recip(), (-k) as usize) };
        return Some(Ex::Num(r));
    }
    if b.is_negative() {
        return None;                                      // a complex value: left as written
    }
    if b.is_zero() {
        return Some(if e.is_positive() { int(0) } else { Ex::Zoo });
    }
    // e = a + s/k with 0 < s < k
    let k = e.denom().to_u32()?;
    let whole = e.numer().div_floor(e.denom());
    let s = e.numer() - &whole * e.denom();
    let mut coeff = num_traits::pow(b.clone(), 0);
    if let Some(w) = whole.to_i64() {
        coeff = if w >= 0 { num_traits::pow(b.clone(), w as usize) } else { num_traits::pow(b.recip(), (-w) as usize) };
    }
    let (rn, restn) = extract_root(b.numer(), k);
    let (rd, restd) = extract_root(b.denom(), k);
    let sv = s.to_usize()?;
    coeff *= Q::new(num_traits::pow(rn, sv), num_traits::pow(rd, sv));
    let frac = Q::new(s, BigInt::from(k));
    let mut factors = vec![Ex::Num(coeff)];
    if !restn.is_one() {
        factors.push(Ex::Pow(Rc::new(int_big(restn)), Rc::new(Ex::Num(frac.clone()))));
    }
    if !restd.is_one() {
        factors.push(Ex::Pow(Rc::new(int_big(restd)), Rc::new(Ex::Num(-frac))));
    }
    if factors.len() == 1 {
        return Some(factors.pop().unwrap());
    }
    let c = match &factors[0] { Ex::Num(c) => c.clone(), _ => Q::one() };
    let rest: Vec<Ex> = factors[1..].to_vec();
    let body = if rest.len() == 1 { rest[0].clone() } else { Ex::Mul(Rc::new(rest)) };
    Some(with_coeff(c, body))
}

pub fn pow(b: Ex, e: Ex) -> Ex {
    if matches!(b, Ex::Nan) || matches!(e, Ex::Nan) {
        return Ex::Nan;
    }
    if is_zero(&e) {
        return int(1);
    }
    if is_one(&e) {
        return b;
    }
    if is_one(&b) {
        return int(1);
    }
    if matches!(b, Ex::Zoo) {
        return match as_num(&e) {
            Some(v) if v.is_positive() => Ex::Zoo,
            Some(_) => int(0),
            None => Ex::Nan,
        };
    }
    if matches!(b, Ex::E) {
        return func(Fun::Exp, e);
    }
    // |u|^(2k) = u^(2k) for a real u
    if let (Ex::Fun(Fun::Abs, u), Some(kv)) = (&b, as_num(&e)) {
        if kv.is_integer() && (kv.clone() / Q::from_integer(BigInt::from(2))).is_integer() {
            return pow((**u).clone(), e);
        }
    }
    // sqrt(u^(2k)) = |u|^k for a real u (every variable here is real), as sympy simplifies it
    if let (Ex::Pow(u, k), Some(h)) = (&b, as_num(&e)) {
        if *h == q(1, 2) {
            if let Some(kv) = as_num(k) {
                let two = Q::from_integer(BigInt::from(2));
                if kv.is_integer() && kv.is_positive() && (kv.clone() / &two).is_integer() && !is_number(u) {
                    return pow(func(Fun::Abs, (**u).clone()), num(kv / two));
                }
            }
        }
    }
    if let (Some(bv), Some(ev)) = (as_num(&b), as_num(&e)) {
        if let Some(r) = rat_pow(bv, ev) {
            return r;
        }
    }
    if is_zero(&b) {
        if let Some(ev) = as_num(&e) {
            return if ev.is_positive() { int(0) } else { Ex::Zoo };
        }
    }
    if let Some(ev) = as_num(&e) {
        if ev.is_integer() {
            match &b {
                // (x^a)^n = x^(a n) for an integer n
                Ex::Pow(bb, be) => return pow((**bb).clone(), mul((**be).clone(), e.clone())),
                Ex::Fun(Fun::Exp, a) => return func(Fun::Exp, mul((**a).clone(), e.clone())),
                // (2 x)^2 = 4 x^2
                Ex::Mul(fs) => return mul_all(fs.iter().map(|f| pow(f.clone(), e.clone())).collect()),
                _ => {}
            }
        }
    }
    Ex::Pow(Rc::new(b), Rc::new(e))
}

/// k with e = k*pi (k rational), when e is a rational multiple of pi
fn pi_multiple(e: &Ex) -> Option<Q> {
    match e {
        Ex::Pi => Some(Q::one()),
        Ex::Num(v) if v.is_zero() => Some(Q::zero()),
        Ex::Mul(fs) if fs.len() == 2 => match (&fs[0], &fs[1]) {
            (Ex::Num(c), Ex::Pi) => Some(c.clone()),
            _ => None,
        },
        _ => None,
    }
}

/// sin(k pi) for k with denominator 1, 2, 3, 4, 6: exact
fn sin_pi(k: &Q) -> Option<Ex> {
    let two = Q::from_integer(BigInt::from(2));
    let mut k = k.clone() % &two;                      // in (-2, 2)
    if k.is_negative() {
        k += &two;
    }
    let (mut sign, mut r) = (1i64, k.clone());
    if r >= Q::one() {
        sign = -1;
        r -= Q::one();
    }
    if r > q(1, 2) {
        r = Q::one() - r;
    }
    let v = if r.is_zero() {
        int(0)
    } else if r == q(1, 6) {
        num(q(1, 2))
    } else if r == q(1, 4) {
        div(sqrt(int(2)), int(2))
    } else if r == q(1, 3) {
        div(sqrt(int(3)), int(2))
    } else if r == q(1, 2) {
        int(1)
    } else {
        return None;
    };
    Some(mul(int(sign), v))
}

fn cos_pi(k: &Q) -> Option<Ex> {
    sin_pi(&(k + q(1, 2)))
}

/// A coefficient of -1 (or any negative number) in front: (true, the term with it positive)
fn looks_negative(a: &Ex) -> bool {
    let (c, _) = split_coeff(a);
    if c.is_negative() {
        return true;
    }
    if let Ex::Add(ts) = a {
        // sympy pulls the sign out when the leading (sorted) term is negative: sin(-x - 1) = -sin(x + 1)
        if let Some(first) = ts.iter().find(|t| !matches!(t, Ex::Num(_))) {
            let (c, _) = split_coeff(first);
            return c.is_negative();
        }
    }
    false
}

/// -a when a minus sign should come out of a function's argument (sin(-x) = -sin(x), Abs(-x) = Abs(x)). A sum
/// whose negation looks negative too (-3x^2 + atan(3x) and 3x^2 - atan(3x)) is decided once by the expressions'
/// order, so that taking the sign out never repeats.
fn negative_arg(a: &Ex) -> Option<Ex> {
    if !looks_negative(a) {
        return None;
    }
    let n = neg(a.clone());
    if looks_negative(&n) && n > *a {
        return None;
    }
    Some(n)
}

pub fn func(f: Fun, a: Ex) -> Ex {
    use Fun::*;
    if matches!(a, Ex::Nan) {
        return Ex::Nan;
    }
    if matches!(a, Ex::Zoo) {
        return if f == Log || f == Abs { Ex::Zoo } else { Ex::Nan };
    }
    let k = pi_multiple(&a);
    match f {
        Sin | Cos | Tan | Sec | Csc | Cot => {
            if let Some(k) = &k {
                let (s, c) = (sin_pi(k), cos_pi(k));
                if let (Some(s), Some(c)) = (s, c) {
                    return match f {
                        Sin => s,
                        Cos => c,
                        Tan => if is_zero(&c) { Ex::Zoo } else { div(s, c) },
                        Sec => if is_zero(&c) { Ex::Zoo } else { div(int(1), c) },
                        Csc => if is_zero(&s) { Ex::Zoo } else { div(int(1), s) },
                        _ => if is_zero(&s) { Ex::Zoo } else { div(c, s) },
                    };
                }
            }
            if let Some(p) = negative_arg(&a) {
                return match f {
                    Cos | Sec => func(f, p),
                    _ => neg(func(f, p)),
                };
            }
        }
        Asin | Atan | Sinh | Tanh => {
            if is_zero(&a) {
                return int(0);
            }
            if let Some(v) = as_num(&a) {
                if f == Asin && v.is_one() {
                    return div(Ex::Pi, int(2));
                }
                if f == Asin && *v == -Q::one() {
                    return neg(div(Ex::Pi, int(2)));
                }
                if f == Atan && v.is_one() {
                    return div(Ex::Pi, int(4));
                }
                if f == Atan && *v == -Q::one() {
                    return neg(div(Ex::Pi, int(4)));
                }
            }
            if let Some(p) = negative_arg(&a) {
                return neg(func(f, p));
            }
        }
        Acos => {
            if let Some(v) = as_num(&a) {
                if v.is_zero() {
                    return div(Ex::Pi, int(2));
                }
                if v.is_one() {
                    return int(0);
                }
                if *v == -Q::one() {
                    return Ex::Pi;
                }
            }
        }
        Cosh => {
            if is_zero(&a) {
                return int(1);
            }
            if let Some(p) = negative_arg(&a) {
                return func(Cosh, p);
            }
        }
        Exp => {
            if is_zero(&a) {
                return int(1);
            }
            if is_one(&a) {
                return Ex::E;
            }
            if let Ex::Fun(Log, inner) = &a {
                return (**inner).clone();                // exp(log(x)) = x
            }
        }
        Log => {
            if is_one(&a) {
                return int(0);
            }
            if is_zero(&a) {
                return Ex::Zoo;
            }
            if matches!(a, Ex::E) {
                return int(1);
            }
            if let Ex::Fun(Exp, inner) = &a {
                return (**inner).clone();                // log(exp(x)) = x for real x
            }
        }
        Abs => {
            if let Some(v) = as_num(&a) {
                return Ex::Num(v.abs());
            }
            if let Some(p) = negative_arg(&a) {
                return func(Abs, p);
            }
            // |u^(2k)| = u^(2k) and ||u|| = |u| for a real u
            if let Ex::Pow(_, k) = &a {
                if let Some(kv) = as_num(k) {
                    if kv.is_integer() && (kv / Q::from_integer(BigInt::from(2))).is_integer() {
                        return a;
                    }
                }
            }
            if let Ex::Fun(Abs, _) = &a {
                return a;
            }
        }
    }
    Ex::Fun(f, Rc::new(a))
}

// ------------------------------------------------------------------------------------------------- derivatives
/// d e / d x, exactly
pub fn diff(e: &Ex, x: &str) -> Ex {
    use Fun::*;
    match e {
        Ex::Num(_) | Ex::Pi | Ex::E => int(0),
        Ex::Zoo | Ex::Nan => Ex::Nan,
        Ex::Sym(s) => int(if &**s == x { 1 } else { 0 }),
        Ex::Add(ts) => add_all(ts.iter().map(|t| diff(t, x)).collect()),
        Ex::Mul(fs) => {
            let mut terms = vec![];
            for i in 0..fs.len() {
                let d = diff(&fs[i], x);
                if is_zero(&d) {
                    continue;
                }
                let mut parts: Vec<Ex> = fs.iter().enumerate().filter(|(j, _)| *j != i).map(|(_, f)| f.clone()).collect();
                parts.push(d);
                terms.push(mul_all(parts));
            }
            add_all(terms)
        }
        Ex::Pow(b, p) => {
            let (b, p) = ((**b).clone(), (**p).clone());
            let (db, dp) = (diff(&b, x), diff(&p, x));
            if is_zero(&dp) {
                // p b^(p-1) b'
                return mul_all(vec![p.clone(), pow(b, sub(p, int(1))), db]);
            }
            // b^p (p' log b + p b'/b)
            mul(pow(b.clone(), p.clone()), add(mul(dp, func(Log, b.clone())), div(mul(p, db), b)))
        }
        Ex::Fun(f, a) => {
            let a = (**a).clone();
            let da = diff(&a, x);
            if is_zero(&da) {
                return int(0);
            }
            let outer = match f {
                Sin => func(Cos, a),
                Cos => neg(func(Sin, a)),
                Tan => add(int(1), pow(func(Tan, a), int(2))),
                Sec => mul(func(Sec, a.clone()), func(Tan, a)),
                Csc => neg(mul(func(Cot, a.clone()), func(Csc, a))),
                Cot => neg(add(int(1), pow(func(Cot, a), int(2)))),
                Asin => pow(sub(int(1), pow(a, int(2))), num(q(-1, 2))),
                Acos => neg(pow(sub(int(1), pow(a, int(2))), num(q(-1, 2)))),
                Atan => pow(add(int(1), pow(a, int(2))), int(-1)),
                Sinh => func(Cosh, a),
                Cosh => func(Sinh, a),
                Tanh => sub(int(1), pow(func(Tanh, a), int(2))),
                Exp => func(Exp, a),
                Log => pow(a, int(-1)),
                Abs => mul(a.clone(), pow(func(Abs, a), int(-1))),           // sign(a) = a/|a|
            };
            mul(outer, da)
        }
    }
}

// ------------------------------------------------------------------------------------------------- values
/// Why a value cannot be given: undefined (division by zero, a pole), or not a real number.
#[derive(Debug, Clone, PartialEq)]
pub enum EvalErr {
    Undefined,
    NotReal,
}

/// The value in double precision; `var` gives a variable's value.
pub fn eval(e: &Ex, var: &dyn Fn(&str) -> Option<f64>) -> Result<f64, EvalErr> {
    use Fun::*;
    let v = match e {
        Ex::Num(v) => v.to_f64().ok_or(EvalErr::Undefined)?,
        Ex::Pi => std::f64::consts::PI,
        Ex::E => std::f64::consts::E,
        Ex::Sym(s) => var(s).ok_or(EvalErr::Undefined)?,
        Ex::Zoo | Ex::Nan => return Err(EvalErr::Undefined),
        Ex::Add(ts) => {
            let mut s = 0.0;
            for t in ts.iter() {
                s += eval(t, var)?;
            }
            s
        }
        Ex::Mul(fs) => {
            let mut p = 1.0;
            for f in fs.iter() {
                p *= eval(f, var)?;
            }
            p
        }
        Ex::Pow(b, p) => {
            let bv = eval(b, var)?;
            if let Ex::Num(pv) = &**p {
                if pv.is_integer() {
                    let k = pv.to_integer().to_i32().ok_or(EvalErr::Undefined)?;
                    if bv == 0.0 && k < 0 {
                        return Err(EvalErr::Undefined);
                    }
                    return Ok(bv.powi(k));
                }
                if *pv == q(1, 2) {
                    if bv < 0.0 {
                        return Err(EvalErr::NotReal);
                    }
                    return Ok(bv.sqrt());
                }
            }
            let pv = eval(p, var)?;
            if bv < 0.0 && pv != pv.trunc() {
                return Err(EvalErr::NotReal);
            }
            if bv == 0.0 && pv < 0.0 {
                return Err(EvalErr::Undefined);
            }
            bv.powf_lm(pv)
        }
        Ex::Fun(f, a) => {
            let x = eval(a, var)?;
            match f {
                Sin => x.sin_lm(),
                Cos => x.cos_lm(),
                Tan => x.tan_lm(),
                Sec => 1.0 / x.cos_lm(),
                Csc => 1.0 / x.sin_lm(),
                Cot => x.cos_lm() / x.sin_lm(),
                Asin | Acos if x.abs() > 1.0 => return Err(EvalErr::NotReal),
                Asin => x.asin_lm(),
                Acos => x.acos_lm(),
                Atan => x.atan_lm(),
                Sinh => x.sinh_lm(),
                Cosh => x.cosh_lm(),
                Tanh => x.tanh_lm(),
                Exp => x.exp_lm(),
                Log if x < 0.0 => return Err(EvalErr::NotReal),
                Log if x == 0.0 => return Err(EvalErr::Undefined),
                Log => x.ln_lm(),
                Abs => x.abs(),
            }
        }
    };
    if !v.is_finite() {
        return Err(EvalErr::Undefined);
    }
    Ok(v)
}

/// e with the variable x replaced by the expression v
pub fn subs(e: &Ex, x: &str, v: &Ex) -> Ex {
    match e {
        Ex::Sym(s) if &**s == x => v.clone(),
        Ex::Add(ts) => add_all(ts.iter().map(|t| subs(t, x, v)).collect()),
        Ex::Mul(fs) => mul_all(fs.iter().map(|f| subs(f, x, v)).collect()),
        Ex::Pow(b, p) => pow(subs(b, x, v), subs(p, x, v)),
        Ex::Fun(f, a) => func(*f, subs(a, x, v)),
        other => other.clone(),
    }
}

// ------------------------------------------------------------------------------------------------- text
fn fun_name(f: Fun) -> &'static str {
    use Fun::*;
    match f {
        Sin => "sin", Cos => "cos", Tan => "tan", Sec => "sec", Csc => "csc", Cot => "cot", Asin => "asin",
        Acos => "acos", Atan => "atan", Sinh => "sinh", Cosh => "cosh", Tanh => "tanh", Exp => "exp", Log => "log",
        Abs => "Abs",
    }
}

/// The polynomial degree of a term in its variables (for printing sums highest degree first, as sympy does).
fn degree(e: &Ex) -> Q {
    match e {
        Ex::Sym(_) => Q::one(),
        Ex::Pow(b, p) => match (&**b, &**p) {
            (Ex::Sym(_), Ex::Num(k)) => k.clone(),
            _ => Q::zero(),
        },
        Ex::Mul(fs) => fs.iter().map(degree).fold(Q::zero(), |a, b| a + b),
        _ => Q::zero(),
    }
}

/// sympy's str() of an expression, as far as these procedures print one (x**2 - 2*x + 1, sqrt(x), exp(-x), 1/x)
pub fn sstr(e: &Ex) -> String {
    match e {
        Ex::Num(v) => if v.is_integer() { v.to_integer().to_string() } else { format!("{}/{}", v.numer(), v.denom()) },
        Ex::Pi => "pi".into(),
        Ex::E => "E".into(),
        Ex::Sym(s) => s.to_string(),
        Ex::Zoo => "zoo".into(),
        Ex::Nan => "nan".into(),
        Ex::Fun(f, a) => format!("{}({})", fun_name(*f), sstr(a)),
        Ex::Add(ts) => {
            let mut ts: Vec<&Ex> = ts.iter().collect();
            // highest degree first, numbers last (sympy's order for the polynomials and sums printed here)
            ts.sort_by(|a, b| degree(b).cmp(&degree(a)).then_with(|| matches!(a, Ex::Num(_)).cmp(&matches!(b, Ex::Num(_)))));
            let mut out = String::new();
            for (k, t) in ts.iter().enumerate() {
                let s = sstr(t);
                if k == 0 {
                    out.push_str(&s);
                } else if let Some(rest) = s.strip_prefix('-') {
                    out.push_str(" - ");
                    out.push_str(rest);
                } else {
                    out.push_str(" + ");
                    out.push_str(&s);
                }
            }
            out
        }
        Ex::Mul(fs) => {
            let (c, rest) = split_coeff(e);
            let mut num_f = vec![];
            let mut den_f = vec![];
            let factors: Vec<Ex> = match &rest {
                Ex::Mul(v) => v.to_vec(),
                other => vec![other.clone()],
            };
            let _ = fs;
            for f in factors {
                match &f {
                    Ex::Pow(b, p) if matches!(&**p, Ex::Num(k) if k.is_negative()) => {
                        let k = as_num(p).unwrap();
                        den_f.push(pow((**b).clone(), Ex::Num(-k.clone())));
                    }
                    _ => num_f.push(f.clone()),
                }
            }
            let atom = |x: &Ex| match x {
                Ex::Add(_) => format!("({})", sstr(x)),
                _ => sstr(x),
            };
            let (cn, cd) = (c.numer().clone(), c.denom().clone());
            let mut top: Vec<String> = vec![];
            if cn.abs() != BigInt::one() || num_f.is_empty() {
                top.push(cn.abs().to_string());
            }
            top.extend(num_f.iter().map(atom));
            let mut bottom: Vec<String> = vec![];
            if !cd.is_one() {
                bottom.push(cd.to_string());
            }
            bottom.extend(den_f.iter().map(atom));
            let sign = if cn.is_negative() { "-" } else { "" };
            let t = top.join("*");
            match bottom.len() {
                0 => format!("{sign}{t}"),
                1 => format!("{sign}{t}/{}", bottom[0]),
                _ => format!("{sign}{t}/({})", bottom.join("*")),
            }
        }
        Ex::Pow(b, p) => {
            let base = match &**b {
                Ex::Add(_) | Ex::Mul(_) | Ex::Pow(..) => format!("({})", sstr(b)),
                Ex::Num(v) if v.is_negative() || !v.is_integer() => format!("({})", sstr(b)),
                _ => sstr(b),
            };
            if let Ex::Num(k) = &**p {
                if *k == q(1, 2) {
                    return format!("sqrt({})", sstr(b));
                }
                if *k == -Q::one() {
                    return format!("1/{base}");
                }
                if *k == q(-1, 2) {
                    return format!("1/sqrt({})", sstr(b));
                }
                if k.is_integer() && !k.is_negative() {
                    return format!("{base}**{}", sstr(p));
                }
                if k.is_negative() {
                    return format!("{base}**({})", sstr(p));
                }
                return format!("{base}**({})", sstr(p));
            }
            let ex = match &**p {
                Ex::Sym(_) | Ex::Pi | Ex::E => sstr(p),
                _ => format!("({})", sstr(p)),
            };
            format!("{base}**{ex}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn x() -> Ex {
        Ex::Sym(Rc::from("x"))
    }

    #[test]
    fn simplifies_as_sympy() {
        assert_eq!(sub(x(), x()), int(0));
        assert_eq!(add(x(), x()), mul(int(2), x()));
        assert_eq!(mul(x(), x()), pow(x(), int(2)));
        assert_eq!(sqrt(int(4)), int(2));
        assert_eq!(sqrt(int(12)), mul(int(2), sqrt(int(3))));
        assert_eq!(func(Fun::Sin, div(Ex::Pi, int(6))), num(q(1, 2)));
        assert_eq!(func(Fun::Tan, div(Ex::Pi, int(2))), Ex::Zoo);
        assert_eq!(func(Fun::Log, int(0)), Ex::Zoo);
        assert_eq!(div(int(1), int(0)), Ex::Zoo);
        assert_eq!(pow(Ex::E, x()), func(Fun::Exp, x()));
        assert_eq!(func(Fun::Cos, neg(x())), func(Fun::Cos, x()));
    }

    #[test]
    fn derivatives() {
        // d/dx x^3 sin(x) at 2: 3*4*sin(2) + 8*cos(2)
        let e = mul(pow(x(), int(3)), func(Fun::Sin, x()));
        let d = diff(&e, "x");
        let v = eval(&d, &|_| Some(2.0)).unwrap();
        assert!((v - (12.0 * 2f64.sin() + 8.0 * 2f64.cos())).abs() < 1e-12);
        // d/dx x^x = x^x (log x + 1)
        let d = diff(&pow(x(), x()), "x");
        let v = eval(&d, &|_| Some(2.0)).unwrap();
        assert!((v - 4.0 * (2f64.ln() + 1.0)).abs() < 1e-12);
    }
}
