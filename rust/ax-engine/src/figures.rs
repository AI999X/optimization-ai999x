//! Live figures for one capability and one visitor's inputs, drawn from what the model computes: its answer, and the
//! same capability solved again around those inputs (a sweep of one input, each choice of a choice input, prefixes of a
//! sequence, a grid of two inputs). Domain-neutral: the figure kinds follow the capability's metadata in the pack
//! (input types, units, choices) and the shape of its answer (a number, a list, a matrix, a complex root, a verdict),
//! never a capability's name.
//!
//!   figures_json(pack, request) -> [{"slug", "title", "svg", "caption"}]      (request as for solve_json)
//!
//! Bounded: at most MAX_SOLVES solves per request (and, natively, MAX_MS of wall time), a work allowance counted in
//! the runtime's operations (meter.rs: deterministic; a solve past its share is stopped and left out), at most
//! MAX_FIGS figures, at most 41 points per sweep (a scan of whole numbers: at most 800 tried), 11 x 11 cells per grid,
//! 64 rows per truth table; lists, matrices and graphs beyond a drawable size are not drawn.
//! The SVG uses the website's figure classes (axp-*: its own colours in light and dark), with a fallback style for a
//! file opened on its own (the page removes <style> and applies its theme).

use crate::engine::{solve, Answer, Input};
use crate::num::{py_g, py_repr_f64};
use crate::pack::{Pack, ProcInput, Val};
use serde_json::{json, Value};

const MAX_SOLVES: usize = 400;
/// the browser's default solves, and natively a request's with "clock": false (the Python package)
const DEFAULT_WASM_SOLVES: usize = 120;
#[cfg(not(target_arch = "wasm32"))]
const MAX_MS: u128 = 250;
const MAX_FIGS: usize = 4;
const SWEEP_N: usize = 41;
const SWEEP_N2: usize = 29;
const GRID_N: usize = 11;
const MAX_LIST: usize = 400;
const MAX_KNOBS: usize = 10;

/// One drawn figure.
#[derive(Clone, Debug)]
pub struct Figure {
    pub slug: String,
    /// what it shows: a model's own figure carries its published kind ("schematic", "bode", ...: figkit.rs); a
    /// generic one the kind it draws ("curve", "sensitivity", "bars", "matrix", "decision", "series", ...)
    pub kind: String,
    pub title: String,
    pub svg: String,
    pub caption: String,
}

/// A request as JSON text (as `solve_json` reads it) -> the figures as JSON text; "[]" when the request cannot be read
/// or the inputs are not solved.
pub fn figures_json(p: &Pack, request: &str) -> String {
    let figs = match serde_json::from_str::<Value>(request) {
        Ok(case) => {
            let mut given = vec![];
            match &case["inputs"] {
                Value::Array(pairs) => {
                    for pair in pairs.iter().take(64) {
                        given.push((pair[0].as_str().unwrap_or("").to_string(), crate::input_of(&pair[1])));
                    }
                }
                Value::Object(map) => {
                    for (k, v) in map.iter().take(64) {
                        given.push((k.clone(), crate::input_of(v)));
                    }
                }
                _ => {}
            }
            // "budget": how many solves the caller can afford (the page measures one solve and sends what fits);
            // "ms": that solve's time, which sizes the work allowed (ALLOW_MS of work at the measured rate)
            let budget = case["budget"].as_u64().map(|b| b as usize);
            let ms = case["ms"].as_f64().filter(|m| m.is_finite() && *m > 0.0);
            let recalled = recall(&case);
            // "clock": false (natively): no wall-clock limit and the browser's default budget, so the figures are the
            // browser module's, the same on every machine and every run (the Python package; docs/PYTHON_ON_RUST.md)
            let clock = case["clock"].as_bool().unwrap_or(true);
            figures_from(p, case["key"].as_str().unwrap_or(""), &given, case["target"].as_str(), budget, ms, recalled,
                         clock)
        }
        Err(_) => vec![],
    };
    let out: Vec<Value> = figs.iter()
        .map(|f| json!({"slug": f.slug, "kind": f.kind, "title": f.title, "svg": f.svg, "caption": f.caption}))
        .collect();
    serde_json::to_string(&out).unwrap_or_else(|_| "[]".into())
}

// ------------------------------------------------------------------------------------------------ the last answer
// The page solves a request, then asks for its figures: the answer of the last solve_json request (and the work it
// took) is kept, so the figures need not solve the visitor's inputs again. Only an identical request (key, inputs,
// target) uses it; one entry.
thread_local! {
    static LAST: std::cell::RefCell<Option<(String, Answer, u64)>> = const { std::cell::RefCell::new(None) };
}

fn request_id(case: &Value) -> String {
    serde_json::to_string(&json!([case["key"], case["inputs"], case["target"]])).unwrap_or_default()
}

/// Keep this request's answer and its work for the figures asked for next.
pub fn remember(case: &Value, a: &Answer, work: u64) {
    let id = request_id(case);
    LAST.with(|l| *l.borrow_mut() = if a.solved() { Some((id, a.clone(), work)) } else { None });
}

fn recall(case: &Value) -> Option<(Answer, u64)> {
    let id = request_id(case);
    LAST.with(|l| l.borrow().as_ref().filter(|x| x.0 == id).map(|x| (x.1.clone(), x.2)))
}

/// The work the figures may spend, in milliseconds of the visitor's measured solve (the page's quick limit is 1 s).
const ALLOW_MS: f64 = 450.0;
/// A solve whose counted work is at least this is measured well by the count (its time is in the counted
/// operations): its figures may spend more, cheaper solves within the same work.
const WORK_TRUST: u64 = 20_000;

// ------------------------------------------------------------------------------------------------ solving, bounded
struct Ctx<'a> {
    p: &'a Pack,
    key: &'a str,
    target: Option<&'a str>,
    /// solves made here (the visitor's inputs solved again when the page's answer is not at hand counts), and the
    /// most allowed
    solves: usize,
    budget: usize,
    /// the page's measured time of the visitor's solve, when sent
    ms: Option<f64>,
    /// the work of the visitor's own solve (meter units, at least WORK_FLOOR), once known; the work of the solves
    /// after it and how many; the work spent here in all. The work allowed is ALLOW_MS at the visitor's rate (or
    /// `budget` solves of its cost, with half again as slack); a solve that would cost more than its share is
    /// stopped (deterministic: counted operations, not time)
    w1: Option<u64>,
    wextra: u64,
    extra: usize,
    wspent: u64,
    /// the last solve was stopped at its work limit (not a refusal: too costly to compute here)
    stopped: bool,
    #[cfg(not(target_arch = "wasm32"))]
    t0: std::time::Instant,
    /// natively: whether the wall-clock limit applies (false: only counted work, as in the browser)
    #[cfg(not(target_arch = "wasm32"))]
    clock: bool,
}

/// The least work a solve is taken to cost (a solve whose time goes to parsing or native code counts little).
const WORK_FLOOR: u64 = 2000;

