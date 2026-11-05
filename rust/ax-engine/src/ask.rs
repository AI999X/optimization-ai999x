//! Questions in words and capability search, with no model and no network: runtime/product.py `ask` (its documented
//! question forms) and a search over every capability, for the desktop apps and the browser.
//!
//! search_json(pack, query, limit)  -> [{key, name, kind, topic, score, snippet}]     prefix, token and typo-tolerant
//! topics_json(pack)                -> {model, topics: [{topic, capabilities: [{key, name, kind}]}]}
//! search_all_json(packs, q, limit) -> the same over several models, each result with its "model"
//! ask_json(pack, question)         -> {outcome, capability, name, kind, inputs, target, request, answer?,
//!                                      alternatives?, needs?, reading, message}
//!
//! Reading a question never guesses. Two readers, in order:
//!   1. the documented questions (`forms`: each with the Python package's own reading, exported with the pack): a
//!      question that matches one word for word, its numbers free, is read as the package reads it, each number in
//!      the place of the one it replaces;
//!   2. a reader of its own: the capability from the words (the same scoring as search), each number bound to the
//!      input whose meaning the words around it name, a formula's quantities by their unit's dimension and the asked
//!      quantity from the wording (never from which symbol lacks a value). It answers only when the reading is
//!      complete and has no rival; otherwise NEEDS_INPUT / AMBIGUOUS / OUT_OF_SCOPE with the capability and the inputs
//!      it could read, so a form can be pre-filled for the user to confirm.
//! Every answer comes from the engine (`solve`): the reader only fills the request. All work is bounded: question
//! length, tokens, list size, candidates.

use crate::engine::{solve, Answer, Input};
use crate::pack::{Pack, ProcInput, Procedure, Relation};
use crate::units::{parse_unit, to_si, Dim};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, OnceLock};

/// the longest question read (characters); a longer one is refused
pub const MAX_QUESTION: usize = 2000;
/// the longest search query (characters)
pub const MAX_QUERY: usize = 200;
const MAX_TOKENS: usize = 400;
const MAX_LIST_CHARS: usize = 4000;
const MAX_UNIT_TOKENS: usize = 7;
const MAX_CANDIDATES: usize = 12;
/// a second complete reading within this factor of the best one's word score makes the question AMBIGUOUS
const RIVAL: f32 = 1.2;
/// the words favour an unreadable capability over the complete reading by more than this: AMBIGUOUS
const FAVOURED: f32 = 1.5;
/// two complete readings giving a result of the same name: rivals unless the words favour one by this factor
const SAME_RESULT: f32 = 2.0;
/// the weight of writing a capability's whole name, in the routing score
const NAME_COVER: f32 = 4.0;
/// search only: the weight of the share of a capability's name the query covers
const SEARCH_NAME: f32 = 3.0;
/// a word used by at most about this many capabilities' texts is distinctive (see `unexplained`)
const DISTINCT_CAPS: f32 = 15.0;
/// routing score per word of the asked phrase the chosen result's meaning shares, and for the main result
const OUTPUT_MATCH: f32 = 1.0;
/// a number goes to an input only when its words name that input this many times more strongly than any other
const NUMBER_MARGIN: f32 = 1.5;
const PRIMARY: f32 = 1.5;
const MAX_LIMIT: usize = 200;

// ================================================================================================== tokens ====
#[derive(Clone, Debug, PartialEq)]
pub enum K {
    Word,
    Num,
    List,
    Punct,
}

/// One token of a question: its kind, its text in lower case (words are compared so) and as written (units are read
/// so: mW is not MW), a number's value, and whether white space came before it.
#[derive(Clone, Debug)]
pub struct Tok {
    pub k: K,
    pub low: String,
    pub orig: String,
    pub val: f64,
    pub gap: bool,
}

fn fold(c: char, out: &mut String) {
    match c {
        '\u{2212}' | '\u{2013}' | '\u{2014}' => out.push('-'),
        '\u{00d7}' | '\u{00b7}' | '\u{22c5}' => out.push('*'),
        '\u{2019}' | '\u{2018}' => out.push('\''),
        '\u{201c}' | '\u{201d}' => out.push('"'),
        '\u{2264}' => out.push_str("<="),
        '\u{2265}' => out.push_str(">="),
        '\u{00a0}' => out.push(' '),
        _ => out.push(c),
    }
}

/// The tokens of a text: words (letters, digits, '_', a hyphen or apostrophe between letters), numbers (a sign only
/// where it cannot be a minus: at the start or after a space or an opening mark), bracketed lists (one token, nested
/// brackets included) and single marks. At most MAX_TOKENS.
pub fn tokenize(text: &str) -> Vec<Tok> {
    let mut s = String::with_capacity(text.len().min(MAX_QUESTION * 2));
    for c in text.chars().take(MAX_QUESTION) {
        fold(c, &mut s);
    }
    let cs: Vec<char> = s.chars().collect();
    let n = cs.len();
    let digit = |k: usize| k < n && cs[k].is_ascii_digit();
    let starts_num = |k: usize| digit(k) || (k < n && cs[k] == '.' && digit(k + 1));
    let mut out: Vec<Tok> = vec![];
    let mut i = 0;
    while i < n && out.len() < MAX_TOKENS {
        let c = cs[i];
        if c.is_whitespace() {
            i += 1;
            continue;
        }
        let gap = i == 0 || cs[i - 1].is_whitespace();
        let tok = |k: K, a: usize, b: usize, val: f64| {
            let orig: String = cs[a..b].iter().collect();
            Tok { k, low: orig.to_lowercase(), orig, val, gap }
        };
        if c == '[' {
            let (mut depth, mut j, mut end) = (0i32, i, None);
            while j < n && j - i <= MAX_LIST_CHARS {
                if cs[j] == '[' {
                    depth += 1;
                } else if cs[j] == ']' {
                    depth -= 1;
                    if depth == 0 {
                        end = Some(j);
                        break;
                    }
                }
                j += 1;
            }
            if let Some(e) = end {
                out.push(tok(K::List, i, e + 1, f64::NAN));
                i = e + 1;
            } else {
                out.push(tok(K::Punct, i, i + 1, f64::NAN));
                i += 1;
            }
            continue;
        }
        // a parenthesised tuple of two or more numbers ("at (1, 2)", "P(10, 3)") is a list
        if c == '(' {
            let mut j = i + 1;
            while j < n && j - i <= MAX_LIST_CHARS && cs[j] != ')' && cs[j] != '(' {
                j += 1;
            }
            if j < n && cs[j] == ')' {
                let inner: String = cs[i + 1..j].iter().collect();
                let parts: Vec<&str> = inner.split(',').map(|x| x.trim()).collect();
                let numeric = |x: &&str| !x.is_empty() && x.chars().any(|c| c.is_ascii_digit())
                    && x.chars().all(|c| c.is_ascii_digit() || "+-.eE".contains(c)) && x.parse::<f64>().is_ok();
                if parts.len() >= 2 && parts.len() <= 2000 && parts.iter().all(numeric) {
                    let orig = format!("({})", parts.join(", "));
                    out.push(Tok { k: K::List, low: orig.clone(), orig, val: f64::NAN, gap });
                    i = j + 1;
                    continue;
                }
            }
        }
        let prev = if i == 0 { None } else { Some(cs[i - 1]) };
        let signed = (c == '-' || c == '+')
            && starts_num(i + 1)
            && match prev {
                None => true,
                Some(p) => p.is_whitespace() || "([{,=:;<>".contains(p),
            };
        if starts_num(i) || signed {
            let st = i;
            if signed {
                i += 1;
            }
            while digit(i) {
                i += 1;
            }
            if i < n && cs[i] == '.' && digit(i + 1) {
                i += 1;
                while digit(i) {
                    i += 1;
                }
            }
            if i < n && (cs[i] == 'e' || cs[i] == 'E') {
                let mut k = i + 1;
                if k < n && (cs[k] == '+' || cs[k] == '-') {
                    k += 1;
                }
                if digit(k) {
                    let mut m = k;
                    while digit(m) {
                        m += 1;
                    }
                    if !(m < n && cs[m].is_alphabetic()) {
                        i = m;
                    }
                }
            }
            let t: String = cs[st..i].iter().collect();
            let v = t.parse::<f64>().unwrap_or(f64::NAN);
            out.push(tok(K::Num, st, i, v));
            continue;
        }
        if c.is_alphabetic() || c == '_' {
            let st = i;
            while i < n
                && (cs[i].is_alphanumeric()
                    || cs[i] == '_'
                    || ((cs[i] == '-' || cs[i] == '\'') && i > st && i + 1 < n && cs[i + 1].is_alphabetic()))
            {
                i += 1;
            }
            out.push(tok(K::Word, st, i, f64::NAN));
            continue;
        }
        out.push(tok(K::Punct, i, i + 1, f64::NAN));
        i += 1;
    }
    out
}

const STOP: &[&str] = &[
    "a", "an", "the", "of", "in", "on", "at", "to", "for", "and", "or", "is", "are", "was", "were", "be", "been", "by",
    "with", "from", "as", "that", "this", "these", "those", "it", "its", "what", "which", "who", "whom", "find",
    "compute", "calculate", "determine", "given", "give", "using", "use", "how", "does", "do", "if", "when", "where",
    "there", "then", "than", "into", "per", "has", "have", "had", "will", "would", "can", "could", "should", "must",
    "we", "you", "your", "our", "i", "me", "my", "they", "their", "he", "she", "his", "her", "them", "so", "such",
    "each", "any", "all", "some", "about", "after", "before", "over", "under", "between", "through", "during", "out",
    "up", "down", "off", "not", "no", "only", "also", "very", "just", "please", "evaluate", "obtain", "get", "whats",
    "what's", "let", "suppose", "assume", "consider", "value", "result", "answer",
];

fn stopword(w: &str) -> bool {
    STOP.contains(&w)
}

/// A word as compared: lower case, a plural 's' dropped ("trials" = "trial"), a Greek letter by its name.
pub fn stem(w: &str) -> String {
    let w = match w {
        "\u{03c3}" => "sigma",
        "\u{03bc}" | "\u{00b5}" => "mu",
        "\u{03bb}" => "lambda",
        "\u{03b1}" => "alpha",
        "\u{03b2}" => "beta",
        "\u{03c1}" => "rho",
        "\u{03b8}" => "theta",
        "\u{03c9}" => "omega",
        "\u{03b4}" | "\u{0394}" => "delta",
        "\u{03c0}" => "pi",
        _ => w,
    };
    if w == "times" {
        return w.to_string();                               // "2 times a year" is a frequency, not a time
    }
    let n = w.chars().count();
    if n > 4 && w.ends_with("ies") {
        return format!("{}y", &w[..w.len() - 3]);
    }
    if n > 3 && w.ends_with('s') && !w.ends_with("ss") && !w.ends_with("us") && !w.ends_with("is") {
        return w[..w.len() - 1].to_string();
    }
    w.to_string()
}

/// The content words of a text, stemmed (hyphenated words also give their parts), in order.
pub fn words(text: &str) -> Vec<String> {
    let mut out = vec![];
    for t in tokenize(text) {
        if t.k != K::Word {
            continue;
        }
        let w = t.low.replace('\'', "");
        let s = stem(&w);
        if !stopword(&w) && !stopword(&s) {
            out.push(s);
        }
        if w.contains('-') {
            for part in w.split('-') {
                if !part.is_empty() && !stopword(part) {
                    out.push(stem(part));
                }
            }
        }
    }
    out
}

fn is_word(t: &Tok) -> bool {
    t.k == K::Word
}

// =================================================================================================== index ====
struct CapDoc {
    key: String,
    name: String,
    kind: &'static str,
    topic: String,
    /// (weight, words) per field: name, key, keywords, topic, inputs and outputs, documented questions, method
    fields: Vec<(f32, HashSet<String>)>,
    /// texts a snippet may come from: documented questions first, then the equation or method
    snippets: Vec<String>,
    lead: String,
}

struct Index {
    caps: Vec<CapDoc>,
    idf: HashMap<String, f32>,
    vocab: Vec<String>,
    forms: Vec<PForm>,
    /// every input symbol of the model, lower case (a value written "name = 3" with a name of another capability)
    symbols: HashSet<String>,
    /// each capability's name words with their weight (how much of its name a question writes)
    name_idf: Vec<(Vec<String>, f32)>,
    /// the procedures that take required inputs of each signature (see `signature`)
    siblings: HashMap<Vec<(String, String)>, Vec<String>>,
}

/// The kinds of a procedure's required inputs, in a fixed order: two procedures with the same signature read the
/// same values, and only the wording says which one is meant.
fn signature(pr: &Procedure) -> Vec<(String, String)> {
    signature_of(pr, true)
}

/// The kinds of a procedure's inputs (required only, or all), in a fixed order.
fn signature_of(pr: &Procedure, required_only: bool) -> Vec<(String, String)> {
    let mut s: Vec<(String, String)> = pr.inputs.iter().filter(|i| i.required || !required_only)
        .map(|i| (if i.ty.is_empty() { "number".to_string() } else { i.ty.clone() }, i.kind.clone())).collect();
    s.sort();
    s
}

/// The capabilities taking inputs of the same kinds as `pr` (itself included): only the wording tells them apart.
fn siblings(ix: &Index, pr: &Procedure) -> Vec<String> {
    let mut all_sig = signature_of(pr, false);
    all_sig.push(("*".into(), "all".into()));
    let mut sibs: Vec<String> = vec![];
    for sig in [signature(pr), all_sig] {
        for k in ix.siblings.get(&sig).into_iter().flatten() {
            if !sibs.contains(k) {
                sibs.push(k.clone());
            }
        }
    }
    sibs
}

/// The distinctive words of a capability's name (stemmed, more than two letters, not generic), each with the
/// hyphenated word it is part of ("two-sample"), or None when the name writes it on its own.
fn cap_name_words(name: &str) -> Vec<(String, Option<String>)> {
    let keep = |w: &str| w.chars().count() > 2 && !stopword(w) && !GENERIC.contains(&w);
    let mut out = vec![];
    for t in tokenize(name).iter().filter(|t| is_word(t)) {
        let w = t.low.replace('\'', "");
        if w.contains('-') {
            for part in w.split('-').map(stem) {
                if keep(&part) {
                    out.push((part, Some(w.clone())));
                }
            }
        } else {
            let s = stem(&w);
            if keep(&s) && !stopword(&w) {
                out.push((s, None));
            }
        }
    }
    out
}

fn add_field(fields: &mut Vec<(f32, HashSet<String>)>, w: f32, texts: &[&str]) {
    let mut set = HashSet::new();
    for t in texts {
        set.extend(words(t));
    }
    fields.push((w, set));
}

