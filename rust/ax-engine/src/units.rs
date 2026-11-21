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
        // as runtime/quantity.py (2026-11-13): mM to pM (no bare "M": "M/S" stays refused), the day as "d", the barn,
        // the gray, the sievert
        "mM" => (1.0, [0, -3, 0, 0, 0, 1, 0, 0], false),
        "uM" | "µM" | "μM" => (1e-3, [0, -3, 0, 0, 0, 1, 0, 0], false),
        "nM" => (1e-6, [0, -3, 0, 0, 0, 1, 0, 0], false),
        "pM" => (1e-9, [0, -3, 0, 0, 0, 1, 0, 0], false),
        "d" => (86400.0, b(2, 1), false),
        "b" => (1e-28, b(1, 2), false),
        "Gy" | "Sv" => (1.0, [0, 2, -2, 0, 0, 0, 0, 0], true),
        // astronomy (2026-11-21: "0.768 arcsec", "10 pc", "4.24 ly" were refused although the inputs' hints list them):
        // the astronomical unit and the light-year are exact by definition (IAU 2012, Julian year); the parsec is
        // 648000/pi au and takes prefixes (kpc, Mpc, Gpc)
        "au" | "AU" => (149597870700.0, b(1, 1), false),
        "ly" => (9460730472580800.0, b(1, 1), false),
        "pc" => (149597870700.0 * 648000.0 / PI, b(1, 1), true),
        // the hectare (2026-11-21, with the hecto prefix), as runtime/quantity.py
        "ha" => (1e4, b(1, 2), false),
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
        // hecto (2026-11-21: "700 hPa" was refused); an atom is looked up first, so "h" stays the hour, "hr" the hour
        // and "hp" the horsepower. deca ("da") is two letters: see `atom`
        'h' => 1e2,
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
        // arc minutes, arc seconds and milliarcseconds (parallax): exact fractions of pi
        "arcmin" => Some(1.0 / 10800.0),
        "arcsec" => Some(1.0 / 648000.0),
        "mas" => Some(1.0 / 648000000.0),
        _ => None,
    };
    if let Some(k) = pi_frac {
        return Ok(Unit { factor: PI * k, dim: Dim::of([0, 0, 0, 0, 0, 0, 0, 1]), offset: 0.0 });
    }
    if let Some(rest) = name.strip_prefix("da") {
        if let Some((f, d, true)) = atom_table(rest) {
            return Ok(Unit { factor: f * 10.0, dim: Dim::of(d), offset: 0.0 });
        }
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

/// A spelled unit word (lower case) as runtime/quantity.py `_WORDS` spells it ("hours" -> "h", "per" -> "/"); the
/// table is generated from quantity.py and must stay equal to it (tests/test_unit_words_parity.py).
pub fn unit_word(w: &str) -> Option<&'static str> {
    Some(match w {
        "second" => "s",
        "seconds" => "s",
        "sec" => "s",
        "secs" => "s",
        "millisecond" => "ms",
        "milliseconds" => "ms",
        "minute" => "min",
        "minutes" => "min",
        "hour" => "h",
        "hours" => "h",
        "days" => "day",
        "meter" => "m",
        "meters" => "m",
        "metre" => "m",
        "metres" => "m",
        "kilometer" => "km",
        "kilometers" => "km",
        "kilometre" => "km",
        "kilometres" => "km",
        "centimeter" => "cm",
        "centimeters" => "cm",
        "centimetre" => "cm",
        "centimetres" => "cm",
        "millimeter" => "mm",
        "millimeters" => "mm",
        "millimetre" => "mm",
        "millimetres" => "mm",
        "gram" => "g",
        "grams" => "g",
        "kilogram" => "kg",
        "kilograms" => "kg",
        "tonne" => "t",
        "tonnes" => "t",
        "newton" => "N",
        "newtons" => "N",
        "kilonewton" => "kN",
        "kilonewtons" => "kN",
        "joule" => "J",
        "joules" => "J",
        "kilojoule" => "kJ",
        "kilojoules" => "kJ",
        "watt" => "W",
        "watts" => "W",
        "kilowatt" => "kW",
        "kilowatts" => "kW",
        "volt" => "V",
        "volts" => "V",
        "amp" => "A",
        "amps" => "A",
        "ampere" => "A",
        "amperes" => "A",
        "ohms" => "ohm",
        "hertz" => "Hz",
        "kilohertz" => "kHz",
        "pascal" => "Pa",
        "pascals" => "Pa",
        "kilopascals" => "kPa",
        "coulomb" => "C",
        "coulombs" => "C",
        "kelvin" => "K",
        "kelvins" => "K",
        "mole" => "mol",
        "moles" => "mol",
        "turn" => "1",
        "turns" => "1",
        "tesla" => "T",
        "teslas" => "T",
        "farad" => "F",
        "farads" => "F",
        "henry" => "H",
        "weber" => "Wb",
        "degree" => "deg",
        "degrees" => "deg",
        "radian" => "rad",
        "radians" => "rad",
        "revolution" => "rev",
        "revolutions" => "rev",
        "liter" => "L",
        "liters" => "L",
        "litre" => "L",
        "litres" => "L",
        "per" => "/",
        "squared" => "^2",
        "cubed" => "^3",
        "celsius" => "°C",
        "foot" => "ft",
        "feet" => "ft",
        "mile" => "mi",
        "miles" => "mi",
        "yard" => "yd",
        "yards" => "yd",
        "calorie" => "cal",
        "calories" => "cal",
        "kilocalorie" => "kcal",
        "kilocalories" => "kcal",
        "horsepower" => "hp",
        "year" => "yr",
        "years" => "yr",
        "milliliter" => "mL",
        "milliliters" => "mL",
        "millilitre" => "mL",
        "milligram" => "mg",
        "milligrams" => "mg",
        "microgram" => "µg",
        "micrograms" => "µg",
        "nanogram" => "ng",
        "nanograms" => "ng",
        "micrometer" => "µm",
        "micrometers" => "µm",
        "micrometre" => "µm",
        "micrometres" => "µm",
        "micron" => "µm",
        "microns" => "µm",
        "nanometer" => "nm",
        "nanometers" => "nm",
        "nanometre" => "nm",
        "nanometres" => "nm",
        "microsecond" => "µs",
        "microseconds" => "µs",
        "nanosecond" => "ns",
        "nanoseconds" => "ns",
        "percent" => "%",
        "pct" => "%",
        "arcsecond" => "arcsec",
        "arcseconds" => "arcsec",
        "arcminute" => "arcmin",
        "arcminutes" => "arcmin",
        "milliarcsecond" => "mas",
        "milliarcseconds" => "mas",
        "parsec" => "pc",
        "parsecs" => "pc",
        "kiloparsec" => "kpc",
        "kiloparsecs" => "kpc",
        "megaparsec" => "Mpc",
        "megaparsecs" => "Mpc",
        "lightyear" => "ly",
        "lightyears" => "ly",
        "hectopascal" => "hPa",
        "hectopascals" => "hPa",
        "hectare" => "ha",
        "hectares" => "ha",
        _ => return None,
    })
}

