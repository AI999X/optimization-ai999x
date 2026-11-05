//! Python values for procedures translated from the packages' Python by tools/py2rust.py. The translated code keeps
//! Python's own rules: int and float are different (7 // 2 is 3, 7 / 2 is 3.5), floor division and modulo round
//! toward minus infinity, sum() is compensated as in Python 3.12+, math functions raise "math domain error" where
//! Python's do, and a ValueError's message is the refusal's reason. Anything this runtime does not model raises
//! Unsupported, which the engine reports as an internal error (never a wrong number).

#[allow(unused_imports)]
use crate::lm::{Lm, LmF64};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub enum Py {
    None,
    Bool(bool),
    Int(i64),
    /// an int beyond 64 bits (always outside i64's range: a result that fits is an Int again), as Python's
    /// unbounded int; made by `int_of`
    Big(Rc<num_bigint::BigInt>),
    Float(f64),
    /// a complex number (re, im); its arithmetic is CPython 3.13's (crate::pycx)
    Complex(f64, f64),
    /// an exact fraction (fractions.Fraction) on unbounded integers (crate::pyfrac)
    Frac(Rc<crate::pyfrac::Q>),
    Str(Rc<str>),
    List(Rc<RefCell<Vec<Py>>>),
    Tuple(Rc<Vec<Py>>),
    Dict(Rc<RefCell<Vec<(Py, Py)>>>),
    /// a set or frozenset: its distinct items in first-seen order (made by py::set). Unlike a list it compares equal
    /// to another set with the same items in any order, `<=`/`<` are subset tests, it is never equal to a list, and
    /// isinstance(x, list) is False for it, as Python's
    Set(Rc<RefCell<Vec<Py>>>),
    /// a function value (a lambda, an inner def, a module function passed as an argument)
    Func(Rc<dyn Fn(&[Py]) -> R>),
    /// a compiled pattern and a match of Python's re module (crate::pyre)
    Re(Rc<crate::pyre::PyRe>),
    Match(Rc<crate::pyre::PyMatch>),
    /// an exact symbolic expression (sympy's role in the calculus procedures: crate::sym)
    Sym(Rc<crate::sym::Ex>),
    /// a polynomial with rational coefficients (sympy's Poly over QQ: crate::pysym)
    Poly(Rc<crate::pysym::Poly>),
    /// an mpmath mpf at an extended working precision (crate::pympf)
    Mpf(Rc<crate::pympf::M>),
}

impl std::fmt::Debug for Py {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Py::Func(_) => write!(f, "<function>"),
            other => write!(f, "{}", repr(other)),
        }
    }
}

/// f(..., **d): the dict's items appended to a call's keyword arguments
pub fn kw_extend(kw: &mut Vec<(Py, Py)>, d: &Py) -> Result<(), PyErr> {
    match d {
        Py::Dict(x) => {
            kw.extend(x.borrow().iter().cloned());
            Ok(())
        }
        _ => Err(PyErr::Type("argument after ** must be a mapping".into())),
    }
}

/// {**m} and d.update(m) into the dict `d`: a mapping's items (or an iterable of pairs, for update) in order,
/// replacing the value of a key already there and keeping its place, as Python's
pub fn kw_merge(d: &Py, m: &Py) -> Result<(), PyErr> {
    match m {
        Py::Dict(x) => {
            let items = x.borrow().clone();
            for (k, v) in items {
                setitem(d, &k, v)?;
            }
            Ok(())
        }
        Py::List(_) | Py::Tuple(_) => {
            for pair in m.items()? {
                let kv = pair.items()?;
                if kv.len() != 2 {
                    return Err(value_error(format!("dictionary update sequence element has length {}; 2 is required",
                                                   kv.len())));
                }
                setitem(d, &kv[0], kv[1].clone())?;
            }
            Ok(())
        }
        _ => Err(PyErr::Type("'object' is not a mapping".into())),
    }
}

/// dict.fromkeys(keys, value): every key (first occurrence's place) mapped to the same value
pub fn dict_fromkeys(keys: &Py, value: Py) -> R {
    let d = Py::dict(vec![]);
    for k in keys.items()? {
        setitem(&d, &k, value.clone())?;
    }
    Ok(d)
}

/// The keyword argument `name`, taken out of a call's keywords (None when not given)
pub fn kw_take(kw: &mut Vec<(Py, Py)>, name: &str) -> Option<Py> {
    let i = kw.iter().position(|(k, _)| matches!(k, Py::Str(s) if &**s == name))?;
    Some(kw.remove(i).1)
}

/// A function value.
pub fn func(f: impl Fn(&[Py]) -> R + 'static) -> Py {
    Py::Func(Rc::new(f))
}

/// A recursive inner function's way to itself: the cell is filled with a weak reference right after the function is
/// made (a strong one would keep the function alive forever), and read when the function calls itself.
pub type RecCell = Rc<RefCell<Option<std::rc::Weak<dyn Fn(&[Py]) -> R>>>>;

pub fn rec_cell() -> RecCell {
    Rc::new(RefCell::new(None))
}

pub fn rec_set(c: &RecCell, f: &Py) {
    if let Py::Func(g) = f {
        *c.borrow_mut() = Some(Rc::downgrade(g));
    }
}

pub fn rec_get(c: &RecCell) -> R {
    c.borrow().as_ref().and_then(|w| w.upgrade()).map(Py::Func)
        .ok_or_else(|| PyErr::Unsupported("a recursive inner function called after its scope ended".into()))
}

/// A closure cell (Python's): a variable shared by a function and the inner functions that use it
pub type Cell = Rc<RefCell<Option<Py>>>;

pub fn cell_new() -> Cell {
    Rc::new(RefCell::new(None))
}

pub fn cell_set(c: &Cell, v: Py) {
    *c.borrow_mut() = Some(v);
}

/// the cell's value; NameError (free variable referenced before assignment) when it was never set
pub fn cell_get(c: &Cell) -> R {
    c.borrow().clone().ok_or_else(|| PyErr::Other("NameError".into()))
}

/// f(args) for a function value.
pub fn call(f: &Py, args: &[Py]) -> R {
    match f {
        Py::Func(g) => g(args),
        _ => Err(PyErr::Type("object is not callable".into())),
    }
}

/// a is b: None, booleans, and the same object.
pub fn is(a: &Py, b: &Py) -> bool {
    match (a, b) {
        (Py::None, Py::None) => true,
        (Py::Bool(x), Py::Bool(y)) => x == y,
        (Py::List(x), Py::List(y)) | (Py::Set(x), Py::Set(y)) => Rc::ptr_eq(x, y),
        (Py::Dict(x), Py::Dict(y)) => Rc::ptr_eq(x, y),
        (Py::Tuple(x), Py::Tuple(y)) => Rc::ptr_eq(x, y),
        (Py::Str(x), Py::Str(y)) => Rc::ptr_eq(x, y),
        (Py::Int(x), Py::Int(y)) => x == y && (-5..=256).contains(x),
        _ => false,
    }
}

/// a += b: a list is extended in place (the same object, as Python), anything else is a + b.
pub fn iadd(a: &Py, b: &Py) -> R {
    if let (Py::List(x), Py::List(_) | Py::Tuple(_)) = (a, b) {
        let items = b.items()?;
        x.borrow_mut().extend(items);
        return Ok(a.clone());
    }
    add(a, b)
}

/// dict(base, **extra): a new dict from a mapping or a list of pairs, then the keyword pairs.
pub fn dict_from(base: &Py, extra: Vec<(Py, Py)>) -> R {
    let out = Py::dict(vec![]);
    match base {
        Py::Dict(d) => {
            for (k, v) in d.borrow().iter() {
                setitem(&out, k, v.clone())?;
            }
        }
        other => {
            for pair in other.items()? {
                let kv = pair.items()?;
                if kv.len() != 2 {
                    return Err(value_error("dictionary update sequence element has the wrong length"));
                }
                setitem(&out, &kv[0], kv[1].clone())?;
            }
        }
    }
    for (k, v) in extra {
        setitem(&out, &k, v)?;
    }
    Ok(out)
}

pub fn enumerate(a: &Py, start: i64) -> R {
    Ok(Py::list(a.items()?.into_iter().enumerate().map(|(k, x)| Py::tuple(vec![Py::Int(start + k as i64), x])).collect()))
}

pub fn zip(args: &[Py]) -> R {
    let lists: Vec<Vec<Py>> = args.iter().map(|a| a.items()).collect::<Result<_, _>>()?;
    let n = lists.iter().map(|l| l.len()).min().unwrap_or(0);
    Ok(Py::list((0..n).map(|k| Py::tuple(lists.iter().map(|l| l[k].clone()).collect())).collect()))
}

pub fn reversed(a: &Py) -> R {
    let mut v = a.items()?;
    v.reverse();
    Ok(Py::list(v))
}

pub fn map(f: &Py, a: &Py) -> R {
    Ok(Py::list(a.items()?.iter().map(|x| call(f, std::slice::from_ref(x))).collect::<Result<_, _>>()?))
}

/// set(xs) / frozenset(xs): the distinct items in first-seen order, as a Py::Set
pub fn set(a: &Py) -> R {
    Ok(Py::Set(Rc::new(RefCell::new(distinct(a.items()?)))))
}

fn distinct(xs: Vec<Py>) -> Vec<Py> {
    let mut out: Vec<Py> = vec![];
    for x in xs {
        if !out.iter().any(|y| eq(y, &x)) {
            out.push(x);
        }
    }
    out
}

/// for x in a: a list is walked by index, reading its length at every step, so items appended (or removed) by the
/// loop's body are seen as Python's list iterator sees them; anything else is iterated over a snapshot of its items
pub fn iter(a: &Py) -> Result<PyIter, PyErr> {
    Ok(match a {
        Py::List(l) => PyIter::Live(l.clone(), 0),
        other => PyIter::Snap(other.items()?.into_iter()),
    })
}

pub enum PyIter {
    Live(Rc<RefCell<Vec<Py>>>, usize),
    Snap(std::vec::IntoIter<Py>),
}

impl Iterator for PyIter {
    type Item = Py;
    fn next(&mut self) -> Option<Py> {
        match self {
            PyIter::Live(l, i) => {
                let x = l.borrow().get(*i).cloned();
                *i += 1;
                x
            }
            PyIter::Snap(it) => it.next(),
        }
    }
}

/// a set's methods (set and frozenset; the in-place ones change the set itself, as Python's)
fn set_method(o: &Py, l: &Rc<RefCell<Vec<Py>>>, name: &str, a: &[Py]) -> R {
    let arg = |k: usize| a.get(k).cloned().ok_or_else(|| PyErr::Type(format!("{name}() missing an argument")));
    let has = |v: &[Py], k: &Py| v.iter().any(|z| eq(z, k));
    let me = l.borrow().clone();
    let others: Vec<Vec<Py>> = if matches!(name, "add" | "discard" | "remove") {
        vec![]
    } else {
        a.iter().map(|x| x.items()).collect::<Result<_, _>>()?
    };
    if matches!(name, "issubset" | "isdisjoint") && others.is_empty() {
        return Err(PyErr::Type(format!("{name}() missing an argument")));
    }
    let new_set = |v: Vec<Py>| Py::Set(Rc::new(RefCell::new(v)));
    match name {
        "add" => {
            let x = arg(0)?;
            if !has(&me, &x) {
                l.borrow_mut().push(x);
            }
            Ok(Py::None)
        }
        "discard" | "remove" => {
            let x = arg(0)?;
            match me.iter().position(|y| eq(y, &x)) {
                Some(p) => {
                    l.borrow_mut().remove(p);
                    Ok(Py::None)
                }
                None if name == "remove" => Err(PyErr::Key(repr(&x))),
                None => Ok(Py::None),
            }
        }
        "pop" => {
            if me.is_empty() {
                return Err(PyErr::Key("'pop from an empty set'".into()));
            }
            Ok(l.borrow_mut().remove(0))
        }
        "clear" => {
            l.borrow_mut().clear();
            Ok(Py::None)
        }
        "copy" => Ok(new_set(me)),
        "union" | "update" => {
            let mut v = me;
            for x in others.into_iter().flatten() {
                if !has(&v, &x) {
                    v.push(x);
                }
            }
            if name == "update" {
                *l.borrow_mut() = v;
                return Ok(Py::None);
            }
            Ok(new_set(v))
        }
        "intersection" | "intersection_update" => {
            let v: Vec<Py> = me.into_iter().filter(|k| others.iter().all(|w| has(w, k))).collect();
            if name == "intersection_update" {
                *l.borrow_mut() = v;
                return Ok(Py::None);
            }
            Ok(new_set(v))
        }
        "difference" | "difference_update" => {
            let v: Vec<Py> = me.into_iter().filter(|k| !others.iter().any(|w| has(w, k))).collect();
            if name == "difference_update" {
                *l.borrow_mut() = v;
                return Ok(Py::None);
            }
            Ok(new_set(v))
        }
        "symmetric_difference" => {
            let w = distinct(arg(0)?.items()?);
            let v: Vec<Py> = me.iter().filter(|k| !has(&w, k)).cloned().chain(w.iter().filter(|k| !has(&me, k)).cloned())
                .collect();
            Ok(new_set(v))
        }
        "issubset" => Ok(Py::Bool(me.iter().all(|k| has(&others[0], k)))),
        "issuperset" => Ok(Py::Bool(arg(0)?.items()?.iter().all(|k| has(&me, k)))),
        "isdisjoint" => Ok(Py::Bool(!me.iter().any(|k| has(&others[0], k)))),
        _ => {
            let _ = o;
            Err(PyErr::Unsupported(format!("method set.{name}")))
        }
    }
}

/// a < b, a <= b, a > b, a >= b of two sets (kept as lists of distinct items): Python's proper subset, subset,
/// proper superset, superset tests
pub fn set_cmp(a: &Py, b: &Py, op: &str) -> Result<bool, PyErr> {
    let (x, y) = (set(a)?.items()?, set(b)?.items()?);
    let sub = |p: &[Py], q: &[Py]| p.iter().all(|v| q.iter().any(|w| eq(v, w)));
    Ok(match op {
        "<=" => sub(&x, &y),
        "<" => x.len() < y.len() && sub(&x, &y),
        ">=" => sub(&y, &x),
        ">" => y.len() < x.len() && sub(&y, &x),
        _ => return Err(PyErr::Unsupported(format!("set comparison {op}"))),
    })
}

pub fn prod(a: &Py) -> R {
    let mut acc = Py::Int(1);
    for x in a.items()? {
        acc = mul(&acc, &x)?;
    }
    Ok(acc)
}

pub fn dist(a: &Py, b: &Py) -> R {
    let (x, y) = (a.items()?, b.items()?);
    if x.len() != y.len() {
        return Err(value_error("both points must have the same number of dimensions"));
    }
    // math.dist: each coordinate as a float, |p - q|, then vector_norm (CPython's math_dist_impl)
    let mut d = vec![];
    let (mut max, mut nan) = (0.0f64, false);
    for (p, q) in x.iter().zip(y.iter()) {
        let v = (p.f()? - q.f()?).abs();
        nan |= v.is_nan();
        if v > max {
            max = v;
        }
        d.push(v);
    }
    Ok(Py::Float(vector_norm(&mut d, max, nan)))
}

/// min/max with key=: the first item whose key is smallest (largest).
pub fn minmax_key(args: &[Py], key: &Py, want_max: bool) -> R {
    let xs = if args.len() == 1 { args[0].items()? } else { args.to_vec() };
    let mut it = xs.into_iter();
    let mut best = it.next().ok_or_else(|| value_error("min() arg is an empty sequence"))?;
    let mut bk = call(key, std::slice::from_ref(&best))?;
    for x in it {
        let k = call(key, std::slice::from_ref(&x))?;
        let better = if want_max { gt(&k, &bk)? } else { lt(&k, &bk)? };
        if better {
            best = x;
            bk = k;
        }
    }
    Ok(best)
}

/// sorted(xs, key=f, reverse=r): a stable sort on the keys (reverse keeps equal items in order, as Python)
pub fn sorted_key(a: &Py, key: &Py, reverse: bool) -> R {
    let xs = a.items()?;
    let mut keyed: Vec<(Py, Py)> = xs.into_iter().map(|x| Ok((call(key, std::slice::from_ref(&x))?, x)))
        .collect::<Result<_, PyErr>>()?;
    let mut err = None;
    keyed.sort_by(|p, q| {
        let o = order(&p.0, &q.0).unwrap_or_else(|e| {
            err = Some(e);
            std::cmp::Ordering::Equal
        });
        if reverse { o.reverse() } else { o }
    });
    if let Some(e) = err {
        return Err(e);
    }
    Ok(Py::list(keyed.into_iter().map(|(_, x)| x).collect()))
}

pub fn method_sort(a: &Py, key: Option<&Py>, reverse: bool) -> R {
    let Py::List(l) = a else { return Err(PyErr::Type("sort of a non-list".into())) };
    let s = match key {
        Some(k) => sorted_key(a, k, reverse)?,
        None => sorted(a, reverse)?,
    };
    *l.borrow_mut() = s.items()?;
    Ok(Py::None)
}

