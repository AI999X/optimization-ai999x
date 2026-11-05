//! Explanation templates filled with the engine's own values: the Rust twin of the Python runtime's
//! `runtime/explanations.py` render() and of the website's `AXCompute.explain`. The three must give identical text for
//! the same template and view (tests/fixtures/explanations_render.json, checked by `tests/explain_fixture.rs`).
//!
//! Placeholders: `{in:s}` an input's value with its unit, `{out:s}` an output's, `{val:s}` either (outputs first),
//! `{unit:s}` the unit alone. A view is `{"inputs": {s: {"value", "unit"}}, "outputs": {...}}`. An integer (a JSON
//! integer token) is written in full, any other number as Python's format(x, ".6g"), text as written, booleans as
//! yes/no, a list as "[a, b]", {"re","im"} as "a + bi". A unit "" or "1" is not written. A placeholder that cannot be
//! filled makes the whole text unavailable (None), never a gap.

use crate::num::py_g;
use serde_json::Value;

fn g6(x: f64) -> String {
    py_g(x, 6)
}

fn number_text(n: &serde_json::Number) -> String {
    let t = n.to_string();
    if !t.contains(['.', 'e', 'E']) {
        // an integer token, digit for digit ("-0" is Python's int 0)
        return if t == "-0" { "0".into() } else { t };
    }
    g6(t.parse::<f64>().unwrap_or(f64::NAN))
}

fn as_f64(v: &Value) -> Option<f64> {
    match v {
        Value::Number(n) => n.to_string().parse::<f64>().ok(),
        _ => None,
    }
}

/// One value as prose writes it; None for a missing value.
pub fn show(v: &Value) -> Option<String> {
    match v {
        Value::Null => None,
        Value::Bool(b) => Some(if *b { "yes" } else { "no" }.into()),
        Value::Number(n) => Some(number_text(n)),
        Value::String(s) => Some(s.clone()),
        Value::Array(xs) => {
            let parts: Option<Vec<String>> = xs.iter().map(show).collect();
            parts.map(|p| format!("[{}]", p.join(", ")))
        }
        Value::Object(m) => {
            if m.len() == 2 && m.contains_key("re") && m.contains_key("im") {
                let re = as_f64(&m["re"])?;
                let im = as_f64(&m["im"])?;
                let neg = im < 0.0 || (im == 0.0 && im.is_sign_negative());
                return Some(format!("{} {} {}i", g6(re), if neg { '-' } else { '+' }, g6(im.abs())));
            }
            let mut parts = vec![];
            for (k, x) in m {
                parts.push(format!("{k}: {}", show(x)?));
            }
            Some(format!("{{{}}}", parts.join(", ")))
        }
    }
}

fn unit_of(e: &Value) -> String {
    let u = match e.get("unit") {
        None | Some(Value::Null) => String::new(),
        Some(Value::String(s)) => s.clone(),
        Some(other) => other.to_string(),
    };
    if u == "1" { String::new() } else { u }
}

fn is_ident(s: &str) -> bool {
    let mut cs = s.chars();
    match cs.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => cs.all(|c| c.is_ascii_alphanumeric() || c == '_'),
        _ => false,
    }
}

/// The placeholders of a text, in order: (kind, symbol).
pub fn placeholders(text: &str) -> Vec<(String, String)> {
    let mut out = vec![];
    let mut rest = text;
    while let Some(i) = rest.find('{') {
        let after = &rest[i + 1..];
        if let Some((kind, sym, len)) = parse_at(after) {
            out.push((kind.to_string(), sym.to_string()));
            rest = &after[len..];
        } else {
            rest = after;
        }
    }
    out
}

// after a '{': Some((kind, symbol, bytes consumed including the closing '}')) when a placeholder starts here
fn parse_at(s: &str) -> Option<(&str, &str, usize)> {
    let colon = s.find(':')?;
    let kind = &s[..colon];
    if !matches!(kind, "in" | "out" | "val" | "unit") {
        return None;
    }
    let close = s[colon + 1..].find('}')? + colon + 1;
    let sym = &s[colon + 1..close];
    if !is_ident(sym) {
        return None;
    }
    Some((kind, sym, close + 1))
}

