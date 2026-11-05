//! Optimization's own live figures: the matrices of a problem with what the model returned beside them, networks
//! with the computed path, a two-machine schedule, the feasible region of a two-variable programme, steepest-descent
//! iterates and value fields over two inputs. Each is drawn for the visitor's inputs from the model's answer and from
//! further solves of the model itself (ax_engine::figkit::Solved); a figure that cannot be drawn faithfully for the
//! inputs is left out.

use ax_engine::figkit::{self as k, num, Canvas, Plot, Solved, P};
use ax_engine::figures::Figure;
use ax_engine::{Answer, Input, Val};
use serde_json::{json, Value};

pub fn live(s: &Solved) -> Vec<Figure> {
    let v: Vec<Option<Figure>> = match s.key {
        "assignment_max_profit" | "assignment_min_cost" | "equality_constrained_qp" | "facility_location"
        | "integer_program" | "linear_program" | "lp_shadow_prices" | "mdp_discounted" | "mixed_nash_2x2"
        | "set_cover" | "topsis" | "traveling_salesman" | "unconstrained_quadratic_min" | "weighted_product_model"
        | "weighted_sum_model" | "zero_sum_2x2" | "zero_sum_game" => {
            let mut v = vec![matrices(s)];
            if matches!(s.key, "equality_constrained_qp" | "integer_program" | "unconstrained_quadratic_min"
                               | "weighted_product_model") {
                v.push(vectors(s));
            }
            v
        }
        "steepest_descent_step" => vec![iteration(s), matrices(s), vectors(s)],
        "transportation_problem" => vec![transport_heat(s)],
        "erlang_b" => vec![field2(s, "A", Rng::Decade, "c", Rng::Count(13), "blocking_probability",
                                  "Erlang B blocking probability over offered traffic and number of servers")],
        "newsvendor_discrete" => vec![field2(s, "p", Rng::Near, "c", Rng::Near, "expected_profit",
                                             "Maximum expected profit over selling price and unit cost")],
        "open_box_max_volume" => vec![field2(s, "L", Rng::Decade, "W", Rng::Decade, "max_volume",
                                             "Maximum box volume over sheet length and sheet width")],
        "revenue_max_linear_demand" => vec![field2(s, "alpha", Rng::Near, "beta", Rng::Near, "price",
                                                   "Revenue-maximising price over the two demand parameters")],
        "johnson_two_machine" => vec![gantt(s)],
        "lp_two_variable_graphical" => vec![region(s)],
        "shortest_path" => vec![path_network(s)],
        _ => vec![],
    };
    v.into_iter().flatten().collect()
}

// ------------------------------------------------------------------------------------------------ helpers
fn ints(v: &Value) -> Option<Vec<usize>> {
    v.as_array()?.iter().map(|x| x.as_f64().filter(|f| *f >= 1.0 && f.fract() == 0.0).map(|f| f as usize)).collect()
}

fn col(v: &[f64]) -> Vec<Vec<f64>> {
    v.iter().map(|x| vec![*x]).collect()
}

fn list_text(v: &[f64]) -> String {
    v.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", ")
}

fn mat_text(m: &[Vec<f64>]) -> String {
    m.iter().map(|r| list_text(r)).collect::<Vec<_>>().join("; ")
}

/// One panel of a matrix figure: its name, its entries, the cells outlined, returned by the model or given.
struct Panel {
    name: String,
    m: Vec<Vec<f64>>,
    marks: Vec<(usize, usize)>,
    returned: bool,
}

fn panel(name: &str, m: Vec<Vec<f64>>, returned: bool) -> Panel {
    Panel { name: name.to_string(), m, marks: vec![], returned }
}

