//! AI999X engine in Rust: the deterministic core of a model, with no algebra at run time. Each model's relations
//! arrive already solved for every symbol (tools/export_rust_pack.py, from the Python reference); this crate reads
//! units, checks dimensions, evaluates the solved forms, keeps the single admissible value or says plainly why there is
//! none, and checks the result against the original equation. Domain-neutral: no model's content lives here.

pub mod armmath;
mod armmath_data;
pub mod ask;
pub mod cli;
pub mod coerce;
pub mod engine;
pub mod explain;
pub mod expr;
pub mod figkit;
pub mod figures;
pub mod handwritten;
pub mod num;
pub mod pack;
pub mod py;
pub mod lm;
pub mod meter;
pub mod pycx;
pub mod pyfrac;
pub mod pymath;
pub mod pylimit;
pub mod pymp;
pub mod pympf;
pub mod pysym;
pub mod pysum;
pub mod sunmath;
pub mod sym;
pub mod symparse;
pub mod pyre;
pub mod pyfmt;
pub mod units;

pub use engine::{solve, Answer, Input};
pub use pack::{Pack, Val};

/// One request as JSON text -> the answer as JSON text (every model's command line, browser module and API).
/// Request: {"key": "<capability>", "inputs": [["m", "3 kg"], ["v", "4 m/s"]], "target": null | "<symbol>"}
/// (an object {"m": "3 kg", ...} is accepted for "inputs" too; a list input is a JSON list).
pub fn solve_json(p: &Pack, request: &str) -> String {
    use serde_json::{json, Value};
    let case: Value = match serde_json::from_str(request) {
        Ok(v) => v,
        Err(e) => return json!({"outcome": "UNRECOGNIZED_FORMAT", "message": format!("not a JSON request: {e}")}).to_string(),
    };
    let mut given = vec![];
    match &case["inputs"] {
        Value::Array(pairs) => {
            for pair in pairs {
                given.push((pair[0].as_str().unwrap_or("").to_string(), input_of(&pair[1])));
            }
        }
        Value::Object(map) => {
            for (k, v) in map {
                given.push((k.clone(), input_of(v)));
            }
        }
        _ => {}
    }
    let w0 = meter::work();
    let a = solve(p, case["key"].as_str().unwrap_or(""), &given, case["target"].as_str());
    // the figures for this very request (the page asks for them next) start from this answer and its work
    figures::remember(&case, &a, meter::work().wrapping_sub(w0));
    serde_json::to_string(&a).unwrap_or_default()
}

/// Every capability for a form: {"capabilities": [{"key", "name", "kind", "inputs": [{"symbol", "meaning", "unit",
/// "type", "choices"}]}]}.
pub fn catalog_json(p: &Pack) -> String {
    use serde_json::json;
    let unit = |k: &str| p.kinds.get(k).map(|x| x.unit.clone()).unwrap_or_default();
    let mut caps = vec![];
    for r in &p.relations {
        caps.push(json!({"key": r.key, "name": r.name, "kind": "formula", "equation": r.equation,
                         "inputs": r.symbols.iter().map(|s| json!({"symbol": s.name, "meaning": s.desc,
                                                                   "unit": unit(&s.kind)})).collect::<Vec<_>>()}));
    }
    for pr in &p.procedures {
        caps.push(json!({"key": pr.key, "name": pr.name, "kind": "procedure",
                         "inputs": pr.inputs.iter().map(|i| {
                             let ty = if i.ty.is_empty() { "number" } else { i.ty.as_str() };
                             let mut v = json!({"symbol": i.symbol, "meaning": i.meaning, "unit": unit(&i.kind),
                                                "type": ty});
                             if !i.choices.is_empty() {
                                 v["choices"] = json!(i.choices);
                             }
                             v
                         }).collect::<Vec<_>>()}));
    }
    json!({"model": p.distribution, "version": p.version, "capabilities": caps}).to_string()
}