impl<'a> Ctx<'a> {
    fn new(p: &'a Pack, key: &'a str, target: Option<&'a str>, budget: usize, ms: Option<f64>) -> Ctx<'a> {
        Ctx { p, key, target, solves: 0, budget, ms, w1: None, wextra: 0, extra: 0, wspent: 0, stopped: false,
              #[cfg(not(target_arch = "wasm32"))] t0: std::time::Instant::now(),
              #[cfg(not(target_arch = "wasm32"))] clock: true }
    }
    fn w1(&self) -> u64 {
        self.w1.unwrap_or(WORK_FLOOR)
    }
    fn wbudget(&self) -> u64 {
        match self.ms {
            Some(ms) => (self.w1() as f64 * (ALLOW_MS / ms).min(MAX_SOLVES as f64)) as u64,
            None => self.w1().saturating_mul(self.budget as u64).saturating_mul(3) / 2,
        }
    }
    /// The most solves: the budget; more (cheaper ones within the same work) when the time is sent and the count
    /// measures the work well.
    fn count_cap(&self) -> usize {
        if self.ms.is_some() && self.w1() >= WORK_TRUST { MAX_SOLVES } else { self.budget }
    }
    fn work_left(&self) -> u64 {
        self.wbudget().saturating_sub(self.wspent)
    }
    /// Whether another solve may start: solves and work left (natively, time too).
    fn left(&self) -> bool {
        if self.solves >= self.count_cap() || (self.w1.is_some() && self.work_left() == 0) {
            return false;
        }
        #[cfg(not(target_arch = "wasm32"))]
        if self.clock {
            if self.t0.elapsed().as_micros() >= MAX_MS * 1000 {
                return false;
            }
        }
        true
    }
    /// How many more solves are expected to fit: the solves left, the work left at the average cost of the solves so
    /// far (the visitor's before any), and natively the time left at the average cost so far.
    fn room(&self) -> usize {
        let mut by_count = self.count_cap().saturating_sub(self.solves);
        if self.w1.is_some() {
            let avg = if self.extra > 0 { (self.wextra / self.extra as u64).max(1) } else { self.w1() };
            by_count = by_count.min((self.work_left() / avg) as usize);
        }
        #[cfg(not(target_arch = "wasm32"))]
        if self.clock {
            let spent = self.t0.elapsed().as_micros();
            let limit = MAX_MS * 1000;
            if spent >= limit {
                return 0;
            }
            if self.solves > 0 {
                let avg = (spent / self.solves as u128).max(1);
                return by_count.min(((limit - spent) / avg) as usize);
            }
        }
        by_count
    }
    /// A share of what is left for one figure: `frac` of the room, at least `lo` (or all that is left), at most `hi`.
    fn share(&self, frac: f64, lo: usize, hi: usize) -> usize {
        let r = self.room();
        ((r as f64 * frac) as usize).max(lo.min(r)).min(hi)
    }
    /// The visitor's own answer as the page computed it, with the work it took (no solve needed here).
    fn known(&mut self, work: u64) {
        self.w1 = Some(work.max(WORK_FLOOR));
    }
    fn run(&mut self, given: &[(String, Input)]) -> Option<Answer> {
        self.stopped = false;
        if !self.left() {
            return None;
        }
        self.solves += 1;
        let w0 = crate::meter::work();
        if self.w1.is_none() {
            // the visitor's own inputs: never stopped, and its work is the measure of every later solve
            let a = solve(self.p, self.key, given, self.target);
            let w = crate::meter::work().wrapping_sub(w0).max(WORK_FLOOR);
            self.w1 = Some(w);
            self.wspent = w;
            return if a.solved() { Some(a) } else { None };
        }
        let cap = self.work_left().min((self.wbudget() / 8).max(self.w1() + self.w1() / 2)).max(1);
        crate::meter::limit_after(cap);
        let a = solve(self.p, self.key, given, self.target);
        self.stopped = crate::meter::unlimit();
        let w = crate::meter::work().wrapping_sub(w0);
        self.wspent = self.wspent.saturating_add(w);
        self.wextra = self.wextra.saturating_add(w);
        self.extra += 1;
        if self.stopped || !a.solved() { None } else { Some(a) }
    }
}

/// Indices 0..n in coarse-to-fine order (ends, middle, quarters, ...): a sweep cut short by its budget still spans
/// its whole range.
fn coarse_to_fine(n: usize) -> Vec<usize> {
    if n == 0 {
        return vec![];
    }
    let mut order = vec![0];
    if n > 1 {
        order.push(n - 1);
    }
    let mut queue = std::collections::VecDeque::from([(0usize, n - 1)]);
    while let Some((a, b)) = queue.pop_front() {
        let m = (a + b) / 2;
        if m != a && m != b {
            order.push(m);
            queue.push_back((a, m));
            queue.push_back((m, b));
        }
    }
    order
}

/// One output of an answer: symbol, value, unit, meaning.
#[derive(Clone, Debug)]
struct Out {
    sym: String,
    val: Value,
    unit: String,
    meaning: String,
}

/// What a relation's symbol means in words: its description, or else its kind in plain words ("moment_of_inertia"
/// -> "moment of inertia"), so a chart never titles itself with a bare id ("L as ir varies"; review 2026-11-12).
fn sym_words(name: &str, desc: &str, kind: &str) -> String {
    if !desc.trim().is_empty() {
        return desc.to_string();
    }
    if name == "g" && kind == "acceleration" {
        return "gravitational acceleration".into();
    }
    if kind.is_empty() || kind == "number" {
        return name.to_string();
    }
    kind.replace('_', " ")
}

/// The unit a chart axis reads a value in, when its SI unit is awkward: the unit the meaning names ("partial pressure
/// of CO2 (e.g. in mmHg)", "volume [L]"), else the first of the usual reading units in which the value lies between
/// 0.1 and 10,000. None keeps SI. Returns the unit's text and its factor/offset to SI.
fn reading_unit(si: &str, meaning: &str, v: f64) -> Option<(String, crate::units::Unit)> {
    use crate::units::parse_unit;
    let base = parse_unit(si).ok()?;
    if si.is_empty() || si == "1" || !v.is_finite() {
        return None;
    }
    let same = |t: &str| parse_unit(t).ok().filter(|u| u.dim == base.dim && t != si);
    // 1. the unit the meaning names: "in X" or "[X]"
    let m = meaning.replace(['(', ')', ',', ';'], " ");
    let words: Vec<&str> = m.split_whitespace().collect();
    for (i, w) in words.iter().enumerate() {
        let cand = if *w == "in" { words.get(i + 1).copied() } else if w.starts_with('[') && w.ends_with(']') {
            Some(w.trim_matches(|c| c == '[' || c == ']'))
        } else { None };
        if let Some(c) = cand {
            if let Some(u) = same(c) {
                return Some((c.to_string(), u));
            }
        }
    }
    // 1b. a unit of the same dimension named inside the meaning's parentheses ("rupture time (hours, h)", "formation
    // energy (per vacancy, e.g. eV)"): materials review, 2026-11-12, axes in s and J against wording in h and eV
    for part in meaning.split('(').skip(1) {
        let inner = part.split(')').next().unwrap_or("");
        for w in inner.split(|c: char| c == ',' || c == ';' || c.is_whitespace()) {
            let w = w.trim_matches(|c: char| c == '.' || c == '[' || c == ']');
            if w.is_empty() || w == "e.g" || w == "i.e" {
                continue;
            }
            if let Some(u) = same(w) {
                return Some((w.to_string(), u));
            }
        }
    }
    // 2. by size, only when the SI number itself is awkward to read
    let a = (v - base.offset).abs() * base.factor;
    if a == 0.0 || (0.1..1e4).contains(&a) {
        return None;
    }
    let list: &[&str] = match si {
        "s" => &["min", "h", "day", "yr", "ms", "us"],
        "kg" => &["g", "mg", "ug", "t"],
        "m" => &["km", "mm", "um", "nm"],
        "m^2" => &["cm^2", "mm^2", "km^2"],
        "m^3" => &["L", "mL", "uL", "km^3"],
        "m^3/s" => &["L/s", "L/min", "mL/min", "mL/s", "m^3/h", "m^3/day"],
        "Pa" => &["kPa", "MPa", "GPa", "mPa"],
        "J" => &["kJ", "MJ", "GJ", "mJ", "eV"],
        "W" => &["kW", "MW", "GW", "mW", "uW"],
        "kg/m^3" => &["g/L", "mg/L", "ug/L"],
        "mol/m^3" => &["mol/L", "mmol/L", "umol/L", "nmol/L"],
        "kg/s" => &["g/s", "kg/h", "kg/day", "mg/s", "t/day"],
        "1/s" => &["1/min", "1/h", "1/day"],
        "V" => &["kV", "mV", "uV"],
        "A" => &["mA", "uA", "kA"],
        "F" => &["uF", "nF", "pF", "mF"],
        "H" => &["mH", "uH"],
        "Hz" => &["kHz", "MHz", "GHz"],
        "N" => &["kN", "MN", "mN"],
        _ => &[],
    };
    for t in list {
        if let Some(u) = same(t) {
            let d = (v * base.factor + base.offset - u.offset) / u.factor;
            if (0.1..1e4).contains(&d.abs()) {
                return Some((t.to_string(), u));
            }
        }
    }
    None
}

/// A symbol as it is printed on a chart: Greek names as Greek letters ("lam" -> "λ", "omega_r" -> "ω_r"),
/// everything else as written.
pub(crate) fn pretty_sym(s: &str) -> String {
    const G: &[(&str, &str)] = &[("lambda", "λ"), ("lam", "λ"), ("omega", "ω"), ("theta", "θ"), ("rho", "ρ"),
        ("sigma", "σ"), ("alpha", "α"), ("beta", "β"), ("gamma", "γ"), ("delta", "δ"), ("epsilon", "ε"), ("eps", "ε"),
        ("eta", "η"), ("mu", "μ"), ("nu", "ν"), ("phi", "φ"), ("psi", "ψ"), ("tau", "τ"), ("kappa", "κ"), ("chi", "χ"),
        ("xi", "ξ"), ("zeta", "ζ"), ("pi", "π")];
    let (head, tail) = match s.find('_') {
        Some(i) => (&s[..i], &s[i..]),
        None => {
            let i = s.trim_end_matches(|c: char| c.is_ascii_digit()).len();
            (&s[..i], &s[i..])
        }
    };
    for (n, g) in G {
        if head == *n {
            return format!("{g}{tail}");
        }
    }
    s.to_string()
}

fn outs_of(p: &Pack, key: &str, a: &Answer) -> Vec<Out> {
    if let Some(pr) = p.procedure(key) {
        return a.values.iter().map(|(s, v, u)| Out {
            sym: s.clone(), val: v.clone(), unit: u.clone(),
            meaning: pr.outputs.iter().find(|o| &o.symbol == s).map(|o| o.meaning.clone()).unwrap_or_default(),
        }).collect();
    }
    let sym = a.target.clone().unwrap_or_default();
    let meaning = p.relation(key).and_then(|r| r.sym(&sym)).map(|s| sym_words(&s.name, &s.desc, &s.kind)).unwrap_or_default();
    match a.value {
        Some(v) => vec![Out { sym, val: json!(v), unit: a.unit.clone().unwrap_or_default(), meaning }],
        None => vec![],
    }
}

/// The kind of a capability's output (a procedure's declared output kind, a relation's symbol kind), "" if unknown.
fn out_kind(p: &Pack, key: &str, sym: &str) -> String {
    if let Some(pr) = p.procedure(key) {
        return pr.outputs.iter().find(|o| o.symbol == sym).map(|o| o.kind.clone()).unwrap_or_default();
    }
    p.relation(key).and_then(|r| r.sym(sym)).map(|s| s.kind.clone()).unwrap_or_default()
}

fn numv(v: &Value) -> Option<f64> {
    match v {
        Value::Number(n) => n.as_f64().filter(|x| x.is_finite()),
        _ => None,
    }
}

pub(crate) fn cxv(v: &Value) -> Option<(f64, f64)> {
    match v {
        Value::Object(o) => {
            let re = o.get("re").and_then(numv)?;
            let im = o.get("im").and_then(numv)?;
            Some((re, im))
        }
        other => numv(other).map(|x| (x, 0.0)),
    }
}

fn is_cx_obj(v: &Value) -> bool {
    matches!(v, Value::Object(o) if o.contains_key("re") && o.contains_key("im"))
}

fn catv(v: &Value) -> Option<String> {
    match v {
        Value::String(s) if !s.is_empty() && s.chars().count() <= 48 => Some(s.clone()),
        Value::Bool(b) => Some(if *b { "yes".into() } else { "no".into() }),
        _ => None,
    }
}

fn numlist(v: &Value) -> Option<Vec<f64>> {
    let a = v.as_array()?;
    if a.is_empty() || a.len() > MAX_LIST {
        return None;
    }
    a.iter().map(numv).collect()
}

fn nummat(v: &Value) -> Option<Vec<Vec<f64>>> {
    let a = v.as_array()?;
    if a.is_empty() || a.len() > 40 {
        return None;
    }
    let m: Vec<Vec<f64>> = a.iter().map(|r| r.as_array().and_then(|r| r.iter().map(numv).collect())).collect::<Option<_>>()?;
    if m[0].is_empty() || m[0].len() > 40 || m.iter().any(|r| r.len() != m[0].len()) {
        return None;
    }
    Some(m)
}

// ------------------------------------------------------------------------------------------------ what is tracked
#[derive(Clone, Debug)]
enum Track {
    S(String),
    E(String, usize),
}

fn track_val(outs: &[Out], t: &Track) -> Option<f64> {
    match t {
        Track::S(s) => outs.iter().find(|o| &o.sym == s).and_then(|o| numv(&o.val)),
        Track::E(s, i) => outs.iter().find(|o| &o.sym == s).and_then(|o| o.val.as_array()?.get(*i).and_then(numv)),
    }
}

fn track_label(outs: &[Out], t: &Track) -> String {
    match t {
        Track::S(s) => pretty_sym(s),
        Track::E(s, i) => {
            let _ = outs;
            format!("{}[{}]", pretty_sym(s), i + 1)
        }
    }
}

/// The numbers a figure follows: the primary output, and other outputs of the same unit (at most 3); or the entries of
/// a short list answer.
fn choose_tracks(outs: &[Out], primary: &str) -> Vec<Track> {
    let nums: Vec<&Out> = outs.iter().filter(|o| numv(&o.val).is_some()).collect();
    let mut t = vec![];
    if let Some(first) = nums.iter().find(|o| o.sym == primary).or(nums.first()) {
        t.push(Track::S(first.sym.clone()));
        let u = first.unit.clone();
        let y0 = numv(&first.val).unwrap_or(0.0).abs();
        if !u.is_empty() && u != "1" {
            for o in &nums {
                // another answer of the same unit and a comparable size (one plot without a log scale)
                let y = numv(&o.val).unwrap_or(0.0).abs();
                let comparable = y0 > 0.0 && y > 0.0 && (y / y0).max(y0 / y) <= 30.0;
                if o.sym != first.sym && o.unit == u && comparable && t.len() < 3 {
                    t.push(Track::S(o.sym.clone()));
                }
            }
        }
        return t;
    }
    // the primary answer's list first
    let mut order: Vec<&Out> = outs.iter().filter(|o| o.sym == primary).collect();
    order.extend(outs.iter().filter(|o| o.sym != primary));
    for o in order {
        if let Some(l) = numlist(&o.val) {
            if (1..=6).contains(&l.len()) {
                for i in 0..l.len().min(4) {
                    t.push(Track::E(o.sym.clone(), i));
                }
                return t;
            }
        }
    }
    t
}

fn choose_cat(outs: &[Out]) -> Option<String> {
    outs.iter().find(|o| catv(&o.val).is_some() && !o.sym.ends_with("_exact") && !o.sym.contains("form"))
        .map(|o| o.sym.clone())
}

fn cat_val(outs: &[Out], sym: &Option<String>) -> Option<String> {
    let s = sym.as_ref()?;
    outs.iter().find(|o| &o.sym == s).and_then(|o| catv(&o.val))
}

// ------------------------------------------------------------------------------------------------ inputs to vary
#[derive(Clone, Debug)]
struct InSpec {
    sym: String,
    meaning: String,
    ty: String,
    unit: String,
    choices: Vec<String>,
    min: Option<f64>,
    max: Option<f64>,
    proc_input: Option<ProcInput>,
}

fn in_specs(p: &Pack, key: &str) -> Vec<InSpec> {
    let unit = |k: &str| p.kinds.get(k).map(|x| x.unit.clone()).unwrap_or_default();
    if let Some(pr) = p.procedure(key) {
        return pr.inputs.iter().map(|i| natural_limits(InSpec {
            sym: i.symbol.clone(), meaning: i.meaning.clone(),
            ty: if i.ty.is_empty() { "number".into() } else { i.ty.clone() },
            unit: if i.kind == "number" { String::new() } else { unit(&i.kind) },
            choices: if i.ty == "boolean" { vec!["true".into(), "false".into()] } else { i.choices.clone() },
            min: i.min.as_ref().map(|x| x.0), max: i.max.as_ref().map(|x| x.0), proc_input: Some(i.clone()),
        }, &i.kind)).collect();
    }
    match p.relation(key) {
        Some(r) => r.symbols.iter().map(|s| natural_limits(InSpec {
            sym: s.name.clone(), meaning: sym_words(&s.name, &s.desc, &s.kind), ty: "number".into(), unit: unit(&s.kind), choices: vec![],
            min: match s.sign.as_str() { "positive" | "nonnegative" | "fraction" => Some(0.0), "at_least_one" => Some(1.0), _ => None },
            max: if s.sign == "fraction" { Some(1.0) } else { None },
            proc_input: None,
        }, &s.kind)).collect(),
        None => vec![],
    }
}

/// The range a quantity of this kind admits, so a chart never sweeps an input into impossible values (an emissivity
/// of 1.5, a ratio of specific heats of 0.4, a cutoff ratio below 1, a Mach number ahead of a normal shock below 1, a
/// latitude beyond 90°; thermofluids and earth science reviews, 2026-11-12/13). Only ever tightens the declared range.
pub(crate) fn kind_range(kind: &str, meaning: &str, unit: &str) -> (Option<f64>, Option<f64>) {
    let m = meaning.to_lowercase();
    let deg = unit != "rad";
    if m.contains("latitude") || m.contains("declination") && m.contains("sun") {
        let q = if deg { 90.0 } else { std::f64::consts::FRAC_PI_2 };
        return (Some(-q), Some(q));
    }
    if m.contains("longitude") {
        let q = if deg { 180.0 } else { std::f64::consts::PI };
        return (Some(-q), Some(q));
    }
    match kind {
        "emissivity" | "absorptivity" | "reflectivity" | "transmissivity" | "efficiency" | "porosity" | "probability" => {
            (Some(0.0), Some(1.0))
        }
        "heat_capacity_ratio" => (Some(1.0), Some(5.0 / 3.0)),
        "cutoff_ratio" | "compression_ratio" => (Some(1.0), None),
        "mach_number" if m.contains("ahead of the shock") || m.contains("upstream") || m.contains("before the shock") => {
            (Some(1.0), None)
        }
        _ if m.contains("emissivity") || m.contains("ratio of specific heats") => {
            if m.contains("emissivity") { (Some(0.0), Some(1.0)) } else { (Some(1.0), Some(5.0 / 3.0)) }
        }
        _ => (None, None),
    }
}

fn natural_limits(mut s: InSpec, kind: &str) -> InSpec {
    let (lo, hi) = kind_range(kind, &s.meaning, &s.unit);
    if let Some(v) = lo {
        s.min = Some(s.min.map_or(v, |m| m.max(v)));
    }
    if let Some(v) = hi {
        s.max = Some(s.max.map_or(v, |m| m.min(v)));
    }
    s
}

/// The text of an input value as the form sends it.
fn text_of(i: &Input) -> Option<String> {
    match i {
        Input::Text(t) => Some(t.clone()),
        Input::Int(n) => Some(n.to_string()),
        Input::Float(x) => Some(py_repr_f64(*x)),
        Input::Json(Value::String(t)) => Some(t.clone()),
        Input::Json(Value::Bool(b)) => Some(b.to_string()),
        _ => None,
    }
}

/// A leading number of a text and what follows it ("3.5 km" -> (3.5, "km")).
fn lead_num(t: &str) -> Option<(f64, String)> {
    let s = t.trim().replace('−', "-");
    let b = s.as_bytes();
    let mut i = 0;
    if i < b.len() && (b[i] == b'-' || b[i] == b'+') {
        i += 1;
    }
    let ds = i;
    while i < b.len() && (b[i].is_ascii_digit() || b[i] == b'.') {
        i += 1;
    }
    if i == ds || !s[ds..i].chars().any(|c| c.is_ascii_digit()) {
        return None;
    }
    if i < b.len() && (b[i] == b'e' || b[i] == b'E') {
        let mut j = i + 1;
        if j < b.len() && (b[j] == b'-' || b[j] == b'+') {
            j += 1;
        }
        let es = j;
        while j < b.len() && b[j].is_ascii_digit() {
            j += 1;
        }
        if j > es {
            i = j;
        }
    }
    let x: f64 = s[..i].parse().ok()?;
    let rest = s[i..].trim().to_string();
    // a fraction or an expression ("1/2", "2*pi") is not a plain number with a unit
    if rest.starts_with(['/', '*', '^', '+', '-', '(']) || !x.is_finite() {
        return None;
    }
    Some((x, rest))
}

#[derive(Clone, Debug)]
struct Knob {
    sym: String,
    idx: Option<usize>,
    x0: f64,
    int: bool,
    rest: String,
    label: String,
    unit: String,
    base: Vec<f64>,
    lo_lim: Option<f64>,
    hi_lim: Option<f64>,
    /// a whole number written inside a text input ("H2O", "2^n/n^3"): the text and the byte range of the number
    text: Option<(String, usize, usize)>,
    /// a lattice step (half-integer quantum numbers): the sweep moves in steps of it instead of continuously
    step: Option<f64>,
    /// what the sweep of this input follows (chosen by the probes: the primary answer when it moves, else the answer
    /// that moves most)
    tracks: Vec<Track>,
    /// the verdict this input changes (shaded bands), if any
    cat: Option<String>,
    /// a wider range (three times the value either side): the answer changes only further away
    wide: bool,
    /// an entry of a matrix input: `base` holds the matrix row by row, this many columns
    mat_cols: Option<usize>,
    /// a range of a tenth to ten times the value (log-spaced): the answer changes only an order of magnitude away
    logwide: bool,
    /// a whole number whose neighbours mostly admit no answer (a prime): the sweep scans outward from it, one by one
    scan: bool,
    /// the exact values to try (a sweep cut to a few solves)
    pts: Option<Vec<f64>>,
}

impl Knob {
    fn plain(sym: &str, x0: f64, int: bool, label: String, unit: String) -> Knob {
        Knob { sym: sym.to_string(), idx: None, x0, int, rest: String::new(), label, unit, base: vec![], lo_lim: None,
               hi_lim: None, text: None, step: None, tracks: vec![], cat: None, wide: false, mat_cols: None,
               logwide: false, scan: false, pts: None }
    }
}

/// Whole numbers written inside a text (at most two): byte ranges of digit runs that are not part of a decimal.
fn text_numbers(t: &str) -> Vec<(usize, usize, i64)> {
    let b = t.as_bytes();
    let mut out = vec![];
    let mut i = 0;
    while i < b.len() && out.len() < 2 {
        if b[i].is_ascii_digit() {
            let s = i;
            while i < b.len() && b[i].is_ascii_digit() {
                i += 1;
            }
            let dotted = (s > 0 && b[s - 1] == b'.') || (i + 1 < b.len() && b[i] == b'.' && b[i + 1].is_ascii_digit());
            if !dotted && i - s <= 6 {
                if let Ok(v) = t[s..i].parse::<i64>() {
                    out.push((s, i, v));
                }
            }
        } else {
            i += 1;
        }
    }
    out
}

fn knobs(specs: &[InSpec], given: &[(String, Input)]) -> Vec<Knob> {
    let mut out = vec![];
    let mut elems = vec![];
    let mut mats = vec![];
    for (name, v) in given {
        let Some(sp) = specs.iter().find(|s| &s.sym == name) else { continue };
        let mut meaning = if sp.meaning.is_empty() { sp.sym.clone() } else { sp.meaning.clone() };
        // an input worded "in per cent" that the engine reads as a fraction (0.009 for 0.9 %): the axis says so, or
        // 0.004-0.014 read as per cent (cell biology review, 2026-11-13)
        let low = meaning.to_lowercase();
        if (low.contains("per cent") || low.contains("percent")) && text_of(v).and_then(|t| lead_num(&t))
            .is_some_and(|(x, rest)| rest.is_empty() && x.abs() <= 1.0) {
            let head = meaning.split(" (").next().unwrap_or(&meaning).replace("in per cent ", "").replace("in percent ", "");
            meaning = format!("{} as a fraction (0.01 = 1 %)", head.trim());
        }
        match sp.ty.as_str() {
            "number" => {
                let Some(t) = text_of(v) else { continue };
                let Some((x, rest)) = lead_num(&t) else { continue };
                let unit = if rest.is_empty() { sp.unit.clone() } else { rest.clone() };
                // a measured quantity of a procedure with no declared limits (a sodium level, a height) is swept
                // within 40% of the visitor's value: a quarter to 1.75 times ran into values no body has (heights of
                // 0.5 m, sodium of 40 mmol/L; physiology review, 2026-11-13)
                let (lo_n, hi_n) = if sp.proc_input.is_some() && sp.min.is_none() && sp.max.is_none()
                    && !sp.unit.is_empty() && sp.unit != "1" && x != 0.0 {
                    // a temperature moves by degrees, not by a share of its absolute value (310 K swept to 520 K)
                    let (a, b) = if sp.unit == "K" { ((x - 40.0).max(x * 0.5), x + 40.0) } else { (x * 0.6, x * 1.4) };
                    (Some(a.min(b)), Some(a.max(b)))
                } else { (None, None) };
                let sp_min = sp.min.or(lo_n);
                let sp_max = sp.max.or(hi_n);
                // a bare SI number (the app's worked examples) is swept in the unit people read it in: the one its
                // meaning names ("e.g. in mmHg") or one that suits its size (5333 Pa -> 40 mmHg, 8.3e-5 m^3/s ->
                // 5 L/min); the engine reads "40 mmHg" itself (physiology review, 2026-11-13)
                if rest.is_empty() {
                    if let Some((label, u)) = reading_unit(&sp.unit, &meaning, x) {
                        let to = |v: f64| (v - u.offset) / u.factor;
                        out.push(Knob { rest: label.clone(), lo_lim: sp_min.map(to), hi_lim: sp_max.map(to),
                                        ..Knob::plain(name, to(x), false, meaning, label) });
                        continue;
                    }
                }
                out.push(Knob { rest, lo_lim: sp_min, hi_lim: sp_max, ..Knob::plain(name, x, false, meaning, unit) });
            }
            "integer" => {
                let Some(t) = text_of(v) else { continue };
                let Some((x, rest)) = lead_num(&t) else { continue };
                // whole numbers up to 1e15 (exact in a double, with room for the steps of a sweep)
                if !rest.is_empty() || x.fract() != 0.0 || x.abs() > 1e15 {
                    continue;
                }
                out.push(Knob { lo_lim: sp.min, hi_lim: sp.max, ..Knob::plain(name, x, true, meaning, String::new()) });
            }
            "vector" => {
                let Some(pi) = &sp.proc_input else { continue };
                let Ok(Val::Vec(vs)) = crate::coerce::coerce(pi, v) else { continue };
                if vs.is_empty() || vs.len() > 64 {
                    continue;
                }
                for (i, x) in vs.iter().enumerate().take(6) {
                    elems.push(Knob { idx: Some(i), base: vs.clone(),
                                      ..Knob::plain(name, *x, false, entry_label(&meaning, i, vs.len()), sp.unit.clone()) });
                }
            }
            "matrix" => {
                // the first entries of a matrix, used only when nothing else can move (an assignment's costs)
                let Some(pi) = &sp.proc_input else { continue };
                let Ok(Val::Mat(m)) = crate::coerce::coerce(pi, v) else { continue };
                if m.len() * m[0].len() > 400 {
                    continue;
                }
                let flat: Vec<f64> = m.iter().flatten().copied().collect();
                for (i, x) in flat.iter().enumerate().take(4) {
                    let (r, c) = (i / m[0].len(), i % m[0].len());
                    mats.push(Knob { idx: Some(i), base: flat.clone(), mat_cols: Some(m[0].len()),
                                     ..Knob::plain(name, *x, false, format!("{meaning}, row {} column {}", r + 1, c + 1),
                                                   sp.unit.clone()) });
                }
            }
            "text" => {
                let Some(t) = text_of(v) else { continue };
                // a truth assignment or a propositional formula is drawn as a truth table, not swept
                if t.len() > 400 || assignment_spans(&t).is_some() || formula_vars(&t).is_some() {
                    continue;
                }
                let nums = text_numbers(&t);
                for &(s, e, n) in &nums {
                    // a number written more than once is marked where it is varied: "the number 2 in 2x^2 - 3x + 1"
                    // swept the exponent under a title that read as the coefficient (math foundations review, 11-13)
                    let label = if nums.iter().filter(|x| x.2 == n).count() > 1 {
                        match (t.get(..s), t.get(s..e), t.get(e..)) {
                            (Some(a), Some(m), Some(z)) => {
                                let marked = format!("{a}[{m}]{z}");
                                let shown = if marked.chars().count() <= 40 { marked } else { clip(&marked, 40) };
                                format!("the number {} marked [ ] in {} ({})", n, name, shown)
                            }
                            _ => format!("the number {} in {} ({})", n, name, clip(&t, 24)),
                        }
                    } else {
                        format!("the number {} in {} ({})", n, name, clip(&t, 24))
                    };
                    elems.push(Knob { text: Some((t.clone(), s, e)), lo_lim: Some(0.0),
                                      ..Knob::plain(name, n as f64, true, label, String::new()) });
                }
            }
            _ => {}
        }
    }
    // two or more lists of one length are paired measurements (concentrations and rates, cycles and Ct values, times
    // and densities): a fit through them. Moving one measured point of such a series draws a misleading hump, so their
    // entries are not swept (Michaelis-Menten, Scatchard, qPCR and growth-fit charts, review 2026-11-13).
    let mut lens: Vec<(String, usize)> = vec![];
    for e in &elems {
        if e.text.is_none() && e.idx.is_some() && !lens.iter().any(|(s, _)| s == &e.sym) {
            lens.push((e.sym.clone(), e.base.len()));
        }
    }
    let paired: Vec<String> = lens.iter().filter(|(_, n)| *n >= 2 && lens.iter().filter(|(_, m)| m == n).count() >= 2)
        .map(|(s, _)| s.clone()).collect();
    elems.retain(|e| !(e.text.is_none() && paired.contains(&e.sym)));
    // single numbers first; entries of lists and numbers inside texts after (at most a few of them)
    let room = MAX_KNOBS.saturating_sub(out.len()).min(8);
    out.extend(elems.into_iter().take(room));
    if out.is_empty() {
        out = mats;
    }
    out.truncate(MAX_KNOBS);
    out
}

fn apply(given: &[(String, Input)], k: &Knob, x: f64) -> Vec<(String, Input)> {
    let text = match (&k.text, k.idx) {
        (Some((t, s, e)), _) => format!("{}{}{}", &t[..*s], x.round() as i64, &t[*e..]),
        (None, Some(i)) => {
            let mut v = k.base.clone();
            v[i] = x;
            match k.mat_cols {
                Some(c) => v.chunks(c.max(1)).map(|r| r.iter().map(|y| py_repr_f64(*y)).collect::<Vec<_>>().join(" "))
                    .collect::<Vec<_>>().join("; "),
                None => v.iter().map(|y| py_repr_f64(*y)).collect::<Vec<_>>().join(", "),
            }
        }
        (None, None) if k.int => format!("{}", x.round() as i64),
        (None, None) => {
            let n = py_repr_f64(x);
            if k.rest.is_empty() { n } else { format!("{n} {}", k.rest) }
        }
    };
    given.iter().map(|(n, v)| if n == &k.sym { (n.clone(), Input::Text(text.clone())) } else { (n.clone(), v.clone()) })
        .collect()
}

fn with_value(given: &[(String, Input)], sym: &str, text: &str) -> Vec<(String, Input)> {
    given.iter().map(|(n, v)| if n == sym { (n.clone(), Input::Text(text.to_string())) } else { (n.clone(), v.clone()) })
        .collect()
}

fn knob_range(k: &Knob) -> (f64, f64) {
    let (mut lo, mut hi) = if let Some(s) = k.step {
        (k.x0 - 8.0 * s, k.x0 + 8.0 * s)
    } else if k.logwide && k.x0 > 0.0 {
        (k.x0 / 10.0, k.x0 * 10.0)
    } else if k.wide {
        let d = if k.x0 == 0.0 { 3.0 } else { 2.0 * k.x0.abs() };
        (k.x0 - d, k.x0 + d)
    } else if k.int && k.x0 >= 10.0 {
        // a larger whole number (a count, a size, an argument of a number-theoretic function): the consecutive whole
        // numbers around the visitor's (sampling far apart would land on one residue class, and the work often grows
        // with the number)
        (k.x0 - 20.0, k.x0 + 20.0)
    } else if k.int {
        let span = (k.x0.abs() * 0.75).ceil().max(4.0);
        let lo = if k.x0 >= 0.0 { (k.x0 - span).max(0.0) } else { k.x0 - span };
        (lo, k.x0 + span)
    } else if k.x0 != 0.0 {
        let (a, b) = (k.x0 * 0.25, k.x0 * 1.75);
        (a.min(b), a.max(b))
    } else {
        (-1.0, 1.0)
    };
    // a limit the visitor's own value lies outside is not this value's scale (an efficiency typed in per cent)
    if let Some(m) = k.lo_lim.filter(|m| k.x0 >= *m) {
        lo = lo.max(m);
    }
    if let Some(m) = k.hi_lim.filter(|m| k.x0 <= *m) {
        hi = hi.min(m);
    }
    if hi <= lo {
        hi = lo + if k.int { 4.0 } else { 1.0 };
    }
    (lo, hi)
}

fn knob_points(k: &Knob, n: usize) -> Vec<f64> {
    let (lo, hi) = knob_range(k);
    let mut xs: Vec<f64> = if let Some(p) = &k.pts {
        p.iter().copied().filter(|x| x.is_finite()).take(64).collect()
    } else if let Some(s) = k.step {
        let half = ((n / 2) as i64).clamp(1, 8);
        (-half..=half).map(|i| k.x0 + s * i as f64).filter(|x| *x >= lo && *x <= hi).collect()
    } else if k.int {
        let count = (hi - lo) as usize + 1;
        let step = ((count as f64) / n as f64).ceil().max(1.0) as usize;
        (0..count).step_by(step).map(|i| lo + i as f64).collect()
    } else if k.logwide && lo > 0.0 {
        (0..n).map(|i| lo * (hi / lo).powf(i as f64 / (n - 1).max(1) as f64)).collect()
    } else {
        (0..n).map(|i| lo + (hi - lo) * i as f64 / (n - 1) as f64).collect()
    };
    if !xs.iter().any(|x| (x - k.x0).abs() <= 1e-12 * k.x0.abs().max(1e-300)) {
        xs.push(k.x0);
        xs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    }
    xs
}

/// A swept line with its jumps left open: a step between two points far larger than the steps either side of it (a
/// piecewise function's break, a root that switches branch) is a discontinuity, drawn as a gap rather than a steep
/// line through values the answer never takes (math foundations review, 2026-11-13).
fn break_jumps(s: Vec<Option<(f64, f64)>>) -> Vec<Option<(f64, f64)>> {
    let ys: Vec<f64> = s.iter().flatten().map(|p| p.1).collect();
    if ys.len() < 6 {
        return s;
    }
    let (lo, hi) = minmax(&ys);
    let r = hi - lo;
    if !(r > 0.0 && r.is_finite()) {
        return s;
    }
    let mut out = Vec::with_capacity(s.len() + 4);
    for i in 0..s.len() {
        if i >= 2 && i + 1 < s.len() {
            if let (Some(a0), Some(a), Some(b), Some(b1)) = (s[i - 2], s[i - 1], s[i], s[i + 1]) {
                let d = (b.1 - a.1).abs();
                let side = (a.1 - a0.1).abs().max((b1.1 - b.1).abs());
                if d > 0.2 * r && d > 8.0 * side {
                    out.push(None);
                }
            }
        }
        out.push(s[i]);
    }
    out
}

/// The words for entry `i` of a list of `n`: the entry's own name when the meaning names them in order ("A, b, c of the
/// line a x + b y = c" -> "c of the line a x + b y = c"), else "<meaning>, entry i+1" (the axis read "A, b, c of the
/// line ..., entry 3"; math foundations review, 2026-11-13).
fn entry_label(meaning: &str, i: usize, n: usize) -> String {
    if let Some((head, rest)) = meaning.split_once(" of ") {
        let names: Vec<&str> = head.split(',').map(|s| s.trim()).collect();
        let short = names.iter().all(|s| !s.is_empty() && s.chars().count() <= 6 && !s.contains(' '));
        if names.len() == n && n >= 2 && short {
            // the first name carries the sentence's capital ("A, b, c"): its own case is the following names' case
            let name = if i == 0 && names.len() > 1 && names[1].chars().all(|c| !c.is_uppercase()) {
                names[0].to_lowercase()
            } else {
                names[i].to_string()
            };
            return format!("{name} of {rest}");
        }
    }
    format!("{meaning}, entry {}", i + 1)
}

fn knob_axis_label(k: &Knob) -> String {
    let pure = k.unit.is_empty() || k.unit == "1";
    let u = if pure { String::new() } else { format!(" ({})", k.unit) };
    let label = if pure { k.label.clone() } else { drop_unit_note(&k.label, &k.unit) };
    // a label that starts with a symbol ("c of the line ...") keeps the symbol's case
    let symbol_first = label.split_once(" of ").is_some_and(|(h, _)| !h.contains(' ') && h.chars().count() <= 6);
    format!("{}{}", clip(&if symbol_first { label.clone() } else { cap1(&label) }, 52), u)
}

// ------------------------------------------------------------------------------------------------ the figures
struct Base<'a> {
    primary: String,
    outs: Vec<Out>,
    tracks: Vec<Track>,
    cat: Option<String>,
    header: String,
    specs: Vec<InSpec>,
    given: &'a [(String, Input)],
}

/// Every figure for the capability `key` and these inputs (none when the inputs are not solved). `budget`: the most
/// solves to spend (default: MAX_SOLVES natively, where the time limit also applies; DEFAULT_WASM_SOLVES in a browser,
/// which has no clock here).
pub fn figures(p: &Pack, key: &str, given: &[(String, Input)], target: Option<&str>, budget: Option<usize>) -> Vec<Figure> {
    figures_from(p, key, given, target, budget, None, None, true)
}

/// `figures`, with the time the visitor's solve took (`ms`, which sizes the work allowed) and that solve's answer and
/// work when at hand (then not solved again). `clock` false (natively): as in the browser, no wall-clock limit and the
/// browser's default budget, so the result does not depend on the machine's speed.
#[allow(clippy::too_many_arguments)]
fn figures_from(p: &Pack, key: &str, given: &[(String, Input)], target: Option<&str>, budget: Option<usize>,
                ms: Option<f64>, known: Option<(Answer, u64)>, clock: bool) -> Vec<Figure> {
    #[cfg(target_arch = "wasm32")]
    let default = { let _ = clock; DEFAULT_WASM_SOLVES };
    #[cfg(not(target_arch = "wasm32"))]
    let default = if clock { MAX_SOLVES } else { DEFAULT_WASM_SOLVES };
    let budget = budget.unwrap_or(default).clamp(1, MAX_SOLVES);
    let mut cx = Ctx::new(p, key, target, budget, ms);
    #[cfg(not(target_arch = "wasm32"))]
    {
        cx.clock = clock;
    }
    let a = match known {
        Some((a, work)) if a.solved() => {
            cx.known(work);
            a
        }
        _ => match cx.run(given) {
            Some(a) => a,
            None => return vec![],
        },
    };
    let outs = outs_of(p, key, &a);
    if outs.is_empty() {
        return vec![];
    }
    let primary = p.procedure(key).map(|pr| pr.primary.clone()).unwrap_or_else(|| a.target.clone().unwrap_or_default());
    let tracks = choose_tracks(&outs, &primary);
    let cat = choose_cat(&outs);
    let header = header_of(&outs, &tracks, &primary);
    let specs = in_specs(p, key);
    let b = Base { primary, outs, tracks, cat, header, specs, given };
    let slug0 = key.replace('_', "-");
    // the model's own figures for this capability (figkit.rs: a circuit, a Bode diagram, ...) come first; one whose
    // drawing holds a non-finite number is left out
    let own: Vec<Figure> = match p.figures {
        Some(draw) => {
            let s = crate::figkit::Solved::new(p, key, given, target, &a);
            draw(&s).into_iter()
                .filter(|f| !f.svg.contains("NaN") && !f.svg.contains("inf\"") && !f.svg.contains("=\"inf"))
                .take(MAX_OWN).collect()
        }
        None => vec![],
    };
    // natively the generic figures get their whole time allowance after the model's own ones (a slow own figure
    // must not crowd out a generic one; wasm has no clock limit)
    #[cfg(not(target_arch = "wasm32"))]
    {
        cx.t0 = std::time::Instant::now();
    }
    let mut figs: Vec<Figure> = vec![];
    // every caption ends with the inputs it was drawn for
    let inputs_note = inputs_note(given);
    let push = |figs: &mut Vec<Figure>, kind: &str, f: Option<(String, String, String)>| {
        if let Some((title, body, caption)) = f {
            let caption = format!("{caption}{inputs_note}");
            if figs.len() < MAX_FIGS {
                let n = figs.len() + 1;
                let id = format!("axr-{}-{n}", slug0);
                figs.push(Figure { slug: format!("{slug0}-live-{n}"), kind: kind.to_string(),
                                   svg: wrap_svg(&id, &title, &caption, &body), title, caption });
            }
        }
    };

    // which inputs move the answer, and which answer each moves: a few probe solves per input (nearer ones when the
    // first are refused; finally steps of 1/2 and 1 for a quantum number). A tiny budget takes the first input as is.
    let mut ks = knobs(&b.specs, given);
    // in a network, a number that names a node (an origin, a facility) is not a quantity to sweep
    let nodes = graph_nodes(&b);
    if !nodes.is_empty() {
        ks.retain(|k| {
            let named = k.x0.fract() == 0.0 && nodes.contains(&format!("{}", k.x0 as i64));
            let ty = b.specs.iter().find(|s| s.sym == k.sym).map(|s| s.ty.as_str()).unwrap_or("");
            !(named && (ty == "number" || ty == "vector"))
        });
    }
    let probe_max = (cx.room() / 10).max(1);
    ks.truncate(probe_max);
    // the answers a sweep may follow: the list entries already chosen, or every single number (at most 12)
    let cands: Vec<Track> = if matches!(b.tracks.first(), Some(Track::E(..))) { b.tracks.clone() } else {
        b.outs.iter().filter(|o| numv(&o.val).is_some()).take(12).map(|o| Track::S(o.sym.clone())).collect()
    };
    let base_c: Vec<Option<f64>> = cands.iter().map(|t| track_val(&b.outs, t)).collect();
    // the verdicts a sweep may shade by (the first one first)
    let cat_cands: Vec<Option<String>> = {
        let mut v: Vec<Option<String>> = b.cat.iter().map(|c| Some(c.clone())).collect();
        for o in &b.outs {
            if v.len() < 4 && catv(&o.val).is_some() && !o.sym.ends_with("_exact") && !v.contains(&Some(o.sym.clone())) {
                v.push(Some(o.sym.clone()));
            }
        }
        v
    };
    let base_cat: Vec<Option<String>> = cat_cands.iter().map(|c| cat_val(&b.outs, c)).collect();
    let mut ranked: Vec<(f64, Knob)> = vec![];
    if cx.room() < 10 {
        // too few solves to probe: the first input, at a few values near the visitor's (a number 30% either side,
        // a whole number by the usual steps)
        if let Some(mut k) = ks.into_iter().next() {
            k.tracks = b.tracks.clone();
            k.cat = b.cat.clone();
            if k.int && k.x0 >= 16.0 && k.text.is_none() && k.idx.is_none() && cx.room() < 3 {
                // fewer solves than the usual steps: smaller whole numbers, halving (where the work grows with the
                // number, they cost little together; the work allowed stops the rest)
                let lo = k.lo_lim.unwrap_or(0.0);
                let mut v: Vec<f64> = vec![];
                for f in [0.5, 0.25, 0.75, 0.125, 0.375, 0.0625, 0.875, 0.03125] {
                    let x = (k.x0 * f).round();
                    if x >= lo && x < k.x0 && !v.contains(&x) {
                        v.push(x);
                    }
                }
                k.pts = Some(v);
                k.logwide = true;                     // drawn on a logarithmic axis
            } else if !k.int && k.x0 != 0.0 && cx.room() < 6 {
                k.pts = Some(vec![k.x0 * 0.7, k.x0 * 1.3]);
            }
            ranked.push((1.0, k));
        }
        ks = vec![];
    }
    for mut k in ks {
        if cx.room() < 3 {
            break;
        }
        let (lo, hi) = knob_range(&k);
        // (values to probe, lattice step, range): nearer probes when the first are refused; steps of 1/2 and 1 for a
        // quantum number; a wider range when nothing changes nearby (three times the value either side, then a tenth
        // to ten times it); for a whole number whose neighbours are refused (a prime), the next ones one by one
        const NEAR: u8 = 0;
        const WIDE: u8 = 1;
        const LOGWIDE: u8 = 2;
        const SCAN: u8 = 3;
        let mut tries: Vec<(Vec<f64>, Option<f64>, u8)> = if k.int {
            let d = (k.x0.abs() * 0.4).round().max(1.0);
            [d, 1.0, 2.0, 3.0, 4.0, 6.0].iter().map(|d| (vec![k.x0 - d, k.x0 + d], None, NEAR)).collect()
        } else if k.x0 != 0.0 {
            vec![(vec![k.x0 * 0.6, k.x0 * 1.4], None, NEAR), (vec![k.x0 * 0.9, k.x0 * 1.1], None, NEAR),
                 (vec![k.x0 * 0.99, k.x0 * 1.01], None, NEAR)]
        } else {
            vec![(vec![-0.5, 0.5], None, NEAR), (vec![-0.05, 0.05], None, NEAR)]
        };
        if !k.int && (k.x0 * 2.0).fract() == 0.0 && k.x0.abs() <= 50.0 && k.text.is_none() && k.idx.is_none() {
            tries.push((vec![k.x0 - 0.5, k.x0 + 0.5], Some(0.5), NEAR));
            tries.push((vec![k.x0 - 1.0, k.x0 + 1.0], Some(1.0), NEAR));
        }
        if !k.int {
            let d = if k.x0 == 0.0 { 3.0 } else { 2.0 * k.x0.abs() };
            tries.push((vec![k.x0 - d, k.x0 + d], None, WIDE));
            if k.x0 > 0.0 && k.text.is_none() && k.mat_cols.is_none() {
                tries.push((vec![k.x0 / 10.0, k.x0 * 10.0, k.x0 / 3.0, k.x0 * 3.0], None, LOGWIDE));
            }
        }
        if k.int && k.x0.abs() >= 10.0 && k.text.is_none() && k.idx.is_none() {
            tries.push(((1..=40).flat_map(|d| [k.x0 - d as f64, k.x0 + d as f64]).collect(), None, SCAN));
        }
        let mut spent = 0;
        for (xs, step, mode) in tries {
            if cx.room() < 3 || (spent >= 14 && mode != SCAN) {
                break;
            }
            let mut ys: Vec<Vec<Option<f64>>> = vec![];
            let mut cats: Vec<Vec<Option<String>>> = vec![];
            for x in xs {
                let inside = x >= lo && x <= hi;
                if k.int && (!inside || x == k.x0) && mode != SCAN {
                    continue;
                }
                if mode == SCAN && (ys.len() >= 8 || cx.room() < 3) {
                    break;
                }
                if mode == SCAN && (k.lo_lim.is_some_and(|m| x < m) || k.hi_lim.is_some_and(|m| x > m)) {
                    continue;
                }
                spent += 1;
                if let Some(a2) = cx.run(&apply(given, &k, x)) {
                    let o2 = outs_of(p, key, &a2);
                    ys.push(cands.iter().map(|t| track_val(&o2, t)).collect());
                    cats.push(cat_cands.iter().map(|c| cat_val(&o2, c)).collect());
                }
            }
            if ys.is_empty() {
                continue;
            }
            // how much each candidate answer moved, and which verdicts changed
            let moved: Vec<f64> = (0..cands.len()).map(|j| ys.iter().filter_map(|y| match (y[j], base_c[j]) {
                (Some(v), Some(v0)) => Some((v - v0).abs() / (v.abs() + v0.abs() + 1e-300)),
                _ => None,
            }).fold(0.0, f64::max)).collect();
            let changed: Vec<bool> = (0..cat_cands.len()).map(|j| cats.iter().any(|c| c[j] != base_cat[j])).collect();
            let mut score = moved.iter().cloned().fold(0.0, f64::max);
            if changed.iter().any(|c| *c) {
                score += 1.0;
            }
            if score <= 1e-9 {
                continue;                                     // solved but nothing moved here: try further
            }
            k.step = step;
            k.wide = mode == WIDE;
            k.logwide = mode == LOGWIDE;
            k.scan = mode == SCAN;
            let first_moves = b.tracks.first().map(|t| {
                cands.iter().position(|c| track_label(&b.outs, c) == track_label(&b.outs, t)).map(|j| moved[j] > 1e-9)
                    .unwrap_or(false)
            }).unwrap_or(false);
            k.tracks = if first_moves || matches!(b.tracks.first(), Some(Track::E(..))) {
                b.tracks.clone()
            } else {
                match (0..cands.len()).filter(|j| moved[*j] > 1e-9)
                    .max_by(|a, b| moved[*a].partial_cmp(&moved[*b]).unwrap_or(std::cmp::Ordering::Equal)) {
                    Some(j) => vec![cands[j].clone()],
                    None => vec![],
                }
            };
            k.cat = (0..cat_cands.len()).find(|j| changed[*j]).and_then(|j| cat_cands[j].clone())
                .or_else(|| b.cat.clone());
            ranked.push((score, k));
            break;
        }
    }
    ranked.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
    // a list entry ranks after every single number that moves the answer as much
    let knobs_ranked: Vec<Knob> = ranked.into_iter().map(|(_, k)| k).collect();

    let mut swept = 0usize;
    // a capability whose one solve uses most of the budget: a sweep of the few solves left (a figure needs two)
    let knobs_ranked: Vec<Knob> = if cx.left() { knobs_ranked } else { vec![] };
    if let Some(k) = knobs_ranked.first() {
        let n = cx.share(0.4, 9, SWEEP_N);
        let f = fig_sweep(&mut cx, &b, k, n);
        if f.is_some() {
            swept = 1;
        }
        push(&mut figs, sweep_kind(k), f);
    }
    if let Some(f) = fig_choices(&mut cx, &b) {
        push(&mut figs, "bars", Some(f));
    }
    // a text input with structure (a truth assignment, a propositional formula, genotypes): the answer over every
    // variant of that structure
    push(&mut figs, "truth", fig_truth(&mut cx, &b));
    for f in fig_genotypes(&mut cx, &b) {
        push(&mut figs, "punnett", Some(f));
    }
    // the answer's own structure
    let mut structural = 0;
    for (kind, f) in [("graph", fig_graph(&b)), ("points", fig_points(&b)),
                      ("complex", fig_complex(&mut cx, &b, knobs_ranked.first())), ("series", fig_lists(&b)),
                      ("matrix", fig_matrix(&b)), ("bars", fig_dict(&b, 2)), ("series", fig_shift(&b))] {
        if f.is_some() && structural < 2 {
            structural += 1;
            push(&mut figs, kind, f);
        }
    }
    if knobs_ranked.len() >= 2 && knobs_ranked[0].cat.is_some() && knobs_ranked[0].idx.is_none()
        && knobs_ranked[1].idx.is_none() && knobs_ranked[0].text.is_none() && knobs_ranked[1].text.is_none() {
        push(&mut figs, "decision", fig_grid(&mut cx, &b, &knobs_ranked[0], &knobs_ranked[1]));
    }
    if knobs_ranked.len() > swept && figs.len() < MAX_FIGS {
        for k in knobs_ranked.iter().skip(swept).take(2) {
            if figs.len() >= MAX_FIGS.min(3) && !figs.is_empty() {
                break;
            }
            let n = cx.share(0.6, 9, SWEEP_N2);
            if n < 5 {
                break;
            }
            let f = fig_sweep(&mut cx, &b, k, n);
            let got = f.is_some();
            push(&mut figs, sweep_kind(k), f);
            if got {
                break;
            }
        }
    }
    if figs.len() < 2 {
        push(&mut figs, "series", fig_prefix(&mut cx, &b));
    }
    if figs.len() < 2 {
        push(&mut figs, "bars", fig_outputs(&b, 2));
    }
    // never nothing: the answer itself (a composition of one element, a single number)
    if figs.is_empty() && own.is_empty() {
        push(&mut figs, "bars", fig_dict(&b, 1));
    }
    if figs.is_empty() && own.is_empty() {
        push(&mut figs, "bars", fig_outputs(&b, 1));
    }
    own.into_iter().chain(figs).collect()
}

/// A model's own figures at most (figkit.rs), before the generic ones.
const MAX_OWN: usize = 4;

/// The kind of a sweep: of one entry of a list input ("sensitivity"), else of a number ("curve").
fn sweep_kind(k: &Knob) -> &'static str {
    if k.idx.is_some() { "sensitivity" } else { "curve" }
}