/// Matrices side by side as heat maps, each shaded by its own largest magnitude (negative values in the second
/// colour), rows and columns numbered, the entries written in the cells, outlined cells marked.
fn draw_panels(ps: &[Panel], header: &str) -> Option<(String, f64)> {
    let ps: Vec<&Panel> = ps.iter().filter(|p| !p.m.is_empty() && !p.m[0].is_empty()).collect();
    if ps.is_empty() {
        return None;
    }
    let rows = ps.iter().map(|p| p.m.len()).max()?;
    let cols: usize = ps.iter().map(|p| p.m.iter().map(|r| r.len()).max().unwrap_or(0)).sum();
    if rows > 16 || cols > 24 || ps.iter().any(|p| p.m.iter().flatten().any(|x| !x.is_finite())) {
        return None;
    }
    let gap = 26.0;
    let avail_w = 520.0 - 44.0 - gap * (ps.len() - 1) as f64;
    let top = 76.0;
    let cell = (avail_w / cols as f64).min(250.0 / rows as f64).min(62.0);
    let total = cell * cols as f64 + gap * (ps.len() - 1) as f64;
    let mut x = 34.0 + (520.0 - 34.0 - 10.0 - total) / 2.0;
    let mut c = Canvas::new(top + cell * rows as f64 + 24.0);
    if !header.is_empty() {
        c.raw(&format!("<text class=\"axp-mt\" x=\"16\" y=\"20\">{}</text>", esc(header)));
    }
    let font = (cell * 0.3).clamp(8.0, 13.0);
    // titles staggered on two lines when one would not fit above its panel
    let widths: Vec<f64> = ps.iter().map(|p| cell * p.m.iter().map(|r| r.len()).max().unwrap_or(0) as f64).collect();
    let stagger = ps.iter().zip(&widths).any(|(p, w)| k::text_w(&title_of(p), false) > w + gap - 4.0);
    for (pi, p) in ps.iter().enumerate() {
        let nc = p.m.iter().map(|r| r.len()).max().unwrap_or(0);
        let w = widths[pi];
        let ty = if stagger && pi % 2 == 1 { 54.0 } else { 38.0 };
        c.text(x + w / 2.0, ty, "middle", if p.returned { "axp-vt" } else { "axp-bt" }, &title_of(p));
        for j in 0..nc {
            c.name(x + cell * (j as f64 + 0.5), top - 6.0, "middle", &(j + 1).to_string());
        }
        if pi == 0 {
            for i in 0..p.m.len() {
                c.name(x - 6.0, top + cell * (i as f64 + 0.5) + 4.0, "end", &(i + 1).to_string());
            }
        }
        let mx = p.m.iter().flatten().fold(0.0f64, |a, v| a.max(v.abs()));
        for (i, r) in p.m.iter().enumerate() {
            for (j, v) in r.iter().enumerate() {
                let (cx, cy) = (x + cell * j as f64, top + cell * i as f64);
                let op = if mx > 0.0 { 0.08 + 0.47 * v.abs() / mx } else { 0.08 };
                c.raw(&format!("<rect class=\"{}\" x=\"{cx:.1}\" y=\"{cy:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
                                fill-opacity=\"{op:.2}\"/>", if *v < 0.0 { "axp-hn" } else { "axp-hp" }, cell - 1.5,
                               cell - 1.5));
                if p.marks.contains(&(i, j)) {
                    c.raw(&format!("<rect class=\"axp-c2\" style=\"stroke-width:3\" x=\"{:.1}\" y=\"{:.1}\" \
                                    width=\"{:.1}\" height=\"{:.1}\"/>", cx + 1.0, cy + 1.0, cell - 3.5, cell - 3.5));
                }
                let t = k::snap(*v, mx);
                // as many digits as fit in the cell (4 to 2)
                let fits = |x: &String| k::text_w(x, true) * font / 13.0 <= cell - 2.0;
                let t = [4, 3, 2].iter().map(|d| ax_engine::num::py_g(t, *d)).find(fits)
                    .unwrap_or_else(|| ax_engine::num::py_g(t, 2));
                if fits(&t) {
                    c.raw(&format!("<text class=\"axp-ht\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" \
                                    font-size=\"{font:.0}\">{}</text>", cx + cell / 2.0, cy + cell / 2.0 + font * 0.35,
                                   esc(&t)));
                }
            }
        }
        x += w + gap;
    }
    let h = c.h;
    Some((c.s, h))
}

fn title_of(p: &Panel) -> String {
    format!("{} ({})", p.name, if p.returned { "returned" } else { "input" })
}

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