/// obj.name(args) for the list, str and dict methods the procedures use.
pub fn method(o: &Py, name: &str, a: &[Py]) -> R {
    let arg = |k: usize| a.get(k).cloned().ok_or_else(|| PyErr::Type(format!("{name}() missing an argument")));
    match (o, name) {
        (Py::Sym(_) | Py::Poly(_), _) => crate::pysym::method(o, name, a).unwrap(),
        (Py::Re(r), _) => crate::pyre::call(r, name, a),
        (Py::Set(l), _) => set_method(o, l, name, a),
        // d.pop(k[, default]): the value, the key removed (the others keep their order); KeyError without a default
        (Py::Dict(d), "pop") => {
            let k = arg(0)?;
            let pos = d.borrow().iter().position(|(kk, _)| eq(kk, &k));
            match pos {
                Some(p) => Ok(d.borrow_mut().remove(p).1),
                None => a.get(1).cloned().ok_or_else(|| PyErr::Key(repr(&k))),
            }
        }
        (Py::Match(m), _) => m.method(name, a),
        (Py::Str(s), "translate") => str_translate(s, &arg(0)?),
        (Py::Complex(r, i), "conjugate") => Ok(Py::Complex(*r, -*i)),
        (Py::Frac(q), "limit_denominator") => {
            let m = match a.first() {
                Some(Py::Int(n)) => num_bigint::BigInt::from(*n),
                None => num_bigint::BigInt::from(1_000_000),
                _ => return Err(type_error("limit_denominator")),
            };
            Ok(frac(fr::limit_denominator(q, &m)?))
        }
        (Py::Frac(_), "conjugate") => Ok(o.clone()),
        (Py::Float(x), "is_integer") => Ok(Py::Bool(x.is_finite() && *x == x.trunc())),
        (Py::Int(_) | Py::Big(_) | Py::Bool(_), "is_integer") => Ok(Py::Bool(true)),
        (Py::Frac(q), "is_integer") => Ok(Py::Bool(q.is_integer())),
        (Py::Int(_) | Py::Big(_) | Py::Bool(_), "bit_length") => Ok(Py::Int(bi(o).map_or(0, |n| n.bits() as i64))),
        (Py::Int(_) | Py::Big(_) | Py::Float(_) | Py::Bool(_), "conjugate") => pos(o),
        (Py::List(l), "append") => {
            l.borrow_mut().push(arg(0)?);
            Ok(Py::None)
        }
        (Py::List(l), "extend") => {
            let items = arg(0)?.items()?;
            l.borrow_mut().extend(items);
            Ok(Py::None)
        }
        (Py::List(l), "insert") => {
            let n = l.borrow().len() as i64;
            let mut i = arg(0)?.i()?;
            if i < 0 {
                i = (i + n).max(0);
            }
            l.borrow_mut().insert(i.min(n) as usize, arg(1)?);
            Ok(Py::None)
        }
        (Py::List(l), "pop") => {
            let n = l.borrow().len();
            if n == 0 {
                return Err(PyErr::Index);
            }
            let i = if a.is_empty() { n - 1 } else { norm_index(arg(0)?.i()?, n)? };
            Ok(l.borrow_mut().remove(i))
        }
        // collections.deque is a list here: its left end
        (Py::List(l), "popleft") => {
            if l.borrow().is_empty() {
                return Err(PyErr::Index);
            }
            Ok(l.borrow_mut().remove(0))
        }
        (Py::List(l), "appendleft") => {
            l.borrow_mut().insert(0, arg(0)?);
            Ok(Py::None)
        }
        (Py::List(l), "remove") => {
            let x = arg(0)?;
            let pos = l.borrow().iter().position(|y| eq(y, &x)).ok_or_else(|| value_error("list.remove(x): x not in list"))?;
            l.borrow_mut().remove(pos);
            Ok(Py::None)
        }
        (Py::List(l), "reverse") => {
            l.borrow_mut().reverse();
            Ok(Py::None)
        }
        (Py::List(_) | Py::Tuple(_), "index") => {
            let x = arg(0)?;
            o.items()?.iter().position(|y| eq(y, &x)).map(|p| Py::Int(p as i64))
                .ok_or_else(|| value_error(format!("{} is not in list", repr(&x))))
        }
        (Py::List(_) | Py::Tuple(_), "count") => {
            let x = arg(0)?;
            Ok(Py::Int(o.items()?.iter().filter(|y| eq(y, &x)).count() as i64))
        }
        // a set is a list of distinct items here (py::set): add, discard, update keep it so; clear empties any
        (Py::List(l), "add") => {
            let x = arg(0)?;
            if !l.borrow().iter().any(|y| eq(y, &x)) {
                l.borrow_mut().push(x);
            }
            Ok(Py::None)
        }
        (Py::List(l), "discard") => {
            let x = arg(0)?;
            let pos = l.borrow().iter().position(|y| eq(y, &x));
            if let Some(p) = pos {
                l.borrow_mut().remove(p);
            }
            Ok(Py::None)
        }
        (Py::List(l), "update") => {
            for x in arg(0)?.items()? {
                if !l.borrow().iter().any(|y| eq(y, &x)) {
                    l.borrow_mut().push(x);
                }
            }
            Ok(Py::None)
        }
        (Py::List(l), "clear") => {
            l.borrow_mut().clear();
            Ok(Py::None)
        }
        (Py::Dict(d), "clear") => {
            d.borrow_mut().clear();
            Ok(Py::None)
        }
        (Py::List(l), "copy") => Ok(Py::list(l.borrow().clone())),
        (Py::Dict(d), "copy") => Ok(Py::dict(d.borrow().clone())),
        (Py::Dict(d), "get") => {
            let k = arg(0)?;
            Ok(d.borrow().iter().find(|(kk, _)| eq(kk, &k)).map(|(_, v)| v.clone())
                .unwrap_or_else(|| a.get(1).cloned().unwrap_or(Py::None)))
        }
        (Py::Dict(d), "items") => Ok(Py::list(d.borrow().iter().map(|(k, v)| Py::tuple(vec![k.clone(), v.clone()])).collect())),
        (Py::Dict(d), "keys") => Ok(Py::list(d.borrow().iter().map(|(k, _)| k.clone()).collect())),
        (Py::Dict(d), "values") => Ok(Py::list(d.borrow().iter().map(|(_, v)| v.clone()).collect())),
        (Py::Dict(_), "setdefault") => {
            let k = arg(0)?;
            if let Ok(v) = getitem(o, &k) {
                return Ok(v);
            }
            setitem(o, &k, arg(1)?)?;
            Ok(arg(1)?)
        }
        (Py::Dict(_), "update") => {
            if let Py::Dict(other) = arg(0)? {
                for (k, v) in other.borrow().iter() {
                    setitem(o, k, v.clone())?;
                }
                return Ok(Py::None);
            }
            Err(PyErr::Unsupported("dict.update with a non-dict".into()))
        }
        (Py::Str(s), _) => str_method(s, name, a),
        _ => Err(PyErr::Unsupported(format!("method .{name}"))),
    }
}

/// s[start:end] for str methods' optional start and end arguments a[k], a[k+1] (None or absent: the ends; negative:
/// from the end; clamped), with the window's start as a char position; None when start lies beyond the string
/// (Python then answers False / -1 / 0 even for an empty substring).
fn str_window<'a>(s: &'a str, a: &[Py], k: usize) -> Result<Option<(&'a str, i64)>, PyErr> {
    let n = s.chars().count() as i64;
    let pos = |v: Option<&Py>, dflt: i64| -> Result<i64, PyErr> {
        Ok(match v {
            None | Some(Py::None) => dflt,
            Some(x) => {
                let i = x.i()?;
                if i < 0 { (i + n).max(0) } else { i.min(n + 1) }
            }
        })
    };
    let st = pos(a.get(k), 0)?;
    let en = pos(a.get(k + 1), n)?.min(n);
    if st > n || en < st {
        return Ok(None);
    }
    let byte = |c: i64| s.char_indices().nth(c as usize).map(|(b, _)| b).unwrap_or(s.len());
    let (b0, b1) = (byte(st), byte(en.max(st)));
    Ok(Some((&s[b0..b1], st)))
}

fn str_method(s: &str, name: &str, a: &[Py]) -> R {
    let sarg = |k: usize| -> Result<String, PyErr> {
        Ok(a.get(k).ok_or_else(|| PyErr::Type("missing argument".into()))?.as_str()?.to_string())
    };
    Ok(match name {
        "join" => {
            let parts: Vec<String> = a.first().ok_or_else(|| PyErr::Type("join".into()))?.items()?.iter()
                .map(|p| p.as_str().map(|x| x.to_string())).collect::<Result<_, _>>()?;
            Py::str(&parts.join(s))
        }
        "upper" => Py::str(&s.to_uppercase()),
        "lower" => Py::str(&s.to_lowercase()),
        "title" => Py::str(&s.split(' ').map(|w| {
            let mut c = w.chars();
            match c.next() {
                Some(f) => f.to_uppercase().collect::<String>() + &c.as_str().to_lowercase(),
                None => String::new(),
            }
        }).collect::<Vec<_>>().join(" ")),
        "capitalize" => {
            let mut c = s.chars();
            Py::str(&match c.next() {
                Some(f) => f.to_uppercase().collect::<String>() + &c.as_str().to_lowercase(),
                None => String::new(),
            })
        }
        "strip" if a.is_empty() => Py::str(s.trim()),
        "lstrip" if a.is_empty() => Py::str(s.trim_start()),
        "rstrip" if a.is_empty() => Py::str(s.trim_end()),
        "strip" | "lstrip" | "rstrip" => {
            let set: Vec<char> = sarg(0)?.chars().collect();
            let f = |c: char| set.contains(&c);
            Py::str(match name { "strip" => s.trim_matches(f), "lstrip" => s.trim_start_matches(f), _ => s.trim_end_matches(f) })
        }
        "replace" => Py::str(&s.replace(&sarg(0)?, &sarg(1)?)),
        "split" if a.is_empty() => Py::list(s.split_whitespace().map(Py::str).collect()),
        "split" => Py::list(s.split(sarg(0)?.as_str()).map(Py::str).collect()),
        "splitlines" => Py::list(s.lines().map(Py::str).collect()),
        // with start/end (s[start:end], char positions, negatives from the end) and a tuple of prefixes, as Python
        "startswith" | "endswith" if a.len() > 1 || matches!(a.first(), Some(Py::Tuple(_))) => {
            let Some((w, _)) = str_window(s, a, 1)? else { return Ok(Py::Bool(false)) };
            let subs: Vec<Py> = match a.first() {
                Some(t @ Py::Tuple(_)) => t.items()?,
                Some(x) => vec![x.clone()],
                None => return Err(PyErr::Type("missing argument".into())),
            };
            let mut hit = false;
            for p in subs {
                let p = p.as_str()?.to_string();
                if if name == "startswith" { w.starts_with(p.as_str()) } else { w.ends_with(p.as_str()) } {
                    hit = true;
                    break;
                }
            }
            Py::Bool(hit)
        }
        "find" | "index" | "count" if a.len() > 1 => {
            let sub = sarg(0)?;
            let Some((w, off)) = str_window(s, a, 1)? else {
                return if name == "index" { Err(value_error("substring not found")) }
                       else { Ok(Py::Int(if name == "count" { 0 } else { -1 })) };
            };
            if name == "count" {
                Py::Int(if sub.is_empty() { w.chars().count() as i64 + 1 } else { w.matches(sub.as_str()).count() as i64 })
            } else {
                match w.find(sub.as_str()) {
                    Some(b) => Py::Int(off + w[..b].chars().count() as i64),
                    None if name == "index" => return Err(value_error("substring not found")),
                    None => Py::Int(-1),
                }
            }
        }
        "startswith" => Py::Bool(s.starts_with(sarg(0)?.as_str())),
        "endswith" => Py::Bool(s.ends_with(sarg(0)?.as_str())),
        "find" => Py::Int(s.find(sarg(0)?.as_str()).map(|b| s[..b].chars().count() as i64).unwrap_or(-1)),
        "isdigit" => Py::Bool(!s.is_empty() && s.chars().all(|c| c.is_ascii_digit())),
        "isascii" => Py::Bool(s.is_ascii()),
        "isalpha" => Py::Bool(!s.is_empty() && s.chars().all(|c| c.is_alphabetic())),
        "isupper" => Py::Bool(s.chars().any(|c| c.is_alphabetic()) && !s.chars().any(|c| c.is_lowercase())),
        "islower" => Py::Bool(s.chars().any(|c| c.is_alphabetic()) && !s.chars().any(|c| c.is_uppercase())),
        "isspace" => Py::Bool(!s.is_empty() && s.chars().all(|c| c.is_whitespace())),
        "count" => Py::Int(if sarg(0)?.is_empty() { s.chars().count() as i64 + 1 } else { s.matches(sarg(0)?.as_str()).count() as i64 }),
        "index" => {
            let sub = sarg(0)?;
            Py::Int(s.find(sub.as_str()).map(|b| s[..b].chars().count() as i64).ok_or_else(|| value_error("substring not found"))?)
        }
        "zfill" => {
            let w = a.first().ok_or_else(|| PyErr::Type("zfill".into()))?.i()? as usize;
            let n = s.chars().count();
            if n >= w {
                Py::str(s)
            } else if let Some(rest) = s.strip_prefix(['-', '+']) {
                Py::str(&format!("{}{}{}", &s[..1], "0".repeat(w - n), rest))
            } else {
                Py::str(&format!("{}{s}", "0".repeat(w - n)))
            }
        }
        "ljust" | "rjust" | "center" => {
            let w = a.first().ok_or_else(|| PyErr::Type("width".into()))?.i()? as usize;
            let fill = if a.len() > 1 { sarg(1)?.chars().next().unwrap_or(' ') } else { ' ' };
            let n = s.chars().count();
            if n >= w {
                Py::str(s)
            } else {
                let pad = w - n;
                let (l, r) = match name {
                    "ljust" => (0, pad),
                    "rjust" => (pad, 0),
                    _ => {
                        let l = pad / 2 + (pad & w & 1);
                        (l, pad - l)
                    }
                };
                Py::str(&format!("{}{s}{}", fill.to_string().repeat(l), fill.to_string().repeat(r)))
            }
        }
        _ => return Err(PyErr::Unsupported(format!("str.{name}"))),
    })
}

#[derive(Clone, Debug)]
pub enum PyErr {
    /// ValueError(message): a stated condition does not hold -> INVALID_INPUT with this reason
    Value(String),
    ZeroDivision,
    Overflow,
    Type(String),
    Key(String),
    Index,
    /// something this runtime does not model (an int beyond 64 bits, a complex result ...)
    Unsupported(String),
    /// another exception the code raises (ArithmeticError, AssertionError ...): not a refusal
    Other(String),
}

pub type R = Result<Py, PyErr>;

pub fn value_error(msg: impl Into<String>) -> PyErr {
    PyErr::Value(msg.into())
}

fn type_error(op: &str) -> PyErr {
    PyErr::Type(format!("unsupported operand for {op}"))
}

impl Py {
    pub fn str(s: &str) -> Py {
        Py::Str(Rc::from(s))
    }
    pub fn list(v: Vec<Py>) -> Py {
        Py::List(Rc::new(RefCell::new(v)))
    }
    pub fn tuple(v: Vec<Py>) -> Py {
        Py::Tuple(Rc::new(v))
    }
    pub fn dict(v: Vec<(Py, Py)>) -> Py {
        Py::Dict(Rc::new(RefCell::new(v)))
    }
    pub fn floats(v: &[f64]) -> Py {
        Py::list(v.iter().map(|x| Py::Float(*x)).collect())
    }

    /// The number as a float (Python's float(x) for int, float and bool).
    pub fn f(&self) -> Result<f64, PyErr> {
        match self {
            Py::Float(x) => Ok(*x),
            Py::Int(i) => Ok(*i as f64),
            Py::Big(n) => big_f64(n),
            Py::Bool(b) => Ok(if *b { 1.0 } else { 0.0 }),
            Py::Frac(q) => Ok(crate::pyfrac::to_f64(q)),            // float(Fraction): correctly rounded
            Py::Mpf(m) => Ok(crate::pympf::to_f64(&m.0)),            // float(mpf): the nearest double
            Py::Sym(e) => crate::sym::eval(e, &|_| None)
                .map_err(|_| PyErr::Type("Cannot convert expression to float".into())),
            _ => Err(PyErr::Type("a number is required".into())),
        }
    }
    pub fn i(&self) -> Result<i64, PyErr> {
        match self {
            Py::Int(i) => Ok(*i),
            Py::Bool(b) => Ok(*b as i64),
            Py::Big(_) => Err(big()),                         // an index or count beyond 64 bits
            _ => Err(PyErr::Type("an integer is required".into())),
        }
    }
    pub fn is_num(&self) -> bool {
        matches!(self, Py::Int(_) | Py::Float(_) | Py::Bool(_) | Py::Big(_) | Py::Mpf(_))
    }

    /// Python truthiness.
    pub fn truthy(&self) -> bool {
        match self {
            Py::None => false,
            Py::Bool(b) => *b,
            Py::Int(i) => *i != 0,
            Py::Big(_) => true,
            Py::Float(x) => *x != 0.0,
            Py::Complex(r, i) => *r != 0.0 || *i != 0.0,
            Py::Frac(q) => !num_traits::Zero::is_zero(q.as_ref()),
            Py::Str(s) => !s.is_empty(),
            Py::List(l) | Py::Set(l) => !l.borrow().is_empty(),
            Py::Tuple(t) => !t.is_empty(),
            Py::Dict(d) => !d.borrow().is_empty(),
            Py::Func(_) | Py::Re(_) | Py::Match(_) => true,
            Py::Sym(e) => !crate::sym::is_zero(e),
            Py::Poly(p) => !p.is_zero(),
            Py::Mpf(m) => !m.0.is_zero(),
        }
    }

    /// The items when iterated (a list, tuple, str, dict keys).
    pub fn items(&self) -> Result<Vec<Py>, PyErr> {
        match self {
            Py::List(l) | Py::Set(l) => Ok(l.borrow().clone()),
            Py::Tuple(t) => Ok(t.as_ref().clone()),
            Py::Str(s) => Ok(s.chars().map(|c| Py::str(&c.to_string())).collect()),
            Py::Dict(d) => Ok(d.borrow().iter().map(|(k, _)| k.clone()).collect()),
            _ => Err(PyErr::Type("object is not iterable".into())),
        }
    }

    pub fn as_str(&self) -> Result<Rc<str>, PyErr> {
        match self {
            Py::Str(s) => Ok(s.clone()),
            _ => Err(PyErr::Type("a str is required".into())),
        }
    }
}

// ------------------------------------------------------------------------------------------------ arithmetic
fn both_int(a: &Py, b: &Py) -> Option<(i64, i64)> {
    match (a, b) {
        (Py::Int(x), Py::Int(y)) => Some((*x, *y)),
        (Py::Bool(x), Py::Int(y)) => Some((*x as i64, *y)),
        (Py::Int(x), Py::Bool(y)) => Some((*x, *y as i64)),
        (Py::Bool(x), Py::Bool(y)) => Some((*x as i64, *y as i64)),
        _ => None,
    }
}

fn big() -> PyErr {
    PyErr::Unsupported("an integer beyond 64 bits".into())
}

use num_bigint::BigInt;

/// An exact integer as a value: an Int when it fits in 64 bits, else a Big.
pub fn int_of(n: BigInt) -> Py {
    match i64::try_from(&n) {
        Ok(i) => Py::Int(i),
        Err(_) => Py::Big(Rc::new(n)),
    }
}

/// An integer constant of the source beyond 64 bits (the translator writes its digits).
pub fn int_text(digits: &str) -> Py {
    int_of(digits.parse::<BigInt>().expect("the translator writes decimal digits"))
}

/// An int, bool or big int as an unbounded integer.
fn bi(a: &Py) -> Option<BigInt> {
    match a {
        Py::Int(i) => Some(BigInt::from(*i)),
        Py::Bool(b) => Some(BigInt::from(*b as i64)),
        Py::Big(n) => Some((**n).clone()),
        _ => None,
    }
}

/// Both integral with at least one beyond 64 bits (both within 64 bits is `both_int`).
fn both_big(a: &Py, b: &Py) -> Option<(BigInt, BigInt)> {
    if !matches!(a, Py::Big(_)) && !matches!(b, Py::Big(_)) {
        return None;
    }
    Some((bi(a)?, bi(b)?))
}

/// The largest integer this runtime computes (in bits): Python's ints are unbounded, but a result of millions of
/// digits is no answer; beyond it the procedure is unsupported (an internal error, never a wrong number).
const MAX_BITS: u64 = 1 << 22;

