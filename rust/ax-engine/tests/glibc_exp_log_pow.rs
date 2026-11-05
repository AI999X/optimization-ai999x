//! exp, log and pow (ax_engine::armmath) give the GNU C library's bits, as the Linux reference Python computes them:
//! 3,300 cases sampled from 2.1 million random ones (all 2.1 million matched when the port was made, 2026-10-29),
//! including large negative exponents whose result depends on which products GCC fuses.

use ax_engine::armmath;

#[test]
fn bit_identical_to_glibc() {
    let text = include_str!("glibc_exp_log_pow.txt");
    let h = |s: &str| f64::from_bits(u64::from_str_radix(s, 16).unwrap());
    let (mut n, mut bad) = (0, vec![]);
    for line in text.lines().filter(|l| !l.starts_with('#') && !l.is_empty()) {
        let p: Vec<&str> = line.split(' ').collect();
        let (x, want) = (h(p[1]), u64::from_str_radix(p[3], 16).unwrap());
        let got = match p[0] {
            "e" => armmath::exp(x),
            "l" => armmath::log(x),
            _ => armmath::pow(x, h(p[2])),
        };
        n += 1;
        if got.to_bits() != want {
            bad.push(line.to_string());
        }
    }
    assert!(n >= 3300, "{n} cases read");
    assert!(bad.is_empty(), "{} of {n} differ, e.g. {:?}", bad.len(), &bad[..bad.len().min(5)]);
}

#[test]
fn special_values_as_the_c_library() {
    assert_eq!(armmath::pow(2.0, -0.5), 0.7071067811865476);  // the libm crate gave 0.7071067811865475
    assert_eq!(armmath::pow(0.0, -1.0), f64::INFINITY);
    assert_eq!(armmath::pow(-8.0, 3.0), -512.0);
    assert!(armmath::pow(-8.0, 0.5).is_nan());
    assert_eq!(armmath::pow(1.0, f64::NAN), 1.0);
    assert_eq!(armmath::exp(0.0), 1.0);
    assert_eq!(armmath::exp(-1000.0), 0.0);
    assert_eq!(armmath::exp(1000.0), f64::INFINITY);
    assert_eq!(armmath::log(1.0), 0.0);
    assert_eq!(armmath::log(0.0), f64::NEG_INFINITY);
    assert!(armmath::log(-1.0).is_nan());
    assert_eq!(armmath::log(f64::from_bits(1)), -744.4400719213812);       // the smallest subnormal
}
