//! Structured inputs of a native procedure, read exactly as runtime/inputs.py `coerce` reads them (same values, same
//! refusal wording): integer, vector, matrix, text, choice, boolean. Nothing is evaluated: numbers are read the way
//! Python's float() reads them.

use crate::engine::Input;
use crate::num::{py_g, py_repr_f64, py_repr_str};
use crate::pack::{ProcInput, Val};
use serde_json::Value;

/// Python's repr() of a value as it arrived (the refusal messages quote it).
pub fn py_repr(v: &Input) -> String {
    match v {
        Input::Text(t) => py_repr_str(t),
        Input::Int(i) => i.to_string(),
        Input::Float(f) => py_repr_f64(*f),
        Input::Json(j) => py_repr_json(j),
    }
}

/// Python's str() of a value as it arrived.
fn py_str(v: &Input) -> String {
    match v {
        Input::Text(t) => t.clone(),
        _ => py_repr(v),
    }
}

pub fn py_repr_json(j: &Value) -> String {
    match j {
        Value::Null => "None".into(),
        Value::Bool(b) => if *b { "True".into() } else { "False".into() },
        Value::Number(n) => match n.as_i64() {
            Some(i) => i.to_string(),
            None => py_repr_f64(n.as_f64().unwrap_or(f64::NAN)),
        },
        Value::String(s) => py_repr_str(s),
        Value::Array(a) => format!("[{}]", a.iter().map(py_repr_json).collect::<Vec<_>>().join(", ")),
        Value::Object(o) => format!("{{{}}}", o.iter().map(|(k, v)| format!("{}: {}", py_repr_str(k), py_repr_json(v)))
            .collect::<Vec<_>>().join(", ")),
    }
}

/// Python's float(text): optional sign, digits with single underscores between them, a point, an exponent; "inf",
/// "infinity", "nan" in any case; surrounding white space ignored.
pub fn py_float(text: &str) -> Option<f64> {
    let t = text.trim();
    let (sign, body) = match t.strip_prefix('-') {
        Some(b) => (-1.0, b),
        None => (1.0, t.strip_prefix('+').unwrap_or(t)),
    };
    let low = body.to_ascii_lowercase();
    if low == "inf" || low == "infinity" {
        return Some(sign * f64::INFINITY);
    }
    if low == "nan" {
        return Some(f64::NAN);
    }
    // underscores only between two digits
    let cs: Vec<char> = body.chars().collect();
    for (i, c) in cs.iter().enumerate() {
        if *c == '_' && !(i > 0 && i + 1 < cs.len() && cs[i - 1].is_ascii_digit() && cs[i + 1].is_ascii_digit()) {
            return None;
        }
    }
    let clean: String = body.chars().filter(|c| *c != '_').collect();
    if clean.is_empty() || !clean.chars().all(|c| c.is_ascii_digit() || matches!(c, '.' | 'e' | 'E' | '+' | '-')) {
        return None;
    }
    if !clean.chars().any(|c| c.is_ascii_digit()) || clean.starts_with(['e', 'E', '+', '-']) {
        return None;
    }
    clean.parse::<f64>().ok().map(|x| sign * x)
}

/// inputs.py `_number`: Err is the InputError message.
fn number(x: &Input, name: &str) -> Result<f64, String> {
    let not_a_number = || format!("{name}: {} is not a number", py_repr(x));
    let v = match x {
        Input::Json(Value::Bool(_)) => return Err(format!("{name}: expected a number, got {}", py_repr(x))),
        Input::Text(t) => py_float(&t.replace('−', "-").replace(',', "")).ok_or_else(not_a_number)?,
        Input::Int(i) => *i as f64,
        Input::Float(f) => *f,
        Input::Json(Value::Number(n)) => n.as_f64().ok_or_else(not_a_number)?,
        Input::Json(Value::String(s)) => py_float(&s.replace('−', "-").replace(',', "")).ok_or_else(not_a_number)?,
        Input::Json(_) => return Err(not_a_number()),
    };
    if !v.is_finite() {
        return Err(format!("{name}: {} is not a finite number", py_repr(x)));
    }
    Ok(v)
}

fn json_item(j: &Value) -> Input {
    match j {
        Value::String(s) => Input::Text(s.clone()),
        Value::Number(n) => match n.as_i64() {
            Some(i) => Input::Int(i),
            None => Input::Float(n.as_f64().unwrap_or(f64::NAN)),
        },
        other => Input::Json(other.clone()),
    }
}