/// CPython's limit on int <-> decimal text (sys.int_info.default_max_str_digits): str(), int(str) and format()
/// raise ValueError beyond it, so a Big of more digits is never written out.
const MAX_STR_DIGITS: usize = 4300;

fn str_digits_error(n: usize) -> PyErr {
    value_error(format!("Exceeds the limit ({MAX_STR_DIGITS} digits) for integer string conversion: value has {n} \
                         digits; use sys.set_int_max_str_digits() to increase the limit"))
}

/// The digits of a big int, or CPython's ValueError (str, format, json) when it has more than MAX_STR_DIGITS,
/// decided by its size in bits before any conversion: 2^14284 < 10^4300 <= 2^14286, and 2^14289 > 10^4301.
fn big_text(n: &BigInt) -> Result<String, PyErr> {
    let too_long = || value_error(format!("Exceeds the limit ({MAX_STR_DIGITS} digits) for integer string \
                                           conversion; use sys.set_int_max_str_digits() to increase the limit"));
    match n.bits() {
        b if b <= 14_284 => Ok(n.to_string()),
        b if b >= 14_289 => Err(too_long()),
        _ => {
            let s = n.to_string();
            if s.trim_start_matches('-').len() > MAX_STR_DIGITS { Err(too_long()) } else { Ok(s) }
        }
    }
}

/// log2 of n * (n - 1) * ... * (n - k + 1): the size in bits of a falling factorial, before it is computed.
fn falling_bits(n: i64, k: i64) -> f64 {
    (0..k).map(|i| ((n - i) as f64).log2_lm()).sum()
}

fn bounded(n: BigInt) -> R {
    if n.bits() > MAX_BITS {
        return Err(big());
    }
    Ok(int_of(n))
}

/// The product lo * (lo + 1) * ... * hi by binary splitting (fast for factorials and falling factorials).
fn range_product(lo: i64, hi: i64) -> BigInt {
    if lo > hi {
        return BigInt::from(1);
    }
    if hi - lo < 16 {
        let mut r = BigInt::from(lo);
        for k in lo + 1..=hi {
            r *= k;
        }
        return r;
    }
    let mid = lo + (hi - lo) / 2;
    range_product(lo, mid) * range_product(mid + 1, hi)
}

/// float(n) for a big int: correctly rounded (num-bigint keeps a sticky bit), OverflowError beyond the float range.
fn big_f64(n: &BigInt) -> Result<f64, PyErr> {
    let x = num_traits::ToPrimitive::to_f64(n).unwrap_or(f64::INFINITY);
    if x.is_infinite() {
        return Err(PyErr::Overflow);
    }
    Ok(x)
}

/// A number as complex: an int or float meeting a complex is made complex first (Python 3.13).
pub fn cx(a: &Py) -> Option<crate::pycx::C> {
    match a {
        Py::Complex(r, i) => Some((*r, *i)),
        // complex(mpf) and complex(a sympy number): the nearest double as the real part
        Py::Int(_) | Py::Big(_) | Py::Float(_) | Py::Bool(_) | Py::Frac(_) | Py::Mpf(_) => a.f().ok().map(|x| (x, 0.0)),
        Py::Sym(_) => a.f().ok().map(|x| (x, 0.0)),
        _ => None,
    }
}

use crate::pyfrac::{self as fr, Q};

pub fn frac(q: Q) -> Py {
    Py::Frac(Rc::new(q))
}

/// An int or Fraction as an exact fraction.
fn qv(a: &Py) -> Option<Q> {
    match a {
        Py::Frac(q) => Some((**q).clone()),
        Py::Int(i) => Some(fr::int(*i)),
        Py::Big(n) => Some(Q::from_integer((**n).clone())),
        Py::Bool(b) => Some(fr::int(*b as i64)),
        _ => None,
    }
}

/// Both exact when one operand is a Fraction and the other a Fraction or an int (Fraction arithmetic).
fn qpair(a: &Py, b: &Py) -> Option<(Q, Q)> {
    if !matches!(a, Py::Frac(_)) && !matches!(b, Py::Frac(_)) {
        return None;
    }
    Some((qv(a)?, qv(b)?))
}

/// A Fraction meeting a float: both as floats (Python converts the Fraction with float()).
fn qfloat(a: &Py, b: &Py) -> Option<(Py, Py)> {
    let (fa, fb) = (matches!(a, Py::Frac(_)), matches!(b, Py::Frac(_)));
    if (fa && matches!(b, Py::Float(_))) || (fb && matches!(a, Py::Float(_))) {
        return Some((Py::Float(a.f().ok()?), Py::Float(b.f().ok()?)));
    }
    None
}

fn big_int(n: &num_bigint::BigInt) -> R {
    Ok(int_of(n.clone()))
}

/// Fraction(), Fraction(int | Fraction | float | str), Fraction(a, b) for ints and Fractions.
pub fn fraction(args: &[Py]) -> R {
    match args {
        [] => Ok(frac(fr::int(0))),
        [Py::Str(s)] => Ok(frac(fr::from_str(s)?)),
        [Py::Float(x)] => Ok(frac(fr::from_f64(*x)?)),
        [x] => qv(x).map(frac).ok_or_else(|| PyErr::Type("argument should be a string or a Rational instance".into())),
        [x, y] => {
            let (p, q) = (qv(x).ok_or_else(|| type_error("Fraction"))?, qv(y).ok_or_else(|| type_error("Fraction"))?);
            if num_traits::Zero::is_zero(&q) {
                return Err(PyErr::ZeroDivision);
            }
            Ok(frac(p / q))
        }
        _ => Err(type_error("Fraction")),
    }
}

/// x.numerator, x.denominator (an int is its own numerator, over 1)
pub fn numerator(a: &Py) -> R {
    match a {
        Py::Frac(q) => big_int(q.numer()),
        Py::Int(_) | Py::Big(_) | Py::Bool(_) => pos(a),
        _ => Err(PyErr::Type("object has no attribute 'numerator'".into())),
    }
}

pub fn denominator(a: &Py) -> R {
    match a {
        Py::Frac(q) => big_int(q.denom()),
        Py::Int(_) | Py::Big(_) | Py::Bool(_) => Ok(Py::Int(1)),
        _ => Err(PyErr::Type("object has no attribute 'denominator'".into())),
    }
}

/// Both operands as complex when at least one is complex and both are numbers.
fn cpair(a: &Py, b: &Py) -> Option<(crate::pycx::C, crate::pycx::C)> {
    if !matches!(a, Py::Complex(..)) && !matches!(b, Py::Complex(..)) {
        return None;
    }
    Some((cx(a)?, cx(b)?))
}

fn cplx(z: crate::pycx::C) -> Py {
    Py::Complex(z.0, z.1)
}

pub fn add(a: &Py, b: &Py) -> R {
    crate::meter::tick()?;
    if let Some(r) = crate::pympf::bin(a, b, "+") {
        return r;
    }
    if let Some(r) = sym_bin(a, b, "+") {
        return r;
    }
    if let Some((x, y)) = both_int(a, b) {
        return Ok(x.checked_add(y).map(Py::Int).unwrap_or_else(|| int_of(BigInt::from(x) + y)));
    }
    if let Some((x, y)) = both_big(a, b) {
        return Ok(int_of(x + y));
    }
    if let Some((x, y)) = cpair(a, b) {
        return Ok(cplx(crate::pycx::add(x, y)));
    }
    if let Some((x, y)) = qpair(a, b) {
        return Ok(frac(x + y));
    }
    if let Some((x, y)) = qfloat(a, b) {
        return add(&x, &y);
    }
    match (a, b) {
        (Py::Str(x), Py::Str(y)) => Ok(Py::str(&format!("{x}{y}"))),
        (Py::List(x), Py::List(y)) => {
            let mut v = x.borrow().clone();
            v.extend(y.borrow().iter().cloned());
            Ok(Py::list(v))
        }
        (Py::Tuple(x), Py::Tuple(y)) => {
            let mut v = x.as_ref().clone();
            v.extend(y.iter().cloned());
            Ok(Py::tuple(v))
        }
        _ if a.is_num() && b.is_num() => Ok(Py::Float(a.f()? + b.f()?)),
        _ => Err(type_error("+")),
    }
}

/// a - b, a | b, a & b, a ^ b of two sets (lists of distinct items here: py::set; Python raises TypeError for these
/// operators on lists, so a translated program only meets them with sets), in first-seen order; d1 | d2 merges dicts
/// A Python number or a symbolic value as an exact expression (an int, Fraction or bool exactly; a float by its exact
/// binary value, the double sympy's Float holds), None for anything else.
pub fn to_ex(p: &Py) -> Option<crate::sym::Ex> {
    use crate::sym;
    Some(match p {
        Py::Sym(e) => (**e).clone(),
        Py::Int(i) => sym::int(*i),
        Py::Bool(b) => sym::int(*b as i64),
        Py::Big(n) => sym::int_big((**n).clone()),
        Py::Frac(q) => sym::num((**q).clone()),
        Py::Float(x) => sym::num(fr::from_f64(*x).ok()?),
        _ => return None,
    })
}

/// a op b when either is symbolic: the expression sympy builds
fn sym_bin(a: &Py, b: &Py, op: &str) -> Option<R> {
    use crate::sym;
    if matches!(a, Py::Poly(_)) || matches!(b, Py::Poly(_)) {
        return Some(crate::pysym::poly_bin(a, b, op));
    }
    if !matches!(a, Py::Sym(_)) && !matches!(b, Py::Sym(_)) {
        return None;
    }
    let (x, y) = match (to_ex(a), to_ex(b)) {
        (Some(x), Some(y)) => (x, y),
        _ => return Some(Err(type_error(op))),
    };
    if op == "/" && sym::is_zero(&y) {
        return Some(Ok(Py::Sym(Rc::new(sym::Ex::Zoo))));      // sympy: x/0 is zoo
    }
    Some(Ok(Py::Sym(Rc::new(match op {
        "+" => sym::add(x, y),
        "-" => sym::sub(x, y),
        "*" => sym::mul(x, y),
        "/" => sym::div(x, y),
        _ => sym::pow(x, y),
    }))))
}

fn set_op(a: &Py, b: &Py, op: &str) -> Option<R> {
    if let (Py::Dict(_), Py::Dict(_), "|") = (a, b, op) {
        let d = Py::dict(vec![]);
        return Some(kw_merge(&d, a).and_then(|_| kw_merge(&d, b)).map(|_| d));
    }
    // two sets (a Py::Set; a list stands for a set only in code translated before sets had their own variant)
    let (x, y) = match (a, b) {
        (Py::List(x) | Py::Set(x), Py::List(y) | Py::Set(y)) => (x.borrow().clone(), y.borrow().clone()),
        _ => return None,
    };
    let has = |v: &[Py], k: &Py| v.iter().any(|z| eq(z, k));
    let out: Vec<Py> = match op {
        "-" => x.iter().filter(|k| !has(&y, k)).cloned().collect(),
        "&" => x.iter().filter(|k| has(&y, k)).cloned().collect(),
        "|" => x.iter().cloned().chain(y.iter().filter(|k| !has(&x, k)).cloned()).collect(),
        _ => x.iter().filter(|k| !has(&y, k)).cloned().chain(y.iter().filter(|k| !has(&x, k)).cloned()).collect(),
    };
    Some(set(&Py::list(out)))
}

pub fn sub(a: &Py, b: &Py) -> R {
    crate::meter::tick()?;
    if let Some(r) = crate::pympf::bin(a, b, "-") {
        return r;
    }
    if let Some(r) = sym_bin(a, b, "-") {
        return r;
    }
    if let Some(r) = set_op(a, b, "-") {
        return r;
    }
    if let Some((x, y)) = both_int(a, b) {
        return Ok(x.checked_sub(y).map(Py::Int).unwrap_or_else(|| int_of(BigInt::from(x) - y)));
    }
    if let Some((x, y)) = both_big(a, b) {
        return Ok(int_of(x - y));
    }
    if let Some((x, y)) = cpair(a, b) {
        return Ok(cplx(crate::pycx::sub(x, y)));
    }
    if let Some((x, y)) = qpair(a, b) {
        return Ok(frac(x - y));
    }
    if let Some((x, y)) = qfloat(a, b) {
        return sub(&x, &y);
    }
    if a.is_num() && b.is_num() {
        return Ok(Py::Float(a.f()? - b.f()?));
    }
    Err(type_error("-"))
}

pub fn mul(a: &Py, b: &Py) -> R {
    crate::meter::tick()?;
    if let Some(r) = crate::pympf::bin(a, b, "*") {
        return r;
    }
    if let Some(r) = sym_bin(a, b, "*") {
        return r;
    }
    if let Some((x, y)) = both_int(a, b) {
        return Ok(x.checked_mul(y).map(Py::Int).unwrap_or_else(|| int_of(BigInt::from(x) * y)));
    }
    if let Some((x, y)) = both_big(a, b) {
        return bounded(x * y);
    }
    if let Some((x, y)) = cpair(a, b) {
        return Ok(cplx(crate::pycx::mul(x, y)));
    }
    if let Some((x, y)) = qpair(a, b) {
        return Ok(frac(x * y));
    }
    if let Some((x, y)) = qfloat(a, b) {
        return mul(&x, &y);
    }
    match (a, b) {
        (Py::Str(s), n) | (n, Py::Str(s)) if matches!(n, Py::Int(_)) => {
            Ok(Py::str(&s.repeat(n.i()?.max(0) as usize)))
        }
        (Py::List(l), n) | (n, Py::List(l)) if matches!(n, Py::Int(_)) => {
            let k = n.i()?.max(0) as usize;
            let base = l.borrow().clone();
            let mut v = Vec::with_capacity(base.len() * k);
            for _ in 0..k {
                v.extend(base.iter().cloned());
            }
            Ok(Py::list(v))
        }
        _ if a.is_num() && b.is_num() => Ok(Py::Float(a.f()? * b.f()?)),
        _ => Err(type_error("*")),
    }
}

pub fn truediv(a: &Py, b: &Py) -> R {
    crate::meter::tick()?;
    if let Some(r) = crate::pympf::bin(a, b, "/") {
        return r;
    }
    if let Some(r) = sym_bin(a, b, "/") {
        return r;
    }
    if let Some((x, y)) = cpair(a, b) {
        return Ok(cplx(crate::pycx::quot(x, y)?));
    }
    if let Some((x, y)) = qpair(a, b) {
        if num_traits::Zero::is_zero(&y) {
            return Err(PyErr::ZeroDivision);
        }
        return Ok(frac(x / y));
    }
    if let Some((x, y)) = qfloat(a, b) {
        return truediv(&x, &y);
    }
    if let Some((x, y)) = both_big(a, b) {
        // int / int is the correctly rounded quotient, even when an operand is beyond the float range
        if num_traits::Zero::is_zero(&y) {
            return Err(PyErr::ZeroDivision);
        }
        let x = fr::to_f64(&Q::new(x, y));
        if x.is_infinite() {
            return Err(PyErr::Overflow);
        }
        return Ok(Py::Float(x));
    }
    if !(a.is_num() && b.is_num()) {
        return Err(type_error("/"));
    }
    let y = b.f()?;
    if y == 0.0 {
        return Err(PyErr::ZeroDivision);
    }
    Ok(Py::Float(a.f()? / y))
}

/// Python's float divmod (Objects/floatobject.c float_divmod): (floor quotient, modulo with the sign of b).
pub fn fdivmod(vx: f64, wx: f64) -> Result<(f64, f64), PyErr> {
    if wx == 0.0 {
        return Err(PyErr::ZeroDivision);
    }
    let mut m = vx % wx;
    let mut div = (vx - m) / wx;
    if m != 0.0 {
        if (wx < 0.0) != (m < 0.0) {
            m += wx;
            div -= 1.0;
        }
    } else {
        m = 0.0f64.copysign(wx);
    }
    let floordiv = if div != 0.0 {
        let mut f = div.floor();
        if div - f > 0.5 {
            f += 1.0;
        }
        f
    } else {
        0.0f64.copysign(vx / wx)
    };
    Ok((floordiv, m))
}

pub fn floordiv(a: &Py, b: &Py) -> R {
    if let Some((x, y)) = both_int(a, b) {
        if y == 0 {
            return Err(PyErr::ZeroDivision);
        }
        return Ok(Py::Int(crate::pyfmt::ifloordiv(x, y)));
    }
    if let Some((x, y)) = both_big(a, b) {
        if num_traits::Zero::is_zero(&y) {
            return Err(PyErr::ZeroDivision);
        }
        return Ok(int_of(num_integer::Integer::div_floor(&x, &y)));
    }
    if let Some((x, y)) = qpair(a, b) {
        if num_traits::Zero::is_zero(&y) {
            return Err(PyErr::ZeroDivision);
        }
        return big_int(&fr::floor(&(x / y)));               // Fraction // Fraction is an int
    }
    if let Some((x, y)) = qfloat(a, b) {
        return floordiv(&x, &y);
    }
    if a.is_num() && b.is_num() {
        return Ok(Py::Float(fdivmod(a.f()?, b.f()?)?.0));
    }
    Err(type_error("//"))
}

pub fn modulo(a: &Py, b: &Py) -> R {
    if let Some((x, y)) = both_int(a, b) {
        if y == 0 {
            return Err(PyErr::ZeroDivision);
        }
        return Ok(Py::Int(crate::pyfmt::imod(x, y)));
    }
    if let Some((x, y)) = both_big(a, b) {
        if num_traits::Zero::is_zero(&y) {
            return Err(PyErr::ZeroDivision);
        }
        return Ok(int_of(num_integer::Integer::mod_floor(&x, &y)));
    }
    if let Some((x, y)) = qpair(a, b) {
        if num_traits::Zero::is_zero(&y) {
            return Err(PyErr::ZeroDivision);
        }
        let k = Q::from_integer(fr::floor(&(&x / &y)));
        return Ok(frac(x - y * k));                          // the sign of the divisor, as Python
    }
    if let Some((x, y)) = qfloat(a, b) {
        return modulo(&x, &y);
    }
    if a.is_num() && b.is_num() {
        return Ok(Py::Float(fdivmod(a.f()?, b.f()?)?.1));
    }
    Err(type_error("%"))
}

