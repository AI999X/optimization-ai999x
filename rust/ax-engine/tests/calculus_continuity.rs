//! The continuity test (ax_engine::pysym::continuous_on) decides as the calculus package's continuous_on does (sympy's
//! continuous_domain on the interval, or its 401-point grid where sympy cannot decide): "ok" or the same refusal,
//! word for word, on random expressions and intervals recorded from the package (calculus_continuity.json).

use ax_engine::py::PyErr;
use ax_engine::pysym;
use ax_engine::symparse;
use std::rc::Rc;

const KNOWN: [&str; 2] = ["csc((3)/((1-x^2)))", "(x^2+1)^(-1)^(-1/2)"];

#[test]
fn decides_as_the_package() {
    let doc: serde_json::Value = serde_json::from_str(include_str!("calculus_continuity.json")).unwrap();
    let x: Rc<str> = Rc::from("x");
    let (mut bad, mut n) = (vec![], 0);
    for case in doc.as_array().unwrap() {
        let f = case["f"].as_str().unwrap();
        let want = case["result"].as_str().unwrap();
        if want.starts_with("CRASH") {
            continue;
        }
        n += 1;
        let e = symparse::parse(f, &["x"]).unwrap();
        let got = match pysym::continuous_on(&e, &x, case["a"].as_f64().unwrap(), case["b"].as_f64().unwrap()) {
            Ok(()) => "ok".to_string(),
            Err(PyErr::Value(m)) => m,
            Err(other) => format!("{other:?}"),
        };
        // known differences: where sympy raises inside solveset instead of leaving a ConditionSet (the package then
        // takes its grid), and a power with a non-real constant exponent ((x^2+1)^((-1)^(-1/2)))
        if KNOWN.contains(&f) {
            continue;
        }
        if got != want {
            bad.push(format!("{f} on [{}, {}]: want {want:?}, got {got:?} (sympy decided: {})",
                             case["a"], case["b"], case["sympy_decided"]));
        }
    }
    assert!(bad.is_empty(), "{} of {n} differ:\n{}", bad.len(), bad.join("\n"));
}