/// inputs.py `_split_list`.
fn split_list(text: &str) -> Vec<String> {
    let mut t = text.trim();
    if t.starts_with('[') && t.ends_with(']') && t.len() >= 2 {
        t = &t[1..t.len() - 1];
    }
    t.split(|c: char| c == ',' || c == ';' || c.is_whitespace()).filter(|p| !p.is_empty()).map(String::from).collect()
}

fn length(name: &str, n: usize, spec: &ProcInput) -> Result<(), String> {
    if let Some(lo) = spec.min_length {
        if n < lo {
            return Err(format!("{name}: needs at least {lo} entries, got {n}"));
        }
    }
    if let Some(hi) = spec.max_length {
        if n > hi {
            return Err(format!("{name}: allows at most {hi} entries, got {n}"));
        }
    }
    Ok(())
}

/// Why a value could not be read: an InputError (its message), or Python's TypeError/ValueError around it (the
/// procedure path then says "expected a <type>").
pub enum Bad {
    Input(String),
    Type,
}

/// The Python iteration of a value that is not text: list(value). None when Python raises TypeError.
fn items_of(v: &Input) -> Option<Vec<Input>> {
    match v {
        Input::Json(Value::Array(a)) => Some(a.iter().map(json_item).collect()),
        Input::Json(Value::Object(o)) => Some(o.keys().map(|k| Input::Text(k.clone())).collect()),
        _ => None,
    }
}

/// inputs.py `coerce` for a structured input (type integer, vector, matrix, text, choice or boolean).
pub fn coerce(spec: &ProcInput, value: &Input) -> Result<Val, Bad> {
    let name = spec.symbol.as_str();
    let input = |m: String| Bad::Input(m);
    match spec.ty.as_str() {
        "integer" => {
            if matches!(value, Input::Json(Value::Bool(_))) {
                return Err(input(format!("{name}: expected a whole number, got {}", py_repr(value))));
            }
            let v: i64 = match value {
                Input::Int(i) => *i,
                Input::Json(Value::Number(n)) if n.is_i64() => n.as_i64().unwrap(),
                Input::Text(t) if is_int_text(t) => {
                    let c: String = t.trim().replace('−', "-").chars().filter(|c| *c != ',' && *c != '_').collect();
                    c.parse::<i64>().map_err(|_| input(format!("{name}: expected a whole number written exactly, got {}",
                                                                py_repr(value))))?
                }
                _ => {
                    let f = number(value, name).map_err(input)?;
                    if f != f.trunc() || f.abs() > 9007199254740992.0 {
                        return Err(input(format!("{name}: expected a whole number written exactly, got {}",
                                                 py_repr(value))));
                    }
                    f as i64
                }
            };
            if let Some((lo, s)) = &spec.min {
                if (v as f64) < *lo {
                    return Err(input(format!("{name}: must be at least {s}, got {}", py_g(v as f64, 6))));
                }
            }
            if let Some((hi, s)) = &spec.max {
                if (v as f64) > *hi {
                    return Err(input(format!("{name}: must be at most {s}, got {}", py_g(v as f64, 6))));
                }
            }
            Ok(Val::Int(v))
        }
        "vector" => {
            let items: Vec<Input> = match value {
                Input::Text(t) => split_list(t).into_iter().map(Input::Text).collect(),
                other => items_of(other).ok_or(Bad::Type)?,
            };
            let out = items.iter().map(|x| number(x, name)).collect::<Result<Vec<f64>, String>>().map_err(input)?;
            length(name, out.len(), spec).map_err(input)?;
            Ok(Val::Vec(out))
        }
        "matrix" => {
            let rows: Vec<Vec<Input>> = match value {
                Input::Text(t) => {
                    let s = t.trim();
                    let raw: Vec<String> = if s.starts_with("[[") {
                        split_rows(s.trim_matches(|c| c == '[' || c == ']'))
                    } else {
                        s.split(';').filter(|r| !r.trim().is_empty()).map(String::from).collect()
                    };
                    raw.iter().map(|r| split_list(r).into_iter().map(Input::Text).collect()).collect()
                }
                other => {
                    let rows = items_of(other).ok_or(Bad::Type)?;
                    let mut out = vec![];
                    for r in rows {
                        out.push(match &r {
                            Input::Text(t) => t.chars().map(|c| Input::Text(c.to_string())).collect(),
                            r => items_of(r).ok_or(Bad::Type)?,
                        });
                    }
                    out
                }
            };
            let out = rows.iter().map(|r| r.iter().map(|x| number(x, name)).collect::<Result<Vec<f64>, String>>())
                .collect::<Result<Vec<Vec<f64>>, String>>().map_err(input)?;
            if out.is_empty() || out[0].is_empty() || out.iter().any(|r| r.len() != out[0].len()) {
                return Err(input(format!("{name}: expected a rectangular matrix (rows of equal length)")));
            }
            Ok(Val::Mat(out))
        }
        "edges" => edges(spec, value),
        "text" => match value {
            Input::Text(t) => {
                length(name, t.chars().count(), spec).map_err(input)?;
                Ok(Val::Text(t.clone()))
            }
            Input::Json(Value::String(t)) => Ok(Val::Text(t.clone())),
            other => Err(input(format!("{name}: expected text, got {}", py_repr(other)))),
        },
        "choice" => {
            let low = py_str(value).trim().to_lowercase();
            match spec.choices.iter().find(|c| c.to_lowercase() == low) {
                Some(c) => Ok(Val::Text(c.clone())),
                None => Err(input(format!("{name}: one of {}; got {}", spec.choices.join(", "), py_repr(value)))),
            }
        }
        "boolean" => {
            if let Input::Json(Value::Bool(b)) = value {
                return Ok(Val::Bool(*b));
            }
            match py_str(value).trim().to_lowercase().as_str() {
                "true" | "yes" | "1" => Ok(Val::Bool(true)),
                "false" | "no" | "0" => Ok(Val::Bool(false)),
                _ => Err(input(format!("{name}: expected true or false, got {}", py_repr(value)))),
            }
        }
        t => Err(input(format!("{name}: unknown input type {}", py_repr_str(t)))),
    }
}

