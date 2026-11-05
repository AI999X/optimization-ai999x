//! The figure kit: how a model crate gives its capabilities their own live figures (a circuit with its solved values,
//! a Bode diagram, a Karnaugh map, ...) drawn for ANY inputs from the solved answer, and the drawing primitives every
//! model shares. The generic engine (figures.rs) places a model's figures first, then its own sweeps.
//!
//!   in the model crate:   p.figures = Some(figures::live);           // fn live(s: &Solved) -> Vec<Figure>
//!   in `live`:            match s.key { "voltage_divider" => divider(s), _ => vec![] }
//!
//! A figure is drawn only from what the model computed: the answer's outputs, its inputs as read, and further solves
//! of the model itself (`Solved::with`, `Solved::solve_other`, at most MAX_MODEL_SOLVES). A figure that cannot be
//! drawn for these inputs is left out (return None / an empty list), never drawn wrong; figures.rs also drops any
//! figure whose drawing holds a non-finite number.
//!
//! Every figure uses the website's figure classes (axp-*), so it follows the page's light and dark theme; the
//! fallback style in figures.rs serves a file opened on its own. Slugs are "<capability-slug>-<kind>", the kind being
//! the published figure's kind ("schematic", "bode", ...): tools/figure_parity.py matches them.

use crate::engine::{solve, Answer, Input};
use crate::figures::{self as f, Figure};
use crate::num::py_g;
use crate::pack::{Pack, Val};
use serde_json::Value;
use std::cell::Cell;

/// A model's figure function: the figures it draws for one solved request (none for a capability it does not draw).
pub type ModelFigs = fn(&Solved) -> Vec<Figure>;

/// The most solves a model's figures may make for one request (a curve family, a grid): its own allowance, apart from
/// the generic sweeps'.
pub const MAX_MODEL_SOLVES: usize = 400;

/// One solved request, as a model's figure function sees it.
pub struct Solved<'a> {
    pub p: &'a Pack,
    pub key: &'a str,
    pub given: &'a [(String, Input)],
    pub a: &'a Answer,
    /// the quantity asked for (a formula solved for one of its symbols), kept when this capability is solved again
    pub target: Option<&'a str>,
    used: Cell<usize>,
    max: usize,
}

impl<'a> Solved<'a> {
    pub fn new(p: &'a Pack, key: &'a str, given: &'a [(String, Input)], target: Option<&'a str>, a: &'a Answer)
               -> Solved<'a> {
        Solved { p, key, given, a, target, used: Cell::new(0), max: MAX_MODEL_SOLVES }
    }

    /// A quantity of this request, whichever side it is on: an input as read (SI), an output, or a formula's answer.
    pub fn get(&self, s: &str) -> Option<f64> {
        self.inp(s).or_else(|| self.out(s))
    }

    /// A number input as read, in SI (None when not given or not finite).
    pub fn inp(&self, s: &str) -> Option<f64> {
        self.a.si.iter().find(|x| x.0 == s).map(|x| x.1).filter(|x| x.is_finite())
    }
    /// A structured input (a list, a matrix, text, a choice) read as the procedure reads it.
    pub fn val(&self, s: &str) -> Option<Val> {
        let spec = self.p.procedure(self.key)?.inputs.iter().find(|i| i.symbol == s)?;
        let v = &self.given.iter().find(|g| g.0 == s)?.1;
        crate::coerce::coerce(spec, v).ok()
    }
    /// A text or choice input as given.
    pub fn txt(&self, s: &str) -> Option<String> {
        match self.val(s)? {
            Val::Text(t) => Some(t),
            _ => None,
        }
    }
    /// A list input.
    pub fn vec(&self, s: &str) -> Option<Vec<f64>> {
        match self.val(s)? {
            Val::Vec(v) if v.iter().all(|x| x.is_finite()) => Some(v),
            Val::Num(x) if x.is_finite() => Some(vec![x]),
            Val::Int(i) => Some(vec![i as f64]),
            _ => None,
        }
    }
    /// A matrix input.
    pub fn mat(&self, s: &str) -> Option<Vec<Vec<f64>>> {
        match self.val(s)? {
            Val::Mat(m) if !m.is_empty() && m.iter().flatten().all(|x| x.is_finite()) => Some(m),
            _ => None,
        }
    }
    /// An output as returned (a number, a list, text, true/false).
    pub fn outj(&self, s: &str) -> Option<&Value> {
        self.a.values.iter().find(|x| x.0 == s).map(|x| &x.1)
    }
    /// A number output (None when absent or not finite).
    pub fn out(&self, s: &str) -> Option<f64> {
        num_of(self.a, s)
    }
    /// A list output (numbers).
    pub fn out_list(&self, s: &str) -> Option<Vec<f64>> {
        list_of(self.a, s)
    }
    /// A text, verdict or true/false output as words.
    pub fn out_text(&self, s: &str) -> Option<String> {
        match self.outj(s)? {
            Value::String(t) => Some(t.clone()),
            Value::Bool(b) => Some(if *b { "yes".into() } else { "no".into() }),
            Value::Number(n) => Some(py_g(n.as_f64()?, 6)),
            Value::Null => None,
            other => Some(other.to_string()),
        }
    }
    /// How many more solves the figures may make.
    pub fn left(&self) -> usize {
        self.max.saturating_sub(self.used.get())
    }
    /// This capability solved again with some number inputs set (SI values); None when refused or out of solves.
    pub fn with(&self, set: &[(&str, f64)]) -> Option<Answer> {
        if set.iter().any(|x| !x.1.is_finite()) {
            return None;
        }
        let v: Vec<(&str, Input)> = set.iter().map(|(k, x)| (*k, Input::Float(*x))).collect();
        self.with_input(&v)
    }
    /// This capability solved again with some inputs replaced (any input: a list as Input::Json, text, a number).
    pub fn with_input(&self, set: &[(&str, Input)]) -> Option<Answer> {
        let mut g: Vec<(String, Input)> = self.given.to_vec();
        for (k, x) in set {
            match g.iter_mut().find(|e| e.0 == *k) {
                Some(e) => e.1 = x.clone(),
                None => g.push((k.to_string(), x.clone())),
            }
        }
        self.solve_given(self.key, &g, self.target)
    }
    /// Another capability of the same model solved with these inputs (numbers in SI).
    pub fn solve_other(&self, key: &str, set: &[(&str, Input)]) -> Option<Answer> {
        let g: Vec<(String, Input)> = set.iter().map(|(k, v)| (k.to_string(), v.clone())).collect();
        self.solve_given(key, &g, None)
    }
    fn solve_given(&self, key: &str, g: &[(String, Input)], target: Option<&str>) -> Option<Answer> {
        if self.used.get() >= self.max {
            return None;
        }
        self.used.set(self.used.get() + 1);
        let a = solve(self.p, key, g, target);
        a.solved().then_some(a)
    }
    /// A curve of one output against one input over `xs` (points the model refuses are left out).
    pub fn curve(&self, x: &str, xs: &[f64], y: &str, fixed: &[(&str, f64)]) -> Vec<(f64, f64)> {
        let mut v = vec![];
        for &xv in xs {
            let mut set: Vec<(&str, f64)> = fixed.to_vec();
            set.push((x, xv));
            if let Some(b) = self.with(&set) {
                if let Some(yv) = num_of(&b, y) {
                    v.push((xv, yv));
                }
            }
        }
        v
    }
    /// A family of curves: `y` against `x` over `xs`, one curve for each value of the input `param` (SI), named
    /// "param = value unit"; curves with fewer than two points are left out.
    pub fn family(&self, x: &str, xs: &[f64], y: &str, param: &str, values: &[f64], unit: &str)
                  -> Vec<(String, Vec<(f64, f64)>)> {
        let mut out = vec![];
        for v in values {
            let pts = self.curve(x, xs, y, &[(param, *v)]);
            if pts.len() >= 2 {
                out.push((eq(param, *v, unit), pts));
            }
        }
        out
    }
    /// The inputs as the caption states them ("Your inputs: ...").
    pub fn inputs_note(&self) -> String {
        f::inputs_note(self.given)
    }
    /// A figure from a drawing body `h` pixels tall (520 wide): its slug "<capability>-<kind>", the caption ending
    /// with the visitor's inputs.
    pub fn figure(&self, kind: &str, title: &str, caption: &str, body: &str, h: f64) -> Figure {
        let slug0 = self.key.replace('_', "-");
        // the answer in plain SI as well, when the drawing states it only with a prefix (1.424 mA for 0.001424 A)
        let mut caption = caption.to_string();
        if let (Some(v), Some(t)) = (self.a.value.filter(|v| v.is_finite()), self.a.target.as_deref().or(self.a.solved_for.as_deref())) {
            let plain = py_g(v, 4);
            if !caption.contains(&plain) {
                let u = self.a.unit.clone().unwrap_or_default();
                let u = if u.is_empty() || u == "1" { String::new() } else { format!(" {u}") };
                caption.push_str(&format!(" In SI units: {t} = {plain}{u}."));
            }
        }
        let caption = format!("{caption}{}", self.inputs_note());
        let id = format!("axm-{slug0}-{kind}");
        Figure { slug: format!("{slug0}-{kind}"), kind: kind.to_string(), title: title.to_string(),
                 svg: f::wrap_svg_h(&id, title, &caption, body, h), caption }
    }
    /// A chart figure (the generic 520 x 340 frame).
    pub fn chart(&self, kind: &str, title: &str, caption: &str, body: &str) -> Figure {
        self.figure(kind, title, caption, body, 340.0)
    }
}

/// A number output of an answer.
pub fn num_of(a: &Answer, s: &str) -> Option<f64> {
    let own = a.values.iter().find(|x| x.0 == s).and_then(|x| x.1.as_f64());
    // a formula's answer is its value, under the symbol it was solved for
    let formula = (a.values.is_empty() && (a.solved_for.as_deref() == Some(s) || a.target.as_deref() == Some(s)))
        .then_some(a.value).flatten();
    own.or(formula).filter(|x| x.is_finite())
}

/// A list output of complex numbers (a root list: numbers, or {"re", "im"} objects) as (re, im) pairs.
pub fn cx_list(a: &Answer, s: &str) -> Option<Vec<(f64, f64)>> {
    let v = a.values.iter().find(|x| x.0 == s)?.1.as_array()?;
    let out: Vec<(f64, f64)> = v.iter().filter_map(f::cxv).filter(|z| z.0.is_finite() && z.1.is_finite()).collect();
    (out.len() == v.len() && !out.is_empty()).then_some(out)
}