/// `text` with every placeholder filled from `view`; None when one cannot be filled.
pub fn render(text: &str, view: &Value) -> Option<String> {
    let empty = Value::Object(Default::default());
    let ins = view.get("inputs").filter(|v| v.is_object()).unwrap_or(&empty);
    let outs = view.get("outputs").filter(|v| v.is_object()).unwrap_or(&empty);
    let mut out = String::with_capacity(text.len() + 32);
    let mut rest = text;
    let mut ok = true;
    while let Some(i) = rest.find('{') {
        out.push_str(&rest[..i]);
        let after = &rest[i + 1..];
        match parse_at(after) {
            Some((kind, sym, len)) => {
                let entry = match kind {
                    "in" => ins.get(sym),
                    "out" => outs.get(sym),
                    _ => outs.get(sym).or_else(|| ins.get(sym)),
                };
                match entry.filter(|e| e.is_object()) {
                    None => ok = false,
                    Some(e) => {
                        let u = unit_of(e);
                        if kind == "unit" {
                            if u.is_empty() {
                                ok = false;
                            }
                            out.push_str(&u);
                        } else {
                            match e.get("value").and_then(show) {
                                None => ok = false,
                                Some(s) if u.is_empty() => out.push_str(&s),
                                Some(s) => {
                                    out.push_str(&s);
                                    out.push(' ');
                                    out.push_str(&u);
                                }
                            }
                        }
                    }
                }
                rest = &after[len..];
            }
            None => {
                out.push('{');
                rest = after;
            }
        }
    }
    out.push_str(rest);
    if ok { Some(out) } else { None }
}

/// render() with the view given as JSON text (integers kept digit for digit).
pub fn render_json(text: &str, view: &str) -> Option<String> {
    let v: Value = serde_json::from_str(view).ok()?;
    render(text, &v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fills_values_with_units() {
        let v = r#"{"inputs": {"v": {"value": 3.0, "unit": "m/s"}, "t": {"value": 2, "unit": "s"}},
                    "outputs": {"d": {"value": 6.000000000000001, "unit": "m"}}}"#;
        assert_eq!(render_json("{val:d} after {in:t} at {val:v} ({unit:d})", v).unwrap(), "6 m after 2 s at 3 m/s (m)");
    }

    #[test]
    fn python_number_format_and_ties() {
        let v = r#"{"outputs": {"a": {"value": 12.03125}, "b": {"value": 1234565.0}, "c": {"value": -0.0},
                    "d": {"value": 0.0001234565, "unit": "1"}, "e": {"value": 12381557655576425121}}}"#;
        assert_eq!(render_json("{out:a} {out:b} {out:c} {out:d} {out:e}", v).unwrap(),
                   "12.0312 1.23456e+06 -0 0.000123457 12381557655576425121");
    }

    #[test]
    fn missing_value_or_unit_gives_none() {
        let v = r#"{"inputs": {"x": {"value": 1.5, "unit": ""}}, "outputs": {"y": {"value": null}}}"#;
        assert!(render_json("{out:x}", v).is_none());
        assert!(render_json("{unit:x}", v).is_none());
        assert!(render_json("{out:y}", v).is_none());
        assert_eq!(render_json("{in:x} {not a placeholder} {in:1x}", v).unwrap(), "1.5 {not a placeholder} {in:1x}");
    }

    #[test]
    fn structured_values() {
        let v = r#"{"outputs": {"z": {"value": {"re": 1.0, "im": -0.5}}, "l": {"value": [1, 2.5, "a", true]},
                    "m": {"value": {"k": 0.1}}}}"#;
        assert_eq!(render_json("{out:z}; {out:l}; {out:m}", v).unwrap(), "1 - 0.5i; [1, 2.5, a, yes]; {k: 0.1}");
        assert_eq!(placeholders("a {in:x} {b} {unit:y_2}"), vec![("in".into(), "x".into()), ("unit".into(), "y_2".into())]);
    }
}