pub fn pow(a: &Py, b: &Py) -> R {
    if let Some(r) = crate::pympf::bin(a, b, "**") {
        return r;
    }
    if let Some(r) = sym_bin(a, b, "**") {
        return r;
    }
    if let Some((x, y)) = cpair(a, b) {
        return Ok(cplx(crate::pycx::pow(x, y)?));
    }
    if matches!(a, Py::Frac(_)) || matches!(b, Py::Frac(_)) {
        // Fraction.__pow__ / __rpow__: an integral exponent stays exact (an int base to a non-negative one stays an
        // int); anything else is float(a) ** float(b)
        let n = match b {
            Py::Int(i) => Some(*i),
            Py::Bool(t) => Some(*t as i64),
            Py::Frac(q) if q.denom() == &num_bigint::BigInt::from(1) => Some(fr::small(q.numer())?),
            _ => None,
        };
        return match (a, n) {
            (Py::Frac(q), Some(n)) => Ok(frac(fr::powi(q, n)?)),
            (Py::Int(_) | Py::Big(_) | Py::Bool(_), Some(n)) if n >= 0 => pow(a, &Py::Int(n)),
            (Py::Int(_) | Py::Big(_) | Py::Bool(_), Some(n)) => Ok(frac(fr::powi(&qv(a).unwrap(), n)?)),
            _ if a.is_num() || matches!(a, Py::Frac(_)) => fpow(a.f()?, b.f()?),
            _ => Err(type_error("**")),
        };
    }
    if let Some((x, y)) = both_int(a, b) {
        if y >= 0 {
            return match x {
                0 => Ok(Py::Int(if y == 0 { 1 } else { 0 })),
                1 => Ok(Py::Int(1)),
                -1 => Ok(Py::Int(if y % 2 == 0 { 1 } else { -1 })),
                _ => {
                    let e = u32::try_from(y).map_err(|_| big())?;
                    match x.checked_pow(e) {
                        Some(r) => Ok(Py::Int(r)),
                        None => big_pow(BigInt::from(x), y),
                    }
                }
            };
        }
        if x == 0 {
            return Err(PyErr::ZeroDivision);
        }
        return fpow(x as f64, y as f64);
    }
    if let Some((x, y)) = both_big(a, b) {
        if num_traits::Signed::is_negative(&y) {
            return fpow(a.f()?, b.f()?);                     // a negative exponent: a float, as Python
        }
        let one = BigInt::from(1);
        if x == one || num_traits::Zero::is_zero(&x) {
            return Ok(int_of(x));
        }
        if x == -one {
            return Ok(Py::Int(if num_integer::Integer::is_even(&y) { 1 } else { -1 }));
        }
        let y = num_traits::ToPrimitive::to_i64(&y).ok_or_else(big)?;
        return big_pow(x, y);
    }
    if a.is_num() && b.is_num() {
        return fpow(a.f()?, b.f()?);
    }
    Err(type_error("**"))
}

/// x ** y for ints with y >= 0 and |x| >= 2, exactly, within MAX_BITS.
fn big_pow(x: BigInt, y: i64) -> R {
    if (x.bits() as u128 - 1) * (y as u128) > MAX_BITS as u128 {
        return Err(big());
    }
    Ok(int_of(num_traits::Pow::pow(x, y as u64)))
}

/// float ** float as Python computes it (Objects/floatobject.c float_pow).
pub fn fpow(x: f64, y: f64) -> R {
    if y == 0.0 {
        return Ok(Py::Float(1.0));
    }
    if x.is_nan() {
        return Ok(Py::Float(if y == 0.0 { 1.0 } else { x }));
    }
    if y.is_nan() {
        return Ok(Py::Float(if x == 1.0 { 1.0 } else { y }));
    }
    if x == 0.0 {
        if y < 0.0 && y.is_finite() {
            return Err(PyErr::ZeroDivision);
        }
    }
    if x < 0.0 && y.is_finite() && y != y.floor() {
        // Python 3: a negative float to a fractional power is complex ((-8.0) ** (1/3) = 1+1.732j)
        return Ok(cplx(crate::pycx::pow((x, 0.0), (y, 0.0))?));
    }
    let r = x.powf_lm(y);
    if r.is_infinite() && x.is_finite() && y.is_finite() {
        return Err(PyErr::Overflow);
    }
    Ok(Py::Float(r))
}

pub fn neg(a: &Py) -> R {
    match a {
        Py::Int(i) => Ok(i.checked_neg().map(Py::Int).unwrap_or_else(|| int_of(-BigInt::from(*i)))),
        Py::Big(n) => Ok(int_of(-(**n).clone())),
        Py::Bool(b) => Ok(Py::Int(-(*b as i64))),
        Py::Float(x) => Ok(Py::Float(-x)),
        Py::Complex(r, i) => Ok(Py::Complex(-r, -i)),
        Py::Frac(q) => Ok(frac(-(**q).clone())),
        Py::Sym(e) => Ok(Py::Sym(Rc::new(crate::sym::neg((**e).clone())))),
        Py::Poly(p) => Ok(Py::Poly(Rc::new(p.neg()))),
        Py::Mpf(m) => Ok(crate::pympf::neg(m)),
        _ => Err(type_error("unary -")),
    }
}

pub fn pos(a: &Py) -> R {
    match a {
        Py::Bool(b) => Ok(Py::Int(*b as i64)),
        Py::Int(_) | Py::Big(_) | Py::Float(_) | Py::Complex(..) | Py::Frac(_) => Ok(a.clone()),
        _ => Err(type_error("unary +")),
    }
}

pub fn not(a: &Py) -> Py {
    Py::Bool(!a.truthy())
}

// ------------------------------------------------------------------------------------------------ comparison
pub fn eq(a: &Py, b: &Py) -> bool {
    match (a, b) {
        (Py::None, Py::None) => true,
        (Py::Str(x), Py::Str(y)) => x == y,
        (Py::List(x), Py::List(y)) => {
            let (x, y) = (x.borrow(), y.borrow());
            x.len() == y.len() && x.iter().zip(y.iter()).all(|(p, q)| eq(p, q))
        }
        (Py::Tuple(x), Py::Tuple(y)) => x.len() == y.len() && x.iter().zip(y.iter()).all(|(p, q)| eq(p, q)),
        // two sets: the same items in any order (each holds distinct items)
        (Py::Set(x), Py::Set(y)) => {
            let (x, y) = (x.borrow(), y.borrow());
            x.len() == y.len() && x.iter().all(|p| y.iter().any(|q| eq(p, q)))
        }
        (Py::Poly(x), Py::Poly(y)) => x == y,
        (Py::Mpf(_), _) | (_, Py::Mpf(_)) => crate::pympf::cmp(a, b) == Some(std::cmp::Ordering::Equal),
        (Py::Poly(_), _) | (_, Py::Poly(_)) => false,
        // sympy's ==: the same expression (x == 0 is False; 2 == Integer(2) is True)
        (Py::Sym(_), _) | (_, Py::Sym(_)) => match (to_ex(a), to_ex(b)) {
            (Some(x), Some(y)) => x == y,
            _ => false,
        },
        // dicts are equal with the same keys and equal values, in any order (as Python's)
        (Py::Dict(x), Py::Dict(y)) => {
            let (x, y) = (x.borrow(), y.borrow());
            x.len() == y.len() && x.iter().all(|(k, v)| y.iter().any(|(k2, v2)| eq(k, k2) && eq(v, v2)))
        }
        _ if cpair(a, b).is_some() => {
            let (x, y) = cpair(a, b).unwrap();
            x.0 == y.0 && x.1 == y.1
        }
        _ if exact_kind(a) || exact_kind(b) => {
            frac_order(a, b).ok().flatten() == Some(std::cmp::Ordering::Equal)
        }
        _ if a.is_num() && b.is_num() => {
            if let Some((x, y)) = both_int(a, b) {
                x == y
            } else {
                a.f().unwrap_or(f64::NAN) == b.f().unwrap_or(f64::NAN)
            }
        }
        _ => false,
    }
}

pub fn ne(a: &Py, b: &Py) -> bool {
    !eq(a, b)
}

/// A Fraction or a big int: compared with numbers exactly (frac_order), never through a float.
fn exact_kind(a: &Py) -> bool {
    matches!(a, Py::Frac(_) | Py::Big(_))
}

/// A Fraction against an int, a Fraction or a float, exactly (Python compares a finite float by its exact value);
/// Ok(None) for a nan (never equal, unordered); Err when the other is not a real number.
fn frac_order(a: &Py, b: &Py) -> Result<Option<std::cmp::Ordering>, PyErr> {
    let exact = |p: &Py| -> Result<Option<Q>, PyErr> {
        match p {
            Py::Float(x) if x.is_nan() => Ok(None),
            Py::Float(x) if x.is_infinite() => Ok(None),
            Py::Float(x) => Ok(Some(fr::from_f64(*x)?)),
            _ => qv(p).map(Some).ok_or_else(|| type_error("<")),
        }
    };
    // an infinity is beyond every fraction
    for (p, flip) in [(b, false), (a, true)] {
        if let Py::Float(x) = p {
            if x.is_infinite() {
                let o = if *x > 0.0 { std::cmp::Ordering::Less } else { std::cmp::Ordering::Greater };
                return Ok(Some(if flip { o.reverse() } else { o }));
            }
        }
    }
    match (exact(a)?, exact(b)?) {
        (Some(x), Some(y)) => Ok(Some(x.cmp(&y))),
        _ => Ok(None),
    }
}

/// Python's ordering: None when unorderable (comparisons raise TypeError then).
fn order(a: &Py, b: &Py) -> Result<std::cmp::Ordering, PyErr> {
    use std::cmp::Ordering::*;
    if let Some((x, y)) = both_int(a, b) {
        return Ok(x.cmp(&y));
    }
    if matches!(a, Py::Mpf(_)) || matches!(b, Py::Mpf(_)) {
        return crate::pympf::cmp(a, b).ok_or(PyErr::Unsupported("nan or not a number".into()));
    }
    if matches!(a, Py::Sym(_)) || matches!(b, Py::Sym(_)) {
        // a symbolic number against a number: by value (sympy decides an inequality of numbers); a variable cannot be
        // ordered ("cannot determine truth value of Relational", a TypeError)
        let (x, y) = (a.f()?, b.f()?);
        return x.partial_cmp(&y).ok_or(PyErr::Unsupported("nan".into()));
    }
    if exact_kind(a) || exact_kind(b) {
        return frac_order(a, b)?.ok_or(PyErr::Unsupported("nan".into()));
    }
    if a.is_num() && b.is_num() {
        let (x, y) = (a.f()?, b.f()?);
        return x.partial_cmp(&y).ok_or(PyErr::Unsupported("nan".into()));
    }
    match (a, b) {
        (Py::Str(x), Py::Str(y)) => Ok(x.cmp(y)),
        (Py::List(_), Py::List(_)) | (Py::Tuple(_), Py::Tuple(_)) => {
            let (x, y) = (a.items()?, b.items()?);
            for (p, q) in x.iter().zip(y.iter()) {
                if !eq(p, q) {
                    return order(p, q);
                }
            }
            Ok(x.len().cmp(&y.len()))
        }
        _ => {
            let _ = (Less, Greater, Equal);
            Err(PyErr::Type("'<' not supported".into()))
        }
    }
}

/// a <= b of two sets: every item of a is in b (Python's subset test; a set is not totally ordered, so it never goes
/// through `order`)
fn subset(a: &Py, b: &Py) -> Option<bool> {
    let (Py::Set(x), Py::Set(y)) = (a, b) else { return None };
    let (x, y) = (x.borrow(), y.borrow());
    Some(x.iter().all(|p| y.iter().any(|q| eq(p, q))))
}

fn set_order(a: &Py, b: &Py, op: &str) -> Option<Result<bool, PyErr>> {
    match (a, b) {
        (Py::Set(_), Py::Set(_)) => {}
        (Py::Set(_), _) | (_, Py::Set(_)) => return Some(Err(type_error(op))),
        _ => return None,
    }
    let ab = subset(a, b)?;
    let ba = subset(b, a)?;
    Some(Ok(match op {
        "<" => ab && !ba,
        "<=" => ab,
        ">" => ba && !ab,
        _ => ba,
    }))
}

/// a < b etc. NaN compares false as in Python.
pub fn lt(a: &Py, b: &Py) -> Result<bool, PyErr> {
    crate::meter::tick()?;
    if let Some(r) = set_order(a, b, "<") {
        return r;
    }
    if matches!(a, Py::Mpf(_)) || matches!(b, Py::Mpf(_)) {
        return Ok(crate::pympf::cmp(a, b) == Some(std::cmp::Ordering::Less));
    }
    if a.is_num() && b.is_num() && both_int(a, b).is_none() && !exact_kind(a) && !exact_kind(b) {
        return Ok(a.f()? < b.f()?);
    }
    Ok(order(a, b)? == std::cmp::Ordering::Less)
}
pub fn le(a: &Py, b: &Py) -> Result<bool, PyErr> {
    crate::meter::tick()?;
    if let Some(r) = set_order(a, b, "<=") {
        return r;
    }
    if a.is_num() && b.is_num() && both_int(a, b).is_none() && !exact_kind(a) && !exact_kind(b) {
        return Ok(a.f()? <= b.f()?);
    }
    Ok(order(a, b)? != std::cmp::Ordering::Greater)
}
pub fn gt(a: &Py, b: &Py) -> Result<bool, PyErr> {
    lt(b, a)
}
pub fn ge(a: &Py, b: &Py) -> Result<bool, PyErr> {
    le(b, a)
}

pub fn contains(container: &Py, x: &Py) -> Result<bool, PyErr> {
    match container {
        Py::Str(s) => Ok(s.contains(x.as_str()?.as_ref())),
        Py::Dict(d) => Ok(d.borrow().iter().any(|(k, _)| eq(k, x))),
        _ => Ok(container.items()?.iter().any(|y| eq(y, x))),
    }
}

// ------------------------------------------------------------------------------------------------ indexing
fn norm_index(i: i64, n: usize) -> Result<usize, PyErr> {
    let j = if i < 0 { i + n as i64 } else { i };
    if j < 0 || j >= n as i64 {
        return Err(PyErr::Index);
    }
    Ok(j as usize)
}

pub fn getitem(a: &Py, k: &Py) -> R {
    match a {
        Py::List(l) => {
            let l = l.borrow();
            Ok(l[norm_index(k.i()?, l.len())?].clone())
        }
        Py::Tuple(t) => Ok(t[norm_index(k.i()?, t.len())?].clone()),
        Py::Str(s) => {
            let cs: Vec<char> = s.chars().collect();
            Ok(Py::str(&cs[norm_index(k.i()?, cs.len())?].to_string()))
        }
        Py::Dict(d) => d.borrow().iter().find(|(kk, _)| eq(kk, k)).map(|(_, v)| v.clone())
            .ok_or_else(|| PyErr::Key(repr(k))),
        Py::Match(m) => m.group(k),                       // m[1] is m.group(1)
        _ => Err(PyErr::Type("object is not subscriptable".into())),
    }
}

pub fn setitem(a: &Py, k: &Py, v: Py) -> Result<(), PyErr> {
    match a {
        Py::List(l) => {
            let n = l.borrow().len();
            let j = norm_index(k.i()?, n)?;
            l.borrow_mut()[j] = v;
            Ok(())
        }
        Py::Dict(d) => {
            let mut d = d.borrow_mut();
            if let Some(slot) = d.iter_mut().find(|(kk, _)| eq(kk, k)) {
                slot.1 = v;
            } else {
                d.push((k.clone(), v));
            }
            Ok(())
        }
        _ => Err(PyErr::Type("object does not support item assignment".into())),
    }
}

/// del a[k]: a list's item (IndexError when out of range) or a dict's key (KeyError when absent); a dict keeps the
/// insertion order of the others, as Python's
pub fn delitem(a: &Py, k: &Py) -> Result<(), PyErr> {
    match a {
        Py::List(l) => {
            let n = l.borrow().len();
            let j = norm_index(k.i()?, n)?;
            l.borrow_mut().remove(j);
            Ok(())
        }
        Py::Dict(d) => {
            let pos = d.borrow().iter().position(|(kk, _)| eq(kk, k)).ok_or_else(|| PyErr::Key(repr(k)))?;
            d.borrow_mut().remove(pos);
            Ok(())
        }
        _ => Err(PyErr::Type("object doesn't support item deletion".into())),
    }
}

/// ~x for an int (or bool): -x - 1
pub fn invert(a: &Py) -> R {
    match a {
        Py::Int(_) | Py::Big(_) | Py::Bool(_) => sub(&neg(a)?, &Py::Int(1)),
        _ => Err(type_error("unary ~")),
    }
}

/// math.gcd(*ints) / math.lcm(*ints) (Python 3.9+): gcd() is 0, lcm() is 1, results are non-negative
pub fn gcd_lcm(name: &str, a: &[Py]) -> R {
    use num_integer::Integer;
    let mut acc = BigInt::from(if name == "gcd" { 0 } else { 1 });
    for x in a {
        let n = bi(x).ok_or_else(|| PyErr::Type(format!("'{}' object cannot be interpreted as an integer",
                                                          type_name(x))))?;
        acc = if name == "gcd" {
            acc.gcd(&n)
        } else if num_traits::Zero::is_zero(&acc) || num_traits::Zero::is_zero(&n) {
            BigInt::from(0)
        } else {
            acc.lcm(&n)
        };
        if acc.bits() > MAX_BITS {
            return Err(big());
        }
    }
    Ok(int_of(acc))
}

pub fn type_name(a: &Py) -> &'static str {
    match a {
        Py::None => "NoneType",
        Py::Bool(_) => "bool",
        Py::Int(_) | Py::Big(_) => "int",
        Py::Float(_) => "float",
        Py::Complex(..) => "complex",
        Py::Frac(_) => "Fraction",
        Py::Str(_) => "str",
        Py::List(_) => "list",
        Py::Set(_) => "set",
        Py::Tuple(_) => "tuple",
        Py::Dict(_) => "dict",
        Py::Func(_) => "function",
        Py::Re(_) => "re.Pattern",
        Py::Match(_) => "re.Match",
        Py::Sym(_) => "Expr",
        Py::Poly(_) => "Poly",
        Py::Mpf(_) => "mpf",
    }
}

/// a[lo:hi:step]
pub fn slice(a: &Py, lo: Option<i64>, hi: Option<i64>, step: Option<i64>) -> R {
    let items = a.items()?;
    let n = items.len() as i64;
    let step = step.unwrap_or(1);
    if step == 0 {
        return Err(value_error("slice step cannot be zero"));
    }
    let clamp = |x: i64, lo_: i64, hi_: i64| x.max(lo_).min(hi_);
    let adj = |x: i64| if x < 0 { x + n } else { x };
    let mut out = vec![];
    if step > 0 {
        let s = clamp(lo.map(adj).unwrap_or(0), 0, n);
        let e = clamp(hi.map(adj).unwrap_or(n), 0, n);
        let mut i = s;
        while i < e {
            out.push(items[i as usize].clone());
            i += step;
        }
    } else {
        let s = clamp(lo.map(adj).unwrap_or(n - 1), -1, n - 1);
        let e = clamp(hi.map(adj).unwrap_or(-1), -1, n - 1);
        let mut i = s;
        while i > e {
            out.push(items[i as usize].clone());
            i += step;
        }
    }
    Ok(match a {
        Py::Str(_) => Py::str(&out.iter().map(|c| c.as_str().map(|s| s.to_string()).unwrap_or_default())
            .collect::<String>()),
        Py::Tuple(_) => Py::tuple(out),
        _ => Py::list(out),
    })
}