/// runtime/quantity.py `words_to_unit`: spelled unit words ("meters per second squared") as a unit spelling.
pub fn words_to_unit(tokens: &[&str]) -> Option<String> {
    let mut expanded: Vec<String> = vec![];
    for tok in tokens {
        let low = tok.to_lowercase();
        let parts: Vec<&str> = low.split('-').collect();
        if parts.len() > 1 && parts.iter().all(|x| unit_word(x).is_some()) {
            expanded.extend(parts.iter().map(|x| x.to_string()));
        } else {
            expanded.push(tok.to_string());
        }
    }
    let mut out: Vec<String> = expanded.iter()
        .map(|t| unit_word(&t.to_lowercase()).map(String::from).unwrap_or_else(|| t.clone())).collect();
    if let Some(i) = (0..out.len().saturating_sub(1)).find(|&i| out[i] == "/" && out[i + 1].to_lowercase() == "cent") {
        out.splice(i..i + 2, ["%".to_string()]);
    }
    if let Some(i) = (0..out.len().saturating_sub(1)).find(|&i| out[i] == "deg" && (out[i + 1] == "°C" || out[i + 1] == "C")) {
        out.splice(i..i + 2, ["°C".to_string()]);
    }
    let mut text = String::new();
    for part in out {
        if part.starts_with('^') || part == "/" || text.ends_with('/') || text.is_empty() {
            text.push_str(&part);
        } else {
            text.push('*');
            text.push_str(&part);
        }
    }
    if text.is_empty() { None } else { Some(text) }
}

