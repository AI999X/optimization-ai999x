//! Every model's command line (the model crate's main.rs calls `run`):
//!
//!   <model> list                              every capability: its id and name
//!   <model> info <capability>                 the equation or method, its inputs and outputs with their units
//!   <model> solve <capability> name=value ... [--target X] [--json]
//!                                             a list input as name="[1, 2, 3]" or name="1, 2, 3"
//!   <model> figures <capability> name=value ... [--target X] [--out DIR]
//!                                             the live figures for those inputs (JSON; with --out, SVG files)
//!   <model> batch < cases.jsonl               one JSON answer per line (scripts, agents, the parity checks)
//!   <model> catalog                           every capability with its inputs, as JSON (forms, agents)
//!   <model> ask "<question in words>"         read the question and answer it, or say what is missing [--json]
//!   <model> ask-batch < questions.txt         one JSON reply per line (a line is the question, or a JSON string)
//!   <model> search <words> [--limit N]        capabilities matching the words, best first [--json]
//!   <model> topics                            every capability by topic [--json]
//!   <model> --version

use crate::engine::{solve, Answer, Input};
use crate::pack::Pack;
use std::io::{self, BufRead, Write};

fn usage(name: &str) -> i32 {
    eprintln!("usage: {name} list | info <capability> | solve <capability> name=value ... [--target X] [--json] \
               | figures <capability> name=value ... [--out DIR] | batch | catalog | ask \"<question>\" | ask-batch \
               | search <words> [--limit N] | topics | --version");
    2
}

/// name=value arguments (a JSON list stays a list: a vector or a matrix; anything else is text, read like a form
/// field), --target X, and flags (--json; --out DIR).
type Args = (Vec<(String, Input)>, Option<String>, Vec<(String, String)>);
fn read_args(args: &[String]) -> Option<Args> {
    let (mut given, mut target, mut flags) = (vec![], None, vec![]);
    let mut i = 0;
    while i < args.len() {
        let a = &args[i];
        if a == "--json" {
            flags.push((a.clone(), String::new()));
        } else if a == "--target" || a == "--out" {
            let v = args.get(i + 1)?.clone();
            if a == "--target" {
                target = Some(v);
            } else {
                flags.push((a.clone(), v));
            }
            i += 1;
        } else if let Some((n, v)) = a.split_once('=') {
            let input = match serde_json::from_str::<serde_json::Value>(v) {
                Ok(j @ serde_json::Value::Array(_)) => Input::Json(j),
                _ => Input::Text(v.to_string()),
            };
            given.push((n.to_string(), input));
        } else {
            return None;
        }
        i += 1;
    }
    Some((given, target, flags))
}

fn fmt10(x: f64) -> String {
    crate::num::py_g(x, 10)
}

