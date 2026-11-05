//! math.hypot and math.dist give CPython's bits (its own vector_norm, the same on every platform): 3,500 random
//! cases of 2 to 4 coordinates over magnitudes from subnormal to 1e93.

use ax_engine::py::{self, Py};

#[test]
fn bit_identical_to_cpython() {
    let h = |s: &str| f64::from_bits(u64::from_str_radix(s, 16).unwrap());
    let (mut n, mut bad) = (0, vec![]);
    for line in include_str!("cpython_hypot.txt").lines().filter(|l| !l.starts_with('#') && !l.is_empty()) {
        let p: Vec<&str> = line.split(' ').collect();
        let xs: Vec<f64> = p[1..p.len() - 1].iter().map(|s| h(s)).collect();
        let want = u64::from_str_radix(p[p.len() - 1], 16).unwrap();
        let got = if p[0] == "h" {
            py::hypot(&xs.iter().map(|x| Py::Float(*x)).collect::<Vec<_>>()).unwrap()
        } else {
            py::dist(&Py::floats(&xs[..3]), &Py::floats(&xs[3..])).unwrap()
        };
        n += 1;
        if got.f().unwrap().to_bits() != want {
            bad.push(line.to_string());
        }
    }
    assert!(n >= 3500, "{n} cases read");
    assert!(bad.is_empty(), "{} of {n} differ, e.g. {:?}", bad.len(), &bad[..bad.len().min(3)]);
}