/// " Your inputs: a = 1, b = 2." (the end of every live figure's caption).
pub(crate) fn inputs_note(given: &[(String, Input)]) -> String {
    let shown: Vec<String> = given.iter().take(8)
        .map(|(n, v)| format!("{n} = {}", clip(&text_of(v).unwrap_or_else(|| "(list)".into()), 28))).collect();
    if shown.is_empty() { String::new() } else { format!(" Your inputs: {}.", shown.join(", ")) }
}

fn header_of(outs: &[Out], tracks: &[Track], primary: &str) -> String {
    let show = |o: &Out| -> Option<String> {
        let u = if o.unit.is_empty() || o.unit == "1" { String::new() } else { format!(" {}", o.unit) };
        if let Some(x) = numv(&o.val) {
            return Some(format!("{} = {}{u}", o.sym, py_g(x, 4)));
        }
        if let Some((re, im)) = cxv(&o.val) {
            return Some(format!("{} = {}", o.sym, cx_text(re, im)));
        }
        if let Some(l) = numlist(&o.val) {
            let items: Vec<String> = l.iter().take(4).map(|x| py_g(*x, 4)).collect();
            return Some(format!("{} = [{}{}]{u}", o.sym, items.join(", "), if l.len() > 4 { ", ..." } else { "" }));
        }
        catv(&o.val).map(|c| format!("{} = {}", o.sym, c))
    };
    // the primary answer first (as the page lists it), else the number the figures follow
    let first = outs.iter().find(|o| o.sym == primary && show(o).is_some()).or_else(|| match tracks.first() {
        Some(Track::S(s)) => outs.iter().find(|o| &o.sym == s),
        _ => None,
    }).or(outs.first());
    let mut h = first.and_then(show).unwrap_or_default();
    // when the first shown is not a plain number (a complex root, a list, a verdict), the answer's first plain number
    // is always stated too (before it, when both do not fit)
    if !first.map(|f| numv(&f.val).is_some()).unwrap_or(true) {
        if let Some(o) = outs.iter().find(|o| Some(&o.sym) != first.map(|f| &f.sym) && numv(&o.val).is_some()) {
            if let Some(t) = show(o) {
                h = if h.chars().count() + t.chars().count() <= 60 { format!("{h};  {t}") } else { format!("{t};  {h}") };
                return clip(&h, 64);
            }
        }
    }
    // a second output when it is short (a verdict, a second number)
    for o in outs {
        if Some(&o.sym) == first.map(|f| &f.sym) || h.chars().count() > 34 {
            continue;
        }
        if let Some(t) = show(o) {
            if h.chars().count() + t.chars().count() < 56 {
                h = format!("{h};  {t}");
            }
            break;
        }
    }
    clip(&h, 64)
}

fn cx_text(re: f64, im: f64) -> String {
    if im == 0.0 {
        return py_g(re, 4);
    }
    format!("{} {} {}i", py_g(re, 4), if im < 0.0 { "-" } else { "+" }, py_g(im.abs(), 4))
}

fn out_label(outs: &[Out], t: &Track) -> String {
    let (sym, extra) = match t {
        Track::S(s) => (s.clone(), String::new()),
        Track::E(s, i) => (s.clone(), format!(", entry {}", i + 1)),
    };
    let o = outs.iter().find(|o| o.sym == sym);
    let unit = o.map(|o| o.unit.clone()).filter(|u| !u.is_empty() && u != "1");
    let m = o.map(|o| o.meaning.clone()).filter(|m| !m.is_empty()).unwrap_or(sym.clone());
    let m = match &unit { Some(u) => drop_unit_note(&m, u), None => m };
    let u = unit.map(|u| format!(" ({u})")).unwrap_or_default();
    format!("{}{extra}{u}", clip(&cap1(&m), 48))
}

/// A meaning without its own trailing unit note when the axis prints the unit itself: "Coriolis parameter (1/s)"
/// with unit 1/s once read "Coriolis parameter (1/s) (1/h)" (earth science review, 2026-11-13). The note goes when
/// it names the unit, a unit of the same dimension, or an example unit ("e.g. eV").
pub(crate) fn drop_unit_note(meaning: &str, si: &str) -> String {
    let m = meaning.trim_end();
    if !m.ends_with(')') {
        return m.to_string();
    }
    let Some(i) = m.rfind('(') else { return m.to_string() };
    let note = &m[i + 1..m.len() - 1];
    let base = crate::units::parse_unit(si).ok();
    let names_unit = note.contains("e.g.") || note.split(|c: char| c == ',' || c == ';' || c.is_whitespace())
        .map(|w| w.trim_matches(|c: char| c == '.' || c == '[' || c == ']'))
        .filter(|w| !w.is_empty())
        .any(|w| w == si || base.as_ref().is_some_and(|b| crate::units::parse_unit(w).is_ok_and(|u| u.dim == b.dim)));
    if names_unit && i > 0 {
        m[..i].trim_end().to_string()
    } else {
        m.to_string()
    }
}

/// The answer as one input sweeps a range around the visitor's value.
fn fig_sweep(cx: &mut Ctx, b: &Base, k: &Knob, n: usize) -> Option<(String, String, String)> {
    let tracks: &[Track] = &k.tracks;
    let kcat = &k.cat;
    if tracks.is_empty() && kcat.is_none() {
        return None;
    }
    let mut pts: Vec<(f64, Vec<Option<f64>>, Option<String>)> = vec![];
    let ok = |p: &(f64, Vec<Option<f64>>, Option<String>)| p.1.iter().any(|v| v.is_some()) || p.2.is_some();
    // values whose solve was stopped as too costly here (not refusals)
    let mut costly: Vec<f64> = vec![];
    let mut xs = knob_points(k, n.max(3));
    if k.scan {
        // the whole numbers next to the visitor's, one by one outward, until enough of them admit an answer
        pts.push((k.x0, tracks.iter().map(|t| track_val(&b.outs, t)).collect(), cat_val(&b.outs, kcat)));
        let want = 16usize;
        let mut tried = 0usize;
        'scan: for d in 1..=400i64 {
            for x in [k.x0 - d as f64, k.x0 + d as f64] {
                if pts.iter().filter(|p| ok(p)).count() >= want || tried >= n.max(24) * 4 || !cx.left() {
                    break 'scan;
                }
                if k.lo_lim.is_some_and(|m| x < m) || k.hi_lim.is_some_and(|m| x > m) {
                    continue;
                }
                tried += 1;
                match cx.run(&apply(b.given, k, x)) {
                    Some(a) => {
                        let o = outs_of(cx.p, cx.key, &a);
                        pts.push((x, tracks.iter().map(|t| track_val(&o, t)).collect(), cat_val(&o, kcat)));
                    }
                    None if cx.stopped => costly.push(x),
                    None => pts.push((x, vec![None; tracks.len()], None)),
                }
            }
        }
        pts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        xs = vec![];
    }
    // coarse to fine: points the budget does not reach are left out (they are not refusals). When most of the range
    // admits no answer, a second pass fills the part that does.
    for pass in 0..2 {
        if xs.is_empty() {
            break;
        }
        for i in coarse_to_fine(xs.len()) {
            let x = xs[i];
            if pts.iter().any(|p| p.0 == x) || costly.contains(&x) {
                continue;
            }
            if (x - k.x0).abs() <= 1e-12 * k.x0.abs().max(1e-300) {
                pts.push((k.x0, tracks.iter().map(|t| track_val(&b.outs, t)).collect(), cat_val(&b.outs, kcat)));
                continue;
            }
            if !cx.left() {
                continue;
            }
            match cx.run(&apply(b.given, k, x)) {
                Some(a) => {
                    let o = outs_of(cx.p, cx.key, &a);
                    pts.push((x, tracks.iter().map(|t| track_val(&o, t)).collect(), cat_val(&o, kcat)));
                }
                None if cx.stopped => costly.push(x),
                None => pts.push((x, vec![None; tracks.len()], None)),
            }
        }
        pts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        let good: Vec<usize> = (0..pts.len()).filter(|i| ok(&pts[*i])).collect();
        if pass == 1 || k.int || k.step.is_some() || k.pts.is_some() || good.len() * 2 >= pts.len() || good.is_empty()
            || cx.room() < 8 {
            break;
        }
        let (i0, i1) = (good[0].saturating_sub(1), (good[good.len() - 1] + 1).min(pts.len() - 1));
        let (lo, hi) = (pts[i0].0, pts[i1].0);
        let m = cx.share(0.5, 8, 25);
        xs = (1..m).map(|i| lo + (hi - lo) * i as f64 / m as f64).collect();
    }
    // an absolute temperature, a mass, an amount or a volume below zero is no answer: left out like a refusal
    for (j, t) in tracks.iter().enumerate() {
        let u = match t { Track::S(s) | Track::E(s, _) => b.outs.iter().find(|o| &o.sym == s).map(|o| (o.unit.clone(), o.meaning.to_lowercase())) };
        if let Some((u, m)) = u {
            let absolute = matches!(u.as_str(), "K" | "kg" | "mol" | "m^3")
                && !["difference", "change", "rise", "drop", "increase", "decrease", "excess", "loss", "gain"].iter().any(|w| m.contains(w));
            if absolute && track_val(&b.outs, t).is_some_and(|v| v > 0.0) {
                for p in pts.iter_mut() {
                    if p.1[j].is_some_and(|y| y <= 0.0) {
                        p.1[j] = None;
                    }
                }
            }
        }
    }
    // a result outside the range its kind admits (an efficiency above 1 where the heat taken in is less than the work,
    // a probability below 0) is no answer either; kept when the visitor's own value lies outside (typed in per cent)
    for (j, t) in tracks.iter().enumerate() {
        let (Track::S(s) | Track::E(s, _)) = t;
        let Some(o) = b.outs.iter().find(|o| &o.sym == s) else { continue };
        let (lo, hi) = kind_range(&out_kind(cx.p, cx.key, s), &o.meaning, &o.unit);
        if lo.is_none() && hi.is_none() {
            continue;
        }
        let inside = |v: f64| lo.map_or(true, |m| v >= m - 1e-12) && hi.map_or(true, |m| v <= m + 1e-12);
        if track_val(&b.outs, t).is_some_and(inside) {
            for p in pts.iter_mut() {
                if p.1[j].is_some_and(|y| !inside(y)) {
                    p.1[j] = None;
                }
            }
        }
    }
    // a sweep that runs through a pole (T1 = T2 in a two-point formula: the answer shoots to +infinity and comes back
    // from -infinity) keeps only the branch the visitor's value is on; the far branch is no answer to their question
    // (chemistry review, 2026-11-12: negative activation energies drawn beyond T1 = T2)
    if let Some(y0) = tracks.first().and_then(|t| track_val(&b.outs, t)).filter(|v| *v != 0.0 && v.is_finite()) {
        let line: Vec<(f64, f64)> = pts.iter().filter_map(|p| p.1.first().copied().flatten().map(|y| (p.0, y))).collect();
        let (mut cut_lo, mut cut_hi) = (f64::NEG_INFINITY, f64::INFINITY);
        for w in line.windows(2) {
            let ((xa, ya), (xb, yb)) = (w[0], w[1]);
            if ya.signum() != yb.signum() && ya.abs() > 2.0 * y0.abs() && yb.abs() > 2.0 * y0.abs() {
                if xb <= k.x0 {
                    cut_lo = cut_lo.max(xb);
                } else if xa >= k.x0 {
                    cut_hi = cut_hi.min(xa);
                }
            }
        }
        if cut_lo.is_finite() || cut_hi.is_finite() {
            pts.retain(|p| p.0 >= cut_lo && p.0 <= cut_hi);
        }
    }
    let solved: Vec<&(f64, Vec<Option<f64>>, Option<String>)> = pts.iter().filter(|p| ok(p)).collect();
    // whole numbers (a flag of 0 or 1, primes among integers) may admit only a few values: two are a comparison
    if solved.len() < if k.int || k.step.is_some() { 2 } else { 3 } {
        return None;
    }
    let ys: Vec<f64> = solved.iter().flat_map(|p| p.1.iter().flatten().copied()).collect();
    let cats: Vec<String> = solved.iter().filter_map(|p| p.2.clone()).collect();
    let count = solved.len();
    // shaded only as a verdict: a few distinct values (a text answer that differs at every point is not one)
    let distinct = cats.iter().fold(Vec::<&String>::new(), |mut v, c| { if !v.contains(&c) { v.push(c); } v }).len();
    let cat_changes = distinct <= 4 && cats.windows(2).any(|w| w[0] != w[1]);
    let flat = ys.is_empty() || {
        let (lo, hi) = minmax(&ys);
        hi - lo <= 1e-12 * hi.abs().max(lo.abs())
    };
    if flat && !cat_changes {
        return None;
    }
    let x_lo = solved.iter().map(|p| p.0).fold(f64::INFINITY, f64::min);
    let x_hi = solved.iter().map(|p| p.0).fold(f64::NEG_INFINITY, f64::max);
    // only the part of the range that has answers is drawn
    let (x_from, x_to) = (x_lo.min(k.x0), x_hi.max(k.x0));
    pts.retain(|p| p.0 >= x_from && p.0 <= x_to);
    // the output axis reads in the visitor's kind of unit (mmHg, L/min), not SI, when SI is awkward
    let o0 = tracks.first().and_then(|t| match t { Track::S(s) | Track::E(s, _) => b.outs.iter().find(|o| &o.sym == s) });
    let y_mid = {
        let mut v: Vec<f64> = ys.iter().map(|y| y.abs()).filter(|y| *y > 0.0).collect();
        v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        v.get(v.len() / 2).copied()
    };
    let yu = o0.zip(y_mid).and_then(|(o, m)| reading_unit(&o.unit, &o.meaning, m).map(|r| (o.unit.clone(), r)));
    let cy = |y: f64| match &yu { Some((_, (_, u))) => (y - u.offset) / u.factor, None => y };
    let mut series = vec![];
    if !flat {
        for (j, t) in tracks.iter().enumerate() {
            let s: Vec<Option<(f64, f64)>> = pts.iter().map(|p| p.1[j].map(|y| (p.0, cy(y)))).collect();
            let s = if k.int || k.step.is_some() { s } else { break_jumps(s) };
            if s.iter().flatten().count() >= 2 {
                series.push((track_label(&b.outs, t), s));
            }
        }
    }
    let bands: Vec<(f64, Option<String>)> = if cat_changes { pts.iter().map(|p| (p.0, p.2.clone())).collect() } else { vec![] };
    let y0 = tracks.first().and_then(|t| track_val(&b.outs, t)).map(cy);
    let mut ylab = if series.is_empty() { String::new() } else if tracks.len() == 1 || series.len() == 1 {
        axis_label_of(&b.outs, &tracks[0])
    } else {
        // several outputs of one unit
        let u = b.outs.iter().find(|o| matches!(&tracks[0], Track::S(s) if s == &o.sym)).map(|o| o.unit.clone())
            .unwrap_or_default();
        if u.is_empty() || u == "1" { "Value".into() } else { format!("Value ({u})") }
    };
    let y_unit = yu.as_ref().map(|(_, (l, _))| format!(" {l}")).unwrap_or_default();
    if let Some((si, (label, _))) = &yu {
        let (from, to) = (format!("({si})"), format!("({label})"));
        if ylab.ends_with(&from) {
            ylab = format!("{}{to}", &ylab[..ylab.len() - from.len()]);
        }
    }
    let marker = if flat { None } else { y0.map(|y| (k.x0, y)) };
    let mlabel = match (tracks.first(), y0) {
        (Some(t), Some(y)) if !flat => format!("{} = {}{y_unit} at {} = {}", track_label(&b.outs, t), py_g(y, 4), k_sym(k), py_g(k.x0, 4)),
        _ => format!("your {} = {}", k_sym(k), py_g(k.x0, 4)),
    };
    let body = line_chart_x(&series, &bands, (x_from, x_to), &knob_axis_label(k), &ylab, marker, &mlabel, &b.header,
                            k.int || k.step.is_some(), k.logwide);
    let what = match tracks.first() {
        Some(t) if !flat => out_label(&b.outs, t),
        _ => kcat.clone().map(|c| out_meaning(&b.outs, &c)).unwrap_or_default(),
    };
    let title = format!("{} as {} varies", cap1(&clip(&strip_unit(&what), 60)), lc1(&clip(&k.label, 50)));
    let mut caption = format!("{} as {} runs from {} to {}{}, every other input as you entered it ({count} solutions).",
                              cap1(&strip_unit(&what)), k_sym(k), py_g(x_lo, 4), py_g(x_hi, 4), unit_suffix(&k.unit));
    let refused = pts.iter().filter(|p| !ok(p)).count();
    if refused > 0 {
        caption.push_str(&format!(" Values in this range where the inputs admit no answer ({refused} of {}) are left out.",
                                  pts.len()));
    }
    let costly = costly.iter().filter(|x| **x >= x_from && **x <= x_to).count();
    if costly > 0 {
        caption.push_str(&format!(" {costly} value{} that would take too long to compute here {} left out.",
                                  if costly == 1 { "" } else { "s" }, if costly == 1 { "is" } else { "are" }));
    }
    if let Some(y) = y0.filter(|_| !flat) {
        caption.push_str(&format!(" Marked: your {} = {}{} gives {} = {}{y_unit}.", k_sym(k), py_g(k.x0, 4), unit_suffix(&k.unit),
                                  track_label(&b.outs, &tracks[0]), py_g(y, 4)));
    }
    if cat_changes {
        let mut seen: Vec<String> = vec![];
        for c in cats {
            if !seen.contains(&c) && seen.len() < 4 {
                seen.push(c);
            }
        }
        caption.push_str(&format!(" Shaded by {}: {}.", lc1(&out_meaning(&b.outs, kcat.as_deref().unwrap_or(""))), seen.join(", ")));
    }
    Some((title, body, caption))
}