fn build_index(p: &Pack) -> Index {
    let mut caps = vec![];
    for r in &p.relations {
        let mut f = vec![];
        add_field(&mut f, 4.0, &[&r.name]);
        add_field(&mut f, 3.5, &[&r.key.replace('_', " ")]);
        add_field(&mut f, 2.0, &[&r.family.replace('_', " ")]);
        let syms: Vec<String> = r.symbols.iter().map(|s| format!("{} {}", s.desc, s.kind.replace('_', " "))).collect();
        add_field(&mut f, 1.5, &syms.iter().map(|s| s.as_str()).collect::<Vec<_>>());
        add_field(&mut f, 1.0, &r.examples.iter().map(|s| s.as_str()).collect::<Vec<_>>());
        add_field(&mut f, 0.7, &r.assumptions.iter().map(|s| s.as_str()).collect::<Vec<_>>());
        let mut snippets = r.examples.clone();
        snippets.push(r.equation.clone());
        caps.push(CapDoc { key: r.key.clone(), name: r.name.clone(), kind: "formula",
                           topic: if r.family.is_empty() { "formulas".into() } else { r.family.replace('_', " ") },
                           fields: f, snippets, lead: r.equation.clone() });
    }
    for pr in &p.procedures {
        let mut f = vec![];
        add_field(&mut f, 4.0, &[&pr.name]);
        add_field(&mut f, 3.5, &[&pr.key.replace('_', " ")]);
        add_field(&mut f, 3.0, &pr.keywords.iter().map(|s| s.as_str()).collect::<Vec<_>>());
        add_field(&mut f, 2.0, &[&pr.topic.replace('_', " ")]);
        let io: Vec<String> = pr.inputs.iter().map(|i| i.meaning.clone())
            .chain(pr.outputs.iter().map(|o| o.meaning.clone())).collect();
        add_field(&mut f, 1.5, &io.iter().map(|s| s.as_str()).collect::<Vec<_>>());
        let docs: Vec<&str> = pr.templates.iter().chain(pr.questions.iter()).map(|s| s.as_str()).collect();
        add_field(&mut f, 1.0, &docs);
        add_field(&mut f, 0.7, &[&pr.method]);
        let mut snippets: Vec<String> = pr.templates.iter().chain(pr.questions.iter()).cloned().collect();
        snippets.push(pr.method.clone());
        caps.push(CapDoc { key: pr.key.clone(), name: pr.name.clone(), kind: "procedure",
                           topic: if pr.topic.is_empty() { "other".into() } else { pr.topic.replace('_', " ") },
                           fields: f, snippets, lead: pr.method.clone() });
    }
    let mut df: HashMap<String, u32> = HashMap::new();
    for c in &caps {
        let mut all: HashSet<&String> = HashSet::new();
        for (_, ws) in &c.fields {
            all.extend(ws.iter());
        }
        for w in all {
            *df.entry(w.clone()).or_insert(0) += 1;
        }
    }
    let n = caps.len().max(1) as f32;
    let idf: HashMap<String, f32> = df.iter().map(|(w, d)| (w.clone(), (1.0 + n / (*d as f32)).ln())).collect();
    let mut vocab: Vec<String> = idf.keys().cloned().collect();
    vocab.sort();
    let forms = p.forms.iter().filter_map(|f| prepare_form(p, f)).collect();
    let mut symbols: HashSet<String> = HashSet::new();
    for r in &p.relations {
        symbols.extend(r.symbols.iter().map(|s| s.name.to_lowercase()));
    }
    for pr in &p.procedures {
        symbols.extend(pr.inputs.iter().map(|i| i.symbol.to_lowercase()));
    }
    let name_idf = caps.iter().map(|c| {
        let mut ws: Vec<String> = words(&c.name);
        ws.sort();                                         // a fixed order: float sums come out the same everywhere
        ws.dedup();
        let total = ws.iter().map(|w| idf.get(w).copied().unwrap_or(0.0)).sum::<f32>();
        (ws, total)
    }).collect();
    let mut siblings: HashMap<Vec<(String, String)>, Vec<String>> = HashMap::new();
    for pr in &p.procedures {
        siblings.entry(signature(pr)).or_default().push(pr.key.clone());
        // all inputs too (one capability's optional input is another's required one): marked apart by a sentinel
        let mut all = signature_of(pr, false);
        all.push(("*".into(), "all".into()));
        siblings.entry(all).or_default().push(pr.key.clone());
    }
    Index { caps, idf, vocab, forms, symbols, name_idf, siblings }
}

fn index(p: &Pack) -> Arc<Index> {
    static CACHE: OnceLock<Mutex<Vec<((usize, u64), Arc<Index>)>>> = OnceLock::new();
    // the pack's address and a fingerprint of its content: a pack dropped and another loaded at the same address
    // never meets the first one's index
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    (&p.model, &p.version, p.forms.len()).hash(&mut h);
    for r in &p.relations {
        r.key.hash(&mut h);
    }
    for pr in &p.procedures {
        (&pr.key, &pr.name, pr.templates.len(), pr.inputs.len()).hash(&mut h);
    }
    let id = (p as *const Pack as usize, h.finish());
    let cache = CACHE.get_or_init(|| Mutex::new(vec![]));
    if let Ok(c) = cache.lock() {
        if let Some((_, ix)) = c.iter().find(|(k, _)| *k == id) {
            return ix.clone();
        }
    }
    let ix = Arc::new(build_index(p));
    if let Ok(mut c) = cache.lock() {
        if c.len() >= 64 {
            c.remove(0);
        }
        c.push((id, ix.clone()));
    }
    ix
}

/// Levenshtein distance, giving up (None) beyond `max`.
fn edit_within(a: &str, b: &str, max: usize) -> Option<usize> {
    let a: Vec<char> = a.chars().take(40).collect();
    let b: Vec<char> = b.chars().take(40).collect();
    if a.len().abs_diff(b.len()) > max {
        return None;
    }
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    for i in 1..=a.len() {
        let mut cur = vec![i; b.len() + 1];
        let mut best = cur[0];
        for j in 1..=b.len() {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            cur[j] = (prev[j] + 1).min(cur[j - 1] + 1).min(prev[j - 1] + cost);
            best = best.min(cur[j]);
        }
        if best > max {
            return None;
        }
        prev = cur;
    }
    if prev[b.len()] <= max { Some(prev[b.len()]) } else { None }
}

/// The vocabulary words a query word stands for, each with how well it matches: the word itself 1, a word it begins
/// (typing in progress) 0.85 for the last word and 0.7 otherwise, one typo 0.65, two typos in a long word 0.5.
fn expand(ix: &Index, q: &str, last: bool) -> Vec<(String, f32)> {
    let mut out: Vec<(String, f32)> = vec![];
    let qn = q.chars().count();
    for v in &ix.vocab {
        let mut m = 0.0f32;
        if v == q {
            m = 1.0;
        } else if qn >= 2 && v.starts_with(q) && (last || qn >= 3) {
            m = if last { 0.85 } else { 0.7 };
        } else if qn >= 4 {
            let max = if qn >= 7 { 2 } else { 1 };
            if let Some(d) = edit_within(q, v, max) {
                m = if d <= 1 { 0.65 } else { 0.5 };
            }
        }
        if m > 0.0 {
            out.push((v.clone(), m));
        }
    }
    out.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap().then(a.0.cmp(&b.0)));
    out.truncate(40);
    out
}

/// Each capability's score for a list of query words (expanded), highest first: (index, score).
fn score_caps(ix: &Index, qwords: &[Vec<(String, f32)>]) -> Vec<(usize, f32)> {
    let mut scores = vec![];
    for (ci, c) in ix.caps.iter().enumerate() {
        let mut total = 0.0f32;
        for exp in qwords {
            let mut best = 0.0f32;
            for (w, m) in exp {
                let idf = ix.idf.get(w).copied().unwrap_or(0.0);
                for (fw, set) in &c.fields {
                    if set.contains(w) {
                        best = best.max(fw * m * idf);
                    }
                }
            }
            total += best;
        }
        if total > 0.0 {
            scores.push((ci, total));
        }
    }
    scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap().then(ix.caps[a.0].key.cmp(&ix.caps[b.0].key)));
    scores
}

fn snippet(c: &CapDoc, matched: &HashSet<String>) -> String {
    let pick = c.snippets.iter().find(|s| words(s).iter().any(|w| matched.contains(w))).unwrap_or(&c.lead);
    let mut s: String = pick.chars().take(160).collect();
    if pick.chars().count() > 160 {
        s.push('\u{2026}');
    }
    s
}

fn search_results(p: &Pack, query: &str, limit: usize) -> Vec<Value> {
    let ix = index(p);
    let q: String = query.chars().take(MAX_QUERY).collect();
    let typing = !q.ends_with(char::is_whitespace);
    let toks = tokenize(&q);
    let mut raw: Vec<String> = toks.iter().filter(|t| is_word(t)).map(|t| t.low.replace('\'', "")).collect();
    raw.truncate(12);
    let n = raw.len();
    let mut qwords = vec![];
    for (i, w) in raw.iter().enumerate() {
        let last = typing && i + 1 == n;
        if stopword(w) && !last {
            continue;
        }
        qwords.push(expand(&ix, &stem(w), last));
        if w.contains('-') {
            for part in w.split('-').filter(|x| x.len() > 1) {
                qwords.push(expand(&ix, &stem(part), false));
            }
        }
    }
    let ql = q.trim().to_lowercase();
    let mut scored = rank(&ix, &qwords);
    for (ci, s) in scored.iter_mut() {
        let c = &ix.caps[*ci];
        // how much of the capability's name the query covers, partial matches (typing, typos) counted by quality:
        // "varianc" puts "Sample variance" before "Bartlett's test for equal variances"
        let (ws, _) = &ix.name_idf[*ci];
        if !ws.is_empty() {
            let got: f32 = ws.iter().map(|w| {
                qwords.iter().flat_map(|e| e.iter()).filter(|(x, _)| x == w).map(|(_, m)| *m).fold(0.0f32, f32::max)
            }).sum();
            *s += SEARCH_NAME * got / ws.len() as f32;
        }
        if !ql.is_empty() && c.key.to_lowercase() == ql.replace(' ', "_") {
            *s += 10.0;
        } else if ql.chars().count() >= 3 && c.name.to_lowercase().contains(&ql) {
            *s += 3.0;
        }
    }
    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap().then(ix.caps[a.0].key.cmp(&ix.caps[b.0].key)));
    let matched: HashSet<String> = qwords.iter().flat_map(|e| e.iter().map(|(w, _)| w.clone())).collect();
    scored.iter().take(limit.clamp(1, MAX_LIMIT)).map(|(ci, s)| {
        let c = &ix.caps[*ci];
        json!({"key": c.key, "name": c.name, "kind": c.kind, "topic": c.topic,
               "score": (*s as f64 * 1000.0).round() / 1000.0, "snippet": snippet(c, &matched)})
    }).collect()
}

/// Search one model's capabilities: [{key, name, kind, topic, score, snippet}], best first (at most `limit`).
pub fn search_json(p: &Pack, query: &str, limit: usize) -> String {
    Value::Array(search_results(p, query, limit)).to_string()
}

/// Search several models at once: the same results, each with its "model", best first across all.
pub fn search_all_json(packs: &[&Pack], query: &str, limit: usize) -> String {
    let mut all = vec![];
    for p in packs.iter().take(64) {
        for mut r in search_results(p, query, limit) {
            r["model"] = json!(p.model);
            all.push(r);
        }
    }
    all.sort_by(|a, b| {
        b["score"].as_f64().unwrap_or(0.0).partial_cmp(&a["score"].as_f64().unwrap_or(0.0)).unwrap()
            .then(a["model"].as_str().cmp(&b["model"].as_str())).then(a["key"].as_str().cmp(&b["key"].as_str()))
    });
    all.truncate(limit.clamp(1, MAX_LIMIT));
    Value::Array(all).to_string()
}

/// Every capability by topic (a formula by its family): {model, topics: [{topic, capabilities: [{key, name, kind}]}]}.
pub fn topics_json(p: &Pack) -> String {
    let ix = index(p);
    let mut by: Vec<(String, Vec<Value>)> = vec![];
    for c in &ix.caps {
        let item = json!({"key": c.key, "name": c.name, "kind": c.kind});
        match by.iter_mut().find(|(t, _)| *t == c.topic) {
            Some((_, v)) => v.push(item),
            None => by.push((c.topic.clone(), vec![item])),
        }
    }
    by.sort_by(|a, b| a.0.cmp(&b.0));
    for (_, v) in by.iter_mut() {
        v.sort_by(|a, b| a["name"].as_str().cmp(&b["name"].as_str()));
    }
    json!({"model": p.model, "topics": by.into_iter().map(|(t, v)| json!({"topic": t, "capabilities": v}))
                                       .collect::<Vec<_>>()}).to_string()
}

// ============================================================================================ quantities ====
/// The unit written after token i (the longest run of adjacent tokens, at most MAX_UNIT_TOKENS, that reads as a
/// unit), as written, and how many tokens it takes. None when no unit follows.
fn unit_after(toks: &[Tok], i: usize) -> Option<(String, usize, crate::units::Unit)> {
    let mut best = None;
    let mut text = String::new();
    for k in 1..=MAX_UNIT_TOKENS {
        let Some(t) = toks.get(i + k) else { break };
        if k > 1 && t.gap {
            break;
        }
        if t.k == K::List || (t.k == K::Punct && !"/^*()\u{00b0}%-".contains(t.orig.as_str())) {
            break;
        }
        if k == 1 && t.k != K::Word && t.orig != "\u{00b0}" && t.orig != "%" {
            break;
        }
        text.push_str(&t.orig);
        if text.ends_with(['/', '^', '*', '(', '-']) {
            continue;
        }
        if let Ok(u) = parse_unit(&text) {
            best = Some((text.clone(), k, u));
        }
    }
    best
}

/// How many tokens after the number at i are its unit or percent sign (not words that describe it).
fn skip_after(toks: &[Tok], i: usize) -> usize {
    if toks.get(i + 1).map(|t| t.orig == "%").unwrap_or(false) {
        return 1;
    }
    unit_after(toks, i).map(|u| u.1).unwrap_or(0)
}

fn dim_one(d: &Dim) -> bool {
    *d == Dim::one()
}

// ================================================================================================= forms ====
#[derive(Clone, Debug)]
enum Tf {
    /// the number itself (`int`: read as a whole number)
    Same { int: bool },
    /// a percentage read as a fraction
    Pct,
    /// a quantity: the number with the form's unit (negated where the package read a sign word)
    Qty { unit: String, neg: bool },
    /// a number the package converted to SI from the unit written after it
    Unit { unit: String },
    /// one element of a list or tuple ("on [0, 1]": a is element 0)
    Elem { at: usize, int: bool },
    /// a list: as JSON numbers, or as the text the package read (with or without the brackets)
    ListJson,
    ListText { brackets: bool },
}

#[derive(Clone, Debug)]
enum FVal {
    Fixed(Value),
    Slot(usize, Tf),
}

#[derive(Clone, Debug)]
struct PForm {
    toks: Vec<Tok>,
    /// tokens compared as written (a unit after a number: mW is not MW)
    exact: Vec<bool>,
    outcome: String,
    key: String,
    target: Option<String>,
    inputs: Vec<(String, FVal)>,
}

const LEADS: &[&[&str]] = &[
    &["please"], &["can", "you"], &["could", "you"], &["what", "is"], &["what", "are"], &["what's"], &["whats"],
    &["find"], &["compute"], &["calculate"], &["determine"], &["evaluate"], &["give"], &["get"], &["obtain"],
    &["tell", "me"], &["work", "out"], &["me"], &["the"],
];

/// The tokens that carry the question: an opening request ("Please find the", "What is the") and closing marks are
/// left out, so "Find the mean of [1, 2]" and "What is the mean of [1, 2]?" are the same question.
fn core(toks: &[Tok]) -> std::ops::Range<usize> {
    let (mut a, mut b) = (0, toks.len());
    let mut changed = true;
    while changed && a < b {
        changed = false;
        for lead in LEADS {
            if b - a > lead.len() && lead.iter().enumerate().all(|(j, w)| toks[a + j].k == K::Word && toks[a + j].low == *w) {
                a += lead.len();
                changed = true;
                break;
            }
        }
    }
    while b > a && toks[b - 1].k == K::Punct && ".?!".contains(toks[b - 1].orig.as_str()) {
        b -= 1;
    }
    a..b
}

/// A bracketed list of numbers written loosely ("[.5, +2, 3]"), nested at most three deep.
fn loose_list(s: &[char], pos: &mut usize, depth: usize) -> Option<Value> {
    if depth > 3 || s.get(*pos) != Some(&'[') {
        return None;
    }
    *pos += 1;
    let mut items = vec![];
    loop {
        while s.get(*pos).map(|c| c.is_whitespace()).unwrap_or(false) {
            *pos += 1;
        }
        match s.get(*pos)? {
            ']' => {
                *pos += 1;
                return Some(Value::Array(items));
            }
            '[' => items.push(loose_list(s, pos, depth + 1)?),
            _ => {
                let st = *pos;
                while s.get(*pos).map(|c| c.is_ascii_digit() || "+-.eE".contains(*c)).unwrap_or(false) {
                    *pos += 1;
                }
                let t: String = s[st..*pos].iter().collect();
                let x = t.parse::<f64>().ok().filter(|x| x.is_finite())?;
                items.push(json!(x));
            }
        }
        if items.len() > 2000 {
            return None;
        }
        while s.get(*pos).map(|c| c.is_whitespace()).unwrap_or(false) {
            *pos += 1;
        }
        match s.get(*pos)? {
            ',' => *pos += 1,
            ']' => {}
            _ => return None,
        }
    }
}

