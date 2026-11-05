//! Dimensional algebra, ported from runtime/quantity.py: compound units, SI prefixes, angles and affine
//! temperatures. A unit is a factor to SI and a dimension over (kg, m, s, A, K, mol, cd, rad).

#[allow(unused_imports)]
use crate::lm::{Lm, LmF64};
use crate::num::Rat;
use std::f64::consts::PI;

pub const BASES: [&str; 8] = ["kg", "m", "s", "A", "K", "mol", "cd", "rad"];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Dim(pub [Rat; 8]);

impl Dim {
    pub fn one() -> Dim {
        Dim([Rat::zero(); 8])
    }
    fn of(e: [i64; 8]) -> Dim {
        let mut d = Dim::one();
        for (i, x) in e.iter().enumerate() {
            d.0[i] = Rat::int(*x);
        }
        d
    }
    pub fn mul(self, o: Dim) -> Dim {
        let mut d = self;
        for i in 0..8 {
            d.0[i] = self.0[i].add(o.0[i]);
        }
        d
    }
    pub fn div(self, o: Dim) -> Dim {
        let mut d = self;
        for i in 0..8 {
            d.0[i] = self.0[i].sub(o.0[i]);
        }
        d
    }
    pub fn pow(self, n: Rat) -> Dim {
        let mut d = self;
        for i in 0..8 {
            d.0[i] = self.0[i].mul(n);
        }
        d
    }
    /// From the exported exponents ("1", "-2", "1/3").
    pub fn parse(exps: &[String]) -> Option<Dim> {
        let mut d = Dim::one();
        for (i, s) in exps.iter().enumerate().take(8) {
            d.0[i] = Rat::parse(s)?;
        }
        Some(d)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Unit {
    pub factor: f64,
    pub dim: Dim,
    pub offset: f64,
}

#[derive(Debug)]
pub struct UnitError(pub String);

// [kg, m, s, A, K, mol, cd, rad]
const N: [i64; 8] = [1, 1, -2, 0, 0, 0, 0, 0];
const J: [i64; 8] = [1, 2, -2, 0, 0, 0, 0, 0];
const W: [i64; 8] = [1, 2, -3, 0, 0, 0, 0, 0];
const V: [i64; 8] = [1, 2, -3, -1, 0, 0, 0, 0];
const PA: [i64; 8] = [-0, -1, -2, 0, 0, 0, 0, 0];

fn pa() -> [i64; 8] {
    let mut p = PA;
    p[0] = 1;
    p
}

/// spelling -> (factor, dimension, takes SI prefixes)
fn atom_table(name: &str) -> Option<(f64, [i64; 8], bool)> {
    let z = [0i64; 8];
    let b = |i: usize, e: i64| {
        let mut d = z;
        d[i] = e;
        d
    };
    Some(match name {
        "kg" => (1.0, b(0, 1), false),
        "g" => (1e-3, b(0, 1), true),
        "t" => (1e3, b(0, 1), false),
        "m" => (1.0, b(1, 1), true),
        "s" => (1.0, b(2, 1), true),
        "A" => (1.0, b(3, 1), true),
        "K" => (1.0, b(4, 1), true),
        "mol" => (1.0, b(5, 1), true),
        "cd" => (1.0, b(6, 1), false),
        "rad" => (1.0, b(7, 1), true),
        "min" => (60.0, b(2, 1), false),
        "h" | "hr" => (3600.0, b(2, 1), false),
        "day" => (86400.0, b(2, 1), false),
        "N" => (1.0, N, true),
        "J" => (1.0, J, true),
        "W" => (1.0, W, true),
        "Pa" => (1.0, pa(), true),
        "atm" => (101325.0, pa(), false),
        "bar" => (1e5, pa(), true),
        "Hz" => (1.0, b(2, -1), true),
        "C" => (1.0, [0, 0, 1, 1, 0, 0, 0, 0], true),
        "V" => (1.0, V, true),
        "ohm" | "Ω" => (1.0, [1, 2, -3, -2, 0, 0, 0, 0], true),
        "F" => (1.0, [-1, -2, 4, 2, 0, 0, 0, 0], true),
        "T" => (1.0, [1, 0, -2, -1, 0, 0, 0, 0], true),
        "Wb" => (1.0, [1, 2, -2, -1, 0, 0, 0, 0], true),
        "H" => (1.0, [1, 2, -2, -2, 0, 0, 0, 0], true),
        "eV" => (1.602176634e-19, J, true),
        "L" => (1e-3, b(1, 3), true),
        "Wh" => (3600.0, J, true),
        "Nm" => (1.0, J, false),
        "ft" => (0.3048, b(1, 1), false),
        "mi" => (1609.344, b(1, 1), false),
        "yd" => (0.9144, b(1, 1), false),
        "mph" => (0.44704, [0, 1, -1, 0, 0, 0, 0, 0], false),
        "mmHg" => (133.322387415, pa(), false),
        "cal" => (4.184, J, true),
        "Cal" => (4184.0, J, false),
        "hp" => (745.69987158227022, W, false),
        "yr" => (3.15576e7, b(2, 1), false),
        "rpm" => (2.0 * PI / 60.0, [0, 0, -1, 0, 0, 0, 0, 1], false),
        "%" => (0.01, z, false),
        "VA" | "var" | "VAr" => (1.0, W, true),
        "S" => (1.0, [-1, -2, 3, 2, 0, 0, 0, 0], true),
        "mho" => (1.0, [-1, -2, 3, 2, 0, 0, 0, 0], false),
        _ => return None,
    })
}

fn prefix(c: char) -> Option<f64> {
    Some(match c {
        'k' => 1e3,
        'M' => 1e6,
        'G' => 1e9,
        'm' => 1e-3,
        'c' => 1e-2,
        'μ' | 'µ' | 'u' => 1e-6,
        'n' => 1e-9,
        'p' => 1e-12,
        'f' => 1e-15,
        _ => return None,
    })
}

fn atom(name: &str) -> Result<Unit, UnitError> {
    if let Some((f, d, _)) = atom_table(name) {
        return Ok(Unit { factor: f, dim: Dim::of(d), offset: 0.0 });
    }
    if name == "°C" || name == "degC" {
        return Ok(Unit { factor: 1.0, dim: Dim::of([0, 0, 0, 0, 1, 0, 0, 0]), offset: 273.15 });
    }
    let pi_frac = match name {
        "deg" | "°" => Some(1.0 / 180.0),
        "rev" => Some(2.0),
        _ => None,
    };
    if let Some(k) = pi_frac {
        return Ok(Unit { factor: PI * k, dim: Dim::of([0, 0, 0, 0, 0, 0, 0, 1]), offset: 0.0 });
    }
    let mut chars = name.chars();
    if let Some(c) = chars.next() {
        let rest: String = chars.collect();
        if name.chars().count() > 1 {
            if let (Some(p), Some((f, d, true))) = (prefix(c), atom_table(&rest)) {
                return Ok(Unit { factor: f * p, dim: Dim::of(d), offset: 0.0 });
            }
        }
    }
    Err(UnitError(format!("unknown unit: {name}")))
}

fn is_letter(c: char) -> bool {
    c.is_ascii_alphabetic() || matches!(c, 'μ' | 'µ' | 'Ω' | '°')
}

/// The tokens of runtime/quantity.py's _TOKEN, or None when the text is not entirely a unit.
fn tokenize(src: &str) -> Option<Vec<String>> {
    let cs: Vec<char> = src.chars().collect();
    let mut i = 0;
    let mut out = Vec::new();
    while i < cs.len() {
        while i < cs.len() && cs[i].is_whitespace() {
            i += 1;
        }
        if i >= cs.len() {
            return None; // trailing whitespace alone does not match _TOKEN
        }
        let c = cs[i];
        if c == '^' {
            // \^\(\s*-?\d+(?:\s*/\s*\d+)?\s*\)   or   \^-?\d+(?:\.\d+)?
            let mut j = i + 1;
            if j < cs.len() && cs[j] == '(' {
                j += 1;
                let skip_ws = |j: &mut usize| {
                    while *j < cs.len() && cs[*j].is_whitespace() {
                        *j += 1;
                    }
                };
                skip_ws(&mut j);
                if j < cs.len() && cs[j] == '-' {
                    j += 1;
                }
                let d0 = j;
                while j < cs.len() && cs[j].is_ascii_digit() {
                    j += 1;
                }
                if j == d0 {
                    return None;
                }
                let save = j;
                skip_ws(&mut j);
                if j < cs.len() && cs[j] == '/' {
                    j += 1;
                    skip_ws(&mut j);
                    let d1 = j;
                    while j < cs.len() && cs[j].is_ascii_digit() {
                        j += 1;
                    }
                    if j == d1 {
                        j = save;
                    }
                } else {
                    j = save;
                }
                skip_ws(&mut j);
                if j < cs.len() && cs[j] == ')' {
                    j += 1;
                    out.push(cs[i..j].iter().collect());
                    i = j;
                    continue;
                }
                return None;
            }
            if j < cs.len() && cs[j] == '-' {
                j += 1;
            }
            let d0 = j;
            while j < cs.len() && cs[j].is_ascii_digit() {
                j += 1;
            }
            if j == d0 {
                return None;
            }
            if j + 1 < cs.len() && cs[j] == '.' && cs[j + 1].is_ascii_digit() {
                j += 1;
                while j < cs.len() && cs[j].is_ascii_digit() {
                    j += 1;
                }
            }
            out.push(cs[i..j].iter().collect());
            i = j;
            continue;
        }
        if matches!(c, '*' | '/' | '(' | ')') {
            out.push(c.to_string());
            i += 1;
            continue;
        }
        if c == '1' {
            let next = cs.get(i + 1).copied();
            let word = next.map(|n| n.is_alphanumeric() || n == '_' || n == '.').unwrap_or(false);
            if !word {
                out.push("1".into());
                i += 1;
                continue;
            }
            return None;
        }
        if is_letter(c) {
            let mut j = i;
            while j < cs.len() && is_letter(cs[j]) {
                j += 1;
            }
            out.push(cs[i..j].iter().collect());
            i = j;
            continue;
        }
        if c == '%' {
            out.push("%".into());
            i += 1;
            continue;
        }
        return None;
    }
    Some(out)
}

pub fn parse_unit(text: &str) -> Result<Unit, UnitError> {
    let src: String = text
        .chars()
        .map(|c| match c {
            '²' => "^2".to_string(),
            '³' => "^3".to_string(),
            '·' | '⋅' => "*".to_string(),
            '−' => "-".to_string(),
            c => c.to_string(),
        })
        .collect::<String>()
        .replace("**", "^");
    let src = src.trim();
    if src.is_empty() || src.chars().count() > 60 {
        return Err(UnitError("empty or oversized unit".into()));
    }
    let toks = tokenize(src).ok_or_else(|| UnitError(format!("not a unit: {text}")))?;
    let mut i = 0usize;
    let u = expr(&toks, &mut i, text)?;
    if i != toks.len() {
        return Err(UnitError(format!("not a unit: {text}")));
    }
    Ok(u)
}

fn factor(t: &[String], i: &mut usize, text: &str) -> Result<Unit, UnitError> {
    if *i >= t.len() {
        return Err(UnitError(format!("not a unit: {text}")));
    }
    let tok = t[*i].clone();
    *i += 1;
    let mut u = if tok == "(" {
        let u = expr(t, i, text)?;
        if *i >= t.len() || t[*i] != ")" {
            return Err(UnitError("unbalanced parenthesis".into()));
        }
        *i += 1;
        u
    } else if tok == "1" {
        Unit { factor: 1.0, dim: Dim::one(), offset: 0.0 }
    } else if tok.starts_with(['^', '*', '/', ')']) {
        return Err(UnitError(format!("not a unit: {text}")));
    } else {
        atom(&tok)?
    };
    if *i < t.len() && t[*i].starts_with('^') {
        let raw: String = t[*i][1..].trim_matches(|c| c == '(' || c == ')' || c == ' ').replace(' ', "");
        let n = Rat::parse(&raw).ok_or_else(|| UnitError(format!("not a unit: {text}")))?;
        *i += 1;
        if u.offset != 0.0 {
            return Err(UnitError("an offset unit cannot be raised to a power".into()));
        }
        u = Unit { factor: u.factor.powf_lm(n.to_f64()), dim: u.dim.pow(n), offset: 0.0 };
    }
    Ok(u)
}

fn expr(t: &[String], i: &mut usize, text: &str) -> Result<Unit, UnitError> {
    let mut u = factor(t, i, text)?;
    while *i < t.len() && (t[*i] == "*" || t[*i] == "/") {
        let op = t[*i].clone();
        *i += 1;
        let v = factor(t, i, text)?;
        if u.offset != 0.0 || v.offset != 0.0 {
            return Err(UnitError("an offset unit cannot be combined with another unit".into()));
        }
        u = if op == "*" {
            Unit { factor: u.factor * v.factor, dim: u.dim.mul(v.dim), offset: 0.0 }
        } else {
            Unit { factor: u.factor / v.factor, dim: u.dim.div(v.dim), offset: 0.0 }
        };
    }
    Ok(u)
}

/// A number in `unit` to SI; a temperature DIFFERENCE ignores the offset.
pub fn to_si(value: f64, unit: &Unit, difference: bool) -> f64 {
    value * unit.factor + if difference { 0.0 } else { unit.offset }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn units() {
        let u = parse_unit("km/h").unwrap();
        assert!((u.factor - 1000.0 / 3600.0).abs() < 1e-15);
        assert_eq!(u.dim, Dim::of([0, 1, -1, 0, 0, 0, 0, 0]));
        assert!(parse_unit("kg*m^2").is_ok());
        assert!(parse_unit("s/m^(1/3)").is_ok());
        assert!(parse_unit("J/(kg*K)").is_ok());
        assert_eq!(parse_unit("°C").unwrap().offset, 273.15);
        assert!(parse_unit("parsec").is_err());
        assert!(parse_unit("°C*m").is_err());
        assert!(parse_unit("mol/m^3").is_ok());
        assert!((parse_unit("deg").unwrap().factor - PI / 180.0).abs() < 1e-18);
    }
}