/// A y-axis label: the meaning with its unit, or the symbol with its unit when the meaning is too long to fit.
fn axis_label_of(outs: &[Out], t: &Track) -> String {
    let l = out_label(outs, t);
    if l.chars().count() <= 34 {
        return l;
    }
    let sym = track_label(outs, t);
    let unit = match t {
        Track::S(s) | Track::E(s, _) => outs.iter().find(|o| &o.sym == s).map(|o| o.unit.clone()).unwrap_or_default(),
    };
    format!("{sym}{}", if unit.is_empty() || unit == "1" { String::new() } else { format!(" ({unit})") })
}

fn k_sym(k: &Knob) -> String {
    if k.text.is_some() {
        return format!("the number in {}", k.sym);
    }
    match k.idx {
        Some(i) if k.mat_cols.is_some() => {
            let c = k.mat_cols.unwrap_or(1).max(1);
            format!("{}[{},{}]", k.sym, i / c + 1, i % c + 1)
        }
        Some(i) => format!("{}[{}]", pretty_sym(&k.sym), i + 1),
        None => pretty_sym(&k.sym),
    }
}

fn out_meaning(outs: &[Out], sym: &str) -> String {
    outs.iter().find(|o| o.sym == sym).map(|o| if o.meaning.is_empty() { o.sym.clone() } else { o.meaning.clone() })
        .unwrap_or_else(|| sym.to_string())
}

fn unit_suffix(u: &str) -> String {
    if u.is_empty() || u == "1" { String::new() } else { format!(" {u}") }
}

fn strip_unit(s: &str) -> String {
    match s.rfind(" (") {
        Some(i) if s.ends_with(')') => s[..i].to_string(),
        _ => s.to_string(),
    }
}

/// The answer for each choice of a choice input.
fn fig_choices(cx: &mut Ctx, b: &Base) -> Option<(String, String, String)> {
    let t0 = b.tracks.first()?.clone();
    for (name, v) in b.given {
        let Some(sp) = b.specs.iter().find(|s| &s.sym == name) else { continue };
        if !(sp.ty == "choice" || sp.ty == "boolean") || sp.choices.len() < 2 || sp.choices.len() > 12
            || cx.room() < sp.choices.len() - 1 {
            continue;
        }
        let mine = text_of(v).unwrap_or_default().trim().to_lowercase();
        let mut labels = vec![];
        let mut vals = vec![];
        let mut hi = None;
        for c in &sp.choices {
            let y = if c.to_lowercase() == mine {
                hi = Some(labels.len());
                track_val(&b.outs, &t0)
            } else {
                cx.run(&with_value(b.given, name, c)).and_then(|a| track_val(&outs_of(cx.p, cx.key, &a), &t0))
            };
            labels.push(c.clone());
            vals.push(y);
        }
        let got: Vec<f64> = vals.iter().flatten().copied().collect();
        if got.len() < 2 {
            continue;
        }
        let (lo, hi_v) = minmax(&got);
        if hi_v - lo <= 1e-12 * hi_v.abs().max(lo.abs()) {
            continue;
        }
        let ylab = out_label(&b.outs, &t0);
        let body = bar_chart(&labels, &[(String::new(), vals.clone())], hi, &cap1(&clip(&sp.meaning, 50)), &ylab,
                             &b.header);
        let title = format!("{} for each choice of {}", clip(&strip_unit(&ylab), 60), lc1(&clip(&sp.meaning, 50)));
        let parts: Vec<String> = labels.iter().zip(&vals).map(|(l, v)| format!("{l}: {}",
            v.map(|x| py_g(x, 4)).unwrap_or_else(|| "no answer".into()))).collect();
        let caption = format!("{} for every choice of {}, every other input as you entered it (highlighted: your \
                               choice). {}.", cap1(&strip_unit(&ylab)), name, parts.join("; "));
        return Some((title, body, caption));
    }
    None
}

/// The verdict over a grid of the two inputs that move the answer most.
fn fig_grid(cx: &mut Ctx, b: &Base, k1: &Knob, k2: &Knob) -> Option<(String, String, String)> {
    let side = ((cx.share(0.6, 25, GRID_N * GRID_N) as f64).sqrt() as usize).min(GRID_N);
    if side < 5 {
        return None;
    }
    let xs = knob_points(k1, side - 1);
    let ys = knob_points(k2, side - 1);
    let xs: Vec<f64> = sample(&xs, side);
    let ys: Vec<f64> = sample(&ys, side);
    let mut cells: Vec<(usize, usize, String)> = vec![];
    let mut names: Vec<String> = vec![];
    for (j, y) in ys.iter().enumerate() {
        for (i, x) in xs.iter().enumerate() {
            let g = apply(&apply(b.given, k1, *x), k2, *y);
            let Some(a) = cx.run(&g) else { continue };
            let Some(c) = cat_val(&outs_of(cx.p, cx.key, &a), &k1.cat) else { continue };
            if !names.contains(&c) {
                if names.len() >= 4 {
                    return None;                              // not a verdict: too many distinct answers
                }
                names.push(c.clone());
            }
            cells.push((i, j, c));
        }
    }
    if names.len() < 2 || cells.len() < 8 {
        return None;
    }
    // one colour per answer in every chart of a capability: by the answer's sorted place, not by the order met
    // (pink meant "increase" in one chart and "decrease" in the next; math foundations review, 2026-11-13)
    names.sort();
    let body = grid_chart(&xs, &ys, &cells, &names, (k1.x0, k2.x0), &knob_axis_label(k1), &knob_axis_label(k2),
                          &b.header);
    let cname = out_meaning(&b.outs, k1.cat.as_ref()?);
    let title = format!("{} over {} and {}", clip(&cap1(&cname), 50), k_sym(k1), k_sym(k2));
    let caption = format!("{} for {} {} x {} combinations of {} and {} around your inputs (the other inputs as entered); \
                           the ring marks your inputs, which give {}. Blank cells admit no answer.",
                          cap1(&cname), cells.len(), xs.len(), ys.len(), k_sym(k1), k_sym(k2),
                          cat_val(&b.outs, &k1.cat).unwrap_or_default());
    Some((title, body, caption))
}

fn sample(xs: &[f64], n: usize) -> Vec<f64> {
    if xs.len() <= n {
        return xs.to_vec();
    }
    (0..n).map(|i| xs[i * (xs.len() - 1) / (n - 1)]).collect()
}

/// A list answer: bars (or a line for a long list); two lists of one length drawn one against the other when one of
/// them increases (nodes and weights, times and a curve).
fn fig_lists(b: &Base) -> Option<(String, String, String)> {
    let lists: Vec<(&Out, Vec<f64>)> = b.outs.iter().filter_map(|o| numlist(&o.val).map(|l| (o, l)))
        .filter(|(_, l)| l.len() >= 2).collect();
    if lists.is_empty() {
        return None;
    }
    // x against y
    for (ox, lx) in &lists {
        let inc = lx.len() >= 4 && lx.windows(2).all(|w| w[1] > w[0]);
        if !inc {
            continue;
        }
        // values against an increasing list (times, nodes); an index list is not a curve
        let ys: Vec<&(&Out, Vec<f64>)> = lists.iter()
            .filter(|(o, l)| o.sym != ox.sym && l.len() == lx.len() && !l.iter().all(|y| y.fract() == 0.0)).take(3).collect();
        if ys.is_empty() {
            continue;
        }
        let series: Vec<(String, Vec<Option<(f64, f64)>>)> = ys.iter()
            .map(|(o, l)| (o.sym.clone(), lx.iter().zip(l).map(|(x, y)| Some((*x, *y))).collect())).collect();
        let (xl, xh) = minmax(lx);
        let ylab = if ys.len() == 1 { out_label(&b.outs, &Track::S(ys[0].0.sym.clone())) } else { "Value".into() };
        let xlab = out_label(&b.outs, &Track::S(ox.sym.clone()));
        let body = stem_chart(&series, (xl, xh), &xlab, &ylab, &b.header);
        let title = format!("{} against {}", clip(&strip_unit(&ylab), 50), lc1(&clip(&strip_unit(&xlab), 50)));
        let caption = format!("The computed {} plotted against {} ({} points, from your inputs).",
                              ys.iter().map(|(o, _)| o.sym.as_str()).collect::<Vec<_>>().join(" and "), ox.sym, lx.len());
        return Some((title, body, caption));
    }
    let (o0, l0) = &lists[0];
    // lists drawn together: one length, one unit, and all whole numbers (indices, counts) or all not
    // and names of one family (m_new, v_new; dual_u, dual_v): an index list is not drawn beside values
    let whole = |l: &Vec<f64>| l.iter().all(|x| x.fract() == 0.0);
    let family = |a: &str, b: &str| -> bool {
        let pre = a.chars().zip(b.chars()).take_while(|(x, y)| x == y).count();
        let suf = a.chars().rev().zip(b.chars().rev()).take_while(|(x, y)| x == y).count();
        a == b || pre >= 3 || suf >= 3 || (a.len() <= 2 && b.len() <= 2)
    };
    let same: Vec<&(&Out, Vec<f64>)> = lists.iter()
        .filter(|(o, l)| l.len() == l0.len() && o.unit == o0.unit && whole(l) == whole(l0) && family(&o.sym, &o0.sym))
        .take(3).collect();
    // names for the entries: a list of text of the same length (classes, categories), else 1, 2, ...
    let names: Vec<String> = b.outs.iter().find_map(|o| {
        let a = o.val.as_array()?;
        if a.len() == l0.len() && a.iter().all(|x| x.is_string()) {
            Some(a.iter().map(|x| clip(x.as_str().unwrap_or(""), 10)).collect())
        } else {
            None
        }
    }).unwrap_or_else(|| (1..=l0.len()).map(|i| i.to_string()).collect());
    let ylab = if same.len() == 1 { out_label(&b.outs, &Track::S(o0.sym.clone())) } else {
        format!("Value{}", if o0.unit.is_empty() || o0.unit == "1" { String::new() } else { format!(" ({})", o0.unit) })
    };
    let syms: Vec<String> = same.iter().map(|(o, _)| o.sym.clone()).collect();
    let body = if l0.len() <= 40 {
        bar_chart(&names, &same.iter().map(|(o, l)| (o.sym.clone(), l.iter().map(|x| Some(*x)).collect())).collect::<Vec<_>>(),
                  None, "Entry", &ylab, &b.header)
    } else {
        let series: Vec<(String, Vec<Option<(f64, f64)>>)> = same.iter()
            .map(|(o, l)| (o.sym.clone(), l.iter().enumerate().map(|(i, y)| Some(((i + 1) as f64, *y))).collect())).collect();
        line_chart(&series, &[], (1.0, l0.len() as f64), "Entry", &ylab, None, "", &b.header, true)
    };
    let title = if same.len() == 1 { computed_title(&strip_unit(&ylab)) } else {
        format!("The computed {}", syms.join(", "))
    };
    let vals: Vec<String> = l0.iter().take(8).map(|x| py_g(*x, 4)).collect();
    let caption = format!("Every entry of {} as computed for your inputs ({} entries{}: {}{}).", syms.join(", "),
                          l0.len(), if same.len() > 1 { format!(", {} first", o0.sym) } else { String::new() },
                          vals.join(", "), if l0.len() > 8 { ", ..." } else { "" });
    Some((title, body, caption))
}

/// "The computed <what>" ("The computed the coordinate vector" reads "The coordinate vector, as computed").
fn computed_title(what: &str) -> String {
    let w = clip(what, 70);
    if w.to_lowercase().starts_with("the ") { format!("{}, as computed", cap1(&w)) } else { format!("The computed {}", lc1(&w)) }
}

/// A matrix answer as a grid of cells.
fn fig_matrix(b: &Base) -> Option<(String, String, String)> {
    for o in &b.outs {
        if o.sym.contains("edge") {
            continue;
        }
        let Some(m) = nummat(&o.val) else { continue };
        if m.len() * m[0].len() < 2 {
            continue;
        }
        // points (x, y) — the meeting points of two curves, a hole — are drawn where they are, not as a heatmap of their
        // coordinates (math foundations review, 2026-11-13)
        let words = format!("{} {}", o.sym, o.meaning).to_lowercase();
        if m[0].len() == 2 && m.len() <= 400 && (words.contains("point") || words.contains("hole") || words.contains("(x, y)")) {
            let p: Vec<(f64, f64)> = m.iter().map(|r| (r[0], r[1])).collect();
            let body = points_chart(&p, None, &[], &[], None, &b.header);
            let m_ = if o.meaning.is_empty() { o.sym.clone() } else { o.meaning.clone() };
            let listed: Vec<String> = p.iter().take(6).map(|q| format!("({}, {})", py_g(q.0, 4), py_g(q.1, 4))).collect();
            let title = computed_title(&clip(&m_, 60));
            let caption = format!("{} as points in the plane (x across, y up): {}{}.", o.sym, listed.join(", "),
                                  if p.len() > 6 { ", ..." } else { "" });
            return Some((title, body, caption));
        }
        let body = heat_chart(&m, &b.header);
        let m_ = if o.meaning.is_empty() { o.sym.clone() } else { o.meaning.clone() };
        let title = format!("{} ({} x {})", computed_title(&clip(&m_, 60)), m.len(), m[0].len());
        let caption = format!("Each cell is one entry of {} for your inputs (row by row; shade by size and sign).", o.sym);
        return Some((title, body, caption));
    }
    None
}

/// An answer that maps names to numbers (a composition): bars by name (at least `min` of them).
fn fig_dict(b: &Base, min: usize) -> Option<(String, String, String)> {
    for o in &b.outs {
        let Value::Object(map) = &o.val else { continue };
        if is_cx_obj(&o.val) || map.len() < min || map.len() > 30 {
            continue;
        }
        let pairs: Vec<(String, f64)> = map.iter().filter_map(|(k, v)| numv(v).map(|x| (k.clone(), x))).collect();
        if pairs.len() < min.max(1) {
            continue;
        }
        let names: Vec<String> = pairs.iter().map(|p| clip(&p.0, 10)).collect();
        let vals: Vec<Option<f64>> = pairs.iter().map(|p| Some(p.1)).collect();
        let ylab = out_label(&b.outs, &Track::S(o.sym.clone()));
        let body = bar_chart(&names, &[(o.sym.clone(), vals)], None, "", &ylab, &b.header);
        let title = computed_title(&strip_unit(&ylab));
        let caption = format!("{} for your inputs: {}.", cap1(&strip_unit(&ylab)),
                              pairs.iter().map(|(k, x)| format!("{k} {}", py_g(*x, 4))).collect::<Vec<_>>().join(", "));
        return Some((title, body, caption));
    }
    None
}

/// Complex roots in the plane, and their path as the most influential input sweeps.
fn fig_complex(cx: &mut Ctx, b: &Base, k: Option<&Knob>) -> Option<(String, String, String)> {
    let syms: Vec<String> = b.outs.iter().filter(|o| is_cx_obj(&o.val)).map(|o| o.sym.clone()).take(4).collect();
    if syms.is_empty() {
        return None;
    }
    let mine: Vec<(String, (f64, f64))> = syms.iter()
        .filter_map(|s| b.outs.iter().find(|o| &o.sym == s).and_then(|o| cxv(&o.val)).map(|z| (s.clone(), z))).collect();
    let mut paths: Vec<(String, Vec<(f64, f64)>)> = syms.iter().map(|s| (s.clone(), vec![])).collect();
    if let Some(k) = k {
        let n = cx.share(0.3, 9, 31);
        let xs = knob_points(k, n.max(2));
        let mut got: Vec<(f64, Vec<Option<(f64, f64)>>)> = vec![];
        for i in coarse_to_fine(xs.len()) {
            if !cx.left() {
                break;
            }
            let Some(a) = cx.run(&apply(b.given, k, xs[i])) else { continue };
            let o = outs_of(cx.p, cx.key, &a);
            got.push((xs[i], syms.iter().map(|s| o.iter().find(|o| &o.sym == s).and_then(|o| cxv(&o.val))).collect()));
        }
        got.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        for (_, zs) in &got {
            for (j, z) in zs.iter().enumerate() {
                if let Some(z) = z {
                    paths[j].1.push(*z);
                }
            }
        }
    }
    let unit_circle = b.outs.iter().any(|o| o.meaning.contains("radius") || o.sym == "radius");
    let body = complex_chart(&paths, &mine, unit_circle, &b.header);
    let title = match k {
        Some(k) => format!("The roots in the complex plane as {} varies", k_sym(k)),
        None => "The roots in the complex plane".to_string(),
    };
    let mut caption = format!("Your inputs give {}.", mine.iter().map(|(s, z)| format!("{s} = {}", cx_text(z.0, z.1)))
        .collect::<Vec<_>>().join(", "));
    if let Some(k) = k {
        let (lo, hi) = knob_range(k);
        caption.push_str(&format!(" The lines trace each root as {} runs from {} to {} (the other inputs as entered).",
                                  k_sym(k), py_g(lo, 4), py_g(hi, 4)));
    }
    if unit_circle {
        caption.push_str(" Dashed: the unit circle.");
    }
    Some((title, body, caption))
}

/// The node names of the capability's graph input (none without one).
fn graph_nodes(b: &Base) -> Vec<String> {
    let mut nodes: Vec<String> = vec![];
    for (name, v) in b.given {
        let Some(sp) = b.specs.iter().find(|s| &s.sym == name) else { continue };
        if sp.ty != "edges" {
            continue;
        }
        if let Some(pi) = &sp.proc_input {
            if let Ok(Val::Edges(e)) = crate::coerce::coerce(pi, v) {
                for (u, w, _) in e.iter().take(200) {
                    for n in [u, w] {
                        if !nodes.contains(n) {
                            nodes.push(n.clone());
                        }
                    }
                }
            }
        }
    }
    nodes
}

/// A graph input drawn with its edges (weights written on them), the nodes an answer names highlighted and per-node
/// answers written under the nodes.
fn fig_graph(b: &Base) -> Option<(String, String, String)> {
    let mut edges: Vec<(String, String, Option<f64>)> = vec![];
    let mut gname = String::new();
    for (name, v) in b.given {
        let Some(sp) = b.specs.iter().find(|s| &s.sym == name) else { continue };
        if sp.ty == "edges" {
            if let Some(pi) = &sp.proc_input {
                if let Ok(Val::Edges(e)) = crate::coerce::coerce(pi, v) {
                    edges = e;
                    gname = name.clone();
                    break;
                }
            }
        }
    }
    let from_output = edges.is_empty();
    if from_output {
        // an answer that is a list of edges (a decoded tree)
        for o in &b.outs {
            if !o.sym.contains("edge") {
                continue;
            }
            if let Some(a) = o.val.as_array() {
                for e in a.iter().take(60) {
                    let e = e.as_array()?;
                    if e.len() < 2 {
                        return None;
                    }
                    let name = |v: &Value| match v {
                        Value::String(s) => s.clone(),
                        other => other.to_string(),
                    };
                    edges.push((name(&e[0]), name(&e[1]), None));
                }
                gname = o.sym.clone();
            }
        }
    }
    if edges.is_empty() || edges.len() > 60 {
        return None;
    }
    let mut nodes: Vec<String> = vec![];
    for (u, v, _) in &edges {
        for n in [u, v] {
            if !nodes.contains(n) {
                nodes.push(n.clone());
            }
        }
    }
    if nodes.len() > 32 {
        return None;
    }
    // coordinates of the nodes, when a two-column matrix input gives one row per node (numbered 0, 1, ...)
    let mut pos: Option<Vec<(f64, f64)>> = None;
    for (name, v) in b.given {
        let Some(sp) = b.specs.iter().find(|s| &s.sym == name) else { continue };
        if sp.ty != "matrix" {
            continue;
        }
        if let Some(pi) = &sp.proc_input {
            if let Ok(Val::Mat(m)) = crate::coerce::coerce(pi, v) {
                if m[0].len() == 2 && nodes.iter().all(|n| n.parse::<usize>().map(|i| i < m.len()).unwrap_or(false)) {
                    pos = Some(nodes.iter().map(|n| { let r = &m[n.parse::<usize>().unwrap()]; (r[0], r[1]) }).collect());
                }
            }
        }
    }
    // nodes the answer names, and per-node values
    let as_name = |v: &Value| -> Option<String> {
        match v {
            Value::String(s) => Some(s.clone()),
            Value::Number(n) => n.as_i64().map(|i| i.to_string()),
            _ => None,
        }
    };
    let mut marked: Vec<String> = vec![];
    let mut per_node: Option<(String, Vec<String>)> = None;
    for o in &b.outs {
        match &o.val {
            Value::Array(a) if !a.is_empty() && a.len() <= 64 => {
                let names: Vec<Option<String>> = a.iter().map(as_name).collect();
                if a.len() == nodes.len() && a.iter().all(|x| numv(x).is_some()) && per_node.is_none()
                    && !names.iter().all(|n| n.as_ref().map(|n| nodes.contains(n)).unwrap_or(false)) {
                    per_node = Some((o.sym.clone(), a.iter().map(|x| py_g(numv(x).unwrap(), 3)).collect()));
                } else if names.iter().all(|n| n.as_ref().map(|n| nodes.contains(n)).unwrap_or(false))
                    && a.len() < nodes.len() {
                    for n in names.into_iter().flatten() {
                        if !marked.contains(&n) {
                            marked.push(n);
                        }
                    }
                }
            }
            v => {
                if let Some(n) = as_name(v) {
                    if nodes.contains(&n) && (o.sym.contains("best") || o.sym.contains("most") || o.sym.contains("node")
                                              || o.sym.contains("nearest") || o.sym.contains("farthest")) {
                        marked.push(n);
                    }
                }
            }
        }
    }
    // nodes the inputs single out (facilities, a source, an origin)
    let mut given_nodes: Vec<String> = vec![];
    for (name, v) in b.given {
        if name == &gname {
            continue;
        }
        let Some(sp) = b.specs.iter().find(|s| &s.sym == name) else { continue };
        let txt = text_of(v).unwrap_or_default();
        let parts: Vec<String> = match sp.ty.as_str() {
            "vector" | "integer" | "text" => txt.split([',', ';', ' ']).map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect(),
            "number" => vec![txt.trim().to_string()],
            _ => vec![],
        };
        if !parts.is_empty() && parts.len() < nodes.len() && parts.iter().all(|p| nodes.contains(p)) {
            for p in parts {
                if !given_nodes.contains(&p) {
                    given_nodes.push(p);
                }
            }
        }
    }
    let body = graph_chart(&nodes, &edges, pos.as_deref(), &marked, &given_nodes, per_node.as_ref(), &b.header);
    let title = if from_output { format!("The computed {} as a graph", gname) } else {
        format!("The network of {} ({} nodes, {} edges)", gname, nodes.len(), edges.len())
    };
    let mut caption = format!("The {} edges as given{}, drawn {}.", edges.len(),
                              if edges.iter().any(|e| e.2.is_some()) { " (weights written on them)" } else { "" },
                              if pos.is_some() { "at the given coordinates" } else { "with the nodes on a circle" });
    if !marked.is_empty() {
        caption.push_str(&format!(" Filled: nodes named by the answer ({}).", marked.join(", ")));
    }
    if !given_nodes.is_empty() {
        caption.push_str(&format!(" Ringed: nodes named in the inputs ({}).", given_nodes.join(", ")));
    }
    if let Some((s, _)) = &per_node {
        caption.push_str(&format!(" Under each node: its {s}."));
    }
    Some((title, body, caption))
}