// ------------------------------------------------------------------------------------------------ matrices
fn matrices(s: &Solved) -> Option<Figure> {
    let mut ps: Vec<Panel> = vec![];
    let header: String;
    let mut what = String::new();
    match s.key {
        "assignment_max_profit" | "assignment_min_cost" => {
            let cm = s.mat("C")?;
            let mut p = panel("C", cm, false);
            let total = if s.key == "assignment_min_cost" { ("min_cost", s.out("min_cost")?) } else {
                ("max_value", s.out("max_value")?)
            };
            header = format!("{} = {}", total.0, num(total.1));
            if let Some(a) = s.outj("assignment").and_then(ints) {
                if a.len() != p.m.len() || a.iter().any(|j| *j > p.m[0].len()) {
                    return None;
                }
                p.marks = a.iter().enumerate().map(|(i, j)| (i, j - 1)).collect();
                what = format!(" Outlined: the cell of each row's assigned column (assignment {}).",
                               a.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "));
                ps.push(p);
                ps.push(panel("assignment", col(&a.iter().map(|x| *x as f64).collect::<Vec<_>>()), true));
            } else {
                what = " The optimal assignment is not unique, so no cell is outlined.".into();
                ps.push(p);
            }
        }
        "equality_constrained_qp" => {
            ps.push(panel("Q", s.mat("Q")?, false));
            ps.push(panel("A", s.mat("A")?, false));
            ps.push(panel("x_star", col(&s.out_list("x_star")?), true));
            ps.push(panel("multipliers", col(&s.out_list("multipliers")?), true));
            header = format!("f_min = {}", num(s.out("f_min")?));
        }
        "facility_location" => {
            let fixed = s.vec("fixed")?;
            let mut pf = panel("fixed", col(&fixed), false);
            let mut pc = panel("cost", s.mat("cost")?, false);
            if let Some(o) = s.outj("open_example").and_then(ints) {
                if o.iter().any(|i| *i > fixed.len()) {
                    return None;
                }
                pf.marks = o.iter().map(|i| (i - 1, 0)).collect();
                pc.marks = o.iter().flat_map(|i| (0..pc.m[i - 1].len()).map(move |j| (i - 1, j))).collect();
                what = format!(" Outlined: the facilities the model opens ({}).",
                               o.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "));
            }
            ps.push(pf);
            ps.push(pc);
            header = format!("min_cost = {}", num(s.out("min_cost")?));
        }
        "integer_program" | "linear_program" | "lp_shadow_prices" => {
            ps.push(panel("A", s.mat("A")?, false));
            ps.push(panel("b", col(&s.vec("b")?), false));
            if s.key == "integer_program" {
                ps.push(panel("x", col(&s.out_list("x_one_optimal")?), true));
            } else {
                if let Some(x) = s.out_list("x") {
                    ps.push(panel("x", col(&x), true));
                }
                ps.push(panel("shadow prices", col(&s.out_list("shadow_prices")?), true));
            }
            header = format!("optimal value = {}", num(s.out("optimal_value")?));
        }
        "mdp_discounted" => {
            ps.push(panel("R", s.mat("R")?, false));
            ps.push(panel("T", s.mat("T")?, false));
            ps.push(panel("values", col(&s.out_list("values")?), true));
            if let Some(p) = s.out_list("policy") {
                ps.push(panel("policy", col(&p), true));
            }
            header = format!("value of state 1 = {}", num(s.out("value_state_1")?));
        }
        "mixed_nash_2x2" => {
            ps.push(panel("A", s.mat("A")?, false));
            ps.push(panel("B", s.mat("B")?, false));
            header = format!("p_row1 = {}, q_col1 = {}", num(s.out("p_row1")?), num(s.out("q_col1")?));
            what = format!(" Computed: payoffs {} (row player) and {} (column player).", num(s.out("row_payoff")?),
                           num(s.out("col_payoff")?));
        }
        "set_cover" => {
            let mut p = panel("sets", s.mat("sets")?, false);
            if let Some(o) = s.outj("sets_example").and_then(ints) {
                if o.iter().any(|i| *i > p.m.len()) {
                    return None;
                }
                p.marks = o.iter().flat_map(|i| p.m[i - 1].iter().enumerate().filter(|x| *x.1 != 0.0)
                    .map(move |(j, _)| (i - 1, j))).collect();
                what = format!(" Outlined: the elements covered by the chosen sets ({}).",
                               o.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "));
            }
            ps.push(p);
            ps.push(panel("costs", col(&s.vec("costs")?), false));
            header = format!("min_cost = {}", num(s.out("min_cost")?));
        }
        "topsis" | "weighted_product_model" | "weighted_sum_model" => {
            let mut p = panel("X", s.mat("X")?, false);
            let best = s.out("best")? as usize;
            if best >= 1 && best <= p.m.len() {
                p.marks = (0..p.m[best - 1].len()).map(|j| (best - 1, j)).collect();
            }
            ps.push(p);
            ps.push(panel("w", vec![s.vec("w")?], false));
            ps.push(panel("scores", col(&s.out_list("scores")?), true));
            header = format!("best = {}, score {}", best, num(s.out("best_score")?));
            what = format!(" Outlined: the best alternative, row {best}.");
        }
        "traveling_salesman" => {
            let mut p = panel("D", s.mat("D")?, false);
            if let Some(t) = s.outj("tour_example").and_then(ints) {
                if t.iter().any(|i| *i > p.m.len()) {
                    return None;
                }
                p.marks = t.windows(2).map(|w| (w[0] - 1, w[1] - 1)).collect();
                what = format!(" Outlined: the legs of the tour {}.",
                               t.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" → "));
            }
            ps.push(p);
            header = format!("shortest tour = {}", num(s.out("min_tour_length")?));
        }
        "unconstrained_quadratic_min" => {
            ps.push(panel("Q", s.mat("Q")?, false));
            ps.push(panel("c", col(&s.vec("c")?), false));
            ps.push(panel("x_star", col(&s.out_list("x_star")?), true));
            header = format!("f_min = {}", num(s.out("f_min")?));
        }
        "steepest_descent_step" => {
            ps.push(panel("Q", s.mat("Q")?, false));
            ps.push(panel("x0", col(&s.vec("x0")?), false));
            ps.push(panel("gradient", col(&s.out_list("gradient")?), true));
            ps.push(panel("x1", col(&s.out_list("x1")?), true));
            header = format!("step {} , f(x1) = {}", num(s.out("alpha")?), num(s.out("f1")?)).replace(" ,", ",");
        }
        "zero_sum_2x2" => {
            ps.push(panel("A", s.mat("A")?, false));
            header = format!("value = {}", num(s.out("value")?));
            if let (Some(p), Some(q)) = (s.out("p_row1"), s.out("q_col1")) {
                ps.push(panel("row strategy", col(&[p, 1.0 - p]), true));
                ps.push(panel("column strategy", vec![vec![q, 1.0 - q]], true));
            } else {
                what = format!(" A pure saddle point exists (saddle point: {}).", s.out_text("saddle_point")?);
            }
        }
        "zero_sum_game" => {
            ps.push(panel("A", s.mat("A")?, false));
            ps.push(panel("row strategy", col(&s.out_list("row_strategy_example")?), true));
            ps.push(panel("column strategy", vec![s.out_list("col_strategy_example")?], true));
            header = format!("value = {}", num(s.out("value")?));
        }
        _ => return None,
    }
    let (body, h) = draw_panels(&ps, &header)?;
    let list = ps.iter().map(|p| format!("{} = [{}]", title_of(p), mat_text(&p.m))).collect::<Vec<_>>().join("; ");
    Some(s.figure("matrix", "The problem's matrices and the computed answer",
                  &format!("The matrices as heat maps, rows and columns numbered, each panel shaded by its own largest \
                            magnitude (negative values in the second colour): {list}. Computed: {header}.{what}"),
                  &body, h))
}

fn transport_heat(s: &Solved) -> Option<Figure> {
    let cost = s.mat("cost")?;
    let sh: Vec<Vec<f64>> = s.outj("shipments")?.as_array()?.iter()
        .map(|r| r.as_array().map(|r| r.iter().filter_map(|x| x.as_f64()).collect()))
        .collect::<Option<Vec<Vec<f64>>>>()?;
    if sh.len() != cost.len() || sh.iter().zip(&cost).any(|(a, b)| a.len() != b.len()) {
        return None;
    }
    let mut pc = panel("unit cost", cost.clone(), false);
    pc.marks = sh.iter().enumerate().flat_map(|(i, r)| r.iter().enumerate().filter(|x| *x.1 > 0.0)
        .map(move |(j, _)| (i, j))).collect();
    let ps = vec![pc, panel("shipments", sh.clone(), true)];
    let mc = s.out("min_cost")?;
    let (body, h) = draw_panels(&ps, &format!("minimum total cost = {}", num(mc)))?;
    Some(s.figure("heatmap", "An optimal transportation plan",
                  &format!("Rows: sources, columns: destinations. Left the unit costs as given, right the quantities the \
                            model ships ({}); outlined on the left: the routes used. Supplies [{}], demands [{}]; \
                            minimum total cost {}.", mat_text(&sh), list_text(&s.vec("supply")?),
                           list_text(&s.vec("demand")?), num(mc)), &body, h))
}

// ------------------------------------------------------------------------------------------------ vectors
fn vectors(s: &Solved) -> Option<Figure> {
    let two = |v: Option<Vec<f64>>| v.filter(|v| v.len() == 2).map(|v| (v[0], v[1]));
    let mut v: Vec<(String, P, bool)> = vec![];
    let ins: &[&str] = match s.key {
        "equality_constrained_qp" | "unconstrained_quadratic_min" => &["c"],
        "integer_program" => &["c", "b"],
        "steepest_descent_step" => &["x0"],
        "weighted_product_model" => &["w"],
        _ => &[],
    };
    for n in ins {
        if let Some(p) = two(s.vec(n)) {
            v.push((n.to_string(), p, false));
        }
    }
    let outs: &[&str] = match s.key {
        "equality_constrained_qp" | "unconstrained_quadratic_min" => &["x_star"],
        "integer_program" => &["x_one_optimal"],
        "steepest_descent_step" => &["x1", "gradient"],
        "weighted_product_model" => &["scores"],
        _ => &[],
    };
    let mut got = vec![];
    for n in outs {
        let p = two(s.out_list(n))?;
        got.push(format!("{n} = ({}, {})", num(p.0), num(p.1)));
        v.push((n.to_string(), p, true));
    }
    let body = k::arrows(&v, &got.join(", "));
    let given: Vec<String> = v.iter().filter(|x| !x.2).map(|x| format!("{} = ({}, {})", x.0, num(x.1 .0), num(x.1 .1)))
        .collect();
    Some(s.chart("vectors", "The plane vectors of the problem and of the answer",
                 &format!("Arrows from the origin: as given {}; computed by the model {} (coloured).",
                          if given.is_empty() { "none".to_string() } else { given.join(", ") }, got.join(", ")),
                 &body))
}

// ------------------------------------------------------------------------------------------------ steepest descent
fn iteration(s: &Solved) -> Option<Figure> {
    let x0 = s.vec("x0")?;
    if x0.len() != 2 {
        return None;
    }
    let mut pts: Vec<P> = vec![(x0[0], x0[1])];
    let mut fs = vec![];
    let mut cur: Option<Answer> = None;
    for _ in 0..6 {
        let a = match &cur {
            None => s.with_input(&[])?,
            Some(prev) => {
                let x = k::list_of(prev, "x1")?;
                match s.with_input(&[("x0", Input::Json(json!(x)))]) {
                    Some(a) => a,
                    None => break,           // at the minimum (zero gradient) the step is refused: stop there
                }
            }
        };
        let x1 = k::list_of(&a, "x1")?;
        if x1.len() != 2 {
            return None;
        }
        fs.push(k::num_of(&a, "f1")?);
        pts.push((x1[0], x1[1]));
        cur = Some(a);
    }
    if pts.len() < 2 {
        return None;
    }
    // the minimiser, from the model's own unconstrained minimisation of the same quadratic
    let star = s.solve_other("unconstrained_quadratic_min", &[("Q", Input::Json(json!(s.mat("Q")?))),
                                                              ("c", Input::Json(json!(s.vec("c")?)))])
        .and_then(|a| k::list_of(&a, "x_star")).filter(|v| v.len() == 2).map(|v| (v[0], v[1]));
    let mut xs: Vec<f64> = pts.iter().map(|p| p.0).collect();
    let mut ys: Vec<f64> = pts.iter().map(|p| p.1).collect();
    if let Some(st) = star {
        xs.push(st.0);
        ys.push(st.1);
    }
    let (xl, xh) = mm(&xs);
    let (yl, yh) = mm(&ys);
    let last = *pts.last()?;
    let n = pts.len() - 1;
    let mut p = Plot::new((xl, xh), (yl, yh), "x₁ (first coordinate)", "x₂ (second coordinate)",
                          &format!("x{} = ({}, {}), f = {}", sub(n), num(last.0), num(last.1), num(*fs.last()?)), (false, false), true);
    p.path(&pts, "axp-c");
    for (i, q) in pts.iter().enumerate() {
        p.dot(*q, 4.0, if i == 0 { "axp-d4" } else { "axp-d" });
        if i < 3 {
            p.label(*q, 8.0, if i % 2 == 0 { -8.0 } else { 16.0 }, "start", "axp-lt",
                    &format!("x{}", ['₀', '₁', '₂'][i]));
        }
    }
    if let Some(st) = star {
        p.dot(st, 6.0, "axp-d2");
    }
    p.mark(last, "");
    p.legend(&[("iterates".into(), "axp-b"), ("minimum".into(), "axp-b2")]);
    Some(s.chart("iteration", "Steepest descent steps towards the minimum",
                 &format!("{n} steps of steepest descent with exact line search from x₀ = ({}, {}), each step one \
                           computation of the model starting from the previous point; f after each step: {}.{}",
                          num(x0[0]), num(x0[1]), list_text(&fs),
                          star.map(|st| format!(" Green: the minimiser the model computes, ({}, {}).", num(st.0),
                                                num(st.1))).unwrap_or_default()), &p.body()))
}

fn sub(n: usize) -> String {
    const D: [char; 10] = ['₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉'];
    n.to_string().chars().map(|c| D[c as usize - '0' as usize]).collect()
}

fn mm(v: &[f64]) -> (f64, f64) {
    v.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), x| (a.min(*x), b.max(*x)))
}