/// list(x) in Python: the characters of a str, the items of a list, the keys of a dict; None (TypeError) otherwise.
fn list_of(x: &Input) -> Option<Vec<Input>> {
    match x {
        Input::Text(t) | Input::Json(Value::String(t)) => Some(t.chars().map(|c| Input::Text(c.to_string())).collect()),
        other => items_of(other),
    }
}

fn repr_list(xs: &[Input]) -> String {
    format!("[{}]", xs.iter().map(py_repr).collect::<Vec<_>>().join(", "))
}

/// inputs.py `coerce` for an edges input: "A-B 4, B C 2" (text) or a list of [u, v] / [u, v, weight].
fn edges(spec: &ProcInput, value: &Input) -> Result<Val, Bad> {
    let name = spec.symbol.as_str();
    let input = |m: String| Bad::Input(m);
    let items: Vec<Vec<Input>> = match value {
        Input::Text(t) => {
            // the same expressions as inputs.py, through the same regular-expression rules (crate::pyre)
            let re = |p: &str| crate::pyre::compile(p, 0).map_err(|_| Bad::Type);
            let sep = re(r"[;,]\s*")?;
            let dash = re(r"\s*([^\s\-–]+)\s*[-–]\s*([^\s\-–]+)(?:\s+([-+−]?[\d.]+(?:[eE][-+]?\d+)?))?\s*")?;
            let space = re(r"\s*(\S+)\s+(\S+)(?:\s+([-+−]?[\d.]+(?:[eE][-+]?\d+)?))?\s*")?;
            let parts = crate::pyre::split(&sep, t.trim(), 0).map_err(|_| Bad::Type)?.items().map_err(|_| Bad::Type)?;
            let mut out = vec![];
            for part in parts {
                let part = part.as_str().map_err(|_| Bad::Type)?;
                if part.trim().is_empty() {
                    continue;
                }
                let mut hit = None;
                for rx in [&dash, &space] {
                    if let crate::py::Py::Match(m) = crate::pyre::find(rx, "fullmatch", &part).map_err(|_| Bad::Type)? {
                        hit = Some(m);
                        break;
                    }
                }
                let Some(m) = hit else {
                    return Err(input(format!("{name}: cannot read the edge {}; write 'A-B 4' or 'A B 4'",
                                             py_repr_str(part.trim()))));
                };
                let groups = m.method("groups", &[]).map_err(|_| Bad::Type)?.items().map_err(|_| Bad::Type)?;
                out.push(groups.iter().filter_map(|g| g.as_str().ok().map(|s| Input::Text(s.to_string()))).collect());
            }
            out
        }
        other => {
            let es = items_of(other).ok_or(Bad::Type)?;
            es.iter().map(list_of).collect::<Option<Vec<_>>>().ok_or(Bad::Type)?
        }
    };
    let mut out: Vec<(String, String, Option<f64>)> = vec![];
    for e in &items {
        if !(e.len() == 2 || e.len() == 3) {
            return Err(input(format!("{name}: each edge is (u, v) or (u, v, weight); got {}", repr_list(e))));
        }
        let (u, v) = (py_str(&e[0]).trim().to_string(), py_str(&e[1]).trim().to_string());
        if u.is_empty() || v.is_empty() {
            return Err(input(format!("{name}: an edge needs two node names; got {}", repr_list(e))));
        }
        let w = if e.len() == 3 { Some(number(&e[2], name).map_err(input)?) } else { None };
        out.push((u, v, w));
    }
    if out.iter().any(|e| e.2.is_some()) && out.iter().any(|e| e.2.is_none()) {
        return Err(input(format!("{name}: give a weight for every edge or for none")));
    }
    length(name, out.len(), spec).map_err(input)?;
    Ok(Val::Edges(out))
}

