//! Python's re module for translated procedures, on fancy-regex (a backtracking engine over the regex crate). A
//! Python pattern is rewritten where the two syntaxes differ: \Z is \z, `$` without MULTILINE also matches before a
//! final newline, a `{` that is not a quantifier is a literal, {,n} is {0,n}. match() and fullmatch() anchor the
//! pattern; positions (start, end, span) count characters, as Python's str indices do. The flags are Python's
//! values (re.I = 2, re.M = 8, re.S = 16, re.X = 64); re.A is not modelled (an unsupported pattern, never a guess).

use crate::py::{Py, PyErr, R};
use fancy_regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub const IGNORECASE: i64 = 2;
pub const MULTILINE: i64 = 8;
pub const DOTALL: i64 = 16;
pub const VERBOSE: i64 = 64;
pub const ASCII: i64 = 256;

/// A compiled pattern (re.compile): searching, anchored at the start (match) and anchored at both ends (fullmatch).
pub struct PyRe {
    pub pattern: String,
    pub flags: i64,
    search: Regex,
    anchored: Regex,
    full: Regex,
    /// the pattern anchored at the end only: fullmatch from a position after the start
    tail: Regex,
    pub groups: usize,
    pub names: Vec<(String, usize)>,
}

/// A match: the text, each group's byte span (None when the group did not take part), the group names.
pub struct PyMatch {
    pub text: Rc<str>,
    pub spans: Vec<Option<(usize, usize)>>,
    pub names: Vec<(String, usize)>,
}

fn re_error(msg: impl std::fmt::Display) -> PyErr {
    PyErr::Other(format!("re.error: {msg}"))
}

/// The pattern in fancy-regex syntax (see the module note); an error for what is not modelled.
fn convert(p: &str, flags: i64) -> Result<String, PyErr> {
    let cs: Vec<char> = p.chars().collect();
    let mut out = String::new();
    let (mut i, mut in_class) = (0usize, false);
    while i < cs.len() {
        let c = cs[i];
        if c == '\\' {
            let n = *cs.get(i + 1).ok_or_else(|| re_error("bad escape (end of pattern)"))?;
            if !in_class && n == 'Z' {
                out.push_str("\\z");
            } else {
                out.push(c);
                out.push(n);
            }
            i += 2;
            continue;
        }
        if in_class {
            if c == ']' {
                in_class = false;
            }
            if c == '[' {
                out.push_str("\\[");                     // Python reads a [ inside a class as a literal
            } else {
                out.push(c);
            }
            i += 1;
            continue;
        }
        match c {
            '[' => {
                in_class = true;
                out.push('[');
                i += 1;
                // a ] right after [ or [^ is a literal in Python
                if cs.get(i) == Some(&'^') {
                    out.push('^');
                    i += 1;
                }
                if cs.get(i) == Some(&']') {
                    out.push_str("\\]");
                    i += 1;
                }
            }
            '$' if flags & MULTILINE == 0 => {
                out.push_str("(?:\\z|(?=\\n\\z))");
                i += 1;
            }
            '{' => {
                // a quantifier {m}, {m,}, {m,n} or {,n}; anything else is the literal {
                let rest: String = cs[i..].iter().take_while(|&&x| x != '}').collect();
                let body = &rest[1..];
                let closed = cs.len() > i + rest.chars().count();
                let ok = closed && {
                    let (a, b) = body.split_once(',').map(|(a, b)| (a, Some(b))).unwrap_or((body, None));
                    let num = |s: &str| !s.is_empty() && s.chars().all(|x| x.is_ascii_digit());
                    match b {
                        None => num(a),
                        Some(b) => (a.is_empty() || num(a)) && (b.is_empty() || num(b)) && !(a.is_empty() && b.is_empty()),
                    }
                };
                if ok {
                    let body = if body.starts_with(',') { format!("0{body}") } else { body.to_string() };
                    out.push('{');
                    out.push_str(&body);
                    out.push('}');
                    i += rest.chars().count() + 1;
                } else {
                    out.push_str("\\{");
                    i += 1;
                }
            }
            '}' => {
                out.push_str("\\}");
                i += 1;
            }
            _ => {
                out.push(c);
                i += 1;
            }
        }
    }
    Ok(out)
}

thread_local! {
    static CACHE: RefCell<HashMap<(String, i64), Rc<PyRe>>> = RefCell::new(HashMap::new());
}