// ------------------------------------------------------------------------------------------------ fields
enum Rng {
    /// half to one and a half times the value
    Near,
    /// a tenth to ten times (logarithmic)
    Decade,
    /// whole numbers from 1 to at least this many (and past twice the value)
    Count(i64),
}

fn in_unit(s: &Solved, sym: &str) -> String {
    let kind = s.p.procedure(s.key).and_then(|p| p.inputs.iter().find(|i| i.symbol == sym)).map(|i| i.kind.clone());
    match kind.and_then(|kd| s.p.kinds.get(&kd).map(|x| x.unit.clone())) {
        Some(u) if u != "1" => u,
        _ => String::new(),
    }
}

fn meaning(s: &Solved, sym: &str, input: bool) -> String {
    let p = s.p.procedure(s.key);
    let m = if input {
        p.and_then(|p| p.inputs.iter().find(|i| i.symbol == sym)).map(|i| i.meaning.clone())
    } else {
        p.and_then(|p| p.outputs.iter().find(|o| o.symbol == sym)).map(|o| o.meaning.clone())
    };
    m.unwrap_or_else(|| sym.to_string())
}

fn axis(s: &Solved, sym: &str) -> String {
    let u = in_unit(s, sym);
    let m = meaning(s, sym, true);
    let m: String = m.split(" (").next().unwrap_or(&m).to_string();
    let t = if u.is_empty() { format!("{sym}: {m}") } else { format!("{sym}: {m} ({u})") };
    if t.chars().count() > 46 { sym.to_string() } else { t }
}

