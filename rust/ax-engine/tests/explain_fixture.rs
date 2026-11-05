//! The shared explanation fixture (tests/fixtures/explanations_render.json at the repository root): Rust must render
//! every case to the same text as the Python runtime and the website.

use ax_engine::explain::render_json;
use serde_json::Value;

#[test]
fn shared_fixture_renders_identically() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/explanations_render.json");
    let text = std::fs::read_to_string(&path).expect("fixture");
    let f: Value = serde_json::from_str(&text).unwrap();
    let cases = f["cases"].as_array().unwrap();
    assert!(cases.len() >= 10);
    for c in cases {
        let got = render_json(c["template"].as_str().unwrap(), c["view"].as_str().unwrap());
        let want = c["expect"].as_str().map(|s| s.to_string());
        assert_eq!(got, want, "case {}", c["name"]);
    }
}
