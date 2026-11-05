//! Python semantics that procedures translated from the packages' Python (tools/py2rust.py) rely on: number formatting
//! as in f-strings, the 12-significant-digit rounding of outputs, floor division and modulo (rounded toward minus
//! infinity, unlike Rust's), round() (half to even on the exact value) and str() of a float.

pub use crate::num::{py_g, py_repr_f64 as repr, py_repr_str};

/// float(f"{x:.12g}") + 0.0: the outputs' 12 significant digits (never a negative zero; non-finite left as is).
pub fn r12(x: f64) -> f64 {
    if x == 0.0 || !x.is_finite() {
        return if x == 0.0 { 0.0 } else { x };
    }
    py_g(x, 12).parse::<f64>().unwrap_or(x) + 0.0
}

/// float(f"{x:.{p}g}")
pub fn round_sig(x: f64, p: usize) -> f64 {
    if !x.is_finite() {
        return x;
    }
    py_g(x, p).parse::<f64>().unwrap_or(x)
}

/// Python's a // b for floats.
pub fn floordiv(a: f64, b: f64) -> f64 {
    let m = pymod(a, b);
    let q = (a - m) / b;
    q.round()
}

/// Python's a % b for floats: the result has the sign of b.
pub fn pymod(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        return f64::NAN;
    }
    let m = a % b;
    if m != 0.0 && ((m < 0.0) != (b < 0.0)) { m + b } else { m + 0.0 }
}

/// Python's a // b and a % b for integers.
pub fn ifloordiv(a: i64, b: i64) -> i64 {
    a.div_euclid(b) - if b < 0 && a.rem_euclid(b) != 0 { 1 } else { 0 }
}

pub fn imod(a: i64, b: i64) -> i64 {
    a - b * ifloordiv(a, b)
}

/// Python's round(x, n): the exact binary value rounded to n decimals, ties to even.
pub fn round_n(x: f64, n: i32) -> f64 {
    if !x.is_finite() {
        return x;
    }
    if n >= 0 {
        format!("{:.*}", n as usize, x).parse::<f64>().unwrap_or(x)
    } else {
        let p = 10f64.powi(-n);
        round_half_even(x / p) * p
    }
}

/// Python's round(x) for a float: the nearest whole number, ties to even.
pub fn round_half_even(x: f64) -> f64 {
    x.round_ties_even()
}

/// Python's format-spec mini-language: [[fill]align][sign][z][#][0][width][,|_][.precision][type].
#[derive(Debug, Clone)]
pub struct Spec {
    pub fill: char,
    pub align: Option<char>,
    pub sign: char,
    pub alt: bool,
    pub zero: bool,
    pub width: usize,
    pub group: Option<char>,
    pub prec: Option<usize>,
    pub ty: String,
}

/// Parse a format spec; None for one this module does not read (the caller keeps its old path).
pub fn parse_spec(spec: &str) -> Option<Spec> {
    let c: Vec<char> = spec.chars().collect();
    let mut i = 0;
    let mut sp = Spec { fill: ' ', align: None, sign: '-', alt: false, zero: false, width: 0, group: None, prec: None,
                        ty: String::new() };
    let is_align = |ch: char| matches!(ch, '<' | '>' | '=' | '^');
    if c.len() >= 2 && is_align(c[1]) {
        sp.fill = c[0];
        sp.align = Some(c[1]);
        i = 2;
    } else if !c.is_empty() && is_align(c[0]) {
        sp.align = Some(c[0]);
        i = 1;
    }
    if i < c.len() && matches!(c[i], '+' | '-' | ' ') {
        sp.sign = c[i];
        i += 1;
    }
    if i < c.len() && c[i] == 'z' {
        i += 1;
    }
    if i < c.len() && c[i] == '#' {
        sp.alt = true;
        i += 1;
    }
    if i < c.len() && c[i] == '0' {
        sp.zero = true;
        i += 1;
    }
    let st = i;
    while i < c.len() && c[i].is_ascii_digit() {
        i += 1;
    }
    if i > st {
        // a width past 1 MiB is refused rather than allocated
        sp.width = c[st..i].iter().collect::<String>().parse().ok().filter(|w: &usize| *w <= 1 << 20)?;
    }
    if i < c.len() && matches!(c[i], ',' | '_') {
        sp.group = Some(c[i]);
        i += 1;
    }
    if i < c.len() && c[i] == '.' {
        i += 1;
        let st = i;
        while i < c.len() && c[i].is_ascii_digit() {
            i += 1;
        }
        if i == st {
            return None;
        }
        sp.prec = Some(c[st..i].iter().collect::<String>().parse().ok()?);
    }
    let ty: String = c[i..].iter().collect();
    if ty.chars().count() > 1 {
        return None;
    }
    sp.ty = ty;
    if sp.zero && sp.align.is_none() {
        sp.fill = '0';
        sp.align = Some('=');
    }
    Some(sp)
}