/// The browser interface (WebAssembly) of a model crate: JavaScript writes the request's UTF-8 bytes into memory from
/// `ax_alloc`, calls `ax_solve(ptr, len)` (or `ax_figures(ptr, len)`, `ax_catalog()`; a question in words
/// `ax_ask(ptr, len)`, a search `ax_search(ptr, len)`, `ax_topics()`: ask.rs), then reads `ax_out_len()` bytes at the
/// returned pointer.
/// A model crate invokes it once with its pack function: `ax_engine::wasm_exports!(pack);`
#[macro_export]
macro_rules! wasm_exports {
    ($pack:path) => {
        #[cfg(target_arch = "wasm32")]
        mod wasm {
            use std::cell::RefCell;

            thread_local! { static OUT: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) }; }

            fn give(text: String) -> *const u8 {
                OUT.with(|o| {
                    *o.borrow_mut() = text.into_bytes();
                    o.borrow().as_ptr()
                })
            }

            #[no_mangle]
            pub extern "C" fn ax_alloc(len: usize) -> *mut u8 {
                let mut v: Vec<u8> = Vec::with_capacity(len.max(1));
                let p = v.as_mut_ptr();
                std::mem::forget(v);
                p
            }

            #[no_mangle]
            pub extern "C" fn ax_free(ptr: *mut u8, len: usize) {
                unsafe { drop(Vec::from_raw_parts(ptr, 0, len.max(1))) }
            }

            #[no_mangle]
            pub extern "C" fn ax_solve(ptr: *const u8, len: usize) -> *const u8 {
                let bytes = unsafe { std::slice::from_raw_parts(ptr, len) };
                give($crate::solve_json($pack(), &String::from_utf8_lossy(bytes)))
            }

            /// the live figures for a request (the same JSON as ax_solve): [{"slug", "title", "svg", "caption"}]
            #[no_mangle]
            pub extern "C" fn ax_figures(ptr: *const u8, len: usize) -> *const u8 {
                let bytes = unsafe { std::slice::from_raw_parts(ptr, len) };
                give($crate::figures::figures_json($pack(), &String::from_utf8_lossy(bytes)))
            }

            #[no_mangle]
            pub extern "C" fn ax_catalog() -> *const u8 {
                give($crate::catalog_json($pack()))
            }

            /// a question in words (UTF-8 text): {outcome, capability, inputs, target, request, answer?, ...}
            #[no_mangle]
            pub extern "C" fn ax_ask(ptr: *const u8, len: usize) -> *const u8 {
                let bytes = unsafe { std::slice::from_raw_parts(ptr, len) };
                give($crate::ask::ask_json($pack(), &String::from_utf8_lossy(bytes)))
            }

            /// a search request {"q": "...", "limit": 20} (or plain text): [{key, name, kind, topic, score, snippet}]
            #[no_mangle]
            pub extern "C" fn ax_search(ptr: *const u8, len: usize) -> *const u8 {
                let bytes = unsafe { std::slice::from_raw_parts(ptr, len) };
                let text = String::from_utf8_lossy(bytes);
                let (q, limit) = $crate::search_request(&text);
                give($crate::ask::search_json($pack(), &q, limit))
            }

            /// every capability by topic: {model, topics: [{topic, capabilities: [{key, name, kind}]}]}
            #[no_mangle]
            pub extern "C" fn ax_topics() -> *const u8 {
                give($crate::ask::topics_json($pack()))
            }

            #[no_mangle]
            pub extern "C" fn ax_out_len() -> usize {
                OUT.with(|o| o.borrow().len())
            }
        }
    };
}

/// A search request as the browser sends it: {"q": "...", "limit": 20}, or the query as plain text (limit 20).
pub fn search_request(text: &str) -> (String, usize) {
    match serde_json::from_str::<serde_json::Value>(text) {
        Ok(v) if v.is_object() => (v["q"].as_str().unwrap_or("").to_string(),
                                   v["limit"].as_u64().map(|n| n as usize).unwrap_or(20)),
        _ => (text.to_string(), 20),
    }
}

/// One request's input value from JSON: text, a whole number, a number, or (a list, true/false, null) as sent.
pub fn input_of(v: &serde_json::Value) -> Input {
    match v {
        serde_json::Value::String(s) => Input::Text(s.clone()),
        serde_json::Value::Number(n) if n.is_i64() => Input::Int(n.as_i64().unwrap()),
        serde_json::Value::Number(n) => Input::Float(n.as_f64().unwrap_or(f64::NAN)),
        other => Input::Json(other.clone()),
    }
}
