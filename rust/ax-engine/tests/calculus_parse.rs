//! The formula reader (ax_engine::symparse) reads text as the calculus package's parse_expr does: the same refusal
//! message for every input it refuses, and for every input it reads the same value at x = 0.37 and x = 1.3 (or the
//! same "not real" / "undefined"). 347 inputs, recorded from the package (calculus_parse.json).

use ax_engine::sym::{self, EvalErr};
use ax_engine::symparse;

#[test]
fn reads_as_the_package() {
    let doc: serde_json::Value = serde_json::from_str(include_str!("calculus_parse.json")).unwrap();
    let mut bad = vec![];
    for case in doc.as_array().unwrap() {
        let text = case["text"].as_str().unwrap();
        let got = symparse::parse(text, &["x"]);
        if let Some(msg) = case.get("err") {
            match &got {
                Err(m) if m == msg.as_str().unwrap() => {}
                other => bad.push(format!("{text:?}: want error {msg}, got {other:?}")),
            }
            continue;
        }
        let e = match got {
            Ok(e) => e,
            Err(m) => {
                bad.push(format!("{text:?}: want a value, got error {m}"));
                continue;
            }
        };
        for (k, xv) in [0.37, 1.3].iter().enumerate() {
            let want = &case["ok"][k];
            let v = sym::eval(&e, &|_| Some(*xv));
            let ok = match (want.as_f64(), &v) {
                (Some(w), Ok(g)) => (g - w).abs() <= 1e-9 * w.abs().max(1.0),
                (None, Err(EvalErr::NotReal)) => want == "notreal",
                (None, Err(EvalErr::Undefined)) => want == "undefined",
                _ => false,
            };
            if !ok {
                bad.push(format!("{text:?} at {xv}: want {want}, got {v:?} ({e:?})"));
            }
        }
    }
    assert!(bad.is_empty(), "{} differences:\n{}", bad.len(), bad.join("\n"));
}