/// runtime/quantity.py `declared_unit`: the unit a pure-number input's meaning names at its end ("time constant in
/// seconds" -> s, "rate in percent" -> %): the one to three words after its last " in ", a unit as written or spelled
/// out. None when there is none (a meaning with a parenthesis there, or words that are not a unit).
pub fn declared_unit(meaning: &str) -> Option<(String, Unit)> {
    let idx = meaning.rfind(" in ")?;
    let tail = meaning[idx + 4..].trim().trim_end_matches(['.', ')']).trim();
    if tail.is_empty() || tail.contains('(') {
        return None;
    }
    let toks: Vec<&str> = tail.split_whitespace().collect();
    if toks.is_empty() || toks.len() > 3 {
        return None;
    }
    let text = match parse_unit(tail) {
        Ok(_) => tail.to_string(),
        Err(_) => words_to_unit(&toks)?,
    };
    let u = parse_unit(&text).ok()?;
    if u.offset != 0.0 || (u.dim == Dim::one() && u.factor == 1.0) {
        return None;
    }
    Some((text, u))
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

#[cfg(test)]
mod astronomy_unit_tests {
    use super::*;
    #[test]
    fn astronomy_units() {
        let f = |u: &str| parse_unit(u).unwrap().factor;
        assert!((f("arcsec") / (PI / 648000.0) - 1.0).abs() < 1e-15);
        assert!((f("mas") / (PI / 648000000.0) - 1.0).abs() < 1e-15);
        assert_eq!(f("au"), 149597870700.0);
        assert_eq!(f("ly"), 9460730472580800.0);
        assert!((f("pc") / 3.0856775814913673e16 - 1.0).abs() < 1e-15);
        assert!((f("kpc") / 3.0856775814913673e19 - 1.0).abs() < 1e-15);
        assert!((f("Mpc") / 3.0856775814913673e22 - 1.0).abs() < 1e-15);
        assert_eq!(parse_unit("arcsec").unwrap().dim, parse_unit("rad").unwrap().dim);
    }
}

#[cfg(test)]
mod hecto_deca_tests {
    use super::*;
    #[test]
    fn hecto_and_deca_prefixes() {
        let f = |u: &str| parse_unit(u).unwrap().factor;
        assert_eq!(f("hPa"), 100.0);
        assert_eq!(parse_unit("hPa").unwrap().dim, parse_unit("Pa").unwrap().dim);
        assert!((f("hL") - 0.1).abs() < 1e-15);
        assert_eq!(f("daN"), 10.0);
        assert_eq!(f("ha"), 1e4);
        assert_eq!(parse_unit("ha").unwrap().dim, parse_unit("m^2").unwrap().dim);
        // the hour, the horsepower and the day keep their meaning
        assert_eq!(f("h"), 3600.0);
        assert_eq!(f("hr"), 3600.0);
        assert!((f("hp") - 745.69987158227022).abs() < 1e-9);
        assert_eq!(f("day"), 86400.0);
        assert_eq!(f("d"), 86400.0);
        assert!((f("1/h") - 1.0 / 3600.0).abs() < 1e-18);
        assert!((f("h^-1") - 1.0 / 3600.0).abs() < 1e-18);
        // atoms that take no prefix stay unprefixed
        assert!(parse_unit("hmin").is_err());
        assert!(parse_unit("hkg").is_err());
    }
}

#[cfg(test)]
mod declared_unit_tests {
    use super::*;
    #[test]
    fn declared_units() {
        assert_eq!(declared_unit("time constant in seconds").unwrap().0, "s");
        assert_eq!(declared_unit("time after the step in seconds").unwrap().0, "s");
        assert_eq!(declared_unit("interest rate in percent").unwrap().0, "%");
        assert!(declared_unit("heart rate in beats/min").is_none());
        assert!(declared_unit("number of moles in the sample").is_none());
        assert!(declared_unit("steady-state (DC) gain of the system").is_none());
        assert_eq!(words_to_unit(&["meters", "per", "second"]).unwrap(), "m/s");
        assert_eq!(words_to_unit(&["per", "hour"]).unwrap(), "/h");
    }
}