/// Points of a two-column matrix input in the plane, coloured by a label list (an input or the answer), with
/// two-column answers (centres) and a two-entry query point; or two-entry vectors as arrows from the origin.
fn fig_points(b: &Base) -> Option<(String, String, String)> {
    let mut pts: Option<(String, Vec<(f64, f64)>)> = None;
    let mut vecs2: Vec<(String, (f64, f64), bool)> = vec![];
    for (name, v) in b.given {
        let Some(sp) = b.specs.iter().find(|s| &s.sym == name) else { continue };
        let Some(pi) = &sp.proc_input else { continue };
        match crate::coerce::coerce(pi, v) {
            Ok(Val::Mat(m)) if m[0].len() == 2 && m.len() >= 3 && m.len() <= 400 && pts.is_none() => {
                pts = Some((name.clone(), m.iter().map(|r| (r[0], r[1])).collect()));
            }
            Ok(Val::Vec(x)) if x.len() == 2 => vecs2.push((name.clone(), (x[0], x[1]), false)),
            _ => {}
        }
    }
    if b.given.iter().any(|(n, _)| b.specs.iter().any(|s| &s.sym == n && s.ty == "edges")) {
        return None;                                                  // drawn as a network
    }
    if let Some((pname, p)) = pts {
        // labels: an answer list of one entry per point, else an input list
        let n = p.len();
        let mut labels: Option<(String, Vec<String>)> = None;
        for o in &b.outs {
            if let Some(a) = o.val.as_array() {
                if a.len() == n && a.iter().all(|x| x.is_number() || x.is_string()) {
                    labels = Some((o.sym.clone(), a.iter().map(|x| match x { Value::String(s) => s.clone(),
                                                                             other => py_g(numv(other).unwrap_or(0.0), 4) }).collect()));
                    break;
                }
            }
        }
        if labels.is_none() {
            for (name, v) in b.given {
                let Some(sp) = b.specs.iter().find(|s| &s.sym == name) else { continue };
                if let Some(pi) = &sp.proc_input {
                    if let Ok(Val::Vec(x)) = crate::coerce::coerce(pi, v) {
                        if x.len() == n {
                            labels = Some((name.clone(), x.iter().map(|y| py_g(*y, 4)).collect()));
                            break;
                        }
                    }
                }
            }
        }
        let mut centres: Vec<(String, Vec<(f64, f64)>)> = vec![];
        for o in &b.outs {
            if let Some(m) = nummat(&o.val) {
                if m[0].len() == 2 {
                    centres.push((o.sym.clone(), m.iter().map(|r| (r[0], r[1])).collect()));
                }
            }
        }
        for (name, v) in b.given {
            let Some(sp) = b.specs.iter().find(|s| &s.sym == name) else { continue };
            if let Some(pi) = &sp.proc_input {
                if let Ok(Val::Mat(m)) = crate::coerce::coerce(pi, v) {
                    if m[0].len() == 2 && name != &pname && m.len() <= 20 {
                        centres.push((name.clone(), m.iter().map(|r| (r[0], r[1])).collect()));
                    }
                }
            }
        }
        // a line w.x + b = 0 when the inputs give a two-entry weight vector and a number b
        let line = vecs2.iter().find(|(n, _, _)| n == "w").and_then(|(_, w, _)| {
            let bv = b.given.iter().find(|(n, _)| n == "b").and_then(|(_, v)| text_of(v)).and_then(|t| lead_num(&t))?.0;
            Some((w.0, w.1, bv))
        });
        let query: Vec<(String, (f64, f64))> = vecs2.iter().filter(|(n, _, _)| n != "w").map(|(n, z, _)| (n.clone(), *z)).collect();
        let body = points_chart(&p, labels.as_ref(), &centres, &query, line, &b.header);
        let title = format!("The points of {} in the plane", pname);
        let mut caption = format!("The {n} rows of {pname} as points (first column across, second up)");
        if let Some((s, _)) = &labels {
            caption.push_str(&format!(", coloured by {s}"));
        }
        caption.push('.');
        for (s, c) in &centres {
            caption.push_str(&format!(" Squares: {s} ({} points).", c.len()));
        }
        for (s, z) in &query {
            caption.push_str(&format!(" Star: {s} = ({}, {}).", py_g(z.0, 4), py_g(z.1, 4)));
        }
        if line.is_some() {
            caption.push_str(" Line: where w.x + b = 0.");
        }
        return Some((title, body, caption));
    }
    // vectors in the plane: two-entry inputs and answers
    for o in &b.outs {
        // only an answer that IS a vector or a point: two coefficients (a quotient 2x + 3, a remainder) are not one
        // (math foundations review, 2026-11-13: polynomial division drawn as plane vectors)
        let m = format!("{} {}", o.sym, o.meaning).to_lowercase();
        if ["coefficient", "polynomial", "quotient", "remainder", "roots", "solutions", "interval", "bounds", "range"]
            .iter().any(|w| m.contains(w)) {
            continue;
        }
        if let Some(l) = numlist(&o.val) {
            if l.len() == 2 {
                vecs2.push((o.sym.clone(), (l[0], l[1]), true));
            }
        }
    }
    // one plane, one unit: the vectors of the first answer vector's unit
    let unit_of = |s: &str, ans: bool| -> String {
        if ans { b.outs.iter().find(|o| o.sym == s).map(|o| o.unit.clone()).unwrap_or_default() } else {
            b.specs.iter().find(|x| x.sym == s).map(|x| x.unit.clone()).unwrap_or_default()
        }
    };
    if let Some(u) = vecs2.iter().find(|v| v.2).map(|v| unit_of(&v.0, true)) {
        vecs2.retain(|v| unit_of(&v.0, v.2) == u);
    }
    let zero: Vec<String> = vecs2.iter().filter(|v| v.1 == (0.0, 0.0)).map(|v| v.0.clone()).collect();
    vecs2.retain(|v| v.1 != (0.0, 0.0));
    if vecs2.len() < 2 || !vecs2.iter().any(|v| v.2) {
        return None;
    }
    vecs2.truncate(5);
    let body = arrows_chart(&vecs2, &b.header);
    let title = "The vectors in the plane".to_string();
    let mut caption = format!("Each vector drawn from the origin: {} (answers drawn bold).",
                              vecs2.iter().map(|(s, z, _)| format!("{s} = ({}, {})", py_g(z.0, 4), py_g(z.1, 4)))
                                  .collect::<Vec<_>>().join(", "));
    if !zero.is_empty() {
        caption.push_str(&format!(" Zero (not drawn): {}.", zero.join(", ")));
    }
    Some((title, body, caption))
}

/// A text answer of the same length as a text input (a cipher): the shift of each letter.
fn fig_shift(b: &Base) -> Option<(String, String, String)> {
    for (name, v) in b.given {
        let Some(sp) = b.specs.iter().find(|s| &s.sym == name) else { continue };
        if sp.ty != "text" {
            continue;
        }
        // the letters only (a cipher may drop spaces and punctuation)
        let t: Vec<char> = text_of(v)?.chars().filter(|c| c.is_ascii_alphabetic()).collect();
        // a DNA or RNA sequence is no cipher: its complement or transcript does not "shift" letters (biology
        // reviews, 2026-11-13: "how far each letter moves" for a reverse complement)
        if t.iter().all(|c| "ACGTUNacgtun".contains(*c)) {
            continue;
        }
        for o in &b.outs {
            let Value::String(r) = &o.val else { continue };
            let r: Vec<char> = r.chars().filter(|c| c.is_ascii_alphabetic()).collect();
            if r.len() != t.len() || t.len() < 2 || t.len() > 400 {
                continue;
            }
            // an answer that only repeats the input (a unit echoed back, "mH" -> "mH") is no cipher
            if r.iter().zip(&t).all(|(c, a)| c.eq_ignore_ascii_case(a)) {
                continue;
            }
            let mut labels = vec![];
            let mut vals = vec![];
            for (a, c) in t.iter().zip(&r) {
                if a.is_ascii_alphabetic() && c.is_ascii_alphabetic() && labels.len() < 40 {
                    let d = ((c.to_ascii_uppercase() as i32 - a.to_ascii_uppercase() as i32).rem_euclid(26)) as f64;
                    labels.push(format!("{a}{c}"));
                    vals.push(Some(d));
                }
            }
            if vals.len() < 2 {
                continue;
            }
            let body = bar_chart(&labels, &[(String::new(), vals.clone())], None, "Letter (in, out)",
                                 "Shift in the alphabet", &b.header);
            let title = format!("How far each letter of {} moves", name);
            let caption = format!("For each letter of {name} and the same position of {}: how many places along the \
                                   alphabet (A to Z, wrapping round) it moved{}.", o.sym,
                                  if t.len() > 40 { " (first 40 letters)" } else { "" });
            return Some((title, body, caption));
        }
    }
    None
}

/// Sequences: the answer for prefixes of one sequence (a line), or of two (a grid).
fn fig_prefix(cx: &mut Ctx, b: &Base) -> Option<(String, String, String)> {
    let t0 = b.tracks.first()?.clone();
    if !matches!(t0, Track::S(_)) {
        return None;
    }
    let texts: Vec<(String, String)> = b.given.iter().filter_map(|(n, v)| {
        let sp = b.specs.iter().find(|s| &s.sym == n)?;
        let t = text_of(v)?;
        (sp.ty == "text" && t.chars().count() >= 3 && t.chars().count() <= 400
         && t.chars().all(|c| c.is_alphabetic() || c == ' ')).then(|| (n.clone(), t))
    }).take(2).collect();
    let ylab = out_label(&b.outs, &t0);
    if texts.len() == 2 {
        let (n1, s1) = &texts[0];
        let (n2, s2) = &texts[1];
        let c1: Vec<char> = s1.chars().collect();
        let c2: Vec<char> = s2.chars().collect();
        let side = ((cx.room() as f64).sqrt() as usize).min(10);
        if side < 3 {
            return None;
        }
        let l1 = sample_lengths(c1.len(), side);
        let l2 = sample_lengths(c2.len(), side);
        let mut m = vec![vec![f64::NAN; l2.len()]; l1.len()];
        for (i, a) in l1.iter().enumerate() {
            for (j, bb) in l2.iter().enumerate() {
                let p1: String = c1[..*a].iter().collect();
                let p2: String = c2[..*bb].iter().collect();
                let g = with_value(&with_value(b.given, n1, &p1), n2, &p2);
                if let Some(v) = if *a == c1.len() && *bb == c2.len() { track_val(&b.outs, &t0) } else {
                    cx.run(&g).and_then(|a| track_val(&outs_of(cx.p, cx.key, &a), &t0)) } {
                    m[i][j] = v;
                }
            }
        }
        if m.iter().flatten().filter(|x| x.is_finite()).count() < 4 {
            return None;
        }
        let rows: Vec<String> = l1.iter().map(|a| c1[..*a].iter().collect::<String>()).map(|s| tail(&s, 6)).collect();
        let cols: Vec<String> = l2.iter().map(|a| c2[..*a].iter().collect::<String>()).map(|s| tail(&s, 6)).collect();
        let body = heat_labeled(&m, &rows, &cols, &b.header);
        let title = format!("{} for the prefixes of {} and {}", clip(&strip_unit(&ylab), 50), n1, n2);
        let caption = format!("Each cell: {} for the first letters of {n1} (row, its last letters shown) and of {n2} \
                               (column); the bottom-right cell is your full input.", lc1(&strip_unit(&ylab)));
        return Some((title, body, caption));
    }
    let (n1, s1) = texts.first()?;
    let c1: Vec<char> = s1.chars().collect();
    let ls = sample_lengths(c1.len(), cx.room().min(40));
    let mut pts = vec![];
    for l in &ls {
        let p: String = c1[..*l].iter().collect();
        let v = if *l == c1.len() { track_val(&b.outs, &t0) } else {
            cx.run(&with_value(b.given, n1, &p)).and_then(|a| track_val(&outs_of(cx.p, cx.key, &a), &t0))
        };
        pts.push(v.map(|y| (*l as f64, y)));
    }
    if pts.iter().flatten().count() < 3 {
        return None;
    }
    let ys: Vec<f64> = pts.iter().flatten().map(|p| p.1).collect();
    let (lo, hi) = minmax(&ys);
    if hi - lo <= 1e-12 * hi.abs().max(lo.abs()) {
        return None;
    }
    let y0 = track_val(&b.outs, &t0)?;
    let body = line_chart(&[(String::new(), pts)], &[], (1.0, c1.len() as f64), &format!("Length of the prefix of {n1}"),
                          &ylab, Some((c1.len() as f64, y0)), "your full input", &b.header, true);
    let title = format!("{} for growing prefixes of {}", clip(&strip_unit(&ylab), 50), n1);
    let caption = format!("{} for the first 1, 2, ... letters of {n1}; the last point is your full input ({} = {}).",
                          cap1(&strip_unit(&ylab)), track_label(&b.outs, &t0), py_g(y0, 4));
    Some((title, body, caption))
}

fn sample_lengths(n: usize, k: usize) -> Vec<usize> {
    if n <= k {
        return (1..=n).collect();
    }
    if k < 2 {
        return vec![n];
    }
    let mut v: Vec<usize> = (0..k).map(|i| 1 + i * (n - 1) / (k - 1)).collect();
    v.dedup();
    v
}

fn tail(s: &str, n: usize) -> String {
    let c: Vec<char> = s.chars().collect();
    if c.len() <= n { s.to_string() } else { format!("..{}", c[c.len() - n + 2..].iter().collect::<String>()) }
}

/// A unit that carries no physical dimension (a pure number).
fn pure_unit(u: &str) -> bool {
    let u = u.trim();
    u.is_empty() || u == "1"
}

/// Several numeric answers of one unit, side by side (at least `min` of them).
fn fig_outputs(b: &Base, min: usize) -> Option<(String, String, String)> {
    // answers of one unit (a last resort for pure numbers: only when nothing else could be drawn)
    let nums: Vec<&Out> = b.outs.iter().filter(|o| numv(&o.val).is_some()).collect();
    let mut best: Vec<&Out> = vec![];
    for o in &nums {
        // pure numbers ("1") are of many kinds: a count, a fraction, a pH, a magnitude, a pressure in mmHg next to
        // one in atm; side by side on one axis they compare unlike things (computational biology, earth science,
        // physiology reviews, 2026-11-13), so a pure number stands alone
        if pure_unit(&o.unit) {
            if best.is_empty() {
                best = vec![*o];
            }
            continue;
        }
        let same: Vec<&Out> = nums.iter().filter(|x| x.unit == o.unit).copied().collect();
        if same.len() > best.len() {
            best = same;
        }
    }
    if best.len() < min.max(1) {
        return None;
    }
    best.truncate(12);
    let labels: Vec<String> = best.iter().map(|o| clip(&o.sym, 12)).collect();
    let vals: Vec<Option<f64>> = best.iter().map(|o| numv(&o.val)).collect();
    let u = &best[0].unit;
    let ylab = if u.is_empty() || u == "1" { "Value".to_string() } else { format!("Value ({u})") };
    let body = bar_chart(&labels, &[(String::new(), vals)], None, "", &ylab, &b.header);
    let title = if best.len() == 1 {
        computed_title(&strip_unit(&out_label(&b.outs, &Track::S(best[0].sym.clone()))))
    } else {
        "The computed values side by side".to_string()
    };
    let caption = format!("{} for your inputs: {}.", if best.len() == 1 { "The answer" } else { "Every answer of one unit" },
                          best.iter().map(|o| format!("{} ({}) = {}", o.sym, if o.meaning.is_empty() { &o.sym } else { &o.meaning },
                                                      py_g(numv(&o.val).unwrap(), 4))).collect::<Vec<_>>().join("; "));
    Some((title, body, caption))
}

// ------------------------------------------------------------------------------------------------ truth tables
/// The most variables a truth table enumerates (64 rows, one solve each).
const MAX_TT_VARS: usize = 6;

/// A text of truth values ("A=1, B=0", "p = true; q = false"): each variable and the byte range of its value, in
/// the order written; None for any other text, or for more than MAX_TT_VARS variables.
fn assignment_spans(t: &str) -> Option<Vec<(String, usize, usize)>> {
    let b = t.as_bytes();
    if b.len() > 400 {
        return None;
    }
    let mut i = 0;
    let mut out: Vec<(String, usize, usize)> = vec![];
    loop {
        while i < b.len() && (b[i].is_ascii_whitespace() || b[i] == b',' || b[i] == b';') {
            i += 1;
        }
        if i >= b.len() {
            break;
        }
        if !(b[i].is_ascii_alphabetic() || b[i] == b'_') {
            return None;
        }
        let s = i;
        while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'_') {
            i += 1;
        }
        let name = t[s..i].to_string();
        while i < b.len() && b[i].is_ascii_whitespace() {
            i += 1;
        }
        if i >= b.len() || !(b[i] == b'=' || b[i] == b':') {
            return None;
        }
        i += 1;
        while i < b.len() && b[i].is_ascii_whitespace() {
            i += 1;
        }
        let vs = i;
        while i < b.len() && b[i].is_ascii_alphanumeric() {
            i += 1;
        }
        if !matches!(t[vs..i].to_ascii_lowercase().as_str(), "0" | "1" | "true" | "false" | "t" | "f")
            || out.iter().any(|o| o.0 == name) || out.len() >= MAX_TT_VARS {
            return None;
        }
        out.push((name, vs, i));
    }
    if out.is_empty() { None } else { Some(out) }
}

fn truth_of(word: &str) -> bool {
    matches!(word.to_ascii_lowercase().as_str(), "1" | "true" | "t")
}

/// A truth value spelled as the visitor spelled one (1/0, true/false, t/f, in the same case).
fn truth_word(style: &str, v: bool) -> String {
    let w = match style.to_ascii_lowercase().as_str() {
        "0" | "1" => return (if v { "1" } else { "0" }).to_string(),
        "t" | "f" => if v { "t" } else { "f" },
        _ => if v { "true" } else { "false" },
    };
    if style.chars().all(|c| c.is_ascii_uppercase()) {
        w.to_ascii_uppercase()
    } else if style.chars().next().is_some_and(|c| c.is_ascii_uppercase()) {
        cap1(w)
    } else {
        w.to_string()
    }
}

/// The assignment text with new truth values (one per variable, in the order written).
fn with_truths(t: &str, spans: &[(String, usize, usize)], vals: &[bool]) -> String {
    let mut out = String::new();
    let mut at = 0;
    for ((_, s, e), v) in spans.iter().zip(vals) {
        out.push_str(&t[at..*s]);
        out.push_str(&truth_word(&t[*s..*e], *v));
        at = *e;
    }
    out.push_str(&t[at..]);
    out
}

/// The variables of a propositional formula written with logic operators (~ ¬ ! & | ^ -> <-> => <=> → ↔ ⇒ ⇔ ∧ ∨ ⊕
/// or the words and, or, not, xor, implies, iff), in alphabetical order, and where each occurs (byte ranges); None
/// for any other text (arithmetic, a chemical formula, prose), for no operator, or for more than MAX_TT_VARS.
#[allow(clippy::type_complexity)]
fn formula_vars(t: &str) -> Option<(Vec<String>, Vec<(usize, usize, String)>)> {
    if t.len() > 400 {
        return None;
    }
    const WORDS: [&str; 9] = ["and", "or", "not", "xor", "implies", "iff", "nand", "nor", "xnor"];
    let mut occ: Vec<(usize, usize, String)> = vec![];
    let mut ops = 0;
    let mut it = t.char_indices().peekable();
    while let Some((i, c)) = it.next() {
        if c.is_whitespace() || c == '(' || c == ')' {
            continue;
        }
        if c.is_ascii_alphabetic() || c == '_' {
            let mut e = i + 1;
            while let Some(&(j, d)) = it.peek() {
                if d.is_ascii_alphanumeric() || d == '_' {
                    e = j + 1;
                    it.next();
                } else {
                    break;
                }
            }
            let w = t[i..e].to_ascii_lowercase();
            if WORDS.contains(&w.as_str()) {
                ops += 1;
            } else if w != "true" && w != "false" {
                occ.push((i, e, t[i..e].to_string()));
            }
            continue;
        }
        if c == '0' || c == '1' {
            if it.peek().is_some_and(|p| p.1.is_ascii_alphanumeric() || p.1 == '.') {
                return None;
            }
            continue;
        }
        match c {
            '~' | '¬' | '!' | '&' | '|' | '^' | '∧' | '∨' | '⊕' | '→' | '↔' | '⇒' | '⇔' => ops += 1,
            '-' | '=' if it.peek().is_some_and(|p| p.1 == '>') => {
                it.next();
                ops += 1;
            }
            '<' if t[i..].starts_with("<->") || t[i..].starts_with("<=>") => {
                it.next();
                it.next();
                ops += 1;
            }
            _ => return None,
        }
    }
    let mut vars: Vec<String> = occ.iter().map(|o| o.2.clone()).collect();
    vars.sort();
    vars.dedup();
    if ops == 0 || vars.is_empty() || vars.len() > MAX_TT_VARS {
        return None;
    }
    Some((vars, occ))
}

/// The formula with every variable replaced by a truth constant.
fn substituted(t: &str, occ: &[(usize, usize, String)], val: &dyn Fn(&str) -> bool, consts: (&str, &str)) -> String {
    let mut out = String::new();
    let mut at = 0;
    for (s, e, name) in occ {
        out.push_str(&t[at..*s]);
        out.push_str(if val(name) { consts.0 } else { consts.1 });
        at = *e;
    }
    out.push_str(&t[at..]);
    out
}

/// An output as a number: a number, or 1/0 for true/false.
fn tval(outs: &[Out], sym: &str) -> Option<f64> {
    outs.iter().find(|o| o.sym == sym).and_then(|o| match &o.val {
        Value::Bool(b) => Some(if *b { 1.0 } else { 0.0 }),
        v => numv(v),
    })
}

