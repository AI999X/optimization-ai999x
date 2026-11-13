//! A model's data as tools/export_rust_pack.py writes it (rust/models/<model>/pack.json).

use crate::expr::Node;
use crate::units::Dim;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Kind {
    pub name: String,
    pub unit: String,
    pub dim: Dim,
    pub difference: bool,
}

#[derive(Clone, Debug)]
pub struct Default {
    pub value: f64,
    pub unit: String,
    pub assumption: Option<String>,
    /// context tags under which the default does not hold (a planet other than the Earth for g)
    pub unless: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Sym {
    pub name: String,
    pub kind: String,
    pub role: String,
    pub sign: String,
    pub desc: String,
    pub default: Option<Default>,
}

#[derive(Clone, Debug)]
pub struct Form {
    pub form: Node,
    /// the same value written to avoid an exponential overflow (used only when `form` is not finite)
    pub alt: Option<Node>,
    /// the same root written without a cancellation (rationalized: a quadratic's small root; with expm1: 1 - exp(-x)
    /// for a tiny x): the engine keeps whichever has the smallest error bound
    pub variants: Vec<Node>,
    pub terms: Vec<Node>,
    pub bounded_terms: Vec<Node>,
}

#[derive(Clone, Debug)]
pub struct Relation {
    pub key: String,
    pub name: String,
    pub equation: String,
    pub lhs: String,
    pub symbols: Vec<Sym>, // in the relation's own order
    pub assumptions: Vec<String>,
    pub requires: Vec<String>,
    pub excludes: Vec<String>,
    pub dimensions_consistent: bool,
    pub forms: HashMap<String, Vec<Form>>,
    pub residual_terms: Vec<Node>,
    pub residual_bounded: Vec<Node>,
    /// the family it belongs to and its documented questions in words (search and the question reader)
    pub family: String,
    pub examples: Vec<String>,
}

impl Relation {
    pub fn sym(&self, name: &str) -> Option<&Sym> {
        self.symbols.iter().find(|s| s.name == name)
    }
}

#[derive(Clone, Debug)]
pub struct ProcInput {
    pub symbol: String,
    pub kind: String,
    pub meaning: String,
    pub required: bool,
    pub ty: String,
    /// a "choice" input's allowed values, in the package's own spelling
    pub choices: Vec<String>,
    /// runtime/inputs.py limits: `min`, `max` (integer; the value and how Python prints it), `min_length`, `max_length`
    pub min: Option<(f64, String)>,
    pub max: Option<(f64, String)>,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
}

/// A value a native procedure receives, read exactly as runtime/inputs.py `coerce` reads it.
#[derive(Clone, Debug, PartialEq)]
pub enum Val {
    Num(f64),
    Int(i64),
    Text(String),
    Bool(bool),
    Vec(Vec<f64>),
    Mat(Vec<Vec<f64>>),
    /// a graph's edges as runtime/inputs.py reads them: (u, v) or (u, v, weight), node names as text
    Edges(Vec<(String, String, Option<f64>)>),
}

impl Val {
    pub fn num(&self) -> f64 {
        match self {
            Val::Num(x) => *x,
            Val::Int(i) => *i as f64,
            Val::Bool(b) => if *b { 1.0 } else { 0.0 },
            _ => f64::NAN,
        }
    }
    pub fn int(&self) -> i64 {
        match self {
            Val::Int(i) => *i,
            Val::Num(x) => *x as i64,
            Val::Bool(b) => *b as i64,
            _ => 0,
        }
    }
    pub fn text(&self) -> &str {
        match self {
            Val::Text(t) => t,
            _ => "",
        }
    }
    pub fn boolean(&self) -> bool {
        matches!(self, Val::Bool(true))
    }
    pub fn vec(&self) -> Vec<f64> {
        match self {
            Val::Vec(v) => v.clone(),
            _ => vec![],
        }
    }
    pub fn mat(&self) -> Vec<Vec<f64>> {
        match self {
            Val::Mat(m) => m.clone(),
            _ => vec![],
        }
    }
}

#[derive(Clone, Debug)]
pub struct ProcOutput {
    pub symbol: String,
    pub kind: String,
    pub meaning: String,
    /// the output as an expression of the inputs (a procedure written as data); None for a native procedure
    pub expr: Option<Node>,
}

/// A certified procedure: either written as data (procedures_compiled.json: outputs as expressions of the inputs, the
/// conditions they hold under, the defining equations the outputs are checked against) or native: an algorithm
/// (an iteration, a case split, a correlation) the model's crate implements in Rust, ported from the package's Python.
#[derive(Clone, Debug)]
pub struct Procedure {
    pub key: String,
    pub name: String,
    pub method: String,
    pub primary: String,
    pub inputs: Vec<ProcInput>,
    pub outputs: Vec<ProcOutput>,
    pub conditions: Vec<(String, Node, String)>, // text, difference, operator
    pub equations: Vec<(Node, Node)>,
    pub assumptions: Vec<String>,
    pub structured_example: String,
    pub native: bool,
    /// its topic, keywords and documented question forms (search and the question reader)
    pub topic: String,
    pub keywords: Vec<String>,
    pub templates: Vec<String>,
    pub questions: Vec<String>,
}

/// A model's native procedures: (key, inputs as read: numbers in SI) -> None when the key is not one of them,
/// Some(Err(reason)) when a stated condition does not hold (the package's ValueError), else Some(Ok((outputs in the
/// package's order, as JSON values: a number, text, a list ..., residual of the package's own check))).
pub type NativeFn = fn(&str, &HashMap<String, Val>) -> Option<Result<(Vec<(String, serde_json::Value)>, f64), String>>;

#[derive(Clone, Debug)]
pub struct Pack {
    pub model: String,
    pub distribution: String,
    pub version: String,
    pub kinds: HashMap<String, Kind>,
    pub relations: Vec<Relation>,
    pub procedures: Vec<Procedure>,
    pub labels: HashMap<String, (String, String)>,
    /// the model crate's native procedures (set after loading the pack)
    pub native: Option<NativeFn>,
    /// the model crate's own live figures (figkit.rs; set after loading the pack, like `native`)
    pub figures: Option<crate::figkit::ModelFigs>,
    /// the wording vocabulary of formulas (context tags, implied values, sign words, refused structures) and the
    /// documented questions with the Python package's reading of each (read by ask.rs)
    pub terms: Value,
    pub forms: Vec<Value>,
    /// which result of one procedure is the quantity an input of another takes (models/<d>/capabilities/links.json,
    /// reviewed; read by the language layer's two-step readings): (from, output, to, input)
    pub links: Vec<(String, String, String, String)>,
    /// words that set a procedure's input to zero ("centreline" -> y = 0): (procedure, input, words)
    pub zero_words: Vec<(String, String, Vec<String>)>,
}

fn s(v: &Value, k: &str) -> String {
    v.get(k).and_then(|x| x.as_str()).unwrap_or("").to_string()
}

fn strs(v: &Value, k: &str) -> Vec<String> {
    v.get(k)
        .and_then(|x| x.as_array())
        .map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect())
        .unwrap_or_default()
}