/// a[sl] where `sl = slice(lo, hi, step)` was bound in the function: the translator holds the slice object as the
/// tuple (lo, hi, step), each None when not given
pub fn slice_by(a: &Py, sl: &Py) -> R {
    let parts = sl.items()?;
    if parts.len() != 3 {
        return Err(PyErr::Type("slice indices".into()));
    }
    let b = |x: &Py| -> Result<Option<i64>, PyErr> { if matches!(x, Py::None) { Ok(None) } else { Ok(Some(x.i()?)) } };
    slice(a, b(&parts[0])?, b(&parts[1])?, b(&parts[2])?)
}

/// a[lo:hi] = v for a list (no step): the items in lo..hi (clamped as Python's slice) replaced by those of v, which
/// may be of another length
pub fn setslice(a: &Py, lo: Option<i64>, hi: Option<i64>, v: &Py) -> Result<(), PyErr> {
    let new = v.items()?;
    match a {
        Py::List(l) => {
            let n = l.borrow().len() as i64;
            let adj = |x: i64| if x < 0 { (x + n).max(0) } else { x.min(n) };
            let s = lo.map(adj).unwrap_or(0);
            let e = hi.map(adj).unwrap_or(n).max(s);
            l.borrow_mut().splice(s as usize..e as usize, new);
            Ok(())
        }
        _ => Err(PyErr::Type("object does not support slice assignment".into())),
    }
}

/// collections.Counter(iterable): each distinct item and how many times it occurs, in first-seen order (a dict)
pub fn counter(it: &Py) -> R {
    if matches!(it, Py::Dict(_)) {
        return Err(PyErr::Unsupported("Counter of a mapping".into()));
    }
    let mut out: Vec<(Py, Py)> = vec![];
    for x in it.items()? {
        if let Some(slot) = out.iter_mut().find(|(k, _)| eq(k, &x)) {
            slot.1 = Py::Int(slot.1.i()? + 1);
        } else {
            out.push((x, Py::Int(1)));
        }
    }
    Ok(Py::dict(out))
}

/// random.Random(seed).shuffle(x) for an int seed: CPython's Mersenne Twister (init_by_array on the 32-bit words of
/// |seed|), _randbelow by getrandbits and rejection, and shuffle's swap order, so the list ends in Python's order
pub fn random_shuffle(seed: &Py, x: &Py) -> Result<(), PyErr> {
    let l = match x {
        Py::List(l) => l,
        _ => return Err(PyErr::Type("shuffle needs a list".into())),
    };
    let mut s = match seed {
        Py::Int(i) => i.unsigned_abs(),
        Py::Bool(b) => *b as u64,
        _ => return Err(PyErr::Unsupported("Random seed other than an int".into())),
    };
    let mut key: Vec<u32> = vec![];
    while s > 0 {
        key.push(s as u32);
        s >>= 32;
    }
    if key.is_empty() {
        key.push(0);
    }
    let mut mt = Mt::new(&key);
    let n = l.borrow().len();
    let mut v = l.borrow_mut();
    for i in (1..n).rev() {
        let j = mt.below((i + 1) as u64)? as usize;
        v.swap(i, j);
    }
    Ok(())
}

struct Mt {
    st: [u32; 624],
    i: usize,
}

impl Mt {
    fn new(key: &[u32]) -> Mt {
        let mut st = [0u32; 624];
        st[0] = 19650218;
        for k in 1..624 {
            st[k] = 1812433253u32.wrapping_mul(st[k - 1] ^ (st[k - 1] >> 30)).wrapping_add(k as u32);
        }
        let (mut i, mut j) = (1usize, 0usize);
        for _ in 0..624.max(key.len()) {
            st[i] = (st[i] ^ (st[i - 1] ^ (st[i - 1] >> 30)).wrapping_mul(1664525)).wrapping_add(key[j]).wrapping_add(j as u32);
            i += 1;
            j += 1;
            if i >= 624 {
                st[0] = st[623];
                i = 1;
            }
            if j >= key.len() {
                j = 0;
            }
        }
        for _ in 0..623 {
            st[i] = (st[i] ^ (st[i - 1] ^ (st[i - 1] >> 30)).wrapping_mul(1566083941)).wrapping_sub(i as u32);
            i += 1;
            if i >= 624 {
                st[0] = st[623];
                i = 1;
            }
        }
        st[0] = 0x8000_0000;
        Mt { st, i: 624 }
    }

    fn next_u32(&mut self) -> u32 {
        if self.i >= 624 {
            for k in 0..624 {
                let y = (self.st[k] & 0x8000_0000) | (self.st[(k + 1) % 624] & 0x7fff_ffff);
                let mut v = self.st[(k + 397) % 624] ^ (y >> 1);
                if y & 1 != 0 {
                    v ^= 0x9908_b0df;
                }
                self.st[k] = v;
            }
            self.i = 0;
        }
        let mut y = self.st[self.i];
        self.i += 1;
        y ^= y >> 11;
        y ^= (y << 7) & 0x9d2c_5680;
        y ^= (y << 15) & 0xefc6_0000;
        y ^ (y >> 18)
    }