fn list_value(raw: &str) -> Option<Value> {
    // a tuple "(1, 2)" is read as the list [1, 2]
    let bracketed;
    let raw = match raw.trim().strip_prefix('(').and_then(|r| r.strip_suffix(')')) {
        Some(inner) => {
            bracketed = format!("[{inner}]");
            bracketed.as_str()
        }
        None => raw,
    };
    let v: Value = match serde_json::from_str(raw) {
        Ok(v) => v,
        Err(_) => {
            let cs: Vec<char> = raw.trim().chars().collect();
            let mut pos = 0;
            let v = loose_list(&cs, &mut pos, 0)?;
            if pos != cs.len() {
                return None;
            }
            v
        }
    };
    fn ok(v: &Value, depth: usize) -> bool {
        match v {
            Value::Number(_) => true,
            Value::Array(a) => depth < 3 && a.len() <= 2000 && a.iter().all(|x| ok(x, depth + 1)),
            _ => false,
        }
    }
    if ok(&v, 0) && v.is_array() { Some(v) } else { None }
}

fn depth(v: &Value) -> usize {
    match v {
        Value::Array(a) => 1 + a.first().map(depth).unwrap_or(0),
        _ => 0,
    }
}

fn same_num(a: f64, b: f64) -> bool {
    a.is_finite() && b.is_finite() && (a - b).abs() <= 1e-9 * a.abs().max(b.abs()).max(1e-300)
}

fn same_list(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => same_num(x.as_f64().unwrap_or(f64::NAN), y.as_f64().unwrap_or(f64::NAN)),
        (Value::Array(x), Value::Array(y)) => x.len() == y.len() && x.iter().zip(y).all(|(p, q)| same_list(p, q)),
        _ => false,
    }
}

/// How a form's token yields an input's value as the package read it, if it does.
fn token_reads(p: &Pack, toks: &[Tok], i: usize, sym: &str, val: &Value, key: &str) -> Option<Tf> {
    let t = &toks[i];
    if p.relation(key).is_some() {
        if t.k != K::Num {
            return None;
        }
        let rel = p.relation(key)?;
        let kind = p.kinds.get(&rel.sym(sym)?.kind)?;
        let si = val.get("si")?.as_f64()?;
        let (unit, u) = match unit_after(toks, i) {
            Some((text, _, u)) if u.dim == kind.dim => (text, Some(u)),
            _ if dim_one(&kind.dim) => (String::new(), None),
            _ => return None,
        };
        let got = match &u {
            Some(u) => to_si(t.val, u, kind.difference),
            None => t.val,
        };
        if same_num(got, si) {
            return Some(Tf::Qty { unit, neg: false });
        }
        if same_num(-got, si) {
            return Some(Tf::Qty { unit, neg: true });
        }
        return None;
    }
    match (&t.k, val) {
        (K::Num, Value::Number(n)) => {
            let v = n.as_f64()?;
            if same_num(v, t.val) {
                return Some(Tf::Same { int: n.is_i64() || n.is_u64() });
            }
            let pct = toks.get(i + 1).map(|x| x.orig == "%").unwrap_or(false);
            if pct && same_num(v, t.val / 100.0) {
                return Some(Tf::Pct);
            }
            // a number the package read in SI from the unit written after it ("75.0 kJ/mol" read as 75000)
            if let Some((unit, _, u)) = unit_after(toks, i) {
                if same_num(v, to_si(t.val, &u, false)) {
                    return Some(Tf::Unit { unit });
                }
            }
            None
        }
        (K::Num, Value::String(s)) => {
            // a quantity read as text: "12 m"
            let st = tokenize(s);
            if st.first().map(|x| x.k == K::Num && same_num(x.val, t.val)).unwrap_or(false) {
                let unit = s.trim().get(st[0].orig.len()..)?.trim().to_string();
                if !unit.is_empty() {
                    let (text, _, _) = unit_after(toks, i)?;
                    if text == unit {
                        return Some(Tf::Qty { unit, neg: false });
                    }
                }
            }
            None
        }
        (K::List, Value::Array(_)) => {
            let lv = list_value(&t.orig)?;
            if same_list(&lv, val) { Some(Tf::ListJson) } else { None }
        }
        (K::List, Value::String(s)) => {
            let lv = list_value(&t.orig)?;
            let s = s.trim();
            let (brackets, body) = if s.starts_with('[') { (true, s.to_string()) } else { (false, format!("[{s}]")) };
            let sv = list_value(&body)?;
            if same_list(&lv, &sv) { Some(Tf::ListText { brackets }) } else { None }
        }
        _ => None,
    }
}

fn prepare_form(p: &Pack, f: &Value) -> Option<PForm> {
    let q = f.get("q")?.as_str()?;
    let all = tokenize(q);
    let r = core(&all);
    let toks: Vec<Tok> = all[r.clone()].to_vec();
    let outcome = f.get("outcome").and_then(|o| o.as_str()).unwrap_or("").to_string();
    let key = f.get("key").and_then(|k| k.as_str()).unwrap_or("").to_string();
    let mut exact = vec![false; toks.len()];
    if outcome == "SOLVED" && key.is_empty() {
        return None;                                      // answered through several capabilities: not one reading
    }
    if outcome != "SOLVED" {
        return Some(PForm { toks, exact, outcome, key: String::new(), target: None, inputs: vec![] });
    }
    let ins = f.get("inputs")?.as_object()?;
    // every (token, input) that reads: kept only where the pairing is one to one
    // (token, element of a list token or NONE, input, how)
    const NONE: usize = usize::MAX;
    let mut pairs: Vec<(usize, usize, String, Tf)> = vec![];
    let exponent = |i: usize| i > 0 && toks[i - 1].orig == "^";
    for i in 0..toks.len() {
        if exponent(i) || (toks[i].k != K::Num && toks[i].k != K::List) {
            continue;                                         // an exponent is never a value
        }
        for (sym, v) in ins {
            if let Some(tf) = token_reads(p, &toks, i, sym, v, &key) {
                pairs.push((i, NONE, sym.clone(), tf));
            } else if toks[i].k == K::List && p.relation(&key).is_none() {
                // one input per element of a list or tuple: "on [0, 1]" read as a = 0, b = 1
                if let (Some(Value::Array(items)), Some(x)) = (list_value(&toks[i].orig), v.as_f64()) {
                    for (j, it) in items.iter().enumerate() {
                        if it.as_f64().map(|y| same_num(x, y)).unwrap_or(false) {
                            pairs.push((i, j, sym.clone(), Tf::Elem { at: j, int: v.is_i64() || v.is_u64() }));
                        }
                    }
                }
            }
        }
    }
    // a number equal to another number of the form (the same value written twice) cannot say which one was read
    let dup_value = |i: usize| {
        toks.iter().enumerate().any(|(j, t)| j != i && !exponent(j) && t.k == toks[i].k && t.low == toks[i].low
                                             && t.k != K::Punct)
    };
    let mut slots: HashMap<String, (usize, Tf)> = HashMap::new();
    for (i, e, sym, tf) in &pairs {
        let by_tok = pairs.iter().filter(|(j, f, _, _)| j == i && (f == e || *f == NONE || *e == NONE)).count();
        let by_sym = pairs.iter().filter(|(_, _, s, _)| s == sym).count();
        if by_tok == 1 && by_sym == 1 && !dup_value(*i) {
            slots.insert(sym.clone(), (*i, tf.clone()));
        }
    }
    // verified against the package: a slot is kept only when a copy of the question with that number changed is
    // read by the package with exactly that input changed, to the value this slot gives (exported `checks`)
    if let Some(checks) = f.get("checks").and_then(|c| c.as_array()) {
        let is_rel = p.relation(&key).is_some();
        let mut confirmed: HashSet<String> = HashSet::new();
        let mut rejected: HashSet<String> = HashSet::new();
        for c in checks.iter().take(32) {
            let (Some(q2), Some(ins2)) = (c.get("q").and_then(|x| x.as_str()), c.get("inputs").and_then(|x| x.as_object()))
                else { continue };
            let all2 = tokenize(q2);
            let t2: Vec<Tok> = all2[core(&all2)].to_vec();
            if t2.len() != toks.len() {
                continue;
            }
            let changed: Vec<usize> = (0..toks.len()).filter(|&j| toks[j].orig != t2[j].orig).collect();
            let [i] = changed.as_slice() else { continue };
            let here: Vec<&String> = slots.iter().filter(|(_, (j, _))| j == i).map(|(s, _)| s).collect();
            if here.is_empty() {
                continue;
            }
            let py_changed: HashSet<&String> = ins.iter()
                .filter(|(s, v)| !ins2.get(*s).map(|w| same_reading(v, w, is_rel)).unwrap_or(false)).map(|(s, _)| s)
                .chain(ins2.keys().filter(|s| !ins.contains_key(*s))).collect();
            let ok = here.len() == py_changed.len() && here.iter().all(|s| {
                py_changed.contains(*s) && {
                    let (j, tf) = &slots[*s];
                    slot_reading(p, &key, s, tf, &t2, *j).zip(ins2.get(*s))
                        .map(|(mine, py)| same_reading(&mine, py, is_rel)).unwrap_or(false)
                }
            });
            for s in here {
                if ok { confirmed.insert(s.clone()); } else { rejected.insert(s.clone()); }
            }
        }
        slots.retain(|s, _| confirmed.contains(s) && !rejected.contains(s));
    }
    let mut inputs = vec![];
    for (sym, v) in ins {
        match slots.get(sym) {
            Some((i, tf)) => {
                if let Tf::Qty { unit, .. } | Tf::Unit { unit } = tf {
                    if !unit.is_empty() {
                        let n = unit_after(&toks, *i).map(|x| x.1).unwrap_or(0);
                        for e in exact.iter_mut().skip(i + 1).take(n) {
                            *e = true;
                        }
                    }
                }
                inputs.push((sym.clone(), FVal::Slot(*i, tf.clone())));
            }
            None => {
                let fixed = if p.relation(&key).is_some() { json!(v.get("si")?.as_f64()?) } else { v.clone() };
                inputs.push((sym.clone(), FVal::Fixed(fixed)));
            }
        }
    }
    let target = f.get("target").and_then(|t| t.as_str()).map(String::from);
    Some(PForm { toks, exact, outcome, key, target, inputs })
}

/// The value a slot gives for the question tokens `q`, as the package's reading writes it (a formula's input as
/// {"si": value}), to compare with the package's reading of the same question.
fn slot_reading(p: &Pack, key: &str, sym: &str, tf: &Tf, q: &[Tok], i: usize) -> Option<Value> {
    let probe = PForm { toks: q.to_vec(), exact: vec![false; q.len()], outcome: "SOLVED".into(), key: key.into(),
                        target: None, inputs: vec![(sym.to_string(), FVal::Slot(i, tf.clone()))] };
    let input = match_form(&probe, q)?.pop()?.1;
    if let Some(rel) = p.relation(key) {
        let kind = p.kinds.get(&rel.sym(sym)?.kind)?;
        let text = match &input {
            Input::Text(t) => t.clone(),
            _ => return None,
        };
        let tt = tokenize(&text);
        let num = tt.first().filter(|t| t.k == K::Num)?.val;
        let si = match unit_after(&tt, 0) {
            Some((_, _, u)) => to_si(num, &u, kind.difference),
            None => num,
        };
        return Some(json!({"si": si}));
    }
    Some(input_json(&input))
}

/// Two readings of one input agree: numbers to 1e-6, lists element by element, a list written as text by its
/// numbers, other text by its tokens (numbers by value); a formula's inputs by their SI value.
fn same_reading(a: &Value, b: &Value, is_rel: bool) -> bool {
    if is_rel {
        return match (a.get("si").and_then(|x| x.as_f64()), b.get("si").and_then(|x| x.as_f64())) {
            (Some(x), Some(y)) => (x - y).abs() <= 1e-6 * x.abs().max(y.abs()).max(1e-300),
            _ => false,
        };
    }
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => {
            let (x, y) = (x.as_f64().unwrap_or(f64::NAN), y.as_f64().unwrap_or(f64::NAN));
            (x - y).abs() <= 1e-6 * x.abs().max(y.abs()).max(1e-300)
        }
        (Value::Array(x), Value::Array(y)) => x.len() == y.len() && x.iter().zip(y).all(|(p, q)| same_reading(p, q, false)),
        (Value::String(x), Value::String(y)) => {
            let lx = list_value(x).or_else(|| list_value(&format!("[{x}]")));
            let ly = list_value(y).or_else(|| list_value(&format!("[{y}]")));
            if let (Some(lx), Some(ly)) = (lx, ly) {
                return same_reading(&lx, &ly, false);
            }
            let (tx, ty) = (tokenize(x), tokenize(y));
            tx.len() == ty.len() && tx.iter().zip(&ty).all(|(p, q)| {
                if p.k == K::Num && q.k == K::Num { same_num(p.val, q.val) } else { p.orig == q.orig }
            })
        }
        (Value::String(_), Value::Array(_)) | (Value::Array(_), Value::String(_)) => {
            let (s, arr) = if a.is_string() { (a, b) } else { (b, a) };
            let s = s.as_str().unwrap_or("");
            list_value(s).or_else(|| list_value(&format!("[{s}]"))).map(|l| same_reading(&l, arr, false)).unwrap_or(false)
        }
        _ => a == b,
    }
}

/// A number as Python writes a float (5.0, 0.25, 1e-05), for the text of a list the package read as text.
fn py_num(v: f64) -> String {
    crate::num::py_repr_f64(v)
}

/// A word whose letter case carries meaning: a symbol of one or two letters (P(n, k) is not p(n, k)) or one with a
/// capital after its first letter (pH, IQR, CO2); the articles "a" and "I" excepted.
fn case_matters(t: &Tok) -> bool {
    t.k == K::Word && !stopword(&t.low)
        && (t.orig.chars().count() <= 2 || t.orig.chars().skip(1).any(|c| c.is_uppercase()))
}

/// The reading of `q` by the form `f`: None when they differ in a word, a mark, a unit or the type of a value.
fn match_form(f: &PForm, q: &[Tok]) -> Option<Vec<(String, Input)>> {
    if f.toks.len() != q.len() {
        return None;
    }
    let mut slot_at: HashMap<usize, Vec<&Tf>> = HashMap::new();
    for (_, v) in &f.inputs {
        if let FVal::Slot(i, tf) = v {
            slot_at.entry(*i).or_default().push(tf);
        }
    }
    for (i, (a, b)) in f.toks.iter().zip(q).enumerate() {
        if let Some(tfs) = slot_at.get(&i) {
            if a.k != b.k {
                return None;
            }
            // elements of a list read one by one: same length, and every element not read is as in the form
            let at: Vec<usize> = tfs.iter().filter_map(|t| match t {
                Tf::Elem { at, .. } => Some(*at),
                _ => None,
            }).collect();
            if !at.is_empty() {
                let (Some(Value::Array(x)), Some(Value::Array(y))) = (list_value(&a.orig), list_value(&b.orig)) else {
                    return None;
                };
                if x.len() != y.len() || x.iter().zip(&y).enumerate()
                    .any(|(j, (p, r))| !at.contains(&j) && !same_list(p, r)) || y.iter().any(|v| !v.is_number()) {
                    return None;
                }
            }
            continue;
        }
        let same = if f.exact[i] || case_matters(a) || case_matters(b) { a.orig == b.orig }
                   else { a.k == b.k && a.low == b.low };
        if !same {
            if a.k == K::List && b.k == K::List && a.low.replace(char::is_whitespace, "") == b.low.replace(char::is_whitespace, "") {
                continue;
            }
            return None;
        }
    }
    let mut out = vec![];
    for (sym, v) in &f.inputs {
        let input = match v {
            FVal::Fixed(j) => crate::input_of(j),
            FVal::Slot(i, tf) => {
                let t = &q[*i];
                match tf {
                    Tf::Same { int } => {
                        if !t.val.is_finite() {
                            return None;
                        }
                        if *int {
                            if t.val.fract() != 0.0 || t.val.abs() > 9.0e15 {
                                return None;
                            }
                            Input::Int(t.val as i64)
                        } else {
                            Input::Float(t.val)
                        }
                    }
                    Tf::Pct => Input::Float(t.val / 100.0),
                    Tf::Elem { at, int } => {
                        let x = list_value(&t.orig)?.get(*at)?.as_f64()?;
                        if *int {
                            if x.fract() != 0.0 || x.abs() > 9.0e15 {
                                return None;
                            }
                            Input::Int(x as i64)
                        } else {
                            Input::Float(x)
                        }
                    }
                    Tf::Unit { unit } => {
                        let u = parse_unit(unit).ok()?;
                        Input::Float(to_si(t.val, &u, false))
                    }
                    Tf::Qty { unit, neg } => {
                        let num = if *neg { format!("-{}", t.orig.trim_start_matches('+')) } else { t.orig.clone() };
                        let num = num.replace("--", "");
                        Input::Text(if unit.is_empty() { num } else { format!("{num} {unit}") })
                    }
                    Tf::ListJson => {
                        let lv = list_value(&t.orig)?;
                        let fv = match &f.toks[*i] {
                            x if x.k == K::List => list_value(&x.orig)?,
                            _ => return None,
                        };
                        if depth(&lv) != depth(&fv) {
                            return None;
                        }
                        Input::Json(lv)
                    }
                    Tf::ListText { brackets } => {
                        let lv = list_value(&t.orig)?;
                        let fv = list_value(&f.toks[*i].orig)?;
                        if depth(&lv) != depth(&fv) {
                            return None;
                        }
                        let inner = t.orig.trim();
                        let inner = &inner[1..inner.len() - 1];
                        let _ = py_num;
                        Input::Text(if *brackets { format!("[{inner}]") } else { inner.to_string() })
                    }
                }
            }
        };
        out.push((sym.clone(), input));
    }
    Some(out)
}

