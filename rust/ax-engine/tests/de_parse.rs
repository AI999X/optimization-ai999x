//! The differential-equations reader (ax_engine::symparse::parse_text_de) reads text as the package's parse_text does
//! (its own rewriting, the engine's allowlisted parser, sinh/cosh/tanh rebuilt piecewise): the same refusal message for
//! every input it refuses, and for every input it reads the same value at two points (or the same "not real" /
//! "undefined"). 335 inputs over several variable lists, recorded from the package (de_parse.json).

use ax_engine::sym::{self, EvalErr};
use ax_engine::symparse;

#[test]
fn reads_as_the_package() {
    let doc: serde_json::Value = serde_json::from_str(include_str!("de_parse.json")).unwrap();
    let mut bad = vec![];
    for case in doc.as_array().unwrap() {
        let text = case["text"].as_str().unwrap();
        let names: Vec<&str> = case["names"].as_array().unwrap().iter().map(|v| v.as_str().unwrap()).collect();
        let got = symparse::parse_text_de(text, &names);
        if let Some(msg) = case.get("err") {
            match &got {
                Err(m) if m == msg.as_str().unwrap() => {}
                other => bad.push(format!("{text:?} {names:?}: want error {msg}, got {other:?}")),
            }
            continue;
        }
        let e = match got {
            Ok(e) => e,
            Err(m) => {
                bad.push(format!("{text:?} {names:?}: want a value, got error {m}"));
                continue;
            }
        };
        for (k, pt) in [[0.37, 1.3], [1.7, -0.6]].iter().enumerate() {
            let want = &case["ok"][k];
            let v = sym::eval(&e, &|s| names.iter().position(|n| *n == s).map(|i| pt[i]));
            let ok = match (want.as_f64(), &v) {
                (Some(w), Ok(g)) => (g - w).abs() <= 1e-9 * w.abs().max(1.0),
                (None, Err(EvalErr::NotReal)) => want == "notreal",
                // a negative base to a huge non-integer power: sympy's value underflows to 0 (its imaginary part too)
                (Some(w), Err(EvalErr::NotReal)) => w == 0.0,
                (None, Err(EvalErr::Undefined)) => want == "undefined",
                _ => false,
            };
            if !ok {
                bad.push(format!("{text:?} {names:?} at {pt:?}: want {want}, got {v:?} ({e:?})"));
            }
        }
    }
    assert!(bad.is_empty(), "{} differences:\n{}", bad.len(), bad.join("\n"));
}
