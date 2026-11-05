//! Discrete math, written by hand (see handwritten/mod.rs): the primes (a byte sieve, the package's bytearray).

use super::arg;
use crate::py::{Py, PyErr, R};

pub fn helper(name: &str, a: &[Py], _kws: &[(String, Py)]) -> Option<R> {
    match name {
        "prime_counting" | "nth_prime" | "_odd_sieve_count" => Some(run(name, a)),
        "_dfs" => Some(dfs(arg(a, 0), arg(a, 1))),
        _ => None,
    }
}

/// _dfs(adj, s): depth-first preorder from s, neighbours visited in _sorted order (the package keeps one iterator per
/// stack level and resumes it; an index per level is the same walk)
fn dfs(adj: &Py, s: &Py) -> R {
    let nbrs = |v: &Py| -> Result<Vec<Py>, PyErr> {
        let row = crate::py::getitem(adj, v)?;
        let mut keys: Vec<Py> = match &row {
            Py::Dict(d) => d.borrow().iter().map(|(k, _)| k.clone()).collect(),
            other => other.items()?,
        };
        let mut err = None;
        keys.sort_by(|x, y| {
            nkey_cmp(&crate::py::to_str(x), &crate::py::to_str(y)).unwrap_or_else(|e| {
                err = Some(e);
                std::cmp::Ordering::Equal
            })
        });
        match err {
            Some(e) => Err(e),
            None => Ok(keys),
        }
    };
    let mut seen = vec![s.clone()];
    let mut order = vec![s.clone()];
    let mut stack: Vec<(Vec<Py>, usize)> = vec![(nbrs(s)?, 0)];
    while let Some((items, pos)) = stack.last_mut() {
        let mut next = None;
        while *pos < items.len() {
            let v = items[*pos].clone();
            *pos += 1;
            if !seen.iter().any(|x| crate::py::eq(x, &v)) {
                next = Some(v);
                break;
            }
        }
        match next {
            Some(v) => {
                seen.push(v.clone());
                order.push(v.clone());
                let n = nbrs(&v)?;
                stack.push((n, 0));
            }
            None => {
                stack.pop();
            }
        }
    }
    Ok(Py::list(order))
}

/// discrete_math's _nkey order: whole numbers (-?digits) by value first, then other names as text
fn nkey_cmp(a: &str, b: &str) -> Result<std::cmp::Ordering, PyErr> {
    use std::cmp::Ordering;
    let num = |s: &str| -> Result<Option<(bool, String)>, PyErr> {
        let d = s.strip_prefix('-').unwrap_or(s);
        if !d.is_empty() && d.chars().all(|c| c.is_ascii_digit()) {
            let mag = d.trim_start_matches('0').to_string();
            return Ok(Some((s.starts_with('-') && !mag.is_empty(), mag)));
        }
        if d.chars().any(|c| c.is_numeric() && !c.is_ascii_digit()) {
            // Python's \d and int() also take other scripts' digits: not modelled here
            return Err(PyErr::Unsupported("_nkey: non-ASCII digits".into()));
        }
        Ok(None)
    };
    Ok(match (num(a)?, num(b)?) {
        (Some((na, ma)), Some((nb, mb))) => {
            let mag = ma.len().cmp(&mb.len()).then_with(|| ma.cmp(&mb));
            match (na, nb) {
                (false, false) => mag,
                (true, true) => mag.reverse(),
                (true, false) => Ordering::Less,
                (false, true) => Ordering::Greater,
            }
        }
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => a.cmp(b),
    })
}

fn run(name: &str, a: &[Py]) -> R {
    match ("discrete_math", name) {
        ("discrete_math", "prime_counting") => {
            let x = whole(arg(a, 0), "x", Some(0), Some(5_000_000))?;
            let count = if x >= 2 { sieve(x as usize).iter().filter(|&&b| b).count() } else { 0 };
            Ok(Py::dict(vec![(Py::str("count"), Py::Int(count as i64))]))
        }
        ("discrete_math", "nth_prime") => {
            let n = whole(arg(a, 0), "n", Some(1), Some(300_000))?;
            let nf = n as f64;
            let lim = if n < 6 { 15 } else { (nf * (nf.ln() + nf.ln().ln())) as usize + 3 };
            let s = sieve(lim);
            let mut c = 0;
            for (i, &p) in s.iter().enumerate() {
                if p {
                    c += 1;
                    if c == n {
                        return Ok(Py::dict(vec![(Py::str("prime"), Py::Int(i as i64))]));
                    }
                }
            }
            Err(PyErr::Value("internal bound too small".into()))
        }
        ("discrete_math", "_odd_sieve_count") => {
            // the verifiers' second count of the primes up to x (an odd-only sieve in the package)
            let x = match arg(a, 0) {
                Py::Int(i) => *i,
                other => other.f()? as i64,
            };
            let count = if x < 2 { 0 } else { sieve(x as usize).iter().filter(|&&b| b).count() };
            Ok(Py::Int(count as i64))
        }
        _ => unreachable!(),
    }
}