fn show_value(v: &serde_json::Value) -> String {
    match v {
        serde_json::Value::Number(n) => fmt10(n.as_f64().unwrap_or(f64::NAN)),
        serde_json::Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

fn show(a: &Answer) {
    if !a.solved() {
        println!("{}: {}", a.outcome, a.message);
        return;
    }
    let sym = a.target.clone().unwrap_or_default();
    let unit = a.unit.clone().unwrap_or_default();
    let unit = if unit == "1" { String::new() } else { format!(" {unit}") };
    if a.values.is_empty() {
        println!("{} ({}) = {}{}", a.phrase, sym, fmt10(a.value.unwrap_or(f64::NAN)), unit);
    } else {
        for (s, v, u) in &a.values {                     // a procedure: every output with its unit
            let u = if u.is_empty() || u == "1" { String::new() } else { format!(" {u}") };
            println!("{s} = {}{u}", show_value(v));
        }
    }
    println!("  by: {}  ({})", a.name, a.equation);
    for (n, v, u, src) in &a.inputs {
        let u = if u == "1" || u.is_empty() { String::new() } else { format!(" {u}") };
        if v.is_nan() && !src.is_empty() {
            println!("  {n} = {src}");
        } else {
            if src.is_empty() {
                println!("  {n} = {}{u}", fmt10(*v));
            } else {
                println!("  {n} = {}{u}   [{src}]", fmt10(*v));
            }
        }
    }
    if !a.assumptions.is_empty() {
        println!("  assumes: {}", a.assumptions.join("; "));
    }
}

/// A reply of `ask` in plain text: which capability the question was read as, then the answer as `solve` prints it
/// (the reply's own solve request run again), or the reader's message and the other readings.
fn show_ask(p: &Pack, v: &serde_json::Value) {
    let s = |k: &str| v[k].as_str().unwrap_or("").to_string();
    if !s("name").is_empty() {
        println!("Read as: {}", s("name"));
    }
    let mut shown = false;
    if s("outcome") == "SOLVED" {
        let req = &v["request"];
        if let (Some(key), Some(inputs)) = (req["key"].as_str(), req["inputs"].as_object()) {
            let given: Vec<(String, Input)> = inputs.iter().map(|(n, x)| {
                let input = match x {
                    serde_json::Value::String(t) => Input::Text(t.clone()),
                    j @ serde_json::Value::Array(_) => Input::Json(j.clone()),
                    other => Input::Text(other.to_string()),
                };
                (n.clone(), input)
            }).collect();
            let a = solve(p, key, &given, req["target"].as_str());
            if a.solved() {
                show(&a);
                shown = true;
            }
        }
    }
    if !shown {
        let msg = s("message");
        println!("{}: {}", s("outcome"), if msg.is_empty() { "the question could not be answered".into() } else { msg });
    }
    if let Some(alts) = v["alternatives"].as_array() {
        let names: Vec<String> = alts.iter().filter_map(|a| {
            a["name"].as_str().or(a.as_str()).map(|n| match a["key"].as_str() {
                Some(k) => format!("{n} ({k})"),
                None => n.to_string(),
            })
        }).collect();
        if !names.is_empty() {
            println!("  or read as: {}", names.join("; "));
        }
    }
}

/// The command line of the model in `p`, called `name`. Returns the process exit code.
pub fn run(p: &Pack, name: &str, args: &[String]) -> i32 {
    match args.first().map(|s| s.as_str()) {
        Some("--version") | Some("-V") => {
            println!("{} {} (Rust build, ax-engine {})", p.distribution, p.version, env!("CARGO_PKG_VERSION"));
            0
        }
        Some("list") => {
            for r in &p.relations {
                println!("{:<40} {}", r.key, r.name);
            }
            for pr in &p.procedures {
                println!("{:<40} {}", pr.key, pr.name);
            }
            0
        }
        Some("catalog") => {
            println!("{}", crate::catalog_json(p));
            0
        }
        Some("info") => {
            let Some(key) = args.get(1) else { return usage(name) };
            if let Some(pr) = p.procedure(key) {
                println!("{}\n  {}", pr.name, pr.method);
                for i in &pr.inputs {
                    let unit = p.kinds.get(&i.kind).map(|k| k.unit.as_str()).unwrap_or("");
                    let ty = if i.ty.is_empty() || i.ty == "number" { String::new() } else { format!(" ({})", i.ty) };
                    println!("  in   {:<8} {}{} [{}]", i.symbol, i.meaning, ty, unit);
                }
                for o in &pr.outputs {
                    let unit = p.kinds.get(&o.kind).map(|k| k.unit.as_str()).unwrap_or("");
                    println!("  out  {:<8} {} [{}]", o.symbol, o.meaning, unit);
                }
                return 0;
            }
            let Some(r) = p.relation(key) else {
                eprintln!("unknown capability '{key}' (see: {name} list)");
                return 1;
            };
            println!("{}\n  {}", r.name, r.equation);
            for s in &r.symbols {
                let unit = p.kinds.get(&s.kind).map(|k| k.unit.as_str()).unwrap_or("");
                let d = if s.default.is_some() { " (has a default)" } else { "" };
                println!("  {:<8} {} [{}]{}", s.name, if s.desc.is_empty() { &s.kind } else { &s.desc }, unit, d);
            }
            if !r.assumptions.is_empty() {
                println!("  assumes: {}", r.assumptions.join("; "));
            }
            0
        }
        Some("solve") => {
            let Some(key) = args.get(1) else { return usage(name) };
            let Some((given, target, flags)) = read_args(&args[2..]) else { return usage(name) };
            let a = solve(p, key, &given, target.as_deref());
            if flags.iter().any(|(f, _)| f == "--json") {
                println!("{}", serde_json::to_string(&a).unwrap());
            } else {
                show(&a);
            }
            if a.solved() { 0 } else { 1 }
        }
        Some("figures") => {
            // the live figures for these inputs, as JSON; with --out DIR each is also written as DIR/<slug>.svg
            let Some(key) = args.get(1) else { return usage(name) };
            let Some((given, target, flags)) = read_args(&args[2..]) else { return usage(name) };
            let out = flags.iter().find(|(f, _)| f == "--out").map(|(_, d)| d.clone());
            let figs = crate::figures::figures(p, key, &given, target.as_deref(), None);
            let mut listed = vec![];
            for f in &figs {
                let mut item = serde_json::json!({"slug": f.slug, "title": f.title, "caption": f.caption});
                match &out {
                    Some(dir) => {
                        let path = std::path::Path::new(dir).join(format!("{}.svg", f.slug));
                        if let Err(e) = std::fs::write(&path, &f.svg) {
                            eprintln!("cannot write {}: {e}", path.display());
                            return 1;
                        }
                        item["file"] = serde_json::json!(path.display().to_string());
                    }
                    None => item["svg"] = serde_json::json!(f.svg),
                }
                listed.push(item);
            }
            println!("{}", serde_json::to_string_pretty(&listed).unwrap_or_default());
            if figs.is_empty() { 1 } else { 0 }
        }
        Some("ask") => {
            let json = args.iter().any(|a| a == "--json");
            let q = args[1..].iter().filter(|a| *a != "--json").cloned().collect::<Vec<_>>().join(" ");
            if q.trim().is_empty() {
                return usage(name);
            }
            let v = crate::ask::ask_value(p, &q);
            if json {
                println!("{}", serde_json::to_string_pretty(&v).unwrap_or_default());
            } else {
                show_ask(p, &v);
            }
            if v["outcome"] == "SOLVED" { 0 } else { 1 }
        }
        Some("ask-batch") => {
            let stdin = io::stdin();
            let mut out = io::BufWriter::new(io::stdout());
            for line in stdin.lock().lines() {
                let Ok(line) = line else { break };
                if line.trim().is_empty() {
                    continue;
                }
                let q = match serde_json::from_str::<serde_json::Value>(&line) {
                    Ok(serde_json::Value::String(s)) => s,
                    _ => line,
                };
                writeln!(out, "{}", crate::ask::ask_json(p, &q)).ok();
            }
            0
        }
        Some("search") => {
            let mut limit = 20usize;
            let mut words = vec![];
            let mut json = false;
            let mut i = 1;
            while i < args.len() {
                if args[i] == "--limit" {
                    limit = args.get(i + 1).and_then(|x| x.parse().ok()).unwrap_or(20);
                    i += 2;
                    continue;
                }
                if args[i] == "--json" {
                    json = true;
                } else {
                    words.push(args[i].clone());
                }
                i += 1;
            }
            let v: serde_json::Value =
                serde_json::from_str(&crate::ask::search_json(p, &words.join(" "), limit)).unwrap_or_default();
            if json {
                println!("{}", serde_json::to_string_pretty(&v).unwrap_or_default());
            } else {
                let hits = v.as_array().cloned().unwrap_or_default();
                if hits.is_empty() {
                    println!("No capability matches \"{}\".", words.join(" "));
                }
                for h in &hits {                          // one line each: id, name, topic
                    let topic = h["topic"].as_str().unwrap_or("");
                    let topic = if topic.is_empty() { String::new() } else { format!("  ({topic})") };
                    println!("{:<36} {}{}", h["key"].as_str().unwrap_or(""), h["name"].as_str().unwrap_or(""), topic);
                }
            }
            0
        }
        Some("topics") => {
            let v: serde_json::Value = serde_json::from_str(&crate::ask::topics_json(p)).unwrap_or_default();
            if args.iter().any(|a| a == "--json") {
                println!("{}", serde_json::to_string_pretty(&v).unwrap_or_default());
            } else {
                for t in v["topics"].as_array().cloned().unwrap_or_default() {
                    println!("{}", t["topic"].as_str().unwrap_or(""));
                    for c in t["capabilities"].as_array().cloned().unwrap_or_default() {
                        println!("  {:<36} {}", c["key"].as_str().unwrap_or(""), c["name"].as_str().unwrap_or(""));
                    }
                }
            }
            0
        }
        Some("batch") => {
            // each line: {"key": "...", "inputs": [[name, value], ...], "target": null | "..."}
            let stdin = io::stdin();
            let mut out = io::BufWriter::new(io::stdout());
            for line in stdin.lock().lines() {
                let Ok(line) = line else { break };
                if line.trim().is_empty() {
                    continue;
                }
                // the same entry point as the browser build
                writeln!(out, "{}", crate::solve_json(p, &line)).ok();
            }
            0
        }
        _ => usage(name),
    }
}