/// A list output of an answer (numbers only).
pub fn list_of(a: &Answer, s: &str) -> Option<Vec<f64>> {
    let v = a.values.iter().find(|x| x.0 == s)?.1.as_array()?;
    let out: Vec<f64> = v.iter().filter_map(|x| x.as_f64()).filter(|x| x.is_finite()).collect();
    (out.len() == v.len() && !out.is_empty()).then_some(out)
}

// ------------------------------------------------------------------------------------------------ numbers as text
/// A value that is rounding noise beside `scale` (cos 90° = 6.1e-17) shown as 0: |x| < 1e-12 |scale|.
pub fn snap(x: f64, scale: f64) -> f64 {
    if x.abs() < 1e-12 * scale.abs() { 0.0 } else { x }
}

/// A number to 4 significant digits ("1.424", "6.874e-05").
pub fn num(x: f64) -> String {
    py_g(x, 4)
}

/// A quantity with an SI prefix and its unit, 4 significant digits: q(0.00142402, "A") = "1.424 mA",
/// q(10000.0, "ohm") = "10 kΩ". Units without prefixes (dB, %, °, rad/s, a pure number "") are written as is.
pub fn q(x: f64, unit: &str) -> String {
    let u = match unit {
        "ohm" | "Ohm" | "ohms" => "Ω",
        "1" => "",
        other => other,
    };
    let prefixed = matches!(u, "V" | "A" | "W" | "Ω" | "F" | "H" | "Hz" | "s" | "VA" | "var" | "J" | "S" | "Wb" | "T"
                               | "N" | "Pa" | "m");
    if !x.is_finite() {
        return "?".into();
    }
    if !prefixed || x == 0.0 {
        return if u.is_empty() { num(x) } else { format!("{} {u}", num(x)) };
    }
    const P: [(&str, i32); 9] = [("p", -12), ("n", -9), ("µ", -6), ("m", -3), ("", 0), ("k", 3), ("M", 6), ("G", 9),
                                 ("T", 12)];
    let e = x.abs().log10().floor() as i32;
    let mut best = P[4];
    for p in P {
        if e >= p.1 {
            best = p;
        }
    }
    // a value just under a power of a thousand (999.96 m) rounds to 1000 at 4 digits: take the next prefix
    let mut m = x / 10f64.powi(best.1);
    if num(m.abs()) == "1000" && best.1 < 12 {
        best = P.iter().copied().find(|p| p.1 == best.1 + 3).unwrap_or(best);
        m = x / 10f64.powi(best.1);
    }
    format!("{} {}{u}", num(m), best.0)
}

/// "name = value unit".
pub fn eq(name: &str, x: f64, unit: &str) -> String {
    format!("{name} = {}", q(x, unit))
}

// ------------------------------------------------------------------------------------------------ the canvas
/// Approximate width of a label (13 px text; the monospace values are about 7.8 px a character).
pub fn text_w(t: &str, mono: bool) -> f64 {
    t.chars().count() as f64 * if mono { 7.9 } else { 7.2 }
}

/// A drawing surface 520 wide and `h` tall, in the site's figure classes: wires and symbols (axp-k), filled marks
/// (axp-kd), names (axp-lt), computed values (axp-vt), headings (axp-bt). Text is kept inside the canvas.
pub struct Canvas {
    pub w: f64,
    pub h: f64,
    pub s: String,
}

pub type P = (f64, f64);

impl Canvas {
    pub fn new(h: f64) -> Canvas {
        Canvas { w: 520.0, h, s: String::new() }
    }
    pub fn raw(&mut self, svg: &str) {
        self.s.push_str(svg);
    }
    /// A polyline wire.
    pub fn wire(&mut self, pts: &[P]) {
        if pts.len() < 2 {
            return;
        }
        let mut d = String::new();
        for (i, p) in pts.iter().enumerate() {
            d.push_str(&format!("{}{:.1},{:.1} ", if i == 0 { "M" } else { "L" }, p.0, p.1));
        }
        self.s.push_str(&format!("<path class=\"axp-k\" d=\"{}\"/>", d.trim_end()));
    }
    pub fn line(&mut self, a: P, b: P) {
        self.wire(&[a, b]);
    }
    /// A dashed line (a guide, a measured gap).
    pub fn dashed(&mut self, a: P, b: P) {
        self.s.push_str(&format!("<path class=\"axp-c4\" stroke-dasharray=\"5 4\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>",
                                 a.0, a.1, b.0, b.1));
    }
    /// A junction dot.
    pub fn dot(&mut self, p: P) {
        self.s.push_str(&format!("<circle class=\"axp-kd\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"3.2\"/>", p.0, p.1));
    }
    /// An open terminal (a small ring) with an optional name.
    pub fn terminal(&mut self, p: P, name: &str) {
        self.s.push_str(&format!("<circle class=\"axp-k\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"4\"/>", p.0, p.1));
        if !name.is_empty() {
            self.name(p.0, p.1 - 10.0, "middle", name);
        }
    }
    /// Text in a class; anchor "start" | "middle" | "end". Moved inside the canvas when it would leave it.
    pub fn text(&mut self, x: f64, y: f64, anchor: &str, class: &str, t: &str) {
        if t.is_empty() {
            return;
        }
        let w = text_w(t, class == "axp-vt" || class == "axp-mt");
        let left = match anchor {
            "middle" => x - w / 2.0,
            "end" => x - w,
            _ => x,
        };
        let shift = if left < 4.0 { 4.0 - left } else if left + w > self.w - 4.0 { self.w - 4.0 - (left + w) } else { 0.0 };
        let y = y.clamp(14.0, self.h - 4.0);
        self.s.push_str(&format!("<text class=\"{class}\" x=\"{:.1}\" y=\"{y:.1}\" text-anchor=\"{anchor}\">{}</text>",
                                 x + shift, f::esc(t)));
    }
    /// A component's name (grey).
    pub fn name(&mut self, x: f64, y: f64, anchor: &str, t: &str) {
        self.text(x, y, anchor, "axp-lt", t);
    }
    /// A computed or stated value (green, monospace).
    pub fn value(&mut self, x: f64, y: f64, anchor: &str, t: &str) {
        self.text(x, y, anchor, "axp-vt", t);
    }
    /// A heading or emphasised label (bold).
    pub fn bold(&mut self, x: f64, y: f64, anchor: &str, t: &str) {
        self.text(x, y, anchor, "axp-bt", t);
    }
    /// Name above value, beside a component at (x, y): anchor "start" puts them right of x, "end" left of it.
    pub fn tag(&mut self, x: f64, y: f64, anchor: &str, name: &str, value: &str) {
        self.name(x, y - 3.0, anchor, name);
        self.value(x, y + 13.0, anchor, value);
    }

