//! Formula text, read exactly as the calculus and differential-equations packages read it (parse_expr): the text is
//! rewritten from written mathematics into Python arithmetic (`_prep`: ^ -> **, implicit products 2x / 3(x+1) / xy,
//! sin^2(x) -> (sin(x))**2), parsed by Python's expression grammar (here: a parser of that grammar, so text Python
//! cannot read is refused with the same message and text Python reads but the allowlist does not take is "unsupported
//! syntax"), counted as ast.walk counts nodes (more than 160 is "too complex"), then walked into an exact expression
//! (crate::sym). Every refusal is the package's ValueError message, word for word.

use crate::pyre;
use crate::py::Py;
use crate::sym::{self, Ex, Fun};
use num_bigint::BigInt;
use num_traits::Signed;
use std::rc::Rc;

const MAX_LEN: usize = 300;
const MAX_NODES: usize = 160;
/// the allowlisted functions and their sympy meaning (sqrt is a power), longest names first for the sin^2 rewrite
const FNS: [(&str, Option<Fun>); 19] = [
    ("arcsin", Some(Fun::Asin)), ("arccos", Some(Fun::Acos)), ("arctan", Some(Fun::Atan)), ("sinh", Some(Fun::Sinh)),
    ("cosh", Some(Fun::Cosh)), ("tanh", Some(Fun::Tanh)), ("asin", Some(Fun::Asin)), ("acos", Some(Fun::Acos)),
    ("atan", Some(Fun::Atan)), ("sqrt", None), ("sin", Some(Fun::Sin)), ("cos", Some(Fun::Cos)), ("tan", Some(Fun::Tan)),
    ("sec", Some(Fun::Sec)), ("csc", Some(Fun::Csc)), ("cot", Some(Fun::Cot)), ("exp", Some(Fun::Exp)),
    ("log", Some(Fun::Log)), ("ln", Some(Fun::Log)),
];
const CONSTS: [&str; 2] = ["pi", "e"];

fn is_fn(w: &str) -> bool {
    FNS.iter().any(|(n, _)| *n == w)
}

/// Python's repr() of the text, as the messages quote it.
fn repr(s: &str) -> String {
    crate::num::py_repr_str(s)
}

fn re_sub(pattern: &str, repl: &str, text: &str) -> String {
    let re = pyre::compile(pattern, 0).expect("a fixed pattern");
    match pyre::sub(&re, &Py::str(repl), &Rc::from(text), 0) {
        Ok(Py::Str(s)) => s.to_string(),
        _ => text.to_string(),
    }
}

/// The package's _prep: written mathematics -> Python arithmetic.
pub fn prep(text: &str, names: &[&str]) -> String {
    let mut s = text.trim().replace('^', "**").replace('−', "-").replace('·', "*").replace('×', "*").replace('π', "pi");
    s = s.replace('²', "**2").replace('³', "**3").replace('θ', "theta").replace('{', "(").replace('}', ")");
    s = s.replace('[', "(").replace(']', ")");
    // sin**2(x) -> (sin(x))**2, at most four times
    let alts: Vec<&str> = FNS.iter().map(|(n, _)| *n).collect();
    let mut sorted = alts.clone();
    sorted.sort_by_key(|n| std::cmp::Reverse(n.len()));
    let pat = format!(r"\b({})\s*\*\*\s*(\d+)\s*\(", sorted.join("|"));
    let re = pyre::compile(&pat, 0).expect("a fixed pattern");
    for _ in 0..4 {
        let m = match pyre::find_at(&re, "search", &Rc::from(s.as_str()), 0, None) {
            Ok(Py::Match(m)) => m,
            _ => break,
        };
        // byte spans of the match and its groups
        let (ms, me) = m.spans[0].unwrap();
        let (g1s, g1e) = m.spans[1].unwrap();
        let (g2s, g2e) = m.spans[2].unwrap();
        let b = s.as_bytes();
        let (mut depth, mut j) = (0i32, me - 1);
        for k in (me - 1)..s.len() {
            j = k;
            depth += match b[k] {
                b'(' => 1,
                b')' => -1,
                _ => 0,
            };
            if depth == 0 {
                break;
            }
        }
        if depth != 0 {
            break;
        }
        let name = &s[g1s..g1e];
        let power = &s[g2s..g2e];
        s = format!("{}({}{})**{}{}", &s[..ms], name, &s[me - 1..=j], power, &s[j + 1..]);
    }
    // words: a function, a constant or a variable stays (a variable or constant before "(" multiplies); a run of
    // single-letter variables and constants is a product (xy -> x*y); anything else stays as written
    let word = pyre::compile(r"[A-Za-z_]+", 0).expect("a fixed pattern");
    let mut out = String::new();
    let mut last = 0;
    let caps = match pyre::findall_spans(&word, &s) {
        Ok(v) => v,
        Err(_) => vec![],
    };
    for (a, b) in caps {
        out.push_str(&s[last..a]);
        let w = &s[a..b];
        let after = s[b..].chars().next();
        let star = if after == Some('(') { "*" } else { "" };
        if is_fn(w) || CONSTS.contains(&w) || names.contains(&w) {
            if names.contains(&w) || CONSTS.contains(&w) {
                out.push_str(w);
                out.push_str(star);
            } else {
                out.push_str(w);
            }
        } else if w.chars().all(|c| names.contains(&c.to_string().as_str()) || CONSTS.contains(&c.to_string().as_str())) {
            let parts: Vec<String> = w.chars().map(|c| c.to_string()).collect();
            out.push_str(&parts.join("*"));
            out.push_str(star);
        } else {
            out.push_str(w);
        }
        last = b;
    }
    out.push_str(&s[last..]);
    let s = re_sub(r"(\d)\s*(?![eE][-+]?\d)([A-Za-z(])", r"\1*\2", &out);
    re_sub(r"\)\s*([A-Za-z0-9(])", r")*\1", &s)
}

