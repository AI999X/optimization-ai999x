//! Every Optimization capability's figures (src/figures.rs first, then the generic engine's): its worked example gives
//! each published figure kind, a changed input changes the drawing, and no drawing holds a non-finite number or text
//! outside its canvas.

use ax_engine::figures::{figures, Figure};
use ax_engine::Input;
use optimization_ai999x::pack;
use serde_json::Value;

/// The published figure kinds of every capability (publish/website/models/optimization-ai999x/figures, except the
/// generic "curve"), as tools/figure_parity.py lists them.
const KINDS: &[(&str, &[&str])] = &[
    ("ahp_consistency_ratio", &["series"]),
    ("ahp_geometric_mean", &["series"]),
    ("ahp_priority_vector", &["series"]),
    ("all_pairs_shortest_paths", &["graph", "matrix"]),
    ("assignment_max_profit", &["matrix", "series"]),
    ("assignment_min_cost", &["matrix", "series"]),
    ("bellman_ford_shortest_path", &["bars", "graph"]),
    ("bin_packing", &["series"]),
    ("bipartite_matching", &["graph"]),
    ("critical_path_method", &["graph", "series"]),
    ("critical_point_two_variables", &["decision"]),
    ("edd_max_lateness", &["series"]),
    ("equality_constrained_qp", &["matrix", "vectors"]),
    ("erlang_b", &["field"]),
    ("facility_location", &["matrix"]),
    ("graph_coloring", &["graph", "series"]),
    ("integer_program", &["matrix", "vectors"]),
    ("johnson_two_machine", &["gantt", "series"]),
    ("knapsack_unbounded", &["series"]),
    ("linear_program", &["matrix", "series"]),
    ("lp_shadow_prices", &["matrix", "series"]),
    ("lp_two_variable_graphical", &["matrix", "region"]),
    ("max_flow", &["graph", "series"]),
    ("max_weight_matching", &["graph"]),
    ("mdp_discounted", &["matrix", "series"]),
    ("min_cost_flow", &["matrix", "series"]),
    ("minimum_spanning_tree", &["graph"]),
    ("minimum_vertex_cover", &["graph"]),
    ("mixed_nash_2x2", &["matrix"]),
    ("moore_hodgson_tardy_jobs", &["series"]),
    ("newsvendor_discrete", &["field"]),
    ("open_box_max_volume", &["field"]),
    ("parallel_machine_makespan", &["series"]),
    ("pert_project", &["graph", "series"]),
    ("pure_nash_equilibria", &["matrix"]),
    ("revenue_max_linear_demand", &["field"]),
    ("set_cover", &["matrix"]),
    ("shortest_path", &["graph", "graph-2"]),
    ("spt_mean_flow_time", &["series"]),
    ("steepest_descent_step", &["iteration", "matrix", "vectors"]),
    ("topsis", &["matrix", "series"]),
    ("transportation_problem", &["heatmap", "matrix", "series"]),
    ("traveling_salesman", &["matrix", "series"]),
    ("unconstrained_quadratic_min", &["matrix", "vectors"]),
    ("wagner_whitin_lot_sizing", &["series"]),
    ("weighted_product_model", &["matrix", "vectors"]),
    ("weighted_sum_model", &["matrix", "series"]),
    ("wspt_weighted_completion", &["series"]),
    ("zero_sum_2x2", &["matrix"]),
    ("zero_sum_game", &["matrix", "series"]),
];