/// re.compile(pattern, flags), cached as Python caches compiled patterns.
pub fn compile(pattern: &str, flags: i64) -> Result<Rc<PyRe>, PyErr> {
    if let Some(r) = CACHE.with(|c| c.borrow().get(&(pattern.to_string(), flags)).cloned()) {
        return Ok(r);
    }
    if flags & ASCII != 0 || flags & !(IGNORECASE | MULTILINE | DOTALL | VERBOSE | ASCII) != 0 {
        return Err(PyErr::Unsupported(format!("re flags {flags}")));
    }
    let mut pre = String::new();
    for (f, ch) in [(IGNORECASE, 'i'), (MULTILINE, 'm'), (DOTALL, 's'), (VERBOSE, 'x')] {
        if flags & f != 0 {
            pre.push(ch);
        }
    }
    let pre = if pre.is_empty() { String::new() } else { format!("(?{pre})") };
    let body = convert(pattern, flags)?;
    let mk = |s: String| Regex::new(&s).map_err(re_error);
    let search = mk(format!("{pre}{body}"))?;
    let anchored = mk(format!("{pre}\\A(?:{body})"))?;
    let full = mk(format!("{pre}\\A(?:{body})\\z"))?;
    let tail = mk(format!("{pre}(?:{body})\\z"))?;
    let groups = search.captures_len() - 1;
    let names = search.capture_names().enumerate().filter_map(|(k, n)| n.map(|n| (n.to_string(), k))).collect();
    let r = Rc::new(PyRe { pattern: pattern.to_string(), flags, search, anchored, full, tail, groups, names });
    CACHE.with(|c| c.borrow_mut().insert((pattern.to_string(), flags), r.clone()));
    Ok(r)
}

fn to_match(re: &PyRe, text: &Rc<str>, caps: &fancy_regex::Captures) -> PyMatch {
    PyMatch {
        text: text.clone(),
        spans: (0..=re.groups).map(|k| caps.get(k).map(|m| (m.start(), m.end()))).collect(),
        names: re.names.clone(),
    }
}

/// search / match / fullmatch: a Match, or None.
pub fn find(re: &PyRe, how: &str, text: &Rc<str>) -> R {
    find_at(re, how, text, 0, None)
}

/// The byte offsets of Python's pos and endpos (character indices, clamped to the string as Python clamps them).
fn bounds(text: &str, pos: i64, endpos: Option<i64>) -> (usize, usize) {
    let n = text.chars().count() as i64;
    let byte = |k: i64| text.char_indices().nth(k.clamp(0, n) as usize).map(|(b, _)| b).unwrap_or(text.len());
    (byte(pos), byte(endpos.unwrap_or(n)))
}

/// p.search / p.match / p.fullmatch(text, pos, endpos): the text is read as if it ended at endpos ($ and \z match
/// there); the search starts at pos, but ^ and \A still match only at the real start (as Python's); match and
/// fullmatch must begin exactly at pos. Spans are of the whole text.
pub fn find_at(re: &PyRe, how: &str, text: &Rc<str>, pos: i64, endpos: Option<i64>) -> R {
    let (p, e) = bounds(text, pos, endpos);
    if e < p {
        return Ok(Py::None);
    }
    let sub = &text[..e];
    let caps = if p == 0 {
        let rx = match how {
            "search" => &re.search,
            "match" => &re.anchored,
            _ => &re.full,
        };
        rx.captures(sub).map_err(re_error)?
    } else {
        // the leftmost match from pos: when any match begins at pos, it is the one an anchored match finds
        let rx = if how == "fullmatch" { &re.tail } else { &re.search };
        rx.captures_from_pos(sub, p).map_err(re_error)?
            .filter(|c| how == "search" || c.get(0).map(|m| m.start()) == Some(p))
    };
    match caps {
        Some(c) => Ok(Py::Match(Rc::new(to_match(re, text, &c)))),
        None => Ok(Py::None),
    }
}

fn all_captures<'t>(re: &PyRe, text: &'t str) -> Result<Vec<fancy_regex::Captures<'t>>, PyErr> {
    re.search.captures_iter(text).map(|c| c.map_err(re_error)).collect()
}

/// The matches of the text read up to endpos, from pos on (Python's finditer order; after an empty match the next
/// search starts one character later).
fn captures_between<'t>(re: &PyRe, text: &'t str, pos: i64, endpos: Option<i64>)
                        -> Result<Vec<fancy_regex::Captures<'t>>, PyErr> {
    let (p, e) = bounds(text, pos, endpos);
    if p == 0 && e == text.len() {
        return all_captures(re, text);
    }
    let sub = &text[..e.max(p)];
    let (mut out, mut at) = (vec![], p);
    while at <= sub.len() {
        let Some(c) = re.search.captures_from_pos(sub, at).map_err(re_error)? else { break };
        let m = c.get(0).unwrap();
        at = if m.end() > m.start() { m.end() } else {
            m.end() + sub[m.end()..].chars().next().map_or(1, char::len_utf8)
        };
        out.push(c);
    }
    Ok(out)
}