// ------------------------------------------------------------------------------------------- Python's grammar
/// A node of Python's expression AST, only as far as the walk needs it; everything else is Other (a construct
/// Python reads but the allowlist refuses), with its node count.
#[derive(Debug, Clone)]
enum Node {
    Int(BigInt),
    Float(f64),
    OtherConst,                         // str, complex, bool, None, Ellipsis
    Name(String),
    Bin(char, Box<Node>, Box<Node>),    // + - * / ^ (power) ; other operators are Other
    Unary(char, Box<Node>),             // + -  ; ~ and not are Other
    Call(String, Vec<Node>, usize),     // a name called with positional args and n keywords
    Other(usize),
}

/// ast.walk's count of a node and everything under it (an operator and a Load context are nodes too).
fn count(n: &Node) -> usize {
    match n {
        Node::Int(_) | Node::Float(_) | Node::OtherConst => 1,
        Node::Name(_) => 2,
        Node::Bin(_, a, b) => 2 + count(a) + count(b),
        Node::Unary(_, a) => 2 + count(a),
        Node::Call(_, args, kws) => 1 + 2 + args.iter().map(count).sum::<usize>() + kws,
        Node::Other(k) => *k,
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Tok {
    Num(String),
    Name(String),
    Str,
    Op(String),
    End,
}

struct SyntaxError;

const KEYWORDS: [&str; 35] = ["False", "None", "True", "and", "as", "assert", "async", "await", "break", "class",
    "continue", "def", "del", "elif", "else", "except", "finally", "for", "from", "global", "if", "import", "in", "is",
    "lambda", "nonlocal", "not", "or", "pass", "raise", "return", "try", "while", "with", "yield"];

fn tokenize(s: &str) -> Result<Vec<Tok>, SyntaxError> {
    let cs: Vec<char> = s.chars().collect();
    let mut out = vec![];
    let mut i = 0;
    let mut depth = 0i32;
    while i < cs.len() {
        let c = cs[i];
        if c == ' ' || c == '\t' || c == '\x0c' {
            i += 1;
            continue;
        }
        if c == '\n' || c == '\r' {
            if depth == 0 {
                // a newline ends the expression: only whitespace and comments may follow
                let rest: String = cs[i..].iter().collect();
                if rest.lines().all(|l| { let t = l.trim(); t.is_empty() || t.starts_with('#') }) {
                    break;
                }
                return Err(SyntaxError);
            }
            i += 1;
            continue;
        }
        if c == '#' {
            while i < cs.len() && cs[i] != '\n' {
                i += 1;
            }
            continue;
        }
        if c == '\\' {
            // a line continuation: backslash then newline
            if i + 1 < cs.len() && (cs[i + 1] == '\n' || cs[i + 1] == '\r') {
                i += 2;
                continue;
            }
            return Err(SyntaxError);
        }
        if c.is_ascii_digit() || (c == '.' && i + 1 < cs.len() && cs[i + 1].is_ascii_digit()) {
            let start = i;
            i += 1;
            while i < cs.len() {
                let d = cs[i];
                let prev = cs[i - 1];
                if d.is_ascii_alphanumeric() || d == '_' || d == '.' || ((d == '+' || d == '-') && (prev == 'e' || prev == 'E')
                    && !cs[start..i].iter().collect::<String>().to_ascii_lowercase().starts_with("0x")) {
                    i += 1;
                } else {
                    break;
                }
            }
            out.push(Tok::Num(cs[start..i].iter().collect()));
            continue;
        }
        if c.is_alphabetic() || c == '_' {
            let start = i;
            while i < cs.len() && (cs[i].is_alphanumeric() || cs[i] == '_') {
                i += 1;
            }
            let w: String = cs[start..i].iter().collect();
            // a string prefix (r"", b'', f"" ...) followed by a quote
            if i < cs.len() && (cs[i] == '\'' || cs[i] == '"') && w.len() <= 2
                && w.chars().all(|x| "rRbBuUfF".contains(x)) {
                i = skip_string(&cs, i)?;
                out.push(Tok::Str);
                continue;
            }
            out.push(Tok::Name(w));
            continue;
        }
        if c == '\'' || c == '"' {
            i = skip_string(&cs, i)?;
            out.push(Tok::Str);
            continue;
        }
        let three: String = cs[i..(i + 3).min(cs.len())].iter().collect();
        let two: String = cs[i..(i + 2).min(cs.len())].iter().collect();
        let op = if ["**=", "//=", ">>=", "<<=", "..."].contains(&three.as_str()) {
            three
        } else if ["**", "//", "<<", ">>", "<=", ">=", "==", "!=", "->", ":=", "+=", "-=", "*=", "/=", "%=", "&=", "|=",
                   "^=", "@="].contains(&two.as_str()) {
            two
        } else if "+-*/%@&|^~<>()[]{},:.;=".contains(c) {
            c.to_string()
        } else {
            return Err(SyntaxError);                       // $, ?, !, ` and other characters Python cannot read
        };
        match op.as_str() {
            "(" | "[" | "{" => depth += 1,
            ")" | "]" | "}" => depth -= 1,
            _ => {}
        }
        if depth < 0 {
            return Err(SyntaxError);
        }
        i += op.chars().count();
        out.push(Tok::Op(op));
    }
    if depth != 0 {
        return Err(SyntaxError);
    }
    out.push(Tok::End);
    Ok(out)
}

fn skip_string(cs: &[char], mut i: usize) -> Result<usize, SyntaxError> {
    let q = cs[i];
    let triple = i + 2 < cs.len() && cs[i + 1] == q && cs[i + 2] == q;
    if triple {
        i += 3;
        while i + 2 < cs.len() && !(cs[i] == q && cs[i + 1] == q && cs[i + 2] == q) {
            i += if cs[i] == '\\' { 2 } else { 1 };
        }
        if i + 2 >= cs.len() {
            return Err(SyntaxError);
        }
        return Ok(i + 3);
    }
    i += 1;
    while i < cs.len() && cs[i] != q {
        if cs[i] == '\n' {
            return Err(SyntaxError);
        }
        i += if cs[i] == '\\' { 2 } else { 1 };
    }
    if i >= cs.len() {
        return Err(SyntaxError);
    }
    Ok(i + 1)
}

/// A Python numeric literal: an int, a float, or another constant (imaginary); SyntaxError when it is not one.
fn number(t: &str) -> Result<Node, SyntaxError> {
    let low = t.to_ascii_lowercase();
    let clean = |s: &str| -> Result<String, SyntaxError> {
        // underscores only between digits
        let b: Vec<char> = s.chars().collect();
        for (k, ch) in b.iter().enumerate() {
            if *ch == '_' && !(k > 0 && k + 1 < b.len() && b[k - 1].is_ascii_hexdigit() && b[k + 1].is_ascii_hexdigit()) {
                return Err(SyntaxError);
            }
        }
        Ok(s.replace('_', ""))
    };
    for (pre, radix) in [("0x", 16), ("0o", 8), ("0b", 2)] {
        if let Some(rest) = low.strip_prefix(pre) {
            let rest = rest.strip_prefix('_').unwrap_or(rest);
            let d = clean(rest)?;
            if d.is_empty() {
                return Err(SyntaxError);
            }
            return BigInt::parse_bytes(d.as_bytes(), radix).map(Node::Int).ok_or(SyntaxError);
        }
    }
    if let Some(body) = low.strip_suffix('j') {
        number(body)?;                                       // an imaginary literal: a constant the walk refuses
        return Ok(Node::OtherConst);
    }
    let d = clean(&low)?;
    if d.chars().all(|c| c.is_ascii_digit()) {
        // a decimal integer: no leading zeros (except 0, 00, ...)
        if d.len() > 1 && d.starts_with('0') && d.chars().any(|c| c != '0') {
            return Err(SyntaxError);
        }
        return d.parse::<BigInt>().map(Node::Int).map_err(|_| SyntaxError);
    }
    // a float: digits with one point and/or an exponent
    let (mant, exp) = match d.split_once('e') {
        Some((m, e)) => (m, Some(e)),
        None => (d.as_str(), None),
    };
    if let Some(e) = exp {
        let e = e.strip_prefix(['+', '-']).unwrap_or(e);
        if e.is_empty() || !e.chars().all(|c| c.is_ascii_digit()) {
            return Err(SyntaxError);
        }
    }
    let dots = mant.matches('.').count();
    if dots > 1 || mant == "." || mant.is_empty() || !mant.chars().all(|c| c.is_ascii_digit() || c == '.') {
        return Err(SyntaxError);
    }
    if dots == 0 && exp.is_none() {
        return Err(SyntaxError);
    }
    d.parse::<f64>().map(Node::Float).map_err(|_| SyntaxError)
}

struct P {
    t: Vec<Tok>,
    i: usize,
}

type PR = Result<Node, SyntaxError>;

impl P {
    fn peek(&self) -> &Tok {
        &self.t[self.i]
    }
    fn op(&self, s: &str) -> bool {
        matches!(self.peek(), Tok::Op(o) if o == s)
    }
    fn kw(&self, s: &str) -> bool {
        matches!(self.peek(), Tok::Name(o) if o == s)
    }
    fn eat_op(&mut self, s: &str) -> Result<(), SyntaxError> {
        if self.op(s) {
            self.i += 1;
            Ok(())
        } else {
            Err(SyntaxError)
        }
    }
    fn other(parts: &[&Node], own: usize) -> Node {
        Node::Other(own + parts.iter().map(|p| count(p)).sum::<usize>())
    }

    // expression: lambda / ternary
    fn expr(&mut self) -> PR {
        if self.kw("lambda") {
            self.i += 1;
            let mut params = 0usize;
            while !self.op(":") {
                match self.peek() {
                    Tok::Name(n) if !KEYWORDS.contains(&n.as_str()) => { self.i += 1; params += 1; }
                    Tok::Op(o) if o == "," || o == "*" || o == "**" || o == "=" || o == "/" => self.i += 1,
                    _ => return Err(SyntaxError),
                }
            }
            self.i += 1;
            let body = self.expr()?;
            return Ok(Self::other(&[&body], 2 + 2 * params));
        }
        let a = self.or_test()?;
        if self.kw("if") {
            self.i += 1;
            let c = self.or_test()?;
            if !self.kw("else") {
                return Err(SyntaxError);
            }
            self.i += 1;
            let b = self.expr()?;
            return Ok(Self::other(&[&a, &c, &b], 1));
        }
        Ok(a)
    }
    fn or_test(&mut self) -> PR {
        self.boolop("or", Self::and_test)
    }
    fn and_test(&mut self) -> PR {
        self.boolop("and", Self::not_test)
    }
    fn boolop(&mut self, word: &str, next: fn(&mut Self) -> PR) -> PR {
        let first = next(self)?;
        if !self.kw(word) {
            return Ok(first);
        }
        let mut parts = vec![first];
        while self.kw(word) {
            self.i += 1;
            parts.push(next(self)?);
        }
        let refs: Vec<&Node> = parts.iter().collect();
        Ok(Self::other(&refs, 2))
    }
    fn not_test(&mut self) -> PR {
        if self.kw("not") {
            self.i += 1;
            let a = self.not_test()?;
            return Ok(Self::other(&[&a], 2));
        }
        self.comparison()
    }
    fn comparison(&mut self) -> PR {
        let a = self.bitor()?;
        let mut parts = vec![a];
        let mut ops = 0;
        loop {
            let is_cmp = matches!(self.peek(), Tok::Op(o) if ["<", ">", "==", ">=", "<=", "!="].contains(&o.as_str()))
                || self.kw("in") || self.kw("is") || self.kw("not");
            if !is_cmp {
                break;
            }
            if self.kw("not") {
                self.i += 1;
                if !self.kw("in") {
                    return Err(SyntaxError);
                }
            } else if self.kw("is") {
                self.i += 1;
                if self.kw("not") {
                    self.i += 1;
                }
                ops += 1;
                parts.push(self.bitor()?);
                continue;
            }
            self.i += 1;
            ops += 1;
            parts.push(self.bitor()?);
        }
        if ops == 0 {
            return Ok(parts.pop().unwrap());
        }
        let refs: Vec<&Node> = parts.iter().collect();
        Ok(Self::other(&refs, 1 + ops))
    }
    fn binlevel(&mut self, ops: &[&str], next: fn(&mut Self) -> PR) -> PR {
        let mut a = next(self)?;
        loop {
            let o = match self.peek() {
                Tok::Op(o) if ops.contains(&o.as_str()) => o.clone(),
                _ => break,
            };
            self.i += 1;
            let b = next(self)?;
            a = match o.as_str() {
                "+" => Node::Bin('+', Box::new(a), Box::new(b)),
                "-" => Node::Bin('-', Box::new(a), Box::new(b)),
                "*" => Node::Bin('*', Box::new(a), Box::new(b)),
                "/" => Node::Bin('/', Box::new(a), Box::new(b)),
                _ => Self::other(&[&a, &b], 2),
            };
        }
        Ok(a)
    }
    fn bitor(&mut self) -> PR {
        self.binlevel(&["|"], Self::bitxor)
    }
    fn bitxor(&mut self) -> PR {
        self.binlevel(&["^"], Self::bitand)
    }
    fn bitand(&mut self) -> PR {
        self.binlevel(&["&"], Self::shift)
    }
    fn shift(&mut self) -> PR {
        self.binlevel(&["<<", ">>"], Self::arith)
    }
    fn arith(&mut self) -> PR {
        self.binlevel(&["+", "-"], Self::term)
    }
    fn term(&mut self) -> PR {
        self.binlevel(&["*", "/", "//", "%", "@"], Self::factor)
    }
    fn factor(&mut self) -> PR {
        for (o, c) in [("+", '+'), ("-", '-')] {
            if self.op(o) {
                self.i += 1;
                let a = self.factor()?;
                return Ok(Node::Unary(c, Box::new(a)));
            }
        }
        if self.op("~") {
            self.i += 1;
            let a = self.factor()?;
            return Ok(Self::other(&[&a], 2));
        }
        self.power()
    }
    fn power(&mut self) -> PR {
        if self.kw("await") {
            return Err(SyntaxError);
        }
        let a = self.atom_expr()?;
        if self.op("**") {
            self.i += 1;
            let b = self.factor()?;                          // right-associative; the exponent may be signed
            return Ok(Node::Bin('^', Box::new(a), Box::new(b)));
        }
        Ok(a)
    }
    fn atom_expr(&mut self) -> PR {
        let mut a = self.atom()?;
        loop {
            if self.op("(") {
                self.i += 1;
                let (args, kws, extra) = self.arglist()?;
                a = match a {
                    Node::Name(n) if extra == 0 => Node::Call(n, args, kws),
                    other => {
                        let refs: Vec<&Node> = args.iter().collect();
                        let mut k = Self::other(&refs, 1 + kws + extra);
                        if let Node::Other(c) = &mut k {
                            *c += count(&other);
                        }
                        k
                    }
                };
            } else if self.op("[") {
                self.i += 1;
                let s = self.subscript()?;
                self.eat_op("]")?;
                a = Self::other(&[&a, &s], 2);
            } else if self.op(".") {
                self.i += 1;
                match self.peek() {
                    Tok::Name(n) if !KEYWORDS.contains(&n.as_str()) => self.i += 1,
                    _ => return Err(SyntaxError),
                }
                a = Self::other(&[&a], 2);
            } else {
                break;
            }
        }
        Ok(a)
    }
    /// call arguments: (positional nodes, keyword count, other parts' node count); consumes ")"
    fn arglist(&mut self) -> Result<(Vec<Node>, usize, usize), SyntaxError> {
        let (mut args, mut kws, mut extra) = (vec![], 0usize, 0usize);
        let mut seen_kw = false;
        while !self.op(")") {
            if self.op("*") || self.op("**") {
                let star = self.op("*");
                self.i += 1;
                let v = self.expr()?;
                extra += count(&v) + if star { 2 } else { 1 };
                if !star {
                    seen_kw = true;
                }
            } else if matches!(self.peek(), Tok::Name(_)) && matches!(&self.t[self.i + 1], Tok::Op(o) if o == "=") {
                self.i += 2;
                let v = self.expr()?;
                kws += 1 + count(&v);
                seen_kw = true;
            } else {
                let v = self.expr()?;
                if self.kw("for") {
                    // a generator argument
                    let g = self.comp_tail(&v)?;
                    extra += count(&g);
                } else {
                    if seen_kw {
                        return Err(SyntaxError);              // a positional argument after a keyword one
                    }
                    args.push(v);
                }
            }
            if self.op(",") {
                self.i += 1;
            } else if !self.op(")") {
                return Err(SyntaxError);
            }
        }
        self.i += 1;
        Ok((args, kws, extra))
    }
    fn comp_tail(&mut self, elt: &Node) -> PR {
        let mut total = 1 + count(elt);
        while self.kw("for") {
            self.i += 1;
            let target = self.bitor()?;
            if !self.kw("in") {
                return Err(SyntaxError);
            }
            self.i += 1;
            let it = self.or_test()?;
            total += 1 + count(&target) + count(&it);
            while self.kw("if") {
                self.i += 1;
                let c = self.or_test()?;
                total += count(&c);
            }
        }
        Ok(Node::Other(total))
    }
    fn subscript(&mut self) -> PR {
        let mut parts = vec![];
        loop {
            if self.op("]") {
                break;
            }
            let mut own = 0;
            if !self.op(":") {
                parts.push(self.expr()?);
            }
            while self.op(":") {
                self.i += 1;
                own = 1;
                if !self.op(":") && !self.op("]") && !self.op(",") {
                    parts.push(self.expr()?);
                }
            }
            parts.push(Node::Other(own));
            if self.op(",") {
                self.i += 1;
            } else {
                break;
            }
        }
        let refs: Vec<&Node> = parts.iter().collect();
        Ok(Self::other(&refs, 0))
    }
    fn atom(&mut self) -> PR {
        let t = self.peek().clone();
        match t {
            Tok::Num(s) => {
                self.i += 1;
                number(&s)
            }
            Tok::Str => {
                while matches!(self.peek(), Tok::Str) {
                    self.i += 1;                              // adjacent strings concatenate
                }
                Ok(Node::OtherConst)
            }
            Tok::Name(n) => {
                if ["True", "False", "None"].contains(&n.as_str()) {
                    self.i += 1;
                    return Ok(Node::OtherConst);
                }
                if KEYWORDS.contains(&n.as_str()) {
                    return Err(SyntaxError);
                }
                self.i += 1;
                Ok(Node::Name(n))
            }
            Tok::Op(o) if o == "(" => {
                self.i += 1;
                if self.op(")") {
                    self.i += 1;
                    return Ok(Node::Other(2));               // ()
                }
                let first = self.expr_or_star()?;
                if self.kw("for") {
                    let g = self.comp_tail(&first)?;
                    self.eat_op(")")?;
                    return Ok(g);
                }
                if self.op(")") {
                    self.i += 1;
                    return Ok(first);
                }
                let mut parts = vec![first];
                while self.op(",") {
                    self.i += 1;
                    if self.op(")") {
                        break;
                    }
                    parts.push(self.expr_or_star()?);
                }
                self.eat_op(")")?;
                let refs: Vec<&Node> = parts.iter().collect();
                Ok(Self::other(&refs, 2))                    // a tuple
            }
            Tok::Op(o) if o == "[" => {
                self.i += 1;
                let mut parts = vec![];
                while !self.op("]") {
                    let v = self.expr_or_star()?;
                    if self.kw("for") {
                        let g = self.comp_tail(&v)?;
                        parts.push(g);
                        break;
                    }
                    parts.push(v);
                    if self.op(",") {
                        self.i += 1;
                    } else if !self.op("]") {
                        return Err(SyntaxError);
                    }
                }
                self.eat_op("]")?;
                let refs: Vec<&Node> = parts.iter().collect();
                Ok(Self::other(&refs, 2))
            }
            Tok::Op(o) if o == "{" => {
                self.i += 1;
                let mut parts = vec![];
                while !self.op("}") {
                    if self.op("**") {
                        self.i += 1;
                    }
                    let k = self.expr_or_star()?;
                    parts.push(k);
                    if self.op(":") {
                        self.i += 1;
                        parts.push(self.expr()?);
                    }
                    if self.kw("for") {
                        let last = parts.pop().unwrap();
                        parts.push(self.comp_tail(&last)?);
                        break;
                    }
                    if self.op(",") {
                        self.i += 1;
                    } else if !self.op("}") {
                        return Err(SyntaxError);
                    }
                }
                self.eat_op("}")?;
                let refs: Vec<&Node> = parts.iter().collect();
                Ok(Self::other(&refs, 1))
            }
            Tok::Op(o) if o == "..." => {
                self.i += 1;
                Ok(Node::OtherConst)
            }
            _ => Err(SyntaxError),
        }
    }
    fn expr_or_star(&mut self) -> PR {
        if self.op("*") {
            self.i += 1;
            let v = self.bitor()?;
            return Ok(Self::other(&[&v], 2));
        }
        let v = self.expr()?;
        if self.op(":=") {
            self.i += 1;
            let w = self.expr()?;
            return Ok(Self::other(&[&v, &w], 1));
        }
        Ok(v)
    }
}

/// ast.parse(src, mode="eval"): the expression's tree, or SyntaxError.
fn python_parse(src: &str) -> Result<Node, SyntaxError> {
    let t = tokenize(src)?;
    let mut p = P { t, i: 0 };
    if matches!(p.peek(), Tok::End) {
        return Err(SyntaxError);
    }
    let first = p.expr_or_star()?;
    let mut node = first;
    if p.op(",") {
        // a bare tuple "a, b"
        let mut parts = vec![node];
        while p.op(",") {
            p.i += 1;
            if matches!(p.peek(), Tok::End) {
                break;
            }
            parts.push(p.expr_or_star()?);
        }
        let refs: Vec<&Node> = parts.iter().collect();
        node = P::other(&refs, 2);
    }
    if !matches!(p.peek(), Tok::End) {
        return Err(SyntaxError);
    }
    Ok(node)
}

// ------------------------------------------------------------------------------------------- the allowlist walk
/// parse_expr(text, names): the exact expression, or the package's ValueError message.
pub fn parse(text: &str, names: &[&str]) -> Result<Ex, String> {
    if text.trim().is_empty() || text.chars().count() > MAX_LEN {
        return Err("the expression is missing or too long".into());
    }
    let src = prep(text, names);
    let cant = || format!("cannot read the expression {}", repr(text));
    let tree = python_parse(&src).map_err(|_| cant())?;
    if 1 + count(&tree) > MAX_NODES {
        return Err("the expression is too complex".into());
    }
    let e = walk(&tree, text, names)?;
    if sym::has_undefined(&e) {
        return Err("the expression is undefined".into());
    }
    Ok(e)
}

fn walk(n: &Node, text: &str, names: &[&str]) -> Result<Ex, String> {
    let unsupported = || format!("unsupported syntax in {}", repr(text));
    Ok(match n {
        Node::Int(i) => sym::int_big(i.clone()),
        Node::Float(x) => sym::rational_of_repr(*x).ok_or_else(unsupported)?,
        Node::Name(id) => {
            if names.contains(&id.as_str()) {
                Ex::Sym(Rc::from(id.as_str()))
            } else if id == "pi" {
                Ex::Pi
            } else if id == "e" {
                Ex::E
            } else {
                return Err(format!("unknown name {} (this capability uses {})", repr(id), names.join(", ")));
            }
        }
        Node::Unary(c, a) => {
            let v = walk(a, text, names)?;
            if *c == '-' { sym::neg(v) } else { v }
        }
        Node::Bin(op, a, b) => {
            let (x, y) = (walk(a, text, names)?, walk(b, text, names)?);
            match op {
                '+' => sym::add(x, y),
                '-' => sym::sub(x, y),
                '*' => sym::mul(x, y),
                '/' => {
                    if sym::is_zero(&y) {
                        return Err("division by zero in the expression".into());
                    }
                    sym::div(x, y)
                }
                _ => {
                    if let Some(v) = sym::numeric_abs(&y) {
                        if v > 64.0 {
                            return Err("exponents are limited to 64 in size".into());
                        }
                    }
                    sym::pow(x, y)
                }
            }
        }
        Node::Call(name, args, kws) => {
            let f = FNS.iter().find(|(nm, _)| *nm == name.as_str());
            match f {
                Some((_, kind)) if args.len() == 1 && *kws == 0 => {
                    let a = walk(&args[0], text, names)?;
                    match kind {
                        Some(k) => sym::func(*k, a),
                        None => sym::sqrt(a),
                    }
                }
                _ => return Err(unsupported()),
            }
        }
        Node::OtherConst | Node::Other(_) => return Err(unsupported()),
    })
}

// ================================================================================ the engine's parser (runtime/expr.py)
/// runtime/expr.py parse(text, symbols, symbolic_powers): the engine's own allowlisted reader, used by the
/// differential-equations procedures (numbers, the named symbols, pi, + - * / **, sqrt sin cos tan exp log asin acos
/// atan; a power of a symbolic exponent only with symbolic_powers, else a rational exponent of at most 8 in size).
/// Err is the ExpressionError's message.
pub fn parse_engine(text: &str, names: &[&str], symbolic_powers: bool) -> Result<Ex, String> {
    if text.trim().is_empty() || text.chars().count() > 400 {
        return Err("expression missing or too long".into());
    }
    let tree = python_parse(text.trim()).map_err(|_| format!("cannot parse: {text}"))?;
    if 1 + count(&tree) > 120 {
        return Err("expression too complex".into());
    }
    walk_engine(&tree, names, symbolic_powers)
}

fn walk_engine(n: &Node, names: &[&str], symbolic_powers: bool) -> Result<Ex, String> {
    let w = |n: &Node| walk_engine(n, names, symbolic_powers);
    Ok(match n {
        Node::Int(i) => sym::int_big(i.clone()),
        // sp.Rational(str(value)): str and repr of a float are the same text
        Node::Float(x) => sym::rational_of_repr(*x).ok_or_else(|| "unsupported syntax".to_string())?,
        Node::Name(id) => {
            if names.contains(&id.as_str()) {
                Ex::Sym(Rc::from(id.as_str()))
            } else if id == "pi" {
                Ex::Pi
            } else {
                return Err(format!("unknown name: {id}"));
            }
        }
        Node::Unary(c, a) => {
            let v = w(a)?;
            if *c == '-' { sym::neg(v) } else { v }
        }
        Node::Bin(op, a, b) => {
            let (x, y) = (w(a)?, w(b)?);
            match op {
                '+' => sym::add(x, y),
                '-' => sym::sub(x, y),
                '*' => sym::mul(x, y),
                '/' => {
                    if sym::is_zero(&y) {
                        // sympy: x/0 is zoo, 0/0 is nan
                        if sym::is_zero(&x) { Ex::Nan } else { Ex::Zoo }
                    } else {
                        sym::div(x, y)
                    }
                }
                _ => {
                    if symbolic_powers && !sym::is_number(&y) {
                        return Ok(sym::pow(x, y));
                    }
                    match &y {
                        Ex::Num(k) if k.abs() <= num_rational::BigRational::from_integer(BigInt::from(8)) => {
                            if sym::is_zero(&x) && k.is_negative() {
                                Ex::Zoo
                            } else {
                                sym::pow(x, y)
                            }
                        }
                        _ => return Err("only small rational exponents are allowed".into()),
                    }
                }
            }
        }
        Node::Call(name, args, kws) => {
            let f = match name.as_str() {
                "sqrt" => None,
                "sin" => Some(Fun::Sin),
                "cos" => Some(Fun::Cos),
                "tan" => Some(Fun::Tan),
                "exp" => Some(Fun::Exp),
                "log" => Some(Fun::Log),
                "asin" => Some(Fun::Asin),
                "acos" => Some(Fun::Acos),
                "atan" => Some(Fun::Atan),
                _ => return Err("unsupported syntax".into()),
            };
            if args.len() != 1 || *kws != 0 {
                return Err("unsupported syntax".into());
            }
            let a = w(&args[0])?;
            match f {
                Some(k) => sym::func(k, a),
                None => sym::sqrt(a),
            }
        }
        Node::OtherConst | Node::Other(_) => return Err("unsupported syntax".into()),
    })
}

/// The differential-equations package's _prep: ^ -> **, ln -> log, implicit products (2t, 3(y+1), ty, y(1-y)),
/// e -> exp(1)
pub fn prep_de(text: &str, names: &[&str]) -> String {
    const FUNCS: [&str; 12] = ["sqrt", "sin", "cos", "tan", "exp", "log", "asin", "acos", "atan", "sinh", "cosh", "tanh"];
    let mut s = text.trim().replace('^', "**").replace('−', "-").replace('·', "*").replace('×', "*").replace('π', "pi");
    s = s.replace('²', "**2").replace('³', "**3").replace('[', "(").replace(']', ")");
    s = re_sub(r"\bln\b", "log", &s);
    s = re_sub(r"(\d)\s*(?![eE][-+]?\d)([A-Za-z(])", r"\1*\2", &s);
    let word = pyre::compile(r"(?<![\w.])[A-Za-z_]+", 0).expect("a fixed pattern");
    let mut out = String::new();
    let mut last = 0;
    for (a, b) in pyre::findall_spans(&word, &s).unwrap_or_default() {
        out.push_str(&s[last..a]);
        let w = &s[a..b];
        let star = if s[b..].starts_with('(') { "*" } else { "" };
        if FUNCS.contains(&w) || w == "pi" {
            out.push_str(w);
        } else if w == "e" {
            out.push_str("exp(1)");
        } else if names.contains(&w) {
            out.push_str(w);
            out.push_str(star);
        } else if w.chars().all(|c| names.contains(&c.to_string().as_str())) {
            let parts: Vec<String> = w.chars().map(|c| c.to_string()).collect();
            out.push_str(&parts.join("*"));
            out.push_str(star);
        } else {
            out.push_str(w);
        }
        last = b;
    }
    out.push_str(&s[last..]);
    re_sub(r"\)\s*([A-Za-z0-9(])", r")*\1", &out)
}

/// The differential-equations package's parse_text(text, names): _prep, sinh/cosh/tanh parsed piecewise and
/// rebuilt (they are not in the engine's function list), the engine's parser; the package's ValueError message.
pub fn parse_text_de(text: &str, names: &[&str]) -> Result<Ex, String> {
    if text.trim().is_empty() {
        return Err("the expression is missing".into());
    }
    let src = prep_de(text, names);
    let hyp: Vec<&str> = ["sinh", "cosh", "tanh"].into_iter().filter(|h| src.contains(h)).collect();
    let mut src2 = src.clone();
    for h in &hyp {
        src2 = re_sub(&format!(r"\b{h}\("), &format!("_{h}("), &src2);
    }
    let wrap = |m: String| format!("cannot read the expression {}: {m}", repr(text));
    let e = if hyp.is_empty() {
        parse_engine(&src2, names, true).map_err(wrap)?
    } else {
        parse_hyp(&src2, names).map_err(wrap)?
    };
    if sym::has_undefined(&e) {
        return Err("the expression is undefined".into());
    }
    Ok(e)
}

fn parse_hyp(src: &str, names: &[&str]) -> Result<Ex, String> {
    let re = pyre::compile(r"_(sinh|cosh|tanh)\(", 0).expect("a fixed pattern");
    let (mut out, mut i) = (String::new(), 0usize);
    let mut pieces: Vec<(String, Ex)> = vec![];
    loop {
        let m = match pyre::find_at(&re, "search", &Rc::from(&src[i..]), 0, None) {
            Ok(Py::Match(m)) => m,
            _ => {
                out.push_str(&src[i..]);
                break;
            }
        };
        let (ms, me) = m.spans[0].unwrap();
        let (gs, ge) = m.spans[1].unwrap();
        let fname = src[i + gs..i + ge].to_string();
        let j0 = i + me - 1;
        let b = src.as_bytes();
        let (mut depth, mut j) = (0i32, j0);
        for (k, ch) in b.iter().enumerate().skip(j0) {
            j = k;
            depth += match ch {
                b'(' => 1,
                b')' => -1,
                _ => 0,
            };
            if depth == 0 {
                break;
            }
        }
        if depth != 0 {
            return Err("unbalanced parentheses".into());
        }
        let arg = parse_engine(&src[j0 + 1..j], names, true)?;
        let name = format!("H{}", pieces.len());
        let f = match fname.as_str() {
            "sinh" => Fun::Sinh,
            "cosh" => Fun::Cosh,
            _ => Fun::Tanh,
        };
        pieces.push((name.clone(), sym::func(f, arg)));
        out.push_str(&src[i..i + ms]);
        out.push_str(&name);
        i = j + 1;
    }
    let mut tmp: Vec<&str> = names.to_vec();
    for (n, _) in &pieces {
        tmp.push(n.as_str());
    }
    let e = parse_engine(&out, &tmp, true)?;
    let pairs: Vec<(Rc<str>, Ex)> = pieces.into_iter().map(|(n, v)| (Rc::from(n.as_str()), v)).collect();
    Ok(crate::pysym::subs_many(&e, &pairs))
}