/// A JSON list as the page writes it: [1, 2] -> "1, 2"; [[1, 2], [3, 4]] -> "1 2; 3 4".
fn page_text(v: &Value) -> String {
    let n = |x: &Value| x.as_f64().map(|f| format!("{f}")).unwrap_or_else(|| x.to_string());
    let s = |x: &Value| x.as_str().map(|t| t.to_string()).unwrap_or_else(|| n(x));
    match v.as_array() {
        // edges: [("S", "A", 4), ...] -> "S-A 4, ..."
        Some(a) if a.iter().all(|r| r.as_array().map(|r| r.len() >= 2 && r[0].is_string()).unwrap_or(false)) => {
            a.iter().map(|r| {
                let r = r.as_array().unwrap();
                let w = r.get(2).map(|w| format!(" {}", n(w))).unwrap_or_default();
                format!("{}-{}{w}", s(&r[0]), s(&r[1]))
            }).collect::<Vec<_>>().join(", ")
        }
        Some(a) if a.iter().all(|r| r.is_array()) => {
            a.iter().map(|r| r.as_array().unwrap().iter().map(n).collect::<Vec<_>>().join(" "))
                .collect::<Vec<_>>().join("; ")
        }
        Some(a) => a.iter().map(n).collect::<Vec<_>>().join(", "),
        None => n(v),
    }
}

/// The arguments of a structured example: solve('key', a=[[1, 2], [3, 4]], b='x', c=3).
fn example(key: &str) -> Vec<(String, Input)> {
    let p = pack();
    let ex = &p.procedure(key).expect(key).structured_example;
    let cs: Vec<char> = ex.chars().collect();
    let mut i = ex.find(',').map(|x| ex[..x].chars().count() + 1).unwrap_or(cs.len());
    let mut out = vec![];
    while i < cs.len() {
        while i < cs.len() && (cs[i] == ' ' || cs[i] == ',') {
            i += 1;
        }
        if i >= cs.len() || cs[i] == ')' {
            break;
        }
        let st = i;
        while cs[i] != '=' {
            i += 1;
        }
        let name: String = cs[st..i].iter().collect();
        i += 1;
        let value: String;
        if cs[i] == '\'' || cs[i] == '"' {
            let qch = cs[i];
            let s0 = i + 1;
            i += 1;
            while cs[i] != qch {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
            i += 1;
        } else {
            let s0 = i;
            let mut depth = 0i32;
            while i < cs.len() && !(depth == 0 && (cs[i] == ',' || cs[i] == ')')) {
                if cs[i] == '[' {
                    depth += 1;
                } else if cs[i] == ']' {
                    depth -= 1;
                }
                i += 1;
            }
            let raw: String = cs[s0..i].iter().collect();
            value = if raw.trim_start().starts_with('[') {
                let js = raw.trim().replace('(', "[").replace(')', "]").replace('\'', "\"");
                page_text(&serde_json::from_str::<Value>(&js).expect("a list"))
            } else {
                raw
            };
        }
        out.push((name.trim().to_string(), Input::Text(value.trim().to_string())));
    }
    out
}

fn check_texts(f: &Figure) {
    if let Err(e) = ax_engine::figkit::check_figure(f) {
        panic!("{e}");
    }
}

#[test]
fn every_capability_draws_its_published_kinds() {
    let p = pack();
    let mut missing = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let a = ax_engine::solve(p, key, &given, None);
        assert!(a.solved(), "{key}: the worked example is not solved");
        let figs = figures(p, key, &given, None, None);
        for kind in *kinds {
            match figs.iter().find(|f| f.kind == *kind) {
                Some(f) => {
                    assert!(!f.svg.contains("NaN") && !f.svg.contains("inf"), "{key} {kind}: a non-finite number");
                    // the model's own figures (the generic engine's are checked in ax-engine)
                    if !f.slug.contains("-live-") {
                        assert_eq!(f.slug, format!("{}-{kind}", key.replace('_', "-")));
                        check_texts(f);
                    }
                }
                None => missing.push(format!("{key}: {kind}")),
            }
        }
        if let Some(i) = figs.iter().position(|f| f.slug.contains("-live-")) {
            assert!(figs[i..].iter().all(|f| f.slug.contains("-live-")), "{key}: generic figures before own ones");
        }
    }
    assert!(missing.is_empty(), "missing figures: {missing:?}");
}