    fn along(a: P, b: P) -> (f64, P, P) {
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let d = (dx * dx + dy * dy).sqrt().max(1e-9);
        (d, (dx / d, dy / d), (-dy / d, dx / d))
    }
    /// The wire from a to the start of a body of length `body` centred on a-b, and from its end to b; returns the
    /// body's start and end.
    fn leads(&mut self, a: P, b: P, body: f64) -> (P, P, P, P) {
        let (d, u, n) = Canvas::along(a, b);
        let body = body.min(d * 0.9);
        let s0 = (a.0 + u.0 * (d - body) / 2.0, a.1 + u.1 * (d - body) / 2.0);
        let s1 = (s0.0 + u.0 * body, s0.1 + u.1 * body);
        self.line(a, s0);
        self.line(s1, b);
        (s0, s1, u, n)
    }
    /// A resistor (zig-zag) from a to b.
    pub fn resistor(&mut self, a: P, b: P) {
        let (s0, _s1, u, n) = self.leads(a, b, 48.0);
        let body = 48.0f64.min(Canvas::along(a, b).0 * 0.9);
        let mut pts = vec![s0];
        for k in 0..6 {
            let t = body * (2 * k + 1) as f64 / 12.0;
            let side = if k % 2 == 0 { 8.0 } else { -8.0 };
            pts.push((s0.0 + u.0 * t + n.0 * side, s0.1 + u.1 * t + n.1 * side));
        }
        pts.push((s0.0 + u.0 * body, s0.1 + u.1 * body));
        self.wire(&pts);
    }
    /// A capacitor (two plates) from a to b.
    pub fn capacitor(&mut self, a: P, b: P) {
        let (s0, s1, _u, n) = self.leads(a, b, 10.0);
        for s in [s0, s1] {
            self.line((s.0 + n.0 * 15.0, s.1 + n.1 * 15.0), (s.0 - n.0 * 15.0, s.1 - n.1 * 15.0));
        }
    }
    /// An inductor (four turns) from a to b.
    pub fn inductor(&mut self, a: P, b: P) {
        let (s0, _s1, u, n) = self.leads(a, b, 48.0);
        let body = 48.0f64.min(Canvas::along(a, b).0 * 0.9);
        let r = body / 8.0;
        let mut d = format!("M{:.1},{:.1}", s0.0, s0.1);
        for k in 1..=4 {
            let e = (s0.0 + u.0 * body * k as f64 / 4.0, s0.1 + u.1 * body * k as f64 / 4.0);
            // the bump on the side of n
            let sweep = if n.0 * u.1 - n.1 * u.0 < 0.0 { 1 } else { 0 };
            d.push_str(&format!(" A{r:.1},{r:.1} 0 0 {sweep} {:.1},{:.1}", e.0, e.1));
        }
        self.s.push_str(&format!("<path class=\"axp-k\" d=\"{d}\"/>"));
    }
    /// A voltage source between a (+) and b (-); `ac`: a sine inside instead of the signs.
    pub fn vsource(&mut self, a: P, b: P, ac: bool) {
        let (s0, s1, u, _n) = self.leads(a, b, 36.0);
        let c = ((s0.0 + s1.0) / 2.0, (s0.1 + s1.1) / 2.0);
        self.s.push_str(&format!("<circle class=\"axp-k\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"18\"/>", c.0, c.1));
        if ac {
            self.s.push_str(&format!("<path class=\"axp-k\" d=\"M{:.1},{:.1} q5,-12 10,0 t10,0\"/>", c.0 - 10.0, c.1));
        } else {
            let p = (c.0 - u.0 * 9.0, c.1 - u.1 * 9.0);
            let m = (c.0 + u.0 * 9.0, c.1 + u.1 * 9.0);
            self.line((p.0 - 4.0, p.1), (p.0 + 4.0, p.1));
            self.line((p.0, p.1 - 4.0), (p.0, p.1 + 4.0));
            self.line((m.0 - 4.0, m.1), (m.0 + 4.0, m.1));
        }
    }
    /// A current source between a and b, its arrow pointing from a to b.
    pub fn isource(&mut self, a: P, b: P) {
        let (s0, s1, u, n) = self.leads(a, b, 36.0);
        let c = ((s0.0 + s1.0) / 2.0, (s0.1 + s1.1) / 2.0);
        self.s.push_str(&format!("<circle class=\"axp-k\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"18\"/>", c.0, c.1));
        let t = (c.0 - u.0 * 10.0, c.1 - u.1 * 10.0);
        let h = (c.0 + u.0 * 10.0, c.1 + u.1 * 10.0);
        self.line(t, h);
        self.head(h, u, n);
    }
    fn head(&mut self, h: P, u: P, n: P) {
        let b = (h.0 - u.0 * 8.0, h.1 - u.1 * 8.0);
        self.s.push_str(&format!("<path class=\"axp-kd\" d=\"M{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} Z\"/>", h.0, h.1,
                                 b.0 + n.0 * 4.5, b.1 + n.1 * 4.5, b.0 - n.0 * 4.5, b.1 - n.1 * 4.5));
    }
    /// An arrow from a to b (a signal, a current direction).
    pub fn arrow(&mut self, a: P, b: P) {
        let (_, u, n) = Canvas::along(a, b);
        self.line(a, (b.0 - u.0 * 6.0, b.1 - u.1 * 6.0));
        self.head(b, u, n);
    }
    /// A diode from anode a to cathode b; `zener`: the cathode bar with bent ends.
    pub fn diode(&mut self, a: P, b: P, zener: bool) {
        let (s0, s1, u, n) = self.leads(a, b, 22.0);
        self.s.push_str(&format!("<path class=\"axp-k\" d=\"M{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} Z\"/>",
                                 s0.0 + n.0 * 11.0, s0.1 + n.1 * 11.0, s0.0 - n.0 * 11.0, s0.1 - n.1 * 11.0, s1.0, s1.1));
        let (p, q) = ((s1.0 + n.0 * 11.0, s1.1 + n.1 * 11.0), (s1.0 - n.0 * 11.0, s1.1 - n.1 * 11.0));
        if zener {
            self.wire(&[(p.0 - u.0 * 5.0, p.1 - u.1 * 5.0), p, q, (q.0 + u.0 * 5.0, q.1 + u.1 * 5.0)]);
        } else {
            self.line(p, q);
        }
    }
    /// A ground symbol below p.
    pub fn ground(&mut self, p: P) {
        self.line(p, (p.0, p.1 + 12.0));
        for (k, w) in [12.8, 8.0, 3.2].iter().enumerate() {
            let y = p.1 + 12.0 + 4.0 * k as f64;
            self.line((p.0 - w, y), (p.0 + w, y));
        }
    }
    /// An npn transistor centred at c; returns (base, collector, emitter) terminals.
    pub fn npn(&mut self, c: P) -> (P, P, P) {
        let (x, y) = c;
        self.s.push_str(&format!("<circle class=\"axp-k\" cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"25\"/>"));
        self.s.push_str(&format!("<path class=\"axp-k\" style=\"stroke-width:3\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>",
                                 x - 12.0, y + 14.0, x - 12.0, y - 14.0));
        self.line((x - 38.0, y), (x - 12.0, y));
        self.wire(&[(x - 12.0, y - 6.0), (x + 12.0, y - 20.0), (x + 12.0, y - 36.0)]);
        self.wire(&[(x - 12.0, y + 6.0), (x + 12.0, y + 20.0), (x + 12.0, y + 36.0)]);
        let (_, u, n) = Canvas::along((x - 12.0, y + 6.0), (x + 12.0, y + 20.0));
        self.head((x + 10.0, y + 18.8), u, n);
        ((x - 38.0, y), (x + 12.0, y - 36.0), (x + 12.0, y + 36.0))
    }
    /// An n-channel MOSFET centred at c; returns (gate, drain, source) terminals.
    pub fn nmos(&mut self, c: P) -> (P, P, P) {
        let (x, y) = c;
        self.line((x - 12.0, y - 16.0), (x - 12.0, y + 16.0));
        self.line((x - 36.0, y + 16.0), (x - 12.0, y + 16.0));
        for (a, b) in [(-20.0, -8.0), (-6.0, 6.0), (8.0, 20.0)] {
            self.s.push_str(&format!("<path class=\"axp-k\" style=\"stroke-width:3\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>",
                                     x - 4.0, y + a, x - 4.0, y + b));
        }
        self.wire(&[(x - 4.0, y - 14.0), (x + 14.0, y - 14.0), (x + 14.0, y - 36.0)]);
        self.wire(&[(x - 4.0, y + 14.0), (x + 14.0, y + 14.0), (x + 14.0, y + 36.0)]);
        self.line((x - 4.0, y), (x + 14.0, y));
        self.line((x + 14.0, y), (x + 14.0, y + 14.0));
        self.head((x - 3.0, y), (-1.0, 0.0), (0.0, 1.0));
        ((x - 36.0, y + 16.0), (x + 14.0, y - 36.0), (x + 14.0, y + 36.0))
    }
    /// An operational amplifier, its inputs on the left edge at x (inverting above), centred on y; returns
    /// (inverting input, non-inverting input, output) terminals.
    pub fn opamp(&mut self, x: f64, y: f64) -> (P, P, P) {
        self.s.push_str(&format!("<path class=\"axp-k\" d=\"M{x:.1},{:.1} L{x:.1},{:.1} L{:.1},{y:.1} Z\"/>",
                                 y - 38.0, y + 38.0, x + 76.0));
        self.line((x + 6.0, y - 18.0), (x + 14.0, y - 18.0));
        self.line((x + 6.0, y + 18.0), (x + 14.0, y + 18.0));
        self.line((x + 10.0, y + 14.0), (x + 10.0, y + 22.0));
        self.line((x - 14.0, y - 18.0), (x, y - 18.0));
        self.line((x - 14.0, y + 18.0), (x, y + 18.0));
        self.line((x + 76.0, y), (x + 90.0, y));
        ((x - 14.0, y - 18.0), (x - 14.0, y + 18.0), (x + 90.0, y))
    }
    /// A labelled block (a transfer function, a motor, a load).
    pub fn block(&mut self, x: f64, y: f64, w: f64, h: f64, label: &str) {
        self.s.push_str(&format!("<rect class=\"axp-ka\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{w:.1}\" height=\"{h:.1}\"/>\
                                  <rect class=\"axp-k\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{w:.1}\" height=\"{h:.1}\" rx=\"4\"/>"));
        self.bold(x + w / 2.0, y + h / 2.0 + 5.0, "middle", label);
    }
    /// A circle with a letter (a motor "M", a summing junction).
    pub fn ring(&mut self, c: P, r: f64, label: &str) {
        self.s.push_str(&format!("<circle class=\"axp-k\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"{r:.1}\"/>", c.0, c.1));
        self.bold(c.0, c.1 + 5.0, "middle", label);
    }
    /// A conductor seen end-on: a ring with a dot (current out of the page) or a cross (into it).
    pub fn wire_end(&mut self, c: P, out: bool) {
        self.s.push_str(&format!("<circle class=\"axp-ka\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"11\"/>\
                                  <circle class=\"axp-k\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"11\"/>", c.0, c.1, c.0, c.1));
        if out {
            self.dot(c);
        } else {
            self.line((c.0 - 6.0, c.1 - 6.0), (c.0 + 6.0, c.1 + 6.0));
            self.line((c.0 - 6.0, c.1 + 6.0), (c.0 + 6.0, c.1 - 6.0));
        }
    }
    /// A closed field line: a circle of radius r round c with two arrowheads showing its sense (anticlockwise on
    /// the page when `ccw`), in the accent colour.
    pub fn field_ring(&mut self, c: P, r: f64, ccw: bool) {
        self.s.push_str(&format!("<circle class=\"axp-c\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"{r:.1}\"/>", c.0, c.1));
        for th in [std::f64::consts::FRAC_PI_4 * 3.0, -std::f64::consts::FRAC_PI_4] {
            let p = (c.0 + r * th.cos(), c.1 - r * th.sin());
            let u = if ccw { (-th.sin(), -th.cos()) } else { (th.sin(), th.cos()) };
            let n = (-u.1, u.0);
            let h = (p.0 + u.0 * 5.0, p.1 + u.1 * 5.0);
            let b = (h.0 - u.0 * 10.0, h.1 - u.1 * 10.0);
            self.s.push_str(&format!("<path class=\"axp-d\" d=\"M{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} Z\"/>", h.0, h.1,
                                     b.0 + n.0 * 5.0, b.1 + n.1 * 5.0, b.0 - n.0 * 5.0, b.1 - n.1 * 5.0));
        }
    }
    /// The drawing (for Solved::figure).
    pub fn body(&self) -> &str {
        &self.s
    }
}

// ------------------------------------------------------------------------------------------------ charts
/// Curves (one per series, a legend when several; a series named "" stays out of it) in the generic 520 x 340
/// frame; `marker`: the visitor's point and its label; `log_x`: a logarithmic horizontal axis. A point with a NaN
/// value breaks its curve (two pieces of one series, e.g. a tolerance band's two edges).
pub fn lines(series: &[(String, Vec<(f64, f64)>)], xlab: &str, ylab: &str, header: &str, marker: Option<(f64, f64)>,
             mlabel: &str, log_x: bool) -> String {
    lines_y(series, xlab, ylab, header, marker, mlabel, log_x, false)
}

