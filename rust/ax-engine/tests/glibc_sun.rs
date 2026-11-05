//! expm1, log10, sinh, cosh, tanh (ax_engine::sunmath), exp2 and log2 (ax_engine::armmath) give the GNU C library's bits, as the Linux reference
//! Python computes them: 500 cases each, sampled from 100,000 that all matched when the port was made (2026-10-30).

use ax_engine::sunmath;

#[test]
fn bit_identical_to_glibc() {
    let h = |s: &str| f64::from_bits(u64::from_str_radix(s, 16).unwrap());
    let (mut n, mut bad) = (0, vec![]);
    for line in include_str!("glibc_sun.txt").lines().filter(|l| !l.starts_with('#') && !l.is_empty()) {
        let p: Vec<&str> = line.split(' ').collect();
        let (x, want) = (h(p[1]), u64::from_str_radix(p[3], 16).unwrap());
        let got = match p[0] {
            "expm1" => sunmath::expm1(x),
            "log10" => sunmath::log10(x),
            "sinh" => sunmath::sinh(x),
            "cosh" => sunmath::cosh(x),
            "exp2" => ax_engine::armmath::exp2(x),
            "log2" => ax_engine::armmath::log2(x),
            _ => sunmath::tanh(x),
        };
        n += 1;
        if got.to_bits() != want {
            bad.push(line.to_string());
        }
    }
    assert!(n >= 3500, "{n} cases read");
    assert!(bad.is_empty(), "{} of {n} differ, e.g. {:?}", bad.len(), &bad[..bad.len().min(3)]);
}