/// The byte spans of every match (finditer's order), for Rust callers that rewrite text around matches.
pub fn findall_spans(re: &PyRe, text: &str) -> Result<Vec<(usize, usize)>, PyErr> {
    Ok(all_captures(re, text)?.iter().map(|c| { let m = c.get(0).unwrap(); (m.start(), m.end()) }).collect())
}

/// findall: the matched strings (no group), the group (one), or tuples of the groups (several); "" for a group
/// that did not take part.
pub fn findall(re: &PyRe, text: &str) -> R {
    findall_at(re, text, 0, None)
}

/// p.findall(text, pos, endpos)
pub fn findall_at(re: &PyRe, text: &str, pos: i64, endpos: Option<i64>) -> R {
    let mut out = vec![];
    for c in captures_between(re, text, pos, endpos)? {
        let g = |k: usize| Py::str(c.get(k).map(|m| m.as_str()).unwrap_or(""));
        out.push(match re.groups {
            0 => g(0),
            1 => g(1),
            n => Py::tuple((1..=n).map(g).collect()),
        });
    }
    Ok(Py::list(out))
}

/// A replacement template as Python expands it: \1 .. \99, \g<n>, \g<name>, \n \t \r \f \v \a \\ ; another escaped
/// ASCII letter is an error, anything else stays as written.
fn expand(template: &str, re: &PyRe, c: &fancy_regex::Captures) -> Result<String, PyErr> {
    let cs: Vec<char> = template.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    let group = |k: usize| -> Result<String, PyErr> {
        if k > re.groups {
            return Err(re_error(format!("invalid group reference {k}")));
        }
        Ok(c.get(k).map(|m| m.as_str().to_string()).unwrap_or_default())
    };
    while i < cs.len() {
        if cs[i] != '\\' || i + 1 == cs.len() {
            out.push(cs[i]);
            i += 1;
            continue;
        }
        let n = cs[i + 1];
        match n {
            '0'..='9' => {
                let mut k = n.to_digit(10).unwrap() as usize;
                i += 2;
                if let Some(d) = cs.get(i).and_then(|x| x.to_digit(10)) {
                    k = k * 10 + d as usize;
                    i += 1;
                }
                out.push_str(&group(k)?);
            }
            'g' => {
                let close = cs[i..].iter().position(|&x| x == '>').ok_or_else(|| re_error("missing >"))?;
                let name: String = cs[i + 3..i + close].iter().collect();
                let k = match name.parse::<usize>() {
                    Ok(k) => k,
                    Err(_) => re.names.iter().find(|(n, _)| *n == name).map(|(_, k)| *k)
                        .ok_or_else(|| re_error(format!("unknown group name '{name}'")))?,
                };
                out.push_str(&group(k)?);
                i += close + 1;
            }
            'n' | 't' | 'r' | 'f' | 'v' | 'a' | '\\' => {
                out.push(match n { 'n' => '\n', 't' => '\t', 'r' => '\r', 'f' => '\x0c', 'v' => '\x0b', 'a' => '\x07', _ => '\\' });
                i += 2;
            }
            x if x.is_ascii_alphabetic() => return Err(re_error(format!("bad escape \\{x}"))),
            x => {
                out.push('\\');
                out.push(x);
                i += 2;
            }
        }
    }
    Ok(out)
}

/// sub(repl, text, count): repl a template or a function of the Match; count 0 replaces all.
pub fn sub(re: &PyRe, repl: &Py, text: &Rc<str>, count: i64) -> R {
    let mut out = String::new();
    let mut last = 0;
    for (n, c) in all_captures(re, text)?.into_iter().enumerate() {
        if count > 0 && n as i64 >= count {
            break;
        }
        let m = c.get(0).unwrap();
        out.push_str(&text[last..m.start()]);
        let piece = match repl {
            Py::Str(t) => expand(t, re, &c)?,
            Py::Func(_) => crate::py::to_str(&crate::py::call(repl, &[Py::Match(Rc::new(to_match(re, text, &c)))])?),
            _ => return Err(PyErr::Type("repl must be a str or a function".into())),
        };
        out.push_str(&piece);
        last = m.end();
    }
    out.push_str(&text[last..]);
    Ok(Py::str(&out))
}

