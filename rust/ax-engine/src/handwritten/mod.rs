//! Procedures (or parts of them) written by hand in Rust where the package's Python uses something the translator
//! cannot carry (tools/py2rust.py REPLACE lists them per model). One file per model; each function keeps the Python's
//! inputs, outputs and refusal wording, and is graded against the Python package by tools/rust_accuracy.py like every
//! translated procedure.

use crate::py::{Py, PyErr, R};

pub mod differential_equations;
pub mod discrete_math;
pub mod materials_science;

pub fn helper(model: &str, name: &str, a: &[Py], kws: &[(String, Py)]) -> R {
    let got = match model {
        "discrete_math" => discrete_math::helper(name, a, kws),
        "differential_equations" => differential_equations::helper(name, a, kws),
        "materials_science" => materials_science::helper(name, a, kws),
        _ => None,
    };
    got.unwrap_or_else(|| Err(PyErr::Unsupported(format!("no hand-written {model}.{name}"))))
}

/// The i-th positional argument (None when absent, as a missing optional input)
pub fn arg(a: &[Py], i: usize) -> &Py {
    a.get(i).unwrap_or(&Py::None)
}