/// The answer over every assignment of true and false to the variables of a text input: a truth table (or, for 5
/// and 6 variables, a grid ordered as a Karnaugh map). Each row is one solve of the capability: either the
/// assignment input rewritten ("A=1, B=0" -> every combination), or each formula with its variables replaced by
/// truth constants; a formula's own column is shown when the answer tells its value apart (the answer for the
/// constant formulas true and false differs), the answer itself otherwise.
fn fig_truth(cx: &mut Ctx, b: &Base) -> Option<(String, String, String)> {
    let texts: Vec<(String, String)> = b.given.iter().filter_map(|(n, v)| {
        let sp = b.specs.iter().find(|s| &s.sym == n)?;
        (sp.ty == "text").then(|| text_of(v).map(|t| (n.clone(), t))).flatten()
    }).collect();
    // the answer followed: the primary output when it is a truth value or a number, else the first such output
    let osym = if tval(&b.outs, &b.primary).is_some() { b.primary.clone() } else {
        b.outs.iter().find(|o| tval(&b.outs, &o.sym).is_some())?.sym.clone()
    };
    let omean = out_meaning(&b.outs, &osym);
    // 1. an assignment input
    if let Some((aname, atext, spans)) = texts.iter().find_map(|(n, t)| assignment_spans(t).map(|s| (n, t, s))) {
        let vars: Vec<String> = spans.iter().map(|s| s.0.clone()).collect();
        let n = vars.len();
        let rows = 1usize << n;
        if cx.room() + 1 < rows {
            return None;
        }
        let mine: usize = spans.iter().fold(0, |acc, (_, s, e)| (acc << 1) | truth_of(&atext[*s..*e]) as usize);
        let mut vals: Vec<Option<f64>> = vec![None; rows];
        for (r, slot) in vals.iter_mut().enumerate() {
            *slot = if r == mine { tval(&b.outs, &osym) } else {
                let bits: Vec<bool> = (0..n).map(|k| (r >> (n - 1 - k)) & 1 == 1).collect();
                let g = with_value(b.given, aname, &with_truths(atext, &spans, &bits));
                cx.run(&g).and_then(|a| tval(&outs_of(cx.p, cx.key, &a), &osym))
            };
        }
        if vals.iter().flatten().count() < 2 {
            return None;
        }
        let ones = vals.iter().flatten().filter(|v| **v != 0.0).count();
        let cols = vec![(osym.clone(), vals)];
        let body = if n <= 4 { truth_chart(&vars, &cols, Some(mine), &b.header) } else {
            kmap_chart(&vars, &cols[0].1, Some(mine), &b.header)
        };
        let title = format!("Truth table: {} for every value of {}", osym, clip(&vars.join(", "), 40));
        let caption = format!("{} for each of the {rows} assignments of 0 and 1 to {}{}, each computed with that \
                               assignment as {aname} and your other inputs as entered; {ones} of {rows} give a nonzero \
                               value. {}: your {aname} ({}) gives {} = {}.", cap1(&strip_unit(&omean)), vars.join(", "),
                              if n <= 4 { "" } else { " (cells ordered as a Karnaugh map)" },
                              if n <= 4 { "Ringed row" } else { "Ringed cell" }, clip(atext, 40), osym,
                              tval(&b.outs, &osym).map(|v| py_g(v, 4)).unwrap_or_default());
        return Some((title, body, caption));
    }
    // 2. formulas: only for an answer that is a truth value
    if !b.outs.iter().any(|o| matches!(o.val, Value::Bool(_))) {
        return None;
    }
    let osym = if matches!(b.outs.iter().find(|o| o.sym == b.primary).map(|o| &o.val), Some(Value::Bool(_))) {
        b.primary.clone()
    } else {
        b.outs.iter().find(|o| matches!(o.val, Value::Bool(_)))?.sym.clone()
    };
    let omean = out_meaning(&b.outs, &osym);
    #[allow(clippy::type_complexity)]
    let forms: Vec<(String, String, Vec<(usize, usize, String)>)> = texts.iter()
        .filter_map(|(n, t)| formula_vars(t).map(|(_, occ)| (n.clone(), t.clone(), occ))).take(3).collect();
    if forms.is_empty() {
        return None;
    }
    let mut vars: Vec<String> = forms.iter().flat_map(|f| f.2.iter().map(|o| o.2.clone())).collect();
    vars.sort();
    vars.dedup();
    let n = vars.len();
    if n > MAX_TT_VARS {
        return None;
    }
    let rows = 1usize << n;
    if cx.room() < rows + 1 {
        return None;
    }
    // the truth constants this capability reads: all formulas true must solve
    let all_const = |c: &str, cx: &mut Ctx| -> Option<f64> {
        let mut g = b.given.to_vec();
        for (name, _, _) in &forms {
            g = with_value(&g, name, c);
        }
        cx.run(&g).and_then(|a| tval(&outs_of(cx.p, cx.key, &a), &osym))
    };
    let consts = if let Some(v) = all_const("1", cx) { (("1", "0"), v) } else {
        (("true", "false"), all_const("true", cx)?)
    };
    let ((one, zero), o_all1) = consts;
    let base_true: Vec<(String, Input)> = forms.iter().fold(b.given.to_vec(), |g, (name, _, _)| with_value(&g, name, one));
    // which formulas the answer tells apart (their own column): the answer with that formula false, the others true
    let mut tells: Vec<bool> = vec![];
    for (name, _, _) in &forms {
        let room_after = cx.room().saturating_sub(1);
        let enough = room_after >= rows * (2 + tells.iter().filter(|t| **t).count());
        let o0 = if enough { cx.run(&with_value(&base_true, name, zero)).and_then(|a| tval(&outs_of(cx.p, cx.key, &a), &osym)) } else { None };
        tells.push(o0.is_some_and(|v| v != o_all1));
    }
    let row_bits = |r: usize| -> Vec<bool> { (0..n).map(|k| (r >> (n - 1 - k)) & 1 == 1).collect() };
    let mut cols: Vec<(String, Vec<Option<f64>>)> = vec![];
    for (fi, (name, text, occ)) in forms.iter().enumerate() {
        if !tells[fi] {
            continue;
        }
        let mut vals = vec![None; rows];
        for (r, slot) in vals.iter_mut().enumerate() {
            let bits = row_bits(r);
            let val = |v: &str| vars.iter().position(|x| x == v).map(|k| bits[k]).unwrap_or(false);
            let g = with_value(&base_true, name, &substituted(text, occ, &val, (one, zero)));
            *slot = cx.run(&g).and_then(|a| tval(&outs_of(cx.p, cx.key, &a), &osym)).map(|v| if v == o_all1 { 1.0 } else { 0.0 });
        }
        cols.push((name.clone(), vals));
    }
    // the answer itself for every row (unless one formula's own column already says it all)
    let mut verdict: Option<Vec<Option<f64>>> = None;
    if !(forms.len() == 1 && cols.len() == 1) {
        let mut vals = vec![None; rows];
        for (r, slot) in vals.iter_mut().enumerate() {
            let bits = row_bits(r);
            let val = |v: &str| vars.iter().position(|x| x == v).map(|k| bits[k]).unwrap_or(false);
            let g = forms.iter().fold(b.given.to_vec(), |g, (name, text, occ)| {
                with_value(&g, name, &substituted(text, occ, &val, (one, zero)))
            });
            *slot = cx.run(&g).and_then(|a| tval(&outs_of(cx.p, cx.key, &a), &osym));
        }
        verdict = Some(vals);
    }
    if let Some(v) = verdict {
        cols.push((osym.clone(), v));
    }
    if cols.iter().all(|c| c.1.iter().flatten().count() < 2) {
        return None;
    }
    // a row the answer names (a satisfying assignment, a counterexample)
    let mut mark: Option<(String, usize)> = None;
    for o in &b.outs {
        if let Value::Object(m) = &o.val {
            if !m.is_empty() && m.len() <= n && m.keys().all(|k| vars.contains(k)) {
                let mut r = 0usize;
                let mut ok = true;
                for (k, v) in vars.iter().enumerate() {
                    let bit = match m.get(v) {
                        Some(Value::Bool(x)) => *x,
                        Some(x) => numv(x).map(|y| y != 0.0).unwrap_or_else(|| { ok = false; false }),
                        None => false,
                    };
                    if bit {
                        r |= 1 << (n - 1 - k);
                    }
                }
                if ok {
                    mark = Some((o.sym.clone(), r));
                    break;
                }
            }
        }
    }
    let shown = cols.last().map(|c| c.1.clone()).unwrap_or_default();
    let ones = shown.iter().flatten().filter(|v| **v != 0.0).count();
    let body = if n <= 4 { truth_chart(&vars, &cols, mark.as_ref().map(|m| m.1), &b.header) } else {
        kmap_chart(&vars, &shown, mark.as_ref().map(|m| m.1), &b.header)
    };
    let names: Vec<&str> = forms.iter().map(|f| f.0.as_str()).collect();
    let title = format!("Truth table of {} over {}", clip(&names.join(" and "), 30), clip(&vars.join(", "), 30));
    let last = cols.last().map(|c| c.0.clone()).unwrap_or_default();
    let mut caption = format!("Every assignment of true (1) and false (0) to {} ({rows} {}), each computed with the \
                               variables of {} replaced by those values. ", vars.join(", "),
                              if n <= 4 { "rows" } else { "cells, ordered as a Karnaugh map" }, names.join(" and "));
    let own: Vec<&str> = cols.iter().filter(|c| c.0 != osym).map(|c| c.0.as_str()).collect();
    if own.len() == 1 && last != osym {
        caption.push_str(&format!("Column {}: the formula's value in that row; {ones} of {rows} rows make it true.",
                                  own[0]));
    } else {
        if !own.is_empty() && n <= 4 {
            caption.push_str(&format!("Columns {}: each formula's value in that row; ", own.join(", ")));
        }
        caption.push_str(&format!("column {osym} ({}): the answer for that row's values alone; {ones} of {rows} rows \
                                   give true.", lc1(&omean)));
    }
    if let Some((s, r)) = &mark {
        let bits = row_bits(*r);
        caption.push_str(&format!(" Ringed: {s} ({}).", vars.iter().zip(&bits)
            .map(|(v, x)| format!("{v} = {}", if *x { 1 } else { 0 })).collect::<Vec<_>>().join(", ")));
    }
    Some((title, body, caption))
}

/// A truth table: the variables' 0/1 columns, then the value columns (shaded where nonzero); one row ringed.
pub(crate) fn truth_chart(vars: &[String], cols: &[(String, Vec<Option<f64>>)], mark: Option<usize>, header: &str) -> String {
    let n = vars.len();
    let rows = 1usize << n;
    let ncol = n + cols.len();
    let (top, bottom) = (34.0, 334.0);
    let rh = ((bottom - top) / (rows + 1) as f64).min(24.0);
    let top = top + ((bottom - top) - rh * (rows + 1) as f64) / 2.0;     // centred in the space below the header
    let cw = (440.0 / ncol as f64).min(72.0);
    let x0 = (520.0 - cw * ncol as f64) / 2.0;
    let font = (rh * 0.72).clamp(8.0, 14.0);
    let mut out = String::new();
    header_text(&mut out, header);
    let names: Vec<String> = vars.iter().cloned().chain(cols.iter().map(|c| c.0.clone())).collect();
    for (j, name) in names.iter().enumerate() {
        out.push_str(&format!("<text class=\"axp-bt\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"{font:.0}\">{}</text>",
                              x0 + cw * (j as f64 + 0.5), top + rh * 0.72, esc(&clip(name, ((cw / 6.8) as usize).max(3)))));
    }
    let xs = x0 + cw * n as f64;
    out.push_str(&format!("<line class=\"axp-g\" x1=\"{xs:.1}\" y1=\"{top:.1}\" x2=\"{xs:.1}\" y2=\"{:.1}\"/>\
                           <line class=\"axp-g\" x1=\"{x0:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\"/>",
                          top + rh * (rows + 1) as f64, top + rh, x0 + cw * ncol as f64, top + rh));
    for r in 0..rows {
        let y = top + rh * (r + 1) as f64;
        for k in 0..n {
            let bit = (r >> (n - 1 - k)) & 1;
            out.push_str(&format!("<text class=\"axp-t\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"{font:.0}\">{bit}</text>",
                                  x0 + cw * (k as f64 + 0.5), y + rh * 0.72));
        }
        for (j, (_, vals)) in cols.iter().enumerate() {
            let x = xs + cw * j as f64;
            match vals.get(r).copied().flatten() {
                Some(v) => {
                    out.push_str(&format!("<rect class=\"{}\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>\
                                           <text class=\"axp-bt\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"{font:.0}\">{}</text>",
                                          if v != 0.0 { "axp-b" } else { "axp-b4" }, x + 3.0, y + 1.0, cw - 6.0, rh - 2.0,
                                          x + cw / 2.0, y + rh * 0.72, esc(&py_g(v, 4))));
                }
                None => out.push_str(&format!("<text class=\"axp-t\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"{font:.0}\">-</text>",
                                              x + cw / 2.0, y + rh * 0.72)),
            }
        }
        if mark == Some(r) {
            out.push_str(&format!("<rect class=\"axp-k\" x=\"{:.1}\" y=\"{y:.1}\" width=\"{:.1}\" height=\"{rh:.1}\"/>",
                                  x0 - 2.0, cw * ncol as f64 + 4.0));
        }
    }
    out
}

/// Values over every assignment of 5 or 6 variables as a grid in Karnaugh-map order (Gray code along each side).
pub(crate) fn kmap_chart(vars: &[String], vals: &[Option<f64>], mark: Option<usize>, header: &str) -> String {
    let n = vars.len();
    let rv = n / 2;
    let cv = n - rv;
    let (nr, nc) = (1usize << rv, 1usize << cv);
    let gray = |k: usize| k ^ (k >> 1);
    let cell = (380.0 / nc as f64).min(240.0 / nr as f64).min(48.0);
    let x0 = 120.0 + (380.0 - cell * nc as f64) / 2.0;
    let y0 = 74.0;
    let font = (cell * 0.4).clamp(8.0, 14.0);
    let mut out = String::new();
    header_text(&mut out, header);
    out.push_str(&format!("<text class=\"axp-l\" x=\"{:.1}\" y=\"48\" text-anchor=\"middle\">{}</text>\
                           <text class=\"axp-l\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\">{}</text>",
                          x0 + cell * nc as f64 / 2.0, esc(&vars[rv..].join(" ")), x0 - 40.0, y0 - 6.0,
                          esc(&vars[..rv].join(" "))));
    let bin = |v: usize, w: usize| (0..w).rev().map(|i| if (v >> i) & 1 == 1 { '1' } else { '0' }).collect::<String>();
    for j in 0..nc {
        out.push_str(&format!("<text class=\"axp-lt\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
                              x0 + cell * (j as f64 + 0.5), y0 - 6.0, bin(gray(j), cv)));
    }
    for i in 0..nr {
        let y = y0 + cell * i as f64;
        out.push_str(&format!("<text class=\"axp-lt\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\">{}</text>",
                              x0 - 6.0, y + cell / 2.0 + 4.0, bin(gray(i), rv)));
        for j in 0..nc {
            let r = (gray(i) << cv) | gray(j);
            let x = x0 + cell * j as f64;
            match vals.get(r).copied().flatten() {
                Some(v) => out.push_str(&format!("<rect class=\"{}\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>\
                                                  <text class=\"axp-bt\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"{font:.0}\">{}</text>",
                                                 if v != 0.0 { "axp-b" } else { "axp-b4" }, cell - 2.0, cell - 2.0,
                                                 x + cell / 2.0 - 1.0, y + cell / 2.0 + font * 0.35, esc(&py_g(v, 4)))),
                None => out.push_str(&format!("<rect class=\"axp-f\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>",
                                              cell - 2.0, cell - 2.0)),
            }
            if mark == Some(r) {
                out.push_str(&format!("<rect class=\"axp-k\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>",
                                      x - 2.0, y - 2.0, cell + 2.0, cell + 2.0));
            }
        }
    }
    out
}

// ------------------------------------------------------------------------------------------------ genotypes
/// A genotype written as letter pairs, one pair per gene ("AaBb", "GGhh"): the pairs; None for any other text.
fn genotype_pairs(t: &str) -> Option<Vec<(char, char)>> {
    let c: Vec<char> = t.trim().chars().collect();
    if c.len() < 2 || c.len() > 16 || c.len() % 2 != 0 || !c.iter().all(|x| x.is_ascii_alphabetic()) {
        return None;
    }
    let pairs: Vec<(char, char)> = c.chunks(2).map(|p| (p[0], p[1])).collect();
    pairs.iter().all(|(a, b)| a.eq_ignore_ascii_case(b)).then_some(pairs)
}

/// The three genotypes of one gene (homozygous dominant, heterozygous as the visitor wrote it, homozygous recessive).
fn gene_variants(p: (char, char)) -> [String; 3] {
    let (u, l) = (p.0.to_ascii_uppercase(), p.0.to_ascii_lowercase());
    let het = if p.0 != p.1 { format!("{}{}", p.0, p.1) } else { format!("{u}{l}") };
    [format!("{u}{u}"), het, format!("{l}{l}")]
}

fn with_gene(pairs: &[(char, char)], j: usize, g: &str) -> String {
    pairs.iter().enumerate().map(|(i, (a, b))| if i == j { g.to_string() } else { format!("{a}{b}") }).collect()
}

/// Inputs that are genotypes of the same genes (a cross): the answer for every genotype of each gene of the last
/// one (the others as entered), and over the genotypes of the first gene of the first two (a 3 x 3 grid).
fn fig_genotypes(cx: &mut Ctx, b: &Base) -> Vec<(String, String, String)> {
    let mut out = vec![];
    let Some(t0) = b.tracks.first().cloned() else { return out };
    let gs: Vec<(String, String, Vec<(char, char)>)> = b.given.iter().filter_map(|(n, v)| {
        let sp = b.specs.iter().find(|s| &s.sym == n)?;
        let t = text_of(v)?;
        (sp.ty == "text").then(|| genotype_pairs(&t).map(|p| (n.clone(), t.trim().to_string(), p))).flatten()
    }).collect();
    let genes = |p: &[(char, char)]| p.iter().map(|x| x.0.to_ascii_lowercase()).collect::<String>();
    if gs.len() < 2 || gs.iter().any(|g| genes(&g.2) != genes(&gs[0].2)) {
        return out;
    }
    let y0 = track_val(&b.outs, &t0);
    let ylab = out_label(&b.outs, &t0);
    let what = strip_unit(&ylab);
    // 1. every genotype of each gene of the last genotype input
    let (name, text, pairs) = gs[gs.len() - 1].clone();
    if cx.room() >= 2 * pairs.len() {
        let mut labels = vec![];
        let mut vals = vec![];
        let mut hl = vec![];
        for (j, p) in pairs.iter().enumerate() {
            let mine = format!("{}{}", p.0, p.1);
            for g in gene_variants(*p) {
                if g == mine {
                    hl.push(labels.len());
                    vals.push(y0);
                } else {
                    let gv = with_value(b.given, &name, &with_gene(&pairs, j, &g));
                    vals.push(cx.run(&gv).and_then(|a| track_val(&outs_of(cx.p, cx.key, &a), &t0)));
                }
                labels.push(g);
            }
        }
        if vals.iter().flatten().count() >= 2 {
            let body = bar_chart_hl(&labels, &[(String::new(), vals)], &hl, &format!("Genotype of one gene of {name}"),
                                    &ylab, &b.header);
            let title = format!("{} for each genotype of each gene of {}", clip(&cap1(&what), 50), name);
            let caption = format!("Each bar: {} with one gene of {name} changed to the genotype under it, every other \
                                   gene and input as you entered them. Highlighted: your {name} = {text}, which gives \
                                   {} = {}.", lc1(&what), track_label(&b.outs, &t0),
                                  y0.map(|v| py_g(v, 4)).unwrap_or_default());
            out.push((title, body, caption));
        }
    }
    // 2. the first gene of the first two genotype inputs, every combination
    let (na, ta, pa) = gs[0].clone();
    let (nb, tb, pb) = gs[1].clone();
    if cx.room() >= 8 {
        let va = gene_variants(pa[0]);
        let vb = gene_variants(pb[0]);
        let mut m = vec![vec![f64::NAN; 3]; 3];
        for (i, ga) in va.iter().enumerate() {
            for (j, gb) in vb.iter().enumerate() {
                let mine = *ga == format!("{}{}", pa[0].0, pa[0].1) && *gb == format!("{}{}", pb[0].0, pb[0].1);
                let v = if mine { y0 } else {
                    let g = with_value(&with_value(b.given, &na, &with_gene(&pa, 0, ga)), &nb, &with_gene(&pb, 0, gb));
                    cx.run(&g).and_then(|a| track_val(&outs_of(cx.p, cx.key, &a), &t0))
                };
                if let Some(v) = v {
                    m[i][j] = v;
                }
            }
        }
        if m.iter().flatten().filter(|x| x.is_finite()).count() >= 4 {
            let rows: Vec<String> = va.iter().map(|g| format!("{na} {g}")).collect();
            let cols: Vec<String> = vb.iter().map(|g| format!("{nb} {g}")).collect();
            let body = heat_labeled(&m, &rows, &cols, &b.header);
            let title = format!("{} for every genotype of the first gene of {na} and {nb}", clip(&cap1(&what), 40));
            let caption = format!("Each cell: {} with the first gene of {na} (row) and of {nb} (column) as labelled, every \
                                   other gene and input as you entered them (your {na} = {ta}, {nb} = {tb}; {} = {}).",
                                  lc1(&what), track_label(&b.outs, &t0), y0.map(|v| py_g(v, 4)).unwrap_or_default());
            out.push((title, body, caption));
        }
    }
    out
}

// ------------------------------------------------------------------------------------------------ SVG drawing
pub(crate) const X0: f64 = 96.0;
pub(crate) const X1: f64 = 504.0;
pub(crate) const Y0: f64 = 44.0;
pub(crate) const Y1: f64 = 278.0;

const STYLE: &str = ".axp-g{stroke:#8a879455;stroke-width:1}.axp-f{fill:none;stroke:#8a8794}\
.axp-t{fill:#55525f;font-family:ui-monospace,Menlo,Consolas,monospace;font-size:16px}\
.axp-l{fill:#4b4856;font-size:15px;font-weight:600}\
.axp-c{fill:none;stroke:#e0306f;stroke-width:2.6;stroke-linejoin:round;stroke-linecap:round}\
.axp-c2,.axp-c3,.axp-c4{fill:none;stroke-width:2.4;stroke-linejoin:round;stroke-linecap:round}\
.axp-c2{stroke:#18a874}.axp-c3{stroke:#7a5cf0}.axp-c4{stroke:#8a8794;stroke-width:1.8}\
.axp-m{fill:#18a874;stroke:#8a8794;stroke-width:1}\
.axp-mt{fill:#0d7a52;font-family:ui-monospace,Menlo,Consolas,monospace;font-size:15px;font-weight:600}\
.axp-d{fill:#e0306f}.axp-d2{fill:#18a874}.axp-d3{fill:#7a5cf0}.axp-d4{fill:#8a8794}\
.axp-b{fill:#e0306f99}.axp-b2{fill:#18a87499}.axp-b3{fill:#7a5cf099}.axp-b4{fill:#8a879473}\
.axp-s{fill:#e0306f40}.axp-s2{fill:#18a87440}.axp-s3{fill:#7a5cf040}.axp-s4{fill:#8a879440}\
.axp-bt{fill:#7f7c8a;font-size:13px;font-weight:600}.axp-lt{fill:#7f7c8a;font-size:13px}\
.axp-k{fill:none;stroke:#8a8794;stroke-width:2;stroke-linejoin:round;stroke-linecap:round}.axp-kd{fill:#8a8794}\
.axp-ka{fill:#8a879424}.axp-vt{fill:#0d7a52;font-family:ui-monospace,Menlo,Consolas,monospace;font-size:13px}\
.axp-hp{fill:#e0306f}.axp-hn{fill:#7a5cf0}.axp-ht{fill:#7f7c8a;font-family:ui-monospace,Menlo,Consolas,monospace}";

fn wrap_svg(id: &str, title: &str, caption: &str, body: &str) -> String {
    wrap_svg_h(id, title, caption, body, 340.0)
}

/// A figure's SVG, 520 wide and `h` tall.
pub(crate) fn wrap_svg_h(id: &str, title: &str, caption: &str, body: &str, h: f64) -> String {
    let h = h.round();
    format!("<svg class=\"axp\" viewBox=\"0 0 520 {h}\" role=\"img\" aria-labelledby=\"{id}-t\" aria-describedby=\"{id}-d\" \
             xmlns=\"http://www.w3.org/2000/svg\" font-family=\"system-ui, sans-serif\" font-size=\"12\" width=\"520\" \
             height=\"{h}\"><style>{STYLE}</style><title id=\"{id}-t\">{}</title><desc id=\"{id}-d\">{}</desc>{}</svg>",
            esc(title), esc(caption), body.replace("CLIPID", &format!("{id}-k")))
}

pub(crate) fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

