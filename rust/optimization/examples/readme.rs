//! Optimization: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p optimization-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&optimization_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "polynomial_extrema_interval", "inputs": {"coefficients": [1, -3, 0, 2], "lo": "-1", "hi": "3"}}));
    println!();
    show(json!({"key": "break_even_point", "inputs": {"F_": "5000", "p": "25", "v": "15"}}));
    println!();
    show(json!({"key": "mdp_discounted", "inputs": {"R": [[1, 0], [0, 2]], "T": [[1, 0], [0, 1], [0, 1], [1, 0]], "gamma": "0.9"}}));
    println!();
    show(json!({"key": "profit_max_linear_demand", "inputs": {"a": "100", "b": "2", "F_": "300", "cv": "20"}}));
    println!();
    show(json!({"key": "zero_sum_2x2", "inputs": {"A": [[3, -1], [-2, 4]]}}));
    println!();
    show(json!({"key": "knapsack_01", "inputs": {"values": [60, 100, 120], "weights": [10, 20, 30], "capacity": "50"}}));
    println!();
    show(json!({"key": "eoq", "inputs": {"D": "1200", "S": "50", "H": "3"}}));
    println!();
    show(json!({"key": "linear_program", "inputs": {"sense": "maximize", "c": [3, 5], "A": [[1, 0], [0, 2], [3, 2]], "b": [4, 12, 18], "relations": "<=,<=,<="}}));
    println!();
    show(json!({"key": "ahp_consistency_ratio", "inputs": {"judgments": [2, 4, 2]}}));
    println!();
    show(json!({"key": "all_pairs_shortest_paths", "inputs": {"edges": "A-B 3, B-C 4, A-C 9, C-D 2", "directed": "false"}}));
    println!();
    show(json!({"key": "equality_constrained_qp", "inputs": {"Q": "2 0; 0 2", "c": "0, 0", "A": "1 1", "b": "2"}}));
    println!();
    show(json!({"key": "critical_path_method", "inputs": {"durations": [3, 4, 2, 5], "precedences": "A-B, A-C, B-D, C-D"}}));
    println!();
    show(json!({"key": "erlang_b", "inputs": {"A": "3", "c": "5"}}));
    println!();
    show(json!({"key": "littlewood_protection", "inputs": {"p1": "400", "p2": "150", "mu": "60", "sigma": "20"}}));
    println!();
    show(json!({"key": "edd_max_lateness", "inputs": {"p": [2, 4, 1, 3], "d": [5, 6, 3, 9]}}));
}