/// `lines` with a linear vertical axis even when the values span orders of magnitude (`lin_y`: a curve rising from
/// zero, which would otherwise be drawn on a logarithmic axis).
#[allow(clippy::too_many_arguments)]
pub fn lines_y(series: &[(String, Vec<(f64, f64)>)], xlab: &str, ylab: &str, header: &str, marker: Option<(f64, f64)>,
               mlabel: &str, log_x: bool, lin_y: bool) -> String {
    let s: Vec<(String, Vec<Option<(f64, f64)>>)> = series.iter()
        .map(|(n, p)| (n.clone(), p.iter().map(|q| (q.0.is_finite() && q.1.is_finite()).then_some(*q)).collect()))
        .collect();
    let xs: Vec<f64> = series.iter().flat_map(|x| x.1.iter().map(|p| p.0)).chain(marker.map(|m| m.0))
        .filter(|x| x.is_finite()).collect();
    if xs.is_empty() {
        return String::new();
    }
    let xr = f::minmax(&xs);
    f::line_chart_xy(&s, &[], xr, xlab, ylab, marker, mlabel, header, false, log_x, lin_y)
}

/// Bars with their values; `highlight`: the bars drawn in the accent colour (the rest grey).
pub fn bars(labels: &[String], values: &[f64], highlight: &[usize], xlab: &str, ylab: &str, header: &str) -> String {
    let g = vec![(String::new(), values.iter().map(|v| Some(*v)).collect())];
    f::bar_chart_hl(labels, &g, highlight, xlab, ylab, header)
}

/// Grouped bars: one colour per group (a legend), values written on the bars when few.
pub fn bar_groups(labels: &[String], groups: &[(String, Vec<Option<f64>>)], xlab: &str, ylab: &str, header: &str)
                  -> String {
    f::bar_chart_hl(labels, groups, &[], xlab, ylab, header)
}

/// Points in a plane with equal scales (the complex plane: real across, imaginary up): `dots` small, `mine` marked and
/// labelled; the axes through the origin dashed.
pub fn plane(dots: &[(f64, f64)], mine: &[(String, (f64, f64))], xlab: &str, ylab: &str, header: &str) -> String {
    let mut xs: Vec<f64> = dots.iter().map(|z| z.0).chain(mine.iter().map(|m| m.1 .0)).collect();
    let mut ys: Vec<f64> = dots.iter().map(|z| z.1).chain(mine.iter().map(|m| m.1 .1)).collect();
    xs.push(0.0);
    ys.push(0.0);
    let (xa, ya) = f::equal_axes(&xs, &ys);
    let mut out = String::new();
    f::frame(&mut out, Some(&xa), Some(&ya), xlab, ylab, header);
    out.push_str(&format!("<clipPath id=\"CLIPID\"><rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"/></clipPath>\
                           <g clip-path=\"url(#CLIPID)\">", f::X0, f::Y0, f::X1 - f::X0, f::Y1 - f::Y0));
    let (ox, oy) = (xa.px(0.0), ya.px(0.0));
    out.push_str(&format!("<path class=\"axp-c4\" stroke-dasharray=\"5 4\" d=\"M{ox:.1},{} L{ox:.1},{} M{},{oy:.1} L{},{oy:.1}\"/>",
                          f::Y0, f::Y1, f::X0, f::X1));
    for (x, y) in dots.iter().take(2000) {
        out.push_str(&format!("<circle class=\"axp-d\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"2.2\"/>", xa.px(*x), ya.px(*y)));
    }
    out.push_str("</g>");
    for (k, (s, (x, y))) in mine.iter().enumerate().take(6) {
        let (px, py) = (xa.px(*x), ya.px(*y));
        let t = f::clip(s, 30);
        let w = text_w(&t, true);
        let tx = if px + 10.0 + w <= f::X1 { px + 10.0 } else { (px - 10.0 - w).max(f::X0) };
        out.push_str(&format!("<circle class=\"axp-m\" cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"6\"/>\
                               <text class=\"axp-mt\" x=\"{tx:.1}\" y=\"{:.1}\">{}</text>",
                              (py - 9.0 + 17.0 * (k % 2) as f64).clamp(f::Y0 + 14.0, f::Y1 - 4.0), f::esc(&t)));
    }
    out
}

/// A matrix as a heat map (positive in the accent colour, negative in the second), rows and columns labelled.
pub fn heat(m: &[Vec<f64>], rows: &[String], cols: &[String], header: &str) -> String {
    if m.is_empty() || m[0].is_empty() {
        return String::new();
    }
    // a non-finite entry is a gap between panels (a matrix and the vector beside it): left blank, not outlined
    let full = f::heat_labeled(m, rows, cols, header);
    let mut out = String::with_capacity(full.len());
    let mut rest = full.as_str();
    while let Some(i) = rest.find("<rect class=\"axp-f\"") {
        out.push_str(&rest[..i]);
        rest = &rest[i..];
        rest = rest.find("/>").map(|j| &rest[j + 2..]).unwrap_or("");
    }
    out.push_str(rest);
    out
}

/// Plane vectors as arrows from the origin: (label, (x, y), returned by the model).
pub fn arrows(v: &[(String, (f64, f64), bool)], header: &str) -> String {
    f::arrows_chart(v, header)
}

/// Points or paths in the complex plane; `mine`: labelled marked points.
pub fn complex(paths: &[(String, Vec<(f64, f64)>)], mine: &[(String, (f64, f64))], header: &str) -> String {
    f::complex_chart(paths, mine, false, header)
}

/// A Karnaugh map of a function of 2 to 6 variables: vals[minterm] (1, 0, or None for a don't-care).
pub fn kmap(vars: &[String], vals: &[Option<f64>], header: &str) -> String {
    f::kmap_chart(vars, vals, None, header)
}

/// A truth table: every assignment of `vars` (rows in binary order) and one column per function.
pub fn truth(vars: &[String], cols: &[(String, Vec<Option<f64>>)], mark: Option<usize>, header: &str) -> String {
    f::truth_chart(vars, cols, mark, header)
}

/// A verdict over a grid of two inputs: cells (i, j, verdict), one colour per verdict, the visitor's point marked.
#[allow(clippy::too_many_arguments)]
pub fn decision(xs: &[f64], ys: &[f64], cells: &[(usize, usize, String)], names: &[String], mine: (f64, f64),
                xlab: &str, ylab: &str, header: &str) -> String {
    f::grid_chart(xs, ys, cells, names, mine, xlab, ylab, header)
}

/// A value over a grid of two inputs, each cell shaded by its value (darker = larger), the visitor's point marked;
/// vals[i][j] at (xs[i], ys[j]); a refused cell is left blank. Logarithmic axes when asked (positive values).
#[allow(clippy::too_many_arguments)]
pub fn field(xs: &[f64], ys: &[f64], vals: &[Vec<Option<f64>>], mine: (f64, f64), xlab: &str, ylab: &str,
             header: &str, log: (bool, bool)) -> String {
    let all: Vec<f64> = vals.iter().flatten().flatten().copied().collect();
    if all.is_empty() || xs.len() < 2 || ys.len() < 2 {
        return String::new();
    }
    let (lo, hi) = f::minmax(&all);
    let lo = snap(lo, hi);
    let xa = f::Ax::new(xs[0], xs[xs.len() - 1], log.0, f::X0, f::X1, false);
    let ya = f::Ax::new(ys[0], ys[ys.len() - 1], log.1, f::Y1, f::Y0, false);
    let edge = |v: &[f64], i: usize, lg: bool| -> (f64, f64) {
        let m = |a: f64, b: f64| if lg { (a * b).sqrt() } else { (a + b) / 2.0 };
        let l = if i == 0 { v[0] } else { m(v[i - 1], v[i]) };
        let r = if i + 1 >= v.len() { v[i] } else { m(v[i], v[i + 1]) };
        (l, r)
    };
    let mut out = String::new();
    for (i, col) in vals.iter().enumerate().take(xs.len()) {
        for (j, v) in col.iter().enumerate().take(ys.len()) {
            let Some(v) = v else { continue };
            let (l, r) = edge(xs, i, log.0);
            let (b, t) = edge(ys, j, log.1);
            let (x1, x2) = (xa.px(l), xa.px(r));
            let (y1, y2) = (ya.px(t), ya.px(b));
            let op = if hi > lo { 0.08 + 0.82 * (v - lo) / (hi - lo) } else { 0.5 };
            out.push_str(&format!("<rect class=\"axp-hp\" x=\"{x1:.1}\" y=\"{y1:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
                                   fill-opacity=\"{op:.2}\"/>", (x2 - x1).abs() + 0.6, (y2 - y1).abs() + 0.6));
        }
    }
    f::frame(&mut out, Some(&xa), Some(&ya), xlab, ylab, header);
    let (px, py) = (xa.px(mine.0), ya.px(mine.1));
    out.push_str(&format!("<circle class=\"axp-k\" cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"8\"/>\
                           <circle class=\"axp-m\" cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"4\"/>"));
    // the scale: light to dark
    out.push_str(&format!("<text class=\"axp-lt\" x=\"{}\" y=\"37\" text-anchor=\"end\">light {} to dark {}</text>",
                          f::X1, f::esc(&num(lo)), f::esc(&num(hi))));
    out
}

/// Evenly spaced values (logarithmically when `log` and both ends positive).
pub fn span(lo: f64, hi: f64, n: usize, log: bool) -> Vec<f64> {
    let n = n.max(2);
    (0..n).map(|i| {
        let t = i as f64 / (n - 1) as f64;
        if log && lo > 0.0 && hi > 0.0 { lo * (hi / lo).powf(t) } else { lo + (hi - lo) * t }
    }).collect()
}

/// Panels drawn one under another, each in the generic 520 x 340 frame: the body and its height.
pub fn stack(panels: &[String]) -> (String, f64) {
    let mut out = String::new();
    for (i, p) in panels.iter().enumerate() {
        out.push_str(&format!("<g transform=\"translate(0,{})\">{}</g>", 340 * i, p.replace("CLIPID", &format!("CLIPID{i}"))));
    }
    (out, 340.0 * panels.len() as f64)
}