pub(crate) fn clip(s: &str, n: usize) -> String {
    let c: Vec<char> = s.chars().collect();
    if c.len() <= n {
        return s.to_string();
    }
    // cut at a word break when one is near, never mid-word ("its nat..."; review 2026-11-12)
    let head: String = c[..n.saturating_sub(3)].iter().collect();
    let cut = match head.rfind(' ') {
        Some(i) if head[..i].chars().count() * 3 >= n * 2 => head[..i].trim_end_matches([',', ';', ':']).to_string(),
        _ => head.trim_end().to_string(),
    };
    format!("{cut}...")
}

fn cap1(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

/// The first letter in lower case when the first word is an ordinary word ("Symbol rate" -> "symbol rate"; "Eb/N0",
/// "pH", "T0" and acronyms stay as written).
fn lc1(s: &str) -> String {
    let word = s.split_whitespace().next().unwrap_or("");
    let mut w = word.chars();
    let ordinary = matches!(w.next(), Some(f) if f.is_uppercase()) && word.chars().count() > 1
        && w.all(|c| c.is_lowercase());
    if !ordinary {
        return s.to_string();
    }
    let mut c = s.chars();
    c.next().map(|f| f.to_lowercase().collect::<String>() + c.as_str()).unwrap_or_default()
}

pub(crate) fn minmax(v: &[f64]) -> (f64, f64) {
    v.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), x| (a.min(*x), b.max(*x)))
}

#[derive(Clone, Copy)]
pub(crate) struct Ax {
    lo: f64,
    hi: f64,
    log: bool,
    a: f64,
    b: f64,
}

impl Ax {
    pub(crate) fn new(lo: f64, hi: f64, log: bool, a: f64, b: f64, pad: bool) -> Ax {
        let (mut lo, mut hi) = (lo, hi);
        if log {
            if !(lo > 0.0) {
                lo = hi / 10.0;
            }
            if hi <= lo {
                hi = lo * 10.0;
            }
            if pad {
                lo /= 1.3;
                hi *= 1.3;
            }
        } else {
            if hi <= lo || !(hi - lo).is_finite() {
                let d = if lo == 0.0 { 1.0 } else { lo.abs() * 0.1 };
                lo -= d;
                hi += d;
            }
            if pad {
                let d = (hi - lo) * 0.06;
                lo -= d;
                hi += d;
            }
        }
        Ax { lo, hi, log, a, b }
    }
    pub(crate) fn px(&self, v: f64) -> f64 {
        let f = if self.log { (v.log10() - self.lo.log10()) / (self.hi.log10() - self.lo.log10()) } else {
            (v - self.lo) / (self.hi - self.lo)
        };
        self.a + f * (self.b - self.a)
    }
    fn ticks(&self) -> Vec<f64> {
        if self.log {
            let (e0, e1) = (self.lo.log10().ceil() as i32, self.hi.log10().floor() as i32);
            let n = (e1 - e0 + 1).max(1);
            let step = ((n as f64) / 6.0).ceil().max(1.0) as i32;
            let mut v: Vec<f64> = (e0..=e1).step_by(step as usize).map(|e| 10f64.powi(e)).collect();
            if v.is_empty() {
                v.push(self.lo * (self.hi / self.lo).sqrt());
            }
            return v;
        }
        nice_ticks(self.lo, self.hi, 5)
    }
}

fn nice_ticks(lo: f64, hi: f64, n: usize) -> Vec<f64> {
    let span = hi - lo;
    if !(span > 0.0) || !span.is_finite() {
        return vec![lo];
    }
    let raw = span / n as f64;
    let mag = 10f64.powf(raw.log10().floor());
    let norm = raw / mag;
    let step = mag * if norm < 1.5 { 1.0 } else if norm < 3.0 { 2.0 } else if norm < 7.0 { 5.0 } else { 10.0 };
    let mut v = vec![];
    let mut t = (lo / step).ceil() * step;
    while t <= hi + step * 1e-9 && v.len() < 12 {
        v.push(if t.abs() < step * 1e-9 { 0.0 } else { t });
        t += step;
    }
    v
}

fn tick_text(v: f64) -> String {
    tick_text_n(v, 4)
}

fn tick_text_n(v: f64, digits: usize) -> String {
    // a true minus sign: the hyphen of "-20" was too thin to see at tick size (chemistry review, 2026-11-12)
    let t = py_g(v, digits);
    match t.strip_prefix('-') {
        Some(rest) => format!("\u{2212}{rest}"),
        None => t,
    }
}

/// The tick labels of one axis, with as many significant digits (4 to 10) as it takes for neighbours to differ: a
/// nearly flat curve once had every tick read "24.94" (materials review, 2026-11-12).
fn tick_texts(ts: &[f64]) -> Vec<String> {
    for digits in 4..=10 {
        let v: Vec<String> = ts.iter().map(|t| tick_text_n(*t, digits)).collect();
        if v.windows(2).all(|w| w[0] != w[1]) {
            return v;
        }
    }
    ts.iter().map(|t| tick_text_n(*t, 10)).collect()
}

pub(crate) fn frame(out: &mut String, xa: Option<&Ax>, ya: Option<&Ax>, xlab: &str, ylab: &str, header: &str) {
    if let Some(xa) = xa {
        let ts = xa.ticks();
        for (t, txt) in ts.iter().copied().zip(tick_texts(&ts)) {
            let x = xa.px(t);
            out.push_str(&format!("<line class=\"axp-g\" x1=\"{x:.1}\" y1=\"{Y0}\" x2=\"{x:.1}\" y2=\"{Y1}\"/>\
                                   <text class=\"axp-t\" x=\"{x:.1}\" y=\"{}\" text-anchor=\"middle\">{}</text>",
                                  Y1 + 20.0, esc(&txt)));
        }
    }
    if let Some(ya) = ya {
        let ts = ya.ticks();
        for (t, txt) in ts.iter().copied().zip(tick_texts(&ts)) {
            let y = ya.px(t);
            out.push_str(&format!("<line class=\"axp-g\" x1=\"{X0}\" y1=\"{y:.1}\" x2=\"{X1}\" y2=\"{y:.1}\"/>\
                                   <text class=\"axp-t\" x=\"{}\" y=\"{:.1}\" text-anchor=\"end\">{}</text>",
                                  X0 - 6.0, y + 5.0, esc(&txt)));
        }
    }
    out.push_str(&format!("<rect class=\"axp-f\" x=\"{X0}\" y=\"{Y0}\" width=\"{}\" height=\"{}\"/>", X1 - X0, Y1 - Y0));
    if !xlab.is_empty() {
        out.push_str(&format!("<text class=\"axp-l\" x=\"{:.1}\" y=\"326\" text-anchor=\"middle\">{}</text>",
                              (X0 + X1) / 2.0, esc(&clip(xlab, 62))));
    }
    if !ylab.is_empty() {
        out.push_str(&format!("<text class=\"axp-l\" transform=\"translate(16,{:.1}) rotate(-90)\" text-anchor=\"middle\">{}</text>",
                              (Y0 + Y1) / 2.0, esc(&clip(ylab, 34))));
    }
    header_text(out, header);
}

fn header_text(out: &mut String, header: &str) {
    if !header.is_empty() {
        out.push_str(&format!("<text class=\"axp-mt\" x=\"{X0}\" y=\"20\">{}</text>", esc(header)));
    }
}

fn path_of(pts: &[Option<(f64, f64)>], xa: &Ax, ya: &Ax) -> String {
    let mut d = String::new();
    let mut pen = false;
    let mut prev: Option<(f64, f64)> = None;                  // the previous point: its value and its height
    for p in pts {
        match p {
            Some((x, y)) if (!ya.log || *y > 0.0) && (!xa.log || *x > 0.0) => {
                let (px, py) = (xa.px(*x), ya.px(*y));
                if !px.is_finite() || !py.is_finite() {
                    pen = false;
                    continue;
                }
                // a jump through a pole (a change of sign across most of the plot) is not joined by a line
                if let Some((y0, py0)) = prev {
                    if y0 * y < 0.0 && (py - py0).abs() > 0.6 * (Y1 - Y0) {
                        pen = false;
                    }
                }
                prev = Some((*y, py));
                d.push_str(&format!("{}{px:.1},{py:.1} ", if pen { "L" } else { "M" }));
                pen = true;
            }
            _ => pen = false,
        }
    }
    d.trim_end().to_string()
}

const LINE_CLASSES: [&str; 4] = ["axp-c", "axp-c2", "axp-c3", "axp-c4"];
const BAR_CLASSES: [&str; 4] = ["axp-b", "axp-b2", "axp-b3", "axp-b4"];
const BAND_CLASSES: [&str; 4] = ["axp-s", "axp-s2", "axp-s3", "axp-s4"];
const DOT_CLASSES: [&str; 4] = ["axp-d", "axp-d2", "axp-d3", "axp-d4"];

#[allow(clippy::too_many_arguments)]
fn line_chart(series: &[(String, Vec<Option<(f64, f64)>>)], bands: &[(f64, Option<String>)], xr: (f64, f64), xlab: &str,
              ylab: &str, marker: Option<(f64, f64)>, mlabel: &str, header: &str, int_x: bool) -> String {
    line_chart_x(series, bands, xr, xlab, ylab, marker, mlabel, header, int_x, false)
}

/// A line chart; `log_x`: a logarithmic horizontal axis (a sweep over orders of magnitude).
#[allow(clippy::too_many_arguments)]
pub(crate) fn line_chart_x(series: &[(String, Vec<Option<(f64, f64)>>)], bands: &[(f64, Option<String>)], xr: (f64, f64),
                xlab: &str, ylab: &str, marker: Option<(f64, f64)>, mlabel: &str, header: &str, int_x: bool,
                log_x: bool) -> String {
    line_chart_xy(series, bands, xr, xlab, ylab, marker, mlabel, header, int_x, log_x, false)
}

/// `line_chart_x`; `lin_y`: a linear vertical axis even over orders of magnitude (a curve rising from zero).
#[allow(clippy::too_many_arguments)]
pub(crate) fn line_chart_xy(series: &[(String, Vec<Option<(f64, f64)>>)], bands: &[(f64, Option<String>)],
                            xr: (f64, f64), xlab: &str, ylab: &str, marker: Option<(f64, f64)>, mlabel: &str,
                            header: &str, int_x: bool, log_x: bool, lin_y: bool) -> String {
    let ys: Vec<f64> = series.iter().flat_map(|s| s.1.iter().flatten().map(|p| p.1)).chain(marker.map(|m| m.1)).collect();
    let mut out = String::new();
    let log_x = log_x && xr.0 > 0.0 && xr.1 > xr.0;
    let xa = Ax::new(xr.0, xr.1, log_x, X0, X1, int_x || xr.0 == xr.1);
    let ya = if ys.is_empty() { None } else {
        let (mut lo, mut hi) = minmax(&ys);
        let log = !lin_y && lo > 0.0 && hi / lo > 1e3;
        if !log && ys.len() >= 10 {
            // a curve that shoots off near a pole: the axis follows the bulk of the values (and the marked one);
            // the rest is clipped at the frame
            let mut s = ys.clone();
            s.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let (q1, q9) = (s[s.len() / 10], s[s.len() * 9 / 10]);
            if q9 > q1 && hi - lo > 8.0 * (q9 - q1) {
                lo = lo.max(q1 - (q9 - q1));
                hi = hi.min(q9 + (q9 - q1));
                if let Some((_, my)) = marker {
                    lo = lo.min(my);
                    hi = hi.max(my);
                }
            }
        }
        Some(Ax::new(lo, hi, log, Y1, Y0, true))
    };
    // shaded verdicts first (behind the curves)
    if !bands.is_empty() {
        // one colour per answer in every chart of a capability: by the answer's sorted place (as the grid charts)
        let mut names: Vec<String> = bands.iter().filter_map(|b| b.1.clone()).collect();
        names.sort();
        names.dedup();
        let mut i = 0;
        while i < bands.len() {
            let Some(c) = bands[i].1.clone() else { i += 1; continue };
            let mut j = i;
            while j + 1 < bands.len() && bands[j + 1].1.as_ref() == Some(&c) {
                j += 1;
            }
            let k = names.iter().position(|n| n == &c).unwrap_or(0);
            let left = if i == 0 { bands[i].0 } else { (bands[i - 1].0 + bands[i].0) / 2.0 };
            let right = if j + 1 >= bands.len() { bands[j].0 } else { (bands[j].0 + bands[j + 1].0) / 2.0 };
            let (l, r) = (xa.px(left).max(X0), xa.px(right).min(X1));
            if r > l {
                out.push_str(&format!("<rect class=\"{}\" x=\"{l:.1}\" y=\"{Y0}\" width=\"{:.1}\" height=\"{}\"/>",
                                      BAND_CLASSES[k % 4], r - l, Y1 - Y0));
            }
            i = j + 1;
        }
        let mut y = Y1 - 8.0 - 16.0 * (names.len().min(4) as f64 - 1.0);
        for (k, n) in names.iter().enumerate().take(4) {
            out.push_str(&format!("<rect class=\"{}\" x=\"{}\" y=\"{:.1}\" width=\"12\" height=\"10\"/>\
                                   <text class=\"axp-lt\" x=\"{}\" y=\"{:.1}\">{}</text>",
                                  BAND_CLASSES[k % 4], X0 + 8.0, y - 9.0, X0 + 24.0, y, esc(&clip(n, 40))));
            y += 16.0;
        }
    }
    frame(&mut out, Some(&xa), ya.as_ref(), xlab, ylab, header);
    if let Some(ya) = ya {
        out.push_str(&format!("<clipPath id=\"CLIPID\"><rect x=\"{X0}\" y=\"{Y0}\" width=\"{}\" height=\"{}\"/></clipPath>\
                               <g clip-path=\"url(#CLIPID)\">", X1 - X0, Y1 - Y0));
        for (k, (_, pts)) in series.iter().enumerate() {
            let d = path_of(pts, &xa, &ya);
            if !d.is_empty() {
                out.push_str(&format!("<path class=\"{}\" d=\"{d}\"/>", LINE_CLASSES[k % 4]));
            }
            // isolated points (a curve broken by refusals) and whole-number sweeps get dots
            if int_x || pts.iter().flatten().count() <= 12 {
                for (x, y) in pts.iter().flatten().take(64) {
                    if ya.log && *y <= 0.0 {
                        continue;
                    }
                    out.push_str(&format!("<circle class=\"{}\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"3\"/>", DOT_CLASSES[k % 4],
                                          xa.px(*x), ya.px(*y)));
                }
            }
        }
        out.push_str("</g>");
        if series.len() > 1 {
            let names: Vec<(String, &str)> = series.iter().enumerate().map(|(k, s)| (s.0.clone(), BAR_CLASSES[k % 4])).collect();
            legend_box(&mut out, &names);
        }
        if let Some((mx, my)) = marker {
            if !(ya.log && my <= 0.0) {
                let (px, py) = (xa.px(mx), ya.px(my));
                let label = clip(mlabel, 40);
                // the label kept inside the plot: to the right of the point when it fits, else to its left, else
                // centred over it (about 9 px per character of the monospace label)
                let w = 9.0 * label.chars().count() as f64;
                let (tx, anchor) = if px + 10.0 + w <= X1 { (px + 10.0, "start") } else if px - 10.0 - w >= X0 {
                    (px - 10.0, "end")
                } else {
                    (((X0 + X1) / 2.0).clamp(X0 + w / 2.0, X1 - w / 2.0), "middle")
                };
                out.push_str(&format!("<circle class=\"axp-m\" cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"6\"/>\
                                       <text class=\"axp-mt\" x=\"{tx:.1}\" y=\"{:.1}\" text-anchor=\"{anchor}\">{}</text>",
                                      if py < Y0 + 30.0 { py + 22.0 } else { py - 10.0 }, esc(&label)));
            }
        }
    } else if let Some((mx, _)) = marker {
        let px = xa.px(mx);
        out.push_str(&format!("<line class=\"axp-k\" x1=\"{px:.1}\" y1=\"{Y0}\" x2=\"{px:.1}\" y2=\"{Y1}\"/>"));
    } else if !bands.is_empty() {
        // a verdict-only sweep: the visitor's value as a line
        let _ = mlabel;
    }
    out
}

fn bar_chart(labels: &[String], groups: &[(String, Vec<Option<f64>>)], highlight: Option<usize>, xlab: &str, ylab: &str,
             header: &str) -> String {
    let hl: Vec<usize> = highlight.into_iter().collect();
    bar_chart_hl(labels, groups, &hl, xlab, ylab, header)
}

/// Bars, one group per series; with one series, the bars at `highlight` drawn in the accent colour, the rest grey.
pub(crate) fn bar_chart_hl(labels: &[String], groups: &[(String, Vec<Option<f64>>)], highlight: &[usize], xlab: &str, ylab: &str,
                header: &str) -> String {
    let vals: Vec<f64> = groups.iter().flat_map(|g| g.1.iter().flatten().copied()).collect();
    let mut out = String::new();
    if vals.is_empty() || labels.is_empty() {
        return out;
    }
    let (lo, hi) = minmax(&vals);
    let log = lo > 0.0 && hi / lo > 1e4;
    let ya = if log { Ax::new(lo, hi, true, Y1, Y0, true) } else { Ax::new(lo.min(0.0), hi.max(0.0), false, Y1, Y0, true) };
    frame(&mut out, None, Some(&ya), xlab, ylab, header);
    let n = labels.len();
    let slot = (X1 - X0) / n as f64;
    let ng = groups.len().max(1);
    let bw = (slot * 0.78 / ng as f64).min(60.0);
    let base = if log { Y1 } else { ya.px(0.0) };
    let every = (n as f64 / 14.0).ceil().max(1.0) as usize;
    for (i, l) in labels.iter().enumerate() {
        let cx = X0 + slot * (i as f64 + 0.5);
        for (g, (_, vs)) in groups.iter().enumerate() {
            let Some(v) = vs.get(i).copied().flatten() else { continue };
            if log && v <= 0.0 {
                continue;
            }
            let x = cx - bw * ng as f64 / 2.0 + bw * g as f64;
            let y = ya.px(v);
            let (top, h) = if y < base { (y, base - y) } else { (base, y - base) };
            let class = if ng == 1 && !highlight.is_empty() {
                if highlight.contains(&i) { "axp-b" } else { "axp-b4" }
            } else {
                BAR_CLASSES[g % 4]
            };
            out.push_str(&format!("<rect class=\"{class}\" x=\"{x:.1}\" y=\"{top:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>",
                                  (bw - 1.0).max(1.0), h.max(0.5)));
            if n * ng <= 16 {
                let ty = if v >= 0.0 || log { top - 4.0 } else { top + h + 13.0 };
                out.push_str(&format!("<text class=\"axp-bt\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
                                      x + bw / 2.0, ty.max(Y0 + 11.0), esc(&py_g(v, 4))));
            }
        }
        if i % every == 0 {
            out.push_str(&format!("<text class=\"axp-t\" x=\"{cx:.1}\" y=\"{}\" text-anchor=\"middle\">{}</text>", Y1 + 20.0,
                                  esc(&clip(l, if n > 8 { 6 } else { 12 }))));
        }
    }
    if ng > 1 {
        let names: Vec<(String, &str)> = groups.iter().enumerate().map(|(k, g)| (g.0.clone(), BAR_CLASSES[k % 4])).collect();
        legend_box(&mut out, &names);
    }
    out
}

fn stem_chart(series: &[(String, Vec<Option<(f64, f64)>>)], xr: (f64, f64), xlab: &str, ylab: &str, header: &str) -> String {
    let ys: Vec<f64> = series.iter().flat_map(|s| s.1.iter().flatten().map(|p| p.1)).collect();
    let (lo, hi) = minmax(&ys);
    let xa = Ax::new(xr.0, xr.1, false, X0, X1, true);
    let ya = Ax::new(lo.min(0.0), hi.max(0.0), false, Y1, Y0, true);
    let mut out = String::new();
    frame(&mut out, Some(&xa), Some(&ya), xlab, ylab, header);
    let base = ya.px(0.0);
    for (k, (_, pts)) in series.iter().enumerate() {
        let d = path_of(pts, &xa, &ya);
        out.push_str(&format!("<path class=\"{}\" d=\"{d}\"/>", LINE_CLASSES[k % 4]));
        for (x, y) in pts.iter().flatten().take(200) {
            let (px, py) = (xa.px(*x), ya.px(*y));
            if pts.len() <= 40 {
                out.push_str(&format!("<line class=\"axp-g\" x1=\"{px:.1}\" y1=\"{base:.1}\" x2=\"{px:.1}\" y2=\"{py:.1}\"/>"));
            }
            out.push_str(&format!("<circle class=\"{}\" cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"3.5\"/>", DOT_CLASSES[k % 4]));
        }
    }
    if series.len() > 1 {
        let names: Vec<(String, &str)> = series.iter().enumerate().map(|(k, s)| (s.0.clone(), BAR_CLASSES[k % 4])).collect();
        legend_box(&mut out, &names);
    }
    out
}

fn heat_chart(m: &[Vec<f64>], header: &str) -> String {
    let rows: Vec<String> = (1..=m.len()).map(|i| i.to_string()).collect();
    let cols: Vec<String> = (1..=m[0].len()).map(|i| i.to_string()).collect();
    heat_labeled(m, &rows, &cols, header)
}

