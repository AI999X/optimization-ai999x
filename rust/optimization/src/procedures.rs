//! optimization: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/optimization/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "ahp_consistency_ratio",
    "ahp_geometric_mean",
    "ahp_priority_vector",
    "all_pairs_shortest_paths",
    "assignment_max_profit",
    "assignment_min_cost",
    "bellman_ford_shortest_path",
    "bin_packing",
    "bipartite_matching",
    "break_even_point",
    "can_min_surface",
    "critical_path_method",
    "critical_point_two_variables",
    "edd_max_lateness",
    "eoq",
    "eoq_planned_shortages",
    "eoq_quantity_discount",
    "epq_production_lot",
    "equality_constrained_qp",
    "erlang_b",
    "facility_location",
    "fence_max_area",
    "graph_coloring",
    "integer_program",
    "johnson_two_machine",
    "knapsack_01",
    "knapsack_unbounded",
    "linear_program",
    "littlewood_protection",
    "lp_shadow_prices",
    "lp_two_variable_graphical",
    "machine_repair_queue",
    "max_flow",
    "max_weight_matching",
    "mdp_discounted",
    "mg1_pollaczek_khinchine",
    "min_cost_flow",
    "minimum_spanning_tree",
    "minimum_vertex_cover",
    "mixed_nash_2x2",
    "mm1_queue",
    "mmc_queue",
    "mmc_staffing",
    "mmck_finite_queue",
    "moore_hodgson_tardy_jobs",
    "newsvendor_discrete",
    "newsvendor_normal",
    "newsvendor_uniform",
    "open_box_max_volume",
    "order_up_to_level",
    "parallel_machine_makespan",
    "pert_project",
    "polynomial_extrema_interval",
    "profit_max_linear_demand",
    "pure_nash_equilibria",
    "quadratic_optimum",
    "reorder_point",
    "revenue_max_linear_demand",
    "set_cover",
    "shortest_path",
    "spt_mean_flow_time",
    "steepest_descent_step",
    "topsis",
    "transportation_problem",
    "traveling_salesman",
    "unconstrained_quadratic_min",
    "wagner_whitin_lot_sizing",
    "weighted_product_model",
    "weighted_sum_model",
    "wspt_weighted_completion",
    "zero_sum_2x2",
    "zero_sum_game",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "ahp_consistency_ratio" => {
            let r = f_ahp_consistency(match v.get("judgments") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_cr(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ahp_geometric_mean" => {
            let r = f_ahp_geometric_mean(match v.get("judgments") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_gm(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ahp_priority_vector" => {
            let r = f_ahp_priority(match v.get("judgments") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_eigen(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "all_pairs_shortest_paths" => {
            let r = f_all_pairs(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } }, match v.get("directed") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_apsp(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "assignment_max_profit" => {
            let r = s_202(match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, (|| -> R { Ok(Py::Bool(true)) })().ok()?);
            finish(r, |i: &Py, o: &Py| -> f64 { match s_253(i.clone(), o.clone(), Py::Bool(true)).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "assignment_min_cost" => {
            let r = s_265(match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, (|| -> R { Ok(Py::Bool(false)) })().ok()?);
            finish(r, |i: &Py, o: &Py| -> f64 { match s_316(i.clone(), o.clone(), Py::Bool(false)).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bellman_ford_shortest_path" => {
            let r = f_bellman_ford(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } }, match v.get("source") { Some(x) => py::from_val(x), None => { return None } }, match v.get("target") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_bf(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bin_packing" => {
            let r = f_bin_packing(match v.get("sizes") { Some(x) => py::from_val(x), None => { return None } }, match v.get("capacity") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_bins(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bipartite_matching" => {
            let r = f_bipartite_matching(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_bm(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "break_even_point" => {
            let r = f_break_even(match v.get("F_") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_516(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "can_min_surface" => {
            let r = f_can_min_surface(match v.get("V") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_522(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "critical_path_method" => {
            let r = f_cpm(match v.get("durations") { Some(x) => py::from_val(x), None => { return None } }, match v.get("precedences") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_cpm(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "critical_point_two_variables" => {
            let r = f_critical_point_2var(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("e") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("f0") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_cp(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "edd_max_lateness" => {
            let r = f_edd(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_630(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "eoq" => {
            let r = f_eoq(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_652(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "eoq_planned_shortages" => {
            let r = f_eoq_backorders(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_eoq_b(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "eoq_quantity_discount" => {
            let r = f_eoq_discount(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i_rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("breaks") { Some(x) => py::from_val(x), None => { return None } }, match v.get("prices") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_discount(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "epq_production_lot" => {
            let r = f_epq(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_717(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "equality_constrained_qp" => {
            let r = f_eq_qp(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_eqqp(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "erlang_b" => {
            let r = f_erlang_b(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_erlang_b(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "facility_location" => {
            let r = f_facility_location(match v.get("fixed") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cost") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_850(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "fence_max_area" => {
            let r = f_fence_max_area(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("wall") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Bool(false)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_865(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "graph_coloring" => {
            let r = f_chromatic(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_col(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "integer_program" => {
            let r = f_integer_program(match v.get("sense") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("relations") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, (|| -> R { Ok(Py::Bool(false)) })().ok()?);
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_ip(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "johnson_two_machine" => {
            let r = f_johnson(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_johnson(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "knapsack_01" => {
            let r = f_knapsack01(match v.get("values") { Some(x) => py::from_val(x), None => { return None } }, match v.get("weights") { Some(x) => py::from_val(x), None => { return None } }, match v.get("capacity") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_knap(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "knapsack_unbounded" => {
            let r = f_knapsack_unbounded(match v.get("values") { Some(x) => py::from_val(x), None => { return None } }, match v.get("weights") { Some(x) => py::from_val(x), None => { return None } }, match v.get("capacity") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1248(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "linear_program" => {
            let r = f_linear_program(match v.get("sense") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("relations") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_lp(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "littlewood_protection" => {
            let r = f_littlewood(match v.get("p1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("capacity") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1472(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "lp_shadow_prices" => {
            let r = f_lp_shadow_prices(match v.get("sense") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("relations") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_lp(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "lp_two_variable_graphical" => {
            let r = f_lp_two_variable(match v.get("sense") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("relations") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_lp2(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "machine_repair_queue" => {
            let r = f_machine_repair(match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1843(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "max_flow" => {
            let r = f_max_flow(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } }, match v.get("source") { Some(x) => py::from_val(x), None => { return None } }, match v.get("target") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_maxflow(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "max_weight_matching" => {
            let r = f_max_weight_matching(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_mwm(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mdp_discounted" => {
            let r = f_mdp_discounted(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_mdp(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mg1_pollaczek_khinchine" => {
            let r = f_mg1(match v.get("lam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ES") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sd") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2086(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "min_cost_flow" => {
            let r = f_min_cost_flow(match v.get("capacity") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cost") { Some(x) => py::from_val(x), None => { return None } }, match v.get("supply") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_mcf(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "minimum_spanning_tree" => {
            let r = f_mst(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_mst(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "minimum_vertex_cover" => {
            let r = f_vertex_cover(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_vc(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mixed_nash_2x2" => {
            let r = f_mixed_nash_2x2(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_mixed(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mm1_queue" => {
            let r = f_mm1(match v.get("lam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_mm1(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mmc_queue" => {
            let r = f_mmc(match v.get("lam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_mmc(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mmc_staffing" => {
            let r = f_mmc_staffing(match v.get("lam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("target") { Some(x) => py::from_val(x), None => { return None } }, match v.get("criterion") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("wait_probability")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_staffing(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mmck_finite_queue" => {
            let r = f_mmc_finite(match v.get("lam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_mmck(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "moore_hodgson_tardy_jobs" => {
            let r = f_moore_hodgson(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_moore(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "newsvendor_discrete" => {
            let r = f_newsvendor_discrete(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("demands") { Some(x) => py::from_val(x), None => { return None } }, match v.get("probs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2685(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "newsvendor_normal" => {
            let r = f_newsvendor_normal(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2697(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "newsvendor_uniform" => {
            let r = f_newsvendor_uniform(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lo") { Some(x) => py::from_val(x), None => { return None } }, match v.get("hi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2707(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "open_box_max_volume" => {
            let r = f_open_box(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("W") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2716(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "order_up_to_level" => {
            let r = f_order_up_to(match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("SL") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2737(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "parallel_machine_makespan" => {
            let r = f_parallel_makespan(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_parallel(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pert_project" => {
            let r = f_pert(match v.get("times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("precedences") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("deadline") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_pert(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "polynomial_extrema_interval" => {
            let r = f_poly_extrema(match v.get("coefficients") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lo") { Some(x) => py::from_val(x), None => { return None } }, match v.get("hi") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_pe(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "profit_max_linear_demand" => {
            let r = f_profit_max_linear(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cv") { Some(x) => py::from_val(x), None => { return None } }, match v.get("F_") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2941(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pure_nash_equilibria" => {
            let r = f_pure_nash(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_pure_ne(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "quadratic_optimum" => {
            let r = f_quadratic_optimum(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("goal") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("optimum")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2999(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "reorder_point" => {
            let r = f_reorder_point(match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("SL") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3014(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "revenue_max_linear_demand" => {
            let r = f_revenue_max_linear(match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3024(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "set_cover" => {
            let r = f_set_cover(match v.get("sets") { Some(x) => py::from_val(x), None => { return None } }, match v.get("costs") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3057(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "shortest_path" => {
            let r = f_shortest_path(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } }, match v.get("source") { Some(x) => py::from_val(x), None => { return None } }, match v.get("target") { Some(x) => py::from_val(x), None => { return None } }, match v.get("directed") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_sp(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "spt_mean_flow_time" => {
            let r = f_spt(match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3165(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "steepest_descent_step" => {
            let r = f_steepest_step(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_sd(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "topsis" => {
            let r = f_topsis(match v.get("X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } }, match v.get("types") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3298(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "transportation_problem" => {
            let r = f_transportation(match v.get("supply") { Some(x) => py::from_val(x), None => { return None } }, match v.get("demand") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cost") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_transport(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "traveling_salesman" => {
            let r = f_tsp(match v.get("D") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_tsp(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "unconstrained_quadratic_min" => {
            let r = f_quadratic_min(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_qmin(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "wagner_whitin_lot_sizing" => {
            let r = f_wagner_whitin(match v.get("demand") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_ww(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "weighted_product_model" => {
            let r = f_weighted_product(match v.get("X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } }, match v.get("types") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3775(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "weighted_sum_model" => {
            let r = f_weighted_sum(match v.get("X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } }, match v.get("types") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3838(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "wspt_weighted_completion" => {
            let r = f_wspt(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3868(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "zero_sum_2x2" => {
            let r = f_zero_sum_2x2(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_2x2(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "zero_sum_game" => {
            let r = f_zero_sum_game(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_game(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c__IP_MEMO() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__RI() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::Int(3i64), Py::Float(0.58)), (Py::Int(4i64), Py::Float(0.9)), (Py::Int(5i64), Py::Float(1.12)), (Py::Int(6i64), Py::Float(1.24)), (Py::Int(7i64), Py::Float(1.32)), (Py::Int(8i64), Py::Float(1.41)), (Py::Int(9i64), Py::Float(1.45)), (Py::Int(10i64), Py::Float(1.49))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__TSP_MEMO() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![])) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__activity_index(mut v_n: Py) -> R {
    let mut v_idx: Py = Py::None;
    let mut v_k: Py = Py::None;
    v_idx = Py::dict(vec![]);
    for t2801 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t2801;
        py::setitem(&v_idx.clone(), &py::chr(&py::add(&py::ord(&Py::str("A"))?, &v_k.clone())?)?, v_k.clone())?;
        py::setitem(&v_idx.clone(), &Py::str(&py::to_str(&py::add(&v_k.clone(), &Py::Int(1i64))?)), v_k.clone())?;
    }
    return Ok(v_idx.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__adj(mut v_E: Py, mut v_directed: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_adj = Py::dict(vec![]);
    for t3082 in py::iter(&(v_E.clone()))? {
        let t3083 = (t3082).items()?;
        if t3083.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_u = t3083[0].clone();
        v_v = t3083[1].clone();
        v_w = t3083[2].clone();
        let _ = py::method(&py::method(&v_adj.clone(), "setdefault", &[v_u.clone(), Py::list(vec![])])?, "append", &[Py::tuple(vec![v_v.clone(), v_w.clone()])])?;
        let _ = py::method(&v_adj.clone(), "setdefault", &[v_v.clone(), Py::list(vec![])])?;
        if (py::not(&v_directed.clone())).truthy() {
            let _ = py::method(&py::getitem(&v_adj.clone(), &v_v.clone())?, "append", &[Py::tuple(vec![v_u.clone(), v_w.clone()])])?;
        }
    }
    return Ok(v_adj.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ahp_matrix(mut v_judgments: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_L = py::len(&v_judgments.clone())?;
    v_n = (match ('g81: { let mut out = vec![]; for t82 in py::range_iter(&[(Py::Int(2i64)).i()?, (Py::Int(11i64)).i()?]) { let mut v_k: Py; 
v_k = t82;
if ({ let t83 = py::floordiv(&py::mul(&v_k.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?, &Py::Int(2i64))?; Py::Bool({ let t84 = v_L.clone(); if py::eq(&t83, &t84) { true } else { false } }) }).truthy() { out.push(v_k.clone()); break 'g81 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => Py::None });
    if ({ let t85 = v_n.clone(); Py::Bool({ let t86 = Py::None; if py::is(&t85, &t86) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give the upper-triangle judgments a12, a13, ..., a1n, a23, ... (3, 6, 10, ... values for 3, 4, 5, ... criteria; at most 10 criteria)"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t87 in py::iter(&(v_judgments.clone()))? { let mut v_v: Py; 
v_v = t87;
out.push({ let t88 = v_v.clone(); Py::Bool({ let t89 = Py::Int(0i64); if py::le(&t88, &t89)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("every pairwise judgment must be > 0"))));
    }
    v_P = { let mut out = vec![]; for t90 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v__: Py; 
v__ = t90;
out.push(py::mul(&Py::list(vec![Py::Float(1.0)]), &v_n.clone())?); } Py::list(out) };
    v_t = Py::Int(0i64);
    for t91 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t91;
        for t92 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_j = t92;
            let t93 = (Py::tuple(vec![py::float(&py::getitem(&v_judgments.clone(), &v_t.clone())?)?, py::truediv(&Py::Float(1.0), &py::float(&py::getitem(&v_judgments.clone(), &v_t.clone())?)?)?])).items()?;
            if t93.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            py::setitem(&py::getitem(&v_P.clone(), &v_i.clone())?, &v_j.clone(), t93[0].clone())?;
            py::setitem(&py::getitem(&v_P.clone(), &v_j.clone())?, &v_i.clone(), t93[1].clone())?;
            v_t = py::iadd(&v_t, &Py::Int(1i64))?;
        }
    }
    return Ok(v_P.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__assignment(mut v_C: Py, mut v_maximize: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_M2: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_a2: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_big: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_unique: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_n = py::len(&v_C.clone())?;
    if ({ let t274 = { let t266 = v_n.clone(); Py::Bool({ let t267 = Py::Int(0i64); if py::eq(&t266, &t267) { true } else { false } }) }; if t274.truthy() { t274 } else { { let t273 = { let t268 = v_n.clone(); Py::Bool({ let t269 = Py::Int(12i64); if py::gt(&t268, &t269)? { true } else { false } }) }; if t273.truthy() { t273 } else { Py::Bool(({ let mut out = vec![]; for t270 in py::iter(&(v_C.clone()))? { let mut v_r: Py; 
v_r = t270;
out.push({ let t271 = py::len(&v_r.clone())?; Py::Bool({ let t272 = v_n.clone(); if py::ne(&t271, &t272) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give a square matrix with at most 12 rows (add dummy rows or columns of zeros for an unbalanced problem)"))));
    }
    v_M = { let mut out = vec![]; for t275 in py::iter(&(v_C.clone()))? { let mut v_r: Py; 
v_r = t275;
out.push({ let mut out = vec![]; for t276 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t276;
out.push((if (v_maximize.clone()).truthy() { py::neg(&f__fr(v_x.clone())?)? } else { f__fr(v_x.clone())? })); } Py::list(out) }); } Py::list(out) };
    let t301 = (f__hungarian(v_M.clone())?).items()?;
    if t301.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t301[0].clone();
    v_u = t301[1].clone();
    v_v = t301[2].clone();
    v_best = py::sum(&{ let mut out = vec![]; for t302 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t302;
out.push(py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &py::getitem(&v_a.clone(), &v_i.clone())?)?); } Py::list(out) }, None)?;
    v_unique = Py::Bool(true);
    for t303 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t303;
        v_big = py::add(&py::sum(&{ let mut out = vec![]; for t304 in py::iter(&(v_M.clone()))? { let mut v_r: Py; 
v_r = t304;
for t305 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t305;
out.push(py::abs(&v_x.clone())?); }} Py::list(out) }, None)?, &Py::Int(1i64))?;
        v_M2 = { let mut out = vec![]; for t306 in py::iter(&(v_M.clone()))? { let mut v_r: Py; 
v_r = t306;
out.push(Py::list((v_r.clone()).items()?)); } Py::list(out) };
        let t307 = py::getitem(&v_M2.clone(), &v_i.clone())?; let t308 = py::getitem(&v_a.clone(), &v_i.clone())?;
        py::setitem(&t307, &t308, py::iadd(&py::getitem(&t307, &t308)?, &v_big.clone())?)?;
        let t309 = (f__hungarian(v_M2.clone())?).items()?;
        if t309.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_a2 = t309[0].clone();
        v__ = t309[1].clone();
        v__ = t309[2].clone();
        if ({ let t310 = py::sum(&{ let mut out = vec![]; for t312 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t312;
out.push(py::getitem(&py::getitem(&v_M2.clone(), &v_k.clone())?, &py::getitem(&v_a2.clone(), &v_k.clone())?)?); } Py::list(out) }, None)?; Py::Bool({ let t311 = v_best.clone(); if py::eq(&t310, &t311) { true } else { false } }) }).truthy() {
            v_unique = Py::Bool(false);
            break;
        }
    }
    v_s = (if (v_maximize.clone()).truthy() { py::neg(&Py::Int(1i64))? } else { Py::Int(1i64) });
    v_out = Py::dict(vec![((if (v_maximize.clone()).truthy() { Py::str("max_value") } else { Py::str("min_cost") }), py::float(&py::mul(&v_s.clone(), &v_best.clone())?)?), ((if (v_unique.clone()).truthy() { Py::str("assignment") } else { Py::str("assignment_example") }), { let mut out = vec![]; for t313 in py::iter(&(v_a.clone()))? { let mut v_j: Py; 
v_j = t313;
out.push(py::add(&v_j.clone(), &Py::Int(1i64))?); } Py::list(out) }), (Py::str("assignment_unique"), (if (v_unique.clone()).truthy() { Py::str("yes") } else { Py::str("no") })), (Py::str("dual_u"), { let mut out = vec![]; for t314 in py::iter(&(v_u.clone()))? { let mut v_x: Py; 
v_x = t314;
out.push(py::float(&py::mul(&v_s.clone(), &v_x.clone())?)?); } Py::list(out) }), (Py::str("dual_v"), { let mut out = vec![]; for t315 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t315;
out.push(py::float(&py::mul(&v_s.clone(), &v_x.clone())?)?); } Py::list(out) })]);
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bellman(mut v_E: Py, mut v_s: Py, mut v_directed: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_arcs: Py = Py::None;
    let mut v_changed: Py = Py::None;
    let mut v_dist: Py = Py::None;
    let mut v_nodes: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_nodes = f__nodes(v_E.clone())?;
    v_arcs = py::add(&{ let mut out = vec![]; for t3125 in py::iter(&(v_E.clone()))? { let mut v_u: Py; let mut v_v: Py; let mut v_w: Py; 
let t3126 = (t3125).items()?;
if t3126.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_u = t3126[0].clone();
v_v = t3126[1].clone();
v_w = t3126[2].clone();
out.push(Py::tuple(vec![v_u.clone(), v_v.clone(), v_w.clone()])); } Py::list(out) }, &(if (v_directed.clone()).truthy() { Py::list(vec![]) } else { { let mut out = vec![]; for t3127 in py::iter(&(v_E.clone()))? { let mut v_u: Py; let mut v_v: Py; let mut v_w: Py; 
let t3128 = (t3127).items()?;
if t3128.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_u = t3128[0].clone();
v_v = t3128[1].clone();
v_w = t3128[2].clone();
out.push(Py::tuple(vec![v_v.clone(), v_u.clone(), v_w.clone()])); } Py::list(out) } }))?;
    v_dist = Py::dict(vec![(v_s.clone(), py::fraction(&[Py::Int(0i64)])?)]);
    for t3129 in py::range_iter(&[(py::len(&v_nodes.clone())?).i()?]) {
        v__ = t3129;
        v_changed = Py::Bool(false);
        for t3130 in py::iter(&(v_arcs.clone()))? {
            let t3139 = (t3130).items()?;
            if t3139.len() != 3 { return Err(PyErr::Value("unpack".into())); }
            v_u = t3139[0].clone();
            v_v = t3139[1].clone();
            v_w = t3139[2].clone();
            if ({ let t3138 = { let t3131 = v_u.clone(); Py::Bool({ let t3132 = v_dist.clone(); if py::contains(&t3132, &t3131)? { true } else { false } }) }; if !t3138.truthy() { t3138 } else { { let t3137 = { let t3133 = v_v.clone(); Py::Bool({ let t3134 = v_dist.clone(); if !py::contains(&t3134, &t3133)? { true } else { false } }) }; if t3137.truthy() { t3137 } else { { let t3135 = py::add(&py::getitem(&v_dist.clone(), &v_u.clone())?, &v_w.clone())?; Py::Bool({ let t3136 = py::getitem(&v_dist.clone(), &v_v.clone())?; if py::lt(&t3135, &t3136)? { true } else { false } }) } } } } }).truthy() {
                py::setitem(&v_dist.clone(), &v_v.clone(), py::add(&py::getitem(&v_dist.clone(), &v_u.clone())?, &v_w.clone())?)?;
                v_changed = Py::Bool(true);
            }
        }
        if (py::not(&v_changed.clone())).truthy() {
            return Ok(v_dist.clone());
        }
    }
    return Err(py::value_error(py::to_str(&Py::str("the graph has a negative cycle reachable from the source: shortest paths are undefined"))));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__best(mut v_scores: Py) -> R {
    let mut v_top: Py = Py::None;
    v_top = py::minmax(&[v_scores.clone()], true)?;
    return Ok(py::add(&(match ('g3830: { let mut out = vec![]; for t3831 in py::iter(&(py::enumerate(&v_scores.clone(), 0)?))? { let mut v_k: Py; let mut v_v: Py; 
let t3832 = (t3831).items()?;
if t3832.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3832[0].clone();
v_v = t3832[1].clone();
if ({ let t3833 = v_v.clone(); Py::Bool({ let t3834 = py::sub(&v_top.clone(), &py::mul(&Py::Float(1e-12), &py::minmax(&[Py::Float(1.0), py::abs(&v_top.clone())?], true)?)?)?; if py::ge(&t3833, &t3834)? { true } else { false } }) }).truthy() { out.push(v_k.clone()); break 'g3830 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) }), &Py::Int(1i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bimatrix(mut v_A: Py, mut v_B: Py, mut v_max_dim: Py) -> R {
    let mut v_MA: Py = Py::None;
    let mut v_MB: Py = Py::None;
    let t2954 = (Py::tuple(vec![f__payoff(v_A.clone(), Py::str("row payoff matrix"), v_max_dim.clone())?, f__payoff(v_B.clone(), Py::str("column payoff matrix"), v_max_dim.clone())?])).items()?;
    if t2954.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_MA = t2954[0].clone();
    v_MB = t2954[1].clone();
    if ({ let t2959 = { let t2955 = py::len(&v_MA.clone())?; Py::Bool({ let t2956 = py::len(&v_MB.clone())?; if py::ne(&t2955, &t2956) { true } else { false } }) }; if t2959.truthy() { t2959 } else { { let t2957 = py::len(&py::getitem(&v_MA.clone(), &Py::Int(0i64))?)?; Py::Bool({ let t2958 = py::len(&py::getitem(&v_MB.clone(), &Py::Int(0i64))?)?; if py::ne(&t2957, &t2958) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the row and column payoff matrices must have the same shape"))));
    }
    return Ok(Py::tuple(vec![v_MA.clone(), v_MB.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cert_from_lp(mut v_sense: Py, mut v_c: Py, mut v_A: Py, mut v_b: Py, mut v_rel: Py, mut v_x: Py, mut v_p: Py, mut v_value: Py) -> R {
    return Ok(f__lp_certificate_residual(Py::dict(vec![(Py::str("sense"), v_sense.clone()), (Py::str("c"), v_c.clone()), (Py::str("A"), v_A.clone()), (Py::str("b"), v_b.clone()), (Py::str("relations"), py::method(&Py::str(","), "join", &[v_rel.clone()])?)]), v_x.clone(), v_p.clone(), v_value.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__check_lp(mut v_sense: Py, mut v_c: Py, mut v_A: Py, mut v_b: Py, mut v_rel: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    if ({ let t3941 = v_sense.clone(); Py::Bool({ let t3942 = Py::tuple(vec![Py::str("maximize"), Py::str("minimize")]); if !py::contains(&t3942, &t3941)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the objective sense must be maximize or minimize"))));
    }
    let t3943 = (Py::tuple(vec![py::len(&v_c.clone())?, py::len(&v_A.clone())?])).items()?;
    if t3943.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t3943[0].clone();
    v_m = t3943[1].clone();
    if ({ let t3948 = { let t3944 = v_n.clone(); Py::Bool({ let t3945 = Py::Int(0i64); if py::eq(&t3944, &t3945) { true } else { false } }) }; if t3948.truthy() { t3948 } else { { let t3946 = v_m.clone(); Py::Bool({ let t3947 = Py::Int(0i64); if py::eq(&t3946, &t3947) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give at least one variable and one constraint"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t3949 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t3949;
out.push({ let t3950 = py::len(&v_r.clone())?; Py::Bool({ let t3951 = v_n.clone(); if py::ne(&t3950, &t3951) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["every constraint row must have ".to_string(), py::format(&v_n.clone(), "")?, " coefficients (one per variable)".to_string()].concat()))));
    }
    if ({ let t3952 = py::len(&v_b.clone())?; Py::Bool({ let t3953 = v_m.clone(); if py::ne(&t3952, &t3953) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["the right-hand side must have ".to_string(), py::format(&v_m.clone(), "")?, " entries (one per constraint)".to_string()].concat()))));
    }
    if ({ let t3959 = { let t3954 = py::len(&v_rel.clone())?; Py::Bool({ let t3955 = v_m.clone(); if py::ne(&t3954, &t3955) { true } else { false } }) }; if t3959.truthy() { t3959 } else { Py::Bool(({ let mut out = vec![]; for t3956 in py::iter(&(v_rel.clone()))? { let mut v_r: Py; 
v_r = t3956;
out.push({ let t3957 = v_r.clone(); Py::Bool({ let t3958 = Py::tuple(vec![Py::str("<="), Py::str(">="), Py::str("=")]); if !py::contains(&t3958, &t3957)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("each constraint relation must be <=, >= or ="))));
    }
    if ({ let t3964 = { let t3960 = v_n.clone(); Py::Bool({ let t3961 = Py::Int(12i64); if py::gt(&t3960, &t3961)? { true } else { false } }) }; if t3964.truthy() { t3964 } else { { let t3962 = v_m.clone(); Py::Bool({ let t3963 = Py::Int(12i64); if py::gt(&t3962, &t3963)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at most 12 variables and 12 constraints"))));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__completion(mut v_seq: Py, mut v_p: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t3864 = (Py::tuple(vec![py::fraction(&[Py::Int(0i64)])?, Py::dict(vec![])])).items()?;
    if t3864.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_t = t3864[0].clone();
    v_C = t3864[1].clone();
    for t3865 in py::iter(&(v_seq.clone()))? {
        v_j = t3865;
        v_t = py::iadd(&v_t, &f__fr(py::getitem(&v_p.clone(), &v_j.clone())?)?)?;
        py::setitem(&v_C.clone(), &v_j.clone(), v_t.clone())?;
    }
    return Ok(v_C.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__dijkstra(mut v_E: Py, mut v_s: Py, mut v_directed: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_adj: Py = Py::None;
    let mut v_cnt: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_dist: Py = Py::None;
    let mut v_done: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_adj = f__adj(v_E.clone(), v_directed.clone())?;
    let t3084 = (Py::tuple(vec![Py::dict(vec![(v_s.clone(), py::fraction(&[Py::Int(0i64)])?)]), py::set(&Py::list(vec![]))?, Py::list(vec![Py::tuple(vec![py::fraction(&[Py::Int(0i64)])?, Py::Int(0i64), v_s.clone()])]), Py::Int(1i64)])).items()?;
    if t3084.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_dist = t3084[0].clone();
    v_done = t3084[1].clone();
    v_h = t3084[2].clone();
    v_cnt = t3084[3].clone();
    while (v_h.clone()).truthy() {
        let t3085 = (py::heapq("heappop", &[v_h.clone()])?).items()?;
        if t3085.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_d = t3085[0].clone();
        v__ = t3085[1].clone();
        v_u = t3085[2].clone();
        if ({ let t3086 = v_u.clone(); Py::Bool({ let t3087 = v_done.clone(); if py::contains(&t3087, &t3086)? { true } else { false } }) }).truthy() {
            continue;
        }
        let _ = py::method(&v_done.clone(), "add", &[v_u.clone()])?;
        for t3088 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? {
            let t3094 = (t3088).items()?;
            if t3094.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_v = t3094[0].clone();
            v_w = t3094[1].clone();
            if ({ let t3093 = { let t3089 = v_v.clone(); Py::Bool({ let t3090 = v_dist.clone(); if !py::contains(&t3090, &t3089)? { true } else { false } }) }; if t3093.truthy() { t3093 } else { { let t3091 = py::add(&v_d.clone(), &v_w.clone())?; Py::Bool({ let t3092 = py::getitem(&v_dist.clone(), &v_v.clone())?; if py::lt(&t3091, &t3092)? { true } else { false } }) } } }).truthy() {
                py::setitem(&v_dist.clone(), &v_v.clone(), py::add(&v_d.clone(), &v_w.clone())?)?;
                let _ = py::heapq("heappush", &[v_h.clone(), Py::tuple(vec![py::add(&v_d.clone(), &v_w.clone())?, v_cnt.clone(), v_v.clone()])])?;
                v_cnt = py::iadd(&v_cnt, &Py::Int(1i64))?;
            }
        }
    }
    return Ok(Py::tuple(vec![v_dist.clone(), v_adj.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__erlang_b(mut v_a: Py, mut v_c: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_k: Py = Py::None;
    v_B = Py::Float(1.0);
    for t2586 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_c.clone(), &Py::Int(1i64))?).i()?]) {
        v_k = t2586;
        v_B = py::truediv(&py::mul(&v_a.clone(), &v_B.clone())?, &py::add(&v_k.clone(), &py::mul(&v_a.clone(), &v_B.clone())?)?)?;
    }
    return Ok(v_B.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__finite(mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t3663 in py::iter(&(v_vals.clone()))? {
        v_v = t3663;
        if (py::not(&{ let t3665 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3665.truthy() { t3665 } else { { let t3664 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t3664.truthy() { t3664 } else { py::math1("isfinite", &v_v.clone())? } } } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("every input must be a finite number"))));
        }
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fl(mut v_x: Py) -> R {
    return Ok({ let mut out = vec![]; for t4101 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t4101;
out.push(py::float(&v_v.clone())?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__flow2(mut v_seq: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_j: Py = Py::None;
    let mut v_t1: Py = Py::None;
    let mut v_t2: Py = Py::None;
    let t1141 = py::fraction(&[Py::Int(0i64)])?;
    v_t1 = t1141.clone();
    v_t2 = t1141.clone();
    for t1142 in py::iter(&(v_seq.clone()))? {
        v_j = t1142;
        v_t1 = py::iadd(&v_t1, &f__fr(py::getitem(&v_a.clone(), &v_j.clone())?)?)?;
        v_t2 = py::add(&py::minmax(&[v_t2.clone(), v_t1.clone()], true)?, &f__fr(py::getitem(&v_b.clone(), &v_j.clone())?)?)?;
    }
    return Ok(v_t2.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fr(mut v_v: Py) -> R {
    if (Py::Bool(matches!(v_v.clone(), Py::Frac(_)))).truthy() {
        return Ok(v_v.clone());
    }
    if (Py::Bool(matches!(v_v.clone(), Py::Bool(_)))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("expected a number"))));
    }
    if (Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_)))).truthy() {
        return Ok(py::fraction(&[v_v.clone()])?);
    }
    v_v = py::float(&v_v.clone())?;
    if (py::not(&py::math1("isfinite", &v_v.clone())?)).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("every number must be finite"))));
    }
    return Ok(py::fraction(&[Py::str(&py::repr(&v_v.clone()))])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__game_value(mut v_M: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_cw: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_ru: Py = Py::None;
    let mut v_vs: Py = Py::None;
    let t3937 = (Py::tuple(vec![py::len(&v_M.clone())?, py::len(&py::getitem(&v_M.clone(), &Py::Int(0i64))?)?])).items()?;
    if t3937.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t3937[0].clone();
    v_n = t3937[1].clone();
    v_k = py::sub(&Py::Int(1i64), &py::minmax(&[{ let mut out = vec![]; for t3938 in py::iter(&(v_M.clone()))? { let mut v_r: Py; 
v_r = t3938;
out.push(py::minmax(&[v_r.clone()], false)?); } Py::list(out) }], false)?)?;
    v_S = { let mut out = vec![]; for t3939 in py::iter(&(v_M.clone()))? { let mut v_r: Py; 
v_r = t3939;
out.push({ let mut out = vec![]; for t3940 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t3940;
out.push(py::add(&v_v.clone(), &v_k.clone())?); } Py::list(out) }); } Py::list(out) };
    v_ru = f__lp(Py::str("minimize"), py::mul(&Py::list(vec![Py::Int(1i64)]), &v_m.clone())?, { let mut out = vec![]; for t4095 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t4095;
out.push({ let mut out = vec![]; for t4096 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t4096;
out.push(py::getitem(&py::getitem(&v_S.clone(), &v_i.clone())?, &v_j.clone())?); } Py::list(out) }); } Py::list(out) }, py::mul(&Py::list(vec![Py::Int(1i64)]), &v_n.clone())?, py::mul(&Py::list(vec![Py::str(">=")]), &v_n.clone())?, Py::Bool(false))?;
    v_cw = f__lp(Py::str("maximize"), py::mul(&Py::list(vec![Py::Int(1i64)]), &v_n.clone())?, { let mut out = vec![]; for t4097 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t4097;
out.push(Py::list((py::getitem(&v_S.clone(), &v_i.clone())?).items()?)); } Py::list(out) }, py::mul(&Py::list(vec![Py::Int(1i64)]), &v_m.clone())?, py::mul(&Py::list(vec![Py::str("<=")]), &v_m.clone())?, Py::Bool(false))?;
    v_vs = py::truediv(&Py::Int(1i64), &py::getitem(&v_ru.clone(), &Py::str("value"))?)?;
    return Ok(Py::tuple(vec![py::sub(&v_vs.clone(), &v_k.clone())?, { let mut out = vec![]; for t4098 in py::iter(&(py::getitem(&v_ru.clone(), &Py::str("x"))?))? { let mut v_u: Py; 
v_u = t4098;
out.push(py::mul(&v_u.clone(), &v_vs.clone())?); } Py::list(out) }, { let mut out = vec![]; for t4099 in py::iter(&(py::getitem(&v_cw.clone(), &Py::str("x"))?))? { let mut v_w: Py; 
v_w = t4099;
out.push(py::mul(&v_w.clone(), &v_vs.clone())?); } Py::list(out) }]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__grad_res(mut v_Q: Py, mut v_c: Py, mut v_x: Py, mut v_extra: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_sc: Py = Py::None;
    v_n = py::len(&v_x.clone())?;
    v_g = { let mut out = vec![]; for t3656 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3656;
out.push(py::add(&py::add(&py::fsum(&{ let mut out = vec![]; for t3657 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3657;
out.push(py::mul(&py::float(&py::getitem(&py::getitem(&v_Q.clone(), &v_i.clone())?, &v_j.clone())?)?, &py::getitem(&v_x.clone(), &v_j.clone())?)?); } Py::list(out) })?, &py::float(&py::getitem(&v_c.clone(), &v_i.clone())?)?)?, &(if (v_extra.clone()).truthy() { py::getitem(&v_extra.clone(), &v_i.clone())? } else { Py::Float(0.0) }))?); } Py::list(out) };
    v_sc = py::mul(&py::minmax(&[py::add(&py::add(&Py::list(vec![Py::Float(1.0)]), &{ let mut out = vec![]; for t3658 in py::iter(&(v_Q.clone()))? { let mut v_r: Py; 
v_r = t3658;
for t3659 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t3659;
out.push(py::abs(&py::float(&v_v.clone())?)?); }} Py::list(out) })?, &{ let mut out = vec![]; for t3660 in py::iter(&(v_c.clone()))? { let mut v_v: Py; 
v_v = t3660;
out.push(py::abs(&py::float(&v_v.clone())?)?); } Py::list(out) })?], true)?, &py::minmax(&[py::add(&Py::list(vec![Py::Float(1.0)]), &{ let mut out = vec![]; for t3661 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t3661;
out.push(py::abs(&v_v.clone())?); } Py::list(out) })?], true)?)?;
    return Ok(py::truediv(&py::minmax(&[{ let mut out = vec![]; for t3662 in py::iter(&(v_g.clone()))? { let mut v_v: Py; 
v_v = t3662;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }], true)?, &v_sc.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__hungarian(mut v_C: Py) -> R {
    let mut v_INF: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_cur: Py = Py::None;
    let mut v_delta: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_i0: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_j0: Py = Py::None;
    let mut v_j1: Py = Py::None;
    let mut v_minv: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_used: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_way: Py = Py::None;
    v_n = py::len(&v_C.clone())?;
    v_INF = py::float(&Py::str("inf"))?;
    let t277 = (Py::tuple(vec![py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &py::add(&v_n.clone(), &Py::Int(1i64))?)?, py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &py::add(&v_n.clone(), &Py::Int(1i64))?)?, py::mul(&Py::list(vec![Py::Int(0i64)]), &py::add(&v_n.clone(), &Py::Int(1i64))?)?, py::mul(&Py::list(vec![Py::Int(0i64)]), &py::add(&v_n.clone(), &Py::Int(1i64))?)?])).items()?;
    if t277.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_u = t277[0].clone();
    v_v = t277[1].clone();
    v_p = t277[2].clone();
    v_way = t277[3].clone();
    for t278 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_i = t278;
        let t279 = (Py::tuple(vec![v_i.clone(), Py::Int(0i64)])).items()?;
        if t279.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_p.clone(), &Py::Int(0i64), t279[0].clone())?;
        v_j0 = t279[1].clone();
        let t280 = (Py::tuple(vec![py::mul(&Py::list(vec![v_INF.clone()]), &py::add(&v_n.clone(), &Py::Int(1i64))?)?, py::mul(&Py::list(vec![Py::Bool(false)]), &py::add(&v_n.clone(), &Py::Int(1i64))?)?])).items()?;
        if t280.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_minv = t280[0].clone();
        v_used = t280[1].clone();
        while (Py::Bool(true)).truthy() {
            py::setitem(&v_used.clone(), &v_j0.clone(), Py::Bool(true))?;
            let t281 = (Py::tuple(vec![py::getitem(&v_p.clone(), &v_j0.clone())?, v_INF.clone(), Py::Int(0i64)])).items()?;
            if t281.len() != 3 { return Err(PyErr::Value("unpack".into())); }
            v_i0 = t281[0].clone();
            v_delta = t281[1].clone();
            v_j1 = t281[2].clone();
            for t282 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
                v_j = t282;
                if (py::not(&py::getitem(&v_used.clone(), &v_j.clone())?)).truthy() {
                    v_cur = py::sub(&py::sub(&py::getitem(&py::getitem(&v_C.clone(), &py::sub(&v_i0.clone(), &Py::Int(1i64))?)?, &py::sub(&v_j.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_u.clone(), &v_i0.clone())?)?, &py::getitem(&v_v.clone(), &v_j.clone())?)?;
                    if ({ let t283 = v_cur.clone(); Py::Bool({ let t284 = py::getitem(&v_minv.clone(), &v_j.clone())?; if py::lt(&t283, &t284)? { true } else { false } }) }).truthy() {
                        let t285 = (Py::tuple(vec![v_cur.clone(), v_j0.clone()])).items()?;
                        if t285.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                        py::setitem(&v_minv.clone(), &v_j.clone(), t285[0].clone())?;
                        py::setitem(&v_way.clone(), &v_j.clone(), t285[1].clone())?;
                    }
                    if ({ let t286 = py::getitem(&v_minv.clone(), &v_j.clone())?; Py::Bool({ let t287 = v_delta.clone(); if py::lt(&t286, &t287)? { true } else { false } }) }).truthy() {
                        let t288 = (Py::tuple(vec![py::getitem(&v_minv.clone(), &v_j.clone())?, v_j.clone()])).items()?;
                        if t288.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                        v_delta = t288[0].clone();
                        v_j1 = t288[1].clone();
                    }
                }
            }
            for t289 in py::range_iter(&[(py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
                v_j = t289;
                if (py::getitem(&v_used.clone(), &v_j.clone())?).truthy() {
                    let t290 = v_u.clone(); let t291 = py::getitem(&v_p.clone(), &v_j.clone())?;
                    py::setitem(&t290, &t291, py::iadd(&py::getitem(&t290, &t291)?, &v_delta.clone())?)?;
                    let t292 = v_v.clone(); let t293 = v_j.clone();
                    py::setitem(&t292, &t293, py::sub(&py::getitem(&t292, &t293)?, &v_delta.clone())?)?;
                } else {
                    let t294 = v_minv.clone(); let t295 = v_j.clone();
                    py::setitem(&t294, &t295, py::sub(&py::getitem(&t294, &t295)?, &v_delta.clone())?)?;
                }
            }
            v_j0 = v_j1.clone();
            if ({ let t296 = py::getitem(&v_p.clone(), &v_j0.clone())?; Py::Bool({ let t297 = Py::Int(0i64); if py::eq(&t296, &t297) { true } else { false } }) }).truthy() {
                break;
            }
        }
        while (Py::Bool(true)).truthy() {
            v_j1 = py::getitem(&v_way.clone(), &v_j0.clone())?;
            py::setitem(&v_p.clone(), &v_j0.clone(), py::getitem(&v_p.clone(), &v_j1.clone())?)?;
            v_j0 = v_j1.clone();
            if ({ let t298 = v_j0.clone(); Py::Bool({ let t299 = Py::Int(0i64); if py::eq(&t298, &t299) { true } else { false } }) }).truthy() {
                break;
            }
        }
    }
    v_a = py::mul(&Py::list(vec![Py::Int(0i64)]), &v_n.clone())?;
    for t300 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_j = t300;
        py::setitem(&v_a.clone(), &py::sub(&py::getitem(&v_p.clone(), &v_j.clone())?, &Py::Int(1i64))?, py::sub(&v_j.clone(), &Py::Int(1i64))?)?;
    }
    return Ok(Py::tuple(vec![v_a.clone(), py::slice(&v_u.clone(), Some((Py::Int(1i64)).i()?), None, None)?, py::slice(&v_v.clone(), Some((Py::Int(1i64)).i()?), None, None)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__jobs(mut v_p: Py, mut v_name: Py) -> R {
    if ({ let t3854 = py::not(&v_p.clone()); if t3854.truthy() { t3854 } else { { let t3852 = py::len(&v_p.clone())?; Py::Bool({ let t3853 = Py::Int(12i64); if py::gt(&t3852, &t3853)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["give between 1 and 12 ".to_string(), py::format(&v_name.clone(), "")?].concat()))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t3855 in py::iter(&(v_p.clone()))? { let mut v_v: Py; 
v_v = t3855;
out.push({ let t3856 = v_v.clone(); Py::Bool({ let t3857 = Py::Int(0i64); if py::lt(&t3856, &t3857)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be >= 0".to_string()].concat()))));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__linear_norm(mut v_X: Py, mut v_types: Py) -> R {
    let mut v_cols: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let t3823 = (Py::tuple(vec![py::len(&v_X.clone())?, py::len(&py::getitem(&v_X.clone(), &Py::Int(0i64))?)?])).items()?;
    if t3823.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t3823[0].clone();
    v_k = t3823[1].clone();
    v_cols = { let mut out = vec![]; for t3824 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t3824;
out.push({ let mut out = vec![]; for t3825 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t3825;
out.push(py::getitem(&py::getitem(&v_X.clone(), &v_i.clone())?, &v_j.clone())?); } Py::list(out) }); } Py::list(out) };
    return Ok({ let mut out = vec![]; for t3826 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t3826;
out.push({ let mut out = vec![]; for t3827 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t3827;
out.push((if ({ let t3828 = py::getitem(&v_types.clone(), &v_j.clone())?; Py::Bool({ let t3829 = Py::Int(1i64); if py::eq(&t3828, &t3829) { true } else { false } }) }).truthy() { py::truediv(&py::getitem(&py::getitem(&v_X.clone(), &v_i.clone())?, &v_j.clone())?, &py::minmax(&[py::getitem(&v_cols.clone(), &v_j.clone())?], true)?)? } else { py::truediv(&py::minmax(&[py::getitem(&v_cols.clone(), &v_j.clone())?], false)?, &py::getitem(&py::getitem(&v_X.clone(), &v_i.clone())?, &v_j.clone())?)? })); } Py::list(out) }); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lp(mut v_sense: Py, mut v_c: Py, mut v_A: Py, mut v_b: Py, mut v_rel: Py, mut v_certify: Py) -> R {
    let mut v_As: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_basis: Py = Py::None;
    let mut v_bs: Py = Py::None;
    let mut v_cost: Py = Py::None;
    let mut v_cs: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_nonbasic: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_redundant: Py = Py::None;
    let mut v_rows: Py = Py::None;
    let mut v_sol: Py = Py::None;
    let mut v_st: Py = Py::None;
    let mut v_status: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_xa: Py = Py::None;
    let mut v_y: Py = Py::None;
    let mut v_zmin: Py = Py::None;
    let _ = f__check_lp(v_sense.clone(), v_c.clone(), v_A.clone(), v_b.clone(), v_rel.clone())?;
    let t3980 = (f__standard(v_sense.clone(), v_c.clone(), v_A.clone(), v_b.clone(), v_rel.clone())?).items()?;
    if t3980.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_As = t3980[0].clone();
    v_bs = t3980[1].clone();
    v_cs = t3980[2].clone();
    let t4052 = (f__simplex_eq(v_As.clone(), v_bs.clone(), v_cs.clone(), Py::Int(20000i64))?).items()?;
    if t4052.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_status = t4052[0].clone();
    v_x = t4052[1].clone();
    v_y = t4052[2].clone();
    v_basis = t4052[3].clone();
    v_redundant = t4052[4].clone();
    if ({ let t4053 = v_status.clone(); Py::Bool({ let t4054 = Py::str("infeasible"); if py::eq(&t4053, &t4054) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the constraints are infeasible: no point satisfies all of them"))));
    }
    if ({ let t4055 = v_status.clone(); Py::Bool({ let t4056 = Py::str("unbounded"); if py::eq(&t4055, &t4056) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the objective is unbounded on the feasible region: there is no finite optimum"))));
    }
    let t4057 = (Py::tuple(vec![py::len(&v_c.clone())?, py::len(&v_A.clone())?])).items()?;
    if t4057.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t4057[0].clone();
    v_m = t4057[1].clone();
    v_zmin = py::sum(&{ let mut out = vec![]; for t4058 in py::iter(&(py::zip(&[v_cs.clone(), v_x.clone()])?))? { let mut v_ci: Py; let mut v_xi: Py; 
let t4059 = (t4058).items()?;
if t4059.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_ci = t4059[0].clone();
v_xi = t4059[1].clone();
out.push(py::mul(&v_ci.clone(), &v_xi.clone())?); } Py::list(out) }, None)?;
    v_out = Py::dict(vec![(Py::str("value"), (if ({ let t4060 = v_sense.clone(); Py::Bool({ let t4061 = Py::str("minimize"); if py::eq(&t4060, &t4061) { true } else { false } }) }).truthy() { v_zmin.clone() } else { py::neg(&v_zmin.clone())? })), (Py::str("x"), py::slice(&v_x.clone(), None, Some((v_n.clone()).i()?), None)?), (Py::str("p"), { let mut out = vec![]; for t4062 in py::iter(&(v_y.clone()))? { let mut v_yi: Py; 
v_yi = t4062;
out.push((if ({ let t4063 = v_sense.clone(); Py::Bool({ let t4064 = Py::str("minimize"); if py::eq(&t4063, &t4064) { true } else { false } }) }).truthy() { v_yi.clone() } else { py::neg(&v_yi.clone())? })); } Py::list(out) })]);
    if (v_certify.clone()).truthy() {
        v_N = py::len(&v_cs.clone())?;
        v_nonbasic = { let mut out = vec![]; for t4065 in py::range_iter(&[(v_N.clone()).i()?]) { let mut v_j: Py; 
v_j = t4065;
if ({ let t4066 = v_j.clone(); Py::Bool({ let t4067 = v_basis.clone(); if !py::contains(&t4067, &t4066)? { true } else { false } }) }).truthy() { out.push(v_j.clone()); }} Py::list(out) };
        let t4071 = (f__simplex_eq(py::add(&v_As.clone(), &Py::list(vec![v_cs.clone()]))?, py::add(&v_bs.clone(), &Py::list(vec![v_zmin.clone()]))?, { let mut out = vec![]; for t4068 in py::range_iter(&[(v_N.clone()).i()?]) { let mut v_j: Py; 
v_j = t4068;
out.push((if ({ let t4069 = v_j.clone(); Py::Bool({ let t4070 = v_nonbasic.clone(); if py::contains(&t4070, &t4069)? { true } else { false } }) }).truthy() { py::fraction(&[py::neg(&Py::Int(1i64))?])? } else { py::fraction(&[Py::Int(0i64)])? })); } Py::list(out) }, Py::Int(20000i64))?).items()?;
        if t4071.len() != 5 { return Err(PyErr::Value("unpack".into())); }
        v_st = t4071[0].clone();
        v_xa = t4071[1].clone();
        v__ = t4071[2].clone();
        v__ = t4071[3].clone();
        v__ = t4071[4].clone();
        py::setitem(&v_out.clone(), &Py::str("x_unique"), { let t4077 = { let t4072 = v_st.clone(); Py::Bool({ let t4073 = Py::str("optimal"); if py::eq(&t4072, &t4073) { true } else { false } }) }; if !t4077.truthy() { t4077 } else { { let t4074 = py::sum(&{ let mut out = vec![]; for t4076 in py::iter(&(v_nonbasic.clone()))? { let mut v_j: Py; 
v_j = t4076;
out.push(py::getitem(&v_xa.clone(), &v_j.clone())?); } Py::list(out) }, None)?; Py::Bool({ let t4075 = Py::Int(0i64); if py::eq(&t4074, &t4075) { true } else { false } }) } } })?;
        if (v_redundant.clone()).truthy() {
            py::setitem(&v_out.clone(), &Py::str("p_unique"), Py::Bool(false))?;
        } else {
            v_rows = Py::list(vec![]);
            for t4078 in py::range_iter(&[(v_N.clone()).i()?]) {
                v_j = t4078;
                let _ = py::method(&v_rows.clone(), "append", &[py::add(&py::add(&{ let mut out = vec![]; for t4079 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t4079;
out.push(py::getitem(&py::getitem(&v_As.clone(), &v_i.clone())?, &v_j.clone())?); } Py::list(out) }, &{ let mut out = vec![]; for t4080 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t4080;
out.push(py::neg(&py::getitem(&py::getitem(&v_As.clone(), &v_i.clone())?, &v_j.clone())?)?); } Py::list(out) })?, &{ let mut out = vec![]; for t4081 in py::range_iter(&[(v_N.clone()).i()?]) { let mut v_k: Py; 
v_k = t4081;
out.push(py::fraction(&[py::int(&{ let t4082 = v_k.clone(); Py::Bool({ let t4083 = v_j.clone(); if py::eq(&t4082, &t4083) { true } else { false } }) })?])?); } Py::list(out) })?])?;
            }
            let _ = py::method(&v_rows.clone(), "append", &[py::add(&py::add(&v_bs.clone(), &{ let mut out = vec![]; for t4084 in py::iter(&(v_bs.clone()))? { let mut v_v: Py; 
v_v = t4084;
out.push(py::neg(&v_v.clone())?); } Py::list(out) })?, &py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &v_N.clone())?)?])?;
            v_cost = py::add(&py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &py::mul(&Py::Int(2i64), &v_m.clone())?)?, &{ let mut out = vec![]; for t4085 in py::range_iter(&[(v_N.clone()).i()?]) { let mut v_j: Py; 
v_j = t4085;
out.push((if ({ let t4086 = v_j.clone(); Py::Bool({ let t4087 = v_basis.clone(); if py::contains(&t4087, &t4086)? { true } else { false } }) }).truthy() { py::fraction(&[py::neg(&Py::Int(1i64))?])? } else { py::fraction(&[Py::Int(0i64)])? })); } Py::list(out) })?;
            let t4088 = (f__simplex_eq(v_rows.clone(), py::add(&v_cs.clone(), &Py::list(vec![v_zmin.clone()]))?, v_cost.clone(), Py::Int(20000i64))?).items()?;
            if t4088.len() != 5 { return Err(PyErr::Value("unpack".into())); }
            v_st = t4088[0].clone();
            v_sol = t4088[1].clone();
            v__ = t4088[2].clone();
            v__ = t4088[3].clone();
            v__ = t4088[4].clone();
            py::setitem(&v_out.clone(), &Py::str("p_unique"), { let t4094 = { let t4089 = v_st.clone(); Py::Bool({ let t4090 = Py::str("optimal"); if py::eq(&t4089, &t4090) { true } else { false } }) }; if !t4094.truthy() { t4094 } else { { let t4091 = py::sum(&{ let mut out = vec![]; for t4093 in py::iter(&(v_basis.clone()))? { let mut v_j: Py; 
v_j = t4093;
out.push(py::getitem(&v_sol.clone(), &py::add(&py::mul(&Py::Int(2i64), &v_m.clone())?, &v_j.clone())?)?); } Py::list(out) }, None)?; Py::Bool({ let t4092 = Py::Int(0i64); if py::eq(&t4091, &t4092) { true } else { false } }) } } })?;
        }
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lp2_vertices(mut v_c: Py, mut v_A: Py, mut v_b: Py, mut v_rel: Py) -> R {
    let mut v_a1: Py = Py::None;
    let mut v_a2: Py = Py::None;
    let mut v_b1: Py = Py::None;
    let mut v_b2: Py = Py::None;
    let mut v_c1: Py = Py::None;
    let mut v_c2: Py = Py::None;
    let mut v_cons: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_det: Py = Py::None;
    let mut v_feasible: Py = Py::None;
    let mut v_lines: Py = Py::None;
    let mut v_ok: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_rays: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    v_lines = { let mut out = vec![]; for t1714 in py::iter(&(py::zip(&[{ let mut out = vec![]; for t1711 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t1711;
out.push({ let mut out = vec![]; for t1712 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t1712;
out.push(f__fr(v_v.clone())?); } Py::list(out) }); } Py::list(out) }, { let mut out = vec![]; for t1713 in py::iter(&(v_b.clone()))? { let mut v_v: Py; 
v_v = t1713;
out.push(f__fr(v_v.clone())?); } Py::list(out) }])?))? { let mut v_bi: Py; let mut v_row: Py; 
let t1715 = (t1714).items()?;
if t1715.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_row = t1715[0].clone();
v_bi = t1715[1].clone();
out.push(Py::tuple(vec![py::fraction(&[py::getitem(&v_row.clone(), &Py::Int(0i64))?])?, py::fraction(&[py::getitem(&v_row.clone(), &Py::Int(1i64))?])?, py::fraction(&[v_bi.clone()])?])); } Py::list(out) };
    v_lines = py::iadd(&v_lines, &Py::list(vec![Py::tuple(vec![py::fraction(&[Py::Int(1i64)])?, py::fraction(&[Py::Int(0i64)])?, py::fraction(&[Py::Int(0i64)])?]), Py::tuple(vec![py::fraction(&[Py::Int(0i64)])?, py::fraction(&[Py::Int(1i64)])?, py::fraction(&[Py::Int(0i64)])?])]))?;
    v_cons = { let mut out = vec![]; for t1716 in py::iter(&(py::zip(&[v_A.clone(), v_b.clone(), v_rel.clone()])?))? { let mut v_bi: Py; let mut v_r: Py; let mut v_rl: Py; 
let t1717 = (t1716).items()?;
if t1717.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_r = t1717[0].clone();
v_bi = t1717[1].clone();
v_rl = t1717[2].clone();
out.push(Py::tuple(vec![f__fr(py::getitem(&v_r.clone(), &Py::Int(0i64))?)?, f__fr(py::getitem(&v_r.clone(), &Py::Int(1i64))?)?, f__fr(v_bi.clone())?, v_rl.clone()])); } Py::list(out) };
    v_feasible = { let v_A = v_A.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_cons = v_cons.clone(); let v_feasible = v_feasible.clone(); let v_lines = v_lines.clone(); let v_rel = v_rel.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_y: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_a1: Py = Py::None;
            let mut v_a2: Py = Py::None;
            let mut v_bb: Py = Py::None;
            let mut v_rl: Py = Py::None;
            let mut v_v: Py = Py::None;
            if ({ let t1722 = { let t1718 = v_x.clone(); Py::Bool({ let t1719 = Py::Int(0i64); if py::lt(&t1718, &t1719)? { true } else { false } }) }; if t1722.truthy() { t1722 } else { { let t1720 = v_y.clone(); Py::Bool({ let t1721 = Py::Int(0i64); if py::lt(&t1720, &t1721)? { true } else { false } }) } } }).truthy() {
                return Ok(Py::Bool(false));
            }
            for t1723 in py::iter(&(v_cons.clone()))? {
                let t1741 = (t1723).items()?;
                if t1741.len() != 4 { return Err(PyErr::Value("unpack".into())); }
                v_a1 = t1741[0].clone();
                v_a2 = t1741[1].clone();
                v_bb = t1741[2].clone();
                v_rl = t1741[3].clone();
                v_v = py::add(&py::mul(&v_a1.clone(), &v_x.clone())?, &py::mul(&v_a2.clone(), &v_y.clone())?)?;
                if ({ let t1740 = { let t1728 = { let t1724 = v_rl.clone(); Py::Bool({ let t1725 = Py::str("<="); if py::eq(&t1724, &t1725) { true } else { false } }) }; if !t1728.truthy() { t1728 } else { { let t1726 = v_v.clone(); Py::Bool({ let t1727 = v_bb.clone(); if py::gt(&t1726, &t1727)? { true } else { false } }) } } }; if t1740.truthy() { t1740 } else { { let t1739 = { let t1733 = { let t1729 = v_rl.clone(); Py::Bool({ let t1730 = Py::str(">="); if py::eq(&t1729, &t1730) { true } else { false } }) }; if !t1733.truthy() { t1733 } else { { let t1731 = v_v.clone(); Py::Bool({ let t1732 = v_bb.clone(); if py::lt(&t1731, &t1732)? { true } else { false } }) } } }; if t1739.truthy() { t1739 } else { { let t1738 = { let t1734 = v_rl.clone(); Py::Bool({ let t1735 = Py::str("="); if py::eq(&t1734, &t1735) { true } else { false } }) }; if !t1738.truthy() { t1738 } else { { let t1736 = v_v.clone(); Py::Bool({ let t1737 = v_bb.clone(); if py::ne(&t1736, &t1737) { true } else { false } }) } } } } } } }).truthy() {
                    return Ok(Py::Bool(false));
                }
            }
            return Ok(Py::Bool(true));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_pts = py::set(&Py::list(vec![]))?;
    for t1742 in py::iter(&(py::itertools("combinations", &[v_lines.clone(), Py::Int(2i64)])?))? {
        let t1746 = (t1742).items()?;
        if t1746.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        let t1747 = (t1746[0].clone()).items()?;
        if t1747.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_a1 = t1747[0].clone();
        v_a2 = t1747[1].clone();
        v_b1 = t1747[2].clone();
        let t1748 = (t1746[1].clone()).items()?;
        if t1748.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_c1 = t1748[0].clone();
        v_c2 = t1748[1].clone();
        v_b2 = t1748[2].clone();
        v_det = py::sub(&py::mul(&v_a1.clone(), &v_c2.clone())?, &py::mul(&v_a2.clone(), &v_c1.clone())?)?;
        if ({ let t1743 = v_det.clone(); Py::Bool({ let t1744 = Py::Int(0i64); if py::eq(&t1743, &t1744) { true } else { false } }) }).truthy() {
            continue;
        }
        let t1745 = (Py::tuple(vec![py::truediv(&py::sub(&py::mul(&v_b1.clone(), &v_c2.clone())?, &py::mul(&v_a2.clone(), &v_b2.clone())?)?, &v_det.clone())?, py::truediv(&py::sub(&py::mul(&v_a1.clone(), &v_b2.clone())?, &py::mul(&v_b1.clone(), &v_c1.clone())?)?, &v_det.clone())?])).items()?;
        if t1745.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_x = t1745[0].clone();
        v_y = t1745[1].clone();
        if (py::call(&v_feasible.clone(), &[v_x.clone(), v_y.clone()])?).truthy() {
            let _ = py::method(&v_pts.clone(), "add", &[Py::tuple(vec![v_x.clone(), v_y.clone()])])?;
        }
    }
    v_rays = Py::list(vec![]);
    for t1753 in py::iter(&(py::add(&py::add(&Py::list(vec![Py::tuple(vec![py::fraction(&[Py::Int(1i64)])?, py::fraction(&[Py::Int(0i64)])?]), Py::tuple(vec![py::fraction(&[Py::Int(0i64)])?, py::fraction(&[Py::Int(1i64)])?])]), &{ let mut out = vec![]; for t1749 in py::iter(&(v_cons.clone()))? { let mut v__: Py; let mut v_a1: Py; let mut v_a2: Py; 
let t1750 = (t1749).items()?;
if t1750.len() != 4 { return Err(PyErr::Value("unpack".into())); }
v_a1 = t1750[0].clone();
v_a2 = t1750[1].clone();
v__ = t1750[2].clone();
v__ = t1750[3].clone();
out.push(Py::tuple(vec![v_a2.clone(), py::neg(&v_a1.clone())?])); } Py::list(out) })?, &{ let mut out = vec![]; for t1751 in py::iter(&(v_cons.clone()))? { let mut v__: Py; let mut v_a1: Py; let mut v_a2: Py; 
let t1752 = (t1751).items()?;
if t1752.len() != 4 { return Err(PyErr::Value("unpack".into())); }
v_a1 = t1752[0].clone();
v_a2 = t1752[1].clone();
v__ = t1752[2].clone();
v__ = t1752[3].clone();
out.push(Py::tuple(vec![py::neg(&v_a2.clone())?, v_a1.clone()])); } Py::list(out) })?))? {
        v_d = t1753;
        if ({ let t1761 = { let t1754 = v_d.clone(); Py::Bool({ let t1755 = Py::tuple(vec![Py::Int(0i64), Py::Int(0i64)]); if py::eq(&t1754, &t1755) { true } else { false } }) }; if t1761.truthy() { t1761 } else { { let t1760 = { let t1756 = py::getitem(&v_d.clone(), &Py::Int(0i64))?; Py::Bool({ let t1757 = Py::Int(0i64); if py::lt(&t1756, &t1757)? { true } else { false } }) }; if t1760.truthy() { t1760 } else { { let t1758 = py::getitem(&v_d.clone(), &Py::Int(1i64))?; Py::Bool({ let t1759 = Py::Int(0i64); if py::lt(&t1758, &t1759)? { true } else { false } }) } } } } }).truthy() {
            continue;
        }
        v_ok = Py::Bool(({ let mut out = vec![]; for t1762 in py::iter(&(v_cons.clone()))? { let mut v__: Py; let mut v_a1: Py; let mut v_a2: Py; let mut v_rl: Py; 
let t1763 = (t1762).items()?;
if t1763.len() != 4 { return Err(PyErr::Value("unpack".into())); }
v_a1 = t1763[0].clone();
v_a2 = t1763[1].clone();
v__ = t1763[2].clone();
v_rl = t1763[3].clone();
out.push(py::not(&{ let t1780 = { let t1768 = { let t1764 = v_rl.clone(); Py::Bool({ let t1765 = Py::str("<="); if py::eq(&t1764, &t1765) { true } else { false } }) }; if !t1768.truthy() { t1768 } else { { let t1766 = py::add(&py::mul(&v_a1.clone(), &py::getitem(&v_d.clone(), &Py::Int(0i64))?)?, &py::mul(&v_a2.clone(), &py::getitem(&v_d.clone(), &Py::Int(1i64))?)?)?; Py::Bool({ let t1767 = Py::Int(0i64); if py::gt(&t1766, &t1767)? { true } else { false } }) } } }; if t1780.truthy() { t1780 } else { { let t1779 = { let t1773 = { let t1769 = v_rl.clone(); Py::Bool({ let t1770 = Py::str(">="); if py::eq(&t1769, &t1770) { true } else { false } }) }; if !t1773.truthy() { t1773 } else { { let t1771 = py::add(&py::mul(&v_a1.clone(), &py::getitem(&v_d.clone(), &Py::Int(0i64))?)?, &py::mul(&v_a2.clone(), &py::getitem(&v_d.clone(), &Py::Int(1i64))?)?)?; Py::Bool({ let t1772 = Py::Int(0i64); if py::lt(&t1771, &t1772)? { true } else { false } }) } } }; if t1779.truthy() { t1779 } else { { let t1778 = { let t1774 = v_rl.clone(); Py::Bool({ let t1775 = Py::str("="); if py::eq(&t1774, &t1775) { true } else { false } }) }; if !t1778.truthy() { t1778 } else { { let t1776 = py::add(&py::mul(&v_a1.clone(), &py::getitem(&v_d.clone(), &Py::Int(0i64))?)?, &py::mul(&v_a2.clone(), &py::getitem(&v_d.clone(), &Py::Int(1i64))?)?)?; Py::Bool({ let t1777 = Py::Int(0i64); if py::ne(&t1776, &t1777) { true } else { false } }) } } } } } } })); } Py::list(out) }).items()?.iter().all(|x| x.truthy()));
        if (v_ok.clone()).truthy() {
            let _ = py::method(&v_rays.clone(), "append", &[v_d.clone()])?;
        }
    }
    return Ok(Py::tuple(vec![py::sorted(&v_pts.clone(), (Py::Bool(false)).truthy())?, v_rays.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lp_certificate_residual(mut v_inputs: Py, mut v_x: Py, mut v_p: Py, mut v_value: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_ax: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_big: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_cm: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_rc: Py = Py::None;
    let mut v_rel: Py = Py::None;
    let mut v_row: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_sense: Py = Py::None;
    let mut v_sv: Py = Py::None;
    let mut v_viol: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let mut v_xs: Py = Py::None;
    let mut v_y: Py = Py::None;
    let mut v_zx: Py = Py::None;
    let mut v_zy: Py = Py::None;
    let t3515 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("sense"))?, py::getitem(&v_inputs.clone(), &Py::str("c"))?, py::getitem(&v_inputs.clone(), &Py::str("A"))?, py::getitem(&v_inputs.clone(), &Py::str("b"))?])).items()?;
    if t3515.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_sense = t3515[0].clone();
    v_c = t3515[1].clone();
    v_A = t3515[2].clone();
    v_b = t3515[3].clone();
    v_rel = f__relations(py::method(&v_inputs.clone(), "get", &[Py::str("relations")])?, py::len(&v_A.clone())?)?;
    if ({ let t3532 = { let t3528 = py::len(&v_p.clone())?; Py::Bool({ let t3529 = py::len(&v_A.clone())?; if py::ne(&t3528, &t3529) { true } else { false } }) }; if t3532.truthy() { t3532 } else { { let t3530 = py::len(&v_x.clone())?; Py::Bool({ let t3531 = py::len(&v_c.clone())?; if py::ne(&t3530, &t3531) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_s = (if ({ let t3533 = v_sense.clone(); Py::Bool({ let t3534 = Py::str("minimize"); if py::eq(&t3533, &t3534) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { py::neg(&Py::Float(1.0))? });
    v_cm = { let mut out = vec![]; for t3535 in py::iter(&(v_c.clone()))? { let mut v_v: Py; 
v_v = t3535;
out.push(py::mul(&v_s.clone(), &v_v.clone())?); } Py::list(out) };
    v_y = { let mut out = vec![]; for t3536 in py::iter(&(v_p.clone()))? { let mut v_v: Py; 
v_v = t3536;
out.push(py::mul(&v_s.clone(), &v_v.clone())?); } Py::list(out) };
    v_scale = py::minmax(&[py::add(&py::add(&py::add(&Py::list(vec![Py::Float(1.0)]), &{ let mut out = vec![]; for t3537 in py::iter(&(v_c.clone()))? { let mut v_v: Py; 
v_v = t3537;
out.push(py::abs(&v_v.clone())?); } Py::list(out) })?, &{ let mut out = vec![]; for t3538 in py::iter(&(v_b.clone()))? { let mut v_v: Py; 
v_v = t3538;
out.push(py::abs(&v_v.clone())?); } Py::list(out) })?, &{ let mut out = vec![]; for t3539 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t3539;
for t3540 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t3540;
out.push(py::abs(&v_v.clone())?); }} Py::list(out) })?], true)?;
    v_xs = py::minmax(&[py::add(&Py::list(vec![Py::Float(1.0)]), &{ let mut out = vec![]; for t3541 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t3541;
out.push(py::abs(&v_v.clone())?); } Py::list(out) })?], true)?;
    v_worst = Py::Float(0.0);
    for t3542 in py::iter(&(py::enumerate(&v_A.clone(), 0)?))? {
        let t3545 = (t3542).items()?;
        if t3545.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t3545[0].clone();
        v_row = t3545[1].clone();
        v_ax = py::sum(&{ let mut out = vec![]; for t3543 in py::iter(&(py::zip(&[v_row.clone(), v_x.clone()])?))? { let mut v_a: Py; let mut v_xi: Py; 
let t3544 = (t3543).items()?;
if t3544.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3544[0].clone();
v_xi = t3544[1].clone();
out.push(py::mul(&v_a.clone(), &v_xi.clone())?); } Py::list(out) }, None)?;
        v_viol = py::getitem(&Py::dict(vec![(Py::str("<="), py::minmax(&[Py::Float(0.0), py::sub(&v_ax.clone(), &py::getitem(&v_b.clone(), &v_i.clone())?)?], true)?), (Py::str(">="), py::minmax(&[Py::Float(0.0), py::sub(&py::getitem(&v_b.clone(), &v_i.clone())?, &v_ax.clone())?], true)?), (Py::str("="), py::abs(&py::sub(&v_ax.clone(), &py::getitem(&v_b.clone(), &v_i.clone())?)?)?)]), &py::getitem(&v_rel.clone(), &v_i.clone())?)?;
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&v_viol.clone(), &py::mul(&v_scale.clone(), &v_xs.clone())?)?], true)?;
        v_sv = py::getitem(&Py::dict(vec![(Py::str("<="), py::minmax(&[Py::Float(0.0), py::getitem(&v_y.clone(), &v_i.clone())?], true)?), (Py::str(">="), py::minmax(&[Py::Float(0.0), py::neg(&py::getitem(&v_y.clone(), &v_i.clone())?)?], true)?), (Py::str("="), Py::Float(0.0))]), &py::getitem(&v_rel.clone(), &v_i.clone())?)?;
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&v_sv.clone(), &v_scale.clone())?], true)?;
    }
    v_worst = py::minmax(&[py::add(&Py::list(vec![v_worst.clone()]), &{ let mut out = vec![]; for t3546 in py::iter(&(v_x.clone()))? { let mut v_xi: Py; 
v_xi = t3546;
out.push(py::truediv(&py::minmax(&[Py::Float(0.0), py::neg(&v_xi.clone())?], true)?, &v_xs.clone())?); } Py::list(out) })?], true)?;
    for t3547 in py::range_iter(&[(py::len(&v_c.clone())?).i()?]) {
        v_j = t3547;
        v_rc = py::sub(&py::getitem(&v_cm.clone(), &v_j.clone())?, &py::sum(&{ let mut out = vec![]; for t3548 in py::range_iter(&[(py::len(&v_A.clone())?).i()?]) { let mut v_i: Py; 
v_i = t3548;
out.push(py::mul(&py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_y.clone(), &v_i.clone())?)?); } Py::list(out) }, None)?)?;
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::minmax(&[Py::Float(0.0), py::neg(&v_rc.clone())?], true)?, &py::mul(&v_scale.clone(), &py::minmax(&[Py::Float(1.0), (if (v_y.clone()).truthy() { py::minmax(&[{ let mut out = vec![]; for t3549 in py::iter(&(v_y.clone()))? { let mut v_v: Py; 
v_v = t3549;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }], true)? } else { Py::Float(1.0) })], true)?)?)?], true)?;
    }
    v_zx = py::sum(&{ let mut out = vec![]; for t3550 in py::iter(&(py::zip(&[v_c.clone(), v_x.clone()])?))? { let mut v_ci: Py; let mut v_xi: Py; 
let t3551 = (t3550).items()?;
if t3551.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_ci = t3551[0].clone();
v_xi = t3551[1].clone();
out.push(py::mul(&v_ci.clone(), &v_xi.clone())?); } Py::list(out) }, None)?;
    v_zy = py::mul(&v_s.clone(), &py::sum(&{ let mut out = vec![]; for t3552 in py::iter(&(py::zip(&[v_b.clone(), v_y.clone()])?))? { let mut v_bi: Py; let mut v_yi: Py; 
let t3553 = (t3552).items()?;
if t3553.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_bi = t3553[0].clone();
v_yi = t3553[1].clone();
out.push(py::mul(&v_bi.clone(), &v_yi.clone())?); } Py::list(out) }, None)?)?;
    v_big = py::minmax(&[Py::Float(1.0), py::abs(&v_value.clone())?, py::abs(&v_zx.clone())?, py::abs(&v_zy.clone())?], true)?;
    return Ok(py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&v_zx.clone(), &v_value.clone())?)?, &v_big.clone())?, py::truediv(&py::abs(&py::sub(&v_zy.clone(), &v_value.clone())?)?, &v_big.clone())?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lp_outputs(mut v_r: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = Py::dict(vec![(Py::str("optimal_value"), py::float(&py::getitem(&v_r.clone(), &Py::str("value"))?)?)]);
    py::setitem(&v_out.clone(), &(if (py::getitem(&v_r.clone(), &Py::str("x_unique"))?).truthy() { Py::str("x") } else { Py::str("x_one_optimal") }), f__fl(py::getitem(&v_r.clone(), &Py::str("x"))?)?)?;
    py::setitem(&v_out.clone(), &Py::str("solution_unique"), (if (py::getitem(&v_r.clone(), &Py::str("x_unique"))?).truthy() { Py::str("yes") } else { Py::str("no") }))?;
    py::setitem(&v_out.clone(), &(if (py::getitem(&v_r.clone(), &Py::str("p_unique"))?).truthy() { Py::str("shadow_prices") } else { Py::str("dual_one_optimal") }), f__fl(py::getitem(&v_r.clone(), &Py::str("p"))?)?)?;
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mcda_in(mut v_X: Py, mut v_w: Py, mut v_types: Py) -> R {
    let mut v_W: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_s: Py = Py::None;
    if ({ let t3792 = py::not(&v_X.clone()); if t3792.truthy() { t3792 } else { Py::Bool(({ let mut out = vec![]; for t3789 in py::iter(&(v_X.clone()))? { let mut v_r: Py; 
v_r = t3789;
out.push({ let t3790 = py::len(&v_r.clone())?; Py::Bool({ let t3791 = py::len(&py::getitem(&v_X.clone(), &Py::Int(0i64))?)?; if py::ne(&t3790, &t3791) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the decision matrix must be rectangular (rows = alternatives, columns = criteria)"))));
    }
    let t3793 = (Py::tuple(vec![py::len(&v_X.clone())?, py::len(&py::getitem(&v_X.clone(), &Py::Int(0i64))?)?])).items()?;
    if t3793.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t3793[0].clone();
    v_k = t3793[1].clone();
    if ({ let t3801 = { let t3794 = v_m.clone(); Py::Bool({ let t3795 = Py::Int(2i64); if py::lt(&t3794, &t3795)? { true } else { false } }) }; if t3801.truthy() { t3801 } else { { let t3800 = { let t3796 = v_m.clone(); Py::Bool({ let t3797 = Py::Int(40i64); if py::gt(&t3796, &t3797)? { true } else { false } }) }; if t3800.truthy() { t3800 } else { { let t3798 = v_k.clone(); Py::Bool({ let t3799 = Py::Int(15i64); if py::gt(&t3798, &t3799)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give 2 to 40 alternatives and at most 15 criteria"))));
    }
    if ({ let t3806 = { let t3802 = py::len(&v_w.clone())?; Py::Bool({ let t3803 = v_k.clone(); if py::ne(&t3802, &t3803) { true } else { false } }) }; if t3806.truthy() { t3806 } else { { let t3804 = py::len(&v_types.clone())?; Py::Bool({ let t3805 = v_k.clone(); if py::ne(&t3804, &t3805) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["give one weight and one type per criterion (".to_string(), py::format(&v_k.clone(), "")?, ")".to_string()].concat()))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t3807 in py::iter(&(v_X.clone()))? { let mut v_r: Py; 
v_r = t3807;
for t3808 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t3808;
out.push({ let t3809 = v_v.clone(); Py::Bool({ let t3810 = Py::Int(0i64); if py::le(&t3809, &t3810)? { true } else { false } }) }); }} Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("every value of the decision matrix must be > 0"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t3811 in py::iter(&(v_w.clone()))? { let mut v_v: Py; 
v_v = t3811;
out.push({ let t3812 = v_v.clone(); Py::Bool({ let t3813 = Py::Int(0i64); if py::le(&t3812, &t3813)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("every weight must be > 0"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t3814 in py::iter(&(v_types.clone()))? { let mut v_t: Py; 
v_t = t3814;
out.push({ let t3815 = v_t.clone(); Py::Bool({ let t3816 = Py::tuple(vec![Py::Int(1i64), py::neg(&Py::Int(1i64))?]); if !py::contains(&t3816, &t3815)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("criterion types are 1 (benefit: larger is better) or -1 (cost: smaller is better)"))));
    }
    v_W = { let mut out = vec![]; for t3817 in py::iter(&(v_w.clone()))? { let mut v_v: Py; 
v_v = t3817;
out.push(f__fr(v_v.clone())?); } Py::list(out) };
    v_s = py::sum(&v_W.clone(), None)?;
    return Ok(Py::tuple(vec![{ let mut out = vec![]; for t3818 in py::iter(&(v_X.clone()))? { let mut v_r: Py; 
v_r = t3818;
out.push({ let mut out = vec![]; for t3819 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t3819;
out.push(py::float(&v_v.clone())?); } Py::list(out) }); } Py::list(out) }, { let mut out = vec![]; for t3820 in py::iter(&(v_W.clone()))? { let mut v_v: Py; 
v_v = t3820;
out.push(py::float(&py::truediv(&v_v.clone(), &v_s.clone())?)?); } Py::list(out) }, { let mut out = vec![]; for t3821 in py::iter(&(v_types.clone()))? { let mut v_t: Py; 
v_t = t3821;
out.push(py::int(&v_t.clone())?); } Py::list(out) }]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mcda_out(mut v_scores: Py) -> R {
    return Ok(Py::dict(vec![(Py::str("scores"), v_scores.clone()), (Py::str("best_score"), py::minmax(&[v_scores.clone()], true)?), (Py::str("best"), py::float(&f__best(v_scores.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mcf_lp(mut v_capacity: Py, mut v_cost: Py, mut v_supply: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_arcs: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_rel: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_n = py::len(&v_supply.clone())?;
    if ({ let t2095 = { let t2087 = py::len(&v_capacity.clone())?; Py::Bool({ let t2088 = v_n.clone(); if py::ne(&t2087, &t2088) { true } else { false } }) }; if t2095.truthy() { t2095 } else { { let t2094 = { let t2089 = py::len(&v_cost.clone())?; Py::Bool({ let t2090 = v_n.clone(); if py::ne(&t2089, &t2090) { true } else { false } }) }; if t2094.truthy() { t2094 } else { Py::Bool(({ let mut out = vec![]; for t2091 in py::iter(&(py::add(&v_capacity.clone(), &v_cost.clone())?))? { let mut v_r: Py; 
v_r = t2091;
out.push({ let t2092 = py::len(&v_r.clone())?; Py::Bool({ let t2093 = v_n.clone(); if py::ne(&t2092, &t2093) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["capacity and cost must be ".to_string(), py::format(&v_n.clone(), "")?, " x ".to_string(), py::format(&v_n.clone(), "")?, " matrices (one row and column per node)".to_string()].concat()))));
    }
    if ({ let t2096 = py::sum(&{ let mut out = vec![]; for t2098 in py::iter(&(v_supply.clone()))? { let mut v_v: Py; 
v_v = t2098;
out.push(f__fr(v_v.clone())?); } Py::list(out) }, None)?; Py::Bool({ let t2097 = Py::Int(0i64); if py::ne(&t2096, &t2097) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("supplies (positive) and demands (negative) must sum to 0"))));
    }
    v_arcs = { let mut out = vec![]; for t2099 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t2099;
for t2100 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t2100;
if ({ let t2105 = { let t2101 = v_i.clone(); Py::Bool({ let t2102 = v_j.clone(); if py::ne(&t2101, &t2102) { true } else { false } }) }; if !t2105.truthy() { t2105 } else { { let t2103 = py::getitem(&py::getitem(&v_capacity.clone(), &v_i.clone())?, &v_j.clone())?; Py::Bool({ let t2104 = Py::Int(0i64); if py::gt(&t2103, &t2104)? { true } else { false } }) } } }).truthy() { out.push(Py::tuple(vec![v_i.clone(), v_j.clone()])); }}} Py::list(out) };
    if ({ let t2108 = py::not(&v_arcs.clone()); if t2108.truthy() { t2108 } else { { let t2106 = py::len(&v_arcs.clone())?; Py::Bool({ let t2107 = Py::Int(60i64); if py::gt(&t2106, &t2107)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give between 1 and 60 arcs (positive capacities)"))));
    }
    v_c = { let mut out = vec![]; for t2109 in py::iter(&(v_arcs.clone()))? { let mut v_i: Py; let mut v_j: Py; 
let t2110 = (t2109).items()?;
if t2110.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t2110[0].clone();
v_j = t2110[1].clone();
out.push(py::getitem(&py::getitem(&v_cost.clone(), &v_i.clone())?, &v_j.clone())?); } Py::list(out) };
    let t2111 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![]), Py::list(vec![])])).items()?;
    if t2111.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t2111[0].clone();
    v_b = t2111[1].clone();
    v_rel = t2111[2].clone();
    for t2112 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_v = t2112;
        let _ = py::method(&v_A.clone(), "append", &[{ let mut out = vec![]; for t2113 in py::iter(&(v_arcs.clone()))? { let mut v_i: Py; let mut v_j: Py; 
let t2114 = (t2113).items()?;
if t2114.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t2114[0].clone();
v_j = t2114[1].clone();
out.push(py::sub(&(if ({ let t2115 = v_i.clone(); Py::Bool({ let t2116 = v_v.clone(); if py::eq(&t2115, &t2116) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { Py::Int(0i64) }), &(if ({ let t2117 = v_j.clone(); Py::Bool({ let t2118 = v_v.clone(); if py::eq(&t2117, &t2118) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { Py::Int(0i64) }))?); } Py::list(out) }])?;
        let _ = py::method(&v_b.clone(), "append", &[py::getitem(&v_supply.clone(), &v_v.clone())?])?;
        let _ = py::method(&v_rel.clone(), "append", &[Py::str("=")])?;
    }
    for t2119 in py::iter(&(py::enumerate(&v_arcs.clone(), 0)?))? {
        let t2123 = (t2119).items()?;
        if t2123.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2123[0].clone();
        let t2124 = (t2123[1].clone()).items()?;
        if t2124.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t2124[0].clone();
        v_j = t2124[1].clone();
        let _ = py::method(&v_A.clone(), "append", &[{ let mut out = vec![]; for t2120 in py::range_iter(&[(py::len(&v_arcs.clone())?).i()?]) { let mut v_q: Py; 
v_q = t2120;
out.push((if ({ let t2121 = v_q.clone(); Py::Bool({ let t2122 = v_k.clone(); if py::eq(&t2121, &t2122) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { Py::Int(0i64) })); } Py::list(out) }])?;
        let _ = py::method(&v_b.clone(), "append", &[py::getitem(&py::getitem(&v_capacity.clone(), &v_i.clone())?, &v_j.clone())?])?;
        let _ = py::method(&v_rel.clone(), "append", &[Py::str("<=")])?;
    }
    return Ok(Py::tuple(vec![v_arcs.clone(), Py::tuple(vec![Py::str("minimize"), v_c.clone(), v_A.clone(), v_b.clone(), v_rel.clone()])]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mdp_in(mut v_R: Py, mut v_T: Py, mut v_gamma: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_fr: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_s: Py = Py::None;
    let _ = f__finite(Py::tuple(vec![v_gamma.clone()]))?;
    if (py::not(&{ let t1990 = Py::Int(0i64); Py::Bool({ let t1991 = v_gamma.clone(); if py::lt(&t1990, &t1991)? { { let t1992 = Py::Int(1i64); if py::lt(&t1991, &t1992)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the discount factor must be between 0 and 1 (exclusive)"))));
    }
    let t1993 = (Py::tuple(vec![py::len(&v_R.clone())?, (if (v_R.clone()).truthy() { py::len(&py::getitem(&v_R.clone(), &Py::Int(0i64))?)? } else { Py::Int(0i64) })])).items()?;
    if t1993.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_S = t1993[0].clone();
    v_A = t1993[1].clone();
    if ({ let t2008 = { let t1994 = v_S.clone(); Py::Bool({ let t1995 = Py::Int(1i64); if py::lt(&t1994, &t1995)? { true } else { false } }) }; if t2008.truthy() { t2008 } else { { let t2007 = { let t1996 = v_S.clone(); Py::Bool({ let t1997 = Py::Int(10i64); if py::gt(&t1996, &t1997)? { true } else { false } }) }; if t2007.truthy() { t2007 } else { { let t2006 = { let t1998 = v_A.clone(); Py::Bool({ let t1999 = Py::Int(1i64); if py::lt(&t1998, &t1999)? { true } else { false } }) }; if t2006.truthy() { t2006 } else { { let t2005 = { let t2000 = v_A.clone(); Py::Bool({ let t2001 = Py::Int(5i64); if py::gt(&t2000, &t2001)? { true } else { false } }) }; if t2005.truthy() { t2005 } else { Py::Bool(({ let mut out = vec![]; for t2002 in py::iter(&(v_R.clone()))? { let mut v_r: Py; 
v_r = t2002;
out.push({ let t2003 = py::len(&v_r.clone())?; Py::Bool({ let t2004 = v_A.clone(); if py::ne(&t2003, &t2004) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } } } } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("rewards: one row per state (at most 10), one column per action (at most 5)"))));
    }
    if ({ let t2014 = { let t2009 = py::len(&v_T.clone())?; Py::Bool({ let t2010 = py::mul(&v_S.clone(), &v_A.clone())?; if py::ne(&t2009, &t2010) { true } else { false } }) }; if t2014.truthy() { t2014 } else { Py::Bool(({ let mut out = vec![]; for t2011 in py::iter(&(v_T.clone()))? { let mut v_r: Py; 
v_r = t2011;
out.push({ let t2012 = py::len(&v_r.clone())?; Py::Bool({ let t2013 = v_S.clone(); if py::ne(&t2012, &t2013) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["transitions: ".to_string(), py::format(&py::mul(&v_S.clone(), &v_A.clone())?, "")?, " rows (state 1 action 1, state 1 action 2, ...) of ".to_string(), py::format(&v_S.clone(), "")?, " probabilities".to_string()].concat()))));
    }
    v_P = Py::list(vec![]);
    for t2015 in py::iter(&(v_T.clone()))? {
        v_r = t2015;
        v_fr = { let mut out = vec![]; for t2016 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t2016;
out.push(f__fr(v_v.clone())?); } Py::list(out) };
        v_s = py::sum(&v_fr.clone(), None)?;
        if ({ let t2022 = Py::Bool(({ let mut out = vec![]; for t2017 in py::iter(&(v_fr.clone()))? { let mut v_v: Py; 
v_v = t2017;
out.push({ let t2018 = v_v.clone(); Py::Bool({ let t2019 = Py::Int(0i64); if py::lt(&t2018, &t2019)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t2022.truthy() { t2022 } else { { let t2020 = py::abs(&py::sub(&v_s.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t2021 = py::fraction(&[Py::Int(1i64), py::pow(&Py::Int(10i64), &Py::Int(9i64))?])?; if py::gt(&t2020, &t2021)? { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("every transition row must be probabilities summing to 1"))));
        }
        let _ = py::method(&v_P.clone(), "append", &[{ let mut out = vec![]; for t2023 in py::iter(&(v_fr.clone()))? { let mut v_v: Py; 
v_v = t2023;
out.push(py::truediv(&v_v.clone(), &v_s.clone())?); } Py::list(out) }])?;
    }
    return Ok(Py::tuple(vec![{ let mut out = vec![]; for t2024 in py::iter(&(v_R.clone()))? { let mut v_r: Py; 
v_r = t2024;
out.push({ let mut out = vec![]; for t2025 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t2025;
out.push(f__fr(v_v.clone())?); } Py::list(out) }); } Py::list(out) }, v_P.clone(), f__fr(v_gamma.clone())?, v_S.clone(), v_A.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nodes(mut v_edges: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_out = Py::list(vec![]);
    for t3075 in py::iter(&(v_edges.clone()))? {
        v_e = t3075;
        for t3076 in py::iter(&(py::slice(&v_e.clone(), None, Some((Py::Int(2i64)).i()?), None)?))? {
            v_v = t3076;
            if ({ let t3077 = v_v.clone(); Py::Bool({ let t3078 = v_out.clone(); if !py::contains(&t3078, &t3077)? { true } else { false } }) }).truthy() {
                let _ = py::method(&v_out.clone(), "append", &[v_v.clone()])?;
            }
        }
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nv_check(mut v_p: Py, mut v_c: Py, mut v_s: Py) -> R {
    let _ = f__finite(Py::tuple(vec![v_p.clone(), v_c.clone(), v_s.clone()]))?;
    if (py::not(&{ let t2701 = v_s.clone(); Py::Bool({ let t2702 = v_c.clone(); if py::lt(&t2701, &t2702)? { { let t2703 = v_p.clone(); if py::lt(&t2702, &t2703)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the salvage value, unit cost and price must satisfy salvage < cost < price"))));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nv_disc_profit(mut v_p: Py, mut v_c: Py, mut v_s: Py, mut v_demands: Py, mut v_probs: Py, mut v_Q: Py) -> R {
    return Ok(py::sum(&{ let mut out = vec![]; for t2686 in py::iter(&(py::zip(&[v_demands.clone(), v_probs.clone()])?))? { let mut v_d: Py; let mut v_pr: Py; 
let t2687 = (t2686).items()?;
if t2687.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_d = t2687[0].clone();
v_pr = t2687[1].clone();
out.push(py::mul(&v_pr.clone(), &py::sub(&py::add(&py::mul(&v_p.clone(), &py::minmax(&[v_d.clone(), v_Q.clone()], false)?)?, &py::mul(&v_s.clone(), &py::minmax(&[py::sub(&v_Q.clone(), &v_d.clone())?, Py::Int(0i64)], true)?)?)?, &py::mul(&v_c.clone(), &v_Q.clone())?)?)?); } Py::list(out) }, None)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__out_p(mut v_out: Py) -> R {
    return Ok(py::method(&v_out.clone(), "get", &[Py::str("shadow_prices"), py::method(&v_out.clone(), "get", &[Py::str("dual_one_optimal")])?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__out_x(mut v_out: Py) -> R {
    return Ok(py::method(&v_out.clone(), "get", &[Py::str("x"), py::method(&v_out.clone(), "get", &[Py::str("x_one_optimal")])?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__payoff(mut v_Am: Py, mut v_name: Py, mut v_max_dim: Py) -> R {
    if ({ let t3929 = py::not(&v_Am.clone()); if t3929.truthy() { t3929 } else { { let t3928 = py::not(&py::getitem(&v_Am.clone(), &Py::Int(0i64))?); if t3928.truthy() { t3928 } else { Py::Bool(({ let mut out = vec![]; for t3925 in py::iter(&(v_Am.clone()))? { let mut v_r: Py; 
v_r = t3925;
out.push({ let t3926 = py::len(&v_r.clone())?; Py::Bool({ let t3927 = py::len(&py::getitem(&v_Am.clone(), &Py::Int(0i64))?)?; if py::ne(&t3926, &t3927) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["the ".to_string(), py::format(&v_name.clone(), "")?, " must be a rectangular matrix".to_string()].concat()))));
    }
    if ({ let t3934 = { let t3930 = py::len(&v_Am.clone())?; Py::Bool({ let t3931 = v_max_dim.clone(); if py::gt(&t3930, &t3931)? { true } else { false } }) }; if t3934.truthy() { t3934 } else { { let t3932 = py::len(&py::getitem(&v_Am.clone(), &Py::Int(0i64))?)?; Py::Bool({ let t3933 = v_max_dim.clone(); if py::gt(&t3932, &t3933)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["at most ".to_string(), py::format(&v_max_dim.clone(), "")?, " strategies per player".to_string()].concat()))));
    }
    return Ok({ let mut out = vec![]; for t3935 in py::iter(&(v_Am.clone()))? { let mut v_r: Py; 
v_r = t3935;
out.push({ let mut out = vec![]; for t3936 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t3936;
out.push(f__fr(v_v.clone())?); } Py::list(out) }); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pder(mut v_cs: Py) -> R {
    let mut v_n: Py = Py::None;
    v_n = py::sub(&py::len(&v_cs.clone())?, &Py::Int(1i64))?;
    return Ok({ let mut out = vec![]; for t2886 in py::iter(&(py::enumerate(&py::slice(&v_cs.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?, 0)?))? { let mut v_a: Py; let mut v_k: Py; 
let t2887 = (t2886).items()?;
if t2887.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2887[0].clone();
v_a = t2887[1].clone();
out.push(py::mul(&v_a.clone(), &py::sub(&v_n.clone(), &v_k.clone())?)?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__perron(mut v_P: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_calm: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_z: Py = Py::None;
    v_n = py::len(&v_P.clone())?;
    v_w = py::mul(&Py::list(vec![py::truediv(&Py::Float(1.0), &v_n.clone())?]), &v_n.clone())?;
    v_calm = Py::Int(0i64);
    for t94 in py::range_iter(&[(Py::Int(20000i64)).i()?]) {
        v__ = t94;
        v_z = { let mut out = vec![]; for t95 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t95;
out.push(py::fsum(&{ let mut out = vec![]; for t96 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t96;
out.push(py::mul(&py::getitem(&py::getitem(&v_P.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_w.clone(), &v_j.clone())?)?); } Py::list(out) })?); } Py::list(out) };
        v_s = py::fsum(&v_z.clone())?;
        v_z = { let mut out = vec![]; for t97 in py::iter(&(v_z.clone()))? { let mut v_v: Py; 
v_v = t97;
out.push(py::truediv(&v_v.clone(), &v_s.clone())?); } Py::list(out) };
        v_calm = (if ({ let t98 = py::minmax(&[{ let mut out = vec![]; for t100 in py::iter(&(py::zip(&[v_z.clone(), v_w.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t101 = (t100).items()?;
if t101.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t101[0].clone();
v_b = t101[1].clone();
out.push(py::truediv(&py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?, &v_b.clone())?); } Py::list(out) }], true)?; Py::Bool({ let t99 = Py::Float(1e-15); if py::le(&t98, &t99)? { true } else { false } }) }).truthy() { py::add(&v_calm.clone(), &Py::Int(1i64))? } else { Py::Int(0i64) });
        v_w = v_z.clone();
        if ({ let t102 = v_calm.clone(); Py::Bool({ let t103 = Py::Int(3i64); if py::ge(&t102, &t103)? { true } else { false } }) }).truthy() {
            break;
        }
    }
    v_lam = py::fsum(&{ let mut out = vec![]; for t104 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t104;
out.push(py::fsum(&{ let mut out = vec![]; for t105 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t105;
out.push(py::mul(&py::getitem(&py::getitem(&v_P.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_w.clone(), &v_j.clone())?)?); } Py::list(out) })?); } Py::list(out) })?;
    return Ok(Py::tuple(vec![v_w.clone(), v_lam.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__peval(mut v_cs: Py, mut v_x: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_v = Py::Float(0.0);
    for t2889 in py::iter(&(v_cs.clone()))? {
        v_a = t2889;
        v_v = py::add(&py::mul(&v_v.clone(), &v_x.clone())?, &v_a.clone())?;
    }
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__project(mut v_dur: Py, mut v_precedences: Py) -> R {
    let mut v_done: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_idx: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_order: Py = Py::None;
    let mut v_preds: Py = Py::None;
    let mut v_ready: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_n = py::len(&v_dur.clone())?;
    if (py::not(&{ let t2795 = Py::Int(1i64); Py::Bool({ let t2796 = v_n.clone(); if py::le(&t2795, &t2796)? { { let t2797 = Py::Int(26i64); if py::le(&t2796, &t2797)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give between 1 and 26 activities"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t2798 in py::iter(&(v_dur.clone()))? { let mut v_v: Py; 
v_v = t2798;
out.push({ let t2799 = v_v.clone(); Py::Bool({ let t2800 = Py::Int(0i64); if py::lt(&t2799, &t2800)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("durations must be >= 0"))));
    }
    v_idx = f__activity_index(v_n.clone())?;
    v_preds = { let mut out = vec![]; for t2802 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v__: Py; 
v__ = t2802;
out.push(py::set(&Py::list(vec![]))?); } Py::list(out) };
    for t2804 in py::iter(&({ let t2803 = v_precedences.clone(); if t2803.truthy() { t2803 } else { Py::list(vec![]) } }))? {
        v_e = t2804;
        if ({ let t2805 = py::len(&v_e.clone())?; Py::Bool({ let t2806 = Py::Int(3i64); if py::eq(&t2805, &t2806) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("a precedence is a pair 'A-B' (A before B) without a number"))));
        }
        let t2807 = (Py::tuple(vec![py::method(&py::method(&Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(0i64))?)), "strip", &[])?, "upper", &[])?, py::method(&py::method(&Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(1i64))?)), "strip", &[])?, "upper", &[])?])).items()?;
        if t2807.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_u = t2807[0].clone();
        v_v = t2807[1].clone();
        if ({ let t2812 = { let t2808 = v_u.clone(); Py::Bool({ let t2809 = v_idx.clone(); if !py::contains(&t2809, &t2808)? { true } else { false } }) }; if t2812.truthy() { t2812 } else { { let t2810 = v_v.clone(); Py::Bool({ let t2811 = v_idx.clone(); if !py::contains(&t2811, &t2810)? { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["precedence ".to_string(), py::format(&v_u.clone(), "")?, "-".to_string(), py::format(&v_v.clone(), "")?, " names an activity that does not exist (activities are A..".to_string(), py::format(&py::chr(&py::sub(&py::add(&py::ord(&Py::str("A"))?, &v_n.clone())?, &Py::Int(1i64))?)?, "")?, " or 1..".to_string(), py::format(&v_n.clone(), "")?, ")".to_string()].concat()))));
        }
        let _ = py::method(&py::getitem(&v_preds.clone(), &py::getitem(&v_idx.clone(), &v_v.clone())?)?, "add", &[py::getitem(&v_idx.clone(), &v_u.clone())?])?;
    }
    let t2813 = (Py::tuple(vec![Py::list(vec![]), py::set(&Py::list(vec![]))?])).items()?;
    if t2813.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_order = t2813[0].clone();
    v_done = t2813[1].clone();
    while ({ let t2820 = py::len(&v_order.clone())?; Py::Bool({ let t2821 = v_n.clone(); if py::lt(&t2820, &t2821)? { true } else { false } }) }).truthy() {
        v_ready = { let mut out = vec![]; for t2814 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t2814;
if ({ let t2819 = { let t2815 = v_k.clone(); Py::Bool({ let t2816 = v_done.clone(); if !py::contains(&t2816, &t2815)? { true } else { false } }) }; if !t2819.truthy() { t2819 } else { { let t2817 = py::getitem(&v_preds.clone(), &v_k.clone())?; Py::Bool({ let t2818 = v_done.clone(); if py::le(&t2817, &t2818)? { true } else { false } }) } } }).truthy() { out.push(v_k.clone()); }} Py::list(out) };
        if (py::not(&v_ready.clone())).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the precedences contain a cycle"))));
        }
        let _ = py::method(&v_order.clone(), "append", &[py::getitem(&v_ready.clone(), &Py::Int(0i64))?])?;
        let _ = py::method(&v_done.clone(), "add", &[py::getitem(&v_ready.clone(), &Py::Int(0i64))?])?;
    }
    return Ok(Py::tuple(vec![v_preds.clone(), v_order.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pure_ne(mut v_MA: Py, mut v_MB: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let t2961 = (Py::tuple(vec![py::len(&v_MA.clone())?, py::len(&py::getitem(&v_MA.clone(), &Py::Int(0i64))?)?])).items()?;
    if t2961.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t2961[0].clone();
    v_n = t2961[1].clone();
    return Ok({ let mut out = vec![]; for t2962 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_r: Py; 
v_r = t2962;
for t2963 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_s: Py; 
v_s = t2963;
if ({ let t2970 = { let t2964 = py::getitem(&py::getitem(&v_MA.clone(), &v_r.clone())?, &v_s.clone())?; Py::Bool({ let t2965 = py::minmax(&[{ let mut out = vec![]; for t2966 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_k: Py; 
v_k = t2966;
out.push(py::getitem(&py::getitem(&v_MA.clone(), &v_k.clone())?, &v_s.clone())?); } Py::list(out) }], true)?; if py::eq(&t2964, &t2965) { true } else { false } }) }; if !t2970.truthy() { t2970 } else { { let t2967 = py::getitem(&py::getitem(&v_MB.clone(), &v_r.clone())?, &v_s.clone())?; Py::Bool({ let t2968 = py::minmax(&[{ let mut out = vec![]; for t2969 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t2969;
out.push(py::getitem(&py::getitem(&v_MB.clone(), &v_r.clone())?, &v_k.clone())?); } Py::list(out) }], true)?; if py::eq(&t2967, &t2968) { true } else { false } }) } } }).truthy() { out.push(Py::list(vec![py::add(&v_r.clone(), &Py::Int(1i64))?, py::add(&v_s.clone(), &Py::Int(1i64))?])); }}} Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__qf(mut v_M: Py, mut v_c: Py, mut v_x: Py) -> R {
    return Ok(py::add(&py::truediv(&py::sum(&{ let mut out = vec![]; for t3644 in py::range_iter(&[(py::len(&v_x.clone())?).i()?]) { let mut v_i: Py; 
v_i = t3644;
for t3645 in py::range_iter(&[(py::len(&v_x.clone())?).i()?]) { let mut v_j: Py; 
v_j = t3645;
out.push(py::mul(&py::mul(&py::getitem(&v_x.clone(), &v_i.clone())?, &py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_j.clone())?)?, &py::getitem(&v_x.clone(), &v_j.clone())?)?); }} Py::list(out) }, None)?, &Py::Int(2i64))?, &py::sum(&{ let mut out = vec![]; for t3646 in py::iter(&(py::zip(&[v_c.clone(), v_x.clone()])?))? { let mut v_ci: Py; let mut v_xi: Py; 
let t3647 = (t3646).items()?;
if t3647.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_ci = t3647[0].clone();
v_xi = t3647[1].clone();
out.push(py::mul(&v_ci.clone(), &v_xi.clone())?); } Py::list(out) }, None)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rel(mut v_a: Py, mut v_b: Py) -> R {
    return Ok(py::truediv(&py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__relations(mut v_text: Py, mut v_m: Py) -> R {
    let mut v_toks: Py = Py::None;
    if ({ let t3520 = { let t3516 = v_text.clone(); Py::Bool({ let t3517 = Py::None; if py::is(&t3516, &t3517) { true } else { false } }) }; if t3520.truthy() { t3520 } else { { let t3518 = py::method(&Py::str(&py::to_str(&v_text.clone())), "strip", &[])?; Py::Bool({ let t3519 = Py::str(""); if py::eq(&t3518, &t3519) { true } else { false } }) } } }).truthy() {
        return Ok(py::mul(&Py::list(vec![Py::str("<=")]), &v_m.clone())?);
    }
    v_toks = { let mut out = vec![]; for t3521 in py::iter(&(ax_engine::pyre::module_call("split", &Py::str("[,\\s;]+"), &[py::method(&py::method(&py::method(&Py::str(&py::to_str(&v_text.clone())), "replace", &[Py::str("≤"), Py::str("<=")])?, "replace", &[Py::str("≥"), Py::str(">=")])?, "strip", &[])?, Py::Int(0)], 0)?))? { let mut v_t: Py; 
v_t = t3521;
if (v_t.clone()).truthy() { out.push(v_t.clone()); }} Py::list(out) };
    v_toks = { let mut out = vec![]; for t3522 in py::iter(&(v_toks.clone()))? { let mut v_t: Py; 
v_t = t3522;
out.push(py::method(&Py::dict(vec![(Py::str("=<"), Py::str("<=")), (Py::str("=>"), Py::str(">=")), (Py::str("=="), Py::str("="))]), "get", &[v_t.clone(), v_t.clone()])?); } Py::list(out) };
    if ({ let t3527 = { let t3523 = py::len(&v_toks.clone())?; Py::Bool({ let t3524 = Py::Int(1i64); if py::eq(&t3523, &t3524) { true } else { false } }) }; if !t3527.truthy() { t3527 } else { { let t3525 = v_m.clone(); Py::Bool({ let t3526 = Py::Int(1i64); if py::gt(&t3525, &t3526)? { true } else { false } }) } } }).truthy() {
        v_toks = py::mul(&v_toks.clone(), &v_m.clone())?;
    }
    return Ok(v_toks.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__roots_in(mut v_cs: Py, mut v_lo: Py, mut v_hi: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_fa: Py = Py::None;
    let mut v_fb: Py = Py::None;
    let mut v_fm: Py = Py::None;
    let mut v_mid: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_r: Py = Py::None;
    while ({ let t2878 = { let t2874 = py::len(&v_cs.clone())?; Py::Bool({ let t2875 = Py::Int(1i64); if py::gt(&t2874, &t2875)? { true } else { false } }) }; if !t2878.truthy() { t2878 } else { { let t2876 = py::getitem(&v_cs.clone(), &Py::Int(0i64))?; Py::Bool({ let t2877 = Py::Int(0i64); if py::eq(&t2876, &t2877) { true } else { false } }) } } }).truthy() {
        v_cs = py::slice(&v_cs.clone(), Some((Py::Int(1i64)).i()?), None, None)?;
    }
    if ({ let t2879 = py::len(&v_cs.clone())?; Py::Bool({ let t2880 = Py::Int(1i64); if py::le(&t2879, &t2880)? { true } else { false } }) }).truthy() {
        return Ok(Py::list(vec![]));
    }
    if ({ let t2881 = py::len(&v_cs.clone())?; Py::Bool({ let t2882 = Py::Int(2i64); if py::eq(&t2881, &t2882) { true } else { false } }) }).truthy() {
        v_r = py::truediv(&py::neg(&py::getitem(&v_cs.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_cs.clone(), &Py::Int(0i64))?)?;
        return Ok((if ({ let t2883 = v_lo.clone(); Py::Bool({ let t2884 = v_r.clone(); if py::le(&t2883, &t2884)? { { let t2885 = v_hi.clone(); if py::le(&t2884, &t2885)? { true } else { false } } } else { false } }) }).truthy() { Py::list(vec![v_r.clone()]) } else { Py::list(vec![]) }));
    }
    v_pts = py::add(&py::add(&Py::list(vec![v_lo.clone()]), &f__roots_in(f__pder(v_cs.clone())?, v_lo.clone(), v_hi.clone())?)?, &Py::list(vec![v_hi.clone()]))?;
    v_out = Py::list(vec![]);
    for t2888 in py::iter(&(py::zip(&[v_pts.clone(), py::slice(&v_pts.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? {
        let t2908 = (t2888).items()?;
        if t2908.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t2908[0].clone();
        v_b = t2908[1].clone();
        let t2890 = (Py::tuple(vec![f__peval(v_cs.clone(), v_a.clone())?, f__peval(v_cs.clone(), v_b.clone())?])).items()?;
        if t2890.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_fa = t2890[0].clone();
        v_fb = t2890[1].clone();
        if ({ let t2891 = v_fa.clone(); Py::Bool({ let t2892 = Py::Int(0i64); if py::eq(&t2891, &t2892) { true } else { false } }) }).truthy() {
            let _ = py::method(&v_out.clone(), "append", &[v_a.clone()])?;
        }
        if ({ let t2893 = py::mul(&v_fa.clone(), &v_fb.clone())?; Py::Bool({ let t2894 = Py::Int(0i64); if py::lt(&t2893, &t2894)? { true } else { false } }) }).truthy() {
            for t2895 in py::range_iter(&[(Py::Int(2000i64)).i()?]) {
                v__ = t2895;
                v_mid = py::truediv(&py::add(&v_a.clone(), &v_b.clone())?, &Py::Int(2i64))?;
                if ({ let t2896 = v_mid.clone(); Py::Bool({ let t2897 = Py::tuple(vec![v_a.clone(), v_b.clone()]); if py::contains(&t2897, &t2896)? { true } else { false } }) }).truthy() {
                    break;
                }
                v_fm = f__peval(v_cs.clone(), v_mid.clone())?;
                if ({ let t2898 = v_fm.clone(); Py::Bool({ let t2899 = Py::Int(0i64); if py::eq(&t2898, &t2899) { true } else { false } }) }).truthy() {
                    let t2900 = v_mid.clone();
                    v_a = t2900.clone();
                    v_b = t2900.clone();
                    break;
                }
                if ({ let t2901 = { let t2903 = v_fm.clone(); Py::Bool({ let t2904 = Py::Int(0i64); if py::lt(&t2903, &t2904)? { true } else { false } }) }; Py::Bool({ let t2902 = { let t2905 = v_fa.clone(); Py::Bool({ let t2906 = Py::Int(0i64); if py::lt(&t2905, &t2906)? { true } else { false } }) }; if py::eq(&t2901, &t2902) { true } else { false } }) }).truthy() {
                    let t2907 = (Py::tuple(vec![v_mid.clone(), v_fm.clone()])).items()?;
                    if t2907.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    v_a = t2907[0].clone();
                    v_fa = t2907[1].clone();
                } else {
                    v_b = v_mid.clone();
                }
            }
            let _ = py::method(&v_out.clone(), "append", &[py::truediv(&py::add(&v_a.clone(), &v_b.clone())?, &Py::Int(2i64))?])?;
        }
    }
    if ({ let t2909 = f__peval(v_cs.clone(), v_hi.clone())?; Py::Bool({ let t2910 = Py::Int(0i64); if py::eq(&t2909, &t2910) { true } else { false } }) }).truthy() {
        let _ = py::method(&v_out.clone(), "append", &[v_hi.clone()])?;
    }
    return Ok(py::sorted(&py::set(&v_out.clone())?, (Py::Bool(false)).truthy())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rr(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_d: Py = Py::None;
    v_d = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?;
    return Ok((if ({ let t3848 = v_d.clone(); Py::Bool({ let t3849 = Py::Float(1e-15); if py::le(&t3848, &t3849)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::truediv(&v_d.clone(), &py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?)? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__simple_graph(mut v_edges: Py, mut v_max_nodes: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_adj: Py = Py::None;
    let mut v_nodes: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_E = f__wedges(v_edges.clone(), Py::Bool(false))?;
    if (Py::Bool(({ let mut out = vec![]; for t2404 in py::iter(&(v_E.clone()))? { let mut v_u: Py; let mut v_v: Py; 
let t2405 = (t2404).items()?;
if t2405.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t2405[0].clone();
v_v = t2405[1].clone();
out.push({ let t2406 = v_u.clone(); Py::Bool({ let t2407 = v_v.clone(); if py::eq(&t2406, &t2407) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a node cannot be joined to itself"))));
    }
    v_nodes = f__nodes(v_E.clone())?;
    if ({ let t2412 = py::len(&v_nodes.clone())?; Py::Bool({ let t2413 = v_max_nodes.clone(); if py::gt(&t2412, &t2413)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["at most ".to_string(), py::format(&v_max_nodes.clone(), "")?, " nodes".to_string()].concat()))));
    }
    v_adj = { let mut out = vec![]; for t2414 in py::iter(&(v_nodes.clone()))? { let mut v_v: Py; 
v_v = t2414;
out.push((v_v.clone(), py::set(&Py::list(vec![]))?)); } Py::dict(out) };
    for t2415 in py::iter(&(v_E.clone()))? {
        let t2416 = (t2415).items()?;
        if t2416.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_u = t2416[0].clone();
        v_v = t2416[1].clone();
        let _ = py::method(&py::getitem(&v_adj.clone(), &v_u.clone())?, "add", &[v_v.clone()])?;
        let _ = py::method(&py::getitem(&v_adj.clone(), &v_v.clone())?, "add", &[v_u.clone()])?;
    }
    return Ok(Py::tuple(vec![v_nodes.clone(), v_adj.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__simplex_eq(mut v_A: Py, mut v_b: Py, mut v_c: Py, mut v_max_iter: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_basis: Py = Py::None;
    let mut v_cb: Py = Py::None;
    let mut v_col: Py = Py::None;
    let mut v_cost: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_phase: Py = Py::None;
    let mut v_pivot: Py = Py::None;
    let mut v_redundant: Py = Py::None;
    let mut v_sign: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let t3981 = (Py::tuple(vec![py::len(&v_A.clone())?, py::len(&v_c.clone())?])).items()?;
    if t3981.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t3981[0].clone();
    v_n = t3981[1].clone();
    v_sign = py::mul(&Py::list(vec![Py::Int(1i64)]), &v_m.clone())?;
    v_A = { let mut out = vec![]; for t3982 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t3982;
out.push(Py::list((v_r.clone()).items()?)); } Py::list(out) };
    v_b = Py::list((v_b.clone()).items()?);
    for t3983 in py::range_iter(&[(v_m.clone()).i()?]) {
        v_i = t3983;
        if ({ let t3984 = py::getitem(&v_b.clone(), &v_i.clone())?; Py::Bool({ let t3985 = Py::Int(0i64); if py::lt(&t3984, &t3985)? { true } else { false } }) }).truthy() {
            let t3987 = (Py::tuple(vec![{ let mut out = vec![]; for t3986 in py::iter(&(py::getitem(&v_A.clone(), &v_i.clone())?))? { let mut v_v: Py; 
v_v = t3986;
out.push(py::neg(&v_v.clone())?); } Py::list(out) }, py::neg(&py::getitem(&v_b.clone(), &v_i.clone())?)?, py::neg(&Py::Int(1i64))?])).items()?;
            if t3987.len() != 3 { return Err(PyErr::Value("unpack".into())); }
            py::setitem(&v_A.clone(), &v_i.clone(), t3987[0].clone())?;
            py::setitem(&v_b.clone(), &v_i.clone(), t3987[1].clone())?;
            py::setitem(&v_sign.clone(), &v_i.clone(), t3987[2].clone())?;
        }
    }
    v_T = { let mut out = vec![]; for t3988 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t3988;
out.push(py::add(&py::add(&py::getitem(&v_A.clone(), &v_i.clone())?, &{ let mut out = vec![]; for t3989 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_k: Py; 
v_k = t3989;
out.push(py::fraction(&[py::int(&{ let t3990 = v_k.clone(); Py::Bool({ let t3991 = v_i.clone(); if py::eq(&t3990, &t3991) { true } else { false } }) })?])?); } Py::list(out) })?, &Py::list(vec![py::getitem(&v_b.clone(), &v_i.clone())?]))?); } Py::list(out) };
    v_basis = { let mut out = vec![]; for t3992 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t3992;
out.push(py::add(&v_n.clone(), &v_i.clone())?); } Py::list(out) };
    v_N = py::add(&v_n.clone(), &v_m.clone())?;
    v_pivot = { let v_A = v_A.clone(); let v_N = v_N.clone(); let v_T = v_T.clone(); let v_b = v_b.clone(); let v_basis = v_basis.clone(); let v_c = v_c.clone(); let v_i = v_i.clone(); let v_m = v_m.clone(); let v_max_iter = v_max_iter.clone(); let v_n = v_n.clone(); let v_pivot = v_pivot.clone(); let v_sign = v_sign.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_col: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_f: Py = Py::None;
            let mut v_i: Py = Py::None;
            let mut v_pv: Py = Py::None;
            v_pv = py::getitem(&py::getitem(&v_T.clone(), &v_r.clone())?, &v_col.clone())?;
            py::setitem(&v_T.clone(), &v_r.clone(), { let mut out = vec![]; for t3993 in py::iter(&(py::getitem(&v_T.clone(), &v_r.clone())?))? { let mut v_v: Py; 
v_v = t3993;
out.push(py::truediv(&v_v.clone(), &v_pv.clone())?); } Py::list(out) })?;
            for t3994 in py::range_iter(&[(py::len(&v_T.clone())?).i()?]) {
                v_i = t3994;
                if ({ let t3999 = { let t3995 = v_i.clone(); Py::Bool({ let t3996 = v_r.clone(); if py::ne(&t3995, &t3996) { true } else { false } }) }; if !t3999.truthy() { t3999 } else { { let t3997 = py::getitem(&py::getitem(&v_T.clone(), &v_i.clone())?, &v_col.clone())?; Py::Bool({ let t3998 = Py::Int(0i64); if py::ne(&t3997, &t3998) { true } else { false } }) } } }).truthy() {
                    v_f = py::getitem(&py::getitem(&v_T.clone(), &v_i.clone())?, &v_col.clone())?;
                    py::setitem(&v_T.clone(), &v_i.clone(), { let mut out = vec![]; for t4000 in py::iter(&(py::zip(&[py::getitem(&v_T.clone(), &v_i.clone())?, py::getitem(&v_T.clone(), &v_r.clone())?])?))? { let mut v_a: Py; let mut v_bb: Py; 
let t4001 = (t4000).items()?;
if t4001.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4001[0].clone();
v_bb = t4001[1].clone();
out.push(py::sub(&v_a.clone(), &py::mul(&v_f.clone(), &v_bb.clone())?)?); } Py::list(out) })?;
                }
            }
            py::setitem(&v_basis.clone(), &v_r.clone(), v_col.clone())?;
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_phase = { let v_A = v_A.clone(); let v_N = v_N.clone(); let v_T = v_T.clone(); let v_b = v_b.clone(); let v_basis = v_basis.clone(); let v_c = v_c.clone(); let v_i = v_i.clone(); let v_m = v_m.clone(); let v_max_iter = v_max_iter.clone(); let v_n = v_n.clone(); let v_phase = v_phase.clone(); let v_pivot = v_pivot.clone(); let v_sign = v_sign.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_cost: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_allowed: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v__: Py = Py::None;
            let mut v_best: Py = Py::None;
            let mut v_cb: Py = Py::None;
            let mut v_enter: Py = Py::None;
            let mut v_i: Py = Py::None;
            let mut v_inb: Py = Py::None;
            let mut v_j: Py = Py::None;
            let mut v_ratio: Py = Py::None;
            let mut v_rc: Py = Py::None;
            for t4002 in py::range_iter(&[(v_max_iter.clone()).i()?]) {
                v__ = t4002;
                v_cb = { let mut out = vec![]; for t4003 in py::iter(&(v_basis.clone()))? { let mut v_j: Py; 
v_j = t4003;
out.push(py::getitem(&v_cost.clone(), &v_j.clone())?); } Py::list(out) };
                v_enter = Py::None;
                v_inb = py::set(&v_basis.clone())?;
                for t4004 in py::range_iter(&[(v_allowed.clone()).i()?]) {
                    v_j = t4004;
                    if ({ let t4005 = v_j.clone(); Py::Bool({ let t4006 = v_inb.clone(); if !py::contains(&t4006, &t4005)? { true } else { false } }) }).truthy() {
                        v_rc = py::sub(&py::getitem(&v_cost.clone(), &v_j.clone())?, &py::sum(&{ let mut out = vec![]; for t4007 in py::range_iter(&[(py::len(&v_T.clone())?).i()?]) { let mut v_i: Py; 
v_i = t4007;
if ({ let t4008 = py::getitem(&py::getitem(&v_T.clone(), &v_i.clone())?, &v_j.clone())?; Py::Bool({ let t4009 = Py::Int(0i64); if py::ne(&t4008, &t4009) { true } else { false } }) }).truthy() { out.push(py::mul(&py::getitem(&v_cb.clone(), &v_i.clone())?, &py::getitem(&py::getitem(&v_T.clone(), &v_i.clone())?, &v_j.clone())?)?); }} Py::list(out) }, None)?)?;
                        if ({ let t4010 = v_rc.clone(); Py::Bool({ let t4011 = Py::Int(0i64); if py::lt(&t4010, &t4011)? { true } else { false } }) }).truthy() {
                            v_enter = v_j.clone();
                            break;
                        }
                    }
                }
                if ({ let t4012 = v_enter.clone(); Py::Bool({ let t4013 = Py::None; if py::is(&t4012, &t4013) { true } else { false } }) }).truthy() {
                    return Ok(Py::str("optimal"));
                }
                v_best = Py::None;
                for t4014 in py::range_iter(&[(py::len(&v_T.clone())?).i()?]) {
                    v_i = t4014;
                    if ({ let t4015 = py::getitem(&py::getitem(&v_T.clone(), &v_i.clone())?, &v_enter.clone())?; Py::Bool({ let t4016 = Py::Int(0i64); if py::gt(&t4015, &t4016)? { true } else { false } }) }).truthy() {
                        v_ratio = py::truediv(&py::getitem(&py::getitem(&v_T.clone(), &v_i.clone())?, &py::neg(&Py::Int(1i64))?)?, &py::getitem(&py::getitem(&v_T.clone(), &v_i.clone())?, &v_enter.clone())?)?;
                        if ({ let t4027 = { let t4017 = v_best.clone(); Py::Bool({ let t4018 = Py::None; if py::is(&t4017, &t4018) { true } else { false } }) }; if t4027.truthy() { t4027 } else { { let t4026 = { let t4019 = v_ratio.clone(); Py::Bool({ let t4020 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; if py::lt(&t4019, &t4020)? { true } else { false } }) }; if t4026.truthy() { t4026 } else { { let t4025 = { let t4021 = v_ratio.clone(); Py::Bool({ let t4022 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; if py::eq(&t4021, &t4022) { true } else { false } }) }; if !t4025.truthy() { t4025 } else { { let t4023 = py::getitem(&v_basis.clone(), &v_i.clone())?; Py::Bool({ let t4024 = py::getitem(&v_basis.clone(), &py::getitem(&v_best.clone(), &Py::Int(1i64))?)?; if py::lt(&t4023, &t4024)? { true } else { false } }) } } } } } } }).truthy() {
                            v_best = Py::tuple(vec![v_ratio.clone(), v_i.clone()]);
                        }
                    }
                }
                if ({ let t4028 = v_best.clone(); Py::Bool({ let t4029 = Py::None; if py::is(&t4028, &t4029) { true } else { false } }) }).truthy() {
                    return Ok(Py::str("unbounded"));
                }
                let _ = py::call(&v_pivot.clone(), &[py::getitem(&v_best.clone(), &Py::Int(1i64))?, v_enter.clone()])?;
            }
            return Err(py::value_error(py::to_str(&Py::str("the simplex method did not finish within its iteration limit"))));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let _ = py::call(&v_phase.clone(), &[py::add(&py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &v_n.clone())?, &py::mul(&Py::list(vec![py::fraction(&[Py::Int(1i64)])?]), &v_m.clone())?)?, v_N.clone()])?;
    if ({ let t4030 = py::sum(&{ let mut out = vec![]; for t4032 in py::range_iter(&[(py::len(&v_T.clone())?).i()?]) { let mut v_i: Py; 
v_i = t4032;
if ({ let t4033 = py::getitem(&v_basis.clone(), &v_i.clone())?; Py::Bool({ let t4034 = v_n.clone(); if py::ge(&t4033, &t4034)? { true } else { false } }) }).truthy() { out.push(py::getitem(&py::getitem(&v_T.clone(), &v_i.clone())?, &py::neg(&Py::Int(1i64))?)?); }} Py::list(out) }, None)?; Py::Bool({ let t4031 = Py::Int(0i64); if py::gt(&t4030, &t4031)? { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![Py::str("infeasible"), Py::None, Py::None, Py::None, Py::Int(0i64)]));
    }
    v_redundant = Py::Int(0i64);
    v_i = Py::Int(0i64);
    while ({ let t4043 = v_i.clone(); Py::Bool({ let t4044 = py::len(&v_T.clone())?; if py::lt(&t4043, &t4044)? { true } else { false } }) }).truthy() {
        if ({ let t4035 = py::getitem(&v_basis.clone(), &v_i.clone())?; Py::Bool({ let t4036 = v_n.clone(); if py::ge(&t4035, &t4036)? { true } else { false } }) }).truthy() {
            v_col = (match ('g4037: { let mut out = vec![]; for t4038 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t4038;
if ({ let t4039 = py::getitem(&py::getitem(&v_T.clone(), &v_i.clone())?, &v_j.clone())?; Py::Bool({ let t4040 = Py::Int(0i64); if py::ne(&t4039, &t4040) { true } else { false } }) }).truthy() { out.push(v_j.clone()); break 'g4037 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => Py::None });
            if ({ let t4041 = v_col.clone(); Py::Bool({ let t4042 = Py::None; if py::is(&t4041, &t4042) { true } else { false } }) }).truthy() {
                let _ = py::method(&v_T.clone(), "pop", &[v_i.clone()])?;
                let _ = py::method(&v_basis.clone(), "pop", &[v_i.clone()])?;
                v_redundant = py::iadd(&v_redundant, &Py::Int(1i64))?;
                continue;
            }
            let _ = py::call(&v_pivot.clone(), &[v_i.clone(), v_col.clone()])?;
        }
        v_i = py::iadd(&v_i, &Py::Int(1i64))?;
    }
    v_cost = py::add(&Py::list((v_c.clone()).items()?), &py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &v_m.clone())?)?;
    if ({ let t4045 = py::call(&v_phase.clone(), &[v_cost.clone(), v_n.clone()])?; Py::Bool({ let t4046 = Py::str("unbounded"); if py::eq(&t4045, &t4046) { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![Py::str("unbounded"), Py::None, Py::None, Py::None, v_redundant.clone()]));
    }
    v_x = py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &v_n.clone())?;
    for t4047 in py::iter(&(py::enumerate(&v_basis.clone(), 0)?))? {
        let t4048 = (t4047).items()?;
        if t4048.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t4048[0].clone();
        v_j = t4048[1].clone();
        py::setitem(&v_x.clone(), &v_j.clone(), py::getitem(&py::getitem(&v_T.clone(), &v_i.clone())?, &py::neg(&Py::Int(1i64))?)?)?;
    }
    v_cb = { let mut out = vec![]; for t4049 in py::iter(&(v_basis.clone()))? { let mut v_j: Py; 
v_j = t4049;
out.push(py::getitem(&v_cost.clone(), &v_j.clone())?); } Py::list(out) };
    v_y = { let mut out = vec![]; for t4050 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_k: Py; 
v_k = t4050;
out.push(py::mul(&py::getitem(&v_sign.clone(), &v_k.clone())?, &py::sum(&{ let mut out = vec![]; for t4051 in py::range_iter(&[(py::len(&v_T.clone())?).i()?]) { let mut v_i: Py; 
v_i = t4051;
out.push(py::mul(&py::getitem(&v_cb.clone(), &v_i.clone())?, &py::getitem(&py::getitem(&v_T.clone(), &v_i.clone())?, &py::add(&v_n.clone(), &v_k.clone())?)?)?); } Py::list(out) }, None)?)?); } Py::list(out) };
    return Ok(Py::tuple(vec![Py::str("optimal"), v_x.clone(), v_y.clone(), Py::list((v_basis.clone()).items()?), v_redundant.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__solve_exact(mut v_M: Py, mut v_rhs: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    v_n = py::len(&v_M.clone())?;
    v_A = { let mut out = vec![]; for t3624 in py::iter(&(py::zip(&[v_M.clone(), v_rhs.clone()])?))? { let mut v_r: Py; let mut v_v: Py; 
let t3625 = (t3624).items()?;
if t3625.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t3625[0].clone();
v_v = t3625[1].clone();
out.push(py::add(&Py::list((v_r.clone()).items()?), &Py::list(vec![v_v.clone()]))?); } Py::list(out) };
    for t3626 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t3626;
        v_p = (match ('g3627: { let mut out = vec![]; for t3628 in py::range_iter(&[(v_k.clone()).i()?, (v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3628;
if ({ let t3629 = py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_k.clone())?; Py::Bool({ let t3630 = Py::Int(0i64); if py::ne(&t3629, &t3630) { true } else { false } }) }).truthy() { out.push(v_i.clone()); break 'g3627 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => Py::None });
        if ({ let t3631 = v_p.clone(); Py::Bool({ let t3632 = Py::None; if py::is(&t3631, &t3632) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the system is singular"))));
        }
        let t3633 = (Py::tuple(vec![py::getitem(&v_A.clone(), &v_p.clone())?, py::getitem(&v_A.clone(), &v_k.clone())?])).items()?;
        if t3633.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_A.clone(), &v_k.clone(), t3633[0].clone())?;
        py::setitem(&v_A.clone(), &v_p.clone(), t3633[1].clone())?;
        for t3634 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_i = t3634;
            if ({ let t3639 = { let t3635 = v_i.clone(); Py::Bool({ let t3636 = v_k.clone(); if py::ne(&t3635, &t3636) { true } else { false } }) }; if !t3639.truthy() { t3639 } else { { let t3637 = py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_k.clone())?; Py::Bool({ let t3638 = Py::Int(0i64); if py::ne(&t3637, &t3638) { true } else { false } }) } } }).truthy() {
                v_f = py::truediv(&py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_k.clone())?, &py::getitem(&py::getitem(&v_A.clone(), &v_k.clone())?, &v_k.clone())?)?;
                py::setitem(&v_A.clone(), &v_i.clone(), { let mut out = vec![]; for t3640 in py::iter(&(py::zip(&[py::getitem(&v_A.clone(), &v_i.clone())?, py::getitem(&v_A.clone(), &v_k.clone())?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3641 = (t3640).items()?;
if t3641.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3641[0].clone();
v_b = t3641[1].clone();
out.push(py::sub(&v_a.clone(), &py::mul(&v_f.clone(), &v_b.clone())?)?); } Py::list(out) })?;
            }
        }
    }
    return Ok({ let mut out = vec![]; for t3642 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3642;
out.push(py::truediv(&py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_n.clone())?, &py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_i.clone())?)?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sp_result(mut v_E: Py, mut v_s: Py, mut v_t: Py, mut v_directed: Py, mut v_dist: Py, mut v_adj: Py) -> R {
    let mut v_count: Py = Py::None;
    let mut v_nodes: Py = Py::None;
    let mut v_path: Py = Py::None;
    let mut v_tight: Py = Py::None;
    let mut v_unique: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_zero: Py = Py::None;
    if ({ let t3096 = v_t.clone(); Py::Bool({ let t3097 = v_dist.clone(); if !py::contains(&t3097, &t3096)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["there is no path from ".to_string(), py::format(&v_s.clone(), "")?, " to ".to_string(), py::format(&v_t.clone(), "")?].concat()))));
    }
    v_nodes = py::sorted_key(&v_dist.clone(), &{ let v_E = v_E.clone(); let v_adj = v_adj.clone(); let v_directed = v_directed.clone(); let v_dist = v_dist.clone(); let v_s = v_s.clone(); let v_t = v_t.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::getitem(&v_dist.clone(), &v_v.clone())?, py::method(&f__nodes(v_E.clone())?, "index", &[v_v.clone()])?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    v_tight = { let mut out = vec![]; for t3098 in py::iter(&(v_nodes.clone()))? { let mut v_v: Py; 
v_v = t3098;
out.push((v_v.clone(), { let mut out = vec![]; for t3099 in py::iter(&(v_nodes.clone()))? { let mut v_u: Py; 
v_u = t3099;
for t3100 in py::iter(&(py::method(&v_adj.clone(), "get", &[v_u.clone(), Py::list(vec![])])?))? { let mut v_w: Py; let mut v_x: Py; 
let t3101 = (t3100).items()?;
if t3101.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t3101[0].clone();
v_w = t3101[1].clone();
if ({ let t3106 = { let t3102 = v_x.clone(); Py::Bool({ let t3103 = v_v.clone(); if py::eq(&t3102, &t3103) { true } else { false } }) }; if !t3106.truthy() { t3106 } else { { let t3104 = py::add(&py::getitem(&v_dist.clone(), &v_u.clone())?, &v_w.clone())?; Py::Bool({ let t3105 = py::getitem(&v_dist.clone(), &v_v.clone())?; if py::eq(&t3104, &t3105) { true } else { false } }) } } }).truthy() { out.push(Py::tuple(vec![v_u.clone(), v_w.clone()])); }}} Py::list(out) })); } Py::dict(out) };
    v_zero = Py::Bool(({ let mut out = vec![]; for t3107 in py::iter(&(v_nodes.clone()))? { let mut v_v: Py; 
v_v = t3107;
for t3108 in py::iter(&(py::getitem(&v_tight.clone(), &v_v.clone())?))? { let mut v__: Py; let mut v_w: Py; 
let t3109 = (t3108).items()?;
if t3109.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v__ = t3109[0].clone();
v_w = t3109[1].clone();
out.push({ let t3110 = v_w.clone(); Py::Bool({ let t3111 = Py::Int(0i64); if py::eq(&t3110, &t3111) { true } else { false } }) }); }} Py::list(out) }).items()?.iter().any(|x| x.truthy()));
    v_count = Py::dict(vec![(v_s.clone(), Py::Int(1i64))]);
    for t3112 in py::iter(&(v_nodes.clone()))? {
        v_v = t3112;
        if ({ let t3113 = v_v.clone(); Py::Bool({ let t3114 = v_s.clone(); if py::ne(&t3113, &t3114) { true } else { false } }) }).truthy() {
            py::setitem(&v_count.clone(), &v_v.clone(), py::sum(&{ let mut out = vec![]; for t3115 in py::iter(&(py::getitem(&v_tight.clone(), &v_v.clone())?))? { let mut v__: Py; let mut v_u: Py; 
let t3116 = (t3115).items()?;
if t3116.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t3116[0].clone();
v__ = t3116[1].clone();
out.push(py::method(&v_count.clone(), "get", &[v_u.clone(), Py::Int(0i64)])?); } Py::list(out) }, None)?)?;
        }
    }
    let t3117 = (Py::tuple(vec![Py::list(vec![v_t.clone()]), v_t.clone()])).items()?;
    if t3117.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_path = t3117[0].clone();
    v_v = t3117[1].clone();
    while ({ let t3120 = v_v.clone(); Py::Bool({ let t3121 = v_s.clone(); if py::ne(&t3120, &t3121) { true } else { false } }) }).truthy() {
        v_v = py::minmax_key(&[{ let mut out = vec![]; for t3118 in py::iter(&(py::getitem(&v_tight.clone(), &v_v.clone())?))? { let mut v__: Py; let mut v_u: Py; 
let t3119 = (t3118).items()?;
if t3119.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t3119[0].clone();
v__ = t3119[1].clone();
out.push(v_u.clone()); } Py::list(out) }], &{ let v_E = v_E.clone(); let v_adj = v_adj.clone(); let v_count = v_count.clone(); let v_directed = v_directed.clone(); let v_dist = v_dist.clone(); let v_nodes = v_nodes.clone(); let v_path = v_path.clone(); let v_s = v_s.clone(); let v_t = v_t.clone(); let v_tight = v_tight.clone(); let v_v = v_v.clone(); let v_zero = v_zero.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_u: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::getitem(&v_dist.clone(), &v_u.clone())?, py::method(&f__nodes(v_E.clone())?, "index", &[v_u.clone()])?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, false)?;
        let _ = py::method(&v_path.clone(), "append", &[v_v.clone()])?;
    }
    let _ = py::method(&v_path.clone(), "reverse", &[])?;
    v_unique = { let t3124 = { let t3122 = py::getitem(&v_count.clone(), &v_t.clone())?; Py::Bool({ let t3123 = Py::Int(1i64); if py::eq(&t3122, &t3123) { true } else { false } }) }; if !t3124.truthy() { t3124 } else { py::not(&v_zero.clone()) } };
    return Ok(Py::dict(vec![(Py::str("distance"), py::float(&py::getitem(&v_dist.clone(), &v_t.clone())?)?), ((if (v_unique.clone()).truthy() { Py::str("path") } else { Py::str("path_example") }), v_path.clone()), (Py::str("path_unique"), (if (v_unique.clone()).truthy() { Py::str("yes") } else { Py::str("no") }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__standard(mut v_sense: Py, mut v_c: Py, mut v_A: Py, mut v_b: Py, mut v_rel: Py) -> R {
    let mut v_As: Py = Py::None;
    let mut v_cs: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_row: Py = Py::None;
    let mut v_slack_cols: Py = Py::None;
    let t3965 = (Py::tuple(vec![py::len(&v_A.clone())?, py::len(&v_c.clone())?])).items()?;
    if t3965.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t3965[0].clone();
    v_n = t3965[1].clone();
    v_slack_cols = { let mut out = vec![]; for t3966 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t3966;
if ({ let t3967 = py::getitem(&v_rel.clone(), &v_i.clone())?; Py::Bool({ let t3968 = Py::tuple(vec![Py::str("<="), Py::str(">=")]); if py::contains(&t3968, &t3967)? { true } else { false } }) }).truthy() { out.push(v_i.clone()); }} Py::list(out) };
    v_As = Py::list(vec![]);
    for t3969 in py::range_iter(&[(v_m.clone()).i()?]) {
        v_i = t3969;
        v_row = { let mut out = vec![]; for t3970 in py::iter(&(py::getitem(&v_A.clone(), &v_i.clone())?))? { let mut v_v: Py; 
v_v = t3970;
out.push(f__fr(v_v.clone())?); } Py::list(out) };
        for t3971 in py::iter(&(v_slack_cols.clone()))? {
            v_k = t3971;
            let _ = py::method(&v_row.clone(), "append", &[(if ({ let t3972 = v_k.clone(); Py::Bool({ let t3973 = v_i.clone(); if py::ne(&t3972, &t3973) { true } else { false } }) }).truthy() { py::fraction(&[Py::Int(0i64)])? } else { (if ({ let t3974 = py::getitem(&v_rel.clone(), &v_i.clone())?; Py::Bool({ let t3975 = Py::str("<="); if py::eq(&t3974, &t3975) { true } else { false } }) }).truthy() { py::fraction(&[Py::Int(1i64)])? } else { py::fraction(&[py::neg(&Py::Int(1i64))?])? }) })])?;
        }
        let _ = py::method(&v_As.clone(), "append", &[v_row.clone()])?;
    }
    v_cs = py::add(&{ let mut out = vec![]; for t3976 in py::iter(&(v_c.clone()))? { let mut v_v: Py; 
v_v = t3976;
out.push((if ({ let t3977 = v_sense.clone(); Py::Bool({ let t3978 = Py::str("minimize"); if py::eq(&t3977, &t3978) { true } else { false } }) }).truthy() { f__fr(v_v.clone())? } else { py::neg(&f__fr(v_v.clone())?)? })); } Py::list(out) }, &py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &py::len(&v_slack_cols.clone())?)?)?;
    return Ok(Py::tuple(vec![v_As.clone(), { let mut out = vec![]; for t3979 in py::iter(&(v_b.clone()))? { let mut v_v: Py; 
v_v = t3979;
out.push(f__fr(v_v.clone())?); } Py::list(out) }, v_cs.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sym_pd(mut v_Q: Py, mut v_name: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_W: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_n = py::len(&v_Q.clone())?;
    if ({ let t3606 = { let t3598 = v_n.clone(); Py::Bool({ let t3599 = Py::Int(0i64); if py::eq(&t3598, &t3599) { true } else { false } }) }; if t3606.truthy() { t3606 } else { { let t3605 = Py::Bool(({ let mut out = vec![]; for t3600 in py::iter(&(v_Q.clone()))? { let mut v_r: Py; 
v_r = t3600;
out.push({ let t3601 = py::len(&v_r.clone())?; Py::Bool({ let t3602 = v_n.clone(); if py::ne(&t3601, &t3602) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t3605.truthy() { t3605 } else { { let t3603 = v_n.clone(); Py::Bool({ let t3604 = Py::Int(10i64); if py::gt(&t3603, &t3604)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a square matrix (at most 10 x 10)".to_string()].concat()))));
    }
    v_M = { let mut out = vec![]; for t3607 in py::iter(&(v_Q.clone()))? { let mut v_r: Py; 
v_r = t3607;
out.push({ let mut out = vec![]; for t3608 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t3608;
out.push(f__fr(v_v.clone())?); } Py::list(out) }); } Py::list(out) };
    if (Py::Bool(({ let mut out = vec![]; for t3609 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3609;
for t3610 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3610;
out.push({ let t3611 = py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_j.clone())?; Py::Bool({ let t3612 = py::getitem(&py::getitem(&v_M.clone(), &v_j.clone())?, &v_i.clone())?; if py::ne(&t3611, &t3612) { true } else { false } }) }); }} Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be symmetric".to_string()].concat()))));
    }
    v_W = { let mut out = vec![]; for t3613 in py::iter(&(v_M.clone()))? { let mut v_r: Py; 
v_r = t3613;
out.push(py::slice(&v_r.clone(), None, None, None)?); } Py::list(out) };
    for t3614 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t3614;
        if ({ let t3615 = py::getitem(&py::getitem(&v_W.clone(), &v_k.clone())?, &v_k.clone())?; Py::Bool({ let t3616 = Py::Int(0i64); if py::le(&t3615, &t3616)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be positive definite (otherwise there is no unique minimum)".to_string()].concat()))));
        }
        for t3617 in py::range_iter(&[(py::add(&v_k.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_i = t3617;
            v_f = py::truediv(&py::getitem(&py::getitem(&v_W.clone(), &v_i.clone())?, &v_k.clone())?, &py::getitem(&py::getitem(&v_W.clone(), &v_k.clone())?, &v_k.clone())?)?;
            for t3618 in py::range_iter(&[(v_k.clone()).i()?, (v_n.clone()).i()?]) {
                v_j = t3618;
                let t3619 = py::getitem(&v_W.clone(), &v_i.clone())?; let t3620 = v_j.clone();
                py::setitem(&t3619, &t3620, py::sub(&py::getitem(&t3619, &t3620)?, &py::mul(&v_f.clone(), &py::getitem(&py::getitem(&v_W.clone(), &v_k.clone())?, &v_j.clone())?)?)?)?;
            }
        }
    }
    return Ok(v_M.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__topsis_check(mut v_X: Py, mut v_w: Py, mut v_types: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let t3299 = (f__mcda_in(v_X.clone(), v_w.clone(), v_types.clone())?).items()?;
    if t3299.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_X = t3299[0].clone();
    v_w = t3299[1].clone();
    v_types = t3299[2].clone();
    let t3300 = (Py::tuple(vec![py::len(&v_X.clone())?, py::len(&py::getitem(&v_X.clone(), &Py::Int(0i64))?)?])).items()?;
    if t3300.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t3300[0].clone();
    v_k = t3300[1].clone();
    v_R = { let mut out = vec![]; for t3301 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t3301;
out.push({ let mut out = vec![]; for t3302 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t3302;
out.push(py::truediv(&py::getitem(&py::getitem(&v_X.clone(), &v_i.clone())?, &v_j.clone())?, &py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t3303 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_t: Py; 
v_t = t3303;
out.push(py::pow(&py::getitem(&py::getitem(&v_X.clone(), &v_t.clone())?, &v_j.clone())?, &Py::Int(2i64))?); } Py::list(out) })?)?)?); } Py::list(out) }); } Py::list(out) };
    v_V = { let mut out = vec![]; for t3304 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t3304;
out.push({ let mut out = vec![]; for t3305 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t3305;
out.push(py::mul(&py::getitem(&v_w.clone(), &v_j.clone())?, &(if ({ let t3306 = py::getitem(&v_types.clone(), &v_j.clone())?; Py::Bool({ let t3307 = Py::Int(1i64); if py::eq(&t3306, &t3307) { true } else { false } }) }).truthy() { py::getitem(&py::getitem(&v_R.clone(), &v_i.clone())?, &v_j.clone())? } else { py::sub(&Py::Int(1i64), &py::getitem(&py::getitem(&v_R.clone(), &v_i.clone())?, &v_j.clone())?)? }))?); } Py::list(out) }); } Py::list(out) };
    v_hi = { let mut out = vec![]; for t3308 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t3308;
out.push(py::minmax(&[{ let mut out = vec![]; for t3309 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t3309;
out.push(py::getitem(&py::getitem(&v_V.clone(), &v_i.clone())?, &v_j.clone())?); } Py::list(out) }], true)?); } Py::list(out) };
    v_lo = { let mut out = vec![]; for t3310 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t3310;
out.push(py::minmax(&[{ let mut out = vec![]; for t3311 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t3311;
out.push(py::getitem(&py::getitem(&v_V.clone(), &v_i.clone())?, &v_j.clone())?); } Py::list(out) }], false)?); } Py::list(out) };
    v_sc = { let mut out = vec![]; for t3312 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t3312;
out.push(py::truediv(&py::dist(&py::getitem(&v_V.clone(), &v_i.clone())?, &v_lo.clone())?, &py::add(&py::dist(&py::getitem(&v_V.clone(), &v_i.clone())?, &v_lo.clone())?, &py::dist(&py::getitem(&v_V.clone(), &v_i.clone())?, &v_hi.clone())?)?)?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("scores"), v_sc.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__transport_lp(mut v_supply: Py, mut v_demand: Py, mut v_cost: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_rel: Py = Py::None;
    let t3322 = (Py::tuple(vec![py::len(&v_supply.clone())?, py::len(&v_demand.clone())?])).items()?;
    if t3322.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t3322[0].clone();
    v_n = t3322[1].clone();
    if ({ let t3328 = { let t3323 = py::len(&v_cost.clone())?; Py::Bool({ let t3324 = v_m.clone(); if py::ne(&t3323, &t3324) { true } else { false } }) }; if t3328.truthy() { t3328 } else { Py::Bool(({ let mut out = vec![]; for t3325 in py::iter(&(v_cost.clone()))? { let mut v_r: Py; 
v_r = t3325;
out.push({ let t3326 = py::len(&v_r.clone())?; Py::Bool({ let t3327 = v_n.clone(); if py::ne(&t3326, &t3327) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["the cost matrix must have ".to_string(), py::format(&v_m.clone(), "")?, " rows (sources) and ".to_string(), py::format(&v_n.clone(), "")?, " columns (destinations)".to_string()].concat()))));
    }
    if ({ let t3335 = Py::Bool(({ let mut out = vec![]; for t3329 in py::iter(&(v_supply.clone()))? { let mut v_v: Py; 
v_v = t3329;
out.push({ let t3330 = v_v.clone(); Py::Bool({ let t3331 = Py::Int(0i64); if py::lt(&t3330, &t3331)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t3335.truthy() { t3335 } else { Py::Bool(({ let mut out = vec![]; for t3332 in py::iter(&(v_demand.clone()))? { let mut v_v: Py; 
v_v = t3332;
out.push({ let t3333 = v_v.clone(); Py::Bool({ let t3334 = Py::Int(0i64); if py::lt(&t3333, &t3334)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("supplies and demands must be >= 0"))));
    }
    if ({ let t3336 = py::sum(&{ let mut out = vec![]; for t3338 in py::iter(&(v_supply.clone()))? { let mut v_v: Py; 
v_v = t3338;
out.push(f__fr(v_v.clone())?); } Py::list(out) }, None)?; Py::Bool({ let t3337 = py::sum(&{ let mut out = vec![]; for t3339 in py::iter(&(v_demand.clone()))? { let mut v_v: Py; 
v_v = t3339;
out.push(f__fr(v_v.clone())?); } Py::list(out) }, None)?; if py::lt(&t3336, &t3337)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("total supply is less than total demand: the demand cannot be met"))));
    }
    if ({ let t3340 = py::mul(&v_m.clone(), &v_n.clone())?; Py::Bool({ let t3341 = Py::Int(60i64); if py::gt(&t3340, &t3341)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at most 60 routes (sources x destinations)"))));
    }
    v_c = { let mut out = vec![]; for t3342 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t3342;
for t3343 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3343;
out.push(py::getitem(&py::getitem(&v_cost.clone(), &v_i.clone())?, &v_j.clone())?); }} Py::list(out) };
    let t3344 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![]), Py::list(vec![])])).items()?;
    if t3344.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3344[0].clone();
    v_b = t3344[1].clone();
    v_rel = t3344[2].clone();
    for t3345 in py::range_iter(&[(v_m.clone()).i()?]) {
        v_i = t3345;
        let _ = py::method(&v_A.clone(), "append", &[{ let mut out = vec![]; for t3346 in py::range_iter(&[(py::mul(&v_m.clone(), &v_n.clone())?).i()?]) { let mut v_k: Py; 
v_k = t3346;
out.push((if ({ let t3347 = py::floordiv(&v_k.clone(), &v_n.clone())?; Py::Bool({ let t3348 = v_i.clone(); if py::eq(&t3347, &t3348) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { Py::Int(0i64) })); } Py::list(out) }])?;
        let _ = py::method(&v_b.clone(), "append", &[py::getitem(&v_supply.clone(), &v_i.clone())?])?;
        let _ = py::method(&v_rel.clone(), "append", &[Py::str("<=")])?;
    }
    for t3349 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_j = t3349;
        let _ = py::method(&v_A.clone(), "append", &[{ let mut out = vec![]; for t3350 in py::range_iter(&[(py::mul(&v_m.clone(), &v_n.clone())?).i()?]) { let mut v_k: Py; 
v_k = t3350;
out.push((if ({ let t3351 = py::modulo(&v_k.clone(), &v_n.clone())?; Py::Bool({ let t3352 = v_j.clone(); if py::eq(&t3351, &t3352) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { Py::Int(0i64) })); } Py::list(out) }])?;
        let _ = py::method(&v_b.clone(), "append", &[py::getitem(&v_demand.clone(), &v_j.clone())?])?;
        let _ = py::method(&v_rel.clone(), "append", &[Py::str("=")])?;
    }
    return Ok(Py::tuple(vec![Py::str("minimize"), v_c.clone(), v_A.clone(), v_b.clone(), v_rel.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_2x2(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t3904 = ({ let mut out = vec![]; for t3902 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("A"))?))? { let mut v_r: Py; 
v_r = t3902;
out.push({ let mut out = vec![]; for t3903 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t3903;
out.push(py::float(&v_v.clone())?); } Py::list(out) }); } Py::list(out) }).items()?;
    if t3904.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    let t3905 = (t3904[0].clone()).items()?;
    if t3905.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3905[0].clone();
    v_b = t3905[1].clone();
    let t3906 = (t3904[1].clone()).items()?;
    if t3906.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t3906[0].clone();
    v_d = t3906[1].clone();
    v_v = py::getitem(&v_o.clone(), &Py::str("value"))?;
    let t3907 = (Py::tuple(vec![py::minmax(&[py::minmax(&[v_a.clone(), v_b.clone()], false)?, py::minmax(&[v_c.clone(), v_d.clone()], false)?], true)?, py::minmax(&[py::minmax(&[v_a.clone(), v_c.clone()], true)?, py::minmax(&[v_b.clone(), v_d.clone()], true)?], false)?])).items()?;
    if t3907.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t3907[0].clone();
    v_hi = t3907[1].clone();
    if ({ let t3908 = py::method(&v_o.clone(), "get", &[Py::str("saddle_point")])?; Py::Bool({ let t3909 = Py::str("yes"); if py::eq(&t3908, &t3909) { true } else { false } }) }).truthy() {
        return Ok((if ({ let t3910 = v_lo.clone(); Py::Bool({ let t3911 = v_hi.clone(); if py::eq(&t3910, &t3911) { true } else { false } }) }).truthy() { py::minmax(&[f__rel(v_v.clone(), v_lo.clone())?, f__rel(v_v.clone(), v_hi.clone())?], true)? } else { Py::Float(1.0) }));
    }
    if ({ let t3916 = { let t3912 = v_lo.clone(); Py::Bool({ let t3913 = v_hi.clone(); if py::eq(&t3912, &t3913) { true } else { false } }) }; if t3916.truthy() { t3916 } else { { let t3914 = Py::str("p_row1"); Py::Bool({ let t3915 = v_o.clone(); if !py::contains(&t3915, &t3914)? { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    let t3917 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("p_row1"))?, py::getitem(&v_o.clone(), &Py::str("q_col1"))?])).items()?;
    if t3917.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t3917[0].clone();
    v_q = t3917[1].clone();
    if (py::not(&{ let t3924 = { let t3918 = Py::Int(0i64); Py::Bool({ let t3919 = v_p.clone(); if py::lt(&t3918, &t3919)? { { let t3920 = Py::Int(1i64); if py::lt(&t3919, &t3920)? { true } else { false } } } else { false } }) }; if !t3924.truthy() { t3924 } else { { let t3921 = Py::Int(0i64); Py::Bool({ let t3922 = v_q.clone(); if py::lt(&t3921, &t3922)? { { let t3923 = Py::Int(1i64); if py::lt(&t3922, &t3923)? { true } else { false } } } else { false } }) } } })).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::minmax(&[f__rel(py::add(&py::mul(&v_p.clone(), &v_a.clone())?, &py::mul(&py::sub(&Py::Int(1i64), &v_p.clone())?, &v_c.clone())?)?, v_v.clone())?, f__rel(py::add(&py::mul(&v_p.clone(), &v_b.clone())?, &py::mul(&py::sub(&Py::Int(1i64), &v_p.clone())?, &v_d.clone())?)?, v_v.clone())?, f__rel(py::add(&py::mul(&v_q.clone(), &v_a.clone())?, &py::mul(&py::sub(&Py::Int(1i64), &v_q.clone())?, &v_b.clone())?)?, v_v.clone())?, f__rel(py::add(&py::mul(&v_q.clone(), &v_c.clone())?, &py::mul(&py::sub(&Py::Int(1i64), &v_q.clone())?, &v_d.clone())?)?, v_v.clone())?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_apsp(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_rows: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_E = f__wedges(py::getitem(&v_i.clone(), &Py::str("edges"))?, Py::Bool(true))?;
    let t201: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            if ({ let t173 = py::sorted(&py::getitem(&v_o.clone(), &Py::str("nodes"))?, (Py::Bool(false)).truthy())?; Py::Bool({ let t174 = py::sorted(&f__nodes(v_E.clone())?, (Py::Bool(false)).truthy())?; if py::ne(&t173, &t174) { true } else { false } }) }).truthy() {
                return Ok(Some(Py::Float(1.0)));
            }
            v_rows = { let mut out = vec![]; for t175 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("nodes"))?))? { let mut v_s: Py; 
v_s = t175;
out.push((v_s.clone(), f__bellman(v_E.clone(), v_s.clone(), py::getitem(&v_i.clone(), &Py::str("directed"))?)?)); } Py::dict(out) };
            if (Py::Bool(({ let mut out = vec![]; for t191 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("nodes"))?))? { let mut v_s: Py; 
v_s = t191;
for t192 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("nodes"))?))? { let mut v_t: Py; 
v_t = t192;
out.push({ let t193 = py::method(&py::getitem(&v_rows.clone(), &v_s.clone())?, "get", &[v_t.clone()])?; Py::Bool({ let t194 = Py::None; if py::is(&t193, &t194) { true } else { false } }) }); }} Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
                return Ok(Some(Py::Float(1.0)));
            }
            v_worst = py::minmax(&[{ let mut out = vec![]; for t195 in py::iter(&(py::zip(&[py::getitem(&v_o.clone(), &Py::str("nodes"))?, py::getitem(&v_o.clone(), &Py::str("distances"))?])?))? { let mut v_ra: Py; let mut v_s: Py; 
let t196 = (t195).items()?;
if t196.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_s = t196[0].clone();
v_ra = t196[1].clone();
for t197 in py::iter(&(py::zip(&[v_ra.clone(), py::getitem(&v_o.clone(), &Py::str("nodes"))?])?))? { let mut v_a: Py; let mut v_t: Py; 
let t198 = (t197).items()?;
if t198.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t198[0].clone();
v_t = t198[1].clone();
out.push(f__rel(v_a.clone(), py::float(&py::getitem(&py::getitem(&v_rows.clone(), &v_s.clone())?, &v_t.clone())?)?)?); }} Py::list(out) }], true)?;
            return Ok(Some(py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_o.clone(), &Py::str("diameter"))?, py::minmax(&[{ let mut out = vec![]; for t199 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("nodes"))?))? { let mut v_s: Py; 
v_s = t199;
for t200 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("nodes"))?))? { let mut v_t: Py; 
v_t = t200;
out.push(py::float(&py::getitem(&py::getitem(&v_rows.clone(), &v_s.clone())?, &v_t.clone())?)?); }} Py::list(out) }], true)?)?], true)?));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t201 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_assign(mut v_i: Py, mut v_o: Py, mut v_maximize: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_C = py::getitem(&v_i.clone(), &Py::str("C"))?;
    v_n = py::len(&v_C.clone())?;
    v_a = { let mut out = vec![]; for t317 in py::iter(&(py::method(&v_o.clone(), "get", &[Py::str("assignment"), py::method(&v_o.clone(), "get", &[Py::str("assignment_example")])?])?))? { let mut v_x: Py; 
v_x = t317;
out.push(py::sub(&py::int(&py::round(&v_x.clone(), None)?)?, &Py::Int(1i64))?); } Py::list(out) };
    if ({ let t318 = py::sorted(&v_a.clone(), (Py::Bool(false)).truthy())?; Py::Bool({ let t319 = Py::list((Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>())).items()?); if py::ne(&t318, &t319) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_key = (if (v_maximize.clone()).truthy() { Py::str("max_value") } else { Py::str("min_cost") });
    v_s = (if (v_maximize.clone()).truthy() { py::neg(&Py::Int(1i64))? } else { Py::Int(1i64) });
    let t322 = (Py::tuple(vec![{ let mut out = vec![]; for t320 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("dual_u"))?))? { let mut v_x: Py; 
v_x = t320;
out.push(py::mul(&v_s.clone(), &v_x.clone())?); } Py::list(out) }, { let mut out = vec![]; for t321 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("dual_v"))?))? { let mut v_x: Py; 
v_x = t321;
out.push(py::mul(&v_s.clone(), &v_x.clone())?); } Py::list(out) }])).items()?;
    if t322.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_u = t322[0].clone();
    v_v = t322[1].clone();
    v_worst = f__rel(py::getitem(&v_o.clone(), &v_key.clone())?, py::sum(&{ let mut out = vec![]; for t323 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t323;
out.push(py::getitem(&py::getitem(&v_C.clone(), &v_k.clone())?, &py::getitem(&v_a.clone(), &v_k.clone())?)?); } Py::list(out) }, None)?)?;
    v_scale = py::minmax(&[Py::Float(1.0), py::minmax(&[{ let mut out = vec![]; for t324 in py::iter(&(v_C.clone()))? { let mut v_r: Py; 
v_r = t324;
for t325 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t325;
out.push(py::abs(&v_x.clone())?); }} Py::list(out) }], true)?], true)?;
    for t326 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_r = t326;
        for t327 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_c = t327;
            v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::minmax(&[Py::Float(0.0), py::sub(&py::add(&py::getitem(&v_u.clone(), &v_r.clone())?, &py::getitem(&v_v.clone(), &v_c.clone())?)?, &py::mul(&v_s.clone(), &py::getitem(&py::getitem(&v_C.clone(), &v_r.clone())?, &v_c.clone())?)?)?], true)?, &v_scale.clone())?], true)?;
        }
    }
    return Ok(py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_o.clone(), &v_key.clone())?, py::mul(&v_s.clone(), &py::add(&py::sum(&v_u.clone(), None)?, &py::sum(&v_v.clone(), None)?)?)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_bf(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_E = f__wedges(py::getitem(&v_i.clone(), &Py::str("edges"))?, Py::Bool(true))?;
    v_d = { let mut out = vec![]; for t363 in py::iter(&(py::method(&py::getitem(&v_o.clone(), &Py::str("distances_from_source"))?, "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t364 = (t363).items()?;
if t364.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t364[0].clone();
v_v = t364[1].clone();
out.push((v_k.clone(), f__fr(v_v.clone())?)); } Py::dict(out) };
    let t365 = (Py::tuple(vec![py::method(&Py::str(&py::to_str(&py::getitem(&v_i.clone(), &Py::str("source"))?)), "strip", &[])?, py::method(&Py::str(&py::to_str(&py::getitem(&v_i.clone(), &Py::str("target"))?)), "strip", &[])?])).items()?;
    if t365.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t365[0].clone();
    v_t = t365[1].clone();
    if ({ let t370 = { let t366 = py::method(&v_d.clone(), "get", &[v_s.clone()])?; Py::Bool({ let t367 = Py::Int(0i64); if py::ne(&t366, &t367) { true } else { false } }) }; if t370.truthy() { t370 } else { { let t368 = v_t.clone(); Py::Bool({ let t369 = v_d.clone(); if !py::contains(&t369, &t368)? { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_worst = f__rel(py::getitem(&v_o.clone(), &Py::str("distance"))?, py::float(&py::getitem(&v_d.clone(), &v_t.clone())?)?)?;
    for t371 in py::iter(&(v_E.clone()))? {
        let t380 = (t371).items()?;
        if t380.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_u = t380[0].clone();
        v_v = t380[1].clone();
        v_w = t380[2].clone();
        if ({ let t379 = { let t372 = v_u.clone(); Py::Bool({ let t373 = v_d.clone(); if py::contains(&t373, &t372)? { true } else { false } }) }; if !t379.truthy() { t379 } else { { let t378 = { let t374 = v_v.clone(); Py::Bool({ let t375 = v_d.clone(); if !py::contains(&t375, &t374)? { true } else { false } }) }; if t378.truthy() { t378 } else { { let t376 = py::add(&py::getitem(&v_d.clone(), &v_u.clone())?, &v_w.clone())?; Py::Bool({ let t377 = py::getitem(&v_d.clone(), &v_v.clone())?; if py::lt(&t376, &t377)? { true } else { false } }) } } } } }).truthy() {
            return Ok(Py::Float(1.0));
        }
    }
    for t381 in py::iter(&(v_d.clone()))? {
        v_v = t381;
        if ({ let t393 = { let t382 = v_v.clone(); Py::Bool({ let t383 = v_s.clone(); if py::ne(&t382, &t383) { true } else { false } }) }; if !t393.truthy() { t393 } else { py::not(&Py::Bool(({ let mut out = vec![]; for t384 in py::iter(&(v_E.clone()))? { let mut v_u: Py; let mut v_w: Py; let mut v_x: Py; 
let t385 = (t384).items()?;
if t385.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_u = t385[0].clone();
v_x = t385[1].clone();
v_w = t385[2].clone();
if ({ let t386 = v_x.clone(); Py::Bool({ let t387 = v_v.clone(); if py::eq(&t386, &t387) { true } else { false } }) }).truthy() { out.push({ let t392 = { let t388 = v_u.clone(); Py::Bool({ let t389 = v_d.clone(); if py::contains(&t389, &t388)? { true } else { false } }) }; if !t392.truthy() { t392 } else { { let t390 = py::add(&py::getitem(&v_d.clone(), &v_u.clone())?, &v_w.clone())?; Py::Bool({ let t391 = py::getitem(&v_d.clone(), &v_v.clone())?; if py::eq(&t390, &t391) { true } else { false } }) } } }); }} Py::list(out) }).items()?.iter().any(|x| x.truthy()))) } }).truthy() {
            return Ok(Py::Float(1.0));
        }
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_bins(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_ks: Py = Py::None;
    let mut v_loads: Py = Py::None;
    v_ks = { let mut out = vec![]; for t434 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("bin_of_item"))?))? { let mut v_x: Py; 
v_x = t434;
out.push(py::sub(&py::int(&py::round(&v_x.clone(), None)?)?, &Py::Int(1i64))?); } Py::list(out) };
    v_loads = Py::dict(vec![]);
    for t435 in py::iter(&(py::enumerate(&v_ks.clone(), 0)?))? {
        let t436 = (t435).items()?;
        if t436.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_j = t436[0].clone();
        v_k = t436[1].clone();
        py::setitem(&v_loads.clone(), &v_k.clone(), py::add(&py::method(&v_loads.clone(), "get", &[v_k.clone(), Py::Int(0i64)])?, &py::getitem(&py::getitem(&v_i.clone(), &Py::str("sizes"))?, &v_j.clone())?)?)?;
    }
    if (Py::Bool(({ let mut out = vec![]; for t437 in py::iter(&(py::method(&v_loads.clone(), "values", &[])?))? { let mut v_v: Py; 
v_v = t437;
out.push({ let t438 = v_v.clone(); Py::Bool({ let t439 = py::add(&py::getitem(&v_i.clone(), &Py::str("capacity"))?, &Py::Float(1e-09))?; if py::gt(&t438, &t439)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::minmax(&[f__rel(py::getitem(&v_o.clone(), &Py::str("min_bins"))?, py::len(&v_loads.clone())?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("min_bins"))?, py::getitem(&f_bin_packing(py::getitem(&v_i.clone(), &Py::str("sizes"))?, py::getitem(&v_i.clone(), &Py::str("capacity"))?)?, &Py::str("min_bins"))?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_bm(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_cov: Py = Py::None;
    let mut v_es: Py = Py::None;
    let mut v_pairs: Py = Py::None;
    v_E = f__wedges(py::getitem(&v_i.clone(), &Py::str("edges"))?, Py::Bool(false))?;
    v_pairs = { let mut out = vec![]; for t490 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("matching_example"))?))? { let mut v_s: Py; 
v_s = t490;
out.push(Py::tuple((py::method(&v_s.clone(), "split", &[Py::str("-"), Py::Int(1i64)])?).items()?)); } Py::list(out) };
    v_es = py::set(&v_E.clone())?;
    if ({ let t501 = Py::Bool(({ let mut out = vec![]; for t491 in py::iter(&(v_pairs.clone()))? { let mut v_p: Py; 
v_p = t491;
out.push({ let t492 = v_p.clone(); Py::Bool({ let t493 = v_es.clone(); if !py::contains(&t493, &t492)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t501.truthy() { t501 } else { { let t500 = { let t494 = py::len(&py::set(&{ let mut out = vec![]; for t496 in py::iter(&(v_pairs.clone()))? { let mut v_p: Py; 
v_p = t496;
out.push(py::getitem(&v_p.clone(), &Py::Int(0i64))?); } Py::list(out) })?)?; Py::Bool({ let t495 = py::len(&v_pairs.clone())?; if py::ne(&t494, &t495) { true } else { false } }) }; if t500.truthy() { t500 } else { { let t497 = py::len(&py::set(&{ let mut out = vec![]; for t499 in py::iter(&(v_pairs.clone()))? { let mut v_p: Py; 
v_p = t499;
out.push(py::getitem(&v_p.clone(), &Py::Int(1i64))?); } Py::list(out) })?)?; Py::Bool({ let t498 = py::len(&v_pairs.clone())?; if py::ne(&t497, &t498) { true } else { false } }) } } } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_cov = py::set(&py::getitem(&v_o.clone(), &Py::str("vertex_cover"))?)?;
    if (Py::Bool(({ let mut out = vec![]; for t502 in py::iter(&(v_E.clone()))? { let mut v_u: Py; let mut v_v: Py; 
let t503 = (t502).items()?;
if t503.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t503[0].clone();
v_v = t503[1].clone();
out.push({ let t508 = { let t504 = v_u.clone(); Py::Bool({ let t505 = v_cov.clone(); if !py::contains(&t505, &t504)? { true } else { false } }) }; if !t508.truthy() { t508 } else { { let t506 = v_v.clone(); Py::Bool({ let t507 = v_cov.clone(); if !py::contains(&t507, &t506)? { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::minmax(&[f__rel(py::getitem(&v_o.clone(), &Py::str("max_matching"))?, py::len(&v_pairs.clone())?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("max_matching"))?, py::len(&v_cov.clone())?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_col(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_cl: Py = Py::None;
    let mut v_nodes: Py = Py::None;
    let t897 = (f__simple_graph(py::getitem(&v_i.clone(), &Py::str("edges"))?, Py::Int(14i64))?).items()?;
    if t897.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_nodes = t897[0].clone();
    v_adj = t897[1].clone();
    v_cl = py::getitem(&v_o.clone(), &Py::str("coloring_example"))?;
    if ({ let t902 = { let t898 = py::method(&v_o.clone(), "get", &[Py::str("nodes")])?; Py::Bool({ let t899 = v_nodes.clone(); if py::ne(&t898, &t899) { true } else { false } }) }; if t902.truthy() { t902 } else { { let t900 = py::len(&v_cl.clone())?; Py::Bool({ let t901 = py::len(&v_nodes.clone())?; if py::ne(&t900, &t901) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_c = py::dict_from(&py::zip(&[v_nodes.clone(), v_cl.clone()])?, vec![])?;
    if ({ let t909 = Py::Bool(({ let mut out = vec![]; for t903 in py::iter(&(v_nodes.clone()))? { let mut v_u: Py; 
v_u = t903;
for t904 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? { let mut v_v: Py; 
v_v = t904;
out.push({ let t905 = py::getitem(&v_c.clone(), &v_u.clone())?; Py::Bool({ let t906 = py::getitem(&v_c.clone(), &v_v.clone())?; if py::eq(&t905, &t906) { true } else { false } }) }); }} Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t909.truthy() { t909 } else { { let t907 = py::len(&py::set(&v_cl.clone())?)?; Py::Bool({ let t908 = py::getitem(&v_o.clone(), &Py::str("chromatic_number"))?; if py::ne(&t907, &t908) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("chromatic_number"))?, py::getitem(&f_chromatic(py::getitem(&v_i.clone(), &Py::str("edges"))?)?, &Py::str("chromatic_number"))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_cp(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_det: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_kind: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let mut v_val: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let t605 = ({ let mut out = vec![]; for t604 in py::iter(&(Py::tuple(vec![Py::str("a"), Py::str("b"), Py::str("c")])))? { let mut v_k: Py; 
v_k = t604;
out.push(py::float(&py::getitem(&v_i.clone(), &v_k.clone())?)?); } Py::list(out) }).items()?;
    if t605.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t605[0].clone();
    v_b = t605[1].clone();
    v_c = t605[2].clone();
    let t607 = ({ let mut out = vec![]; for t606 in py::iter(&(Py::tuple(vec![Py::str("d"), Py::str("e"), Py::str("f0")])))? { let mut v_k: Py; 
v_k = t606;
out.push(py::float(&py::method(&v_i.clone(), "get", &[v_k.clone(), Py::Float(0.0)])?)?); } Py::list(out) }).items()?;
    if t607.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_d = t607[0].clone();
    v_e = t607[1].clone();
    v_g = t607[2].clone();
    let t608 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("x"))?, py::getitem(&v_o.clone(), &Py::str("y"))?])).items()?;
    if t608.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t608[0].clone();
    v_y = t608[1].clone();
    v_sc = py::mul(&py::minmax(&[Py::Float(1.0), py::abs(&v_a.clone())?, py::abs(&v_b.clone())?, py::abs(&v_c.clone())?, py::abs(&v_d.clone())?, py::abs(&v_e.clone())?], true)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_x.clone())?, py::abs(&v_y.clone())?], true)?)?;
    v_det = py::sub(&py::mul(&py::mul(&Py::Int(4i64), &v_a.clone())?, &v_c.clone())?, &py::mul(&v_b.clone(), &v_b.clone())?)?;
    v_kind = (if ({ let t609 = v_det.clone(); Py::Bool({ let t610 = Py::Int(0i64); if py::lt(&t609, &t610)? { true } else { false } }) }).truthy() { Py::str("saddle point") } else { (if ({ let t611 = v_a.clone(); Py::Bool({ let t612 = Py::Int(0i64); if py::gt(&t611, &t612)? { true } else { false } }) }).truthy() { Py::str("local minimum") } else { Py::str("local maximum") }) });
    v_val = py::add(&py::add(&py::add(&py::add(&py::add(&py::mul(&py::mul(&v_a.clone(), &v_x.clone())?, &v_x.clone())?, &py::mul(&py::mul(&v_b.clone(), &v_x.clone())?, &v_y.clone())?)?, &py::mul(&py::mul(&v_c.clone(), &v_y.clone())?, &v_y.clone())?)?, &py::mul(&v_d.clone(), &v_x.clone())?)?, &py::mul(&v_e.clone(), &v_y.clone())?)?, &v_g.clone())?;
    return Ok(py::minmax(&[py::truediv(&py::abs(&py::add(&py::add(&py::mul(&py::mul(&Py::Int(2i64), &v_a.clone())?, &v_x.clone())?, &py::mul(&v_b.clone(), &v_y.clone())?)?, &v_d.clone())?)?, &v_sc.clone())?, py::truediv(&py::abs(&py::add(&py::add(&py::mul(&v_b.clone(), &v_x.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &v_c.clone())?, &v_y.clone())?)?, &v_e.clone())?)?, &v_sc.clone())?, f__rel(py::getitem(&v_o.clone(), &Py::str("f_value"))?, v_val.clone())?, (if ({ let t613 = py::getitem(&v_o.clone(), &Py::str("type"))?; Py::Bool({ let t614 = v_kind.clone(); if py::eq(&t613, &t614) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) })], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_cpm(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t567: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f_cpm(py::getitem(&v_i.clone(), &Py::str("durations"))?, py::method(&v_i.clone(), "get", &[Py::str("precedences")])?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t567 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    v_worst = f__rel(py::getitem(&v_o.clone(), &Py::str("project_duration"))?, py::getitem(&v_r.clone(), &Py::str("project_duration"))?)?;
    for t568 in py::iter(&(Py::tuple(vec![Py::str("slack"), Py::str("earliest_start"), Py::str("latest_start")])))? {
        v_k = t568;
        v_worst = py::minmax(&[py::add(&Py::list(vec![v_worst.clone()]), &{ let mut out = vec![]; for t569 in py::iter(&(py::zip(&[py::getitem(&v_o.clone(), &v_k.clone())?, py::getitem(&v_r.clone(), &v_k.clone())?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t570 = (t569).items()?;
if t570.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t570[0].clone();
v_b = t570[1].clone();
out.push(f__rel(v_a.clone(), v_b.clone())?); } Py::list(out) })?], true)?;
    }
    return Ok((if ({ let t571 = py::getitem(&v_o.clone(), &Py::str("critical_activities"))?; Py::Bool({ let t572 = py::getitem(&v_r.clone(), &Py::str("critical_activities"))?; if py::eq(&t571, &t572) { true } else { false } }) }).truthy() { v_worst.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_cr(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_ci: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_n = py::len(&py::getitem(&v_o.clone(), &Py::str("weights"))?)?;
    if ({ let t31 = v_n.clone(); Py::Bool({ let t32 = Py::Int(3i64); if py::lt(&t31, &t32)? { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_ci = py::truediv(&py::sub(&py::getitem(&v_o.clone(), &Py::str("lambda_max"))?, &v_n.clone())?, &py::sub(&v_n.clone(), &Py::Int(1i64))?)?;
    return Ok(py::minmax(&[f__verify_eigen(v_i.clone(), v_o.clone())?, f__rr(py::getitem(&v_o.clone(), &Py::str("CI"))?, v_ci.clone())?, f__rr(py::getitem(&v_o.clone(), &Py::str("CR"))?, py::truediv(&v_ci.clone(), &py::getitem(&c__RI()?, &v_n.clone())?)?)?, (if ({ let t44 = py::getitem(&v_o.clone(), &Py::str("acceptable"))?; Py::Bool({ let t45 = (if ({ let t46 = py::getitem(&v_o.clone(), &Py::str("CR"))?; Py::Bool({ let t47 = Py::Float(0.1); if py::lt(&t46, &t47)? { true } else { false } }) }).truthy() { Py::str("yes") } else { Py::str("no") }); if py::eq(&t44, &t45) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) })], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_discount(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_Qk: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_ir: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_nxt: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_pk: Py = Py::None;
    let mut v_q0: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t693 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("D"))?, py::getitem(&v_i.clone(), &Py::str("S"))?, py::getitem(&v_i.clone(), &Py::str("i_rate"))?])).items()?;
    if t693.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_D = t693[0].clone();
    v_S = t693[1].clone();
    v_ir = t693[2].clone();
    let t694 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("order_quantity"))?, py::getitem(&v_o.clone(), &Py::str("unit_price"))?])).items()?;
    if t694.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Q = t694[0].clone();
    v_p = t694[1].clone();
    v_k = (if ({ let t695 = v_p.clone(); Py::Bool({ let t696 = py::getitem(&v_i.clone(), &Py::str("prices"))?; if py::contains(&t696, &t695)? { true } else { false } }) }).truthy() { py::method(&py::getitem(&v_i.clone(), &Py::str("prices"))?, "index", &[v_p.clone()])? } else { py::neg(&Py::Int(1i64))? });
    if ({ let t701 = { let t697 = v_k.clone(); Py::Bool({ let t698 = Py::Int(0i64); if py::lt(&t697, &t698)? { true } else { false } }) }; if t701.truthy() { t701 } else { { let t699 = v_Q.clone(); Py::Bool({ let t700 = py::sub(&py::getitem(&py::getitem(&v_i.clone(), &Py::str("breaks"))?, &v_k.clone())?, &Py::Float(1e-09))?; if py::lt(&t699, &t700)? { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_worst = f__rel(py::getitem(&v_o.clone(), &Py::str("total_cost"))?, py::add(&py::add(&py::mul(&v_D.clone(), &v_p.clone())?, &py::truediv(&py::mul(&v_D.clone(), &v_S.clone())?, &v_Q.clone())?)?, &py::truediv(&py::mul(&py::mul(&v_ir.clone(), &v_p.clone())?, &v_Q.clone())?, &Py::Int(2i64))?)?)?;
    for t702 in py::iter(&(py::zip(&[py::getitem(&v_i.clone(), &Py::str("breaks"))?, py::getitem(&v_i.clone(), &Py::str("prices"))?])?))? {
        let t709 = (t702).items()?;
        if t709.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_q0 = t709[0].clone();
        v_pk = t709[1].clone();
        v_Qk = py::minmax(&[py::math1("sqrt", &py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &v_D.clone())?, &v_S.clone())?, &py::mul(&v_ir.clone(), &v_pk.clone())?)?)?, v_q0.clone(), Py::Float(1e-300)], true)?;
        v_nxt = { let mut out = vec![]; for t703 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("breaks"))?))? { let mut v_b: Py; 
v_b = t703;
if ({ let t704 = v_b.clone(); Py::Bool({ let t705 = v_q0.clone(); if py::gt(&t704, &t705)? { true } else { false } }) }).truthy() { out.push(v_b.clone()); }} Py::list(out) };
        if ({ let t708 = v_nxt.clone(); if !t708.truthy() { t708 } else { { let t706 = v_Qk.clone(); Py::Bool({ let t707 = py::getitem(&v_nxt.clone(), &Py::Int(0i64))?; if py::ge(&t706, &t707)? { true } else { false } }) } } }).truthy() {
            v_Qk = py::getitem(&v_nxt.clone(), &Py::Int(0i64))?;
            v_pk = py::getitem(&py::getitem(&v_i.clone(), &Py::str("prices"))?, &py::method(&py::getitem(&v_i.clone(), &Py::str("breaks"))?, "index", &[py::getitem(&v_nxt.clone(), &Py::Int(0i64))?])?)?;
        }
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::minmax(&[Py::Float(0.0), py::sub(&py::getitem(&v_o.clone(), &Py::str("total_cost"))?, &py::add(&py::add(&py::mul(&v_D.clone(), &v_pk.clone())?, &py::truediv(&py::mul(&v_D.clone(), &v_S.clone())?, &v_Qk.clone())?)?, &py::truediv(&py::mul(&py::mul(&v_ir.clone(), &v_pk.clone())?, &v_Qk.clone())?, &Py::Int(2i64))?)?)?], true)?, &py::getitem(&v_o.clone(), &Py::str("total_cost"))?)?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_eigen(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t112: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_P = f__ahp_matrix(py::getitem(&v_i.clone(), &Py::str("judgments"))?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t112 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    let t113 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("weights"))?, py::getitem(&v_o.clone(), &Py::str("lambda_max"))?])).items()?;
    if t113.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_w = t113[0].clone();
    v_lam = t113[1].clone();
    if ({ let t118 = { let t114 = py::len(&v_w.clone())?; Py::Bool({ let t115 = py::len(&v_P.clone())?; if py::ne(&t114, &t115) { true } else { false } }) }; if t118.truthy() { t118 } else { { let t116 = py::minmax(&[v_w.clone()], false)?; Py::Bool({ let t117 = Py::Int(0i64); if py::le(&t116, &t117)? { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::minmax(&[{ let mut out = vec![]; for t119 in py::range_iter(&[(py::len(&v_w.clone())?).i()?]) { let mut v_a: Py; 
v_a = t119;
out.push(f__rr(py::fsum(&{ let mut out = vec![]; for t122 in py::range_iter(&[(py::len(&v_w.clone())?).i()?]) { let mut v_b: Py; 
v_b = t122;
out.push(py::mul(&py::getitem(&py::getitem(&v_P.clone(), &v_a.clone())?, &v_b.clone())?, &py::getitem(&v_w.clone(), &v_b.clone())?)?); } Py::list(out) })?, py::mul(&v_lam.clone(), &py::getitem(&v_w.clone(), &v_a.clone())?)?)?); } Py::list(out) }], true)?;
    return Ok(py::minmax(&[v_r.clone(), py::abs(&py::sub(&py::fsum(&v_w.clone())?, &Py::Float(1.0))?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_eoq_b(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_D: Py = Py::None;
    let mut v_H: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_g_q: Py = Py::None;
    let mut v_g_s: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_tc: Py = Py::None;
    let t658 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("D"))?, py::getitem(&v_i.clone(), &Py::str("S"))?, py::getitem(&v_i.clone(), &Py::str("H"))?, py::getitem(&v_i.clone(), &Py::str("B"))?])).items()?;
    if t658.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_D = t658[0].clone();
    v_S = t658[1].clone();
    v_H = t658[2].clone();
    v_B = t658[3].clone();
    let t659 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("Q"))?, py::getitem(&v_o.clone(), &Py::str("max_backorder"))?])).items()?;
    if t659.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Q = t659[0].clone();
    v_s = t659[1].clone();
    v_tc = py::add(&py::add(&py::truediv(&py::mul(&v_D.clone(), &v_S.clone())?, &v_Q.clone())?, &py::truediv(&py::mul(&v_H.clone(), &py::pow(&py::sub(&v_Q.clone(), &v_s.clone())?, &Py::Int(2i64))?)?, &py::mul(&Py::Int(2i64), &v_Q.clone())?)?)?, &py::truediv(&py::mul(&py::mul(&v_B.clone(), &v_s.clone())?, &v_s.clone())?, &py::mul(&Py::Int(2i64), &v_Q.clone())?)?)?;
    v_g_s = py::truediv(&py::add(&py::mul(&py::neg(&v_H.clone())?, &py::sub(&v_Q.clone(), &v_s.clone())?)?, &py::mul(&v_B.clone(), &v_s.clone())?)?, &v_Q.clone())?;
    v_g_q = py::sub(&py::add(&py::truediv(&py::mul(&py::neg(&v_D.clone())?, &v_S.clone())?, &py::pow(&v_Q.clone(), &Py::Int(2i64))?)?, &py::truediv(&py::mul(&v_H.clone(), &py::sub(&py::mul(&v_Q.clone(), &v_Q.clone())?, &py::mul(&v_s.clone(), &v_s.clone())?)?)?, &py::mul(&py::mul(&Py::Int(2i64), &v_Q.clone())?, &v_Q.clone())?)?)?, &py::truediv(&py::mul(&py::mul(&v_B.clone(), &v_s.clone())?, &v_s.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &v_Q.clone())?, &v_Q.clone())?)?)?;
    return Ok(py::minmax(&[f__rel(py::getitem(&v_o.clone(), &Py::str("total_cost"))?, v_tc.clone())?, py::truediv(&py::abs(&v_g_s.clone())?, &py::minmax(&[Py::Float(1.0), py::add(&v_H.clone(), &v_B.clone())?], true)?)?, py::truediv(&py::mul(&py::abs(&v_g_q.clone())?, &v_Q.clone())?, &py::minmax(&[Py::Float(1.0), v_tc.clone()], true)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("max_inventory"))?, py::sub(&v_Q.clone(), &v_s.clone())?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_eqqp(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_M: Py = Py::None;
    let mut v_atl: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_feas: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t788: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_M = f__sym_pd(py::getitem(&v_i.clone(), &Py::str("Q"))?, Py::str("Q"))?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t788 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    let t789 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("x_star"))?, py::getitem(&v_o.clone(), &Py::str("multipliers"))?, py::getitem(&v_i.clone(), &Py::str("A"))?])).items()?;
    if t789.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_x = t789[0].clone();
    v_lam = t789[1].clone();
    v_A = t789[2].clone();
    if ({ let t794 = { let t790 = py::len(&v_x.clone())?; Py::Bool({ let t791 = py::len(&v_M.clone())?; if py::ne(&t790, &t791) { true } else { false } }) }; if t794.truthy() { t794 } else { { let t792 = py::len(&v_lam.clone())?; Py::Bool({ let t793 = py::len(&v_A.clone())?; if py::ne(&t792, &t793) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_atl = { let mut out = vec![]; for t795 in py::range_iter(&[(py::len(&v_x.clone())?).i()?]) { let mut v_j: Py; 
v_j = t795;
out.push(py::fsum(&{ let mut out = vec![]; for t796 in py::range_iter(&[(py::len(&v_A.clone())?).i()?]) { let mut v_k: Py; 
v_k = t796;
out.push(py::mul(&py::float(&py::getitem(&py::getitem(&v_A.clone(), &v_k.clone())?, &v_j.clone())?)?, &py::getitem(&v_lam.clone(), &v_k.clone())?)?); } Py::list(out) })?); } Py::list(out) };
    v_feas = py::truediv(&py::minmax(&[{ let mut out = vec![]; for t797 in py::range_iter(&[(py::len(&v_A.clone())?).i()?]) { let mut v_k: Py; 
v_k = t797;
out.push(py::abs(&py::sub(&py::fsum(&{ let mut out = vec![]; for t798 in py::range_iter(&[(py::len(&v_x.clone())?).i()?]) { let mut v_j: Py; 
v_j = t798;
out.push(py::mul(&py::float(&py::getitem(&py::getitem(&v_A.clone(), &v_k.clone())?, &v_j.clone())?)?, &py::getitem(&v_x.clone(), &v_j.clone())?)?); } Py::list(out) })?, &py::float(&py::getitem(&py::getitem(&v_i.clone(), &Py::str("b"))?, &v_k.clone())?)?)?)?); } Py::list(out) }], true)?, &py::minmax(&[py::add(&py::add(&Py::list(vec![Py::Float(1.0)]), &{ let mut out = vec![]; for t799 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("b"))?))? { let mut v_v: Py; 
v_v = t799;
out.push(py::abs(&py::float(&v_v.clone())?)?); } Py::list(out) })?, &{ let mut out = vec![]; for t800 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t800;
for t801 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t801;
out.push(py::abs(&py::float(&v_v.clone())?)?); }} Py::list(out) })?], true)?)?;
    v_f = py::add(&py::mul(&Py::Float(0.5), &py::fsum(&{ let mut out = vec![]; for t802 in py::range_iter(&[(py::len(&v_x.clone())?).i()?]) { let mut v_a: Py; 
v_a = t802;
for t803 in py::range_iter(&[(py::len(&v_x.clone())?).i()?]) { let mut v_b: Py; 
v_b = t803;
out.push(py::mul(&py::mul(&py::getitem(&v_x.clone(), &v_a.clone())?, &py::float(&py::getitem(&py::getitem(&v_M.clone(), &v_a.clone())?, &v_b.clone())?)?)?, &py::getitem(&v_x.clone(), &v_b.clone())?)?); }} Py::list(out) })?)?, &py::fsum(&{ let mut out = vec![]; for t804 in py::iter(&(py::zip(&[py::getitem(&v_i.clone(), &Py::str("c"))?, v_x.clone()])?))? { let mut v_t: Py; let mut v_v: Py; 
let t805 = (t804).items()?;
if t805.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t805[0].clone();
v_t = t805[1].clone();
out.push(py::mul(&py::float(&v_v.clone())?, &v_t.clone())?); } Py::list(out) })?)?;
    return Ok(py::minmax(&[f__grad_res(py::getitem(&v_i.clone(), &Py::str("Q"))?, py::getitem(&v_i.clone(), &Py::str("c"))?, v_x.clone(), v_atl.clone())?, v_feas.clone(), f__rel(py::getitem(&v_o.clone(), &Py::str("f_min"))?, v_f.clone())?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_erlang_b(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_logs: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let t822 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("A"))?, py::int(&py::getitem(&v_i.clone(), &Py::str("c"))?)?])).items()?;
    if t822.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t822[0].clone();
    v_c = t822[1].clone();
    v_logs = { let mut out = vec![]; for t823 in py::range_iter(&[(py::add(&v_c.clone(), &Py::Int(1i64))?).i()?]) { let mut v_n: Py; 
v_n = t823;
out.push(py::sub(&py::mul(&v_n.clone(), &py::math1("log", &v_A.clone())?)?, &py::math1("lgamma", &py::add(&v_n.clone(), &Py::Int(1i64))?)?)?); } Py::list(out) };
    v_mx = py::minmax(&[v_logs.clone()], true)?;
    v_B = py::truediv(&py::math1("exp", &py::sub(&py::getitem(&v_logs.clone(), &py::neg(&Py::Int(1i64))?)?, &v_mx.clone())?)?, &py::fsum(&{ let mut out = vec![]; for t824 in py::iter(&(v_logs.clone()))? { let mut v_v: Py; 
v_v = t824;
out.push(py::math1("exp", &py::sub(&v_v.clone(), &v_mx.clone())?)?); } Py::list(out) })?)?;
    return Ok(py::minmax(&[f__rr(py::getitem(&v_o.clone(), &Py::str("blocking_probability"))?, v_B.clone())?, f__rr(py::getitem(&v_o.clone(), &Py::str("carried_traffic"))?, py::mul(&v_A.clone(), &py::sub(&Py::Int(1i64), &py::getitem(&v_o.clone(), &Py::str("blocking_probability"))?)?)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_game(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_bad: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    v_M = { let mut out = vec![]; for t4102 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("A"))?))? { let mut v_r: Py; 
v_r = t4102;
out.push({ let mut out = vec![]; for t4103 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t4103;
out.push(py::float(&v_v.clone())?); } Py::list(out) }); } Py::list(out) };
    let t4104 = (Py::tuple(vec![py::len(&v_M.clone())?, py::len(&py::getitem(&v_M.clone(), &Py::Int(0i64))?)?])).items()?;
    if t4104.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t4104[0].clone();
    v_n = t4104[1].clone();
    let t4105 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("row_strategy_example"))?, py::getitem(&v_o.clone(), &Py::str("col_strategy_example"))?, py::getitem(&v_o.clone(), &Py::str("value"))?])).items()?;
    if t4105.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_x = t4105[0].clone();
    v_y = t4105[1].clone();
    v_v = t4105[2].clone();
    if ({ let t4110 = { let t4106 = py::len(&v_x.clone())?; Py::Bool({ let t4107 = v_m.clone(); if py::ne(&t4106, &t4107) { true } else { false } }) }; if t4110.truthy() { t4110 } else { { let t4108 = py::len(&v_y.clone())?; Py::Bool({ let t4109 = v_n.clone(); if py::ne(&t4108, &t4109) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_sc = py::minmax(&[py::add(&Py::list(vec![Py::Float(1.0)]), &{ let mut out = vec![]; for t4111 in py::iter(&(v_M.clone()))? { let mut v_r: Py; 
v_r = t4111;
for t4112 in py::iter(&(v_r.clone()))? { let mut v_a: Py; 
v_a = t4112;
out.push(py::abs(&v_a.clone())?); }} Py::list(out) })?], true)?;
    v_bad = py::minmax(&[py::add(&Py::list(vec![py::abs(&py::sub(&py::sum(&v_x.clone(), None)?, &Py::Int(1i64))?)?, py::abs(&py::sub(&py::sum(&v_y.clone(), None)?, &Py::Int(1i64))?)?]), &{ let mut out = vec![]; for t4113 in py::iter(&(py::add(&v_x.clone(), &v_y.clone())?))? { let mut v_t: Py; 
v_t = t4113;
out.push(py::minmax(&[Py::Float(0.0), py::neg(&v_t.clone())?], true)?); } Py::list(out) })?], true)?;
    v_lo = py::minmax(&[{ let mut out = vec![]; for t4114 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t4114;
out.push(py::sum(&{ let mut out = vec![]; for t4115 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_r: Py; 
v_r = t4115;
out.push(py::mul(&py::getitem(&v_x.clone(), &v_r.clone())?, &py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_j.clone())?)?); } Py::list(out) }, None)?); } Py::list(out) }], false)?;
    v_hi = py::minmax(&[{ let mut out = vec![]; for t4116 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_r: Py; 
v_r = t4116;
out.push(py::sum(&{ let mut out = vec![]; for t4117 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t4117;
out.push(py::mul(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_j.clone())?, &py::getitem(&v_y.clone(), &v_j.clone())?)?); } Py::list(out) }, None)?); } Py::list(out) }], true)?;
    return Ok(py::minmax(&[v_bad.clone(), py::truediv(&py::minmax(&[Py::Float(0.0), py::sub(&v_v.clone(), &v_lo.clone())?], true)?, &v_sc.clone())?, py::truediv(&py::minmax(&[Py::Float(0.0), py::sub(&v_hi.clone(), &v_v.clone())?], true)?, &v_sc.clone())?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_gm(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t69: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_P = f__ahp_matrix(py::getitem(&v_i.clone(), &Py::str("judgments"))?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t69 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    v_w = py::getitem(&v_o.clone(), &Py::str("weights"))?;
    if ({ let t74 = { let t70 = py::len(&v_w.clone())?; Py::Bool({ let t71 = py::len(&v_P.clone())?; if py::ne(&t70, &t71) { true } else { false } }) }; if t74.truthy() { t74 } else { { let t72 = py::minmax(&[v_w.clone()], false)?; Py::Bool({ let t73 = Py::Int(0i64); if py::le(&t72, &t73)? { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_n = py::len(&v_w.clone())?;
    v_res = py::minmax(&[{ let mut out = vec![]; for t75 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_a: Py; 
v_a = t75;
out.push(py::abs(&py::fsum(&{ let mut out = vec![]; for t76 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_b: Py; 
v_b = t76;
out.push(py::add(&py::sub(&py::math1("log", &py::getitem(&py::getitem(&v_P.clone(), &v_a.clone())?, &v_b.clone())?)?, &py::math1("log", &py::getitem(&v_w.clone(), &v_a.clone())?)?)?, &py::math1("log", &py::getitem(&v_w.clone(), &v_b.clone())?)?)?); } Py::list(out) })?)?); } Py::list(out) }], true)?;
    return Ok(py::minmax(&[py::truediv(&v_res.clone(), &v_n.clone())?, py::abs(&py::sub(&py::fsum(&v_w.clone())?, &Py::Int(1i64))?)?, f__rr(py::getitem(&v_o.clone(), &Py::str("max_weight"))?, py::minmax(&[v_w.clone()], true)?)?, (if ({ let t79 = py::int(&py::getitem(&v_o.clone(), &Py::str("best"))?)?; Py::Bool({ let t80 = f__best(v_w.clone())?; if py::eq(&t79, &t80) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) })], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_johnson(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_seq: Py = Py::None;
    v_seq = { let mut out = vec![]; for t1144 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("sequence"))?))? { let mut v_v: Py; 
v_v = t1144;
out.push(py::sub(&py::int(&py::round(&v_v.clone(), None)?)?, &Py::Int(1i64))?); } Py::list(out) };
    if ({ let t1145 = py::sorted(&v_seq.clone(), (Py::Bool(false)).truthy())?; Py::Bool({ let t1146 = Py::list((Py::list((py::range_iter(&[(py::len(&py::getitem(&v_i.clone(), &Py::str("a"))?)?).i()?])).into_iter().collect::<Vec<Py>>())).items()?); if py::ne(&t1145, &t1146) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::minmax(&[f__rel(py::getitem(&v_o.clone(), &Py::str("makespan"))?, py::float(&f__flow2(v_seq.clone(), py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?)?)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("makespan"))?, py::getitem(&f_johnson(py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?)?, &Py::str("makespan"))?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_knap(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_it: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_it = { let mut out = vec![]; for t1191 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("items_example"))?))? { let mut v_x: Py; 
v_x = t1191;
out.push(py::sub(&py::int(&py::round(&v_x.clone(), None)?)?, &Py::Int(1i64))?); } Py::list(out) };
    if ({ let t1198 = { let t1192 = py::len(&py::set(&v_it.clone())?)?; Py::Bool({ let t1193 = py::len(&v_it.clone())?; if py::ne(&t1192, &t1193) { true } else { false } }) }; if t1198.truthy() { t1198 } else { Py::Bool(({ let mut out = vec![]; for t1194 in py::iter(&(v_it.clone()))? { let mut v_k: Py; 
v_k = t1194;
out.push(py::not(&{ let t1195 = Py::Int(0i64); Py::Bool({ let t1196 = v_k.clone(); if py::le(&t1195, &t1196)? { { let t1197 = py::len(&py::getitem(&v_i.clone(), &Py::str("values"))?)?; if py::lt(&t1196, &t1197)? { true } else { false } } } else { false } }) })); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_w = py::sum(&{ let mut out = vec![]; for t1199 in py::iter(&(v_it.clone()))? { let mut v_k: Py; 
v_k = t1199;
out.push(py::getitem(&py::getitem(&v_i.clone(), &Py::str("weights"))?, &v_k.clone())?); } Py::list(out) }, None)?;
    if ({ let t1200 = v_w.clone(); Py::Bool({ let t1201 = py::add(&py::getitem(&v_i.clone(), &Py::str("capacity"))?, &Py::Float(1e-09))?; if py::gt(&t1200, &t1201)? { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::minmax(&[f__rel(py::getitem(&v_o.clone(), &Py::str("max_value"))?, py::sum(&{ let mut out = vec![]; for t1202 in py::iter(&(v_it.clone()))? { let mut v_k: Py; 
v_k = t1202;
out.push(py::getitem(&py::getitem(&v_i.clone(), &Py::str("values"))?, &v_k.clone())?); } Py::list(out) }, None)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("max_value"))?, py::getitem(&f_knapsack01(py::getitem(&v_i.clone(), &Py::str("values"))?, py::getitem(&v_i.clone(), &Py::str("weights"))?, py::getitem(&v_i.clone(), &Py::str("capacity"))?)?, &Py::str("max_value"))?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_maxflow(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_bal: Py = Py::None;
    let mut v_cut: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_val: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_E = f__wedges(py::getitem(&v_i.clone(), &Py::str("edges"))?, Py::Bool(true))?;
    let t1901 = (Py::tuple(vec![py::method(&Py::str(&py::to_str(&py::getitem(&v_i.clone(), &Py::str("source"))?)), "strip", &[])?, py::method(&Py::str(&py::to_str(&py::getitem(&v_i.clone(), &Py::str("target"))?)), "strip", &[])?])).items()?;
    if t1901.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t1901[0].clone();
    v_t = t1901[1].clone();
    v_f = { let mut out = vec![]; for t1902 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("edge_flows"))?))? { let mut v_v: Py; 
v_v = t1902;
out.push(f__fr(v_v.clone())?); } Py::list(out) };
    if ({ let t1913 = { let t1903 = py::len(&v_f.clone())?; Py::Bool({ let t1904 = py::len(&v_E.clone())?; if py::ne(&t1903, &t1904) { true } else { false } }) }; if t1913.truthy() { t1913 } else { Py::Bool(({ let mut out = vec![]; for t1905 in py::iter(&(py::zip(&[v_f.clone(), v_E.clone()])?))? { let mut v__: Py; let mut v_w: Py; let mut v_x: Py; 
let t1906 = (t1905).items()?;
if t1906.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1906[0].clone();
let t1907 = (t1906[1].clone()).items()?;
if t1907.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v__ = t1907[0].clone();
v__ = t1907[1].clone();
v_w = t1907[2].clone();
out.push({ let t1912 = { let t1908 = v_x.clone(); Py::Bool({ let t1909 = Py::Int(0i64); if py::lt(&t1908, &t1909)? { true } else { false } }) }; if t1912.truthy() { t1912 } else { { let t1910 = v_x.clone(); Py::Bool({ let t1911 = v_w.clone(); if py::gt(&t1910, &t1911)? { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_bal = Py::dict(vec![]);
    for t1914 in py::iter(&(py::zip(&[v_f.clone(), v_E.clone()])?))? {
        let t1915 = (t1914).items()?;
        if t1915.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_x = t1915[0].clone();
        let t1916 = (t1915[1].clone()).items()?;
        if t1916.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_u = t1916[0].clone();
        v_v = t1916[1].clone();
        v__ = t1916[2].clone();
        py::setitem(&v_bal.clone(), &v_u.clone(), py::sub(&py::method(&v_bal.clone(), "get", &[v_u.clone(), Py::Int(0i64)])?, &v_x.clone())?)?;
        py::setitem(&v_bal.clone(), &v_v.clone(), py::add(&py::method(&v_bal.clone(), "get", &[v_v.clone(), Py::Int(0i64)])?, &v_x.clone())?)?;
    }
    if (Py::Bool(({ let mut out = vec![]; for t1917 in py::iter(&(py::method(&v_bal.clone(), "items", &[])?))? { let mut v_b: Py; let mut v_k: Py; 
let t1918 = (t1917).items()?;
if t1918.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1918[0].clone();
v_b = t1918[1].clone();
if ({ let t1919 = v_k.clone(); Py::Bool({ let t1920 = Py::tuple(vec![v_s.clone(), v_t.clone()]); if !py::contains(&t1920, &t1919)? { true } else { false } }) }).truthy() { out.push({ let t1921 = v_b.clone(); Py::Bool({ let t1922 = Py::Int(0i64); if py::ne(&t1921, &t1922) { true } else { false } }) }); }} Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_S = py::set(&py::getitem(&v_o.clone(), &Py::str("min_cut_source_side"))?)?;
    if ({ let t1927 = { let t1923 = v_s.clone(); Py::Bool({ let t1924 = v_S.clone(); if !py::contains(&t1924, &t1923)? { true } else { false } }) }; if t1927.truthy() { t1927 } else { { let t1925 = v_t.clone(); Py::Bool({ let t1926 = v_S.clone(); if py::contains(&t1926, &t1925)? { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_cut = py::sum(&{ let mut out = vec![]; for t1928 in py::iter(&(v_E.clone()))? { let mut v_u: Py; let mut v_v: Py; let mut v_w: Py; 
let t1929 = (t1928).items()?;
if t1929.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_u = t1929[0].clone();
v_v = t1929[1].clone();
v_w = t1929[2].clone();
if ({ let t1934 = { let t1930 = v_u.clone(); Py::Bool({ let t1931 = v_S.clone(); if py::contains(&t1931, &t1930)? { true } else { false } }) }; if !t1934.truthy() { t1934 } else { { let t1932 = v_v.clone(); Py::Bool({ let t1933 = v_S.clone(); if !py::contains(&t1933, &t1932)? { true } else { false } }) } } }).truthy() { out.push(v_w.clone()); }} Py::list(out) }, None)?;
    v_val = py::neg(&py::method(&v_bal.clone(), "get", &[v_s.clone(), Py::Int(0i64)])?)?;
    return Ok(py::minmax(&[f__rel(py::getitem(&v_o.clone(), &Py::str("max_flow"))?, py::float(&v_val.clone())?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("max_flow"))?, py::float(&v_cut.clone())?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_mcf(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_arcs: Py = Py::None;
    let mut v_fl: Py = Py::None;
    let mut v_lp: Py = Py::None;
    let t2286: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t2285 = (f__mcf_lp(py::getitem(&v_i.clone(), &Py::str("capacity"))?, py::getitem(&v_i.clone(), &Py::str("cost"))?, py::getitem(&v_i.clone(), &Py::str("supply"))?)?).items()?;
            if t2285.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_arcs = t2285[0].clone();
            v_lp = t2285[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2286 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    v_fl = py::method(&v_o.clone(), "get", &[Py::str("flows"), py::method(&v_o.clone(), "get", &[Py::str("flows_example")])?])?;
    return Ok({ let mut v: Vec<Py> = vec![]; v.extend((v_lp.clone()).items()?); v.push({ let mut out = vec![]; for t2326 in py::iter(&(v_arcs.clone()))? { let mut v_a: Py; let mut v_b: Py; 
let t2327 = (t2326).items()?;
if t2327.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2327[0].clone();
v_b = t2327[1].clone();
out.push(py::getitem(&py::getitem(&v_fl.clone(), &v_a.clone())?, &v_b.clone())?); } Py::list(out) }); v.push(py::getitem(&v_o.clone(), &Py::str("dual"))?); v.push(py::getitem(&v_o.clone(), &Py::str("min_cost"))?); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "sense") { Some(x) => x, None => { return Err(PyErr::Type("_cert_from_lp() missing argument sense".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "c") { Some(x) => x, None => { return Err(PyErr::Type("_cert_from_lp() missing argument c".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "A") { Some(x) => x, None => { return Err(PyErr::Type("_cert_from_lp() missing argument A".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "b") { Some(x) => x, None => { return Err(PyErr::Type("_cert_from_lp() missing argument b".into())) } } }; let a4 = if 4 < v.len() { v[4].clone() } else { match py::kw_take(&mut d, "rel") { Some(x) => x, None => { return Err(PyErr::Type("_cert_from_lp() missing argument rel".into())) } } }; let a5 = if 5 < v.len() { v[5].clone() } else { match py::kw_take(&mut d, "x") { Some(x) => x, None => { return Err(PyErr::Type("_cert_from_lp() missing argument x".into())) } } }; let a6 = if 6 < v.len() { v[6].clone() } else { match py::kw_take(&mut d, "p") { Some(x) => x, None => { return Err(PyErr::Type("_cert_from_lp() missing argument p".into())) } } }; let a7 = if 7 < v.len() { v[7].clone() } else { match py::kw_take(&mut d, "value") { Some(x) => x, None => { return Err(PyErr::Type("_cert_from_lp() missing argument value".into())) } } }; if v.len() > 8 { return Err(PyErr::Type("_cert_from_lp() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("_cert_from_lp() got an unexpected keyword argument".into())) } f__cert_from_lp(a0, a1, a2, a3, a4, a5, a6, a7)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_mdp(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_Qv: Py = Py::None;
    let mut v_Rf: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_bell: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_pol: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let t2064: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t2063 = (f__mdp_in(py::getitem(&v_i.clone(), &Py::str("R"))?, py::getitem(&v_i.clone(), &Py::str("T"))?, py::getitem(&v_i.clone(), &Py::str("gamma"))?)?).items()?;
            if t2063.len() != 5 { return Err(PyErr::Value("unpack".into())); }
            v_Rf = t2063[0].clone();
            v_P = t2063[1].clone();
            v_g = t2063[2].clone();
            v_S = t2063[3].clone();
            v_A = t2063[4].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2064 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    v_V = py::getitem(&v_o.clone(), &Py::str("values"))?;
    if ({ let t2065 = py::len(&v_V.clone())?; Py::Bool({ let t2066 = v_S.clone(); if py::ne(&t2065, &t2066) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_g = py::float(&v_g.clone())?;
    v_sc = py::minmax(&[py::add(&Py::list(vec![Py::Float(1.0)]), &{ let mut out = vec![]; for t2067 in py::iter(&(v_V.clone()))? { let mut v_v: Py; 
v_v = t2067;
out.push(py::abs(&v_v.clone())?); } Py::list(out) })?], true)?;
    v_Qv = { let mut out = vec![]; for t2068 in py::range_iter(&[(v_S.clone()).i()?]) { let mut v_s: Py; 
v_s = t2068;
out.push({ let mut out = vec![]; for t2069 in py::range_iter(&[(v_A.clone()).i()?]) { let mut v_a: Py; 
v_a = t2069;
out.push(py::add(&py::float(&py::getitem(&py::getitem(&v_Rf.clone(), &v_s.clone())?, &v_a.clone())?)?, &py::mul(&v_g.clone(), &py::fsum(&{ let mut out = vec![]; for t2070 in py::range_iter(&[(v_S.clone()).i()?]) { let mut v_t: Py; 
v_t = t2070;
out.push(py::mul(&py::float(&py::getitem(&py::getitem(&v_P.clone(), &py::add(&py::mul(&v_s.clone(), &v_A.clone())?, &v_a.clone())?)?, &v_t.clone())?)?, &py::getitem(&v_V.clone(), &v_t.clone())?)?); } Py::list(out) })?)?)?); } Py::list(out) }); } Py::list(out) };
    v_bell = py::truediv(&py::minmax(&[{ let mut out = vec![]; for t2071 in py::range_iter(&[(v_S.clone()).i()?]) { let mut v_s: Py; 
v_s = t2071;
out.push(py::abs(&py::sub(&py::getitem(&v_V.clone(), &v_s.clone())?, &py::minmax(&[py::getitem(&v_Qv.clone(), &v_s.clone())?], true)?)?)?); } Py::list(out) }], true)?, &v_sc.clone())?;
    v_pol = py::truediv(&py::minmax(&[{ let mut out = vec![]; for t2072 in py::range_iter(&[(v_S.clone()).i()?]) { let mut v_s: Py; 
v_s = t2072;
out.push(py::sub(&py::minmax(&[py::getitem(&v_Qv.clone(), &v_s.clone())?], true)?, &py::getitem(&py::getitem(&v_Qv.clone(), &v_s.clone())?, &py::sub(&py::int(&py::getitem(&py::getitem(&v_o.clone(), &Py::str("policy"))?, &v_s.clone())?)?, &Py::Int(1i64))?)?)?); } Py::list(out) }], true)?, &v_sc.clone())?;
    return Ok(py::minmax(&[v_bell.clone(), v_pol.clone(), f__rel(py::getitem(&v_o.clone(), &Py::str("value_state_1"))?, py::getitem(&v_V.clone(), &Py::Int(0i64))?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_mixed(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a11: Py = Py::None;
    let mut v_a12: Py = Py::None;
    let mut v_a21: Py = Py::None;
    let mut v_a22: Py = Py::None;
    let mut v_b11: Py = Py::None;
    let mut v_b12: Py = Py::None;
    let mut v_b21: Py = Py::None;
    let mut v_b22: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t2504 = ({ let mut out = vec![]; for t2502 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("A"))?))? { let mut v_r: Py; 
v_r = t2502;
out.push({ let mut out = vec![]; for t2503 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t2503;
out.push(py::float(&v_v.clone())?); } Py::list(out) }); } Py::list(out) }).items()?;
    if t2504.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    let t2505 = (t2504[0].clone()).items()?;
    if t2505.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a11 = t2505[0].clone();
    v_a12 = t2505[1].clone();
    let t2506 = (t2504[1].clone()).items()?;
    if t2506.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a21 = t2506[0].clone();
    v_a22 = t2506[1].clone();
    let t2509 = ({ let mut out = vec![]; for t2507 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("B"))?))? { let mut v_r: Py; 
v_r = t2507;
out.push({ let mut out = vec![]; for t2508 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t2508;
out.push(py::float(&v_v.clone())?); } Py::list(out) }); } Py::list(out) }).items()?;
    if t2509.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    let t2510 = (t2509[0].clone()).items()?;
    if t2510.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_b11 = t2510[0].clone();
    v_b12 = t2510[1].clone();
    let t2511 = (t2509[1].clone()).items()?;
    if t2511.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_b21 = t2511[0].clone();
    v_b22 = t2511[1].clone();
    let t2512 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("p_row1"))?, py::getitem(&v_o.clone(), &Py::str("q_col1"))?])).items()?;
    if t2512.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t2512[0].clone();
    v_q = t2512[1].clone();
    if (py::not(&{ let t2519 = { let t2513 = Py::Int(0i64); Py::Bool({ let t2514 = v_p.clone(); if py::lt(&t2513, &t2514)? { { let t2515 = Py::Int(1i64); if py::lt(&t2514, &t2515)? { true } else { false } } } else { false } }) }; if !t2519.truthy() { t2519 } else { { let t2516 = Py::Int(0i64); Py::Bool({ let t2517 = v_q.clone(); if py::lt(&t2516, &t2517)? { { let t2518 = Py::Int(1i64); if py::lt(&t2517, &t2518)? { true } else { false } } } else { false } }) } } })).truthy() {
        return Ok(Py::Float(1.0));
    }
    let t2520 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("row_payoff"))?, py::getitem(&v_o.clone(), &Py::str("col_payoff"))?])).items()?;
    if t2520.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_u = t2520[0].clone();
    v_w = t2520[1].clone();
    return Ok(py::minmax(&[f__rel(py::add(&py::mul(&v_q.clone(), &v_a11.clone())?, &py::mul(&py::sub(&Py::Int(1i64), &v_q.clone())?, &v_a12.clone())?)?, v_u.clone())?, f__rel(py::add(&py::mul(&v_q.clone(), &v_a21.clone())?, &py::mul(&py::sub(&Py::Int(1i64), &v_q.clone())?, &v_a22.clone())?)?, v_u.clone())?, f__rel(py::add(&py::mul(&v_p.clone(), &v_b11.clone())?, &py::mul(&py::sub(&Py::Int(1i64), &v_p.clone())?, &v_b21.clone())?)?, v_w.clone())?, f__rel(py::add(&py::mul(&v_p.clone(), &v_b12.clone())?, &py::mul(&py::sub(&Py::Int(1i64), &v_p.clone())?, &v_b22.clone())?)?, v_w.clone())?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_mm1(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_lam: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let t2531 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("lam"))?, py::getitem(&v_i.clone(), &Py::str("mu"))?])).items()?;
    if t2531.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lam = t2531[0].clone();
    v_mu = t2531[1].clone();
    return Ok(py::minmax(&[f__rel(py::getitem(&v_o.clone(), &Py::str("L"))?, py::mul(&v_lam.clone(), &py::getitem(&v_o.clone(), &Py::str("W"))?)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("Lq"))?, py::mul(&v_lam.clone(), &py::getitem(&v_o.clone(), &Py::str("Wq"))?)?)?, f__rel(py::sub(&py::getitem(&v_o.clone(), &Py::str("W"))?, &py::getitem(&v_o.clone(), &Py::str("Wq"))?)?, py::truediv(&Py::Int(1i64), &v_mu.clone())?)?, py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("W"))?, &py::sub(&v_mu.clone(), &v_lam.clone())?)?, &Py::Int(1i64))?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("P0"))?, py::sub(&Py::Int(1i64), &py::truediv(&v_lam.clone(), &v_mu.clone())?)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_mmc(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p0: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_terms: Py = Py::None;
    let t2549 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("lam"))?, py::getitem(&v_i.clone(), &Py::str("mu"))?, py::int(&py::getitem(&v_i.clone(), &Py::str("c"))?)?])).items()?;
    if t2549.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_lam = t2549[0].clone();
    v_mu = t2549[1].clone();
    v_c = t2549[2].clone();
    let t2550 = (Py::tuple(vec![py::truediv(&v_lam.clone(), &v_mu.clone())?, py::truediv(&v_lam.clone(), &py::mul(&v_c.clone(), &v_mu.clone())?)?])).items()?;
    if t2550.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2550[0].clone();
    v_rho = t2550[1].clone();
    let t2551 = (Py::tuple(vec![Py::Float(0.0), Py::Float(1.0)])).items()?;
    if t2551.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_terms = t2551[0].clone();
    v_t = t2551[1].clone();
    for t2552 in py::range_iter(&[(v_c.clone()).i()?]) {
        v_n = t2552;
        v_terms = py::iadd(&v_terms, &v_t.clone())?;
        v_t = py::mul(&v_t, &py::truediv(&v_a.clone(), &py::add(&v_n.clone(), &Py::Int(1i64))?)?)?;
    }
    v_p0 = py::truediv(&Py::Int(1i64), &py::add(&v_terms.clone(), &py::truediv(&v_t.clone(), &py::sub(&Py::Int(1i64), &v_rho.clone())?)?)?)?;
    return Ok(py::minmax(&[f__rr(py::getitem(&v_o.clone(), &Py::str("L"))?, py::mul(&v_lam.clone(), &py::getitem(&v_o.clone(), &Py::str("W"))?)?)?, f__rr(py::getitem(&v_o.clone(), &Py::str("Lq"))?, py::mul(&v_lam.clone(), &py::getitem(&v_o.clone(), &Py::str("Wq"))?)?)?, f__rr(py::sub(&py::getitem(&v_o.clone(), &Py::str("W"))?, &py::getitem(&v_o.clone(), &Py::str("Wq"))?)?, py::truediv(&Py::Int(1i64), &v_mu.clone())?)?, f__rr(py::getitem(&v_o.clone(), &Py::str("P0"))?, v_p0.clone())?, f__rr(py::getitem(&v_o.clone(), &Py::str("P_wait"))?, py::truediv(&py::mul(&v_p0.clone(), &v_t.clone())?, &py::sub(&Py::Int(1i64), &v_rho.clone())?)?)?, f__rr(py::getitem(&v_o.clone(), &Py::str("Lq"))?, py::truediv(&py::mul(&py::getitem(&v_o.clone(), &Py::str("P_wait"))?, &v_rho.clone())?, &py::sub(&Py::Int(1i64), &v_rho.clone())?)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_mmck(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_lam: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let t2623 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("lam"))?, py::getitem(&v_i.clone(), &Py::str("mu"))?])).items()?;
    if t2623.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lam = t2623[0].clone();
    v_mu = t2623[1].clone();
    return Ok(py::minmax(&[f__rel(py::getitem(&v_o.clone(), &Py::str("lam_eff"))?, py::mul(&v_lam.clone(), &py::sub(&Py::Int(1i64), &py::getitem(&v_o.clone(), &Py::str("P_block"))?)?)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("L"))?, py::mul(&py::getitem(&v_o.clone(), &Py::str("lam_eff"))?, &py::getitem(&v_o.clone(), &Py::str("W"))?)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("Lq"))?, py::mul(&py::getitem(&v_o.clone(), &Py::str("lam_eff"))?, &py::getitem(&v_o.clone(), &Py::str("Wq"))?)?)?, f__rel(py::sub(&py::getitem(&v_o.clone(), &Py::str("L"))?, &py::getitem(&v_o.clone(), &Py::str("Lq"))?)?, py::truediv(&py::getitem(&v_o.clone(), &Py::str("lam_eff"))?, &v_mu.clone())?)?, f__rel(py::sub(&py::getitem(&v_o.clone(), &Py::str("W"))?, &py::getitem(&v_o.clone(), &Py::str("Wq"))?)?, py::truediv(&Py::Int(1i64), &v_mu.clone())?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_moore(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_on: Py = Py::None;
    let mut v_tardy: Py = Py::None;
    v_on = { let mut out = vec![]; for t2638 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("on_time_sequence"))?))? { let mut v_v: Py; 
v_v = t2638;
out.push(py::sub(&py::int(&py::round(&v_v.clone(), None)?)?, &Py::Int(1i64))?); } Py::list(out) };
    v_tardy = { let mut out = vec![]; for t2639 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("tardy_jobs"))?))? { let mut v_v: Py; 
v_v = t2639;
out.push(py::sub(&py::int(&py::round(&v_v.clone(), None)?)?, &Py::Int(1i64))?); } Py::list(out) };
    if ({ let t2644 = { let t2640 = py::sorted(&py::add(&v_on.clone(), &v_tardy.clone())?, (Py::Bool(false)).truthy())?; Py::Bool({ let t2641 = Py::list((Py::list((py::range_iter(&[(py::len(&py::getitem(&v_i.clone(), &Py::str("p"))?)?).i()?])).into_iter().collect::<Vec<Py>>())).items()?); if py::ne(&t2640, &t2641) { true } else { false } }) }; if t2644.truthy() { t2644 } else { { let t2642 = py::getitem(&v_o.clone(), &Py::str("min_tardy_jobs"))?; Py::Bool({ let t2643 = py::len(&v_tardy.clone())?; if py::ne(&t2642, &t2643) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    for t2647 in py::iter(&(py::method(&f__completion(v_on.clone(), py::getitem(&v_i.clone(), &Py::str("p"))?)?, "items", &[])?))? {
        let t2650 = (t2647).items()?;
        if t2650.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_j = t2650[0].clone();
        v_c = t2650[1].clone();
        if ({ let t2648 = v_c.clone(); Py::Bool({ let t2649 = f__fr(py::getitem(&py::getitem(&v_i.clone(), &Py::str("d"))?, &v_j.clone())?)?; if py::gt(&t2648, &t2649)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(1.0));
        }
    }
    return Ok((if ({ let t2651 = py::getitem(&f_moore_hodgson(py::getitem(&v_i.clone(), &Py::str("p"))?, py::getitem(&v_i.clone(), &Py::str("d"))?)?, &Py::str("min_tardy_jobs"))?; Py::Bool({ let t2652 = py::getitem(&v_o.clone(), &Py::str("min_tardy_jobs"))?; if py::eq(&t2651, &t2652) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_mst(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_cand: Py = Py::None;
    let mut v_frontier: Py = Py::None;
    let mut v_inn: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_names: Py = Py::None;
    let mut v_nodes: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_E = f__wedges(py::getitem(&v_i.clone(), &Py::str("edges"))?, Py::Bool(true))?;
    v_nodes = f__nodes(v_E.clone())?;
    v_names = py::method(&v_o.clone(), "get", &[Py::str("tree_edges"), py::method(&v_o.clone(), "get", &[Py::str("tree_edges_example"), Py::list(vec![])])?])?;
    if ({ let t2371 = py::len(&v_names.clone())?; Py::Bool({ let t2372 = py::sub(&py::len(&v_nodes.clone())?, &Py::Int(1i64))?; if py::ne(&t2371, &t2372) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_best = Py::dict(vec![]);
    for t2373 in py::iter(&(v_E.clone()))? {
        let t2375 = (t2373).items()?;
        if t2375.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_u = t2375[0].clone();
        v_v = t2375[1].clone();
        v_w = t2375[2].clone();
        for t2374 in py::iter(&(Py::tuple(vec![Py::str(&[py::format(&v_u.clone(), "")?, "-".to_string(), py::format(&v_v.clone(), "")?].concat()), Py::str(&[py::format(&v_v.clone(), "")?, "-".to_string(), py::format(&v_u.clone(), "")?].concat())])))? {
            v_key = t2374;
            py::setitem(&v_best.clone(), &v_key.clone(), py::minmax(&[py::method(&v_best.clone(), "get", &[v_key.clone(), v_w.clone()])?, v_w.clone()], false)?)?;
        }
    }
    if (Py::Bool(({ let mut out = vec![]; for t2376 in py::iter(&(v_names.clone()))? { let mut v_nm: Py; 
v_nm = t2376;
out.push({ let t2377 = v_nm.clone(); Py::Bool({ let t2378 = v_best.clone(); if !py::contains(&t2378, &t2377)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_tot = py::sum(&{ let mut out = vec![]; for t2379 in py::iter(&(v_names.clone()))? { let mut v_nm: Py; 
v_nm = t2379;
out.push(py::getitem(&v_best.clone(), &v_nm.clone())?); } Py::list(out) }, None)?;
    let t2380 = (Py::tuple(vec![py::set(&Py::list(vec![py::getitem(&v_nodes.clone(), &Py::Int(0i64))?]))?, py::fraction(&[Py::Int(0i64)])?])).items()?;
    if t2380.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_inn = t2380[0].clone();
    v_frontier = t2380[1].clone();
    while ({ let t2396 = py::len(&v_inn.clone())?; Py::Bool({ let t2397 = py::len(&v_nodes.clone())?; if py::lt(&t2396, &t2397)? { true } else { false } }) }).truthy() {
        v_cand = py::add(&{ let mut out = vec![]; for t2381 in py::iter(&(v_E.clone()))? { let mut v_u: Py; let mut v_v: Py; let mut v_w: Py; 
let t2382 = (t2381).items()?;
if t2382.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_u = t2382[0].clone();
v_v = t2382[1].clone();
v_w = t2382[2].clone();
if ({ let t2387 = { let t2383 = v_u.clone(); Py::Bool({ let t2384 = v_inn.clone(); if py::contains(&t2384, &t2383)? { true } else { false } }) }; if !t2387.truthy() { t2387 } else { { let t2385 = v_v.clone(); Py::Bool({ let t2386 = v_inn.clone(); if !py::contains(&t2386, &t2385)? { true } else { false } }) } } }).truthy() { out.push(Py::tuple(vec![v_w.clone(), v_v.clone()])); }} Py::list(out) }, &{ let mut out = vec![]; for t2388 in py::iter(&(v_E.clone()))? { let mut v_u: Py; let mut v_v: Py; let mut v_w: Py; 
let t2389 = (t2388).items()?;
if t2389.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_u = t2389[0].clone();
v_v = t2389[1].clone();
v_w = t2389[2].clone();
if ({ let t2394 = { let t2390 = v_v.clone(); Py::Bool({ let t2391 = v_inn.clone(); if py::contains(&t2391, &t2390)? { true } else { false } }) }; if !t2394.truthy() { t2394 } else { { let t2392 = v_u.clone(); Py::Bool({ let t2393 = v_inn.clone(); if !py::contains(&t2393, &t2392)? { true } else { false } }) } } }).truthy() { out.push(Py::tuple(vec![v_w.clone(), v_u.clone()])); }} Py::list(out) })?;
        if (py::not(&v_cand.clone())).truthy() {
            return Ok(Py::Float(1.0));
        }
        let t2395 = (py::minmax(&[v_cand.clone()], false)?).items()?;
        if t2395.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_w = t2395[0].clone();
        v_v = t2395[1].clone();
        v_frontier = py::iadd(&v_frontier, &v_w.clone())?;
        let _ = py::method(&v_inn.clone(), "add", &[v_v.clone()])?;
    }
    return Ok(py::minmax(&[f__rel(py::getitem(&v_o.clone(), &Py::str("total_weight"))?, py::float(&v_tot.clone())?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("total_weight"))?, py::float(&v_frontier.clone())?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_mwm(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_W: Py = Py::None;
    let mut v_pairs: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_used: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_E = f__wedges(py::getitem(&v_i.clone(), &Py::str("edges"))?, Py::Bool(true))?;
    v_W = Py::dict(vec![]);
    for t1974 in py::iter(&(v_E.clone()))? {
        let t1976 = (t1974).items()?;
        if t1976.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_u = t1976[0].clone();
        v_v = t1976[1].clone();
        v_w = t1976[2].clone();
        let t1975 = py::minmax(&[py::method(&v_W.clone(), "get", &[Py::tuple(vec![v_u.clone(), v_v.clone()]), v_w.clone()])?, v_w.clone()], true)?;
        py::setitem(&v_W.clone(), &Py::tuple(vec![v_u.clone(), v_v.clone()]), t1975.clone())?;
        py::setitem(&v_W.clone(), &Py::tuple(vec![v_v.clone(), v_u.clone()]), t1975.clone())?;
    }
    v_pairs = { let mut out = vec![]; for t1977 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("matching_example"))?))? { let mut v_s: Py; 
v_s = t1977;
out.push(Py::tuple((py::method(&v_s.clone(), "split", &[Py::str("-"), Py::Int(1i64)])?).items()?)); } Py::list(out) };
    v_used = { let mut out = vec![]; for t1978 in py::iter(&(v_pairs.clone()))? { let mut v_p: Py; 
v_p = t1978;
for t1979 in py::iter(&(v_p.clone()))? { let mut v_x: Py; 
v_x = t1979;
out.push(v_x.clone()); }} Py::list(out) };
    if ({ let t1985 = { let t1980 = py::len(&v_used.clone())?; Py::Bool({ let t1981 = py::len(&py::set(&v_used.clone())?)?; if py::ne(&t1980, &t1981) { true } else { false } }) }; if t1985.truthy() { t1985 } else { Py::Bool(({ let mut out = vec![]; for t1982 in py::iter(&(v_pairs.clone()))? { let mut v_p: Py; 
v_p = t1982;
out.push({ let t1983 = v_p.clone(); Py::Bool({ let t1984 = v_W.clone(); if !py::contains(&t1984, &t1983)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::minmax(&[f__rel(py::getitem(&v_o.clone(), &Py::str("max_weight"))?, py::float(&py::sum(&{ let mut out = vec![]; for t1986 in py::iter(&(v_pairs.clone()))? { let mut v_p: Py; 
v_p = t1986;
out.push(py::getitem(&v_W.clone(), &v_p.clone())?); } Py::list(out) }, None)?)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("max_weight"))?, py::getitem(&f_max_weight_matching(py::getitem(&v_i.clone(), &Py::str("edges"))?)?, &Py::str("max_weight"))?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_parallel(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_ks: Py = Py::None;
    let mut v_loads: Py = Py::None;
    let mut v_m: Py = Py::None;
    v_m = py::int(&py::getitem(&v_i.clone(), &Py::str("m"))?)?;
    v_ks = { let mut out = vec![]; for t2770 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("machine_of_job"))?))? { let mut v_v: Py; 
v_v = t2770;
out.push(py::sub(&py::int(&py::round(&v_v.clone(), None)?)?, &Py::Int(1i64))?); } Py::list(out) };
    if ({ let t2777 = { let t2771 = py::len(&v_ks.clone())?; Py::Bool({ let t2772 = py::len(&py::getitem(&v_i.clone(), &Py::str("p"))?)?; if py::ne(&t2771, &t2772) { true } else { false } }) }; if t2777.truthy() { t2777 } else { Py::Bool(({ let mut out = vec![]; for t2773 in py::iter(&(v_ks.clone()))? { let mut v_k: Py; 
v_k = t2773;
out.push(py::not(&{ let t2774 = Py::Int(0i64); Py::Bool({ let t2775 = v_k.clone(); if py::le(&t2774, &t2775)? { { let t2776 = v_m.clone(); if py::lt(&t2775, &t2776)? { true } else { false } } } else { false } }) })); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_loads = py::mul(&Py::list(vec![Py::Float(0.0)]), &v_m.clone())?;
    for t2778 in py::iter(&(py::enumerate(&v_ks.clone(), 0)?))? {
        let t2781 = (t2778).items()?;
        if t2781.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_j = t2781[0].clone();
        v_k = t2781[1].clone();
        let t2779 = v_loads.clone(); let t2780 = v_k.clone();
        py::setitem(&t2779, &t2780, py::iadd(&py::getitem(&t2779, &t2780)?, &py::getitem(&py::getitem(&v_i.clone(), &Py::str("p"))?, &v_j.clone())?)?)?;
    }
    return Ok(py::minmax(&[f__rel(py::minmax(&[v_loads.clone()], true)?, py::getitem(&v_o.clone(), &Py::str("makespan"))?)?, f__rel(py::getitem(&f_parallel_makespan(py::getitem(&v_i.clone(), &Py::str("p"))?, v_m.clone())?, &Py::str("makespan"))?, py::getitem(&v_o.clone(), &Py::str("makespan"))?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_pe(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_cand: Py = Py::None;
    let mut v_cs: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_vals: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_cs = { let mut out = vec![]; for t2925 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("coefficients"))?))? { let mut v_v: Py; 
v_v = t2925;
out.push(py::float(&v_v.clone())?); } Py::list(out) };
    let t2926 = (Py::tuple(vec![py::float(&py::getitem(&v_i.clone(), &Py::str("lo"))?)?, py::float(&py::getitem(&v_i.clone(), &Py::str("hi"))?)?])).items()?;
    if t2926.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t2926[0].clone();
    v_hi = t2926[1].clone();
    v_cand = py::add(&Py::list(vec![v_lo.clone(), v_hi.clone()]), &f__roots_in(f__pder(v_cs.clone())?, v_lo.clone(), v_hi.clone())?)?;
    v_vals = { let mut out = vec![]; for t2927 in py::iter(&(v_cand.clone()))? { let mut v_x: Py; 
v_x = t2927;
out.push(f__peval(v_cs.clone(), v_x.clone())?); } Py::list(out) };
    v_out = Py::list(vec![f__rel(py::getitem(&v_o.clone(), &Py::str("min_value"))?, py::minmax(&[v_vals.clone()], false)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("max_value"))?, py::minmax(&[v_vals.clone()], true)?)?]);
    for t2928 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::str("argmin_example"), py::getitem(&v_o.clone(), &Py::str("min_value"))?]), Py::tuple(vec![Py::str("argmax_example"), py::getitem(&v_o.clone(), &Py::str("max_value"))?])])))? {
        let t2932 = (t2928).items()?;
        if t2932.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_key = t2932[0].clone();
        v_v = t2932[1].clone();
        v_x = py::getitem(&v_o.clone(), &v_key.clone())?;
        let _ = py::method(&v_out.clone(), "append", &[(if (py::not(&{ let t2929 = v_lo.clone(); Py::Bool({ let t2930 = v_x.clone(); if py::le(&t2929, &t2930)? { { let t2931 = v_hi.clone(); if py::le(&t2930, &t2931)? { true } else { false } } } else { false } }) })).truthy() { Py::Float(1.0) } else { f__rel(f__peval(v_cs.clone(), v_x.clone())?, v_v.clone())? })])?;
    }
    return Ok(py::minmax(&[v_out.clone()], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_pert(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_names: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_te: Py = Py::None;
    let mut v_times: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_times = py::getitem(&v_i.clone(), &Py::str("times"))?;
    v_te = { let mut out = vec![]; for t2855 in py::iter(&(v_times.clone()))? { let mut v_a: Py; let mut v_b: Py; let mut v_m: Py; 
let t2856 = (t2855).items()?;
if t2856.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_a = t2856[0].clone();
v_m = t2856[1].clone();
v_b = t2856[2].clone();
out.push(py::truediv(&py::add(&py::add(&v_a.clone(), &py::mul(&Py::Int(4i64), &v_m.clone())?)?, &v_b.clone())?, &Py::Int(6i64))?); } Py::list(out) };
    v_names = { let mut out = vec![]; for t2857 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("critical_path"))?))? { let mut v_c: Py; 
v_c = t2857;
out.push(py::sub(&py::ord(&v_c.clone())?, &Py::Int(65i64))?); } Py::list(out) };
    v_worst = f__rel(py::getitem(&v_o.clone(), &Py::str("expected_duration"))?, py::sum(&{ let mut out = vec![]; for t2858 in py::iter(&(v_names.clone()))? { let mut v_k: Py; 
v_k = t2858;
out.push(py::getitem(&v_te.clone(), &v_k.clone())?); } Py::list(out) }, None)?)?;
    v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_o.clone(), &Py::str("variance"))?, py::sum(&{ let mut out = vec![]; for t2859 in py::iter(&(v_names.clone()))? { let mut v_k: Py; 
v_k = t2859;
out.push(py::pow(&py::truediv(&py::sub(&py::getitem(&py::getitem(&v_times.clone(), &v_k.clone())?, &Py::Int(2i64))?, &py::getitem(&py::getitem(&v_times.clone(), &v_k.clone())?, &Py::Int(0i64))?)?, &Py::Int(6i64))?, &Py::Int(2i64))?); } Py::list(out) }, None)?)?], true)?;
    let t2860: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f_pert(v_times.clone(), py::method(&v_i.clone(), "get", &[Py::str("precedences")])?, py::method(&v_i.clone(), "get", &[Py::str("deadline")])?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2860 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_o.clone(), &Py::str("expected_duration"))?, py::getitem(&v_r.clone(), &Py::str("expected_duration"))?)?], true)?;
    if ({ let t2861 = Py::str("probability"); Py::Bool({ let t2862 = v_r.clone(); if py::contains(&t2862, &t2861)? { true } else { false } }) }).truthy() {
        v_worst = py::minmax(&[v_worst.clone(), py::abs(&py::sub(&py::method(&v_o.clone(), "get", &[Py::str("probability"), py::neg(&Py::Int(1i64))?])?, &py::getitem(&v_r.clone(), &Py::str("probability"))?)?)?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_pure_ne(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_eq: Py = Py::None;
    let mut v_same: Py = Py::None;
    let t2973: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_eq = { let mut v: Vec<Py> = vec![]; v.extend((f__bimatrix(py::getitem(&v_i.clone(), &Py::str("A"))?, py::getitem(&v_i.clone(), &Py::str("B"))?, Py::Int(8i64))?).items()?); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "MA") { Some(x) => x, None => { return Err(PyErr::Type("_pure_ne() missing argument MA".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "MB") { Some(x) => x, None => { return Err(PyErr::Type("_pure_ne() missing argument MB".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("_pure_ne() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("_pure_ne() got an unexpected keyword argument".into())) } f__pure_ne(a0, a1)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2973 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    v_same = { let t2974 = { let mut out = vec![]; for t2976 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("equilibria"))?))? { let mut v_a: Py; let mut v_b: Py; 
let t2977 = (t2976).items()?;
if t2977.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2977[0].clone();
v_b = t2977[1].clone();
out.push(Py::list(vec![py::int(&py::round(&v_a.clone(), None)?)?, py::int(&py::round(&v_b.clone(), None)?)?])); } Py::list(out) }; Py::Bool({ let t2975 = v_eq.clone(); if py::eq(&t2974, &t2975) { true } else { false } }) };
    return Ok((if ({ let t2980 = v_same.clone(); if !t2980.truthy() { t2980 } else { { let t2978 = py::getitem(&v_o.clone(), &Py::str("count"))?; Py::Bool({ let t2979 = py::len(&v_eq.clone())?; if py::eq(&t2978, &t2979) { true } else { false } }) } } }).truthy() { Py::Float(0.0) } else { py::minmax(&[Py::Float(1.0), py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("count"))?, &py::len(&v_eq.clone())?)?)?], true)? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_qmin(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t3649: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_M = f__sym_pd(py::getitem(&v_i.clone(), &Py::str("Q"))?, Py::str("Q"))?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3649 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    v_x = py::getitem(&v_o.clone(), &Py::str("x_star"))?;
    if ({ let t3650 = py::len(&v_x.clone())?; Py::Bool({ let t3651 = py::len(&v_M.clone())?; if py::ne(&t3650, &t3651) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_f = py::add(&py::mul(&Py::Float(0.5), &py::fsum(&{ let mut out = vec![]; for t3652 in py::range_iter(&[(py::len(&v_x.clone())?).i()?]) { let mut v_a: Py; 
v_a = t3652;
for t3653 in py::range_iter(&[(py::len(&v_x.clone())?).i()?]) { let mut v_b: Py; 
v_b = t3653;
out.push(py::mul(&py::mul(&py::getitem(&v_x.clone(), &v_a.clone())?, &py::float(&py::getitem(&py::getitem(&v_M.clone(), &v_a.clone())?, &v_b.clone())?)?)?, &py::getitem(&v_x.clone(), &v_b.clone())?)?); }} Py::list(out) })?)?, &py::fsum(&{ let mut out = vec![]; for t3654 in py::iter(&(py::zip(&[py::getitem(&v_i.clone(), &Py::str("c"))?, v_x.clone()])?))? { let mut v_t: Py; let mut v_v: Py; 
let t3655 = (t3654).items()?;
if t3655.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t3655[0].clone();
v_t = t3655[1].clone();
out.push(py::mul(&py::float(&v_v.clone())?, &v_t.clone())?); } Py::list(out) })?)?;
    return Ok(py::minmax(&[f__grad_res(py::getitem(&v_i.clone(), &Py::str("Q"))?, py::getitem(&v_i.clone(), &Py::str("c"))?, v_x.clone(), Py::None)?, f__rel(py::getitem(&v_o.clone(), &Py::str("f_min"))?, v_f.clone())?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_sd(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_Qf: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_f1: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_g1: Py = Py::None;
    let mut v_gn: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_orth: Py = Py::None;
    let mut v_step: Py = Py::None;
    let mut v_x0: Py = Py::None;
    let mut v_x1: Py = Py::None;
    let t3219: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_M = f__sym_pd(py::getitem(&v_i.clone(), &Py::str("Q"))?, Py::str("Q"))?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3219 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    v_n = py::len(&v_M.clone())?;
    let t3224 = (Py::tuple(vec![{ let mut out = vec![]; for t3220 in py::iter(&(v_M.clone()))? { let mut v_r: Py; 
v_r = t3220;
out.push({ let mut out = vec![]; for t3221 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t3221;
out.push(py::float(&v_v.clone())?); } Py::list(out) }); } Py::list(out) }, { let mut out = vec![]; for t3222 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("c"))?))? { let mut v_v: Py; 
v_v = t3222;
out.push(py::float(&v_v.clone())?); } Py::list(out) }, { let mut out = vec![]; for t3223 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("x0"))?))? { let mut v_v: Py; 
v_v = t3223;
out.push(py::float(&v_v.clone())?); } Py::list(out) }])).items()?;
    if t3224.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Qf = t3224[0].clone();
    v_c = t3224[1].clone();
    v_x0 = t3224[2].clone();
    v_g = { let mut out = vec![]; for t3225 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_a: Py; 
v_a = t3225;
out.push(py::add(&py::fsum(&{ let mut out = vec![]; for t3226 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_b: Py; 
v_b = t3226;
out.push(py::mul(&py::getitem(&py::getitem(&v_Qf.clone(), &v_a.clone())?, &v_b.clone())?, &py::getitem(&v_x0.clone(), &v_b.clone())?)?); } Py::list(out) })?, &py::getitem(&v_c.clone(), &v_a.clone())?)?); } Py::list(out) };
    v_x1 = py::getitem(&v_o.clone(), &Py::str("x1"))?;
    v_g1 = { let mut out = vec![]; for t3227 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_a: Py; 
v_a = t3227;
out.push(py::add(&py::fsum(&{ let mut out = vec![]; for t3228 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_b: Py; 
v_b = t3228;
out.push(py::mul(&py::getitem(&py::getitem(&v_Qf.clone(), &v_a.clone())?, &v_b.clone())?, &py::getitem(&v_x1.clone(), &v_b.clone())?)?); } Py::list(out) })?, &py::getitem(&v_c.clone(), &v_a.clone())?)?); } Py::list(out) };
    v_gn = py::fsum(&{ let mut out = vec![]; for t3229 in py::iter(&(v_g.clone()))? { let mut v_v: Py; 
v_v = t3229;
out.push(py::mul(&v_v.clone(), &v_v.clone())?); } Py::list(out) })?;
    v_orth = py::truediv(&py::abs(&py::fsum(&{ let mut out = vec![]; for t3230 in py::iter(&(py::zip(&[v_g.clone(), v_g1.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3231 = (t3230).items()?;
if t3231.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3231[0].clone();
v_b = t3231[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?)?, &v_gn.clone())?;
    v_step = py::minmax(&[{ let mut out = vec![]; for t3232 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_a: Py; 
v_a = t3232;
out.push(f__rel(py::getitem(&v_x1.clone(), &v_a.clone())?, py::sub(&py::getitem(&v_x0.clone(), &v_a.clone())?, &py::mul(&py::getitem(&v_o.clone(), &Py::str("alpha"))?, &py::getitem(&v_g.clone(), &v_a.clone())?)?)?)?); } Py::list(out) }], true)?;
    v_f1 = py::add(&py::mul(&Py::Float(0.5), &py::fsum(&{ let mut out = vec![]; for t3233 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_a: Py; 
v_a = t3233;
for t3234 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_b: Py; 
v_b = t3234;
out.push(py::mul(&py::mul(&py::getitem(&v_x1.clone(), &v_a.clone())?, &py::getitem(&py::getitem(&v_Qf.clone(), &v_a.clone())?, &v_b.clone())?)?, &py::getitem(&v_x1.clone(), &v_b.clone())?)?); }} Py::list(out) })?)?, &py::fsum(&{ let mut out = vec![]; for t3235 in py::iter(&(py::zip(&[v_c.clone(), v_x1.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3236 = (t3235).items()?;
if t3236.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3236[0].clone();
v_b = t3236[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?)?;
    return Ok(py::minmax(&[v_orth.clone(), v_step.clone(), f__rel(py::getitem(&v_o.clone(), &Py::str("f1"))?, v_f1.clone())?, py::minmax(&[{ let mut out = vec![]; for t3237 in py::iter(&(py::zip(&[py::getitem(&v_o.clone(), &Py::str("gradient"))?, v_g.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3238 = (t3237).items()?;
if t3238.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3238[0].clone();
v_b = t3238[1].clone();
out.push(f__rel(v_a.clone(), v_b.clone())?); } Py::list(out) }], true)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_seq(mut v_i: Py, mut v_o: Py, mut v_key_fn: Py, mut v_value_fn: Py, mut v_value_key: Py, mut v_n_key: Py) -> R {
    let mut v_keys: Py = Py::None;
    let mut v_seq: Py = Py::None;
    v_seq = { let mut out = vec![]; for t3869 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("sequence"))?))? { let mut v_v: Py; 
v_v = t3869;
out.push(py::sub(&py::int(&py::round(&v_v.clone(), None)?)?, &Py::Int(1i64))?); } Py::list(out) };
    if ({ let t3870 = py::sorted(&v_seq.clone(), (Py::Bool(false)).truthy())?; Py::Bool({ let t3871 = Py::list((Py::list((py::range_iter(&[(py::len(&py::getitem(&v_i.clone(), &v_n_key.clone())?)?).i()?])).into_iter().collect::<Vec<Py>>())).items()?); if py::ne(&t3870, &t3871) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_keys = { let mut out = vec![]; for t3872 in py::iter(&(v_seq.clone()))? { let mut v_j: Py; 
v_j = t3872;
out.push(py::call(&v_key_fn.clone(), &[v_i.clone(), v_j.clone()])?); } Py::list(out) };
    if (Py::Bool(({ let mut out = vec![]; for t3873 in py::iter(&(py::zip(&[v_keys.clone(), py::slice(&v_keys.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_k1: Py; let mut v_k2: Py; 
let t3874 = (t3873).items()?;
if t3874.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k1 = t3874[0].clone();
v_k2 = t3874[1].clone();
out.push({ let t3875 = v_k2.clone(); Py::Bool({ let t3876 = v_k1.clone(); if py::lt(&t3875, &t3876)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(f__rel(py::getitem(&v_o.clone(), &v_value_key.clone())?, py::call(&v_value_fn.clone(), &[v_i.clone(), v_seq.clone()])?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_sp(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_adj: Py = Py::None;
    let mut v_dist: Py = Py::None;
    let mut v_path: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_ws: Py = Py::None;
    let t3140: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_E = f__wedges(py::getitem(&v_i.clone(), &Py::str("edges"))?, Py::Bool(true))?;
            v_dist = f__bellman(v_E.clone(), py::method(&Py::str(&py::to_str(&py::getitem(&v_i.clone(), &Py::str("source"))?)), "strip", &[])?, py::getitem(&v_i.clone(), &Py::str("directed"))?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3140 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    v_t = py::method(&Py::str(&py::to_str(&py::getitem(&v_i.clone(), &Py::str("target"))?)), "strip", &[])?;
    v_path = py::method(&v_o.clone(), "get", &[Py::str("path"), py::method(&v_o.clone(), "get", &[Py::str("path_example")])?])?;
    if ({ let t3149 = { let t3141 = v_t.clone(); Py::Bool({ let t3142 = v_dist.clone(); if !py::contains(&t3142, &t3141)? { true } else { false } }) }; if t3149.truthy() { t3149 } else { { let t3148 = py::not(&v_path.clone()); if t3148.truthy() { t3148 } else { { let t3147 = { let t3143 = py::getitem(&v_path.clone(), &Py::Int(0i64))?; Py::Bool({ let t3144 = py::method(&Py::str(&py::to_str(&py::getitem(&v_i.clone(), &Py::str("source"))?)), "strip", &[])?; if py::ne(&t3143, &t3144) { true } else { false } }) }; if t3147.truthy() { t3147 } else { { let t3145 = py::getitem(&v_path.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t3146 = v_t.clone(); if py::ne(&t3145, &t3146) { true } else { false } }) } } } } } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_adj = f__adj(v_E.clone(), py::getitem(&v_i.clone(), &Py::str("directed"))?)?;
    v_tot = py::fraction(&[Py::Int(0i64)])?;
    for t3150 in py::iter(&(py::zip(&[v_path.clone(), py::slice(&v_path.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? {
        let t3155 = (t3150).items()?;
        if t3155.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_u = t3155[0].clone();
        v_v = t3155[1].clone();
        v_ws = { let mut out = vec![]; for t3151 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? { let mut v_w: Py; let mut v_x: Py; 
let t3152 = (t3151).items()?;
if t3152.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t3152[0].clone();
v_w = t3152[1].clone();
if ({ let t3153 = v_x.clone(); Py::Bool({ let t3154 = v_v.clone(); if py::eq(&t3153, &t3154) { true } else { false } }) }).truthy() { out.push(v_w.clone()); }} Py::list(out) };
        if (py::not(&v_ws.clone())).truthy() {
            return Ok(Py::Float(1.0));
        }
        v_tot = py::iadd(&v_tot, &py::minmax(&[v_ws.clone()], false)?)?;
    }
    return Ok(py::minmax(&[f__rel(py::getitem(&v_o.clone(), &Py::str("distance"))?, py::float(&py::getitem(&v_dist.clone(), &v_t.clone())?)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("distance"))?, py::float(&v_tot.clone())?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_staffing(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_crit: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_v2: Py = Py::None;
    let mut v_val: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_c = py::getitem(&v_o.clone(), &Py::str("servers"))?;
    if ({ let t2591 = v_c.clone(); Py::Bool({ let t2592 = py::int(&v_c.clone())?; if py::ne(&t2591, &t2592) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_c = py::int(&v_c.clone())?;
    let t2593: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f_mmc(py::getitem(&v_i.clone(), &Py::str("lam"))?, py::getitem(&v_i.clone(), &Py::str("mu"))?, v_c.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2593 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    v_crit = py::method(&v_i.clone(), "get", &[Py::str("criterion"), Py::str("wait_probability")])?;
    v_val = (if ({ let t2594 = v_crit.clone(); Py::Bool({ let t2595 = Py::str("wait_probability"); if py::eq(&t2594, &t2595) { true } else { false } }) }).truthy() { py::getitem(&v_r.clone(), &Py::str("P_wait"))? } else { py::getitem(&v_r.clone(), &Py::str("Wq"))? });
    v_worst = py::minmax(&[f__rel(py::getitem(&v_o.clone(), &Py::str("P_wait"))?, py::getitem(&v_r.clone(), &Py::str("P_wait"))?)?, py::truediv(&py::minmax(&[Py::Float(0.0), py::sub(&v_val.clone(), &py::getitem(&v_i.clone(), &Py::str("target"))?)?], true)?, &py::getitem(&v_i.clone(), &Py::str("target"))?)?], true)?;
    if ({ let t2596 = py::sub(&v_c.clone(), &Py::Int(1i64))?; Py::Bool({ let t2597 = py::truediv(&py::getitem(&v_i.clone(), &Py::str("lam"))?, &py::getitem(&v_i.clone(), &Py::str("mu"))?)?; if py::gt(&t2596, &t2597)? { true } else { false } }) }).truthy() {
        v_r2 = f_mmc(py::getitem(&v_i.clone(), &Py::str("lam"))?, py::getitem(&v_i.clone(), &Py::str("mu"))?, py::sub(&v_c.clone(), &Py::Int(1i64))?)?;
        v_v2 = (if ({ let t2598 = v_crit.clone(); Py::Bool({ let t2599 = Py::str("wait_probability"); if py::eq(&t2598, &t2599) { true } else { false } }) }).truthy() { py::getitem(&v_r2.clone(), &Py::str("P_wait"))? } else { py::getitem(&v_r2.clone(), &Py::str("Wq"))? });
        v_worst = py::minmax(&[v_worst.clone(), (if ({ let t2600 = v_v2.clone(); Py::Bool({ let t2601 = py::getitem(&v_i.clone(), &Py::str("target"))?; if py::gt(&t2600, &t2601)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) })], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_transport(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_plan: Py = Py::None;
    let mut v_rel: Py = Py::None;
    let mut v_sense: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t3512: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t3511 = (f__transport_lp(py::getitem(&v_i.clone(), &Py::str("supply"))?, py::getitem(&v_i.clone(), &Py::str("demand"))?, py::getitem(&v_i.clone(), &Py::str("cost"))?)?).items()?;
            if t3511.len() != 5 { return Err(PyErr::Value("unpack".into())); }
            v_sense = t3511[0].clone();
            v_c = t3511[1].clone();
            v_A = t3511[2].clone();
            v_b = t3511[3].clone();
            v_rel = t3511[4].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3512 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    v_plan = py::method(&v_o.clone(), "get", &[Py::str("shipments"), py::method(&v_o.clone(), "get", &[Py::str("shipments_example")])?])?;
    v_x = { let mut out = vec![]; for t3513 in py::iter(&(v_plan.clone()))? { let mut v_r: Py; 
v_r = t3513;
for t3514 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t3514;
out.push(v_v.clone()); }} Py::list(out) };
    return Ok(f__cert_from_lp(v_sense.clone(), v_c.clone(), v_A.clone(), v_b.clone(), v_rel.clone(), v_x.clone(), py::getitem(&v_o.clone(), &Py::str("dual"))?, py::getitem(&v_o.clone(), &Py::str("min_cost"))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_tsp(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_D = py::getitem(&v_i.clone(), &Py::str("D"))?;
    v_t = { let mut out = vec![]; for t3583 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("tour_example"))?))? { let mut v_x: Py; 
v_x = t3583;
out.push(py::sub(&py::int(&py::round(&v_x.clone(), None)?)?, &Py::Int(1i64))?); } Py::list(out) };
    v_n = py::len(&v_D.clone())?;
    if ({ let t3591 = { let t3584 = py::getitem(&v_t.clone(), &Py::Int(0i64))?; Py::Bool({ let t3585 = Py::Int(0i64); if py::ne(&t3584, &t3585) { true } else { false } }) }; if t3591.truthy() { t3591 } else { { let t3590 = { let t3586 = py::getitem(&v_t.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t3587 = Py::Int(0i64); if py::ne(&t3586, &t3587) { true } else { false } }) }; if t3590.truthy() { t3590 } else { { let t3588 = py::sorted(&py::slice(&v_t.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?, (Py::Bool(false)).truthy())?; Py::Bool({ let t3589 = Py::list((Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>())).items()?); if py::ne(&t3588, &t3589) { true } else { false } }) } } } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_worst = f__rel(py::getitem(&v_o.clone(), &Py::str("min_tour_length"))?, py::sum(&{ let mut out = vec![]; for t3592 in py::iter(&(py::zip(&[v_t.clone(), py::slice(&v_t.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3593 = (t3592).items()?;
if t3593.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3593[0].clone();
v_b = t3593[1].clone();
out.push(py::getitem(&py::getitem(&v_D.clone(), &v_a.clone())?, &v_b.clone())?); } Py::list(out) }, None)?)?;
    v_key = Py::str(&py::repr(&v_D.clone()));
    if ({ let t3594 = v_key.clone(); Py::Bool({ let t3595 = c__TSP_MEMO()?; if !py::contains(&t3595, &t3594)? { true } else { false } }) }).truthy() {
        let _ = (if ({ let t3596 = py::len(&c__TSP_MEMO()?)?; Py::Bool({ let t3597 = Py::Int(64i64); if py::gt(&t3596, &t3597)? { true } else { false } }) }).truthy() { py::method(&c__TSP_MEMO()?, "clear", &[])? } else { Py::None });
        py::setitem(&c__TSP_MEMO()?, &v_key.clone(), py::getitem(&f_tsp(v_D.clone())?, &Py::str("min_tour_length"))?)?;
    }
    return Ok(py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_o.clone(), &Py::str("min_tour_length"))?, py::getitem(&c__TSP_MEMO()?, &v_key.clone())?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_vc(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_cov: Py = Py::None;
    let mut v_nodes: Py = Py::None;
    let t2434 = (f__simple_graph(py::getitem(&v_i.clone(), &Py::str("edges"))?, Py::Int(30i64))?).items()?;
    if t2434.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_nodes = t2434[0].clone();
    v_adj = t2434[1].clone();
    v_cov = py::set(&py::getitem(&v_o.clone(), &Py::str("cover_example"))?)?;
    if ({ let t2447 = py::not(&{ let t2435 = v_cov.clone(); Py::Bool({ let t2436 = py::set(&v_nodes.clone())?; if py::set_cmp(&t2435, &t2436, "<=")? { true } else { false } }) }); if t2447.truthy() { t2447 } else { { let t2446 = Py::Bool(({ let mut out = vec![]; for t2437 in py::iter(&(v_nodes.clone()))? { let mut v_u: Py; 
v_u = t2437;
for t2438 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? { let mut v_v: Py; 
v_v = t2438;
out.push({ let t2443 = { let t2439 = v_u.clone(); Py::Bool({ let t2440 = v_cov.clone(); if !py::contains(&t2440, &t2439)? { true } else { false } }) }; if !t2443.truthy() { t2443 } else { { let t2441 = v_v.clone(); Py::Bool({ let t2442 = v_cov.clone(); if !py::contains(&t2442, &t2441)? { true } else { false } }) } } }); }} Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t2446.truthy() { t2446 } else { { let t2444 = py::len(&v_cov.clone())?; Py::Bool({ let t2445 = py::getitem(&v_o.clone(), &Py::str("min_cover_size"))?; if py::ne(&t2444, &t2445) { true } else { false } }) } } } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::minmax(&[f__rel(py::getitem(&v_o.clone(), &Py::str("min_cover_size"))?, py::getitem(&f_vertex_cover(py::getitem(&v_i.clone(), &Py::str("edges"))?)?, &Py::str("min_cover_size"))?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("max_independent_set_size"))?, py::sub(&py::len(&v_nodes.clone())?, &py::len(&v_cov.clone())?)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_ww(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_cost: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_dt: Py = Py::None;
    let mut v_inv: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_qt: Py = Py::None;
    let t3708 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("order_quantities_example"))?, { let mut out = vec![]; for t3707 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("demand"))?))? { let mut v_v: Py; 
v_v = t3707;
out.push(py::float(&v_v.clone())?); } Py::list(out) }])).items()?;
    if t3708.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_q = t3708[0].clone();
    v_d = t3708[1].clone();
    if ({ let t3713 = { let t3709 = py::len(&v_q.clone())?; Py::Bool({ let t3710 = py::len(&v_d.clone())?; if py::ne(&t3709, &t3710) { true } else { false } }) }; if t3713.truthy() { t3713 } else { { let t3711 = py::minmax(&[v_q.clone()], false)?; Py::Bool({ let t3712 = Py::Int(0i64); if py::lt(&t3711, &t3712)? { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    let t3714 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0)])).items()?;
    if t3714.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_inv = t3714[0].clone();
    v_cost = t3714[1].clone();
    for t3715 in py::iter(&(py::zip(&[v_q.clone(), v_d.clone()])?))? {
        let t3720 = (t3715).items()?;
        if t3720.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_qt = t3720[0].clone();
        v_dt = t3720[1].clone();
        v_inv = py::iadd(&v_inv, &py::sub(&v_qt.clone(), &v_dt.clone())?)?;
        if ({ let t3716 = v_inv.clone(); Py::Bool({ let t3717 = py::mul(&py::neg(&Py::Float(1e-09))?, &py::minmax(&[Py::Float(1.0), v_dt.clone()], true)?)?; if py::lt(&t3716, &t3717)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(1.0));
        }
        v_cost = py::iadd(&v_cost, &py::add(&(if ({ let t3718 = v_qt.clone(); Py::Bool({ let t3719 = Py::Int(0i64); if py::gt(&t3718, &t3719)? { true } else { false } }) }).truthy() { py::float(&py::getitem(&v_i.clone(), &Py::str("K"))?)? } else { Py::Float(0.0) }), &py::mul(&py::float(&py::getitem(&v_i.clone(), &Py::str("h"))?)?, &v_inv.clone())?)?)?;
    }
    return Ok(py::minmax(&[f__rel(py::getitem(&v_o.clone(), &Py::str("min_cost"))?, v_cost.clone())?, f__rel(py::getitem(&v_o.clone(), &Py::str("min_cost"))?, py::getitem(&f_wagner_whitin(py::getitem(&v_i.clone(), &Py::str("demand"))?, py::getitem(&v_i.clone(), &Py::str("K"))?, py::getitem(&v_i.clone(), &Py::str("h"))?)?, &Py::str("min_cost"))?)?, (if ({ let t3721 = py::getitem(&v_o.clone(), &Py::str("orders"))?; Py::Bool({ let t3722 = py::sum(&{ let mut out = vec![]; for t3723 in py::iter(&(v_q.clone()))? { let mut v_v: Py; 
v_v = t3723;
if ({ let t3724 = v_v.clone(); Py::Bool({ let t3725 = Py::Int(0i64); if py::gt(&t3724, &t3725)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?; if py::eq(&t3721, &t3722) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) })], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__wedges(mut v_edges: Py, mut v_need_weight: Py) -> R {
    if (py::not(&v_edges.clone())).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give at least one edge"))));
    }
    if ({ let t3065 = v_need_weight.clone(); if !t3065.truthy() { t3065 } else { Py::Bool(({ let mut out = vec![]; for t3062 in py::iter(&(v_edges.clone()))? { let mut v_e: Py; 
v_e = t3062;
out.push({ let t3063 = py::len(&v_e.clone())?; Py::Bool({ let t3064 = Py::Int(3i64); if py::ne(&t3063, &t3064) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("every edge needs a weight: 'A-B 4'"))));
    }
    return Ok((if (v_need_weight.clone()).truthy() { { let mut out = vec![]; for t3066 in py::iter(&(v_edges.clone()))? { let mut v_e: Py; 
v_e = t3066;
out.push(Py::tuple(vec![Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(0i64))?)), Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(1i64))?)), f__fr(py::getitem(&v_e.clone(), &Py::Int(2i64))?)?])); } Py::list(out) } } else { { let mut out = vec![]; for t3067 in py::iter(&(v_edges.clone()))? { let mut v_e: Py; 
v_e = t3067;
out.push(Py::tuple(vec![Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(0i64))?)), Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(1i64))?))])); } Py::list(out) } }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ahp_consistency(mut v_judgments: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_ci: Py = Py::None;
    let mut v_cr: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_P = f__ahp_matrix(v_judgments.clone())?;
    v_n = py::len(&v_P.clone())?;
    if ({ let t14 = v_n.clone(); Py::Bool({ let t15 = Py::Int(3i64); if py::lt(&t14, &t15)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the consistency ratio needs at least 3 criteria"))));
    }
    let t28 = (f__perron(v_P.clone())?).items()?;
    if t28.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_w = t28[0].clone();
    v_lam = t28[1].clone();
    v_ci = py::truediv(&py::sub(&v_lam.clone(), &v_n.clone())?, &py::sub(&v_n.clone(), &Py::Int(1i64))?)?;
    v_cr = py::truediv(&v_ci.clone(), &py::getitem(&c__RI()?, &v_n.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("CR"), v_cr.clone()), (Py::str("CI"), v_ci.clone()), (Py::str("lambda_max"), v_lam.clone()), (Py::str("weights"), v_w.clone()), (Py::str("acceptable"), (if ({ let t29 = v_cr.clone(); Py::Bool({ let t30 = Py::Float(0.1); if py::lt(&t29, &t30)? { true } else { false } }) }).truthy() { Py::str("yes") } else { Py::str("no") }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ahp_geometric_mean(mut v_judgments: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_P = f__ahp_matrix(v_judgments.clone())?;
    v_n = py::len(&v_P.clone())?;
    v_g = { let mut out = vec![]; for t61 in py::iter(&(v_P.clone()))? { let mut v_r: Py; 
v_r = t61;
out.push(py::math1("exp", &py::truediv(&py::fsum(&{ let mut out = vec![]; for t62 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t62;
out.push(py::math1("log", &v_v.clone())?); } Py::list(out) })?, &v_n.clone())?)?); } Py::list(out) };
    v_s = py::fsum(&v_g.clone())?;
    v_w = { let mut out = vec![]; for t63 in py::iter(&(v_g.clone()))? { let mut v_v: Py; 
v_v = t63;
out.push(py::truediv(&v_v.clone(), &v_s.clone())?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("weights"), v_w.clone()), (Py::str("max_weight"), py::minmax(&[v_w.clone()], true)?), (Py::str("best"), py::float(&f__best(v_w.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ahp_priority(mut v_judgments: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_P = f__ahp_matrix(v_judgments.clone())?;
    let t106 = (f__perron(v_P.clone())?).items()?;
    if t106.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_w = t106[0].clone();
    v_lam = t106[1].clone();
    return Ok(Py::dict(vec![(Py::str("weights"), v_w.clone()), (Py::str("lambda_max"), v_lam.clone()), (Py::str("best"), py::float(&f__best(v_w.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_all_pairs(mut v_edges: Py, mut v_directed: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_E: Py = Py::None;
    let mut v_INF: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_ix: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_nodes: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_E = f__wedges(v_edges.clone(), Py::Bool(true))?;
    v_nodes = f__nodes(v_E.clone())?;
    if ({ let t133 = py::len(&v_nodes.clone())?; Py::Bool({ let t134 = Py::Int(40i64); if py::gt(&t133, &t134)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at most 40 nodes"))));
    }
    v_n = py::len(&v_nodes.clone())?;
    v_ix = { let mut out = vec![]; for t135 in py::iter(&(py::enumerate(&v_nodes.clone(), 0)?))? { let mut v_k: Py; let mut v_v: Py; 
let t136 = (t135).items()?;
if t136.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t136[0].clone();
v_v = t136[1].clone();
out.push((v_v.clone(), v_k.clone())); } Py::dict(out) };
    v_INF = Py::None;
    v_D = { let mut out = vec![]; for t137 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_a: Py; 
v_a = t137;
out.push({ let mut out = vec![]; for t138 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_b: Py; 
v_b = t138;
out.push((if ({ let t139 = v_a.clone(); Py::Bool({ let t140 = v_b.clone(); if py::eq(&t139, &t140) { true } else { false } }) }).truthy() { py::fraction(&[Py::Int(0i64)])? } else { v_INF.clone() })); } Py::list(out) }); } Py::list(out) };
    for t141 in py::iter(&(v_E.clone()))? {
        let t149 = (t141).items()?;
        if t149.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_u = t149[0].clone();
        v_v = t149[1].clone();
        v_w = t149[2].clone();
        for t142 in py::iter(&(py::add(&Py::tuple(vec![Py::tuple(vec![v_u.clone(), v_v.clone()])]), &(if (v_directed.clone()).truthy() { Py::tuple(vec![]) } else { Py::tuple(vec![Py::tuple(vec![v_v.clone(), v_u.clone()])]) }))?))? {
            let t148 = (t142).items()?;
            if t148.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t148[0].clone();
            v_b = t148[1].clone();
            if ({ let t147 = { let t143 = py::getitem(&py::getitem(&v_D.clone(), &py::getitem(&v_ix.clone(), &v_a.clone())?)?, &py::getitem(&v_ix.clone(), &v_b.clone())?)?; Py::Bool({ let t144 = Py::None; if py::is(&t143, &t144) { true } else { false } }) }; if t147.truthy() { t147 } else { { let t145 = v_w.clone(); Py::Bool({ let t146 = py::getitem(&py::getitem(&v_D.clone(), &py::getitem(&v_ix.clone(), &v_a.clone())?)?, &py::getitem(&v_ix.clone(), &v_b.clone())?)?; if py::lt(&t145, &t146)? { true } else { false } }) } } }).truthy() {
                py::setitem(&py::getitem(&v_D.clone(), &py::getitem(&v_ix.clone(), &v_a.clone())?)?, &py::getitem(&v_ix.clone(), &v_b.clone())?, v_w.clone())?;
            }
        }
    }
    for t150 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t150;
        for t151 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_a = t151;
            if ({ let t152 = py::getitem(&py::getitem(&v_D.clone(), &v_a.clone())?, &v_k.clone())?; Py::Bool({ let t153 = Py::None; if py::is(&t152, &t153) { true } else { false } }) }).truthy() {
                continue;
            }
            for t154 in py::range_iter(&[(v_n.clone()).i()?]) {
                v_b = t154;
                if ({ let t162 = { let t155 = py::getitem(&py::getitem(&v_D.clone(), &v_k.clone())?, &v_b.clone())?; Py::Bool({ let t156 = Py::None; if !py::is(&t155, &t156) { true } else { false } }) }; if !t162.truthy() { t162 } else { { let t161 = { let t157 = py::getitem(&py::getitem(&v_D.clone(), &v_a.clone())?, &v_b.clone())?; Py::Bool({ let t158 = Py::None; if py::is(&t157, &t158) { true } else { false } }) }; if t161.truthy() { t161 } else { { let t159 = py::add(&py::getitem(&py::getitem(&v_D.clone(), &v_a.clone())?, &v_k.clone())?, &py::getitem(&py::getitem(&v_D.clone(), &v_k.clone())?, &v_b.clone())?)?; Py::Bool({ let t160 = py::getitem(&py::getitem(&v_D.clone(), &v_a.clone())?, &v_b.clone())?; if py::lt(&t159, &t160)? { true } else { false } }) } } } } }).truthy() {
                    py::setitem(&py::getitem(&v_D.clone(), &v_a.clone())?, &v_b.clone(), py::add(&py::getitem(&py::getitem(&v_D.clone(), &v_a.clone())?, &v_k.clone())?, &py::getitem(&py::getitem(&v_D.clone(), &v_k.clone())?, &v_b.clone())?)?)?;
                }
            }
        }
    }
    if (Py::Bool(({ let mut out = vec![]; for t163 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_a: Py; 
v_a = t163;
out.push({ let t164 = py::getitem(&py::getitem(&v_D.clone(), &v_a.clone())?, &v_a.clone())?; Py::Bool({ let t165 = Py::Int(0i64); if py::lt(&t164, &t165)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the graph has a negative cycle"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t166 in py::iter(&(v_D.clone()))? { let mut v_r: Py; 
v_r = t166;
for t167 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t167;
out.push({ let t168 = v_x.clone(); Py::Bool({ let t169 = Py::None; if py::is(&t168, &t169) { true } else { false } }) }); }} Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("some node cannot reach another (the distance would be infinite); give a connected graph"))));
    }
    return Ok(Py::dict(vec![(Py::str("nodes"), v_nodes.clone()), (Py::str("distances"), { let mut out = vec![]; for t170 in py::iter(&(v_D.clone()))? { let mut v_r: Py; 
v_r = t170;
out.push({ let mut out = vec![]; for t171 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t171;
out.push(py::float(&v_x.clone())?); } Py::list(out) }); } Py::list(out) }), (Py::str("diameter"), py::float(&py::minmax(&[{ let mut out = vec![]; for t172 in py::iter(&(v_D.clone()))? { let mut v_r: Py; 
v_r = t172;
out.push(py::minmax(&[v_r.clone()], true)?); } Py::list(out) }], true)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bellman_ford(mut v_edges: Py, mut v_source: Py, mut v_target: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_dist: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_E = f__wedges(v_edges.clone(), Py::Bool(true))?;
    let t334 = (Py::tuple(vec![py::method(&Py::str(&py::to_str(&v_source.clone())), "strip", &[])?, py::method(&Py::str(&py::to_str(&v_target.clone())), "strip", &[])?])).items()?;
    if t334.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t334[0].clone();
    v_t = t334[1].clone();
    if ({ let t343 = { let t335 = v_s.clone(); Py::Bool({ let t336 = f__nodes(v_E.clone())?; if !py::contains(&t336, &t335)? { true } else { false } }) }; if t343.truthy() { t343 } else { { let t341 = v_t.clone(); Py::Bool({ let t342 = f__nodes(v_E.clone())?; if !py::contains(&t342, &t341)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the source and the target must be nodes of the graph"))));
    }
    v_dist = f__bellman(v_E.clone(), v_s.clone(), Py::Bool(true))?;
    if ({ let t359 = v_t.clone(); Py::Bool({ let t360 = v_dist.clone(); if !py::contains(&t360, &t359)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["there is no path from ".to_string(), py::format(&v_s.clone(), "")?, " to ".to_string(), py::format(&v_t.clone(), "")?].concat()))));
    }
    return Ok(Py::dict(vec![(Py::str("distance"), py::float(&py::getitem(&v_dist.clone(), &v_t.clone())?)?), (Py::str("distances_from_source"), { let mut out = vec![]; for t361 in py::iter(&(py::sorted_key(&py::method(&v_dist.clone(), "items", &[])?, &{ let v_E = v_E.clone(); let v_dist = v_dist.clone(); let v_edges = v_edges.clone(); let v_s = v_s.clone(); let v_source = v_source.clone(); let v_t = v_t.clone(); let v_target = v_target.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_kv: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::method(&f__nodes(v_E.clone())?, "index", &[py::getitem(&v_kv.clone(), &Py::Int(0i64))?])?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?))? { let mut v_d: Py; let mut v_v: Py; 
let t362 = (t361).items()?;
if t362.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t362[0].clone();
v_d = t362[1].clone();
out.push((v_v.clone(), py::float(&v_d.clone())?)); } Py::dict(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bin_packing(mut v_sizes: Py, mut v_capacity: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_cap: Py = Py::None;
    let mut v_dfs: Py = Py::None;
    let mut v_lb: Py = Py::None;
    let mut v_loads: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_order: Py = Py::None;
    let _ = f__finite(Py::tuple(vec![v_capacity.clone()]))?;
    v_n = py::len(&v_sizes.clone())?;
    if ({ let t401 = { let t397 = v_n.clone(); Py::Bool({ let t398 = Py::Int(0i64); if py::eq(&t397, &t398) { true } else { false } }) }; if t401.truthy() { t401 } else { { let t399 = v_n.clone(); Py::Bool({ let t400 = Py::Int(14i64); if py::gt(&t399, &t400)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give between 1 and 14 items"))));
    }
    if ({ let t410 = { let t402 = v_capacity.clone(); Py::Bool({ let t403 = Py::Int(0i64); if py::le(&t402, &t403)? { true } else { false } }) }; if t410.truthy() { t410 } else { Py::Bool(({ let mut out = vec![]; for t404 in py::iter(&(v_sizes.clone()))? { let mut v_s: Py; 
v_s = t404;
out.push({ let t409 = { let t405 = v_s.clone(); Py::Bool({ let t406 = Py::Int(0i64); if py::le(&t405, &t406)? { true } else { false } }) }; if t409.truthy() { t409 } else { { let t407 = v_s.clone(); Py::Bool({ let t408 = v_capacity.clone(); if py::gt(&t407, &t408)? { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("every item size must be > 0 and fit in a bin"))));
    }
    let t412 = (Py::tuple(vec![{ let mut out = vec![]; for t411 in py::iter(&(v_sizes.clone()))? { let mut v_s: Py; 
v_s = t411;
out.push(f__fr(v_s.clone())?); } Py::list(out) }, f__fr(v_capacity.clone())?])).items()?;
    if t412.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_S = t412[0].clone();
    v_cap = t412[1].clone();
    v_order = py::sorted_key(&Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>()), &{ let v_S = v_S.clone(); let v_cap = v_cap.clone(); let v_capacity = v_capacity.clone(); let v_n = v_n.clone(); let v_sizes = v_sizes.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_j: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::neg(&py::getitem(&v_S.clone(), &v_j.clone())?)?, v_j.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    v_best = Py::list(vec![py::add(&v_n.clone(), &Py::Int(1i64))?, Py::None]);
    v_lb = py::math1("ceil", &py::truediv(&py::sum(&v_S.clone(), None)?, &v_cap.clone())?)?;
    let t413 = (Py::tuple(vec![Py::list(vec![]), py::mul(&Py::list(vec![Py::Int(0i64)]), &v_n.clone())?])).items()?;
    if t413.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_loads = t413[0].clone();
    v_a = t413[1].clone();
    let rec414 = py::rec_cell();
    v_dfs = { let v_S = v_S.clone(); let v_a = v_a.clone(); let v_best = v_best.clone(); let v_cap = v_cap.clone(); let v_capacity = v_capacity.clone(); let v_lb = v_lb.clone(); let v_loads = v_loads.clone(); let v_n = v_n.clone(); let v_order = v_order.clone(); let v_sizes = v_sizes.clone(); let rec414 = rec414.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            let v_dfs: Py = py::rec_get(&rec414)?;
            let mut v_j: Py = Py::None;
            let mut v_k: Py = Py::None;
            let mut v_seen: Py = Py::None;
            if ({ let t419 = { let t415 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; Py::Bool({ let t416 = v_lb.clone(); if py::eq(&t415, &t416) { true } else { false } }) }; if t419.truthy() { t419 } else { { let t417 = py::len(&v_loads.clone())?; Py::Bool({ let t418 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; if py::ge(&t417, &t418)? { true } else { false } }) } } }).truthy() {
                return Ok(Py::None);
            }
            if ({ let t420 = v_t.clone(); Py::Bool({ let t421 = v_n.clone(); if py::eq(&t420, &t421) { true } else { false } }) }).truthy() {
                let t422 = (Py::tuple(vec![py::len(&v_loads.clone())?, Py::list((v_a.clone()).items()?)])).items()?;
                if t422.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                py::setitem(&v_best.clone(), &Py::Int(0i64), t422[0].clone())?;
                py::setitem(&v_best.clone(), &Py::Int(1i64), t422[1].clone())?;
                return Ok(Py::None);
            }
            v_j = py::getitem(&v_order.clone(), &v_t.clone())?;
            v_seen = py::set(&Py::list(vec![]))?;
            for t423 in py::range_iter(&[(py::len(&v_loads.clone())?).i()?]) {
                v_k = t423;
                if ({ let t428 = { let t424 = py::add(&py::getitem(&v_loads.clone(), &v_k.clone())?, &py::getitem(&v_S.clone(), &v_j.clone())?)?; Py::Bool({ let t425 = v_cap.clone(); if py::le(&t424, &t425)? { true } else { false } }) }; if !t428.truthy() { t428 } else { { let t426 = py::getitem(&v_loads.clone(), &v_k.clone())?; Py::Bool({ let t427 = v_seen.clone(); if !py::contains(&t427, &t426)? { true } else { false } }) } } }).truthy() {
                    let _ = py::method(&v_seen.clone(), "add", &[py::getitem(&v_loads.clone(), &v_k.clone())?])?;
                    let t429 = v_loads.clone(); let t430 = v_k.clone();
                    py::setitem(&t429, &t430, py::iadd(&py::getitem(&t429, &t430)?, &py::getitem(&v_S.clone(), &v_j.clone())?)?)?;
                    py::setitem(&v_a.clone(), &v_j.clone(), v_k.clone())?;
                    let _ = py::call(&v_dfs.clone(), &[py::add(&v_t.clone(), &Py::Int(1i64))?])?;
                    let t431 = v_loads.clone(); let t432 = v_k.clone();
                    py::setitem(&t431, &t432, py::sub(&py::getitem(&t431, &t432)?, &py::getitem(&v_S.clone(), &v_j.clone())?)?)?;
                }
            }
            let _ = py::method(&v_loads.clone(), "append", &[py::getitem(&v_S.clone(), &v_j.clone())?])?;
            py::setitem(&v_a.clone(), &v_j.clone(), py::sub(&py::len(&v_loads.clone())?, &Py::Int(1i64))?)?;
            let _ = py::call(&v_dfs.clone(), &[py::add(&v_t.clone(), &Py::Int(1i64))?])?;
            let _ = py::method(&v_loads.clone(), "pop", &[])?;
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec414, &v_dfs);
    let _ = py::call(&v_dfs.clone(), &[Py::Int(0i64)])?;
    return Ok(Py::dict(vec![(Py::str("min_bins"), py::float(&py::getitem(&v_best.clone(), &Py::Int(0i64))?)?), (Py::str("bin_of_item"), { let mut out = vec![]; for t433 in py::iter(&(py::getitem(&v_best.clone(), &Py::Int(1i64))?))? { let mut v_k: Py; 
v_k = t433;
out.push(py::add(&v_k.clone(), &Py::Int(1i64))?); } Py::list(out) }), (Py::str("lower_bound"), py::float(&v_lb.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bipartite_matching(mut v_edges: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_Z: Py = Py::None;
    let mut v_adj: Py = Py::None;
    let mut v_aug: Py = Py::None;
    let mut v_cover: Py = Py::None;
    let mut v_left: Py = Py::None;
    let mut v_match_l: Py = Py::None;
    let mut v_match_r: Py = Py::None;
    let mut v_right: Py = Py::None;
    let mut v_stack: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_E = f__wedges(v_edges.clone(), Py::Bool(false))?;
    v_left = f__nodes({ let mut out = vec![]; for t450 in py::iter(&(v_E.clone()))? { let mut v__: Py; let mut v_u: Py; 
let t451 = (t450).items()?;
if t451.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t451[0].clone();
v__ = t451[1].clone();
out.push(Py::tuple(vec![v_u.clone(), v_u.clone()])); } Py::list(out) })?;
    v_right = f__nodes({ let mut out = vec![]; for t452 in py::iter(&(v_E.clone()))? { let mut v__: Py; let mut v_v: Py; 
let t453 = (t452).items()?;
if t453.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v__ = t453[0].clone();
v_v = t453[1].clone();
out.push(Py::tuple(vec![v_v.clone(), v_v.clone()])); } Py::list(out) })?;
    if (py::bitand(&py::set(&v_left.clone())?, &py::set(&v_right.clone())?)?).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a node appears on both sides: give edges as left-right pairs of a bipartite graph"))));
    }
    v_adj = { let mut out = vec![]; for t454 in py::iter(&(v_left.clone()))? { let mut v_u: Py; 
v_u = t454;
out.push((v_u.clone(), Py::list(vec![]))); } Py::dict(out) };
    for t455 in py::iter(&(v_E.clone()))? {
        let t458 = (t455).items()?;
        if t458.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_u = t458[0].clone();
        v_v = t458[1].clone();
        if ({ let t456 = v_v.clone(); Py::Bool({ let t457 = py::getitem(&v_adj.clone(), &v_u.clone())?; if !py::contains(&t457, &t456)? { true } else { false } }) }).truthy() {
            let _ = py::method(&py::getitem(&v_adj.clone(), &v_u.clone())?, "append", &[v_v.clone()])?;
        }
    }
    v_match_r = Py::dict(vec![]);
    let rec459 = py::rec_cell();
    v_aug = { let v_E = v_E.clone(); let v_adj = v_adj.clone(); let v_edges = v_edges.clone(); let v_left = v_left.clone(); let v_match_r = v_match_r.clone(); let v_right = v_right.clone(); let v_v = v_v.clone(); let rec459 = rec459.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_u: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_seen: Py = a.get(1).cloned().unwrap_or(Py::None);
            let v_aug: Py = py::rec_get(&rec459)?;
            let mut v_v: Py = Py::None;
            for t460 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? {
                v_v = t460;
                if ({ let t461 = v_v.clone(); Py::Bool({ let t462 = v_seen.clone(); if !py::contains(&t462, &t461)? { true } else { false } }) }).truthy() {
                    let _ = py::method(&v_seen.clone(), "add", &[v_v.clone()])?;
                    if ({ let t465 = { let t463 = v_v.clone(); Py::Bool({ let t464 = v_match_r.clone(); if !py::contains(&t464, &t463)? { true } else { false } }) }; if t465.truthy() { t465 } else { py::call(&v_aug.clone(), &[py::getitem(&v_match_r.clone(), &v_v.clone())?, v_seen.clone()])? } }).truthy() {
                        py::setitem(&v_match_r.clone(), &v_v.clone(), v_u.clone())?;
                        return Ok(Py::Bool(true));
                    }
                }
            }
            return Ok(Py::Bool(false));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec459, &v_aug);
    for t466 in py::iter(&(v_left.clone()))? {
        v_u = t466;
        let _ = py::call(&v_aug.clone(), &[v_u.clone(), py::set(&Py::list(vec![]))?])?;
    }
    v_match_l = { let mut out = vec![]; for t467 in py::iter(&(py::method(&v_match_r.clone(), "items", &[])?))? { let mut v_u: Py; let mut v_v: Py; 
let t468 = (t467).items()?;
if t468.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t468[0].clone();
v_u = t468[1].clone();
out.push((v_u.clone(), v_v.clone())); } Py::dict(out) };
    let t472 = (Py::tuple(vec![py::set(&Py::list(vec![]))?, { let mut out = vec![]; for t469 in py::iter(&(v_left.clone()))? { let mut v_u: Py; 
v_u = t469;
if ({ let t470 = v_u.clone(); Py::Bool({ let t471 = v_match_l.clone(); if !py::contains(&t471, &t470)? { true } else { false } }) }).truthy() { out.push(v_u.clone()); }} Py::list(out) }])).items()?;
    if t472.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Z = t472[0].clone();
    v_stack = t472[1].clone();
    let _ = py::method(&v_Z.clone(), "update", &[v_stack.clone()])?;
    while (v_stack.clone()).truthy() {
        v_u = py::method(&v_stack.clone(), "pop", &[])?;
        for t473 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? {
            v_v = t473;
            if ({ let t474 = v_v.clone(); Py::Bool({ let t475 = v_Z.clone(); if !py::contains(&t475, &t474)? { true } else { false } }) }).truthy() {
                let _ = py::method(&v_Z.clone(), "add", &[v_v.clone()])?;
                v_w = py::method(&v_match_r.clone(), "get", &[v_v.clone()])?;
                if ({ let t480 = { let t476 = v_w.clone(); Py::Bool({ let t477 = Py::None; if !py::is(&t476, &t477) { true } else { false } }) }; if !t480.truthy() { t480 } else { { let t478 = v_w.clone(); Py::Bool({ let t479 = v_Z.clone(); if !py::contains(&t479, &t478)? { true } else { false } }) } } }).truthy() {
                    let _ = py::method(&v_Z.clone(), "add", &[v_w.clone()])?;
                    let _ = py::method(&v_stack.clone(), "append", &[v_w.clone()])?;
                }
            }
        }
    }
    v_cover = py::add(&{ let mut out = vec![]; for t481 in py::iter(&(v_left.clone()))? { let mut v_u: Py; 
v_u = t481;
if ({ let t482 = v_u.clone(); Py::Bool({ let t483 = v_Z.clone(); if !py::contains(&t483, &t482)? { true } else { false } }) }).truthy() { out.push(v_u.clone()); }} Py::list(out) }, &{ let mut out = vec![]; for t484 in py::iter(&(v_right.clone()))? { let mut v_v: Py; 
v_v = t484;
if ({ let t485 = v_v.clone(); Py::Bool({ let t486 = v_Z.clone(); if py::contains(&t486, &t485)? { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) })?;
    return Ok(Py::dict(vec![(Py::str("max_matching"), py::float(&py::len(&v_match_l.clone())?)?), (Py::str("matching_example"), { let mut out = vec![]; for t487 in py::iter(&(v_left.clone()))? { let mut v_u: Py; 
v_u = t487;
if ({ let t488 = v_u.clone(); Py::Bool({ let t489 = v_match_l.clone(); if py::contains(&t489, &t488)? { true } else { false } }) }).truthy() { out.push(Py::str(&[py::format(&v_u.clone(), "")?, "-".to_string(), py::format(&py::getitem(&v_match_l.clone(), &v_u.clone())?, "")?].concat())); }} Py::list(out) }), (Py::str("vertex_cover"), v_cover.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_break_even(mut v_F_: Py, mut v_p: Py, mut v_v: Py) -> R {
    let mut v_q: Py = Py::None;
    let _ = f__finite(Py::tuple(vec![v_F_.clone(), v_p.clone(), v_v.clone()]))?;
    if ({ let t512 = v_F_.clone(); Py::Bool({ let t513 = Py::Int(0i64); if py::lt(&t512, &t513)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("fixed cost must be >= 0"))));
    }
    if ({ let t514 = v_p.clone(); Py::Bool({ let t515 = v_v.clone(); if py::le(&t514, &t515)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the price must exceed the variable cost per unit, otherwise no break-even point exists"))));
    }
    v_q = py::truediv(&f__fr(v_F_.clone())?, &py::sub(&f__fr(v_p.clone())?, &f__fr(v_v.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("quantity"), py::float(&v_q.clone())?), (Py::str("revenue"), py::float(&py::mul(&v_q.clone(), &f__fr(v_p.clone())?)?)?), (Py::str("contribution_margin"), py::float(&py::sub(&f__fr(v_p.clone())?, &f__fr(v_v.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_can_min_surface(mut v_V: Py) -> R {
    let mut v_h: Py = Py::None;
    let mut v_r: Py = Py::None;
    let _ = f__finite(Py::tuple(vec![v_V.clone()]))?;
    if ({ let t520 = v_V.clone(); Py::Bool({ let t521 = Py::Int(0i64); if py::le(&t520, &t521)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the volume must be > 0"))));
    }
    v_r = py::pow(&py::truediv(&v_V.clone(), &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?, &py::truediv(&Py::Int(1i64), &Py::Int(3i64))?)?;
    v_r = py::sub(&v_r.clone(), &py::truediv(&py::sub(&py::mul(&py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?, &py::pow(&v_r.clone(), &Py::Int(3i64))?)?, &py::mul(&Py::Int(2i64), &v_V.clone())?)?, &py::mul(&py::mul(&Py::Int(12i64), &Py::Float(std::f64::consts::PI))?, &py::pow(&v_r.clone(), &Py::Int(2i64))?)?)?)?;
    v_h = py::truediv(&v_V.clone(), &py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_r.clone())?, &v_r.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("radius"), v_r.clone()), (Py::str("height"), v_h.clone()), (Py::str("min_area"), py::add(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_r.clone())?, &v_r.clone())?, &py::truediv(&py::mul(&Py::Int(2i64), &v_V.clone())?, &v_r.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_chromatic(mut v_edges: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_bt: Py = Py::None;
    let mut v_col: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_nodes: Py = Py::None;
    let mut v_order: Py = Py::None;
    let t885 = (f__simple_graph(v_edges.clone(), Py::Int(14i64))?).items()?;
    if t885.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_nodes = t885[0].clone();
    v_adj = t885[1].clone();
    v_order = py::sorted_key(&v_nodes.clone(), &{ let v_adj = v_adj.clone(); let v_edges = v_edges.clone(); let v_nodes = v_nodes.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::neg(&py::len(&py::getitem(&v_adj.clone(), &v_v.clone())?)?)?, py::method(&v_nodes.clone(), "index", &[v_v.clone()])?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    for t886 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&py::len(&v_nodes.clone())?, &Py::Int(1i64))?).i()?]) {
        v_k = t886;
        v_col = Py::dict(vec![]);
        let rec887 = py::rec_cell();
        v_bt = { let v_adj = v_adj.clone(); let v_col = v_col.clone(); let v_edges = v_edges.clone(); let v_k = v_k.clone(); let v_nodes = v_nodes.clone(); let v_order = v_order.clone(); let rec887 = rec887.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            let v_bt: Py = py::rec_get(&rec887)?;
            let mut v_cc: Py = Py::None;
            let mut v_used: Py = Py::None;
            let mut v_v: Py = Py::None;
            if ({ let t888 = v_t.clone(); Py::Bool({ let t889 = py::len(&v_order.clone())?; if py::eq(&t888, &t889) { true } else { false } }) }).truthy() {
                return Ok(Py::Bool(true));
            }
            v_v = py::getitem(&v_order.clone(), &v_t.clone())?;
            v_used = py::set(&{ let mut out = vec![]; for t890 in py::iter(&(py::getitem(&v_adj.clone(), &v_v.clone())?))? { let mut v_u: Py; 
v_u = t890;
if ({ let t891 = v_u.clone(); Py::Bool({ let t892 = v_col.clone(); if py::contains(&t892, &t891)? { true } else { false } }) }).truthy() { out.push(py::getitem(&v_col.clone(), &v_u.clone())?); }} Py::list(out) })?;
            for t893 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&py::minmax(&[v_k.clone(), py::add(&{ let t = py::method(&v_col.clone(), "values", &[])?; if t.items()?.is_empty() { Py::Int(0i64) } else { py::minmax(&[t], true)? } }, &Py::Int(1i64))?], false)?, &Py::Int(1i64))?).i()?]) {
                v_cc = t893;
                if ({ let t894 = v_cc.clone(); Py::Bool({ let t895 = v_used.clone(); if !py::contains(&t895, &t894)? { true } else { false } }) }).truthy() {
                    py::setitem(&v_col.clone(), &v_v.clone(), v_cc.clone())?;
                    if (py::call(&v_bt.clone(), &[py::add(&v_t.clone(), &Py::Int(1i64))?])?).truthy() {
                        return Ok(Py::Bool(true));
                    }
                    py::delitem(&v_col.clone(), &v_v.clone())?;
                }
            }
            return Ok(Py::Bool(false));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
        py::rec_set(&rec887, &v_bt);
        if (py::call(&v_bt.clone(), &[Py::Int(0i64)])?).truthy() {
            return Ok(Py::dict(vec![(Py::str("chromatic_number"), py::float(&v_k.clone())?), (Py::str("coloring_example"), { let mut out = vec![]; for t896 in py::iter(&(v_nodes.clone()))? { let mut v_v: Py; 
v_v = t896;
out.push(py::float(&py::getitem(&v_col.clone(), &v_v.clone())?)?); } Py::list(out) }), (Py::str("nodes"), v_nodes.clone())]));
        }
    }
    return Err(py::value_error(py::to_str(&Py::str("no colouring found"))));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cpm(mut v_durations: Py, mut v_precedences: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_ES: Py = Py::None;
    let mut v_LF: Py = Py::None;
    let mut v_LS: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_order: Py = Py::None;
    let mut v_preds: Py = Py::None;
    let mut v_slack: Py = Py::None;
    let mut v_succ: Py = Py::None;
    v_D = { let mut out = vec![]; for t523 in py::iter(&(v_durations.clone()))? { let mut v_v: Py; 
v_v = t523;
out.push(f__fr(v_v.clone())?); } Py::list(out) };
    let t551 = (f__project(v_durations.clone(), v_precedences.clone())?).items()?;
    if t551.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_preds = t551[0].clone();
    v_order = t551[1].clone();
    v_n = py::len(&v_D.clone())?;
    v_ES = py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &v_n.clone())?;
    for t552 in py::iter(&(v_order.clone()))? {
        v_k = t552;
        py::setitem(&v_ES.clone(), &v_k.clone(), { let t = { let mut out = vec![]; for t553 in py::iter(&(py::getitem(&v_preds.clone(), &v_k.clone())?))? { let mut v_j: Py; 
v_j = t553;
out.push(py::add(&py::getitem(&v_ES.clone(), &v_j.clone())?, &py::getitem(&v_D.clone(), &v_j.clone())?)?); } Py::list(out) }; if t.items()?.is_empty() { py::fraction(&[Py::Int(0i64)])? } else { py::minmax(&[t], true)? } })?;
    }
    v_T = py::minmax(&[{ let mut out = vec![]; for t554 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t554;
out.push(py::add(&py::getitem(&v_ES.clone(), &v_k.clone())?, &py::getitem(&v_D.clone(), &v_k.clone())?)?); } Py::list(out) }], true)?;
    v_succ = { let mut out = vec![]; for t555 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t555;
out.push({ let mut out = vec![]; for t556 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t556;
if ({ let t557 = v_j.clone(); Py::Bool({ let t558 = py::getitem(&v_preds.clone(), &v_k.clone())?; if py::contains(&t558, &t557)? { true } else { false } }) }).truthy() { out.push(v_k.clone()); }} Py::list(out) }); } Py::list(out) };
    v_LF = py::mul(&Py::list(vec![v_T.clone()]), &v_n.clone())?;
    for t559 in py::iter(&(py::reversed(&v_order.clone())?))? {
        v_k = t559;
        py::setitem(&v_LF.clone(), &v_k.clone(), { let t = { let mut out = vec![]; for t560 in py::iter(&(py::getitem(&v_succ.clone(), &v_k.clone())?))? { let mut v_s: Py; 
v_s = t560;
out.push(py::sub(&py::getitem(&v_LF.clone(), &v_s.clone())?, &py::getitem(&v_D.clone(), &v_s.clone())?)?); } Py::list(out) }; if t.items()?.is_empty() { v_T.clone() } else { py::minmax(&[t], false)? } })?;
    }
    v_LS = { let mut out = vec![]; for t561 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t561;
out.push(py::sub(&py::getitem(&v_LF.clone(), &v_k.clone())?, &py::getitem(&v_D.clone(), &v_k.clone())?)?); } Py::list(out) };
    v_slack = { let mut out = vec![]; for t562 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t562;
out.push(py::sub(&py::getitem(&v_LS.clone(), &v_k.clone())?, &py::getitem(&v_ES.clone(), &v_k.clone())?)?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("project_duration"), py::float(&v_T.clone())?), (Py::str("critical_activities"), { let mut out = vec![]; for t563 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t563;
if ({ let t564 = py::getitem(&v_slack.clone(), &v_k.clone())?; Py::Bool({ let t565 = Py::Int(0i64); if py::eq(&t564, &t565) { true } else { false } }) }).truthy() { out.push(py::chr(&py::add(&Py::Int(65i64), &v_k.clone())?)?); }} Py::list(out) }), (Py::str("slack"), f__fl(v_slack.clone())?), (Py::str("earliest_start"), f__fl(v_ES.clone())?), (Py::str("latest_start"), f__fl(v_LS.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_critical_point_2var(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_d: Py, mut v_e: Py, mut v_f0: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_D: Py = Py::None;
    let mut v_E: Py = Py::None;
    let mut v_G: Py = Py::None;
    let mut v_det: Py = Py::None;
    let mut v_kind: Py = Py::None;
    let mut v_val: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let _ = f__finite(Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone(), v_d.clone(), v_e.clone(), v_f0.clone()]))?;
    let t577 = ({ let mut out = vec![]; for t576 in py::iter(&(Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone(), v_d.clone(), v_e.clone(), v_f0.clone()])))? { let mut v_v: Py; 
v_v = t576;
out.push(f__fr(v_v.clone())?); } Py::list(out) }).items()?;
    if t577.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_A = t577[0].clone();
    v_B = t577[1].clone();
    v_C = t577[2].clone();
    v_D = t577[3].clone();
    v_E = t577[4].clone();
    v_G = t577[5].clone();
    v_det = py::sub(&py::mul(&py::mul(&Py::Int(4i64), &v_A.clone())?, &v_C.clone())?, &py::mul(&v_B.clone(), &v_B.clone())?)?;
    if ({ let t578 = v_det.clone(); Py::Bool({ let t579 = Py::Int(0i64); if py::eq(&t578, &t579) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("4ac - b^2 = 0: no isolated critical point (the second-derivative test is inconclusive)"))));
    }
    let t599 = (f__solve_exact(Py::list(vec![Py::list(vec![py::mul(&Py::Int(2i64), &v_A.clone())?, v_B.clone()]), Py::list(vec![v_B.clone(), py::mul(&Py::Int(2i64), &v_C.clone())?])]), Py::list(vec![py::neg(&v_D.clone())?, py::neg(&v_E.clone())?]))?).items()?;
    if t599.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t599[0].clone();
    v_y = t599[1].clone();
    v_kind = (if ({ let t600 = v_det.clone(); Py::Bool({ let t601 = Py::Int(0i64); if py::lt(&t600, &t601)? { true } else { false } }) }).truthy() { Py::str("saddle point") } else { (if ({ let t602 = v_A.clone(); Py::Bool({ let t603 = Py::Int(0i64); if py::gt(&t602, &t603)? { true } else { false } }) }).truthy() { Py::str("local minimum") } else { Py::str("local maximum") }) });
    v_val = py::add(&py::add(&py::add(&py::add(&py::add(&py::mul(&py::mul(&v_A.clone(), &v_x.clone())?, &v_x.clone())?, &py::mul(&py::mul(&v_B.clone(), &v_x.clone())?, &v_y.clone())?)?, &py::mul(&py::mul(&v_C.clone(), &v_y.clone())?, &v_y.clone())?)?, &py::mul(&v_D.clone(), &v_x.clone())?)?, &py::mul(&v_E.clone(), &v_y.clone())?)?, &v_G.clone())?;
    return Ok(Py::dict(vec![(Py::str("f_value"), py::float(&v_val.clone())?), (Py::str("x"), py::float(&v_x.clone())?), (Py::str("y"), py::float(&v_y.clone())?), (Py::str("type"), v_kind.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_edd(mut v_p: Py, mut v_d: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_Lmax: Py = Py::None;
    let mut v_late: Py = Py::None;
    let mut v_seq: Py = Py::None;
    let _ = f__jobs(v_p.clone(), Py::str("processing times"))?;
    if ({ let t621 = py::len(&v_d.clone())?; Py::Bool({ let t622 = py::len(&v_p.clone())?; if py::ne(&t621, &t622) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give one due date per job"))));
    }
    v_seq = py::sorted_key(&Py::list((py::range_iter(&[(py::len(&v_p.clone())?).i()?])).into_iter().collect::<Vec<Py>>()), &{ let v_d = v_d.clone(); let v_p = v_p.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_j: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::getitem(&v_d.clone(), &v_j.clone())?, v_j.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    v_C = f__completion(v_seq.clone(), v_p.clone())?;
    v_late = { let mut out = vec![]; for t625 in py::iter(&(v_seq.clone()))? { let mut v_j: Py; 
v_j = t625;
out.push(py::sub(&py::getitem(&v_C.clone(), &v_j.clone())?, &f__fr(py::getitem(&v_d.clone(), &v_j.clone())?)?)?); } Py::list(out) };
    v_Lmax = py::minmax(&[v_late.clone()], true)?;
    return Ok(Py::dict(vec![(Py::str("sequence"), { let mut out = vec![]; for t626 in py::iter(&(v_seq.clone()))? { let mut v_j: Py; 
v_j = t626;
out.push(py::add(&v_j.clone(), &Py::Int(1i64))?); } Py::list(out) }), (Py::str("max_lateness"), py::float(&v_Lmax.clone())?), (Py::str("max_tardiness"), py::float(&py::minmax(&[v_Lmax.clone(), Py::Int(0i64)], true)?)?), (Py::str("tardy_jobs_in_sequence"), py::float(&py::sum(&{ let mut out = vec![]; for t627 in py::iter(&(v_late.clone()))? { let mut v_v: Py; 
v_v = t627;
if ({ let t628 = v_v.clone(); Py::Bool({ let t629 = Py::Int(0i64); if py::gt(&t628, &t629)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_eoq(mut v_D: Py, mut v_S: Py, mut v_H: Py) -> R {
    let mut v_Q: Py = Py::None;
    let _ = f__finite(Py::tuple(vec![v_D.clone(), v_S.clone(), v_H.clone()]))?;
    if ({ let t651 = { let t644 = v_D.clone(); Py::Bool({ let t645 = Py::Int(0i64); if py::le(&t644, &t645)? { true } else { false } }) }; if t651.truthy() { t651 } else { { let t650 = { let t646 = v_S.clone(); Py::Bool({ let t647 = Py::Int(0i64); if py::le(&t646, &t647)? { true } else { false } }) }; if t650.truthy() { t650 } else { { let t648 = v_H.clone(); Py::Bool({ let t649 = Py::Int(0i64); if py::le(&t648, &t649)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("demand, ordering cost and holding cost must all be > 0"))));
    }
    v_Q = py::math1("sqrt", &py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &v_D.clone())?, &v_S.clone())?, &v_H.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("Q"), v_Q.clone()), (Py::str("total_cost"), py::math1("sqrt", &py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_D.clone())?, &v_S.clone())?, &v_H.clone())?)?), (Py::str("orders_per_period"), py::truediv(&v_D.clone(), &v_Q.clone())?), (Py::str("cycle_time"), py::truediv(&v_Q.clone(), &v_D.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_eoq_backorders(mut v_D: Py, mut v_S: Py, mut v_H: Py, mut v_B: Py) -> R {
    let mut v_Q: Py = Py::None;
    let mut v_s: Py = Py::None;
    let _ = f__finite(Py::tuple(vec![v_D.clone(), v_S.clone(), v_H.clone(), v_B.clone()]))?;
    if ({ let t656 = py::minmax(&[v_D.clone(), v_S.clone(), v_H.clone(), v_B.clone()], false)?; Py::Bool({ let t657 = Py::Int(0i64); if py::le(&t656, &t657)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("demand, ordering, holding and shortage costs must all be > 0"))));
    }
    v_Q = py::math1("sqrt", &py::truediv(&py::mul(&py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &v_D.clone())?, &v_S.clone())?, &v_H.clone())?, &py::add(&v_H.clone(), &v_B.clone())?)?, &v_B.clone())?)?;
    v_s = py::truediv(&py::mul(&v_Q.clone(), &v_H.clone())?, &py::add(&v_H.clone(), &v_B.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("Q"), v_Q.clone()), (Py::str("max_backorder"), v_s.clone()), (Py::str("max_inventory"), py::sub(&v_Q.clone(), &v_s.clone())?), (Py::str("total_cost"), py::math1("sqrt", &py::truediv(&py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_D.clone())?, &v_S.clone())?, &v_H.clone())?, &v_B.clone())?, &py::add(&v_H.clone(), &v_B.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_eoq_discount(mut v_D: Py, mut v_S: Py, mut v_i_rate: Py, mut v_breaks: Py, mut v_prices: Py) -> R {
    let mut v_Q: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q0: Py = Py::None;
    let mut v_tc: Py = Py::None;
    let _ = f__finite(Py::tuple(vec![v_D.clone(), v_S.clone(), v_i_rate.clone()]))?;
    if ({ let t663 = py::minmax(&[v_D.clone(), v_S.clone(), v_i_rate.clone()], false)?; Py::Bool({ let t664 = Py::Int(0i64); if py::le(&t663, &t664)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("demand, ordering cost and holding rate must be > 0"))));
    }
    if ({ let t667 = { let t665 = py::len(&v_breaks.clone())?; Py::Bool({ let t666 = py::len(&v_prices.clone())?; if py::ne(&t665, &t666) { true } else { false } }) }; if t667.truthy() { t667 } else { py::not(&v_breaks.clone()) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give one price per quantity break"))));
    }
    if ({ let t672 = { let t668 = py::getitem(&v_breaks.clone(), &Py::Int(0i64))?; Py::Bool({ let t669 = Py::Int(0i64); if py::ne(&t668, &t669) { true } else { false } }) }; if !t672.truthy() { t672 } else { { let t670 = py::getitem(&v_breaks.clone(), &Py::Int(0i64))?; Py::Bool({ let t671 = Py::Int(1i64); if py::gt(&t670, &t671)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the first price break must start at 0 or 1 unit"))));
    }
    if ({ let t680 = Py::Bool(({ let mut out = vec![]; for t673 in py::iter(&(py::zip(&[v_breaks.clone(), py::slice(&v_breaks.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_b1: Py; let mut v_b2: Py; 
let t674 = (t673).items()?;
if t674.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_b1 = t674[0].clone();
v_b2 = t674[1].clone();
out.push({ let t675 = v_b2.clone(); Py::Bool({ let t676 = v_b1.clone(); if py::le(&t675, &t676)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t680.truthy() { t680 } else { Py::Bool(({ let mut out = vec![]; for t677 in py::iter(&(v_prices.clone()))? { let mut v_p: Py; 
v_p = t677;
out.push({ let t678 = v_p.clone(); Py::Bool({ let t679 = Py::Int(0i64); if py::le(&t678, &t679)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("quantity breaks must increase and prices must be > 0"))));
    }
    v_best = Py::None;
    for t681 in py::iter(&(py::enumerate(&py::zip(&[v_breaks.clone(), v_prices.clone()])?, 0)?))? {
        let t691 = (t681).items()?;
        if t691.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t691[0].clone();
        let t692 = (t691[1].clone()).items()?;
        if t692.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_q0 = t692[0].clone();
        v_p = t692[1].clone();
        v_hi = (if ({ let t682 = py::add(&v_k.clone(), &Py::Int(1i64))?; Py::Bool({ let t683 = py::len(&v_breaks.clone())?; if py::lt(&t682, &t683)? { true } else { false } }) }).truthy() { py::getitem(&v_breaks.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)? } else { Py::Float(f64::INFINITY) });
        v_Q = py::math1("sqrt", &py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &v_D.clone())?, &v_S.clone())?, &py::mul(&v_i_rate.clone(), &v_p.clone())?)?)?;
        if ({ let t684 = v_Q.clone(); Py::Bool({ let t685 = v_hi.clone(); if py::ge(&t684, &t685)? { true } else { false } }) }).truthy() {
            continue;
        }
        v_Q = py::minmax(&[v_Q.clone(), v_q0.clone(), Py::Float(1e-300)], true)?;
        v_tc = py::add(&py::add(&py::mul(&v_D.clone(), &v_p.clone())?, &py::truediv(&py::mul(&v_D.clone(), &v_S.clone())?, &v_Q.clone())?)?, &py::truediv(&py::mul(&py::mul(&v_i_rate.clone(), &v_p.clone())?, &v_Q.clone())?, &Py::Int(2i64))?)?;
        if ({ let t690 = { let t686 = v_best.clone(); Py::Bool({ let t687 = Py::None; if py::is(&t686, &t687) { true } else { false } }) }; if t690.truthy() { t690 } else { { let t688 = v_tc.clone(); Py::Bool({ let t689 = py::sub(&py::getitem(&v_best.clone(), &Py::Int(0i64))?, &py::mul(&Py::Float(1e-12), &v_tc.clone())?)?; if py::lt(&t688, &t689)? { true } else { false } }) } } }).truthy() {
            v_best = Py::tuple(vec![v_tc.clone(), v_Q.clone(), v_p.clone()]);
        }
    }
    return Ok(Py::dict(vec![(Py::str("order_quantity"), py::getitem(&v_best.clone(), &Py::Int(1i64))?), (Py::str("total_cost"), py::getitem(&v_best.clone(), &Py::Int(0i64))?), (Py::str("unit_price"), py::getitem(&v_best.clone(), &Py::Int(2i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_epq(mut v_D: Py, mut v_P: Py, mut v_S: Py, mut v_H: Py) -> R {
    let mut v_Q: Py = Py::None;
    let mut v_k: Py = Py::None;
    let _ = f__finite(Py::tuple(vec![v_D.clone(), v_P.clone(), v_S.clone(), v_H.clone()]))?;
    if ({ let t713 = py::minmax(&[v_D.clone(), v_P.clone(), v_S.clone(), v_H.clone()], false)?; Py::Bool({ let t714 = Py::Int(0i64); if py::le(&t713, &t714)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("demand, production rate, setup and holding costs must be > 0"))));
    }
    if ({ let t715 = v_P.clone(); Py::Bool({ let t716 = v_D.clone(); if py::le(&t715, &t716)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the production rate must exceed the demand rate"))));
    }
    v_k = py::sub(&Py::Int(1i64), &py::truediv(&v_D.clone(), &v_P.clone())?)?;
    v_Q = py::math1("sqrt", &py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &v_D.clone())?, &v_S.clone())?, &py::mul(&v_H.clone(), &v_k.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("Q"), v_Q.clone()), (Py::str("max_inventory"), py::mul(&v_Q.clone(), &v_k.clone())?), (Py::str("total_cost"), py::math1("sqrt", &py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_D.clone())?, &v_S.clone())?, &v_H.clone())?, &v_k.clone())?)?), (Py::str("runs_per_period"), py::truediv(&v_D.clone(), &v_Q.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_eq_qp(mut v_Q: Py, mut v_c: Py, mut v_A: Py, mut v_b: Py) -> R {
    let mut v_Af: Py = Py::None;
    let mut v_K: Py = Py::None;
    let mut v_M: Py = Py::None;
    let mut v_bf: Py = Py::None;
    let mut v_cf: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_z: Py = Py::None;
    v_M = f__sym_pd(v_Q.clone(), Py::str("Q"))?;
    v_n = py::len(&v_M.clone())?;
    if ({ let t741 = py::len(&v_c.clone())?; Py::Bool({ let t742 = v_n.clone(); if py::ne(&t741, &t742) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["c must have ".to_string(), py::format(&v_n.clone(), "")?, " entries".to_string()].concat()))));
    }
    if ({ let t752 = py::not(&v_A.clone()); if t752.truthy() { t752 } else { { let t751 = Py::Bool(({ let mut out = vec![]; for t743 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t743;
out.push({ let t744 = py::len(&v_r.clone())?; Py::Bool({ let t745 = v_n.clone(); if py::ne(&t744, &t745) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t751.truthy() { t751 } else { { let t750 = { let t746 = py::len(&v_A.clone())?; Py::Bool({ let t747 = py::len(&v_b.clone())?; if py::ne(&t746, &t747) { true } else { false } }) }; if t750.truthy() { t750 } else { { let t748 = py::len(&v_A.clone())?; Py::Bool({ let t749 = py::add(&v_n.clone(), &Py::Int(1i64))?; if py::ge(&t748, &t749)? { true } else { false } }) } } } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["A must have ".to_string(), py::format(&v_n.clone(), "")?, " columns and one row per entry of b (at most ".to_string(), py::format(&v_n.clone(), "")?, " rows)".to_string()].concat()))));
    }
    let t757 = (Py::tuple(vec![{ let mut out = vec![]; for t753 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t753;
out.push({ let mut out = vec![]; for t754 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t754;
out.push(f__fr(v_v.clone())?); } Py::list(out) }); } Py::list(out) }, { let mut out = vec![]; for t755 in py::iter(&(v_b.clone()))? { let mut v_v: Py; 
v_v = t755;
out.push(f__fr(v_v.clone())?); } Py::list(out) }, { let mut out = vec![]; for t756 in py::iter(&(v_c.clone()))? { let mut v_v: Py; 
v_v = t756;
out.push(f__fr(v_v.clone())?); } Py::list(out) }])).items()?;
    if t757.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Af = t757[0].clone();
    v_bf = t757[1].clone();
    v_cf = t757[2].clone();
    v_p = py::len(&v_Af.clone())?;
    v_K = py::add(&{ let mut out = vec![]; for t758 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t758;
out.push(py::add(&py::getitem(&v_M.clone(), &v_i.clone())?, &{ let mut out = vec![]; for t759 in py::range_iter(&[(v_p.clone()).i()?]) { let mut v_k: Py; 
v_k = t759;
out.push(py::getitem(&py::getitem(&v_Af.clone(), &v_k.clone())?, &v_i.clone())?); } Py::list(out) })?); } Py::list(out) }, &{ let mut out = vec![]; for t760 in py::range_iter(&[(v_p.clone()).i()?]) { let mut v_k: Py; 
v_k = t760;
out.push(py::add(&py::getitem(&v_Af.clone(), &v_k.clone())?, &py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &v_p.clone())?)?); } Py::list(out) })?;
    let t781: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_z = f__solve_exact(v_K.clone(), py::add(&{ let mut out = vec![]; for t780 in py::iter(&(v_cf.clone()))? { let mut v_v: Py; 
v_v = t780;
out.push(py::neg(&v_v.clone())?); } Py::list(out) }, &v_bf.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t781 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            return Err(py::value_error(py::to_str(&Py::str("the constraint rows must be linearly independent"))));
        }
        Err(e) => return Err(e),
    }
    let t782 = (Py::tuple(vec![py::slice(&v_z.clone(), None, Some((v_n.clone()).i()?), None)?, py::slice(&v_z.clone(), Some((v_n.clone()).i()?), None, None)?])).items()?;
    if t782.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t782[0].clone();
    v_lam = t782[1].clone();
    return Ok(Py::dict(vec![(Py::str("f_min"), py::float(&f__qf(v_M.clone(), v_cf.clone(), v_x.clone())?)?), (Py::str("x_star"), f__fl(v_x.clone())?), (Py::str("multipliers"), f__fl(v_lam.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_erlang_b(mut v_A: Py, mut v_c: Py) -> R {
    let mut v_B: Py = Py::None;
    let _ = f__finite(Py::tuple(vec![v_A.clone()]))?;
    if ({ let t816 = v_A.clone(); Py::Bool({ let t817 = Py::Int(0i64); if py::le(&t816, &t817)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the offered traffic must be > 0"))));
    }
    if (py::not(&{ let t818 = Py::Int(1i64); Py::Bool({ let t819 = v_c.clone(); if py::le(&t818, &t819)? { { let t820 = Py::Int(5000i64); if py::le(&t819, &t820)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the number of servers must be between 1 and 5000"))));
    }
    v_B = f__erlang_b(v_A.clone(), v_c.clone())?;
    return Ok(Py::dict(vec![(Py::str("blocking_probability"), v_B.clone()), (Py::str("carried_traffic"), py::mul(&v_A.clone(), &py::sub(&Py::Int(1i64), &v_B.clone())?)?), (Py::str("lost_traffic"), py::mul(&v_A.clone(), &v_B.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_facility_location(mut v_fixed: Py, mut v_cost: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_Fx: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_op: Py = Py::None;
    let mut v_pick: Py = Py::None;
    let mut v_tot: Py = Py::None;
    v_m = py::len(&v_fixed.clone())?;
    if ({ let t834 = { let t827 = v_m.clone(); Py::Bool({ let t828 = Py::Int(0i64); if py::eq(&t827, &t828) { true } else { false } }) }; if t834.truthy() { t834 } else { { let t833 = { let t829 = v_m.clone(); Py::Bool({ let t830 = Py::Int(15i64); if py::gt(&t829, &t830)? { true } else { false } }) }; if t833.truthy() { t833 } else { { let t831 = py::len(&v_cost.clone())?; Py::Bool({ let t832 = v_m.clone(); if py::ne(&t831, &t832) { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give between 1 and 15 facilities: one fixed cost and one row of service costs each"))));
    }
    v_n = py::len(&py::getitem(&v_cost.clone(), &Py::Int(0i64))?)?;
    let t838 = (Py::tuple(vec![{ let mut out = vec![]; for t835 in py::iter(&(v_fixed.clone()))? { let mut v_f: Py; 
v_f = t835;
out.push(f__fr(v_f.clone())?); } Py::list(out) }, { let mut out = vec![]; for t836 in py::iter(&(v_cost.clone()))? { let mut v_r: Py; 
v_r = t836;
out.push({ let mut out = vec![]; for t837 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t837;
out.push(f__fr(v_x.clone())?); } Py::list(out) }); } Py::list(out) }])).items()?;
    if t838.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Fx = t838[0].clone();
    v_C = t838[1].clone();
    v_best = Py::None;
    for t839 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::lshift(&Py::Int(1i64), &v_m.clone())?).i()?]) {
        v_pick = t839;
        v_op = { let mut out = vec![]; for t840 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t840;
if (py::bitand(&py::rshift(&v_pick.clone(), &v_i.clone())?, &Py::Int(1i64))?).truthy() { out.push(v_i.clone()); }} Py::list(out) };
        v_tot = py::add(&py::sum(&{ let mut out = vec![]; for t841 in py::iter(&(v_op.clone()))? { let mut v_i: Py; 
v_i = t841;
out.push(py::getitem(&v_Fx.clone(), &v_i.clone())?); } Py::list(out) }, None)?, &py::sum(&{ let mut out = vec![]; for t842 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t842;
out.push(py::minmax(&[{ let mut out = vec![]; for t843 in py::iter(&(v_op.clone()))? { let mut v_i: Py; 
v_i = t843;
out.push(py::getitem(&py::getitem(&v_C.clone(), &v_i.clone())?, &v_j.clone())?); } Py::list(out) }], false)?); } Py::list(out) }, None)?)?;
        if ({ let t848 = { let t844 = v_best.clone(); Py::Bool({ let t845 = Py::None; if py::is(&t844, &t845) { true } else { false } }) }; if t848.truthy() { t848 } else { { let t846 = v_tot.clone(); Py::Bool({ let t847 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; if py::lt(&t846, &t847)? { true } else { false } }) } } }).truthy() {
            v_best = Py::tuple(vec![v_tot.clone(), v_op.clone()]);
        }
    }
    return Ok(Py::dict(vec![(Py::str("min_cost"), py::float(&py::getitem(&v_best.clone(), &Py::Int(0i64))?)?), (Py::str("open_example"), { let mut out = vec![]; for t849 in py::iter(&(py::getitem(&v_best.clone(), &Py::Int(1i64))?))? { let mut v_i: Py; 
v_i = t849;
out.push(py::add(&v_i.clone(), &Py::Int(1i64))?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fence_max_area(mut v_L: Py, mut v_wall: Py) -> R {
    let mut v_Lf: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__finite(Py::tuple(vec![v_L.clone()]))?;
    if ({ let t863 = v_L.clone(); Py::Bool({ let t864 = Py::Int(0i64); if py::le(&t863, &t864)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the fence length must be > 0"))));
    }
    v_Lf = f__fr(v_L.clone())?;
    if (v_wall.clone()).truthy() {
        v_x = py::truediv(&v_Lf.clone(), &Py::Int(4i64))?;
        return Ok(Py::dict(vec![(Py::str("width"), py::float(&v_x.clone())?), (Py::str("length"), py::float(&py::sub(&v_Lf.clone(), &py::mul(&Py::Int(2i64), &v_x.clone())?)?)?), (Py::str("max_area"), py::float(&py::mul(&v_x.clone(), &py::sub(&v_Lf.clone(), &py::mul(&Py::Int(2i64), &v_x.clone())?)?)?)?)]));
    }
    return Ok(Py::dict(vec![(Py::str("width"), py::float(&py::truediv(&v_Lf.clone(), &Py::Int(4i64))?)?), (Py::str("length"), py::float(&py::truediv(&v_Lf.clone(), &Py::Int(4i64))?)?), (Py::str("max_area"), py::float(&py::truediv(&py::mul(&v_Lf.clone(), &v_Lf.clone())?, &Py::Int(16i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_integer_program(mut v_sense: Py, mut v_c: Py, mut v_A: Py, mut v_b: Py, mut v_relations: Py, mut v__last: Py) -> R {
    let mut v_best: Py = Py::None;
    let mut v_better: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_nodes: Py = Py::None;
    let mut v_rel: Py = Py::None;
    let mut v_solve: Py = Py::None;
    v_rel = f__relations(v_relations.clone(), py::len(&v_A.clone())?)?;
    let _ = f__check_lp(v_sense.clone(), v_c.clone(), v_A.clone(), v_b.clone(), v_rel.clone())?;
    v_n = py::len(&v_c.clone())?;
    v_best = Py::list(vec![Py::None, Py::None]);
    v_nodes = Py::list(vec![Py::Int(0i64)]);
    v_better = (if ({ let t946 = v_sense.clone(); Py::Bool({ let t947 = Py::str("maximize"); if py::eq(&t946, &t947) { true } else { false } }) }).truthy() { { let v_A = v_A.clone(); let v__last = v__last.clone(); let v_b = v_b.clone(); let v_best = v_best.clone(); let v_c = v_c.clone(); let v_n = v_n.clone(); let v_nodes = v_nodes.clone(); let v_rel = v_rel.clone(); let v_relations = v_relations.clone(); let v_sense = v_sense.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_u: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_v: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok({ let t948 = v_u.clone(); Py::Bool({ let t949 = v_v.clone(); if py::gt(&t948, &t949)? { true } else { false } }) });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) } } else { { let v_A = v_A.clone(); let v__last = v__last.clone(); let v_b = v_b.clone(); let v_best = v_best.clone(); let v_c = v_c.clone(); let v_n = v_n.clone(); let v_nodes = v_nodes.clone(); let v_rel = v_rel.clone(); let v_relations = v_relations.clone(); let v_sense = v_sense.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_u: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_v: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok({ let t950 = v_u.clone(); Py::Bool({ let t951 = v_v.clone(); if py::lt(&t950, &t951)? { true } else { false } }) });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) } });
    let rec952 = py::rec_cell();
    v_solve = { let v_A = v_A.clone(); let v__last = v__last.clone(); let v_b = v_b.clone(); let v_best = v_best.clone(); let v_better = v_better.clone(); let v_c = v_c.clone(); let v_n = v_n.clone(); let v_nodes = v_nodes.clone(); let v_rel = v_rel.clone(); let v_relations = v_relations.clone(); let v_sense = v_sense.clone(); let rec952 = rec952.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_extraA: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_extrab: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_extrar: Py = a.get(2).cloned().unwrap_or(Py::None);
            let v_solve: Py = py::rec_get(&rec952)?;
            let mut v_br: Py = Py::None;
            let mut v_down: Py = Py::None;
            let mut v_exc: Py = Py::None;
            let mut v_fl: Py = Py::None;
            let mut v_frac: Py = Py::None;
            let mut v_fracs: Py = Py::None;
            let mut v_r: Py = Py::None;
            let mut v_row: Py = Py::None;
            let mut v_up: Py = Py::None;
            let mut v_v: Py = Py::None;
            let t953 = v_nodes.clone(); let t954 = Py::Int(0i64);
            py::setitem(&t953, &t954, py::iadd(&py::getitem(&t953, &t954)?, &Py::Int(1i64))?)?;
            if ({ let t955 = py::getitem(&v_nodes.clone(), &Py::Int(0i64))?; Py::Bool({ let t956 = Py::Int(4000i64); if py::gt(&t955, &t956)? { true } else { false } }) }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("branch and bound exceeded 4000 nodes; this problem is too large for this capability"))));
            }
            let t1087: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                    v_r = f__lp(v_sense.clone(), v_c.clone(), py::add(&Py::list((v_A.clone()).items()?), &v_extraA.clone())?, py::add(&Py::list((v_b.clone()).items()?), &v_extrab.clone())?, py::add(&v_rel.clone(), &v_extrar.clone())?, Py::Bool(false))?;
                    #[allow(unreachable_code)]
                    Ok(None)
                })();
            match t1087 {
                Ok(Some(r)) => { return Ok(r); }
                Ok(None) => {}
                Err(e) if (matches!(e, PyErr::Value(_))) => {
                    v_exc = Py::str(&match &e { PyErr::Value(m) => m.clone(), other => format!("{:?}", other) });
                    if ({ let t1090 = { let t1088 = Py::str("unbounded"); Py::Bool({ let t1089 = Py::str(&py::to_str(&v_exc.clone())); if py::contains(&t1089, &t1088)? { true } else { false } }) }; if !t1090.truthy() { t1090 } else { py::not(&v_extraA.clone()) } }).truthy() {
                        return Err(py::value_error(py::to_str(&Py::str("the LP relaxation is unbounded; this capability needs a bounded relaxation"))));
                    }
                    if ({ let t1091 = Py::str("unbounded"); Py::Bool({ let t1092 = Py::str(&py::to_str(&v_exc.clone())); if py::contains(&t1092, &t1091)? { true } else { false } }) }).truthy() {
                        return Err(e.clone());
                    }
                    return Ok(Py::None);
                }
                Err(e) => return Err(e),
            }
            if ({ let t1095 = { let t1093 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; Py::Bool({ let t1094 = Py::None; if !py::is(&t1093, &t1094) { true } else { false } }) }; if !t1095.truthy() { t1095 } else { py::not(&py::call(&v_better.clone(), &[py::getitem(&v_r.clone(), &Py::str("value"))?, py::getitem(&v_best.clone(), &Py::Int(0i64))?])?) } }).truthy() {
                return Ok(Py::None);
            }
            v_fracs = { let mut out = vec![]; for t1096 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t1096;
if ({ let t1097 = py::denominator(&py::getitem(&py::getitem(&v_r.clone(), &Py::str("x"))?, &v_j.clone())?)?; Py::Bool({ let t1098 = Py::Int(1i64); if py::ne(&t1097, &t1098) { true } else { false } }) }).truthy() { out.push(v_j.clone()); }} Py::list(out) };
            v_frac = (if (v_fracs.clone()).truthy() { (if (v__last.clone()).truthy() { py::getitem(&v_fracs.clone(), &py::neg(&Py::Int(1i64))?)? } else { py::getitem(&v_fracs.clone(), &Py::Int(0i64))? }) } else { Py::None });
            if ({ let t1099 = v_frac.clone(); Py::Bool({ let t1100 = Py::None; if py::is(&t1099, &t1100) { true } else { false } }) }).truthy() {
                let t1101 = (Py::tuple(vec![py::getitem(&v_r.clone(), &Py::str("value"))?, py::getitem(&v_r.clone(), &Py::str("x"))?])).items()?;
                if t1101.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                py::setitem(&v_best.clone(), &Py::Int(0i64), t1101[0].clone())?;
                py::setitem(&v_best.clone(), &Py::Int(1i64), t1101[1].clone())?;
                return Ok(Py::None);
            }
            v_v = py::getitem(&py::getitem(&v_r.clone(), &Py::str("x"))?, &v_frac.clone())?;
            v_row = py::mul(&Py::list(vec![Py::Int(0i64)]), &v_n.clone())?;
            py::setitem(&v_row.clone(), &v_frac.clone(), Py::Int(1i64))?;
            v_fl = py::floordiv(&py::numerator(&v_v.clone())?, &py::denominator(&v_v.clone())?)?;
            v_down = Py::tuple(vec![py::add(&v_extraA.clone(), &Py::list(vec![v_row.clone()]))?, py::add(&v_extrab.clone(), &Py::list(vec![v_fl.clone()]))?, py::add(&v_extrar.clone(), &Py::list(vec![Py::str("<=")]))?]);
            v_up = Py::tuple(vec![py::add(&v_extraA.clone(), &Py::list(vec![v_row.clone()]))?, py::add(&v_extrab.clone(), &Py::list(vec![py::add(&v_fl.clone(), &Py::Int(1i64))?]))?, py::add(&v_extrar.clone(), &Py::list(vec![Py::str(">=")]))?]);
            for t1102 in py::iter(&((if (v__last.clone()).truthy() { Py::tuple(vec![v_up.clone(), v_down.clone()]) } else { Py::tuple(vec![v_down.clone(), v_up.clone()]) })))? {
                v_br = t1102;
                let _ = py::call(&v_solve.clone(), &({ let mut v: Vec<Py> = vec![]; v.extend((v_br.clone()).items()?); v })[..])?;
            }
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec952, &v_solve);
    let _ = py::call(&v_solve.clone(), &[Py::list(vec![]), Py::list(vec![]), Py::list(vec![])])?;
    if ({ let t1103 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; Py::Bool({ let t1104 = Py::None; if py::is(&t1103, &t1104) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("there is no integer point satisfying all the constraints"))));
    }
    return Ok(Py::dict(vec![(Py::str("optimal_value"), py::float(&py::getitem(&v_best.clone(), &Py::Int(0i64))?)?), (Py::str("x_one_optimal"), f__fl(py::getitem(&v_best.clone(), &Py::Int(1i64))?)?), (Py::str("nodes"), py::getitem(&v_nodes.clone(), &Py::Int(0i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_johnson(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_first: Py = Py::None;
    let mut v_last: Py = Py::None;
    let mut v_seq: Py = Py::None;
    let _ = f__jobs(v_a.clone(), Py::str("machine-1 times"))?;
    if ({ let t1134 = { let t1129 = py::len(&v_b.clone())?; Py::Bool({ let t1130 = py::len(&v_a.clone())?; if py::ne(&t1129, &t1130) { true } else { false } }) }; if t1134.truthy() { t1134 } else { Py::Bool(({ let mut out = vec![]; for t1131 in py::iter(&(v_b.clone()))? { let mut v_v: Py; 
v_v = t1131;
out.push({ let t1132 = v_v.clone(); Py::Bool({ let t1133 = Py::Int(0i64); if py::lt(&t1132, &t1133)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give one machine-2 time >= 0 per job"))));
    }
    v_first = py::sorted_key(&{ let mut out = vec![]; for t1135 in py::range_iter(&[(py::len(&v_a.clone())?).i()?]) { let mut v_j: Py; 
v_j = t1135;
if ({ let t1136 = py::getitem(&v_a.clone(), &v_j.clone())?; Py::Bool({ let t1137 = py::getitem(&v_b.clone(), &v_j.clone())?; if py::lt(&t1136, &t1137)? { true } else { false } }) }).truthy() { out.push(v_j.clone()); }} Py::list(out) }, &{ let v_a = v_a.clone(); let v_b = v_b.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_j: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::getitem(&v_a.clone(), &v_j.clone())?, v_j.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    v_last = py::sorted_key(&{ let mut out = vec![]; for t1138 in py::range_iter(&[(py::len(&v_a.clone())?).i()?]) { let mut v_j: Py; 
v_j = t1138;
if ({ let t1139 = py::getitem(&v_a.clone(), &v_j.clone())?; Py::Bool({ let t1140 = py::getitem(&v_b.clone(), &v_j.clone())?; if py::ge(&t1139, &t1140)? { true } else { false } }) }).truthy() { out.push(v_j.clone()); }} Py::list(out) }, &{ let v_a = v_a.clone(); let v_b = v_b.clone(); let v_first = v_first.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_j: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::neg(&py::getitem(&v_b.clone(), &v_j.clone())?)?, v_j.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    v_seq = py::add(&v_first.clone(), &v_last.clone())?;
    return Ok(Py::dict(vec![(Py::str("makespan"), py::float(&f__flow2(v_seq.clone(), v_a.clone(), v_b.clone())?)?), (Py::str("sequence"), { let mut out = vec![]; for t1143 in py::iter(&(v_seq.clone()))? { let mut v_j: Py; 
v_j = t1143;
out.push(py::add(&v_j.clone(), &Py::Int(1i64))?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_knapsack01(mut v_values: Py, mut v_weights: Py, mut v_capacity: Py) -> R {
    let mut v_V: Py = Py::None;
    let mut v_Wt: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_cap: Py = Py::None;
    let mut v_front: Py = Py::None;
    let mut v_it: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_new: Py = Py::None;
    let mut v_nw: Py = Py::None;
    let mut v_states: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__finite(Py::tuple(vec![v_capacity.clone()]))?;
    v_n = py::len(&v_values.clone())?;
    if ({ let t1157 = { let t1150 = v_n.clone(); Py::Bool({ let t1151 = Py::Int(0i64); if py::eq(&t1150, &t1151) { true } else { false } }) }; if t1157.truthy() { t1157 } else { { let t1156 = { let t1152 = v_n.clone(); Py::Bool({ let t1153 = Py::Int(40i64); if py::gt(&t1152, &t1153)? { true } else { false } }) }; if t1156.truthy() { t1156 } else { { let t1154 = py::len(&v_weights.clone())?; Py::Bool({ let t1155 = v_n.clone(); if py::ne(&t1154, &t1155) { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give between 1 and 40 items, one weight per value"))));
    }
    if ({ let t1167 = Py::Bool(({ let mut out = vec![]; for t1158 in py::iter(&(v_weights.clone()))? { let mut v_w: Py; 
v_w = t1158;
out.push({ let t1159 = v_w.clone(); Py::Bool({ let t1160 = Py::Int(0i64); if py::lt(&t1159, &t1160)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t1167.truthy() { t1167 } else { { let t1166 = Py::Bool(({ let mut out = vec![]; for t1161 in py::iter(&(v_values.clone()))? { let mut v_v: Py; 
v_v = t1161;
out.push({ let t1162 = v_v.clone(); Py::Bool({ let t1163 = Py::Int(0i64); if py::lt(&t1162, &t1163)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t1166.truthy() { t1166 } else { { let t1164 = v_capacity.clone(); Py::Bool({ let t1165 = Py::Int(0i64); if py::lt(&t1164, &t1165)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("values, weights and capacity must be >= 0"))));
    }
    let t1170 = (Py::tuple(vec![{ let mut out = vec![]; for t1168 in py::iter(&(v_values.clone()))? { let mut v_v: Py; 
v_v = t1168;
out.push(f__fr(v_v.clone())?); } Py::list(out) }, { let mut out = vec![]; for t1169 in py::iter(&(v_weights.clone()))? { let mut v_w: Py; 
v_w = t1169;
out.push(f__fr(v_w.clone())?); } Py::list(out) }, f__fr(v_capacity.clone())?])).items()?;
    if t1170.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_V = t1170[0].clone();
    v_Wt = t1170[1].clone();
    v_cap = t1170[2].clone();
    v_states = Py::dict(vec![(py::fraction(&[Py::Int(0i64)])?, Py::tuple(vec![py::fraction(&[Py::Int(0i64)])?, Py::tuple(vec![])]))]);
    for t1171 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t1171;
        v_new = py::dict_from(&v_states.clone(), vec![])?;
        for t1172 in py::iter(&(py::method(&v_states.clone(), "items", &[])?))? {
            let t1181 = (t1172).items()?;
            if t1181.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_w = t1181[0].clone();
            let t1182 = (t1181[1].clone()).items()?;
            if t1182.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_v = t1182[0].clone();
            v_it = t1182[1].clone();
            v_nw = py::add(&v_w.clone(), &py::getitem(&v_Wt.clone(), &v_k.clone())?)?;
            if ({ let t1180 = { let t1173 = v_nw.clone(); Py::Bool({ let t1174 = v_cap.clone(); if py::le(&t1173, &t1174)? { true } else { false } }) }; if !t1180.truthy() { t1180 } else { { let t1179 = { let t1175 = v_nw.clone(); Py::Bool({ let t1176 = v_new.clone(); if !py::contains(&t1176, &t1175)? { true } else { false } }) }; if t1179.truthy() { t1179 } else { { let t1177 = py::add(&v_v.clone(), &py::getitem(&v_V.clone(), &v_k.clone())?)?; Py::Bool({ let t1178 = py::getitem(&py::getitem(&v_new.clone(), &v_nw.clone())?, &Py::Int(0i64))?; if py::gt(&t1177, &t1178)? { true } else { false } }) } } } } }).truthy() {
                py::setitem(&v_new.clone(), &v_nw.clone(), Py::tuple(vec![py::add(&v_v.clone(), &py::getitem(&v_V.clone(), &v_k.clone())?)?, py::add(&v_it.clone(), &Py::tuple(vec![v_k.clone()]))?]))?;
            }
        }
        let t1183 = (Py::tuple(vec![Py::dict(vec![]), py::fraction(&[py::neg(&Py::Int(1i64))?])?])).items()?;
        if t1183.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_front = t1183[0].clone();
        v_best = t1183[1].clone();
        for t1184 in py::iter(&(py::sorted(&v_new.clone(), (Py::Bool(false)).truthy())?))? {
            v_w = t1184;
            if ({ let t1185 = py::getitem(&py::getitem(&v_new.clone(), &v_w.clone())?, &Py::Int(0i64))?; Py::Bool({ let t1186 = v_best.clone(); if py::gt(&t1185, &t1186)? { true } else { false } }) }).truthy() {
                let t1187 = (Py::tuple(vec![py::getitem(&v_new.clone(), &v_w.clone())?, py::getitem(&py::getitem(&v_new.clone(), &v_w.clone())?, &Py::Int(0i64))?])).items()?;
                if t1187.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                py::setitem(&v_front.clone(), &v_w.clone(), t1187[0].clone())?;
                v_best = t1187[1].clone();
            }
        }
        v_states = v_front.clone();
    }
    let t1188 = (py::minmax_key(&[py::method(&v_states.clone(), "items", &[])?], &{ let v_V = v_V.clone(); let v_Wt = v_Wt.clone(); let v_best = v_best.clone(); let v_cap = v_cap.clone(); let v_capacity = v_capacity.clone(); let v_front = v_front.clone(); let v_it = v_it.clone(); let v_k = v_k.clone(); let v_n = v_n.clone(); let v_new = v_new.clone(); let v_nw = v_nw.clone(); let v_states = v_states.clone(); let v_v = v_v.clone(); let v_values = v_values.clone(); let v_w = v_w.clone(); let v_weights = v_weights.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_kv: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::getitem(&py::getitem(&v_kv.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?, py::neg(&py::getitem(&v_kv.clone(), &Py::Int(0i64))?)?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, true)?).items()?;
    if t1188.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_w = t1188[0].clone();
    let t1189 = (t1188[1].clone()).items()?;
    if t1189.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v = t1189[0].clone();
    v_it = t1189[1].clone();
    return Ok(Py::dict(vec![(Py::str("max_value"), py::float(&v_v.clone())?), (Py::str("items_example"), { let mut out = vec![]; for t1190 in py::iter(&(v_it.clone()))? { let mut v_k: Py; 
v_k = t1190;
out.push(py::add(&v_k.clone(), &Py::Int(1i64))?); } Py::list(out) }), (Py::str("total_weight"), py::float(&v_w.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_knapsack_unbounded(mut v_values: Py, mut v_weights: Py, mut v_capacity: Py) -> R {
    let mut v_V: Py = Py::None;
    let mut v_W: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_cap: Py = Py::None;
    let mut v_choice: Py = Py::None;
    let mut v_counts: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_n = py::len(&v_values.clone())?;
    if ({ let t1210 = { let t1203 = v_n.clone(); Py::Bool({ let t1204 = Py::Int(0i64); if py::eq(&t1203, &t1204) { true } else { false } }) }; if t1210.truthy() { t1210 } else { { let t1209 = { let t1205 = v_n.clone(); Py::Bool({ let t1206 = Py::Int(40i64); if py::gt(&t1205, &t1206)? { true } else { false } }) }; if t1209.truthy() { t1209 } else { { let t1207 = py::len(&v_weights.clone())?; Py::Bool({ let t1208 = v_n.clone(); if py::ne(&t1207, &t1208) { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give between 1 and 40 item types, one weight per value"))));
    }
    if ({ let t1223 = Py::Bool(({ let mut out = vec![]; for t1211 in py::iter(&(v_weights.clone()))? { let mut v_w: Py; 
v_w = t1211;
out.push({ let t1216 = { let t1212 = v_w.clone(); Py::Bool({ let t1213 = Py::Int(0i64); if py::le(&t1212, &t1213)? { true } else { false } }) }; if t1216.truthy() { t1216 } else { { let t1214 = v_w.clone(); Py::Bool({ let t1215 = py::int(&v_w.clone())?; if py::ne(&t1214, &t1215) { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t1223.truthy() { t1223 } else { { let t1222 = { let t1217 = v_capacity.clone(); Py::Bool({ let t1218 = py::int(&v_capacity.clone())?; if py::ne(&t1217, &t1218) { true } else { false } }) }; if t1222.truthy() { t1222 } else { py::not(&{ let t1219 = Py::Int(0i64); Py::Bool({ let t1220 = v_capacity.clone(); if py::le(&t1219, &t1220)? { { let t1221 = Py::Int(100000i64); if py::le(&t1220, &t1221)? { true } else { false } } } else { false } }) }) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("weights must be positive whole numbers and the capacity a whole number up to 100000"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t1224 in py::iter(&(v_values.clone()))? { let mut v_v: Py; 
v_v = t1224;
out.push({ let t1225 = v_v.clone(); Py::Bool({ let t1226 = Py::Int(0i64); if py::lt(&t1225, &t1226)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("values must be >= 0"))));
    }
    let t1229 = (Py::tuple(vec![py::int(&v_capacity.clone())?, { let mut out = vec![]; for t1227 in py::iter(&(v_weights.clone()))? { let mut v_w: Py; 
v_w = t1227;
out.push(py::int(&v_w.clone())?); } Py::list(out) }, { let mut out = vec![]; for t1228 in py::iter(&(v_values.clone()))? { let mut v_v: Py; 
v_v = t1228;
out.push(f__fr(v_v.clone())?); } Py::list(out) }])).items()?;
    if t1229.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_cap = t1229[0].clone();
    v_W = t1229[1].clone();
    v_V = t1229[2].clone();
    let t1230 = (Py::tuple(vec![py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &py::add(&v_cap.clone(), &Py::Int(1i64))?)?, py::mul(&Py::list(vec![py::neg(&Py::Int(1i64))?]), &py::add(&v_cap.clone(), &Py::Int(1i64))?)?])).items()?;
    if t1230.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_best = t1230[0].clone();
    v_choice = t1230[1].clone();
    for t1231 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_cap.clone(), &Py::Int(1i64))?).i()?]) {
        v_c = t1231;
        let t1232 = (Py::tuple(vec![py::getitem(&v_best.clone(), &py::sub(&v_c.clone(), &Py::Int(1i64))?)?, py::neg(&Py::Int(1i64))?])).items()?;
        if t1232.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_best.clone(), &v_c.clone(), t1232[0].clone())?;
        py::setitem(&v_choice.clone(), &v_c.clone(), t1232[1].clone())?;
        for t1233 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_k = t1233;
            if ({ let t1238 = { let t1234 = py::getitem(&v_W.clone(), &v_k.clone())?; Py::Bool({ let t1235 = v_c.clone(); if py::le(&t1234, &t1235)? { true } else { false } }) }; if !t1238.truthy() { t1238 } else { { let t1236 = py::add(&py::getitem(&v_best.clone(), &py::sub(&v_c.clone(), &py::getitem(&v_W.clone(), &v_k.clone())?)?)?, &py::getitem(&v_V.clone(), &v_k.clone())?)?; Py::Bool({ let t1237 = py::getitem(&v_best.clone(), &v_c.clone())?; if py::gt(&t1236, &t1237)? { true } else { false } }) } } }).truthy() {
                let t1239 = (Py::tuple(vec![py::add(&py::getitem(&v_best.clone(), &py::sub(&v_c.clone(), &py::getitem(&v_W.clone(), &v_k.clone())?)?)?, &py::getitem(&v_V.clone(), &v_k.clone())?)?, v_k.clone()])).items()?;
                if t1239.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                py::setitem(&v_best.clone(), &v_c.clone(), t1239[0].clone())?;
                py::setitem(&v_choice.clone(), &v_c.clone(), t1239[1].clone())?;
            }
        }
    }
    let t1240 = (Py::tuple(vec![py::mul(&Py::list(vec![Py::Int(0i64)]), &v_n.clone())?, v_cap.clone()])).items()?;
    if t1240.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_counts = t1240[0].clone();
    v_c = t1240[1].clone();
    while ({ let t1245 = v_c.clone(); Py::Bool({ let t1246 = Py::Int(0i64); if py::gt(&t1245, &t1246)? { true } else { false } }) }).truthy() {
        if ({ let t1241 = py::getitem(&v_choice.clone(), &v_c.clone())?; Py::Bool({ let t1242 = Py::Int(0i64); if py::lt(&t1241, &t1242)? { true } else { false } }) }).truthy() {
            v_c = py::sub(&v_c, &Py::Int(1i64))?;
        } else {
            let t1243 = v_counts.clone(); let t1244 = py::getitem(&v_choice.clone(), &v_c.clone())?;
            py::setitem(&t1243, &t1244, py::iadd(&py::getitem(&t1243, &t1244)?, &Py::Int(1i64))?)?;
            v_c = py::sub(&v_c, &py::getitem(&v_W.clone(), &py::getitem(&v_choice.clone(), &v_c.clone())?)?)?;
        }
    }
    return Ok(Py::dict(vec![(Py::str("max_value"), py::float(&py::getitem(&v_best.clone(), &v_cap.clone())?)?), (Py::str("copies_example"), { let mut out = vec![]; for t1247 in py::iter(&(v_counts.clone()))? { let mut v_x: Py; 
v_x = t1247;
out.push(py::float(&v_x.clone())?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_linear_program(mut v_sense: Py, mut v_c: Py, mut v_A: Py, mut v_b: Py, mut v_relations: Py) -> R {
    let mut v_rel: Py = Py::None;
    v_rel = f__relations(v_relations.clone(), py::len(&v_A.clone())?)?;
    return Ok(f__lp_outputs(f__lp(v_sense.clone(), v_c.clone(), v_A.clone(), v_b.clone(), v_rel.clone(), Py::Bool(true))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_littlewood(mut v_p1: Py, mut v_p2: Py, mut v_mu: Py, mut v_sigma: Py, mut v_capacity: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_y: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__finite(Py::tuple(vec![v_p1.clone(), v_p2.clone(), v_mu.clone(), v_sigma.clone()]))?;
    if (py::not(&{ let t1463 = Py::Int(0i64); Py::Bool({ let t1464 = v_p2.clone(); if py::lt(&t1463, &t1464)? { { let t1465 = v_p1.clone(); if py::lt(&t1464, &t1465)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the discount fare must be positive and below the full fare"))));
    }
    if ({ let t1466 = v_sigma.clone(); Py::Bool({ let t1467 = Py::Int(0i64); if py::le(&t1466, &t1467)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the standard deviation of full-fare demand must be > 0"))));
    }
    v_z = py::normal_dist("inv_cdf", &Py::Float(0.0), &Py::Float(1.0), &[py::sub(&Py::Int(1i64), &py::truediv(&v_p2.clone(), &v_p1.clone())?)?])?;
    v_y = py::add(&v_mu.clone(), &py::mul(&v_z.clone(), &v_sigma.clone())?)?;
    v_out = Py::dict(vec![(Py::str("protection_level"), v_y.clone()), (Py::str("z"), v_z.clone())]);
    if ({ let t1468 = v_capacity.clone(); Py::Bool({ let t1469 = Py::None; if !py::is(&t1468, &t1469) { true } else { false } }) }).truthy() {
        let _ = f__finite(Py::tuple(vec![v_capacity.clone()]))?;
        if ({ let t1470 = v_capacity.clone(); Py::Bool({ let t1471 = Py::Int(0i64); if py::le(&t1470, &t1471)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("capacity must be > 0"))));
        }
        py::setitem(&v_out.clone(), &Py::str("booking_limit"), py::minmax(&[Py::Float(0.0), py::sub(&v_capacity.clone(), &py::minmax(&[v_y.clone(), Py::Float(0.0)], true)?)?], true)?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lp_shadow_prices(mut v_sense: Py, mut v_c: Py, mut v_A: Py, mut v_b: Py, mut v_relations: Py) -> R {
    let mut v_r: Py = Py::None;
    v_r = f__lp(v_sense.clone(), v_c.clone(), v_A.clone(), v_b.clone(), f__relations(v_relations.clone(), py::len(&v_A.clone())?)?, Py::Bool(true))?;
    if (py::not(&py::getitem(&v_r.clone(), &Py::str("p_unique"))?)).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the shadow prices are not unique at this optimum (a degenerate optimal solution); only one-sided rates of change exist"))));
    }
    return Ok(f__lp_outputs(v_r.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lp_two_variable(mut v_sense: Py, mut v_c: Py, mut v_A: Py, mut v_b: Py, mut v_relations: Py) -> R {
    let mut v_best: Py = Py::None;
    let mut v_c0: Py = Py::None;
    let mut v_c1: Py = Py::None;
    let mut v_flat_ray: Py = Py::None;
    let mut v_opt: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_rays: Py = Py::None;
    let mut v_rel: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_vals: Py = Py::None;
    v_rel = f__relations(v_relations.clone(), py::len(&v_A.clone())?)?;
    if ({ let t1685 = py::len(&v_c.clone())?; Py::Bool({ let t1686 = Py::Int(2i64); if py::ne(&t1685, &t1686) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the graphical method takes exactly 2 variables"))));
    }
    let _ = f__check_lp(v_sense.clone(), v_c.clone(), v_A.clone(), v_b.clone(), v_rel.clone())?;
    let t1781 = (f__lp2_vertices(v_c.clone(), v_A.clone(), v_b.clone(), v_rel.clone())?).items()?;
    if t1781.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pts = t1781[0].clone();
    v_rays = t1781[1].clone();
    if (py::not(&v_pts.clone())).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the constraints are infeasible: no point satisfies all of them"))));
    }
    v_s = (if ({ let t1782 = v_sense.clone(); Py::Bool({ let t1783 = Py::str("maximize"); if py::eq(&t1782, &t1783) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { py::neg(&Py::Int(1i64))? });
    let t1784 = (Py::tuple(vec![f__fr(py::getitem(&v_c.clone(), &Py::Int(0i64))?)?, f__fr(py::getitem(&v_c.clone(), &Py::Int(1i64))?)?])).items()?;
    if t1784.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c0 = t1784[0].clone();
    v_c1 = t1784[1].clone();
    if (Py::Bool(({ let mut out = vec![]; for t1785 in py::iter(&(v_rays.clone()))? { let mut v_d: Py; 
v_d = t1785;
out.push({ let t1786 = py::mul(&v_s.clone(), &py::add(&py::mul(&v_c0.clone(), &py::getitem(&v_d.clone(), &Py::Int(0i64))?)?, &py::mul(&v_c1.clone(), &py::getitem(&v_d.clone(), &Py::Int(1i64))?)?)?)?; Py::Bool({ let t1787 = Py::Int(0i64); if py::gt(&t1786, &t1787)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the objective is unbounded on the feasible region: there is no finite optimum"))));
    }
    v_vals = { let mut out = vec![]; for t1788 in py::iter(&(v_pts.clone()))? { let mut v_x: Py; let mut v_y: Py; 
let t1789 = (t1788).items()?;
if t1789.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1789[0].clone();
v_y = t1789[1].clone();
out.push(Py::tuple(vec![py::add(&py::mul(&v_c0.clone(), &v_x.clone())?, &py::mul(&v_c1.clone(), &v_y.clone())?)?, Py::tuple(vec![v_x.clone(), v_y.clone()])])); } Py::list(out) };
    v_best = (if ({ let t1790 = v_s.clone(); Py::Bool({ let t1791 = Py::Int(1i64); if py::eq(&t1790, &t1791) { true } else { false } }) }).truthy() { py::minmax(&[{ let mut out = vec![]; for t1792 in py::iter(&(v_vals.clone()))? { let mut v__: Py; let mut v_v: Py; 
let t1793 = (t1792).items()?;
if t1793.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t1793[0].clone();
v__ = t1793[1].clone();
out.push(v_v.clone()); } Py::list(out) }], true)? } else { py::minmax(&[{ let mut out = vec![]; for t1794 in py::iter(&(v_vals.clone()))? { let mut v__: Py; let mut v_v: Py; 
let t1795 = (t1794).items()?;
if t1795.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t1795[0].clone();
v__ = t1795[1].clone();
out.push(v_v.clone()); } Py::list(out) }], false)? });
    v_opt = { let mut out = vec![]; for t1796 in py::iter(&(v_vals.clone()))? { let mut v_p: Py; let mut v_v: Py; 
let t1797 = (t1796).items()?;
if t1797.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t1797[0].clone();
v_p = t1797[1].clone();
if ({ let t1798 = v_v.clone(); Py::Bool({ let t1799 = v_best.clone(); if py::eq(&t1798, &t1799) { true } else { false } }) }).truthy() { out.push(v_p.clone()); }} Py::list(out) };
    v_flat_ray = Py::Bool(({ let mut out = vec![]; for t1800 in py::iter(&(v_rays.clone()))? { let mut v_d: Py; 
v_d = t1800;
out.push({ let t1801 = py::add(&py::mul(&v_c0.clone(), &py::getitem(&v_d.clone(), &Py::Int(0i64))?)?, &py::mul(&v_c1.clone(), &py::getitem(&v_d.clone(), &Py::Int(1i64))?)?)?; Py::Bool({ let t1802 = Py::Int(0i64); if py::eq(&t1801, &t1802) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()));
    v_out = Py::dict(vec![(Py::str("optimal_value"), py::float(&v_best.clone())?), (Py::str("optimal_corners"), { let mut out = vec![]; for t1803 in py::iter(&(v_opt.clone()))? { let mut v_x: Py; let mut v_y: Py; 
let t1804 = (t1803).items()?;
if t1804.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1804[0].clone();
v_y = t1804[1].clone();
out.push(Py::list(vec![py::float(&v_x.clone())?, py::float(&v_y.clone())?])); } Py::list(out) }), (Py::str("corner_points"), { let mut out = vec![]; for t1805 in py::iter(&(v_pts.clone()))? { let mut v_x: Py; let mut v_y: Py; 
let t1806 = (t1805).items()?;
if t1806.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1806[0].clone();
v_y = t1806[1].clone();
out.push(Py::list(vec![py::float(&v_x.clone())?, py::float(&v_y.clone())?])); } Py::list(out) })]);
    if ({ let t1809 = { let t1807 = py::len(&v_opt.clone())?; Py::Bool({ let t1808 = Py::Int(1i64); if py::eq(&t1807, &t1808) { true } else { false } }) }; if !t1809.truthy() { t1809 } else { py::not(&v_flat_ray.clone()) } }).truthy() {
        let t1810 = (Py::tuple(vec![py::float(&py::getitem(&py::getitem(&v_opt.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?)?, py::float(&py::getitem(&py::getitem(&v_opt.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?)?, Py::str("yes")])).items()?;
        if t1810.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_out.clone(), &Py::str("x"), t1810[0].clone())?;
        py::setitem(&v_out.clone(), &Py::str("y"), t1810[1].clone())?;
        py::setitem(&v_out.clone(), &Py::str("solution_unique"), t1810[2].clone())?;
    } else {
        py::setitem(&v_out.clone(), &Py::str("solution_unique"), Py::str("no"))?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_machine_repair(mut v_N: Py, mut v_lam: Py, mut v_mu: Py, mut v_c: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_Lq: Py = Py::None;
    let mut v_base: Py = Py::None;
    let mut v_lam_eff: Py = Py::None;
    let mut v_logs: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pi: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__finite(Py::tuple(vec![v_lam.clone(), v_mu.clone()]))?;
    if ({ let t1829 = { let t1825 = v_lam.clone(); Py::Bool({ let t1826 = Py::Int(0i64); if py::le(&t1825, &t1826)? { true } else { false } }) }; if t1829.truthy() { t1829 } else { { let t1827 = v_mu.clone(); Py::Bool({ let t1828 = Py::Int(0i64); if py::le(&t1827, &t1828)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the failure and repair rates must be > 0"))));
    }
    if (py::not(&{ let t1830 = Py::Int(1i64); Py::Bool({ let t1831 = v_c.clone(); if py::le(&t1830, &t1831)? { { let t1832 = v_N.clone(); if py::le(&t1831, &t1832)? { { let t1833 = Py::Int(1000i64); if py::le(&t1832, &t1833)? { true } else { false } } } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("need 1 <= repairers <= machines <= 1000"))));
    }
    v_r = py::truediv(&v_lam.clone(), &v_mu.clone())?;
    v_logs = Py::list(vec![]);
    for t1834 in py::range_iter(&[(py::add(&v_N.clone(), &Py::Int(1i64))?).i()?]) {
        v_n = t1834;
        v_base = py::add(&py::sub(&py::math1("lgamma", &py::add(&v_N.clone(), &Py::Int(1i64))?)?, &py::math1("lgamma", &py::add(&py::sub(&v_N.clone(), &v_n.clone())?, &Py::Int(1i64))?)?)?, &py::mul(&v_n.clone(), &py::math1("log", &v_r.clone())?)?)?;
        let _ = py::method(&v_logs.clone(), "append", &[py::sub(&v_base.clone(), &(if ({ let t1835 = v_n.clone(); Py::Bool({ let t1836 = v_c.clone(); if py::le(&t1835, &t1836)? { true } else { false } }) }).truthy() { py::math1("lgamma", &py::add(&v_n.clone(), &Py::Int(1i64))?)? } else { py::add(&py::math1("lgamma", &py::add(&v_c.clone(), &Py::Int(1i64))?)?, &py::mul(&py::sub(&v_n.clone(), &v_c.clone())?, &py::math1("log", &v_c.clone())?)?)? }))?])?;
    }
    v_mx = py::minmax(&[v_logs.clone()], true)?;
    v_w = { let mut out = vec![]; for t1837 in py::iter(&(v_logs.clone()))? { let mut v_v: Py; 
v_v = t1837;
out.push(py::math1("exp", &py::sub(&v_v.clone(), &v_mx.clone())?)?); } Py::list(out) };
    v_tot = py::fsum(&v_w.clone())?;
    v_pi = { let mut out = vec![]; for t1838 in py::iter(&(v_w.clone()))? { let mut v_v: Py; 
v_v = t1838;
out.push(py::truediv(&v_v.clone(), &v_tot.clone())?); } Py::list(out) };
    v_L = py::fsum(&{ let mut out = vec![]; for t1839 in py::iter(&(py::enumerate(&v_pi.clone(), 0)?))? { let mut v_n: Py; let mut v_p: Py; 
let t1840 = (t1839).items()?;
if t1840.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_n = t1840[0].clone();
v_p = t1840[1].clone();
out.push(py::mul(&v_n.clone(), &v_p.clone())?); } Py::list(out) })?;
    v_Lq = py::fsum(&{ let mut out = vec![]; for t1841 in py::iter(&(py::enumerate(&v_pi.clone(), 0)?))? { let mut v_n: Py; let mut v_p: Py; 
let t1842 = (t1841).items()?;
if t1842.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_n = t1842[0].clone();
v_p = t1842[1].clone();
out.push(py::mul(&py::minmax(&[py::sub(&v_n.clone(), &v_c.clone())?, Py::Int(0i64)], true)?, &v_p.clone())?); } Py::list(out) })?;
    v_lam_eff = py::mul(&v_lam.clone(), &py::sub(&v_N.clone(), &v_L.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("L"), v_L.clone()), (Py::str("Lq"), v_Lq.clone()), (Py::str("lam_eff"), v_lam_eff.clone()), (Py::str("W"), py::truediv(&v_L.clone(), &v_lam_eff.clone())?), (Py::str("Wq"), py::truediv(&v_Lq.clone(), &v_lam_eff.clone())?), (Py::str("P0"), py::getitem(&v_pi.clone(), &Py::Int(0i64))?), (Py::str("working"), py::sub(&v_N.clone(), &v_L.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_max_flow(mut v_edges: Py, mut v_source: Py, mut v_target: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_aug: Py = Py::None;
    let mut v_cap: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_flow: Py = Py::None;
    let mut v_left: Py = Py::None;
    let mut v_nbr: Py = Py::None;
    let mut v_nodes: Py = Py::None;
    let mut v_path: Py = Py::None;
    let mut v_per: Py = Py::None;
    let mut v_prev: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_side: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_value: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    v_E = f__wedges(v_edges.clone(), Py::Bool(true))?;
    if (Py::Bool(({ let mut out = vec![]; for t1850 in py::iter(&(v_E.clone()))? { let mut v__: Py; let mut v_w: Py; 
let t1851 = (t1850).items()?;
if t1851.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v__ = t1851[0].clone();
v__ = t1851[1].clone();
v_w = t1851[2].clone();
out.push({ let t1852 = v_w.clone(); Py::Bool({ let t1853 = Py::Int(0i64); if py::lt(&t1852, &t1853)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("capacities must be >= 0"))));
    }
    let t1854 = (Py::tuple(vec![py::method(&Py::str(&py::to_str(&v_source.clone())), "strip", &[])?, py::method(&Py::str(&py::to_str(&v_target.clone())), "strip", &[])?])).items()?;
    if t1854.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t1854[0].clone();
    v_t = t1854[1].clone();
    v_nodes = f__nodes(v_E.clone())?;
    if ({ let t1866 = { let t1859 = v_s.clone(); Py::Bool({ let t1860 = v_nodes.clone(); if !py::contains(&t1860, &t1859)? { true } else { false } }) }; if t1866.truthy() { t1866 } else { { let t1865 = { let t1861 = v_t.clone(); Py::Bool({ let t1862 = v_nodes.clone(); if !py::contains(&t1862, &t1861)? { true } else { false } }) }; if t1865.truthy() { t1865 } else { { let t1863 = v_s.clone(); Py::Bool({ let t1864 = v_t.clone(); if py::eq(&t1863, &t1864) { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the source and the sink must be two different nodes of the network"))));
    }
    v_cap = Py::dict(vec![]);
    for t1867 in py::iter(&(v_E.clone()))? {
        let t1868 = (t1867).items()?;
        if t1868.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_u = t1868[0].clone();
        v_v = t1868[1].clone();
        v_w = t1868[2].clone();
        py::setitem(&v_cap.clone(), &Py::tuple(vec![v_u.clone(), v_v.clone()]), py::add(&py::method(&v_cap.clone(), "get", &[Py::tuple(vec![v_u.clone(), v_v.clone()]), py::fraction(&[Py::Int(0i64)])?])?, &v_w.clone())?)?;
        let _ = py::method(&v_cap.clone(), "setdefault", &[Py::tuple(vec![v_v.clone(), v_u.clone()]), py::fraction(&[Py::Int(0i64)])?])?;
    }
    v_flow = { let mut out = vec![]; for t1869 in py::iter(&(v_cap.clone()))? { let mut v_k: Py; 
v_k = t1869;
out.push((v_k.clone(), py::fraction(&[Py::Int(0i64)])?)); } Py::dict(out) };
    v_nbr = Py::dict(vec![]);
    for t1870 in py::iter(&(v_cap.clone()))? {
        let t1871 = (t1870).items()?;
        if t1871.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_u = t1871[0].clone();
        v_v = t1871[1].clone();
        let _ = py::method(&py::method(&v_nbr.clone(), "setdefault", &[v_u.clone(), Py::list(vec![])])?, "append", &[v_v.clone()])?;
    }
    v_value = py::fraction(&[Py::Int(0i64)])?;
    while (Py::Bool(true)).truthy() {
        let t1872 = (Py::tuple(vec![Py::dict(vec![(v_s.clone(), Py::None)]), Py::list(vec![v_s.clone()])])).items()?;
        if t1872.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_prev = t1872[0].clone();
        v_q = t1872[1].clone();
        for t1873 in py::iter(&(v_q.clone()))? {
            v_x = t1873;
            for t1874 in py::iter(&(py::method(&v_nbr.clone(), "get", &[v_x.clone(), Py::list(vec![])])?))? {
                v_y = t1874;
                if ({ let t1879 = { let t1875 = v_y.clone(); Py::Bool({ let t1876 = v_prev.clone(); if !py::contains(&t1876, &t1875)? { true } else { false } }) }; if !t1879.truthy() { t1879 } else { { let t1877 = py::sub(&py::getitem(&v_cap.clone(), &Py::tuple(vec![v_x.clone(), v_y.clone()]))?, &py::getitem(&v_flow.clone(), &Py::tuple(vec![v_x.clone(), v_y.clone()]))?)?; Py::Bool({ let t1878 = Py::Int(0i64); if py::gt(&t1877, &t1878)? { true } else { false } }) } } }).truthy() {
                    py::setitem(&v_prev.clone(), &v_y.clone(), v_x.clone())?;
                    let _ = py::method(&v_q.clone(), "append", &[v_y.clone()])?;
                }
            }
        }
        if ({ let t1880 = v_t.clone(); Py::Bool({ let t1881 = v_prev.clone(); if !py::contains(&t1881, &t1880)? { true } else { false } }) }).truthy() {
            break;
        }
        let t1882 = (Py::tuple(vec![Py::list(vec![]), v_t.clone()])).items()?;
        if t1882.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_path = t1882[0].clone();
        v_y = t1882[1].clone();
        while ({ let t1883 = py::getitem(&v_prev.clone(), &v_y.clone())?; Py::Bool({ let t1884 = Py::None; if !py::is(&t1883, &t1884) { true } else { false } }) }).truthy() {
            let _ = py::method(&v_path.clone(), "append", &[Py::tuple(vec![py::getitem(&v_prev.clone(), &v_y.clone())?, v_y.clone()])])?;
            v_y = py::getitem(&v_prev.clone(), &v_y.clone())?;
        }
        v_aug = py::minmax(&[{ let mut out = vec![]; for t1885 in py::iter(&(v_path.clone()))? { let mut v_e: Py; 
v_e = t1885;
out.push(py::sub(&py::getitem(&v_cap.clone(), &v_e.clone())?, &py::getitem(&v_flow.clone(), &v_e.clone())?)?); } Py::list(out) }], false)?;
        for t1886 in py::iter(&(v_path.clone()))? {
            let t1891 = (t1886).items()?;
            if t1891.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_x = t1891[0].clone();
            v_y = t1891[1].clone();
            let t1887 = v_flow.clone(); let t1888 = Py::tuple(vec![v_x.clone(), v_y.clone()]);
            py::setitem(&t1887, &t1888, py::iadd(&py::getitem(&t1887, &t1888)?, &v_aug.clone())?)?;
            let t1889 = v_flow.clone(); let t1890 = Py::tuple(vec![v_y.clone(), v_x.clone()]);
            py::setitem(&t1889, &t1890, py::sub(&py::getitem(&t1889, &t1890)?, &v_aug.clone())?)?;
        }
        v_value = py::iadd(&v_value, &v_aug.clone())?;
    }
    v_side = { let mut out = vec![]; for t1892 in py::iter(&(v_nodes.clone()))? { let mut v_v: Py; 
v_v = t1892;
if ({ let t1893 = v_v.clone(); Py::Bool({ let t1894 = v_prev.clone(); if py::contains(&t1894, &t1893)? { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) };
    let t1896 = (Py::tuple(vec![{ let mut out = vec![]; for t1895 in py::iter(&(v_cap.clone()))? { let mut v_k: Py; 
v_k = t1895;
out.push((v_k.clone(), py::minmax(&[py::getitem(&v_flow.clone(), &v_k.clone())?, py::fraction(&[Py::Int(0i64)])?], true)?)); } Py::dict(out) }, Py::list(vec![])])).items()?;
    if t1896.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_left = t1896[0].clone();
    v_per = t1896[1].clone();
    for t1897 in py::iter(&(v_E.clone()))? {
        let t1900 = (t1897).items()?;
        if t1900.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_u = t1900[0].clone();
        v_v = t1900[1].clone();
        v_w = t1900[2].clone();
        v_f = py::minmax(&[v_w.clone(), py::getitem(&v_left.clone(), &Py::tuple(vec![v_u.clone(), v_v.clone()]))?], false)?;
        let t1898 = v_left.clone(); let t1899 = Py::tuple(vec![v_u.clone(), v_v.clone()]);
        py::setitem(&t1898, &t1899, py::sub(&py::getitem(&t1898, &t1899)?, &v_f.clone())?)?;
        let _ = py::method(&v_per.clone(), "append", &[py::float(&v_f.clone())?])?;
    }
    return Ok(Py::dict(vec![(Py::str("max_flow"), py::float(&v_value.clone())?), (Py::str("min_cut_source_side"), v_side.clone()), (Py::str("edge_flows"), v_per.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_max_weight_matching(mut v_edges: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_W: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_ix: Py = Py::None;
    let mut v_memo: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_nodes: Py = Py::None;
    let mut v_pairs: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_val: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_E = f__wedges(v_edges.clone(), Py::Bool(true))?;
    v_nodes = f__nodes(v_E.clone())?;
    if ({ let t1945 = py::len(&v_nodes.clone())?; Py::Bool({ let t1946 = Py::Int(16i64); if py::gt(&t1945, &t1946)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at most 16 nodes"))));
    }
    v_ix = { let mut out = vec![]; for t1947 in py::iter(&(py::enumerate(&v_nodes.clone(), 0)?))? { let mut v_k: Py; let mut v_v: Py; 
let t1948 = (t1947).items()?;
if t1948.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1948[0].clone();
v_v = t1948[1].clone();
out.push((v_v.clone(), v_k.clone())); } Py::dict(out) };
    v_W = Py::dict(vec![]);
    for t1949 in py::iter(&(v_E.clone()))? {
        let t1953 = (t1949).items()?;
        if t1953.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_u = t1953[0].clone();
        v_v = t1953[1].clone();
        v_w = t1953[2].clone();
        if ({ let t1950 = v_u.clone(); Py::Bool({ let t1951 = v_v.clone(); if py::ne(&t1950, &t1951) { true } else { false } }) }).truthy() {
            let t1952 = (py::sorted(&Py::tuple(vec![py::getitem(&v_ix.clone(), &v_u.clone())?, py::getitem(&v_ix.clone(), &v_v.clone())?]), (Py::Bool(false)).truthy())?).items()?;
            if t1952.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t1952[0].clone();
            v_b = t1952[1].clone();
            py::setitem(&v_W.clone(), &Py::tuple(vec![v_a.clone(), v_b.clone()]), py::minmax(&[py::method(&v_W.clone(), "get", &[Py::tuple(vec![v_a.clone(), v_b.clone()]), v_w.clone()])?, v_w.clone()], true)?)?;
        }
    }
    v_n = py::len(&v_nodes.clone())?;
    v_memo = Py::dict(vec![]);
    let rec1954 = py::rec_cell();
    v_f = { let v_E = v_E.clone(); let v_W = v_W.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_edges = v_edges.clone(); let v_ix = v_ix.clone(); let v_memo = v_memo.clone(); let v_n = v_n.clone(); let v_nodes = v_nodes.clone(); let v_u = v_u.clone(); let v_v = v_v.clone(); let v_w = v_w.clone(); let rec1954 = rec1954.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_mask: Py = a.get(0).cloned().unwrap_or(Py::None);
            let v_f: Py = py::rec_get(&rec1954)?;
            let mut v_best: Py = Py::None;
            let mut v_i: Py = Py::None;
            let mut v_j: Py = Py::None;
            let mut v_pairs: Py = Py::None;
            let mut v_val: Py = Py::None;
            if ({ let t1955 = v_mask.clone(); Py::Bool({ let t1956 = py::sub(&py::lshift(&Py::Int(1i64), &v_n.clone())?, &Py::Int(1i64))?; if py::eq(&t1955, &t1956) { true } else { false } }) }).truthy() {
                return Ok(Py::tuple(vec![py::fraction(&[Py::Int(0i64)])?, Py::tuple(vec![])]));
            }
            if ({ let t1957 = v_mask.clone(); Py::Bool({ let t1958 = v_memo.clone(); if py::contains(&t1958, &t1957)? { true } else { false } }) }).truthy() {
                return Ok(py::getitem(&v_memo.clone(), &v_mask.clone())?);
            }
            v_i = (match ('g1959: { let mut out = vec![]; for t1960 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t1960;
if (py::not(&py::bitand(&py::rshift(&v_mask.clone(), &v_k.clone())?, &Py::Int(1i64))?)).truthy() { out.push(v_k.clone()); break 'g1959 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) });
            v_best = py::call(&v_f.clone(), &[py::bitor(&v_mask.clone(), &py::lshift(&Py::Int(1i64), &v_i.clone())?)?])?;
            for t1961 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
                v_j = t1961;
                if ({ let t1967 = py::not(&py::bitand(&py::rshift(&v_mask.clone(), &v_j.clone())?, &Py::Int(1i64))?); if !t1967.truthy() { t1967 } else { { let t1966 = { let t1962 = Py::tuple(vec![v_i.clone(), v_j.clone()]); Py::Bool({ let t1963 = v_W.clone(); if py::contains(&t1963, &t1962)? { true } else { false } }) }; if !t1966.truthy() { t1966 } else { { let t1964 = py::getitem(&v_W.clone(), &Py::tuple(vec![v_i.clone(), v_j.clone()]))?; Py::Bool({ let t1965 = Py::Int(0i64); if py::gt(&t1964, &t1965)? { true } else { false } }) } } } } }).truthy() {
                    let t1968 = (py::call(&v_f.clone(), &[py::bitor(&py::bitor(&v_mask.clone(), &py::lshift(&Py::Int(1i64), &v_i.clone())?)?, &py::lshift(&Py::Int(1i64), &v_j.clone())?)?])?).items()?;
                    if t1968.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    v_val = t1968[0].clone();
                    v_pairs = t1968[1].clone();
                    if ({ let t1969 = py::add(&v_val.clone(), &py::getitem(&v_W.clone(), &Py::tuple(vec![v_i.clone(), v_j.clone()]))?)?; Py::Bool({ let t1970 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; if py::gt(&t1969, &t1970)? { true } else { false } }) }).truthy() {
                        v_best = Py::tuple(vec![py::add(&v_val.clone(), &py::getitem(&v_W.clone(), &Py::tuple(vec![v_i.clone(), v_j.clone()]))?)?, py::add(&Py::tuple(vec![Py::tuple(vec![v_i.clone(), v_j.clone()])]), &v_pairs.clone())?]);
                    }
                }
            }
            py::setitem(&v_memo.clone(), &v_mask.clone(), v_best.clone())?;
            return Ok(v_best.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec1954, &v_f);
    let t1971 = (py::call(&v_f.clone(), &[Py::Int(0i64)])?).items()?;
    if t1971.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_val = t1971[0].clone();
    v_pairs = t1971[1].clone();
    return Ok(Py::dict(vec![(Py::str("max_weight"), py::float(&v_val.clone())?), (Py::str("matching_example"), { let mut out = vec![]; for t1972 in py::iter(&(v_pairs.clone()))? { let mut v_a: Py; let mut v_b: Py; 
let t1973 = (t1972).items()?;
if t1973.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1973[0].clone();
v_b = t1973[1].clone();
out.push(Py::str(&[py::format(&py::getitem(&v_nodes.clone(), &v_a.clone())?, "")?, "-".to_string(), py::format(&py::getitem(&v_nodes.clone(), &v_b.clone())?, "")?].concat())); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mdp_discounted(mut v_R: Py, mut v_T: Py, mut v_gamma: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_Qv: Py = Py::None;
    let mut v_Rf: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_new: Py = Py::None;
    let mut v_pol: Py = Py::None;
    let t2026 = (f__mdp_in(v_R.clone(), v_T.clone(), v_gamma.clone())?).items()?;
    if t2026.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_Rf = t2026[0].clone();
    v_P = t2026[1].clone();
    v_g = t2026[2].clone();
    v_S = t2026[3].clone();
    v_A = t2026[4].clone();
    v_pol = py::mul(&Py::list(vec![Py::Int(0i64)]), &v_S.clone())?;
    for t2027 in py::range_iter(&[(Py::Int(1000i64)).i()?]) {
        v__ = t2027;
        v_V = f__solve_exact({ let mut out = vec![]; for t2047 in py::range_iter(&[(v_S.clone()).i()?]) { let mut v_s: Py; 
v_s = t2047;
out.push({ let mut out = vec![]; for t2048 in py::range_iter(&[(v_S.clone()).i()?]) { let mut v_t: Py; 
v_t = t2048;
out.push(py::sub(&(if ({ let t2049 = v_s.clone(); Py::Bool({ let t2050 = v_t.clone(); if py::eq(&t2049, &t2050) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { Py::Int(0i64) }), &py::mul(&v_g.clone(), &py::getitem(&py::getitem(&v_P.clone(), &py::add(&py::mul(&v_s.clone(), &v_A.clone())?, &py::getitem(&v_pol.clone(), &v_s.clone())?)?)?, &v_t.clone())?)?)?); } Py::list(out) }); } Py::list(out) }, { let mut out = vec![]; for t2051 in py::range_iter(&[(v_S.clone()).i()?]) { let mut v_s: Py; 
v_s = t2051;
out.push(py::getitem(&py::getitem(&v_Rf.clone(), &v_s.clone())?, &py::getitem(&v_pol.clone(), &v_s.clone())?)?); } Py::list(out) })?;
        v_Qv = { let mut out = vec![]; for t2052 in py::range_iter(&[(v_S.clone()).i()?]) { let mut v_s: Py; 
v_s = t2052;
out.push({ let mut out = vec![]; for t2053 in py::range_iter(&[(v_A.clone()).i()?]) { let mut v_a: Py; 
v_a = t2053;
out.push(py::add(&py::getitem(&py::getitem(&v_Rf.clone(), &v_s.clone())?, &v_a.clone())?, &py::mul(&v_g.clone(), &py::sum(&{ let mut out = vec![]; for t2054 in py::range_iter(&[(v_S.clone()).i()?]) { let mut v_t: Py; 
v_t = t2054;
out.push(py::mul(&py::getitem(&py::getitem(&v_P.clone(), &py::add(&py::mul(&v_s.clone(), &v_A.clone())?, &v_a.clone())?)?, &v_t.clone())?, &py::getitem(&v_V.clone(), &v_t.clone())?)?); } Py::list(out) }, None)?)?)?); } Py::list(out) }); } Py::list(out) };
        v_new = { let mut out = vec![]; for t2055 in py::range_iter(&[(v_S.clone()).i()?]) { let mut v_s: Py; 
v_s = t2055;
out.push((if ({ let t2056 = py::getitem(&py::getitem(&v_Qv.clone(), &v_s.clone())?, &py::getitem(&v_pol.clone(), &v_s.clone())?)?; Py::Bool({ let t2057 = py::minmax(&[py::getitem(&v_Qv.clone(), &v_s.clone())?], true)?; if py::eq(&t2056, &t2057) { true } else { false } }) }).truthy() { py::getitem(&v_pol.clone(), &v_s.clone())? } else { py::method(&py::getitem(&v_Qv.clone(), &v_s.clone())?, "index", &[py::minmax(&[py::getitem(&v_Qv.clone(), &v_s.clone())?], true)?])? })); } Py::list(out) };
        if ({ let t2058 = v_new.clone(); Py::Bool({ let t2059 = v_pol.clone(); if py::eq(&t2058, &t2059) { true } else { false } }) }).truthy() {
            break;
        }
        v_pol = v_new.clone();
    }
    v_best = { let mut out = vec![]; for t2060 in py::range_iter(&[(v_S.clone()).i()?]) { let mut v_s: Py; 
v_s = t2060;
out.push(py::add(&py::method(&py::getitem(&v_Qv.clone(), &v_s.clone())?, "index", &[py::minmax(&[py::getitem(&v_Qv.clone(), &v_s.clone())?], true)?])?, &Py::Int(1i64))?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("value_state_1"), py::float(&py::getitem(&v_V.clone(), &Py::Int(0i64))?)?), (Py::str("values"), f__fl(v_V.clone())?), (Py::str("policy"), { let mut out = vec![]; for t2062 in py::iter(&(v_best.clone()))? { let mut v_a: Py; 
v_a = t2062;
out.push(py::float(&v_a.clone())?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mg1(mut v_lam: Py, mut v_ES: Py, mut v_sd: Py) -> R {
    let mut v_Lq: Py = Py::None;
    let mut v_Wq: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let _ = f__finite(Py::tuple(vec![v_lam.clone(), v_ES.clone(), v_sd.clone()]))?;
    if ({ let t2083 = { let t2076 = v_lam.clone(); Py::Bool({ let t2077 = Py::Int(0i64); if py::le(&t2076, &t2077)? { true } else { false } }) }; if t2083.truthy() { t2083 } else { { let t2082 = { let t2078 = v_ES.clone(); Py::Bool({ let t2079 = Py::Int(0i64); if py::le(&t2078, &t2079)? { true } else { false } }) }; if t2082.truthy() { t2082 } else { { let t2080 = v_sd.clone(); Py::Bool({ let t2081 = Py::Int(0i64); if py::lt(&t2080, &t2081)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the arrival rate and mean service time must be > 0 and the standard deviation >= 0"))));
    }
    v_rho = py::mul(&v_lam.clone(), &v_ES.clone())?;
    if ({ let t2084 = v_rho.clone(); Py::Bool({ let t2085 = Py::Int(1i64); if py::ge(&t2084, &t2085)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the queue is unstable: lambda times the mean service time must be below 1"))));
    }
    v_Lq = py::truediv(&py::add(&py::mul(&py::mul(&py::mul(&v_lam.clone(), &v_lam.clone())?, &v_sd.clone())?, &v_sd.clone())?, &py::mul(&v_rho.clone(), &v_rho.clone())?)?, &py::mul(&Py::Int(2i64), &py::sub(&Py::Int(1i64), &v_rho.clone())?)?)?;
    v_Wq = py::truediv(&v_Lq.clone(), &v_lam.clone())?;
    return Ok(Py::dict(vec![(Py::str("rho"), v_rho.clone()), (Py::str("Lq"), v_Lq.clone()), (Py::str("L"), py::add(&v_Lq.clone(), &v_rho.clone())?), (Py::str("Wq"), v_Wq.clone()), (Py::str("W"), py::add(&v_Wq.clone(), &v_ES.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_min_cost_flow(mut v_capacity: Py, mut v_cost: Py, mut v_supply: Py) -> R {
    let mut v_arcs: Py = Py::None;
    let mut v_fl: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_lp: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t2125 = (f__mcf_lp(v_capacity.clone(), v_cost.clone(), v_supply.clone())?).items()?;
    if t2125.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_arcs = t2125[0].clone();
    v_lp = t2125[1].clone();
    v_r = { let mut v: Vec<Py> = vec![]; v.extend((v_lp.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "sense") { Some(x) => x, None => { return Err(PyErr::Type("_lp() missing argument sense".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "c") { Some(x) => x, None => { return Err(PyErr::Type("_lp() missing argument c".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "A") { Some(x) => x, None => { return Err(PyErr::Type("_lp() missing argument A".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "b") { Some(x) => x, None => { return Err(PyErr::Type("_lp() missing argument b".into())) } } }; let a4 = if 4 < v.len() { v[4].clone() } else { match py::kw_take(&mut d, "rel") { Some(x) => x, None => { return Err(PyErr::Type("_lp() missing argument rel".into())) } } }; let a5 = if 5 < v.len() { v[5].clone() } else { match py::kw_take(&mut d, "certify") { Some(x) => x, None => Py::Bool(true) } }; if v.len() > 6 { return Err(PyErr::Type("_lp() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("_lp() got an unexpected keyword argument".into())) } f__lp(a0, a1, a2, a3, a4, a5)? };
    v_n = py::len(&v_supply.clone())?;
    v_fl = { let mut out = vec![]; for t2280 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v__: Py; 
v__ = t2280;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?); } Py::list(out) };
    for t2281 in py::iter(&(py::zip(&[v_arcs.clone(), py::getitem(&v_r.clone(), &Py::str("x"))?])?))? {
        let t2282 = (t2281).items()?;
        if t2282.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        let t2283 = (t2282[0].clone()).items()?;
        if t2283.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t2283[0].clone();
        v_j = t2283[1].clone();
        v_x = t2282[1].clone();
        py::setitem(&py::getitem(&v_fl.clone(), &v_i.clone())?, &v_j.clone(), py::float(&v_x.clone())?)?;
    }
    return Ok(Py::dict(vec![(Py::str("min_cost"), py::float(&py::getitem(&v_r.clone(), &Py::str("value"))?)?), ((if (py::getitem(&v_r.clone(), &Py::str("x_unique"))?).truthy() { Py::str("flows") } else { Py::str("flows_example") }), v_fl.clone()), (Py::str("flow_unique"), (if (py::getitem(&v_r.clone(), &Py::str("x_unique"))?).truthy() { Py::str("yes") } else { Py::str("no") })), (Py::str("dual"), f__fl(py::getitem(&v_r.clone(), &Py::str("p"))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mixed_nash_2x2(mut v_A: Py, mut v_B: Py) -> R {
    let mut v_DA: Py = Py::None;
    let mut v_DB: Py = Py::None;
    let mut v_MA: Py = Py::None;
    let mut v_MB: Py = Py::None;
    let mut v_a11: Py = Py::None;
    let mut v_a12: Py = Py::None;
    let mut v_a21: Py = Py::None;
    let mut v_a22: Py = Py::None;
    let mut v_b11: Py = Py::None;
    let mut v_b12: Py = Py::None;
    let mut v_b21: Py = Py::None;
    let mut v_b22: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let t2466 = (f__bimatrix(v_A.clone(), v_B.clone(), Py::Int(8i64))?).items()?;
    if t2466.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_MA = t2466[0].clone();
    v_MB = t2466[1].clone();
    if ({ let t2471 = { let t2467 = py::len(&v_MA.clone())?; Py::Bool({ let t2468 = Py::Int(2i64); if py::ne(&t2467, &t2468) { true } else { false } }) }; if t2471.truthy() { t2471 } else { { let t2469 = py::len(&py::getitem(&v_MA.clone(), &Py::Int(0i64))?)?; Py::Bool({ let t2470 = Py::Int(2i64); if py::ne(&t2469, &t2470) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give 2 x 2 payoff matrices"))));
    }
    if (f__pure_ne(v_MA.clone(), v_MB.clone())?).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the game has a pure-strategy equilibrium: its mixed equilibrium is not the only one (ask for the pure equilibria)"))));
    }
    let t2482 = (v_MA.clone()).items()?;
    if t2482.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    let t2483 = (t2482[0].clone()).items()?;
    if t2483.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a11 = t2483[0].clone();
    v_a12 = t2483[1].clone();
    let t2484 = (t2482[1].clone()).items()?;
    if t2484.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a21 = t2484[0].clone();
    v_a22 = t2484[1].clone();
    let t2485 = (v_MB.clone()).items()?;
    if t2485.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    let t2486 = (t2485[0].clone()).items()?;
    if t2486.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_b11 = t2486[0].clone();
    v_b12 = t2486[1].clone();
    let t2487 = (t2485[1].clone()).items()?;
    if t2487.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_b21 = t2487[0].clone();
    v_b22 = t2487[1].clone();
    let t2488 = (Py::tuple(vec![py::add(&py::sub(&py::sub(&v_a11.clone(), &v_a12.clone())?, &v_a21.clone())?, &v_a22.clone())?, py::add(&py::sub(&py::sub(&v_b11.clone(), &v_b12.clone())?, &v_b21.clone())?, &v_b22.clone())?])).items()?;
    if t2488.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_DA = t2488[0].clone();
    v_DB = t2488[1].clone();
    if ({ let t2493 = { let t2489 = v_DA.clone(); Py::Bool({ let t2490 = Py::Int(0i64); if py::eq(&t2489, &t2490) { true } else { false } }) }; if t2493.truthy() { t2493 } else { { let t2491 = v_DB.clone(); Py::Bool({ let t2492 = Py::Int(0i64); if py::eq(&t2491, &t2492) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("degenerate game: no unique mixed equilibrium"))));
    }
    let t2494 = (Py::tuple(vec![py::truediv(&py::sub(&v_a22.clone(), &v_a12.clone())?, &v_DA.clone())?, py::truediv(&py::sub(&v_b22.clone(), &v_b21.clone())?, &v_DB.clone())?])).items()?;
    if t2494.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_q = t2494[0].clone();
    v_p = t2494[1].clone();
    if (py::not(&{ let t2501 = { let t2495 = Py::Int(0i64); Py::Bool({ let t2496 = v_p.clone(); if py::lt(&t2495, &t2496)? { { let t2497 = Py::Int(1i64); if py::lt(&t2496, &t2497)? { true } else { false } } } else { false } }) }; if !t2501.truthy() { t2501 } else { { let t2498 = Py::Int(0i64); Py::Bool({ let t2499 = v_q.clone(); if py::lt(&t2498, &t2499)? { { let t2500 = Py::Int(1i64); if py::lt(&t2499, &t2500)? { true } else { false } } } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("no completely mixed equilibrium"))));
    }
    return Ok(Py::dict(vec![(Py::str("p_row1"), py::float(&v_p.clone())?), (Py::str("q_col1"), py::float(&v_q.clone())?), (Py::str("row_payoff"), py::float(&py::add(&py::mul(&v_q.clone(), &v_a11.clone())?, &py::mul(&py::sub(&Py::Int(1i64), &v_q.clone())?, &v_a12.clone())?)?)?), (Py::str("col_payoff"), py::float(&py::add(&py::mul(&v_p.clone(), &v_b11.clone())?, &py::mul(&py::sub(&Py::Int(1i64), &v_p.clone())?, &v_b21.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mm1(mut v_lam: Py, mut v_mu: Py) -> R {
    let mut v_rho: Py = Py::None;
    let _ = f__finite(Py::tuple(vec![v_lam.clone(), v_mu.clone()]))?;
    if ({ let t2528 = { let t2524 = v_lam.clone(); Py::Bool({ let t2525 = Py::Int(0i64); if py::le(&t2524, &t2525)? { true } else { false } }) }; if t2528.truthy() { t2528 } else { { let t2526 = v_mu.clone(); Py::Bool({ let t2527 = Py::Int(0i64); if py::le(&t2526, &t2527)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the arrival and service rates must be > 0"))));
    }
    if ({ let t2529 = v_lam.clone(); Py::Bool({ let t2530 = v_mu.clone(); if py::ge(&t2529, &t2530)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the queue is unstable: the arrival rate must be below the service rate"))));
    }
    v_rho = py::truediv(&v_lam.clone(), &v_mu.clone())?;
    return Ok(Py::dict(vec![(Py::str("rho"), v_rho.clone()), (Py::str("L"), py::truediv(&v_rho.clone(), &py::sub(&Py::Int(1i64), &v_rho.clone())?)?), (Py::str("Lq"), py::truediv(&py::mul(&v_rho.clone(), &v_rho.clone())?, &py::sub(&Py::Int(1i64), &v_rho.clone())?)?), (Py::str("W"), py::truediv(&Py::Int(1i64), &py::sub(&v_mu.clone(), &v_lam.clone())?)?), (Py::str("Wq"), py::truediv(&v_rho.clone(), &py::sub(&v_mu.clone(), &v_lam.clone())?)?), (Py::str("P0"), py::sub(&Py::Int(1i64), &v_rho.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mmc(mut v_lam: Py, mut v_mu: Py, mut v_c: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_Lq: Py = Py::None;
    let mut v_P0: Py = Py::None;
    let mut v_W: Py = Py::None;
    let mut v_Wq: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_log_t: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let _ = f__finite(Py::tuple(vec![v_lam.clone(), v_mu.clone()]))?;
    if ({ let t2577 = { let t2573 = v_lam.clone(); Py::Bool({ let t2574 = Py::Int(0i64); if py::le(&t2573, &t2574)? { true } else { false } }) }; if t2577.truthy() { t2577 } else { { let t2575 = v_mu.clone(); Py::Bool({ let t2576 = Py::Int(0i64); if py::le(&t2575, &t2576)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the arrival and service rates must be > 0"))));
    }
    if ({ let t2582 = { let t2578 = v_c.clone(); Py::Bool({ let t2579 = Py::Int(1i64); if py::lt(&t2578, &t2579)? { true } else { false } }) }; if t2582.truthy() { t2582 } else { { let t2580 = v_c.clone(); Py::Bool({ let t2581 = Py::Int(500i64); if py::gt(&t2580, &t2581)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the number of servers must be between 1 and 500"))));
    }
    let t2583 = (Py::tuple(vec![py::truediv(&v_lam.clone(), &v_mu.clone())?, py::truediv(&v_lam.clone(), &py::mul(&v_c.clone(), &v_mu.clone())?)?])).items()?;
    if t2583.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2583[0].clone();
    v_rho = t2583[1].clone();
    if ({ let t2584 = v_rho.clone(); Py::Bool({ let t2585 = Py::Int(1i64); if py::ge(&t2584, &t2585)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the queue is unstable: lambda must be below c * mu"))));
    }
    v_B = f__erlang_b(v_a.clone(), v_c.clone())?;
    v_C = py::truediv(&v_B.clone(), &py::sub(&Py::Int(1i64), &py::mul(&v_rho.clone(), &py::sub(&Py::Int(1i64), &v_B.clone())?)?)?)?;
    v_Lq = py::truediv(&py::mul(&v_C.clone(), &v_rho.clone())?, &py::sub(&Py::Int(1i64), &v_rho.clone())?)?;
    v_Wq = py::truediv(&v_Lq.clone(), &v_lam.clone())?;
    v_W = py::add(&v_Wq.clone(), &py::truediv(&Py::Int(1i64), &v_mu.clone())?)?;
    v_log_t = py::sub(&py::sub(&py::mul(&v_c.clone(), &py::math1("log", &v_a.clone())?)?, &py::math1("lgamma", &py::add(&v_c.clone(), &Py::Int(1i64))?)?)?, &py::math1("log", &py::sub(&Py::Int(1i64), &v_rho.clone())?)?)?;
    v_P0 = py::truediv(&v_C.clone(), &py::math1("exp", &v_log_t.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("rho"), v_rho.clone()), (Py::str("P_wait"), v_C.clone()), (Py::str("Lq"), v_Lq.clone()), (Py::str("Wq"), v_Wq.clone()), (Py::str("W"), v_W.clone()), (Py::str("L"), py::mul(&v_lam.clone(), &v_W.clone())?), (Py::str("P0"), v_P0.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mmc_finite(mut v_lam: Py, mut v_mu: Py, mut v_c: Py, mut v_K: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_Lq: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_lam_eff: Py = Py::None;
    let mut v_logs: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pi: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__finite(Py::tuple(vec![v_lam.clone(), v_mu.clone()]))?;
    if ({ let t2609 = { let t2605 = v_lam.clone(); Py::Bool({ let t2606 = Py::Int(0i64); if py::le(&t2605, &t2606)? { true } else { false } }) }; if t2609.truthy() { t2609 } else { { let t2607 = v_mu.clone(); Py::Bool({ let t2608 = Py::Int(0i64); if py::le(&t2607, &t2608)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the arrival and service rates must be > 0"))));
    }
    if (py::not(&{ let t2610 = Py::Int(1i64); Py::Bool({ let t2611 = v_c.clone(); if py::le(&t2610, &t2611)? { { let t2612 = v_K.clone(); if py::le(&t2611, &t2612)? { { let t2613 = Py::Int(2000i64); if py::le(&t2612, &t2613)? { true } else { false } } } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("need 1 <= servers <= system capacity <= 2000"))));
    }
    v_a = py::truediv(&v_lam.clone(), &v_mu.clone())?;
    v_logs = Py::list(vec![]);
    for t2614 in py::range_iter(&[(py::add(&v_K.clone(), &Py::Int(1i64))?).i()?]) {
        v_n = t2614;
        if ({ let t2615 = v_n.clone(); Py::Bool({ let t2616 = v_c.clone(); if py::le(&t2615, &t2616)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_logs.clone(), "append", &[py::sub(&py::mul(&v_n.clone(), &py::math1("log", &v_a.clone())?)?, &py::math1("lgamma", &py::add(&v_n.clone(), &Py::Int(1i64))?)?)?])?;
        } else {
            let _ = py::method(&v_logs.clone(), "append", &[py::add(&py::sub(&py::mul(&v_c.clone(), &py::math1("log", &v_a.clone())?)?, &py::math1("lgamma", &py::add(&v_c.clone(), &Py::Int(1i64))?)?)?, &py::mul(&py::sub(&v_n.clone(), &v_c.clone())?, &py::math1("log", &py::truediv(&v_a.clone(), &v_c.clone())?)?)?)?])?;
        }
    }
    v_mx = py::minmax(&[v_logs.clone()], true)?;
    v_w = { let mut out = vec![]; for t2617 in py::iter(&(v_logs.clone()))? { let mut v_v: Py; 
v_v = t2617;
out.push(py::math1("exp", &py::sub(&v_v.clone(), &v_mx.clone())?)?); } Py::list(out) };
    v_tot = py::fsum(&v_w.clone())?;
    v_pi = { let mut out = vec![]; for t2618 in py::iter(&(v_w.clone()))? { let mut v_v: Py; 
v_v = t2618;
out.push(py::truediv(&v_v.clone(), &v_tot.clone())?); } Py::list(out) };
    v_L = py::fsum(&{ let mut out = vec![]; for t2619 in py::iter(&(py::enumerate(&v_pi.clone(), 0)?))? { let mut v_n: Py; let mut v_p: Py; 
let t2620 = (t2619).items()?;
if t2620.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_n = t2620[0].clone();
v_p = t2620[1].clone();
out.push(py::mul(&v_n.clone(), &v_p.clone())?); } Py::list(out) })?;
    v_Lq = py::fsum(&{ let mut out = vec![]; for t2621 in py::iter(&(py::enumerate(&v_pi.clone(), 0)?))? { let mut v_n: Py; let mut v_p: Py; 
let t2622 = (t2621).items()?;
if t2622.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_n = t2622[0].clone();
v_p = t2622[1].clone();
out.push(py::mul(&py::minmax(&[py::sub(&v_n.clone(), &v_c.clone())?, Py::Int(0i64)], true)?, &v_p.clone())?); } Py::list(out) })?;
    v_lam_eff = py::mul(&v_lam.clone(), &py::sub(&Py::Int(1i64), &py::getitem(&v_pi.clone(), &v_K.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("P_block"), py::getitem(&v_pi.clone(), &v_K.clone())?), (Py::str("lam_eff"), v_lam_eff.clone()), (Py::str("L"), v_L.clone()), (Py::str("Lq"), v_Lq.clone()), (Py::str("W"), py::truediv(&v_L.clone(), &v_lam_eff.clone())?), (Py::str("Wq"), py::truediv(&v_Lq.clone(), &v_lam_eff.clone())?), (Py::str("P0"), py::getitem(&v_pi.clone(), &Py::Int(0i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mmc_staffing(mut v_lam: Py, mut v_mu: Py, mut v_target: Py, mut v_criterion: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_val: Py = Py::None;
    let _ = f__finite(Py::tuple(vec![v_lam.clone(), v_mu.clone(), v_target.clone()]))?;
    if ({ let t2565 = { let t2558 = v_lam.clone(); Py::Bool({ let t2559 = Py::Int(0i64); if py::le(&t2558, &t2559)? { true } else { false } }) }; if t2565.truthy() { t2565 } else { { let t2564 = { let t2560 = v_mu.clone(); Py::Bool({ let t2561 = Py::Int(0i64); if py::le(&t2560, &t2561)? { true } else { false } }) }; if t2564.truthy() { t2564 } else { { let t2562 = v_target.clone(); Py::Bool({ let t2563 = Py::Int(0i64); if py::le(&t2562, &t2563)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("rates and the target must be > 0"))));
    }
    if ({ let t2570 = { let t2566 = v_criterion.clone(); Py::Bool({ let t2567 = Py::str("wait_probability"); if py::eq(&t2566, &t2567) { true } else { false } }) }; if !t2570.truthy() { t2570 } else { { let t2568 = v_target.clone(); Py::Bool({ let t2569 = Py::Int(1i64); if py::ge(&t2568, &t2569)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a target probability of waiting must be below 1"))));
    }
    v_a = py::truediv(&v_lam.clone(), &v_mu.clone())?;
    if ({ let t2571 = v_a.clone(); Py::Bool({ let t2572 = Py::Int(400i64); if py::gt(&t2571, &t2572)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("offered load too large (at most 400)"))));
    }
    v_c = py::add(&py::int(&py::math1("floor", &v_a.clone())?)?, &Py::Int(1i64))?;
    while (Py::Bool(true)).truthy() {
        v_r = f_mmc(v_lam.clone(), v_mu.clone(), v_c.clone())?;
        v_val = (if ({ let t2587 = v_criterion.clone(); Py::Bool({ let t2588 = Py::str("wait_probability"); if py::eq(&t2587, &t2588) { true } else { false } }) }).truthy() { py::getitem(&v_r.clone(), &Py::str("P_wait"))? } else { py::getitem(&v_r.clone(), &Py::str("Wq"))? });
        if ({ let t2589 = v_val.clone(); Py::Bool({ let t2590 = v_target.clone(); if py::le(&t2589, &t2590)? { true } else { false } }) }).truthy() {
            return Ok(Py::dict(vec![(Py::str("servers"), py::float(&v_c.clone())?), (Py::str("P_wait"), py::getitem(&v_r.clone(), &Py::str("P_wait"))?), (Py::str("Wq"), py::getitem(&v_r.clone(), &Py::str("Wq"))?), (Py::str("utilisation"), py::getitem(&v_r.clone(), &Py::str("rho"))?)]));
        }
        v_c = py::iadd(&v_c, &Py::Int(1i64))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_moore_hodgson(mut v_p: Py, mut v_d: Py) -> R {
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_order: Py = Py::None;
    let mut v_sched: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_tardy: Py = Py::None;
    let _ = f__jobs(v_p.clone(), Py::str("processing times"))?;
    if ({ let t2630 = py::len(&v_d.clone())?; Py::Bool({ let t2631 = py::len(&v_p.clone())?; if py::ne(&t2630, &t2631) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give one due date per job"))));
    }
    v_order = py::sorted_key(&Py::list((py::range_iter(&[(py::len(&v_p.clone())?).i()?])).into_iter().collect::<Vec<Py>>()), &{ let v_d = v_d.clone(); let v_p = v_p.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_j: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::getitem(&v_d.clone(), &v_j.clone())?, v_j.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    let t2632 = (Py::tuple(vec![Py::list(vec![]), py::fraction(&[Py::Int(0i64)])?, Py::list(vec![])])).items()?;
    if t2632.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_sched = t2632[0].clone();
    v_t = t2632[1].clone();
    v_tardy = t2632[2].clone();
    for t2633 in py::iter(&(v_order.clone()))? {
        v_j = t2633;
        let _ = py::method(&v_sched.clone(), "append", &[v_j.clone()])?;
        v_t = py::iadd(&v_t, &f__fr(py::getitem(&v_p.clone(), &v_j.clone())?)?)?;
        if ({ let t2634 = v_t.clone(); Py::Bool({ let t2635 = f__fr(py::getitem(&v_d.clone(), &v_j.clone())?)?; if py::gt(&t2634, &t2635)? { true } else { false } }) }).truthy() {
            v_k = py::minmax_key(&[v_sched.clone()], &{ let v_d = v_d.clone(); let v_j = v_j.clone(); let v_order = v_order.clone(); let v_p = v_p.clone(); let v_sched = v_sched.clone(); let v_t = v_t.clone(); let v_tardy = v_tardy.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::getitem(&v_p.clone(), &v_x.clone())?, v_x.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, true)?;
            let _ = py::method(&v_sched.clone(), "remove", &[v_k.clone()])?;
            let _ = py::method(&v_tardy.clone(), "append", &[v_k.clone()])?;
            v_t = py::sub(&v_t, &f__fr(py::getitem(&v_p.clone(), &v_k.clone())?)?)?;
        }
    }
    return Ok(Py::dict(vec![(Py::str("min_tardy_jobs"), py::float(&py::len(&v_tardy.clone())?)?), (Py::str("on_time_sequence"), { let mut out = vec![]; for t2636 in py::iter(&(v_sched.clone()))? { let mut v_j: Py; 
v_j = t2636;
out.push(py::add(&v_j.clone(), &Py::Int(1i64))?); } Py::list(out) }), (Py::str("tardy_jobs"), py::sorted(&{ let mut out = vec![]; for t2637 in py::iter(&(v_tardy.clone()))? { let mut v_j: Py; 
v_j = t2637;
out.push(py::add(&v_j.clone(), &Py::Int(1i64))?); } Py::list(out) }, (Py::Bool(false)).truthy())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mst(mut v_edges: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_adj: Py = Py::None;
    let mut v_find: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_maxw: Py = Py::None;
    let mut v_nodes: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_parent: Py = Py::None;
    let mut v_ru: Py = Py::None;
    let mut v_rv: Py = Py::None;
    let mut v_tree: Py = Py::None;
    let mut v_tset: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_unique: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_E = f__wedges(v_edges.clone(), Py::Bool(true))?;
    v_nodes = f__nodes(v_E.clone())?;
    v_parent = { let mut out = vec![]; for t2338 in py::iter(&(v_nodes.clone()))? { let mut v_v: Py; 
v_v = t2338;
out.push((v_v.clone(), v_v.clone())); } Py::dict(out) };
    v_find = { let v_E = v_E.clone(); let v_edges = v_edges.clone(); let v_find = v_find.clone(); let v_nodes = v_nodes.clone(); let v_parent = v_parent.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            while ({ let t2339 = py::getitem(&v_parent.clone(), &v_x.clone())?; Py::Bool({ let t2340 = v_x.clone(); if py::ne(&t2339, &t2340) { true } else { false } }) }).truthy() {
                py::setitem(&v_parent.clone(), &v_x.clone(), py::getitem(&v_parent.clone(), &py::getitem(&v_parent.clone(), &v_x.clone())?)?)?;
                v_x = py::getitem(&v_parent.clone(), &v_x.clone())?;
            }
            return Ok(v_x.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_tree = Py::list(vec![]);
    for t2341 in py::iter(&(py::sorted_key(&Py::list((py::range_iter(&[(py::len(&v_E.clone())?).i()?])).into_iter().collect::<Vec<Py>>()), &{ let v_E = v_E.clone(); let v_edges = v_edges.clone(); let v_find = v_find.clone(); let v_nodes = v_nodes.clone(); let v_parent = v_parent.clone(); let v_tree = v_tree.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_k: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::getitem(&py::getitem(&v_E.clone(), &v_k.clone())?, &Py::Int(2i64))?, v_k.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?))? {
        v_k = t2341;
        let t2342 = (py::getitem(&v_E.clone(), &v_k.clone())?).items()?;
        if t2342.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_u = t2342[0].clone();
        v_v = t2342[1].clone();
        v_w = t2342[2].clone();
        let t2343 = (Py::tuple(vec![py::call(&v_find.clone(), &[v_u.clone()])?, py::call(&v_find.clone(), &[v_v.clone()])?])).items()?;
        if t2343.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_ru = t2343[0].clone();
        v_rv = t2343[1].clone();
        if ({ let t2344 = v_ru.clone(); Py::Bool({ let t2345 = v_rv.clone(); if py::ne(&t2344, &t2345) { true } else { false } }) }).truthy() {
            py::setitem(&v_parent.clone(), &v_ru.clone(), v_rv.clone())?;
            let _ = py::method(&v_tree.clone(), "append", &[v_k.clone()])?;
        }
    }
    if ({ let t2346 = py::len(&v_tree.clone())?; Py::Bool({ let t2347 = py::sub(&py::len(&v_nodes.clone())?, &Py::Int(1i64))?; if py::ne(&t2346, &t2347) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the graph is not connected: no spanning tree exists"))));
    }
    v_tset = py::set(&v_tree.clone())?;
    v_adj = { let mut out = vec![]; for t2348 in py::iter(&(v_nodes.clone()))? { let mut v_v: Py; 
v_v = t2348;
out.push((v_v.clone(), Py::list(vec![]))); } Py::dict(out) };
    for t2349 in py::iter(&(v_tree.clone()))? {
        v_k = t2349;
        let t2350 = (py::getitem(&v_E.clone(), &v_k.clone())?).items()?;
        if t2350.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_u = t2350[0].clone();
        v_v = t2350[1].clone();
        v_w = t2350[2].clone();
        let _ = py::method(&py::getitem(&v_adj.clone(), &v_u.clone())?, "append", &[Py::tuple(vec![v_v.clone(), v_w.clone()])])?;
        let _ = py::method(&py::getitem(&v_adj.clone(), &v_v.clone())?, "append", &[Py::tuple(vec![v_u.clone(), v_w.clone()])])?;
    }
    v_maxw = { let v_E = v_E.clone(); let v_adj = v_adj.clone(); let v_edges = v_edges.clone(); let v_find = v_find.clone(); let v_k = v_k.clone(); let v_maxw = v_maxw.clone(); let v_nodes = v_nodes.clone(); let v_parent = v_parent.clone(); let v_ru = v_ru.clone(); let v_rv = v_rv.clone(); let v_tree = v_tree.clone(); let v_tset = v_tset.clone(); let v_u = v_u.clone(); let v_v = v_v.clone(); let v_w = v_w.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_a: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_b: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_m: Py = Py::None;
            let mut v_seen: Py = Py::None;
            let mut v_st: Py = Py::None;
            let mut v_w: Py = Py::None;
            let mut v_x: Py = Py::None;
            let mut v_y: Py = Py::None;
            let t2351 = (Py::tuple(vec![Py::list(vec![Py::tuple(vec![v_a.clone(), py::fraction(&[py::neg(&py::pow(&Py::Int(10i64), &Py::Int(18i64))?)?])?])]), py::set(&Py::list(vec![v_a.clone()]))?])).items()?;
            if t2351.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_st = t2351[0].clone();
            v_seen = t2351[1].clone();
            while (v_st.clone()).truthy() {
                let t2352 = (py::method(&v_st.clone(), "pop", &[])?).items()?;
                if t2352.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_x = t2352[0].clone();
                v_m = t2352[1].clone();
                if ({ let t2353 = v_x.clone(); Py::Bool({ let t2354 = v_b.clone(); if py::eq(&t2353, &t2354) { true } else { false } }) }).truthy() {
                    return Ok(v_m.clone());
                }
                for t2355 in py::iter(&(py::getitem(&v_adj.clone(), &v_x.clone())?))? {
                    let t2358 = (t2355).items()?;
                    if t2358.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    v_y = t2358[0].clone();
                    v_w = t2358[1].clone();
                    if ({ let t2356 = v_y.clone(); Py::Bool({ let t2357 = v_seen.clone(); if !py::contains(&t2357, &t2356)? { true } else { false } }) }).truthy() {
                        let _ = py::method(&v_seen.clone(), "add", &[v_y.clone()])?;
                        let _ = py::method(&v_st.clone(), "append", &[Py::tuple(vec![v_y.clone(), py::minmax(&[v_m.clone(), v_w.clone()], true)?])])?;
                    }
                }
            }
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_unique = Py::Bool(({ let mut out = vec![]; for t2359 in py::iter(&(py::enumerate(&v_E.clone(), 0)?))? { let mut v_k: Py; let mut v_u: Py; let mut v_v: Py; let mut v_w: Py; 
let t2360 = (t2359).items()?;
if t2360.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2360[0].clone();
let t2361 = (t2360[1].clone()).items()?;
if t2361.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_u = t2361[0].clone();
v_v = t2361[1].clone();
v_w = t2361[2].clone();
if ({ let t2366 = { let t2362 = v_k.clone(); Py::Bool({ let t2363 = v_tset.clone(); if !py::contains(&t2363, &t2362)? { true } else { false } }) }; if !t2366.truthy() { t2366 } else { { let t2364 = v_u.clone(); Py::Bool({ let t2365 = v_v.clone(); if py::ne(&t2364, &t2365) { true } else { false } }) } } }).truthy() { out.push({ let t2367 = py::call(&v_maxw.clone(), &[v_u.clone(), v_v.clone()])?; Py::Bool({ let t2368 = v_w.clone(); if py::lt(&t2367, &t2368)? { true } else { false } }) }); }} Py::list(out) }).items()?.iter().all(|x| x.truthy()));
    v_out = Py::dict(vec![(Py::str("total_weight"), py::float(&py::sum(&{ let mut out = vec![]; for t2369 in py::iter(&(v_tree.clone()))? { let mut v_k: Py; 
v_k = t2369;
out.push(py::getitem(&py::getitem(&v_E.clone(), &v_k.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?)?), (Py::str("tree_unique"), (if (v_unique.clone()).truthy() { Py::str("yes") } else { Py::str("no") }))]);
    py::setitem(&v_out.clone(), &(if (v_unique.clone()).truthy() { Py::str("tree_edges") } else { Py::str("tree_edges_example") }), { let mut out = vec![]; for t2370 in py::iter(&(v_tree.clone()))? { let mut v_k: Py; 
v_k = t2370;
out.push(Py::str(&[py::format(&py::getitem(&py::getitem(&v_E.clone(), &v_k.clone())?, &Py::Int(0i64))?, "")?, "-".to_string(), py::format(&py::getitem(&py::getitem(&v_E.clone(), &v_k.clone())?, &Py::Int(1i64))?, "")?].concat())); } Py::list(out) })?;
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_newsvendor_discrete(mut v_p: Py, mut v_c: Py, mut v_demands: Py, mut v_probs: Py, mut v_s: Py) -> R {
    let mut v_Df: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_Qf: Py = Py::None;
    let mut v_cr: Py = Py::None;
    let mut v_cum: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_pr: Py = Py::None;
    let mut v_prof: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let _ = f__nv_check(v_p.clone(), v_c.clone(), v_s.clone())?;
    if ({ let t2661 = { let t2659 = py::len(&v_demands.clone())?; Py::Bool({ let t2660 = py::len(&v_probs.clone())?; if py::ne(&t2659, &t2660) { true } else { false } }) }; if t2661.truthy() { t2661 } else { py::not(&v_demands.clone()) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give one probability per demand value"))));
    }
    if ({ let t2667 = Py::Bool(({ let mut out = vec![]; for t2662 in py::iter(&(v_probs.clone()))? { let mut v_pr: Py; 
v_pr = t2662;
out.push({ let t2663 = v_pr.clone(); Py::Bool({ let t2664 = Py::Int(0i64); if py::lt(&t2663, &t2664)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t2667.truthy() { t2667 } else { { let t2665 = py::abs(&py::sub(&py::sum(&v_probs.clone(), None)?, &Py::Int(1i64))?)?; Py::Bool({ let t2666 = Py::Float(1e-09); if py::gt(&t2665, &t2666)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the probabilities must be >= 0 and sum to 1"))));
    }
    if ({ let t2674 = Py::Bool(({ let mut out = vec![]; for t2668 in py::iter(&(py::zip(&[v_demands.clone(), py::slice(&v_demands.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_d1: Py; let mut v_d2: Py; 
let t2669 = (t2668).items()?;
if t2669.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_d1 = t2669[0].clone();
v_d2 = t2669[1].clone();
out.push({ let t2670 = v_d2.clone(); Py::Bool({ let t2671 = v_d1.clone(); if py::le(&t2670, &t2671)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t2674.truthy() { t2674 } else { { let t2672 = py::getitem(&v_demands.clone(), &Py::Int(0i64))?; Py::Bool({ let t2673 = Py::Int(0i64); if py::lt(&t2672, &t2673)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("demand values must be >= 0 and increasing"))));
    }
    v_P = { let mut out = vec![]; for t2675 in py::iter(&(v_probs.clone()))? { let mut v_v: Py; 
v_v = t2675;
out.push(f__fr(v_v.clone())?); } Py::list(out) };
    v_tot = py::sum(&v_P.clone(), None)?;
    v_P = { let mut out = vec![]; for t2676 in py::iter(&(v_P.clone()))? { let mut v_v: Py; 
v_v = t2676;
out.push(py::truediv(&v_v.clone(), &v_tot.clone())?); } Py::list(out) };
    v_cr = py::truediv(&py::sub(&f__fr(v_p.clone())?, &f__fr(v_c.clone())?)?, &py::sub(&f__fr(v_p.clone())?, &f__fr(v_s.clone())?)?)?;
    let t2677 = (Py::tuple(vec![py::fraction(&[Py::Int(0i64)])?, py::getitem(&v_demands.clone(), &py::neg(&Py::Int(1i64))?)?])).items()?;
    if t2677.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cum = t2677[0].clone();
    v_Q = t2677[1].clone();
    for t2678 in py::iter(&(py::zip(&[v_demands.clone(), v_P.clone()])?))? {
        let t2681 = (t2678).items()?;
        if t2681.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_d = t2681[0].clone();
        v_pr = t2681[1].clone();
        v_cum = py::iadd(&v_cum, &v_pr.clone())?;
        if ({ let t2679 = v_cum.clone(); Py::Bool({ let t2680 = v_cr.clone(); if py::ge(&t2679, &t2680)? { true } else { false } }) }).truthy() {
            v_Q = v_d.clone();
            break;
        }
    }
    v_Df = { let mut out = vec![]; for t2682 in py::iter(&(v_demands.clone()))? { let mut v_d: Py; 
v_d = t2682;
out.push(f__fr(v_d.clone())?); } Py::list(out) };
    v_Qf = f__fr(v_Q.clone())?;
    v_prof = py::sum(&{ let mut out = vec![]; for t2683 in py::iter(&(py::zip(&[v_Df.clone(), v_P.clone()])?))? { let mut v_d: Py; let mut v_pr: Py; 
let t2684 = (t2683).items()?;
if t2684.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_d = t2684[0].clone();
v_pr = t2684[1].clone();
out.push(py::mul(&v_pr.clone(), &py::sub(&py::add(&py::mul(&f__fr(v_p.clone())?, &py::minmax(&[v_d.clone(), v_Qf.clone()], false)?)?, &py::mul(&f__fr(v_s.clone())?, &py::minmax(&[py::sub(&v_Qf.clone(), &v_d.clone())?, Py::Int(0i64)], true)?)?)?, &py::mul(&f__fr(v_c.clone())?, &v_Qf.clone())?)?)?); } Py::list(out) }, None)?;
    return Ok(Py::dict(vec![(Py::str("critical_ratio"), py::float(&v_cr.clone())?), (Py::str("Q"), py::float(&v_Q.clone())?), (Py::str("expected_profit"), py::float(&v_prof.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_newsvendor_normal(mut v_p: Py, mut v_c: Py, mut v_mu: Py, mut v_sigma: Py, mut v_s: Py) -> R {
    let mut v_Q: Py = Py::None;
    let mut v_cr: Py = Py::None;
    let mut v_loss: Py = Py::None;
    let mut v_sales: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__nv_check(v_p.clone(), v_c.clone(), v_s.clone())?;
    let _ = f__finite(Py::tuple(vec![v_mu.clone(), v_sigma.clone()]))?;
    if ({ let t2695 = v_sigma.clone(); Py::Bool({ let t2696 = Py::Int(0i64); if py::le(&t2695, &t2696)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the standard deviation of demand must be > 0"))));
    }
    v_cr = py::truediv(&py::sub(&v_p.clone(), &v_c.clone())?, &py::sub(&v_p.clone(), &v_s.clone())?)?;
    v_z = py::normal_dist("inv_cdf", &Py::Float(0.0), &Py::Float(1.0), &[v_cr.clone()])?;
    v_Q = py::add(&v_mu.clone(), &py::mul(&v_z.clone(), &v_sigma.clone())?)?;
    v_loss = py::mul(&v_sigma.clone(), &py::sub(&py::normal_dist("pdf", &Py::Float(0.0), &Py::Float(1.0), &[v_z.clone()])?, &py::mul(&v_z.clone(), &py::sub(&Py::Int(1i64), &v_cr.clone())?)?)?)?;
    v_sales = py::sub(&v_mu.clone(), &v_loss.clone())?;
    return Ok(Py::dict(vec![(Py::str("critical_ratio"), v_cr.clone()), (Py::str("Q"), v_Q.clone()), (Py::str("expected_profit"), py::sub(&py::mul(&py::sub(&v_p.clone(), &v_s.clone())?, &v_sales.clone())?, &py::mul(&py::sub(&v_c.clone(), &v_s.clone())?, &v_Q.clone())?)?), (Py::str("expected_shortage"), v_loss.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_newsvendor_uniform(mut v_p: Py, mut v_c: Py, mut v_lo: Py, mut v_hi: Py, mut v_s: Py) -> R {
    let mut v_Q: Py = Py::None;
    let mut v_cr: Py = Py::None;
    let mut v_loss: Py = Py::None;
    let mut v_sales: Py = Py::None;
    let _ = f__nv_check(v_p.clone(), v_c.clone(), v_s.clone())?;
    let _ = f__finite(Py::tuple(vec![v_lo.clone(), v_hi.clone()]))?;
    if (py::not(&{ let t2704 = Py::Int(0i64); Py::Bool({ let t2705 = v_lo.clone(); if py::le(&t2704, &t2705)? { { let t2706 = v_hi.clone(); if py::lt(&t2705, &t2706)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("uniform demand needs 0 <= lower < upper"))));
    }
    v_cr = py::truediv(&py::sub(&v_p.clone(), &v_c.clone())?, &py::sub(&v_p.clone(), &v_s.clone())?)?;
    v_Q = py::add(&v_lo.clone(), &py::mul(&v_cr.clone(), &py::sub(&v_hi.clone(), &v_lo.clone())?)?)?;
    v_loss = py::truediv(&py::pow(&py::sub(&v_hi.clone(), &v_Q.clone())?, &Py::Int(2i64))?, &py::mul(&Py::Int(2i64), &py::sub(&v_hi.clone(), &v_lo.clone())?)?)?;
    v_sales = py::sub(&py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?, &v_loss.clone())?;
    return Ok(Py::dict(vec![(Py::str("critical_ratio"), v_cr.clone()), (Py::str("Q"), v_Q.clone()), (Py::str("expected_profit"), py::sub(&py::mul(&py::sub(&v_p.clone(), &v_s.clone())?, &v_sales.clone())?, &py::mul(&py::sub(&v_c.clone(), &v_s.clone())?, &v_Q.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_open_box(mut v_L: Py, mut v_W: Py) -> R {
    let mut v_x: Py = Py::None;
    let _ = f__finite(Py::tuple(vec![v_L.clone(), v_W.clone()]))?;
    if ({ let t2715 = { let t2711 = v_L.clone(); Py::Bool({ let t2712 = Py::Int(0i64); if py::le(&t2711, &t2712)? { true } else { false } }) }; if t2715.truthy() { t2715 } else { { let t2713 = v_W.clone(); Py::Bool({ let t2714 = Py::Int(0i64); if py::le(&t2713, &t2714)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the sheet dimensions must be > 0"))));
    }
    v_x = py::truediv(&py::sub(&py::add(&v_L.clone(), &v_W.clone())?, &py::math1("sqrt", &py::add(&py::sub(&py::mul(&v_L.clone(), &v_L.clone())?, &py::mul(&v_L.clone(), &v_W.clone())?)?, &py::mul(&v_W.clone(), &v_W.clone())?)?)?)?, &Py::Int(6i64))?;
    return Ok(Py::dict(vec![(Py::str("cut"), v_x.clone()), (Py::str("max_volume"), py::mul(&py::mul(&v_x.clone(), &py::sub(&v_L.clone(), &py::mul(&Py::Int(2i64), &v_x.clone())?)?)?, &py::sub(&v_W.clone(), &py::mul(&Py::Int(2i64), &v_x.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_order_up_to(mut v_d: Py, mut v_sigma: Py, mut v_R: Py, mut v_L: Py, mut v_SL: Py) -> R {
    let mut v_ss: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__finite(Py::tuple(vec![v_d.clone(), v_sigma.clone(), v_R.clone(), v_L.clone(), v_SL.clone()]))?;
    if ({ let t2733 = { let t2723 = v_d.clone(); Py::Bool({ let t2724 = Py::Int(0i64); if py::lt(&t2723, &t2724)? { true } else { false } }) }; if t2733.truthy() { t2733 } else { { let t2732 = { let t2725 = v_sigma.clone(); Py::Bool({ let t2726 = Py::Int(0i64); if py::lt(&t2725, &t2726)? { true } else { false } }) }; if t2732.truthy() { t2732 } else { { let t2731 = { let t2727 = v_R.clone(); Py::Bool({ let t2728 = Py::Int(0i64); if py::le(&t2727, &t2728)? { true } else { false } }) }; if t2731.truthy() { t2731 } else { { let t2729 = v_L.clone(); Py::Bool({ let t2730 = Py::Int(0i64); if py::lt(&t2729, &t2730)? { true } else { false } }) } } } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("demand and its standard deviation must be >= 0, the review period > 0 and the lead time >= 0"))));
    }
    if (py::not(&{ let t2734 = Py::Float(0.5); Py::Bool({ let t2735 = v_SL.clone(); if py::le(&t2734, &t2735)? { { let t2736 = Py::Int(1i64); if py::lt(&t2735, &t2736)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the service level must be at least 0.5 and below 1"))));
    }
    v_z = py::normal_dist("inv_cdf", &Py::Float(0.0), &Py::Float(1.0), &[v_SL.clone()])?;
    v_ss = py::mul(&py::mul(&v_z.clone(), &v_sigma.clone())?, &py::math1("sqrt", &py::add(&v_R.clone(), &v_L.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("order_up_to_level"), py::add(&py::mul(&v_d.clone(), &py::add(&v_R.clone(), &v_L.clone())?)?, &v_ss.clone())?), (Py::str("safety_stock"), v_ss.clone()), (Py::str("z"), v_z.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_parallel_makespan(mut v_p: Py, mut v_m: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_assign: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_cur: Py = Py::None;
    let mut v_dfs: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lb: Py = Py::None;
    let mut v_loads: Py = Py::None;
    let mut v_order: Py = Py::None;
    let _ = f__jobs(v_p.clone(), Py::str("processing times"))?;
    if (py::not(&{ let t2744 = Py::Int(1i64); Py::Bool({ let t2745 = v_m.clone(); if py::le(&t2744, &t2745)? { { let t2746 = Py::Int(6i64); if py::le(&t2745, &t2746)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the number of machines must be between 1 and 6"))));
    }
    v_P = { let mut out = vec![]; for t2747 in py::iter(&(v_p.clone()))? { let mut v_v: Py; 
v_v = t2747;
out.push(f__fr(v_v.clone())?); } Py::list(out) };
    v_order = py::sorted_key(&Py::list((py::range_iter(&[(py::len(&v_P.clone())?).i()?])).into_iter().collect::<Vec<Py>>()), &{ let v_P = v_P.clone(); let v_m = v_m.clone(); let v_p = v_p.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_j: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::neg(&py::getitem(&v_P.clone(), &v_j.clone())?)?, v_j.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    v_loads = py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &v_m.clone())?;
    v_assign = py::mul(&Py::list(vec![Py::Int(0i64)]), &py::len(&v_P.clone())?)?;
    for t2748 in py::iter(&(v_order.clone()))? {
        v_j = t2748;
        v_k = py::minmax_key(&[Py::list((py::range_iter(&[(v_m.clone()).i()?])).into_iter().collect::<Vec<Py>>())], &{ let v_P = v_P.clone(); let v_assign = v_assign.clone(); let v_j = v_j.clone(); let v_loads = v_loads.clone(); let v_m = v_m.clone(); let v_order = v_order.clone(); let v_p = v_p.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_i: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::getitem(&v_loads.clone(), &v_i.clone())?, v_i.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, false)?;
        let t2749 = v_loads.clone(); let t2750 = v_k.clone();
        py::setitem(&t2749, &t2750, py::iadd(&py::getitem(&t2749, &t2750)?, &py::getitem(&v_P.clone(), &v_j.clone())?)?)?;
        py::setitem(&v_assign.clone(), &v_j.clone(), v_k.clone())?;
    }
    v_best = Py::list(vec![py::minmax(&[v_loads.clone()], true)?, Py::list((v_assign.clone()).items()?)]);
    v_lb = py::minmax(&[py::minmax(&[v_P.clone()], true)?, py::truediv(&py::sum(&v_P.clone(), None)?, &v_m.clone())?], true)?;
    v_cur = py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &v_m.clone())?;
    v_a = py::mul(&Py::list(vec![Py::Int(0i64)]), &py::len(&v_P.clone())?)?;
    let rec2751 = py::rec_cell();
    v_dfs = { let v_P = v_P.clone(); let v_a = v_a.clone(); let v_assign = v_assign.clone(); let v_best = v_best.clone(); let v_cur = v_cur.clone(); let v_j = v_j.clone(); let v_k = v_k.clone(); let v_lb = v_lb.clone(); let v_loads = v_loads.clone(); let v_m = v_m.clone(); let v_order = v_order.clone(); let v_p = v_p.clone(); let rec2751 = rec2751.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            let v_dfs: Py = py::rec_get(&rec2751)?;
            let mut v_j: Py = Py::None;
            let mut v_k: Py = Py::None;
            let mut v_mk: Py = Py::None;
            let mut v_seen: Py = Py::None;
            if ({ let t2752 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; Py::Bool({ let t2753 = v_lb.clone(); if py::eq(&t2752, &t2753) { true } else { false } }) }).truthy() {
                return Ok(Py::None);
            }
            if ({ let t2754 = v_t.clone(); Py::Bool({ let t2755 = py::len(&v_order.clone())?; if py::eq(&t2754, &t2755) { true } else { false } }) }).truthy() {
                v_mk = py::minmax(&[v_cur.clone()], true)?;
                if ({ let t2756 = v_mk.clone(); Py::Bool({ let t2757 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; if py::lt(&t2756, &t2757)? { true } else { false } }) }).truthy() {
                    let t2758 = (Py::tuple(vec![v_mk.clone(), Py::list((v_a.clone()).items()?)])).items()?;
                    if t2758.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    py::setitem(&v_best.clone(), &Py::Int(0i64), t2758[0].clone())?;
                    py::setitem(&v_best.clone(), &Py::Int(1i64), t2758[1].clone())?;
                }
                return Ok(Py::None);
            }
            v_j = py::getitem(&v_order.clone(), &v_t.clone())?;
            v_seen = py::set(&Py::list(vec![]))?;
            for t2759 in py::range_iter(&[(v_m.clone()).i()?]) {
                v_k = t2759;
                if ({ let t2764 = { let t2760 = py::getitem(&v_cur.clone(), &v_k.clone())?; Py::Bool({ let t2761 = v_seen.clone(); if py::contains(&t2761, &t2760)? { true } else { false } }) }; if t2764.truthy() { t2764 } else { { let t2762 = py::add(&py::getitem(&v_cur.clone(), &v_k.clone())?, &py::getitem(&v_P.clone(), &v_j.clone())?)?; Py::Bool({ let t2763 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; if py::ge(&t2762, &t2763)? { true } else { false } }) } } }).truthy() {
                    continue;
                }
                let _ = py::method(&v_seen.clone(), "add", &[py::getitem(&v_cur.clone(), &v_k.clone())?])?;
                let t2765 = v_cur.clone(); let t2766 = v_k.clone();
                py::setitem(&t2765, &t2766, py::iadd(&py::getitem(&t2765, &t2766)?, &py::getitem(&v_P.clone(), &v_j.clone())?)?)?;
                py::setitem(&v_a.clone(), &v_j.clone(), v_k.clone())?;
                let _ = py::call(&v_dfs.clone(), &[py::add(&v_t.clone(), &Py::Int(1i64))?])?;
                let t2767 = v_cur.clone(); let t2768 = v_k.clone();
                py::setitem(&t2767, &t2768, py::sub(&py::getitem(&t2767, &t2768)?, &py::getitem(&v_P.clone(), &v_j.clone())?)?)?;
            }
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec2751, &v_dfs);
    let _ = py::call(&v_dfs.clone(), &[Py::Int(0i64)])?;
    return Ok(Py::dict(vec![(Py::str("makespan"), py::float(&py::getitem(&v_best.clone(), &Py::Int(0i64))?)?), (Py::str("machine_of_job"), { let mut out = vec![]; for t2769 in py::iter(&(py::getitem(&v_best.clone(), &Py::Int(1i64))?))? { let mut v_k: Py; 
v_k = t2769;
out.push(py::add(&v_k.clone(), &Py::Int(1i64))?); } Py::list(out) }), (Py::str("lower_bound"), py::float(&v_lb.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pert(mut v_times: Py, mut v_precedences: Py, mut v_deadline: Py) -> R {
    let mut v_ES: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_cnt: Py = Py::None;
    let mut v_ends: Py = Py::None;
    let mut v_finish: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_order: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_path: Py = Py::None;
    let mut v_preds: Py = Py::None;
    let mut v_te: Py = Py::None;
    let mut v_var: Py = Py::None;
    if (Py::Bool(({ let mut out = vec![]; for t2782 in py::iter(&(v_times.clone()))? { let mut v_r: Py; 
v_r = t2782;
out.push({ let t2783 = py::len(&v_r.clone())?; Py::Bool({ let t2784 = Py::Int(3i64); if py::ne(&t2783, &t2784) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("each activity needs three times: optimistic, most likely, pessimistic"))));
    }
    for t2785 in py::iter(&(v_times.clone()))? {
        let t2790 = (t2785).items()?;
        if t2790.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_a = t2790[0].clone();
        v_m = t2790[1].clone();
        v_b = t2790[2].clone();
        if (py::not(&{ let t2786 = Py::Int(0i64); Py::Bool({ let t2787 = v_a.clone(); if py::le(&t2786, &t2787)? { { let t2788 = v_m.clone(); if py::le(&t2787, &t2788)? { { let t2789 = v_b.clone(); if py::le(&t2788, &t2789)? { true } else { false } } } else { false } } } else { false } }) })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("each activity needs 0 <= optimistic <= most likely <= pessimistic"))));
        }
    }
    v_te = { let mut out = vec![]; for t2791 in py::iter(&(v_times.clone()))? { let mut v_a: Py; let mut v_b: Py; let mut v_m: Py; 
let t2792 = (t2791).items()?;
if t2792.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_a = t2792[0].clone();
v_m = t2792[1].clone();
v_b = t2792[2].clone();
out.push(py::truediv(&py::add(&py::add(&f__fr(v_a.clone())?, &py::mul(&Py::Int(4i64), &f__fr(v_m.clone())?)?)?, &f__fr(v_b.clone())?)?, &Py::Int(6i64))?); } Py::list(out) };
    v_var = { let mut out = vec![]; for t2793 in py::iter(&(v_times.clone()))? { let mut v_a: Py; let mut v_b: Py; let mut v_m: Py; 
let t2794 = (t2793).items()?;
if t2794.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_a = t2794[0].clone();
v_m = t2794[1].clone();
v_b = t2794[2].clone();
out.push(py::pow(&py::truediv(&py::sub(&f__fr(v_b.clone())?, &f__fr(v_a.clone())?)?, &Py::Int(6i64))?, &Py::Int(2i64))?); } Py::list(out) };
    let t2823 = (f__project({ let mut out = vec![]; for t2822 in py::iter(&(v_te.clone()))? { let mut v_v: Py; 
v_v = t2822;
out.push(py::float(&v_v.clone())?); } Py::list(out) }, v_precedences.clone())?).items()?;
    if t2823.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_preds = t2823[0].clone();
    v_order = t2823[1].clone();
    v_n = py::len(&v_te.clone())?;
    let t2824 = (Py::tuple(vec![py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &v_n.clone())?, py::mul(&Py::list(vec![Py::Int(1i64)]), &v_n.clone())?])).items()?;
    if t2824.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ES = t2824[0].clone();
    v_cnt = t2824[1].clone();
    for t2825 in py::iter(&(v_order.clone()))? {
        v_k = t2825;
        if (py::getitem(&v_preds.clone(), &v_k.clone())?).truthy() {
            v_best = py::minmax(&[{ let mut out = vec![]; for t2826 in py::iter(&(py::getitem(&v_preds.clone(), &v_k.clone())?))? { let mut v_j: Py; 
v_j = t2826;
out.push(py::add(&py::getitem(&v_ES.clone(), &v_j.clone())?, &py::getitem(&v_te.clone(), &v_j.clone())?)?); } Py::list(out) }], true)?;
            let t2830 = (Py::tuple(vec![v_best.clone(), py::sum(&{ let mut out = vec![]; for t2827 in py::iter(&(py::getitem(&v_preds.clone(), &v_k.clone())?))? { let mut v_j: Py; 
v_j = t2827;
if ({ let t2828 = py::add(&py::getitem(&v_ES.clone(), &v_j.clone())?, &py::getitem(&v_te.clone(), &v_j.clone())?)?; Py::Bool({ let t2829 = v_best.clone(); if py::eq(&t2828, &t2829) { true } else { false } }) }).truthy() { out.push(py::getitem(&v_cnt.clone(), &v_j.clone())?); }} Py::list(out) }, None)?])).items()?;
            if t2830.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            py::setitem(&v_ES.clone(), &v_k.clone(), t2830[0].clone())?;
            py::setitem(&v_cnt.clone(), &v_k.clone(), t2830[1].clone())?;
        }
    }
    v_finish = { let mut out = vec![]; for t2831 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t2831;
out.push(py::add(&py::getitem(&v_ES.clone(), &v_k.clone())?, &py::getitem(&v_te.clone(), &v_k.clone())?)?); } Py::list(out) };
    v_T = py::minmax(&[v_finish.clone()], true)?;
    v_ends = { let mut out = vec![]; for t2832 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t2832;
if ({ let t2833 = py::getitem(&v_finish.clone(), &v_k.clone())?; Py::Bool({ let t2834 = v_T.clone(); if py::eq(&t2833, &t2834) { true } else { false } }) }).truthy() { out.push(v_k.clone()); }} Py::list(out) };
    if ({ let t2835 = py::sum(&{ let mut out = vec![]; for t2837 in py::iter(&(v_ends.clone()))? { let mut v_k: Py; 
v_k = t2837;
out.push(py::getitem(&v_cnt.clone(), &v_k.clone())?); } Py::list(out) }, None)?; Py::Bool({ let t2836 = Py::Int(1i64); if py::gt(&t2835, &t2836)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("there is more than one critical path, so the PERT variance of the project is not determined"))));
    }
    let t2838 = (Py::tuple(vec![Py::list(vec![]), py::getitem(&v_ends.clone(), &Py::Int(0i64))?])).items()?;
    if t2838.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_path = t2838[0].clone();
    v_k = t2838[1].clone();
    while (Py::Bool(true)).truthy() {
        let _ = py::method(&v_path.clone(), "append", &[v_k.clone()])?;
        if (py::not(&py::getitem(&v_preds.clone(), &v_k.clone())?)).truthy() {
            break;
        }
        v_k = (match ('g2839: { let mut out = vec![]; for t2840 in py::iter(&(py::getitem(&v_preds.clone(), &v_k.clone())?))? { let mut v_j: Py; 
v_j = t2840;
if ({ let t2841 = py::add(&py::getitem(&v_ES.clone(), &v_j.clone())?, &py::getitem(&v_te.clone(), &v_j.clone())?)?; Py::Bool({ let t2842 = py::getitem(&v_ES.clone(), &v_k.clone())?; if py::eq(&t2841, &t2842) { true } else { false } }) }).truthy() { out.push(v_j.clone()); break 'g2839 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) });
    }
    let _ = py::method(&v_path.clone(), "reverse", &[])?;
    v_V = py::sum(&{ let mut out = vec![]; for t2843 in py::iter(&(v_path.clone()))? { let mut v_k: Py; 
v_k = t2843;
out.push(py::getitem(&v_var.clone(), &v_k.clone())?); } Py::list(out) }, None)?;
    v_out = Py::dict(vec![(Py::str("expected_duration"), py::float(&v_T.clone())?), (Py::str("variance"), py::float(&v_V.clone())?), (Py::str("std_dev"), py::math1("sqrt", &v_V.clone())?), (Py::str("critical_path"), { let mut out = vec![]; for t2844 in py::iter(&(v_path.clone()))? { let mut v_k: Py; 
v_k = t2844;
out.push(py::chr(&py::add(&Py::Int(65i64), &v_k.clone())?)?); } Py::list(out) }), (Py::str("expected_times"), f__fl(v_te.clone())?)]);
    if ({ let t2846 = v_deadline.clone(); Py::Bool({ let t2847 = Py::None; if !py::is(&t2846, &t2847) { true } else { false } }) }).truthy() {
        let _ = f__finite(Py::tuple(vec![v_deadline.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("probability"), (if ({ let t2851 = v_V.clone(); Py::Bool({ let t2852 = Py::Int(0i64); if py::eq(&t2851, &t2852) { true } else { false } }) }).truthy() { (if ({ let t2853 = v_deadline.clone(); Py::Bool({ let t2854 = v_T.clone(); if py::ge(&t2853, &t2854)? { true } else { false } }) }).truthy() { Py::Float(1.0) } else { Py::Float(0.0) }) } else { py::normal_dist("cdf", &Py::Float(0.0), &Py::Float(1.0), &[py::truediv(&py::sub(&v_deadline.clone(), &py::float(&v_T.clone())?)?, &py::math1("sqrt", &v_V.clone())?)?])? }))?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_poly_extrema(mut v_coefficients: Py, mut v_lo: Py, mut v_hi: Py) -> R {
    let mut v_cand: Py = Py::None;
    let mut v_cs: Py = Py::None;
    let mut v_mn: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_tol: Py = Py::None;
    let mut v_vals: Py = Py::None;
    let _ = f__finite({ let mut v = vec![v_lo.clone(), v_hi.clone()]; v.extend((v_coefficients.clone()).items()?); Py::tuple(v) })?;
    if (py::not(&{ let t2866 = v_lo.clone(); Py::Bool({ let t2867 = v_hi.clone(); if py::lt(&t2866, &t2867)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the interval needs lo < hi"))));
    }
    v_cs = { let mut out = vec![]; for t2868 in py::iter(&(v_coefficients.clone()))? { let mut v_v: Py; 
v_v = t2868;
out.push(py::float(&v_v.clone())?); } Py::list(out) };
    if ({ let t2873 = { let t2869 = py::len(&v_cs.clone())?; Py::Bool({ let t2870 = Py::Int(2i64); if py::lt(&t2869, &t2870)? { true } else { false } }) }; if t2873.truthy() { t2873 } else { { let t2871 = py::len(&v_cs.clone())?; Py::Bool({ let t2872 = Py::Int(6i64); if py::gt(&t2871, &t2872)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give 2 to 6 coefficients (degree 1 to 5), highest power first"))));
    }
    v_cand = py::add(&Py::list(vec![v_lo.clone(), v_hi.clone()]), &f__roots_in(f__pder(v_cs.clone())?, v_lo.clone(), v_hi.clone())?)?;
    v_vals = { let mut out = vec![]; for t2911 in py::iter(&(v_cand.clone()))? { let mut v_x: Py; 
v_x = t2911;
out.push(Py::tuple(vec![f__peval(v_cs.clone(), v_x.clone())?, v_x.clone()])); } Py::list(out) };
    let t2916 = (Py::tuple(vec![py::minmax(&[{ let mut out = vec![]; for t2912 in py::iter(&(v_vals.clone()))? { let mut v__: Py; let mut v_v: Py; 
let t2913 = (t2912).items()?;
if t2913.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t2913[0].clone();
v__ = t2913[1].clone();
out.push(v_v.clone()); } Py::list(out) }], false)?, py::minmax(&[{ let mut out = vec![]; for t2914 in py::iter(&(v_vals.clone()))? { let mut v__: Py; let mut v_v: Py; 
let t2915 = (t2914).items()?;
if t2915.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t2915[0].clone();
v__ = t2915[1].clone();
out.push(v_v.clone()); } Py::list(out) }], true)?])).items()?;
    if t2916.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mn = t2916[0].clone();
    v_mx = t2916[1].clone();
    v_tol = { let v_cand = v_cand.clone(); let v_coefficients = v_coefficients.clone(); let v_cs = v_cs.clone(); let v_hi = v_hi.clone(); let v_lo = v_lo.clone(); let v_mn = v_mn.clone(); let v_mx = v_mx.clone(); let v_vals = v_vals.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::mul(&Py::Float(1e-12), &py::minmax(&[Py::Float(1.0), py::abs(&v_t.clone())?], true)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok(Py::dict(vec![(Py::str("min_value"), v_mn.clone()), (Py::str("max_value"), v_mx.clone()), (Py::str("argmin_example"), py::minmax(&[{ let mut out = vec![]; for t2917 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; let mut v_x: Py; 
let t2918 = (t2917).items()?;
if t2918.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t2918[0].clone();
v_x = t2918[1].clone();
if ({ let t2919 = v_v.clone(); Py::Bool({ let t2920 = py::add(&v_mn.clone(), &py::call(&v_tol.clone(), &[v_mn.clone()])?)?; if py::le(&t2919, &t2920)? { true } else { false } }) }).truthy() { out.push(v_x.clone()); }} Py::list(out) }], false)?), (Py::str("argmax_example"), py::minmax(&[{ let mut out = vec![]; for t2921 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; let mut v_x: Py; 
let t2922 = (t2921).items()?;
if t2922.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t2922[0].clone();
v_x = t2922[1].clone();
if ({ let t2923 = v_v.clone(); Py::Bool({ let t2924 = py::sub(&v_mx.clone(), &py::call(&v_tol.clone(), &[v_mx.clone()])?)?; if py::ge(&t2923, &t2924)? { true } else { false } }) }).truthy() { out.push(v_x.clone()); }} Py::list(out) }], false)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_profit_max_linear(mut v_a: Py, mut v_b: Py, mut v_cv: Py, mut v_F_: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_Fx: Py = Py::None;
    let mut v_qq: Py = Py::None;
    let _ = f__finite(Py::tuple(vec![v_a.clone(), v_b.clone(), v_cv.clone(), v_F_.clone()]))?;
    if ({ let t2936 = v_b.clone(); Py::Bool({ let t2937 = Py::Int(0i64); if py::le(&t2936, &t2937)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the demand slope b must be > 0 (price falls as quantity rises)"))));
    }
    if ({ let t2938 = v_a.clone(); Py::Bool({ let t2939 = v_cv.clone(); if py::le(&t2938, &t2939)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the demand intercept must exceed the marginal cost, otherwise no positive output is profitable"))));
    }
    let t2940 = (Py::tuple(vec![f__fr(v_a.clone())?, f__fr(v_b.clone())?, f__fr(v_cv.clone())?, f__fr(v_F_.clone())?])).items()?;
    if t2940.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_A = t2940[0].clone();
    v_B = t2940[1].clone();
    v_C = t2940[2].clone();
    v_Fx = t2940[3].clone();
    v_qq = py::truediv(&py::sub(&v_A.clone(), &v_C.clone())?, &py::mul(&Py::Int(2i64), &v_B.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("quantity"), py::float(&v_qq.clone())?), (Py::str("price"), py::float(&py::sub(&v_A.clone(), &py::mul(&v_B.clone(), &v_qq.clone())?)?)?), (Py::str("profit"), py::float(&py::sub(&py::sub(&py::mul(&py::sub(&v_A.clone(), &py::mul(&v_B.clone(), &v_qq.clone())?)?, &v_qq.clone())?, &py::mul(&v_C.clone(), &v_qq.clone())?)?, &v_Fx.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pure_nash(mut v_A: Py, mut v_B: Py) -> R {
    let mut v_MA: Py = Py::None;
    let mut v_MB: Py = Py::None;
    let mut v_eq: Py = Py::None;
    let t2960 = (f__bimatrix(v_A.clone(), v_B.clone(), Py::Int(8i64))?).items()?;
    if t2960.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_MA = t2960[0].clone();
    v_MB = t2960[1].clone();
    v_eq = f__pure_ne(v_MA.clone(), v_MB.clone())?;
    return Ok(Py::dict(vec![(Py::str("count"), py::float(&py::len(&v_eq.clone())?)?), (Py::str("equilibria"), { let mut out = vec![]; for t2971 in py::iter(&(v_eq.clone()))? { let mut v_a: Py; let mut v_b: Py; 
let t2972 = (t2971).items()?;
if t2972.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2972[0].clone();
v_b = t2972[1].clone();
out.push(Py::list(vec![py::float(&v_a.clone())?, py::float(&v_b.clone())?])); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_quadratic_min(mut v_Q: Py, mut v_c: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_cf: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_M = f__sym_pd(v_Q.clone(), Py::str("Q"))?;
    if ({ let t3621 = py::len(&v_c.clone())?; Py::Bool({ let t3622 = py::len(&v_M.clone())?; if py::ne(&t3621, &t3622) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["c must have ".to_string(), py::format(&py::len(&v_M.clone())?, "")?, " entries".to_string()].concat()))));
    }
    v_cf = { let mut out = vec![]; for t3623 in py::iter(&(v_c.clone()))? { let mut v_v: Py; 
v_v = t3623;
out.push(f__fr(v_v.clone())?); } Py::list(out) };
    v_x = f__solve_exact(v_M.clone(), { let mut out = vec![]; for t3643 in py::iter(&(v_cf.clone()))? { let mut v_v: Py; 
v_v = t3643;
out.push(py::neg(&v_v.clone())?); } Py::list(out) })?;
    return Ok(Py::dict(vec![(Py::str("f_min"), py::float(&f__qf(v_M.clone(), v_cf.clone(), v_x.clone())?)?), (Py::str("x_star"), f__fl(v_x.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_quadratic_optimum(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_goal: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_Cc: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__finite(Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone()]))?;
    if ({ let t2984 = v_a.clone(); Py::Bool({ let t2985 = Py::Int(0i64); if py::eq(&t2984, &t2985) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a must not be 0: a linear function has no interior optimum"))));
    }
    if ({ let t2990 = { let t2986 = v_goal.clone(); Py::Bool({ let t2987 = Py::str("minimum"); if py::eq(&t2986, &t2987) { true } else { false } }) }; if !t2990.truthy() { t2990 } else { { let t2988 = v_a.clone(); Py::Bool({ let t2989 = Py::Int(0i64); if py::lt(&t2988, &t2989)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("with a < 0 the parabola opens downward: it has a maximum, no minimum"))));
    }
    if ({ let t2995 = { let t2991 = v_goal.clone(); Py::Bool({ let t2992 = Py::str("maximum"); if py::eq(&t2991, &t2992) { true } else { false } }) }; if !t2995.truthy() { t2995 } else { { let t2993 = v_a.clone(); Py::Bool({ let t2994 = Py::Int(0i64); if py::gt(&t2993, &t2994)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("with a > 0 the parabola opens upward: it has a minimum, no maximum"))));
    }
    let t2996 = (Py::tuple(vec![f__fr(v_a.clone())?, f__fr(v_b.clone())?, f__fr(v_c.clone())?])).items()?;
    if t2996.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t2996[0].clone();
    v_B = t2996[1].clone();
    v_Cc = t2996[2].clone();
    v_x = py::truediv(&py::neg(&v_B.clone())?, &py::mul(&Py::Int(2i64), &v_A.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("x_opt"), py::float(&v_x.clone())?), (Py::str("f_opt"), py::float(&py::sub(&v_Cc.clone(), &py::truediv(&py::mul(&v_B.clone(), &v_B.clone())?, &py::mul(&Py::Int(4i64), &v_A.clone())?)?)?)?), (Py::str("type"), (if ({ let t2997 = v_a.clone(); Py::Bool({ let t2998 = Py::Int(0i64); if py::gt(&t2997, &t2998)? { true } else { false } }) }).truthy() { Py::str("minimum") } else { Py::str("maximum") }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_reorder_point(mut v_d: Py, mut v_L: Py, mut v_sigma: Py, mut v_SL: Py) -> R {
    let mut v_ss: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__finite(Py::tuple(vec![v_d.clone(), v_L.clone(), v_sigma.clone(), v_SL.clone()]))?;
    if ({ let t3010 = { let t3003 = v_d.clone(); Py::Bool({ let t3004 = Py::Int(0i64); if py::lt(&t3003, &t3004)? { true } else { false } }) }; if t3010.truthy() { t3010 } else { { let t3009 = { let t3005 = v_L.clone(); Py::Bool({ let t3006 = Py::Int(0i64); if py::le(&t3005, &t3006)? { true } else { false } }) }; if t3009.truthy() { t3009 } else { { let t3007 = v_sigma.clone(); Py::Bool({ let t3008 = Py::Int(0i64); if py::lt(&t3007, &t3008)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("demand must be >= 0, lead time > 0 and the standard deviation >= 0"))));
    }
    if (py::not(&{ let t3011 = Py::Float(0.5); Py::Bool({ let t3012 = v_SL.clone(); if py::le(&t3011, &t3012)? { { let t3013 = Py::Int(1i64); if py::lt(&t3012, &t3013)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the service level must be at least 0.5 and below 1 (e.g. 0.95)"))));
    }
    v_z = py::normal_dist("inv_cdf", &Py::Float(0.0), &Py::Float(1.0), &[v_SL.clone()])?;
    v_ss = py::mul(&py::mul(&v_z.clone(), &v_sigma.clone())?, &py::math1("sqrt", &v_L.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("reorder_point"), py::add(&py::mul(&v_d.clone(), &v_L.clone())?, &v_ss.clone())?), (Py::str("safety_stock"), v_ss.clone()), (Py::str("z"), v_z.clone()), (Py::str("lead_time_demand"), py::mul(&v_d.clone(), &v_L.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_revenue_max_linear(mut v_alpha: Py, mut v_beta: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_p: Py = Py::None;
    let _ = f__finite(Py::tuple(vec![v_alpha.clone(), v_beta.clone()]))?;
    if ({ let t3022 = { let t3018 = v_alpha.clone(); Py::Bool({ let t3019 = Py::Int(0i64); if py::le(&t3018, &t3019)? { true } else { false } }) }; if t3022.truthy() { t3022 } else { { let t3020 = v_beta.clone(); Py::Bool({ let t3021 = Py::Int(0i64); if py::le(&t3020, &t3021)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("demand q = alpha - beta p needs alpha > 0 and beta > 0"))));
    }
    let t3023 = (Py::tuple(vec![f__fr(v_alpha.clone())?, f__fr(v_beta.clone())?])).items()?;
    if t3023.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3023[0].clone();
    v_B = t3023[1].clone();
    v_p = py::truediv(&v_A.clone(), &py::mul(&Py::Int(2i64), &v_B.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("price"), py::float(&v_p.clone())?), (Py::str("quantity"), py::float(&py::sub(&v_A.clone(), &py::mul(&v_B.clone(), &v_p.clone())?)?)?), (Py::str("revenue"), py::float(&py::mul(&v_p.clone(), &py::sub(&v_A.clone(), &py::mul(&v_B.clone(), &v_p.clone())?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_set_cover(mut v_sets: Py, mut v_costs: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_cost: Py = Py::None;
    let mut v_cov: Py = Py::None;
    let mut v_full: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_masks: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pick: Py = Py::None;
    v_m = py::len(&v_sets.clone())?;
    if ({ let t3032 = { let t3025 = v_m.clone(); Py::Bool({ let t3026 = Py::Int(0i64); if py::eq(&t3025, &t3026) { true } else { false } }) }; if t3032.truthy() { t3032 } else { { let t3031 = { let t3027 = v_m.clone(); Py::Bool({ let t3028 = Py::Int(18i64); if py::gt(&t3027, &t3028)? { true } else { false } }) }; if t3031.truthy() { t3031 } else { { let t3029 = py::len(&v_costs.clone())?; Py::Bool({ let t3030 = v_m.clone(); if py::ne(&t3029, &t3030) { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give between 1 and 18 sets, one cost each"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t3033 in py::iter(&(v_sets.clone()))? { let mut v_r: Py; 
v_r = t3033;
for t3034 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t3034;
out.push({ let t3035 = v_x.clone(); Py::Bool({ let t3036 = Py::tuple(vec![Py::Int(0i64), Py::Int(1i64)]); if !py::contains(&t3036, &t3035)? { true } else { false } }) }); }} Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the incidence matrix must contain only 0 and 1"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t3037 in py::iter(&(v_costs.clone()))? { let mut v_c: Py; 
v_c = t3037;
out.push({ let t3038 = v_c.clone(); Py::Bool({ let t3039 = Py::Int(0i64); if py::lt(&t3038, &t3039)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("costs must be >= 0"))));
    }
    v_n = py::len(&py::getitem(&v_sets.clone(), &Py::Int(0i64))?)?;
    v_masks = { let mut out = vec![]; for t3040 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t3040;
out.push(py::sum(&{ let mut out = vec![]; for t3041 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3041;
if (py::getitem(&py::getitem(&v_sets.clone(), &v_i.clone())?, &v_j.clone())?).truthy() { out.push(py::lshift(&Py::Int(1i64), &v_j.clone())?); }} Py::list(out) }, None)?); } Py::list(out) };
    v_full = py::sub(&py::lshift(&Py::Int(1i64), &v_n.clone())?, &Py::Int(1i64))?;
    v_C = { let mut out = vec![]; for t3042 in py::iter(&(v_costs.clone()))? { let mut v_c: Py; 
v_c = t3042;
out.push(f__fr(v_c.clone())?); } Py::list(out) };
    v_best = Py::None;
    for t3043 in py::range_iter(&[(py::lshift(&Py::Int(1i64), &v_m.clone())?).i()?]) {
        v_pick = t3043;
        let t3044 = (Py::tuple(vec![Py::Int(0i64), py::fraction(&[Py::Int(0i64)])?])).items()?;
        if t3044.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_cov = t3044[0].clone();
        v_cost = t3044[1].clone();
        for t3045 in py::range_iter(&[(v_m.clone()).i()?]) {
            v_i = t3045;
            if (py::bitand(&py::rshift(&v_pick.clone(), &v_i.clone())?, &Py::Int(1i64))?).truthy() {
                v_cov = py::bitor(&v_cov, &py::getitem(&v_masks.clone(), &v_i.clone())?)?;
                v_cost = py::iadd(&v_cost, &py::getitem(&v_C.clone(), &v_i.clone())?)?;
            }
        }
        if ({ let t3053 = { let t3046 = v_cov.clone(); Py::Bool({ let t3047 = v_full.clone(); if py::eq(&t3046, &t3047) { true } else { false } }) }; if !t3053.truthy() { t3053 } else { { let t3052 = { let t3048 = v_best.clone(); Py::Bool({ let t3049 = Py::None; if py::is(&t3048, &t3049) { true } else { false } }) }; if t3052.truthy() { t3052 } else { { let t3050 = v_cost.clone(); Py::Bool({ let t3051 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; if py::lt(&t3050, &t3051)? { true } else { false } }) } } } } }).truthy() {
            v_best = Py::tuple(vec![v_cost.clone(), v_pick.clone()]);
        }
    }
    if ({ let t3054 = v_best.clone(); Py::Bool({ let t3055 = Py::None; if py::is(&t3054, &t3055) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("some element is in no set: no cover exists"))));
    }
    return Ok(Py::dict(vec![(Py::str("min_cost"), py::float(&py::getitem(&v_best.clone(), &Py::Int(0i64))?)?), (Py::str("sets_example"), { let mut out = vec![]; for t3056 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t3056;
if (py::bitand(&py::rshift(&py::getitem(&v_best.clone(), &Py::Int(1i64))?, &v_i.clone())?, &Py::Int(1i64))?).truthy() { out.push(py::add(&v_i.clone(), &Py::Int(1i64))?); }} Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_shortest_path(mut v_edges: Py, mut v_source: Py, mut v_target: Py, mut v_directed: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_adj: Py = Py::None;
    let mut v_dist: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_E = f__wedges(v_edges.clone(), Py::Bool(true))?;
    if (Py::Bool(({ let mut out = vec![]; for t3068 in py::iter(&(v_E.clone()))? { let mut v__: Py; let mut v_w: Py; 
let t3069 = (t3068).items()?;
if t3069.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v__ = t3069[0].clone();
v__ = t3069[1].clone();
v_w = t3069[2].clone();
out.push({ let t3070 = v_w.clone(); Py::Bool({ let t3071 = Py::Int(0i64); if py::lt(&t3070, &t3071)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("negative weights: use the Bellman-Ford capability"))));
    }
    let t3072 = (Py::tuple(vec![py::method(&Py::str(&py::to_str(&v_source.clone())), "strip", &[])?, py::method(&Py::str(&py::to_str(&v_target.clone())), "strip", &[])?])).items()?;
    if t3072.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t3072[0].clone();
    v_t = t3072[1].clone();
    if ({ let t3081 = { let t3073 = v_s.clone(); Py::Bool({ let t3074 = f__nodes(v_E.clone())?; if !py::contains(&t3074, &t3073)? { true } else { false } }) }; if t3081.truthy() { t3081 } else { { let t3079 = v_t.clone(); Py::Bool({ let t3080 = f__nodes(v_E.clone())?; if !py::contains(&t3080, &t3079)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the source and the target must be nodes of the graph"))));
    }
    let t3095 = (f__dijkstra(v_E.clone(), v_s.clone(), v_directed.clone())?).items()?;
    if t3095.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_dist = t3095[0].clone();
    v_adj = t3095[1].clone();
    return Ok(f__sp_result(v_E.clone(), v_s.clone(), v_t.clone(), v_directed.clone(), v_dist.clone(), v_adj.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_spt(mut v_p: Py) -> R {
    let mut v_seq: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let _ = f__jobs(v_p.clone(), Py::str("processing times"))?;
    v_seq = py::sorted_key(&Py::list((py::range_iter(&[(py::len(&v_p.clone())?).i()?])).into_iter().collect::<Vec<Py>>()), &{ let v_p = v_p.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_j: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::getitem(&v_p.clone(), &v_j.clone())?, v_j.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    v_tot = py::sum(&py::method(&f__completion(v_seq.clone(), v_p.clone())?, "values", &[])?, None)?;
    return Ok(Py::dict(vec![(Py::str("sequence"), { let mut out = vec![]; for t3164 in py::iter(&(v_seq.clone()))? { let mut v_j: Py; 
v_j = t3164;
out.push(py::add(&v_j.clone(), &Py::Int(1i64))?); } Py::list(out) }), (Py::str("total_completion_time"), py::float(&v_tot.clone())?), (Py::str("mean_flow_time"), py::float(&py::truediv(&v_tot.clone(), &py::len(&v_p.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_steepest_step(mut v_Q: Py, mut v_c: Py, mut v_x0: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_alpha: Py = Py::None;
    let mut v_cf: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_gg: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_x1: Py = Py::None;
    v_M = f__sym_pd(v_Q.clone(), Py::str("Q"))?;
    v_n = py::len(&v_M.clone())?;
    if ({ let t3201 = { let t3197 = py::len(&v_c.clone())?; Py::Bool({ let t3198 = v_n.clone(); if py::ne(&t3197, &t3198) { true } else { false } }) }; if t3201.truthy() { t3201 } else { { let t3199 = py::len(&v_x0.clone())?; Py::Bool({ let t3200 = v_n.clone(); if py::ne(&t3199, &t3200) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["c and x0 must have ".to_string(), py::format(&v_n.clone(), "")?, " entries".to_string()].concat()))));
    }
    let t3204 = (Py::tuple(vec![{ let mut out = vec![]; for t3202 in py::iter(&(v_c.clone()))? { let mut v_v: Py; 
v_v = t3202;
out.push(f__fr(v_v.clone())?); } Py::list(out) }, { let mut out = vec![]; for t3203 in py::iter(&(v_x0.clone()))? { let mut v_v: Py; 
v_v = t3203;
out.push(f__fr(v_v.clone())?); } Py::list(out) }])).items()?;
    if t3204.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cf = t3204[0].clone();
    v_x = t3204[1].clone();
    v_g = { let mut out = vec![]; for t3205 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3205;
out.push(py::add(&py::sum(&{ let mut out = vec![]; for t3206 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3206;
out.push(py::mul(&py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_x.clone(), &v_j.clone())?)?); } Py::list(out) }, None)?, &py::getitem(&v_cf.clone(), &v_i.clone())?)?); } Py::list(out) };
    v_gg = py::sum(&{ let mut out = vec![]; for t3207 in py::iter(&(v_g.clone()))? { let mut v_v: Py; 
v_v = t3207;
out.push(py::mul(&v_v.clone(), &v_v.clone())?); } Py::list(out) }, None)?;
    if ({ let t3208 = v_gg.clone(); Py::Bool({ let t3209 = Py::Int(0i64); if py::eq(&t3208, &t3209) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("x0 is already the minimiser (the gradient is zero)"))));
    }
    v_alpha = py::truediv(&v_gg.clone(), &py::sum(&{ let mut out = vec![]; for t3210 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3210;
for t3211 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3211;
out.push(py::mul(&py::mul(&py::getitem(&v_g.clone(), &v_i.clone())?, &py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_j.clone())?)?, &py::getitem(&v_g.clone(), &v_j.clone())?)?); }} Py::list(out) }, None)?)?;
    v_x1 = { let mut out = vec![]; for t3212 in py::iter(&(py::zip(&[v_x.clone(), v_g.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3213 = (t3212).items()?;
if t3213.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3213[0].clone();
v_b = t3213[1].clone();
out.push(py::sub(&v_a.clone(), &py::mul(&v_alpha.clone(), &v_b.clone())?)?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("alpha"), py::float(&v_alpha.clone())?), (Py::str("x1"), f__fl(v_x1.clone())?), (Py::str("f1"), py::float(&f__qf(v_M.clone(), v_cf.clone(), v_x1.clone())?)?), (Py::str("gradient"), f__fl(v_g.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_topsis(mut v_X: Py, mut v_w: Py, mut v_types: Py) -> R {
    let mut v_V: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_dm: Py = Py::None;
    let mut v_dp: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_norms: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3272 = (f__mcda_in(v_X.clone(), v_w.clone(), v_types.clone())?).items()?;
    if t3272.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_X = t3272[0].clone();
    v_w = t3272[1].clone();
    v_types = t3272[2].clone();
    let t3273 = (Py::tuple(vec![py::len(&v_X.clone())?, py::len(&py::getitem(&v_X.clone(), &Py::Int(0i64))?)?])).items()?;
    if t3273.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t3273[0].clone();
    v_k = t3273[1].clone();
    v_norms = { let mut out = vec![]; for t3274 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t3274;
out.push(py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t3275 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t3275;
out.push(py::pow(&py::getitem(&py::getitem(&v_X.clone(), &v_i.clone())?, &v_j.clone())?, &Py::Int(2i64))?); } Py::list(out) })?)?); } Py::list(out) };
    v_V = { let mut out = vec![]; for t3276 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t3276;
out.push({ let mut out = vec![]; for t3277 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t3277;
out.push(py::truediv(&py::mul(&py::getitem(&v_w.clone(), &v_j.clone())?, &py::getitem(&py::getitem(&v_X.clone(), &v_i.clone())?, &v_j.clone())?)?, &py::getitem(&v_norms.clone(), &v_j.clone())?)?); } Py::list(out) }); } Py::list(out) };
    v_best = { let mut out = vec![]; for t3278 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t3278;
out.push((if ({ let t3279 = py::getitem(&v_types.clone(), &v_j.clone())?; Py::Bool({ let t3280 = Py::Int(1i64); if py::eq(&t3279, &t3280) { true } else { false } }) }).truthy() { py::minmax(&[{ let mut out = vec![]; for t3281 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t3281;
out.push(py::getitem(&py::getitem(&v_V.clone(), &v_i.clone())?, &v_j.clone())?); } Py::list(out) }], true)? } else { py::minmax(&[{ let mut out = vec![]; for t3282 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t3282;
out.push(py::getitem(&py::getitem(&v_V.clone(), &v_i.clone())?, &v_j.clone())?); } Py::list(out) }], false)? })); } Py::list(out) };
    v_worst = { let mut out = vec![]; for t3283 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t3283;
out.push((if ({ let t3284 = py::getitem(&v_types.clone(), &v_j.clone())?; Py::Bool({ let t3285 = Py::Int(1i64); if py::eq(&t3284, &t3285) { true } else { false } }) }).truthy() { py::minmax(&[{ let mut out = vec![]; for t3286 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t3286;
out.push(py::getitem(&py::getitem(&v_V.clone(), &v_i.clone())?, &v_j.clone())?); } Py::list(out) }], false)? } else { py::minmax(&[{ let mut out = vec![]; for t3287 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t3287;
out.push(py::getitem(&py::getitem(&v_V.clone(), &v_i.clone())?, &v_j.clone())?); } Py::list(out) }], true)? })); } Py::list(out) };
    v_sc = Py::list(vec![]);
    for t3288 in py::range_iter(&[(v_m.clone()).i()?]) {
        v_i = t3288;
        v_dp = py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t3289 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t3289;
out.push(py::pow(&py::sub(&py::getitem(&py::getitem(&v_V.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_best.clone(), &v_j.clone())?)?, &Py::Int(2i64))?); } Py::list(out) })?)?;
        v_dm = py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t3290 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t3290;
out.push(py::pow(&py::sub(&py::getitem(&py::getitem(&v_V.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_worst.clone(), &v_j.clone())?)?, &Py::Int(2i64))?); } Py::list(out) })?)?;
        if ({ let t3291 = py::add(&v_dp.clone(), &v_dm.clone())?; Py::Bool({ let t3292 = Py::Int(0i64); if py::eq(&t3291, &t3292) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("every alternative is identical: no ranking"))));
        }
        let _ = py::method(&v_sc.clone(), "append", &[py::truediv(&v_dm.clone(), &py::add(&v_dp.clone(), &v_dm.clone())?)?])?;
    }
    return Ok(f__mcda_out(v_sc.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_transportation(mut v_supply: Py, mut v_demand: Py, mut v_cost: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_plan: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_rel: Py = Py::None;
    let mut v_sense: Py = Py::None;
    let t3353 = (f__transport_lp(v_supply.clone(), v_demand.clone(), v_cost.clone())?).items()?;
    if t3353.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_sense = t3353[0].clone();
    v_c = t3353[1].clone();
    v_A = t3353[2].clone();
    v_b = t3353[3].clone();
    v_rel = t3353[4].clone();
    v_r = f__lp(v_sense.clone(), v_c.clone(), v_A.clone(), v_b.clone(), v_rel.clone(), Py::Bool(true))?;
    v_n = py::len(&v_demand.clone())?;
    v_plan = { let mut out = vec![]; for t3508 in py::range_iter(&[(py::len(&v_supply.clone())?).i()?]) { let mut v_i: Py; 
v_i = t3508;
out.push({ let mut out = vec![]; for t3509 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3509;
out.push(py::float(&py::getitem(&py::getitem(&v_r.clone(), &Py::str("x"))?, &py::add(&py::mul(&v_i.clone(), &v_n.clone())?, &v_j.clone())?)?)?); } Py::list(out) }); } Py::list(out) };
    v_out = Py::dict(vec![(Py::str("min_cost"), py::float(&py::getitem(&v_r.clone(), &Py::str("value"))?)?), ((if (py::getitem(&v_r.clone(), &Py::str("x_unique"))?).truthy() { Py::str("shipments") } else { Py::str("shipments_example") }), v_plan.clone()), (Py::str("plan_unique"), (if (py::getitem(&v_r.clone(), &Py::str("x_unique"))?).truthy() { Py::str("yes") } else { Py::str("no") })), (Py::str("dual"), f__fl(py::getitem(&v_r.clone(), &Py::str("p"))?)?)]);
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_tsp(mut v_D: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_base: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_cand: Py = Py::None;
    let mut v_dp: Py = Py::None;
    let mut v_full: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_mask: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_prev: Py = Py::None;
    let mut v_tour: Py = Py::None;
    v_n = py::len(&v_D.clone())?;
    if ({ let t3562 = { let t3554 = v_n.clone(); Py::Bool({ let t3555 = Py::Int(2i64); if py::lt(&t3554, &t3555)? { true } else { false } }) }; if t3562.truthy() { t3562 } else { { let t3561 = { let t3556 = v_n.clone(); Py::Bool({ let t3557 = Py::Int(11i64); if py::gt(&t3556, &t3557)? { true } else { false } }) }; if t3561.truthy() { t3561 } else { Py::Bool(({ let mut out = vec![]; for t3558 in py::iter(&(v_D.clone()))? { let mut v_r: Py; 
v_r = t3558;
out.push({ let t3559 = py::len(&v_r.clone())?; Py::Bool({ let t3560 = v_n.clone(); if py::ne(&t3559, &t3560) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give a square distance matrix with 2 to 11 cities"))));
    }
    v_M = { let mut out = vec![]; for t3563 in py::iter(&(v_D.clone()))? { let mut v_r: Py; 
v_r = t3563;
out.push({ let mut out = vec![]; for t3564 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t3564;
out.push(f__fr(v_x.clone())?); } Py::list(out) }); } Py::list(out) };
    v_full = py::lshift(&Py::Int(1i64), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?;
    v_dp = { let mut out = vec![]; for t3565 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t3565;
out.push((Py::tuple(vec![py::lshift(&Py::Int(1i64), &v_k.clone())?, v_k.clone()]), Py::tuple(vec![py::getitem(&py::getitem(&v_M.clone(), &Py::Int(0i64))?, &py::add(&v_k.clone(), &Py::Int(1i64))?)?, Py::Int(0i64)]))); } Py::dict(out) };
    for t3566 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_full.clone()).i()?]) {
        v_mask = t3566;
        for t3567 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
            v_k = t3567;
            if ({ let t3570 = py::not(&py::bitand(&py::rshift(&v_mask.clone(), &v_k.clone())?, &Py::Int(1i64))?); if t3570.truthy() { t3570 } else { { let t3568 = Py::tuple(vec![v_mask.clone(), v_k.clone()]); Py::Bool({ let t3569 = v_dp.clone(); if !py::contains(&t3569, &t3568)? { true } else { false } }) } } }).truthy() {
                continue;
            }
            v_base = py::getitem(&py::getitem(&v_dp.clone(), &Py::tuple(vec![v_mask.clone(), v_k.clone()]))?, &Py::Int(0i64))?;
            for t3571 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
                v_j = t3571;
                if (py::bitand(&py::rshift(&v_mask.clone(), &v_j.clone())?, &Py::Int(1i64))?).truthy() {
                    continue;
                }
                v_key = Py::tuple(vec![py::bitor(&v_mask.clone(), &py::lshift(&Py::Int(1i64), &v_j.clone())?)?, v_j.clone()]);
                v_cand = py::add(&v_base.clone(), &py::getitem(&py::getitem(&v_M.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?, &py::add(&v_j.clone(), &Py::Int(1i64))?)?)?;
                if ({ let t3576 = { let t3572 = v_key.clone(); Py::Bool({ let t3573 = v_dp.clone(); if !py::contains(&t3573, &t3572)? { true } else { false } }) }; if t3576.truthy() { t3576 } else { { let t3574 = v_cand.clone(); Py::Bool({ let t3575 = py::getitem(&py::getitem(&v_dp.clone(), &v_key.clone())?, &Py::Int(0i64))?; if py::lt(&t3574, &t3575)? { true } else { false } }) } } }).truthy() {
                    py::setitem(&v_dp.clone(), &v_key.clone(), Py::tuple(vec![v_cand.clone(), v_k.clone()]))?;
                }
            }
        }
    }
    v_best = py::minmax(&[{ let mut out = vec![]; for t3577 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t3577;
out.push(Py::tuple(vec![py::add(&py::getitem(&py::getitem(&v_dp.clone(), &Py::tuple(vec![py::sub(&v_full.clone(), &Py::Int(1i64))?, v_k.clone()]))?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_M.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?, &Py::Int(0i64))?)?, v_k.clone()])); } Py::list(out) }], false)?;
    let t3578 = (Py::tuple(vec![Py::list(vec![]), py::sub(&v_full.clone(), &Py::Int(1i64))?, py::getitem(&v_best.clone(), &Py::Int(1i64))?])).items()?;
    if t3578.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_tour = t3578[0].clone();
    v_mask = t3578[1].clone();
    v_k = t3578[2].clone();
    while (Py::Bool(true)).truthy() {
        let _ = py::method(&v_tour.clone(), "append", &[py::add(&v_k.clone(), &Py::Int(1i64))?])?;
        v_prev = py::getitem(&py::getitem(&v_dp.clone(), &Py::tuple(vec![v_mask.clone(), v_k.clone()]))?, &Py::Int(1i64))?;
        if ({ let t3579 = v_mask.clone(); Py::Bool({ let t3580 = py::lshift(&Py::Int(1i64), &v_k.clone())?; if py::eq(&t3579, &t3580) { true } else { false } }) }).truthy() {
            break;
        }
        let t3581 = (Py::tuple(vec![py::bitxor(&v_mask.clone(), &py::lshift(&Py::Int(1i64), &v_k.clone())?)?, v_prev.clone()])).items()?;
        if t3581.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_mask = t3581[0].clone();
        v_k = t3581[1].clone();
    }
    v_tour = py::add(&Py::list(vec![Py::Int(1i64)]), &{ let mut out = vec![]; for t3582 in py::iter(&(py::reversed(&v_tour.clone())?))? { let mut v_c: Py; 
v_c = t3582;
out.push(py::add(&v_c.clone(), &Py::Int(1i64))?); } Py::list(out) })?;
    return Ok(Py::dict(vec![(Py::str("min_tour_length"), py::float(&py::getitem(&v_best.clone(), &Py::Int(0i64))?)?), (Py::str("tour_example"), py::add(&v_tour.clone(), &Py::list(vec![Py::Int(1i64)]))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_ip(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_again: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_bi: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_rel: Py = Py::None;
    let mut v_rl: Py = Py::None;
    let mut v_row: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t1106 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("c"))?, py::getitem(&v_inputs.clone(), &Py::str("A"))?, py::getitem(&v_inputs.clone(), &Py::str("b"))?])).items()?;
    if t1106.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_c = t1106[0].clone();
    v_A = t1106[1].clone();
    v_b = t1106[2].clone();
    v_rel = f__relations(py::method(&v_inputs.clone(), "get", &[Py::str("relations")])?, py::len(&v_A.clone())?)?;
    v_x = py::method(&v_outputs.clone(), "get", &[Py::str("x_one_optimal")])?;
    if ({ let t1107 = v_x.clone(); Py::Bool({ let t1108 = Py::None; if py::is(&t1107, &t1108) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_worst = py::minmax(&[{ let mut out = vec![]; for t1109 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t1109;
out.push(py::abs(&py::sub(&v_v.clone(), &py::round(&v_v.clone(), None)?)?)?); } Py::list(out) }], true)?;
    v_worst = py::minmax(&[v_worst.clone(), py::minmax(&[Py::Float(0.0), py::neg(&py::minmax(&[v_x.clone()], false)?)?], true)?], true)?;
    for t1110 in py::iter(&(py::zip(&[v_A.clone(), v_b.clone(), v_rel.clone()])?))? {
        let t1113 = (t1110).items()?;
        if t1113.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_row = t1113[0].clone();
        v_bi = t1113[1].clone();
        v_rl = t1113[2].clone();
        v_v = py::sum(&{ let mut out = vec![]; for t1111 in py::iter(&(py::zip(&[v_row.clone(), v_x.clone()])?))? { let mut v_a: Py; let mut v_xi: Py; 
let t1112 = (t1111).items()?;
if t1112.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1112[0].clone();
v_xi = t1112[1].clone();
out.push(py::mul(&v_a.clone(), &v_xi.clone())?); } Py::list(out) }, None)?;
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::getitem(&Py::dict(vec![(Py::str("<="), py::minmax(&[Py::Float(0.0), py::sub(&v_v.clone(), &v_bi.clone())?], true)?), (Py::str(">="), py::minmax(&[Py::Float(0.0), py::sub(&v_bi.clone(), &v_v.clone())?], true)?), (Py::str("="), py::abs(&py::sub(&v_v.clone(), &v_bi.clone())?)?)]), &v_rl.clone())?, &py::minmax(&[Py::Float(1.0), py::abs(&v_bi.clone())?], true)?)?], true)?;
    }
    v_worst = py::minmax(&[v_worst.clone(), f__rel(py::sum(&{ let mut out = vec![]; for t1114 in py::iter(&(py::zip(&[v_c.clone(), v_x.clone()])?))? { let mut v_ci: Py; let mut v_xi: Py; 
let t1115 = (t1114).items()?;
if t1115.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_ci = t1115[0].clone();
v_xi = t1115[1].clone();
out.push(py::mul(&v_ci.clone(), &v_xi.clone())?); } Py::list(out) }, None)?, py::getitem(&v_outputs.clone(), &Py::str("optimal_value"))?)?], true)?;
    v_key = Py::str(&py::repr(&Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("sense"))?, v_c.clone(), v_A.clone(), v_b.clone(), py::method(&v_inputs.clone(), "get", &[Py::str("relations")])?])));
    if ({ let t1116 = v_key.clone(); Py::Bool({ let t1117 = c__IP_MEMO()?; if !py::contains(&t1117, &t1116)? { true } else { false } }) }).truthy() {
        let t1118: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&c__IP_MEMO()?, &v_key.clone(), py::getitem(&f_integer_program(py::getitem(&v_inputs.clone(), &Py::str("sense"))?, v_c.clone(), v_A.clone(), v_b.clone(), py::method(&v_inputs.clone(), "get", &[Py::str("relations")])?, Py::Bool(true))?, &Py::str("optimal_value"))?)?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1118 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (matches!(e, PyErr::Value(_))) => {
                py::setitem(&c__IP_MEMO()?, &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1119 = py::len(&c__IP_MEMO()?)?; Py::Bool({ let t1120 = Py::Int(256i64); if py::gt(&t1119, &t1120)? { true } else { false } }) }).truthy() {
            let _ = py::method(&c__IP_MEMO()?, "pop", &[(match (c__IP_MEMO()?).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_again = py::getitem(&c__IP_MEMO()?, &v_key.clone())?;
    if ({ let t1121 = v_again.clone(); Py::Bool({ let t1122 = Py::None; if py::is(&t1121, &t1122) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::minmax(&[v_worst.clone(), f__rel(v_again.clone(), py::getitem(&v_outputs.clone(), &Py::str("optimal_value"))?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_lp(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_p: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t1640 = (Py::tuple(vec![f__out_x(v_outputs.clone())?, f__out_p(v_outputs.clone())?])).items()?;
    if t1640.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t1640[0].clone();
    v_p = t1640[1].clone();
    if ({ let t1645 = { let t1641 = v_x.clone(); Py::Bool({ let t1642 = Py::None; if py::is(&t1641, &t1642) { true } else { false } }) }; if t1645.truthy() { t1645 } else { { let t1643 = v_p.clone(); Py::Bool({ let t1644 = Py::None; if py::is(&t1643, &t1644) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(f__lp_certificate_residual(v_inputs.clone(), v_x.clone(), v_p.clone(), py::getitem(&v_outputs.clone(), &Py::str("optimal_value"))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_lp2(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_bi: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_rel: Py = Py::None;
    let mut v_rl: Py = Py::None;
    let mut v_row: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_vals: Py = Py::None;
    let mut v_viol: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let t1811 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("c"))?, py::getitem(&v_inputs.clone(), &Py::str("A"))?, py::getitem(&v_inputs.clone(), &Py::str("b"))?])).items()?;
    if t1811.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_c = t1811[0].clone();
    v_A = t1811[1].clone();
    v_b = t1811[2].clone();
    v_rel = f__relations(py::method(&v_inputs.clone(), "get", &[Py::str("relations")])?, py::len(&v_A.clone())?)?;
    let t1813: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t1812 = (f__lp2_vertices(v_c.clone(), v_A.clone(), v_b.clone(), v_rel.clone())?).items()?;
            if t1812.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_pts = t1812[0].clone();
            v__ = t1812[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1813 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    v_vals = { let mut out = vec![]; for t1814 in py::iter(&(v_pts.clone()))? { let mut v_x: Py; let mut v_y: Py; 
let t1815 = (t1814).items()?;
if t1815.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1815[0].clone();
v_y = t1815[1].clone();
out.push(py::float(&py::add(&py::mul(&py::getitem(&v_c.clone(), &Py::Int(0i64))?, &v_x.clone())?, &py::mul(&py::getitem(&v_c.clone(), &Py::Int(1i64))?, &v_y.clone())?)?)?); } Py::list(out) };
    if (py::not(&v_vals.clone())).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_best = (if ({ let t1816 = py::getitem(&v_inputs.clone(), &Py::str("sense"))?; Py::Bool({ let t1817 = Py::str("maximize"); if py::eq(&t1816, &t1817) { true } else { false } }) }).truthy() { py::minmax(&[v_vals.clone()], true)? } else { py::minmax(&[v_vals.clone()], false)? });
    v_worst = f__rel(v_best.clone(), py::getitem(&v_outputs.clone(), &Py::str("optimal_value"))?)?;
    for t1818 in py::iter(&(py::method(&v_outputs.clone(), "get", &[Py::str("optimal_corners"), Py::list(vec![])])?))? {
        let t1821 = (t1818).items()?;
        if t1821.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_x = t1821[0].clone();
        v_y = t1821[1].clone();
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::add(&py::mul(&py::getitem(&v_c.clone(), &Py::Int(0i64))?, &v_x.clone())?, &py::mul(&py::getitem(&v_c.clone(), &Py::Int(1i64))?, &v_y.clone())?)?, py::getitem(&v_outputs.clone(), &Py::str("optimal_value"))?)?], true)?;
        for t1819 in py::iter(&(py::zip(&[v_A.clone(), v_b.clone(), v_rel.clone()])?))? {
            let t1820 = (t1819).items()?;
            if t1820.len() != 3 { return Err(PyErr::Value("unpack".into())); }
            v_row = t1820[0].clone();
            v_bi = t1820[1].clone();
            v_rl = t1820[2].clone();
            v_v = py::add(&py::mul(&py::getitem(&v_row.clone(), &Py::Int(0i64))?, &v_x.clone())?, &py::mul(&py::getitem(&v_row.clone(), &Py::Int(1i64))?, &v_y.clone())?)?;
            v_viol = py::getitem(&Py::dict(vec![(Py::str("<="), py::minmax(&[Py::Float(0.0), py::sub(&v_v.clone(), &v_bi.clone())?], true)?), (Py::str(">="), py::minmax(&[Py::Float(0.0), py::sub(&v_bi.clone(), &v_v.clone())?], true)?), (Py::str("="), py::abs(&py::sub(&v_v.clone(), &v_bi.clone())?)?)]), &v_rl.clone())?;
            v_worst = py::minmax(&[v_worst.clone(), py::truediv(&v_viol.clone(), &py::minmax(&[Py::Float(1.0), py::abs(&v_bi.clone())?], true)?)?], true)?;
        }
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_vertex_cover(mut v_edges: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_cov: Py = Py::None;
    let mut v_nodes: Py = Py::None;
    let mut v_rec: Py = Py::None;
    let t2417 = (f__simple_graph(v_edges.clone(), Py::Int(30i64))?).items()?;
    if t2417.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_nodes = t2417[0].clone();
    v_adj = t2417[1].clone();
    v_best = Py::list(vec![py::set(&v_nodes.clone())?]);
    let rec2418 = py::rec_cell();
    v_rec = { let v_adj = v_adj.clone(); let v_best = v_best.clone(); let v_edges = v_edges.clone(); let v_nodes = v_nodes.clone(); let rec2418 = rec2418.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_cover: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_rem: Py = a.get(1).cloned().unwrap_or(Py::None);
            let v_rec: Py = py::rec_get(&rec2418)?;
            let mut v_choice: Py = Py::None;
            let mut v_live: Py = Py::None;
            let mut v_r2: Py = Py::None;
            let mut v_take: Py = Py::None;
            let mut v_u: Py = Py::None;
            let mut v_v: Py = Py::None;
            let mut v_w: Py = Py::None;
            let mut v_x: Py = Py::None;
            if ({ let t2419 = py::len(&v_cover.clone())?; Py::Bool({ let t2420 = py::len(&py::getitem(&v_best.clone(), &Py::Int(0i64))?)?; if py::ge(&t2419, &t2420)? { true } else { false } }) }).truthy() {
                return Ok(Py::None);
            }
            v_live = { let mut out = vec![]; for t2421 in py::iter(&(v_rem.clone()))? { let mut v_v: Py; 
v_v = t2421;
if (py::getitem(&v_rem.clone(), &v_v.clone())?).truthy() { out.push((v_v.clone(), py::set(&{ let mut out = vec![]; for t2422 in py::iter(&(py::getitem(&v_rem.clone(), &v_v.clone())?))? { let mut v_u: Py; 
v_u = t2422;
out.push(v_u.clone()); } Py::list(out) })?)); }} Py::dict(out) };
            if (py::not(&v_live.clone())).truthy() {
                py::setitem(&v_best.clone(), &Py::Int(0i64), py::set(&v_cover.clone())?)?;
                return Ok(Py::None);
            }
            v_v = py::minmax_key(&[v_live.clone()], &{ let v_adj = v_adj.clone(); let v_best = v_best.clone(); let v_cover = v_cover.clone(); let v_edges = v_edges.clone(); let v_live = v_live.clone(); let v_nodes = v_nodes.clone(); let v_rec = v_rec.clone(); let v_rem = v_rem.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::len(&py::getitem(&v_live.clone(), &v_x.clone())?)?, py::neg(&py::method(&v_nodes.clone(), "index", &[v_x.clone()])?)?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, true)?;
            if ({ let t2423 = py::len(&py::getitem(&v_live.clone(), &v_v.clone())?)?; Py::Bool({ let t2424 = Py::Int(1i64); if py::eq(&t2423, &t2424) { true } else { false } }) }).truthy() {
                v_u = (match (py::getitem(&v_live.clone(), &v_v.clone())?).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) });
                v_take = Py::list(vec![v_u.clone()]);
            } else {
                v_take = Py::None;
            }
            for t2425 in py::iter(&((if (v_take.clone()).truthy() { Py::list(vec![v_take.clone()]) } else { Py::list(vec![Py::list(vec![v_v.clone()]), py::sorted_key(&py::getitem(&v_live.clone(), &v_v.clone())?, &{ let o = v_nodes.clone().clone(); py::func(move |a: &[Py]| -> R { py::method(&o, "index", a) }) }, (Py::Bool(false)).truthy())?]) })))? {
                v_choice = t2425;
                v_r2 = { let mut out = vec![]; for t2426 in py::iter(&(py::method(&v_live.clone(), "items", &[])?))? { let mut v_s: Py; let mut v_x: Py; 
let t2427 = (t2426).items()?;
if t2427.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2427[0].clone();
v_s = t2427[1].clone();
out.push((v_x.clone(), py::set(&v_s.clone())?)); } Py::dict(out) };
                for t2428 in py::iter(&(v_choice.clone()))? {
                    v_w = t2428;
                    for t2429 in py::iter(&(py::method(&v_r2.clone(), "pop", &[v_w.clone(), py::set(&Py::list(vec![]))?])?))? {
                        v_x = t2429;
                        let _ = py::method(&py::method(&v_r2.clone(), "get", &[v_x.clone(), py::set(&Py::list(vec![]))?])?, "discard", &[v_w.clone()])?;
                    }
                }
                let _ = py::call(&v_rec.clone(), &[py::bitor(&v_cover.clone(), &py::set(&v_choice.clone())?)?, v_r2.clone()])?;
            }
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec2418, &v_rec);
    let _ = py::call(&v_rec.clone(), &[py::set(&Py::list(vec![]))?, { let mut out = vec![]; for t2430 in py::iter(&(v_nodes.clone()))? { let mut v_v: Py; 
v_v = t2430;
out.push((v_v.clone(), py::set(&py::getitem(&v_adj.clone(), &v_v.clone())?)?)); } Py::dict(out) }])?;
    v_cov = { let mut out = vec![]; for t2431 in py::iter(&(v_nodes.clone()))? { let mut v_v: Py; 
v_v = t2431;
if ({ let t2432 = v_v.clone(); Py::Bool({ let t2433 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; if py::contains(&t2433, &t2432)? { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("min_cover_size"), py::float(&py::len(&v_cov.clone())?)?), (Py::str("cover_example"), v_cov.clone()), (Py::str("max_independent_set_size"), py::float(&py::sub(&py::len(&v_nodes.clone())?, &py::len(&v_cov.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_wagner_whitin(mut v_demand: Py, mut v_K: Py, mut v_h: Py) -> R {
    let mut v_Kf: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_arg: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_cost: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_hf: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    let _ = f__finite(Py::tuple(vec![v_K.clone(), v_h.clone()]))?;
    v_d = { let mut out = vec![]; for t3666 in py::iter(&(v_demand.clone()))? { let mut v_v: Py; 
v_v = t3666;
out.push(f__fr(v_v.clone())?); } Py::list(out) };
    v_T = py::len(&v_d.clone())?;
    if ({ let t3671 = { let t3667 = v_T.clone(); Py::Bool({ let t3668 = Py::Int(0i64); if py::eq(&t3667, &t3668) { true } else { false } }) }; if t3671.truthy() { t3671 } else { { let t3669 = v_T.clone(); Py::Bool({ let t3670 = Py::Int(52i64); if py::gt(&t3669, &t3670)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give 1 to 52 periods of demand"))));
    }
    if ({ let t3680 = Py::Bool(({ let mut out = vec![]; for t3672 in py::iter(&(v_d.clone()))? { let mut v_v: Py; 
v_v = t3672;
out.push({ let t3673 = v_v.clone(); Py::Bool({ let t3674 = Py::Int(0i64); if py::lt(&t3673, &t3674)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t3680.truthy() { t3680 } else { { let t3679 = { let t3675 = v_K.clone(); Py::Bool({ let t3676 = Py::Int(0i64); if py::lt(&t3675, &t3676)? { true } else { false } }) }; if t3679.truthy() { t3679 } else { { let t3677 = v_h.clone(); Py::Bool({ let t3678 = Py::Int(0i64); if py::lt(&t3677, &t3678)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("demands, the setup cost and the holding cost must be >= 0"))));
    }
    let t3681 = (Py::tuple(vec![f__fr(v_K.clone())?, f__fr(v_h.clone())?])).items()?;
    if t3681.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Kf = t3681[0].clone();
    v_hf = t3681[1].clone();
    let t3682 = (Py::tuple(vec![py::add(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &py::mul(&Py::list(vec![Py::None]), &v_T.clone())?)?, py::mul(&Py::list(vec![Py::None]), &py::add(&v_T.clone(), &Py::Int(1i64))?)?])).items()?;
    if t3682.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_best = t3682[0].clone();
    v_arg = t3682[1].clone();
    for t3683 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_T.clone(), &Py::Int(1i64))?).i()?]) {
        v_t = t3683;
        if ({ let t3688 = { let t3684 = py::getitem(&v_d.clone(), &py::sub(&v_t.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t3685 = Py::Int(0i64); if py::eq(&t3684, &t3685) { true } else { false } }) }; if !t3688.truthy() { t3688 } else { { let t3686 = py::getitem(&v_best.clone(), &py::sub(&v_t.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t3687 = Py::None; if !py::is(&t3686, &t3687) { true } else { false } }) } } }).truthy() {
            let t3689 = (Py::tuple(vec![py::getitem(&v_best.clone(), &py::sub(&v_t.clone(), &Py::Int(1i64))?)?, Py::None])).items()?;
            if t3689.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            py::setitem(&v_best.clone(), &v_t.clone(), t3689[0].clone())?;
            py::setitem(&v_arg.clone(), &v_t.clone(), t3689[1].clone())?;
        }
        for t3690 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_t.clone(), &Py::Int(1i64))?).i()?]) {
            v_s = t3690;
            v_cost = py::add(&py::add(&py::getitem(&v_best.clone(), &py::sub(&v_s.clone(), &Py::Int(1i64))?)?, &v_Kf.clone())?, &py::mul(&v_hf.clone(), &py::sum(&{ let mut out = vec![]; for t3691 in py::range_iter(&[(v_s.clone()).i()?, (py::add(&v_t.clone(), &Py::Int(1i64))?).i()?]) { let mut v_u: Py; 
v_u = t3691;
out.push(py::mul(&py::sub(&v_u.clone(), &v_s.clone())?, &py::getitem(&v_d.clone(), &py::sub(&v_u.clone(), &Py::Int(1i64))?)?)?); } Py::list(out) }, None)?)?)?;
            if ({ let t3696 = { let t3692 = py::getitem(&v_best.clone(), &v_t.clone())?; Py::Bool({ let t3693 = Py::None; if py::is(&t3692, &t3693) { true } else { false } }) }; if t3696.truthy() { t3696 } else { { let t3694 = v_cost.clone(); Py::Bool({ let t3695 = py::getitem(&v_best.clone(), &v_t.clone())?; if py::lt(&t3694, &t3695)? { true } else { false } }) } } }).truthy() {
                let t3697 = (Py::tuple(vec![v_cost.clone(), v_s.clone()])).items()?;
                if t3697.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                py::setitem(&v_best.clone(), &v_t.clone(), t3697[0].clone())?;
                py::setitem(&v_arg.clone(), &v_t.clone(), t3697[1].clone())?;
            }
        }
    }
    let t3698 = (Py::tuple(vec![py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &v_T.clone())?, v_T.clone()])).items()?;
    if t3698.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_q = t3698[0].clone();
    v_t = t3698[1].clone();
    while ({ let t3701 = v_t.clone(); Py::Bool({ let t3702 = Py::Int(0i64); if py::gt(&t3701, &t3702)? { true } else { false } }) }).truthy() {
        v_s = py::getitem(&v_arg.clone(), &v_t.clone())?;
        if ({ let t3699 = v_s.clone(); Py::Bool({ let t3700 = Py::None; if py::is(&t3699, &t3700) { true } else { false } }) }).truthy() {
            v_t = py::sub(&v_t, &Py::Int(1i64))?;
            continue;
        }
        py::setitem(&v_q.clone(), &py::sub(&v_s.clone(), &Py::Int(1i64))?, py::sum(&py::slice(&v_d.clone(), Some((py::sub(&v_s.clone(), &Py::Int(1i64))?).i()?), Some((v_t.clone()).i()?), None)?, None)?)?;
        v_t = py::sub(&v_s.clone(), &Py::Int(1i64))?;
    }
    return Ok(Py::dict(vec![(Py::str("min_cost"), py::float(&py::getitem(&v_best.clone(), &v_T.clone())?)?), (Py::str("order_quantities_example"), f__fl(v_q.clone())?), (Py::str("orders"), py::float(&py::sum(&{ let mut out = vec![]; for t3704 in py::iter(&(v_q.clone()))? { let mut v_v: Py; 
v_v = t3704;
if ({ let t3705 = v_v.clone(); Py::Bool({ let t3706 = Py::Int(0i64); if py::gt(&t3705, &t3706)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_weighted_product(mut v_X: Py, mut v_w: Py, mut v_types: Py) -> R {
    let mut v_N: Py = Py::None;
    let t3759 = (f__mcda_in(v_X.clone(), v_w.clone(), v_types.clone())?).items()?;
    if t3759.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_X = t3759[0].clone();
    v_w = t3759[1].clone();
    v_types = t3759[2].clone();
    v_N = f__linear_norm(v_X.clone(), v_types.clone())?;
    return Ok(f__mcda_out({ let mut out = vec![]; for t3772 in py::iter(&(v_N.clone()))? { let mut v_r: Py; 
v_r = t3772;
out.push(py::math1("exp", &py::fsum(&{ let mut out = vec![]; for t3773 in py::iter(&(py::zip(&[v_w.clone(), v_r.clone()])?))? { let mut v_v: Py; let mut v_wj: Py; 
let t3774 = (t3773).items()?;
if t3774.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_wj = t3774[0].clone();
v_v = t3774[1].clone();
out.push(py::mul(&v_wj.clone(), &py::math1("log", &v_v.clone())?)?); } Py::list(out) })?)?); } Py::list(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_weighted_sum(mut v_X: Py, mut v_w: Py, mut v_types: Py) -> R {
    let mut v_N: Py = Py::None;
    let t3822 = (f__mcda_in(v_X.clone(), v_w.clone(), v_types.clone())?).items()?;
    if t3822.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_X = t3822[0].clone();
    v_w = t3822[1].clone();
    v_types = t3822[2].clone();
    v_N = f__linear_norm(v_X.clone(), v_types.clone())?;
    return Ok(f__mcda_out({ let mut out = vec![]; for t3835 in py::iter(&(v_N.clone()))? { let mut v_r: Py; 
v_r = t3835;
out.push(py::fsum(&{ let mut out = vec![]; for t3836 in py::iter(&(py::zip(&[v_w.clone(), v_r.clone()])?))? { let mut v_v: Py; let mut v_wj: Py; 
let t3837 = (t3836).items()?;
if t3837.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_wj = t3837[0].clone();
v_v = t3837[1].clone();
out.push(py::mul(&v_wj.clone(), &v_v.clone())?); } Py::list(out) })?); } Py::list(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_wspt(mut v_p: Py, mut v_w: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_seq: Py = Py::None;
    let _ = f__jobs(v_p.clone(), Py::str("processing times"))?;
    if ({ let t3863 = { let t3858 = py::len(&v_w.clone())?; Py::Bool({ let t3859 = py::len(&v_p.clone())?; if py::ne(&t3858, &t3859) { true } else { false } }) }; if t3863.truthy() { t3863 } else { Py::Bool(({ let mut out = vec![]; for t3860 in py::iter(&(v_w.clone()))? { let mut v_v: Py; 
v_v = t3860;
out.push({ let t3861 = v_v.clone(); Py::Bool({ let t3862 = Py::Int(0i64); if py::le(&t3861, &t3862)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give one positive weight per job"))));
    }
    v_seq = py::sorted_key(&Py::list((py::range_iter(&[(py::len(&v_p.clone())?).i()?])).into_iter().collect::<Vec<Py>>()), &{ let v_p = v_p.clone(); let v_w = v_w.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_j: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::truediv(&f__fr(py::getitem(&v_p.clone(), &v_j.clone())?)?, &f__fr(py::getitem(&v_w.clone(), &v_j.clone())?)?)?, v_j.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    v_C = f__completion(v_seq.clone(), v_p.clone())?;
    return Ok(Py::dict(vec![(Py::str("sequence"), { let mut out = vec![]; for t3866 in py::iter(&(v_seq.clone()))? { let mut v_j: Py; 
v_j = t3866;
out.push(py::add(&v_j.clone(), &Py::Int(1i64))?); } Py::list(out) }), (Py::str("weighted_completion_time"), py::float(&py::sum(&{ let mut out = vec![]; for t3867 in py::iter(&(v_seq.clone()))? { let mut v_j: Py; 
v_j = t3867;
out.push(py::mul(&f__fr(py::getitem(&v_w.clone(), &v_j.clone())?)?, &py::getitem(&v_C.clone(), &v_j.clone())?)?); } Py::list(out) }, None)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_zero_sum_2x2(mut v_A: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_M: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    v_M = f__payoff(v_A.clone(), Py::str("payoff matrix"), Py::Int(12i64))?;
    if ({ let t3895 = { let t3891 = py::len(&v_M.clone())?; Py::Bool({ let t3892 = Py::Int(2i64); if py::ne(&t3891, &t3892) { true } else { false } }) }; if t3895.truthy() { t3895 } else { { let t3893 = py::len(&py::getitem(&v_M.clone(), &Py::Int(0i64))?)?; Py::Bool({ let t3894 = Py::Int(2i64); if py::ne(&t3893, &t3894) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give a 2 x 2 payoff matrix (use the general zero-sum game capability otherwise)"))));
    }
    let t3896 = (v_M.clone()).items()?;
    if t3896.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    let t3897 = (t3896[0].clone()).items()?;
    if t3897.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3897[0].clone();
    v_b = t3897[1].clone();
    let t3898 = (t3896[1].clone()).items()?;
    if t3898.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t3898[0].clone();
    v_d = t3898[1].clone();
    let t3899 = (Py::tuple(vec![py::minmax(&[py::minmax(&[v_a.clone(), v_b.clone()], false)?, py::minmax(&[v_c.clone(), v_d.clone()], false)?], true)?, py::minmax(&[py::minmax(&[v_a.clone(), v_c.clone()], true)?, py::minmax(&[v_b.clone(), v_d.clone()], true)?], false)?])).items()?;
    if t3899.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t3899[0].clone();
    v_hi = t3899[1].clone();
    if ({ let t3900 = v_lo.clone(); Py::Bool({ let t3901 = v_hi.clone(); if py::eq(&t3900, &t3901) { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("value"), py::float(&v_lo.clone())?), (Py::str("saddle_point"), Py::str("yes"))]));
    }
    v_D = py::sub(&py::sub(&py::add(&v_a.clone(), &v_d.clone())?, &v_b.clone())?, &v_c.clone())?;
    return Ok(Py::dict(vec![(Py::str("value"), py::float(&py::truediv(&py::sub(&py::mul(&v_a.clone(), &v_d.clone())?, &py::mul(&v_b.clone(), &v_c.clone())?)?, &v_D.clone())?)?), (Py::str("p_row1"), py::float(&py::truediv(&py::sub(&v_d.clone(), &v_c.clone())?, &v_D.clone())?)?), (Py::str("q_col1"), py::float(&py::truediv(&py::sub(&v_d.clone(), &v_b.clone())?, &v_D.clone())?)?), (Py::str("saddle_point"), Py::str("no"))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_zero_sum_game(mut v_A: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    v_M = f__payoff(v_A.clone(), Py::str("payoff matrix"), Py::Int(12i64))?;
    let t4100 = (f__game_value(v_M.clone())?).items()?;
    if t4100.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_v = t4100[0].clone();
    v_x = t4100[1].clone();
    v_y = t4100[2].clone();
    return Ok(Py::dict(vec![(Py::str("value"), py::float(&v_v.clone())?), (Py::str("row_strategy_example"), f__fl(v_x.clone())?), (Py::str("col_strategy_example"), f__fl(v_y.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1248(mut v_i: Py, mut v_o: Py) -> R {
    return Ok((if ({ let t1258 = { let t1249 = py::sum(&{ let mut out = vec![]; for t1251 in py::range_iter(&[(py::len(&py::getitem(&v_i.clone(), &Py::str("values"))?)?).i()?]) { let mut v_k: Py; 
v_k = t1251;
out.push(py::mul(&py::getitem(&py::getitem(&v_i.clone(), &Py::str("weights"))?, &v_k.clone())?, &py::getitem(&py::getitem(&v_o.clone(), &Py::str("copies_example"))?, &v_k.clone())?)?); } Py::list(out) }, None)?; Py::Bool({ let t1250 = py::getitem(&v_i.clone(), &Py::str("capacity"))?; if py::gt(&t1249, &t1250)? { true } else { false } }) }; if t1258.truthy() { t1258 } else { Py::Bool(({ let mut out = vec![]; for t1252 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("copies_example"))?))? { let mut v_x: Py; 
v_x = t1252;
out.push({ let t1257 = { let t1253 = v_x.clone(); Py::Bool({ let t1254 = Py::Int(0i64); if py::lt(&t1253, &t1254)? { true } else { false } }) }; if t1257.truthy() { t1257 } else { { let t1255 = v_x.clone(); Py::Bool({ let t1256 = py::int(&v_x.clone())?; if py::ne(&t1255, &t1256) { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() { Py::Float(1.0) } else { py::minmax(&[f__rel(py::getitem(&v_o.clone(), &Py::str("max_value"))?, py::sum(&{ let mut out = vec![]; for t1259 in py::range_iter(&[(py::len(&py::getitem(&v_i.clone(), &Py::str("values"))?)?).i()?]) { let mut v_k: Py; 
v_k = t1259;
out.push(py::mul(&py::getitem(&py::getitem(&v_i.clone(), &Py::str("values"))?, &v_k.clone())?, &py::getitem(&py::getitem(&v_o.clone(), &Py::str("copies_example"))?, &v_k.clone())?)?); } Py::list(out) }, None)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("max_value"))?, py::getitem(&f_knapsack_unbounded(py::getitem(&v_i.clone(), &Py::str("values"))?, py::getitem(&v_i.clone(), &Py::str("weights"))?, py::getitem(&v_i.clone(), &Py::str("capacity"))?)?, &Py::str("max_value"))?)?], true)? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1472(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::minmax(&[py::abs(&py::sub(&py::sub(&Py::Int(1i64), &py::normal_dist("cdf", &Py::Float(0.0), &Py::Float(1.0), &[py::truediv(&py::sub(&py::getitem(&v_o.clone(), &Py::str("protection_level"))?, &py::getitem(&v_i.clone(), &Py::str("mu"))?)?, &py::getitem(&v_i.clone(), &Py::str("sigma"))?)?])?)?, &py::truediv(&py::getitem(&v_i.clone(), &Py::str("p2"))?, &py::getitem(&v_i.clone(), &Py::str("p1"))?)?)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("protection_level"))?, py::add(&py::getitem(&v_i.clone(), &Py::str("mu"))?, &py::mul(&py::getitem(&v_o.clone(), &Py::str("z"))?, &py::getitem(&v_i.clone(), &Py::str("sigma"))?)?)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1843(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::minmax(&[f__rel(py::getitem(&v_o.clone(), &Py::str("lam_eff"))?, py::mul(&py::getitem(&v_i.clone(), &Py::str("lam"))?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("N"))?, &py::getitem(&v_o.clone(), &Py::str("L"))?)?)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("L"))?, py::mul(&py::getitem(&v_o.clone(), &Py::str("lam_eff"))?, &py::getitem(&v_o.clone(), &Py::str("W"))?)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("Lq"))?, py::mul(&py::getitem(&v_o.clone(), &Py::str("lam_eff"))?, &py::getitem(&v_o.clone(), &Py::str("Wq"))?)?)?, f__rel(py::sub(&py::getitem(&v_o.clone(), &Py::str("L"))?, &py::getitem(&v_o.clone(), &Py::str("Lq"))?)?, py::truediv(&py::getitem(&v_o.clone(), &Py::str("lam_eff"))?, &py::getitem(&v_i.clone(), &Py::str("mu"))?)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("working"))?, py::sub(&py::getitem(&v_i.clone(), &Py::str("N"))?, &py::getitem(&v_o.clone(), &Py::str("L"))?)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_202(mut v_C: Py, mut v__m: Py) -> R {
    return Ok(f__assignment(v_C.clone(), v__m.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2086(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::minmax(&[f__rel(py::mul(&py::mul(&Py::Int(2i64), &py::sub(&Py::Int(1i64), &py::getitem(&v_o.clone(), &Py::str("rho"))?)?)?, &py::getitem(&v_o.clone(), &Py::str("Lq"))?)?, py::add(&py::mul(&py::pow(&py::getitem(&v_i.clone(), &Py::str("lam"))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("sd"))?, &Py::Int(2i64))?)?, &py::pow(&py::getitem(&v_o.clone(), &Py::str("rho"))?, &Py::Int(2i64))?)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("Lq"))?, py::mul(&py::getitem(&v_i.clone(), &Py::str("lam"))?, &py::getitem(&v_o.clone(), &Py::str("Wq"))?)?)?, f__rel(py::sub(&py::getitem(&v_o.clone(), &Py::str("W"))?, &py::getitem(&v_o.clone(), &Py::str("Wq"))?)?, py::getitem(&v_i.clone(), &Py::str("ES"))?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("L"))?, py::mul(&py::getitem(&v_i.clone(), &Py::str("lam"))?, &py::getitem(&v_o.clone(), &Py::str("W"))?)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("rho"))?, py::mul(&py::getitem(&v_i.clone(), &Py::str("lam"))?, &py::getitem(&v_i.clone(), &Py::str("ES"))?)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_253(mut v_i: Py, mut v_o: Py, mut v__m: Py) -> R {
    return Ok(f__verify_assign(v_i.clone(), v_o.clone(), v__m.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_265(mut v_C: Py, mut v__m: Py) -> R {
    return Ok(f__assignment(v_C.clone(), v__m.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2685(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::minmax(&[f__rel(py::getitem(&v_o.clone(), &Py::str("expected_profit"))?, f__nv_disc_profit(py::getitem(&v_i.clone(), &Py::str("p"))?, py::getitem(&v_i.clone(), &Py::str("c"))?, py::method(&v_i.clone(), "get", &[Py::str("s"), Py::Float(0.0)])?, py::getitem(&v_i.clone(), &Py::str("demands"))?, py::getitem(&v_i.clone(), &Py::str("probs"))?, py::getitem(&v_o.clone(), &Py::str("Q"))?)?)?, py::truediv(&py::minmax(&[{ let mut out = vec![]; for t2688 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("demands"))?))? { let mut v_d: Py; 
v_d = t2688;
out.push(py::minmax(&[Py::Float(0.0), py::sub(&f__nv_disc_profit(py::getitem(&v_i.clone(), &Py::str("p"))?, py::getitem(&v_i.clone(), &Py::str("c"))?, py::method(&v_i.clone(), "get", &[Py::str("s"), Py::Float(0.0)])?, py::getitem(&v_i.clone(), &Py::str("demands"))?, py::getitem(&v_i.clone(), &Py::str("probs"))?, v_d.clone())?, &py::getitem(&v_o.clone(), &Py::str("expected_profit"))?)?], true)?); } Py::list(out) }], true)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_o.clone(), &Py::str("expected_profit"))?)?], true)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2697(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::minmax(&[py::abs(&py::sub(&py::normal_dist("cdf", &Py::Float(0.0), &Py::Float(1.0), &[py::truediv(&py::sub(&py::getitem(&v_o.clone(), &Py::str("Q"))?, &py::getitem(&v_i.clone(), &Py::str("mu"))?)?, &py::getitem(&v_i.clone(), &Py::str("sigma"))?)?])?, &py::truediv(&py::sub(&py::getitem(&v_i.clone(), &Py::str("p"))?, &py::getitem(&v_i.clone(), &Py::str("c"))?)?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("p"))?, &py::method(&v_i.clone(), "get", &[Py::str("s"), Py::Float(0.0)])?)?)?)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("expected_profit"))?, py::sub(&py::mul(&py::sub(&py::getitem(&v_i.clone(), &Py::str("p"))?, &py::method(&v_i.clone(), "get", &[Py::str("s"), Py::Float(0.0)])?)?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("mu"))?, &py::getitem(&v_o.clone(), &Py::str("expected_shortage"))?)?)?, &py::mul(&py::sub(&py::getitem(&v_i.clone(), &Py::str("c"))?, &py::method(&v_i.clone(), "get", &[Py::str("s"), Py::Float(0.0)])?)?, &py::getitem(&v_o.clone(), &Py::str("Q"))?)?)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2707(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::minmax(&[f__rel(py::truediv(&py::sub(&py::getitem(&v_o.clone(), &Py::str("Q"))?, &py::getitem(&v_i.clone(), &Py::str("lo"))?)?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("hi"))?, &py::getitem(&v_i.clone(), &Py::str("lo"))?)?)?, py::truediv(&py::sub(&py::getitem(&v_i.clone(), &Py::str("p"))?, &py::getitem(&v_i.clone(), &Py::str("c"))?)?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("p"))?, &py::method(&v_i.clone(), "get", &[Py::str("s"), Py::Float(0.0)])?)?)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("expected_profit"))?, py::sub(&py::mul(&py::sub(&py::getitem(&v_i.clone(), &Py::str("p"))?, &py::method(&v_i.clone(), "get", &[Py::str("s"), Py::Float(0.0)])?)?, &py::sub(&py::truediv(&py::add(&py::getitem(&v_i.clone(), &Py::str("lo"))?, &py::getitem(&v_i.clone(), &Py::str("hi"))?)?, &Py::Int(2i64))?, &py::truediv(&py::pow(&py::sub(&py::getitem(&v_i.clone(), &Py::str("hi"))?, &py::getitem(&v_o.clone(), &Py::str("Q"))?)?, &Py::Int(2i64))?, &py::mul(&Py::Int(2i64), &py::sub(&py::getitem(&v_i.clone(), &Py::str("hi"))?, &py::getitem(&v_i.clone(), &Py::str("lo"))?)?)?)?)?)?, &py::mul(&py::sub(&py::getitem(&v_i.clone(), &Py::str("c"))?, &py::method(&v_i.clone(), "get", &[Py::str("s"), Py::Float(0.0)])?)?, &py::getitem(&v_o.clone(), &Py::str("Q"))?)?)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2716(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::minmax(&[py::truediv(&py::abs(&py::add(&py::sub(&py::mul(&Py::Int(12i64), &py::pow(&py::getitem(&v_o.clone(), &Py::str("cut"))?, &Py::Int(2i64))?)?, &py::mul(&py::mul(&Py::Int(4i64), &py::add(&py::getitem(&v_i.clone(), &Py::str("L"))?, &py::getitem(&v_i.clone(), &Py::str("W"))?)?)?, &py::getitem(&v_o.clone(), &Py::str("cut"))?)?)?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("L"))?, &py::getitem(&v_i.clone(), &Py::str("W"))?)?)?)?, &py::minmax(&[Py::Float(1.0), py::mul(&py::getitem(&v_i.clone(), &Py::str("L"))?, &py::getitem(&v_i.clone(), &Py::str("W"))?)?], true)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("max_volume"))?, py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("cut"))?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("L"))?, &py::mul(&Py::Int(2i64), &py::getitem(&v_o.clone(), &Py::str("cut"))?)?)?)?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("W"))?, &py::mul(&Py::Int(2i64), &py::getitem(&v_o.clone(), &Py::str("cut"))?)?)?)?)?, (if ({ let t2717 = Py::Int(0i64); Py::Bool({ let t2718 = py::getitem(&v_o.clone(), &Py::str("cut"))?; if py::lt(&t2717, &t2718)? { { let t2719 = py::truediv(&py::minmax(&[py::getitem(&v_i.clone(), &Py::str("L"))?, py::getitem(&v_i.clone(), &Py::str("W"))?], false)?, &Py::Int(2i64))?; if py::lt(&t2718, &t2719)? { true } else { false } } } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) })], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2737(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::minmax(&[f__rel(py::getitem(&v_o.clone(), &Py::str("order_up_to_level"))?, py::add(&py::mul(&py::getitem(&v_i.clone(), &Py::str("d"))?, &py::add(&py::getitem(&v_i.clone(), &Py::str("R"))?, &py::getitem(&v_i.clone(), &Py::str("L"))?)?)?, &py::getitem(&v_o.clone(), &Py::str("safety_stock"))?)?)?, py::abs(&py::sub(&py::normal_dist("cdf", &Py::Float(0.0), &Py::Float(1.0), &[py::getitem(&v_o.clone(), &Py::str("z"))?])?, &py::getitem(&v_i.clone(), &Py::str("SL"))?)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("safety_stock"))?, py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("z"))?, &py::getitem(&v_i.clone(), &Py::str("sigma"))?)?, &py::math1("sqrt", &py::add(&py::getitem(&v_i.clone(), &Py::str("R"))?, &py::getitem(&v_i.clone(), &Py::str("L"))?)?)?)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2941(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::minmax(&[py::truediv(&py::abs(&py::sub(&py::sub(&py::getitem(&v_i.clone(), &Py::str("a"))?, &py::mul(&py::mul(&Py::Int(2i64), &py::getitem(&v_i.clone(), &Py::str("b"))?)?, &py::getitem(&v_o.clone(), &Py::str("quantity"))?)?)?, &py::getitem(&v_i.clone(), &Py::str("cv"))?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("a"))?)?], true)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("profit"))?, py::sub(&py::sub(&py::mul(&py::sub(&py::getitem(&v_i.clone(), &Py::str("a"))?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("b"))?, &py::getitem(&v_o.clone(), &Py::str("quantity"))?)?)?, &py::getitem(&v_o.clone(), &Py::str("quantity"))?)?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("cv"))?, &py::getitem(&v_o.clone(), &Py::str("quantity"))?)?)?, &py::method(&v_i.clone(), "get", &[Py::str("F_"), Py::Float(0.0)])?)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2999(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::minmax(&[py::truediv(&py::abs(&py::add(&py::mul(&py::mul(&Py::Int(2i64), &py::getitem(&v_i.clone(), &Py::str("a"))?)?, &py::getitem(&v_o.clone(), &Py::str("x_opt"))?)?, &py::getitem(&v_i.clone(), &Py::str("b"))?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("b"))?)?, py::abs(&py::mul(&py::mul(&Py::Int(2i64), &py::getitem(&v_i.clone(), &Py::str("a"))?)?, &py::getitem(&v_o.clone(), &Py::str("x_opt"))?)?)?], true)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("f_opt"))?, py::add(&py::add(&py::mul(&py::getitem(&v_i.clone(), &Py::str("a"))?, &py::pow(&py::getitem(&v_o.clone(), &Py::str("x_opt"))?, &Py::Int(2i64))?)?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("b"))?, &py::getitem(&v_o.clone(), &Py::str("x_opt"))?)?)?, &py::getitem(&v_i.clone(), &Py::str("c"))?)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3014(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::minmax(&[f__rel(py::getitem(&v_o.clone(), &Py::str("reorder_point"))?, py::add(&py::mul(&py::getitem(&v_i.clone(), &Py::str("d"))?, &py::getitem(&v_i.clone(), &Py::str("L"))?)?, &py::getitem(&v_o.clone(), &Py::str("safety_stock"))?)?)?, py::abs(&py::sub(&py::normal_dist("cdf", &Py::Float(0.0), &Py::Float(1.0), &[py::getitem(&v_o.clone(), &Py::str("z"))?])?, &py::getitem(&v_i.clone(), &Py::str("SL"))?)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("safety_stock"))?, py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("z"))?, &py::getitem(&v_i.clone(), &Py::str("sigma"))?)?, &py::math1("sqrt", &py::getitem(&v_i.clone(), &Py::str("L"))?)?)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3024(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::minmax(&[py::truediv(&py::abs(&py::sub(&py::getitem(&v_i.clone(), &Py::str("alpha"))?, &py::mul(&py::mul(&Py::Int(2i64), &py::getitem(&v_i.clone(), &Py::str("beta"))?)?, &py::getitem(&v_o.clone(), &Py::str("price"))?)?)?)?, &py::minmax(&[Py::Float(1.0), py::getitem(&v_i.clone(), &Py::str("alpha"))?], true)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("revenue"))?, py::mul(&py::getitem(&v_o.clone(), &Py::str("price"))?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("alpha"))?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("beta"))?, &py::getitem(&v_o.clone(), &Py::str("price"))?)?)?)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3057(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_pick: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok((if (Py::Bool(({ let mut out = vec![]; for t3058 in py::range_iter(&[(py::len(&py::getitem(&py::getitem(&v_i.clone(), &Py::str("sets"))?, &Py::Int(0i64))?)?).i()?]) { let mut v_j: Py; 
v_j = t3058;
out.push(py::not(&Py::Bool(({ let mut out = vec![]; for t3059 in py::iter(&(v_pick.clone()))? { let mut v_k: Py; 
v_k = t3059;
out.push(py::getitem(&py::getitem(&py::getitem(&v_i.clone(), &Py::str("sets"))?, &v_k.clone())?, &v_j.clone())?); } Py::list(out) }).items()?.iter().any(|x| x.truthy())))); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() { Py::Float(1.0) } else { py::minmax(&[f__rel(py::getitem(&v_o.clone(), &Py::str("min_cost"))?, py::sum(&{ let mut out = vec![]; for t3060 in py::iter(&(v_pick.clone()))? { let mut v_k: Py; 
v_k = t3060;
out.push(py::getitem(&py::getitem(&v_i.clone(), &Py::str("costs"))?, &v_k.clone())?); } Py::list(out) }, None)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("min_cost"))?, py::getitem(&f_set_cover(py::getitem(&v_i.clone(), &Py::str("sets"))?, py::getitem(&v_i.clone(), &Py::str("costs"))?)?, &Py::str("min_cost"))?)?], true)? }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[{ let mut out = vec![]; for t3061 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("sets_example"))?))? { let mut v_x: Py; 
v_x = t3061;
out.push(py::sub(&py::int(&py::round(&v_x.clone(), None)?)?, &Py::Int(1i64))?); } Py::list(out) }])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_316(mut v_i: Py, mut v_o: Py, mut v__m: Py) -> R {
    return Ok(f__verify_assign(v_i.clone(), v_o.clone(), v__m.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3165(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__verify_seq(v_i.clone(), v_o.clone(), { let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_i: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_j: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::getitem(&py::getitem(&v_i.clone(), &Py::str("p"))?, &v_j.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, { let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_i: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_seq: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::float(&py::sum(&py::method(&f__completion(v_seq.clone(), py::getitem(&v_i.clone(), &Py::str("p"))?)?, "values", &[])?, None)?)?, &py::len(&py::getitem(&v_i.clone(), &Py::str("p"))?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::str("mean_flow_time"), Py::str("p"))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3298(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_ref: Py = Py::None;
    let t3313: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_ref = py::getitem(&f__topsis_check(py::getitem(&v_i.clone(), &Py::str("X"))?, py::getitem(&v_i.clone(), &Py::str("w"))?, py::getitem(&v_i.clone(), &Py::str("types"))?)?, &Py::str("scores"))?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3313 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3314 = py::len(&py::getitem(&v_o.clone(), &Py::str("scores"))?)?; Py::Bool({ let t3315 = py::len(&v_ref.clone())?; if py::ne(&t3314, &t3315) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::minmax(&[py::add(&{ let mut out = vec![]; for t3316 in py::iter(&(py::zip(&[py::getitem(&v_o.clone(), &Py::str("scores"))?, v_ref.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3317 = (t3316).items()?;
if t3317.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3317[0].clone();
v_b = t3317[1].clone();
out.push(f__rr(v_a.clone(), v_b.clone())?); } Py::list(out) }, &Py::list(vec![f__rr(py::getitem(&v_o.clone(), &Py::str("best_score"))?, py::minmax(&[py::getitem(&v_o.clone(), &Py::str("scores"))?], true)?)?, (if ({ let t3320 = py::int(&py::getitem(&v_o.clone(), &Py::str("best"))?)?; Py::Bool({ let t3321 = f__best(py::getitem(&v_o.clone(), &Py::str("scores"))?)?; if py::eq(&t3320, &t3321) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) })]))?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3775(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_ref: Py = Py::None;
    let t3780: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_ref = py::getitem(&s_3776(py::getitem(&v_i.clone(), &Py::str("X"))?, py::getitem(&v_i.clone(), &Py::str("w"))?, py::getitem(&v_i.clone(), &Py::str("types"))?)?, &Py::str("scores"))?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3780 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3781 = py::len(&py::getitem(&v_o.clone(), &Py::str("scores"))?)?; Py::Bool({ let t3782 = py::len(&v_ref.clone())?; if py::ne(&t3781, &t3782) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::minmax(&[py::add(&{ let mut out = vec![]; for t3783 in py::iter(&(py::zip(&[py::getitem(&v_o.clone(), &Py::str("scores"))?, v_ref.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3784 = (t3783).items()?;
if t3784.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3784[0].clone();
v_b = t3784[1].clone();
out.push(f__rr(v_a.clone(), v_b.clone())?); } Py::list(out) }, &Py::list(vec![f__rr(py::getitem(&v_o.clone(), &Py::str("best_score"))?, py::minmax(&[py::getitem(&v_o.clone(), &Py::str("scores"))?], true)?)?, (if ({ let t3787 = py::int(&py::getitem(&v_o.clone(), &Py::str("best"))?)?; Py::Bool({ let t3788 = f__best(py::getitem(&v_o.clone(), &Py::str("scores"))?)?; if py::eq(&t3787, &t3788) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) })]))?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3776(mut v_X: Py, mut v_w: Py, mut v_t: Py) -> R {
    return Ok(Py::dict(vec![(Py::str("scores"), { let mut out = vec![]; for t3777 in py::iter(&({ let mut v: Vec<Py> = vec![]; v.extend((py::slice(&f__mcda_in(v_X.clone(), v_w.clone(), v_t.clone())?, Some((Py::Int(0i64)).i()?), Some((Py::Int(3i64)).i()?), Some((Py::Int(2i64)).i()?))?).items()?); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "X") { Some(x) => x, None => { return Err(PyErr::Type("_linear_norm() missing argument X".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "types") { Some(x) => x, None => { return Err(PyErr::Type("_linear_norm() missing argument types".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("_linear_norm() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("_linear_norm() got an unexpected keyword argument".into())) } f__linear_norm(a0, a1)? }))? { let mut v_r: Py; 
v_r = t3777;
out.push(py::prod(&{ let mut out = vec![]; for t3778 in py::iter(&(py::zip(&[py::getitem(&f__mcda_in(v_X.clone(), v_w.clone(), v_t.clone())?, &Py::Int(1i64))?, v_r.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3779 = (t3778).items()?;
if t3779.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3779[0].clone();
v_b = t3779[1].clone();
out.push(py::pow(&v_b.clone(), &v_a.clone())?); } Py::list(out) })?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3838(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_ref: Py = Py::None;
    let t3843: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_ref = py::getitem(&s_3839(py::getitem(&v_i.clone(), &Py::str("X"))?, py::getitem(&v_i.clone(), &Py::str("w"))?, py::getitem(&v_i.clone(), &Py::str("types"))?)?, &Py::str("scores"))?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3843 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3844 = py::len(&py::getitem(&v_o.clone(), &Py::str("scores"))?)?; Py::Bool({ let t3845 = py::len(&v_ref.clone())?; if py::ne(&t3844, &t3845) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::minmax(&[py::add(&{ let mut out = vec![]; for t3846 in py::iter(&(py::zip(&[py::getitem(&v_o.clone(), &Py::str("scores"))?, v_ref.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3847 = (t3846).items()?;
if t3847.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3847[0].clone();
v_b = t3847[1].clone();
out.push(f__rr(v_a.clone(), v_b.clone())?); } Py::list(out) }, &Py::list(vec![f__rr(py::getitem(&v_o.clone(), &Py::str("best_score"))?, py::minmax(&[py::getitem(&v_o.clone(), &Py::str("scores"))?], true)?)?, (if ({ let t3850 = py::int(&py::getitem(&v_o.clone(), &Py::str("best"))?)?; Py::Bool({ let t3851 = f__best(py::getitem(&v_o.clone(), &Py::str("scores"))?)?; if py::eq(&t3850, &t3851) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) })]))?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3839(mut v_X: Py, mut v_w: Py, mut v_t: Py) -> R {
    return Ok(Py::dict(vec![(Py::str("scores"), { let mut out = vec![]; for t3840 in py::iter(&({ let mut v: Vec<Py> = vec![]; v.extend((py::slice(&f__mcda_in(v_X.clone(), v_w.clone(), v_t.clone())?, Some((Py::Int(0i64)).i()?), Some((Py::Int(3i64)).i()?), Some((Py::Int(2i64)).i()?))?).items()?); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "X") { Some(x) => x, None => { return Err(PyErr::Type("_linear_norm() missing argument X".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "types") { Some(x) => x, None => { return Err(PyErr::Type("_linear_norm() missing argument types".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("_linear_norm() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("_linear_norm() got an unexpected keyword argument".into())) } f__linear_norm(a0, a1)? }))? { let mut v_r: Py; 
v_r = t3840;
out.push(py::fsum(&{ let mut out = vec![]; for t3841 in py::iter(&(py::zip(&[py::getitem(&f__mcda_in(v_X.clone(), v_w.clone(), v_t.clone())?, &Py::Int(1i64))?, v_r.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3842 = (t3841).items()?;
if t3842.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3842[0].clone();
v_b = t3842[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3868(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__verify_seq(v_i.clone(), v_o.clone(), { let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_i: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_j: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&f__fr(py::getitem(&py::getitem(&v_i.clone(), &Py::str("p"))?, &v_j.clone())?)?, &f__fr(py::getitem(&py::getitem(&v_i.clone(), &Py::str("w"))?, &v_j.clone())?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, { let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_i: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_seq: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::float(&py::sum(&{ let mut out = vec![]; for t3877 in py::iter(&(py::method(&f__completion(v_seq.clone(), py::getitem(&v_i.clone(), &Py::str("p"))?)?, "items", &[])?))? { let mut v_c: Py; let mut v_j: Py; 
let t3878 = (t3877).items()?;
if t3878.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_j = t3878[0].clone();
v_c = t3878[1].clone();
out.push(py::mul(&f__fr(py::getitem(&py::getitem(&v_i.clone(), &Py::str("w"))?, &v_j.clone())?)?, &v_c.clone())?); } Py::list(out) }, None)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::str("weighted_completion_time"), Py::str("p"))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_516(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_i.clone(), &Py::str("p"))?, &py::getitem(&v_o.clone(), &Py::str("quantity"))?)?, py::add(&py::getitem(&v_i.clone(), &Py::str("F_"))?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("v"))?, &py::getitem(&v_o.clone(), &Py::str("quantity"))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_522(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::minmax(&[py::truediv(&py::abs(&py::sub(&py::mul(&py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?, &py::getitem(&v_o.clone(), &Py::str("radius"))?)?, &py::truediv(&py::mul(&Py::Int(2i64), &py::getitem(&v_i.clone(), &Py::str("V"))?)?, &py::pow(&py::getitem(&v_o.clone(), &Py::str("radius"))?, &Py::Int(2i64))?)?)?)?, &py::minmax(&[Py::Float(1.0), py::mul(&py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?, &py::getitem(&v_o.clone(), &Py::str("radius"))?)?], true)?)?, f__rel(py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("height"))?, &Py::Float(std::f64::consts::PI))?, &py::pow(&py::getitem(&v_o.clone(), &Py::str("radius"))?, &Py::Int(2i64))?)?, py::getitem(&v_i.clone(), &Py::str("V"))?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("min_area"))?, py::add(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &py::pow(&py::getitem(&v_o.clone(), &Py::str("radius"))?, &Py::Int(2i64))?)?, &py::mul(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &py::getitem(&v_o.clone(), &Py::str("radius"))?)?, &py::getitem(&v_o.clone(), &Py::str("height"))?)?)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_630(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__verify_seq(v_i.clone(), v_o.clone(), { let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_i: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_j: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::getitem(&py::getitem(&v_i.clone(), &Py::str("d"))?, &v_j.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, { let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_i: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_seq: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::float(&py::minmax(&[{ let mut out = vec![]; for t639 in py::iter(&(py::method(&f__completion(v_seq.clone(), py::getitem(&v_i.clone(), &Py::str("p"))?)?, "items", &[])?))? { let mut v_c: Py; let mut v_j: Py; 
let t640 = (t639).items()?;
if t640.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_j = t640[0].clone();
v_c = t640[1].clone();
out.push(py::sub(&v_c.clone(), &f__fr(py::getitem(&py::getitem(&v_i.clone(), &Py::str("d"))?, &v_j.clone())?)?)?); } Py::list(out) }], true)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::str("max_lateness"), Py::str("p"))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_652(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::minmax(&[f__rel(py::truediv(&py::mul(&py::getitem(&v_i.clone(), &Py::str("D"))?, &py::getitem(&v_i.clone(), &Py::str("S"))?)?, &py::getitem(&v_o.clone(), &Py::str("Q"))?)?, py::truediv(&py::mul(&py::getitem(&v_i.clone(), &Py::str("H"))?, &py::getitem(&v_o.clone(), &Py::str("Q"))?)?, &Py::Int(2i64))?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("total_cost"))?, py::add(&py::truediv(&py::mul(&py::getitem(&v_i.clone(), &Py::str("D"))?, &py::getitem(&v_i.clone(), &Py::str("S"))?)?, &py::getitem(&v_o.clone(), &Py::str("Q"))?)?, &py::truediv(&py::mul(&py::getitem(&v_i.clone(), &Py::str("H"))?, &py::getitem(&v_o.clone(), &Py::str("Q"))?)?, &Py::Int(2i64))?)?)?, f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("orders_per_period"))?, &py::getitem(&v_o.clone(), &Py::str("Q"))?)?, py::getitem(&v_i.clone(), &Py::str("D"))?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_717(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::minmax(&[f__rel(py::truediv(&py::mul(&py::getitem(&v_i.clone(), &Py::str("D"))?, &py::getitem(&v_i.clone(), &Py::str("S"))?)?, &py::getitem(&v_o.clone(), &Py::str("Q"))?)?, py::truediv(&py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("H"))?, &py::getitem(&v_o.clone(), &Py::str("Q"))?)?, &py::sub(&Py::Int(1i64), &py::truediv(&py::getitem(&v_i.clone(), &Py::str("D"))?, &py::getitem(&v_i.clone(), &Py::str("P"))?)?)?)?, &Py::Int(2i64))?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("total_cost"))?, py::add(&py::truediv(&py::mul(&py::getitem(&v_i.clone(), &Py::str("D"))?, &py::getitem(&v_i.clone(), &Py::str("S"))?)?, &py::getitem(&v_o.clone(), &Py::str("Q"))?)?, &py::truediv(&py::mul(&py::getitem(&v_i.clone(), &Py::str("H"))?, &py::getitem(&v_o.clone(), &Py::str("max_inventory"))?)?, &Py::Int(2i64))?)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("max_inventory"))?, py::mul(&py::getitem(&v_o.clone(), &Py::str("Q"))?, &py::sub(&Py::Int(1i64), &py::truediv(&py::getitem(&v_i.clone(), &Py::str("D"))?, &py::getitem(&v_i.clone(), &Py::str("P"))?)?)?)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_850(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_op: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok((if ({ let t855 = py::not(&v_op.clone()); if t855.truthy() { t855 } else { Py::Bool(({ let mut out = vec![]; for t851 in py::iter(&(v_op.clone()))? { let mut v_k: Py; 
v_k = t851;
out.push(py::not(&{ let t852 = Py::Int(0i64); Py::Bool({ let t853 = v_k.clone(); if py::le(&t852, &t853)? { { let t854 = py::len(&py::getitem(&v_i.clone(), &Py::str("fixed"))?)?; if py::lt(&t853, &t854)? { true } else { false } } } else { false } }) })); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() { Py::Float(1.0) } else { py::minmax(&[f__rel(py::getitem(&v_o.clone(), &Py::str("min_cost"))?, py::add(&py::sum(&{ let mut out = vec![]; for t856 in py::iter(&(v_op.clone()))? { let mut v_k: Py; 
v_k = t856;
out.push(py::getitem(&py::getitem(&v_i.clone(), &Py::str("fixed"))?, &v_k.clone())?); } Py::list(out) }, None)?, &py::sum(&{ let mut out = vec![]; for t857 in py::range_iter(&[(py::len(&py::getitem(&py::getitem(&v_i.clone(), &Py::str("cost"))?, &Py::Int(0i64))?)?).i()?]) { let mut v_j: Py; 
v_j = t857;
out.push(py::minmax(&[{ let mut out = vec![]; for t858 in py::iter(&(v_op.clone()))? { let mut v_k: Py; 
v_k = t858;
out.push(py::getitem(&py::getitem(&py::getitem(&v_i.clone(), &Py::str("cost"))?, &v_k.clone())?, &v_j.clone())?); } Py::list(out) }], false)?); } Py::list(out) }, None)?)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("min_cost"))?, py::getitem(&f_facility_location(py::getitem(&v_i.clone(), &Py::str("fixed"))?, py::getitem(&v_i.clone(), &Py::str("cost"))?)?, &Py::str("min_cost"))?)?], true)? }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[{ let mut out = vec![]; for t859 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("open_example"))?))? { let mut v_x: Py; 
v_x = t859;
out.push(py::sub(&py::int(&py::round(&v_x.clone(), None)?)?, &Py::Int(1i64))?); } Py::list(out) }])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_865(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::minmax(&[f__rel(py::getitem(&v_o.clone(), &Py::str("max_area"))?, py::mul(&py::getitem(&v_o.clone(), &Py::str("width"))?, &py::getitem(&v_o.clone(), &Py::str("length"))?)?)?, f__rel((if (py::method(&v_i.clone(), "get", &[Py::str("wall")])?).truthy() { py::add(&py::mul(&Py::Int(2i64), &py::getitem(&v_o.clone(), &Py::str("width"))?)?, &py::getitem(&v_o.clone(), &Py::str("length"))?)? } else { py::mul(&Py::Int(2i64), &py::add(&py::getitem(&v_o.clone(), &Py::str("width"))?, &py::getitem(&v_o.clone(), &Py::str("length"))?)?)? }), py::getitem(&v_i.clone(), &Py::str("L"))?)?, py::truediv(&py::abs(&(if (py::method(&v_i.clone(), "get", &[Py::str("wall")])?).truthy() { py::sub(&py::getitem(&v_i.clone(), &Py::str("L"))?, &py::mul(&Py::Int(4i64), &py::getitem(&v_o.clone(), &Py::str("width"))?)?)? } else { py::sub(&py::getitem(&v_o.clone(), &Py::str("width"))?, &py::getitem(&v_o.clone(), &Py::str("length"))?)? }))?, &py::minmax(&[Py::Float(1.0), py::getitem(&v_i.clone(), &Py::str("L"))?], true)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}