// ======================================================================================== reading: shared ====
#[derive(Clone, Debug, Default)]
struct Reading {
    key: String,
    target: Option<String>,
    inputs: Vec<(String, Input)>,
    needs: Vec<String>,
    /// numbers of the question no input explains, and why the reading cannot be trusted
    unread: usize,
    doubt: Vec<String>,
    score: f32,
    /// added to the word score: how well the asked result matches (see OUTPUT_MATCH, PRIMARY)
    bonus: f32,
}

impl Reading {
    fn complete(&self) -> bool {
        self.needs.is_empty() && self.unread == 0 && self.doubt.is_empty()
    }
}

fn input_json(i: &Input) -> Value {
    match i {
        Input::Text(s) => json!(s),
        Input::Int(n) => json!(n),
        Input::Float(x) => json!(x),
        Input::Json(v) => v.clone(),
    }
}

fn has_phrase(qw: &[String], phrase: &[String]) -> bool {
    !phrase.is_empty() && qw.windows(phrase.len()).any(|w| w == phrase)
}

/// The words of a question in order (lower case, no stemming, hyphenated words split), for phrase tests.
fn plain_words(toks: &[Tok]) -> Vec<String> {
    let mut out = vec![];
    for t in toks.iter().filter(|t| is_word(t)) {
        for part in t.low.split('-') {
            if !part.is_empty() {
                out.push(part.replace('\'', ""));
            }
        }
    }
    out
}

fn phrase_words(s: &str) -> Vec<String> {
    plain_words(&tokenize(&s.to_lowercase().replace(['_', '-'], " ")))
}

/// The words of the asked part of a question: what follows the last "what", "find", "how" ... up to the end of its
/// sentence (or the whole question when there is no such word).
const ASK_CUES: &[&str] = &["what", "find", "calculate", "compute", "determine", "how", "estimate", "evaluate",
                            "obtain", "which", "give", "state", "express", "predict"];

fn asked_span(toks: &[Tok]) -> std::ops::Range<usize> {
    let mut start = None;
    for (i, t) in toks.iter().enumerate() {
        if t.k == K::Word && ASK_CUES.contains(&t.low.as_str()) {
            start = Some(i);
        }
    }
    let a = start.unwrap_or(0);
    let mut b = toks.len();
    for (j, t) in toks.iter().enumerate().skip(a + 1) {
        if t.k == K::Punct && ".?!;".contains(t.orig.as_str()) {
            b = j;
            break;
        }
    }
    a..b
}

/// The words of the asked quantity itself: after the last "what", "find", "how" ..., up to the first word that
/// starts its conditions ("given", "of", "for", "when" ...), a value or a mark. "Compute the absolute pressure at the
/// depth given a pressure at the surface of ..." asks for "absolute pressure".
fn asked_phrase(toks: &[Tok]) -> Vec<String> {
    // verbs that open a question without naming what it asks for ("Verify whether ...", "Is the ...")
    const NOT_ASKING: &[&str] = &["verify", "check", "show", "prove", "decide", "test", "confirm", "is", "are", "does",
                                  "do", "can", "will", "would", "should", "could", "if", "given", "suppose", "assume",
                                  "consider", "let", "use", "using", "with", "for", "in", "a", "an", "there", "we",
                                  "i", "you", "they", "it", "here", "this", "that", "these", "those", "imagine",
                                  "assess", "tell", "say", "on", "at", "after", "when", "two", "three"];
    let span = asked_span(toks);
    let cued = toks.get(span.start).map(|t| t.k == K::Word && ASK_CUES.contains(&t.low.as_str())).unwrap_or(false);
    if !cued && toks.first().map(|t| t.k != K::Word || NOT_ASKING.contains(&t.low.as_str())).unwrap_or(true) {
        return vec![];
    }
    asked_parts(toks, span.start + cued as usize, span.end).0
}

/// The asked phrase from token `a` (articles and "value" before it passed over) and its complement: the words of an
/// "of ..." / "for ..." that follows it ("the lower limit FOR THE VARIANCE"), up to a value, a mark or a stop word.
fn asked_parts(toks: &[Tok], a: usize, b: usize) -> (Vec<String>, Vec<String>) {
    const STOPS: &[&str] = &["given", "with", "when", "if", "where", "using", "that", "which", "whose", "assuming",
                             "for", "from", "of", "in", "at", "to", "by", "on", "after", "before", "under", "between",
                             "while", "as", "than", "and", "or", "is", "are", "was", "were", "be", "will", "does",
                             "do", "would", "should", "can", "could", "must", "has", "have", "had", "about", "around",
                             "over", "across", "among", "within", "per", "versus", "vs", "into", "onto", "through"];
    const SKIP: &[&str] = &["is", "are", "was", "were", "the", "a", "an", "its", "their", "this", "that", "these",
                            "be", "will", "would", "value", "values", "exact", "resulting", "required", "needed",
                            "expected", "new", "corresponding", "approximate", "numerical", "total"];
    let mut out = vec![];
    let mut started = false;
    let mut j = a;
    while j < b.min(toks.len()) {
        let t = &toks[j];
        if t.k != K::Word {
            // before the quantity's words: an apostrophe, or a number qualifying it ("the 95% confidence interval")
            if !started && (t.orig == "'" || t.orig == "%" || t.k == K::Num) {
                j += 1;
                continue;
            }
            break;
        }
        let w = t.low.replace('\'', "");
        if !started && (SKIP.contains(&w.as_str()) || matches!(w.as_str(), "much" | "many")) {
            j += 1;
            continue;
        }
        if started && STOPS.contains(&w.as_str()) {
            break;
        }
        started = true;
        out.push(w);
        j += 1;
        if out.len() >= 8 {
            break;
        }
    }
    let mut comp = vec![];
    if j < b.min(toks.len()) && matches!(toks[j].low.as_str(), "of" | "for" | "about" | "around" | "in") {
        j += 1;
        while j < b.min(toks.len()) && comp.len() < 6 {
            let t = &toks[j];
            if t.k != K::Word {
                break;
            }
            let w = t.low.replace('\'', "");
            if matches!(w.as_str(), "the" | "a" | "an" | "this" | "these" | "its" | "their" | "each" | "of" | "for") {
                j += 1;                                   // "the lower limit of the interval FOR THE VARIANCE"
                continue;
            }
            if STOPS.contains(&w.as_str()) {
                break;
            }
            comp.push(w);
            j += 1;
        }
    }
    (out, comp)
}

/// The asked phrase and its complement of a question (see asked_parts).
fn asked_full(toks: &[Tok]) -> (Vec<String>, Vec<String>) {
    let span = asked_span(toks);
    let cued = toks.get(span.start).map(|t| t.k == K::Word && ASK_CUES.contains(&t.low.as_str())).unwrap_or(false);
    if asked_phrase(toks).is_empty() {
        return (vec![], vec![]);
    }
    asked_parts(toks, span.start + cued as usize, span.end)
}

/// Context words of the number at token i: (word, weight) for words before it (back to the previous value, "and",
/// a comma or the start of the sentence, at most 6) and after it (at most 3, up to the next value, "and" or a comma).
/// The word right after a number ("10 elements") and the two right before ("a mean of 96.2") weigh most.
fn context(toks: &[Tok], i: usize, skip_unit: usize) -> Vec<(String, f32)> {
    let mut out = vec![];
    let mut d = 0;
    let mut j = i;
    while j > 0 && d < 6 {
        j -= 1;
        let t = &toks[j];
        if t.k == K::Num || t.k == K::List || (t.k == K::Punct && ".?!;,".contains(t.orig.as_str()))
            || t.low == "and" {
            break;
        }
        if t.k == K::Word {
            d += 1;
            let w = (if d <= 2 { 1.0 } else { 0.8 }) as f32;
            for part in std::iter::once(t.low.as_str()).chain(t.low.split('-')) {
                let s = stem(&part.replace('\'', ""));
                if !stopword(&s) {
                    out.push((s, w));
                }
            }
        }
    }
    let mut d = 0;
    let mut j = i + 1 + skip_unit;
    while j < toks.len() && d < 3 {
        let t = &toks[j];
        if t.k == K::Num || t.k == K::List || (t.k == K::Punct && ".?!;,".contains(t.orig.as_str()))
            || t.low == "and" {
            break;
        }
        if t.k == K::Word {
            d += 1;
            let w = if d == 1 { 1.5 } else { 0.5 };          // "165 payments", "18 years": the noun after it
            for part in std::iter::once(t.low.as_str()).chain(t.low.split('-')) {
                let s = stem(&part.replace('\'', ""));
                if !stopword(&s) {
                    out.push((s, w));
                }
            }
        }
        j += 1;
    }
    out
}

fn ctx_score(ctx: &[(String, f32)], names: &HashSet<String>, idf: &HashMap<String, f32>) -> f32 {
    let mut seen: HashSet<&str> = HashSet::new();
    let mut s = 0.0;
    for (w, wt) in ctx {
        if names.contains(w) && seen.insert(w.as_str()) {
            s += wt * idf.get(w).copied().unwrap_or(1.0).max(0.5);
        }
    }
    s
}

/// Assign values to slots one pair at a time, best first; a pair is taken only when it is clearly better (by `margin`)
/// than every other free choice for that value and for that slot. A value torn between two slots, or a slot between
/// two values, is set aside (never decided by a small difference). Returns the pairs, the values left over and the
/// slots left free.
fn assign(scores: &[Vec<f32>], margin: f32) -> (Vec<(usize, usize)>, Vec<usize>, Vec<usize>) {
    let nv = scores.len();
    let ns = scores.first().map(|r| r.len()).unwrap_or(0);
    let mut vfree = vec![true; nv];
    let mut sfree = vec![true; ns];
    let mut vtorn = vec![false; nv];
    let mut storn = vec![false; ns];
    let mut out = vec![];
    for _ in 0..(nv + ns) {
        let mut best: Option<(usize, usize, f32)> = None;
        for v in 0..nv {
            for s in 0..ns {
                if vfree[v] && sfree[s] && !vtorn[v] && !storn[s] && scores[v][s] > 0.0
                    && best.map(|b| scores[v][s] > b.2).unwrap_or(true) {
                    best = Some((v, s, scores[v][s]));
                }
            }
        }
        let Some((v, s, sc)) = best else { break };
        // rivals among ALL slots and values, taken or not: a value is never placed by elimination (its words must
        // favour its slot over every other one, including a slot another value already took)
        let rival_s = (0..ns).filter(|&o| o != s).map(|o| scores[v][o]).fold(0.0f32, f32::max);
        let rival_v = (0..nv).filter(|&o| o != v).map(|o| scores[o][s]).fold(0.0f32, f32::max);
        if sc < margin * rival_s {
            vtorn[v] = true;
            continue;
        }
        if sc < margin * rival_v {
            storn[s] = true;
            continue;
        }
        vfree[v] = false;
        sfree[s] = false;
        out.push((v, s));
    }
    let left = (0..nv).filter(|&v| vfree[v]).collect();
    let free = (0..ns).filter(|&s| sfree[s]).collect();
    (out, left, free)
}

// =================================================================================== reading: procedures ====
fn name_words(i: &ProcInput) -> HashSet<String> {
    let mut set: HashSet<String> = words(&i.meaning).into_iter().collect();
    for part in i.symbol.to_lowercase().split('_') {
        if !part.is_empty() {
            set.insert(stem(part));
        }
    }
    set
}

fn is_number_input(i: &ProcInput) -> bool {
    matches!(i.ty.as_str(), "" | "number" | "integer")
}