pub(crate) fn heat_labeled(m: &[Vec<f64>], rows: &[String], cols: &[String], header: &str) -> String {
    let mut out = String::new();
    header_text(&mut out, header);
    let (r, c) = (m.len(), m[0].len());
    let left = 110.0;
    let (w, h) = (X1 - left, Y1 + 20.0 - Y0);
    let cell = (w / c as f64).min(h / r as f64).min(70.0);
    let x0 = left + (w - cell * c as f64) / 2.0;
    let y0 = Y0 + 8.0;
    let mx = m.iter().flatten().filter(|x| x.is_finite()).fold(0.0f64, |a, x| a.max(x.abs()));
    let font = (cell * 0.32).clamp(8.0, 14.0);
    for (i, row) in m.iter().enumerate() {
        for (j, v) in row.iter().enumerate() {
            let (x, y) = (x0 + cell * j as f64, y0 + cell * i as f64);
            if !v.is_finite() {
                out.push_str(&format!("<rect class=\"axp-f\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{cell:.1}\" height=\"{cell:.1}\"/>"));
                continue;
            }
            let op = if mx > 0.0 { 0.12 + 0.78 * v.abs() / mx } else { 0.12 };
            out.push_str(&format!("<rect class=\"{}\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
                                   fill-opacity=\"{op:.2}\"/>", if *v < 0.0 { "axp-hn" } else { "axp-hp" }, cell - 1.0, cell - 1.0));
            if r * c <= 144 {
                out.push_str(&format!("<text class=\"axp-ht\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"{font:.0}\">{}</text>",
                                      x + cell / 2.0, y + cell / 2.0 + font * 0.35, esc(&py_g(*v, 3))));
            }
        }
    }
    let lf = 12.0f64.min(cell * 0.6).max(8.0);
    let every_r = (r as f64 / 16.0).ceil().max(1.0) as usize;
    let every_c = (c as f64 / 16.0).ceil().max(1.0) as usize;
    for (i, l) in rows.iter().enumerate().step_by(every_r) {
        out.push_str(&format!("<text class=\"axp-lt\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\" font-size=\"{lf:.0}\">{}</text>",
                              x0 - 5.0, y0 + cell * (i as f64 + 0.5) + 4.0, esc(&clip(l, 10))));
    }
    for (j, l) in cols.iter().enumerate().step_by(every_c) {
        out.push_str(&format!("<text class=\"axp-lt\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"{lf:.0}\">{}</text>",
                              x0 + cell * (j as f64 + 0.5), y0 + cell * r as f64 + 15.0,
                              esc(&tail(l, ((cell * every_c as f64 / 7.5) as usize).clamp(2, 8)))));
    }
    out
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn grid_chart(xs: &[f64], ys: &[f64], cells: &[(usize, usize, String)], names: &[String], mine: (f64, f64), xlab: &str,
              ylab: &str, header: &str) -> String {
    let xa = Ax::new(xs[0], xs[xs.len() - 1], false, X0, X1, true);
    let ya = Ax::new(ys[0], ys[ys.len() - 1], false, Y1, Y0, true);
    let mut out = String::new();
    let half = |v: &[f64], i: usize| -> (f64, f64) {
        let l = if i == 0 { v[0] - (v.get(1).unwrap_or(&v[0]) - v[0]) / 2.0 } else { (v[i - 1] + v[i]) / 2.0 };
        let r = if i + 1 >= v.len() { v[i] + (v[i] - v[i.saturating_sub(1)]) / 2.0 } else { (v[i] + v[i + 1]) / 2.0 };
        (l, r)
    };
    for (i, j, c) in cells {
        let k = names.iter().position(|n| n == c).unwrap_or(0);
        let (l, r) = half(xs, *i);
        let (bt, tp) = half(ys, *j);
        let (x1, x2) = (xa.px(l).max(X0), xa.px(r).min(X1));
        let (y1, y2) = (ya.px(tp).max(Y0), ya.px(bt).min(Y1));
        if x2 > x1 && y2 > y1 {
            out.push_str(&format!("<rect class=\"{}\" x=\"{x1:.1}\" y=\"{y1:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>",
                                  BAR_CLASSES[k % 4], x2 - x1, y2 - y1));
        }
    }
    frame(&mut out, Some(&xa), Some(&ya), xlab, ylab, header);
    let (px, py) = (xa.px(mine.0), ya.px(mine.1));
    out.push_str(&format!("<circle class=\"axp-k\" cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"8\"/><circle class=\"axp-m\" cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"4\"/>"));
    let names: Vec<(String, &str)> = names.iter().enumerate().map(|(k, n)| (n.clone(), BAR_CLASSES[k % 4])).collect();
    legend_box(&mut out, &names);
    out
}

/// A legend in its own box above the plot, right-aligned (for figures that fill the plot area).
fn legend_box(out: &mut String, names: &[(String, &str)]) {
    let mut x = X1;
    for (n, class) in names.iter().filter(|n| !n.0.is_empty()).rev().take(4) {
        let t = clip(n, 22);
        let w = 7.0 * t.chars().count() as f64 + 22.0;
        x -= w;
        out.push_str(&format!("<rect class=\"{class}\" x=\"{x:.1}\" y=\"28\" width=\"12\" height=\"10\"/>\
                               <text class=\"axp-lt\" x=\"{:.1}\" y=\"37\">{}</text>", x + 16.0, esc(&t)));
    }
}

pub(crate) fn equal_axes(xs: &[f64], ys: &[f64]) -> (Ax, Ax) {
    let (mut xl, mut xh) = minmax(xs);
    let (mut yl, mut yh) = minmax(ys);
    let span = (xh - xl).max(yh - yl).max(1e-300);
    let (cxm, cym) = ((xl + xh) / 2.0, (yl + yh) / 2.0);
    // the plot box is wider than tall: equal scales on both axes
    let (w, h) = (X1 - X0, Y1 - Y0);
    let half_y = span * 0.58;
    let half_x = half_y * w / h;
    xl = cxm - half_x;
    xh = cxm + half_x;
    yl = cym - half_y;
    yh = cym + half_y;
    (Ax::new(xl, xh, false, X0, X1, false), Ax::new(yl, yh, false, Y1, Y0, false))
}

fn points_chart(p: &[(f64, f64)], labels: Option<&(String, Vec<String>)>, centres: &[(String, Vec<(f64, f64)>)],
                query: &[(String, (f64, f64))], line: Option<(f64, f64, f64)>, header: &str) -> String {
    let mut xs: Vec<f64> = p.iter().map(|q| q.0).collect();
    let mut ys: Vec<f64> = p.iter().map(|q| q.1).collect();
    for (_, c) in centres {
        xs.extend(c.iter().map(|q| q.0));
        ys.extend(c.iter().map(|q| q.1));
    }
    for (_, q) in query {
        xs.push(q.0);
        ys.push(q.1);
    }
    let (xa, ya) = equal_axes(&xs, &ys);
    let mut out = String::new();
    frame(&mut out, Some(&xa), Some(&ya), "First column", "Second column", header);
    out.push_str(&format!("<clipPath id=\"CLIPID\"><rect x=\"{X0}\" y=\"{Y0}\" width=\"{}\" height=\"{}\"/></clipPath>\
                           <g clip-path=\"url(#CLIPID)\">", X1 - X0, Y1 - Y0));
    if let Some((w0, w1, bb)) = line {
        // w0 x + w1 y + b = 0 across the plot
        let mut seg = vec![];
        if w1.abs() > 1e-300 {
            for x in [xa.lo, xa.hi] {
                seg.push((x, -(w0 * x + bb) / w1));
            }
        } else if w0.abs() > 1e-300 {
            for y in [ya.lo, ya.hi] {
                seg.push((-(bb) / w0, y));
            }
        }
        if seg.len() == 2 {
            out.push_str(&format!("<line class=\"axp-c4\" stroke-dasharray=\"6 4\" x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\"/>",
                                  xa.px(seg[0].0), ya.px(seg[0].1), xa.px(seg[1].0), ya.px(seg[1].1)));
        }
    }
    let mut names: Vec<String> = vec![];
    for (i, q) in p.iter().enumerate() {
        let class = match labels {
            Some((_, l)) => {
                let lab = l.get(i).cloned().unwrap_or_default();
                let k = match names.iter().position(|n| n == &lab) {
                    Some(k) => k,
                    None => {
                        names.push(lab);
                        names.len() - 1
                    }
                };
                DOT_CLASSES[k % 4]
            }
            None => "axp-d",
        };
        out.push_str(&format!("<circle class=\"{class}\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"5\"/>", xa.px(q.0), ya.px(q.1)));
    }
    for (_, c) in centres {
        for q in c.iter().take(40) {
            out.push_str(&format!("<rect class=\"axp-k\" x=\"{:.1}\" y=\"{:.1}\" width=\"12\" height=\"12\"/>",
                                  xa.px(q.0) - 6.0, ya.px(q.1) - 6.0));
        }
    }
    for (_, q) in query {
        let (x, y) = (xa.px(q.0), ya.px(q.1));
        out.push_str(&format!("<path class=\"axp-m\" d=\"M{x:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} Z\"/>",
                              y - 9.0, x + 8.0, y + 6.0, x - 8.0, y - 3.0, x + 8.0, y - 3.0, x - 8.0, y + 6.0));
    }
    out.push_str("</g>");
    if let Some((s, _)) = labels {
        if names.len() <= 4 {
            let n: Vec<(String, &str)> = names.iter().enumerate().map(|(k, n)| (format!("{s} {n}"), DOT_CLASSES[k % 4])).collect();
            legend_box(&mut out, &n);
        }
    }
    out
}

pub(crate) fn arrows_chart(v: &[(String, (f64, f64), bool)], header: &str) -> String {
    let mut xs: Vec<f64> = v.iter().map(|q| q.1 .0).collect();
    let mut ys: Vec<f64> = v.iter().map(|q| q.1 .1).collect();
    xs.push(0.0);
    ys.push(0.0);
    let (xa, ya) = equal_axes(&xs, &ys);
    let mut out = String::new();
    frame(&mut out, Some(&xa), Some(&ya), "First entry", "Second entry", header);
    let (ox, oy) = (xa.px(0.0), ya.px(0.0));
    for (k, (s, (x, y), ans)) in v.iter().enumerate() {
        let (px, py) = (xa.px(*x), ya.px(*y));
        let class = if *ans { LINE_CLASSES[k % 3] } else { "axp-c4" };
        let dot = if *ans { DOT_CLASSES[k % 3] } else { "axp-d4" };
        // the label right of the tip, or left of it when it would leave the figure
        let label = format!("{s} ({}, {})", py_g(*x, 4), py_g(*y, 4));
        let w = 7.6 * label.chars().count() as f64;
        let (tx, anchor) = if px + 7.0 + w <= 516.0 { (px + 7.0, "start") } else { ((px - 7.0).max(w + 4.0), "end") };
        out.push_str(&format!("<line class=\"{class}\" x1=\"{ox:.1}\" y1=\"{oy:.1}\" x2=\"{px:.1}\" y2=\"{py:.1}\"/>\
                               <circle class=\"{dot}\" cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"4.5\"/>\
                               <text class=\"axp-bt\" x=\"{tx:.1}\" y=\"{:.1}\" text-anchor=\"{anchor}\">{}</text>",
                              (py - 7.0).max(16.0), esc(&label)));
    }
    out
}

pub(crate) fn complex_chart(paths: &[(String, Vec<(f64, f64)>)], mine: &[(String, (f64, f64))], unit_circle: bool, header: &str) -> String {
    let mut xs: Vec<f64> = paths.iter().flat_map(|p| p.1.iter().map(|z| z.0)).chain(mine.iter().map(|m| m.1 .0)).collect();
    let mut ys: Vec<f64> = paths.iter().flat_map(|p| p.1.iter().map(|z| z.1)).chain(mine.iter().map(|m| m.1 .1)).collect();
    xs.push(0.0);
    ys.push(0.0);
    if unit_circle {
        xs.extend([-1.0, 1.0]);
        ys.extend([-1.0, 1.0]);
    }
    let (xa, ya) = equal_axes(&xs, &ys);
    let mut out = String::new();
    frame(&mut out, Some(&xa), Some(&ya), "Real part", "Imaginary part", header);
    out.push_str(&format!("<clipPath id=\"CLIPID\"><rect x=\"{X0}\" y=\"{Y0}\" width=\"{}\" height=\"{}\"/></clipPath>\
                           <g clip-path=\"url(#CLIPID)\">", X1 - X0, Y1 - Y0));
    if unit_circle {
        let r = (xa.px(1.0) - xa.px(0.0)).abs();
        out.push_str(&format!("<circle class=\"axp-c4\" stroke-dasharray=\"5 4\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"{r:.1}\"/>",
                              xa.px(0.0), ya.px(0.0)));
    }
    for (k, (_, p)) in paths.iter().enumerate() {
        let pts: Vec<Option<(f64, f64)>> = p.iter().map(|z| Some(*z)).collect();
        let d = path_of(&pts, &xa, &ya);
        if !d.is_empty() {
            out.push_str(&format!("<path class=\"{}\" d=\"{d}\"/>", LINE_CLASSES[k % 4]));
        }
    }
    out.push_str("</g>");
    for (k, (s, (re, im))) in mine.iter().enumerate() {
        let (px, py) = (xa.px(*re), ya.px(*im));
        out.push_str(&format!("<circle class=\"axp-m\" cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"6\"/>\
                               <text class=\"axp-mt\" x=\"{:.1}\" y=\"{:.1}\">{}</text>",
                              px + 9.0, py - 8.0 + 16.0 * k as f64, esc(&format!("{s} = {}", cx_text(*re, *im)))));
    }
    out
}

fn graph_chart(nodes: &[String], edges: &[(String, String, Option<f64>)], pos: Option<&[(f64, f64)]>, marked: &[String],
               ringed: &[String], per_node: Option<&(String, Vec<String>)>, header: &str) -> String {
    let n = nodes.len();
    let xy: Vec<(f64, f64)> = match pos {
        Some(p) => {
            let xs: Vec<f64> = p.iter().map(|q| q.0).collect();
            let ys: Vec<f64> = p.iter().map(|q| q.1).collect();
            let (xa, ya) = equal_axes(&xs, &ys);
            p.iter().map(|q| (xa.px(q.0), ya.px(q.1))).collect()
        }
        None => {
            let (cxp, cyp, r) = ((X0 + X1) / 2.0, (Y0 + Y1) / 2.0 + 4.0, (Y1 - Y0) / 2.0 - 8.0);
            (0..n).map(|i| {
                let a = -std::f64::consts::FRAC_PI_2 + 2.0 * std::f64::consts::PI * i as f64 / n as f64;
                (cxp + r * 1.25 * a.cos(), cyp + r * a.sin())
            }).collect()
        }
    };
    let at = |s: &str| nodes.iter().position(|x| x == s).map(|i| xy[i]).unwrap_or((0.0, 0.0));
    let mut out = String::new();
    header_text(&mut out, header);
    for (u, v, w) in edges {
        let (a, b) = (at(u), at(v));
        out.push_str(&format!("<line class=\"axp-k\" x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\"/>", a.0, a.1, b.0, b.1));
        if let Some(w) = w {
            if edges.len() <= 30 {
                out.push_str(&format!("<text class=\"axp-vt\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
                                      (a.0 + b.0) / 2.0 + 4.0, (a.1 + b.1) / 2.0 - 4.0, esc(&py_g(*w, 4))));
            }
        }
    }
    let r = if n > 16 { 9.0 } else { 13.0 };
    for (i, s) in nodes.iter().enumerate() {
        let (x, y) = xy[i];
        let class = if marked.contains(s) { "axp-d" } else { "axp-d4" };
        if ringed.contains(s) {
            out.push_str(&format!("<circle class=\"axp-k\" cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"{:.1}\"/>", r + 5.0));
        }
        out.push_str(&format!("<circle class=\"{class}\" cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"{r}\"/>\
                               <text class=\"axp-bt\" x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
                              y + 4.5, esc(&clip(s, 4))));
        if let Some((_, vals)) = per_node {
            if let Some(v) = vals.get(i) {
                out.push_str(&format!("<text class=\"axp-vt\" x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
                                      y + r + 15.0, esc(v)));
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chart_axes_read_in_everyday_units() {
        // the meaning names the unit
        let (l, u) = reading_unit("Pa", "arterial partial pressure of CO2 (e.g. in mmHg)", 5333.0).unwrap();
        assert_eq!(l, "mmHg");
        assert!(((5333.0 - u.offset) / u.factor - 40.0).abs() < 0.1);
        // by size: a cardiac output of 8.3e-5 m^3/s reads as 5 L/min
        let (l, u) = reading_unit("m^3/s", "cardiac output", 8.333e-5).unwrap();
        assert!(l == "L/s" || l == "L/min", "{l}");
        assert!(((8.333e-5 - u.offset) / u.factor) >= 0.1);
        // a readable SI number stays SI; a pure number never changes
        assert!(reading_unit("Pa", "pressure", 250.0).is_none());
        assert!(reading_unit("1", "ratio", 1e-6).is_none());
        // a word after "in" that is no unit of the same kind is ignored
        assert!(reading_unit("m", "length in the tube", 3.0).is_none());
    }

    #[test]
    fn leading_numbers_and_units() {
        assert_eq!(lead_num("3.5 km"), Some((3.5, "km".into())));
        assert_eq!(lead_num("-2e-3 m/s"), Some((-0.002, "m/s".into())));
        assert_eq!(lead_num("7"), Some((7.0, String::new())));
        assert_eq!(lead_num("1/2"), None);
        assert_eq!(lead_num("abc"), None);
        assert_eq!(lead_num("−4 K"), Some((-4.0, "K".into())));
    }

    #[test]
    fn ticks_are_round_and_bounded() {
        let t = nice_ticks(0.0, 1.0, 5);
        assert_eq!(t, vec![0.0, 0.2, 0.4, 0.6000000000000001, 0.8, 1.0]);
        assert!(nice_ticks(-3.7, 12.2, 5).len() <= 12);
        let a = Ax::new(1e-6, 1.0, true, Y1, Y0, false);
        assert!(a.ticks().len() <= 7 && a.ticks().iter().all(|x| *x > 0.0));
        assert_eq!(tick_text(0.6000000000000001), "0.6");
        // neighbours never read the same (a nearly flat curve once read 24.94 at every tick)
        let t = tick_texts(&[24.9401, 24.9402, 24.9403]);
        assert!(t.windows(2).all(|w| w[0] != w[1]), "{t:?}");
    }

    #[test]
    fn svg_text_is_escaped_and_ids_are_unique() {
        let s = wrap_svg("axr-x-1", "a < b & c", "d \"e\"", "<g clip-path=\"url(#CLIPID)\"></g>");
        assert!(s.contains("a &lt; b &amp; c") && s.contains("d &quot;e&quot;") && s.contains("url(#axr-x-1-k)"));
        assert!(s.starts_with("<svg class=\"axp\"") && s.ends_with("</svg>"));
    }

    #[test]
    fn a_line_chart_marks_the_value() {
        let pts: Vec<Option<(f64, f64)>> = (0..10).map(|i| Some((i as f64, (i * i) as f64))).collect();
        let body = line_chart(&[("y".into(), pts)], &[], (0.0, 9.0), "x (m)", "y (s)", Some((3.0, 9.0)), "your x = 3",
                              "y = 9 s", false);
        assert!(body.contains("axp-m") && body.contains("your x = 3") && body.contains(">y = 9 s<"));
        assert!(body.contains("<path class=\"axp-c\""));
    }

    #[test]
    fn a_sweep_steps_whole_numbers_and_keeps_the_value() {
        let k = Knob { lo_lim: Some(1.0), ..Knob::plain("n", 7.0, true, "n".into(), String::new()) };
        let xs = knob_points(&k, 41);
        assert!(xs.contains(&7.0) && xs.iter().all(|x| x.fract() == 0.0 && *x >= 1.0) && xs.len() <= 42);
        let k = Knob { int: false, x0: 2.5, lo_lim: None, ..k };
        let xs = knob_points(&k, 41);
        assert!(xs.contains(&2.5) && xs.len() <= 42 && xs[0] > 0.0);
        let k = Knob { step: Some(0.5), ..k };                       // a half-integer lattice
        let xs = knob_points(&k, 41);
        assert!(xs.contains(&2.5) && xs.contains(&3.0) && xs.iter().all(|x| (x * 2.0).fract() == 0.0) && xs.len() <= 17);
        let k = Knob::plain("n", 268992.0, true, "n".into(), String::new());   // consecutive numbers around a large one
        let xs = knob_points(&k, 41);
        assert!(xs.contains(&268993.0) && xs.iter().all(|x| (x - 268992.0).abs() <= 20.0) && xs.len() <= 42);
    }

    #[test]
    fn apply_rewrites_one_input() {
        let g = vec![("v".to_string(), Input::Text("1, 2, 3".into())), ("t".to_string(), Input::Text("4 s".into()))];
        let k = Knob { rest: "s".into(), ..Knob::plain("t", 4.0, false, "t".into(), "s".into()) };
        let g2 = apply(&g, &k, 5.5);
        assert!(matches!(&g2[1].1, Input::Text(t) if t == "5.5 s"));
        let k = Knob { sym: "v".into(), idx: Some(1), x0: 2.0, base: vec![1.0, 2.0, 3.0], rest: String::new(), ..k };
        let g3 = apply(&g, &k, 9.0);
        assert!(matches!(&g3[0].1, Input::Text(t) if t == "1.0, 9.0, 3.0"));
        // a whole number inside a text
        let g = vec![("f".to_string(), Input::Text("2^n/n^3".into()))];
        let (s, e, n) = text_numbers("2^n/n^3")[0];
        let k = Knob { text: Some(("2^n/n^3".into(), s, e)), ..Knob::plain("f", n as f64, true, "f".into(), String::new()) };
        assert!(matches!(&apply(&g, &k, 5.0)[0].1, Input::Text(t) if t == "5^n/n^3"));
        assert_eq!(text_numbers("x = 1.5, H2O"), vec![(10, 11, 2)]);
    }

    #[test]
    fn truth_assignments_are_read_and_rewritten_in_the_visitors_spelling() {
        let t = "A=1, B=0; c : True";
        let s = assignment_spans(t).unwrap();
        assert_eq!(s.iter().map(|x| x.0.as_str()).collect::<Vec<_>>(), vec!["A", "B", "c"]);
        assert_eq!(with_truths(t, &s, &[false, true, false]), "A=0, B=1; c : False");
        assert!(assignment_spans("A=2, B=0").is_none() && assignment_spans("x = 3 m").is_none());
        assert!(assignment_spans("a=1,b=1,c=1,d=1,e=1,f=1,g=1").is_none());       // more than MAX_TT_VARS
        assert_eq!(truth_word("T", true), "T");
        assert_eq!(truth_word("TRUE", false), "FALSE");
    }

    #[test]
    fn formulas_are_told_from_arithmetic_and_chemistry() {
        let (v, occ) = formula_vars("(p -> q) & ~r | x1 <-> p").unwrap();
        assert_eq!(v, vec!["p", "q", "r", "x1"]);
        assert_eq!(occ.len(), 5);
        let val = |n: &str| n == "q";
        assert_eq!(substituted("(p -> q) & ~p", &formula_vars("(p -> q) & ~p").unwrap().1, &val, ("1", "0")),
                   "(0 -> 1) & ~0");
        assert!(formula_vars("p and not q").is_some());
        for t in ["x^2 + 1", "H2SO4", "2*pi", "a - b", "hello world", "p", "3 kg"] {
            assert!(formula_vars(t).is_none(), "{t}");
        }
    }

    #[test]
    fn genotypes_are_letter_pairs_of_one_gene_each() {
        assert_eq!(genotype_pairs("AaBB"), Some(vec![('A', 'a'), ('B', 'B')]));
        assert!(genotype_pairs("AB").is_none() && genotype_pairs("Aab").is_none() && genotype_pairs("A1").is_none());
        assert_eq!(gene_variants(('a', 'A')), ["AA".to_string(), "aA".into(), "aa".into()]);
        assert_eq!(with_gene(&[('A', 'a'), ('B', 'b')], 1, "bb"), "Aabb");
    }

    #[test]
    fn coarse_to_fine_visits_every_index_once() {
        for n in [1usize, 2, 3, 9, 41] {
            let mut o = coarse_to_fine(n);
            assert_eq!(o.len(), n);
            o.sort();
            assert_eq!(o, (0..n).collect::<Vec<_>>());
        }
        assert_eq!(&coarse_to_fine(9)[..3], &[0, 8, 4]);
    }

    #[test]
    fn axis_labels_carry_one_unit() {
        // the meaning's own unit note goes when the axis prints the unit (once "(1/s) (1/h)")
        assert_eq!(drop_unit_note("Coriolis parameter (1/s)", "1/s"), "Coriolis parameter");
        assert_eq!(drop_unit_note("discharge (m^3/s)", "m^3/s"), "discharge");
        assert_eq!(drop_unit_note("formation energy (per vacancy, e.g. eV)", "J"), "formation energy");
        // a note that is no unit stays
        assert_eq!(drop_unit_note("inclination (positive downward)", "rad"), "inclination (positive downward)");
        // a unit named in the meaning's parentheses is the reading unit
        let (l, _) = reading_unit("s", "rupture time (hours, h)", 3.6e6).unwrap();
        assert_eq!(l, "h");
        let (l, _) = reading_unit("J", "vacancy formation energy (per vacancy, e.g. eV)", 1.442e-19).unwrap();
        assert_eq!(l, "eV");
    }

    #[test]
    fn a_jump_is_drawn_as_a_gap_and_a_steep_smooth_curve_is_not() {
        // a step function: 5 up to x = 2, then 3
        let step: Vec<Option<(f64, f64)>> = (0..10).map(|i| { let x = i as f64 * 0.5; Some((x, if x <= 2.0 { 5.0 + 0.01 * x } else { 3.0 + 0.01 * x })) }).collect();
        let out = break_jumps(step);
        assert_eq!(out.iter().filter(|p| p.is_none()).count(), 1);
        // a parabola: steep at the end, but every step is close to its neighbours
        let par: Vec<Option<(f64, f64)>> = (0..20).map(|i| { let x = i as f64; Some((x, x * x)) }).collect();
        assert_eq!(break_jumps(par).iter().filter(|p| p.is_none()).count(), 0);
    }

    #[test]
    fn a_list_entry_is_named_by_the_meaning_when_it_names_its_entries() {
        assert_eq!(entry_label("A, b, c of the line a x + b y = c", 2, 3), "c of the line a x + b y = c");
        assert_eq!(entry_label("A, b, c of the line a x + b y = c", 0, 3), "a of the line a x + b y = c");
        assert_eq!(entry_label("A, B, C of A x^2 + B xy + C y^2", 1, 3), "B of A x^2 + B xy + C y^2");
        assert_eq!(entry_label("measured heights of the plants", 1, 4), "measured heights of the plants, entry 2");
        assert_eq!(entry_label("A, b, c of the line", 1, 4), "A, b, c of the line, entry 2");
    }

    #[test]
    fn sweeps_stay_in_the_range_a_kind_admits() {
        assert_eq!(kind_range("emissivity", "emissivity of the second plate", ""), (Some(0.0), Some(1.0)));
        assert_eq!(kind_range("heat_capacity_ratio", "ratio of specific heats", ""), (Some(1.0), Some(5.0 / 3.0)));
        assert_eq!(kind_range("cutoff_ratio", "cutoff ratio V3/V2", ""), (Some(1.0), None));
        assert_eq!(kind_range("mach_number", "Mach number ahead of the shock", ""), (Some(1.0), None));
        assert_eq!(kind_range("mach_number", "Mach number behind the shock", ""), (None, None));
        assert_eq!(kind_range("number", "longitude of the first point (degrees)", ""), (Some(-180.0), Some(180.0)));
        assert_eq!(kind_range("number", "latitude", ""), (Some(-90.0), Some(90.0)));
        // a limit the visitor's own value lies outside is not applied (an efficiency typed as 85 per cent)
        let mut k = Knob::plain("eta", 85.0, false, "efficiency".into(), String::new());
        k.lo_lim = Some(0.0);
        k.hi_lim = Some(1.0);
        let (lo, hi) = knob_range(&k);
        assert!(lo < 85.0 && hi > 85.0);
        let mut k = Knob::plain("e2", 0.8, false, "emissivity".into(), String::new());
        k.lo_lim = Some(0.0);
        k.hi_lim = Some(1.0);
        assert!(knob_range(&k).1 <= 1.0);
        assert!(pure_unit("1") && pure_unit("") && !pure_unit("Pa"));
    }
}