    /// Random._randbelow_with_getrandbits(n)
    fn below(&mut self, n: u64) -> Result<u64, PyErr> {
        let k = 64 - n.leading_zeros();
        if k > 32 {
            return Err(PyErr::Unsupported("shuffle of more than 2^32 items".into()));
        }
        loop {
            let r = (self.next_u32() >> (32 - k)) as u64;
            if r < n {
                return Ok(r);
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------ text
pub fn repr(a: &Py) -> String {
    match a {
        Py::None => "None".into(),
        Py::Bool(b) => if *b { "True".into() } else { "False".into() },
        Py::Int(i) => i.to_string(),
        Py::Big(n) => big_text(n).unwrap_or_else(|_| format!("<int of {} bits>", n.bits())),
        Py::Float(x) => crate::num::py_repr_f64(*x),
        Py::Complex(r, i) => crate::pycx::repr((*r, *i)),
        Py::Frac(q) => fr::repr(q),
        Py::Str(s) => crate::num::py_repr_str(s),
        Py::List(l) => format!("[{}]", l.borrow().iter().map(repr).collect::<Vec<_>>().join(", ")),
        // (in first-seen order: Python prints a set in its hash order, which this runtime does not model)
        Py::Set(l) if l.borrow().is_empty() => "set()".into(),
        Py::Set(l) => format!("{{{}}}", l.borrow().iter().map(repr).collect::<Vec<_>>().join(", ")),
        Py::Tuple(t) => {
            if t.len() == 1 {
                format!("({},)", repr(&t[0]))
            } else {
                format!("({})", t.iter().map(repr).collect::<Vec<_>>().join(", "))
            }
        }
        Py::Dict(d) => format!("{{{}}}", d.borrow().iter().map(|(k, v)| format!("{}: {}", repr(k), repr(v)))
            .collect::<Vec<_>>().join(", ")),
        Py::Func(_) => "<function>".into(),
        Py::Sym(e) => crate::sym::sstr(e),
        Py::Poly(p) => format!("Poly({}, {}, domain='QQ')", crate::sym::sstr(&p.as_ex()), p.gen),
        Py::Mpf(m) => crate::pympf::repr(m),
        Py::Re(r) => format!("re.compile({})", crate::num::py_repr_str(&r.pattern)),
        Py::Match(m) => {
            let (a, b) = m.spans[0].unwrap_or((0, 0));
            let c = |x: usize| m.text[..x].chars().count();
            format!("<re.Match object; span=({}, {}), match={}>", c(a), c(b), crate::num::py_repr_str(&m.text[a..b]))
        }
    }
}

/// str.maketrans(x[, y[, z]]): a dict from code points to code points, strings or None (deletion)
pub fn str_maketrans(a: &[Py]) -> R {
    let ord = |c: char| Py::Int(c as i64);
    match a {
        [Py::Dict(d)] => Ok(Py::dict(d.borrow().iter().map(|(k, v)| {
            let k = match k {
                Py::Str(s) if s.chars().count() == 1 => ord(s.chars().next().unwrap()),
                other => other.clone(),
            };
            (k, v.clone())
        }).collect())),
        [Py::Str(x), Py::Str(y), rest @ ..] => {
            if x.chars().count() != y.chars().count() {
                return Err(value_error("the first two maketrans arguments must have equal length"));
            }
            let mut pairs: Vec<(Py, Py)> = x.chars().zip(y.chars()).map(|(p, q)| (ord(p), ord(q))).collect();
            if let Some(Py::Str(z)) = rest.first() {
                pairs.extend(z.chars().map(|c| (ord(c), Py::None)));
            }
            Ok(Py::dict(pairs))
        }
        _ => Err(type_error("maketrans")),
    }
}

/// s.translate(table): each character looked up by its code point (a missing one stays)
fn str_translate(s: &str, table: &Py) -> R {
    let mut out = String::new();
    for c in s.chars() {
        let hit = match table {
            Py::Dict(d) => d.borrow().iter().find(|(k, _)| eq(k, &Py::Int(c as i64))).map(|(_, v)| v.clone()),
            _ => return Err(type_error("translate")),
        };
        match hit {
            None => out.push(c),
            Some(Py::None) => {}
            Some(Py::Int(n)) => out.push(char::from_u32(n as u32).ok_or_else(|| value_error("chr() arg not in range"))?),
            Some(Py::Str(t)) => out.push_str(&t),
            Some(_) => return Err(PyErr::Type("character mapping must return integer, None or str".into())),
        }
    }
    Ok(Py::str(&out))
}

/// str(x)
pub fn to_str(a: &Py) -> String {
    match a {
        Py::Str(s) => s.to_string(),
        Py::Frac(q) => fr::to_str(q),                      // str(Fraction(3, 4)) is "3/4"
        _ => repr(a),
    }
}

/// format(x, spec) inside an f-string
pub fn format(a: &Py, spec: &str) -> Result<String, PyErr> {
    Ok(match a {
        Py::Float(x) => crate::pyfmt::fmt(*x, spec),
        Py::Int(i) => {
            // an integer presentation type (b, o, x, X, d, n, c, none; fill, width, sign, #, grouping) exactly
            match crate::pyfmt::fmt_bigint(&BigInt::from(*i), spec) {
                Some(Ok(s)) => s,
                Some(Err(m)) => return Err(value_error(m)),
                None => crate::pyfmt::fmt(*i as f64, spec),
            }
        }
        Py::Bool(b) if !spec.is_empty() => return format(&Py::Int(*b as i64), spec),
        Py::Big(n) => match spec {
            "" | "d" => big_text(n)?,
            "," | ",d" => crate::pyfmt::group(&big_text(n)?),
            _ => match crate::pyfmt::fmt_bigint(n, spec) {
                Some(Ok(s)) => s,
                Some(Err(m)) => return Err(value_error(m)),
                None => crate::pyfmt::fmt(big_f64(n)?, spec),
            },
        },
        Py::Complex(r, i) if !spec.is_empty() => crate::pycx::format((*r, *i), spec),
        // Fraction.__format__ (3.12+): ".Nf" rounds the exact value; another float spec, the value as a float
        Py::Frac(q) if !spec.is_empty() => match fr::format_fixed(q, spec) {
            Some(s) => s,
            None => crate::pyfmt::fmt(fr::to_f64(q), spec),
        },
        Py::Bool(b) if spec.is_empty() => if *b { "True".into() } else { "False".into() },
        Py::Str(s) if spec.is_empty() => s.to_string(),
        _ if spec.is_empty() => to_str(a),
        _ => return Err(PyErr::Unsupported(format!("format spec {spec} for this value"))),
    })
}

// ------------------------------------------------------------------------------------------------ builtins
/// Python 3.12+ sum(): ints exactly, floats with Neumaier compensation (Python/bltinmodule.c).
pub fn sum(items: &Py, start: Option<Py>) -> R {
    let mut acc = start.unwrap_or(Py::Int(0));
    let xs = items.items()?;
    let mut i = 0;
    // the integer prefix, exactly
    while i < xs.len() {
        match (&acc, &xs[i]) {
            (Py::Int(_) | Py::Big(_), Py::Int(_) | Py::Big(_) | Py::Bool(_)) => {
                acc = add(&acc, &xs[i])?;
                i += 1;
            }
            _ => break,
        }
    }
    if i == xs.len() {
        return Ok(acc);
    }
    if acc.is_num() && xs[i..].iter().all(|x| x.is_num()) {
        let mut f = acc.f()?;
        let mut c = 0.0f64;
        for x in &xs[i..] {
            let x = x.f()?;
            let t = f + x;
            if f.abs() >= x.abs() {
                c += (f - t) + x;
            } else {
                c += (x - t) + f;
            }
            f = t;
        }
        if c != 0.0 && c.is_finite() {
            f += c;
        }
        return Ok(Py::Float(f));
    }
    for x in &xs[i..] {
        acc = add(&acc, x)?;
    }
    Ok(acc)
}

/// math.fsum: the correctly rounded sum (Shewchuk's algorithm, as Modules/mathmodule.c).
pub fn fsum(items: &Py) -> R {
    let mut partials: Vec<f64> = vec![];
    for x in items.items()? {
        let mut x = x.f()?;
        let mut i = 0;
        for j in 0..partials.len() {
            let mut y = partials[j];
            if x.abs() < y.abs() {
                std::mem::swap(&mut x, &mut y);
            }
            let hi = x + y;
            let lo = y - (hi - x);
            if lo != 0.0 {
                partials[i] = lo;
                i += 1;
            }
            x = hi;
        }
        partials.truncate(i);
        partials.push(x);
    }
    let mut hi = 0.0;
    if let Some(mut n) = partials.len().checked_sub(1) {
        hi = partials[n];
        let mut lo = 0.0;
        while n > 0 {
            let x = hi;
            n -= 1;
            let y = partials[n];
            hi = x + y;
            let yr = hi - x;
            lo = y - yr;
            if lo != 0.0 {
                break;
            }
        }
        if n > 0 && ((lo < 0.0 && partials[n - 1] < 0.0) || (lo > 0.0 && partials[n - 1] > 0.0)) {
            let y = lo * 2.0;
            let x = hi + y;
            let yr = x - hi;
            if y == yr {
                hi = x;
            }
        }
    }
    Ok(Py::Float(hi))
}

pub fn len(a: &Py) -> R {
    Ok(Py::Int(match a {
        Py::Str(s) => s.chars().count(),
        Py::List(l) | Py::Set(l) => l.borrow().len(),
        Py::Tuple(t) => t.len(),
        Py::Dict(d) => d.borrow().len(),
        _ => return Err(PyErr::Type("object has no len()".into())),
    } as i64))
}

pub fn abs(a: &Py) -> R {
    match a {
        Py::Int(i) => Ok(i.checked_abs().map(Py::Int).unwrap_or_else(|| int_of(num_traits::Signed::abs(&BigInt::from(*i))))),
        Py::Big(n) => Ok(int_of(num_traits::Signed::abs(n.as_ref()))),
        Py::Bool(b) => Ok(Py::Int(*b as i64)),
        Py::Float(x) => Ok(Py::Float(x.abs())),
        Py::Complex(r, i) => Ok(Py::Float(crate::pycx::abs((*r, *i))?)),
        Py::Frac(q) => Ok(frac(num_traits::Signed::abs(q.as_ref()))),
        Py::Mpf(m) => Ok(crate::pympf::abs(m)),
        _ => Err(type_error("abs")),
    }
}

/// min/max of the arguments, or of one iterable argument (first of equals kept, as Python).
pub fn minmax(args: &[Py], want_max: bool) -> R {
    let xs = if args.len() == 1 { args[0].items()? } else { args.to_vec() };
    let mut it = xs.into_iter();
    let mut best = it.next().ok_or_else(|| value_error("min() arg is an empty sequence"))?;
    for x in it {
        let better = if want_max { gt(&x, &best)? } else { lt(&x, &best)? };
        if better {
            best = x;
        }
    }
    Ok(best)
}

/// range(...) in a for loop: lazy, as Python's (a loop that breaks early never makes the rest; a range(300000)
/// with an early break made 300,000 values on every call and was 650 times slower than Python)
pub fn range_iter(args: &[i64]) -> impl Iterator<Item = Py> {
    let (start, stop, step) = match args {
        [n] => (0, *n, 1),
        [a, b] => (*a, *b, 1),
        [a, b, s] => (*a, *b, *s),
        _ => (0, 0, 1),
    };
    let mut i = start;
    std::iter::from_fn(move || {
        let more = (step > 0 && i < stop) || (step < 0 && i > stop);
        if !more {
            return None;
        }
        let v = i;
        i += step;
        Some(Py::Int(v))
    })
}

pub fn range(args: &[i64]) -> Vec<Py> {
    let (start, stop, step) = match args {
        [n] => (0, *n, 1),
        [a, b] => (*a, *b, 1),
        [a, b, s] => (*a, *b, *s),
        _ => (0, 0, 1),
    };
    let mut out = vec![];
    let mut i = start;
    if step > 0 {
        while i < stop {
            out.push(Py::Int(i));
            i += step;
        }
    } else if step < 0 {
        while i > stop {
            out.push(Py::Int(i));
            i += step;
        }
    }
    out
}

/// int(x): truncation for a float (OverflowError for inf, ValueError for nan), parse for a str
pub fn int(a: &Py) -> R {
    match a {
        Py::Int(_) | Py::Big(_) => Ok(a.clone()),
        Py::Bool(b) => Ok(Py::Int(*b as i64)),
        Py::Float(x) => {
            if x.is_nan() {
                return Err(value_error("cannot convert float NaN to integer"));
            }
            if x.is_infinite() {
                return Err(PyErr::Overflow);
            }
            float_int(x.trunc())
        }
        Py::Frac(q) => {                                    // int(Fraction): toward zero
            let t = q.trunc();
            big_int(t.numer())
        }
        // int() of a sympy number: a rational exactly, toward zero; another number by its value (int(sp.sign(c)),
        // int(p.degree())); int() of an mpf: its value toward zero
        Py::Sym(e) => match &**e {
            crate::sym::Ex::Num(q) => big_int(q.trunc().numer()),
            _ => int(&Py::Float(a.f()?)),
        },
        Py::Mpf(_) => int(&Py::Float(a.f()?)),
        Py::Str(s) => {
            // Python: optional sign, digits with single underscores between them, surrounding whitespace
            let t = s.trim();
            let digits = t.strip_prefix(['+', '-']).unwrap_or(t);
            let ok = !digits.is_empty() && !digits.starts_with('_') && !digits.ends_with('_') && !digits.contains("__")
                && digits.chars().all(|c| c.is_ascii_digit() || c == '_');
            if !ok {
                return Err(value_error(format!("invalid literal for int() with base 10: {}", repr(a))));
            }
            let clean = t.replace('_', "");
            let nd = clean.trim_start_matches(['+', '-']).len();
            if nd > MAX_STR_DIGITS {
                return Err(str_digits_error(nd));               // checked before parsing, as CPython does
            }
            let n: BigInt = clean.parse().map_err(|_| big())?;
            bounded(n)
        }
        _ => Err(type_error("int")),
    }
}

/// int(text, base): base 0 (the prefix decides) or 2..36; sign, surrounding space, single underscores between
/// digits and a 0b/0o/0x prefix matching the base, as Python reads them
pub fn int_base(a: &Py, base: &Py) -> R {
    let b = base.i()?;
    let Py::Str(s) = a else {
        return Err(PyErr::Type("int() can't convert non-string with explicit base".into()));
    };
    if b != 0 && !(2..=36).contains(&b) {
        return Err(value_error("int() base must be >= 2 and <= 36, or 0"));
    }
    let bad = || value_error(format!("invalid literal for int() with base {b}: {}", repr(a)));
    let t = s.trim();
    let (neg, body) = match t.strip_prefix('-') {
        Some(x) => (true, x),
        None => (false, t.strip_prefix('+').unwrap_or(t)),
    };
    let low = body.to_ascii_lowercase();
    let (radix, digits) = match (b, low.get(..2)) {
        (0 | 2, Some("0b")) => (2, &body[2..]),
        (0 | 8, Some("0o")) => (8, &body[2..]),
        (0 | 16, Some("0x")) => (16, &body[2..]),
        (0, _) => (10, body),
        (r, _) => (r as u32, body),
    };
    // after a prefix an underscore may come first ("0x_ff"); elsewhere only between digits
    let digits = if digits.len() < body.len() { digits.strip_prefix('_').unwrap_or(digits) } else { digits };
    if digits.is_empty() || digits.starts_with('_') || digits.ends_with('_') || digits.contains("__") {
        return Err(bad());
    }
    let clean: String = digits.chars().filter(|c| *c != '_').collect();
    if !clean.chars().all(|c| c.is_digit(radix)) {
        return Err(bad());
    }
    // base 0 and decimal: no leading zeros except for zero itself ("010" is an error in Python 3)
    if b == 0 && radix == 10 && clean.len() > 1 && clean.starts_with('0') && clean.chars().any(|c| c != '0') {
        return Err(bad());
    }
    let nd = clean.len();
    if radix == 10 && nd > MAX_STR_DIGITS {
        return Err(str_digits_error(nd));
    }
    let n = BigInt::parse_bytes(clean.as_bytes(), radix).ok_or_else(bad)?;
    bounded(if neg { -n } else { n })
}

/// bin(x), oct(x), hex(x): the prefix after the sign, lower-case digits
pub fn int_text_base(a: &Py, which: &str) -> R {
    let n = bi(a).ok_or_else(|| PyErr::Type(format!("'{}' object cannot be interpreted as an integer", type_name(a))))?;
    let (radix, prefix) = match which { "bin" => (2, "0b"), "oct" => (8, "0o"), _ => (16, "0x") };
    let mag = num_traits::Signed::abs(&n).to_str_radix(radix);
    let sign = if num_traits::Signed::is_negative(&n) { "-" } else { "" };
    Ok(Py::str(&format!("{sign}{prefix}{mag}")))
}

/// An integral float as an int (exact: every float beyond 2^53 is an integer).
fn float_int(x: f64) -> R {
    if x.abs() < 9.2e18 {
        return Ok(Py::Int(x as i64));
    }
    num_traits::FromPrimitive::from_f64(x).map(int_of).ok_or_else(big)
}

/// float(x)
pub fn float(a: &Py) -> R {
    match a {
        Py::Str(s) => crate::coerce::py_float(s).map(Py::Float)
            .ok_or_else(|| value_error(format!("could not convert string to float: {}", repr(a)))),
        _ => Ok(Py::Float(a.f()?)),
    }
}

/// round(x) -> int, round(x, n) -> float (an int stays an int)
pub fn round(a: &Py, n: Option<i64>) -> R {
    match (a, n) {
        (Py::Int(_) | Py::Big(_), _) => Ok(a.clone()),
        (Py::Frac(q), None) => big_int(&fr::round_even(q)),
        (Py::Frac(q), Some(n)) => Ok(frac(fr::round_n(q, n)?)),
        (_, None) => {
            let x = a.f()?;
            if !x.is_finite() {
                return Err(if x.is_nan() { value_error("cannot convert float NaN to integer") } else { PyErr::Overflow });
            }
            float_int(crate::pyfmt::round_half_even(x))
        }
        (_, Some(n)) => Ok(Py::Float(crate::pyfmt::round_n(a.f()?, n as i32))),
    }
}

/// sorted(xs[, reverse])
pub fn sorted(a: &Py, reverse: bool) -> R {
    let mut xs = a.items()?;
    let mut err = None;
    xs.sort_by(|p, q| match order(p, q) {
        Ok(o) => o,
        Err(e) => {
            err = Some(e);
            std::cmp::Ordering::Equal
        }
    });
    if let Some(e) = err {
        return Err(e);
    }
    if reverse {
        xs.reverse();
    }
    Ok(Py::list(xs))
}

/// The value as JSON, as the Python package's answer serializes it.
pub fn to_json(a: &Py) -> serde_json::Value {
    use serde_json::{json, Value};
    match a {
        Py::None => Value::Null,
        Py::Bool(b) => json!(b),
        Py::Int(i) => json!(i),
        // every digit, as Python's json writes an int (serde_json's arbitrary_precision keeps the text)
        Py::Big(n) => big_text(n).ok().and_then(|s| serde_json::from_str(&s).ok()).unwrap_or(Value::Null),
        Py::Float(x) => if x.is_finite() { json!(*x + 0.0) } else { Value::Null },
        // as the package's answers carry a complex value: {"re", "im"}
        Py::Complex(r, i) => json!({"re": to_json(&Py::Float(*r)), "im": to_json(&Py::Float(*i))}),
        Py::Frac(q) => to_json(&Py::Float(fr::to_f64(q))),
        Py::Str(s) => json!(s.as_ref()),
        Py::List(l) | Py::Set(l) => Value::Array(l.borrow().iter().map(to_json).collect()),
        Py::Tuple(t) => Value::Array(t.iter().map(to_json).collect()),
        Py::Dict(d) => Value::Object(d.borrow().iter().map(|(k, v)| (to_str(k), to_json(v))).collect()),
        Py::Func(_) | Py::Re(_) | Py::Match(_) => Value::Null,
        Py::Sym(e) => match crate::sym::eval(e, &|_| None) {
            Ok(v) if crate::sym::is_number(e) => to_json(&Py::Float(v)),
            _ => json!(crate::sym::sstr(e)),
        },
        Py::Poly(p) => json!(crate::sym::sstr(&p.as_ex())),
        Py::Mpf(m) => to_json(&Py::Float(crate::pympf::to_f64(&m.0))),
    }
}

/// An input as the procedure receives it.
pub fn from_val(v: &crate::pack::Val) -> Py {
    use crate::pack::Val;
    match v {
        Val::Num(x) => Py::Float(*x),
        Val::Int(i) => Py::Int(*i),
        Val::Text(t) => Py::str(t),
        Val::Bool(b) => Py::Bool(*b),
        Val::Vec(xs) => Py::floats(xs),
        Val::Mat(rows) => Py::list(rows.iter().map(|r| Py::floats(r)).collect()),
        // a list of tuples (u, v) or (u, v, weight), as inputs.py hands them to the procedure
        Val::Edges(es) => Py::list(es.iter().map(|(u, v, w)| {
            let mut t = vec![Py::str(u), Py::str(v)];
            if let Some(w) = w {
                t.push(Py::Float(*w));
            }
            Py::tuple(t)
        }).collect()),
    }
}

// ------------------------------------------------------------------------------------------------ math module
fn domain() -> PyErr {
    value_error("math domain error")
}

fn checked(x: f64, args_finite: bool) -> R {
    if x.is_nan() && args_finite {
        return Err(domain());
    }
    if x.is_infinite() && args_finite {
        return Err(PyErr::Overflow);
    }
    Ok(Py::Float(x))
}

pub fn math1(name: &str, a: &Py) -> R {
    if let Py::Frac(q) = a {                             // math.floor/ceil/trunc of a Fraction are exact ints
        match name {
            "floor" => return big_int(&fr::floor(q)),
            "ceil" => return big_int(&-fr::floor(&-(**q).clone())),
            "trunc" => return big_int(q.trunc().numer()),
            _ => {}
        }
    }
    if let Py::Big(n) = a {
        if matches!(name, "floor" | "ceil" | "trunc") {
            return Ok(a.clone());
        }
        if matches!(name, "log" | "log10" | "log2") && big_f64(n).is_err() {
            // CPython's loghelper for an int beyond the float range: log(x) + log(2) * e with n = x * 2**e
            if num_traits::Signed::is_negative(n.as_ref()) {
                return Err(domain());
            }
            let mut e = n.bits() as i64;
            let mut x = fr::to_f64(&Q::new((**n).clone(), BigInt::from(1) << e as usize));
            if x == 1.0 {
                x = 0.5;
                e += 1;
            }
            let f = |v: f64| match name { "log" => v.ln_lm(), "log10" => v.log10_lm(), _ => v.log2_lm() };
            return Ok(Py::Float(f(x) + f(2.0) * e as f64));
        }
    }
    let x = a.f()?;
    let fin = x.is_finite();
    match name {
        "sqrt" => if x < 0.0 { Err(domain()) } else { Ok(Py::Float(x.sqrt())) },
        "exp" => checked(x.exp_lm(), fin),
        "expm1" => checked(x.exp_m1_lm(), fin),
        "log" => if x <= 0.0 { Err(domain()) } else { Ok(Py::Float(x.ln_lm())) },
        "log10" => if x <= 0.0 { Err(domain()) } else { Ok(Py::Float(x.log10_lm())) },
        "log2" => if x <= 0.0 { Err(domain()) } else { Ok(Py::Float(x.log2_lm())) },
        "log1p" => if x <= -1.0 { Err(domain()) } else { Ok(Py::Float(x.ln_1p_lm())) },
        "sin" => if x.is_infinite() { Err(domain()) } else { Ok(Py::Float(x.sin_lm())) },
        "cos" => if x.is_infinite() { Err(domain()) } else { Ok(Py::Float(x.cos_lm())) },
        "tan" => if x.is_infinite() { Err(domain()) } else { Ok(Py::Float(x.tan_lm())) },
        "asin" => if !(-1.0..=1.0).contains(&x) { Err(domain()) } else { Ok(Py::Float(x.asin_lm())) },
        "acos" => if !(-1.0..=1.0).contains(&x) { Err(domain()) } else { Ok(Py::Float(x.acos_lm())) },
        "atan" => Ok(Py::Float(x.atan_lm())),
        "sinh" => checked(x.sinh_lm(), fin),
        "cosh" => checked(x.cosh_lm(), fin),
        "tanh" => Ok(Py::Float(x.tanh_lm())),
        "asinh" => Ok(Py::Float(x.asinh_lm())),
        "acosh" => if x < 1.0 { Err(domain()) } else { Ok(Py::Float(x.acosh_lm())) },
        "atanh" => if x <= -1.0 || x >= 1.0 { Err(domain()) } else { Ok(Py::Float(x.atanh_lm())) },
        "radians" => Ok(Py::Float(x * (std::f64::consts::PI / 180.0))),
        "degrees" => Ok(Py::Float(x * (180.0 / std::f64::consts::PI))),
        "fabs" => Ok(Py::Float(x.abs())),
        "isfinite" => Ok(Py::Bool(x.is_finite())),
        "isnan" => Ok(Py::Bool(x.is_nan())),
        "isinf" => Ok(Py::Bool(x.is_infinite())),
        "floor" | "ceil" | "trunc" => {
            if let Py::Int(_) = a {
                return Ok(a.clone());
            }
            if x.is_nan() {
                return Err(value_error("cannot convert float NaN to integer"));
            }
            if x.is_infinite() {
                return Err(PyErr::Overflow);
            }
            float_int(match name { "floor" => x.floor(), "ceil" => x.ceil(), _ => x.trunc() })
        }
        // CPython's own algorithms (crate::pymath): bit for bit the package's values
        "gamma" | "lgamma" => {
            let r = if name == "gamma" { crate::pymath::tgamma(x) } else { crate::pymath::lgamma(x) };
            match r {
                Ok(v) => Ok(Py::Float(v)),
                Err(crate::pymath::MathErr::Domain) => Err(domain()),
                Err(crate::pymath::MathErr::Range) => Err(PyErr::Overflow),
            }
        }
        "erf" => Ok(Py::Float(crate::pymath::erf(x))),
        "erfc" => Ok(Py::Float(crate::pymath::erfc(x))),
        _ => Err(PyErr::Unsupported(format!("math.{name}"))),
    }
}

pub fn atan2(y: &Py, x: &Py) -> R {
    Ok(Py::Float(y.f()?.atan2_lm(x.f()?)))
}

/// math.hypot(*coordinates): CPython's own vector_norm (Modules/mathmodule.c, not the C library's hypot), so the
/// result is Python's on every platform
pub fn hypot(args: &[Py]) -> R {
    let mut xs: Vec<f64> = args.iter().map(|a| a.f().map(f64::abs)).collect::<Result<_, _>>()?;
    let (mut max, mut nan) = (0.0f64, false);
    for &x in &xs {
        nan |= x.is_nan();
        if x > max {
            max = x;
        }
    }
    Ok(Py::Float(vector_norm(&mut xs, max, nan)))
}

/// The length of (x, y) correctly rounded (CPython's vector_norm): abs() of a complex number, whose C-library hypot is
/// correctly rounded on the reference Linux (glibc) where the libm crate's can be one unit in the last place off
pub(crate) fn hypot2(x: f64, y: f64) -> f64 {
    let mut v = [x.abs(), y.abs()];
    let max = v[0].max(v[1]);
    let nan = x.is_nan() || y.is_nan();
    if (x.is_infinite() || y.is_infinite()) && nan {
        return f64::INFINITY;
    }
    vector_norm(&mut v, max, nan)
}

/// CPython 3.13 vector_norm: lossless scaling and squaring (an exact product by fused multiply-add), compensated
/// sums, one differential correction. `v` holds absolute values; `max` the largest.
fn vector_norm(v: &mut [f64], max: f64, found_nan: bool) -> f64 {
    if max.is_infinite() {
        return max;
    }
    if found_nan {
        return f64::NAN;
    }
    if max == 0.0 || v.len() <= 1 {
        return max;
    }
    let (_, max_e) = libm::frexp(max);
    if max_e < -1023 {
        for x in v.iter_mut() {
            *x /= f64::MIN_POSITIVE;                          // subnormals to normals
        }
        return f64::MIN_POSITIVE * vector_norm(v, max / f64::MIN_POSITIVE, found_nan);
    }
    let scale = libm::ldexp(1.0, -max_e);
    let dl_mul = |x: f64, y: f64| {
        let z = x * y;
        (z, x.mul_add(y, -z))
    };
    let dl_fast_sum = |a: f64, b: f64| {
        let x = a + b;
        (x, (a - x) + b)
    };
    let (mut csum, mut frac1, mut frac2) = (1.0f64, 0.0f64, 0.0f64);
    for &x in v.iter() {
        let x = x * scale;
        let pr = dl_mul(x, x);
        let sm = dl_fast_sum(csum, pr.0);
        csum = sm.0;
        frac1 += pr.1;
        frac2 += sm.1;
    }
    let mut h = (csum - 1.0 + (frac1 + frac2)).sqrt();
    let pr = dl_mul(-h, h);
    let sm = dl_fast_sum(csum, pr.0);
    csum = sm.0;
    frac1 += pr.1;
    frac2 += sm.1;
    let x = csum - 1.0 + (frac1 + frac2);
    h += x / (2.0 * h);
    h / scale
}

pub fn math_pow(a: &Py, b: &Py) -> R {
    let (x, y) = (a.f()?, b.f()?);
    if x == 0.0 && y < 0.0 {
        return Err(domain());
    }
    if x < 0.0 && y.is_finite() && y != y.floor() {
        return Err(domain());
    }
    let r = x.powf_lm(y);
    if r.is_infinite() && x.is_finite() && y.is_finite() {
        return Err(PyErr::Overflow);
    }
    Ok(Py::Float(r))
}

pub fn isclose(a: &Py, b: &Py, rel_tol: f64, abs_tol: f64) -> R {
    let (x, y) = (a.f()?, b.f()?);
    if x == y {
        return Ok(Py::Bool(true));
    }
    if x.is_infinite() || y.is_infinite() {
        return Ok(Py::Bool(false));
    }
    let d = (x - y).abs();
    Ok(Py::Bool(d <= (rel_tol * y.abs()).max(rel_tol * x.abs()) || d <= abs_tol))
}

pub fn copysign(a: &Py, b: &Py) -> R {
    Ok(Py::Float(a.f()?.copysign(b.f()?)))
}

pub fn fmod(a: &Py, b: &Py) -> R {
    let (x, y) = (a.f()?, b.f()?);
    if y == 0.0 {
        return Err(domain());
    }
    Ok(Py::Float(x % y))
}

pub fn factorial(a: &Py) -> R {
    let n = match a {
        Py::Int(n) => *n,
        _ => return Err(PyErr::Type("factorial() only accepts integral values".into())),
    };
    if n < 0 {
        return Err(value_error("factorial() not defined for negative values"));
    }
    if n > 200_000 {
        return Err(big());                                  // over 3 million digits: beyond MAX_BITS anyway
    }
    bounded(range_product(2, n))
}

pub fn comb(n: &Py, k: &Py) -> R {
    let (n, k) = (n.i()?, k.i()?);
    if n < 0 {
        return Err(value_error("n must be a non-negative integer"));
    }
    if k < 0 {
        return Err(value_error("k must be a non-negative integer"));
    }
    if k > n {
        return Ok(Py::Int(0));
    }
    let k = k.min(n - k);
    let mut r: i128 = 1;
    for i in 0..k {
        r = r * (n - i) as i128 / (i + 1) as i128;          // exact: r is C(n, i + 1) after each step
        if r > i64::MAX as i128 / (n as i128 + 1) {
            // continue exactly on unbounded integers: C(n, k) = n! / ((n - k)! k!) from the falling factorial
            if k > 200_000 || falling_bits(n, k) > MAX_BITS as f64 {
                return Err(big());                          // sized before it is computed
            }
            return bounded(range_product(n - k + 1, n) / range_product(2, k));
        }
    }
    Ok(Py::Int(r as i64))
}

pub fn gcd(a: &Py, b: &Py) -> R {
    if let (Some(x), Some(y)) = (bi(a), bi(b)) {
        return Ok(int_of(num_integer::Integer::gcd(&x, &y)));
    }
    Err(PyErr::Type("an integer is required".into()))
}

// gamma, lgamma, erf by standard series (Lanczos; erf by a high-accuracy rational approximation): the procedures
// that use them are checked against Python to 10 digits by tools/rust_accuracy.py
// ------------------------------------------------------------------------------------------------ statistics
/// statistics.NormalDist(mu, sigma).<method>(args) for inv_cdf, cdf, pdf and zscore, with CPython 3.13's formulas
/// and its StatisticsError messages (a ValueError: the refusal's reason).
pub fn normal_dist(method: &str, mu: &Py, sigma: &Py, args: &[Py]) -> R {
    let (mu, sigma) = (float(mu)?.f()?, float(sigma)?.f()?);
    if sigma < 0.0 {
        return Err(value_error("sigma must be non-negative"));
    }
    let x = args.first().ok_or_else(|| type_error(method))?;
    match method {
        "inv_cdf" => {
            let p = x.f()?;
            if p <= 0.0 || p >= 1.0 {
                return Err(value_error("p must be in the range 0.0 < p < 1.0"));
            }
            Ok(Py::Float(crate::pymath::normal_inv_cdf(p, mu, sigma)))
        }
        "cdf" => {
            if sigma == 0.0 {
                return Err(value_error("cdf() not defined when sigma is zero"));
            }
            let x = x.f()?;
            Ok(Py::Float(0.5 * (1.0 + crate::pymath::erf((x - mu) / (sigma * std::f64::consts::SQRT_2)))))
        }
        "pdf" => {
            let variance = sigma * sigma;
            if variance == 0.0 {
                return Err(value_error("pdf() not defined when sigma is zero"));
            }
            let diff = x.f()? - mu;
            Ok(Py::Float((diff * diff / (-2.0 * variance)).exp_lm() / (std::f64::consts::TAU * variance).sqrt()))
        }
        "zscore" => {
            if sigma == 0.0 {
                return Err(value_error("zscore() not defined when sigma is zero"));
            }
            Ok(Py::Float((x.f()? - mu) / sigma))
        }
        _ => Err(PyErr::Unsupported(format!("NormalDist.{method}"))),
    }
}

// ------------------------------------------------------------------------------------------------ integer builtins
/// pow(b, e, m): modular power of ints (a negative e is the modular inverse's power, Python 3.8+); the result takes
/// the sign of m, as Python's
pub fn pow_mod(b: &Py, e: &Py, m: &Py) -> R {
    if [b, e, m].iter().any(|x| matches!(x, Py::Big(_))) {
        return big_pow_mod(b, e, m);
    }
    let (b, e, m) = (b.i()? as i128, e.i()? as i128, m.i()? as i128);
    if m == 0 {
        return Err(value_error("pow() 3rd argument cannot be 0"));
    }
    let md = m.abs();
    let mut base = b.rem_euclid(md);
    let mut e = e;
    if e < 0 {
        // the inverse of base modulo |m| (extended Euclid)
        let (mut r0, mut r1, mut s0, mut s1) = (md, base, 0i128, 1i128);
        while r1 != 0 {
            let q = r0 / r1;
            (r0, r1) = (r1, r0 - q * r1);
            (s0, s1) = (s1, s0 - q * s1);
        }
        if r0 != 1 {
            return Err(value_error("base is not invertible for the given modulus"));
        }
        base = s0.rem_euclid(md);
        e = -e;
    }
    let mut r: i128 = 1 % md;
    while e > 0 {
        if e & 1 == 1 {
            r = r * base % md;
        }
        base = base * base % md;
        e >>= 1;
    }
    // Python: the result has the sign of m (0 <= r < m, or m < r <= 0)
    let r = if m < 0 && r != 0 { r - md } else { r };
    Ok(Py::Int(r as i64))
}

/// pow(b, e, m) with an operand beyond 64 bits, on unbounded integers (same rules as pow_mod).
fn big_pow_mod(b: &Py, e: &Py, m: &Py) -> R {
    use num_traits::{One, Signed, Zero};
    let need = |x: &Py| bi(x).ok_or_else(|| PyErr::Type("pow() 3rd argument not allowed unless all arguments are integers".into()));
    let (b, e, m) = (need(b)?, need(e)?, need(m)?);
    if m.is_zero() {
        return Err(value_error("pow() 3rd argument cannot be 0"));
    }
    let md = m.abs();
    let mut base = num_integer::Integer::mod_floor(&b, &md);
    let mut e = e;
    if e.is_negative() {
        let g = num_integer::Integer::extended_gcd(&base, &md);
        if !g.gcd.is_one() {
            return Err(value_error("base is not invertible for the given modulus"));
        }
        base = num_integer::Integer::mod_floor(&g.x, &md);
        e = -e;
    }
    // the work grows with the exponent's bits times the modulus' size squared: beyond a budget, unsupported
    let (eb, mb) = (e.bits() as u128, md.bits() as u128);
    if eb * mb * mb > 1u128 << 40 {
        return Err(big());
    }
    let r = base.modpow(&e, &md);
    Ok(int_of(if m.is_negative() && !r.is_zero() { r - md } else { r }))
}

/// math.isqrt(n): the integer square root
pub fn isqrt(n: &Py) -> R {
    if let Py::Big(b) = n {
        if num_traits::Signed::is_negative(b.as_ref()) {
            return Err(value_error("isqrt() argument must be nonnegative"));
        }
        return Ok(int_of(num_integer::Roots::sqrt(b.as_ref())));
    }
    let n = n.i()?;
    if n < 0 {
        return Err(value_error("isqrt() argument must be nonnegative"));
    }
    Ok(Py::Int((n as u64).isqrt() as i64))
}

/// math.perm(n, k): n! / (n - k)!, 0 when k > n
pub fn perm(n: &Py, k: &Py) -> R {
    let (n, k) = (n.i()?, k.i()?);
    if n < 0 {
        return Err(value_error("n must be a non-negative integer"));
    }
    if k < 0 {
        return Err(value_error("k must be a non-negative integer"));
    }
    if k > n {
        return Ok(Py::Int(0));
    }
    if k > 200_000 || falling_bits(n, k) > MAX_BITS as f64 {
        return Err(big());
    }
    bounded(range_product(n - k + 1, n))
}

/// ord(c), chr(i)
pub fn ord(c: &Py) -> R {
    let s = c.as_str()?;
    let mut it = s.chars();
    match (it.next(), it.next()) {
        (Some(ch), None) => Ok(Py::Int(ch as i64)),
        _ => Err(PyErr::Type(format!("ord() expected a character, but string of length {} found", s.chars().count()))),
    }
}

pub fn chr(i: &Py) -> R {
    let n = i.i()?;
    u32::try_from(n).ok().and_then(char::from_u32).map(|c| Py::str(&c.to_string()))
        .ok_or_else(|| value_error("chr() arg not in range(0x110000)"))
}

// ------------------------------------------------------------------------------------------------ integer bits
fn int_pair(a: &Py, b: &Py, op: &str) -> Result<(i64, i64), PyErr> {
    both_int(a, b).ok_or_else(|| type_error(op))
}

/// a & b (ints and bools; two bools give a bool, as Python)
pub fn bitand(a: &Py, b: &Py) -> R {
    if let Some(r) = set_op(a, b, "&") {
        return r;
    }
    if let (Py::Bool(x), Py::Bool(y)) = (a, b) {
        return Ok(Py::Bool(*x & *y));
    }
    if let Some((x, y)) = both_big(a, b) {
        return Ok(int_of(x & y));                           // two's complement, as Python's ints
    }
    let (x, y) = int_pair(a, b, "&")?;
    Ok(Py::Int(x & y))
}

pub fn bitor(a: &Py, b: &Py) -> R {
    if let Some(r) = set_op(a, b, "|") {
        return r;
    }
    if let (Py::Bool(x), Py::Bool(y)) = (a, b) {
        return Ok(Py::Bool(*x | *y));
    }
    if let Some((x, y)) = both_big(a, b) {
        return Ok(int_of(x | y));
    }
    let (x, y) = int_pair(a, b, "|")?;
    Ok(Py::Int(x | y))
}

pub fn bitxor(a: &Py, b: &Py) -> R {
    if let Some(r) = set_op(a, b, "^") {
        return r;
    }
    if let (Py::Bool(x), Py::Bool(y)) = (a, b) {
        return Ok(Py::Bool(*x ^ *y));
    }
    if let Some((x, y)) = both_big(a, b) {
        return Ok(int_of(x ^ y));
    }
    let (x, y) = int_pair(a, b, "^")?;
    Ok(Py::Int(x ^ y))
}

/// The value and the count of a shift: ints (the value may be big); ValueError for a negative count.
fn shift_args(a: &Py, b: &Py, op: &str) -> Result<(BigInt, i64), PyErr> {
    let x = bi(a).ok_or_else(|| type_error(op))?;
    let n = match b {
        Py::Int(n) => *n,
        Py::Bool(t) => *t as i64,
        Py::Big(n) if num_traits::Signed::is_negative(n.as_ref()) => -1,
        Py::Big(_) => i64::MAX,
        _ => return Err(type_error(op)),
    };
    if n < 0 {
        return Err(value_error("negative shift count"));
    }
    Ok((x, n))
}

/// a << n: a * 2**n exactly (within MAX_BITS)
pub fn lshift(a: &Py, b: &Py) -> R {
    let (x, n) = shift_args(a, b, "<<")?;
    if num_traits::Zero::is_zero(&x) {
        return Ok(Py::Int(0));
    }
    if x.bits().saturating_add(n as u64) > MAX_BITS {
        return Err(big());
    }
    Ok(int_of(x << n as usize))
}

/// a >> n: an arithmetic shift (floor division by 2**n, as Python)
pub fn rshift(a: &Py, b: &Py) -> R {
    let (x, n) = shift_args(a, b, ">>")?;
    if n as u64 >= x.bits() {
        return Ok(Py::Int(if num_traits::Signed::is_negative(&x) { -1 } else { 0 }));
    }
    Ok(int_of(num_integer::Integer::div_floor(&x, &(BigInt::from(1) << n as usize))))
}

// ------------------------------------------------------------------------------------------------ heapq, itertools
fn heap_list(h: &Py) -> Result<Rc<RefCell<Vec<Py>>>, PyErr> {
    match h {
        Py::List(l) => Ok(l.clone()),
        _ => Err(PyErr::Type("heap argument must be a list".into())),
    }
}

/// heapq._siftdown (Lib/heapq.py; the C module runs the same steps): items compared with < only, so equal keys
/// leave the heap in the order Python leaves it
fn siftdown(h: &mut [Py], start: usize, mut pos: usize) -> Result<(), PyErr> {
    let item = h[pos].clone();
    while pos > start {
        let parent = (pos - 1) >> 1;
        if lt(&item, &h[parent])? {
            h[pos] = h[parent].clone();
            pos = parent;
            continue;
        }
        break;
    }
    h[pos] = item;
    Ok(())
}

/// heapq._siftup
fn siftup(h: &mut [Py], mut pos: usize) -> Result<(), PyErr> {
    let (end, start) = (h.len(), pos);
    let item = h[pos].clone();
    let mut child = 2 * pos + 1;
    while child < end {
        let right = child + 1;
        if right < end && !lt(&h[child], &h[right])? {
            child = right;
        }
        h[pos] = h[child].clone();
        pos = child;
        child = 2 * pos + 1;
    }
    h[pos] = item;
    siftdown(h, start, pos)
}

/// heapq.nsmallest(n, items, key=None) / nlargest: as Python documents them, sorted(items, key=key)[:n] (reverse for
/// nlargest), the sort stable
pub fn nsmallest(n: &Py, items: &Py, key: Option<&Py>, largest: bool) -> R {
    let k = n.i()?.max(0) as usize;
    let l = Py::list(items.items()?);
    method_sort(&l, key, largest)?;
    let v = l.items()?;
    Ok(Py::list(v.into_iter().take(k).collect()))
}

/// heapq.heappush / heappop / heapify / heapreplace / heappushpop, in place on the list
pub fn heapq(name: &str, a: &[Py]) -> R {
    let l = heap_list(a.first().ok_or_else(|| type_error(name))?)?;
    let mut h = l.borrow_mut();
    match (name, a.len()) {
        ("heappush", 2) => {
            h.push(a[1].clone());
            let n = h.len() - 1;
            siftdown(&mut h, 0, n)?;
            Ok(Py::None)
        }
        ("heappop", 1) => {
            let last = h.pop().ok_or_else(|| PyErr::Other("IndexError: index out of range".into()))?;
            if h.is_empty() {
                return Ok(last);
            }
            let top = std::mem::replace(&mut h[0], last);
            siftup(&mut h, 0)?;
            Ok(top)
        }
        ("heapify", 1) => {
            for i in (0..h.len() / 2).rev() {
                siftup(&mut h, i)?;
            }
            Ok(Py::None)
        }
        ("heapreplace", 2) => {
            if h.is_empty() {
                return Err(PyErr::Other("IndexError: index out of range".into()));
            }
            let top = std::mem::replace(&mut h[0], a[1].clone());
            siftup(&mut h, 0)?;
            Ok(top)
        }
        ("heappushpop", 2) => {
            if !h.is_empty() && lt(&h[0], &a[1])? {
                let top = std::mem::replace(&mut h[0], a[1].clone());
                siftup(&mut h, 0)?;
                return Ok(top);
            }
            Ok(a[1].clone())
        }
        _ => Err(PyErr::Unsupported(format!("heapq.{name}"))),
    }
}

/// itertools.combinations / permutations / product / chain, as lists of tuples in Python's order
pub fn itertools(name: &str, a: &[Py]) -> R {
    let r_of = |k: usize, n: usize| -> Result<usize, PyErr> {
        match a.get(k) {
            None | Some(Py::None) => Ok(n),
            Some(x) => {
                let r = x.i()?;
                if r < 0 {
                    return Err(value_error("r must be non-negative"));
                }
                Ok(r as usize)
            }
        }
    };
    match name {
        "combinations" | "permutations" => {
            let pool = a.first().ok_or_else(|| type_error(name))?.items()?;
            let n = pool.len();
            let r = if name == "combinations" {
                r_of(1, usize::MAX).and_then(|r| if r == usize::MAX { Err(type_error(name)) } else { Ok(r) })?
            } else {
                r_of(1, n)?
            };
            let mut out = vec![];
            if r > n {
                return Ok(Py::list(out));
            }
            if name == "combinations" {
                // indices in lexicographic order (the itertools recipe)
                let mut idx: Vec<usize> = (0..r).collect();
                loop {
                    out.push(Py::tuple(idx.iter().map(|&i| pool[i].clone()).collect()));
                    let Some(i) = (0..r).rev().find(|&i| idx[i] != i + n - r) else { break };
                    idx[i] += 1;
                    for j in i + 1..r {
                        idx[j] = idx[j - 1] + 1;
                    }
                }
            } else {
                // the itertools recipe's order: indices and cycles
                let mut idx: Vec<usize> = (0..n).collect();
                let mut cycles: Vec<usize> = (0..r).map(|i| n - i).collect();
                out.push(Py::tuple(idx[..r].iter().map(|&i| pool[i].clone()).collect()));
                'outer: while n > 0 {
                    for i in (0..r).rev() {
                        cycles[i] -= 1;
                        if cycles[i] == 0 {
                            let x = idx.remove(i);
                            idx.push(x);
                            cycles[i] = n - i;
                        } else {
                            let j = cycles[i];
                            idx.swap(i, n - j);
                            out.push(Py::tuple(idx[..r].iter().map(|&k| pool[k].clone()).collect()));
                            continue 'outer;
                        }
                    }
                    break;
                }
            }
            if out.len() > 10_000_000 {
                return Err(big());
            }
            Ok(Py::list(out))
        }
        "product" => {
            let pools: Vec<Vec<Py>> = a.iter().map(|p| p.items()).collect::<Result<_, _>>()?;
            let mut out: Vec<Vec<Py>> = vec![vec![]];
            for pool in &pools {
                let mut next = Vec::with_capacity(out.len() * pool.len());
                for x in &out {
                    for y in pool {
                        let mut t = x.clone();
                        t.push(y.clone());
                        next.push(t);
                    }
                }
                if next.len() > 10_000_000 {
                    return Err(big());
                }
                out = next;
            }
            Ok(Py::list(out.into_iter().map(Py::tuple).collect()))
        }
        "chain" => {
            let mut out = vec![];
            for p in a {
                out.extend(p.items()?);
            }
            Ok(Py::list(out))
        }
        _ => Err(PyErr::Unsupported(format!("itertools.{name}"))),
    }
}

// ------------------------------------------------------------------------------------------------ complex
/// x.real: a complex's real part; an int or float is its own real part (int stays int).
/// functools.reduce(f, items[, initial]): f folded over the items from the left (TypeError for an empty sequence
/// without an initial value, as Python)
pub fn reduce(f: &Py, items: &Py, initial: Option<Py>) -> R {
    let mut it = items.items()?.into_iter();
    let mut acc = match initial {
        Some(v) => v,
        None => it.next().ok_or_else(|| PyErr::Type("reduce() of empty iterable with no initial value".into()))?,
    };
    for x in it {
        acc = call(f, &[acc, x])?;
    }
    Ok(acc)
}

/// an attribute of a symbolic value or a polynomial (e.is_number, p.gen, r.p): crate::pysym
pub fn getattr(a: &Py, name: &str) -> R {
    crate::pysym::attr(a, name).unwrap_or_else(|| Err(PyErr::Type(format!("'{}' object has no attribute '{name}'", type_name(a)))))
}

pub fn real(a: &Py) -> R {
    match a {
        Py::Complex(r, _) => Ok(Py::Float(*r)),
        Py::Bool(_) | Py::Int(_) | Py::Big(_) | Py::Float(_) => pos(a),
        _ => Err(PyErr::Type("object has no attribute 'real'".into())),
    }
}

/// x.imag: a complex's imaginary part; 0 for an int, 0.0 for a float.
pub fn imag(a: &Py) -> R {
    match a {
        Py::Complex(_, i) => Ok(Py::Float(*i)),
        Py::Bool(_) | Py::Int(_) | Py::Big(_) => Ok(Py::Int(0)),
        Py::Float(_) => Ok(Py::Float(0.0)),
        _ => Err(PyErr::Type("object has no attribute 'imag'".into())),
    }
}

/// complex(), complex(x), complex(re, im) for numbers (complex_new: re + im*1j, so a complex im turns).
pub fn complex(args: &[Py]) -> R {
    match args {
        [] => Ok(Py::Complex(0.0, 0.0)),
        [x] => cx(x).map(cplx).ok_or_else(|| type_error("complex")),
        [x, y] => {
            let (a, b) = (cx(x).ok_or_else(|| type_error("complex"))?, cx(y).ok_or_else(|| type_error("complex"))?);
            // complex(a, b) = a + b*1j with b = (b.re, b.im): real a.re - b.im, imag a.im + b.re
            let re = if matches!(y, Py::Complex(..)) { a.0 - b.1 } else { a.0 };
            let im = if matches!(x, Py::Complex(..)) { a.1 + b.0 } else { b.0 };
            Ok(Py::Complex(re, im))
        }
        _ => Err(type_error("complex")),
    }
}

/// The cmath functions the procedures use, on numbers made complex.
pub fn cmath(name: &str, args: &[Py]) -> R {
    use crate::pycx as c;
    let z = |k: usize| args.get(k).and_then(cx).ok_or_else(|| type_error(name));
    match (name, args.len()) {
        ("exp", 1) => Ok(cplx(c::exp(z(0)?)?)),
        ("sqrt", 1) => Ok(cplx(c::sqrt(z(0)?)?)),
        ("log", 1) => Ok(cplx(c::log(z(0)?)?)),
        ("log", 2) => Ok(cplx(c::quot(c::log(z(0)?)?, c::log(z(1)?)?)?)),
        ("log10", 1) => {
            let l = c::log(z(0)?)?;
            Ok(Py::Complex(l.0 / std::f64::consts::LN_10, l.1 / std::f64::consts::LN_10))
        }
        ("cosh", 1) => Ok(cplx(c::cosh(z(0)?)?)),
        ("sinh", 1) => Ok(cplx(c::sinh(z(0)?)?)),
        ("tanh", 1) => Ok(cplx(c::tanh(z(0)?)?)),
        ("cos", 1) => Ok(cplx(c::cos(z(0)?)?)),
        ("sin", 1) => Ok(cplx(c::sin(z(0)?)?)),
        ("tan", 1) => Ok(cplx(c::tan(z(0)?)?)),
        ("acosh", 1) => Ok(cplx(c::acosh(z(0)?)?)),
        ("asinh", 1) => Ok(cplx(c::asinh(z(0)?)?)),
        ("atanh", 1) => Ok(cplx(c::atanh(z(0)?)?)),
        ("acos", 1) => Ok(cplx(c::acos(z(0)?)?)),
        ("asin", 1) => Ok(cplx(c::asin(z(0)?)?)),
        ("atan", 1) => Ok(cplx(c::atan(z(0)?)?)),
        ("phase", 1) => Ok(Py::Float(c::phase(z(0)?))),
        ("polar", 1) => {
            let v = z(0)?;
            Ok(Py::tuple(vec![Py::Float(c::abs(v)?), Py::Float(c::phase(v))]))
        }
        ("rect", 2) => Ok(cplx(c::rect(args[0].f()?, args[1].f()?)?)),
        ("isfinite", 1) => {
            let v = z(0)?;
            Ok(Py::Bool(v.0.is_finite() && v.1.is_finite()))
        }
        ("isnan", 1) => {
            let v = z(0)?;
            Ok(Py::Bool(v.0.is_nan() || v.1.is_nan()))
        }
        _ => Err(PyErr::Unsupported(format!("cmath.{name}"))),
    }
}

#[cfg(test)]
mod shuffle_slice_tests {
    use super::*;

    fn ints(p: &Py) -> Vec<i64> {
        p.items().unwrap().iter().map(|x| x.i().unwrap()).collect()
    }

    #[test]
    fn random_shuffle_is_pythons() {
        // CPython: x = list(range(n)); random.Random(seed).shuffle(x)
        let cases: [(i64, i64, &[i64]); 4] = [
            (20261022, 10, &[3, 7, 6, 8, 9, 1, 0, 2, 4, 5]),
            (0, 7, &[4, 2, 1, 0, 5, 3, 6]),
            (-5, 30, &[4, 26, 13, 2, 10, 9, 29, 24, 12, 17, 28, 6, 15, 18, 21, 3, 5, 1, 7, 14, 0, 16, 20, 27, 22, 25,
                       11, 23, 8, 19]),
            (1099511627779, 12, &[10, 7, 2, 6, 9, 11, 5, 4, 1, 8, 0, 3]),
        ];
        for (seed, n, want) in cases {
            let x = Py::list((0..n).map(Py::Int).collect());
            random_shuffle(&Py::Int(seed), &x).unwrap();
            assert_eq!(ints(&x), want.to_vec(), "seed {seed}");
        }
    }

    #[test]
    fn slices_and_counter() {
        let x = Py::list((0..6).map(Py::Int).collect());
        let sl = Py::tuple(vec![Py::Int(1), Py::Int(4), Py::None]);
        assert_eq!(ints(&slice_by(&x, &sl).unwrap()), vec![1, 2, 3]);
        setslice(&x, Some(2), Some(4), &Py::list(vec![Py::Int(9)])).unwrap();
        assert_eq!(ints(&x), vec![0, 1, 9, 4, 5]);
        setslice(&x, Some(-1), None, &Py::list(vec![Py::Int(7), Py::Int(8)])).unwrap();
        assert_eq!(ints(&x), vec![0, 1, 9, 4, 7, 8]);
        let c = counter(&Py::list(vec![Py::str("a"), Py::str("b"), Py::str("a")])).unwrap();
        assert_eq!(repr(&c), "{'a': 2, 'b': 1}");
    }
}

#[cfg(test)]
mod big_int_tests {
    //! Python 3.13's own results (printed by CPython for these expressions)
    use super::*;

    fn i(n: i64) -> Py {
        Py::Int(n)
    }
    fn b(s: &str) -> Py {
        int_of(s.parse::<BigInt>().unwrap())
    }
    fn r(x: R) -> String {
        repr(&x.unwrap())
    }
    fn p2(n: u32) -> Py {
        pow(&i(2), &i(n as i64)).unwrap()
    }

    #[test]
    fn arithmetic_as_python() {
        assert_eq!(r(pow(&i(39), &i(12))), "12381557655576425121");
        assert_eq!(r(add(&p2(64), &i(5))), "18446744073709551621");
        assert_eq!(r(floordiv(&p2(64), &i(7))), "2635249153387078802");
        assert_eq!(r(floordiv(&neg(&p2(70)).unwrap(), &i(3))), "-393530540239137101142");
        assert_eq!(r(modulo(&neg(&p2(70)).unwrap(), &i(7))), "5");
        assert_eq!(r(truediv(&pow(&i(10), &i(30)).unwrap(), &i(3))), "3.333333333333333e+29");
        assert_eq!(r(sub(&mul(&p2(70), &i(3)).unwrap(), &p2(70))), "2361183241434822606848");
        assert_eq!(r(truediv(&p2(70), &add(&p2(60), &i(1)).unwrap())), "1024.0");
        // a result that fits again is an ordinary int
        assert!(matches!(sub(&add(&p2(64), &i(5)).unwrap(), &p2(64)).unwrap(), Py::Int(5)));
    }

    #[test]
    fn integer_functions_as_python() {
        assert_eq!(r(comb(&i(100), &i(50))), "100891344545564193334812497256");
        assert_eq!(r(factorial(&i(30))), "265252859812191058636308480000000");
        assert_eq!(r(perm(&i(40), &i(20))), "335367096786357081410764800000");
        assert_eq!(r(pow_mod(&i(3), &add(&p2(65), &i(1)).unwrap(), &sub(&p2(61), &i(1)).unwrap())), "5559060566555523");
        assert_eq!(r(pow_mod(&i(5), &i(-1), &b("100000000000000000039"))), "20000000000000000008");
        assert_eq!(r(isqrt(&b("10000000000000000000000000000000000012345"))), "100000000000000000000");
        assert_eq!(r(rshift(&p2(100), &i(37))), "9223372036854775808");
        assert_eq!(r(rshift(&neg(&p2(100)).unwrap(), &i(37))), "-9223372036854775808");
        assert_eq!(r(lshift(&i(1), &i(100))), "1267650600228229401496703205376");
        assert_eq!(r(gcd(&p2(80), &pow(&i(6), &i(40)).unwrap())), "1099511627776");
        assert_eq!(r(method(&p2(64), "bit_length", &[])), "65");
        assert_eq!(r(math1("floor", &p2(70))), "1180591620717411303424");
    }

    #[test]
    fn conversions_as_python() {
        assert_eq!(r(math1("log", &pow(&i(10), &i(400)).unwrap())), "921.0340371976182");
        assert_eq!(r(math1("log10", &pow(&i(10), &i(400)).unwrap())), "400.0");
        assert_eq!(r(math1("log2", &add(&p2(2000), &i(1)).unwrap())), "2000.0");
        assert_eq!(r(float(&add(&p2(80), &i(1)).unwrap())), "1.2089258196146292e+24");
        assert_eq!(r(round(&Py::Float(1e20), None)), "100000000000000000000");
        assert_eq!(r(int(&Py::Float(1.5e19))), "15000000000000000000");
        assert_eq!(r(int(&Py::str("123456789012345678901234567890"))), "123456789012345678901234567890");
        assert!(eq(&p2(80), &Py::Float(2f64.powi(80))));
        assert!(gt(&add(&p2(80), &i(1)).unwrap(), &Py::Float(2f64.powi(80))).unwrap());
        assert_eq!(format(&p2(70), ",").unwrap(), "1,180,591,620,717,411,303,424");
        assert_eq!(to_json(&pow(&i(39), &i(12)).unwrap()).to_string(), "12381557655576425121");
        assert_eq!(to_json(&p2(100)).to_string(), "1267650600228229401496703205376");
    }

    #[test]
    fn heapq_and_itertools_as_python() {
        let t = |k: i64, s: &str| Py::tuple(vec![i(k), Py::str(s)]);
        let h = Py::list(vec![]);
        for x in [t(3, "c"), t(1, "a"), t(2, "b"), t(1, "z"), t(0, "q")] {
            heapq("heappush", &[h.clone(), x]).unwrap();
        }
        assert_eq!(repr(&h), "[(0, 'q'), (1, 'a'), (2, 'b'), (3, 'c'), (1, 'z')]");
        assert_eq!(r(heapq("heappop", &[h.clone()])), "(0, 'q')");
        assert_eq!(repr(&h), "[(1, 'a'), (1, 'z'), (2, 'b'), (3, 'c')]");
        let x = Py::list([5, 3, 8, 1, 9, 2, 7].iter().map(|&k| i(k)).collect());
        heapq("heapify", &[x.clone()]).unwrap();
        assert_eq!(repr(&x), "[1, 3, 2, 5, 9, 8, 7]");
        assert_eq!(r(heapq("heapreplace", &[x.clone(), i(6)])), "1");
        assert_eq!(repr(&x), "[2, 3, 6, 5, 9, 8, 7]");
        assert_eq!(r(heapq("heappushpop", &[x.clone(), i(0)])), "0");
        let s = |v: &str| Py::str(v);
        assert_eq!(r(itertools("combinations", &[s("abcd"), i(2)])),
                   "[('a', 'b'), ('a', 'c'), ('a', 'd'), ('b', 'c'), ('b', 'd'), ('c', 'd')]");
        assert_eq!(r(itertools("permutations", &[Py::list(vec![i(1), i(2), i(3)])])),
                   "[(1, 2, 3), (1, 3, 2), (2, 1, 3), (2, 3, 1), (3, 1, 2), (3, 2, 1)]");
        assert_eq!(r(itertools("permutations", &[s("abc"), i(2)])),
                   "[('a', 'b'), ('a', 'c'), ('b', 'a'), ('b', 'c'), ('c', 'a'), ('c', 'b')]");
        assert_eq!(r(itertools("product", &[s("ab"), Py::list(vec![i(1), i(2)])])),
                   "[('a', 1), ('a', 2), ('b', 1), ('b', 2)]");
    }

    #[test]
    fn bases_as_python() {
        let s = |x: &str| Py::str(x);
        assert_eq!(r(int_base(&s("ff"), &i(16))), "255");
        assert_eq!(r(int_base(&s("0x1F"), &i(16))), "31");
        assert_eq!(r(int_base(&s("0x_1f"), &i(0))), "31");
        assert_eq!(r(int_base(&s(" -101 "), &i(2))), "-5");
        assert_eq!(r(int_base(&s("0o17"), &i(0))), "15");
        assert_eq!(r(int_base(&s("z"), &i(36))), "35");
        assert_eq!(r(int_base(&s("1_000"), &i(0))), "1000");
        let msg = |x: &str, b: i64| match int_base(&s(x), &i(b)) { Err(PyErr::Value(m)) => m, o => format!("{o:?}") };
        assert_eq!(msg("010", 0), "invalid literal for int() with base 0: '010'");
        assert_eq!(msg("12", 2), "invalid literal for int() with base 2: '12'");
        assert_eq!(msg("0b", 2), "invalid literal for int() with base 2: '0b'");
        assert_eq!(msg("_1", 10), "invalid literal for int() with base 10: '_1'");
        assert_eq!(msg("1__0", 10), "invalid literal for int() with base 10: '1__0'");
        assert_eq!(r(int_text_base(&i(5), "bin")), "'0b101'");
        assert_eq!(r(int_text_base(&i(-5), "bin")), "'-0b101'");
        assert_eq!(r(int_text_base(&i(64), "oct")), "'0o100'");
        assert_eq!(r(int_text_base(&i(-255), "hex")), "'-0xff'");
        assert_eq!(r(int_text_base(&p2(70), "hex")), "'0x400000000000000000'");
    }

    #[test]
    fn small_builtins_as_python() {
        assert_eq!(r(gcd_lcm("lcm", &[i(4), i(6), i(-10)])), "60");
        assert_eq!(r(gcd_lcm("gcd", &[i(12), i(-18), i(30)])), "6");
        assert_eq!(r(gcd_lcm("lcm", &[])), "1");
        assert_eq!(r(gcd_lcm("gcd", &[])), "0");
        assert_eq!(r(gcd_lcm("lcm", &[i(0), i(5)])), "0");
        assert_eq!(r(invert(&i(5))), "-6");
        assert_eq!(r(invert(&i(-1))), "0");
        assert_eq!(r(invert(&Py::Bool(true))), "-2");
        let d = Py::dict(vec![(Py::str("a"), i(1)), (Py::str("b"), i(2)), (Py::str("c"), i(3))]);
        delitem(&d, &Py::str("b")).unwrap();
        assert_eq!(repr(&d), "{'a': 1, 'c': 3}");
        let l = Py::list(vec![i(1), i(2), i(3)]);
        delitem(&l, &i(-1)).unwrap();
        assert_eq!(repr(&l), "[1, 2]");
        let s = set(&Py::list(vec![i(1), i(2)])).unwrap();
        method(&s, "add", &[i(2)]).unwrap();
        method(&s, "add", &[i(3)]).unwrap();
        method(&s, "discard", &[i(1)]).unwrap();
        method(&s, "discard", &[i(9)]).unwrap();
        assert_eq!(repr(&s), "{2, 3}");
    }

    #[test]
    fn sets_compare_as_python_sets() {
        let s = |v: Vec<Py>| set(&Py::list(v)).unwrap();
        let (a, b) = (s(vec![Py::str("u"), Py::str("v")]), s(vec![Py::str("v"), Py::str("u")]));
        assert!(eq(&a, &b));                                          // {u, v} == {v, u}
        assert!(!eq(&a, &Py::list(vec![Py::str("u"), Py::str("v")])));  // a set is never equal to a list
        assert!(le(&s(vec![i(2)]), &s(vec![i(1), i(2)])).unwrap());     // subset, not lexicographic
        assert!(!le(&s(vec![i(3)]), &s(vec![i(1), i(2)])).unwrap());
        assert!(lt(&s(vec![i(2)]), &s(vec![i(2), i(1)])).unwrap());
        assert!(!lt(&a, &b).unwrap());
        assert!(ge(&s(vec![i(1), i(2), i(3)]), &s(vec![i(3), i(1)])).unwrap());
        let u = method(&s(vec![]), "union", &[Py::list(vec![i(1), i(2)]), s(vec![i(2), i(3)])]).unwrap();
        assert!(eq(&u, &s(vec![i(3), i(2), i(1)])));
        // frozenset keys: found whatever order they were built in
        let d = Py::dict(vec![(a.clone(), i(7))]);
        assert!(eq(&getitem(&d, &b).unwrap(), &i(7)));
        // dict.pop with and without a default
        let d = Py::dict(vec![(Py::str("x"), i(1)), (Py::str("y"), i(2))]);
        assert!(eq(&method(&d, "pop", &[Py::str("x")]).unwrap(), &i(1)));
        assert!(eq(&method(&d, "pop", &[Py::str("x"), i(9)]).unwrap(), &i(9)));
        assert!(method(&d, "pop", &[Py::str("x")]).is_err());
        assert_eq!(len(&d).unwrap().i().unwrap(), 1);
    }

    #[test]
    fn a_for_loop_sees_items_its_body_appends() {
        let l = Py::list(vec![i(1)]);
        let mut seen = vec![];
        for x in iter(&l).unwrap() {
            let k = x.i().unwrap();
            seen.push(k);
            if k < 4 {
                method(&l, "append", &[i(k + 1)]).unwrap();
            }
        }
        assert_eq!(seen, vec![1, 2, 3, 4]);
    }

    #[test]
    fn dict_answers_keep_python_order() {
        let d = Py::dict(vec![(Py::str("S"), i(0)), (Py::str("B"), i(2)), (Py::str("A"), i(5))]);
        assert_eq!(to_json(&d).to_string(), r#"{"S":0,"B":2,"A":5}"#);  // not sorted (serde_json preserve_order)
    }

    #[test]
    fn json_decimals_read_exactly() {
        // serde_json's default parser read these 1 ulp off; float_roundtrip (Cargo.toml) reads them as Python does
        for t in ["3.8000000000000003", "0.30000000000000004", "6.800000000000001", "2.2250738585072014e-308"] {
            let a: f64 = serde_json::from_str(t).unwrap();
            let v: serde_json::Value = serde_json::from_str(t).unwrap();
            assert_eq!(a.to_bits(), t.parse::<f64>().unwrap().to_bits(), "{t}");
            assert_eq!(v.as_f64().unwrap().to_bits(), t.parse::<f64>().unwrap().to_bits(), "{t}");
        }
    }

    #[test]
    fn digit_limit_as_python() {
        let msg = |e: PyErr| match e { PyErr::Value(m) => m, other => format!("{other:?}") };
        assert_eq!(msg(int(&Py::str(&"1".repeat(5000))).unwrap_err()),
                   "Exceeds the limit (4300 digits) for integer string conversion: value has 5000 digits; use \
                    sys.set_int_max_str_digits() to increase the limit");
        let huge = pow(&i(10), &i(5000)).unwrap();
        assert_eq!(msg(format(&huge, ",").unwrap_err()),
                   "Exceeds the limit (4300 digits) for integer string conversion; use sys.set_int_max_str_digits() \
                    to increase the limit");
        assert_eq!(format(&pow(&i(10), &i(4299)).unwrap(), "").unwrap().len(), 4300);
        assert!(format(&pow(&i(10), &i(4300)).unwrap(), "").is_err());
        assert!(matches!(pow_mod(&p2(70), &pow(&i(10), &i(4000)).unwrap(), &pow(&i(10), &i(3000)).unwrap()),
                         Err(PyErr::Unsupported(_))));
    }
}