/// A limit as a number and as Python's str() writes it (3, not 3.0, for an int).
fn limit(v: Option<&Value>) -> Option<(f64, String)> {
    let v = v?;
    if let Some(i) = v.as_i64() {
        return Some((i as f64, i.to_string()));
    }
    v.as_f64().map(|x| (x, crate::num::py_repr_f64(x)))
}

fn nodes(v: &Value) -> Result<Vec<Node>, String> {
    v.as_array().ok_or("expected a list")?.iter().map(Node::from_json).collect()
}

impl Pack {
    pub fn from_json(text: &str) -> Result<Pack, String> {
        let doc: Value = serde_json::from_str(text).map_err(|e| e.to_string())?;
        let mut kinds = HashMap::new();
        for (name, k) in doc["kinds"].as_object().ok_or("no kinds")? {
            let dim = Dim::parse(&strs(k, "dim")).ok_or_else(|| format!("bad dimension of {name}"))?;
            kinds.insert(name.clone(), Kind { name: name.clone(), unit: s(k, "unit"), dim,
                                              difference: k["difference"].as_bool().unwrap_or(false) });
        }
        let mut relations = Vec::new();
        for r in doc["relations"].as_array().ok_or("no relations")? {
            let symbols_obj = r["symbols"].as_object().ok_or("no symbols")?;
            let mut symbols = Vec::new();
            for name in strs(r, "order") {
                let x = &symbols_obj[&name];
                let default = match x.get("default") {
                    Some(d) if d.is_object() => Some(Default {
                        value: match &d["value"] {
                            Value::Number(n) => n.as_f64().unwrap_or(0.0),
                            Value::String(t) => t.trim().parse::<f64>().map_err(|e| e.to_string())?,
                            _ => return Err(format!("{}: bad default", s(r, "key"))),
                        },
                        unit: s(d, "unit"),
                        assumption: d.get("assumption").and_then(|a| a.as_str()).map(String::from),
                        unless: strs(d, "unless"),
                    }),
                    _ => None,
                };
                let role = s(x, "role");
                let sign = s(x, "sign");
                symbols.push(Sym { name: name.clone(), kind: s(x, "kind"),
                                   role: if role.is_empty() { "plain".into() } else { role },
                                   sign: if sign.is_empty() { "any".into() } else { sign },
                                   desc: s(x, "desc"), default });
            }
            let mut forms = HashMap::new();
            for (t, list) in r["forms"].as_object().ok_or("no forms")? {
                let mut fs = Vec::new();
                for f in list.as_array().ok_or("bad forms")? {
                    let alt = match f.get("alt") {
                        Some(a) if a.is_object() => Some(Node::from_json(a)?),
                        _ => None,
                    };
                    let variants = match f.get("variants") {
                        Some(v) if v.is_array() => nodes(v)?,
                        _ => vec![],
                    };
                    fs.push(Form { form: Node::from_json(&f["form"])?, alt, variants, terms: nodes(&f["terms"])?,
                                   bounded_terms: nodes(&f["bounded_terms"])? });
                }
                forms.insert(t.clone(), fs);
            }
            relations.push(Relation {
                key: s(r, "key"), name: s(r, "name"), equation: s(r, "equation"), lhs: s(r, "lhs"), symbols,
                assumptions: strs(r, "assumptions"), requires: strs(r, "requires"), excludes: strs(r, "excludes"),
                dimensions_consistent: r["dimensions_consistent"].as_bool().unwrap_or(false), forms,
                residual_terms: nodes(&r["residual_terms"])?, residual_bounded: nodes(&r["residual_bounded"])?,
                family: s(r, "family"), examples: strs(r, "examples"),
            });
        }
        let mut procedures = Vec::new();
        for p in doc["procedures"].as_array().cloned().unwrap_or_default() {
            let inputs = p["inputs"].as_array().cloned().unwrap_or_default().iter().map(|i| ProcInput {
                symbol: s(i, "symbol"), kind: s(i, "kind"), meaning: s(i, "meaning"),
                required: i["required"].as_bool().unwrap_or(true), ty: s(i, "type"), choices: strs(i, "choices"),
                min: limit(i.get("min")), max: limit(i.get("max")),
                min_length: i.get("min_length").and_then(|x| x.as_u64()).map(|x| x as usize),
                max_length: i.get("max_length").and_then(|x| x.as_u64()).map(|x| x as usize),
            }).collect();
            let mut outputs = Vec::new();
            for o in p["outputs"].as_array().cloned().unwrap_or_default() {
                let expr = match o.get("expr") {
                    Some(e) if !e.is_null() => Some(Node::from_json(e)?),
                    _ => None,
                };
                outputs.push(ProcOutput { symbol: s(&o, "symbol"), kind: s(&o, "kind"), meaning: s(&o, "meaning"), expr });
            }
            let mut conditions = Vec::new();
            for c in p["conditions"].as_array().cloned().unwrap_or_default() {
                conditions.push((s(&c, "text"), Node::from_json(&c["diff"])?, s(&c, "op")));
            }
            let mut equations = Vec::new();
            for e in p["equations"].as_array().cloned().unwrap_or_default() {
                equations.push((Node::from_json(&e["lhs"])?, Node::from_json(&e["rhs"])?));
            }
            procedures.push(Procedure { key: s(&p, "key"), name: s(&p, "name"), method: s(&p, "method"),
                                        primary: s(&p, "primary"), inputs, outputs, conditions, equations,
                                        assumptions: strs(&p, "assumptions"),
                                        structured_example: s(&p, "structured_example"),
                                        native: p["native"].as_bool().unwrap_or(false),
                                        topic: s(&p, "topic"), keywords: strs(&p, "keywords"),
                                        templates: strs(&p, "templates"), questions: strs(&p, "questions") });
        }
        let mut labels = HashMap::new();
        if let Some(o) = doc["labels"].as_object() {
            for (k, v) in o {
                if let Some(a) = v.as_array() {
                    if a.len() == 2 {
                        labels.insert(k.clone(), (a[0].as_str().unwrap_or("").to_string(),
                                                  a[1].as_str().unwrap_or("").to_string()));
                    }
                }
            }
        }
        Ok(Pack { model: s(&doc, "model"), distribution: s(&doc, "distribution"), version: s(&doc, "version"),
                  kinds, relations, procedures, labels, native: None, figures: None,
                  terms: doc.get("terms").cloned().unwrap_or(Value::Null),
                  forms: doc.get("forms").and_then(|f| f.as_array()).cloned().unwrap_or_default(),
                  links: doc["links"]["links"].as_array().cloned().unwrap_or_default().iter()
                      .map(|l| (s(l, "from"), s(l, "output"), s(l, "to"), s(l, "input"))).collect(),
                  zero_words: doc["links"]["zero"].as_array().cloned().unwrap_or_default().iter()
                      .map(|z| (s(z, "capability"), s(z, "input"), strs(z, "words"))).collect() })
    }

    pub fn relation(&self, key: &str) -> Option<&Relation> {
        self.relations.iter().find(|r| r.key == key)
    }

    pub fn procedure(&self, key: &str) -> Option<&Procedure> {
        self.procedures.iter().find(|p| p.key == key)
    }
}