fn range_of(x0: f64, r: &Rng) -> Option<(Vec<f64>, bool, bool)> {
    match r {
        Rng::Near if x0 != 0.0 => Some((k::span((x0 * 0.5).min(x0 * 1.5), (x0 * 0.5).max(x0 * 1.5), 15, false), false,
                                        false)),
        Rng::Decade if x0 > 0.0 => Some((k::span(x0 / 10.0, x0 * 10.0, 15, true), true, false)),
        Rng::Count(n) => {
            // at most 20 rows (15 columns: 300 solves, within the figures' allowance)
            let c = x0.round() as i64;
            let (lo, hi) = if c * 2 + 3 <= 20 { (1, (*n).max(c * 2 + 3).min(20)) } else { (c - 10, c + 9) };
            Some(((lo..=hi).map(|i| i as f64).collect(), false, true))
        }
        _ => None,
    }
}

/// An output over a grid of two inputs, each cell one computation of the model.
fn field2(s: &Solved, xn: &str, xr: Rng, yn: &str, yr: Rng, out: &str, title: &str) -> Option<Figure> {
    let (x0, y0) = (s.inp(xn).or_else(|| s.val(xn).map(|v| v.num()))?, s.inp(yn).or_else(|| s.val(yn).map(|v| v.num()))?);
    let z0 = s.out(out)?;
    let (xs, xlog, xint) = range_of(x0, &xr)?;
    let (ys, ylog, yint) = range_of(y0, &yr)?;
    let arg = |v: f64, int: bool| if int { Input::Int(v as i64) } else { Input::Float(v) };
    let mut vals = vec![];
    let mut n = 0;
    for x in &xs {
        let mut cl = vec![];
        for y in &ys {
            let v = s.with_input(&[(xn, arg(*x, xint)), (yn, arg(*y, yint))]).and_then(|a| k::num_of(&a, out));
            n += v.is_some() as usize;
            cl.push(v);
        }
        vals.push(cl);
    }
    if n < 4 {
        return None;
    }
    let all: Vec<f64> = vals.iter().flatten().flatten().copied().collect();
    let (lo, hi) = mm(&all);
    // values spanning many orders of magnitude are shaded by their logarithm
    let logz = lo > 0.0 && hi / lo > 1e3;
    let shade: Vec<Vec<Option<f64>>> = if logz {
        vals.iter().map(|c| c.iter().map(|v| v.map(|x| x.log10())).collect()).collect()
    } else {
        vals.clone()
    };
    let body = k::field(&xs, &ys, &shade, (x0, y0), &axis(s, xn), &axis(s, yn),
                        &format!("{out} = {}{}", num(z0), if logz { " (shaded by log10)" } else { "" }), (xlog, ylog));
    let span_t = |v: &[f64]| format!("{} to {}", num(v[0]), num(v[v.len() - 1]));
    Some(s.chart("field", title,
                 &format!("{} ({out}) over {xn} from {} and {yn} from {}{}: each of the {n} shaded cells is one \
                           computation of the model, darker for a larger value ({}from {} to {}; a blank cell: inputs \
                           the model refuses). Marked: the inputs given, {out} = {}.", meaning(s, out, false),
                          span_t(&xs), span_t(&ys), if xlog || ylog { " (logarithmic axes)" } else { "" },
                          if logz { "shaded by the logarithm of the value, " } else { "" }, num(lo), num(hi), num(z0)),
                 &body))
}