/// Rows of bits (a register, a code) with each bit's weight above: rows of (label, bits as '0'/'1' text); the 1s
/// shaded; `weights`: the weight written over each column of the first row (e.g. 128 ... 1, or -128 for a sign bit).
pub fn bits(rows: &[(String, String)], weights: &[String], header: &str) -> (String, f64) {
    let n = rows.iter().map(|r| r.1.chars().count()).max().unwrap_or(0).max(1);
    let label_w = 110.0;
    let cell = ((520.0 - label_w - 20.0) / n as f64).min(44.0);
    let x0 = label_w + (520.0 - label_w - 20.0 - cell * n as f64) / 2.0;
    let mut out = String::new();
    if !header.is_empty() {
        out.push_str(&format!("<text class=\"axp-mt\" x=\"16\" y=\"20\">{}</text>", f::esc(header)));
    }
    let mut y = 40.0;
    let font = (cell * 0.32).clamp(8.0, 12.0);
    if !weights.is_empty() {
        for (j, w) in weights.iter().enumerate().take(n) {
            out.push_str(&format!("<text class=\"axp-t\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"{font:.0}\">{}</text>",
                                  x0 + cell * (j as f64 + 0.5), y + 8.0, f::esc(w)));
        }
        y += 16.0;
    }
    for (label, b) in rows {
        let cs: Vec<char> = b.chars().collect();
        let off = n - cs.len();
        out.push_str(&format!("<text class=\"axp-lt\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\">{}</text>", x0 - 8.0,
                              y + cell * 0.62, f::esc(&f::clip(label, 14))));
        for (j, c) in cs.iter().enumerate() {
            let x = x0 + cell * (j + off) as f64;
            let class = if *c == '1' { "axp-b" } else { "axp-b4" };
            out.push_str(&format!("<rect class=\"{class}\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>\
                                   <text class=\"axp-bt\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
                                  cell - 3.0, cell - 3.0, x + (cell - 3.0) / 2.0, y + cell * 0.62, c));
        }
        y += cell + 8.0;
    }
    (out, (y + 10.0).max(110.0))
}

/// A table of text: a header row and rows; `mark`: a row outlined. Returns the body and its height.
pub fn table(head: &[String], rows: &[Vec<String>], mark: Option<usize>, header: &str) -> (String, f64) {
    let nc = head.len().max(1);
    let cw = (488.0 / nc as f64).min(140.0);
    let x0 = (520.0 - cw * nc as f64) / 2.0;
    let rh = 22.0;
    let mut out = String::new();
    let mut y = 30.0;
    if !header.is_empty() {
        out.push_str(&format!("<text class=\"axp-mt\" x=\"16\" y=\"20\">{}</text>", f::esc(header)));
        y = 40.0;
    }
    let chars = ((cw / 7.6) as usize).max(3);
    for (j, h) in head.iter().enumerate() {
        out.push_str(&format!("<text class=\"axp-bt\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
                              x0 + cw * (j as f64 + 0.5), y + 15.0, f::esc(&f::clip(h, chars))));
    }
    out.push_str(&format!("<line class=\"axp-g\" x1=\"{x0:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\"/>", y + rh,
                          x0 + cw * nc as f64, y + rh));
    y += rh;
    for (i, r) in rows.iter().enumerate() {
        if mark == Some(i) {
            out.push_str(&format!("<rect class=\"axp-ka\" x=\"{x0:.1}\" y=\"{y:.1}\" width=\"{:.1}\" height=\"{rh}\"/>",
                                  cw * nc as f64));
        }
        for (j, c) in r.iter().enumerate().take(nc) {
            out.push_str(&format!("<text class=\"{}\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
                                  if j == 0 { "axp-lt" } else { "axp-vt" }, x0 + cw * (j as f64 + 0.5), y + 15.0,
                                  f::esc(&f::clip(c, chars))));
        }
        y += rh;
    }
    (out, y + 12.0)
}

