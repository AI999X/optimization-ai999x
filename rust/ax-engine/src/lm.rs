//! One maths library on every platform: exp, log and pow are the GNU C library's own algorithms (crate::armmath,
//! bit-identical to the Linux reference Python), the trigonometric and hyperbolic functions come from the `libm`
//! crate (a pure-Rust port of musl's), never from the operating system. The native builds on Windows, Linux and
//! macOS and the browser build (whose Rust standard library already uses this port) then give bit-identical answers;
//! the operating systems' own libraries differ in the last bit on some inputs, which a cancellation can magnify into
//! a different refusal or a figure's axis. sqrt and the basic arithmetic are correctly rounded by IEEE 754 everywhere
//! and stay as they are. Call sites use `x.exp_lm()` and so on (tests/test_rust_lm checks no platform call remains).

pub trait Lm: Sized {
    fn exp_lm(self) -> Self;
    fn ln_lm(self) -> Self;
    fn sin_lm(self) -> Self;
    fn cos_lm(self) -> Self;
}

/// The f64-only functions (a complex number has its own forms where it needs them).
pub trait LmF64 {
    fn log10_lm(self) -> f64;
    fn log2_lm(self) -> f64;
    fn ln_1p_lm(self) -> f64;
    fn exp_m1_lm(self) -> f64;
    fn exp2_lm(self) -> f64;
    fn powf_lm(self, e: f64) -> f64;
    fn log_lm(self, base: f64) -> f64;
    fn tan_lm(self) -> f64;
    fn asin_lm(self) -> f64;
    fn acos_lm(self) -> f64;
    fn atan_lm(self) -> f64;
    fn atan2_lm(self, x: f64) -> f64;
    fn sinh_lm(self) -> f64;
    fn cosh_lm(self) -> f64;
    fn tanh_lm(self) -> f64;
    fn asinh_lm(self) -> f64;
    fn acosh_lm(self) -> f64;
    fn atanh_lm(self) -> f64;
    fn hypot_lm(self, y: f64) -> f64;
    fn cbrt_lm(self) -> f64;
}

impl Lm for f64 {
    fn exp_lm(self) -> f64 { crate::armmath::exp(self) }
    fn ln_lm(self) -> f64 { crate::armmath::log(self) }
    fn sin_lm(self) -> f64 { libm::sin(self) }
    fn cos_lm(self) -> f64 { libm::cos(self) }
}

impl LmF64 for f64 {
    fn log10_lm(self) -> f64 { crate::sunmath::log10(self) }
    fn log2_lm(self) -> f64 { crate::armmath::log2(self) }
    fn ln_1p_lm(self) -> f64 { crate::sunmath::log1p(self) }
    fn exp_m1_lm(self) -> f64 { crate::sunmath::expm1(self) }
    fn exp2_lm(self) -> f64 { crate::armmath::exp2(self) }
    fn powf_lm(self, e: f64) -> f64 { crate::armmath::pow(self, e) }
    // as the standard library defines it: ln(x) / ln(base)
    fn log_lm(self, base: f64) -> f64 { crate::armmath::log(self) / crate::armmath::log(base) }
    fn tan_lm(self) -> f64 { libm::tan(self) }
    fn asin_lm(self) -> f64 { libm::asin(self) }
    fn acos_lm(self) -> f64 { libm::acos(self) }
    fn atan_lm(self) -> f64 { libm::atan(self) }
    fn atan2_lm(self, x: f64) -> f64 { libm::atan2(self, x) }
    fn sinh_lm(self) -> f64 { crate::sunmath::sinh(self) }
    fn cosh_lm(self) -> f64 { crate::sunmath::cosh(self) }
    fn tanh_lm(self) -> f64 { crate::sunmath::tanh(self) }
    fn asinh_lm(self) -> f64 { libm::asinh(self) }
    fn acosh_lm(self) -> f64 { libm::acosh(self) }
    fn atanh_lm(self) -> f64 { libm::atanh(self) }
    fn hypot_lm(self, y: f64) -> f64 { libm::hypot(self, y) }
    fn cbrt_lm(self) -> f64 { libm::cbrt(self) }
}