// ------------------------------------------------------------------------------------------------ schedule
fn gantt(s: &Solved) -> Option<Figure> {
    let (a, b) = (s.vec("a")?, s.vec("b")?);
    let seq = s.outj("sequence").and_then(ints)?;
    let mk = s.out("makespan")?;
    let n = a.len();
    if n == 0 || b.len() != n || seq.len() != n || seq.iter().any(|j| *j > n) || n > 12 {
        return None;
    }
    // machine 2 finishes the k-th job of the order when the first k jobs alone would be finished: the model's own
    // makespan of that prefix (a prefix of a Johnson order is a Johnson order of its jobs)
    let mut m1 = 0.0;
    let mut prev2 = 0.0;
    let mut bars: Vec<(usize, f64, f64, f64, f64)> = vec![];
    for kk in 1..=n {
        let pre = &seq[..kk];
        let pa: Vec<f64> = pre.iter().map(|j| a[j - 1]).collect();
        let pb: Vec<f64> = pre.iter().map(|j| b[j - 1]).collect();
        let c2 = if kk == n { mk } else {
            let r = s.with_input(&[("a", Input::Json(json!(pa))), ("b", Input::Json(json!(pb)))])?;
            k::num_of(&r, "makespan")?
        };
        let j = seq[kk - 1];
        let s1 = m1;
        m1 += a[j - 1];
        let s2 = c2 - b[j - 1];
        if s2 < m1 - 1e-9 * mk.abs().max(1.0) || s2 < prev2 - 1e-9 * mk.abs().max(1.0) {
            return None;
        }
        bars.push((j, s1, m1, s2, c2));
        prev2 = c2;
    }
    let mut p = Plot::rows((0.0, mk * 1.04), &["machine 1".into(), "machine 2".into()], "Time",
                           &format!("makespan {}", num(mk)), false);
    let classes = ["axp-b", "axp-b2", "axp-b3", "axp-b4"];
    for (i, (j, s1, e1, s2, e2)) in bars.iter().enumerate() {
        let c = classes[i % 4];
        p.rect((*s1, -0.3), (*e1, 0.3), c, &format!("J{j}"));
        p.rect((*s2, 0.7), (*e2, 1.3), c, &format!("J{j}"));
    }
    p.vline(mk, "axp-c4", &format!("makespan {}", num(mk)));
    let order = seq.iter().map(|j| format!("job {j}")).collect::<Vec<_>>().join(" → ");
    Some(s.chart("gantt", "A two-machine schedule in the computed order",
                 &format!("Processing times [{}] on machine 1 and [{}] on machine 2, jobs in the computed order {order}; \
                           each job leaves machine 2 when the model's makespan of the jobs up to it is reached; \
                           makespan {}.", list_text(&a), list_text(&b), num(mk)), &p.body()))
}