/// Numbers in a text multiplied by f ("12" -> "16.44"; "3, 6" -> "4.11, 8.22").
fn scale_text(t: &str, f: f64) -> Option<String> {
    let mut out = String::new();
    let mut cur = String::new();
    let mut changed = false;
    let flush = |cur: &mut String, out: &mut String, changed: &mut bool| {
        if !cur.is_empty() {
            match cur.parse::<f64>() {
                Ok(x) if x != 0.0 => {
                    out.push_str(&format!("{}", x * f));
                    *changed = true;
                }
                _ => out.push_str(cur),
            }
            cur.clear();
        }
    };
    for c in t.chars() {
        if c.is_ascii_digit() || c == '.' || (c == '-' && cur.is_empty()) || ((c == 'e' || c == 'E') && !cur.is_empty()) {
            cur.push(c);
        } else {
            flush(&mut cur, &mut out, &mut changed);
            out.push(c);
        }
    }
    flush(&mut cur, &mut out, &mut changed);
    changed.then_some(out)
}

/// The capability's own figure kinds (drawn by src/figures.rs).
fn own_kinds(key: &str) -> Vec<String> {
    let p = pack();
    figures(p, key, &example(key), None, None).into_iter().filter(|f| !f.slug.contains("-live-")).map(|f| f.kind)
        .collect()
}

#[test]
fn a_changed_input_changes_the_drawing() {
    let p = pack();
    let mut unchanged = vec![];
    for (key, _) in KINDS {
        let kinds = own_kinds(key);
        if kinds.is_empty() {
            continue;
        }
        let given = example(key);
        let base = figures(p, key, &given, None, None);
        let mut tries: Vec<Vec<(String, Input)>> = vec![];
        for f in [1.37, 0.73, 2.0] {
            for i in 0..given.len() {
                if let Input::Text(t) = &given[i].1 {
                    if let Some(t2) = scale_text(t, f) {
                        let mut g = given.clone();
                        g[i].1 = Input::Text(t2);
                        tries.push(g);
                    }
                }
            }
        }
        let mut ok = false;
        for g in tries {
            if !ax_engine::solve(p, key, &g, None).solved() {
                continue;
            }
            let figs = figures(p, key, &g, None, None);
            let differs = kinds.iter().all(|kind| {
                match (base.iter().find(|f| f.kind == *kind), figs.iter().find(|f| f.kind == *kind)) {
                    (Some(a), Some(b)) => a.svg != b.svg,
                    _ => false,
                }
            });
            if differs {
                ok = true;
                for f in figs.iter().filter(|f| !f.slug.contains("-live-")) {
                    check_texts(f);
                }
                break;
            }
        }
        if !ok {
            unchanged.push(*key);
        }
    }
    assert!(unchanged.is_empty(), "drawing did not follow a changed input: {unchanged:?}");
}

#[test]
fn a_refused_input_draws_nothing() {
    // an infeasible programme is refused by the model: no figure at all, never a wrong one
    let p = pack();
    let given: Vec<(String, Input)> = [("sense", "maximize"), ("c", "3, 5"), ("A", "1 0; -1 0"), ("b", "1, -2")]
        .iter().map(|(k, v)| (k.to_string(), Input::Text(v.to_string()))).collect();
    let a = ax_engine::solve(p, "lp_two_variable_graphical", &given, None);
    let figs = figures(p, "lp_two_variable_graphical", &given, None, None);
    if !a.solved() {
        assert!(figs.is_empty());
    }
    let given: Vec<(String, Input)> = [("A", "-3"), ("c", "5")].iter()
        .map(|(k, v)| (k.to_string(), Input::Text(v.to_string()))).collect();
    if !ax_engine::solve(p, "erlang_b", &given, None).solved() {
        assert!(figures(p, "erlang_b", &given, None, None).is_empty());
    }
}

#[test]
fn the_schedule_matches_the_makespan() {
    // the last job leaves machine 2 at the makespan the model returns
    let p = pack();
    let given = example("johnson_two_machine");
    let figs = figures(p, "johnson_two_machine", &given, None, None);
    let g = figs.iter().find(|f| f.kind == "gantt").expect("a schedule");
    assert!(g.caption.contains("makespan 22"), "{}", g.caption);
}
