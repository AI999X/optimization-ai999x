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
    "lp_duality_sensitivity",
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
        "lp_duality_sensitivity" => {
            let r = f_lp_sensitivity(match v.get("sense") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("relations") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_lp_sensitivity(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2159(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2402(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3001(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "newsvendor_normal" => {
            let r = f_newsvendor_normal(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3013(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "newsvendor_uniform" => {
            let r = f_newsvendor_uniform(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lo") { Some(x) => py::from_val(x), None => { return None } }, match v.get("hi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3023(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "open_box_max_volume" => {
            let r = f_open_box(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("W") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3032(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "order_up_to_level" => {
            let r = f_order_up_to(match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("SL") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3053(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3257(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pure_nash_equilibria" => {
            let r = f_pure_nash(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_pure_ne(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "quadratic_optimum" => {
            let r = f_quadratic_optimum(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("goal") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("optimum")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3315(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "reorder_point" => {
            let r = f_reorder_point(match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("SL") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3330(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "revenue_max_linear_demand" => {
            let r = f_revenue_max_linear(match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3340(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "set_cover" => {
            let r = f_set_cover(match v.get("sets") { Some(x) => py::from_val(x), None => { return None } }, match v.get("costs") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3373(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "shortest_path" => {
            let r = f_shortest_path(match v.get("edges") { Some(x) => py::from_val(x), None => { return None } }, match v.get("source") { Some(x) => py::from_val(x), None => { return None } }, match v.get("target") { Some(x) => py::from_val(x), None => { return None } }, match v.get("directed") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_sp(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "spt_mean_flow_time" => {
            let r = f_spt(match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3481(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "steepest_descent_step" => {
            let r = f_steepest_step(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_sd(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "topsis" => {
            let r = f_topsis(match v.get("X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } }, match v.get("types") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3614(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4091(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "weighted_sum_model" => {
            let r = f_weighted_sum(match v.get("X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } }, match v.get("types") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4154(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "wspt_weighted_completion" => {
            let r = f_wspt(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4184(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
    for t3117 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t3117;
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
    for t3398 in py::iter(&(v_E.clone()))? {
        let t3399 = (t3398).items()?;
        if t3399.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_u = t3399[0].clone();
        v_v = t3399[1].clone();
        v_w = t3399[2].clone();
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
    v_arcs = py::add(&{ let mut out = vec![]; for t3441 in py::iter(&(v_E.clone()))? { let mut v_u: Py; let mut v_v: Py; let mut v_w: Py; 
let t3442 = (t3441).items()?;
if t3442.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_u = t3442[0].clone();
v_v = t3442[1].clone();
v_w = t3442[2].clone();
out.push(Py::tuple(vec![v_u.clone(), v_v.clone(), v_w.clone()])); } Py::list(out) }, &(if (v_directed.clone()).truthy() { Py::list(vec![]) } else { { let mut out = vec![]; for t3443 in py::iter(&(v_E.clone()))? { let mut v_u: Py; let mut v_v: Py; let mut v_w: Py; 
let t3444 = (t3443).items()?;
if t3444.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_u = t3444[0].clone();
v_v = t3444[1].clone();
v_w = t3444[2].clone();
out.push(Py::tuple(vec![v_v.clone(), v_u.clone(), v_w.clone()])); } Py::list(out) } }))?;
    v_dist = Py::dict(vec![(v_s.clone(), py::fraction(&[Py::Int(0i64)])?)]);
    for t3445 in py::range_iter(&[(py::len(&v_nodes.clone())?).i()?]) {
        v__ = t3445;
        v_changed = Py::Bool(false);
        for t3446 in py::iter(&(v_arcs.clone()))? {
            let t3455 = (t3446).items()?;
            if t3455.len() != 3 { return Err(PyErr::Value("unpack".into())); }
            v_u = t3455[0].clone();
            v_v = t3455[1].clone();
            v_w = t3455[2].clone();
            if ({ let t3454 = { let t3447 = v_u.clone(); Py::Bool({ let t3448 = v_dist.clone(); if py::contains(&t3448, &t3447)? { true } else { false } }) }; if !t3454.truthy() { t3454 } else { { let t3453 = { let t3449 = v_v.clone(); Py::Bool({ let t3450 = v_dist.clone(); if !py::contains(&t3450, &t3449)? { true } else { false } }) }; if t3453.truthy() { t3453 } else { { let t3451 = py::add(&py::getitem(&v_dist.clone(), &v_u.clone())?, &v_w.clone())?; Py::Bool({ let t3452 = py::getitem(&v_dist.clone(), &v_v.clone())?; if py::lt(&t3451, &t3452)? { true } else { false } }) } } } } }).truthy() {
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
    return Ok(py::add(&(match ('g4146: { let mut out = vec![]; for t4147 in py::iter(&(py::enumerate(&v_scores.clone(), 0)?))? { let mut v_k: Py; let mut v_v: Py; 
let t4148 = (t4147).items()?;
if t4148.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t4148[0].clone();
v_v = t4148[1].clone();
if ({ let t4149 = v_v.clone(); Py::Bool({ let t4150 = py::sub(&v_top.clone(), &py::mul(&Py::Float(1e-12), &py::minmax(&[Py::Float(1.0), py::abs(&v_top.clone())?], true)?)?)?; if py::ge(&t4149, &t4150)? { true } else { false } }) }).truthy() { out.push(v_k.clone()); break 'g4146 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) }), &Py::Int(1i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bimatrix(mut v_A: Py, mut v_B: Py, mut v_max_dim: Py) -> R {
    let mut v_MA: Py = Py::None;
    let mut v_MB: Py = Py::None;
    let t3270 = (Py::tuple(vec![f__payoff(v_A.clone(), Py::str("row payoff matrix"), v_max_dim.clone())?, f__payoff(v_B.clone(), Py::str("column payoff matrix"), v_max_dim.clone())?])).items()?;
    if t3270.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_MA = t3270[0].clone();
    v_MB = t3270[1].clone();
    if ({ let t3275 = { let t3271 = py::len(&v_MA.clone())?; Py::Bool({ let t3272 = py::len(&v_MB.clone())?; if py::ne(&t3271, &t3272) { true } else { false } }) }; if t3275.truthy() { t3275 } else { { let t3273 = py::len(&py::getitem(&v_MA.clone(), &Py::Int(0i64))?)?; Py::Bool({ let t3274 = py::len(&py::getitem(&v_MB.clone(), &Py::Int(0i64))?)?; if py::ne(&t3273, &t3274) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the row and column payoff matrices must have the same shape"))));
    }
    return Ok(Py::tuple(vec![v_MA.clone(), v_MB.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__binv(mut v_Bm: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_col: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_piv: Py = Py::None;
    let mut v_pv: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_n = py::len(&v_Bm.clone())?;
    v_M = { let mut out = vec![]; for t1602 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t1602;
out.push(py::add(&Py::list((py::getitem(&v_Bm.clone(), &v_i.clone())?).items()?), &{ let mut out = vec![]; for t1603 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t1603;
out.push(py::fraction(&[py::int(&{ let t1604 = v_k.clone(); Py::Bool({ let t1605 = v_i.clone(); if py::eq(&t1604, &t1605) { true } else { false } }) })?])?); } Py::list(out) })?); } Py::list(out) };
    for t1606 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_col = t1606;
        v_piv = py::neg(&Py::Int(1i64))?;
        for t1607 in py::range_iter(&[(v_col.clone()).i()?, (v_n.clone()).i()?]) {
            v_r = t1607;
            if ({ let t1608 = py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_col.clone())?; Py::Bool({ let t1609 = Py::Int(0i64); if py::ne(&t1608, &t1609) { true } else { false } }) }).truthy() {
                v_piv = v_r.clone();
                break;
            }
        }
        if ({ let t1610 = v_piv.clone(); Py::Bool({ let t1611 = Py::Int(0i64); if py::lt(&t1610, &t1611)? { true } else { false } }) }).truthy() {
            return Ok(Py::None);
        }
        let t1612 = (Py::tuple(vec![py::getitem(&v_M.clone(), &v_piv.clone())?, py::getitem(&v_M.clone(), &v_col.clone())?])).items()?;
        if t1612.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_M.clone(), &v_col.clone(), t1612[0].clone())?;
        py::setitem(&v_M.clone(), &v_piv.clone(), t1612[1].clone())?;
        v_pv = py::getitem(&py::getitem(&v_M.clone(), &v_col.clone())?, &v_col.clone())?;
        py::setitem(&v_M.clone(), &v_col.clone(), { let mut out = vec![]; for t1613 in py::iter(&(py::getitem(&v_M.clone(), &v_col.clone())?))? { let mut v_v: Py; 
v_v = t1613;
out.push(py::truediv(&v_v.clone(), &v_pv.clone())?); } Py::list(out) })?;
        for t1614 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_r = t1614;
            if ({ let t1619 = { let t1615 = v_r.clone(); Py::Bool({ let t1616 = v_col.clone(); if py::ne(&t1615, &t1616) { true } else { false } }) }; if !t1619.truthy() { t1619 } else { { let t1617 = py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_col.clone())?; Py::Bool({ let t1618 = Py::Int(0i64); if py::ne(&t1617, &t1618) { true } else { false } }) } } }).truthy() {
                v_f = py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_col.clone())?;
                py::setitem(&v_M.clone(), &v_r.clone(), { let mut out = vec![]; for t1620 in py::iter(&(py::zip(&[py::getitem(&v_M.clone(), &v_r.clone())?, py::getitem(&v_M.clone(), &v_col.clone())?])?))? { let mut v_a: Py; let mut v_bb: Py; 
let t1621 = (t1620).items()?;
if t1621.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1621[0].clone();
v_bb = t1621[1].clone();
out.push(py::sub(&v_a.clone(), &py::mul(&v_f.clone(), &v_bb.clone())?)?); } Py::list(out) })?;
            }
        }
    }
    return Ok({ let mut out = vec![]; for t1622 in py::iter(&(v_M.clone()))? { let mut v_row: Py; 
v_row = t1622;
out.push(py::slice(&v_row.clone(), Some((v_n.clone()).i()?), None, None)?); } Py::list(out) });
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
    if ({ let t4257 = v_sense.clone(); Py::Bool({ let t4258 = Py::tuple(vec![Py::str("maximize"), Py::str("minimize")]); if !py::contains(&t4258, &t4257)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the objective sense must be maximize or minimize"))));
    }
    let t4259 = (Py::tuple(vec![py::len(&v_c.clone())?, py::len(&v_A.clone())?])).items()?;
    if t4259.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t4259[0].clone();
    v_m = t4259[1].clone();
    if ({ let t4264 = { let t4260 = v_n.clone(); Py::Bool({ let t4261 = Py::Int(0i64); if py::eq(&t4260, &t4261) { true } else { false } }) }; if t4264.truthy() { t4264 } else { { let t4262 = v_m.clone(); Py::Bool({ let t4263 = Py::Int(0i64); if py::eq(&t4262, &t4263) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give at least one variable and one constraint"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t4265 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t4265;
out.push({ let t4266 = py::len(&v_r.clone())?; Py::Bool({ let t4267 = v_n.clone(); if py::ne(&t4266, &t4267) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["every constraint row must have ".to_string(), py::format(&v_n.clone(), "")?, " coefficients (one per variable)".to_string()].concat()))));
    }
    if ({ let t4268 = py::len(&v_b.clone())?; Py::Bool({ let t4269 = v_m.clone(); if py::ne(&t4268, &t4269) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["the right-hand side must have ".to_string(), py::format(&v_m.clone(), "")?, " entries (one per constraint)".to_string()].concat()))));
    }
    if ({ let t4275 = { let t4270 = py::len(&v_rel.clone())?; Py::Bool({ let t4271 = v_m.clone(); if py::ne(&t4270, &t4271) { true } else { false } }) }; if t4275.truthy() { t4275 } else { Py::Bool(({ let mut out = vec![]; for t4272 in py::iter(&(v_rel.clone()))? { let mut v_r: Py; 
v_r = t4272;
out.push({ let t4273 = v_r.clone(); Py::Bool({ let t4274 = Py::tuple(vec![Py::str("<="), Py::str(">="), Py::str("=")]); if !py::contains(&t4274, &t4273)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("each constraint relation must be <=, >= or ="))));
    }
    if ({ let t4280 = { let t4276 = v_n.clone(); Py::Bool({ let t4277 = Py::Int(12i64); if py::gt(&t4276, &t4277)? { true } else { false } }) }; if t4280.truthy() { t4280 } else { { let t4278 = v_m.clone(); Py::Bool({ let t4279 = Py::Int(12i64); if py::gt(&t4278, &t4279)? { true } else { false } }) } } }).truthy() {
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
    let t4180 = (Py::tuple(vec![py::fraction(&[Py::Int(0i64)])?, Py::dict(vec![])])).items()?;
    if t4180.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_t = t4180[0].clone();
    v_C = t4180[1].clone();
    for t4181 in py::iter(&(v_seq.clone()))? {
        v_j = t4181;
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
    let t3400 = (Py::tuple(vec![Py::dict(vec![(v_s.clone(), py::fraction(&[Py::Int(0i64)])?)]), py::set(&Py::list(vec![]))?, Py::list(vec![Py::tuple(vec![py::fraction(&[Py::Int(0i64)])?, Py::Int(0i64), v_s.clone()])]), Py::Int(1i64)])).items()?;
    if t3400.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_dist = t3400[0].clone();
    v_done = t3400[1].clone();
    v_h = t3400[2].clone();
    v_cnt = t3400[3].clone();
    while (v_h.clone()).truthy() {
        let t3401 = (py::heapq("heappop", &[v_h.clone()])?).items()?;
        if t3401.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_d = t3401[0].clone();
        v__ = t3401[1].clone();
        v_u = t3401[2].clone();
        if ({ let t3402 = v_u.clone(); Py::Bool({ let t3403 = v_done.clone(); if py::contains(&t3403, &t3402)? { true } else { false } }) }).truthy() {
            continue;
        }
        let _ = py::method(&v_done.clone(), "add", &[v_u.clone()])?;
        for t3404 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? {
            let t3410 = (t3404).items()?;
            if t3410.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_v = t3410[0].clone();
            v_w = t3410[1].clone();
            if ({ let t3409 = { let t3405 = v_v.clone(); Py::Bool({ let t3406 = v_dist.clone(); if !py::contains(&t3406, &t3405)? { true } else { false } }) }; if t3409.truthy() { t3409 } else { { let t3407 = py::add(&v_d.clone(), &v_w.clone())?; Py::Bool({ let t3408 = py::getitem(&v_dist.clone(), &v_v.clone())?; if py::lt(&t3407, &t3408)? { true } else { false } }) } } }).truthy() {
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
    for t2902 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_c.clone(), &Py::Int(1i64))?).i()?]) {
        v_k = t2902;
        v_B = py::truediv(&py::mul(&v_a.clone(), &v_B.clone())?, &py::add(&v_k.clone(), &py::mul(&v_a.clone(), &v_B.clone())?)?)?;
    }
    return Ok(v_B.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__finite(mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t3979 in py::iter(&(v_vals.clone()))? {
        v_v = t3979;
        if (py::not(&{ let t3981 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3981.truthy() { t3981 } else { { let t3980 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t3980.truthy() { t3980 } else { py::math1("isfinite", &v_v.clone())? } } } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("every input must be a finite number"))));
        }
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fl(mut v_x: Py) -> R {
    return Ok({ let mut out = vec![]; for t4417 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t4417;
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
    let t4253 = (Py::tuple(vec![py::len(&v_M.clone())?, py::len(&py::getitem(&v_M.clone(), &Py::Int(0i64))?)?])).items()?;
    if t4253.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t4253[0].clone();
    v_n = t4253[1].clone();
    v_k = py::sub(&Py::Int(1i64), &py::minmax(&[{ let mut out = vec![]; for t4254 in py::iter(&(v_M.clone()))? { let mut v_r: Py; 
v_r = t4254;
out.push(py::minmax(&[v_r.clone()], false)?); } Py::list(out) }], false)?)?;
    v_S = { let mut out = vec![]; for t4255 in py::iter(&(v_M.clone()))? { let mut v_r: Py; 
v_r = t4255;
out.push({ let mut out = vec![]; for t4256 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t4256;
out.push(py::add(&v_v.clone(), &v_k.clone())?); } Py::list(out) }); } Py::list(out) };
    v_ru = f__lp(Py::str("minimize"), py::mul(&Py::list(vec![Py::Int(1i64)]), &v_m.clone())?, { let mut out = vec![]; for t4411 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t4411;
out.push({ let mut out = vec![]; for t4412 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t4412;
out.push(py::getitem(&py::getitem(&v_S.clone(), &v_i.clone())?, &v_j.clone())?); } Py::list(out) }); } Py::list(out) }, py::mul(&Py::list(vec![Py::Int(1i64)]), &v_n.clone())?, py::mul(&Py::list(vec![Py::str(">=")]), &v_n.clone())?, Py::Bool(false))?;
    v_cw = f__lp(Py::str("maximize"), py::mul(&Py::list(vec![Py::Int(1i64)]), &v_n.clone())?, { let mut out = vec![]; for t4413 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t4413;
out.push(Py::list((py::getitem(&v_S.clone(), &v_i.clone())?).items()?)); } Py::list(out) }, py::mul(&Py::list(vec![Py::Int(1i64)]), &v_m.clone())?, py::mul(&Py::list(vec![Py::str("<=")]), &v_m.clone())?, Py::Bool(false))?;
    v_vs = py::truediv(&Py::Int(1i64), &py::getitem(&v_ru.clone(), &Py::str("value"))?)?;
    return Ok(Py::tuple(vec![py::sub(&v_vs.clone(), &v_k.clone())?, { let mut out = vec![]; for t4414 in py::iter(&(py::getitem(&v_ru.clone(), &Py::str("x"))?))? { let mut v_u: Py; 
v_u = t4414;
out.push(py::mul(&v_u.clone(), &v_vs.clone())?); } Py::list(out) }, { let mut out = vec![]; for t4415 in py::iter(&(py::getitem(&v_cw.clone(), &Py::str("x"))?))? { let mut v_w: Py; 
v_w = t4415;
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
    v_g = { let mut out = vec![]; for t3972 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3972;
out.push(py::add(&py::add(&py::fsum(&{ let mut out = vec![]; for t3973 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3973;
out.push(py::mul(&py::float(&py::getitem(&py::getitem(&v_Q.clone(), &v_i.clone())?, &v_j.clone())?)?, &py::getitem(&v_x.clone(), &v_j.clone())?)?); } Py::list(out) })?, &py::float(&py::getitem(&v_c.clone(), &v_i.clone())?)?)?, &(if (v_extra.clone()).truthy() { py::getitem(&v_extra.clone(), &v_i.clone())? } else { Py::Float(0.0) }))?); } Py::list(out) };
    v_sc = py::mul(&py::minmax(&[py::add(&py::add(&Py::list(vec![Py::Float(1.0)]), &{ let mut out = vec![]; for t3974 in py::iter(&(v_Q.clone()))? { let mut v_r: Py; 
v_r = t3974;
for t3975 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t3975;
out.push(py::abs(&py::float(&v_v.clone())?)?); }} Py::list(out) })?, &{ let mut out = vec![]; for t3976 in py::iter(&(v_c.clone()))? { let mut v_v: Py; 
v_v = t3976;
out.push(py::abs(&py::float(&v_v.clone())?)?); } Py::list(out) })?], true)?, &py::minmax(&[py::add(&Py::list(vec![Py::Float(1.0)]), &{ let mut out = vec![]; for t3977 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t3977;
out.push(py::abs(&v_v.clone())?); } Py::list(out) })?], true)?)?;
    return Ok(py::truediv(&py::minmax(&[{ let mut out = vec![]; for t3978 in py::iter(&(v_g.clone()))? { let mut v_v: Py; 
v_v = t3978;
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
    if ({ let t4170 = py::not(&v_p.clone()); if t4170.truthy() { t4170 } else { { let t4168 = py::len(&v_p.clone())?; Py::Bool({ let t4169 = Py::Int(12i64); if py::gt(&t4168, &t4169)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["give between 1 and 12 ".to_string(), py::format(&v_name.clone(), "")?].concat()))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t4171 in py::iter(&(v_p.clone()))? { let mut v_v: Py; 
v_v = t4171;
out.push({ let t4172 = v_v.clone(); Py::Bool({ let t4173 = Py::Int(0i64); if py::lt(&t4172, &t4173)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
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
    let t4139 = (Py::tuple(vec![py::len(&v_X.clone())?, py::len(&py::getitem(&v_X.clone(), &Py::Int(0i64))?)?])).items()?;
    if t4139.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t4139[0].clone();
    v_k = t4139[1].clone();
    v_cols = { let mut out = vec![]; for t4140 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t4140;
out.push({ let mut out = vec![]; for t4141 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t4141;
out.push(py::getitem(&py::getitem(&v_X.clone(), &v_i.clone())?, &v_j.clone())?); } Py::list(out) }); } Py::list(out) };
    return Ok({ let mut out = vec![]; for t4142 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t4142;
out.push({ let mut out = vec![]; for t4143 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t4143;
out.push((if ({ let t4144 = py::getitem(&v_types.clone(), &v_j.clone())?; Py::Bool({ let t4145 = Py::Int(1i64); if py::eq(&t4144, &t4145) { true } else { false } }) }).truthy() { py::truediv(&py::getitem(&py::getitem(&v_X.clone(), &v_i.clone())?, &v_j.clone())?, &py::minmax(&[py::getitem(&v_cols.clone(), &v_j.clone())?], true)?)? } else { py::truediv(&py::minmax(&[py::getitem(&v_cols.clone(), &v_j.clone())?], false)?, &py::getitem(&py::getitem(&v_X.clone(), &v_i.clone())?, &v_j.clone())?)? })); } Py::list(out) }); } Py::list(out) });
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
    let t4296 = (f__standard(v_sense.clone(), v_c.clone(), v_A.clone(), v_b.clone(), v_rel.clone())?).items()?;
    if t4296.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_As = t4296[0].clone();
    v_bs = t4296[1].clone();
    v_cs = t4296[2].clone();
    let t4368 = (f__simplex_eq(v_As.clone(), v_bs.clone(), v_cs.clone(), Py::Int(20000i64))?).items()?;
    if t4368.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_status = t4368[0].clone();
    v_x = t4368[1].clone();
    v_y = t4368[2].clone();
    v_basis = t4368[3].clone();
    v_redundant = t4368[4].clone();
    if ({ let t4369 = v_status.clone(); Py::Bool({ let t4370 = Py::str("infeasible"); if py::eq(&t4369, &t4370) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the constraints are infeasible: no point satisfies all of them"))));
    }
    if ({ let t4371 = v_status.clone(); Py::Bool({ let t4372 = Py::str("unbounded"); if py::eq(&t4371, &t4372) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the objective is unbounded on the feasible region: there is no finite optimum"))));
    }
    let t4373 = (Py::tuple(vec![py::len(&v_c.clone())?, py::len(&v_A.clone())?])).items()?;
    if t4373.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t4373[0].clone();
    v_m = t4373[1].clone();
    v_zmin = py::sum(&{ let mut out = vec![]; for t4374 in py::iter(&(py::zip(&[v_cs.clone(), v_x.clone()])?))? { let mut v_ci: Py; let mut v_xi: Py; 
let t4375 = (t4374).items()?;
if t4375.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_ci = t4375[0].clone();
v_xi = t4375[1].clone();
out.push(py::mul(&v_ci.clone(), &v_xi.clone())?); } Py::list(out) }, None)?;
    v_out = Py::dict(vec![(Py::str("value"), (if ({ let t4376 = v_sense.clone(); Py::Bool({ let t4377 = Py::str("minimize"); if py::eq(&t4376, &t4377) { true } else { false } }) }).truthy() { v_zmin.clone() } else { py::neg(&v_zmin.clone())? })), (Py::str("x"), py::slice(&v_x.clone(), None, Some((v_n.clone()).i()?), None)?), (Py::str("p"), { let mut out = vec![]; for t4378 in py::iter(&(v_y.clone()))? { let mut v_yi: Py; 
v_yi = t4378;
out.push((if ({ let t4379 = v_sense.clone(); Py::Bool({ let t4380 = Py::str("minimize"); if py::eq(&t4379, &t4380) { true } else { false } }) }).truthy() { v_yi.clone() } else { py::neg(&v_yi.clone())? })); } Py::list(out) })]);
    if (v_certify.clone()).truthy() {
        v_N = py::len(&v_cs.clone())?;
        v_nonbasic = { let mut out = vec![]; for t4381 in py::range_iter(&[(v_N.clone()).i()?]) { let mut v_j: Py; 
v_j = t4381;
if ({ let t4382 = v_j.clone(); Py::Bool({ let t4383 = v_basis.clone(); if !py::contains(&t4383, &t4382)? { true } else { false } }) }).truthy() { out.push(v_j.clone()); }} Py::list(out) };
        let t4387 = (f__simplex_eq(py::add(&v_As.clone(), &Py::list(vec![v_cs.clone()]))?, py::add(&v_bs.clone(), &Py::list(vec![v_zmin.clone()]))?, { let mut out = vec![]; for t4384 in py::range_iter(&[(v_N.clone()).i()?]) { let mut v_j: Py; 
v_j = t4384;
out.push((if ({ let t4385 = v_j.clone(); Py::Bool({ let t4386 = v_nonbasic.clone(); if py::contains(&t4386, &t4385)? { true } else { false } }) }).truthy() { py::fraction(&[py::neg(&Py::Int(1i64))?])? } else { py::fraction(&[Py::Int(0i64)])? })); } Py::list(out) }, Py::Int(20000i64))?).items()?;
        if t4387.len() != 5 { return Err(PyErr::Value("unpack".into())); }
        v_st = t4387[0].clone();
        v_xa = t4387[1].clone();
        v__ = t4387[2].clone();
        v__ = t4387[3].clone();
        v__ = t4387[4].clone();
        py::setitem(&v_out.clone(), &Py::str("x_unique"), { let t4393 = { let t4388 = v_st.clone(); Py::Bool({ let t4389 = Py::str("optimal"); if py::eq(&t4388, &t4389) { true } else { false } }) }; if !t4393.truthy() { t4393 } else { { let t4390 = py::sum(&{ let mut out = vec![]; for t4392 in py::iter(&(v_nonbasic.clone()))? { let mut v_j: Py; 
v_j = t4392;
out.push(py::getitem(&v_xa.clone(), &v_j.clone())?); } Py::list(out) }, None)?; Py::Bool({ let t4391 = Py::Int(0i64); if py::eq(&t4390, &t4391) { true } else { false } }) } } })?;
        if (v_redundant.clone()).truthy() {
            py::setitem(&v_out.clone(), &Py::str("p_unique"), Py::Bool(false))?;
        } else {
            v_rows = Py::list(vec![]);
            for t4394 in py::range_iter(&[(v_N.clone()).i()?]) {
                v_j = t4394;
                let _ = py::method(&v_rows.clone(), "append", &[py::add(&py::add(&{ let mut out = vec![]; for t4395 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t4395;
out.push(py::getitem(&py::getitem(&v_As.clone(), &v_i.clone())?, &v_j.clone())?); } Py::list(out) }, &{ let mut out = vec![]; for t4396 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t4396;
out.push(py::neg(&py::getitem(&py::getitem(&v_As.clone(), &v_i.clone())?, &v_j.clone())?)?); } Py::list(out) })?, &{ let mut out = vec![]; for t4397 in py::range_iter(&[(v_N.clone()).i()?]) { let mut v_k: Py; 
v_k = t4397;
out.push(py::fraction(&[py::int(&{ let t4398 = v_k.clone(); Py::Bool({ let t4399 = v_j.clone(); if py::eq(&t4398, &t4399) { true } else { false } }) })?])?); } Py::list(out) })?])?;
            }
            let _ = py::method(&v_rows.clone(), "append", &[py::add(&py::add(&v_bs.clone(), &{ let mut out = vec![]; for t4400 in py::iter(&(v_bs.clone()))? { let mut v_v: Py; 
v_v = t4400;
out.push(py::neg(&v_v.clone())?); } Py::list(out) })?, &py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &v_N.clone())?)?])?;
            v_cost = py::add(&py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &py::mul(&Py::Int(2i64), &v_m.clone())?)?, &{ let mut out = vec![]; for t4401 in py::range_iter(&[(v_N.clone()).i()?]) { let mut v_j: Py; 
v_j = t4401;
out.push((if ({ let t4402 = v_j.clone(); Py::Bool({ let t4403 = v_basis.clone(); if py::contains(&t4403, &t4402)? { true } else { false } }) }).truthy() { py::fraction(&[py::neg(&Py::Int(1i64))?])? } else { py::fraction(&[Py::Int(0i64)])? })); } Py::list(out) })?;
            let t4404 = (f__simplex_eq(v_rows.clone(), py::add(&v_cs.clone(), &Py::list(vec![v_zmin.clone()]))?, v_cost.clone(), Py::Int(20000i64))?).items()?;
            if t4404.len() != 5 { return Err(PyErr::Value("unpack".into())); }
            v_st = t4404[0].clone();
            v_sol = t4404[1].clone();
            v__ = t4404[2].clone();
            v__ = t4404[3].clone();
            v__ = t4404[4].clone();
            py::setitem(&v_out.clone(), &Py::str("p_unique"), { let t4410 = { let t4405 = v_st.clone(); Py::Bool({ let t4406 = Py::str("optimal"); if py::eq(&t4405, &t4406) { true } else { false } }) }; if !t4410.truthy() { t4410 } else { { let t4407 = py::sum(&{ let mut out = vec![]; for t4409 in py::iter(&(v_basis.clone()))? { let mut v_j: Py; 
v_j = t4409;
out.push(py::getitem(&v_sol.clone(), &py::add(&py::mul(&Py::Int(2i64), &v_m.clone())?, &v_j.clone())?)?); } Py::list(out) }, None)?; Py::Bool({ let t4408 = Py::Int(0i64); if py::eq(&t4407, &t4408) { true } else { false } }) } } })?;
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
    v_lines = { let mut out = vec![]; for t2030 in py::iter(&(py::zip(&[{ let mut out = vec![]; for t2027 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t2027;
out.push({ let mut out = vec![]; for t2028 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t2028;
out.push(f__fr(v_v.clone())?); } Py::list(out) }); } Py::list(out) }, { let mut out = vec![]; for t2029 in py::iter(&(v_b.clone()))? { let mut v_v: Py; 
v_v = t2029;
out.push(f__fr(v_v.clone())?); } Py::list(out) }])?))? { let mut v_bi: Py; let mut v_row: Py; 
let t2031 = (t2030).items()?;
if t2031.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_row = t2031[0].clone();
v_bi = t2031[1].clone();
out.push(Py::tuple(vec![py::fraction(&[py::getitem(&v_row.clone(), &Py::Int(0i64))?])?, py::fraction(&[py::getitem(&v_row.clone(), &Py::Int(1i64))?])?, py::fraction(&[v_bi.clone()])?])); } Py::list(out) };
    v_lines = py::iadd(&v_lines, &Py::list(vec![Py::tuple(vec![py::fraction(&[Py::Int(1i64)])?, py::fraction(&[Py::Int(0i64)])?, py::fraction(&[Py::Int(0i64)])?]), Py::tuple(vec![py::fraction(&[Py::Int(0i64)])?, py::fraction(&[Py::Int(1i64)])?, py::fraction(&[Py::Int(0i64)])?])]))?;
    v_cons = { let mut out = vec![]; for t2032 in py::iter(&(py::zip(&[v_A.clone(), v_b.clone(), v_rel.clone()])?))? { let mut v_bi: Py; let mut v_r: Py; let mut v_rl: Py; 
let t2033 = (t2032).items()?;
if t2033.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_r = t2033[0].clone();
v_bi = t2033[1].clone();
v_rl = t2033[2].clone();
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
            if ({ let t2038 = { let t2034 = v_x.clone(); Py::Bool({ let t2035 = Py::Int(0i64); if py::lt(&t2034, &t2035)? { true } else { false } }) }; if t2038.truthy() { t2038 } else { { let t2036 = v_y.clone(); Py::Bool({ let t2037 = Py::Int(0i64); if py::lt(&t2036, &t2037)? { true } else { false } }) } } }).truthy() {
                return Ok(Py::Bool(false));
            }
            for t2039 in py::iter(&(v_cons.clone()))? {
                let t2057 = (t2039).items()?;
                if t2057.len() != 4 { return Err(PyErr::Value("unpack".into())); }
                v_a1 = t2057[0].clone();
                v_a2 = t2057[1].clone();
                v_bb = t2057[2].clone();
                v_rl = t2057[3].clone();
                v_v = py::add(&py::mul(&v_a1.clone(), &v_x.clone())?, &py::mul(&v_a2.clone(), &v_y.clone())?)?;
                if ({ let t2056 = { let t2044 = { let t2040 = v_rl.clone(); Py::Bool({ let t2041 = Py::str("<="); if py::eq(&t2040, &t2041) { true } else { false } }) }; if !t2044.truthy() { t2044 } else { { let t2042 = v_v.clone(); Py::Bool({ let t2043 = v_bb.clone(); if py::gt(&t2042, &t2043)? { true } else { false } }) } } }; if t2056.truthy() { t2056 } else { { let t2055 = { let t2049 = { let t2045 = v_rl.clone(); Py::Bool({ let t2046 = Py::str(">="); if py::eq(&t2045, &t2046) { true } else { false } }) }; if !t2049.truthy() { t2049 } else { { let t2047 = v_v.clone(); Py::Bool({ let t2048 = v_bb.clone(); if py::lt(&t2047, &t2048)? { true } else { false } }) } } }; if t2055.truthy() { t2055 } else { { let t2054 = { let t2050 = v_rl.clone(); Py::Bool({ let t2051 = Py::str("="); if py::eq(&t2050, &t2051) { true } else { false } }) }; if !t2054.truthy() { t2054 } else { { let t2052 = v_v.clone(); Py::Bool({ let t2053 = v_bb.clone(); if py::ne(&t2052, &t2053) { true } else { false } }) } } } } } } }).truthy() {
                    return Ok(Py::Bool(false));
                }
            }
            return Ok(Py::Bool(true));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_pts = py::set(&Py::list(vec![]))?;
    for t2058 in py::iter(&(py::itertools("combinations", &[v_lines.clone(), Py::Int(2i64)])?))? {
        let t2062 = (t2058).items()?;
        if t2062.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        let t2063 = (t2062[0].clone()).items()?;
        if t2063.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_a1 = t2063[0].clone();
        v_a2 = t2063[1].clone();
        v_b1 = t2063[2].clone();
        let t2064 = (t2062[1].clone()).items()?;
        if t2064.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_c1 = t2064[0].clone();
        v_c2 = t2064[1].clone();
        v_b2 = t2064[2].clone();
        v_det = py::sub(&py::mul(&v_a1.clone(), &v_c2.clone())?, &py::mul(&v_a2.clone(), &v_c1.clone())?)?;
        if ({ let t2059 = v_det.clone(); Py::Bool({ let t2060 = Py::Int(0i64); if py::eq(&t2059, &t2060) { true } else { false } }) }).truthy() {
            continue;
        }
        let t2061 = (Py::tuple(vec![py::truediv(&py::sub(&py::mul(&v_b1.clone(), &v_c2.clone())?, &py::mul(&v_a2.clone(), &v_b2.clone())?)?, &v_det.clone())?, py::truediv(&py::sub(&py::mul(&v_a1.clone(), &v_b2.clone())?, &py::mul(&v_b1.clone(), &v_c1.clone())?)?, &v_det.clone())?])).items()?;
        if t2061.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_x = t2061[0].clone();
        v_y = t2061[1].clone();
        if (py::call(&v_feasible.clone(), &[v_x.clone(), v_y.clone()])?).truthy() {
            let _ = py::method(&v_pts.clone(), "add", &[Py::tuple(vec![v_x.clone(), v_y.clone()])])?;
        }
    }
    v_rays = Py::list(vec![]);
    for t2069 in py::iter(&(py::add(&py::add(&Py::list(vec![Py::tuple(vec![py::fraction(&[Py::Int(1i64)])?, py::fraction(&[Py::Int(0i64)])?]), Py::tuple(vec![py::fraction(&[Py::Int(0i64)])?, py::fraction(&[Py::Int(1i64)])?])]), &{ let mut out = vec![]; for t2065 in py::iter(&(v_cons.clone()))? { let mut v__: Py; let mut v_a1: Py; let mut v_a2: Py; 
let t2066 = (t2065).items()?;
if t2066.len() != 4 { return Err(PyErr::Value("unpack".into())); }
v_a1 = t2066[0].clone();
v_a2 = t2066[1].clone();
v__ = t2066[2].clone();
v__ = t2066[3].clone();
out.push(Py::tuple(vec![v_a2.clone(), py::neg(&v_a1.clone())?])); } Py::list(out) })?, &{ let mut out = vec![]; for t2067 in py::iter(&(v_cons.clone()))? { let mut v__: Py; let mut v_a1: Py; let mut v_a2: Py; 
let t2068 = (t2067).items()?;
if t2068.len() != 4 { return Err(PyErr::Value("unpack".into())); }
v_a1 = t2068[0].clone();
v_a2 = t2068[1].clone();
v__ = t2068[2].clone();
v__ = t2068[3].clone();
out.push(Py::tuple(vec![py::neg(&v_a2.clone())?, v_a1.clone()])); } Py::list(out) })?))? {
        v_d = t2069;
        if ({ let t2077 = { let t2070 = v_d.clone(); Py::Bool({ let t2071 = Py::tuple(vec![Py::Int(0i64), Py::Int(0i64)]); if py::eq(&t2070, &t2071) { true } else { false } }) }; if t2077.truthy() { t2077 } else { { let t2076 = { let t2072 = py::getitem(&v_d.clone(), &Py::Int(0i64))?; Py::Bool({ let t2073 = Py::Int(0i64); if py::lt(&t2072, &t2073)? { true } else { false } }) }; if t2076.truthy() { t2076 } else { { let t2074 = py::getitem(&v_d.clone(), &Py::Int(1i64))?; Py::Bool({ let t2075 = Py::Int(0i64); if py::lt(&t2074, &t2075)? { true } else { false } }) } } } } }).truthy() {
            continue;
        }
        v_ok = Py::Bool(({ let mut out = vec![]; for t2078 in py::iter(&(v_cons.clone()))? { let mut v__: Py; let mut v_a1: Py; let mut v_a2: Py; let mut v_rl: Py; 
let t2079 = (t2078).items()?;
if t2079.len() != 4 { return Err(PyErr::Value("unpack".into())); }
v_a1 = t2079[0].clone();
v_a2 = t2079[1].clone();
v__ = t2079[2].clone();
v_rl = t2079[3].clone();
out.push(py::not(&{ let t2096 = { let t2084 = { let t2080 = v_rl.clone(); Py::Bool({ let t2081 = Py::str("<="); if py::eq(&t2080, &t2081) { true } else { false } }) }; if !t2084.truthy() { t2084 } else { { let t2082 = py::add(&py::mul(&v_a1.clone(), &py::getitem(&v_d.clone(), &Py::Int(0i64))?)?, &py::mul(&v_a2.clone(), &py::getitem(&v_d.clone(), &Py::Int(1i64))?)?)?; Py::Bool({ let t2083 = Py::Int(0i64); if py::gt(&t2082, &t2083)? { true } else { false } }) } } }; if t2096.truthy() { t2096 } else { { let t2095 = { let t2089 = { let t2085 = v_rl.clone(); Py::Bool({ let t2086 = Py::str(">="); if py::eq(&t2085, &t2086) { true } else { false } }) }; if !t2089.truthy() { t2089 } else { { let t2087 = py::add(&py::mul(&v_a1.clone(), &py::getitem(&v_d.clone(), &Py::Int(0i64))?)?, &py::mul(&v_a2.clone(), &py::getitem(&v_d.clone(), &Py::Int(1i64))?)?)?; Py::Bool({ let t2088 = Py::Int(0i64); if py::lt(&t2087, &t2088)? { true } else { false } }) } } }; if t2095.truthy() { t2095 } else { { let t2094 = { let t2090 = v_rl.clone(); Py::Bool({ let t2091 = Py::str("="); if py::eq(&t2090, &t2091) { true } else { false } }) }; if !t2094.truthy() { t2094 } else { { let t2092 = py::add(&py::mul(&v_a1.clone(), &py::getitem(&v_d.clone(), &Py::Int(0i64))?)?, &py::mul(&v_a2.clone(), &py::getitem(&v_d.clone(), &Py::Int(1i64))?)?)?; Py::Bool({ let t2093 = Py::Int(0i64); if py::ne(&t2092, &t2093) { true } else { false } }) } } } } } } })); } Py::list(out) }).items()?.iter().all(|x| x.truthy()));
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
    let t3831 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("sense"))?, py::getitem(&v_inputs.clone(), &Py::str("c"))?, py::getitem(&v_inputs.clone(), &Py::str("A"))?, py::getitem(&v_inputs.clone(), &Py::str("b"))?])).items()?;
    if t3831.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_sense = t3831[0].clone();
    v_c = t3831[1].clone();
    v_A = t3831[2].clone();
    v_b = t3831[3].clone();
    v_rel = f__relations(py::method(&v_inputs.clone(), "get", &[Py::str("relations")])?, py::len(&v_A.clone())?)?;
    if ({ let t3848 = { let t3844 = py::len(&v_p.clone())?; Py::Bool({ let t3845 = py::len(&v_A.clone())?; if py::ne(&t3844, &t3845) { true } else { false } }) }; if t3848.truthy() { t3848 } else { { let t3846 = py::len(&v_x.clone())?; Py::Bool({ let t3847 = py::len(&v_c.clone())?; if py::ne(&t3846, &t3847) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_s = (if ({ let t3849 = v_sense.clone(); Py::Bool({ let t3850 = Py::str("minimize"); if py::eq(&t3849, &t3850) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { py::neg(&Py::Float(1.0))? });
    v_cm = { let mut out = vec![]; for t3851 in py::iter(&(v_c.clone()))? { let mut v_v: Py; 
v_v = t3851;
out.push(py::mul(&v_s.clone(), &v_v.clone())?); } Py::list(out) };
    v_y = { let mut out = vec![]; for t3852 in py::iter(&(v_p.clone()))? { let mut v_v: Py; 
v_v = t3852;
out.push(py::mul(&v_s.clone(), &v_v.clone())?); } Py::list(out) };
    v_scale = py::minmax(&[py::add(&py::add(&py::add(&Py::list(vec![Py::Float(1.0)]), &{ let mut out = vec![]; for t3853 in py::iter(&(v_c.clone()))? { let mut v_v: Py; 
v_v = t3853;
out.push(py::abs(&v_v.clone())?); } Py::list(out) })?, &{ let mut out = vec![]; for t3854 in py::iter(&(v_b.clone()))? { let mut v_v: Py; 
v_v = t3854;
out.push(py::abs(&v_v.clone())?); } Py::list(out) })?, &{ let mut out = vec![]; for t3855 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t3855;
for t3856 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t3856;
out.push(py::abs(&v_v.clone())?); }} Py::list(out) })?], true)?;
    v_xs = py::minmax(&[py::add(&Py::list(vec![Py::Float(1.0)]), &{ let mut out = vec![]; for t3857 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t3857;
out.push(py::abs(&v_v.clone())?); } Py::list(out) })?], true)?;
    v_worst = Py::Float(0.0);
    for t3858 in py::iter(&(py::enumerate(&v_A.clone(), 0)?))? {
        let t3861 = (t3858).items()?;
        if t3861.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t3861[0].clone();
        v_row = t3861[1].clone();
        v_ax = py::sum(&{ let mut out = vec![]; for t3859 in py::iter(&(py::zip(&[v_row.clone(), v_x.clone()])?))? { let mut v_a: Py; let mut v_xi: Py; 
let t3860 = (t3859).items()?;
if t3860.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3860[0].clone();
v_xi = t3860[1].clone();
out.push(py::mul(&v_a.clone(), &v_xi.clone())?); } Py::list(out) }, None)?;
        v_viol = py::getitem(&Py::dict(vec![(Py::str("<="), py::minmax(&[Py::Float(0.0), py::sub(&v_ax.clone(), &py::getitem(&v_b.clone(), &v_i.clone())?)?], true)?), (Py::str(">="), py::minmax(&[Py::Float(0.0), py::sub(&py::getitem(&v_b.clone(), &v_i.clone())?, &v_ax.clone())?], true)?), (Py::str("="), py::abs(&py::sub(&v_ax.clone(), &py::getitem(&v_b.clone(), &v_i.clone())?)?)?)]), &py::getitem(&v_rel.clone(), &v_i.clone())?)?;
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&v_viol.clone(), &py::mul(&v_scale.clone(), &v_xs.clone())?)?], true)?;
        v_sv = py::getitem(&Py::dict(vec![(Py::str("<="), py::minmax(&[Py::Float(0.0), py::getitem(&v_y.clone(), &v_i.clone())?], true)?), (Py::str(">="), py::minmax(&[Py::Float(0.0), py::neg(&py::getitem(&v_y.clone(), &v_i.clone())?)?], true)?), (Py::str("="), Py::Float(0.0))]), &py::getitem(&v_rel.clone(), &v_i.clone())?)?;
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&v_sv.clone(), &v_scale.clone())?], true)?;
    }
    v_worst = py::minmax(&[py::add(&Py::list(vec![v_worst.clone()]), &{ let mut out = vec![]; for t3862 in py::iter(&(v_x.clone()))? { let mut v_xi: Py; 
v_xi = t3862;
out.push(py::truediv(&py::minmax(&[Py::Float(0.0), py::neg(&v_xi.clone())?], true)?, &v_xs.clone())?); } Py::list(out) })?], true)?;
    for t3863 in py::range_iter(&[(py::len(&v_c.clone())?).i()?]) {
        v_j = t3863;
        v_rc = py::sub(&py::getitem(&v_cm.clone(), &v_j.clone())?, &py::sum(&{ let mut out = vec![]; for t3864 in py::range_iter(&[(py::len(&v_A.clone())?).i()?]) { let mut v_i: Py; 
v_i = t3864;
out.push(py::mul(&py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_y.clone(), &v_i.clone())?)?); } Py::list(out) }, None)?)?;
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::minmax(&[Py::Float(0.0), py::neg(&v_rc.clone())?], true)?, &py::mul(&v_scale.clone(), &py::minmax(&[Py::Float(1.0), (if (v_y.clone()).truthy() { py::minmax(&[{ let mut out = vec![]; for t3865 in py::iter(&(v_y.clone()))? { let mut v_v: Py; 
v_v = t3865;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }], true)? } else { Py::Float(1.0) })], true)?)?)?], true)?;
    }
    v_zx = py::sum(&{ let mut out = vec![]; for t3866 in py::iter(&(py::zip(&[v_c.clone(), v_x.clone()])?))? { let mut v_ci: Py; let mut v_xi: Py; 
let t3867 = (t3866).items()?;
if t3867.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_ci = t3867[0].clone();
v_xi = t3867[1].clone();
out.push(py::mul(&v_ci.clone(), &v_xi.clone())?); } Py::list(out) }, None)?;
    v_zy = py::mul(&v_s.clone(), &py::sum(&{ let mut out = vec![]; for t3868 in py::iter(&(py::zip(&[v_b.clone(), v_y.clone()])?))? { let mut v_bi: Py; let mut v_yi: Py; 
let t3869 = (t3868).items()?;
if t3869.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_bi = t3869[0].clone();
v_yi = t3869[1].clone();
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
    if ({ let t4108 = py::not(&v_X.clone()); if t4108.truthy() { t4108 } else { Py::Bool(({ let mut out = vec![]; for t4105 in py::iter(&(v_X.clone()))? { let mut v_r: Py; 
v_r = t4105;
out.push({ let t4106 = py::len(&v_r.clone())?; Py::Bool({ let t4107 = py::len(&py::getitem(&v_X.clone(), &Py::Int(0i64))?)?; if py::ne(&t4106, &t4107) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the decision matrix must be rectangular (rows = alternatives, columns = criteria)"))));
    }
    let t4109 = (Py::tuple(vec![py::len(&v_X.clone())?, py::len(&py::getitem(&v_X.clone(), &Py::Int(0i64))?)?])).items()?;
    if t4109.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t4109[0].clone();
    v_k = t4109[1].clone();
    if ({ let t4117 = { let t4110 = v_m.clone(); Py::Bool({ let t4111 = Py::Int(2i64); if py::lt(&t4110, &t4111)? { true } else { false } }) }; if t4117.truthy() { t4117 } else { { let t4116 = { let t4112 = v_m.clone(); Py::Bool({ let t4113 = Py::Int(40i64); if py::gt(&t4112, &t4113)? { true } else { false } }) }; if t4116.truthy() { t4116 } else { { let t4114 = v_k.clone(); Py::Bool({ let t4115 = Py::Int(15i64); if py::gt(&t4114, &t4115)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give 2 to 40 alternatives and at most 15 criteria"))));
    }
    if ({ let t4122 = { let t4118 = py::len(&v_w.clone())?; Py::Bool({ let t4119 = v_k.clone(); if py::ne(&t4118, &t4119) { true } else { false } }) }; if t4122.truthy() { t4122 } else { { let t4120 = py::len(&v_types.clone())?; Py::Bool({ let t4121 = v_k.clone(); if py::ne(&t4120, &t4121) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["give one weight and one type per criterion (".to_string(), py::format(&v_k.clone(), "")?, ")".to_string()].concat()))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t4123 in py::iter(&(v_X.clone()))? { let mut v_r: Py; 
v_r = t4123;
for t4124 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t4124;
out.push({ let t4125 = v_v.clone(); Py::Bool({ let t4126 = Py::Int(0i64); if py::le(&t4125, &t4126)? { true } else { false } }) }); }} Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("every value of the decision matrix must be > 0"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t4127 in py::iter(&(v_w.clone()))? { let mut v_v: Py; 
v_v = t4127;
out.push({ let t4128 = v_v.clone(); Py::Bool({ let t4129 = Py::Int(0i64); if py::le(&t4128, &t4129)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("every weight must be > 0"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t4130 in py::iter(&(v_types.clone()))? { let mut v_t: Py; 
v_t = t4130;
out.push({ let t4131 = v_t.clone(); Py::Bool({ let t4132 = Py::tuple(vec![Py::Int(1i64), py::neg(&Py::Int(1i64))?]); if !py::contains(&t4132, &t4131)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("criterion types are 1 (benefit: larger is better) or -1 (cost: smaller is better)"))));
    }
    v_W = { let mut out = vec![]; for t4133 in py::iter(&(v_w.clone()))? { let mut v_v: Py; 
v_v = t4133;
out.push(f__fr(v_v.clone())?); } Py::list(out) };
    v_s = py::sum(&v_W.clone(), None)?;
    return Ok(Py::tuple(vec![{ let mut out = vec![]; for t4134 in py::iter(&(v_X.clone()))? { let mut v_r: Py; 
v_r = t4134;
out.push({ let mut out = vec![]; for t4135 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t4135;
out.push(py::float(&v_v.clone())?); } Py::list(out) }); } Py::list(out) }, { let mut out = vec![]; for t4136 in py::iter(&(v_W.clone()))? { let mut v_v: Py; 
v_v = t4136;
out.push(py::float(&py::truediv(&v_v.clone(), &v_s.clone())?)?); } Py::list(out) }, { let mut out = vec![]; for t4137 in py::iter(&(v_types.clone()))? { let mut v_t: Py; 
v_t = t4137;
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
    if ({ let t2411 = { let t2403 = py::len(&v_capacity.clone())?; Py::Bool({ let t2404 = v_n.clone(); if py::ne(&t2403, &t2404) { true } else { false } }) }; if t2411.truthy() { t2411 } else { { let t2410 = { let t2405 = py::len(&v_cost.clone())?; Py::Bool({ let t2406 = v_n.clone(); if py::ne(&t2405, &t2406) { true } else { false } }) }; if t2410.truthy() { t2410 } else { Py::Bool(({ let mut out = vec![]; for t2407 in py::iter(&(py::add(&v_capacity.clone(), &v_cost.clone())?))? { let mut v_r: Py; 
v_r = t2407;
out.push({ let t2408 = py::len(&v_r.clone())?; Py::Bool({ let t2409 = v_n.clone(); if py::ne(&t2408, &t2409) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["capacity and cost must be ".to_string(), py::format(&v_n.clone(), "")?, " x ".to_string(), py::format(&v_n.clone(), "")?, " matrices (one row and column per node)".to_string()].concat()))));
    }
    if ({ let t2412 = py::sum(&{ let mut out = vec![]; for t2414 in py::iter(&(v_supply.clone()))? { let mut v_v: Py; 
v_v = t2414;
out.push(f__fr(v_v.clone())?); } Py::list(out) }, None)?; Py::Bool({ let t2413 = Py::Int(0i64); if py::ne(&t2412, &t2413) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("supplies (positive) and demands (negative) must sum to 0"))));
    }
    v_arcs = { let mut out = vec![]; for t2415 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t2415;
for t2416 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t2416;
if ({ let t2421 = { let t2417 = v_i.clone(); Py::Bool({ let t2418 = v_j.clone(); if py::ne(&t2417, &t2418) { true } else { false } }) }; if !t2421.truthy() { t2421 } else { { let t2419 = py::getitem(&py::getitem(&v_capacity.clone(), &v_i.clone())?, &v_j.clone())?; Py::Bool({ let t2420 = Py::Int(0i64); if py::gt(&t2419, &t2420)? { true } else { false } }) } } }).truthy() { out.push(Py::tuple(vec![v_i.clone(), v_j.clone()])); }}} Py::list(out) };
    if ({ let t2424 = py::not(&v_arcs.clone()); if t2424.truthy() { t2424 } else { { let t2422 = py::len(&v_arcs.clone())?; Py::Bool({ let t2423 = Py::Int(60i64); if py::gt(&t2422, &t2423)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give between 1 and 60 arcs (positive capacities)"))));
    }
    v_c = { let mut out = vec![]; for t2425 in py::iter(&(v_arcs.clone()))? { let mut v_i: Py; let mut v_j: Py; 
let t2426 = (t2425).items()?;
if t2426.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t2426[0].clone();
v_j = t2426[1].clone();
out.push(py::getitem(&py::getitem(&v_cost.clone(), &v_i.clone())?, &v_j.clone())?); } Py::list(out) };
    let t2427 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![]), Py::list(vec![])])).items()?;
    if t2427.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t2427[0].clone();
    v_b = t2427[1].clone();
    v_rel = t2427[2].clone();
    for t2428 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_v = t2428;
        let _ = py::method(&v_A.clone(), "append", &[{ let mut out = vec![]; for t2429 in py::iter(&(v_arcs.clone()))? { let mut v_i: Py; let mut v_j: Py; 
let t2430 = (t2429).items()?;
if t2430.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t2430[0].clone();
v_j = t2430[1].clone();
out.push(py::sub(&(if ({ let t2431 = v_i.clone(); Py::Bool({ let t2432 = v_v.clone(); if py::eq(&t2431, &t2432) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { Py::Int(0i64) }), &(if ({ let t2433 = v_j.clone(); Py::Bool({ let t2434 = v_v.clone(); if py::eq(&t2433, &t2434) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { Py::Int(0i64) }))?); } Py::list(out) }])?;
        let _ = py::method(&v_b.clone(), "append", &[py::getitem(&v_supply.clone(), &v_v.clone())?])?;
        let _ = py::method(&v_rel.clone(), "append", &[Py::str("=")])?;
    }
    for t2435 in py::iter(&(py::enumerate(&v_arcs.clone(), 0)?))? {
        let t2439 = (t2435).items()?;
        if t2439.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2439[0].clone();
        let t2440 = (t2439[1].clone()).items()?;
        if t2440.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t2440[0].clone();
        v_j = t2440[1].clone();
        let _ = py::method(&v_A.clone(), "append", &[{ let mut out = vec![]; for t2436 in py::range_iter(&[(py::len(&v_arcs.clone())?).i()?]) { let mut v_q: Py; 
v_q = t2436;
out.push((if ({ let t2437 = v_q.clone(); Py::Bool({ let t2438 = v_k.clone(); if py::eq(&t2437, &t2438) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { Py::Int(0i64) })); } Py::list(out) }])?;
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
    if (py::not(&{ let t2306 = Py::Int(0i64); Py::Bool({ let t2307 = v_gamma.clone(); if py::lt(&t2306, &t2307)? { { let t2308 = Py::Int(1i64); if py::lt(&t2307, &t2308)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the discount factor must be between 0 and 1 (exclusive)"))));
    }
    let t2309 = (Py::tuple(vec![py::len(&v_R.clone())?, (if (v_R.clone()).truthy() { py::len(&py::getitem(&v_R.clone(), &Py::Int(0i64))?)? } else { Py::Int(0i64) })])).items()?;
    if t2309.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_S = t2309[0].clone();
    v_A = t2309[1].clone();
    if ({ let t2324 = { let t2310 = v_S.clone(); Py::Bool({ let t2311 = Py::Int(1i64); if py::lt(&t2310, &t2311)? { true } else { false } }) }; if t2324.truthy() { t2324 } else { { let t2323 = { let t2312 = v_S.clone(); Py::Bool({ let t2313 = Py::Int(10i64); if py::gt(&t2312, &t2313)? { true } else { false } }) }; if t2323.truthy() { t2323 } else { { let t2322 = { let t2314 = v_A.clone(); Py::Bool({ let t2315 = Py::Int(1i64); if py::lt(&t2314, &t2315)? { true } else { false } }) }; if t2322.truthy() { t2322 } else { { let t2321 = { let t2316 = v_A.clone(); Py::Bool({ let t2317 = Py::Int(5i64); if py::gt(&t2316, &t2317)? { true } else { false } }) }; if t2321.truthy() { t2321 } else { Py::Bool(({ let mut out = vec![]; for t2318 in py::iter(&(v_R.clone()))? { let mut v_r: Py; 
v_r = t2318;
out.push({ let t2319 = py::len(&v_r.clone())?; Py::Bool({ let t2320 = v_A.clone(); if py::ne(&t2319, &t2320) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } } } } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("rewards: one row per state (at most 10), one column per action (at most 5)"))));
    }
    if ({ let t2330 = { let t2325 = py::len(&v_T.clone())?; Py::Bool({ let t2326 = py::mul(&v_S.clone(), &v_A.clone())?; if py::ne(&t2325, &t2326) { true } else { false } }) }; if t2330.truthy() { t2330 } else { Py::Bool(({ let mut out = vec![]; for t2327 in py::iter(&(v_T.clone()))? { let mut v_r: Py; 
v_r = t2327;
out.push({ let t2328 = py::len(&v_r.clone())?; Py::Bool({ let t2329 = v_S.clone(); if py::ne(&t2328, &t2329) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["transitions: ".to_string(), py::format(&py::mul(&v_S.clone(), &v_A.clone())?, "")?, " rows (state 1 action 1, state 1 action 2, ...) of ".to_string(), py::format(&v_S.clone(), "")?, " probabilities".to_string()].concat()))));
    }
    v_P = Py::list(vec![]);
    for t2331 in py::iter(&(v_T.clone()))? {
        v_r = t2331;
        v_fr = { let mut out = vec![]; for t2332 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t2332;
out.push(f__fr(v_v.clone())?); } Py::list(out) };
        v_s = py::sum(&v_fr.clone(), None)?;
        if ({ let t2338 = Py::Bool(({ let mut out = vec![]; for t2333 in py::iter(&(v_fr.clone()))? { let mut v_v: Py; 
v_v = t2333;
out.push({ let t2334 = v_v.clone(); Py::Bool({ let t2335 = Py::Int(0i64); if py::lt(&t2334, &t2335)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t2338.truthy() { t2338 } else { { let t2336 = py::abs(&py::sub(&v_s.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t2337 = py::fraction(&[Py::Int(1i64), py::pow(&Py::Int(10i64), &Py::Int(9i64))?])?; if py::gt(&t2336, &t2337)? { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("every transition row must be probabilities summing to 1"))));
        }
        let _ = py::method(&v_P.clone(), "append", &[{ let mut out = vec![]; for t2339 in py::iter(&(v_fr.clone()))? { let mut v_v: Py; 
v_v = t2339;
out.push(py::truediv(&v_v.clone(), &v_s.clone())?); } Py::list(out) }])?;
    }
    return Ok(Py::tuple(vec![{ let mut out = vec![]; for t2340 in py::iter(&(v_R.clone()))? { let mut v_r: Py; 
v_r = t2340;
out.push({ let mut out = vec![]; for t2341 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t2341;
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
    for t3391 in py::iter(&(v_edges.clone()))? {
        v_e = t3391;
        for t3392 in py::iter(&(py::slice(&v_e.clone(), None, Some((Py::Int(2i64)).i()?), None)?))? {
            v_v = t3392;
            if ({ let t3393 = v_v.clone(); Py::Bool({ let t3394 = v_out.clone(); if !py::contains(&t3394, &t3393)? { true } else { false } }) }).truthy() {
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
    if (py::not(&{ let t3017 = v_s.clone(); Py::Bool({ let t3018 = v_c.clone(); if py::lt(&t3017, &t3018)? { { let t3019 = v_p.clone(); if py::lt(&t3018, &t3019)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the salvage value, unit cost and price must satisfy salvage < cost < price"))));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nv_disc_profit(mut v_p: Py, mut v_c: Py, mut v_s: Py, mut v_demands: Py, mut v_probs: Py, mut v_Q: Py) -> R {
    return Ok(py::sum(&{ let mut out = vec![]; for t3002 in py::iter(&(py::zip(&[v_demands.clone(), v_probs.clone()])?))? { let mut v_d: Py; let mut v_pr: Py; 
let t3003 = (t3002).items()?;
if t3003.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_d = t3003[0].clone();
v_pr = t3003[1].clone();
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
    if ({ let t4245 = py::not(&v_Am.clone()); if t4245.truthy() { t4245 } else { { let t4244 = py::not(&py::getitem(&v_Am.clone(), &Py::Int(0i64))?); if t4244.truthy() { t4244 } else { Py::Bool(({ let mut out = vec![]; for t4241 in py::iter(&(v_Am.clone()))? { let mut v_r: Py; 
v_r = t4241;
out.push({ let t4242 = py::len(&v_r.clone())?; Py::Bool({ let t4243 = py::len(&py::getitem(&v_Am.clone(), &Py::Int(0i64))?)?; if py::ne(&t4242, &t4243) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["the ".to_string(), py::format(&v_name.clone(), "")?, " must be a rectangular matrix".to_string()].concat()))));
    }
    if ({ let t4250 = { let t4246 = py::len(&v_Am.clone())?; Py::Bool({ let t4247 = v_max_dim.clone(); if py::gt(&t4246, &t4247)? { true } else { false } }) }; if t4250.truthy() { t4250 } else { { let t4248 = py::len(&py::getitem(&v_Am.clone(), &Py::Int(0i64))?)?; Py::Bool({ let t4249 = v_max_dim.clone(); if py::gt(&t4248, &t4249)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["at most ".to_string(), py::format(&v_max_dim.clone(), "")?, " strategies per player".to_string()].concat()))));
    }
    return Ok({ let mut out = vec![]; for t4251 in py::iter(&(v_Am.clone()))? { let mut v_r: Py; 
v_r = t4251;
out.push({ let mut out = vec![]; for t4252 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t4252;
out.push(f__fr(v_v.clone())?); } Py::list(out) }); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pder(mut v_cs: Py) -> R {
    let mut v_n: Py = Py::None;
    v_n = py::sub(&py::len(&v_cs.clone())?, &Py::Int(1i64))?;
    return Ok({ let mut out = vec![]; for t3202 in py::iter(&(py::enumerate(&py::slice(&v_cs.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?, 0)?))? { let mut v_a: Py; let mut v_k: Py; 
let t3203 = (t3202).items()?;
if t3203.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3203[0].clone();
v_a = t3203[1].clone();
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
    for t3205 in py::iter(&(v_cs.clone()))? {
        v_a = t3205;
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
    if (py::not(&{ let t3111 = Py::Int(1i64); Py::Bool({ let t3112 = v_n.clone(); if py::le(&t3111, &t3112)? { { let t3113 = Py::Int(26i64); if py::le(&t3112, &t3113)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give between 1 and 26 activities"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t3114 in py::iter(&(v_dur.clone()))? { let mut v_v: Py; 
v_v = t3114;
out.push({ let t3115 = v_v.clone(); Py::Bool({ let t3116 = Py::Int(0i64); if py::lt(&t3115, &t3116)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("durations must be >= 0"))));
    }
    v_idx = f__activity_index(v_n.clone())?;
    v_preds = { let mut out = vec![]; for t3118 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v__: Py; 
v__ = t3118;
out.push(py::set(&Py::list(vec![]))?); } Py::list(out) };
    for t3120 in py::iter(&({ let t3119 = v_precedences.clone(); if t3119.truthy() { t3119 } else { Py::list(vec![]) } }))? {
        v_e = t3120;
        if ({ let t3121 = py::len(&v_e.clone())?; Py::Bool({ let t3122 = Py::Int(3i64); if py::eq(&t3121, &t3122) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("a precedence is a pair 'A-B' (A before B) without a number"))));
        }
        let t3123 = (Py::tuple(vec![py::method(&py::method(&Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(0i64))?)), "strip", &[])?, "upper", &[])?, py::method(&py::method(&Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(1i64))?)), "strip", &[])?, "upper", &[])?])).items()?;
        if t3123.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_u = t3123[0].clone();
        v_v = t3123[1].clone();
        if ({ let t3128 = { let t3124 = v_u.clone(); Py::Bool({ let t3125 = v_idx.clone(); if !py::contains(&t3125, &t3124)? { true } else { false } }) }; if t3128.truthy() { t3128 } else { { let t3126 = v_v.clone(); Py::Bool({ let t3127 = v_idx.clone(); if !py::contains(&t3127, &t3126)? { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["precedence ".to_string(), py::format(&v_u.clone(), "")?, "-".to_string(), py::format(&v_v.clone(), "")?, " names an activity that does not exist (activities are A..".to_string(), py::format(&py::chr(&py::sub(&py::add(&py::ord(&Py::str("A"))?, &v_n.clone())?, &Py::Int(1i64))?)?, "")?, " or 1..".to_string(), py::format(&v_n.clone(), "")?, ")".to_string()].concat()))));
        }
        let _ = py::method(&py::getitem(&v_preds.clone(), &py::getitem(&v_idx.clone(), &v_v.clone())?)?, "add", &[py::getitem(&v_idx.clone(), &v_u.clone())?])?;
    }
    let t3129 = (Py::tuple(vec![Py::list(vec![]), py::set(&Py::list(vec![]))?])).items()?;
    if t3129.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_order = t3129[0].clone();
    v_done = t3129[1].clone();
    while ({ let t3136 = py::len(&v_order.clone())?; Py::Bool({ let t3137 = v_n.clone(); if py::lt(&t3136, &t3137)? { true } else { false } }) }).truthy() {
        v_ready = { let mut out = vec![]; for t3130 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t3130;
if ({ let t3135 = { let t3131 = v_k.clone(); Py::Bool({ let t3132 = v_done.clone(); if !py::contains(&t3132, &t3131)? { true } else { false } }) }; if !t3135.truthy() { t3135 } else { { let t3133 = py::getitem(&v_preds.clone(), &v_k.clone())?; Py::Bool({ let t3134 = v_done.clone(); if py::le(&t3133, &t3134)? { true } else { false } }) } } }).truthy() { out.push(v_k.clone()); }} Py::list(out) };
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
    let t3277 = (Py::tuple(vec![py::len(&v_MA.clone())?, py::len(&py::getitem(&v_MA.clone(), &Py::Int(0i64))?)?])).items()?;
    if t3277.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t3277[0].clone();
    v_n = t3277[1].clone();
    return Ok({ let mut out = vec![]; for t3278 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_r: Py; 
v_r = t3278;
for t3279 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_s: Py; 
v_s = t3279;
if ({ let t3286 = { let t3280 = py::getitem(&py::getitem(&v_MA.clone(), &v_r.clone())?, &v_s.clone())?; Py::Bool({ let t3281 = py::minmax(&[{ let mut out = vec![]; for t3282 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_k: Py; 
v_k = t3282;
out.push(py::getitem(&py::getitem(&v_MA.clone(), &v_k.clone())?, &v_s.clone())?); } Py::list(out) }], true)?; if py::eq(&t3280, &t3281) { true } else { false } }) }; if !t3286.truthy() { t3286 } else { { let t3283 = py::getitem(&py::getitem(&v_MB.clone(), &v_r.clone())?, &v_s.clone())?; Py::Bool({ let t3284 = py::minmax(&[{ let mut out = vec![]; for t3285 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t3285;
out.push(py::getitem(&py::getitem(&v_MB.clone(), &v_r.clone())?, &v_k.clone())?); } Py::list(out) }], true)?; if py::eq(&t3283, &t3284) { true } else { false } }) } } }).truthy() { out.push(Py::list(vec![py::add(&v_r.clone(), &Py::Int(1i64))?, py::add(&v_s.clone(), &Py::Int(1i64))?])); }}} Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__qf(mut v_M: Py, mut v_c: Py, mut v_x: Py) -> R {
    return Ok(py::add(&py::truediv(&py::sum(&{ let mut out = vec![]; for t3960 in py::range_iter(&[(py::len(&v_x.clone())?).i()?]) { let mut v_i: Py; 
v_i = t3960;
for t3961 in py::range_iter(&[(py::len(&v_x.clone())?).i()?]) { let mut v_j: Py; 
v_j = t3961;
out.push(py::mul(&py::mul(&py::getitem(&v_x.clone(), &v_i.clone())?, &py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_j.clone())?)?, &py::getitem(&v_x.clone(), &v_j.clone())?)?); }} Py::list(out) }, None)?, &Py::Int(2i64))?, &py::sum(&{ let mut out = vec![]; for t3962 in py::iter(&(py::zip(&[v_c.clone(), v_x.clone()])?))? { let mut v_ci: Py; let mut v_xi: Py; 
let t3963 = (t3962).items()?;
if t3963.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_ci = t3963[0].clone();
v_xi = t3963[1].clone();
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
    if ({ let t3836 = { let t3832 = v_text.clone(); Py::Bool({ let t3833 = Py::None; if py::is(&t3832, &t3833) { true } else { false } }) }; if t3836.truthy() { t3836 } else { { let t3834 = py::method(&Py::str(&py::to_str(&v_text.clone())), "strip", &[])?; Py::Bool({ let t3835 = Py::str(""); if py::eq(&t3834, &t3835) { true } else { false } }) } } }).truthy() {
        return Ok(py::mul(&Py::list(vec![Py::str("<=")]), &v_m.clone())?);
    }
    v_toks = { let mut out = vec![]; for t3837 in py::iter(&(ax_engine::pyre::module_call("split", &Py::str("[,\\s;]+"), &[py::method(&py::method(&py::method(&Py::str(&py::to_str(&v_text.clone())), "replace", &[Py::str("≤"), Py::str("<=")])?, "replace", &[Py::str("≥"), Py::str(">=")])?, "strip", &[])?, Py::Int(0)], 0)?))? { let mut v_t: Py; 
v_t = t3837;
if (v_t.clone()).truthy() { out.push(v_t.clone()); }} Py::list(out) };
    v_toks = { let mut out = vec![]; for t3838 in py::iter(&(v_toks.clone()))? { let mut v_t: Py; 
v_t = t3838;
out.push(py::method(&Py::dict(vec![(Py::str("=<"), Py::str("<=")), (Py::str("=>"), Py::str(">=")), (Py::str("=="), Py::str("="))]), "get", &[v_t.clone(), v_t.clone()])?); } Py::list(out) };
    if ({ let t3843 = { let t3839 = py::len(&v_toks.clone())?; Py::Bool({ let t3840 = Py::Int(1i64); if py::eq(&t3839, &t3840) { true } else { false } }) }; if !t3843.truthy() { t3843 } else { { let t3841 = v_m.clone(); Py::Bool({ let t3842 = Py::Int(1i64); if py::gt(&t3841, &t3842)? { true } else { false } }) } } }).truthy() {
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
    while ({ let t3194 = { let t3190 = py::len(&v_cs.clone())?; Py::Bool({ let t3191 = Py::Int(1i64); if py::gt(&t3190, &t3191)? { true } else { false } }) }; if !t3194.truthy() { t3194 } else { { let t3192 = py::getitem(&v_cs.clone(), &Py::Int(0i64))?; Py::Bool({ let t3193 = Py::Int(0i64); if py::eq(&t3192, &t3193) { true } else { false } }) } } }).truthy() {
        v_cs = py::slice(&v_cs.clone(), Some((Py::Int(1i64)).i()?), None, None)?;
    }
    if ({ let t3195 = py::len(&v_cs.clone())?; Py::Bool({ let t3196 = Py::Int(1i64); if py::le(&t3195, &t3196)? { true } else { false } }) }).truthy() {
        return Ok(Py::list(vec![]));
    }
    if ({ let t3197 = py::len(&v_cs.clone())?; Py::Bool({ let t3198 = Py::Int(2i64); if py::eq(&t3197, &t3198) { true } else { false } }) }).truthy() {
        v_r = py::truediv(&py::neg(&py::getitem(&v_cs.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_cs.clone(), &Py::Int(0i64))?)?;
        return Ok((if ({ let t3199 = v_lo.clone(); Py::Bool({ let t3200 = v_r.clone(); if py::le(&t3199, &t3200)? { { let t3201 = v_hi.clone(); if py::le(&t3200, &t3201)? { true } else { false } } } else { false } }) }).truthy() { Py::list(vec![v_r.clone()]) } else { Py::list(vec![]) }));
    }
    v_pts = py::add(&py::add(&Py::list(vec![v_lo.clone()]), &f__roots_in(f__pder(v_cs.clone())?, v_lo.clone(), v_hi.clone())?)?, &Py::list(vec![v_hi.clone()]))?;
    v_out = Py::list(vec![]);
    for t3204 in py::iter(&(py::zip(&[v_pts.clone(), py::slice(&v_pts.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? {
        let t3224 = (t3204).items()?;
        if t3224.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t3224[0].clone();
        v_b = t3224[1].clone();
        let t3206 = (Py::tuple(vec![f__peval(v_cs.clone(), v_a.clone())?, f__peval(v_cs.clone(), v_b.clone())?])).items()?;
        if t3206.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_fa = t3206[0].clone();
        v_fb = t3206[1].clone();
        if ({ let t3207 = v_fa.clone(); Py::Bool({ let t3208 = Py::Int(0i64); if py::eq(&t3207, &t3208) { true } else { false } }) }).truthy() {
            let _ = py::method(&v_out.clone(), "append", &[v_a.clone()])?;
        }
        if ({ let t3209 = py::mul(&v_fa.clone(), &v_fb.clone())?; Py::Bool({ let t3210 = Py::Int(0i64); if py::lt(&t3209, &t3210)? { true } else { false } }) }).truthy() {
            for t3211 in py::range_iter(&[(Py::Int(2000i64)).i()?]) {
                v__ = t3211;
                v_mid = py::truediv(&py::add(&v_a.clone(), &v_b.clone())?, &Py::Int(2i64))?;
                if ({ let t3212 = v_mid.clone(); Py::Bool({ let t3213 = Py::tuple(vec![v_a.clone(), v_b.clone()]); if py::contains(&t3213, &t3212)? { true } else { false } }) }).truthy() {
                    break;
                }
                v_fm = f__peval(v_cs.clone(), v_mid.clone())?;
                if ({ let t3214 = v_fm.clone(); Py::Bool({ let t3215 = Py::Int(0i64); if py::eq(&t3214, &t3215) { true } else { false } }) }).truthy() {
                    let t3216 = v_mid.clone();
                    v_a = t3216.clone();
                    v_b = t3216.clone();
                    break;
                }
                if ({ let t3217 = { let t3219 = v_fm.clone(); Py::Bool({ let t3220 = Py::Int(0i64); if py::lt(&t3219, &t3220)? { true } else { false } }) }; Py::Bool({ let t3218 = { let t3221 = v_fa.clone(); Py::Bool({ let t3222 = Py::Int(0i64); if py::lt(&t3221, &t3222)? { true } else { false } }) }; if py::eq(&t3217, &t3218) { true } else { false } }) }).truthy() {
                    let t3223 = (Py::tuple(vec![v_mid.clone(), v_fm.clone()])).items()?;
                    if t3223.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    v_a = t3223[0].clone();
                    v_fa = t3223[1].clone();
                } else {
                    v_b = v_mid.clone();
                }
            }
            let _ = py::method(&v_out.clone(), "append", &[py::truediv(&py::add(&v_a.clone(), &v_b.clone())?, &Py::Int(2i64))?])?;
        }
    }
    if ({ let t3225 = f__peval(v_cs.clone(), v_hi.clone())?; Py::Bool({ let t3226 = Py::Int(0i64); if py::eq(&t3225, &t3226) { true } else { false } }) }).truthy() {
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
    return Ok((if ({ let t4164 = v_d.clone(); Py::Bool({ let t4165 = Py::Float(1e-15); if py::le(&t4164, &t4165)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::truediv(&v_d.clone(), &py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?)? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sens_same(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    if ({ let t1776 = Py::Bool(matches!(v_a.clone(), Py::Str(_))); if t1776.truthy() { t1776 } else { { let t1775 = Py::Bool(matches!(v_b.clone(), Py::Str(_))); if t1775.truthy() { t1775 } else { { let t1774 = { let t1770 = v_a.clone(); Py::Bool({ let t1771 = Py::None; if py::is(&t1770, &t1771) { true } else { false } }) }; if t1774.truthy() { t1774 } else { { let t1772 = v_b.clone(); Py::Bool({ let t1773 = Py::None; if py::is(&t1772, &t1773) { true } else { false } }) } } } } } } }).truthy() {
        return Ok({ let t1777 = v_a.clone(); Py::Bool({ let t1778 = v_b.clone(); if py::eq(&t1777, &t1778) { true } else { false } }) });
    }
    if ({ let t1779 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if t1779.truthy() { t1779 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }).truthy() {
        if ({ let t1783 = py::not(&{ let t1780 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if !t1780.truthy() { t1780 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }); if t1783.truthy() { t1783 } else { { let t1781 = py::len(&v_a.clone())?; Py::Bool({ let t1782 = py::len(&v_b.clone())?; if py::ne(&t1781, &t1782) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Bool(false));
        }
        for t1784 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? {
            let t1785 = (t1784).items()?;
            if t1785.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_u = t1785[0].clone();
            v_v = t1785[1].clone();
            if (py::not(&f__sens_same(v_u.clone(), v_v.clone())?)).truthy() {
                return Ok(Py::Bool(false));
            }
        }
        return Ok(Py::Bool(true));
    }
    return Ok({ let t1786 = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?; Py::Bool({ let t1787 = py::mul(&Py::Float(1e-12), &py::minmax(&[Py::Float(1.0), py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?)?; if py::le(&t1786, &t1787)? { true } else { false } }) });
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
    if (Py::Bool(({ let mut out = vec![]; for t2720 in py::iter(&(v_E.clone()))? { let mut v_u: Py; let mut v_v: Py; 
let t2721 = (t2720).items()?;
if t2721.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t2721[0].clone();
v_v = t2721[1].clone();
out.push({ let t2722 = v_u.clone(); Py::Bool({ let t2723 = v_v.clone(); if py::eq(&t2722, &t2723) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a node cannot be joined to itself"))));
    }
    v_nodes = f__nodes(v_E.clone())?;
    if ({ let t2728 = py::len(&v_nodes.clone())?; Py::Bool({ let t2729 = v_max_nodes.clone(); if py::gt(&t2728, &t2729)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["at most ".to_string(), py::format(&v_max_nodes.clone(), "")?, " nodes".to_string()].concat()))));
    }
    v_adj = { let mut out = vec![]; for t2730 in py::iter(&(v_nodes.clone()))? { let mut v_v: Py; 
v_v = t2730;
out.push((v_v.clone(), py::set(&Py::list(vec![]))?)); } Py::dict(out) };
    for t2731 in py::iter(&(v_E.clone()))? {
        let t2732 = (t2731).items()?;
        if t2732.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_u = t2732[0].clone();
        v_v = t2732[1].clone();
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
    let t4297 = (Py::tuple(vec![py::len(&v_A.clone())?, py::len(&v_c.clone())?])).items()?;
    if t4297.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t4297[0].clone();
    v_n = t4297[1].clone();
    v_sign = py::mul(&Py::list(vec![Py::Int(1i64)]), &v_m.clone())?;
    v_A = { let mut out = vec![]; for t4298 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t4298;
out.push(Py::list((v_r.clone()).items()?)); } Py::list(out) };
    v_b = Py::list((v_b.clone()).items()?);
    for t4299 in py::range_iter(&[(v_m.clone()).i()?]) {
        v_i = t4299;
        if ({ let t4300 = py::getitem(&v_b.clone(), &v_i.clone())?; Py::Bool({ let t4301 = Py::Int(0i64); if py::lt(&t4300, &t4301)? { true } else { false } }) }).truthy() {
            let t4303 = (Py::tuple(vec![{ let mut out = vec![]; for t4302 in py::iter(&(py::getitem(&v_A.clone(), &v_i.clone())?))? { let mut v_v: Py; 
v_v = t4302;
out.push(py::neg(&v_v.clone())?); } Py::list(out) }, py::neg(&py::getitem(&v_b.clone(), &v_i.clone())?)?, py::neg(&Py::Int(1i64))?])).items()?;
            if t4303.len() != 3 { return Err(PyErr::Value("unpack".into())); }
            py::setitem(&v_A.clone(), &v_i.clone(), t4303[0].clone())?;
            py::setitem(&v_b.clone(), &v_i.clone(), t4303[1].clone())?;
            py::setitem(&v_sign.clone(), &v_i.clone(), t4303[2].clone())?;
        }
    }
    v_T = { let mut out = vec![]; for t4304 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t4304;
out.push(py::add(&py::add(&py::getitem(&v_A.clone(), &v_i.clone())?, &{ let mut out = vec![]; for t4305 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_k: Py; 
v_k = t4305;
out.push(py::fraction(&[py::int(&{ let t4306 = v_k.clone(); Py::Bool({ let t4307 = v_i.clone(); if py::eq(&t4306, &t4307) { true } else { false } }) })?])?); } Py::list(out) })?, &Py::list(vec![py::getitem(&v_b.clone(), &v_i.clone())?]))?); } Py::list(out) };
    v_basis = { let mut out = vec![]; for t4308 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t4308;
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
            py::setitem(&v_T.clone(), &v_r.clone(), { let mut out = vec![]; for t4309 in py::iter(&(py::getitem(&v_T.clone(), &v_r.clone())?))? { let mut v_v: Py; 
v_v = t4309;
out.push(py::truediv(&v_v.clone(), &v_pv.clone())?); } Py::list(out) })?;
            for t4310 in py::range_iter(&[(py::len(&v_T.clone())?).i()?]) {
                v_i = t4310;
                if ({ let t4315 = { let t4311 = v_i.clone(); Py::Bool({ let t4312 = v_r.clone(); if py::ne(&t4311, &t4312) { true } else { false } }) }; if !t4315.truthy() { t4315 } else { { let t4313 = py::getitem(&py::getitem(&v_T.clone(), &v_i.clone())?, &v_col.clone())?; Py::Bool({ let t4314 = Py::Int(0i64); if py::ne(&t4313, &t4314) { true } else { false } }) } } }).truthy() {
                    v_f = py::getitem(&py::getitem(&v_T.clone(), &v_i.clone())?, &v_col.clone())?;
                    py::setitem(&v_T.clone(), &v_i.clone(), { let mut out = vec![]; for t4316 in py::iter(&(py::zip(&[py::getitem(&v_T.clone(), &v_i.clone())?, py::getitem(&v_T.clone(), &v_r.clone())?])?))? { let mut v_a: Py; let mut v_bb: Py; 
let t4317 = (t4316).items()?;
if t4317.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4317[0].clone();
v_bb = t4317[1].clone();
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
            for t4318 in py::range_iter(&[(v_max_iter.clone()).i()?]) {
                v__ = t4318;
                v_cb = { let mut out = vec![]; for t4319 in py::iter(&(v_basis.clone()))? { let mut v_j: Py; 
v_j = t4319;
out.push(py::getitem(&v_cost.clone(), &v_j.clone())?); } Py::list(out) };
                v_enter = Py::None;
                v_inb = py::set(&v_basis.clone())?;
                for t4320 in py::range_iter(&[(v_allowed.clone()).i()?]) {
                    v_j = t4320;
                    if ({ let t4321 = v_j.clone(); Py::Bool({ let t4322 = v_inb.clone(); if !py::contains(&t4322, &t4321)? { true } else { false } }) }).truthy() {
                        v_rc = py::sub(&py::getitem(&v_cost.clone(), &v_j.clone())?, &py::sum(&{ let mut out = vec![]; for t4323 in py::range_iter(&[(py::len(&v_T.clone())?).i()?]) { let mut v_i: Py; 
v_i = t4323;
if ({ let t4324 = py::getitem(&py::getitem(&v_T.clone(), &v_i.clone())?, &v_j.clone())?; Py::Bool({ let t4325 = Py::Int(0i64); if py::ne(&t4324, &t4325) { true } else { false } }) }).truthy() { out.push(py::mul(&py::getitem(&v_cb.clone(), &v_i.clone())?, &py::getitem(&py::getitem(&v_T.clone(), &v_i.clone())?, &v_j.clone())?)?); }} Py::list(out) }, None)?)?;
                        if ({ let t4326 = v_rc.clone(); Py::Bool({ let t4327 = Py::Int(0i64); if py::lt(&t4326, &t4327)? { true } else { false } }) }).truthy() {
                            v_enter = v_j.clone();
                            break;
                        }
                    }
                }
                if ({ let t4328 = v_enter.clone(); Py::Bool({ let t4329 = Py::None; if py::is(&t4328, &t4329) { true } else { false } }) }).truthy() {
                    return Ok(Py::str("optimal"));
                }
                v_best = Py::None;
                for t4330 in py::range_iter(&[(py::len(&v_T.clone())?).i()?]) {
                    v_i = t4330;
                    if ({ let t4331 = py::getitem(&py::getitem(&v_T.clone(), &v_i.clone())?, &v_enter.clone())?; Py::Bool({ let t4332 = Py::Int(0i64); if py::gt(&t4331, &t4332)? { true } else { false } }) }).truthy() {
                        v_ratio = py::truediv(&py::getitem(&py::getitem(&v_T.clone(), &v_i.clone())?, &py::neg(&Py::Int(1i64))?)?, &py::getitem(&py::getitem(&v_T.clone(), &v_i.clone())?, &v_enter.clone())?)?;
                        if ({ let t4343 = { let t4333 = v_best.clone(); Py::Bool({ let t4334 = Py::None; if py::is(&t4333, &t4334) { true } else { false } }) }; if t4343.truthy() { t4343 } else { { let t4342 = { let t4335 = v_ratio.clone(); Py::Bool({ let t4336 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; if py::lt(&t4335, &t4336)? { true } else { false } }) }; if t4342.truthy() { t4342 } else { { let t4341 = { let t4337 = v_ratio.clone(); Py::Bool({ let t4338 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; if py::eq(&t4337, &t4338) { true } else { false } }) }; if !t4341.truthy() { t4341 } else { { let t4339 = py::getitem(&v_basis.clone(), &v_i.clone())?; Py::Bool({ let t4340 = py::getitem(&v_basis.clone(), &py::getitem(&v_best.clone(), &Py::Int(1i64))?)?; if py::lt(&t4339, &t4340)? { true } else { false } }) } } } } } } }).truthy() {
                            v_best = Py::tuple(vec![v_ratio.clone(), v_i.clone()]);
                        }
                    }
                }
                if ({ let t4344 = v_best.clone(); Py::Bool({ let t4345 = Py::None; if py::is(&t4344, &t4345) { true } else { false } }) }).truthy() {
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
    if ({ let t4346 = py::sum(&{ let mut out = vec![]; for t4348 in py::range_iter(&[(py::len(&v_T.clone())?).i()?]) { let mut v_i: Py; 
v_i = t4348;
if ({ let t4349 = py::getitem(&v_basis.clone(), &v_i.clone())?; Py::Bool({ let t4350 = v_n.clone(); if py::ge(&t4349, &t4350)? { true } else { false } }) }).truthy() { out.push(py::getitem(&py::getitem(&v_T.clone(), &v_i.clone())?, &py::neg(&Py::Int(1i64))?)?); }} Py::list(out) }, None)?; Py::Bool({ let t4347 = Py::Int(0i64); if py::gt(&t4346, &t4347)? { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![Py::str("infeasible"), Py::None, Py::None, Py::None, Py::Int(0i64)]));
    }
    v_redundant = Py::Int(0i64);
    v_i = Py::Int(0i64);
    while ({ let t4359 = v_i.clone(); Py::Bool({ let t4360 = py::len(&v_T.clone())?; if py::lt(&t4359, &t4360)? { true } else { false } }) }).truthy() {
        if ({ let t4351 = py::getitem(&v_basis.clone(), &v_i.clone())?; Py::Bool({ let t4352 = v_n.clone(); if py::ge(&t4351, &t4352)? { true } else { false } }) }).truthy() {
            v_col = (match ('g4353: { let mut out = vec![]; for t4354 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t4354;
if ({ let t4355 = py::getitem(&py::getitem(&v_T.clone(), &v_i.clone())?, &v_j.clone())?; Py::Bool({ let t4356 = Py::Int(0i64); if py::ne(&t4355, &t4356) { true } else { false } }) }).truthy() { out.push(v_j.clone()); break 'g4353 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => Py::None });
            if ({ let t4357 = v_col.clone(); Py::Bool({ let t4358 = Py::None; if py::is(&t4357, &t4358) { true } else { false } }) }).truthy() {
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
    if ({ let t4361 = py::call(&v_phase.clone(), &[v_cost.clone(), v_n.clone()])?; Py::Bool({ let t4362 = Py::str("unbounded"); if py::eq(&t4361, &t4362) { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![Py::str("unbounded"), Py::None, Py::None, Py::None, v_redundant.clone()]));
    }
    v_x = py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &v_n.clone())?;
    for t4363 in py::iter(&(py::enumerate(&v_basis.clone(), 0)?))? {
        let t4364 = (t4363).items()?;
        if t4364.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t4364[0].clone();
        v_j = t4364[1].clone();
        py::setitem(&v_x.clone(), &v_j.clone(), py::getitem(&py::getitem(&v_T.clone(), &v_i.clone())?, &py::neg(&Py::Int(1i64))?)?)?;
    }
    v_cb = { let mut out = vec![]; for t4365 in py::iter(&(v_basis.clone()))? { let mut v_j: Py; 
v_j = t4365;
out.push(py::getitem(&v_cost.clone(), &v_j.clone())?); } Py::list(out) };
    v_y = { let mut out = vec![]; for t4366 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_k: Py; 
v_k = t4366;
out.push(py::mul(&py::getitem(&v_sign.clone(), &v_k.clone())?, &py::sum(&{ let mut out = vec![]; for t4367 in py::range_iter(&[(py::len(&v_T.clone())?).i()?]) { let mut v_i: Py; 
v_i = t4367;
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
    v_A = { let mut out = vec![]; for t3940 in py::iter(&(py::zip(&[v_M.clone(), v_rhs.clone()])?))? { let mut v_r: Py; let mut v_v: Py; 
let t3941 = (t3940).items()?;
if t3941.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t3941[0].clone();
v_v = t3941[1].clone();
out.push(py::add(&Py::list((v_r.clone()).items()?), &Py::list(vec![v_v.clone()]))?); } Py::list(out) };
    for t3942 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t3942;
        v_p = (match ('g3943: { let mut out = vec![]; for t3944 in py::range_iter(&[(v_k.clone()).i()?, (v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3944;
if ({ let t3945 = py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_k.clone())?; Py::Bool({ let t3946 = Py::Int(0i64); if py::ne(&t3945, &t3946) { true } else { false } }) }).truthy() { out.push(v_i.clone()); break 'g3943 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => Py::None });
        if ({ let t3947 = v_p.clone(); Py::Bool({ let t3948 = Py::None; if py::is(&t3947, &t3948) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the system is singular"))));
        }
        let t3949 = (Py::tuple(vec![py::getitem(&v_A.clone(), &v_p.clone())?, py::getitem(&v_A.clone(), &v_k.clone())?])).items()?;
        if t3949.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_A.clone(), &v_k.clone(), t3949[0].clone())?;
        py::setitem(&v_A.clone(), &v_p.clone(), t3949[1].clone())?;
        for t3950 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_i = t3950;
            if ({ let t3955 = { let t3951 = v_i.clone(); Py::Bool({ let t3952 = v_k.clone(); if py::ne(&t3951, &t3952) { true } else { false } }) }; if !t3955.truthy() { t3955 } else { { let t3953 = py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_k.clone())?; Py::Bool({ let t3954 = Py::Int(0i64); if py::ne(&t3953, &t3954) { true } else { false } }) } } }).truthy() {
                v_f = py::truediv(&py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_k.clone())?, &py::getitem(&py::getitem(&v_A.clone(), &v_k.clone())?, &v_k.clone())?)?;
                py::setitem(&v_A.clone(), &v_i.clone(), { let mut out = vec![]; for t3956 in py::iter(&(py::zip(&[py::getitem(&v_A.clone(), &v_i.clone())?, py::getitem(&v_A.clone(), &v_k.clone())?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3957 = (t3956).items()?;
if t3957.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3957[0].clone();
v_b = t3957[1].clone();
out.push(py::sub(&v_a.clone(), &py::mul(&v_f.clone(), &v_b.clone())?)?); } Py::list(out) })?;
            }
        }
    }
    return Ok({ let mut out = vec![]; for t3958 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3958;
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
    if ({ let t3412 = v_t.clone(); Py::Bool({ let t3413 = v_dist.clone(); if !py::contains(&t3413, &t3412)? { true } else { false } }) }).truthy() {
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
    v_tight = { let mut out = vec![]; for t3414 in py::iter(&(v_nodes.clone()))? { let mut v_v: Py; 
v_v = t3414;
out.push((v_v.clone(), { let mut out = vec![]; for t3415 in py::iter(&(v_nodes.clone()))? { let mut v_u: Py; 
v_u = t3415;
for t3416 in py::iter(&(py::method(&v_adj.clone(), "get", &[v_u.clone(), Py::list(vec![])])?))? { let mut v_w: Py; let mut v_x: Py; 
let t3417 = (t3416).items()?;
if t3417.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t3417[0].clone();
v_w = t3417[1].clone();
if ({ let t3422 = { let t3418 = v_x.clone(); Py::Bool({ let t3419 = v_v.clone(); if py::eq(&t3418, &t3419) { true } else { false } }) }; if !t3422.truthy() { t3422 } else { { let t3420 = py::add(&py::getitem(&v_dist.clone(), &v_u.clone())?, &v_w.clone())?; Py::Bool({ let t3421 = py::getitem(&v_dist.clone(), &v_v.clone())?; if py::eq(&t3420, &t3421) { true } else { false } }) } } }).truthy() { out.push(Py::tuple(vec![v_u.clone(), v_w.clone()])); }}} Py::list(out) })); } Py::dict(out) };
    v_zero = Py::Bool(({ let mut out = vec![]; for t3423 in py::iter(&(v_nodes.clone()))? { let mut v_v: Py; 
v_v = t3423;
for t3424 in py::iter(&(py::getitem(&v_tight.clone(), &v_v.clone())?))? { let mut v__: Py; let mut v_w: Py; 
let t3425 = (t3424).items()?;
if t3425.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v__ = t3425[0].clone();
v_w = t3425[1].clone();
out.push({ let t3426 = v_w.clone(); Py::Bool({ let t3427 = Py::Int(0i64); if py::eq(&t3426, &t3427) { true } else { false } }) }); }} Py::list(out) }).items()?.iter().any(|x| x.truthy()));
    v_count = Py::dict(vec![(v_s.clone(), Py::Int(1i64))]);
    for t3428 in py::iter(&(v_nodes.clone()))? {
        v_v = t3428;
        if ({ let t3429 = v_v.clone(); Py::Bool({ let t3430 = v_s.clone(); if py::ne(&t3429, &t3430) { true } else { false } }) }).truthy() {
            py::setitem(&v_count.clone(), &v_v.clone(), py::sum(&{ let mut out = vec![]; for t3431 in py::iter(&(py::getitem(&v_tight.clone(), &v_v.clone())?))? { let mut v__: Py; let mut v_u: Py; 
let t3432 = (t3431).items()?;
if t3432.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t3432[0].clone();
v__ = t3432[1].clone();
out.push(py::method(&v_count.clone(), "get", &[v_u.clone(), Py::Int(0i64)])?); } Py::list(out) }, None)?)?;
        }
    }
    let t3433 = (Py::tuple(vec![Py::list(vec![v_t.clone()]), v_t.clone()])).items()?;
    if t3433.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_path = t3433[0].clone();
    v_v = t3433[1].clone();
    while ({ let t3436 = v_v.clone(); Py::Bool({ let t3437 = v_s.clone(); if py::ne(&t3436, &t3437) { true } else { false } }) }).truthy() {
        v_v = py::minmax_key(&[{ let mut out = vec![]; for t3434 in py::iter(&(py::getitem(&v_tight.clone(), &v_v.clone())?))? { let mut v__: Py; let mut v_u: Py; 
let t3435 = (t3434).items()?;
if t3435.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t3435[0].clone();
v__ = t3435[1].clone();
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
    v_unique = { let t3440 = { let t3438 = py::getitem(&v_count.clone(), &v_t.clone())?; Py::Bool({ let t3439 = Py::Int(1i64); if py::eq(&t3438, &t3439) { true } else { false } }) }; if !t3440.truthy() { t3440 } else { py::not(&v_zero.clone()) } };
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
    let t4281 = (Py::tuple(vec![py::len(&v_A.clone())?, py::len(&v_c.clone())?])).items()?;
    if t4281.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t4281[0].clone();
    v_n = t4281[1].clone();
    v_slack_cols = { let mut out = vec![]; for t4282 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t4282;
if ({ let t4283 = py::getitem(&v_rel.clone(), &v_i.clone())?; Py::Bool({ let t4284 = Py::tuple(vec![Py::str("<="), Py::str(">=")]); if py::contains(&t4284, &t4283)? { true } else { false } }) }).truthy() { out.push(v_i.clone()); }} Py::list(out) };
    v_As = Py::list(vec![]);
    for t4285 in py::range_iter(&[(v_m.clone()).i()?]) {
        v_i = t4285;
        v_row = { let mut out = vec![]; for t4286 in py::iter(&(py::getitem(&v_A.clone(), &v_i.clone())?))? { let mut v_v: Py; 
v_v = t4286;
out.push(f__fr(v_v.clone())?); } Py::list(out) };
        for t4287 in py::iter(&(v_slack_cols.clone()))? {
            v_k = t4287;
            let _ = py::method(&v_row.clone(), "append", &[(if ({ let t4288 = v_k.clone(); Py::Bool({ let t4289 = v_i.clone(); if py::ne(&t4288, &t4289) { true } else { false } }) }).truthy() { py::fraction(&[Py::Int(0i64)])? } else { (if ({ let t4290 = py::getitem(&v_rel.clone(), &v_i.clone())?; Py::Bool({ let t4291 = Py::str("<="); if py::eq(&t4290, &t4291) { true } else { false } }) }).truthy() { py::fraction(&[Py::Int(1i64)])? } else { py::fraction(&[py::neg(&Py::Int(1i64))?])? }) })])?;
        }
        let _ = py::method(&v_As.clone(), "append", &[v_row.clone()])?;
    }
    v_cs = py::add(&{ let mut out = vec![]; for t4292 in py::iter(&(v_c.clone()))? { let mut v_v: Py; 
v_v = t4292;
out.push((if ({ let t4293 = v_sense.clone(); Py::Bool({ let t4294 = Py::str("minimize"); if py::eq(&t4293, &t4294) { true } else { false } }) }).truthy() { f__fr(v_v.clone())? } else { py::neg(&f__fr(v_v.clone())?)? })); } Py::list(out) }, &py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &py::len(&v_slack_cols.clone())?)?)?;
    return Ok(Py::tuple(vec![v_As.clone(), { let mut out = vec![]; for t4295 in py::iter(&(v_b.clone()))? { let mut v_v: Py; 
v_v = t4295;
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
    if ({ let t3922 = { let t3914 = v_n.clone(); Py::Bool({ let t3915 = Py::Int(0i64); if py::eq(&t3914, &t3915) { true } else { false } }) }; if t3922.truthy() { t3922 } else { { let t3921 = Py::Bool(({ let mut out = vec![]; for t3916 in py::iter(&(v_Q.clone()))? { let mut v_r: Py; 
v_r = t3916;
out.push({ let t3917 = py::len(&v_r.clone())?; Py::Bool({ let t3918 = v_n.clone(); if py::ne(&t3917, &t3918) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t3921.truthy() { t3921 } else { { let t3919 = v_n.clone(); Py::Bool({ let t3920 = Py::Int(10i64); if py::gt(&t3919, &t3920)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a square matrix (at most 10 x 10)".to_string()].concat()))));
    }
    v_M = { let mut out = vec![]; for t3923 in py::iter(&(v_Q.clone()))? { let mut v_r: Py; 
v_r = t3923;
out.push({ let mut out = vec![]; for t3924 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t3924;
out.push(f__fr(v_v.clone())?); } Py::list(out) }); } Py::list(out) };
    if (Py::Bool(({ let mut out = vec![]; for t3925 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3925;
for t3926 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3926;
out.push({ let t3927 = py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_j.clone())?; Py::Bool({ let t3928 = py::getitem(&py::getitem(&v_M.clone(), &v_j.clone())?, &v_i.clone())?; if py::ne(&t3927, &t3928) { true } else { false } }) }); }} Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be symmetric".to_string()].concat()))));
    }
    v_W = { let mut out = vec![]; for t3929 in py::iter(&(v_M.clone()))? { let mut v_r: Py; 
v_r = t3929;
out.push(py::slice(&v_r.clone(), None, None, None)?); } Py::list(out) };
    for t3930 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t3930;
        if ({ let t3931 = py::getitem(&py::getitem(&v_W.clone(), &v_k.clone())?, &v_k.clone())?; Py::Bool({ let t3932 = Py::Int(0i64); if py::le(&t3931, &t3932)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be positive definite (otherwise there is no unique minimum)".to_string()].concat()))));
        }
        for t3933 in py::range_iter(&[(py::add(&v_k.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_i = t3933;
            v_f = py::truediv(&py::getitem(&py::getitem(&v_W.clone(), &v_i.clone())?, &v_k.clone())?, &py::getitem(&py::getitem(&v_W.clone(), &v_k.clone())?, &v_k.clone())?)?;
            for t3934 in py::range_iter(&[(v_k.clone()).i()?, (v_n.clone()).i()?]) {
                v_j = t3934;
                let t3935 = py::getitem(&v_W.clone(), &v_i.clone())?; let t3936 = v_j.clone();
                py::setitem(&t3935, &t3936, py::sub(&py::getitem(&t3935, &t3936)?, &py::mul(&v_f.clone(), &py::getitem(&py::getitem(&v_W.clone(), &v_k.clone())?, &v_j.clone())?)?)?)?;
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
    let t3615 = (f__mcda_in(v_X.clone(), v_w.clone(), v_types.clone())?).items()?;
    if t3615.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_X = t3615[0].clone();
    v_w = t3615[1].clone();
    v_types = t3615[2].clone();
    let t3616 = (Py::tuple(vec![py::len(&v_X.clone())?, py::len(&py::getitem(&v_X.clone(), &Py::Int(0i64))?)?])).items()?;
    if t3616.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t3616[0].clone();
    v_k = t3616[1].clone();
    v_R = { let mut out = vec![]; for t3617 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t3617;
out.push({ let mut out = vec![]; for t3618 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t3618;
out.push(py::truediv(&py::getitem(&py::getitem(&v_X.clone(), &v_i.clone())?, &v_j.clone())?, &py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t3619 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_t: Py; 
v_t = t3619;
out.push(py::pow(&py::getitem(&py::getitem(&v_X.clone(), &v_t.clone())?, &v_j.clone())?, &Py::Int(2i64))?); } Py::list(out) })?)?)?); } Py::list(out) }); } Py::list(out) };
    v_V = { let mut out = vec![]; for t3620 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t3620;
out.push({ let mut out = vec![]; for t3621 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t3621;
out.push(py::mul(&py::getitem(&v_w.clone(), &v_j.clone())?, &(if ({ let t3622 = py::getitem(&v_types.clone(), &v_j.clone())?; Py::Bool({ let t3623 = Py::Int(1i64); if py::eq(&t3622, &t3623) { true } else { false } }) }).truthy() { py::getitem(&py::getitem(&v_R.clone(), &v_i.clone())?, &v_j.clone())? } else { py::sub(&Py::Int(1i64), &py::getitem(&py::getitem(&v_R.clone(), &v_i.clone())?, &v_j.clone())?)? }))?); } Py::list(out) }); } Py::list(out) };
    v_hi = { let mut out = vec![]; for t3624 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t3624;
out.push(py::minmax(&[{ let mut out = vec![]; for t3625 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t3625;
out.push(py::getitem(&py::getitem(&v_V.clone(), &v_i.clone())?, &v_j.clone())?); } Py::list(out) }], true)?); } Py::list(out) };
    v_lo = { let mut out = vec![]; for t3626 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t3626;
out.push(py::minmax(&[{ let mut out = vec![]; for t3627 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t3627;
out.push(py::getitem(&py::getitem(&v_V.clone(), &v_i.clone())?, &v_j.clone())?); } Py::list(out) }], false)?); } Py::list(out) };
    v_sc = { let mut out = vec![]; for t3628 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t3628;
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
    let t3638 = (Py::tuple(vec![py::len(&v_supply.clone())?, py::len(&v_demand.clone())?])).items()?;
    if t3638.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t3638[0].clone();
    v_n = t3638[1].clone();
    if ({ let t3644 = { let t3639 = py::len(&v_cost.clone())?; Py::Bool({ let t3640 = v_m.clone(); if py::ne(&t3639, &t3640) { true } else { false } }) }; if t3644.truthy() { t3644 } else { Py::Bool(({ let mut out = vec![]; for t3641 in py::iter(&(v_cost.clone()))? { let mut v_r: Py; 
v_r = t3641;
out.push({ let t3642 = py::len(&v_r.clone())?; Py::Bool({ let t3643 = v_n.clone(); if py::ne(&t3642, &t3643) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["the cost matrix must have ".to_string(), py::format(&v_m.clone(), "")?, " rows (sources) and ".to_string(), py::format(&v_n.clone(), "")?, " columns (destinations)".to_string()].concat()))));
    }
    if ({ let t3651 = Py::Bool(({ let mut out = vec![]; for t3645 in py::iter(&(v_supply.clone()))? { let mut v_v: Py; 
v_v = t3645;
out.push({ let t3646 = v_v.clone(); Py::Bool({ let t3647 = Py::Int(0i64); if py::lt(&t3646, &t3647)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t3651.truthy() { t3651 } else { Py::Bool(({ let mut out = vec![]; for t3648 in py::iter(&(v_demand.clone()))? { let mut v_v: Py; 
v_v = t3648;
out.push({ let t3649 = v_v.clone(); Py::Bool({ let t3650 = Py::Int(0i64); if py::lt(&t3649, &t3650)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("supplies and demands must be >= 0"))));
    }
    if ({ let t3652 = py::sum(&{ let mut out = vec![]; for t3654 in py::iter(&(v_supply.clone()))? { let mut v_v: Py; 
v_v = t3654;
out.push(f__fr(v_v.clone())?); } Py::list(out) }, None)?; Py::Bool({ let t3653 = py::sum(&{ let mut out = vec![]; for t3655 in py::iter(&(v_demand.clone()))? { let mut v_v: Py; 
v_v = t3655;
out.push(f__fr(v_v.clone())?); } Py::list(out) }, None)?; if py::lt(&t3652, &t3653)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("total supply is less than total demand: the demand cannot be met"))));
    }
    if ({ let t3656 = py::mul(&v_m.clone(), &v_n.clone())?; Py::Bool({ let t3657 = Py::Int(60i64); if py::gt(&t3656, &t3657)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at most 60 routes (sources x destinations)"))));
    }
    v_c = { let mut out = vec![]; for t3658 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t3658;
for t3659 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3659;
out.push(py::getitem(&py::getitem(&v_cost.clone(), &v_i.clone())?, &v_j.clone())?); }} Py::list(out) };
    let t3660 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![]), Py::list(vec![])])).items()?;
    if t3660.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3660[0].clone();
    v_b = t3660[1].clone();
    v_rel = t3660[2].clone();
    for t3661 in py::range_iter(&[(v_m.clone()).i()?]) {
        v_i = t3661;
        let _ = py::method(&v_A.clone(), "append", &[{ let mut out = vec![]; for t3662 in py::range_iter(&[(py::mul(&v_m.clone(), &v_n.clone())?).i()?]) { let mut v_k: Py; 
v_k = t3662;
out.push((if ({ let t3663 = py::floordiv(&v_k.clone(), &v_n.clone())?; Py::Bool({ let t3664 = v_i.clone(); if py::eq(&t3663, &t3664) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { Py::Int(0i64) })); } Py::list(out) }])?;
        let _ = py::method(&v_b.clone(), "append", &[py::getitem(&v_supply.clone(), &v_i.clone())?])?;
        let _ = py::method(&v_rel.clone(), "append", &[Py::str("<=")])?;
    }
    for t3665 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_j = t3665;
        let _ = py::method(&v_A.clone(), "append", &[{ let mut out = vec![]; for t3666 in py::range_iter(&[(py::mul(&v_m.clone(), &v_n.clone())?).i()?]) { let mut v_k: Py; 
v_k = t3666;
out.push((if ({ let t3667 = py::modulo(&v_k.clone(), &v_n.clone())?; Py::Bool({ let t3668 = v_j.clone(); if py::eq(&t3667, &t3668) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { Py::Int(0i64) })); } Py::list(out) }])?;
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
    let t4220 = ({ let mut out = vec![]; for t4218 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("A"))?))? { let mut v_r: Py; 
v_r = t4218;
out.push({ let mut out = vec![]; for t4219 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t4219;
out.push(py::float(&v_v.clone())?); } Py::list(out) }); } Py::list(out) }).items()?;
    if t4220.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    let t4221 = (t4220[0].clone()).items()?;
    if t4221.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t4221[0].clone();
    v_b = t4221[1].clone();
    let t4222 = (t4220[1].clone()).items()?;
    if t4222.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t4222[0].clone();
    v_d = t4222[1].clone();
    v_v = py::getitem(&v_o.clone(), &Py::str("value"))?;
    let t4223 = (Py::tuple(vec![py::minmax(&[py::minmax(&[v_a.clone(), v_b.clone()], false)?, py::minmax(&[v_c.clone(), v_d.clone()], false)?], true)?, py::minmax(&[py::minmax(&[v_a.clone(), v_c.clone()], true)?, py::minmax(&[v_b.clone(), v_d.clone()], true)?], false)?])).items()?;
    if t4223.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t4223[0].clone();
    v_hi = t4223[1].clone();
    if ({ let t4224 = py::method(&v_o.clone(), "get", &[Py::str("saddle_point")])?; Py::Bool({ let t4225 = Py::str("yes"); if py::eq(&t4224, &t4225) { true } else { false } }) }).truthy() {
        return Ok((if ({ let t4226 = v_lo.clone(); Py::Bool({ let t4227 = v_hi.clone(); if py::eq(&t4226, &t4227) { true } else { false } }) }).truthy() { py::minmax(&[f__rel(v_v.clone(), v_lo.clone())?, f__rel(v_v.clone(), v_hi.clone())?], true)? } else { Py::Float(1.0) }));
    }
    if ({ let t4232 = { let t4228 = v_lo.clone(); Py::Bool({ let t4229 = v_hi.clone(); if py::eq(&t4228, &t4229) { true } else { false } }) }; if t4232.truthy() { t4232 } else { { let t4230 = Py::str("p_row1"); Py::Bool({ let t4231 = v_o.clone(); if !py::contains(&t4231, &t4230)? { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    let t4233 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("p_row1"))?, py::getitem(&v_o.clone(), &Py::str("q_col1"))?])).items()?;
    if t4233.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t4233[0].clone();
    v_q = t4233[1].clone();
    if (py::not(&{ let t4240 = { let t4234 = Py::Int(0i64); Py::Bool({ let t4235 = v_p.clone(); if py::lt(&t4234, &t4235)? { { let t4236 = Py::Int(1i64); if py::lt(&t4235, &t4236)? { true } else { false } } } else { false } }) }; if !t4240.truthy() { t4240 } else { { let t4237 = Py::Int(0i64); Py::Bool({ let t4238 = v_q.clone(); if py::lt(&t4237, &t4238)? { { let t4239 = Py::Int(1i64); if py::lt(&t4238, &t4239)? { true } else { false } } } else { false } }) } } })).truthy() {
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
    v_M = { let mut out = vec![]; for t4418 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("A"))?))? { let mut v_r: Py; 
v_r = t4418;
out.push({ let mut out = vec![]; for t4419 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t4419;
out.push(py::float(&v_v.clone())?); } Py::list(out) }); } Py::list(out) };
    let t4420 = (Py::tuple(vec![py::len(&v_M.clone())?, py::len(&py::getitem(&v_M.clone(), &Py::Int(0i64))?)?])).items()?;
    if t4420.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t4420[0].clone();
    v_n = t4420[1].clone();
    let t4421 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("row_strategy_example"))?, py::getitem(&v_o.clone(), &Py::str("col_strategy_example"))?, py::getitem(&v_o.clone(), &Py::str("value"))?])).items()?;
    if t4421.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_x = t4421[0].clone();
    v_y = t4421[1].clone();
    v_v = t4421[2].clone();
    if ({ let t4426 = { let t4422 = py::len(&v_x.clone())?; Py::Bool({ let t4423 = v_m.clone(); if py::ne(&t4422, &t4423) { true } else { false } }) }; if t4426.truthy() { t4426 } else { { let t4424 = py::len(&v_y.clone())?; Py::Bool({ let t4425 = v_n.clone(); if py::ne(&t4424, &t4425) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_sc = py::minmax(&[py::add(&Py::list(vec![Py::Float(1.0)]), &{ let mut out = vec![]; for t4427 in py::iter(&(v_M.clone()))? { let mut v_r: Py; 
v_r = t4427;
for t4428 in py::iter(&(v_r.clone()))? { let mut v_a: Py; 
v_a = t4428;
out.push(py::abs(&v_a.clone())?); }} Py::list(out) })?], true)?;
    v_bad = py::minmax(&[py::add(&Py::list(vec![py::abs(&py::sub(&py::sum(&v_x.clone(), None)?, &Py::Int(1i64))?)?, py::abs(&py::sub(&py::sum(&v_y.clone(), None)?, &Py::Int(1i64))?)?]), &{ let mut out = vec![]; for t4429 in py::iter(&(py::add(&v_x.clone(), &v_y.clone())?))? { let mut v_t: Py; 
v_t = t4429;
out.push(py::minmax(&[Py::Float(0.0), py::neg(&v_t.clone())?], true)?); } Py::list(out) })?], true)?;
    v_lo = py::minmax(&[{ let mut out = vec![]; for t4430 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t4430;
out.push(py::sum(&{ let mut out = vec![]; for t4431 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_r: Py; 
v_r = t4431;
out.push(py::mul(&py::getitem(&v_x.clone(), &v_r.clone())?, &py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_j.clone())?)?); } Py::list(out) }, None)?); } Py::list(out) }], false)?;
    v_hi = py::minmax(&[{ let mut out = vec![]; for t4432 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_r: Py; 
v_r = t4432;
out.push(py::sum(&{ let mut out = vec![]; for t4433 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t4433;
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
    let t2217 = (Py::tuple(vec![py::method(&Py::str(&py::to_str(&py::getitem(&v_i.clone(), &Py::str("source"))?)), "strip", &[])?, py::method(&Py::str(&py::to_str(&py::getitem(&v_i.clone(), &Py::str("target"))?)), "strip", &[])?])).items()?;
    if t2217.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t2217[0].clone();
    v_t = t2217[1].clone();
    v_f = { let mut out = vec![]; for t2218 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("edge_flows"))?))? { let mut v_v: Py; 
v_v = t2218;
out.push(f__fr(v_v.clone())?); } Py::list(out) };
    if ({ let t2229 = { let t2219 = py::len(&v_f.clone())?; Py::Bool({ let t2220 = py::len(&v_E.clone())?; if py::ne(&t2219, &t2220) { true } else { false } }) }; if t2229.truthy() { t2229 } else { Py::Bool(({ let mut out = vec![]; for t2221 in py::iter(&(py::zip(&[v_f.clone(), v_E.clone()])?))? { let mut v__: Py; let mut v_w: Py; let mut v_x: Py; 
let t2222 = (t2221).items()?;
if t2222.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2222[0].clone();
let t2223 = (t2222[1].clone()).items()?;
if t2223.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v__ = t2223[0].clone();
v__ = t2223[1].clone();
v_w = t2223[2].clone();
out.push({ let t2228 = { let t2224 = v_x.clone(); Py::Bool({ let t2225 = Py::Int(0i64); if py::lt(&t2224, &t2225)? { true } else { false } }) }; if t2228.truthy() { t2228 } else { { let t2226 = v_x.clone(); Py::Bool({ let t2227 = v_w.clone(); if py::gt(&t2226, &t2227)? { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_bal = Py::dict(vec![]);
    for t2230 in py::iter(&(py::zip(&[v_f.clone(), v_E.clone()])?))? {
        let t2231 = (t2230).items()?;
        if t2231.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_x = t2231[0].clone();
        let t2232 = (t2231[1].clone()).items()?;
        if t2232.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_u = t2232[0].clone();
        v_v = t2232[1].clone();
        v__ = t2232[2].clone();
        py::setitem(&v_bal.clone(), &v_u.clone(), py::sub(&py::method(&v_bal.clone(), "get", &[v_u.clone(), Py::Int(0i64)])?, &v_x.clone())?)?;
        py::setitem(&v_bal.clone(), &v_v.clone(), py::add(&py::method(&v_bal.clone(), "get", &[v_v.clone(), Py::Int(0i64)])?, &v_x.clone())?)?;
    }
    if (Py::Bool(({ let mut out = vec![]; for t2233 in py::iter(&(py::method(&v_bal.clone(), "items", &[])?))? { let mut v_b: Py; let mut v_k: Py; 
let t2234 = (t2233).items()?;
if t2234.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2234[0].clone();
v_b = t2234[1].clone();
if ({ let t2235 = v_k.clone(); Py::Bool({ let t2236 = Py::tuple(vec![v_s.clone(), v_t.clone()]); if !py::contains(&t2236, &t2235)? { true } else { false } }) }).truthy() { out.push({ let t2237 = v_b.clone(); Py::Bool({ let t2238 = Py::Int(0i64); if py::ne(&t2237, &t2238) { true } else { false } }) }); }} Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_S = py::set(&py::getitem(&v_o.clone(), &Py::str("min_cut_source_side"))?)?;
    if ({ let t2243 = { let t2239 = v_s.clone(); Py::Bool({ let t2240 = v_S.clone(); if !py::contains(&t2240, &t2239)? { true } else { false } }) }; if t2243.truthy() { t2243 } else { { let t2241 = v_t.clone(); Py::Bool({ let t2242 = v_S.clone(); if py::contains(&t2242, &t2241)? { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_cut = py::sum(&{ let mut out = vec![]; for t2244 in py::iter(&(v_E.clone()))? { let mut v_u: Py; let mut v_v: Py; let mut v_w: Py; 
let t2245 = (t2244).items()?;
if t2245.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_u = t2245[0].clone();
v_v = t2245[1].clone();
v_w = t2245[2].clone();
if ({ let t2250 = { let t2246 = v_u.clone(); Py::Bool({ let t2247 = v_S.clone(); if py::contains(&t2247, &t2246)? { true } else { false } }) }; if !t2250.truthy() { t2250 } else { { let t2248 = v_v.clone(); Py::Bool({ let t2249 = v_S.clone(); if !py::contains(&t2249, &t2248)? { true } else { false } }) } } }).truthy() { out.push(v_w.clone()); }} Py::list(out) }, None)?;
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
    let t2602: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t2601 = (f__mcf_lp(py::getitem(&v_i.clone(), &Py::str("capacity"))?, py::getitem(&v_i.clone(), &Py::str("cost"))?, py::getitem(&v_i.clone(), &Py::str("supply"))?)?).items()?;
            if t2601.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_arcs = t2601[0].clone();
            v_lp = t2601[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2602 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    v_fl = py::method(&v_o.clone(), "get", &[Py::str("flows"), py::method(&v_o.clone(), "get", &[Py::str("flows_example")])?])?;
    return Ok({ let mut v: Vec<Py> = vec![]; v.extend((v_lp.clone()).items()?); v.push({ let mut out = vec![]; for t2642 in py::iter(&(v_arcs.clone()))? { let mut v_a: Py; let mut v_b: Py; 
let t2643 = (t2642).items()?;
if t2643.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2643[0].clone();
v_b = t2643[1].clone();
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
    let t2380: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t2379 = (f__mdp_in(py::getitem(&v_i.clone(), &Py::str("R"))?, py::getitem(&v_i.clone(), &Py::str("T"))?, py::getitem(&v_i.clone(), &Py::str("gamma"))?)?).items()?;
            if t2379.len() != 5 { return Err(PyErr::Value("unpack".into())); }
            v_Rf = t2379[0].clone();
            v_P = t2379[1].clone();
            v_g = t2379[2].clone();
            v_S = t2379[3].clone();
            v_A = t2379[4].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2380 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    v_V = py::getitem(&v_o.clone(), &Py::str("values"))?;
    if ({ let t2381 = py::len(&v_V.clone())?; Py::Bool({ let t2382 = v_S.clone(); if py::ne(&t2381, &t2382) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_g = py::float(&v_g.clone())?;
    v_sc = py::minmax(&[py::add(&Py::list(vec![Py::Float(1.0)]), &{ let mut out = vec![]; for t2383 in py::iter(&(v_V.clone()))? { let mut v_v: Py; 
v_v = t2383;
out.push(py::abs(&v_v.clone())?); } Py::list(out) })?], true)?;
    v_Qv = { let mut out = vec![]; for t2384 in py::range_iter(&[(v_S.clone()).i()?]) { let mut v_s: Py; 
v_s = t2384;
out.push({ let mut out = vec![]; for t2385 in py::range_iter(&[(v_A.clone()).i()?]) { let mut v_a: Py; 
v_a = t2385;
out.push(py::add(&py::float(&py::getitem(&py::getitem(&v_Rf.clone(), &v_s.clone())?, &v_a.clone())?)?, &py::mul(&v_g.clone(), &py::fsum(&{ let mut out = vec![]; for t2386 in py::range_iter(&[(v_S.clone()).i()?]) { let mut v_t: Py; 
v_t = t2386;
out.push(py::mul(&py::float(&py::getitem(&py::getitem(&v_P.clone(), &py::add(&py::mul(&v_s.clone(), &v_A.clone())?, &v_a.clone())?)?, &v_t.clone())?)?, &py::getitem(&v_V.clone(), &v_t.clone())?)?); } Py::list(out) })?)?)?); } Py::list(out) }); } Py::list(out) };
    v_bell = py::truediv(&py::minmax(&[{ let mut out = vec![]; for t2387 in py::range_iter(&[(v_S.clone()).i()?]) { let mut v_s: Py; 
v_s = t2387;
out.push(py::abs(&py::sub(&py::getitem(&v_V.clone(), &v_s.clone())?, &py::minmax(&[py::getitem(&v_Qv.clone(), &v_s.clone())?], true)?)?)?); } Py::list(out) }], true)?, &v_sc.clone())?;
    v_pol = py::truediv(&py::minmax(&[{ let mut out = vec![]; for t2388 in py::range_iter(&[(v_S.clone()).i()?]) { let mut v_s: Py; 
v_s = t2388;
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
    let t2820 = ({ let mut out = vec![]; for t2818 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("A"))?))? { let mut v_r: Py; 
v_r = t2818;
out.push({ let mut out = vec![]; for t2819 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t2819;
out.push(py::float(&v_v.clone())?); } Py::list(out) }); } Py::list(out) }).items()?;
    if t2820.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    let t2821 = (t2820[0].clone()).items()?;
    if t2821.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a11 = t2821[0].clone();
    v_a12 = t2821[1].clone();
    let t2822 = (t2820[1].clone()).items()?;
    if t2822.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a21 = t2822[0].clone();
    v_a22 = t2822[1].clone();
    let t2825 = ({ let mut out = vec![]; for t2823 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("B"))?))? { let mut v_r: Py; 
v_r = t2823;
out.push({ let mut out = vec![]; for t2824 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t2824;
out.push(py::float(&v_v.clone())?); } Py::list(out) }); } Py::list(out) }).items()?;
    if t2825.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    let t2826 = (t2825[0].clone()).items()?;
    if t2826.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_b11 = t2826[0].clone();
    v_b12 = t2826[1].clone();
    let t2827 = (t2825[1].clone()).items()?;
    if t2827.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_b21 = t2827[0].clone();
    v_b22 = t2827[1].clone();
    let t2828 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("p_row1"))?, py::getitem(&v_o.clone(), &Py::str("q_col1"))?])).items()?;
    if t2828.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t2828[0].clone();
    v_q = t2828[1].clone();
    if (py::not(&{ let t2835 = { let t2829 = Py::Int(0i64); Py::Bool({ let t2830 = v_p.clone(); if py::lt(&t2829, &t2830)? { { let t2831 = Py::Int(1i64); if py::lt(&t2830, &t2831)? { true } else { false } } } else { false } }) }; if !t2835.truthy() { t2835 } else { { let t2832 = Py::Int(0i64); Py::Bool({ let t2833 = v_q.clone(); if py::lt(&t2832, &t2833)? { { let t2834 = Py::Int(1i64); if py::lt(&t2833, &t2834)? { true } else { false } } } else { false } }) } } })).truthy() {
        return Ok(Py::Float(1.0));
    }
    let t2836 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("row_payoff"))?, py::getitem(&v_o.clone(), &Py::str("col_payoff"))?])).items()?;
    if t2836.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_u = t2836[0].clone();
    v_w = t2836[1].clone();
    return Ok(py::minmax(&[f__rel(py::add(&py::mul(&v_q.clone(), &v_a11.clone())?, &py::mul(&py::sub(&Py::Int(1i64), &v_q.clone())?, &v_a12.clone())?)?, v_u.clone())?, f__rel(py::add(&py::mul(&v_q.clone(), &v_a21.clone())?, &py::mul(&py::sub(&Py::Int(1i64), &v_q.clone())?, &v_a22.clone())?)?, v_u.clone())?, f__rel(py::add(&py::mul(&v_p.clone(), &v_b11.clone())?, &py::mul(&py::sub(&Py::Int(1i64), &v_p.clone())?, &v_b21.clone())?)?, v_w.clone())?, f__rel(py::add(&py::mul(&v_p.clone(), &v_b12.clone())?, &py::mul(&py::sub(&Py::Int(1i64), &v_p.clone())?, &v_b22.clone())?)?, v_w.clone())?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_mm1(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_lam: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let t2847 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("lam"))?, py::getitem(&v_i.clone(), &Py::str("mu"))?])).items()?;
    if t2847.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lam = t2847[0].clone();
    v_mu = t2847[1].clone();
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
    let t2865 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("lam"))?, py::getitem(&v_i.clone(), &Py::str("mu"))?, py::int(&py::getitem(&v_i.clone(), &Py::str("c"))?)?])).items()?;
    if t2865.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_lam = t2865[0].clone();
    v_mu = t2865[1].clone();
    v_c = t2865[2].clone();
    let t2866 = (Py::tuple(vec![py::truediv(&v_lam.clone(), &v_mu.clone())?, py::truediv(&v_lam.clone(), &py::mul(&v_c.clone(), &v_mu.clone())?)?])).items()?;
    if t2866.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2866[0].clone();
    v_rho = t2866[1].clone();
    let t2867 = (Py::tuple(vec![Py::Float(0.0), Py::Float(1.0)])).items()?;
    if t2867.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_terms = t2867[0].clone();
    v_t = t2867[1].clone();
    for t2868 in py::range_iter(&[(v_c.clone()).i()?]) {
        v_n = t2868;
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
    let t2939 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("lam"))?, py::getitem(&v_i.clone(), &Py::str("mu"))?])).items()?;
    if t2939.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lam = t2939[0].clone();
    v_mu = t2939[1].clone();
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
    v_on = { let mut out = vec![]; for t2954 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("on_time_sequence"))?))? { let mut v_v: Py; 
v_v = t2954;
out.push(py::sub(&py::int(&py::round(&v_v.clone(), None)?)?, &Py::Int(1i64))?); } Py::list(out) };
    v_tardy = { let mut out = vec![]; for t2955 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("tardy_jobs"))?))? { let mut v_v: Py; 
v_v = t2955;
out.push(py::sub(&py::int(&py::round(&v_v.clone(), None)?)?, &Py::Int(1i64))?); } Py::list(out) };
    if ({ let t2960 = { let t2956 = py::sorted(&py::add(&v_on.clone(), &v_tardy.clone())?, (Py::Bool(false)).truthy())?; Py::Bool({ let t2957 = Py::list((Py::list((py::range_iter(&[(py::len(&py::getitem(&v_i.clone(), &Py::str("p"))?)?).i()?])).into_iter().collect::<Vec<Py>>())).items()?); if py::ne(&t2956, &t2957) { true } else { false } }) }; if t2960.truthy() { t2960 } else { { let t2958 = py::getitem(&v_o.clone(), &Py::str("min_tardy_jobs"))?; Py::Bool({ let t2959 = py::len(&v_tardy.clone())?; if py::ne(&t2958, &t2959) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    for t2963 in py::iter(&(py::method(&f__completion(v_on.clone(), py::getitem(&v_i.clone(), &Py::str("p"))?)?, "items", &[])?))? {
        let t2966 = (t2963).items()?;
        if t2966.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_j = t2966[0].clone();
        v_c = t2966[1].clone();
        if ({ let t2964 = v_c.clone(); Py::Bool({ let t2965 = f__fr(py::getitem(&py::getitem(&v_i.clone(), &Py::str("d"))?, &v_j.clone())?)?; if py::gt(&t2964, &t2965)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(1.0));
        }
    }
    return Ok((if ({ let t2967 = py::getitem(&f_moore_hodgson(py::getitem(&v_i.clone(), &Py::str("p"))?, py::getitem(&v_i.clone(), &Py::str("d"))?)?, &Py::str("min_tardy_jobs"))?; Py::Bool({ let t2968 = py::getitem(&v_o.clone(), &Py::str("min_tardy_jobs"))?; if py::eq(&t2967, &t2968) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }));
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
    if ({ let t2687 = py::len(&v_names.clone())?; Py::Bool({ let t2688 = py::sub(&py::len(&v_nodes.clone())?, &Py::Int(1i64))?; if py::ne(&t2687, &t2688) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_best = Py::dict(vec![]);
    for t2689 in py::iter(&(v_E.clone()))? {
        let t2691 = (t2689).items()?;
        if t2691.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_u = t2691[0].clone();
        v_v = t2691[1].clone();
        v_w = t2691[2].clone();
        for t2690 in py::iter(&(Py::tuple(vec![Py::str(&[py::format(&v_u.clone(), "")?, "-".to_string(), py::format(&v_v.clone(), "")?].concat()), Py::str(&[py::format(&v_v.clone(), "")?, "-".to_string(), py::format(&v_u.clone(), "")?].concat())])))? {
            v_key = t2690;
            py::setitem(&v_best.clone(), &v_key.clone(), py::minmax(&[py::method(&v_best.clone(), "get", &[v_key.clone(), v_w.clone()])?, v_w.clone()], false)?)?;
        }
    }
    if (Py::Bool(({ let mut out = vec![]; for t2692 in py::iter(&(v_names.clone()))? { let mut v_nm: Py; 
v_nm = t2692;
out.push({ let t2693 = v_nm.clone(); Py::Bool({ let t2694 = v_best.clone(); if !py::contains(&t2694, &t2693)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_tot = py::sum(&{ let mut out = vec![]; for t2695 in py::iter(&(v_names.clone()))? { let mut v_nm: Py; 
v_nm = t2695;
out.push(py::getitem(&v_best.clone(), &v_nm.clone())?); } Py::list(out) }, None)?;
    let t2696 = (Py::tuple(vec![py::set(&Py::list(vec![py::getitem(&v_nodes.clone(), &Py::Int(0i64))?]))?, py::fraction(&[Py::Int(0i64)])?])).items()?;
    if t2696.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_inn = t2696[0].clone();
    v_frontier = t2696[1].clone();
    while ({ let t2712 = py::len(&v_inn.clone())?; Py::Bool({ let t2713 = py::len(&v_nodes.clone())?; if py::lt(&t2712, &t2713)? { true } else { false } }) }).truthy() {
        v_cand = py::add(&{ let mut out = vec![]; for t2697 in py::iter(&(v_E.clone()))? { let mut v_u: Py; let mut v_v: Py; let mut v_w: Py; 
let t2698 = (t2697).items()?;
if t2698.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_u = t2698[0].clone();
v_v = t2698[1].clone();
v_w = t2698[2].clone();
if ({ let t2703 = { let t2699 = v_u.clone(); Py::Bool({ let t2700 = v_inn.clone(); if py::contains(&t2700, &t2699)? { true } else { false } }) }; if !t2703.truthy() { t2703 } else { { let t2701 = v_v.clone(); Py::Bool({ let t2702 = v_inn.clone(); if !py::contains(&t2702, &t2701)? { true } else { false } }) } } }).truthy() { out.push(Py::tuple(vec![v_w.clone(), v_v.clone()])); }} Py::list(out) }, &{ let mut out = vec![]; for t2704 in py::iter(&(v_E.clone()))? { let mut v_u: Py; let mut v_v: Py; let mut v_w: Py; 
let t2705 = (t2704).items()?;
if t2705.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_u = t2705[0].clone();
v_v = t2705[1].clone();
v_w = t2705[2].clone();
if ({ let t2710 = { let t2706 = v_v.clone(); Py::Bool({ let t2707 = v_inn.clone(); if py::contains(&t2707, &t2706)? { true } else { false } }) }; if !t2710.truthy() { t2710 } else { { let t2708 = v_u.clone(); Py::Bool({ let t2709 = v_inn.clone(); if !py::contains(&t2709, &t2708)? { true } else { false } }) } } }).truthy() { out.push(Py::tuple(vec![v_w.clone(), v_u.clone()])); }} Py::list(out) })?;
        if (py::not(&v_cand.clone())).truthy() {
            return Ok(Py::Float(1.0));
        }
        let t2711 = (py::minmax(&[v_cand.clone()], false)?).items()?;
        if t2711.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_w = t2711[0].clone();
        v_v = t2711[1].clone();
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
    for t2290 in py::iter(&(v_E.clone()))? {
        let t2292 = (t2290).items()?;
        if t2292.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_u = t2292[0].clone();
        v_v = t2292[1].clone();
        v_w = t2292[2].clone();
        let t2291 = py::minmax(&[py::method(&v_W.clone(), "get", &[Py::tuple(vec![v_u.clone(), v_v.clone()]), v_w.clone()])?, v_w.clone()], true)?;
        py::setitem(&v_W.clone(), &Py::tuple(vec![v_u.clone(), v_v.clone()]), t2291.clone())?;
        py::setitem(&v_W.clone(), &Py::tuple(vec![v_v.clone(), v_u.clone()]), t2291.clone())?;
    }
    v_pairs = { let mut out = vec![]; for t2293 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("matching_example"))?))? { let mut v_s: Py; 
v_s = t2293;
out.push(Py::tuple((py::method(&v_s.clone(), "split", &[Py::str("-"), Py::Int(1i64)])?).items()?)); } Py::list(out) };
    v_used = { let mut out = vec![]; for t2294 in py::iter(&(v_pairs.clone()))? { let mut v_p: Py; 
v_p = t2294;
for t2295 in py::iter(&(v_p.clone()))? { let mut v_x: Py; 
v_x = t2295;
out.push(v_x.clone()); }} Py::list(out) };
    if ({ let t2301 = { let t2296 = py::len(&v_used.clone())?; Py::Bool({ let t2297 = py::len(&py::set(&v_used.clone())?)?; if py::ne(&t2296, &t2297) { true } else { false } }) }; if t2301.truthy() { t2301 } else { Py::Bool(({ let mut out = vec![]; for t2298 in py::iter(&(v_pairs.clone()))? { let mut v_p: Py; 
v_p = t2298;
out.push({ let t2299 = v_p.clone(); Py::Bool({ let t2300 = v_W.clone(); if !py::contains(&t2300, &t2299)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::minmax(&[f__rel(py::getitem(&v_o.clone(), &Py::str("max_weight"))?, py::float(&py::sum(&{ let mut out = vec![]; for t2302 in py::iter(&(v_pairs.clone()))? { let mut v_p: Py; 
v_p = t2302;
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
    v_ks = { let mut out = vec![]; for t3086 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("machine_of_job"))?))? { let mut v_v: Py; 
v_v = t3086;
out.push(py::sub(&py::int(&py::round(&v_v.clone(), None)?)?, &Py::Int(1i64))?); } Py::list(out) };
    if ({ let t3093 = { let t3087 = py::len(&v_ks.clone())?; Py::Bool({ let t3088 = py::len(&py::getitem(&v_i.clone(), &Py::str("p"))?)?; if py::ne(&t3087, &t3088) { true } else { false } }) }; if t3093.truthy() { t3093 } else { Py::Bool(({ let mut out = vec![]; for t3089 in py::iter(&(v_ks.clone()))? { let mut v_k: Py; 
v_k = t3089;
out.push(py::not(&{ let t3090 = Py::Int(0i64); Py::Bool({ let t3091 = v_k.clone(); if py::le(&t3090, &t3091)? { { let t3092 = v_m.clone(); if py::lt(&t3091, &t3092)? { true } else { false } } } else { false } }) })); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_loads = py::mul(&Py::list(vec![Py::Float(0.0)]), &v_m.clone())?;
    for t3094 in py::iter(&(py::enumerate(&v_ks.clone(), 0)?))? {
        let t3097 = (t3094).items()?;
        if t3097.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_j = t3097[0].clone();
        v_k = t3097[1].clone();
        let t3095 = v_loads.clone(); let t3096 = v_k.clone();
        py::setitem(&t3095, &t3096, py::iadd(&py::getitem(&t3095, &t3096)?, &py::getitem(&py::getitem(&v_i.clone(), &Py::str("p"))?, &v_j.clone())?)?)?;
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
    v_cs = { let mut out = vec![]; for t3241 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("coefficients"))?))? { let mut v_v: Py; 
v_v = t3241;
out.push(py::float(&v_v.clone())?); } Py::list(out) };
    let t3242 = (Py::tuple(vec![py::float(&py::getitem(&v_i.clone(), &Py::str("lo"))?)?, py::float(&py::getitem(&v_i.clone(), &Py::str("hi"))?)?])).items()?;
    if t3242.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t3242[0].clone();
    v_hi = t3242[1].clone();
    v_cand = py::add(&Py::list(vec![v_lo.clone(), v_hi.clone()]), &f__roots_in(f__pder(v_cs.clone())?, v_lo.clone(), v_hi.clone())?)?;
    v_vals = { let mut out = vec![]; for t3243 in py::iter(&(v_cand.clone()))? { let mut v_x: Py; 
v_x = t3243;
out.push(f__peval(v_cs.clone(), v_x.clone())?); } Py::list(out) };
    v_out = Py::list(vec![f__rel(py::getitem(&v_o.clone(), &Py::str("min_value"))?, py::minmax(&[v_vals.clone()], false)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("max_value"))?, py::minmax(&[v_vals.clone()], true)?)?]);
    for t3244 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::str("argmin_example"), py::getitem(&v_o.clone(), &Py::str("min_value"))?]), Py::tuple(vec![Py::str("argmax_example"), py::getitem(&v_o.clone(), &Py::str("max_value"))?])])))? {
        let t3248 = (t3244).items()?;
        if t3248.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_key = t3248[0].clone();
        v_v = t3248[1].clone();
        v_x = py::getitem(&v_o.clone(), &v_key.clone())?;
        let _ = py::method(&v_out.clone(), "append", &[(if (py::not(&{ let t3245 = v_lo.clone(); Py::Bool({ let t3246 = v_x.clone(); if py::le(&t3245, &t3246)? { { let t3247 = v_hi.clone(); if py::le(&t3246, &t3247)? { true } else { false } } } else { false } }) })).truthy() { Py::Float(1.0) } else { f__rel(f__peval(v_cs.clone(), v_x.clone())?, v_v.clone())? })])?;
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
    v_te = { let mut out = vec![]; for t3171 in py::iter(&(v_times.clone()))? { let mut v_a: Py; let mut v_b: Py; let mut v_m: Py; 
let t3172 = (t3171).items()?;
if t3172.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_a = t3172[0].clone();
v_m = t3172[1].clone();
v_b = t3172[2].clone();
out.push(py::truediv(&py::add(&py::add(&v_a.clone(), &py::mul(&Py::Int(4i64), &v_m.clone())?)?, &v_b.clone())?, &Py::Int(6i64))?); } Py::list(out) };
    v_names = { let mut out = vec![]; for t3173 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("critical_path"))?))? { let mut v_c: Py; 
v_c = t3173;
out.push(py::sub(&py::ord(&v_c.clone())?, &Py::Int(65i64))?); } Py::list(out) };
    v_worst = f__rel(py::getitem(&v_o.clone(), &Py::str("expected_duration"))?, py::sum(&{ let mut out = vec![]; for t3174 in py::iter(&(v_names.clone()))? { let mut v_k: Py; 
v_k = t3174;
out.push(py::getitem(&v_te.clone(), &v_k.clone())?); } Py::list(out) }, None)?)?;
    v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_o.clone(), &Py::str("variance"))?, py::sum(&{ let mut out = vec![]; for t3175 in py::iter(&(v_names.clone()))? { let mut v_k: Py; 
v_k = t3175;
out.push(py::pow(&py::truediv(&py::sub(&py::getitem(&py::getitem(&v_times.clone(), &v_k.clone())?, &Py::Int(2i64))?, &py::getitem(&py::getitem(&v_times.clone(), &v_k.clone())?, &Py::Int(0i64))?)?, &Py::Int(6i64))?, &Py::Int(2i64))?); } Py::list(out) }, None)?)?], true)?;
    let t3176: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f_pert(v_times.clone(), py::method(&v_i.clone(), "get", &[Py::str("precedences")])?, py::method(&v_i.clone(), "get", &[Py::str("deadline")])?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3176 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_o.clone(), &Py::str("expected_duration"))?, py::getitem(&v_r.clone(), &Py::str("expected_duration"))?)?], true)?;
    if ({ let t3177 = Py::str("probability"); Py::Bool({ let t3178 = v_r.clone(); if py::contains(&t3178, &t3177)? { true } else { false } }) }).truthy() {
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
    let t3289: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_eq = { let mut v: Vec<Py> = vec![]; v.extend((f__bimatrix(py::getitem(&v_i.clone(), &Py::str("A"))?, py::getitem(&v_i.clone(), &Py::str("B"))?, Py::Int(8i64))?).items()?); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "MA") { Some(x) => x, None => { return Err(PyErr::Type("_pure_ne() missing argument MA".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "MB") { Some(x) => x, None => { return Err(PyErr::Type("_pure_ne() missing argument MB".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("_pure_ne() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("_pure_ne() got an unexpected keyword argument".into())) } f__pure_ne(a0, a1)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3289 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    v_same = { let t3290 = { let mut out = vec![]; for t3292 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("equilibria"))?))? { let mut v_a: Py; let mut v_b: Py; 
let t3293 = (t3292).items()?;
if t3293.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3293[0].clone();
v_b = t3293[1].clone();
out.push(Py::list(vec![py::int(&py::round(&v_a.clone(), None)?)?, py::int(&py::round(&v_b.clone(), None)?)?])); } Py::list(out) }; Py::Bool({ let t3291 = v_eq.clone(); if py::eq(&t3290, &t3291) { true } else { false } }) };
    return Ok((if ({ let t3296 = v_same.clone(); if !t3296.truthy() { t3296 } else { { let t3294 = py::getitem(&v_o.clone(), &Py::str("count"))?; Py::Bool({ let t3295 = py::len(&v_eq.clone())?; if py::eq(&t3294, &t3295) { true } else { false } }) } } }).truthy() { Py::Float(0.0) } else { py::minmax(&[Py::Float(1.0), py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("count"))?, &py::len(&v_eq.clone())?)?)?], true)? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_qmin(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t3965: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_M = f__sym_pd(py::getitem(&v_i.clone(), &Py::str("Q"))?, Py::str("Q"))?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3965 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    v_x = py::getitem(&v_o.clone(), &Py::str("x_star"))?;
    if ({ let t3966 = py::len(&v_x.clone())?; Py::Bool({ let t3967 = py::len(&v_M.clone())?; if py::ne(&t3966, &t3967) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_f = py::add(&py::mul(&Py::Float(0.5), &py::fsum(&{ let mut out = vec![]; for t3968 in py::range_iter(&[(py::len(&v_x.clone())?).i()?]) { let mut v_a: Py; 
v_a = t3968;
for t3969 in py::range_iter(&[(py::len(&v_x.clone())?).i()?]) { let mut v_b: Py; 
v_b = t3969;
out.push(py::mul(&py::mul(&py::getitem(&v_x.clone(), &v_a.clone())?, &py::float(&py::getitem(&py::getitem(&v_M.clone(), &v_a.clone())?, &v_b.clone())?)?)?, &py::getitem(&v_x.clone(), &v_b.clone())?)?); }} Py::list(out) })?)?, &py::fsum(&{ let mut out = vec![]; for t3970 in py::iter(&(py::zip(&[py::getitem(&v_i.clone(), &Py::str("c"))?, v_x.clone()])?))? { let mut v_t: Py; let mut v_v: Py; 
let t3971 = (t3970).items()?;
if t3971.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t3971[0].clone();
v_t = t3971[1].clone();
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
    let t3535: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_M = f__sym_pd(py::getitem(&v_i.clone(), &Py::str("Q"))?, Py::str("Q"))?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3535 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    v_n = py::len(&v_M.clone())?;
    let t3540 = (Py::tuple(vec![{ let mut out = vec![]; for t3536 in py::iter(&(v_M.clone()))? { let mut v_r: Py; 
v_r = t3536;
out.push({ let mut out = vec![]; for t3537 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t3537;
out.push(py::float(&v_v.clone())?); } Py::list(out) }); } Py::list(out) }, { let mut out = vec![]; for t3538 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("c"))?))? { let mut v_v: Py; 
v_v = t3538;
out.push(py::float(&v_v.clone())?); } Py::list(out) }, { let mut out = vec![]; for t3539 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("x0"))?))? { let mut v_v: Py; 
v_v = t3539;
out.push(py::float(&v_v.clone())?); } Py::list(out) }])).items()?;
    if t3540.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Qf = t3540[0].clone();
    v_c = t3540[1].clone();
    v_x0 = t3540[2].clone();
    v_g = { let mut out = vec![]; for t3541 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_a: Py; 
v_a = t3541;
out.push(py::add(&py::fsum(&{ let mut out = vec![]; for t3542 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_b: Py; 
v_b = t3542;
out.push(py::mul(&py::getitem(&py::getitem(&v_Qf.clone(), &v_a.clone())?, &v_b.clone())?, &py::getitem(&v_x0.clone(), &v_b.clone())?)?); } Py::list(out) })?, &py::getitem(&v_c.clone(), &v_a.clone())?)?); } Py::list(out) };
    v_x1 = py::getitem(&v_o.clone(), &Py::str("x1"))?;
    v_g1 = { let mut out = vec![]; for t3543 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_a: Py; 
v_a = t3543;
out.push(py::add(&py::fsum(&{ let mut out = vec![]; for t3544 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_b: Py; 
v_b = t3544;
out.push(py::mul(&py::getitem(&py::getitem(&v_Qf.clone(), &v_a.clone())?, &v_b.clone())?, &py::getitem(&v_x1.clone(), &v_b.clone())?)?); } Py::list(out) })?, &py::getitem(&v_c.clone(), &v_a.clone())?)?); } Py::list(out) };
    v_gn = py::fsum(&{ let mut out = vec![]; for t3545 in py::iter(&(v_g.clone()))? { let mut v_v: Py; 
v_v = t3545;
out.push(py::mul(&v_v.clone(), &v_v.clone())?); } Py::list(out) })?;
    v_orth = py::truediv(&py::abs(&py::fsum(&{ let mut out = vec![]; for t3546 in py::iter(&(py::zip(&[v_g.clone(), v_g1.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3547 = (t3546).items()?;
if t3547.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3547[0].clone();
v_b = t3547[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?)?, &v_gn.clone())?;
    v_step = py::minmax(&[{ let mut out = vec![]; for t3548 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_a: Py; 
v_a = t3548;
out.push(f__rel(py::getitem(&v_x1.clone(), &v_a.clone())?, py::sub(&py::getitem(&v_x0.clone(), &v_a.clone())?, &py::mul(&py::getitem(&v_o.clone(), &Py::str("alpha"))?, &py::getitem(&v_g.clone(), &v_a.clone())?)?)?)?); } Py::list(out) }], true)?;
    v_f1 = py::add(&py::mul(&Py::Float(0.5), &py::fsum(&{ let mut out = vec![]; for t3549 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_a: Py; 
v_a = t3549;
for t3550 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_b: Py; 
v_b = t3550;
out.push(py::mul(&py::mul(&py::getitem(&v_x1.clone(), &v_a.clone())?, &py::getitem(&py::getitem(&v_Qf.clone(), &v_a.clone())?, &v_b.clone())?)?, &py::getitem(&v_x1.clone(), &v_b.clone())?)?); }} Py::list(out) })?)?, &py::fsum(&{ let mut out = vec![]; for t3551 in py::iter(&(py::zip(&[v_c.clone(), v_x1.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3552 = (t3551).items()?;
if t3552.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3552[0].clone();
v_b = t3552[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?)?;
    return Ok(py::minmax(&[v_orth.clone(), v_step.clone(), f__rel(py::getitem(&v_o.clone(), &Py::str("f1"))?, v_f1.clone())?, py::minmax(&[{ let mut out = vec![]; for t3553 in py::iter(&(py::zip(&[py::getitem(&v_o.clone(), &Py::str("gradient"))?, v_g.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3554 = (t3553).items()?;
if t3554.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3554[0].clone();
v_b = t3554[1].clone();
out.push(f__rel(v_a.clone(), v_b.clone())?); } Py::list(out) }], true)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_seq(mut v_i: Py, mut v_o: Py, mut v_key_fn: Py, mut v_value_fn: Py, mut v_value_key: Py, mut v_n_key: Py) -> R {
    let mut v_keys: Py = Py::None;
    let mut v_seq: Py = Py::None;
    v_seq = { let mut out = vec![]; for t4185 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("sequence"))?))? { let mut v_v: Py; 
v_v = t4185;
out.push(py::sub(&py::int(&py::round(&v_v.clone(), None)?)?, &Py::Int(1i64))?); } Py::list(out) };
    if ({ let t4186 = py::sorted(&v_seq.clone(), (Py::Bool(false)).truthy())?; Py::Bool({ let t4187 = Py::list((Py::list((py::range_iter(&[(py::len(&py::getitem(&v_i.clone(), &v_n_key.clone())?)?).i()?])).into_iter().collect::<Vec<Py>>())).items()?); if py::ne(&t4186, &t4187) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_keys = { let mut out = vec![]; for t4188 in py::iter(&(v_seq.clone()))? { let mut v_j: Py; 
v_j = t4188;
out.push(py::call(&v_key_fn.clone(), &[v_i.clone(), v_j.clone()])?); } Py::list(out) };
    if (Py::Bool(({ let mut out = vec![]; for t4189 in py::iter(&(py::zip(&[v_keys.clone(), py::slice(&v_keys.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_k1: Py; let mut v_k2: Py; 
let t4190 = (t4189).items()?;
if t4190.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k1 = t4190[0].clone();
v_k2 = t4190[1].clone();
out.push({ let t4191 = v_k2.clone(); Py::Bool({ let t4192 = v_k1.clone(); if py::lt(&t4191, &t4192)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
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
    let t3456: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_E = f__wedges(py::getitem(&v_i.clone(), &Py::str("edges"))?, Py::Bool(true))?;
            v_dist = f__bellman(v_E.clone(), py::method(&Py::str(&py::to_str(&py::getitem(&v_i.clone(), &Py::str("source"))?)), "strip", &[])?, py::getitem(&v_i.clone(), &Py::str("directed"))?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3456 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    v_t = py::method(&Py::str(&py::to_str(&py::getitem(&v_i.clone(), &Py::str("target"))?)), "strip", &[])?;
    v_path = py::method(&v_o.clone(), "get", &[Py::str("path"), py::method(&v_o.clone(), "get", &[Py::str("path_example")])?])?;
    if ({ let t3465 = { let t3457 = v_t.clone(); Py::Bool({ let t3458 = v_dist.clone(); if !py::contains(&t3458, &t3457)? { true } else { false } }) }; if t3465.truthy() { t3465 } else { { let t3464 = py::not(&v_path.clone()); if t3464.truthy() { t3464 } else { { let t3463 = { let t3459 = py::getitem(&v_path.clone(), &Py::Int(0i64))?; Py::Bool({ let t3460 = py::method(&Py::str(&py::to_str(&py::getitem(&v_i.clone(), &Py::str("source"))?)), "strip", &[])?; if py::ne(&t3459, &t3460) { true } else { false } }) }; if t3463.truthy() { t3463 } else { { let t3461 = py::getitem(&v_path.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t3462 = v_t.clone(); if py::ne(&t3461, &t3462) { true } else { false } }) } } } } } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_adj = f__adj(v_E.clone(), py::getitem(&v_i.clone(), &Py::str("directed"))?)?;
    v_tot = py::fraction(&[Py::Int(0i64)])?;
    for t3466 in py::iter(&(py::zip(&[v_path.clone(), py::slice(&v_path.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? {
        let t3471 = (t3466).items()?;
        if t3471.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_u = t3471[0].clone();
        v_v = t3471[1].clone();
        v_ws = { let mut out = vec![]; for t3467 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? { let mut v_w: Py; let mut v_x: Py; 
let t3468 = (t3467).items()?;
if t3468.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t3468[0].clone();
v_w = t3468[1].clone();
if ({ let t3469 = v_x.clone(); Py::Bool({ let t3470 = v_v.clone(); if py::eq(&t3469, &t3470) { true } else { false } }) }).truthy() { out.push(v_w.clone()); }} Py::list(out) };
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
    if ({ let t2907 = v_c.clone(); Py::Bool({ let t2908 = py::int(&v_c.clone())?; if py::ne(&t2907, &t2908) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_c = py::int(&v_c.clone())?;
    let t2909: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f_mmc(py::getitem(&v_i.clone(), &Py::str("lam"))?, py::getitem(&v_i.clone(), &Py::str("mu"))?, v_c.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2909 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    v_crit = py::method(&v_i.clone(), "get", &[Py::str("criterion"), Py::str("wait_probability")])?;
    v_val = (if ({ let t2910 = v_crit.clone(); Py::Bool({ let t2911 = Py::str("wait_probability"); if py::eq(&t2910, &t2911) { true } else { false } }) }).truthy() { py::getitem(&v_r.clone(), &Py::str("P_wait"))? } else { py::getitem(&v_r.clone(), &Py::str("Wq"))? });
    v_worst = py::minmax(&[f__rel(py::getitem(&v_o.clone(), &Py::str("P_wait"))?, py::getitem(&v_r.clone(), &Py::str("P_wait"))?)?, py::truediv(&py::minmax(&[Py::Float(0.0), py::sub(&v_val.clone(), &py::getitem(&v_i.clone(), &Py::str("target"))?)?], true)?, &py::getitem(&v_i.clone(), &Py::str("target"))?)?], true)?;
    if ({ let t2912 = py::sub(&v_c.clone(), &Py::Int(1i64))?; Py::Bool({ let t2913 = py::truediv(&py::getitem(&v_i.clone(), &Py::str("lam"))?, &py::getitem(&v_i.clone(), &Py::str("mu"))?)?; if py::gt(&t2912, &t2913)? { true } else { false } }) }).truthy() {
        v_r2 = f_mmc(py::getitem(&v_i.clone(), &Py::str("lam"))?, py::getitem(&v_i.clone(), &Py::str("mu"))?, py::sub(&v_c.clone(), &Py::Int(1i64))?)?;
        v_v2 = (if ({ let t2914 = v_crit.clone(); Py::Bool({ let t2915 = Py::str("wait_probability"); if py::eq(&t2914, &t2915) { true } else { false } }) }).truthy() { py::getitem(&v_r2.clone(), &Py::str("P_wait"))? } else { py::getitem(&v_r2.clone(), &Py::str("Wq"))? });
        v_worst = py::minmax(&[v_worst.clone(), (if ({ let t2916 = v_v2.clone(); Py::Bool({ let t2917 = py::getitem(&v_i.clone(), &Py::str("target"))?; if py::gt(&t2916, &t2917)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) })], true)?;
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
    let t3828: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t3827 = (f__transport_lp(py::getitem(&v_i.clone(), &Py::str("supply"))?, py::getitem(&v_i.clone(), &Py::str("demand"))?, py::getitem(&v_i.clone(), &Py::str("cost"))?)?).items()?;
            if t3827.len() != 5 { return Err(PyErr::Value("unpack".into())); }
            v_sense = t3827[0].clone();
            v_c = t3827[1].clone();
            v_A = t3827[2].clone();
            v_b = t3827[3].clone();
            v_rel = t3827[4].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3828 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    v_plan = py::method(&v_o.clone(), "get", &[Py::str("shipments"), py::method(&v_o.clone(), "get", &[Py::str("shipments_example")])?])?;
    v_x = { let mut out = vec![]; for t3829 in py::iter(&(v_plan.clone()))? { let mut v_r: Py; 
v_r = t3829;
for t3830 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t3830;
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
    v_t = { let mut out = vec![]; for t3899 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("tour_example"))?))? { let mut v_x: Py; 
v_x = t3899;
out.push(py::sub(&py::int(&py::round(&v_x.clone(), None)?)?, &Py::Int(1i64))?); } Py::list(out) };
    v_n = py::len(&v_D.clone())?;
    if ({ let t3907 = { let t3900 = py::getitem(&v_t.clone(), &Py::Int(0i64))?; Py::Bool({ let t3901 = Py::Int(0i64); if py::ne(&t3900, &t3901) { true } else { false } }) }; if t3907.truthy() { t3907 } else { { let t3906 = { let t3902 = py::getitem(&v_t.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t3903 = Py::Int(0i64); if py::ne(&t3902, &t3903) { true } else { false } }) }; if t3906.truthy() { t3906 } else { { let t3904 = py::sorted(&py::slice(&v_t.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?, (Py::Bool(false)).truthy())?; Py::Bool({ let t3905 = Py::list((Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>())).items()?); if py::ne(&t3904, &t3905) { true } else { false } }) } } } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_worst = f__rel(py::getitem(&v_o.clone(), &Py::str("min_tour_length"))?, py::sum(&{ let mut out = vec![]; for t3908 in py::iter(&(py::zip(&[v_t.clone(), py::slice(&v_t.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3909 = (t3908).items()?;
if t3909.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3909[0].clone();
v_b = t3909[1].clone();
out.push(py::getitem(&py::getitem(&v_D.clone(), &v_a.clone())?, &v_b.clone())?); } Py::list(out) }, None)?)?;
    v_key = Py::str(&py::repr(&v_D.clone()));
    if ({ let t3910 = v_key.clone(); Py::Bool({ let t3911 = c__TSP_MEMO()?; if !py::contains(&t3911, &t3910)? { true } else { false } }) }).truthy() {
        let _ = (if ({ let t3912 = py::len(&c__TSP_MEMO()?)?; Py::Bool({ let t3913 = Py::Int(64i64); if py::gt(&t3912, &t3913)? { true } else { false } }) }).truthy() { py::method(&c__TSP_MEMO()?, "clear", &[])? } else { Py::None });
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
    let t2750 = (f__simple_graph(py::getitem(&v_i.clone(), &Py::str("edges"))?, Py::Int(30i64))?).items()?;
    if t2750.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_nodes = t2750[0].clone();
    v_adj = t2750[1].clone();
    v_cov = py::set(&py::getitem(&v_o.clone(), &Py::str("cover_example"))?)?;
    if ({ let t2763 = py::not(&{ let t2751 = v_cov.clone(); Py::Bool({ let t2752 = py::set(&v_nodes.clone())?; if py::set_cmp(&t2751, &t2752, "<=")? { true } else { false } }) }); if t2763.truthy() { t2763 } else { { let t2762 = Py::Bool(({ let mut out = vec![]; for t2753 in py::iter(&(v_nodes.clone()))? { let mut v_u: Py; 
v_u = t2753;
for t2754 in py::iter(&(py::getitem(&v_adj.clone(), &v_u.clone())?))? { let mut v_v: Py; 
v_v = t2754;
out.push({ let t2759 = { let t2755 = v_u.clone(); Py::Bool({ let t2756 = v_cov.clone(); if !py::contains(&t2756, &t2755)? { true } else { false } }) }; if !t2759.truthy() { t2759 } else { { let t2757 = v_v.clone(); Py::Bool({ let t2758 = v_cov.clone(); if !py::contains(&t2758, &t2757)? { true } else { false } }) } } }); }} Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t2762.truthy() { t2762 } else { { let t2760 = py::len(&v_cov.clone())?; Py::Bool({ let t2761 = py::getitem(&v_o.clone(), &Py::str("min_cover_size"))?; if py::ne(&t2760, &t2761) { true } else { false } }) } } } } }).truthy() {
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
    let t4024 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("order_quantities_example"))?, { let mut out = vec![]; for t4023 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("demand"))?))? { let mut v_v: Py; 
v_v = t4023;
out.push(py::float(&v_v.clone())?); } Py::list(out) }])).items()?;
    if t4024.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_q = t4024[0].clone();
    v_d = t4024[1].clone();
    if ({ let t4029 = { let t4025 = py::len(&v_q.clone())?; Py::Bool({ let t4026 = py::len(&v_d.clone())?; if py::ne(&t4025, &t4026) { true } else { false } }) }; if t4029.truthy() { t4029 } else { { let t4027 = py::minmax(&[v_q.clone()], false)?; Py::Bool({ let t4028 = Py::Int(0i64); if py::lt(&t4027, &t4028)? { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    let t4030 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0)])).items()?;
    if t4030.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_inv = t4030[0].clone();
    v_cost = t4030[1].clone();
    for t4031 in py::iter(&(py::zip(&[v_q.clone(), v_d.clone()])?))? {
        let t4036 = (t4031).items()?;
        if t4036.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_qt = t4036[0].clone();
        v_dt = t4036[1].clone();
        v_inv = py::iadd(&v_inv, &py::sub(&v_qt.clone(), &v_dt.clone())?)?;
        if ({ let t4032 = v_inv.clone(); Py::Bool({ let t4033 = py::mul(&py::neg(&Py::Float(1e-09))?, &py::minmax(&[Py::Float(1.0), v_dt.clone()], true)?)?; if py::lt(&t4032, &t4033)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(1.0));
        }
        v_cost = py::iadd(&v_cost, &py::add(&(if ({ let t4034 = v_qt.clone(); Py::Bool({ let t4035 = Py::Int(0i64); if py::gt(&t4034, &t4035)? { true } else { false } }) }).truthy() { py::float(&py::getitem(&v_i.clone(), &Py::str("K"))?)? } else { Py::Float(0.0) }), &py::mul(&py::float(&py::getitem(&v_i.clone(), &Py::str("h"))?)?, &v_inv.clone())?)?)?;
    }
    return Ok(py::minmax(&[f__rel(py::getitem(&v_o.clone(), &Py::str("min_cost"))?, v_cost.clone())?, f__rel(py::getitem(&v_o.clone(), &Py::str("min_cost"))?, py::getitem(&f_wagner_whitin(py::getitem(&v_i.clone(), &Py::str("demand"))?, py::getitem(&v_i.clone(), &Py::str("K"))?, py::getitem(&v_i.clone(), &Py::str("h"))?)?, &Py::str("min_cost"))?)?, (if ({ let t4037 = py::getitem(&v_o.clone(), &Py::str("orders"))?; Py::Bool({ let t4038 = py::sum(&{ let mut out = vec![]; for t4039 in py::iter(&(v_q.clone()))? { let mut v_v: Py; 
v_v = t4039;
if ({ let t4040 = v_v.clone(); Py::Bool({ let t4041 = Py::Int(0i64); if py::gt(&t4040, &t4041)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?; if py::eq(&t4037, &t4038) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) })], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__wedges(mut v_edges: Py, mut v_need_weight: Py) -> R {
    if (py::not(&v_edges.clone())).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give at least one edge"))));
    }
    if ({ let t3381 = v_need_weight.clone(); if !t3381.truthy() { t3381 } else { Py::Bool(({ let mut out = vec![]; for t3378 in py::iter(&(v_edges.clone()))? { let mut v_e: Py; 
v_e = t3378;
out.push({ let t3379 = py::len(&v_e.clone())?; Py::Bool({ let t3380 = Py::Int(3i64); if py::ne(&t3379, &t3380) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("every edge needs a weight: 'A-B 4'"))));
    }
    return Ok((if (v_need_weight.clone()).truthy() { { let mut out = vec![]; for t3382 in py::iter(&(v_edges.clone()))? { let mut v_e: Py; 
v_e = t3382;
out.push(Py::tuple(vec![Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(0i64))?)), Py::str(&py::to_str(&py::getitem(&v_e.clone(), &Py::Int(1i64))?)), f__fr(py::getitem(&v_e.clone(), &Py::Int(2i64))?)?])); } Py::list(out) } } else { { let mut out = vec![]; for t3383 in py::iter(&(v_edges.clone()))? { let mut v_e: Py; 
v_e = t3383;
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
fn f_lp_sensitivity(mut v_sense: Py, mut v_c: Py, mut v_A: Py, mut v_b: Py, mut v_relations: Py) -> R {
    let mut v_As: Py = Py::None;
    let mut v_Bi: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v__x: Py = Py::None;
    let mut v__y: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_alpha: Py = Py::None;
    let mut v_basis: Py = Py::None;
    let mut v_bi: Py = Py::None;
    let mut v_bs: Py = Py::None;
    let mut v_cB: Py = Py::None;
    let mut v_cj: Py = Py::None;
    let mut v_col: Py = Py::None;
    let mut v_cost_hi: Py = Py::None;
    let mut v_cost_lo: Py = Py::None;
    let mut v_cs: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_hi_d: Py = Py::None;
    let mut v_hi_m: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_lo_d: Py = Py::None;
    let mut v_lo_m: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_nonbasic: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_pos: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_red: Py = Py::None;
    let mut v_redundant: Py = Py::None;
    let mut v_rel: Py = Py::None;
    let mut v_rhs_hi: Py = Py::None;
    let mut v_rhs_lo: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_signs: Py = Py::None;
    let mut v_status: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_value: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_xB: Py = Py::None;
    let mut v_yv: Py = Py::None;
    v_rel = f__relations(v_relations.clone(), py::len(&v_A.clone())?)?;
    let _ = f__check_lp(v_sense.clone(), v_c.clone(), v_A.clone(), v_b.clone(), v_rel.clone())?;
    let t1524 = (f__standard(v_sense.clone(), v_c.clone(), v_A.clone(), v_b.clone(), v_rel.clone())?).items()?;
    if t1524.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_As = t1524[0].clone();
    v_bs = t1524[1].clone();
    v_cs = t1524[2].clone();
    let t1596 = (f__simplex_eq(v_As.clone(), v_bs.clone(), v_cs.clone(), Py::Int(20000i64))?).items()?;
    if t1596.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_status = t1596[0].clone();
    v__x = t1596[1].clone();
    v__y = t1596[2].clone();
    v_basis = t1596[3].clone();
    v_redundant = t1596[4].clone();
    if ({ let t1597 = v_status.clone(); Py::Bool({ let t1598 = Py::str("infeasible"); if py::eq(&t1597, &t1598) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the constraints are infeasible: no point satisfies all of them"))));
    }
    if ({ let t1599 = v_status.clone(); Py::Bool({ let t1600 = Py::str("unbounded"); if py::eq(&t1599, &t1600) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the objective is unbounded on the feasible region: there is no finite optimum"))));
    }
    if (v_redundant.clone()).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("an equality constraint is a combination of the others: the shadow prices and ranges are not unique"))));
    }
    let t1601 = (Py::tuple(vec![py::len(&v_c.clone())?, py::len(&v_A.clone())?, py::len(&v_cs.clone())?])).items()?;
    if t1601.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_n = t1601[0].clone();
    v_m = t1601[1].clone();
    v_N = t1601[2].clone();
    v_Bi = f__binv({ let mut out = vec![]; for t1623 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t1623;
out.push({ let mut out = vec![]; for t1624 in py::iter(&(v_basis.clone()))? { let mut v_j: Py; 
v_j = t1624;
out.push(py::getitem(&py::getitem(&v_As.clone(), &v_i.clone())?, &v_j.clone())?); } Py::list(out) }); } Py::list(out) })?;
    if ({ let t1625 = v_Bi.clone(); Py::Bool({ let t1626 = Py::None; if py::is(&t1625, &t1626) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the optimal basis is singular"))));
    }
    v_xB = { let mut out = vec![]; for t1627 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_r: Py; 
v_r = t1627;
out.push(py::sum(&{ let mut out = vec![]; for t1628 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t1628;
out.push(py::mul(&py::getitem(&py::getitem(&v_Bi.clone(), &v_r.clone())?, &v_i.clone())?, &py::getitem(&v_bs.clone(), &v_i.clone())?)?); } Py::list(out) }, None)?); } Py::list(out) };
    for t1629 in py::iter(&(v_xB.clone()))? {
        v_v = t1629;
        if ({ let t1630 = v_v.clone(); Py::Bool({ let t1631 = Py::Int(0i64); if py::le(&t1630, &t1631)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the optimal solution is degenerate (a basic variable is 0): the shadow prices and ranges depend on the basis, so none are given"))));
        }
    }
    v_pos = py::mul(&Py::list(vec![py::neg(&Py::Int(1i64))?]), &v_N.clone())?;
    for t1632 in py::range_iter(&[(v_m.clone()).i()?]) {
        v_r = t1632;
        py::setitem(&v_pos.clone(), &py::getitem(&v_basis.clone(), &v_r.clone())?, v_r.clone())?;
    }
    v_cB = { let mut out = vec![]; for t1633 in py::iter(&(v_basis.clone()))? { let mut v_j: Py; 
v_j = t1633;
out.push(py::getitem(&v_cs.clone(), &v_j.clone())?); } Py::list(out) };
    v_yv = { let mut out = vec![]; for t1634 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t1634;
out.push(py::sum(&{ let mut out = vec![]; for t1635 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_r: Py; 
v_r = t1635;
out.push(py::mul(&py::getitem(&v_cB.clone(), &v_r.clone())?, &py::getitem(&py::getitem(&v_Bi.clone(), &v_r.clone())?, &v_i.clone())?)?); } Py::list(out) }, None)?); } Py::list(out) };
    v_nonbasic = { let mut out = vec![]; for t1636 in py::range_iter(&[(v_N.clone()).i()?]) { let mut v_j: Py; 
v_j = t1636;
if ({ let t1637 = py::getitem(&v_pos.clone(), &v_j.clone())?; Py::Bool({ let t1638 = Py::Int(0i64); if py::lt(&t1637, &t1638)? { true } else { false } }) }).truthy() { out.push(v_j.clone()); }} Py::list(out) };
    let t1639 = (Py::tuple(vec![Py::dict(vec![]), Py::dict(vec![])])).items()?;
    if t1639.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_red = t1639[0].clone();
    v_alpha = t1639[1].clone();
    for t1640 in py::iter(&(v_nonbasic.clone()))? {
        v_j = t1640;
        v_col = { let mut out = vec![]; for t1641 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t1641;
out.push(py::getitem(&py::getitem(&v_As.clone(), &v_i.clone())?, &v_j.clone())?); } Py::list(out) };
        py::setitem(&v_red.clone(), &v_j.clone(), py::sub(&py::getitem(&v_cs.clone(), &v_j.clone())?, &py::sum(&{ let mut out = vec![]; for t1642 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t1642;
out.push(py::mul(&py::getitem(&v_yv.clone(), &v_i.clone())?, &py::getitem(&v_col.clone(), &v_i.clone())?)?); } Py::list(out) }, None)?)?)?;
        py::setitem(&v_alpha.clone(), &v_j.clone(), { let mut out = vec![]; for t1643 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_r: Py; 
v_r = t1643;
out.push(py::sum(&{ let mut out = vec![]; for t1644 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t1644;
out.push(py::mul(&py::getitem(&py::getitem(&v_Bi.clone(), &v_r.clone())?, &v_i.clone())?, &py::getitem(&v_col.clone(), &v_i.clone())?)?); } Py::list(out) }, None)?); } Py::list(out) })?;
        if ({ let t1645 = py::getitem(&v_red.clone(), &v_j.clone())?; Py::Bool({ let t1646 = Py::Int(0i64); if py::le(&t1645, &t1646)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the optimum is not unique (alternative optimal solutions): the ranges depend on which optimal solution is reported, so none are given"))));
        }
    }
    v_s = (if ({ let t1647 = v_sense.clone(); Py::Bool({ let t1648 = Py::str("minimize"); if py::eq(&t1647, &t1648) { true } else { false } }) }).truthy() { py::fraction(&[Py::Int(1i64)])? } else { py::fraction(&[py::neg(&Py::Int(1i64))?])? });
    let t1649 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![])])).items()?;
    if t1649.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cost_lo = t1649[0].clone();
    v_cost_hi = t1649[1].clone();
    for t1650 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_j = t1650;
        v_cj = f__fr(py::getitem(&v_c.clone(), &v_j.clone())?)?;
        let t1651 = (Py::tuple(vec![Py::None, Py::None])).items()?;
        if t1651.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_lo_m = t1651[0].clone();
        v_hi_m = t1651[1].clone();
        if ({ let t1652 = py::getitem(&v_pos.clone(), &v_j.clone())?; Py::Bool({ let t1653 = Py::Int(0i64); if py::lt(&t1652, &t1653)? { true } else { false } }) }).truthy() {
            v_lo_m = py::neg(&py::getitem(&v_red.clone(), &v_j.clone())?)?;
        } else {
            v_r = py::getitem(&v_pos.clone(), &v_j.clone())?;
            for t1654 in py::iter(&(v_nonbasic.clone()))? {
                v_k = t1654;
                v_a = py::getitem(&py::getitem(&v_alpha.clone(), &v_k.clone())?, &v_r.clone())?;
                if ({ let t1655 = v_a.clone(); Py::Bool({ let t1656 = Py::Int(0i64); if py::gt(&t1655, &t1656)? { true } else { false } }) }).truthy() {
                    v_t = py::truediv(&py::getitem(&v_red.clone(), &v_k.clone())?, &v_a.clone())?;
                    if ({ let t1661 = { let t1657 = v_hi_m.clone(); Py::Bool({ let t1658 = Py::None; if py::is(&t1657, &t1658) { true } else { false } }) }; if t1661.truthy() { t1661 } else { { let t1659 = v_t.clone(); Py::Bool({ let t1660 = v_hi_m.clone(); if py::lt(&t1659, &t1660)? { true } else { false } }) } } }).truthy() {
                        v_hi_m = v_t.clone();
                    }
                } else {
                    if ({ let t1662 = v_a.clone(); Py::Bool({ let t1663 = Py::Int(0i64); if py::lt(&t1662, &t1663)? { true } else { false } }) }).truthy() {
                        v_t = py::truediv(&py::getitem(&v_red.clone(), &v_k.clone())?, &v_a.clone())?;
                        if ({ let t1668 = { let t1664 = v_lo_m.clone(); Py::Bool({ let t1665 = Py::None; if py::is(&t1664, &t1665) { true } else { false } }) }; if t1668.truthy() { t1668 } else { { let t1666 = v_t.clone(); Py::Bool({ let t1667 = v_lo_m.clone(); if py::gt(&t1666, &t1667)? { true } else { false } }) } } }).truthy() {
                            v_lo_m = v_t.clone();
                        }
                    }
                }
            }
        }
        if ({ let t1669 = v_s.clone(); Py::Bool({ let t1670 = Py::Int(1i64); if py::eq(&t1669, &t1670) { true } else { false } }) }).truthy() {
            let t1675 = (Py::tuple(vec![(if ({ let t1671 = v_lo_m.clone(); Py::Bool({ let t1672 = Py::None; if py::is(&t1671, &t1672) { true } else { false } }) }).truthy() { Py::None } else { py::add(&v_cj.clone(), &v_lo_m.clone())? }), (if ({ let t1673 = v_hi_m.clone(); Py::Bool({ let t1674 = Py::None; if py::is(&t1673, &t1674) { true } else { false } }) }).truthy() { Py::None } else { py::add(&v_cj.clone(), &v_hi_m.clone())? })])).items()?;
            if t1675.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_lo = t1675[0].clone();
            v_hi = t1675[1].clone();
        } else {
            let t1680 = (Py::tuple(vec![(if ({ let t1676 = v_hi_m.clone(); Py::Bool({ let t1677 = Py::None; if py::is(&t1676, &t1677) { true } else { false } }) }).truthy() { Py::None } else { py::sub(&v_cj.clone(), &v_hi_m.clone())? }), (if ({ let t1678 = v_lo_m.clone(); Py::Bool({ let t1679 = Py::None; if py::is(&t1678, &t1679) { true } else { false } }) }).truthy() { Py::None } else { py::sub(&v_cj.clone(), &v_lo_m.clone())? })])).items()?;
            if t1680.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_lo = t1680[0].clone();
            v_hi = t1680[1].clone();
        }
        let _ = py::method(&v_cost_lo.clone(), "append", &[(if ({ let t1681 = v_lo.clone(); Py::Bool({ let t1682 = Py::None; if py::is(&t1681, &t1682) { true } else { false } }) }).truthy() { Py::None } else { py::float(&v_lo.clone())? })])?;
        let _ = py::method(&v_cost_hi.clone(), "append", &[(if ({ let t1683 = v_hi.clone(); Py::Bool({ let t1684 = Py::None; if py::is(&t1683, &t1684) { true } else { false } }) }).truthy() { Py::None } else { py::float(&v_hi.clone())? })])?;
    }
    let t1685 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![])])).items()?;
    if t1685.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rhs_lo = t1685[0].clone();
    v_rhs_hi = t1685[1].clone();
    for t1686 in py::range_iter(&[(v_m.clone()).i()?]) {
        v_i = t1686;
        let t1687 = (Py::tuple(vec![Py::None, Py::None])).items()?;
        if t1687.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_lo_d = t1687[0].clone();
        v_hi_d = t1687[1].clone();
        for t1688 in py::range_iter(&[(v_m.clone()).i()?]) {
            v_r = t1688;
            v_g = py::getitem(&py::getitem(&v_Bi.clone(), &v_r.clone())?, &v_i.clone())?;
            if ({ let t1689 = v_g.clone(); Py::Bool({ let t1690 = Py::Int(0i64); if py::gt(&t1689, &t1690)? { true } else { false } }) }).truthy() {
                v_t = py::truediv(&py::neg(&py::getitem(&v_xB.clone(), &v_r.clone())?)?, &v_g.clone())?;
                if ({ let t1695 = { let t1691 = v_lo_d.clone(); Py::Bool({ let t1692 = Py::None; if py::is(&t1691, &t1692) { true } else { false } }) }; if t1695.truthy() { t1695 } else { { let t1693 = v_t.clone(); Py::Bool({ let t1694 = v_lo_d.clone(); if py::gt(&t1693, &t1694)? { true } else { false } }) } } }).truthy() {
                    v_lo_d = v_t.clone();
                }
            } else {
                if ({ let t1696 = v_g.clone(); Py::Bool({ let t1697 = Py::Int(0i64); if py::lt(&t1696, &t1697)? { true } else { false } }) }).truthy() {
                    v_t = py::truediv(&py::neg(&py::getitem(&v_xB.clone(), &v_r.clone())?)?, &v_g.clone())?;
                    if ({ let t1702 = { let t1698 = v_hi_d.clone(); Py::Bool({ let t1699 = Py::None; if py::is(&t1698, &t1699) { true } else { false } }) }; if t1702.truthy() { t1702 } else { { let t1700 = v_t.clone(); Py::Bool({ let t1701 = v_hi_d.clone(); if py::lt(&t1700, &t1701)? { true } else { false } }) } } }).truthy() {
                        v_hi_d = v_t.clone();
                    }
                }
            }
        }
        v_bi = f__fr(py::getitem(&v_b.clone(), &v_i.clone())?)?;
        let _ = py::method(&v_rhs_lo.clone(), "append", &[(if ({ let t1703 = v_lo_d.clone(); Py::Bool({ let t1704 = Py::None; if py::is(&t1703, &t1704) { true } else { false } }) }).truthy() { Py::None } else { py::float(&py::add(&v_bi.clone(), &v_lo_d.clone())?)? })])?;
        let _ = py::method(&v_rhs_hi.clone(), "append", &[(if ({ let t1705 = v_hi_d.clone(); Py::Bool({ let t1706 = Py::None; if py::is(&t1705, &t1706) { true } else { false } }) }).truthy() { Py::None } else { py::float(&py::add(&v_bi.clone(), &v_hi_d.clone())?)? })])?;
    }
    v_p = { let mut out = vec![]; for t1707 in py::iter(&(v_yv.clone()))? { let mut v_v: Py; 
v_v = t1707;
out.push(py::mul(&v_s.clone(), &v_v.clone())?); } Py::list(out) };
    v_x = py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &v_n.clone())?;
    for t1708 in py::range_iter(&[(v_m.clone()).i()?]) {
        v_r = t1708;
        if ({ let t1709 = py::getitem(&v_basis.clone(), &v_r.clone())?; Py::Bool({ let t1710 = v_n.clone(); if py::lt(&t1709, &t1710)? { true } else { false } }) }).truthy() {
            py::setitem(&v_x.clone(), &py::getitem(&v_basis.clone(), &v_r.clone())?, py::getitem(&v_xB.clone(), &v_r.clone())?)?;
        }
    }
    v_value = py::mul(&v_s.clone(), &py::sum(&{ let mut out = vec![]; for t1711 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_r: Py; 
v_r = t1711;
out.push(py::mul(&py::getitem(&v_cB.clone(), &v_r.clone())?, &py::getitem(&v_xB.clone(), &v_r.clone())?)?); } Py::list(out) }, None)?)?;
    v_signs = Py::list(vec![]);
    for t1712 in py::range_iter(&[(v_m.clone()).i()?]) {
        v_i = t1712;
        if ({ let t1713 = py::getitem(&v_rel.clone(), &v_i.clone())?; Py::Bool({ let t1714 = Py::str("="); if py::eq(&t1713, &t1714) { true } else { false } }) }).truthy() {
            let _ = py::method(&v_signs.clone(), "append", &[Py::str("free")])?;
        } else {
            if ({ let t1715 = { let t1717 = py::getitem(&v_rel.clone(), &v_i.clone())?; Py::Bool({ let t1718 = Py::str("<="); if py::eq(&t1717, &t1718) { true } else { false } }) }; Py::Bool({ let t1716 = { let t1719 = v_sense.clone(); Py::Bool({ let t1720 = Py::str("maximize"); if py::eq(&t1719, &t1720) { true } else { false } }) }; if py::eq(&t1715, &t1716) { true } else { false } }) }).truthy() {
                let _ = py::method(&v_signs.clone(), "append", &[Py::str(">= 0")])?;
            } else {
                let _ = py::method(&v_signs.clone(), "append", &[Py::str("<= 0")])?;
            }
        }
    }
    return Ok(Py::dict(vec![(Py::str("optimal_value"), py::float(&v_value.clone())?), (Py::str("x"), f__fl(v_x.clone())?), (Py::str("dual_solution"), f__fl(v_p.clone())?), (Py::str("dual_value"), py::float(&py::sum(&{ let mut out = vec![]; for t1722 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t1722;
out.push(py::mul(&f__fr(py::getitem(&v_b.clone(), &v_i.clone())?)?, &py::getitem(&v_p.clone(), &v_i.clone())?)?); } Py::list(out) }, None)?)?), (Py::str("cost_lower"), v_cost_lo.clone()), (Py::str("cost_upper"), v_cost_hi.clone()), (Py::str("rhs_lower"), v_rhs_lo.clone()), (Py::str("rhs_upper"), v_rhs_hi.clone()), (Py::str("dual_sense"), (if ({ let t1723 = v_sense.clone(); Py::Bool({ let t1724 = Py::str("maximize"); if py::eq(&t1723, &t1724) { true } else { false } }) }).truthy() { Py::str("minimize") } else { Py::str("maximize") })), (Py::str("dual_objective"), { let mut out = vec![]; for t1725 in py::iter(&(v_b.clone()))? { let mut v_v: Py; 
v_v = t1725;
out.push(py::float(&f__fr(v_v.clone())?)?); } Py::list(out) }), (Py::str("dual_matrix"), { let mut out = vec![]; for t1726 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t1726;
out.push({ let mut out = vec![]; for t1727 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t1727;
out.push(py::float(&f__fr(py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone())?)?)?); } Py::list(out) }); } Py::list(out) }), (Py::str("dual_relations"), py::method(&Py::str(", "), "join", &[py::mul(&Py::list(vec![(if ({ let t1728 = v_sense.clone(); Py::Bool({ let t1729 = Py::str("maximize"); if py::eq(&t1728, &t1729) { true } else { false } }) }).truthy() { Py::str(">=") } else { Py::str("<=") })]), &v_n.clone())?])?), (Py::str("dual_signs"), py::method(&Py::str(", "), "join", &[v_signs.clone()])?)]));
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
    if ({ let t2001 = py::len(&v_c.clone())?; Py::Bool({ let t2002 = Py::Int(2i64); if py::ne(&t2001, &t2002) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the graphical method takes exactly 2 variables"))));
    }
    let _ = f__check_lp(v_sense.clone(), v_c.clone(), v_A.clone(), v_b.clone(), v_rel.clone())?;
    let t2097 = (f__lp2_vertices(v_c.clone(), v_A.clone(), v_b.clone(), v_rel.clone())?).items()?;
    if t2097.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pts = t2097[0].clone();
    v_rays = t2097[1].clone();
    if (py::not(&v_pts.clone())).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the constraints are infeasible: no point satisfies all of them"))));
    }
    v_s = (if ({ let t2098 = v_sense.clone(); Py::Bool({ let t2099 = Py::str("maximize"); if py::eq(&t2098, &t2099) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { py::neg(&Py::Int(1i64))? });
    let t2100 = (Py::tuple(vec![f__fr(py::getitem(&v_c.clone(), &Py::Int(0i64))?)?, f__fr(py::getitem(&v_c.clone(), &Py::Int(1i64))?)?])).items()?;
    if t2100.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c0 = t2100[0].clone();
    v_c1 = t2100[1].clone();
    if (Py::Bool(({ let mut out = vec![]; for t2101 in py::iter(&(v_rays.clone()))? { let mut v_d: Py; 
v_d = t2101;
out.push({ let t2102 = py::mul(&v_s.clone(), &py::add(&py::mul(&v_c0.clone(), &py::getitem(&v_d.clone(), &Py::Int(0i64))?)?, &py::mul(&v_c1.clone(), &py::getitem(&v_d.clone(), &Py::Int(1i64))?)?)?)?; Py::Bool({ let t2103 = Py::Int(0i64); if py::gt(&t2102, &t2103)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the objective is unbounded on the feasible region: there is no finite optimum"))));
    }
    v_vals = { let mut out = vec![]; for t2104 in py::iter(&(v_pts.clone()))? { let mut v_x: Py; let mut v_y: Py; 
let t2105 = (t2104).items()?;
if t2105.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2105[0].clone();
v_y = t2105[1].clone();
out.push(Py::tuple(vec![py::add(&py::mul(&v_c0.clone(), &v_x.clone())?, &py::mul(&v_c1.clone(), &v_y.clone())?)?, Py::tuple(vec![v_x.clone(), v_y.clone()])])); } Py::list(out) };
    v_best = (if ({ let t2106 = v_s.clone(); Py::Bool({ let t2107 = Py::Int(1i64); if py::eq(&t2106, &t2107) { true } else { false } }) }).truthy() { py::minmax(&[{ let mut out = vec![]; for t2108 in py::iter(&(v_vals.clone()))? { let mut v__: Py; let mut v_v: Py; 
let t2109 = (t2108).items()?;
if t2109.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t2109[0].clone();
v__ = t2109[1].clone();
out.push(v_v.clone()); } Py::list(out) }], true)? } else { py::minmax(&[{ let mut out = vec![]; for t2110 in py::iter(&(v_vals.clone()))? { let mut v__: Py; let mut v_v: Py; 
let t2111 = (t2110).items()?;
if t2111.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t2111[0].clone();
v__ = t2111[1].clone();
out.push(v_v.clone()); } Py::list(out) }], false)? });
    v_opt = { let mut out = vec![]; for t2112 in py::iter(&(v_vals.clone()))? { let mut v_p: Py; let mut v_v: Py; 
let t2113 = (t2112).items()?;
if t2113.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t2113[0].clone();
v_p = t2113[1].clone();
if ({ let t2114 = v_v.clone(); Py::Bool({ let t2115 = v_best.clone(); if py::eq(&t2114, &t2115) { true } else { false } }) }).truthy() { out.push(v_p.clone()); }} Py::list(out) };
    v_flat_ray = Py::Bool(({ let mut out = vec![]; for t2116 in py::iter(&(v_rays.clone()))? { let mut v_d: Py; 
v_d = t2116;
out.push({ let t2117 = py::add(&py::mul(&v_c0.clone(), &py::getitem(&v_d.clone(), &Py::Int(0i64))?)?, &py::mul(&v_c1.clone(), &py::getitem(&v_d.clone(), &Py::Int(1i64))?)?)?; Py::Bool({ let t2118 = Py::Int(0i64); if py::eq(&t2117, &t2118) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()));
    v_out = Py::dict(vec![(Py::str("optimal_value"), py::float(&v_best.clone())?), (Py::str("optimal_corners"), { let mut out = vec![]; for t2119 in py::iter(&(v_opt.clone()))? { let mut v_x: Py; let mut v_y: Py; 
let t2120 = (t2119).items()?;
if t2120.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2120[0].clone();
v_y = t2120[1].clone();
out.push(Py::list(vec![py::float(&v_x.clone())?, py::float(&v_y.clone())?])); } Py::list(out) }), (Py::str("corner_points"), { let mut out = vec![]; for t2121 in py::iter(&(v_pts.clone()))? { let mut v_x: Py; let mut v_y: Py; 
let t2122 = (t2121).items()?;
if t2122.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2122[0].clone();
v_y = t2122[1].clone();
out.push(Py::list(vec![py::float(&v_x.clone())?, py::float(&v_y.clone())?])); } Py::list(out) })]);
    if ({ let t2125 = { let t2123 = py::len(&v_opt.clone())?; Py::Bool({ let t2124 = Py::Int(1i64); if py::eq(&t2123, &t2124) { true } else { false } }) }; if !t2125.truthy() { t2125 } else { py::not(&v_flat_ray.clone()) } }).truthy() {
        let t2126 = (Py::tuple(vec![py::float(&py::getitem(&py::getitem(&v_opt.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?)?, py::float(&py::getitem(&py::getitem(&v_opt.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?)?, Py::str("yes")])).items()?;
        if t2126.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_out.clone(), &Py::str("x"), t2126[0].clone())?;
        py::setitem(&v_out.clone(), &Py::str("y"), t2126[1].clone())?;
        py::setitem(&v_out.clone(), &Py::str("solution_unique"), t2126[2].clone())?;
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
    if ({ let t2145 = { let t2141 = v_lam.clone(); Py::Bool({ let t2142 = Py::Int(0i64); if py::le(&t2141, &t2142)? { true } else { false } }) }; if t2145.truthy() { t2145 } else { { let t2143 = v_mu.clone(); Py::Bool({ let t2144 = Py::Int(0i64); if py::le(&t2143, &t2144)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the failure and repair rates must be > 0"))));
    }
    if (py::not(&{ let t2146 = Py::Int(1i64); Py::Bool({ let t2147 = v_c.clone(); if py::le(&t2146, &t2147)? { { let t2148 = v_N.clone(); if py::le(&t2147, &t2148)? { { let t2149 = Py::Int(1000i64); if py::le(&t2148, &t2149)? { true } else { false } } } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("need 1 <= repairers <= machines <= 1000"))));
    }
    v_r = py::truediv(&v_lam.clone(), &v_mu.clone())?;
    v_logs = Py::list(vec![]);
    for t2150 in py::range_iter(&[(py::add(&v_N.clone(), &Py::Int(1i64))?).i()?]) {
        v_n = t2150;
        v_base = py::add(&py::sub(&py::math1("lgamma", &py::add(&v_N.clone(), &Py::Int(1i64))?)?, &py::math1("lgamma", &py::add(&py::sub(&v_N.clone(), &v_n.clone())?, &Py::Int(1i64))?)?)?, &py::mul(&v_n.clone(), &py::math1("log", &v_r.clone())?)?)?;
        let _ = py::method(&v_logs.clone(), "append", &[py::sub(&v_base.clone(), &(if ({ let t2151 = v_n.clone(); Py::Bool({ let t2152 = v_c.clone(); if py::le(&t2151, &t2152)? { true } else { false } }) }).truthy() { py::math1("lgamma", &py::add(&v_n.clone(), &Py::Int(1i64))?)? } else { py::add(&py::math1("lgamma", &py::add(&v_c.clone(), &Py::Int(1i64))?)?, &py::mul(&py::sub(&v_n.clone(), &v_c.clone())?, &py::math1("log", &v_c.clone())?)?)? }))?])?;
    }
    v_mx = py::minmax(&[v_logs.clone()], true)?;
    v_w = { let mut out = vec![]; for t2153 in py::iter(&(v_logs.clone()))? { let mut v_v: Py; 
v_v = t2153;
out.push(py::math1("exp", &py::sub(&v_v.clone(), &v_mx.clone())?)?); } Py::list(out) };
    v_tot = py::fsum(&v_w.clone())?;
    v_pi = { let mut out = vec![]; for t2154 in py::iter(&(v_w.clone()))? { let mut v_v: Py; 
v_v = t2154;
out.push(py::truediv(&v_v.clone(), &v_tot.clone())?); } Py::list(out) };
    v_L = py::fsum(&{ let mut out = vec![]; for t2155 in py::iter(&(py::enumerate(&v_pi.clone(), 0)?))? { let mut v_n: Py; let mut v_p: Py; 
let t2156 = (t2155).items()?;
if t2156.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_n = t2156[0].clone();
v_p = t2156[1].clone();
out.push(py::mul(&v_n.clone(), &v_p.clone())?); } Py::list(out) })?;
    v_Lq = py::fsum(&{ let mut out = vec![]; for t2157 in py::iter(&(py::enumerate(&v_pi.clone(), 0)?))? { let mut v_n: Py; let mut v_p: Py; 
let t2158 = (t2157).items()?;
if t2158.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_n = t2158[0].clone();
v_p = t2158[1].clone();
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
    if (Py::Bool(({ let mut out = vec![]; for t2166 in py::iter(&(v_E.clone()))? { let mut v__: Py; let mut v_w: Py; 
let t2167 = (t2166).items()?;
if t2167.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v__ = t2167[0].clone();
v__ = t2167[1].clone();
v_w = t2167[2].clone();
out.push({ let t2168 = v_w.clone(); Py::Bool({ let t2169 = Py::Int(0i64); if py::lt(&t2168, &t2169)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("capacities must be >= 0"))));
    }
    let t2170 = (Py::tuple(vec![py::method(&Py::str(&py::to_str(&v_source.clone())), "strip", &[])?, py::method(&Py::str(&py::to_str(&v_target.clone())), "strip", &[])?])).items()?;
    if t2170.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t2170[0].clone();
    v_t = t2170[1].clone();
    v_nodes = f__nodes(v_E.clone())?;
    if ({ let t2182 = { let t2175 = v_s.clone(); Py::Bool({ let t2176 = v_nodes.clone(); if !py::contains(&t2176, &t2175)? { true } else { false } }) }; if t2182.truthy() { t2182 } else { { let t2181 = { let t2177 = v_t.clone(); Py::Bool({ let t2178 = v_nodes.clone(); if !py::contains(&t2178, &t2177)? { true } else { false } }) }; if t2181.truthy() { t2181 } else { { let t2179 = v_s.clone(); Py::Bool({ let t2180 = v_t.clone(); if py::eq(&t2179, &t2180) { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the source and the sink must be two different nodes of the network"))));
    }
    v_cap = Py::dict(vec![]);
    for t2183 in py::iter(&(v_E.clone()))? {
        let t2184 = (t2183).items()?;
        if t2184.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_u = t2184[0].clone();
        v_v = t2184[1].clone();
        v_w = t2184[2].clone();
        py::setitem(&v_cap.clone(), &Py::tuple(vec![v_u.clone(), v_v.clone()]), py::add(&py::method(&v_cap.clone(), "get", &[Py::tuple(vec![v_u.clone(), v_v.clone()]), py::fraction(&[Py::Int(0i64)])?])?, &v_w.clone())?)?;
        let _ = py::method(&v_cap.clone(), "setdefault", &[Py::tuple(vec![v_v.clone(), v_u.clone()]), py::fraction(&[Py::Int(0i64)])?])?;
    }
    v_flow = { let mut out = vec![]; for t2185 in py::iter(&(v_cap.clone()))? { let mut v_k: Py; 
v_k = t2185;
out.push((v_k.clone(), py::fraction(&[Py::Int(0i64)])?)); } Py::dict(out) };
    v_nbr = Py::dict(vec![]);
    for t2186 in py::iter(&(v_cap.clone()))? {
        let t2187 = (t2186).items()?;
        if t2187.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_u = t2187[0].clone();
        v_v = t2187[1].clone();
        let _ = py::method(&py::method(&v_nbr.clone(), "setdefault", &[v_u.clone(), Py::list(vec![])])?, "append", &[v_v.clone()])?;
    }
    v_value = py::fraction(&[Py::Int(0i64)])?;
    while (Py::Bool(true)).truthy() {
        let t2188 = (Py::tuple(vec![Py::dict(vec![(v_s.clone(), Py::None)]), Py::list(vec![v_s.clone()])])).items()?;
        if t2188.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_prev = t2188[0].clone();
        v_q = t2188[1].clone();
        for t2189 in py::iter(&(v_q.clone()))? {
            v_x = t2189;
            for t2190 in py::iter(&(py::method(&v_nbr.clone(), "get", &[v_x.clone(), Py::list(vec![])])?))? {
                v_y = t2190;
                if ({ let t2195 = { let t2191 = v_y.clone(); Py::Bool({ let t2192 = v_prev.clone(); if !py::contains(&t2192, &t2191)? { true } else { false } }) }; if !t2195.truthy() { t2195 } else { { let t2193 = py::sub(&py::getitem(&v_cap.clone(), &Py::tuple(vec![v_x.clone(), v_y.clone()]))?, &py::getitem(&v_flow.clone(), &Py::tuple(vec![v_x.clone(), v_y.clone()]))?)?; Py::Bool({ let t2194 = Py::Int(0i64); if py::gt(&t2193, &t2194)? { true } else { false } }) } } }).truthy() {
                    py::setitem(&v_prev.clone(), &v_y.clone(), v_x.clone())?;
                    let _ = py::method(&v_q.clone(), "append", &[v_y.clone()])?;
                }
            }
        }
        if ({ let t2196 = v_t.clone(); Py::Bool({ let t2197 = v_prev.clone(); if !py::contains(&t2197, &t2196)? { true } else { false } }) }).truthy() {
            break;
        }
        let t2198 = (Py::tuple(vec![Py::list(vec![]), v_t.clone()])).items()?;
        if t2198.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_path = t2198[0].clone();
        v_y = t2198[1].clone();
        while ({ let t2199 = py::getitem(&v_prev.clone(), &v_y.clone())?; Py::Bool({ let t2200 = Py::None; if !py::is(&t2199, &t2200) { true } else { false } }) }).truthy() {
            let _ = py::method(&v_path.clone(), "append", &[Py::tuple(vec![py::getitem(&v_prev.clone(), &v_y.clone())?, v_y.clone()])])?;
            v_y = py::getitem(&v_prev.clone(), &v_y.clone())?;
        }
        v_aug = py::minmax(&[{ let mut out = vec![]; for t2201 in py::iter(&(v_path.clone()))? { let mut v_e: Py; 
v_e = t2201;
out.push(py::sub(&py::getitem(&v_cap.clone(), &v_e.clone())?, &py::getitem(&v_flow.clone(), &v_e.clone())?)?); } Py::list(out) }], false)?;
        for t2202 in py::iter(&(v_path.clone()))? {
            let t2207 = (t2202).items()?;
            if t2207.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_x = t2207[0].clone();
            v_y = t2207[1].clone();
            let t2203 = v_flow.clone(); let t2204 = Py::tuple(vec![v_x.clone(), v_y.clone()]);
            py::setitem(&t2203, &t2204, py::iadd(&py::getitem(&t2203, &t2204)?, &v_aug.clone())?)?;
            let t2205 = v_flow.clone(); let t2206 = Py::tuple(vec![v_y.clone(), v_x.clone()]);
            py::setitem(&t2205, &t2206, py::sub(&py::getitem(&t2205, &t2206)?, &v_aug.clone())?)?;
        }
        v_value = py::iadd(&v_value, &v_aug.clone())?;
    }
    v_side = { let mut out = vec![]; for t2208 in py::iter(&(v_nodes.clone()))? { let mut v_v: Py; 
v_v = t2208;
if ({ let t2209 = v_v.clone(); Py::Bool({ let t2210 = v_prev.clone(); if py::contains(&t2210, &t2209)? { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) };
    let t2212 = (Py::tuple(vec![{ let mut out = vec![]; for t2211 in py::iter(&(v_cap.clone()))? { let mut v_k: Py; 
v_k = t2211;
out.push((v_k.clone(), py::minmax(&[py::getitem(&v_flow.clone(), &v_k.clone())?, py::fraction(&[Py::Int(0i64)])?], true)?)); } Py::dict(out) }, Py::list(vec![])])).items()?;
    if t2212.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_left = t2212[0].clone();
    v_per = t2212[1].clone();
    for t2213 in py::iter(&(v_E.clone()))? {
        let t2216 = (t2213).items()?;
        if t2216.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_u = t2216[0].clone();
        v_v = t2216[1].clone();
        v_w = t2216[2].clone();
        v_f = py::minmax(&[v_w.clone(), py::getitem(&v_left.clone(), &Py::tuple(vec![v_u.clone(), v_v.clone()]))?], false)?;
        let t2214 = v_left.clone(); let t2215 = Py::tuple(vec![v_u.clone(), v_v.clone()]);
        py::setitem(&t2214, &t2215, py::sub(&py::getitem(&t2214, &t2215)?, &v_f.clone())?)?;
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
    if ({ let t2261 = py::len(&v_nodes.clone())?; Py::Bool({ let t2262 = Py::Int(16i64); if py::gt(&t2261, &t2262)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at most 16 nodes"))));
    }
    v_ix = { let mut out = vec![]; for t2263 in py::iter(&(py::enumerate(&v_nodes.clone(), 0)?))? { let mut v_k: Py; let mut v_v: Py; 
let t2264 = (t2263).items()?;
if t2264.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2264[0].clone();
v_v = t2264[1].clone();
out.push((v_v.clone(), v_k.clone())); } Py::dict(out) };
    v_W = Py::dict(vec![]);
    for t2265 in py::iter(&(v_E.clone()))? {
        let t2269 = (t2265).items()?;
        if t2269.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_u = t2269[0].clone();
        v_v = t2269[1].clone();
        v_w = t2269[2].clone();
        if ({ let t2266 = v_u.clone(); Py::Bool({ let t2267 = v_v.clone(); if py::ne(&t2266, &t2267) { true } else { false } }) }).truthy() {
            let t2268 = (py::sorted(&Py::tuple(vec![py::getitem(&v_ix.clone(), &v_u.clone())?, py::getitem(&v_ix.clone(), &v_v.clone())?]), (Py::Bool(false)).truthy())?).items()?;
            if t2268.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t2268[0].clone();
            v_b = t2268[1].clone();
            py::setitem(&v_W.clone(), &Py::tuple(vec![v_a.clone(), v_b.clone()]), py::minmax(&[py::method(&v_W.clone(), "get", &[Py::tuple(vec![v_a.clone(), v_b.clone()]), v_w.clone()])?, v_w.clone()], true)?)?;
        }
    }
    v_n = py::len(&v_nodes.clone())?;
    v_memo = Py::dict(vec![]);
    let rec2270 = py::rec_cell();
    v_f = { let v_E = v_E.clone(); let v_W = v_W.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_edges = v_edges.clone(); let v_ix = v_ix.clone(); let v_memo = v_memo.clone(); let v_n = v_n.clone(); let v_nodes = v_nodes.clone(); let v_u = v_u.clone(); let v_v = v_v.clone(); let v_w = v_w.clone(); let rec2270 = rec2270.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_mask: Py = a.get(0).cloned().unwrap_or(Py::None);
            let v_f: Py = py::rec_get(&rec2270)?;
            let mut v_best: Py = Py::None;
            let mut v_i: Py = Py::None;
            let mut v_j: Py = Py::None;
            let mut v_pairs: Py = Py::None;
            let mut v_val: Py = Py::None;
            if ({ let t2271 = v_mask.clone(); Py::Bool({ let t2272 = py::sub(&py::lshift(&Py::Int(1i64), &v_n.clone())?, &Py::Int(1i64))?; if py::eq(&t2271, &t2272) { true } else { false } }) }).truthy() {
                return Ok(Py::tuple(vec![py::fraction(&[Py::Int(0i64)])?, Py::tuple(vec![])]));
            }
            if ({ let t2273 = v_mask.clone(); Py::Bool({ let t2274 = v_memo.clone(); if py::contains(&t2274, &t2273)? { true } else { false } }) }).truthy() {
                return Ok(py::getitem(&v_memo.clone(), &v_mask.clone())?);
            }
            v_i = (match ('g2275: { let mut out = vec![]; for t2276 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t2276;
if (py::not(&py::bitand(&py::rshift(&v_mask.clone(), &v_k.clone())?, &Py::Int(1i64))?)).truthy() { out.push(v_k.clone()); break 'g2275 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) });
            v_best = py::call(&v_f.clone(), &[py::bitor(&v_mask.clone(), &py::lshift(&Py::Int(1i64), &v_i.clone())?)?])?;
            for t2277 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
                v_j = t2277;
                if ({ let t2283 = py::not(&py::bitand(&py::rshift(&v_mask.clone(), &v_j.clone())?, &Py::Int(1i64))?); if !t2283.truthy() { t2283 } else { { let t2282 = { let t2278 = Py::tuple(vec![v_i.clone(), v_j.clone()]); Py::Bool({ let t2279 = v_W.clone(); if py::contains(&t2279, &t2278)? { true } else { false } }) }; if !t2282.truthy() { t2282 } else { { let t2280 = py::getitem(&v_W.clone(), &Py::tuple(vec![v_i.clone(), v_j.clone()]))?; Py::Bool({ let t2281 = Py::Int(0i64); if py::gt(&t2280, &t2281)? { true } else { false } }) } } } } }).truthy() {
                    let t2284 = (py::call(&v_f.clone(), &[py::bitor(&py::bitor(&v_mask.clone(), &py::lshift(&Py::Int(1i64), &v_i.clone())?)?, &py::lshift(&Py::Int(1i64), &v_j.clone())?)?])?).items()?;
                    if t2284.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    v_val = t2284[0].clone();
                    v_pairs = t2284[1].clone();
                    if ({ let t2285 = py::add(&v_val.clone(), &py::getitem(&v_W.clone(), &Py::tuple(vec![v_i.clone(), v_j.clone()]))?)?; Py::Bool({ let t2286 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; if py::gt(&t2285, &t2286)? { true } else { false } }) }).truthy() {
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
    py::rec_set(&rec2270, &v_f);
    let t2287 = (py::call(&v_f.clone(), &[Py::Int(0i64)])?).items()?;
    if t2287.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_val = t2287[0].clone();
    v_pairs = t2287[1].clone();
    return Ok(Py::dict(vec![(Py::str("max_weight"), py::float(&v_val.clone())?), (Py::str("matching_example"), { let mut out = vec![]; for t2288 in py::iter(&(v_pairs.clone()))? { let mut v_a: Py; let mut v_b: Py; 
let t2289 = (t2288).items()?;
if t2289.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2289[0].clone();
v_b = t2289[1].clone();
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
    let t2342 = (f__mdp_in(v_R.clone(), v_T.clone(), v_gamma.clone())?).items()?;
    if t2342.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_Rf = t2342[0].clone();
    v_P = t2342[1].clone();
    v_g = t2342[2].clone();
    v_S = t2342[3].clone();
    v_A = t2342[4].clone();
    v_pol = py::mul(&Py::list(vec![Py::Int(0i64)]), &v_S.clone())?;
    for t2343 in py::range_iter(&[(Py::Int(1000i64)).i()?]) {
        v__ = t2343;
        v_V = f__solve_exact({ let mut out = vec![]; for t2363 in py::range_iter(&[(v_S.clone()).i()?]) { let mut v_s: Py; 
v_s = t2363;
out.push({ let mut out = vec![]; for t2364 in py::range_iter(&[(v_S.clone()).i()?]) { let mut v_t: Py; 
v_t = t2364;
out.push(py::sub(&(if ({ let t2365 = v_s.clone(); Py::Bool({ let t2366 = v_t.clone(); if py::eq(&t2365, &t2366) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { Py::Int(0i64) }), &py::mul(&v_g.clone(), &py::getitem(&py::getitem(&v_P.clone(), &py::add(&py::mul(&v_s.clone(), &v_A.clone())?, &py::getitem(&v_pol.clone(), &v_s.clone())?)?)?, &v_t.clone())?)?)?); } Py::list(out) }); } Py::list(out) }, { let mut out = vec![]; for t2367 in py::range_iter(&[(v_S.clone()).i()?]) { let mut v_s: Py; 
v_s = t2367;
out.push(py::getitem(&py::getitem(&v_Rf.clone(), &v_s.clone())?, &py::getitem(&v_pol.clone(), &v_s.clone())?)?); } Py::list(out) })?;
        v_Qv = { let mut out = vec![]; for t2368 in py::range_iter(&[(v_S.clone()).i()?]) { let mut v_s: Py; 
v_s = t2368;
out.push({ let mut out = vec![]; for t2369 in py::range_iter(&[(v_A.clone()).i()?]) { let mut v_a: Py; 
v_a = t2369;
out.push(py::add(&py::getitem(&py::getitem(&v_Rf.clone(), &v_s.clone())?, &v_a.clone())?, &py::mul(&v_g.clone(), &py::sum(&{ let mut out = vec![]; for t2370 in py::range_iter(&[(v_S.clone()).i()?]) { let mut v_t: Py; 
v_t = t2370;
out.push(py::mul(&py::getitem(&py::getitem(&v_P.clone(), &py::add(&py::mul(&v_s.clone(), &v_A.clone())?, &v_a.clone())?)?, &v_t.clone())?, &py::getitem(&v_V.clone(), &v_t.clone())?)?); } Py::list(out) }, None)?)?)?); } Py::list(out) }); } Py::list(out) };
        v_new = { let mut out = vec![]; for t2371 in py::range_iter(&[(v_S.clone()).i()?]) { let mut v_s: Py; 
v_s = t2371;
out.push((if ({ let t2372 = py::getitem(&py::getitem(&v_Qv.clone(), &v_s.clone())?, &py::getitem(&v_pol.clone(), &v_s.clone())?)?; Py::Bool({ let t2373 = py::minmax(&[py::getitem(&v_Qv.clone(), &v_s.clone())?], true)?; if py::eq(&t2372, &t2373) { true } else { false } }) }).truthy() { py::getitem(&v_pol.clone(), &v_s.clone())? } else { py::method(&py::getitem(&v_Qv.clone(), &v_s.clone())?, "index", &[py::minmax(&[py::getitem(&v_Qv.clone(), &v_s.clone())?], true)?])? })); } Py::list(out) };
        if ({ let t2374 = v_new.clone(); Py::Bool({ let t2375 = v_pol.clone(); if py::eq(&t2374, &t2375) { true } else { false } }) }).truthy() {
            break;
        }
        v_pol = v_new.clone();
    }
    v_best = { let mut out = vec![]; for t2376 in py::range_iter(&[(v_S.clone()).i()?]) { let mut v_s: Py; 
v_s = t2376;
out.push(py::add(&py::method(&py::getitem(&v_Qv.clone(), &v_s.clone())?, "index", &[py::minmax(&[py::getitem(&v_Qv.clone(), &v_s.clone())?], true)?])?, &Py::Int(1i64))?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("value_state_1"), py::float(&py::getitem(&v_V.clone(), &Py::Int(0i64))?)?), (Py::str("values"), f__fl(v_V.clone())?), (Py::str("policy"), { let mut out = vec![]; for t2378 in py::iter(&(v_best.clone()))? { let mut v_a: Py; 
v_a = t2378;
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
    if ({ let t2399 = { let t2392 = v_lam.clone(); Py::Bool({ let t2393 = Py::Int(0i64); if py::le(&t2392, &t2393)? { true } else { false } }) }; if t2399.truthy() { t2399 } else { { let t2398 = { let t2394 = v_ES.clone(); Py::Bool({ let t2395 = Py::Int(0i64); if py::le(&t2394, &t2395)? { true } else { false } }) }; if t2398.truthy() { t2398 } else { { let t2396 = v_sd.clone(); Py::Bool({ let t2397 = Py::Int(0i64); if py::lt(&t2396, &t2397)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the arrival rate and mean service time must be > 0 and the standard deviation >= 0"))));
    }
    v_rho = py::mul(&v_lam.clone(), &v_ES.clone())?;
    if ({ let t2400 = v_rho.clone(); Py::Bool({ let t2401 = Py::Int(1i64); if py::ge(&t2400, &t2401)? { true } else { false } }) }).truthy() {
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
    let t2441 = (f__mcf_lp(v_capacity.clone(), v_cost.clone(), v_supply.clone())?).items()?;
    if t2441.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_arcs = t2441[0].clone();
    v_lp = t2441[1].clone();
    v_r = { let mut v: Vec<Py> = vec![]; v.extend((v_lp.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "sense") { Some(x) => x, None => { return Err(PyErr::Type("_lp() missing argument sense".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "c") { Some(x) => x, None => { return Err(PyErr::Type("_lp() missing argument c".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "A") { Some(x) => x, None => { return Err(PyErr::Type("_lp() missing argument A".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "b") { Some(x) => x, None => { return Err(PyErr::Type("_lp() missing argument b".into())) } } }; let a4 = if 4 < v.len() { v[4].clone() } else { match py::kw_take(&mut d, "rel") { Some(x) => x, None => { return Err(PyErr::Type("_lp() missing argument rel".into())) } } }; let a5 = if 5 < v.len() { v[5].clone() } else { match py::kw_take(&mut d, "certify") { Some(x) => x, None => Py::Bool(true) } }; if v.len() > 6 { return Err(PyErr::Type("_lp() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("_lp() got an unexpected keyword argument".into())) } f__lp(a0, a1, a2, a3, a4, a5)? };
    v_n = py::len(&v_supply.clone())?;
    v_fl = { let mut out = vec![]; for t2596 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v__: Py; 
v__ = t2596;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?); } Py::list(out) };
    for t2597 in py::iter(&(py::zip(&[v_arcs.clone(), py::getitem(&v_r.clone(), &Py::str("x"))?])?))? {
        let t2598 = (t2597).items()?;
        if t2598.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        let t2599 = (t2598[0].clone()).items()?;
        if t2599.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t2599[0].clone();
        v_j = t2599[1].clone();
        v_x = t2598[1].clone();
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
    let t2782 = (f__bimatrix(v_A.clone(), v_B.clone(), Py::Int(8i64))?).items()?;
    if t2782.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_MA = t2782[0].clone();
    v_MB = t2782[1].clone();
    if ({ let t2787 = { let t2783 = py::len(&v_MA.clone())?; Py::Bool({ let t2784 = Py::Int(2i64); if py::ne(&t2783, &t2784) { true } else { false } }) }; if t2787.truthy() { t2787 } else { { let t2785 = py::len(&py::getitem(&v_MA.clone(), &Py::Int(0i64))?)?; Py::Bool({ let t2786 = Py::Int(2i64); if py::ne(&t2785, &t2786) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give 2 x 2 payoff matrices"))));
    }
    if (f__pure_ne(v_MA.clone(), v_MB.clone())?).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the game has a pure-strategy equilibrium: its mixed equilibrium is not the only one (ask for the pure equilibria)"))));
    }
    let t2798 = (v_MA.clone()).items()?;
    if t2798.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    let t2799 = (t2798[0].clone()).items()?;
    if t2799.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a11 = t2799[0].clone();
    v_a12 = t2799[1].clone();
    let t2800 = (t2798[1].clone()).items()?;
    if t2800.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a21 = t2800[0].clone();
    v_a22 = t2800[1].clone();
    let t2801 = (v_MB.clone()).items()?;
    if t2801.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    let t2802 = (t2801[0].clone()).items()?;
    if t2802.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_b11 = t2802[0].clone();
    v_b12 = t2802[1].clone();
    let t2803 = (t2801[1].clone()).items()?;
    if t2803.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_b21 = t2803[0].clone();
    v_b22 = t2803[1].clone();
    let t2804 = (Py::tuple(vec![py::add(&py::sub(&py::sub(&v_a11.clone(), &v_a12.clone())?, &v_a21.clone())?, &v_a22.clone())?, py::add(&py::sub(&py::sub(&v_b11.clone(), &v_b12.clone())?, &v_b21.clone())?, &v_b22.clone())?])).items()?;
    if t2804.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_DA = t2804[0].clone();
    v_DB = t2804[1].clone();
    if ({ let t2809 = { let t2805 = v_DA.clone(); Py::Bool({ let t2806 = Py::Int(0i64); if py::eq(&t2805, &t2806) { true } else { false } }) }; if t2809.truthy() { t2809 } else { { let t2807 = v_DB.clone(); Py::Bool({ let t2808 = Py::Int(0i64); if py::eq(&t2807, &t2808) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("degenerate game: no unique mixed equilibrium"))));
    }
    let t2810 = (Py::tuple(vec![py::truediv(&py::sub(&v_a22.clone(), &v_a12.clone())?, &v_DA.clone())?, py::truediv(&py::sub(&v_b22.clone(), &v_b21.clone())?, &v_DB.clone())?])).items()?;
    if t2810.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_q = t2810[0].clone();
    v_p = t2810[1].clone();
    if (py::not(&{ let t2817 = { let t2811 = Py::Int(0i64); Py::Bool({ let t2812 = v_p.clone(); if py::lt(&t2811, &t2812)? { { let t2813 = Py::Int(1i64); if py::lt(&t2812, &t2813)? { true } else { false } } } else { false } }) }; if !t2817.truthy() { t2817 } else { { let t2814 = Py::Int(0i64); Py::Bool({ let t2815 = v_q.clone(); if py::lt(&t2814, &t2815)? { { let t2816 = Py::Int(1i64); if py::lt(&t2815, &t2816)? { true } else { false } } } else { false } }) } } })).truthy() {
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
    if ({ let t2844 = { let t2840 = v_lam.clone(); Py::Bool({ let t2841 = Py::Int(0i64); if py::le(&t2840, &t2841)? { true } else { false } }) }; if t2844.truthy() { t2844 } else { { let t2842 = v_mu.clone(); Py::Bool({ let t2843 = Py::Int(0i64); if py::le(&t2842, &t2843)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the arrival and service rates must be > 0"))));
    }
    if ({ let t2845 = v_lam.clone(); Py::Bool({ let t2846 = v_mu.clone(); if py::ge(&t2845, &t2846)? { true } else { false } }) }).truthy() {
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
    if ({ let t2893 = { let t2889 = v_lam.clone(); Py::Bool({ let t2890 = Py::Int(0i64); if py::le(&t2889, &t2890)? { true } else { false } }) }; if t2893.truthy() { t2893 } else { { let t2891 = v_mu.clone(); Py::Bool({ let t2892 = Py::Int(0i64); if py::le(&t2891, &t2892)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the arrival and service rates must be > 0"))));
    }
    if ({ let t2898 = { let t2894 = v_c.clone(); Py::Bool({ let t2895 = Py::Int(1i64); if py::lt(&t2894, &t2895)? { true } else { false } }) }; if t2898.truthy() { t2898 } else { { let t2896 = v_c.clone(); Py::Bool({ let t2897 = Py::Int(500i64); if py::gt(&t2896, &t2897)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the number of servers must be between 1 and 500"))));
    }
    let t2899 = (Py::tuple(vec![py::truediv(&v_lam.clone(), &v_mu.clone())?, py::truediv(&v_lam.clone(), &py::mul(&v_c.clone(), &v_mu.clone())?)?])).items()?;
    if t2899.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2899[0].clone();
    v_rho = t2899[1].clone();
    if ({ let t2900 = v_rho.clone(); Py::Bool({ let t2901 = Py::Int(1i64); if py::ge(&t2900, &t2901)? { true } else { false } }) }).truthy() {
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
    if ({ let t2925 = { let t2921 = v_lam.clone(); Py::Bool({ let t2922 = Py::Int(0i64); if py::le(&t2921, &t2922)? { true } else { false } }) }; if t2925.truthy() { t2925 } else { { let t2923 = v_mu.clone(); Py::Bool({ let t2924 = Py::Int(0i64); if py::le(&t2923, &t2924)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the arrival and service rates must be > 0"))));
    }
    if (py::not(&{ let t2926 = Py::Int(1i64); Py::Bool({ let t2927 = v_c.clone(); if py::le(&t2926, &t2927)? { { let t2928 = v_K.clone(); if py::le(&t2927, &t2928)? { { let t2929 = Py::Int(2000i64); if py::le(&t2928, &t2929)? { true } else { false } } } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("need 1 <= servers <= system capacity <= 2000"))));
    }
    v_a = py::truediv(&v_lam.clone(), &v_mu.clone())?;
    v_logs = Py::list(vec![]);
    for t2930 in py::range_iter(&[(py::add(&v_K.clone(), &Py::Int(1i64))?).i()?]) {
        v_n = t2930;
        if ({ let t2931 = v_n.clone(); Py::Bool({ let t2932 = v_c.clone(); if py::le(&t2931, &t2932)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_logs.clone(), "append", &[py::sub(&py::mul(&v_n.clone(), &py::math1("log", &v_a.clone())?)?, &py::math1("lgamma", &py::add(&v_n.clone(), &Py::Int(1i64))?)?)?])?;
        } else {
            let _ = py::method(&v_logs.clone(), "append", &[py::add(&py::sub(&py::mul(&v_c.clone(), &py::math1("log", &v_a.clone())?)?, &py::math1("lgamma", &py::add(&v_c.clone(), &Py::Int(1i64))?)?)?, &py::mul(&py::sub(&v_n.clone(), &v_c.clone())?, &py::math1("log", &py::truediv(&v_a.clone(), &v_c.clone())?)?)?)?])?;
        }
    }
    v_mx = py::minmax(&[v_logs.clone()], true)?;
    v_w = { let mut out = vec![]; for t2933 in py::iter(&(v_logs.clone()))? { let mut v_v: Py; 
v_v = t2933;
out.push(py::math1("exp", &py::sub(&v_v.clone(), &v_mx.clone())?)?); } Py::list(out) };
    v_tot = py::fsum(&v_w.clone())?;
    v_pi = { let mut out = vec![]; for t2934 in py::iter(&(v_w.clone()))? { let mut v_v: Py; 
v_v = t2934;
out.push(py::truediv(&v_v.clone(), &v_tot.clone())?); } Py::list(out) };
    v_L = py::fsum(&{ let mut out = vec![]; for t2935 in py::iter(&(py::enumerate(&v_pi.clone(), 0)?))? { let mut v_n: Py; let mut v_p: Py; 
let t2936 = (t2935).items()?;
if t2936.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_n = t2936[0].clone();
v_p = t2936[1].clone();
out.push(py::mul(&v_n.clone(), &v_p.clone())?); } Py::list(out) })?;
    v_Lq = py::fsum(&{ let mut out = vec![]; for t2937 in py::iter(&(py::enumerate(&v_pi.clone(), 0)?))? { let mut v_n: Py; let mut v_p: Py; 
let t2938 = (t2937).items()?;
if t2938.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_n = t2938[0].clone();
v_p = t2938[1].clone();
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
    if ({ let t2881 = { let t2874 = v_lam.clone(); Py::Bool({ let t2875 = Py::Int(0i64); if py::le(&t2874, &t2875)? { true } else { false } }) }; if t2881.truthy() { t2881 } else { { let t2880 = { let t2876 = v_mu.clone(); Py::Bool({ let t2877 = Py::Int(0i64); if py::le(&t2876, &t2877)? { true } else { false } }) }; if t2880.truthy() { t2880 } else { { let t2878 = v_target.clone(); Py::Bool({ let t2879 = Py::Int(0i64); if py::le(&t2878, &t2879)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("rates and the target must be > 0"))));
    }
    if ({ let t2886 = { let t2882 = v_criterion.clone(); Py::Bool({ let t2883 = Py::str("wait_probability"); if py::eq(&t2882, &t2883) { true } else { false } }) }; if !t2886.truthy() { t2886 } else { { let t2884 = v_target.clone(); Py::Bool({ let t2885 = Py::Int(1i64); if py::ge(&t2884, &t2885)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a target probability of waiting must be below 1"))));
    }
    v_a = py::truediv(&v_lam.clone(), &v_mu.clone())?;
    if ({ let t2887 = v_a.clone(); Py::Bool({ let t2888 = Py::Int(400i64); if py::gt(&t2887, &t2888)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("offered load too large (at most 400)"))));
    }
    v_c = py::add(&py::int(&py::math1("floor", &v_a.clone())?)?, &Py::Int(1i64))?;
    while (Py::Bool(true)).truthy() {
        v_r = f_mmc(v_lam.clone(), v_mu.clone(), v_c.clone())?;
        v_val = (if ({ let t2903 = v_criterion.clone(); Py::Bool({ let t2904 = Py::str("wait_probability"); if py::eq(&t2903, &t2904) { true } else { false } }) }).truthy() { py::getitem(&v_r.clone(), &Py::str("P_wait"))? } else { py::getitem(&v_r.clone(), &Py::str("Wq"))? });
        if ({ let t2905 = v_val.clone(); Py::Bool({ let t2906 = v_target.clone(); if py::le(&t2905, &t2906)? { true } else { false } }) }).truthy() {
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
    if ({ let t2946 = py::len(&v_d.clone())?; Py::Bool({ let t2947 = py::len(&v_p.clone())?; if py::ne(&t2946, &t2947) { true } else { false } }) }).truthy() {
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
    let t2948 = (Py::tuple(vec![Py::list(vec![]), py::fraction(&[Py::Int(0i64)])?, Py::list(vec![])])).items()?;
    if t2948.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_sched = t2948[0].clone();
    v_t = t2948[1].clone();
    v_tardy = t2948[2].clone();
    for t2949 in py::iter(&(v_order.clone()))? {
        v_j = t2949;
        let _ = py::method(&v_sched.clone(), "append", &[v_j.clone()])?;
        v_t = py::iadd(&v_t, &f__fr(py::getitem(&v_p.clone(), &v_j.clone())?)?)?;
        if ({ let t2950 = v_t.clone(); Py::Bool({ let t2951 = f__fr(py::getitem(&v_d.clone(), &v_j.clone())?)?; if py::gt(&t2950, &t2951)? { true } else { false } }) }).truthy() {
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
    return Ok(Py::dict(vec![(Py::str("min_tardy_jobs"), py::float(&py::len(&v_tardy.clone())?)?), (Py::str("on_time_sequence"), { let mut out = vec![]; for t2952 in py::iter(&(v_sched.clone()))? { let mut v_j: Py; 
v_j = t2952;
out.push(py::add(&v_j.clone(), &Py::Int(1i64))?); } Py::list(out) }), (Py::str("tardy_jobs"), py::sorted(&{ let mut out = vec![]; for t2953 in py::iter(&(v_tardy.clone()))? { let mut v_j: Py; 
v_j = t2953;
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
    v_parent = { let mut out = vec![]; for t2654 in py::iter(&(v_nodes.clone()))? { let mut v_v: Py; 
v_v = t2654;
out.push((v_v.clone(), v_v.clone())); } Py::dict(out) };
    v_find = { let v_E = v_E.clone(); let v_edges = v_edges.clone(); let v_find = v_find.clone(); let v_nodes = v_nodes.clone(); let v_parent = v_parent.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            while ({ let t2655 = py::getitem(&v_parent.clone(), &v_x.clone())?; Py::Bool({ let t2656 = v_x.clone(); if py::ne(&t2655, &t2656) { true } else { false } }) }).truthy() {
                py::setitem(&v_parent.clone(), &v_x.clone(), py::getitem(&v_parent.clone(), &py::getitem(&v_parent.clone(), &v_x.clone())?)?)?;
                v_x = py::getitem(&v_parent.clone(), &v_x.clone())?;
            }
            return Ok(v_x.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_tree = Py::list(vec![]);
    for t2657 in py::iter(&(py::sorted_key(&Py::list((py::range_iter(&[(py::len(&v_E.clone())?).i()?])).into_iter().collect::<Vec<Py>>()), &{ let v_E = v_E.clone(); let v_edges = v_edges.clone(); let v_find = v_find.clone(); let v_nodes = v_nodes.clone(); let v_parent = v_parent.clone(); let v_tree = v_tree.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_k: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::getitem(&py::getitem(&v_E.clone(), &v_k.clone())?, &Py::Int(2i64))?, v_k.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?))? {
        v_k = t2657;
        let t2658 = (py::getitem(&v_E.clone(), &v_k.clone())?).items()?;
        if t2658.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_u = t2658[0].clone();
        v_v = t2658[1].clone();
        v_w = t2658[2].clone();
        let t2659 = (Py::tuple(vec![py::call(&v_find.clone(), &[v_u.clone()])?, py::call(&v_find.clone(), &[v_v.clone()])?])).items()?;
        if t2659.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_ru = t2659[0].clone();
        v_rv = t2659[1].clone();
        if ({ let t2660 = v_ru.clone(); Py::Bool({ let t2661 = v_rv.clone(); if py::ne(&t2660, &t2661) { true } else { false } }) }).truthy() {
            py::setitem(&v_parent.clone(), &v_ru.clone(), v_rv.clone())?;
            let _ = py::method(&v_tree.clone(), "append", &[v_k.clone()])?;
        }
    }
    if ({ let t2662 = py::len(&v_tree.clone())?; Py::Bool({ let t2663 = py::sub(&py::len(&v_nodes.clone())?, &Py::Int(1i64))?; if py::ne(&t2662, &t2663) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the graph is not connected: no spanning tree exists"))));
    }
    v_tset = py::set(&v_tree.clone())?;
    v_adj = { let mut out = vec![]; for t2664 in py::iter(&(v_nodes.clone()))? { let mut v_v: Py; 
v_v = t2664;
out.push((v_v.clone(), Py::list(vec![]))); } Py::dict(out) };
    for t2665 in py::iter(&(v_tree.clone()))? {
        v_k = t2665;
        let t2666 = (py::getitem(&v_E.clone(), &v_k.clone())?).items()?;
        if t2666.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_u = t2666[0].clone();
        v_v = t2666[1].clone();
        v_w = t2666[2].clone();
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
            let t2667 = (Py::tuple(vec![Py::list(vec![Py::tuple(vec![v_a.clone(), py::fraction(&[py::neg(&py::pow(&Py::Int(10i64), &Py::Int(18i64))?)?])?])]), py::set(&Py::list(vec![v_a.clone()]))?])).items()?;
            if t2667.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_st = t2667[0].clone();
            v_seen = t2667[1].clone();
            while (v_st.clone()).truthy() {
                let t2668 = (py::method(&v_st.clone(), "pop", &[])?).items()?;
                if t2668.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_x = t2668[0].clone();
                v_m = t2668[1].clone();
                if ({ let t2669 = v_x.clone(); Py::Bool({ let t2670 = v_b.clone(); if py::eq(&t2669, &t2670) { true } else { false } }) }).truthy() {
                    return Ok(v_m.clone());
                }
                for t2671 in py::iter(&(py::getitem(&v_adj.clone(), &v_x.clone())?))? {
                    let t2674 = (t2671).items()?;
                    if t2674.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    v_y = t2674[0].clone();
                    v_w = t2674[1].clone();
                    if ({ let t2672 = v_y.clone(); Py::Bool({ let t2673 = v_seen.clone(); if !py::contains(&t2673, &t2672)? { true } else { false } }) }).truthy() {
                        let _ = py::method(&v_seen.clone(), "add", &[v_y.clone()])?;
                        let _ = py::method(&v_st.clone(), "append", &[Py::tuple(vec![v_y.clone(), py::minmax(&[v_m.clone(), v_w.clone()], true)?])])?;
                    }
                }
            }
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_unique = Py::Bool(({ let mut out = vec![]; for t2675 in py::iter(&(py::enumerate(&v_E.clone(), 0)?))? { let mut v_k: Py; let mut v_u: Py; let mut v_v: Py; let mut v_w: Py; 
let t2676 = (t2675).items()?;
if t2676.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2676[0].clone();
let t2677 = (t2676[1].clone()).items()?;
if t2677.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_u = t2677[0].clone();
v_v = t2677[1].clone();
v_w = t2677[2].clone();
if ({ let t2682 = { let t2678 = v_k.clone(); Py::Bool({ let t2679 = v_tset.clone(); if !py::contains(&t2679, &t2678)? { true } else { false } }) }; if !t2682.truthy() { t2682 } else { { let t2680 = v_u.clone(); Py::Bool({ let t2681 = v_v.clone(); if py::ne(&t2680, &t2681) { true } else { false } }) } } }).truthy() { out.push({ let t2683 = py::call(&v_maxw.clone(), &[v_u.clone(), v_v.clone()])?; Py::Bool({ let t2684 = v_w.clone(); if py::lt(&t2683, &t2684)? { true } else { false } }) }); }} Py::list(out) }).items()?.iter().all(|x| x.truthy()));
    v_out = Py::dict(vec![(Py::str("total_weight"), py::float(&py::sum(&{ let mut out = vec![]; for t2685 in py::iter(&(v_tree.clone()))? { let mut v_k: Py; 
v_k = t2685;
out.push(py::getitem(&py::getitem(&v_E.clone(), &v_k.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?)?), (Py::str("tree_unique"), (if (v_unique.clone()).truthy() { Py::str("yes") } else { Py::str("no") }))]);
    py::setitem(&v_out.clone(), &(if (v_unique.clone()).truthy() { Py::str("tree_edges") } else { Py::str("tree_edges_example") }), { let mut out = vec![]; for t2686 in py::iter(&(v_tree.clone()))? { let mut v_k: Py; 
v_k = t2686;
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
    if ({ let t2977 = { let t2975 = py::len(&v_demands.clone())?; Py::Bool({ let t2976 = py::len(&v_probs.clone())?; if py::ne(&t2975, &t2976) { true } else { false } }) }; if t2977.truthy() { t2977 } else { py::not(&v_demands.clone()) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give one probability per demand value"))));
    }
    if ({ let t2983 = Py::Bool(({ let mut out = vec![]; for t2978 in py::iter(&(v_probs.clone()))? { let mut v_pr: Py; 
v_pr = t2978;
out.push({ let t2979 = v_pr.clone(); Py::Bool({ let t2980 = Py::Int(0i64); if py::lt(&t2979, &t2980)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t2983.truthy() { t2983 } else { { let t2981 = py::abs(&py::sub(&py::sum(&v_probs.clone(), None)?, &Py::Int(1i64))?)?; Py::Bool({ let t2982 = Py::Float(1e-09); if py::gt(&t2981, &t2982)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the probabilities must be >= 0 and sum to 1"))));
    }
    if ({ let t2990 = Py::Bool(({ let mut out = vec![]; for t2984 in py::iter(&(py::zip(&[v_demands.clone(), py::slice(&v_demands.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_d1: Py; let mut v_d2: Py; 
let t2985 = (t2984).items()?;
if t2985.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_d1 = t2985[0].clone();
v_d2 = t2985[1].clone();
out.push({ let t2986 = v_d2.clone(); Py::Bool({ let t2987 = v_d1.clone(); if py::le(&t2986, &t2987)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t2990.truthy() { t2990 } else { { let t2988 = py::getitem(&v_demands.clone(), &Py::Int(0i64))?; Py::Bool({ let t2989 = Py::Int(0i64); if py::lt(&t2988, &t2989)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("demand values must be >= 0 and increasing"))));
    }
    v_P = { let mut out = vec![]; for t2991 in py::iter(&(v_probs.clone()))? { let mut v_v: Py; 
v_v = t2991;
out.push(f__fr(v_v.clone())?); } Py::list(out) };
    v_tot = py::sum(&v_P.clone(), None)?;
    v_P = { let mut out = vec![]; for t2992 in py::iter(&(v_P.clone()))? { let mut v_v: Py; 
v_v = t2992;
out.push(py::truediv(&v_v.clone(), &v_tot.clone())?); } Py::list(out) };
    v_cr = py::truediv(&py::sub(&f__fr(v_p.clone())?, &f__fr(v_c.clone())?)?, &py::sub(&f__fr(v_p.clone())?, &f__fr(v_s.clone())?)?)?;
    let t2993 = (Py::tuple(vec![py::fraction(&[Py::Int(0i64)])?, py::getitem(&v_demands.clone(), &py::neg(&Py::Int(1i64))?)?])).items()?;
    if t2993.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cum = t2993[0].clone();
    v_Q = t2993[1].clone();
    for t2994 in py::iter(&(py::zip(&[v_demands.clone(), v_P.clone()])?))? {
        let t2997 = (t2994).items()?;
        if t2997.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_d = t2997[0].clone();
        v_pr = t2997[1].clone();
        v_cum = py::iadd(&v_cum, &v_pr.clone())?;
        if ({ let t2995 = v_cum.clone(); Py::Bool({ let t2996 = v_cr.clone(); if py::ge(&t2995, &t2996)? { true } else { false } }) }).truthy() {
            v_Q = v_d.clone();
            break;
        }
    }
    v_Df = { let mut out = vec![]; for t2998 in py::iter(&(v_demands.clone()))? { let mut v_d: Py; 
v_d = t2998;
out.push(f__fr(v_d.clone())?); } Py::list(out) };
    v_Qf = f__fr(v_Q.clone())?;
    v_prof = py::sum(&{ let mut out = vec![]; for t2999 in py::iter(&(py::zip(&[v_Df.clone(), v_P.clone()])?))? { let mut v_d: Py; let mut v_pr: Py; 
let t3000 = (t2999).items()?;
if t3000.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_d = t3000[0].clone();
v_pr = t3000[1].clone();
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
    if ({ let t3011 = v_sigma.clone(); Py::Bool({ let t3012 = Py::Int(0i64); if py::le(&t3011, &t3012)? { true } else { false } }) }).truthy() {
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
    if (py::not(&{ let t3020 = Py::Int(0i64); Py::Bool({ let t3021 = v_lo.clone(); if py::le(&t3020, &t3021)? { { let t3022 = v_hi.clone(); if py::lt(&t3021, &t3022)? { true } else { false } } } else { false } }) })).truthy() {
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
    if ({ let t3031 = { let t3027 = v_L.clone(); Py::Bool({ let t3028 = Py::Int(0i64); if py::le(&t3027, &t3028)? { true } else { false } }) }; if t3031.truthy() { t3031 } else { { let t3029 = v_W.clone(); Py::Bool({ let t3030 = Py::Int(0i64); if py::le(&t3029, &t3030)? { true } else { false } }) } } }).truthy() {
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
    if ({ let t3049 = { let t3039 = v_d.clone(); Py::Bool({ let t3040 = Py::Int(0i64); if py::lt(&t3039, &t3040)? { true } else { false } }) }; if t3049.truthy() { t3049 } else { { let t3048 = { let t3041 = v_sigma.clone(); Py::Bool({ let t3042 = Py::Int(0i64); if py::lt(&t3041, &t3042)? { true } else { false } }) }; if t3048.truthy() { t3048 } else { { let t3047 = { let t3043 = v_R.clone(); Py::Bool({ let t3044 = Py::Int(0i64); if py::le(&t3043, &t3044)? { true } else { false } }) }; if t3047.truthy() { t3047 } else { { let t3045 = v_L.clone(); Py::Bool({ let t3046 = Py::Int(0i64); if py::lt(&t3045, &t3046)? { true } else { false } }) } } } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("demand and its standard deviation must be >= 0, the review period > 0 and the lead time >= 0"))));
    }
    if (py::not(&{ let t3050 = Py::Float(0.5); Py::Bool({ let t3051 = v_SL.clone(); if py::le(&t3050, &t3051)? { { let t3052 = Py::Int(1i64); if py::lt(&t3051, &t3052)? { true } else { false } } } else { false } }) })).truthy() {
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
    if (py::not(&{ let t3060 = Py::Int(1i64); Py::Bool({ let t3061 = v_m.clone(); if py::le(&t3060, &t3061)? { { let t3062 = Py::Int(6i64); if py::le(&t3061, &t3062)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the number of machines must be between 1 and 6"))));
    }
    v_P = { let mut out = vec![]; for t3063 in py::iter(&(v_p.clone()))? { let mut v_v: Py; 
v_v = t3063;
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
    for t3064 in py::iter(&(v_order.clone()))? {
        v_j = t3064;
        v_k = py::minmax_key(&[Py::list((py::range_iter(&[(v_m.clone()).i()?])).into_iter().collect::<Vec<Py>>())], &{ let v_P = v_P.clone(); let v_assign = v_assign.clone(); let v_j = v_j.clone(); let v_loads = v_loads.clone(); let v_m = v_m.clone(); let v_order = v_order.clone(); let v_p = v_p.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_i: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::getitem(&v_loads.clone(), &v_i.clone())?, v_i.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, false)?;
        let t3065 = v_loads.clone(); let t3066 = v_k.clone();
        py::setitem(&t3065, &t3066, py::iadd(&py::getitem(&t3065, &t3066)?, &py::getitem(&v_P.clone(), &v_j.clone())?)?)?;
        py::setitem(&v_assign.clone(), &v_j.clone(), v_k.clone())?;
    }
    v_best = Py::list(vec![py::minmax(&[v_loads.clone()], true)?, Py::list((v_assign.clone()).items()?)]);
    v_lb = py::minmax(&[py::minmax(&[v_P.clone()], true)?, py::truediv(&py::sum(&v_P.clone(), None)?, &v_m.clone())?], true)?;
    v_cur = py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &v_m.clone())?;
    v_a = py::mul(&Py::list(vec![Py::Int(0i64)]), &py::len(&v_P.clone())?)?;
    let rec3067 = py::rec_cell();
    v_dfs = { let v_P = v_P.clone(); let v_a = v_a.clone(); let v_assign = v_assign.clone(); let v_best = v_best.clone(); let v_cur = v_cur.clone(); let v_j = v_j.clone(); let v_k = v_k.clone(); let v_lb = v_lb.clone(); let v_loads = v_loads.clone(); let v_m = v_m.clone(); let v_order = v_order.clone(); let v_p = v_p.clone(); let rec3067 = rec3067.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            let v_dfs: Py = py::rec_get(&rec3067)?;
            let mut v_j: Py = Py::None;
            let mut v_k: Py = Py::None;
            let mut v_mk: Py = Py::None;
            let mut v_seen: Py = Py::None;
            if ({ let t3068 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; Py::Bool({ let t3069 = v_lb.clone(); if py::eq(&t3068, &t3069) { true } else { false } }) }).truthy() {
                return Ok(Py::None);
            }
            if ({ let t3070 = v_t.clone(); Py::Bool({ let t3071 = py::len(&v_order.clone())?; if py::eq(&t3070, &t3071) { true } else { false } }) }).truthy() {
                v_mk = py::minmax(&[v_cur.clone()], true)?;
                if ({ let t3072 = v_mk.clone(); Py::Bool({ let t3073 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; if py::lt(&t3072, &t3073)? { true } else { false } }) }).truthy() {
                    let t3074 = (Py::tuple(vec![v_mk.clone(), Py::list((v_a.clone()).items()?)])).items()?;
                    if t3074.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    py::setitem(&v_best.clone(), &Py::Int(0i64), t3074[0].clone())?;
                    py::setitem(&v_best.clone(), &Py::Int(1i64), t3074[1].clone())?;
                }
                return Ok(Py::None);
            }
            v_j = py::getitem(&v_order.clone(), &v_t.clone())?;
            v_seen = py::set(&Py::list(vec![]))?;
            for t3075 in py::range_iter(&[(v_m.clone()).i()?]) {
                v_k = t3075;
                if ({ let t3080 = { let t3076 = py::getitem(&v_cur.clone(), &v_k.clone())?; Py::Bool({ let t3077 = v_seen.clone(); if py::contains(&t3077, &t3076)? { true } else { false } }) }; if t3080.truthy() { t3080 } else { { let t3078 = py::add(&py::getitem(&v_cur.clone(), &v_k.clone())?, &py::getitem(&v_P.clone(), &v_j.clone())?)?; Py::Bool({ let t3079 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; if py::ge(&t3078, &t3079)? { true } else { false } }) } } }).truthy() {
                    continue;
                }
                let _ = py::method(&v_seen.clone(), "add", &[py::getitem(&v_cur.clone(), &v_k.clone())?])?;
                let t3081 = v_cur.clone(); let t3082 = v_k.clone();
                py::setitem(&t3081, &t3082, py::iadd(&py::getitem(&t3081, &t3082)?, &py::getitem(&v_P.clone(), &v_j.clone())?)?)?;
                py::setitem(&v_a.clone(), &v_j.clone(), v_k.clone())?;
                let _ = py::call(&v_dfs.clone(), &[py::add(&v_t.clone(), &Py::Int(1i64))?])?;
                let t3083 = v_cur.clone(); let t3084 = v_k.clone();
                py::setitem(&t3083, &t3084, py::sub(&py::getitem(&t3083, &t3084)?, &py::getitem(&v_P.clone(), &v_j.clone())?)?)?;
            }
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec3067, &v_dfs);
    let _ = py::call(&v_dfs.clone(), &[Py::Int(0i64)])?;
    return Ok(Py::dict(vec![(Py::str("makespan"), py::float(&py::getitem(&v_best.clone(), &Py::Int(0i64))?)?), (Py::str("machine_of_job"), { let mut out = vec![]; for t3085 in py::iter(&(py::getitem(&v_best.clone(), &Py::Int(1i64))?))? { let mut v_k: Py; 
v_k = t3085;
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
    if (Py::Bool(({ let mut out = vec![]; for t3098 in py::iter(&(v_times.clone()))? { let mut v_r: Py; 
v_r = t3098;
out.push({ let t3099 = py::len(&v_r.clone())?; Py::Bool({ let t3100 = Py::Int(3i64); if py::ne(&t3099, &t3100) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("each activity needs three times: optimistic, most likely, pessimistic"))));
    }
    for t3101 in py::iter(&(v_times.clone()))? {
        let t3106 = (t3101).items()?;
        if t3106.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_a = t3106[0].clone();
        v_m = t3106[1].clone();
        v_b = t3106[2].clone();
        if (py::not(&{ let t3102 = Py::Int(0i64); Py::Bool({ let t3103 = v_a.clone(); if py::le(&t3102, &t3103)? { { let t3104 = v_m.clone(); if py::le(&t3103, &t3104)? { { let t3105 = v_b.clone(); if py::le(&t3104, &t3105)? { true } else { false } } } else { false } } } else { false } }) })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("each activity needs 0 <= optimistic <= most likely <= pessimistic"))));
        }
    }
    v_te = { let mut out = vec![]; for t3107 in py::iter(&(v_times.clone()))? { let mut v_a: Py; let mut v_b: Py; let mut v_m: Py; 
let t3108 = (t3107).items()?;
if t3108.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_a = t3108[0].clone();
v_m = t3108[1].clone();
v_b = t3108[2].clone();
out.push(py::truediv(&py::add(&py::add(&f__fr(v_a.clone())?, &py::mul(&Py::Int(4i64), &f__fr(v_m.clone())?)?)?, &f__fr(v_b.clone())?)?, &Py::Int(6i64))?); } Py::list(out) };
    v_var = { let mut out = vec![]; for t3109 in py::iter(&(v_times.clone()))? { let mut v_a: Py; let mut v_b: Py; let mut v_m: Py; 
let t3110 = (t3109).items()?;
if t3110.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_a = t3110[0].clone();
v_m = t3110[1].clone();
v_b = t3110[2].clone();
out.push(py::pow(&py::truediv(&py::sub(&f__fr(v_b.clone())?, &f__fr(v_a.clone())?)?, &Py::Int(6i64))?, &Py::Int(2i64))?); } Py::list(out) };
    let t3139 = (f__project({ let mut out = vec![]; for t3138 in py::iter(&(v_te.clone()))? { let mut v_v: Py; 
v_v = t3138;
out.push(py::float(&v_v.clone())?); } Py::list(out) }, v_precedences.clone())?).items()?;
    if t3139.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_preds = t3139[0].clone();
    v_order = t3139[1].clone();
    v_n = py::len(&v_te.clone())?;
    let t3140 = (Py::tuple(vec![py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &v_n.clone())?, py::mul(&Py::list(vec![Py::Int(1i64)]), &v_n.clone())?])).items()?;
    if t3140.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ES = t3140[0].clone();
    v_cnt = t3140[1].clone();
    for t3141 in py::iter(&(v_order.clone()))? {
        v_k = t3141;
        if (py::getitem(&v_preds.clone(), &v_k.clone())?).truthy() {
            v_best = py::minmax(&[{ let mut out = vec![]; for t3142 in py::iter(&(py::getitem(&v_preds.clone(), &v_k.clone())?))? { let mut v_j: Py; 
v_j = t3142;
out.push(py::add(&py::getitem(&v_ES.clone(), &v_j.clone())?, &py::getitem(&v_te.clone(), &v_j.clone())?)?); } Py::list(out) }], true)?;
            let t3146 = (Py::tuple(vec![v_best.clone(), py::sum(&{ let mut out = vec![]; for t3143 in py::iter(&(py::getitem(&v_preds.clone(), &v_k.clone())?))? { let mut v_j: Py; 
v_j = t3143;
if ({ let t3144 = py::add(&py::getitem(&v_ES.clone(), &v_j.clone())?, &py::getitem(&v_te.clone(), &v_j.clone())?)?; Py::Bool({ let t3145 = v_best.clone(); if py::eq(&t3144, &t3145) { true } else { false } }) }).truthy() { out.push(py::getitem(&v_cnt.clone(), &v_j.clone())?); }} Py::list(out) }, None)?])).items()?;
            if t3146.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            py::setitem(&v_ES.clone(), &v_k.clone(), t3146[0].clone())?;
            py::setitem(&v_cnt.clone(), &v_k.clone(), t3146[1].clone())?;
        }
    }
    v_finish = { let mut out = vec![]; for t3147 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t3147;
out.push(py::add(&py::getitem(&v_ES.clone(), &v_k.clone())?, &py::getitem(&v_te.clone(), &v_k.clone())?)?); } Py::list(out) };
    v_T = py::minmax(&[v_finish.clone()], true)?;
    v_ends = { let mut out = vec![]; for t3148 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t3148;
if ({ let t3149 = py::getitem(&v_finish.clone(), &v_k.clone())?; Py::Bool({ let t3150 = v_T.clone(); if py::eq(&t3149, &t3150) { true } else { false } }) }).truthy() { out.push(v_k.clone()); }} Py::list(out) };
    if ({ let t3151 = py::sum(&{ let mut out = vec![]; for t3153 in py::iter(&(v_ends.clone()))? { let mut v_k: Py; 
v_k = t3153;
out.push(py::getitem(&v_cnt.clone(), &v_k.clone())?); } Py::list(out) }, None)?; Py::Bool({ let t3152 = Py::Int(1i64); if py::gt(&t3151, &t3152)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("there is more than one critical path, so the PERT variance of the project is not determined"))));
    }
    let t3154 = (Py::tuple(vec![Py::list(vec![]), py::getitem(&v_ends.clone(), &Py::Int(0i64))?])).items()?;
    if t3154.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_path = t3154[0].clone();
    v_k = t3154[1].clone();
    while (Py::Bool(true)).truthy() {
        let _ = py::method(&v_path.clone(), "append", &[v_k.clone()])?;
        if (py::not(&py::getitem(&v_preds.clone(), &v_k.clone())?)).truthy() {
            break;
        }
        v_k = (match ('g3155: { let mut out = vec![]; for t3156 in py::iter(&(py::getitem(&v_preds.clone(), &v_k.clone())?))? { let mut v_j: Py; 
v_j = t3156;
if ({ let t3157 = py::add(&py::getitem(&v_ES.clone(), &v_j.clone())?, &py::getitem(&v_te.clone(), &v_j.clone())?)?; Py::Bool({ let t3158 = py::getitem(&v_ES.clone(), &v_k.clone())?; if py::eq(&t3157, &t3158) { true } else { false } }) }).truthy() { out.push(v_j.clone()); break 'g3155 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) });
    }
    let _ = py::method(&v_path.clone(), "reverse", &[])?;
    v_V = py::sum(&{ let mut out = vec![]; for t3159 in py::iter(&(v_path.clone()))? { let mut v_k: Py; 
v_k = t3159;
out.push(py::getitem(&v_var.clone(), &v_k.clone())?); } Py::list(out) }, None)?;
    v_out = Py::dict(vec![(Py::str("expected_duration"), py::float(&v_T.clone())?), (Py::str("variance"), py::float(&v_V.clone())?), (Py::str("std_dev"), py::math1("sqrt", &v_V.clone())?), (Py::str("critical_path"), { let mut out = vec![]; for t3160 in py::iter(&(v_path.clone()))? { let mut v_k: Py; 
v_k = t3160;
out.push(py::chr(&py::add(&Py::Int(65i64), &v_k.clone())?)?); } Py::list(out) }), (Py::str("expected_times"), f__fl(v_te.clone())?)]);
    if ({ let t3162 = v_deadline.clone(); Py::Bool({ let t3163 = Py::None; if !py::is(&t3162, &t3163) { true } else { false } }) }).truthy() {
        let _ = f__finite(Py::tuple(vec![v_deadline.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("probability"), (if ({ let t3167 = v_V.clone(); Py::Bool({ let t3168 = Py::Int(0i64); if py::eq(&t3167, &t3168) { true } else { false } }) }).truthy() { (if ({ let t3169 = v_deadline.clone(); Py::Bool({ let t3170 = v_T.clone(); if py::ge(&t3169, &t3170)? { true } else { false } }) }).truthy() { Py::Float(1.0) } else { Py::Float(0.0) }) } else { py::normal_dist("cdf", &Py::Float(0.0), &Py::Float(1.0), &[py::truediv(&py::sub(&v_deadline.clone(), &py::float(&v_T.clone())?)?, &py::math1("sqrt", &v_V.clone())?)?])? }))?;
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
    if (py::not(&{ let t3182 = v_lo.clone(); Py::Bool({ let t3183 = v_hi.clone(); if py::lt(&t3182, &t3183)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the interval needs lo < hi"))));
    }
    v_cs = { let mut out = vec![]; for t3184 in py::iter(&(v_coefficients.clone()))? { let mut v_v: Py; 
v_v = t3184;
out.push(py::float(&v_v.clone())?); } Py::list(out) };
    if ({ let t3189 = { let t3185 = py::len(&v_cs.clone())?; Py::Bool({ let t3186 = Py::Int(2i64); if py::lt(&t3185, &t3186)? { true } else { false } }) }; if t3189.truthy() { t3189 } else { { let t3187 = py::len(&v_cs.clone())?; Py::Bool({ let t3188 = Py::Int(6i64); if py::gt(&t3187, &t3188)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give 2 to 6 coefficients (degree 1 to 5), highest power first"))));
    }
    v_cand = py::add(&Py::list(vec![v_lo.clone(), v_hi.clone()]), &f__roots_in(f__pder(v_cs.clone())?, v_lo.clone(), v_hi.clone())?)?;
    v_vals = { let mut out = vec![]; for t3227 in py::iter(&(v_cand.clone()))? { let mut v_x: Py; 
v_x = t3227;
out.push(Py::tuple(vec![f__peval(v_cs.clone(), v_x.clone())?, v_x.clone()])); } Py::list(out) };
    let t3232 = (Py::tuple(vec![py::minmax(&[{ let mut out = vec![]; for t3228 in py::iter(&(v_vals.clone()))? { let mut v__: Py; let mut v_v: Py; 
let t3229 = (t3228).items()?;
if t3229.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t3229[0].clone();
v__ = t3229[1].clone();
out.push(v_v.clone()); } Py::list(out) }], false)?, py::minmax(&[{ let mut out = vec![]; for t3230 in py::iter(&(v_vals.clone()))? { let mut v__: Py; let mut v_v: Py; 
let t3231 = (t3230).items()?;
if t3231.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t3231[0].clone();
v__ = t3231[1].clone();
out.push(v_v.clone()); } Py::list(out) }], true)?])).items()?;
    if t3232.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mn = t3232[0].clone();
    v_mx = t3232[1].clone();
    v_tol = { let v_cand = v_cand.clone(); let v_coefficients = v_coefficients.clone(); let v_cs = v_cs.clone(); let v_hi = v_hi.clone(); let v_lo = v_lo.clone(); let v_mn = v_mn.clone(); let v_mx = v_mx.clone(); let v_vals = v_vals.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::mul(&Py::Float(1e-12), &py::minmax(&[Py::Float(1.0), py::abs(&v_t.clone())?], true)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok(Py::dict(vec![(Py::str("min_value"), v_mn.clone()), (Py::str("max_value"), v_mx.clone()), (Py::str("argmin_example"), py::minmax(&[{ let mut out = vec![]; for t3233 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; let mut v_x: Py; 
let t3234 = (t3233).items()?;
if t3234.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t3234[0].clone();
v_x = t3234[1].clone();
if ({ let t3235 = v_v.clone(); Py::Bool({ let t3236 = py::add(&v_mn.clone(), &py::call(&v_tol.clone(), &[v_mn.clone()])?)?; if py::le(&t3235, &t3236)? { true } else { false } }) }).truthy() { out.push(v_x.clone()); }} Py::list(out) }], false)?), (Py::str("argmax_example"), py::minmax(&[{ let mut out = vec![]; for t3237 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; let mut v_x: Py; 
let t3238 = (t3237).items()?;
if t3238.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t3238[0].clone();
v_x = t3238[1].clone();
if ({ let t3239 = v_v.clone(); Py::Bool({ let t3240 = py::sub(&v_mx.clone(), &py::call(&v_tol.clone(), &[v_mx.clone()])?)?; if py::ge(&t3239, &t3240)? { true } else { false } }) }).truthy() { out.push(v_x.clone()); }} Py::list(out) }], false)?)]));
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
    if ({ let t3252 = v_b.clone(); Py::Bool({ let t3253 = Py::Int(0i64); if py::le(&t3252, &t3253)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the demand slope b must be > 0 (price falls as quantity rises)"))));
    }
    if ({ let t3254 = v_a.clone(); Py::Bool({ let t3255 = v_cv.clone(); if py::le(&t3254, &t3255)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the demand intercept must exceed the marginal cost, otherwise no positive output is profitable"))));
    }
    let t3256 = (Py::tuple(vec![f__fr(v_a.clone())?, f__fr(v_b.clone())?, f__fr(v_cv.clone())?, f__fr(v_F_.clone())?])).items()?;
    if t3256.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3256[0].clone();
    v_B = t3256[1].clone();
    v_C = t3256[2].clone();
    v_Fx = t3256[3].clone();
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
    let t3276 = (f__bimatrix(v_A.clone(), v_B.clone(), Py::Int(8i64))?).items()?;
    if t3276.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_MA = t3276[0].clone();
    v_MB = t3276[1].clone();
    v_eq = f__pure_ne(v_MA.clone(), v_MB.clone())?;
    return Ok(Py::dict(vec![(Py::str("count"), py::float(&py::len(&v_eq.clone())?)?), (Py::str("equilibria"), { let mut out = vec![]; for t3287 in py::iter(&(v_eq.clone()))? { let mut v_a: Py; let mut v_b: Py; 
let t3288 = (t3287).items()?;
if t3288.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3288[0].clone();
v_b = t3288[1].clone();
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
    if ({ let t3937 = py::len(&v_c.clone())?; Py::Bool({ let t3938 = py::len(&v_M.clone())?; if py::ne(&t3937, &t3938) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["c must have ".to_string(), py::format(&py::len(&v_M.clone())?, "")?, " entries".to_string()].concat()))));
    }
    v_cf = { let mut out = vec![]; for t3939 in py::iter(&(v_c.clone()))? { let mut v_v: Py; 
v_v = t3939;
out.push(f__fr(v_v.clone())?); } Py::list(out) };
    v_x = f__solve_exact(v_M.clone(), { let mut out = vec![]; for t3959 in py::iter(&(v_cf.clone()))? { let mut v_v: Py; 
v_v = t3959;
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
    if ({ let t3300 = v_a.clone(); Py::Bool({ let t3301 = Py::Int(0i64); if py::eq(&t3300, &t3301) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a must not be 0: a linear function has no interior optimum"))));
    }
    if ({ let t3306 = { let t3302 = v_goal.clone(); Py::Bool({ let t3303 = Py::str("minimum"); if py::eq(&t3302, &t3303) { true } else { false } }) }; if !t3306.truthy() { t3306 } else { { let t3304 = v_a.clone(); Py::Bool({ let t3305 = Py::Int(0i64); if py::lt(&t3304, &t3305)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("with a < 0 the parabola opens downward: it has a maximum, no minimum"))));
    }
    if ({ let t3311 = { let t3307 = v_goal.clone(); Py::Bool({ let t3308 = Py::str("maximum"); if py::eq(&t3307, &t3308) { true } else { false } }) }; if !t3311.truthy() { t3311 } else { { let t3309 = v_a.clone(); Py::Bool({ let t3310 = Py::Int(0i64); if py::gt(&t3309, &t3310)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("with a > 0 the parabola opens upward: it has a minimum, no maximum"))));
    }
    let t3312 = (Py::tuple(vec![f__fr(v_a.clone())?, f__fr(v_b.clone())?, f__fr(v_c.clone())?])).items()?;
    if t3312.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3312[0].clone();
    v_B = t3312[1].clone();
    v_Cc = t3312[2].clone();
    v_x = py::truediv(&py::neg(&v_B.clone())?, &py::mul(&Py::Int(2i64), &v_A.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("x_opt"), py::float(&v_x.clone())?), (Py::str("f_opt"), py::float(&py::sub(&v_Cc.clone(), &py::truediv(&py::mul(&v_B.clone(), &v_B.clone())?, &py::mul(&Py::Int(4i64), &v_A.clone())?)?)?)?), (Py::str("type"), (if ({ let t3313 = v_a.clone(); Py::Bool({ let t3314 = Py::Int(0i64); if py::gt(&t3313, &t3314)? { true } else { false } }) }).truthy() { Py::str("minimum") } else { Py::str("maximum") }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_reorder_point(mut v_d: Py, mut v_L: Py, mut v_sigma: Py, mut v_SL: Py) -> R {
    let mut v_ss: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__finite(Py::tuple(vec![v_d.clone(), v_L.clone(), v_sigma.clone(), v_SL.clone()]))?;
    if ({ let t3326 = { let t3319 = v_d.clone(); Py::Bool({ let t3320 = Py::Int(0i64); if py::lt(&t3319, &t3320)? { true } else { false } }) }; if t3326.truthy() { t3326 } else { { let t3325 = { let t3321 = v_L.clone(); Py::Bool({ let t3322 = Py::Int(0i64); if py::le(&t3321, &t3322)? { true } else { false } }) }; if t3325.truthy() { t3325 } else { { let t3323 = v_sigma.clone(); Py::Bool({ let t3324 = Py::Int(0i64); if py::lt(&t3323, &t3324)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("demand must be >= 0, lead time > 0 and the standard deviation >= 0"))));
    }
    if (py::not(&{ let t3327 = Py::Float(0.5); Py::Bool({ let t3328 = v_SL.clone(); if py::le(&t3327, &t3328)? { { let t3329 = Py::Int(1i64); if py::lt(&t3328, &t3329)? { true } else { false } } } else { false } }) })).truthy() {
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
    if ({ let t3338 = { let t3334 = v_alpha.clone(); Py::Bool({ let t3335 = Py::Int(0i64); if py::le(&t3334, &t3335)? { true } else { false } }) }; if t3338.truthy() { t3338 } else { { let t3336 = v_beta.clone(); Py::Bool({ let t3337 = Py::Int(0i64); if py::le(&t3336, &t3337)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("demand q = alpha - beta p needs alpha > 0 and beta > 0"))));
    }
    let t3339 = (Py::tuple(vec![f__fr(v_alpha.clone())?, f__fr(v_beta.clone())?])).items()?;
    if t3339.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3339[0].clone();
    v_B = t3339[1].clone();
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
    if ({ let t3348 = { let t3341 = v_m.clone(); Py::Bool({ let t3342 = Py::Int(0i64); if py::eq(&t3341, &t3342) { true } else { false } }) }; if t3348.truthy() { t3348 } else { { let t3347 = { let t3343 = v_m.clone(); Py::Bool({ let t3344 = Py::Int(18i64); if py::gt(&t3343, &t3344)? { true } else { false } }) }; if t3347.truthy() { t3347 } else { { let t3345 = py::len(&v_costs.clone())?; Py::Bool({ let t3346 = v_m.clone(); if py::ne(&t3345, &t3346) { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give between 1 and 18 sets, one cost each"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t3349 in py::iter(&(v_sets.clone()))? { let mut v_r: Py; 
v_r = t3349;
for t3350 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t3350;
out.push({ let t3351 = v_x.clone(); Py::Bool({ let t3352 = Py::tuple(vec![Py::Int(0i64), Py::Int(1i64)]); if !py::contains(&t3352, &t3351)? { true } else { false } }) }); }} Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the incidence matrix must contain only 0 and 1"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t3353 in py::iter(&(v_costs.clone()))? { let mut v_c: Py; 
v_c = t3353;
out.push({ let t3354 = v_c.clone(); Py::Bool({ let t3355 = Py::Int(0i64); if py::lt(&t3354, &t3355)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("costs must be >= 0"))));
    }
    v_n = py::len(&py::getitem(&v_sets.clone(), &Py::Int(0i64))?)?;
    v_masks = { let mut out = vec![]; for t3356 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t3356;
out.push(py::sum(&{ let mut out = vec![]; for t3357 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3357;
if (py::getitem(&py::getitem(&v_sets.clone(), &v_i.clone())?, &v_j.clone())?).truthy() { out.push(py::lshift(&Py::Int(1i64), &v_j.clone())?); }} Py::list(out) }, None)?); } Py::list(out) };
    v_full = py::sub(&py::lshift(&Py::Int(1i64), &v_n.clone())?, &Py::Int(1i64))?;
    v_C = { let mut out = vec![]; for t3358 in py::iter(&(v_costs.clone()))? { let mut v_c: Py; 
v_c = t3358;
out.push(f__fr(v_c.clone())?); } Py::list(out) };
    v_best = Py::None;
    for t3359 in py::range_iter(&[(py::lshift(&Py::Int(1i64), &v_m.clone())?).i()?]) {
        v_pick = t3359;
        let t3360 = (Py::tuple(vec![Py::Int(0i64), py::fraction(&[Py::Int(0i64)])?])).items()?;
        if t3360.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_cov = t3360[0].clone();
        v_cost = t3360[1].clone();
        for t3361 in py::range_iter(&[(v_m.clone()).i()?]) {
            v_i = t3361;
            if (py::bitand(&py::rshift(&v_pick.clone(), &v_i.clone())?, &Py::Int(1i64))?).truthy() {
                v_cov = py::bitor(&v_cov, &py::getitem(&v_masks.clone(), &v_i.clone())?)?;
                v_cost = py::iadd(&v_cost, &py::getitem(&v_C.clone(), &v_i.clone())?)?;
            }
        }
        if ({ let t3369 = { let t3362 = v_cov.clone(); Py::Bool({ let t3363 = v_full.clone(); if py::eq(&t3362, &t3363) { true } else { false } }) }; if !t3369.truthy() { t3369 } else { { let t3368 = { let t3364 = v_best.clone(); Py::Bool({ let t3365 = Py::None; if py::is(&t3364, &t3365) { true } else { false } }) }; if t3368.truthy() { t3368 } else { { let t3366 = v_cost.clone(); Py::Bool({ let t3367 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; if py::lt(&t3366, &t3367)? { true } else { false } }) } } } } }).truthy() {
            v_best = Py::tuple(vec![v_cost.clone(), v_pick.clone()]);
        }
    }
    if ({ let t3370 = v_best.clone(); Py::Bool({ let t3371 = Py::None; if py::is(&t3370, &t3371) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("some element is in no set: no cover exists"))));
    }
    return Ok(Py::dict(vec![(Py::str("min_cost"), py::float(&py::getitem(&v_best.clone(), &Py::Int(0i64))?)?), (Py::str("sets_example"), { let mut out = vec![]; for t3372 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t3372;
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
    if (Py::Bool(({ let mut out = vec![]; for t3384 in py::iter(&(v_E.clone()))? { let mut v__: Py; let mut v_w: Py; 
let t3385 = (t3384).items()?;
if t3385.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v__ = t3385[0].clone();
v__ = t3385[1].clone();
v_w = t3385[2].clone();
out.push({ let t3386 = v_w.clone(); Py::Bool({ let t3387 = Py::Int(0i64); if py::lt(&t3386, &t3387)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("negative weights: use the Bellman-Ford capability"))));
    }
    let t3388 = (Py::tuple(vec![py::method(&Py::str(&py::to_str(&v_source.clone())), "strip", &[])?, py::method(&Py::str(&py::to_str(&v_target.clone())), "strip", &[])?])).items()?;
    if t3388.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t3388[0].clone();
    v_t = t3388[1].clone();
    if ({ let t3397 = { let t3389 = v_s.clone(); Py::Bool({ let t3390 = f__nodes(v_E.clone())?; if !py::contains(&t3390, &t3389)? { true } else { false } }) }; if t3397.truthy() { t3397 } else { { let t3395 = v_t.clone(); Py::Bool({ let t3396 = f__nodes(v_E.clone())?; if !py::contains(&t3396, &t3395)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the source and the target must be nodes of the graph"))));
    }
    let t3411 = (f__dijkstra(v_E.clone(), v_s.clone(), v_directed.clone())?).items()?;
    if t3411.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_dist = t3411[0].clone();
    v_adj = t3411[1].clone();
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
    return Ok(Py::dict(vec![(Py::str("sequence"), { let mut out = vec![]; for t3480 in py::iter(&(v_seq.clone()))? { let mut v_j: Py; 
v_j = t3480;
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
    if ({ let t3517 = { let t3513 = py::len(&v_c.clone())?; Py::Bool({ let t3514 = v_n.clone(); if py::ne(&t3513, &t3514) { true } else { false } }) }; if t3517.truthy() { t3517 } else { { let t3515 = py::len(&v_x0.clone())?; Py::Bool({ let t3516 = v_n.clone(); if py::ne(&t3515, &t3516) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["c and x0 must have ".to_string(), py::format(&v_n.clone(), "")?, " entries".to_string()].concat()))));
    }
    let t3520 = (Py::tuple(vec![{ let mut out = vec![]; for t3518 in py::iter(&(v_c.clone()))? { let mut v_v: Py; 
v_v = t3518;
out.push(f__fr(v_v.clone())?); } Py::list(out) }, { let mut out = vec![]; for t3519 in py::iter(&(v_x0.clone()))? { let mut v_v: Py; 
v_v = t3519;
out.push(f__fr(v_v.clone())?); } Py::list(out) }])).items()?;
    if t3520.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cf = t3520[0].clone();
    v_x = t3520[1].clone();
    v_g = { let mut out = vec![]; for t3521 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3521;
out.push(py::add(&py::sum(&{ let mut out = vec![]; for t3522 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3522;
out.push(py::mul(&py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_x.clone(), &v_j.clone())?)?); } Py::list(out) }, None)?, &py::getitem(&v_cf.clone(), &v_i.clone())?)?); } Py::list(out) };
    v_gg = py::sum(&{ let mut out = vec![]; for t3523 in py::iter(&(v_g.clone()))? { let mut v_v: Py; 
v_v = t3523;
out.push(py::mul(&v_v.clone(), &v_v.clone())?); } Py::list(out) }, None)?;
    if ({ let t3524 = v_gg.clone(); Py::Bool({ let t3525 = Py::Int(0i64); if py::eq(&t3524, &t3525) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("x0 is already the minimiser (the gradient is zero)"))));
    }
    v_alpha = py::truediv(&v_gg.clone(), &py::sum(&{ let mut out = vec![]; for t3526 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3526;
for t3527 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3527;
out.push(py::mul(&py::mul(&py::getitem(&v_g.clone(), &v_i.clone())?, &py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_j.clone())?)?, &py::getitem(&v_g.clone(), &v_j.clone())?)?); }} Py::list(out) }, None)?)?;
    v_x1 = { let mut out = vec![]; for t3528 in py::iter(&(py::zip(&[v_x.clone(), v_g.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3529 = (t3528).items()?;
if t3529.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3529[0].clone();
v_b = t3529[1].clone();
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
    let t3588 = (f__mcda_in(v_X.clone(), v_w.clone(), v_types.clone())?).items()?;
    if t3588.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_X = t3588[0].clone();
    v_w = t3588[1].clone();
    v_types = t3588[2].clone();
    let t3589 = (Py::tuple(vec![py::len(&v_X.clone())?, py::len(&py::getitem(&v_X.clone(), &Py::Int(0i64))?)?])).items()?;
    if t3589.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t3589[0].clone();
    v_k = t3589[1].clone();
    v_norms = { let mut out = vec![]; for t3590 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t3590;
out.push(py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t3591 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t3591;
out.push(py::pow(&py::getitem(&py::getitem(&v_X.clone(), &v_i.clone())?, &v_j.clone())?, &Py::Int(2i64))?); } Py::list(out) })?)?); } Py::list(out) };
    v_V = { let mut out = vec![]; for t3592 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t3592;
out.push({ let mut out = vec![]; for t3593 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t3593;
out.push(py::truediv(&py::mul(&py::getitem(&v_w.clone(), &v_j.clone())?, &py::getitem(&py::getitem(&v_X.clone(), &v_i.clone())?, &v_j.clone())?)?, &py::getitem(&v_norms.clone(), &v_j.clone())?)?); } Py::list(out) }); } Py::list(out) };
    v_best = { let mut out = vec![]; for t3594 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t3594;
out.push((if ({ let t3595 = py::getitem(&v_types.clone(), &v_j.clone())?; Py::Bool({ let t3596 = Py::Int(1i64); if py::eq(&t3595, &t3596) { true } else { false } }) }).truthy() { py::minmax(&[{ let mut out = vec![]; for t3597 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t3597;
out.push(py::getitem(&py::getitem(&v_V.clone(), &v_i.clone())?, &v_j.clone())?); } Py::list(out) }], true)? } else { py::minmax(&[{ let mut out = vec![]; for t3598 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t3598;
out.push(py::getitem(&py::getitem(&v_V.clone(), &v_i.clone())?, &v_j.clone())?); } Py::list(out) }], false)? })); } Py::list(out) };
    v_worst = { let mut out = vec![]; for t3599 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t3599;
out.push((if ({ let t3600 = py::getitem(&v_types.clone(), &v_j.clone())?; Py::Bool({ let t3601 = Py::Int(1i64); if py::eq(&t3600, &t3601) { true } else { false } }) }).truthy() { py::minmax(&[{ let mut out = vec![]; for t3602 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t3602;
out.push(py::getitem(&py::getitem(&v_V.clone(), &v_i.clone())?, &v_j.clone())?); } Py::list(out) }], false)? } else { py::minmax(&[{ let mut out = vec![]; for t3603 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t3603;
out.push(py::getitem(&py::getitem(&v_V.clone(), &v_i.clone())?, &v_j.clone())?); } Py::list(out) }], true)? })); } Py::list(out) };
    v_sc = Py::list(vec![]);
    for t3604 in py::range_iter(&[(v_m.clone()).i()?]) {
        v_i = t3604;
        v_dp = py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t3605 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t3605;
out.push(py::pow(&py::sub(&py::getitem(&py::getitem(&v_V.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_best.clone(), &v_j.clone())?)?, &Py::Int(2i64))?); } Py::list(out) })?)?;
        v_dm = py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t3606 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t3606;
out.push(py::pow(&py::sub(&py::getitem(&py::getitem(&v_V.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_worst.clone(), &v_j.clone())?)?, &Py::Int(2i64))?); } Py::list(out) })?)?;
        if ({ let t3607 = py::add(&v_dp.clone(), &v_dm.clone())?; Py::Bool({ let t3608 = Py::Int(0i64); if py::eq(&t3607, &t3608) { true } else { false } }) }).truthy() {
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
    let t3669 = (f__transport_lp(v_supply.clone(), v_demand.clone(), v_cost.clone())?).items()?;
    if t3669.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_sense = t3669[0].clone();
    v_c = t3669[1].clone();
    v_A = t3669[2].clone();
    v_b = t3669[3].clone();
    v_rel = t3669[4].clone();
    v_r = f__lp(v_sense.clone(), v_c.clone(), v_A.clone(), v_b.clone(), v_rel.clone(), Py::Bool(true))?;
    v_n = py::len(&v_demand.clone())?;
    v_plan = { let mut out = vec![]; for t3824 in py::range_iter(&[(py::len(&v_supply.clone())?).i()?]) { let mut v_i: Py; 
v_i = t3824;
out.push({ let mut out = vec![]; for t3825 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3825;
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
    if ({ let t3878 = { let t3870 = v_n.clone(); Py::Bool({ let t3871 = Py::Int(2i64); if py::lt(&t3870, &t3871)? { true } else { false } }) }; if t3878.truthy() { t3878 } else { { let t3877 = { let t3872 = v_n.clone(); Py::Bool({ let t3873 = Py::Int(11i64); if py::gt(&t3872, &t3873)? { true } else { false } }) }; if t3877.truthy() { t3877 } else { Py::Bool(({ let mut out = vec![]; for t3874 in py::iter(&(v_D.clone()))? { let mut v_r: Py; 
v_r = t3874;
out.push({ let t3875 = py::len(&v_r.clone())?; Py::Bool({ let t3876 = v_n.clone(); if py::ne(&t3875, &t3876) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give a square distance matrix with 2 to 11 cities"))));
    }
    v_M = { let mut out = vec![]; for t3879 in py::iter(&(v_D.clone()))? { let mut v_r: Py; 
v_r = t3879;
out.push({ let mut out = vec![]; for t3880 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t3880;
out.push(f__fr(v_x.clone())?); } Py::list(out) }); } Py::list(out) };
    v_full = py::lshift(&Py::Int(1i64), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?;
    v_dp = { let mut out = vec![]; for t3881 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t3881;
out.push((Py::tuple(vec![py::lshift(&Py::Int(1i64), &v_k.clone())?, v_k.clone()]), Py::tuple(vec![py::getitem(&py::getitem(&v_M.clone(), &Py::Int(0i64))?, &py::add(&v_k.clone(), &Py::Int(1i64))?)?, Py::Int(0i64)]))); } Py::dict(out) };
    for t3882 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_full.clone()).i()?]) {
        v_mask = t3882;
        for t3883 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
            v_k = t3883;
            if ({ let t3886 = py::not(&py::bitand(&py::rshift(&v_mask.clone(), &v_k.clone())?, &Py::Int(1i64))?); if t3886.truthy() { t3886 } else { { let t3884 = Py::tuple(vec![v_mask.clone(), v_k.clone()]); Py::Bool({ let t3885 = v_dp.clone(); if !py::contains(&t3885, &t3884)? { true } else { false } }) } } }).truthy() {
                continue;
            }
            v_base = py::getitem(&py::getitem(&v_dp.clone(), &Py::tuple(vec![v_mask.clone(), v_k.clone()]))?, &Py::Int(0i64))?;
            for t3887 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
                v_j = t3887;
                if (py::bitand(&py::rshift(&v_mask.clone(), &v_j.clone())?, &Py::Int(1i64))?).truthy() {
                    continue;
                }
                v_key = Py::tuple(vec![py::bitor(&v_mask.clone(), &py::lshift(&Py::Int(1i64), &v_j.clone())?)?, v_j.clone()]);
                v_cand = py::add(&v_base.clone(), &py::getitem(&py::getitem(&v_M.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?, &py::add(&v_j.clone(), &Py::Int(1i64))?)?)?;
                if ({ let t3892 = { let t3888 = v_key.clone(); Py::Bool({ let t3889 = v_dp.clone(); if !py::contains(&t3889, &t3888)? { true } else { false } }) }; if t3892.truthy() { t3892 } else { { let t3890 = v_cand.clone(); Py::Bool({ let t3891 = py::getitem(&py::getitem(&v_dp.clone(), &v_key.clone())?, &Py::Int(0i64))?; if py::lt(&t3890, &t3891)? { true } else { false } }) } } }).truthy() {
                    py::setitem(&v_dp.clone(), &v_key.clone(), Py::tuple(vec![v_cand.clone(), v_k.clone()]))?;
                }
            }
        }
    }
    v_best = py::minmax(&[{ let mut out = vec![]; for t3893 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t3893;
out.push(Py::tuple(vec![py::add(&py::getitem(&py::getitem(&v_dp.clone(), &Py::tuple(vec![py::sub(&v_full.clone(), &Py::Int(1i64))?, v_k.clone()]))?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_M.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?, &Py::Int(0i64))?)?, v_k.clone()])); } Py::list(out) }], false)?;
    let t3894 = (Py::tuple(vec![Py::list(vec![]), py::sub(&v_full.clone(), &Py::Int(1i64))?, py::getitem(&v_best.clone(), &Py::Int(1i64))?])).items()?;
    if t3894.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_tour = t3894[0].clone();
    v_mask = t3894[1].clone();
    v_k = t3894[2].clone();
    while (Py::Bool(true)).truthy() {
        let _ = py::method(&v_tour.clone(), "append", &[py::add(&v_k.clone(), &Py::Int(1i64))?])?;
        v_prev = py::getitem(&py::getitem(&v_dp.clone(), &Py::tuple(vec![v_mask.clone(), v_k.clone()]))?, &Py::Int(1i64))?;
        if ({ let t3895 = v_mask.clone(); Py::Bool({ let t3896 = py::lshift(&Py::Int(1i64), &v_k.clone())?; if py::eq(&t3895, &t3896) { true } else { false } }) }).truthy() {
            break;
        }
        let t3897 = (Py::tuple(vec![py::bitxor(&v_mask.clone(), &py::lshift(&Py::Int(1i64), &v_k.clone())?)?, v_prev.clone()])).items()?;
        if t3897.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_mask = t3897[0].clone();
        v_k = t3897[1].clone();
    }
    v_tour = py::add(&Py::list(vec![Py::Int(1i64)]), &{ let mut out = vec![]; for t3898 in py::iter(&(py::reversed(&v_tour.clone())?))? { let mut v_c: Py; 
v_c = t3898;
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
    let t1956 = (Py::tuple(vec![f__out_x(v_outputs.clone())?, f__out_p(v_outputs.clone())?])).items()?;
    if t1956.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t1956[0].clone();
    v_p = t1956[1].clone();
    if ({ let t1961 = { let t1957 = v_x.clone(); Py::Bool({ let t1958 = Py::None; if py::is(&t1957, &t1958) { true } else { false } }) }; if t1961.truthy() { t1961 } else { { let t1959 = v_p.clone(); Py::Bool({ let t1960 = Py::None; if py::is(&t1959, &t1960) { true } else { false } }) } } }).truthy() {
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
    let t2127 = (Py::tuple(vec![py::getitem(&v_inputs.clone(), &Py::str("c"))?, py::getitem(&v_inputs.clone(), &Py::str("A"))?, py::getitem(&v_inputs.clone(), &Py::str("b"))?])).items()?;
    if t2127.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_c = t2127[0].clone();
    v_A = t2127[1].clone();
    v_b = t2127[2].clone();
    v_rel = f__relations(py::method(&v_inputs.clone(), "get", &[Py::str("relations")])?, py::len(&v_A.clone())?)?;
    let t2129: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t2128 = (f__lp2_vertices(v_c.clone(), v_A.clone(), v_b.clone(), v_rel.clone())?).items()?;
            if t2128.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_pts = t2128[0].clone();
            v__ = t2128[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2129 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    v_vals = { let mut out = vec![]; for t2130 in py::iter(&(v_pts.clone()))? { let mut v_x: Py; let mut v_y: Py; 
let t2131 = (t2130).items()?;
if t2131.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2131[0].clone();
v_y = t2131[1].clone();
out.push(py::float(&py::add(&py::mul(&py::getitem(&v_c.clone(), &Py::Int(0i64))?, &v_x.clone())?, &py::mul(&py::getitem(&v_c.clone(), &Py::Int(1i64))?, &v_y.clone())?)?)?); } Py::list(out) };
    if (py::not(&v_vals.clone())).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_best = (if ({ let t2132 = py::getitem(&v_inputs.clone(), &Py::str("sense"))?; Py::Bool({ let t2133 = Py::str("maximize"); if py::eq(&t2132, &t2133) { true } else { false } }) }).truthy() { py::minmax(&[v_vals.clone()], true)? } else { py::minmax(&[v_vals.clone()], false)? });
    v_worst = f__rel(v_best.clone(), py::getitem(&v_outputs.clone(), &Py::str("optimal_value"))?)?;
    for t2134 in py::iter(&(py::method(&v_outputs.clone(), "get", &[Py::str("optimal_corners"), Py::list(vec![])])?))? {
        let t2137 = (t2134).items()?;
        if t2137.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_x = t2137[0].clone();
        v_y = t2137[1].clone();
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::add(&py::mul(&py::getitem(&v_c.clone(), &Py::Int(0i64))?, &v_x.clone())?, &py::mul(&py::getitem(&v_c.clone(), &Py::Int(1i64))?, &v_y.clone())?)?, py::getitem(&v_outputs.clone(), &Py::str("optimal_value"))?)?], true)?;
        for t2135 in py::iter(&(py::zip(&[v_A.clone(), v_b.clone(), v_rel.clone()])?))? {
            let t2136 = (t2135).items()?;
            if t2136.len() != 3 { return Err(PyErr::Value("unpack".into())); }
            v_row = t2136[0].clone();
            v_bi = t2136[1].clone();
            v_rl = t2136[2].clone();
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
fn f_verify_lp_sensitivity(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t1730 = (Py::tuple(vec![py::method(&v_outputs.clone(), "get", &[Py::str("x")])?, py::method(&v_outputs.clone(), "get", &[Py::str("dual_solution")])?])).items()?;
    if t1730.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t1730[0].clone();
    v_p = t1730[1].clone();
    if ({ let t1738 = { let t1731 = v_x.clone(); Py::Bool({ let t1732 = Py::None; if py::is(&t1731, &t1732) { true } else { false } }) }; if t1738.truthy() { t1738 } else { { let t1737 = { let t1733 = v_p.clone(); Py::Bool({ let t1734 = Py::None; if py::is(&t1733, &t1734) { true } else { false } }) }; if t1737.truthy() { t1737 } else { { let t1735 = Py::str("optimal_value"); Py::Bool({ let t1736 = v_outputs.clone(); if !py::contains(&t1736, &t1735)? { true } else { false } }) } } } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_worst = f__lp_certificate_residual(v_inputs.clone(), v_x.clone(), v_p.clone(), py::getitem(&v_outputs.clone(), &Py::str("optimal_value"))?)?;
    let t1766: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = f_lp_sensitivity(py::getitem(&v_inputs.clone(), &Py::str("sense"))?, py::getitem(&v_inputs.clone(), &Py::str("c"))?, py::getitem(&v_inputs.clone(), &Py::str("A"))?, py::getitem(&v_inputs.clone(), &Py::str("b"))?, py::method(&v_inputs.clone(), "get", &[Py::str("relations")])?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1766 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    for t1767 in py::iter(&(v_want.clone()))? {
        v_k = t1767;
        if ({ let t1788 = { let t1768 = v_k.clone(); Py::Bool({ let t1769 = v_outputs.clone(); if !py::contains(&t1769, &t1768)? { true } else { false } }) }; if t1788.truthy() { t1788 } else { py::not(&f__sens_same(py::getitem(&v_outputs.clone(), &v_k.clone())?, py::getitem(&v_want.clone(), &v_k.clone())?)?) } }).truthy() {
            return Ok(Py::Float(1.0));
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
    let t2733 = (f__simple_graph(v_edges.clone(), Py::Int(30i64))?).items()?;
    if t2733.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_nodes = t2733[0].clone();
    v_adj = t2733[1].clone();
    v_best = Py::list(vec![py::set(&v_nodes.clone())?]);
    let rec2734 = py::rec_cell();
    v_rec = { let v_adj = v_adj.clone(); let v_best = v_best.clone(); let v_edges = v_edges.clone(); let v_nodes = v_nodes.clone(); let rec2734 = rec2734.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_cover: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_rem: Py = a.get(1).cloned().unwrap_or(Py::None);
            let v_rec: Py = py::rec_get(&rec2734)?;
            let mut v_choice: Py = Py::None;
            let mut v_live: Py = Py::None;
            let mut v_r2: Py = Py::None;
            let mut v_take: Py = Py::None;
            let mut v_u: Py = Py::None;
            let mut v_v: Py = Py::None;
            let mut v_w: Py = Py::None;
            let mut v_x: Py = Py::None;
            if ({ let t2735 = py::len(&v_cover.clone())?; Py::Bool({ let t2736 = py::len(&py::getitem(&v_best.clone(), &Py::Int(0i64))?)?; if py::ge(&t2735, &t2736)? { true } else { false } }) }).truthy() {
                return Ok(Py::None);
            }
            v_live = { let mut out = vec![]; for t2737 in py::iter(&(v_rem.clone()))? { let mut v_v: Py; 
v_v = t2737;
if (py::getitem(&v_rem.clone(), &v_v.clone())?).truthy() { out.push((v_v.clone(), py::set(&{ let mut out = vec![]; for t2738 in py::iter(&(py::getitem(&v_rem.clone(), &v_v.clone())?))? { let mut v_u: Py; 
v_u = t2738;
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
            if ({ let t2739 = py::len(&py::getitem(&v_live.clone(), &v_v.clone())?)?; Py::Bool({ let t2740 = Py::Int(1i64); if py::eq(&t2739, &t2740) { true } else { false } }) }).truthy() {
                v_u = (match (py::getitem(&v_live.clone(), &v_v.clone())?).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) });
                v_take = Py::list(vec![v_u.clone()]);
            } else {
                v_take = Py::None;
            }
            for t2741 in py::iter(&((if (v_take.clone()).truthy() { Py::list(vec![v_take.clone()]) } else { Py::list(vec![Py::list(vec![v_v.clone()]), py::sorted_key(&py::getitem(&v_live.clone(), &v_v.clone())?, &{ let o = v_nodes.clone().clone(); py::func(move |a: &[Py]| -> R { py::method(&o, "index", a) }) }, (Py::Bool(false)).truthy())?]) })))? {
                v_choice = t2741;
                v_r2 = { let mut out = vec![]; for t2742 in py::iter(&(py::method(&v_live.clone(), "items", &[])?))? { let mut v_s: Py; let mut v_x: Py; 
let t2743 = (t2742).items()?;
if t2743.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2743[0].clone();
v_s = t2743[1].clone();
out.push((v_x.clone(), py::set(&v_s.clone())?)); } Py::dict(out) };
                for t2744 in py::iter(&(v_choice.clone()))? {
                    v_w = t2744;
                    for t2745 in py::iter(&(py::method(&v_r2.clone(), "pop", &[v_w.clone(), py::set(&Py::list(vec![]))?])?))? {
                        v_x = t2745;
                        let _ = py::method(&py::method(&v_r2.clone(), "get", &[v_x.clone(), py::set(&Py::list(vec![]))?])?, "discard", &[v_w.clone()])?;
                    }
                }
                let _ = py::call(&v_rec.clone(), &[py::bitor(&v_cover.clone(), &py::set(&v_choice.clone())?)?, v_r2.clone()])?;
            }
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec2734, &v_rec);
    let _ = py::call(&v_rec.clone(), &[py::set(&Py::list(vec![]))?, { let mut out = vec![]; for t2746 in py::iter(&(v_nodes.clone()))? { let mut v_v: Py; 
v_v = t2746;
out.push((v_v.clone(), py::set(&py::getitem(&v_adj.clone(), &v_v.clone())?)?)); } Py::dict(out) }])?;
    v_cov = { let mut out = vec![]; for t2747 in py::iter(&(v_nodes.clone()))? { let mut v_v: Py; 
v_v = t2747;
if ({ let t2748 = v_v.clone(); Py::Bool({ let t2749 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; if py::contains(&t2749, &t2748)? { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) };
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
    v_d = { let mut out = vec![]; for t3982 in py::iter(&(v_demand.clone()))? { let mut v_v: Py; 
v_v = t3982;
out.push(f__fr(v_v.clone())?); } Py::list(out) };
    v_T = py::len(&v_d.clone())?;
    if ({ let t3987 = { let t3983 = v_T.clone(); Py::Bool({ let t3984 = Py::Int(0i64); if py::eq(&t3983, &t3984) { true } else { false } }) }; if t3987.truthy() { t3987 } else { { let t3985 = v_T.clone(); Py::Bool({ let t3986 = Py::Int(52i64); if py::gt(&t3985, &t3986)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give 1 to 52 periods of demand"))));
    }
    if ({ let t3996 = Py::Bool(({ let mut out = vec![]; for t3988 in py::iter(&(v_d.clone()))? { let mut v_v: Py; 
v_v = t3988;
out.push({ let t3989 = v_v.clone(); Py::Bool({ let t3990 = Py::Int(0i64); if py::lt(&t3989, &t3990)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t3996.truthy() { t3996 } else { { let t3995 = { let t3991 = v_K.clone(); Py::Bool({ let t3992 = Py::Int(0i64); if py::lt(&t3991, &t3992)? { true } else { false } }) }; if t3995.truthy() { t3995 } else { { let t3993 = v_h.clone(); Py::Bool({ let t3994 = Py::Int(0i64); if py::lt(&t3993, &t3994)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("demands, the setup cost and the holding cost must be >= 0"))));
    }
    let t3997 = (Py::tuple(vec![f__fr(v_K.clone())?, f__fr(v_h.clone())?])).items()?;
    if t3997.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Kf = t3997[0].clone();
    v_hf = t3997[1].clone();
    let t3998 = (Py::tuple(vec![py::add(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &py::mul(&Py::list(vec![Py::None]), &v_T.clone())?)?, py::mul(&Py::list(vec![Py::None]), &py::add(&v_T.clone(), &Py::Int(1i64))?)?])).items()?;
    if t3998.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_best = t3998[0].clone();
    v_arg = t3998[1].clone();
    for t3999 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_T.clone(), &Py::Int(1i64))?).i()?]) {
        v_t = t3999;
        if ({ let t4004 = { let t4000 = py::getitem(&v_d.clone(), &py::sub(&v_t.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t4001 = Py::Int(0i64); if py::eq(&t4000, &t4001) { true } else { false } }) }; if !t4004.truthy() { t4004 } else { { let t4002 = py::getitem(&v_best.clone(), &py::sub(&v_t.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t4003 = Py::None; if !py::is(&t4002, &t4003) { true } else { false } }) } } }).truthy() {
            let t4005 = (Py::tuple(vec![py::getitem(&v_best.clone(), &py::sub(&v_t.clone(), &Py::Int(1i64))?)?, Py::None])).items()?;
            if t4005.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            py::setitem(&v_best.clone(), &v_t.clone(), t4005[0].clone())?;
            py::setitem(&v_arg.clone(), &v_t.clone(), t4005[1].clone())?;
        }
        for t4006 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_t.clone(), &Py::Int(1i64))?).i()?]) {
            v_s = t4006;
            v_cost = py::add(&py::add(&py::getitem(&v_best.clone(), &py::sub(&v_s.clone(), &Py::Int(1i64))?)?, &v_Kf.clone())?, &py::mul(&v_hf.clone(), &py::sum(&{ let mut out = vec![]; for t4007 in py::range_iter(&[(v_s.clone()).i()?, (py::add(&v_t.clone(), &Py::Int(1i64))?).i()?]) { let mut v_u: Py; 
v_u = t4007;
out.push(py::mul(&py::sub(&v_u.clone(), &v_s.clone())?, &py::getitem(&v_d.clone(), &py::sub(&v_u.clone(), &Py::Int(1i64))?)?)?); } Py::list(out) }, None)?)?)?;
            if ({ let t4012 = { let t4008 = py::getitem(&v_best.clone(), &v_t.clone())?; Py::Bool({ let t4009 = Py::None; if py::is(&t4008, &t4009) { true } else { false } }) }; if t4012.truthy() { t4012 } else { { let t4010 = v_cost.clone(); Py::Bool({ let t4011 = py::getitem(&v_best.clone(), &v_t.clone())?; if py::lt(&t4010, &t4011)? { true } else { false } }) } } }).truthy() {
                let t4013 = (Py::tuple(vec![v_cost.clone(), v_s.clone()])).items()?;
                if t4013.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                py::setitem(&v_best.clone(), &v_t.clone(), t4013[0].clone())?;
                py::setitem(&v_arg.clone(), &v_t.clone(), t4013[1].clone())?;
            }
        }
    }
    let t4014 = (Py::tuple(vec![py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &v_T.clone())?, v_T.clone()])).items()?;
    if t4014.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_q = t4014[0].clone();
    v_t = t4014[1].clone();
    while ({ let t4017 = v_t.clone(); Py::Bool({ let t4018 = Py::Int(0i64); if py::gt(&t4017, &t4018)? { true } else { false } }) }).truthy() {
        v_s = py::getitem(&v_arg.clone(), &v_t.clone())?;
        if ({ let t4015 = v_s.clone(); Py::Bool({ let t4016 = Py::None; if py::is(&t4015, &t4016) { true } else { false } }) }).truthy() {
            v_t = py::sub(&v_t, &Py::Int(1i64))?;
            continue;
        }
        py::setitem(&v_q.clone(), &py::sub(&v_s.clone(), &Py::Int(1i64))?, py::sum(&py::slice(&v_d.clone(), Some((py::sub(&v_s.clone(), &Py::Int(1i64))?).i()?), Some((v_t.clone()).i()?), None)?, None)?)?;
        v_t = py::sub(&v_s.clone(), &Py::Int(1i64))?;
    }
    return Ok(Py::dict(vec![(Py::str("min_cost"), py::float(&py::getitem(&v_best.clone(), &v_T.clone())?)?), (Py::str("order_quantities_example"), f__fl(v_q.clone())?), (Py::str("orders"), py::float(&py::sum(&{ let mut out = vec![]; for t4020 in py::iter(&(v_q.clone()))? { let mut v_v: Py; 
v_v = t4020;
if ({ let t4021 = v_v.clone(); Py::Bool({ let t4022 = Py::Int(0i64); if py::gt(&t4021, &t4022)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_weighted_product(mut v_X: Py, mut v_w: Py, mut v_types: Py) -> R {
    let mut v_N: Py = Py::None;
    let t4075 = (f__mcda_in(v_X.clone(), v_w.clone(), v_types.clone())?).items()?;
    if t4075.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_X = t4075[0].clone();
    v_w = t4075[1].clone();
    v_types = t4075[2].clone();
    v_N = f__linear_norm(v_X.clone(), v_types.clone())?;
    return Ok(f__mcda_out({ let mut out = vec![]; for t4088 in py::iter(&(v_N.clone()))? { let mut v_r: Py; 
v_r = t4088;
out.push(py::math1("exp", &py::fsum(&{ let mut out = vec![]; for t4089 in py::iter(&(py::zip(&[v_w.clone(), v_r.clone()])?))? { let mut v_v: Py; let mut v_wj: Py; 
let t4090 = (t4089).items()?;
if t4090.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_wj = t4090[0].clone();
v_v = t4090[1].clone();
out.push(py::mul(&v_wj.clone(), &py::math1("log", &v_v.clone())?)?); } Py::list(out) })?)?); } Py::list(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_weighted_sum(mut v_X: Py, mut v_w: Py, mut v_types: Py) -> R {
    let mut v_N: Py = Py::None;
    let t4138 = (f__mcda_in(v_X.clone(), v_w.clone(), v_types.clone())?).items()?;
    if t4138.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_X = t4138[0].clone();
    v_w = t4138[1].clone();
    v_types = t4138[2].clone();
    v_N = f__linear_norm(v_X.clone(), v_types.clone())?;
    return Ok(f__mcda_out({ let mut out = vec![]; for t4151 in py::iter(&(v_N.clone()))? { let mut v_r: Py; 
v_r = t4151;
out.push(py::fsum(&{ let mut out = vec![]; for t4152 in py::iter(&(py::zip(&[v_w.clone(), v_r.clone()])?))? { let mut v_v: Py; let mut v_wj: Py; 
let t4153 = (t4152).items()?;
if t4153.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_wj = t4153[0].clone();
v_v = t4153[1].clone();
out.push(py::mul(&v_wj.clone(), &v_v.clone())?); } Py::list(out) })?); } Py::list(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_wspt(mut v_p: Py, mut v_w: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_seq: Py = Py::None;
    let _ = f__jobs(v_p.clone(), Py::str("processing times"))?;
    if ({ let t4179 = { let t4174 = py::len(&v_w.clone())?; Py::Bool({ let t4175 = py::len(&v_p.clone())?; if py::ne(&t4174, &t4175) { true } else { false } }) }; if t4179.truthy() { t4179 } else { Py::Bool(({ let mut out = vec![]; for t4176 in py::iter(&(v_w.clone()))? { let mut v_v: Py; 
v_v = t4176;
out.push({ let t4177 = v_v.clone(); Py::Bool({ let t4178 = Py::Int(0i64); if py::le(&t4177, &t4178)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
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
    return Ok(Py::dict(vec![(Py::str("sequence"), { let mut out = vec![]; for t4182 in py::iter(&(v_seq.clone()))? { let mut v_j: Py; 
v_j = t4182;
out.push(py::add(&v_j.clone(), &Py::Int(1i64))?); } Py::list(out) }), (Py::str("weighted_completion_time"), py::float(&py::sum(&{ let mut out = vec![]; for t4183 in py::iter(&(v_seq.clone()))? { let mut v_j: Py; 
v_j = t4183;
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
    if ({ let t4211 = { let t4207 = py::len(&v_M.clone())?; Py::Bool({ let t4208 = Py::Int(2i64); if py::ne(&t4207, &t4208) { true } else { false } }) }; if t4211.truthy() { t4211 } else { { let t4209 = py::len(&py::getitem(&v_M.clone(), &Py::Int(0i64))?)?; Py::Bool({ let t4210 = Py::Int(2i64); if py::ne(&t4209, &t4210) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give a 2 x 2 payoff matrix (use the general zero-sum game capability otherwise)"))));
    }
    let t4212 = (v_M.clone()).items()?;
    if t4212.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    let t4213 = (t4212[0].clone()).items()?;
    if t4213.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t4213[0].clone();
    v_b = t4213[1].clone();
    let t4214 = (t4212[1].clone()).items()?;
    if t4214.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t4214[0].clone();
    v_d = t4214[1].clone();
    let t4215 = (Py::tuple(vec![py::minmax(&[py::minmax(&[v_a.clone(), v_b.clone()], false)?, py::minmax(&[v_c.clone(), v_d.clone()], false)?], true)?, py::minmax(&[py::minmax(&[v_a.clone(), v_c.clone()], true)?, py::minmax(&[v_b.clone(), v_d.clone()], true)?], false)?])).items()?;
    if t4215.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t4215[0].clone();
    v_hi = t4215[1].clone();
    if ({ let t4216 = v_lo.clone(); Py::Bool({ let t4217 = v_hi.clone(); if py::eq(&t4216, &t4217) { true } else { false } }) }).truthy() {
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
    let t4416 = (f__game_value(v_M.clone())?).items()?;
    if t4416.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_v = t4416[0].clone();
    v_x = t4416[1].clone();
    v_y = t4416[2].clone();
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
fn s_202(mut v_C: Py, mut v__m: Py) -> R {
    return Ok(f__assignment(v_C.clone(), v__m.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2159(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::minmax(&[f__rel(py::getitem(&v_o.clone(), &Py::str("lam_eff"))?, py::mul(&py::getitem(&v_i.clone(), &Py::str("lam"))?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("N"))?, &py::getitem(&v_o.clone(), &Py::str("L"))?)?)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("L"))?, py::mul(&py::getitem(&v_o.clone(), &Py::str("lam_eff"))?, &py::getitem(&v_o.clone(), &Py::str("W"))?)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("Lq"))?, py::mul(&py::getitem(&v_o.clone(), &Py::str("lam_eff"))?, &py::getitem(&v_o.clone(), &Py::str("Wq"))?)?)?, f__rel(py::sub(&py::getitem(&v_o.clone(), &Py::str("L"))?, &py::getitem(&v_o.clone(), &Py::str("Lq"))?)?, py::truediv(&py::getitem(&v_o.clone(), &Py::str("lam_eff"))?, &py::getitem(&v_i.clone(), &Py::str("mu"))?)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("working"))?, py::sub(&py::getitem(&v_i.clone(), &Py::str("N"))?, &py::getitem(&v_o.clone(), &Py::str("L"))?)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2402(mut v_i: Py, mut v_o: Py) -> R {
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
fn s_3001(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::minmax(&[f__rel(py::getitem(&v_o.clone(), &Py::str("expected_profit"))?, f__nv_disc_profit(py::getitem(&v_i.clone(), &Py::str("p"))?, py::getitem(&v_i.clone(), &Py::str("c"))?, py::method(&v_i.clone(), "get", &[Py::str("s"), Py::Float(0.0)])?, py::getitem(&v_i.clone(), &Py::str("demands"))?, py::getitem(&v_i.clone(), &Py::str("probs"))?, py::getitem(&v_o.clone(), &Py::str("Q"))?)?)?, py::truediv(&py::minmax(&[{ let mut out = vec![]; for t3004 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("demands"))?))? { let mut v_d: Py; 
v_d = t3004;
out.push(py::minmax(&[Py::Float(0.0), py::sub(&f__nv_disc_profit(py::getitem(&v_i.clone(), &Py::str("p"))?, py::getitem(&v_i.clone(), &Py::str("c"))?, py::method(&v_i.clone(), "get", &[Py::str("s"), Py::Float(0.0)])?, py::getitem(&v_i.clone(), &Py::str("demands"))?, py::getitem(&v_i.clone(), &Py::str("probs"))?, v_d.clone())?, &py::getitem(&v_o.clone(), &Py::str("expected_profit"))?)?], true)?); } Py::list(out) }], true)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_o.clone(), &Py::str("expected_profit"))?)?], true)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3013(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::minmax(&[py::abs(&py::sub(&py::normal_dist("cdf", &Py::Float(0.0), &Py::Float(1.0), &[py::truediv(&py::sub(&py::getitem(&v_o.clone(), &Py::str("Q"))?, &py::getitem(&v_i.clone(), &Py::str("mu"))?)?, &py::getitem(&v_i.clone(), &Py::str("sigma"))?)?])?, &py::truediv(&py::sub(&py::getitem(&v_i.clone(), &Py::str("p"))?, &py::getitem(&v_i.clone(), &Py::str("c"))?)?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("p"))?, &py::method(&v_i.clone(), "get", &[Py::str("s"), Py::Float(0.0)])?)?)?)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("expected_profit"))?, py::sub(&py::mul(&py::sub(&py::getitem(&v_i.clone(), &Py::str("p"))?, &py::method(&v_i.clone(), "get", &[Py::str("s"), Py::Float(0.0)])?)?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("mu"))?, &py::getitem(&v_o.clone(), &Py::str("expected_shortage"))?)?)?, &py::mul(&py::sub(&py::getitem(&v_i.clone(), &Py::str("c"))?, &py::method(&v_i.clone(), "get", &[Py::str("s"), Py::Float(0.0)])?)?, &py::getitem(&v_o.clone(), &Py::str("Q"))?)?)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3023(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::minmax(&[f__rel(py::truediv(&py::sub(&py::getitem(&v_o.clone(), &Py::str("Q"))?, &py::getitem(&v_i.clone(), &Py::str("lo"))?)?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("hi"))?, &py::getitem(&v_i.clone(), &Py::str("lo"))?)?)?, py::truediv(&py::sub(&py::getitem(&v_i.clone(), &Py::str("p"))?, &py::getitem(&v_i.clone(), &Py::str("c"))?)?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("p"))?, &py::method(&v_i.clone(), "get", &[Py::str("s"), Py::Float(0.0)])?)?)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("expected_profit"))?, py::sub(&py::mul(&py::sub(&py::getitem(&v_i.clone(), &Py::str("p"))?, &py::method(&v_i.clone(), "get", &[Py::str("s"), Py::Float(0.0)])?)?, &py::sub(&py::truediv(&py::add(&py::getitem(&v_i.clone(), &Py::str("lo"))?, &py::getitem(&v_i.clone(), &Py::str("hi"))?)?, &Py::Int(2i64))?, &py::truediv(&py::pow(&py::sub(&py::getitem(&v_i.clone(), &Py::str("hi"))?, &py::getitem(&v_o.clone(), &Py::str("Q"))?)?, &Py::Int(2i64))?, &py::mul(&Py::Int(2i64), &py::sub(&py::getitem(&v_i.clone(), &Py::str("hi"))?, &py::getitem(&v_i.clone(), &Py::str("lo"))?)?)?)?)?)?, &py::mul(&py::sub(&py::getitem(&v_i.clone(), &Py::str("c"))?, &py::method(&v_i.clone(), "get", &[Py::str("s"), Py::Float(0.0)])?)?, &py::getitem(&v_o.clone(), &Py::str("Q"))?)?)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3032(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::minmax(&[py::truediv(&py::abs(&py::add(&py::sub(&py::mul(&Py::Int(12i64), &py::pow(&py::getitem(&v_o.clone(), &Py::str("cut"))?, &Py::Int(2i64))?)?, &py::mul(&py::mul(&Py::Int(4i64), &py::add(&py::getitem(&v_i.clone(), &Py::str("L"))?, &py::getitem(&v_i.clone(), &Py::str("W"))?)?)?, &py::getitem(&v_o.clone(), &Py::str("cut"))?)?)?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("L"))?, &py::getitem(&v_i.clone(), &Py::str("W"))?)?)?)?, &py::minmax(&[Py::Float(1.0), py::mul(&py::getitem(&v_i.clone(), &Py::str("L"))?, &py::getitem(&v_i.clone(), &Py::str("W"))?)?], true)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("max_volume"))?, py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("cut"))?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("L"))?, &py::mul(&Py::Int(2i64), &py::getitem(&v_o.clone(), &Py::str("cut"))?)?)?)?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("W"))?, &py::mul(&Py::Int(2i64), &py::getitem(&v_o.clone(), &Py::str("cut"))?)?)?)?)?, (if ({ let t3033 = Py::Int(0i64); Py::Bool({ let t3034 = py::getitem(&v_o.clone(), &Py::str("cut"))?; if py::lt(&t3033, &t3034)? { { let t3035 = py::truediv(&py::minmax(&[py::getitem(&v_i.clone(), &Py::str("L"))?, py::getitem(&v_i.clone(), &Py::str("W"))?], false)?, &Py::Int(2i64))?; if py::lt(&t3034, &t3035)? { true } else { false } } } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) })], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3053(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::minmax(&[f__rel(py::getitem(&v_o.clone(), &Py::str("order_up_to_level"))?, py::add(&py::mul(&py::getitem(&v_i.clone(), &Py::str("d"))?, &py::add(&py::getitem(&v_i.clone(), &Py::str("R"))?, &py::getitem(&v_i.clone(), &Py::str("L"))?)?)?, &py::getitem(&v_o.clone(), &Py::str("safety_stock"))?)?)?, py::abs(&py::sub(&py::normal_dist("cdf", &Py::Float(0.0), &Py::Float(1.0), &[py::getitem(&v_o.clone(), &Py::str("z"))?])?, &py::getitem(&v_i.clone(), &Py::str("SL"))?)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("safety_stock"))?, py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("z"))?, &py::getitem(&v_i.clone(), &Py::str("sigma"))?)?, &py::math1("sqrt", &py::add(&py::getitem(&v_i.clone(), &Py::str("R"))?, &py::getitem(&v_i.clone(), &Py::str("L"))?)?)?)?)?], true)?);
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
fn s_3257(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::minmax(&[py::truediv(&py::abs(&py::sub(&py::sub(&py::getitem(&v_i.clone(), &Py::str("a"))?, &py::mul(&py::mul(&Py::Int(2i64), &py::getitem(&v_i.clone(), &Py::str("b"))?)?, &py::getitem(&v_o.clone(), &Py::str("quantity"))?)?)?, &py::getitem(&v_i.clone(), &Py::str("cv"))?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("a"))?)?], true)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("profit"))?, py::sub(&py::sub(&py::mul(&py::sub(&py::getitem(&v_i.clone(), &Py::str("a"))?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("b"))?, &py::getitem(&v_o.clone(), &Py::str("quantity"))?)?)?, &py::getitem(&v_o.clone(), &Py::str("quantity"))?)?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("cv"))?, &py::getitem(&v_o.clone(), &Py::str("quantity"))?)?)?, &py::method(&v_i.clone(), "get", &[Py::str("F_"), Py::Float(0.0)])?)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3315(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::minmax(&[py::truediv(&py::abs(&py::add(&py::mul(&py::mul(&Py::Int(2i64), &py::getitem(&v_i.clone(), &Py::str("a"))?)?, &py::getitem(&v_o.clone(), &Py::str("x_opt"))?)?, &py::getitem(&v_i.clone(), &Py::str("b"))?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("b"))?)?, py::abs(&py::mul(&py::mul(&Py::Int(2i64), &py::getitem(&v_i.clone(), &Py::str("a"))?)?, &py::getitem(&v_o.clone(), &Py::str("x_opt"))?)?)?], true)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("f_opt"))?, py::add(&py::add(&py::mul(&py::getitem(&v_i.clone(), &Py::str("a"))?, &py::pow(&py::getitem(&v_o.clone(), &Py::str("x_opt"))?, &Py::Int(2i64))?)?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("b"))?, &py::getitem(&v_o.clone(), &Py::str("x_opt"))?)?)?, &py::getitem(&v_i.clone(), &Py::str("c"))?)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3330(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::minmax(&[f__rel(py::getitem(&v_o.clone(), &Py::str("reorder_point"))?, py::add(&py::mul(&py::getitem(&v_i.clone(), &Py::str("d"))?, &py::getitem(&v_i.clone(), &Py::str("L"))?)?, &py::getitem(&v_o.clone(), &Py::str("safety_stock"))?)?)?, py::abs(&py::sub(&py::normal_dist("cdf", &Py::Float(0.0), &Py::Float(1.0), &[py::getitem(&v_o.clone(), &Py::str("z"))?])?, &py::getitem(&v_i.clone(), &Py::str("SL"))?)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("safety_stock"))?, py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("z"))?, &py::getitem(&v_i.clone(), &Py::str("sigma"))?)?, &py::math1("sqrt", &py::getitem(&v_i.clone(), &Py::str("L"))?)?)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3340(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::minmax(&[py::truediv(&py::abs(&py::sub(&py::getitem(&v_i.clone(), &Py::str("alpha"))?, &py::mul(&py::mul(&Py::Int(2i64), &py::getitem(&v_i.clone(), &Py::str("beta"))?)?, &py::getitem(&v_o.clone(), &Py::str("price"))?)?)?)?, &py::minmax(&[Py::Float(1.0), py::getitem(&v_i.clone(), &Py::str("alpha"))?], true)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("revenue"))?, py::mul(&py::getitem(&v_o.clone(), &Py::str("price"))?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("alpha"))?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("beta"))?, &py::getitem(&v_o.clone(), &Py::str("price"))?)?)?)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3373(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_pick: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok((if (Py::Bool(({ let mut out = vec![]; for t3374 in py::range_iter(&[(py::len(&py::getitem(&py::getitem(&v_i.clone(), &Py::str("sets"))?, &Py::Int(0i64))?)?).i()?]) { let mut v_j: Py; 
v_j = t3374;
out.push(py::not(&Py::Bool(({ let mut out = vec![]; for t3375 in py::iter(&(v_pick.clone()))? { let mut v_k: Py; 
v_k = t3375;
out.push(py::getitem(&py::getitem(&py::getitem(&v_i.clone(), &Py::str("sets"))?, &v_k.clone())?, &v_j.clone())?); } Py::list(out) }).items()?.iter().any(|x| x.truthy())))); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() { Py::Float(1.0) } else { py::minmax(&[f__rel(py::getitem(&v_o.clone(), &Py::str("min_cost"))?, py::sum(&{ let mut out = vec![]; for t3376 in py::iter(&(v_pick.clone()))? { let mut v_k: Py; 
v_k = t3376;
out.push(py::getitem(&py::getitem(&v_i.clone(), &Py::str("costs"))?, &v_k.clone())?); } Py::list(out) }, None)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("min_cost"))?, py::getitem(&f_set_cover(py::getitem(&v_i.clone(), &Py::str("sets"))?, py::getitem(&v_i.clone(), &Py::str("costs"))?)?, &Py::str("min_cost"))?)?], true)? }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[{ let mut out = vec![]; for t3377 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("sets_example"))?))? { let mut v_x: Py; 
v_x = t3377;
out.push(py::sub(&py::int(&py::round(&v_x.clone(), None)?)?, &Py::Int(1i64))?); } Py::list(out) }])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3481(mut v_i: Py, mut v_o: Py) -> R {
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
fn s_3614(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_ref: Py = Py::None;
    let t3629: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_ref = py::getitem(&f__topsis_check(py::getitem(&v_i.clone(), &Py::str("X"))?, py::getitem(&v_i.clone(), &Py::str("w"))?, py::getitem(&v_i.clone(), &Py::str("types"))?)?, &Py::str("scores"))?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3629 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3630 = py::len(&py::getitem(&v_o.clone(), &Py::str("scores"))?)?; Py::Bool({ let t3631 = py::len(&v_ref.clone())?; if py::ne(&t3630, &t3631) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::minmax(&[py::add(&{ let mut out = vec![]; for t3632 in py::iter(&(py::zip(&[py::getitem(&v_o.clone(), &Py::str("scores"))?, v_ref.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3633 = (t3632).items()?;
if t3633.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3633[0].clone();
v_b = t3633[1].clone();
out.push(f__rr(v_a.clone(), v_b.clone())?); } Py::list(out) }, &Py::list(vec![f__rr(py::getitem(&v_o.clone(), &Py::str("best_score"))?, py::minmax(&[py::getitem(&v_o.clone(), &Py::str("scores"))?], true)?)?, (if ({ let t3636 = py::int(&py::getitem(&v_o.clone(), &Py::str("best"))?)?; Py::Bool({ let t3637 = f__best(py::getitem(&v_o.clone(), &Py::str("scores"))?)?; if py::eq(&t3636, &t3637) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) })]))?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4091(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_ref: Py = Py::None;
    let t4096: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_ref = py::getitem(&s_4092(py::getitem(&v_i.clone(), &Py::str("X"))?, py::getitem(&v_i.clone(), &Py::str("w"))?, py::getitem(&v_i.clone(), &Py::str("types"))?)?, &Py::str("scores"))?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4096 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4097 = py::len(&py::getitem(&v_o.clone(), &Py::str("scores"))?)?; Py::Bool({ let t4098 = py::len(&v_ref.clone())?; if py::ne(&t4097, &t4098) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::minmax(&[py::add(&{ let mut out = vec![]; for t4099 in py::iter(&(py::zip(&[py::getitem(&v_o.clone(), &Py::str("scores"))?, v_ref.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t4100 = (t4099).items()?;
if t4100.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4100[0].clone();
v_b = t4100[1].clone();
out.push(f__rr(v_a.clone(), v_b.clone())?); } Py::list(out) }, &Py::list(vec![f__rr(py::getitem(&v_o.clone(), &Py::str("best_score"))?, py::minmax(&[py::getitem(&v_o.clone(), &Py::str("scores"))?], true)?)?, (if ({ let t4103 = py::int(&py::getitem(&v_o.clone(), &Py::str("best"))?)?; Py::Bool({ let t4104 = f__best(py::getitem(&v_o.clone(), &Py::str("scores"))?)?; if py::eq(&t4103, &t4104) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) })]))?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4092(mut v_X: Py, mut v_w: Py, mut v_t: Py) -> R {
    return Ok(Py::dict(vec![(Py::str("scores"), { let mut out = vec![]; for t4093 in py::iter(&({ let mut v: Vec<Py> = vec![]; v.extend((py::slice(&f__mcda_in(v_X.clone(), v_w.clone(), v_t.clone())?, Some((Py::Int(0i64)).i()?), Some((Py::Int(3i64)).i()?), Some((Py::Int(2i64)).i()?))?).items()?); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "X") { Some(x) => x, None => { return Err(PyErr::Type("_linear_norm() missing argument X".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "types") { Some(x) => x, None => { return Err(PyErr::Type("_linear_norm() missing argument types".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("_linear_norm() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("_linear_norm() got an unexpected keyword argument".into())) } f__linear_norm(a0, a1)? }))? { let mut v_r: Py; 
v_r = t4093;
out.push(py::prod(&{ let mut out = vec![]; for t4094 in py::iter(&(py::zip(&[py::getitem(&f__mcda_in(v_X.clone(), v_w.clone(), v_t.clone())?, &Py::Int(1i64))?, v_r.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t4095 = (t4094).items()?;
if t4095.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4095[0].clone();
v_b = t4095[1].clone();
out.push(py::pow(&v_b.clone(), &v_a.clone())?); } Py::list(out) })?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4154(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_ref: Py = Py::None;
    let t4159: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_ref = py::getitem(&s_4155(py::getitem(&v_i.clone(), &Py::str("X"))?, py::getitem(&v_i.clone(), &Py::str("w"))?, py::getitem(&v_i.clone(), &Py::str("types"))?)?, &Py::str("scores"))?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4159 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t4160 = py::len(&py::getitem(&v_o.clone(), &Py::str("scores"))?)?; Py::Bool({ let t4161 = py::len(&v_ref.clone())?; if py::ne(&t4160, &t4161) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::minmax(&[py::add(&{ let mut out = vec![]; for t4162 in py::iter(&(py::zip(&[py::getitem(&v_o.clone(), &Py::str("scores"))?, v_ref.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t4163 = (t4162).items()?;
if t4163.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4163[0].clone();
v_b = t4163[1].clone();
out.push(f__rr(v_a.clone(), v_b.clone())?); } Py::list(out) }, &Py::list(vec![f__rr(py::getitem(&v_o.clone(), &Py::str("best_score"))?, py::minmax(&[py::getitem(&v_o.clone(), &Py::str("scores"))?], true)?)?, (if ({ let t4166 = py::int(&py::getitem(&v_o.clone(), &Py::str("best"))?)?; Py::Bool({ let t4167 = f__best(py::getitem(&v_o.clone(), &Py::str("scores"))?)?; if py::eq(&t4166, &t4167) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) })]))?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4155(mut v_X: Py, mut v_w: Py, mut v_t: Py) -> R {
    return Ok(Py::dict(vec![(Py::str("scores"), { let mut out = vec![]; for t4156 in py::iter(&({ let mut v: Vec<Py> = vec![]; v.extend((py::slice(&f__mcda_in(v_X.clone(), v_w.clone(), v_t.clone())?, Some((Py::Int(0i64)).i()?), Some((Py::Int(3i64)).i()?), Some((Py::Int(2i64)).i()?))?).items()?); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "X") { Some(x) => x, None => { return Err(PyErr::Type("_linear_norm() missing argument X".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "types") { Some(x) => x, None => { return Err(PyErr::Type("_linear_norm() missing argument types".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("_linear_norm() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("_linear_norm() got an unexpected keyword argument".into())) } f__linear_norm(a0, a1)? }))? { let mut v_r: Py; 
v_r = t4156;
out.push(py::fsum(&{ let mut out = vec![]; for t4157 in py::iter(&(py::zip(&[py::getitem(&f__mcda_in(v_X.clone(), v_w.clone(), v_t.clone())?, &Py::Int(1i64))?, v_r.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t4158 = (t4157).items()?;
if t4158.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4158[0].clone();
v_b = t4158[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4184(mut v_i: Py, mut v_o: Py) -> R {
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
            return Ok(py::float(&py::sum(&{ let mut out = vec![]; for t4193 in py::iter(&(py::method(&f__completion(v_seq.clone(), py::getitem(&v_i.clone(), &Py::str("p"))?)?, "items", &[])?))? { let mut v_c: Py; let mut v_j: Py; 
let t4194 = (t4193).items()?;
if t4194.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_j = t4194[0].clone();
v_c = t4194[1].clone();
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