/// split(text, maxsplit): the pieces between matches, each match's groups between them.
pub fn split(re: &PyRe, text: &str, maxsplit: i64) -> R {
    let mut out = vec![];
    let mut last = 0;
    for (n, c) in all_captures(re, text)?.into_iter().enumerate() {
        if maxsplit > 0 && n as i64 >= maxsplit {
            break;
        }
        let m = c.get(0).unwrap();
        out.push(Py::str(&text[last..m.start()]));
        for k in 1..=re.groups {
            out.push(c.get(k).map(|g| Py::str(g.as_str())).unwrap_or(Py::None));
        }
        last = m.end();
    }
    out.push(Py::str(&text[last..]));
    Ok(Py::list(out))
}

impl PyMatch {
    fn index(&self, g: &Py) -> Result<usize, PyErr> {
        match g {
            Py::Int(k) if *k >= 0 && (*k as usize) < self.spans.len() => Ok(*k as usize),
            Py::Str(s) => self.names.iter().find(|(n, _)| **n == **s).map(|(_, k)| *k)
                .ok_or_else(|| PyErr::Index),
            _ => Err(PyErr::Index),
        }
    }

    fn chars(&self, byte: usize) -> i64 {
        self.text[..byte].chars().count() as i64
    }

    /// m.group(g), m[g]: the group's text, None when it did not take part
    pub fn group(&self, g: &Py) -> R {
        Ok(match self.spans[self.index(g)?] {
            Some((a, b)) => Py::str(&self.text[a..b]),
            None => Py::None,
        })
    }

    /// group / groups / groupdict / start / end / span, as Python's Match methods
    pub fn method(&self, name: &str, a: &[Py]) -> R {
        let g0 = Py::Int(0);
        match name {
            "group" if a.len() <= 1 => self.group(a.first().unwrap_or(&g0)),
            "group" => Ok(Py::tuple(a.iter().map(|g| self.group(g)).collect::<Result<_, _>>()?)),
            "groups" => {
                let d = a.first().cloned().unwrap_or(Py::None);
                Ok(Py::tuple((1..self.spans.len()).map(|k| match self.spans[k] {
                    Some((x, y)) => Py::str(&self.text[x..y]),
                    None => d.clone(),
                }).collect()))
            }
            "groupdict" => {
                let d = a.first().cloned().unwrap_or(Py::None);
                Ok(Py::dict(self.names.iter().map(|(n, k)| (Py::str(n), match self.spans[*k] {
                    Some((x, y)) => Py::str(&self.text[x..y]),
                    None => d.clone(),
                })).collect()))
            }
            "start" | "end" | "span" => {
                let k = self.index(a.first().unwrap_or(&g0))?;
                let (s, e) = match self.spans[k] {
                    Some((x, y)) => (self.chars(x), self.chars(y)),
                    None => (-1, -1),
                };
                Ok(match name {
                    "start" => Py::Int(s),
                    "end" => Py::Int(e),
                    _ => Py::tuple(vec![Py::Int(s), Py::Int(e)]),
                })
            }
            _ => Err(PyErr::Unsupported(format!("Match.{name}"))),
        }
    }
}

/// The module-level functions (re.search(p, s, flags) ...) and a compiled pattern's methods (p.search(s) ...).
pub fn call(re: &PyRe, name: &str, a: &[Py]) -> R {
    let text = |k: usize| -> Result<Rc<str>, PyErr> { a.get(k).ok_or_else(|| PyErr::Type(format!("{name}()")))?.as_str() };
    let int = |k: usize| -> Result<i64, PyErr> { a.get(k).map(|x| x.i()).unwrap_or(Ok(0)) };
    // pos and endpos of a compiled pattern's search, match, fullmatch and findall (None: the end)
    let endpos = || -> Result<Option<i64>, PyErr> {
        match a.get(2) {
            None | Some(Py::None) => Ok(None),
            Some(x) => x.i().map(Some),
        }
    };
    match name {
        "search" | "match" | "fullmatch" => find_at(re, name, &text(0)?, int(1)?, endpos()?),
        "findall" => findall_at(re, &text(0)?, int(1)?, endpos()?),
        "sub" => sub(re, a.first().ok_or_else(|| PyErr::Type("sub()".into()))?, &text(1)?, int(2)?),
        "split" => split(re, &text(0)?, int(1)?),
        _ => Err(PyErr::Unsupported(format!("re.{name}"))),
    }
}