/// Sieve of Eratosthenes on 0..=n: one byte per number (the Python's bytearray)
fn sieve(n: usize) -> Vec<bool> {
    let mut s = vec![true; n + 1];
    for v in s.iter_mut().take(2.min(n + 1)) {
        *v = false;
    }
    let mut i = 2;
    while i * i <= n {
        if s[i] {
            let mut j = i * i;
            while j <= n {
                s[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    s
}

/// discrete_math's _whole(v, name, lo, hi): a whole number, with the package's own refusal wording
fn whole(v: &Py, name: &str, lo: Option<i64>, hi: Option<i64>) -> Result<i64, PyErr> {
    let bad = || PyErr::Value(format!("{name} must be a whole number"));
    let n: i64 = match v {
        Py::Bool(_) => return Err(bad()),
        Py::Float(x) => {
            if !x.is_finite() || *x != x.trunc() {
                return Err(bad());
            }
            if x.abs() >= 9.2e18 {
                return Err(PyErr::Value(format!("{name} must be at most {} in size", hi.unwrap_or(i64::MAX))));
            }
            *x as i64
        }
        Py::Int(i) => *i,
        Py::Big(_) => {
            return Err(PyErr::Value(format!("{name} must be at most {} in size", hi.unwrap_or(i64::MAX))));
        }
        other => {
            let t = crate::py::to_str(other);
            let t = t.trim();
            let digits = t.strip_prefix(['+', '-']).unwrap_or(t);
            if digits.is_empty() || digits.starts_with('_') || digits.ends_with('_') || digits.contains("__")
                || !digits.chars().all(|c| c.is_ascii_digit() || c == '_')
            {
                return Err(bad());
            }
            match t.replace('_', "").parse::<i64>() {
                Ok(i) => i,
                Err(_) => {
                    return Err(PyErr::Value(format!("{name} must be at most {} in size", hi.unwrap_or(i64::MAX))))
                }
            }
        }
    };
    if let Some(lo) = lo {
        if n < lo {
            return Err(PyErr::Value(format!("{name} must be at least {lo}")));
        }
    }
    if let Some(hi) = hi {
        if n.unsigned_abs() > hi as u64 {
            return Err(PyErr::Value(format!("{name} must be at most {hi} in size")));
        }
    }
    Ok(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn val(r: R, key: &str) -> i64 {
        match r.unwrap() {
            Py::Dict(d) => match d.borrow().iter().find(|(k, _)| crate::py::to_str(k) == key).unwrap().1 {
                Py::Int(i) => i,
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    #[test]
    fn dfs_order() {
        // the package: _dfs(adj, "1") with numeric names by value, other names after them as text
        let row = |ks: &[&str]| Py::dict(ks.iter().map(|k| (Py::str(k), Py::Float(1.0))).collect());
        let adj = Py::dict(vec![
            (Py::str("1"), row(&["B", "10", "2", "-3"])),
            (Py::str("10"), row(&["1"])),
            (Py::str("2"), row(&["1", "B"])),
            (Py::str("B"), row(&["1", "2"])),
            (Py::str("-3"), row(&["1"])),
        ]);
        let got = helper("_dfs", &[adj, Py::str("1")], &[]).unwrap().unwrap();
        assert_eq!(crate::py::repr(&got), "['1', '-3', '2', 'B', '10']");
    }

    #[test]
    fn primes() {
        assert_eq!(val(helper("prime_counting", &[Py::Int(100)], &[]).unwrap(), "count"), 25);
        assert_eq!(val(helper("prime_counting", &[Py::Int(1)], &[]).unwrap(), "count"), 0);
        assert_eq!(val(helper("prime_counting", &[Py::Int(1_000_000)], &[]).unwrap(), "count"), 78498);
        assert_eq!(val(helper("nth_prime", &[Py::Int(1)], &[]).unwrap(), "prime"), 2);
        assert_eq!(val(helper("nth_prime", &[Py::Int(10)], &[]).unwrap(), "prime"), 29);
        assert_eq!(val(helper("nth_prime", &[Py::Int(300000)], &[]).unwrap(), "prime"), 4256233);
        let e = helper("nth_prime", &[Py::Int(0)], &[]).unwrap().unwrap_err();
        assert!(matches!(e, PyErr::Value(ref m) if m == "n must be at least 1"));
        let e = helper("nth_prime", &[Py::Float(2.5)], &[]).unwrap().unwrap_err();
        assert!(matches!(e, PyErr::Value(ref m) if m == "n must be a whole number"));
    }
}