// ------------------------------------------------------------------------------------------------ feasible region
fn pairs(v: &Value) -> Option<Vec<P>> {
    v.as_array()?.iter().map(|p| {
        let p = p.as_array()?;
        (p.len() == 2).then(|| Some((p[0].as_f64()?, p[1].as_f64()?))).flatten()
    }).collect()
}

fn region(s: &Solved) -> Option<Figure> {
    let (c, am, b) = (s.vec("c")?, s.mat("A")?, s.vec("b")?);
    if c.len() != 2 || am.len() != b.len() || am.iter().any(|r| r.len() != 2) {
        return None;
    }
    let corners = s.outj("corner_points").and_then(pairs)?;
    let opt = s.outj("optimal_corners").and_then(pairs).unwrap_or_default();
    let val = s.out("optimal_value")?;
    if corners.is_empty() {
        return None;
    }
    let (cx, cy) = (corners.iter().map(|p| p.0).sum::<f64>() / corners.len() as f64,
                    corners.iter().map(|p| p.1).sum::<f64>() / corners.len() as f64);
    let mut hull = corners.clone();
    hull.sort_by(|p, q| (p.1 - cy).atan2(p.0 - cx).partial_cmp(&(q.1 - cy).atan2(q.0 - cx))
        .unwrap_or(std::cmp::Ordering::Equal));
    let (xs, ys): (Vec<f64>, Vec<f64>) = hull.iter().copied().unzip();
    let (xl, xh) = mm(&xs);
    let (yl, yh) = mm(&ys);
    let pad = |l: f64, h: f64| { let d = (h - l).max(1.0) * 0.3; (l.min(0.0) - d * 0.2, h + d) };
    let (xr, yr) = (pad(xl, xh), pad(yl, yh));
    // each edge of the region must lie on a constraint line (or an axis): otherwise the region is not closed by its
    // corners (unbounded) and only the corners are drawn
    let scale = xs.iter().chain(&ys).fold(1.0f64, |a, v| a.max(v.abs()));
    let on = |p: P, q: P| -> bool {
        let tol = 1e-7 * scale;
        (p.0.abs() < tol && q.0.abs() < tol) || (p.1.abs() < tol && q.1.abs() < tol)
            || am.iter().zip(&b).any(|(r, bi)| {
                let nr = (r[0].hypot(r[1])).max(1e-300);
                ((r[0] * p.0 + r[1] * p.1 - bi) / nr).abs() < tol && ((r[0] * q.0 + r[1] * q.1 - bi) / nr).abs() < tol
            })
    };
    let closed = hull.len() >= 3 && (0..hull.len()).all(|i| on(hull[i], hull[(i + 1) % hull.len()]));
    let mut p = Plot::new(xr, yr, "x", "y", &format!("optimum {} = {}", objective(&c), num(val)), (false, false),
                          false);
    if closed {
        p.polygon(&hull, "axp-s2", "axp-c2");
    }
    // the constraint lines as given
    let line = |r: &[f64], bi: f64| -> Vec<P> {
        if r[1].abs() > 1e-12 {
            vec![(xr.0, (bi - r[0] * xr.0) / r[1]), (xr.1, (bi - r[0] * xr.1) / r[1])]
        } else if r[0].abs() > 1e-12 {
            vec![(bi / r[0], yr.0), (bi / r[0], yr.1)]
        } else {
            vec![]
        }
    };
    for (r, bi) in am.iter().zip(&b) {
        p.path(&line(r, *bi), "axp-c4");
    }
    if !opt.is_empty() {
        p.dashed(&line(&c, val), "axp-c");
    }
    for q in &corners {
        p.dot(*q, 4.0, "axp-d4");
    }
    for o in &opt {
        p.mark(*o, &format!("optimum ({}, {})", num(o.0), num(o.1)));
    }
    p.legend(&[("feasible region".into(), "axp-s2"), ("constraints".into(), "axp-b4"), ("objective".into(), "axp-b")]);
    let cl = corners.iter().map(|q| format!("({}, {})", num(q.0), num(q.1))).collect::<Vec<_>>().join(", ");
    let rels: Vec<String> = match s.txt("relations") {
        Some(t) if !t.trim().is_empty() => t.split([',', ';', ' ']).filter(|x| !x.is_empty())
            .map(|x| x.replace("<=", "≤").replace(">=", "≥")).collect(),
        _ => vec!["≤".to_string(); am.len()],
    };
    if rels.len() != am.len() {
        return None;
    }
    let cons = am.iter().zip(&b).zip(&rels).map(|((r, bi), rl)| format!("{} {rl} {}", lin(r), num(*bi)))
        .collect::<Vec<_>>().join("; ");
    Some(s.chart("region", "The feasible region and the optimal corner",
                 &format!("{} {} subject to {cons}: the constraint lines as given, {}the corner points the model \
                           returns ({cl}) and the objective line through the optimum, {} = {}.",
                          s.txt("sense").map(|t| if t.starts_with("min") { "Minimise" } else { "Maximise" })
                              .unwrap_or("Optimise"), objective(&c),
                          if closed { "the feasible region (shaded), " } else { "" }, objective(&c), num(val)),
                 &p.body()))
}