/// inputs.py: re.fullmatch(r"\s*[-+−]?\d[\d,_]*\s*", value)
fn is_int_text(t: &str) -> bool {
    let s = t.trim();
    let s = s.strip_prefix(['-', '+', '−']).unwrap_or(s);
    let mut cs = s.chars();
    matches!(cs.next(), Some(c) if c.is_ascii_digit()) && cs.all(|c| c.is_ascii_digit() || c == ',' || c == '_')
}

/// re.split(r"\]\s*,?\s*\[", s) without the empty rows
fn split_rows(s: &str) -> Vec<String> {
    let mut out = vec![];
    let mut cur = String::new();
    let cs: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < cs.len() {
        if cs[i] == ']' {
            let mut j = i + 1;
            while j < cs.len() && cs[j].is_whitespace() {
                j += 1;
            }
            if j < cs.len() && cs[j] == ',' {
                j += 1;
            }
            while j < cs.len() && cs[j].is_whitespace() {
                j += 1;
            }
            if j < cs.len() && cs[j] == '[' {
                out.push(std::mem::take(&mut cur));
                i = j + 1;
                continue;
            }
        }
        cur.push(cs[i]);
        i += 1;
    }
    out.push(cur);
    out.into_iter().filter(|r| !r.trim().is_empty()).collect()
}

#[cfg(test)]
mod tests {
    //! runtime/inputs.py coerce's own results for these inputs (printed by Python 3.13)
    use super::*;

    fn read(v: Input) -> String {
        let spec = ProcInput { symbol: "edges".into(), kind: "number".into(), meaning: String::new(), required: true,
                               ty: "edges".into(), choices: vec![], min: None, max: None, min_length: None,
                               max_length: None };
        match coerce(&spec, &v) {
            Ok(Val::Edges(es)) => format!("{:?}", es),
            Ok(other) => format!("{other:?}"),
            Err(Bad::Input(m)) => format!("ERR {m}"),
            Err(Bad::Type) => "TYPE".into(),
        }
    }

    #[test]
    fn edges_as_python() {
        let t = |s: &str| read(Input::Text(s.into()));
        let j = |s: &str| read(Input::Json(serde_json::from_str(s).unwrap()));
        assert_eq!(t("A-B 4, B C 2"), r#"[("A", "B", Some(4.0)), ("B", "C", Some(2.0))]"#);
        assert_eq!(t("B-A -3"), r#"[("B", "A", Some(-3.0))]"#);
        assert_eq!(t("A-B; B-C"), r#"[("A", "B", None), ("B", "C", None)]"#);
        assert_eq!(t("x y"), r#"[("x", "y", None)]"#);
        assert_eq!(t("A-B 4, B-C"), "ERR edges: give a weight for every edge or for none");
        assert_eq!(t("A--B"), "ERR edges: cannot read the edge 'A--B'; write 'A-B 4' or 'A B 4'");
        assert_eq!(t(" ,A-B 1e2 ,"), r#"[("A", "B", Some(100.0))]"#);
        assert_eq!(t("A B C D"), "ERR edges: cannot read the edge 'A B C D'; write 'A-B 4' or 'A B 4'");
        assert_eq!(t("A–B 2"), r#"[("A", "B", Some(2.0))]"#);
        assert_eq!(t("A-B 1,5"), "ERR edges: cannot read the edge '5'; write 'A-B 4' or 'A B 4'");
        assert_eq!(j(r#"[["A","B",2],["B","C",3]]"#), r#"[("A", "B", Some(2.0)), ("B", "C", Some(3.0))]"#);
        assert_eq!(j(r#"[["A","B"]]"#), r#"[("A", "B", None)]"#);
        assert_eq!(j(r#"["AB"]"#), r#"[("A", "B", None)]"#);
        assert_eq!(j(r#"[["A","B","C","D"]]"#), "ERR edges: each edge is (u, v) or (u, v, weight); got ['A', 'B', 'C', 'D']");
        assert_eq!(j(r#"[[" ","B"]]"#), "ERR edges: an edge needs two node names; got [' ', 'B']");
    }
}
