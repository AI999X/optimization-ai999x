//! solve(capability, inputs): runtime/product.py `_solve_capability` and runtime/relation.py `solve`/`residual`,
//! ported line for line. Every message is the Python reference's, character for character (tools/rust_parity.py).

#[allow(unused_imports)]
use crate::lm::{Lm, LmF64};
use crate::num::{py_g, py_repr_str};
use crate::pack::{Pack, Relation, Val};
use crate::units::{parse_unit, to_si};
use serde::Serialize;
use std::collections::HashMap;

/// A value as the caller gives it: text ("3 m/s") or a number (in the quantity's SI unit).
#[derive(Clone, Debug)]
pub enum Input {
    Text(String),
    Int(i64),
    Float(f64),
    /// a list, true/false or null as sent (vectors, matrices, booleans)
    Json(serde_json::Value),
}

impl Input {
    fn repr(&self) -> String {
        crate::coerce::py_repr(self)
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Answer {
    pub outcome: String,
    pub message: String,
    pub value: Option<f64>,
    pub unit: Option<String>,
    /// for an independent check: the relation's symbol that was solved for and every input in SI (set when the
    /// inputs were read and the relation was solved, whether or not it gave a value)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solved_for: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub si: Vec<(String, f64)>,
    /// every output of a procedure, in its order, with its unit ("" for a pure number); a number, or a list or text
    /// for a procedure that returns one
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<(String, serde_json::Value, String)>,
    // for display (the website shows them under the answer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub phrase: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub assumptions: Vec<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub equation: String,
    #[serde(skip)]
    pub inputs: Vec<(String, f64, String, String)>, // symbol, SI value, SI unit, as given
}

impl Answer {
    fn refuse(outcome: &str, message: String) -> Answer {
        Answer { outcome: outcome.into(), message, value: None, unit: None, solved_for: None, si: vec![], values: vec![],
                 target: None, phrase: String::new(),
                 assumptions: vec![], name: String::new(), equation: String::new(), inputs: vec![] }
    }
    pub fn solved(&self) -> bool {
        self.outcome == "SOLVED"
    }
}

enum Why {
    InputSign { name: String, value: f64 },
    ResultSign { name: String, values: Vec<f64> },
    NoReal { name: String },
    Branches { name: String, values: Vec<f64> },
    /// the value exists but is outside what a double can hold: about 10^log10
    Range { name: String, log10: f64 },
    /// double precision cannot tell this value (its size depends on digits a cancellation destroyed)
    Unreliable { name: String },
    /// beyond 1e308 and of a sign the quantity cannot have (a temperature of about -1e638 K)
    RangeSign { name: String, sign: f64, log10: f64 },
}

fn admissible(sign: &str, v: f64) -> bool {
    !((sign == "positive" && v <= 0.0) || (sign == "nonnegative" && v < 0.0))
}

/// The equation's residual lhs - rhs (summed terms) with `target` = x, and a bound on its rounding error.
fn residual_at(rel: &Relation, values: &HashMap<String, f64>, target: &str, x: f64) -> Option<(f64, f64)> {
    let mut with = values.clone();
    with.insert(target.to_string(), x);
    let (mut sum, mut err) = (0.0, 0.0);
    for t in &rel.residual_terms {
        let (v, e) = t.eval_bound(&with).ok()?;
        if !v.is_finite() || v.im != 0.0 {
            return None;
        }
        sum += v.re;
        err += e;
    }
    Some((sum, err))
}

/// A candidate refined by Newton's method on the equation as written, and a bound on its error from the
/// equation's own conditioning. A solved form can lose digits a perfect answer would keep (the small root of a
/// quadratic, k - sqrt(k^2 - ...)); a few Newton steps on the original equation restore them. Only small corrections
/// are taken (never a jump to another solution).
fn polish(rel: &Relation, values: &HashMap<String, f64>, target: &str, x0: f64) -> Option<(f64, f64)> {
    let mut x = x0;
    let (mut f, mut ferr) = residual_at(rel, values, target, x)?;
    let mut slope = 0.0;
    for _ in 0..6 {
        let h = 1e-7 * x.abs().max(1e-300);
        let (fp, _) = residual_at(rel, values, target, x + h)?;
        let (fm, _) = residual_at(rel, values, target, x - h)?;
        slope = (fp - fm) / (2.0 * h);
        // a residual inside its own rounding noise carries no direction: refining on it would move an accurate value
        // (V = emf*(1 - exp(-x)) with x ~ 1e-8 cannot be evaluated to better than 1e-8 of emf)
        if f == 0.0 || f.abs() <= 2.0 * ferr || slope == 0.0 || !slope.is_finite() {
            break;
        }
        let x1 = x - f / slope;
        if !x1.is_finite() || (x1 - x).abs() > 1e-3 * x.abs().max(1e-300) {
            break;
        }
        let (f1, e1) = residual_at(rel, values, target, x1)?;
        if f1.abs() >= f.abs() {
            break;
        }
        (x, f, ferr) = (x1, f1, e1);
    }
    // the answer is known to within what rounding allows the equation to show, divided by its sensitivity to x
    let err = if slope != 0.0 && slope.is_finite() { (ferr + f.abs()) / slope.abs() } else { f64::INFINITY };
    Some((x, err))
}

/// The single admissible value of `target` (relation.py Relation.solve), with a bound on its rounding error.
fn solve_relation(rel: &Relation, target: &str, values: &HashMap<String, f64>) -> Result<(f64, f64), Why> {
    for s in &rel.symbols {
        if let Some(v) = values.get(&s.name) {
            if !admissible(&s.sign, *v) {
                return Err(Why::InputSign { name: s.name.clone(), value: *v });
            }
        }
    }
    let sign = rel.sym(target).map(|s| s.sign.as_str()).unwrap_or("any");
    let mut roots: Vec<f64> = vec![];
    let mut terms: Vec<f64> = vec![];
    let mut errs: Vec<f64> = vec![];
    let mut wrong_sign: Vec<f64> = vec![];
    let mut out_of_range: Option<f64> = None;
    // a form that overflowed, underflowed or gave NaN where its size cannot be known (Node::size_known): the value
    // exists or not, but double precision cannot tell; said so, never "no real value" or "0, impossible"
    let mut unreliable = false;
    let mut huge_wrong_sign: Option<(f64, f64)> = None;   // (sign, log10 of the size)
    for f in rel.forms.get(target).map(|v| v.as_slice()).unwrap_or(&[]) {
        let mut used = &f.form;
        let mut z = match f.form.eval(values) {
            Ok(z) => z,
            Err(_) => continue,
        };
        if !z.is_finite() {
            // an exponential overflowed (the reference's arithmetic has an unbounded exponent): the same value,
            // written to avoid it (the exp alternative, else a variant: the logarithms left uncombined)
            let finite_variant = f.variants.iter()
                .find_map(|r| r.eval(values).ok().filter(|w| w.is_finite()).map(|w| (r, w)));
            match f.alt.as_ref().map(|a| (a, a.eval(values))) {
                Some((a, Ok(w))) if w.is_finite() => {
                    z = w;
                    used = a;
                }
                _ if finite_variant.is_some() => {
                    let (r, w) = finite_variant.unwrap();
                    z = w;
                    used = r;
                }
                _ => {
                    // truly beyond 1e308: say how large, if the size is known
                    // the size from the first way of writing the value whose size is known
                    match std::iter::once(&f.form).chain(f.variants.iter()).find(|n| n.size_known(values)) {
                        // only the log1p rewrite loses a value this way; a plain division by zero has no value
                        None if f.form.has_log1p() => unreliable = true,
                        None => {}
                        Some(n) => {
                            if let Some(l) = n.log10_abs(values).filter(|l| *l > 307.0) {
                                match n.sign_of(values) {
                                    // beyond 1e308 and of a sign the quantity cannot have: no admissible value
                                    Some(s) if !admissible(sign, s) => huge_wrong_sign = Some((s, l)),
                                    Some(_) => out_of_range = Some(l),
                                    None => unreliable = true,
                                }
                            }
                        }
                    }
                    continue;
                }
            }
        }
        // the same root written without a cancellation (a - sqrt(D) rationalized, 1 - exp(-x) as -expm1(-x)): the
        // variant with the smallest relative error bound is kept
        for r in &f.variants {
            if let (Ok((w, ew)), Ok((_, ez))) = (r.eval_bound(values), used.eval_bound(values)) {
                let rel_now = if z.abs() > 0.0 { ez / z.abs() } else { f64::INFINITY };
                if w.is_finite() && w.abs() > 0.0 && ew / w.abs() < rel_now {
                    z = w;
                    used = r;
                }
            }
        }
        // a way of writing the value whose size is unknown (log1p of a number that rounded to just below -1 gives a
        // complex 1e-151 for a true 1e-19563) gives way to one whose size is known
        if !used.size_known(values) {
            if let Some((r, w)) = f.variants.iter().filter(|r| r.size_known(values))
                .find_map(|r| r.eval(values).ok().filter(|w| w.is_finite()).map(|w| (r, w))) {
                z = w;
                used = r;
            }
        }
        // inside the subnormal range a double keeps only a few digits (2e-317 to four): beyond what this program
        // represents faithfully, said so like any value too small to represent
        if z.is_finite() && z.im == 0.0 && z.re != 0.0 && z.re.abs() < f64::MIN_POSITIVE {
            out_of_range = Some(z.re.abs().log10_lm());
            continue;
        }
        if z == crate::num::C::r(0.0) {
            if !used.size_known(values) && used.has_log1p() {
                let skip = match std::iter::once(&f.form).chain(f.variants.iter()).find(|n| n.size_known(values)) {
                    Some(n) if !admissible(sign, 0.0) => {
                        if let Some(l) = n.log10_abs(values).filter(|l| *l < -307.0) {
                            out_of_range = Some(l);
                        } else {
                            unreliable = true;
                        }
                        true
                    }
                    Some(_) => false,                    // zero is allowed and its size is known: judged below
                    None => {
                        unreliable = true;
                        true
                    }
                };
                if skip {
                    continue;
                }
            }
            // an exact zero may be an underflow (V0*exp(-10^7)): the value is real and positive, only too small
            if let Some(l) = used.log10_abs(values).filter(|l| *l < -307.0) {
                if !admissible(sign, 0.0) {
                    out_of_range = Some(l);
                    continue;
                }
            }
        }
        let scale = 1.0f64.max(z.re.abs());
        if z.im.abs() > 1e-9 * scale {
            continue;
        }
        let mut z = crate::num::C::new(if z.re == 0.0 { 0.0 } else { z.re }, z.im);   // an exact zero has no sign
        // a value inside its own rounding bound is zero BEFORE its sign is judged: g*sin(423 turns) is exactly 0,
        // the -3.5e-13 that rounding leaves must not be refused as a negative acceleration
        if let Ok((_, ez)) = used.eval_bound(values) {
            if z.re != 0.0 && z.re.abs() <= ez {
                z = crate::num::C::r(0.0);
            }
        }
        let mut polished_err = None;
        if z.re != 0.0 {
            if let Some((x, e)) = polish(rel, values, target, z.re) {
                z = crate::num::C::r(x);
                polished_err = Some(e);
            }
        }
        if !admissible(sign, z.re) {
            wrong_sign.push(z.re);
            continue;
        }
        // a candidate must satisfy the equation as written: a solved form can carry a branch the written equation
        // cannot take (theta = acos(..) never exceeds pi), or divide by zero there (1/f with f = 0)
        let mut with = values.clone();
        with.insert(target.to_string(), z.re);
        match residual(rel, &with) {
            Some((res, res_err)) if res <= 1e-9 || res <= 4.0 * res_err => {}
            _ => continue,
        }
        // one value, not two, when two branches meet to a millionth (theta = asin(1) and pi - asin(1) at 90 degrees,
        // which rounding leaves 2e-8 apart; owner 2026-10-30: answers are held to 4 significant digits)
        if !roots.iter().any(|r| (z.re - r).abs() <= 1e-6 * 1.0f64.max(r.abs())) {
            roots.push(z.re);
            let size = || -> Option<f64> {
                let mut m = 0.0f64;
                for t in f.terms.iter().chain(f.bounded_terms.iter()) {
                    let v = t.eval(values).ok()?;
                    if !v.is_finite() {
                        return None;
                    }
                    m = m.max(v.abs());
                }
                Some(m)
            };
            terms.push(size().unwrap_or(z.re.abs()));
            // the tighter of: the solved form's own rounding bound, the refined value's bound from the equation
            let form_err = used.eval_bound(values).map(|(_, e)| e).unwrap_or(f64::INFINITY);
            errs.push(form_err.min(polished_err.unwrap_or(f64::INFINITY)));
        }
    }
    if roots.is_empty() {
        if let Some(l) = out_of_range {
            return Err(Why::Range { name: target.into(), log10: l });
        }
        if let Some((s, l)) = huge_wrong_sign {
            return Err(Why::RangeSign { name: target.into(), sign: s, log10: l });
        }
        if unreliable {
            return Err(Why::Unreliable { name: target.into() });
        }
        return Err(if wrong_sign.is_empty() {
            Why::NoReal { name: target.into() }
        } else {
            Why::ResultSign { name: target.into(), values: wrong_sign }
        });
    }
    if roots.len() > 1 {
        return Err(Why::Branches { name: target.into(), values: roots });
    }
    let v = roots[0];
    // zero when the value is below its own error bound: rounding noise of a cancellation, not a measurement. (The
    // Python reference compares with the size of the expanded terms instead, which for C*R*log(emf/(emf - V))
    // means log(emf) and log(emf - V), 10^8 times the true value: it zeroed a correct 3.5e-5 s.)
    let _ = &terms;
    if errs[0].is_finite() && v.abs() <= errs[0] {
        // rounding noise of a cancellation: the value is zero, and zero must itself be allowed (a torque at exactly
        // 110 turns is 0, which a quantity that must be positive cannot be)
        if !admissible(sign, 0.0) {
            return Err(Why::ResultSign { name: target.into(), values: vec![0.0] });
        }
        return Ok((0.0, errs[0]));
    }
    Ok((v, errs[0]))
}

/// |lhs - rhs| relative to the size of the equation's terms (relation.py Relation.residual).
/// None when a term of the equation is undefined at these values (a division by zero): the formula does not hold
/// there, whatever value was computed. (The Python reference reads that case as a residual of 0 and answers;
/// rust/models/<model>/FINDINGS.md.)
/// Also returns the bound on the residual that rounding alone can produce (relative to the same scale): an equation
/// written with a cancellation (sqrt(k/m - b^2/(4 m^2)) with the two terms nearly equal) cannot show a residual
/// below that, however exact the answer.
fn residual(rel: &Relation, values: &HashMap<String, f64>) -> Option<(f64, f64)> {
    let mut sum = crate::num::C::r(0.0);
    let (mut abs_sum, mut err) = (0.0, 0.0);
    for t in &rel.residual_terms {
        let (v, e) = t.eval_bound(values).ok().filter(|(v, _)| v.is_finite())?;
        sum = sum.add(v);
        abs_sum += v.abs();
        err += e;
    }
    let mut bounded = 0.0;
    for t in &rel.residual_bounded {
        bounded += t.eval(values).ok().filter(|v| v.is_finite())?.abs();
    }
    let scale = abs_sum.max(bounded);
    Some(if scale > 0.0 { (sum.abs() / scale, err / scale) } else { (0.0, 0.0) })
}

fn must(sign: &str) -> &'static str {
    match sign {
        "positive" => "greater than zero",
        "nonnegative" => "zero or more",
        _ => "valid",
    }
}

/// product.py `_plain_refusal`.
fn plain_refusal(rel: &Relation, p: &Pack, why: &Why, sources: &HashMap<String, String>) -> String {
    let name = match why {
        Why::InputSign { name, .. } | Why::ResultSign { name, .. } | Why::NoReal { name } | Why::Branches { name, .. }
        | Why::Range { name, .. } | Why::Unreliable { name } | Why::RangeSign { name, .. } => name,
    };
    let Some(s) = rel.sym(name) else { return String::new() };
    let what = if s.desc.is_empty() { s.kind.replace('_', " ") } else { s.desc.clone() };
    let unit = p.kinds.get(&s.kind).map(|k| k.unit.clone()).filter(|u| !u.is_empty() && u != "1").unwrap_or_default();
    let amount = |v: f64| format!("{}{}", py_g(v, 4), if unit.is_empty() { String::new() } else { format!(" {unit}") });
    let check = "These inputs cannot all hold at once: check each value and its unit.";
    match why {
        Why::InputSign { value, .. } => {
            let given = sources.get(name).cloned().unwrap_or_else(|| amount(*value));
            format!("{what} ({name}) must be {}, but it was given as {given}. Check this input.", must(&s.sign))
        }
        Why::ResultSign { values, .. } if !values.is_empty() => format!(
            "with these inputs, {what} ({name}) would come out as {}, which is impossible: it must be {}. {check}",
            amount(values[0]), must(&s.sign)),
        Why::NoReal { .. } => format!(
            "with these inputs, no real value of {what} ({name}) satisfies the formula (it would need, for example, \
             the square root or logarithm of a negative number, or a division by zero). {check}"),
        Why::Branches { values, .. } if !values.is_empty() => {
            let vals: Vec<String> = values.iter().map(|v| amount(*v)).collect();
            format!("these inputs allow {} values of {what} ({name}): {}. The formula alone cannot choose between \
                     them; use a capability or a condition that tells them apart.", values.len(), vals.join(", "))
        }
        Why::Range { log10, .. } => {
            let unit = if unit.is_empty() { String::new() } else { format!(" {unit}") };
            if *log10 < 0.0 {
                format!("with these inputs, {what} ({name}) comes out at about 1e{:.0}{unit}, smaller than the smallest \
                         number this program can represent (about 1e-308): it is effectively zero.", log10.floor())
            } else {
                format!("with these inputs, {what} ({name}) comes out at about 1e{:.0}{unit}, larger than the largest \
                         number this program can represent (about 1e308).", log10.floor())
            }
        }
        Why::RangeSign { sign, log10, .. } => {
            let unit = if unit.is_empty() { String::new() } else { format!(" {unit}") };
            format!("with these inputs, {what} ({name}) would come out at about {}1e{:.0}{unit}, which is impossible: \
                     it must be {}. {check}", if *sign < 0.0 { "-" } else { "" }, log10.floor(), must(&s.sign))
        }
        Why::Unreliable { .. } => format!(
            "with these inputs {what} ({name}) cannot be computed reliably: nearly equal quantities cancel in the \
             formula and no digit of the result survives in double precision. Check each value and its unit."),
        _ => String::new(),
    }
}

/// product.py `_symbol_of`.
fn symbol_of(rel: &Relation, p: &Pack, name: &str) -> Option<String> {
    if rel.sym(name).is_some() {
        return Some(name.to_string());
    }
    let low = name.trim().to_lowercase().replace('_', " ");
    if let Some((k, r)) = name.split_once('/') {
        let role = if r.is_empty() { "plain" } else { r };
        let hit: Vec<_> = rel.symbols.iter().filter(|s| s.kind == k && s.role == role).collect();
        return if hit.len() == 1 { Some(hit[0].name.clone()) } else { None };
    }
    let by_kind: Vec<_> = rel.symbols.iter().filter(|s| s.kind == low.replace(' ', "_")).collect();
    if by_kind.len() == 1 {
        return Some(by_kind[0].name.clone());
    }
    if let Some((k, r)) = p.labels.get(&low) {
        let exact: Vec<_> = rel.symbols.iter().filter(|s| &s.kind == k && &s.role == r).collect();
        let same_kind: Vec<_> = rel.symbols.iter().filter(|s| &s.kind == k).collect();
        let hit = if exact.is_empty() { same_kind } else { exact };
        return if hit.len() == 1 { Some(hit[0].name.clone()) } else { None };
    }
    let by_desc: Vec<_> = rel.symbols.iter().filter(|s| !s.desc.is_empty() && low == s.desc.to_lowercase()).collect();
    if by_desc.len() == 1 { Some(by_desc[0].name.clone()) } else { None }
}

/// product.py `_value`: (number, unit as written) or None.
fn read_value(v: &Input, unit_si: &str) -> Option<(f64, String)> {
    match v {
        Input::Int(i) => Some((*i as f64, unit_si.to_string())),
        Input::Float(f) => Some((*f, unit_si.to_string())),
        Input::Json(_) => None,
        Input::Text(t) => {
            // \s*([-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?)\s*(.*)$
            let cs: Vec<char> = t.chars().collect();
            let mut i = 0;
            while i < cs.len() && cs[i].is_whitespace() {
                i += 1;
            }
            let start = i;
            if i < cs.len() && (cs[i] == '+' || cs[i] == '-') {
                i += 1;
            }
            let d0 = i;
            while i < cs.len() && cs[i].is_ascii_digit() {
                i += 1;
            }
            if i > d0 {
                if i < cs.len() && cs[i] == '.' {
                    i += 1;
                    while i < cs.len() && cs[i].is_ascii_digit() {
                        i += 1;
                    }
                }
            } else if i < cs.len() && cs[i] == '.' && i + 1 < cs.len() && cs[i + 1].is_ascii_digit() {
                i += 1;
                while i < cs.len() && cs[i].is_ascii_digit() {
                    i += 1;
                }
            } else {
                return None;
            }
            if i < cs.len() && (cs[i] == 'e' || cs[i] == 'E') {
                let mut j = i + 1;
                if j < cs.len() && (cs[j] == '+' || cs[j] == '-') {
                    j += 1;
                }
                let e0 = j;
                while j < cs.len() && cs[j].is_ascii_digit() {
                    j += 1;
                }
                if j > e0 {
                    i = j;
                }
            }
            let num: String = cs[start..i].iter().collect();
            let rest: String = cs[i..].iter().collect::<String>().trim().to_string();
            let x = num.parse::<f64>().ok()?;
            Some((x, if rest.is_empty() { unit_si.to_string() } else { rest }))
        }
    }
}

/// A number written as Python's float(Fraction(text)) reads it: "3", "-1.5", "2e-3", "3/4" (spaces ignored).
fn fraction_number(text: &str) -> Option<f64> {
    let t: String = text.chars().filter(|c| !c.is_whitespace()).collect();
    if let Some((a, b)) = t.split_once('/') {
        let (a, b) = (a.parse::<f64>().ok()?, b.parse::<f64>().ok()?);
        return if b == 0.0 { None } else { Some(a / b) };
    }
    if t.is_empty() || t.chars().any(|c| c.is_alphabetic() && c != 'e' && c != 'E') {
        return None;
    }
    t.parse::<f64>().ok()
}

/// runtime/product.py `_solve_procedure` for a procedure written as data (runtime/procedure_spec.CompiledProcedure).
pub fn solve_procedure(p: &Pack, key: &str, given: &[(String, Input)]) -> Answer {
    let Some(pr) = p.procedure(key) else {
        return Answer::refuse("INVALID_INPUT", format!("unknown capability '{key}'"));
    };
    let unit_of = |kind: &str| -> String {
        if kind == "number" { String::new() } else { p.kinds.get(kind).map(|k| k.unit.clone()).unwrap_or_default() }
    };
    let mut values: HashMap<String, f64> = HashMap::new();
    let mut texts: HashMap<String, String> = HashMap::new();
    // every input as read (numbers in SI, and the structured ones): what a native procedure receives
    let mut vals: HashMap<String, Val> = HashMap::new();
    let mut order: Vec<String> = vec![];
    for (name, v) in given {
        let Some(inp) = pr.inputs.iter().find(|i| &i.symbol == name) else {
            let takes: Vec<String> = pr.inputs.iter().map(|i| format!("{} ({})", i.symbol, i.meaning)).collect();
            return Answer::refuse("INVALID_INPUT", format!("'{name}' is not an input of {key}. It takes: {}.",
                                                           takes.join(", ")));
        };
        if !matches!(inp.ty.as_str(), "number" | "") {
            // runtime/inputs.py coerce: a whole number, a list, a matrix, text, one of the choices, true or false
            if inp.ty != "choice" && !pr.native {
                return Answer::refuse("INVALID_INPUT", format!("{name}: a {} input is not supported by this build.",
                                                               inp.ty));
            }
            match crate::coerce::coerce(inp, v) {
                Ok(val) => {
                    if let Val::Text(t) = &val {
                        texts.insert(name.clone(), t.clone());
                    }
                    vals.insert(name.clone(), val);
                }
                Err(crate::coerce::Bad::Input(m)) => return Answer::refuse("INVALID_INPUT", format!("{m}.")),
                Err(crate::coerce::Bad::Type) => return Answer::refuse("INVALID_INPUT", format!(
                    "{name}: expected a {}, got {}.", inp.ty, v.repr())),
            }
            continue;
        }
        let value = if let Some(kind) = (inp.kind != "number").then(|| p.kinds.get(&inp.kind)).flatten() {
            let bad = || Answer::refuse("INVALID_INPUT", format!(
                "cannot read {name}={}: give a number and a unit of {} (e.g. '3 {}').", v.repr(),
                kind.name.replace('_', " "), kind.unit));
            let Some((number, unit)) = read_value(v, &kind.unit) else { return bad() };
            let Ok(u) = parse_unit(&unit) else { return bad() };
            if u.dim != kind.dim {
                return Answer::refuse("INVALID_INPUT", format!("{name} is a {}: its unit must be like {}, not '{unit}'.",
                                                               kind.name.replace('_', " "), kind.unit));
            }
            to_si(number, &u, kind.difference)
        } else {
            let x = match v {
                Input::Int(i) => Some(*i as f64),
                Input::Float(f) => Some(*f),
                Input::Text(t) => fraction_number(t),
                // float(True) is 1.0 in Python; float of a list or None raises
                Input::Json(serde_json::Value::Bool(b)) => Some(if *b { 1.0 } else { 0.0 }),
                Input::Json(_) => None,
            };
            let Some(x) = x else {
                return Answer::refuse("INVALID_INPUT", format!("{name}={} is not a number.", v.repr()));
            };
            x
        };
        if !values.contains_key(name) {
            order.push(name.clone());
        }
        values.insert(name.clone(), value);
        vals.insert(name.clone(), Val::Num(value));
    }
    let needs: Vec<&crate::pack::ProcInput> = pr.inputs.iter()
        .filter(|i| i.required && !vals.contains_key(&i.symbol)).collect();
    if !needs.is_empty() {
        return Answer::refuse("NEEDS_INPUT", format!("{key} needs {} ({}). Example: {}.",
            needs.iter().map(|i| i.symbol.as_str()).collect::<Vec<_>>().join(", "),
            needs.iter().map(|i| i.meaning.as_str()).collect::<Vec<_>>().join("; "), pr.structured_example));
    }
    let si: Vec<(String, f64)> = order.iter().map(|n| (n.clone(), values[n])).collect();
    let not_applicable = |why: String| {
        let mut a = Answer::refuse("INVALID_INPUT", format!("{}: {why}.", pr.name));
        a.solved_for = Some(pr.primary.clone());
        a.si = si.clone();
        a
    };
    for i in &pr.inputs {
        if i.kind == "integer" {
            if let Some(v) = values.get(&i.symbol) {
                if *v != v.trunc() {
                    return not_applicable(format!("{} must be a whole number", i.symbol));
                }
            }
        }
    }
    for (text, diff, op) in &pr.conditions {
        let d = diff.eval(&values).ok();
        let d = d.filter(|z| z.im.abs() <= 1e-12 * 1.0f64.max(z.re.abs())).map(|z| z.re).unwrap_or(f64::NAN);
        let ok = match op.as_str() {
            "!=" => d.abs() > 1e-12,
            ">" => d > 0.0,
            "<" => d < 0.0,
            ">=" => d >= 0.0,
            "<=" => d <= 0.0,
            _ => false,
        };
        if !ok {
            return not_applicable(format!("the condition {text} does not hold for these inputs"));
        }
    }
    if pr.native {                                       // an algorithm the model's crate implements (ported by hand)
        let Some(native) = p.native else { return internal_error() };
        let (outs, residual) = match native(key, &vals) {
            None => return internal_error(),
            Some(Err(why)) => return not_applicable(why),
            Some(Ok(r)) => r,
        };
        let outs: Vec<(String, serde_json::Value, String)> = outs.into_iter().map(|(s, v)| {
            let kind = pr.outputs.iter().find(|o| o.symbol == s).map(|o| o.kind.clone()).unwrap_or_default();
            (s, v, unit_of(&kind))
        }).collect();
        return finish(p, pr, outs, residual, si, &order, &values, &texts, unit_of);
    }
    let mut outs: Vec<(String, serde_json::Value, String)> = vec![];
    for o in &pr.outputs {
        let Some(expr) = &o.expr else { return internal_error() };
        let z = match expr.eval(&values) {
            Ok(z) => z,
            Err(_) => return internal_error(),
        };
        if !z.is_finite() {
            return internal_error();                     // the reference divides by zero here: a defect, not an answer
        }
        if z.im.abs() > 1e-12 * 1.0f64.max(z.re.abs()) {
            return not_applicable(format!("{} has no real value for these inputs", o.symbol));
        }
        // an output that underflowed to 0 (or a subnormal) while its exact size is below double precision is not an
        // answer to ten digits: 1.963 * 0.022^379 is about 1e-628, never 0.0 (the formula engine refuses it alike)
        if z.re == 0.0 || z.re.abs() < f64::MIN_POSITIVE {
            if let Some(l) = expr.log10_abs(&values).filter(|l| l.is_finite() && *l < -307.0) {
                return not_applicable(format!(
                    "with these inputs, {} comes out at about 1e{:.0}, smaller than the smallest number this program \
                     can represent (about 1e-308): it is effectively zero.", o.symbol, l.floor()));
            }
        }
        outs.push((o.symbol.clone(), serde_json::json!(z.re + 0.0), unit_of(&o.kind)));
    }
    // the outputs substituted back into the defining equations
    let mut all = values.clone();
    for (s, v, _) in &outs {
        all.insert(s.clone(), v.as_f64().unwrap_or(f64::NAN));
    }
    let mut worst = 0.0f64;
    for (l, r) in &pr.equations {
        match (l.eval(&all), r.eval(&all)) {
            (Ok(a), Ok(b)) => {
                let res = crate::num::C::new(a.re - b.re, a.im - b.im).abs();
                worst = worst.max(res / 1.0f64.max(a.abs() + b.abs()));
            }
            _ => worst = f64::INFINITY,
        }
    }
    finish(p, pr, outs, worst, si, &order, &values, &texts, unit_of)
}

/// The answer of a procedure from its outputs and the residual of its check (shared by both kinds of procedure).
#[allow(clippy::too_many_arguments)]
fn finish(p: &Pack, pr: &crate::pack::Procedure, outs: Vec<(String, serde_json::Value, String)>, worst: f64,
          si: Vec<(String, f64)>, order: &[String], values: &HashMap<String, f64>, texts: &HashMap<String, String>,
          unit_of: impl Fn(&str) -> String) -> Answer {
    let _ = p;
    let primary_kind = pr.outputs.iter().find(|o| o.symbol == pr.primary).map(|o| o.kind.clone()).unwrap_or_default();
    let ok = worst <= 1e-9;
    // the answer's value: the primary output when it is a number (Python: isinstance(primary, (int, float)))
    let primary = outs.iter().find(|(s, _, _)| s == &pr.primary).and_then(|(_, v, _)| v.as_f64());
    let mut a = Answer::refuse(if ok { "SOLVED" } else { "INTERNAL_ERROR" },
                               if ok { String::new() } else {
                                   "I computed a value but it did not pass the checks, so I am not showing it.".into() });
    a.value = if ok { primary } else { None };
    a.unit = Some(unit_of(&primary_kind));
    a.solved_for = Some(pr.primary.clone());
    a.si = si;
    a.values = if ok { outs } else { vec![] };
    a.target = Some(pr.primary.clone());
    a.phrase = pr.outputs.first().map(|o| o.meaning.clone()).unwrap_or_default();
    a.name = pr.name.clone();
    a.equation = pr.method.clone();
    a.assumptions = pr.assumptions.clone();
    a.inputs = order.iter().map(|n| {
        let k = pr.inputs.iter().find(|i| &i.symbol == n).map(|i| unit_of(&i.kind)).unwrap_or_default();
        (n.clone(), values[n], k, String::new())
    }).collect();
    // a text or choice input, as given, in the capability's own input order (a HashMap's order changes from one
    // process to the next: the command line printed "f" and "g" in either order, 2026-11-04); any other text after
    let mut text_names: Vec<&String> = pr.inputs.iter().map(|i| &i.symbol).filter(|s| texts.contains_key(*s)).collect();
    let mut rest: Vec<&String> = texts.keys().filter(|k| !text_names.contains(k)).collect();
    rest.sort();
    text_names.extend(rest);
    for n in text_names {
        a.inputs.push((n.clone(), f64::NAN, String::new(), texts[n].clone()));
    }
    a
}

fn internal_error() -> Answer {
    Answer::refuse("INTERNAL_ERROR", "Internal error: this is a defect in the engine, not a property of your question. \
                                      Please report it with the question.".into())
}

/// solve(key, inputs, target): the capability contract path of the Python package (a formula or a procedure).
pub fn solve(p: &Pack, key: &str, given: &[(String, Input)], target: Option<&str>) -> Answer {
    if p.procedure(key).is_some() {
        return solve_procedure(p, key, given);
    }
    let Some(rel) = p.relation(key) else {
        return Answer::refuse("INVALID_INPUT", format!("unknown capability '{key}'"));
    };
    let kunit = |kind: &str| p.kinds.get(kind).map(|k| k.unit.clone()).unwrap_or_default();
    let takes: Vec<String> = rel.symbols.iter().map(|s| {
        format!("{} ({}{}, {})", s.name, s.kind.replace('_', " "),
                if s.role != "plain" { format!("/{}", s.role) } else { String::new() }, kunit(&s.kind))
    }).collect();
    let takes = takes.join(", ");
    let mut values: HashMap<String, f64> = HashMap::new();
    let mut order: Vec<String> = vec![];
    let mut sources: HashMap<String, String> = HashMap::new();
    for (name, v) in given {
        let Some(sym) = symbol_of(rel, p, name) else {
            return Answer::refuse("INVALID_INPUT", format!("'{name}' is not an input of {key}. It takes: {takes}."));
        };
        let s = rel.sym(&sym).unwrap();
        let kind = &p.kinds[&s.kind];
        let bad = || Answer::refuse("INVALID_INPUT", format!(
            "cannot read {name}={}: give a number and a unit of {} (e.g. '3 {}').", v.repr(),
            kind.name.replace('_', " "), kind.unit));
        let Some((number, unit)) = read_value(v, &kind.unit) else { return bad() };
        let Ok(u) = parse_unit(&unit) else { return bad() };
        if u.dim != kind.dim {
            return Answer::refuse("INVALID_INPUT", format!("{name} is a {}: its unit must be like {}, not '{unit}'.",
                                                           kind.name.replace('_', " "), kind.unit));
        }
        if !values.contains_key(&sym) {
            order.push(sym.clone());
        }
        values.insert(sym.clone(), to_si(number, &u, kind.difference));
        sources.insert(sym.clone(), format!("{} {}", py_g(number, 6), unit).trim().to_string());
    }
    let mut tgt: Option<String> = None;
    if let Some(t) = target {
        let Some(sym) = symbol_of(rel, p, t) else {
            return Answer::refuse("INVALID_INPUT", format!("target '{t}' is not a quantity of {key}. It has: {takes}."));
        };
        if values.contains_key(&sym) {
            return Answer::refuse("INVALID_INPUT", format!("{sym} is given as an input and asked for as the target."));
        }
        tgt = Some(sym);
    }
    let mut assumptions = rel.assumptions.clone();
    for s in &rel.symbols {
        if !values.contains_key(&s.name) && tgt.as_deref() != Some(&s.name) {
            if let Some(d) = &s.default {
                let Ok(u) = parse_unit(&d.unit) else { continue };
                values.insert(s.name.clone(), to_si(d.value, &u, false));
                order.push(s.name.clone());
                sources.insert(s.name.clone(), format!("default: {}", d.assumption.clone().unwrap_or_default()));
                assumptions.push(d.assumption.clone().unwrap_or_else(|| format!("default {}", s.name)));
            }
        }
    }
    let unknown: Vec<String> = rel.symbols.iter().filter(|s| !values.contains_key(&s.name)).map(|s| s.name.clone()).collect();
    if tgt.is_none() {
        tgt = if unknown.len() == 1 {
            Some(unknown[0].clone())
        } else if unknown.contains(&rel.lhs) {
            Some(rel.lhs.clone())
        } else {
            None
        };
    }
    if unknown.is_empty() {
        return Answer::refuse("INVALID_INPUT", format!(
            "every quantity of {key} is given; leave out the one to compute (or pass target=)."));
    }
    let needs: Vec<String> = unknown.iter().filter(|n| Some(n.as_str()) != tgt.as_deref()).cloned().collect();
    if tgt.is_none() || !needs.is_empty() {
        let skip = tgt.clone().unwrap_or_else(|| rel.lhs.clone());
        let example: Vec<String> = rel.symbols.iter().filter(|s| s.name != skip)
            .map(|s| format!("{}='... {}'", s.name, kunit(&s.kind))).collect();
        let missing: Vec<String> = needs.iter().map(|n| {
            let s = rel.sym(n).unwrap();
            format!("{} ({}, {})", n, if s.desc.is_empty() { s.kind.clone() } else { s.desc.clone() }, kunit(&s.kind))
        }).collect();
        let missing = if missing.is_empty() { "to know which quantity to compute (pass target=)".to_string() } else { missing.join(", ") };
        return Answer::refuse("NEEDS_INPUT", format!("{key} needs {missing}. Example: solve({}, {}).",
                                                     py_repr_str(key), example.join(", ")));
    }
    let tgt = tgt.unwrap();
    let si: Vec<(String, f64)> = order.iter().map(|n| (n.clone(), values[n])).collect();
    let (value, value_err) = match solve_relation(rel, &tgt, &values) {
        Ok(v) => v,
        Err(why) => {
            let outcome = match why {
                Why::Branches { .. } => "AMBIGUOUS",
                Why::Range { .. } | Why::Unreliable { .. } => "OUT_OF_SCOPE",
                _ => "INVALID_INPUT",
            };
            let plain = plain_refusal(rel, p, &why, &sources);
            let fallback = match &why {
                Why::InputSign { name, value } => {
                    let sign = rel.sym(name).map(|s| s.sign.clone()).unwrap_or_default();
                    format!("{name} = {} violates '{sign}'.", py_g(*value, 6))
                }
                Why::Branches { name, values } => format!("{} admissible values for {name} ({}); the branch is not determined.",
                    values.len(), values.iter().map(|v| py_g(*v, 6)).collect::<Vec<_>>().join(", ")),
                Why::ResultSign { name, .. } | Why::NoReal { name } | Why::Range { name, .. } =>
                    format!("no admissible real value for {name}."),
                Why::Unreliable { name } => format!("{name} cannot be computed reliably."),
                Why::RangeSign { name, .. } => format!("no admissible real value for {name}."),
            };
            let mut a = Answer::refuse(outcome, format!("{}: {}", rel.name, if plain.is_empty() { fallback } else { plain }));
            a.solved_for = Some(tgt.clone());
            a.si = si;
            return a;
        }
    };
    let mut all = values.clone();
    all.insert(tgt.clone(), value);
    let Some((res, res_err)) = residual(rel, &all) else {
        // the formula is undefined at these values (a division by zero): no value satisfies it
        let why = Why::NoReal { name: tgt.clone() };
        let mut a = Answer::refuse("INVALID_INPUT", format!("{}: {}", rel.name, plain_refusal(rel, p, &why, &sources)));
        a.solved_for = Some(tgt.clone());
        a.si = si;
        return a;
    };
    // the answer satisfies its equation: to 1e-9 of the terms, or as closely as rounding lets the equation be checked
    let satisfies = res <= 1e-9 || res <= 4.0 * res_err;
    // five significant digits must be certain (the rounding bound of the value below 1e-5 of it): one more than the
    // 4 digits answers are held to (owner, 2026-10-30; it was ten digits, 2026-10-28). Fewer, and the answer says how
    // many and is not given as a result
    let rel_err = if value != 0.0 { value_err / value.abs() } else { 0.0 };
    if satisfies && rel.dimensions_consistent && rel_err > 1e-5 {
        let s = rel.sym(&tgt).unwrap();
        let what = if s.desc.is_empty() { s.kind.replace('_', " ") } else { s.desc.clone() };
        let u = kunit(&s.kind);
        let u = if u.is_empty() || u == "1" { String::new() } else { format!(" {u}") };
        let digits = (-rel_err.log10_lm()).floor().max(0.0) as usize;
        let msg = if digits >= 2 {
            format!("{}: with these inputs only about {digits} significant digits of {what} ({tgt}) are certain \
                     (nearly equal quantities cancel in the formula): {what} ≈ {}{u}. Give the inputs with more \
                     significant digits, or check that they are not almost equal by mistake.", rel.name,
                    py_g(value, digits))
        } else {
            format!("{}: with these inputs {what} ({tgt}) cannot be computed reliably: nearly equal quantities \
                     cancel in the formula. Give the inputs with more significant digits, or check that they are not \
                     almost equal by mistake.", rel.name)
        };
        let mut a = Answer::refuse("OUT_OF_SCOPE", msg);
        a.solved_for = Some(tgt.clone());
        a.si = si;
        return a;
    }
    let ok = satisfies && rel.dimensions_consistent;
    if !rel.requires.is_empty() {
        assumptions.push(format!("the situation is: {}", rel.requires.iter().map(|t| t.replace('_', " ")).collect::<Vec<_>>().join(", ")));
    }
    let s = rel.sym(&tgt).unwrap();
    let unit = kunit(&s.kind);
    let inputs = order.iter().map(|n| {
        let k = rel.sym(n).map(|x| kunit(&x.kind)).unwrap_or_default();
        (n.clone(), values[n], k, sources.get(n).cloned().unwrap_or_default())
    }).collect();
    Answer {
        outcome: if ok { "SOLVED".into() } else { "INTERNAL_ERROR".into() },
        message: String::new(),
        value: if ok { Some(value) } else { None },
        unit: Some(unit),
        solved_for: Some(tgt.clone()),
        si,
        values: vec![],
        target: Some(tgt.clone()),
        phrase: if s.desc.is_empty() { s.kind.replace('_', " ") } else { s.desc.clone() },
        assumptions,
        name: rel.name.clone(),
        equation: rel.equation.clone(),
        inputs,
    }
}