fn read_procedure(p: &Pack, ix: &Index, pr: &Procedure, toks: &[Tok]) -> Reading {
    let mut r = Reading { key: pr.key.clone(), ..Default::default() };
    let mut used = vec![false; toks.len()];
    let mut bound: HashMap<String, Input> = HashMap::new();
    let by_sym = |w: &str| pr.inputs.iter().find(|i| i.symbol.to_lowercase() == w);
    // 1. "name = value"
    for i in 0..toks.len().saturating_sub(2) {
        if toks[i].k == K::Word && (toks[i + 1].orig == "=" || toks[i + 1].orig == ":") {
            if let Some(inp) = by_sym(&toks[i].low) {
                let v = &toks[i + 2];
                let kind_dim = p.kinds.get(&inp.kind).map(|k| k.dim).filter(|d| !dim_one(d));
                let unit = unit_after(toks, i + 2);
                let ends = |k: usize| toks.get(k).map(|t| matches!(t.orig.as_str(), "," | ";" | "." | "?" | "!")
                                                         || t.low == "and").unwrap_or(true);
                let val = match v.k {
                    K::Num if is_number_input(inp) => match (kind_dim, &unit) {
                        // a quantity: its unit must be written and be of its dimension
                        (Some(d), Some((u, n, un))) if un.dim == d => {
                            for e in used.iter_mut().skip(i + 3).take(*n) {
                                *e = true;
                            }
                            Some(Input::Text(format!("{} {}", v.orig, u)))
                        }
                        (Some(_), _) => {
                            r.doubt.push(format!("{} needs a unit of its kind", inp.symbol));
                            None
                        }
                        (None, Some((_, _, un))) if !dim_one(&un.dim) => {
                            r.doubt.push(format!("{} is a pure number, but a unit is written after it", inp.symbol));
                            None
                        }
                        _ => {
                            let pct = toks.get(i + 3).map(|x| x.orig == "%").unwrap_or(false);
                            if pct { used[i + 3] = true; }
                            Some(Input::Float(if pct { v.val / 100.0 } else { v.val }))
                        }
                    },
                    K::List if !is_number_input(inp) => list_value(&v.orig).map(Input::Json),
                    // a word value only when the value ends there ("P = y*z" is not "y")
                    K::Word if (inp.ty == "choice" || inp.ty == "text") && ends(i + 3) => Some(Input::Text(v.orig.clone())),
                    K::Word if inp.ty == "choice" || inp.ty == "text" => {
                        r.doubt.push(format!("{} is written as an expression this reader does not read", inp.symbol));
                        None
                    }
                    _ => None,
                };
                if let Some(val) = val {
                    if bound.contains_key(&inp.symbol) {
                        r.doubt.push(format!("{} is given twice", inp.symbol));
                    }
                    bound.insert(inp.symbol.clone(), val);
                    used[i] = true;
                    used[i + 1] = true;
                    used[i + 2] = true;
                }
            } else if ix.symbols.contains(&toks[i].low) && matches!(toks[i + 2].k, K::Num | K::List) {
                // a value given by the name another capability uses ("sigma = 2" to a test that takes s): this
                // capability does not take it
                used[i + 2] = true;
                r.unread += 1;
            }
        }
    }
    // 2. lists, in order, into the list inputs not yet given: bracketed lists, and runs of three or more numbers
    //    written with commas ("12.1, 9.8, 10.4 and 11"), each run one list
    let mut lists: Vec<(usize, Value)> = vec![];
    let has_lists = pr.inputs.iter().any(|i| matches!(i.ty.as_str(), "vector" | "matrix" | "edges"));
    let mut i = 0;
    while i < toks.len() {
        if toks[i].k == K::List && !used[i] {
            if toks[i].orig.starts_with('(') && toks.iter().filter(|t| t.k == K::List && t.orig.starts_with('(')).count() > 1 {
                // several pairs "(-7, 0.76) and (-5, 8.1)" are points, not one list per input
                r.doubt.push("points written as pairs are read only by a documented form".into());
            }
            match list_value(&toks[i].orig) {
                Some(v) => lists.push((i, v)),
                None => r.doubt.push("a list that is not a list of numbers".into()),
            }
            used[i] = true;
            i += 1;
            continue;
        }
        if has_lists && toks[i].k == K::Num && !used[i] {
            let mut run = vec![i];
            let mut j = i;
            loop {
                let comma = toks.get(j + 1).map(|t| t.orig == ",").unwrap_or(false);
                let and = toks.get(j + 1 + comma as usize).map(|t| t.low == "and").unwrap_or(false);
                let k = j + 1 + comma as usize + and as usize;
                // "a, b and c" ends a run at c; "a, b and c, d, e" is two lists (the second starts at c)
                let ends = !and || toks.get(k + 1).map(|t| t.orig != ",").unwrap_or(true);
                if (comma || and) && ends && toks.get(k).map(|t| t.k == K::Num && !used[k]).unwrap_or(false)
                    && run.len() < 2000 {
                    run.push(k);
                    j = k;
                    if and {
                        break;
                    }
                } else {
                    break;
                }
            }
            if run.len() >= 3 {
                let nums: Vec<Value> = run.iter().map(|&k| json!(toks[k].val)).collect();
                for &k in &run {
                    used[k] = true;
                }
                lists.push((i, Value::Array(nums)));
                i = j + 1;
                continue;
            }
        }
        i += 1;
    }
    let free_lists: Vec<&ProcInput> = pr.inputs.iter()
        .filter(|i| matches!(i.ty.as_str(), "vector" | "matrix" | "edges") && !bound.contains_key(&i.symbol)).collect();
    // several lists: each to the list input its preceding words name, when they all say so clearly; else in order
    if lists.len() >= 2 && free_lists.len() >= 2 {
        let names: Vec<HashSet<String>> = free_lists.iter().map(|i| name_words(i)).collect();
        let scores: Vec<Vec<f32>> = lists.iter().map(|(ti, v)| {
            let ctx = context(toks, *ti, 0);
            free_lists.iter().enumerate().map(|(b, inp)| {
                let fits = match inp.ty.as_str() {
                    "vector" => depth(v) == 1,
                    _ => depth(v) == 2,
                };
                if fits { ctx_score(&ctx, &names[b], &ix.idf) } else { 0.0 }
            }).collect()
        }).collect();
        let (pairs, left, _) = assign(&scores, 1.25);
        if left.is_empty() && pairs.len() == lists.len() {
            for (a, b) in pairs {
                bound.insert(free_lists[b].symbol.clone(), Input::Json(lists[a].1.clone()));
            }
            lists.clear();
        } else if scores.iter().flatten().any(|&s| s > 0.0) {
            // the words before the lists say something, but not clearly which list is which: no order is assumed
            r.doubt.push("which list is which is not clear from the words before them".into());
        }
    }
    let mut li = 0;
    for (_ti, v) in lists {
        let d = depth(&v);
        let want = |i: &ProcInput| match i.ty.as_str() {
            "vector" => d == 1,
            "matrix" | "edges" => d == 2,
            _ => false,
        };
        while li < free_lists.len() && !want(free_lists[li]) && !free_lists[li].required {
            li += 1;
        }
        if li < free_lists.len() && want(free_lists[li]) {
            bound.insert(free_lists[li].symbol.clone(), Input::Json(v));
            li += 1;
        } else {
            r.unread += 1;
        }
    }
    // 3. numbers, each to the input its words name
    let nums: Vec<usize> = (0..toks.len())
        .filter(|&i| toks[i].k == K::Num && !used[i] && !(i > 0 && toks[i - 1].orig == "^")).collect();
    let slots: Vec<&ProcInput> = pr.inputs.iter().filter(|i| is_number_input(i) && !bound.contains_key(&i.symbol)).collect();
    let names: Vec<HashSet<String>> = slots.iter().map(|i| name_words(i)).collect();
    let mut scores = vec![vec![0.0f32; slots.len()]; nums.len()];
    for (a, &ti) in nums.iter().enumerate() {
        let unit = unit_after(toks, ti);
        let pct = toks.get(ti + 1).map(|x| x.orig == "%").unwrap_or(false);
        let ctx = context(toks, ti, skip_after(toks, ti));
        for (b, inp) in slots.iter().enumerate() {
            let kind = p.kinds.get(&inp.kind);
            let kinded = kind.map(|k| !dim_one(&k.dim)).unwrap_or(false);
            if kinded && unit.as_ref().map(|u| Some(&u.2.dim) != kind.map(|k| &k.dim)).unwrap_or(true) {
                continue;
            }
            // a pure number is never a value written with a physical unit ("8.7e-4 mol" is not a count)
            if !kinded && unit.as_ref().map(|u| !dim_one(&u.2.dim)).unwrap_or(false) {
                continue;
            }
            if inp.ty == "integer" && (toks[ti].val.fract() != 0.0 || pct) {
                continue;
            }
            scores[a][b] = ctx_score(&ctx, &names[b], &ix.idf);
        }
    }
    let (mut pairs, mut left, free) = assign(&scores, NUMBER_MARGIN);
    // one number and one required input left, the number named by no input's words: it is that input's value
    let req_free: Vec<usize> = free.iter().copied().filter(|&b| slots[b].required).collect();
    if left.len() == 1 && req_free.len() == 1 && scores[left[0]].iter().all(|&s| s == 0.0) {
        let (a, b) = (left[0], req_free[0]);
        let ti = nums[a];
        let kind = p.kinds.get(&slots[b].kind);
        let kinded = kind.map(|k| !dim_one(&k.dim)).unwrap_or(false);
        let int_ok = slots[b].ty != "integer" || toks[ti].val.fract() == 0.0;
        let unit_ok = unit_after(toks, ti).map(|u| dim_one(&u.2.dim)).unwrap_or(true);
        // only for an input described generically ("the value"), and a number whose words name no result
        let generic = words(&slots[b].meaning).iter().all(|w| GENERIC.contains(&w.as_str()) || w.chars().count() <= 1);
        let ctx = context(toks, ti, skip_after(toks, ti));
        let names_result = pr.outputs.iter().any(|o| {
            let ws: HashSet<String> = words(&o.meaning).into_iter().collect();
            ctx_score(&ctx, &ws, &ix.idf) > 0.0
        });
        if !kinded && int_ok && unit_ok && generic && !names_result {
            pairs.push((a, b));
            left.clear();
        }
    }
    // a number its words describe as one of the results ("a hypothesised STANDARD DEVIATION of 19" given to a test
    // of a mean, whose result is a standard deviation) is not that input
    let outs: Vec<HashSet<String>> = pr.outputs.iter().map(|o| {
        let mut ws: HashSet<String> = words(&o.meaning).into_iter().collect();
        ws.extend(o.symbol.to_lowercase().split('_').filter(|x| !x.is_empty()).map(stem));
        ws
    }).collect();
    for &(a, b) in &pairs {
        let ti = nums[a];
        let ctx = context(toks, ti, skip_after(toks, ti));
        let own = ctx_score(&ctx, &names[b], &ix.idf);
        if outs.iter().any(|o| ctx_score(&ctx, o, &ix.idf) > own) {
            r.doubt.push(format!("the words before {} describe a result, not {}", toks[ti].orig, slots[b].symbol));
        }
    }
    for (a, b) in pairs {
        let ti = nums[a];
        let inp = slots[b];
        let t = &toks[ti];
        let pct = toks.get(ti + 1).map(|x| x.orig == "%").unwrap_or(false);
        let kinded = p.kinds.get(&inp.kind).map(|k| !dim_one(&k.dim)).unwrap_or(false);
        let val = if kinded {
            let (u, _, _) = unit_after(toks, ti).unwrap();
            Input::Text(format!("{} {}", t.orig, u))
        } else if pct && !inp.meaning.to_lowercase().contains("percent") {
            Input::Float(t.val / 100.0)
        } else if inp.ty == "integer" {
            Input::Int(t.val as i64)
        } else {
            Input::Float(t.val)
        };
        bound.insert(inp.symbol.clone(), val);
    }
    r.unread += left.len();
    // 4. a choice by its word
    let qw = plain_words(toks);
    // as written; when no choice is written as such, word by word in their stems: "a proton's speed", "detects
    // protons" name the choice "proton" (and "2 cations ... the common anion" stays the anion, written as such)
    let qw_stems: Vec<String> = qw.iter().map(|w| stem(w)).collect();
    let open: Vec<&ProcInput> = pr.inputs.iter().filter(|i| i.ty == "choice" && !bound.contains_key(&i.symbol)).collect();
    for inp in open {
        let mut hits: Vec<&String> = inp.choices.iter().filter(|c| has_phrase(&qw, &phrase_words(c))).collect();
        if hits.is_empty() {
            hits = inp.choices.iter()
                .filter(|c| has_phrase(&qw_stems, &phrase_words(c).iter().map(|w| stem(w)).collect::<Vec<_>>()))
                .collect();
        }
        match hits.len() {
            0 => {}
            1 => {
                bound.insert(inp.symbol.clone(), Input::Text(hits[0].clone()));
            }
            _ => r.doubt.push(format!("several values of {} are written", inp.symbol)),
        }
    }
    // an optional input left at its default while the question writes its distinctive words ("precedences A-B,
    // A-C" for a schedule, "an existing wall" for a fence) is a silent default: not read
    let qstems: HashSet<String> = qw.iter().map(|w| stem(w)).collect();
    let floor = (1.0 + ix.caps.len() as f32 / DISTINCT_CAPS).ln();
    for inp in &pr.inputs {
        match bound.remove(&inp.symbol) {
            Some(v) => r.inputs.push((inp.symbol.clone(), v)),
            None if inp.required => r.needs.push(inp.symbol.clone()),
            None if inp.ty != "choice" => {
                if let Some(w) = words(&inp.meaning).into_iter().find(|w| {
                    !GENERIC.contains(&w.as_str()) && qstems.contains(w)
                        && ix.idf.get(w).map(|i| *i >= floor).unwrap_or(false)
                }) {
                    r.doubt.push(format!("the question mentions '{w}' of the optional input {}, which is not read",
                                         inp.symbol));
                }
            }
            None => {}
        }
    }
    r.target = asked_output(pr, toks);
    if r.target.is_none() {
        r.doubt.push("the question does not name a result this capability gives".into());
    }
    // the words that qualify the result's name ("BALANCED accuracy", "MAXIMUM rate") must be written somewhere in
    // the question: "What is the accuracy?" is not the balanced accuracy
    if let Some(o) = r.target.as_ref().and_then(|t| pr.outputs.iter().find(|o| &o.symbol == t)) {
        let mt = tokenize(&o.meaning);
        let ph = asked_parts(&mt, 0, mt.len()).0;
        let all: HashSet<String> = plain_words(toks).iter().map(|w| stem(w)).collect();
        if let Some(m) = ph.iter().take(ph.len().saturating_sub(1)).flat_map(|w| w.split('-').map(stem).collect::<Vec<_>>())
            .find(|w| w.chars().count() > 2 && !stopword(w) && !GENERIC.contains(&w.as_str()) && !all.contains(w)) {
            r.doubt.push(format!("the result is the '{m}' one, which the question does not say"));
        }
    }
    if let Some(w) = unexplained(ix, &pr.key, toks) {
        r.doubt.push(format!("the question asks about '{w}', which this capability does not mention"));
    }
    // a distinctive word of the capability's own name ("UNBOUNDED knapsack", "NEGATIVE binomial", "PROPPED
    // cantilever") that the question never writes, when a sibling capability takes inputs of the same kinds (only
    // the wording tells the two apart)
    let sibs = siblings(ix, pr);
    if sibs.len() > 1 {
        // a name word the name writes on its own is written by the question anywhere ("double-escape peak" writes
        // the "double" of "single and double escape peaks"); a part of a hyphenated name word only on its own or in
        // the same hyphenated word: "TWO-sided" does not write the "two" of "two-sample"
        let qw: Vec<&Tok> = toks.iter().filter(|t| is_word(t)).collect();
        let qall: HashSet<String> = plain_words(toks).iter().map(|w| stem(w)).collect();
        let alone: HashSet<String> = qw.iter().filter(|t| !t.low.contains('-'))
            .map(|t| stem(&t.low.replace('\'', ""))).collect();
        let joined: HashSet<String> = qw.iter().filter(|t| t.low.contains('-')).map(|t| t.low.replace('\'', "")).collect();
        let written = |w: &str, from: &Option<String>| match from {
            None => qall.contains(w),
            Some(c) => alone.contains(w) || joined.contains(c),
        };
        let mine = cap_name_words(&pr.name);
        for other in sibs.iter().filter(|k| **k != pr.key).take(64) {
            let Some(o) = p.procedure(other) else { continue };
            let theirs: HashSet<String> = cap_name_words(&o.name).into_iter().map(|x| x.0).collect();
            let mut diff: Vec<&(String, Option<String>)> = mine.iter().filter(|(w, _)| !theirs.contains(w)).collect();
            diff.sort();                                      // the message is the same on every run and platform
            if !diff.is_empty() && !diff.iter().any(|(w, c)| written(w, c)) {
                r.doubt.push(format!("the question does not say '{}', which tells this capability from {}",
                                     diff[0].0, o.name));
                break;
            }
        }
    }
    // every word of the asked quantity must be a word of this capability ("the MIDDLE value of ..." is not read),
    // and a quantity written as a bare symbol ("determine p(n, k)": p is not P) is read only by a documented form
    let (phrase, comp) = asked_full(toks);
    if phrase.is_empty() {
        r.doubt.push("the question does not say which quantity it asks for".into());
    }
    // the asked phrase names one of the inputs better than the result read ("the NOMINAL ANNUAL RATE for 0.25 EAR"
    // asked of a capability that takes the nominal rate): it asks for something this capability is given
    let pst: HashSet<String> = phrase.iter().chain(comp.iter()).map(|w| stem(w))
        .filter(|w| !stopword(w) && !GENERIC.contains(&w.as_str())).collect();
    if let Some(t) = &r.target {
        let ow: HashSet<String> = pr.outputs.iter().filter(|o| &o.symbol == t)
            .flat_map(|o| words(&o.meaning).into_iter().chain(std::iter::once(stem(&o.symbol.to_lowercase())))).collect();
        let got = pst.intersection(&ow).count();
        // the result asked for, the better its words match and when it is the capability's main result
        r.bonus = got as f32 * OUTPUT_MATCH + if *t == pr.primary { PRIMARY } else { 0.0 };
        if let Some(i) = pr.inputs.iter().find(|i| name_words(i).intersection(&pst).count() > got) {
            r.doubt.push(format!("the asked quantity is the input {}", i.symbol));
        }
    }
    // ... and every word of the asked quantity is a word of the result read or of the capability's name, keywords or
    // topic ("the MEAN per-class recall" is not the per-class recall)
    let mut own: HashSet<String> = words(&pr.name).into_iter().chain(words(&pr.topic)).collect();
    for k in &pr.keywords {
        own.extend(words(k));
    }
    if let Some(o) = r.target.as_ref().and_then(|t| pr.outputs.iter().find(|o| &o.symbol == t)) {
        own.extend(words(&o.meaning));
        own.extend(o.symbol.to_lowercase().split('_').filter(|x| !x.is_empty()).map(stem));
        own.insert(stem(&o.symbol.to_lowercase()));
    }
    let known = |w: &str| own.contains(w);
    for whole in &phrase {
        let w = stem(whole);
        if stopword(&w) || GENERIC.contains(&w.as_str()) {
            continue;
        }
        if w.chars().count() <= 2 {
            r.doubt.push(format!("the asked quantity is written as the symbol '{w}'"));
            break;
        }
        let parts_known = whole.contains('-') && whole.split('-').map(stem)
            .all(|x| x.chars().count() <= 2 || stopword(&x) || GENERIC.contains(&x.as_str()) || known(&x));
        if !known(&w) && !parts_known {
            r.doubt.push(format!("the asked '{w}' is not a word of this capability"));
            break;
        }
    }
    if !comp.is_empty() {
        let mut own: HashSet<String> = words(&pr.name).into_iter().collect();
        for o in &pr.outputs {
            own.extend(words(&o.meaning));
            own.insert(stem(&o.symbol.to_lowercase()));
        }
        for i in &pr.inputs {
            own.extend(words(&i.meaning));
        }
        for k in &pr.keywords {
            own.extend(words(k));
        }
        for w in comp.iter().flat_map(|w| w.split('-').map(stem).collect::<Vec<_>>()) {
            if !stopword(&w) && !GENERIC.contains(&w.as_str()) && !own.contains(&w) {
                r.doubt.push(format!("the question asks about a result of '{w}', which this capability does not name"));
                break;
            }
        }
    }
    r
}