/// Pad sign + prefix + digits to the spec's width with its fill and alignment (Python's rules; '=' pads after the
/// sign and prefix).
fn pad(sp: &Spec, sign: &str, prefix: &str, digits: &str, default_align: char) -> String {
    let len = sign.chars().count() + prefix.chars().count() + digits.chars().count();
    if len >= sp.width {
        return format!("{sign}{prefix}{digits}");
    }
    let n = sp.width - len;
    let f = |k: usize| std::iter::repeat(sp.fill).take(k).collect::<String>();
    match sp.align.unwrap_or(default_align) {
        '<' => format!("{sign}{prefix}{digits}{}", f(n)),
        '^' => format!("{}{sign}{prefix}{digits}{}", f(n / 2), f(n - n / 2)),
        '=' => format!("{sign}{prefix}{}{digits}", f(n)),
        _ => format!("{}{sign}{prefix}{digits}", f(n)),
    }
}

/// Insert a separator every k digits from the right.
fn group_every(d: &str, k: usize, sep: char) -> String {
    let n = d.len();
    let mut out = String::new();
    for (i, ch) in d.chars().enumerate() {
        if i > 0 && (n - i) % k == 0 {
            out.push(sep);
        }
        out.push(ch);
    }
    out
}

/// format(n, spec) for an int with an integer presentation type (b, o, x, X, d, n, c or none): Some(text), or None
/// when the type is a float one (e, f, g, %) and the caller formats float(n), or Err(message) for a spec Python
/// refuses for an int.
pub fn fmt_bigint(n: &num_bigint::BigInt, spec: &str) -> Option<Result<String, String>> {
    let sp = parse_spec(spec)?;
    let (radix, upper) = match sp.ty.as_str() {
        "" | "d" | "n" => (10u32, false),
        "b" => (2, false),
        "o" => (8, false),
        "x" => (16, false),
        "X" => (16, true),
        "c" => {
            let ch = num_traits::ToPrimitive::to_u32(n).and_then(char::from_u32);
            return Some(match ch {
                Some(ch) => Ok(pad(&sp, "", "", &ch.to_string(), '<')),
                None => Err("%c arg not in range(0x110000)".into()),
            });
        }
        _ => return None,
    };
    if sp.prec.is_some() {
        return Some(Err("Precision not allowed in integer format specifier".into()));
    }
    if sp.group == Some(',') && radix != 10 {
        return Some(Err(format!("Cannot specify ',' with '{}'.", sp.ty)));
    }
    // CPython's limit on decimal digits (4300), checked on the size before any digits are produced
    if radix == 10 && n.bits() > 14_284 && (n.bits() >= 14_289 || n.to_string().trim_start_matches('-').len() > 4300) {
        return Some(Err("Exceeds the limit (4300 digits) for integer string conversion; use                          sys.set_int_max_str_digits() to increase the limit".into()));
    }
    let neg = n.sign() == num_bigint::Sign::Minus;
    let mag = n.magnitude().to_str_radix(radix);
    let mut digits = if upper { mag.to_uppercase() } else { mag };
    if let Some(g) = sp.group {
        digits = group_every(&digits, if radix == 10 { 3 } else { 4 }, g);
    }
    let sign = if neg { "-" } else { match sp.sign { '+' => "+", ' ' => " ", _ => "" } };
    let prefix = if sp.alt {
        match sp.ty.as_str() { "b" => "0b", "o" => "0o", "x" => "0x", "X" => "0X", _ => "" }
    } else {
        ""
    };
    Some(Ok(pad(&sp, sign, prefix, &digits, '>')))
}