fn lin(r: &[f64]) -> String {
    let t = |a: f64, v: &str| if a == 1.0 { v.to_string() } else if a == -1.0 { format!("-{v}") } else {
        format!("{}{v}", num(a))
    };
    match (r[0] == 0.0, r[1] == 0.0) {
        (true, true) => "0".into(),
        (false, true) => t(r[0], "x"),
        (true, false) => t(r[1], "y"),
        _ => format!("{} {} {}", t(r[0], "x"), if r[1] < 0.0 { "-" } else { "+" }, t(r[1].abs(), "y")),
    }
}

fn objective(c: &[f64]) -> String {
    lin(c)
}

// ------------------------------------------------------------------------------------------------ network
fn path_network(s: &Solved) -> Option<Figure> {
    let Val::Edges(es) = s.val("edges")? else { return None };
    let path: Vec<String> = s.outj("path")?.as_array()?.iter().map(|v| match v {
        Value::String(t) => Some(t.clone()),
        Value::Number(n) => Some(n.to_string()),
        _ => None,
    }).collect::<Option<Vec<_>>>()?;
    let dist = s.out("distance")?;
    let directed = s.val("directed").map(|v| v.boolean()).unwrap_or(false);
    let mut nodes: Vec<String> = vec![];
    for (u, v, _) in &es {
        for x in [u, v] {
            if !nodes.contains(x) {
                nodes.push(x.clone());
            }
        }
    }
    if nodes.len() > 24 || es.len() > 60 || path.iter().any(|p| !nodes.contains(p)) {
        return None;
    }
    let steps: Vec<(&String, &String)> = path.windows(2).map(|w| (&w[0], &w[1])).collect();
    let hot: Vec<bool> = es.iter().map(|(u, v, _)| steps.iter().any(|(a, b)| (*a == u && *b == v)
        || (!directed && *a == v && *b == u))).collect();
    let edges: Vec<(String, String, Option<String>)> = es.iter().map(|(u, v, w)| (u.clone(), v.clone(),
                                                                                 w.map(num))).collect();
    let src = s.txt("source").unwrap_or_default();
    let tgt = s.txt("target").unwrap_or_default();
    let ringed: Vec<String> = [src.clone(), tgt.clone()].into_iter().filter(|x| nodes.contains(x)).collect();
    let body = k::network(&nodes, &edges, &hot, directed, None, &path, &ringed, &[],
                          &format!("distance {} along {}", num(dist), path.join(" → ")));
    Some(s.chart("graph-2", "The network with the computed shortest path",
                 &format!("The {} edges as given ({}), weights written beside them, nodes on a circle; highlighted: the \
                           shortest path the model returns, {}, of length {}; ringed: source {src} and target {tgt}.",
                          es.len(), if directed { "directed" } else { "undirected" }, path.join(" → "), num(dist)),
                 &body))
}