/// words naming the data a question is about rather than a quantity ("the mean of the SAMPLE")
const GENERIC: &[&str] = &["data", "dataset", "sample", "value", "set", "list", "number", "observation", "measurement",
                           "following", "given", "reading", "score", "point", "item", "series", "collection", "these",
                           "those", "above", "below", "result", "entry", "figure", "record", "group"];

/// words that turn a value's direction around ("rotate by 48 degrees CLOCKWISE"): not read by the reader of its own
const DIRECTIONS: &[&str] = &["clockwise", "counterclockwise", "anticlockwise", "counter-clockwise", "anti-clockwise"];

/// A distinctive word of the asked part of the question (one that few capabilities use) that this capability never
/// uses: "the RATIO of the standard deviation to the mean" is not a standard deviation. None when there is none.
fn unexplained(ix: &Index, key: &str, toks: &[Tok]) -> Option<String> {
    let ci = ix.caps.iter().position(|c| c.key == key)?;
    let n = ix.caps.len() as f32;
    let floor = (1.0 + n / DISTINCT_CAPS).ln();
    let span = asked_span(toks);
    let has = |w: &str| ix.caps[ci].fields.iter().any(|(_, set)| set.contains(w));
    // an abbreviation anywhere in the question that other capabilities write and this one does not ("a DPSK
    // receiver" is not a QPSK one)
    for t in toks.iter().filter(|t| is_word(t)) {
        let caps = t.orig.chars().count() >= 2 && t.orig.chars().all(|c| c.is_uppercase() || c.is_ascii_digit())
            && t.orig.chars().any(|c| c.is_alphabetic());
        let w = stem(&t.low);
        if caps && ix.idf.contains_key(&w) && !has(&w) {
            return Some(w);
        }
    }
    for t in toks[span].iter() {
        // an arithmetic mark in the asked part: a combination of results ("s / mean"), not one result
        if t.k == K::Punct && "/+*|".contains(t.orig.as_str()) {
            return Some(t.orig.clone());
        }
        if !is_word(t) {
            continue;
        }
        // an abbreviation written in capitals ("MAD", "IQR") that this capability never writes
        let caps = t.orig.chars().count() >= 2 && t.orig.chars().all(|c| c.is_uppercase() || c.is_ascii_digit());
        for part in std::iter::once(t.low.as_str()).chain(t.low.split('-')) {
            let w = stem(&part.replace('\'', ""));
            if stopword(&w) || w.chars().count() < 2 {
                continue;
            }
            if (caps || OPERATIONS.contains(&w.as_str())) && !has(&w) {
                return Some(w);
            }
            let Some(idf) = ix.idf.get(&w) else { continue };
            if *idf >= floor && !has(&w) {
                return Some(w);
            }
        }
    }
    None
}

/// words that combine results ("the standard deviation DIVIDED BY the mean"): a capability that does not write the
/// word does not compute the combination
const OPERATIONS: &[&str] = &["divided", "divide", "ratio", "times", "plus", "minus", "multiplied", "multiply",
                              "product", "squared", "square", "root", "twice", "double", "triple", "half", "adding",
                              "add", "subtract", "subtracting", "difference", "quotient", "reciprocal", "inverse",
                              "percent", "percentage", "fraction", "proportion", "total", "combined"];

/// Each capability's word score plus how much of its name the question writes (all of "Geometric probability" is
/// stronger evidence than "probability" alone), best first.
fn rank(ix: &Index, qwords: &[Vec<(String, f32)>]) -> Vec<(usize, f32)> {
    let mut scored = score_caps(ix, qwords);
    let q: HashSet<&String> = qwords.iter().flat_map(|e| e.iter().filter(|(_, m)| *m >= 1.0).map(|(w, _)| w)).collect();
    for (ci, s) in scored.iter_mut() {
        let (ws, total) = &ix.name_idf[*ci];
        if *total > 0.0 {
            let got: f32 = ws.iter().filter(|w| q.contains(w)).map(|w| ix.idf.get(w).copied().unwrap_or(0.0)).sum();
            *s += NAME_COVER * got / total;
        }
    }
    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap().then(ix.caps[a.0].key.cmp(&ix.caps[b.0].key)));
    scored
}

/// words that turn a phrase around ("without a population correction", "not counting"): the reader of its own does
/// not read them, so a question with one is answered only through a documented form
const NEGATIONS: &[&str] = &["without", "not", "no", "excluding", "except", "ignoring", "ignore", "neither", "nor",
                             "instead", "rather", "never", "cannot", "cant", "dont", "doesnt", "isnt", "arent",
                             "wont", "shouldnt", "neglect", "neglecting", "omit", "omitting"];

/// The output a question asks for. When the asked quantity is written ("the smallest prime"), its last word (the head
/// noun; a one-letter symbol after it is passed over) must be a word of the output's meaning or name: "the smallest
/// prime" is not "the smallest primitive root". Among several such outputs, the one sharing most of the phrase. With
/// no written phrase ("Determine the 95% interval ..."), the output whose meaning the asked part names best. None
/// when no output fits or two tie.
fn asked_output(pr: &Procedure, toks: &[Tok]) -> Option<String> {
    let phrase: Vec<String> = asked_phrase(toks).iter().flat_map(|w| {
        let mut v = vec![stem(w)];
        if w.contains('-') {
            v.extend(w.split('-').filter(|x| !x.is_empty()).map(stem));
        }
        v
    }).filter(|w| !stopword(w) && !GENERIC.contains(&w.as_str())).collect();
    let out_words: Vec<(HashSet<String>, &str)> = pr.outputs.iter().map(|o| {
        let mut ws: HashSet<String> = words(&o.meaning).into_iter().collect();
        ws.extend(o.symbol.to_lowercase().split('_').filter(|x| !x.is_empty()).map(stem));
        ws.insert(stem(&o.symbol.to_lowercase()));
        (ws, o.symbol.as_str())
    }).collect();
    let head = phrase.iter().rev().find(|w| w.chars().count() > 1 || phrase.len() == 1);
    if let Some(head) = head {
        // outputs whose own head noun (or symbol) is the asked head: "maximum rate of climb" is RC_max ("maximum
        // RATE of climb"), not V ("SPEED for the maximum rate of climb"); else any output whose meaning has it
        let own_head = |o: &crate::pack::ProcOutput| {
            let t = tokenize(&o.meaning);
            let h = asked_parts(&t, 0, t.len()).0.last().map(|w| stem(w));
            h.as_deref() == Some(head.as_str()) || o.symbol.to_lowercase().split('_').any(|x| stem(x) == *head)
        };
        let headed: Vec<usize> = pr.outputs.iter().enumerate().filter(|(_, o)| own_head(o)).map(|(k, _)| k).collect();
        let pool: Vec<usize> = if headed.is_empty() { (0..out_words.len()).collect() } else { headed };
        // per fitting output: (asked words it has, words of its meaning the asked phrase does not have, symbol); a
        // capital letter naming a point or a support ("weight of A", "reaction at B") counts as a word
        let asked_set: HashSet<&String> = phrase.iter().collect();
        let fits: Vec<(usize, usize, &str)> = pool.iter().filter(|&&k| out_words[k].0.contains(head)).map(|&k| {
            let (ws, s) = &out_words[k];
            let meaning = &pr.outputs[k].meaning;
            let mut extra: HashSet<String> = words(meaning).into_iter()
                .filter(|w| !asked_set.contains(w) && !GENERIC.contains(&w.as_str())).collect();
            extra.extend(tokenize(meaning).iter()
                .filter(|t| t.k == K::Word && t.orig.chars().count() == 1 && t.orig.chars().all(|c| c.is_uppercase()))
                .map(|t| format!("label {}", t.orig)));
            (phrase.iter().filter(|w| ws.contains(*w)).count(), extra.len(), *s)
        }).collect();
        if fits.is_empty() {
            // the head of the capability's own name ("the Bessel FUNCTION of the first kind", "the CURVATURE of a
            // graph"; not "space" of "Distance between two lines in space"): its primary result
            let nt = tokenize(&pr.name);
            let name_head = asked_parts(&nt, 0, nt.len()).0.last().map(|w| stem(w));
            if name_head.as_deref() == Some(head.as_str()) {
                if pr.outputs.iter().any(|o| o.symbol == pr.primary) {
                    return Some(pr.primary.clone());
                }
                if pr.outputs.len() == 1 {
                    return Some(pr.outputs[0].symbol.clone());
                }
            }
            return None;
        }
        // the output with the most asked words; of several, the one whose meaning IS the asked phrase (no word the
        // question does not write): "collector current" is IC ("collector current"), not IC_mA ("collector current in
        // milliamperes"). Fewer extra words alone is not enough: "bit error rate" is neither "BPSK bit error rate" nor
        // "coherent BFSK bit error rate", and "reaction" neither "reaction at A" nor "reaction at B" (measured: a
        // fewest-extra-words rule answered 13 questions wrongly across the models). Otherwise not read.
        let best = fits.iter().map(|x| x.0).max()?;
        let top: Vec<&(usize, usize, &str)> = fits.iter().filter(|x| x.0 == best).collect();
        if top.len() == 1 {
            return Some(top[0].2.to_string());
        }
        let exact: Vec<&str> = top.iter().filter(|x| x.1 == 0).map(|x| x.2).collect();
        return if exact.len() == 1 { Some(exact[0].to_string()) } else { None };
    }
    let span = asked_span(toks);
    let asked: HashSet<String> = toks[span].iter().filter(|t| is_word(t))
        .map(|t| stem(&t.low.replace('\'', ""))).filter(|w| !stopword(w) && !GENERIC.contains(&w.as_str())).collect();
    let mut best: Vec<(usize, &str)> = pr.outputs.iter().map(|o| {
        let mut ws: HashSet<String> = words(&o.meaning).into_iter().collect();
        ws.insert(stem(&o.symbol.to_lowercase()));
        (ws.intersection(&asked).count(), o.symbol.as_str())
    }).collect();
    best.sort_by(|a, b| b.0.cmp(&a.0));
    match best.as_slice() {
        [(a, s), (b, _), ..] if *a > 0 && a > b => Some(s.to_string()),
        [(a, s)] if *a > 0 => Some(s.to_string()),
        _ => None,
    }
}

// ===================================================================================== reading: formulas ====
struct Quantity {
    value: String,
    dim: Dim,
    ctx: Vec<(String, f32)>,
}

fn terms_list<'a>(p: &'a Pack, k: &str) -> Vec<&'a Value> {
    p.terms.get(k).and_then(|v| v.as_array()).map(|a| a.iter().collect()).unwrap_or_default()
}

fn read_relation(p: &Pack, ix: &Index, rel: &Relation, toks: &[Tok], qs: &[Quantity], tags: &HashSet<String>,
                 target: &Option<(String, String)>, asked: &HashSet<String>) -> Option<Reading> {
    let mut r = Reading { key: rel.key.clone(), ..Default::default() };
    // the asked symbol: its kind and role as the wording names them (a role only when the wording gives one)
    let (tk, trole) = target.clone()?;
    let cands: Vec<&crate::pack::Sym> = rel.symbols.iter().filter(|s| s.kind == tk && (trole == "plain" || s.role == trole))
        .collect();
    let t = match cands.as_slice() {
        [one] => *one,
        [] => return None,
        many => {
            // several of that kind: the one whose description the asked words name best, if one does
            let mut sc: Vec<(usize, &crate::pack::Sym)> = many.iter().map(|s| {
                let ws: HashSet<String> = words(&s.desc).into_iter().collect();
                (ws.intersection(asked).count(), *s)
            }).collect();
            sc.sort_by(|a, b| b.0.cmp(&a.0));
            if sc[0].0 == 0 || sc[0].0 == sc[1].0 {
                return None;
            }
            sc[0].1
        }
    };
    r.target = Some(t.name.clone());
    for need in &rel.requires {
        if !tags.contains(need) {
            r.doubt.push(format!("the situation '{}' is not stated", need.replace('_', " ")));
        }
    }
    if rel.excludes.iter().any(|x| tags.contains(x)) {
        return None;
    }
    // the stated quantities, each to a symbol of the same dimension (and, among several, the one its words name)
    let syms: Vec<&crate::pack::Sym> = rel.symbols.iter().filter(|s| s.name != t.name).collect();
    let names: Vec<HashSet<String>> = syms.iter().map(|s| {
        let mut set: HashSet<String> = words(&s.desc).into_iter().collect();
        for (ph, (k, role)) in &p.labels {
            if *k == s.kind && (*role == s.role || role == "plain" && s.role == "plain") {
                set.extend(words(ph));
            }
        }
        set
    }).collect();
    let mut scores = vec![vec![0.0f32; syms.len()]; qs.len()];
    for (a, q) in qs.iter().enumerate() {
        for (b, s) in syms.iter().enumerate() {
            let Some(kind) = p.kinds.get(&s.kind) else { continue };
            if kind.dim != q.dim {
                continue;
            }
            let same_dim = syms.iter().filter(|o| p.kinds.get(&o.kind).map(|k| k.dim == q.dim).unwrap_or(false)).count();
            scores[a][b] = if same_dim == 1 { 1.0 + ctx_score(&q.ctx, &names[b], &ix.idf) }
                           else { ctx_score(&q.ctx, &names[b], &ix.idf) };
        }
    }
    let (pairs, left, _) = assign(&scores, 1.25);
    if !left.is_empty() {
        return None;                                   // a stated number this relation does not explain
    }
    let mut bound: HashMap<String, Input> = HashMap::new();
    for (a, b) in pairs {
        bound.insert(syms[b].name.clone(), Input::Text(qs[a].value.clone()));
    }
    // values the wording implies ("from rest")
    let qw = plain_words(toks);
    for im in terms_list(p, "implied") {
        let ph = phrase_words(im["phrase"].as_str().unwrap_or(""));
        if !has_phrase(&qw, &ph) {
            continue;
        }
        let (k, role) = (im["kind"].as_str().unwrap_or(""), im["role"].as_str().unwrap_or("plain"));
        if let Some(s) = syms.iter().find(|s| s.kind == k && s.role == role && !bound.contains_key(&s.name)) {
            let v = im["value"].as_f64().unwrap_or(f64::NAN);
            bound.insert(s.name.clone(), Input::Text(format!("{} {}", py_num(v), im["unit"].as_str().unwrap_or(""))));
        }
    }
    for s in &syms {
        if bound.contains_key(&s.name) {
            continue;
        }
        match &s.default {
            Some(d) if !d.unless.iter().any(|u| tags.contains(u)) => {}
            Some(_) => r.needs.push(s.name.clone()),
            None => r.needs.push(s.name.clone()),
        }
    }
    for s in &rel.symbols {
        if let Some(v) = bound.remove(&s.name) {
            r.inputs.push((s.name.clone(), v));
        }
    }
    Some(r)
}