/// re.<name>(pattern, ...args, flags): the arguments after the pattern as the compiled pattern's method takes them.
pub fn module_call(name: &str, pattern: &Py, rest: &[Py], flags: i64) -> R {
    let p = match pattern {
        Py::Re(r) => r.clone(),
        _ => compile(&pattern.as_str()?, flags)?,
    };
    if name == "compile" {
        return Ok(Py::Re(p));
    }
    call(&p, name, rest)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(x: &str) -> Py {
        Py::str(x)
    }

    #[test]
    fn positions_as_python() {
        // p.match(s, pos) begins at pos (a formula reader loops on it: ignoring pos never ended, 2026-10-29)
        let s = |x: &str| Rc::<str>::from(x);
        let span = |m: R| match m.unwrap() {
            Py::Match(m) => repr(&m.method("span", &[]).unwrap()),
            other => repr(&other),
        };
        use crate::py::repr;
        let p = compile(r"([A-Z][a-z]?)(\d*)", 0).unwrap();
        assert_eq!(span(find_at(&p, "match", &s("C6H6"), 2, None)), "(2, 4)");
        assert_eq!(span(find_at(&p, "match", &s("C6H6"), 1, None)), "None");
        assert_eq!(span(find_at(&compile("C", 0).unwrap(), "search", &s("xxC"), 1, None)), "(2, 3)");
        assert_eq!(span(find_at(&compile("bc", 0).unwrap(), "fullmatch", &s("abc"), 1, None)), "(1, 3)");
        assert_eq!(span(find_at(&compile("^a", 0).unwrap(), "search", &s("ba"), 1, None)), "None");
        assert_eq!(span(find_at(&compile("c$", 0).unwrap(), "search", &s("abcd"), 0, Some(3))), "(2, 3)");
        assert_eq!(span(find_at(&compile("b", 0).unwrap(), "match", &s("abc"), 5, None)), "None");
        assert_eq!(span(find_at(&compile("", 0).unwrap(), "search", &s("ab"), 1, Some(0))), "None");
        assert_eq!(repr(&findall_at(&compile(r"\d", 0).unwrap(), "a1b2c3", 2, None).unwrap()), "['2', '3']");
        assert_eq!(repr(&findall_at(&compile("a*", 0).unwrap(), "baaa", 1, None).unwrap()), "['aaa', '']");
    }

    #[test]
    fn as_python_does() {
        let t: Rc<str> = Rc::from("ACGTN acgt");
        let p = compile("[ACGT]+", IGNORECASE).unwrap();
        assert!(matches!(find(&p, "fullmatch", &Rc::from("acgT")).unwrap(), Py::Match(_)));
        assert!(matches!(find(&p, "fullmatch", &t).unwrap(), Py::None));
        assert_eq!(crate::py::repr(&findall(&p, &t).unwrap()), "['ACGT', 'acgt']");
        let q = compile(r"(\d+)-(\d+)", 0).unwrap();
        assert_eq!(crate::py::repr(&findall(&q, "1-2 and 30-40").unwrap()), "[('1', '2'), ('30', '40')]");
        assert_eq!(crate::py::repr(&sub(&q, &s(r"\2:\g<1>"), &Rc::from("a 1-2 b"), 0).unwrap()), "'a 2:1 b'");
        let sp = compile(r"\s*,\s*", 0).unwrap();
        assert_eq!(crate::py::repr(&split(&sp, "a , b,c", 0).unwrap()), "['a', 'b', 'c']");
        let lb = compile(r"(?<=\d)\*(?=[a-z])", 0).unwrap();
        assert_eq!(crate::py::repr(&sub(&lb, &s(""), &Rc::from("2*x + 3*4"), 0).unwrap()), "'2x + 3*4'");
        // $ before a final newline, a literal {, {,n}
        let d = compile(r"ab$", 0).unwrap();
        assert!(matches!(find(&d, "search", &Rc::from("ab\n")).unwrap(), Py::Match(_)));
        let br = compile(r"x{y}|a{,2}b", 0).unwrap();
        assert!(matches!(find(&br, "fullmatch", &Rc::from("x{y}")).unwrap(), Py::Match(_)));
        assert!(matches!(find(&br, "fullmatch", &Rc::from("aab")).unwrap(), Py::Match(_)));
        // positions in characters
        let u = compile("b", 0).unwrap();
        if let Py::Match(m) = find(&u, "search", &Rc::from("²ab")).unwrap() {
            assert_eq!(crate::py::repr(&m.method("span", &[]).unwrap()), "(2, 3)");
        } else {
            panic!("no match");
        }
    }
}