/// format(x, spec) for a float, the specs procedures use: [+][,][.p](f|e|g|%) and the empty spec (str()); a width,
/// fill or alignment pads the result as Python does.
pub fn fmt(x: f64, spec: &str) -> String {
    if spec.is_empty() {
        return repr(x);
    }
    if let Some(sp) = parse_spec(spec) {
        if sp.width > 0 || sp.align.is_some() || sp.sign == ' ' {
            // the body without width, then padded
            let mut core = String::new();
            if sp.sign == '+' {
                core.push('+');
            }
            if let Some(g) = sp.group {
                core.push(g);
            }
            if let Some(p) = sp.prec {
                core.push_str(&format!(".{p}"));
            }
            core.push_str(&sp.ty);
            let body = if core.is_empty() { repr(x) } else { fmt(x, &core) };
            let (sign, digits) = match body.strip_prefix('-').or_else(|| body.strip_prefix('+')) {
                Some(rest) => (&body[..1], rest.to_string()),
                None => (if sp.sign == ' ' { " " } else { "" }, body.clone()),
            };
            return pad(&sp, sign, "", &digits, '>');
        }
    }
    let mut s = spec;
    let plus = s.starts_with('+');
    if plus {
        s = &s[1..];
    }
    let comma = s.starts_with(',');
    if comma {
        s = &s[1..];
    }
    let (prec, ty) = match s.find(|c: char| c.is_ascii_alphabetic() || c == '%') {
        Some(i) => (&s[..i], &s[i..]),
        None => (s, ""),
    };
    let p: Option<usize> = prec.strip_prefix('.').and_then(|d| d.parse().ok());
    let body = match ty {
        "f" | "F" => fixed(x, p.unwrap_or(6), comma),
        "e" | "E" => sci(x, p.unwrap_or(6)),
        "g" | "G" | "" => {
            let g = py_g(x, p.unwrap_or(if ty.is_empty() { 12 } else { 6 }));
            if comma { group(&g) } else { g }
        }
        "%" => format!("{}%", fixed(x * 100.0, p.unwrap_or(6), comma)),
        _ => repr(x),
    };
    if plus && !body.starts_with('-') { format!("+{body}") } else { body }
}

/// format(i, spec) for an int.
pub fn fmt_int(i: i64, spec: &str) -> String {
    match spec {
        "" | "d" => i.to_string(),
        "," | ",d" => group(&i.to_string()),
        _ => match fmt_bigint(&num_bigint::BigInt::from(i), spec) {
            Some(Ok(s)) => s,
            _ => fmt(i as f64, spec),
        },
    }
}

fn fixed(x: f64, p: usize, comma: bool) -> String {
    if !x.is_finite() {
        return py_g(x, 6);
    }
    let s = format!("{:.*}", p, x);
    if comma { group(&s) } else { s }
}

fn sci(x: f64, p: usize) -> String {
    if !x.is_finite() {
        return py_g(x, 6);
    }
    let s = format!("{:.*e}", p, x);
    let (m, e) = s.split_once('e').unwrap();
    let e: i32 = e.parse().unwrap();
    format!("{m}e{}{:02}", if e < 0 { '-' } else { '+' }, e.abs())
}

/// thousands separators on the integer part
pub fn group(s: &str) -> String {
    let (sign, rest) = if let Some(r) = s.strip_prefix('-') { ("-", r) } else { ("", s) };
    let (int, frac) = match rest.find(|c: char| !c.is_ascii_digit()) {
        Some(i) => (&rest[..i], &rest[i..]),
        None => (rest, ""),
    };
    let mut out = String::new();
    for (i, c) in int.chars().enumerate() {
        if i > 0 && (int.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    format!("{sign}{out}{frac}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn python_semantics() {
        assert_eq!(pymod(-7.0, 3.0), 2.0);
        assert_eq!(floordiv(-7.0, 3.0), -3.0);
        assert_eq!(ifloordiv(-7, 3), -3);
        assert_eq!(imod(-7, 3), 2);
        assert_eq!(fmt(1234567.891, ",.2f"), "1,234,567.89");
        assert_eq!(fmt(0.000123, "g"), "0.000123");
        assert_eq!(fmt(2.5, ".1e"), "2.5e+00");
        assert_eq!(fmt(0.256, ".1%"), "25.6%");
        assert_eq!(r12(1.0 / 3.0), 0.333333333333);
        assert_eq!(round_n(2.675, 2), 2.67);
        assert_eq!(round_half_even(2.5), 2.0);
        assert_eq!(round_half_even(3.5), 4.0);
    }

    #[test]
    fn int_and_padded_specs_as_python() {
        // format(v, spec) in CPython 3.13
        let b = |v: i64, s: &str| fmt_int(v, s);
        assert_eq!(b(61, "08b"), "00111101");
        assert_eq!(b(255, "#x"), "0xff");
        assert_eq!(b(1000, "_b"), "11_1110_1000");
        assert_eq!(b(42, ">6"), "    42");
        assert_eq!(b(-7, "+05d"), "-0007");
        assert_eq!(b(8, "^7o"), "  10   ");
        assert_eq!(b(48879, "X"), "BEEF");
        assert_eq!(b(1234567, ","), "1,234,567");
        assert_eq!(b(2, "010.3f"), "000002.000");
        assert_eq!(b(5, " d"), " 5");
        assert_eq!(fmt(3.14159, "10.2f"), "      3.14");
        assert_eq!(fmt(-3.14159, "<10.2f"), "-3.14     ");
        assert_eq!(fmt(2.5, "08.3f"), "0002.500");
    }
}