fn formula_readings(p: &Pack, ix: &Index, toks: &[Tok], ranked: &[(usize, f32)]) -> (Vec<Reading>, Vec<String>) {
    let mut doubt = vec![];
    let qw = plain_words(toks);
    let tw = p.terms.get("unsupported").and_then(|v| v.as_array()).cloned().unwrap_or_default();
    for w in tw.iter().filter_map(|w| w.as_str()) {
        if has_phrase(&qw, &phrase_words(w)) {
            doubt.push(format!("the wording '{w}' is not read by this reader"));
        }
    }
    for k in ["sign", "refine"] {
        for w in p.terms.get(k).and_then(|v| v.as_array()).cloned().unwrap_or_default().iter().filter_map(|w| w.as_str()) {
            let ph = phrase_words(w);
            if has_phrase(&qw, &ph) || (ph.len() == 1 && qw.iter().any(|x| x.starts_with(&ph[0]) && ph[0].len() >= 6)) {
                doubt.push(format!("the wording '{w}' is not read by this reader"));
            }
        }
    }
    let mut tags: HashSet<String> = HashSet::new();
    for c in terms_list(p, "context") {
        if has_phrase(&qw, &phrase_words(c["phrase"].as_str().unwrap_or(""))) {
            tags.insert(c["tag"].as_str().unwrap_or("").to_string());
        }
    }
    // the stated quantities: a number and its unit (a pure number where no unit follows)
    let mut qs = vec![];
    let mut i = 0;
    while i < toks.len() {
        let t = &toks[i];
        if t.k == K::List {
            doubt.push("a list of values".into());
        }
        if t.k == K::Num && !(i > 0 && toks[i - 1].orig == "^") {
            let (value, dim, n) = match unit_after(toks, i) {
                Some((u, n, unit)) => (format!("{} {}", t.orig, u), unit.dim, n),
                None => (t.orig.clone(), Dim::one(), 0),
            };
            let ctx = context(toks, i, n);
            qs.push(Quantity { value, dim, ctx });
            i += 1 + n;
            continue;
        }
        i += 1;
    }
    for (ph, k) in p.terms.get("kind_tags").and_then(|v| v.as_object()).cloned().unwrap_or_default() {
        if let Some(kind) = p.kinds.get(&ph) {
            if qs.iter().any(|q| q.dim == kind.dim && !dim_one(&q.dim)) {
                tags.insert(k.as_str().unwrap_or("").to_string());
            }
        }
    }
    for t in p.terms.get("role_default_tags").and_then(|v| v.as_array()).cloned().unwrap_or_default() {
        if tags.contains(t.as_str().unwrap_or("")) {
            doubt.push("the wording sets which state a stated quantity belongs to".into());
        }
    }
    // the asked quantity, from the asked words: the longest label phrase they contain
    let span = asked_span(toks);
    let mut asked_plain: Vec<String> = vec![];
    if toks.get(span.start).map(|t| t.low == "how").unwrap_or(false) {
        asked_plain.push("how".into());               // "how fast", "how far": the pack's asking idioms
    }
    for w in asked_phrase(toks) {
        asked_plain.extend(w.split('-').filter(|x| !x.is_empty()).map(String::from));
    }
    let asked: HashSet<String> = asked_plain.iter().map(|w| stem(w)).filter(|w| !stopword(w)).collect();
    let mut target: Option<(String, String)> = None;
    let mut best_len = 0;
    let mut tie = false;
    let mut label_words: Vec<String> = vec![];
    let mut labels: Vec<(&String, &(String, String))> = p.labels.iter().collect();
    labels.sort();                                         // a fixed order: the same reading on every run
    for (ph, (k, role)) in labels {
        let w = phrase_words(ph);
        if w.len() >= best_len && has_phrase(&asked_plain, &w) {
            let cand = (k.clone(), role.clone());
            if w.len() > best_len {
                target = Some(cand);
                tie = false;
                best_len = w.len();
                label_words = w;
            } else if target.as_ref() != Some(&cand) {
                tie = true;
            }
        }
    }
    if tie {
        target = None;
    }
    // asked words the label does not cover ("ANGULAR velocity" read as "velocity") must be words of the symbol
    let covered: HashSet<String> = label_words.iter().map(|w| stem(w)).collect();
    let mut uncovered: Vec<String> = asked.iter().filter(|w| !covered.contains(*w) && w.as_str() != "how").cloned().collect();
    uncovered.sort();
    let mut out = vec![];
    for (ci, sc) in ranked.iter().take(40) {
        let Some(rel) = p.relation(&ix.caps[*ci].key) else { continue };
        if let Some(mut r) = read_relation(p, ix, rel, toks, &qs, &tags, &target, &asked) {
            r.score = *sc;
            r.doubt.extend(doubt.iter().cloned());
            if let Some(s) = r.target.as_ref().and_then(|t| rel.sym(t)) {
                let mut own: HashSet<String> = words(&s.desc).into_iter().collect();
                own.extend(s.kind.split('_').map(stem));
                own.extend(s.role.split('_').map(stem));
                if let Some(w) = uncovered.iter().find(|w| !own.contains(*w) && !GENERIC.contains(&w.as_str())) {
                    r.doubt.push(format!("the asked '{w}' is not part of {}", s.desc));
                }
            }
            out.push(r);
        }
    }
    if target.is_none() {
        doubt.push("the asked quantity is not named".into());
    }
    (out, doubt)
}

// ==================================================================================================== ask ====
fn reply(outcome: &str, message: String) -> Value {
    json!({"outcome": outcome, "capability": null, "inputs": {}, "target": null, "message": message})
}

fn cap_name(p: &Pack, key: &str) -> (String, &'static str) {
    if let Some(r) = p.relation(key) {
        return (r.name.clone(), "formula");
    }
    if let Some(pr) = p.procedure(key) {
        return (pr.name.clone(), "procedure");
    }
    (String::new(), "")
}

fn request_json(key: &str, inputs: &[(String, Input)], target: &Option<String>, is_rel: bool) -> Value {
    let mut m = serde_json::Map::new();
    for (s, v) in inputs {
        m.insert(s.clone(), input_json(v));
    }
    json!({"key": key, "inputs": Value::Object(m), "target": if is_rel { json!(target) } else { Value::Null }})
}

/// The reply for a reading: the engine solves it when it is complete; otherwise the capability and what is missing.
fn finish(p: &Pack, r: &Reading, how: &str, alternatives: Vec<Value>) -> Value {
    let (name, kind) = cap_name(p, &r.key);
    let is_rel = kind == "formula";
    let req = request_json(&r.key, &r.inputs, &r.target, is_rel);
    let mut out = json!({"capability": r.key, "name": name, "kind": kind, "inputs": req["inputs"].clone(),
                         "target": r.target, "request": req, "reading": how});
    if !alternatives.is_empty() {
        out["alternatives"] = json!(alternatives);
    }
    if !r.complete() {
        let mut why = vec![];
        if !r.needs.is_empty() {
            why.push(format!("it needs: {}", r.needs.join(", ")));
        }
        if r.unread > 0 {
            why.push(format!("{} stated value(s) could not be placed", r.unread));
        }
        why.extend(r.doubt.iter().cloned());
        out["outcome"] = json!("NEEDS_INPUT");
        out["needs"] = json!(r.needs);
        out["message"] = json!(format!("This looks like {name}, but the question could not be read completely ({}). \
                                        Check the inputs and solve it from the form.", why.join("; ")));
        return out;
    }
    let a: Answer = solve(p, &r.key, &r.inputs, if is_rel { r.target.as_deref() } else { None });
    out["outcome"] = json!(a.outcome);
    out["message"] = json!(if a.solved() { format!("{name}.") } else { a.message.clone() });
    out["answer"] = serde_json::to_value(&a).unwrap_or(Value::Null);
    out
}

fn alt_list(p: &Pack, keys: &[String]) -> Vec<Value> {
    keys.iter().map(|k| json!({"capability": k, "name": cap_name(p, k).0})).collect()
}

/// Read a question in words and answer it, or say exactly why not. See the module comment.
pub fn ask_value(p: &Pack, question: &str) -> Value {
    if question.chars().count() > MAX_QUESTION {
        return reply("INVALID_INPUT", format!("The question is longer than {MAX_QUESTION} characters."));
    }
    let all = tokenize(question);
    if all.len() >= MAX_TOKENS {
        return reply("INVALID_INPUT", "The question is too long to read.".into());
    }
    if all.is_empty() {
        return reply("UNRECOGNIZED_FORMAT", "Type a question, e.g. one of the documented examples.".into());
    }
    let ix = index(p);
    let r = core(&all);
    let q = &all[r];
    // 1. a documented question, numbers free
    let mut hits: Vec<(&PForm, Vec<(String, Input)>)> = vec![];
    for f in &ix.forms {
        if let Some(ins) = match_form(f, q) {
            hits.push((f, ins));
            if hits.len() > 16 {
                break;
            }
        }
    }
    if !hits.is_empty() {
        let keys: Vec<String> = {
            let mut k: Vec<String> = hits.iter().map(|(f, _)| if f.key.is_empty() { format!("!{}", f.outcome) } else { f.key.clone() }).collect();
            k.sort();
            k.dedup();
            k
        };
        let same = |a: &[(String, Input)], b: &[(String, Input)]| {
            a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x.0 == y.0 && input_json(&x.1) == input_json(&y.1))
        };
        // a documented question the package refuses (no capability: its outcome only) whose words are also, with
        // other numbers, a documented question of one capability ("... Vin = 6 V, Vref = 5 V ..."): the package reads
        // it as that capability and the engine refuses these values with its reason. It is answered so only when every
        // capability reading agrees and the engine gives exactly the documented outcome; otherwise it stays AMBIGUOUS
        // below. A refusal is never turned into a number here.
        let refused: Vec<&String> = hits.iter().filter(|(f, _)| f.key.is_empty()).map(|(f, _)| &f.outcome).collect();
        let keyed: Vec<&(&PForm, Vec<(String, Input)>)> = hits.iter().filter(|(f, _)| !f.key.is_empty()).collect();
        if let (Some(want), Some((k0, j0))) = (refused.first(), keyed.first()) {
            let one = refused.iter().all(|o| o == want)
                && keyed.iter().all(|(f, ins)| f.key == k0.key && f.target == k0.target && same(ins, j0));
            if one && want.as_str() != "SOLVED" && !want.is_empty() {
                let mut rd = Reading { key: k0.key.clone(), target: k0.target.clone(), inputs: j0.clone(), ..Default::default() };
                if let Some(pr) = p.procedure(&rd.key) {
                    rd.target = asked_output(pr, &all);
                }
                let out = finish(p, &rd, "form", vec![]);
                if out["outcome"].as_str() == Some(want.as_str()) {
                    return out;
                }
            }
        }
        // a documented question itself, word for word and number for number, whose words, numbers free, are also
        // other documented questions ("Find the determinant of [[1, 2], [3, 4]]." is determinant_2x2's, and with
        // other numbers determinant's and determinant_3x3's; "Interest rate conversion: i = 5, m = 12" is documented
        // UNRECOGNIZED_FORMAT): it has its documented reading and outcome, as the package answers it, not AMBIGUOUS,
        // when every documented question it is word for word agrees
        if keys.len() > 1 {
            let verbatim: Vec<&(&PForm, Vec<(String, Input)>)> = hits.iter()
                .filter(|(f, _)| f.toks.len() == q.len() && f.toks.iter().zip(q).all(|(a, b)| a.orig == b.orig))
                .collect();
            if let Some((v0, j0)) = verbatim.first() {
                let one = verbatim.iter().all(|(f, ins)| f.key == v0.key && f.outcome == v0.outcome
                                                          && f.target == v0.target && same(ins, j0));
                if one && v0.key.is_empty() && !v0.outcome.is_empty() && v0.outcome != "SOLVED" {
                    let mut out = reply(&v0.outcome, "This documented question is not answered by the package: see \
                                                      the capability's guide for what it computes.".into());
                    out["reading"] = json!("form");
                    return out;
                }
                if one && !v0.key.is_empty() {
                    let mut rd = Reading { key: v0.key.clone(), target: v0.target.clone(), inputs: j0.clone(),
                                           ..Default::default() };
                    if let Some(pr) = p.procedure(&rd.key) {
                        rd.target = asked_output(pr, &all);
                    }
                    return finish(p, &rd, "form", vec![]);
                }
            }
        }
        let (f0, i0) = &hits[0];
        let agree = keys.len() == 1 && hits.iter().all(|(f, ins)| f.target == f0.target && same(ins, i0));
        if agree {
            if f0.key.is_empty() {
                let mut out = reply(if f0.outcome.is_empty() { "UNRECOGNIZED_FORMAT" } else { &f0.outcome },
                                    "This documented question is not answered by the package: see the capability's \
                                     guide for what it computes.".into());
                out["reading"] = json!("form");
                return out;
            }
            let mut rd = Reading { key: f0.key.clone(), target: f0.target.clone(), inputs: i0.clone(), ..Default::default() };
            if let Some(pr) = p.procedure(&rd.key) {
                rd.target = asked_output(pr, &all);
            }
            return finish(p, &rd, "form", vec![]);
        }
        let real: Vec<String> = keys.into_iter().filter(|k| !k.starts_with('!')).collect();
        let mut out = reply("AMBIGUOUS", "This question matches documented questions of different capabilities, or \
                                          reads its values differently: choose one.".into());
        out["alternatives"] = json!(alt_list(p, &real));
        out["reading"] = json!("form");
        return out;
    }
    // 2. the reader of its own
    let qwords: Vec<Vec<(String, f32)>> = {
        let mut seen = HashSet::new();
        q.iter().filter(|t| is_word(t)).map(|t| stem(&t.low.replace('\'', "")))
            .filter(|w| !stopword(w) && seen.insert(w.clone())).take(60)
            .map(|w| vec![(w, 1.0f32)]).collect()
    };
    let ranked = rank(&ix, &qwords);
    if ranked.is_empty() {
        return reply("UNRECOGNIZED_FORMAT", "No capability of this model matches these words. Search the \
                                             capabilities or use one of the documented examples.".into());
    }
    let top = ranked[0].1;
    let mut readings: Vec<Reading> = vec![];
    let mut doubt_all = vec![];
    let has_rel = !p.relations.is_empty();
    if has_rel {
        let (rs, d) = formula_readings(p, &ix, &all, &ranked);
        readings.extend(rs);
        doubt_all = d;
    }
    for (ci, sc) in ranked.iter().take(MAX_CANDIDATES) {
        if *sc < 0.3 * top {
            break;
        }
        if let Some(pr) = p.procedure(&ix.caps[*ci].key) {
            let mut rd = read_procedure(p, &ix, pr, &all);
            rd.score = *sc + rd.bonus;
            // wording the formula reader does not interpret ("speeds up", "round trip") is not read here either
            rd.doubt.extend(doubt_all.iter().filter(|d| d.starts_with("the wording")).cloned());
            readings.push(rd);
        }
    }
    readings.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap().then(a.key.cmp(&b.key)));
    if let Some(w) = plain_words(&all).iter().find(|w| NEGATIONS.contains(&w.as_str()) || DIRECTIONS.contains(&w.as_str())) {
        for r in readings.iter_mut() {
            r.doubt.push(format!("the word '{w}' changes the meaning in a way this reader does not read"));
        }
    }
    // a formula read completely is preferred to a procedure (as the package's reader of physical questions does):
    // the procedures then neither compete nor count as the words' favourite
    let formula_done = readings.iter().any(|r| r.complete() && p.relation(&r.key).is_some());
    if formula_done {
        readings.retain(|r| p.relation(&r.key).is_some());
    }
    if std::env::var("AX_ASK_DEBUG").map(|v| v == "1").unwrap_or(false) {
        for r in readings.iter().take(8) {
            eprintln!("{:<40} score {:>7.3} complete {} needs {:?} unread {} doubt {:?} target {:?}", r.key, r.score,
                      r.complete(), r.needs, r.unread, r.doubt, r.target);
        }
    }
    let complete: Vec<&Reading> = readings.iter().filter(|r| r.complete()).collect();
    let alts = |skip: &str| -> Vec<Value> {
        let ks: Vec<String> = readings.iter().map(|r| r.key.clone()).filter(|k| k != skip).take(4).collect();
        alt_list(p, &ks)
    };
    // a complete reading answers only when no other complete reading is close to it in the words, and the words do
    // not point clearly to another capability that could not be read
    let rivals: Vec<&Reading> = match complete.first() {
        // a second method for the same result (a Wald and a Wilson interval's lower limit) is a rival unless the
        // words clearly favour one
        Some(c0) => complete[1..].iter().copied()
            .filter(|r| r.score * RIVAL >= c0.score
                        || ((r.target.is_some() && r.target == c0.target)
                            || result_kind(p, r).is_some() && result_kind(p, r) == result_kind(p, c0))
                           && r.score * SAME_RESULT >= c0.score)
            .collect(),
        None => vec![],
    };
    match complete.as_slice() {
        [c0, ..] if rivals.is_empty() => {
            let best = &readings[0];
            // the words favour a capability that takes the same kinds of inputs and gives a result of the same name
            // (a propped cantilever's hogging moment, complete only as a fixed-fixed beam's): the wording is the only
            // evidence between the two, so its favourite is not overruled because the other could be read completely
            let sibling = || p.procedure(&c0.key).is_some_and(|pr| siblings(&ix, pr).contains(&best.key))
                && result_kind(p, best).is_some() && result_kind(p, best) == result_kind(p, c0);
            if best.key == c0.key || (best.score < FAVOURED * c0.score && !sibling()) {
                return finish(p, c0, "reader", vec![]);
            }
            let mut out = finish(p, best, "reader", alt_list(p, &[c0.key.clone()]));
            out["outcome"] = json!("AMBIGUOUS");
            out.as_object_mut().map(|o| o.remove("answer"));
            out["message"] = json!(format!("This could be {} or {}: choose one.", cap_name(p, &best.key).0,
                                           cap_name(p, &c0.key).0));
            out
        }
        [] => {
            let Some(best) = readings.first() else {
                let mut out = reply("OUT_OF_SCOPE", format!("No capability of this model reads this question{}. \
                                    Search the capabilities, or use a documented example.",
                                    if doubt_all.is_empty() { String::new() } else { format!(" ({})", doubt_all.join("; ")) }));
                let ks: Vec<String> = ranked.iter().take(4).map(|(ci, _)| ix.caps[*ci].key.clone()).collect();
                out["alternatives"] = json!(alt_list(p, &ks));
                return out;
            };
            finish(p, best, "reader", alts(&best.key))
        }
        _ => {
            let many: Vec<&Reading> = std::iter::once(complete[0]).chain(rivals.iter().copied()).collect();
            let ks: Vec<String> = many.iter().map(|r| r.key.clone()).collect();
            let mut out = finish(p, many[0], "reader", alt_list(p, &ks[1..]));
            out["outcome"] = json!("AMBIGUOUS");
            out.as_object_mut().map(|o| o.remove("answer"));
            out["message"] = json!(format!("This question can be read by several capabilities ({}): choose one.",
                                           ks.join(", ")));
            out
        }
    }
}