/// For a model's tests: every text of a figure lies inside its canvas (estimated widths: headings 9 px a character,
/// other text 7.6 px; rotated axis labels aside; a stacked panel's translate counted) and it holds no non-finite
/// number. Err names the first text that leaves it.
pub fn check_figure(fig: &Figure) -> Result<(), String> {
    let svg = &fig.svg;
    if svg.contains("NaN") || svg.contains("=\"inf") || svg.contains("inf\"") {
        return Err(format!("{}: a non-finite number in the drawing", fig.slug));
    }
    let h: f64 = svg.split("viewBox=\"0 0 520 ").nth(1).and_then(|x| x.split('"').next())
        .and_then(|x| x.parse().ok()).ok_or("no viewBox")?;
    let attr = |tag: &str, name: &str| -> Option<f64> {
        tag.split(&format!(" {name}=\"")).nth(1)?.split('"').next()?.parse().ok()
    };
    let mut offset = 0.0;
    for part in svg.split('<').skip(1) {
        if let Some(rest) = part.strip_prefix("g transform=\"translate(0,") {
            offset = rest.split(')').next().and_then(|x| x.parse().ok()).unwrap_or(0.0);
        }
        if !part.starts_with("text ") || part.contains("transform=") {
            continue;
        }
        let (Some(x), Some(y)) = (attr(part, "x"), attr(part, "y")) else { continue };
        let y = y + offset;
        let content = part.split('>').nth(1).unwrap_or("");
        let n = content.replace("&amp;", "&").replace("&lt;", "<").replace("&gt;", ">").replace("&quot;", "\"")
            .chars().count() as f64;
        let w = n * if part.contains("axp-mt") { 9.0 } else { 7.6 };
        let left = if part.contains("text-anchor=\"middle\"") { x - w / 2.0 } else if part.contains("text-anchor=\"end\"") {
            x - w
        } else {
            x
        };
        if left < -6.0 || left + w > 526.0 {
            return Err(format!("{}: text out of the canvas across: '{content}' at x = {x}", fig.slug));
        }
        if y < 8.0 || y > h + 2.0 {
            return Err(format!("{}: text out of the canvas downward: '{content}' at y = {y} (height {h})", fig.slug));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quantities_take_prefixes() {
        assert_eq!(q(0.00142402, "A"), "1.424 mA");
        assert_eq!(q(10000.0, "ohm"), "10 kΩ");
        assert_eq!(q(2200.0, "ohm"), "2.2 kΩ");
        assert_eq!(q(12.0, "V"), "12 V");
        assert_eq!(q(1e-6, "F"), "1 µF");
        assert_eq!(q(0.9999999, "V"), "1 V");
        assert_eq!(q(-2.35, "V"), "-2.35 V");
        assert_eq!(q(5.0, "dB"), "5 dB");
        assert_eq!(q(0.0, "A"), "0 A");
        assert_eq!(q(3.0, ""), "3");
    }

    #[test]
    fn canvas_keeps_text_inside() {
        let mut c = Canvas::new(200.0);
        c.value(515.0, 100.0, "start", "VCE = 5.435 V");
        c.value(2.0, 300.0, "end", "VB = 2.138 V");
        for cap in c.s.split("<text").skip(1) {
            let x: f64 = cap.split("x=\"").nth(1).unwrap().split('"').next().unwrap().parse().unwrap();
            let y: f64 = cap.split("y=\"").nth(1).unwrap().split('"').next().unwrap().parse().unwrap();
            assert!((0.0..=520.0).contains(&x) && (0.0..=200.0).contains(&y), "{cap}");
        }
    }

    #[test]
    fn primitives_draw_finite_paths() {
        let mut c = Canvas::new(300.0);
        c.resistor((10.0, 10.0), (10.0, 100.0));
        c.resistor((10.0, 10.0), (100.0, 10.0));
        c.capacitor((50.0, 50.0), (50.0, 120.0));
        c.inductor((60.0, 50.0), (160.0, 50.0));
        c.vsource((200.0, 50.0), (200.0, 150.0), false);
        c.vsource((250.0, 50.0), (250.0, 150.0), true);
        c.isource((300.0, 50.0), (300.0, 150.0));
        c.diode((320.0, 50.0), (320.0, 150.0), true);
        c.ground((350.0, 150.0));
        let _ = c.npn((400.0, 100.0));
        let _ = c.nmos((450.0, 100.0));
        let _ = c.opamp(300.0, 220.0);
        c.block(20.0, 200.0, 80.0, 40.0, "G(s)");
        c.arrow((100.0, 220.0), (150.0, 220.0));
        assert!(!c.s.contains("NaN") && !c.s.contains("inf"));
        assert!(c.s.matches("<path").count() > 20);
    }

    #[test]
    fn charts_are_drawn() {
        let s = vec![("a".to_string(), vec![(1.0, 2.0), (2.0, 3.0)]), ("b".to_string(), vec![(1.0, 1.0), (2.0, 4.0)])];
        let l = lines(&s, "x", "y", "h", Some((1.5, 2.5)), "here", false);
        assert!(l.contains("axp-c2") && l.contains("here"));
        let (b, h) = bits(&[("binary".into(), "1011".into())], &["8".into(), "4".into(), "2".into(), "1".into()], "x");
        assert!(b.matches("axp-b\"").count() == 3 && h >= 110.0);
        let fd = field(&[1.0, 2.0, 3.0], &[1.0, 2.0], &[vec![Some(1.0), Some(2.0)], vec![Some(3.0), None],
                                                         vec![Some(5.0), Some(6.0)]], (2.0, 1.5), "x", "y", "h",
                       (false, false));
        assert_eq!(fd.matches("axp-hp").count(), 5);
        let (t, _) = table(&["a".into(), "b".into()], &[vec!["1".into(), "2".into()]], Some(0), "");
        assert!(t.contains("axp-ka"));
        let (st, h) = stack(&[l.clone(), l]);
        assert_eq!(h, 680.0);
        assert!(st.contains("CLIPID1"));
    }

    #[test]
    fn spans() {
        assert_eq!(span(0.0, 1.0, 3, false), vec![0.0, 0.5, 1.0]);
        let s = span(1.0, 100.0, 3, true);
        assert!((s[1] - 10.0).abs() < 1e-12);
    }
}

// ================================================================================================================
// ADDED BY THE FIGURE PARITY WAVE, GROUP E (Bayesian Statistics, Optimization, Discrete Math, Industrial Engineering)
// Generic primitives a model crate cannot build from the kit above: a free plot frame in data coordinates (shaded
// areas, intervals, bands, bars of a schedule, regions, ellipses, stems), and a network drawing.
// ================================================================================================================

/// A free chart in the generic 520 x 340 frame: axes with ticks, then marks placed in data coordinates (clipped to
/// the plotting area). For drawings no fixed chart covers: a density with its shaded interval, credible intervals, a
/// Gantt schedule, a feasible region, an ellipse.
///
///   let mut p = Plot::new((0.0, 1.0), (0.0, 3.0), "p", "density", "header", false, false);
///   p.area(&pts, 0.0, "axp-s");  p.path(&pts, "axp-c");  p.mark((x, y), "x = 0.3");  let body = p.body();
pub struct Plot {
    xa: f::Ax,
    ya: f::Ax,
    under: String,
    fill: String,
    out: String,
    over: String,
}

impl Plot {
    /// Axes over these ranges (padded slightly when `pad`); logarithmic when asked (positive ranges).
    #[allow(clippy::too_many_arguments)]
    pub fn new(xr: (f64, f64), yr: (f64, f64), xlab: &str, ylab: &str, header: &str, log: (bool, bool), pad: bool)
               -> Plot {
        let xa = f::Ax::new(xr.0, xr.1, log.0, f::X0, f::X1, pad);
        let ya = f::Ax::new(yr.0, yr.1, log.1, f::Y1, f::Y0, pad);
        let mut under = String::new();
        f::frame(&mut under, Some(&xa), Some(&ya), xlab, ylab, header);
        Plot { xa, ya, under, fill: String::new(), out: String::new(), over: String::new() }
    }
    /// A frame without ticks on the vertical axis: `ylabels` written at their data heights instead (rows of a
    /// schedule, the names of intervals).
    #[allow(clippy::too_many_arguments)]
    pub fn rows(xr: (f64, f64), ylabels: &[String], xlab: &str, header: &str, pad: bool) -> Plot {
        let n = ylabels.len().max(1) as f64;
        let xa = f::Ax::new(xr.0, xr.1, false, f::X0, f::X1, pad);
        let ya = f::Ax::new(-0.5, n - 0.5, false, f::Y0, f::Y1, false);
        let mut under = String::new();
        f::frame(&mut under, Some(&xa), None, xlab, "", header);
        for (i, l) in ylabels.iter().enumerate() {
            under.push_str(&format!("<text class=\"axp-lt\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\">{}</text>",
                                    f::X0 - 6.0, ya.px(i as f64) + 4.5, f::esc(&f::clip(l, 11))));
        }
        Plot { xa, ya, under, fill: String::new(), out: String::new(), over: String::new() }
    }
    /// Pixel position of a data point.
    pub fn px(&self, p: P) -> P {
        (self.xa.px(p.0), self.ya.px(p.1))
    }
    fn ok(&self, p: P) -> bool {
        let q = self.px(p);
        q.0.is_finite() && q.1.is_finite()
    }
    /// A polyline through data points (a non-finite point breaks it); class axp-c .. axp-c4, axp-k.
    pub fn path(&mut self, pts: &[P], class: &str) {
        let mut d = String::new();
        let mut pen = false;
        for p in pts {
            if p.0.is_finite() && p.1.is_finite() && self.ok(*p) {
                let q = self.px(*p);
                d.push_str(&format!("{}{:.1},{:.1} ", if pen { "L" } else { "M" }, q.0, q.1));
                pen = true;
            } else {
                pen = false;
            }
        }
        if !d.is_empty() {
            self.out.push_str(&format!("<path class=\"{class}\" d=\"{}\"/>", d.trim_end()));
        }
    }
    /// A dashed polyline (a reference, a limit).
    pub fn dashed(&mut self, pts: &[P], class: &str) {
        let n = self.out.len();
        self.path(pts, class);
        if self.out.len() > n {
            self.out.insert_str(n + 5, " stroke-dasharray=\"6 4\"");
        }
    }
    /// The area between a curve and the height `base` (a shaded interval of a density); class axp-s .. axp-s4.
    pub fn area(&mut self, pts: &[P], base: f64, class: &str) {
        let pts: Vec<P> = pts.iter().copied().filter(|p| p.0.is_finite() && p.1.is_finite() && self.ok(*p)).collect();
        if pts.len() < 2 {
            return;
        }
        let mut d = String::new();
        let b0 = self.px((pts[0].0, base));
        d.push_str(&format!("M{:.1},{:.1} ", b0.0, b0.1));
        for p in &pts {
            let q = self.px(*p);
            d.push_str(&format!("L{:.1},{:.1} ", q.0, q.1));
        }
        let b1 = self.px((pts[pts.len() - 1].0, base));
        d.push_str(&format!("L{:.1},{:.1} Z", b1.0, b1.1));
        self.fill.push_str(&format!("<path class=\"{class}\" d=\"{d}\"/>"));
    }
    /// A closed polygon in data coordinates (a feasible region, an ellipse); fill class (axp-s .., axp-ka) and an
    /// optional outline class.
    pub fn polygon(&mut self, pts: &[P], fill: &str, outline: &str) {
        let q: Vec<P> = pts.iter().copied().filter(|p| self.ok(*p)).map(|p| self.px(p)).collect();
        if q.len() < 3 {
            return;
        }
        let d: String = q.iter().enumerate()
            .map(|(i, p)| format!("{}{:.1},{:.1} ", if i == 0 { "M" } else { "L" }, p.0, p.1)).collect();
        if !fill.is_empty() {
            self.fill.push_str(&format!("<path class=\"{fill}\" d=\"{}Z\"/>", d));
        }
        if !outline.is_empty() {
            self.out.push_str(&format!("<path class=\"{outline}\" d=\"{}Z\"/>", d));
        }
    }
    /// A rectangle between two data corners (a bar of a schedule, an interval as a bar); class axp-b .., axp-ka; a
    /// label written inside it when it fits.
    pub fn rect(&mut self, a: P, b: P, class: &str, label: &str) {
        let (p, q) = (self.px(a), self.px(b));
        if !(p.0.is_finite() && p.1.is_finite() && q.0.is_finite() && q.1.is_finite()) {
            return;
        }
        let (x, y, w, h) = (p.0.min(q.0), p.1.min(q.1), (p.0 - q.0).abs(), (p.1 - q.1).abs());
        self.out.push_str(&format!("<rect class=\"{class}\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{w:.1}\" height=\"{h:.1}\"/>"));
        if !label.is_empty() && text_w(label, false) + 4.0 <= w {
            self.over.push_str(&format!("<text class=\"axp-bt\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
                                        x + w / 2.0, y + h / 2.0 + 4.5, f::esc(label)));
        }
    }
    /// A straight segment between two data points (a whisker, an interval).
    pub fn seg(&mut self, a: P, b: P, class: &str) {
        self.path(&[a, b], class);
    }
    /// A dot at a data point; class axp-d .. axp-d4.
    pub fn dot(&mut self, p: P, r: f64, class: &str) {
        if self.ok(p) {
            let q = self.px(p);
            self.out.push_str(&format!("<circle class=\"{class}\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"{r:.1}\"/>", q.0, q.1));
        }
    }
    /// Stems from height 0 to each value with a dot on top (a probability mass function); class axp-d ...
    pub fn stems(&mut self, pts: &[P], class: &str) {
        for p in pts {
            if self.ok(*p) && self.ok((p.0, 0.0)) {
                let (q, b) = (self.px(*p), self.px((p.0, 0.0)));
                self.out.push_str(&format!("<line class=\"axp-k\" x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\"/>\
                                            <circle class=\"{class}\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"3.6\"/>",
                                           b.0, b.1, q.0, q.1, q.0, q.1));
            }
        }
    }
    /// Text at a data point moved by (dx, dy) pixels, kept inside the plotting area; class axp-lt, axp-vt, axp-bt.
    pub fn label(&mut self, p: P, dx: f64, dy: f64, anchor: &str, class: &str, t: &str) {
        if !self.ok(p) || t.is_empty() {
            return;
        }
        let q = self.px(p);
        let w = text_w(t, class == "axp-vt" || class == "axp-mt");
        let (x, y) = (q.0 + dx, q.1 + dy);
        let left = match anchor {
            "middle" => x - w / 2.0,
            "end" => x - w,
            _ => x,
        };
        let shift = if left < f::X0 + 2.0 { f::X0 + 2.0 - left } else if left + w > f::X1 - 2.0 {
            f::X1 - 2.0 - (left + w)
        } else {
            0.0
        };
        let y = y.clamp(f::Y0 + 13.0, f::Y1 - 4.0);
        self.over.push_str(&format!("<text class=\"{class}\" x=\"{:.1}\" y=\"{y:.1}\" text-anchor=\"{anchor}\">{}</text>",
                                    x + shift, f::esc(t)));
    }
    /// The visitor's point: a ringed marker and its label.
    pub fn mark(&mut self, p: P, label: &str) {
        if !self.ok(p) {
            return;
        }
        let q = self.px(p);
        self.over.push_str(&format!("<circle class=\"axp-m\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"6\"/>", q.0, q.1));
        let right = q.0 + 10.0 + text_w(label, true) <= f::X1;
        self.label(p, if right { 10.0 } else { -10.0 }, -9.0, if right { "start" } else { "end" }, "axp-mt", label);
    }
    /// A vertical line at x across the plot (a threshold, a decision boundary), dashed, with a label at the top.
    pub fn vline(&mut self, x: f64, class: &str, label: &str) {
        let px = self.xa.px(x);
        if !px.is_finite() || px < f::X0 - 0.5 || px > f::X1 + 0.5 {
            return;
        }
        self.out.push_str(&format!("<path class=\"{class}\" stroke-dasharray=\"6 4\" d=\"M{px:.1},{} L{px:.1},{}\"/>",
                                   f::Y0, f::Y1));
        if !label.is_empty() {
            let right = px + 6.0 + text_w(label, false) <= f::X1;
            let t = f::esc(label);
            self.over.push_str(&format!("<text class=\"axp-lt\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"{}\">{t}</text>",
                                        if right { px + 5.0 } else { px - 5.0 }, f::Y0 + 14.0,
                                        if right { "start" } else { "end" }));
        }
    }
    /// A legend at the top right: (name, fill class such as axp-b, axp-b2, axp-s).
    pub fn legend(&mut self, names: &[(String, &str)]) {
        let mut x = f::X1;
        for (n, class) in names.iter().filter(|n| !n.0.is_empty()).rev().take(4) {
            let t = f::clip(n, 22);
            let w = 7.0 * t.chars().count() as f64 + 30.0;
            x -= w;
            self.over.push_str(&format!("<rect class=\"{class}\" x=\"{x:.1}\" y=\"28\" width=\"12\" height=\"10\"/>\
                                         <text class=\"axp-lt\" x=\"{:.1}\" y=\"37\">{}</text>", x + 16.0, f::esc(&t)));
        }
    }
    /// The drawing: shaded areas under the marks, clipped to the plotting area, labels on top.
    pub fn body(&self) -> String {
        format!("{}<clipPath id=\"CLIPID\"><rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"/></clipPath>\
                 <g clip-path=\"url(#CLIPID)\">{}{}</g>{}", self.under, f::X0, f::Y0, f::X1 - f::X0, f::Y1 - f::Y0,
                self.fill, self.out, self.over)
    }
}

/// A network with some edges highlighted (a path, a tree, a matching, a flow): the highlighted edges drawn thick in
/// the accent colour over the others; directed edges get an arrowhead at their second node. Nodes on a circle or at
/// `pos`; `marked` nodes filled; `ringed` ringed; `note` per node under it. Returns the 520 x 340 body.
#[allow(clippy::too_many_arguments)]
pub fn network(nodes: &[String], edges: &[(String, String, Option<String>)], hot: &[bool], directed: bool,
               pos: Option<&[(f64, f64)]>, marked: &[String], ringed: &[String], note: &[String], header: &str) -> String {
    let n = nodes.len();
    if n == 0 {
        return String::new();
    }
    let xy: Vec<(f64, f64)> = match pos {
        Some(p) if p.len() == n => {
            let xs: Vec<f64> = p.iter().map(|q| q.0).collect();
            let ys: Vec<f64> = p.iter().map(|q| q.1).collect();
            let (xa, ya) = f::equal_axes(&xs, &ys);
            p.iter().map(|q| (xa.px(q.0), ya.px(q.1))).collect()
        }
        _ => {
            let (cxp, cyp, r) = ((f::X0 + f::X1) / 2.0 - 30.0, (f::Y0 + f::Y1) / 2.0 + 8.0, (f::Y1 - f::Y0) / 2.0 - 4.0);
            (0..n).map(|i| {
                let a = -std::f64::consts::FRAC_PI_2 + 2.0 * std::f64::consts::PI * i as f64 / n as f64;
                (cxp + r * 1.45 * a.cos(), cyp + r * a.sin())
            }).collect()
        }
    };
    let rad = if n > 16 { 9.0 } else { 13.0 };
    let at = |s: &str| nodes.iter().position(|x| x == s).map(|i| xy[i]);
    let mut out = String::new();
    if !header.is_empty() {
        out.push_str(&format!("<text class=\"axp-mt\" x=\"16\" y=\"20\">{}</text>", f::esc(header)));
    }
    let mut labels = String::new();
    for pass in [false, true] {
        for (i, (u, v, w)) in edges.iter().enumerate() {
            let h = hot.get(i).copied().unwrap_or(false);
            if h != pass {
                continue;
            }
            let (Some(a), Some(b)) = (at(u), at(v)) else { continue };
            let (dx, dy) = (b.0 - a.0, b.1 - a.1);
            let d = (dx * dx + dy * dy).sqrt().max(1e-9);
            let (ux, uy) = (dx / d, dy / d);
            let a2 = (a.0 + ux * rad, a.1 + uy * rad);
            let b2 = (b.0 - ux * (rad + if directed { 2.0 } else { 0.0 }), b.1 - uy * (rad + if directed { 2.0 } else { 0.0 }));
            let class = if h { "axp-c" } else { "axp-k" };
            out.push_str(&format!("<path class=\"{class}\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>", a2.0, a2.1, b2.0, b2.1));
            if directed {
                let (bx, by) = (b2.0 - ux * 9.0, b2.1 - uy * 9.0);
                out.push_str(&format!("<path class=\"{}\" d=\"M{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} Z\"/>",
                                      if h { "axp-d" } else { "axp-kd" }, b2.0, b2.1, bx - uy * 4.5, by + ux * 4.5,
                                      bx + uy * 4.5, by - ux * 4.5));
            }
            if let Some(w) = w {
                if edges.len() <= 40 && !w.is_empty() {
                    // the weight beside the edge's middle, off the line
                    let (mx, my) = ((a.0 + b.0) / 2.0 - uy * 9.0, (a.1 + b.1) / 2.0 + ux * 9.0 + 4.0);
                    labels.push_str(&format!("<text class=\"axp-vt\" x=\"{mx:.1}\" y=\"{my:.1}\" text-anchor=\"middle\">{}</text>",
                                             f::esc(&f::clip(w, 12))));
                }
            }
        }
    }
    for (i, s) in nodes.iter().enumerate() {
        let (x, y) = xy[i];
        let class = if marked.contains(s) { "axp-d" } else { "axp-d4" };
        if ringed.contains(s) {
            out.push_str(&format!("<circle class=\"axp-k\" cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"{:.1}\"/>", rad + 5.0));
        }
        out.push_str(&format!("<circle class=\"{class}\" cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"{rad}\"/>\
                               <text class=\"axp-bt\" x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
                              y + 4.5, f::esc(&f::clip(s, 4))));
        if let Some(t) = note.get(i) {
            if !t.is_empty() {
                let tx = x.clamp(8.0 + text_w(t, true) / 2.0, 512.0 - text_w(t, true) / 2.0);
                labels.push_str(&format!("<text class=\"axp-vt\" x=\"{tx:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
                                         (y + rad + 15.0).min(334.0), f::esc(t)));
            }
        }
    }
    out.push_str(&labels);
    out
}

#[cfg(test)]
mod group_e_tests {
    use super::*;

    #[test]
    fn a_free_plot_and_a_network_are_drawn() {
        let mut p = Plot::new((0.0, 1.0), (0.0, 2.0), "x", "y", "h", (false, false), false);
        let pts: Vec<P> = (0..11).map(|i| (i as f64 / 10.0, 2.0 * i as f64 / 10.0)).collect();
        p.area(&pts[2..6], 0.0, "axp-s");
        p.path(&pts, "axp-c");
        p.dashed(&pts, "axp-c4");
        p.stems(&pts, "axp-d");
        p.rect((0.1, 0.2), (0.5, 0.6), "axp-b", "a long label here");
        p.vline(0.4, "axp-c4", "x = 0.4");
        p.mark((0.3, 0.6), "x = 0.3");
        p.legend(&[("a".into(), "axp-b")]);
        let b = p.body();
        assert!(b.contains("axp-s") && b.contains("stroke-dasharray") && b.contains("x = 0.3") && !b.contains("NaN"));
        let r = Plot::rows((0.0, 10.0), &["J1".into(), "J2".into()], "time", "", false);
        assert!(r.body().contains(">J2<"));
        let nodes: Vec<String> = ["A", "B", "C"].iter().map(|s| s.to_string()).collect();
        let edges = vec![("A".to_string(), "B".to_string(), Some("4".to_string())), ("B".into(), "C".into(), None)];
        let g = network(&nodes, &edges, &[true, false], true, None, &["A".into()], &[], &["0".into()], "path");
        assert!(g.contains("axp-c") && g.contains(">4<") && g.matches("<circle").count() == 3);
    }
}

// ================================================================================================================
// Added by the figure parity wave, group C (2026-11-02): a plot in data coordinates, for figures the fixed charts
// above cannot draw (an area under a curve, a solid of revolution, a polar region, a phase portrait, a cobweb, a
// polygon with its points). Generic: axes and grid as every chart (the 520 x 340 frame), everything drawn in the
// visitor's units and clipped to the plot box; labels kept inside the canvas.
// ================================================================================================================

/// A chart in data coordinates: `DPlot::new` (free scales) or `DPlot::equal` (equal scales, shapes to scale), then
/// curves, filled regions, segments, arrows, dots and labels in data units; `body()` gives the drawing for
/// `Solved::chart`.
pub struct DPlot {
    xa: f::Ax,
    ya: f::Ax,
    under: String,
    clip: String,
    over: String,
    xlab: String,
    ylab: String,
    header: String,
    legend: Vec<(String, usize)>,
}

const PLOT_LINE: [&str; 4] = ["axp-c", "axp-c2", "axp-c3", "axp-c4"];
const PLOT_FILL: [&str; 4] = ["axp-s", "axp-s2", "axp-s3", "axp-s4"];
const PLOT_DOT: [&str; 4] = ["axp-d", "axp-d2", "axp-d3", "axp-d4"];

impl DPlot {
    /// Free scales over x in xr and y in yr (a little margin added); `log`: logarithmic axes (positive ranges).
    pub fn new(xr: (f64, f64), yr: (f64, f64), xlab: &str, ylab: &str, header: &str) -> DPlot {
        DPlot::with_log(xr, yr, (false, false), xlab, ylab, header)
    }
    pub fn with_log(xr: (f64, f64), yr: (f64, f64), log: (bool, bool), xlab: &str, ylab: &str, header: &str) -> DPlot {
        let lx = log.0 && xr.0 > 0.0 && xr.1 > xr.0;
        let ly = log.1 && yr.0 > 0.0 && yr.1 > yr.0;
        let xa = f::Ax::new(xr.0, xr.1, lx, f::X0, f::X1, !lx);
        let ya = f::Ax::new(yr.0, yr.1, ly, f::Y1, f::Y0, !ly);
        DPlot::of(xa, ya, xlab, ylab, header)
    }
    /// Equal scales on both axes, every point of `xs` and `ys` inside (a circle stays round).
    pub fn equal(xs: &[f64], ys: &[f64], xlab: &str, ylab: &str, header: &str) -> DPlot {
        let xs: Vec<f64> = xs.iter().copied().filter(|v| v.is_finite()).collect();
        let ys: Vec<f64> = ys.iter().copied().filter(|v| v.is_finite()).collect();
        let (xs, ys) = if xs.is_empty() || ys.is_empty() { (vec![-1.0, 1.0], vec![-1.0, 1.0]) } else { (xs, ys) };
        let (xa, ya) = f::equal_axes(&xs, &ys);
        DPlot::of(xa, ya, xlab, ylab, header)
    }
    fn of(xa: f::Ax, ya: f::Ax, xlab: &str, ylab: &str, header: &str) -> DPlot {
        DPlot { xa, ya, under: String::new(), clip: String::new(), over: String::new(), xlab: xlab.into(),
               ylab: ylab.into(), header: header.into(), legend: vec![] }
    }
    /// A data point in pixels.
    pub fn px(&self, x: f64, y: f64) -> P {
        (self.xa.px(x), self.ya.px(y))
    }
    /// The data ranges the plot box shows: ((x at the left, x at the right), (y at the bottom, y at the top)).
    pub fn ranges(&self) -> ((f64, f64), (f64, f64)) {
        let inv = |ax: &f::Ax, a: f64, b: f64| -> (f64, f64) {
            // the axis is linear (or logarithmic) in pixels: two probes give its inverse
            let (p0, p1) = (ax.px(0.0), ax.px(1.0));
            if (p1 - p0).abs() > 0.0 && p0.is_finite() && p1.is_finite() {
                ((a - p0) / (p1 - p0), (b - p0) / (p1 - p0))
            } else {
                let (q1, q10) = (ax.px(1.0), ax.px(10.0));
                let d = q10 - q1;
                (10f64.powf((a - q1) / d), 10f64.powf((b - q1) / d))
            }
        };
        (inv(&self.xa, f::X0, f::X1), inv(&self.ya, f::Y1, f::Y0))
    }
    fn path(&self, pts: &[(f64, f64)]) -> String {
        let mut d = String::new();
        let mut pen = false;
        for (x, y) in pts {
            let (px, py) = self.px(*x, *y);
            if !px.is_finite() || !py.is_finite() || px.abs() > 1e5 || py.abs() > 1e5 {
                pen = false;
                continue;
            }
            d.push_str(&format!("{}{px:.1},{py:.1} ", if pen { "L" } else { "M" }));
            pen = true;
        }
        d.trim_end().to_string()
    }
    /// A curve in colour k (0 accent, 1 green, 2 violet, 3 grey); a non-finite point breaks it.
    pub fn curve(&mut self, pts: &[(f64, f64)], k: usize) {
        let d = self.path(pts);
        if !d.is_empty() {
            self.clip.push_str(&format!("<path class=\"{}\" d=\"{d}\"/>", PLOT_LINE[k % 4]));
        }
    }
    /// A dashed curve in colour k.
    pub fn dashed(&mut self, pts: &[(f64, f64)], k: usize) {
        let d = self.path(pts);
        if !d.is_empty() {
            self.clip.push_str(&format!("<path class=\"{}\" stroke-dasharray=\"6 4\" d=\"{d}\"/>", PLOT_LINE[k % 4]));
        }
    }
    /// A closed region (a polygon through the points) shaded in colour k, under the curves.
    pub fn fill(&mut self, pts: &[(f64, f64)], k: usize) {
        let d = self.path(pts);
        if !d.is_empty() && !d.contains(" M") {
            self.under.push_str(&format!("<path class=\"{}\" d=\"{d} Z\"/>", PLOT_FILL[k % 4]));
        }
    }
    /// A closed outline in colour k (a polygon's edges).
    pub fn outline(&mut self, pts: &[(f64, f64)], k: usize) {
        let d = self.path(pts);
        if !d.is_empty() && !d.contains(" M") {
            self.clip.push_str(&format!("<path class=\"{}\" d=\"{d} Z\"/>", PLOT_LINE[k % 4]));
        }
    }
    /// A straight segment (colour k), dashed when asked.
    pub fn seg(&mut self, a: (f64, f64), b: (f64, f64), k: usize, dash: bool) {
        if dash { self.dashed(&[a, b], k) } else { self.curve(&[a, b], k) }
    }
    /// A full-width horizontal or full-height vertical guide (dashed grey).
    pub fn hline(&mut self, y: f64) {
        let ((x0, x1), _) = self.ranges();
        self.dashed(&[(x0, y), (x1, y)], 3);
    }
    pub fn vline(&mut self, x: f64) {
        let (_, (y0, y1)) = self.ranges();
        self.dashed(&[(x, y0), (x, y1)], 3);
    }
    /// An arrow from a to b in data units (colour k); `head` in pixels (0: the usual 8).
    pub fn arrow(&mut self, a: (f64, f64), b: (f64, f64), k: usize) {
        let (pa, pb) = (self.px(a.0, a.1), self.px(b.0, b.1));
        let (dx, dy) = (pb.0 - pa.0, pb.1 - pa.1);
        let d = (dx * dx + dy * dy).sqrt();
        if !(d > 0.5) || !d.is_finite() {
            return;
        }
        let (u, n) = ((dx / d, dy / d), (-dy / d, dx / d));
        let hl = 8.0f64.min(d * 0.5);
        let base = (pb.0 - u.0 * hl, pb.1 - u.1 * hl);
        self.clip.push_str(&format!("<path class=\"{}\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>", PLOT_LINE[k % 4], pa.0, pa.1,
                                    base.0, base.1));
        self.clip.push_str(&format!("<path class=\"{}\" d=\"M{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} Z\"/>", PLOT_DOT[k % 4],
                                    pb.0, pb.1, base.0 + n.0 * hl * 0.45, base.1 + n.1 * hl * 0.45,
                                    base.0 - n.0 * hl * 0.45, base.1 - n.1 * hl * 0.45));
    }
    /// A dot (colour k, radius r pixels).
    pub fn dot(&mut self, x: f64, y: f64, k: usize, r: f64) {
        let (px, py) = self.px(x, y);
        if px.is_finite() && py.is_finite() {
            self.clip.push_str(&format!("<circle class=\"{}\" cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"{r:.1}\"/>", PLOT_DOT[k % 4]));
        }
    }
    /// A rectangle between x0..x1 and y0..y1 (a Riemann bar), shaded in colour k and outlined.
    pub fn rect(&mut self, x0: f64, x1: f64, y0: f64, y1: f64, k: usize) {
        let pts = [(x0, y0), (x1, y0), (x1, y1), (x0, y1)];
        self.fill(&pts, k);
        let d = self.path(&pts);
        if !d.is_empty() {
            self.clip.push_str(&format!("<path class=\"axp-c4\" d=\"{d} Z\"/>"));
        }
    }
    /// A marked point (the visitor's) with its label beside it.
    pub fn mark(&mut self, x: f64, y: f64, label: &str) {
        let (px, py) = self.px(x, y);
        if !px.is_finite() || !py.is_finite() || px < f::X0 - 1.0 || px > f::X1 + 1.0 || py < f::Y0 - 1.0 || py > f::Y1 + 1.0 {
            return;
        }
        self.over.push_str(&format!("<circle class=\"axp-k\" cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"8\"/>\
                                     <circle class=\"axp-m\" cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"4\"/>"));
        if !label.is_empty() {
            let t = f::clip(label, 34);
            let w = text_w(&t, true) * 1.1;
            let (tx, anchor) = if px + 12.0 + w <= 512.0 { (px + 12.0, "start") } else { (px - 12.0, "end") };
            let ty = if py - 10.0 < f::Y0 + 12.0 { py + 22.0 } else { py - 10.0 };
            self.over.push_str(&format!("<text class=\"axp-mt\" x=\"{tx:.1}\" y=\"{ty:.1}\" text-anchor=\"{anchor}\">{}</text>",
                                        f::esc(&t)));
        }
    }
    /// A text label at a data point (class: "axp-lt" name, "axp-vt" value, "axp-bt" bold), kept inside the canvas.
    pub fn label(&mut self, x: f64, y: f64, anchor: &str, class: &str, t: &str) {
        let (px, py) = self.px(x, y);
        if !px.is_finite() || !py.is_finite() || t.is_empty() {
            return;
        }
        let mono = class == "axp-vt" || class == "axp-mt";
        let w = text_w(t, mono) * if class == "axp-mt" { 1.2 } else { 1.06 };
        let left = match anchor {
            "middle" => px - w / 2.0,
            "end" => px - w,
            _ => px,
        };
        let shift = if left < 4.0 { 4.0 - left } else if left + w > 516.0 { 516.0 - (left + w) } else { 0.0 };
        let py = py.clamp(f::Y0 + 12.0, 334.0);
        self.over.push_str(&format!("<text class=\"{class}\" x=\"{:.1}\" y=\"{py:.1}\" text-anchor=\"{anchor}\">{}</text>",
                                    px + shift, f::esc(t)));
    }
    /// A legend entry (drawn above the plot, right-aligned): a name in colour k.
    pub fn key(&mut self, name: &str, k: usize) {
        self.legend.push((name.to_string(), k));
    }
    /// Raw SVG in pixels, inside the clipped plot box.
    pub fn raw(&mut self, svg: &str) {
        self.clip.push_str(svg);
    }
    /// The drawing: grid and axes, shaded regions, curves (clipped to the box), marks and labels, the legend.
    pub fn body(&self) -> String {
        let mut out = String::new();
        f::frame(&mut out, Some(&self.xa), Some(&self.ya), &self.xlab, &self.ylab, &self.header);
        out.push_str(&format!("<clipPath id=\"CLIPID\"><rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"/></clipPath>\
                               <g clip-path=\"url(#CLIPID)\">{}{}</g>", f::X0, f::Y0, f::X1 - f::X0, f::Y1 - f::Y0,
                              self.under, self.clip));
        out.push_str(&self.over);
        let mut x = f::X1;
        for (n, k) in self.legend.iter().rev().take(4) {
            let t = f::clip(n, 24);
            let w = 7.0 * t.chars().count() as f64 + 26.0;
            x -= w;
            out.push_str(&format!("<path class=\"{}\" d=\"M{:.1},33 L{:.1},33\"/><text class=\"axp-lt\" x=\"{:.1}\" y=\"37\">{}</text>",
                                  PLOT_LINE[k % 4], x, x + 14.0, x + 18.0, f::esc(&t)));
        }
        out
    }
}

#[cfg(test)]
mod plot_tests {
    use super::*;

    #[test]
    fn a_plot_draws_in_data_units() {
        let mut p = DPlot::new((0.0, 2.0), (0.0, 4.0), "x", "y", "h");
        let c: Vec<(f64, f64)> = (0..=20).map(|i| (i as f64 * 0.1, (i as f64 * 0.1).powi(2))).collect();
        p.curve(&c, 0);
        p.fill(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0)], 1);
        p.rect(0.0, 0.5, 0.0, 0.25, 2);
        p.arrow((0.0, 0.0), (1.0, 1.0), 0);
        p.mark(1.0, 1.0, "x = 1");
        p.key("f", 0);
        let ((x0, x1), (y0, y1)) = p.ranges();
        assert!(x0 < 0.0 && x1 > 2.0 && y0 < 0.0 && y1 > 4.0);
        let b = p.body();
        assert!(b.contains("axp-s2") && b.contains("axp-m") && !b.contains("NaN"));
        let q = DPlot::equal(&[0.0, 1.0], &[0.0, 1.0], "x", "y", "");
        let (a, b2) = (q.px(0.0, 0.0), q.px(1.0, 1.0));
        assert!(((b2.0 - a.0) - (a.1 - b2.1)).abs() < 1e-6, "equal scales");
    }
}