/// What a procedure reading's result is, in words (the head phrase of its meaning: "hogging moment" for both a
/// propped cantilever's MA and a fixed beam's M_end), to tell two methods for the same result.
fn result_kind(p: &Pack, r: &Reading) -> Option<Vec<String>> {
    let o = p.procedure(&r.key)?.outputs.iter().find(|o| Some(&o.symbol) == r.target.as_ref())?;
    let t = tokenize(&o.meaning);
    let ph: Vec<String> = asked_parts(&t, 0, t.len()).0.iter().map(|w| stem(w)).collect();
    if ph.is_empty() { None } else { Some(ph) }
}

/// ask_value as JSON text (the browser module and the command line).
pub fn ask_json(p: &Pack, question: &str) -> String {
    ask_value(p, question).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokens_read_numbers_lists_words_and_signs() {
        let t = tokenize("Find the mean of [2, 4, 9] and x = -3.5e2 m/s^2, 5%? z-test");
        let kinds: Vec<K> = t.iter().map(|x| x.k.clone()).collect();
        assert_eq!(t[4].k, K::List);
        assert_eq!(t[4].orig, "[2, 4, 9]");
        let n = t.iter().find(|x| x.k == K::Num).unwrap();
        assert_eq!(n.val, -350.0);
        assert!(kinds.contains(&K::Punct));
        assert!(t.iter().any(|x| x.low == "z-test"));
        let m = tokenize("5 - 3");
        assert_eq!(m[2].val, 3.0);                      // a minus between two numbers is not a sign
        assert_eq!(tokenize("x-3")[0].low, "x");
    }

    #[test]
    fn stems_and_stopwords() {
        assert_eq!(stem("trials"), "trial");
        assert_eq!(stem("probabilities"), "probability");
        assert_eq!(stem("class"), "class");
        assert_eq!(words("What is the mean of the values?"), vec!["mean".to_string()]);
    }

    #[test]
    fn core_drops_the_request_words() {
        let a = tokenize("Please find the mean of [1, 2].");
        let b = tokenize("What is the mean of [1, 2]?");
        let (ra, rb) = (core(&a), core(&b));
        let la: Vec<&str> = a[ra].iter().map(|t| t.low.as_str()).collect();
        let lb: Vec<&str> = b[rb].iter().map(|t| t.low.as_str()).collect();
        assert_eq!(la, lb);
    }

    #[test]
    fn edit_distance_is_bounded() {
        assert_eq!(edit_within("kinetic", "kinetik", 1), Some(1));
        assert_eq!(edit_within("energy", "enrgy", 1), Some(1));
        assert_eq!(edit_within("mass", "speed", 2), None);
    }

    #[test]
    fn assignment_needs_a_clear_winner() {
        let (pairs, left, _) = assign(&[vec![2.0, 0.5], vec![0.4, 1.5]], 1.25);
        assert_eq!(pairs.len(), 2);
        assert!(left.is_empty());
        let (pairs, left, _) = assign(&[vec![1.0, 1.0]], 1.25);
        assert!(pairs.is_empty());
        assert_eq!(left, vec![0]);
    }

    #[test]
    fn units_after_a_number_keep_their_case() {
        let t = tokenize("a pressure of 3 MPa and 4 kg/m^3");
        let i = t.iter().position(|x| x.orig == "3").unwrap();
        assert_eq!(unit_after(&t, i).unwrap().0, "MPa");
        let j = t.iter().position(|x| x.orig == "4").unwrap();
        assert_eq!(unit_after(&t, j).unwrap().0, "kg/m^3");
    }

    fn tiny_pack() -> Pack {
        let doc = json!({
            "model": "tiny", "distribution": "tiny-ai999x", "version": "1.0.0",
            "kinds": {"number": {"unit": "1", "dim": ["0","0","0","0","0","0","0","0"], "difference": false}},
            "relations": [], "labels": {},
            "procedures": [{
                "key": "mean", "name": "Arithmetic mean", "method": "sum divided by count", "primary": "mean",
                "native": false, "topic": "descriptive statistics", "keywords": ["average"],
                "templates": ["Find the mean of [2, 4, 9]."], "questions": [],
                "inputs": [{"symbol": "x", "kind": "number", "meaning": "the data values", "required": true, "type": "vector"}],
                "outputs": [{"symbol": "mean", "kind": "number", "meaning": "arithmetic mean", "expr": null}],
                "conditions": [], "equations": [], "assumptions": [], "structured_example": ""
            }],
            "forms": [{"q": "Find the mean of [2, 4, 9].", "outcome": "SOLVED", "key": "mean",
                       "inputs": {"x": "[2, 4, 9]"}},
                      {"q": "Is this question refused?", "outcome": "OUT_OF_SCOPE"}],
        });
        Pack::from_json(&doc.to_string()).unwrap()
    }

    #[test]
    fn search_finds_by_prefix_typo_and_keyword() {
        let p = tiny_pack();
        let r: Value = serde_json::from_str(&search_json(&p, "mea", 5)).unwrap();
        assert_eq!(r[0]["key"], "mean");
        let r: Value = serde_json::from_str(&search_json(&p, "arithmetik ", 5)).unwrap();
        assert_eq!(r[0]["key"], "mean");
        let r: Value = serde_json::from_str(&search_json(&p, "average ", 5)).unwrap();
        assert_eq!(r[0]["key"], "mean");
        let t: Value = serde_json::from_str(&topics_json(&p)).unwrap();
        assert_eq!(t["topics"][0]["topic"], "descriptive statistics");
        let both: Value = serde_json::from_str(&search_all_json(&[&p, &p], "mean", 3)).unwrap();
        assert_eq!(both[0]["model"], "tiny");
    }

    #[test]
    fn tuples_and_loose_lists_are_lists() {
        let t = tokenize("at (1, 2) and P(10, 3) but f(x) and (5)");
        let lists: Vec<&str> = t.iter().filter(|x| x.k == K::List).map(|x| x.orig.as_str()).collect();
        assert_eq!(lists, vec!["(1, 2)", "(10, 3)"]);
        assert_eq!(list_value("(1, 2)"), Some(json!([1, 2])));
        assert_eq!(list_value("[.5, +2, 3]"), Some(json!([0.5, 2.0, 3.0])));
        assert_eq!(list_value("[[1, 2], [3, 4]]").map(|v| depth(&v)), Some(2));
        assert!(list_value("[1, x]").is_none());
    }

    #[test]
    fn the_asked_phrase_is_the_quantity_not_its_conditions() {
        let t = tokenize("Compute the absolute pressure at the depth given a pressure at the surface of 1 Pa.");
        assert_eq!(asked_phrase(&t), vec!["absolute".to_string(), "pressure".to_string()]);
        let t = tokenize("Verify whether the absolute pressure is at least 3 Pa.");
        assert!(asked_phrase(&t).is_empty());
        let t = tokenize("Find the lower limit of the confidence interval for the population variance.");
        let (ph, comp) = asked_full(&t);
        assert_eq!(ph, vec!["lower".to_string(), "limit".to_string()]);
        assert!(comp.contains(&"variance".to_string()));
    }

    fn pack_with(procs: Value, forms: Value) -> Pack {
        let doc = json!({
            "model": "tiny", "distribution": "tiny-ai999x", "version": "1.0.0",
            "kinds": {"number": {"unit": "1", "dim": ["0","0","0","0","0","0","0","0"], "difference": false}},
            "relations": [], "labels": {}, "procedures": procs, "forms": forms,
        });
        Pack::from_json(&doc.to_string()).unwrap()
    }

    fn proc(key: &str, name: &str, inputs: Value, outputs: Value) -> Value {
        json!({"key": key, "name": name, "method": "", "primary": outputs[0]["symbol"], "native": false,
               "topic": "", "keywords": [], "templates": [], "questions": [], "inputs": inputs, "outputs": outputs,
               "conditions": [], "equations": [], "assumptions": [], "structured_example": ""})
    }

    #[test]
    fn forms_read_list_elements_units_and_respect_symbol_case() {
        let n = |s: &str, m: &str| json!({"symbol": s, "kind": "number", "meaning": m, "required": true, "type": "integer"});
        let x = |s: &str, m: &str| json!({"symbol": s, "kind": "number", "meaning": m, "required": true, "type": "number"});
        let o = |s: &str, m: &str| json!([{"symbol": s, "kind": "number", "meaning": m, "expr": null}]);
        let p = pack_with(
            json!([proc("permutations", "Permutations", json!([n("n", "items"), n("k", "chosen")]), o("permutations", "P(n, k)")),
                   proc("arrhenius", "Arrhenius", json!([x("Ea", "activation energy")]), o("k", "rate constant"))]),
            json!([{"q": "Find P(10, 3).", "outcome": "SOLVED", "key": "permutations", "inputs": {"n": 10, "k": 3}},
                   {"q": "What is k when Ea = 75 kJ/mol?", "outcome": "SOLVED", "key": "arrhenius", "inputs": {"Ea": 75000.0}}]));
        let ix = index(&p);
        let read = |q: &str| -> Option<Vec<Value>> {
            let all = tokenize(q);
            let core_toks = &all[core(&all)];
            ix.forms.iter().find_map(|f| match_form(f, core_toks)).map(|v| v.iter().map(|(_, i)| input_json(i)).collect())
        };
        assert_eq!(read("Find P(12, 4)."), Some(vec![json!(12), json!(4)]));
        assert_eq!(read("Find p(12, 4)."), None);               // p(n, k) is another function than P(n, k)
        assert_eq!(read("Find P(12, 4, 5)."), None);
        assert_eq!(read("What is k when Ea = 80 kJ/mol?"), Some(vec![json!(80000.0)]));
        assert_eq!(read("What is k when Ea = 80 kcal/mol?"), None);
    }

    #[test]
    fn the_reader_binds_runs_of_numbers_and_named_values() {
        let v = |s: &str, m: &str| json!({"symbol": s, "kind": "number", "meaning": m, "required": true, "type": "vector"});
        let o = |s: &str, m: &str| json!([{"symbol": s, "kind": "number", "meaning": m, "expr": null}]);
        let p = pack_with(json!([proc("median", "Median", json!([v("x", "the data values")]), o("median", "the median"))]),
                          json!([]));
        let a = ask_value(&p, "What is the median of 3, 9, 4 and 7?");
        assert_eq!(a["capability"], "median");
        assert_eq!(a["inputs"]["x"], json!([3.0, 9.0, 4.0, 7.0]));
        assert_eq!(a["target"], "median");
        // the asked quantity is not this capability's result: never answered by the reader
        let b = ask_value(&p, "What is the ratio of the median to 2 for 3, 9, 4?");
        assert_ne!(b["outcome"], "SOLVED");
        assert!(b["answer"].is_null());
    }

    #[test]
    fn the_asked_result_is_the_output_named_most_exactly() {
        // several outputs share the asked words ("collector current" is in IC and in IC_mA "collector current in
        // milliamperes"): the one whose meaning has fewest words the question does not write is the asked one
        let outs = |v: &[(&str, &str)]| -> Value {
            Value::Array(v.iter().map(|(s, m)| json!({"symbol": s, "kind": "number", "meaning": m, "expr": null}))
                .collect())
        };
        let p = pack_with(json!([
            proc("bjt", "DC operating point of a voltage-divider biased BJT", json!([]),
                 outs(&[("IC_mA", "collector current in milliamperes"), ("IC", "collector current"),
                        ("IB", "base current"), ("IE", "emitter current"), ("VCE", "collector-emitter voltage")])),
            proc("ceiling", "Absolute ceiling of a jet", json!([]),
                 outs(&[("h_abs", "absolute ceiling"), ("h_abs_ft", "absolute ceiling in feet")])),
            proc("cruise", "Breguet range of a jet", json!([]),
                 outs(&[("R_km", "range in km"), ("R", "range"), ("t", "flight time"), ("t_h", "flight time in hours")])),
            proc("tie", "Two equal readings", json!([]),
                 outs(&[("a", "peak current in amperes"), ("b", "peak current in milliamperes")])),
            proc("ber", "Bit error rate of BPSK", json!([]),
                 outs(&[("ber", "BPSK bit error rate"), ("ber_bfsk", "coherent BFSK bit error rate")])),
            proc("beam", "Fixed-fixed beam", json!([]),
                 outs(&[("RA", "reaction at A"), ("RB", "reaction at B"), ("MA", "hogging moment at A")])),
        ]), json!([]));
        let target = |key: &str, q: &str| asked_output(p.procedure(key).unwrap(), &tokenize(q));
        assert_eq!(target("bjt", "Find the collector current.").as_deref(), Some("IC"));
        assert_eq!(target("bjt", "What is the base current?").as_deref(), Some("IB"));
        assert_eq!(target("bjt", "Find the collector-emitter voltage.").as_deref(), Some("VCE"));
        assert_eq!(target("ceiling", "What is the absolute ceiling?").as_deref(), Some("h_abs"));
        assert_eq!(target("cruise", "Find the range.").as_deref(), Some("R"));
        assert_eq!(target("cruise", "Find the flight time.").as_deref(), Some("t"));
        // no meaning is the asked phrase itself: not read, even when one has fewer extra words (a point's capital
        // letter is a word: "reaction at A" is not "the reaction")
        assert_eq!(target("tie", "Find the peak current."), None);
        assert_eq!(target("ber", "What is the bit error rate?"), None);
        assert_eq!(target("beam", "Determine the reaction at the fixed end A."), None);
        assert_eq!(target("beam", "Find the hogging moment at A."), Some("MA".into()));
    }

    #[test]
    fn a_documented_form_reads_new_numbers() {
        let p = tiny_pack();
        let ix = index(&p);
        let f = &ix.forms[0];
        assert!(matches!(f.inputs[0].1, FVal::Slot(_, Tf::ListText { brackets: true })));
        let all = tokenize("What is the mean of [1, 5, 6, 8]?");
        let ins = match_form(f, &all[core(&all)]).unwrap();
        assert_eq!(input_json(&ins[0].1), json!("[1, 5, 6, 8]"));
        assert!(match_form(f, &tokenize("the median of [1, 2]")).is_none());
        let v = ask_value(&p, "Is this question refused?");
        assert_eq!(v["outcome"], "OUT_OF_SCOPE");
        let v = ask_value(&p, &"x".repeat(MAX_QUESTION + 1));
        assert_eq!(v["outcome"], "INVALID_INPUT");
    }
}
