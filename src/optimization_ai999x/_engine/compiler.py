# GENERATED from ai999x_factory/runtime by tools/build_package.py — do not edit.
"""Text -> typed problem. Deterministic and conservative: it reads what is asked, binds every number to a
kind of quantity and a role, and refuses when anything in the question is left unexplained.

The compiler knows no subject; phrases, kinds and implied facts come from the domain pack. It is the part a
small trained model will later assist — the typed `Problem` it emits is the model's training target.
"""
from __future__ import annotations

import re
from dataclasses import asdict, dataclass, field, replace

from ._lazy import sp
from .pack import Pack
from .quantity import UnitError, parse_unit, to_si, words_to_unit

_NUM = re.compile(r"(?<![\w.^/*])[-−+]?(?:\d+(?:,\d{3})+(?:\.\d+)?|\d+(?:\.\d+)?|\.\d+)"
                  r"(?:\s*[x×]\s*10\s*\^?\s*[-−]?\d+|[eE][-+]?\d+)?")
_SENT = re.compile(r"[.?!;](?=\s|$)")
_ASK = re.compile(r"\b(what|how|find|calculate|determine|compute|estimate|evaluate|give)\b", re.I)
# an interrogative that directly names the asked quantity: "what is the", "how much", "find the total" + <quantity>
_INTERROGATIVE = re.compile(r"\b(?:what(?:\s+(?:is|are|was|were|will\s+be|would\s+be))?(?:\s+the)?|which|how\s+(?:much|many)|"
                            r"(?:find|determine|calculate|compute|estimate|evaluate|give)(?:\s+the)?)"
                            r"(?:\s+(?:total|net|new|resulting|final|initial|average|maximum|minimum|required|effective|local))?"
                            r"\s+$")
_ASKED_UNIT = re.compile(r"\b(?:in\s+terms\s+of|in\s+units\s+of|expressed\s+in|in)\s+([a-z/^\-\d\s]+?)\s*(?:\?|$|\.|,|\))")


def _asked_unit_conflict(question: str, kind_unit, kinds=None) -> str | None:
    """The unit the question explicitly asks for ("... in radians per second?", "... in terms of mass?"), when its
    dimension is not that of the target's kind; None when no unit is asked or it agrees (or cannot be read: then the
    old behaviour holds). A kind's name stands for its unit."""
    from .quantity import UnitError, parse_unit
    if not kind_unit:
        return None
    m = None
    for m in _ASKED_UNIT.finditer(question):
        pass                                          # the last "in <unit>" of the question sentence
    if m is None:
        return None
    words = m.group(1).strip()
    named = (kinds or {}).get(words.replace(" ", "_"))
    if named is not None and getattr(named, "unit", None):
        words_unit = named.unit
        try:
            same = parse_unit(re.sub(r"\brad\b", "1", words_unit)).dim == parse_unit(re.sub(r"\brad\b", "1", kind_unit)).dim
        except (UnitError, ValueError, KeyError, ZeroDivisionError):
            return None
        return None if same else words
    sym = " " + words + " "
    for w, s in _UNIT_WORDS:
        sym = re.sub(rf"\s{w}\s", f" {s} ", sym)
    sym = re.sub(r"\s*/\s*", "/", re.sub(r"\s+per\s+", "/", sym)).strip().replace(" ", "*")

    def dim(u):                                       # radians are dimensionless: rad/s and 1/s are one dimension
        return parse_unit(re.sub(r"\brad\b", "1", u)).dim
    try:
        asked, want = dim(sym), dim(kind_unit)
    except (UnitError, ValueError, KeyError, ZeroDivisionError):
        return None
    return None if asked == want else words


_UNIT_WORDS = [(w, s) for s, ws in {
    "rad": ("radians", "radian"), "s": ("seconds", "second", "secs", "sec"), "m": ("meters", "meter", "metres", "metre"),
    "km": ("kilometers", "kilometer", "kilometres", "kilometre"), "cm": ("centimeters", "centimeter", "centimetres", "centimetre"),
    "kg": ("kilograms", "kilogram"), "g": ("grams", "gram"), "Hz": ("hertz",), "N": ("newtons", "newton"),
    "J": ("joules", "joule"), "W": ("watts", "watt"), "A": ("amperes", "ampere", "amps", "amp"), "V": ("volts", "volt"),
    "ohm": ("ohms",), "Pa": ("pascals", "pascal"), "K": ("kelvin",), "h": ("hours", "hour"), "min": ("minutes", "minute"),
}.items() for w in ws]
# words for the END of the motion; checked before _INITIAL, so "just before it hits the floor" is the final state
_FINAL = re.compile(r"\b(after|afterwards|final|finally|at the end|ends up|reaches|just before|hits|lands|strikes|"
                    r"impact|leaves|leave)\b", re.I)
_INITIAL = re.compile(r"\b(initial|initially|at the start|at first|before)\b", re.I)
_CONNECT = re.compile(r"^[\s:=]*(?:(?:of|is|was|equals|equal to|μ|mu)[\s:=]*)*$", re.I)
# what separates a value from a label written after it in another clause: punctuation, a conjunction, an interrogative
# or a verb that starts a new statement ("195 V, determine the peak voltage", "1.05e6 1 and the Rayleigh number")
_CLAUSE_BREAK = re.compile(r"[,;:()?]|\b(?:and|or|but|nor|what|which|how|find|calculate|determine|compute|estimate|"
                           r"evaluate|give|if|when|while|where|whereas|then|so|is|are|was|were|will|would|should|"
                           r"does|do|did)\b", re.I)


@dataclass(frozen=True)
class Given:
    id: int
    kind: str
    role: str
    value: float                 # SI
    text: str                    # the words it came from
    source: str = "stated"       # stated | implied | default
    exact: object = None         # an exact value (angles in degrees) when one exists
    assumption: str = ""


@dataclass(frozen=True)
class Target:
    kind: str
    role: str
    phrase: str


@dataclass
class Problem:
    status: str                  # compiled | needs_clarification | unsupported | invalid
    question: str
    target: Target | None = None
    givens: list = field(default_factory=list)
    tags: tuple = ()
    message: str = ""
    code: str = ""               # why it did not compile, as a fixed word (see engine.UNCERTAINTY)

    def to_dict(self) -> dict:
        d = asdict(self)
        for g in d["givens"]:
            g["exact"] = None if g["exact"] is None else str(g["exact"])
        return d


def _number(text: str) -> float:
    t = text.replace("−", "-").replace(",", "")
    m = re.match(r"\s*([-+]?[\d.]+)\s*[x×]\s*10\s*\^?\s*(-?\d+)", t)
    return float(m.group(1)) * 10.0 ** int(m.group(2)) if m else float(t)


def _strip(tok: str) -> str:
    tok = tok.rstrip(".,;:?!")
    while tok.endswith(")") and tok.count(")") > tok.count("("):     # "(n=0.035 s/m^(1/3))": the sentence's bracket
        tok = tok[:-1].rstrip(".,;:?!")
    if tok.startswith("(") and ")" not in tok:
        tok = tok[1:]
    return tok


def _unit_after(text: str, pos: int):
    """The longest unit spelled right after a number: (unit object, spelling, end position) or None."""
    start = pos + re.match(r"[ \t]*-?[ \t]*", text[pos:]).end()       # "3 kg", "3-kg", "3kg", "30°"
    # up to six words: "kilogram meters squared per second" is five (with four, the reading stopped at kg*m^2)
    toks = [(start + t.start(), t.group()) for t in re.finditer(r"\S+", text[start:start + 80])][:6]
    for k in range(len(toks), 0, -1):
        words = [_strip(t) for _, t in toks[:k]]
        if not all(words):
            continue
        if any(w != t for w, (_, t) in zip(words[:-1], toks[:k - 1])):
            continue                                   # punctuation inside: the unit would span a clause break
        spelled = words_to_unit(words)
        try:
            unit = parse_unit(spelled)
        except UnitError:
            continue
        last_start, last_tok = toks[k - 1]
        lead = 1 if last_tok.startswith("(") and ")" not in last_tok else 0
        return unit, spelled, last_start + lead + len(words[-1])
    return None


_WORDS_RX: dict = {}


def _word(phrase: str, end: bool = True):
    """A vocabulary phrase as a whole-word pattern, compiled once: the vocabulary is larger than the regex module's own
    cache (512), and recompiling every phrase on every question cost 80% of the answer time."""
    key = (phrase, end)
    rx = _WORDS_RX.get(key)
    if rx is None:
        rx = _WORDS_RX[key] = re.compile(r"(?<![a-z])" + re.escape(phrase) + (r"(?![a-z])" if end else ""))
    return rx


def _occurrences(text: str, entries: list) -> list:
    """Non-overlapping phrase matches, longest phrase first: (start, end, entry)."""
    low, taken, out = text.lower(), [], []
    for e in sorted(entries, key=lambda e: -len(e["phrase"])):
        for m in _word(e["phrase"].lower()).finditer(low):
            if all(m.end() <= a or m.start() >= b for a, b in taken):
                taken.append((m.start(), m.end()))
                out.append((m.start(), m.end(), e))
    return sorted(out, key=lambda o: o[0])


def context_tags(terms: dict, low: str) -> set:
    """The context tags the (lower-cased) wording produces. An entry may name `unless_tag` (a tag or a list): its tag is
    not produced by that phrase when the wording also produces one of those tags. "pendulum" inside "double pendulum"
    must not make the SIMPLE pendulum's relations applicable: the more specific wording contradicts their assumption
    (a point mass on one light string)."""
    hits = [c for c in terms.get("context", []) if _word(c["phrase"]).search(low)]
    found = {c["tag"] for c in hits}

    def blocked(c):
        u = c.get("unless_tag") or ()
        return bool(set([u] if isinstance(u, str) else u) & found)
    return {c["tag"] for c in hits if not blocked(c)}


def _symbol_uses(text: str, terms: dict, spans: list, tags: set) -> tuple:
    """A pack's declared symbols written as NOTATION: `zeta=0.5`, `I 0.01 kg*m^2`, `b?`. Matched case-sensitively and
    never inside a measurement; a symbol that is also an English word (`"word": true`, e.g. "a") counts as a given
    only with '=' or ':'. Returns ({position the number must start at: [entries]}, [(start, end, entry) asked])."""
    given, asked = {}, []
    for e in terms.get("symbols", []):
        if (e.get("when_tag") and e["when_tag"] not in tags) or (e.get("unless_tag") and e["unless_tag"] in tags):
            continue
        for m in re.finditer(r"(?<![\w.^*/])" + re.escape(e["symbol"]) + r"(?![\w^])", text):
            if any(a <= m.start() < b for a, b in spans):
                continue
            after = text[m.end():]
            sep = re.match(r"[ \t]*([=:])?[ \t]*", after)
            if sep.group(1) or not e.get("word"):
                given.setdefault(m.end() + sep.end(), []).append(e)
            if re.match(r"\s*(?:=\s*)?(?:in\s+[^\s?]+\s*)?\?", after):
                asked.append((m.start(), m.end(), e))
    return given, asked


def _one_reading(entries: list):
    """The entry when all entries agree on kind and role, else None (a symbol with two meanings is not read)."""
    readings = {(e["kind"], e.get("role", "plain")) for e in entries}
    return entries[0] if len(readings) == 1 else None


def _sentence(text: str, pos: int) -> tuple:
    cuts = [0] + [m.end() for m in _SENT.finditer(text)] + [len(text)]
    for a, b in zip(cuts, cuts[1:]):
        if a <= pos < b:
            return a, b
    return 0, len(text)


_HINT = re.compile(r"\(\s*hint\b[^)]*\)|(?:(?<=[.?!])|^)\s*hint\s*:[^.?!]*[.?!]?", re.I)

# Notation, not meaning: each rewrite turns a common way of WRITING a number or unit into the one the reader reads,
# and none adds or removes a quantity. Numbers that are not measurements (a year, an exercise number) are removed.
def _drop_unit_definition(m: re.Match) -> str:
    """'(1 kilometer equals 3,281 feet)', '1 W = 1 J/s': a conversion between two units, not a measurement."""
    try:
        parse_unit(words_to_unit([m.group(1)]) or "")
        parse_unit(words_to_unit(m.group(2).split()) or "")
    except UnitError:
        return m.group(0)
    return " "


def _drop_reference(m: re.Match) -> str:
    """'Exercise 9.19' is a reference, not a measure: capitalized as written, and never followed by a unit
    ('a table 1.2 m high' keeps its number)."""
    if _unit_after(m.string, m.end()) is not None:
        return m.group(0)
    return m.group(1)


_NUMBER_WORDS = {"zero": 0, "one": 1, "two": 2, "three": 3, "four": 4, "five": 5, "six": 6, "seven": 7, "eight": 8,
                 "nine": 9, "ten": 10, "eleven": 11, "twelve": 12, "thirteen": 13, "fourteen": 14, "fifteen": 15,
                 "sixteen": 16, "seventeen": 17, "eighteen": 18, "nineteen": 19, "twenty": 20, "thirty": 30,
                 "forty": 40, "fifty": 50, "sixty": 60, "seventy": 70, "eighty": 80, "ninety": 90, "hundred": 100}
# a number written in words is still a stated number when a unit or a counted thing follows it ("every five seconds",
# "ten complete waves"): written as digits it must be used like any other, or the question is refused
# (physics-questions-3: "ten complete waves ... every five seconds" was ignored and a free fall answered instead)
_NUM_WORD = r"(?:" + "|".join(sorted(_NUMBER_WORDS, key=len, reverse=True)) + r"|thousand)"
_SPELLED_MEASURE = re.compile(
    r"\b(" + _NUM_WORD + r"(?:(?:[\s-]+(?:and\s+)?|\s+point\s+)" + _NUM_WORD + r")*)\s+"
    r"(?=(?:(?:complete|full|whole|equal|identical|more|extra)\s+)?"
    r"(?:seconds?|minutes?|hours?|days?|years?|met(?:er|re)s?|kilomet(?:er|re)s?|centimet(?:er|re)s?|"
    r"millimet(?:er|re)s?|kilograms?|grams?|newtons?|joules?|watts?|volts?|amperes?|amps?|ohms?|hertz|kelvins?|"
    r"degrees?|pascals?|lit(?:er|re)s?|coulombs?|farads?|henr(?:y|ies)|teslas?|waves?|oscillations?|cycles?|"
    r"revolutions?|rotations?|turns?|vibrations?|swings?|crests?|pulses?|laps?|loops?|wavelengths?)\b)", re.I)


def spelled_number(phrase: str):
    """'twelve hundred' -> 1200, 'forty-five' -> 45, 'zero point five' -> 0.5, 'two thousand and fifty' -> 2050;
    None when the words do not form one number (then the text is left as written)."""
    words = [w for w in re.split(r"[\s-]+", phrase.lower()) if w and w != "and"]
    if "point" in words:
        i = words.index("point")
        whole = spelled_number(" ".join(words[:i])) if i else 0
        digits = words[i + 1:]
        if whole is None or not digits or any(_NUMBER_WORDS.get(d, 10) > 9 for d in digits):
            return None
        return float(f"{int(whole)}." + "".join(str(_NUMBER_WORDS[d]) for d in digits))
    total = current = 0
    last = None
    for w in words:
        if w == "hundred":
            current = (current or 1) * 100
        elif w == "thousand":
            total += (current or 1) * 1000
            current = 0
        elif w in _NUMBER_WORDS:
            v = _NUMBER_WORDS[w]
            if last in _NUMBER_WORDS and last != "hundred" and not (_NUMBER_WORDS[last] >= 20 and v < 10 and _NUMBER_WORDS[last] % 10 == 0):
                return None                                # "five three" is not one number
            current += v
        else:
            return None
        last = w
    return total + current


def _spelled(m) -> str:
    value = spelled_number(m.group(1))
    if value is None:
        return m.group(0)
    return f"{value:g} "


_NOTATION = [
    (_SPELLED_MEASURE, _spelled),
    (re.compile(r"\^\s*\(\s*[–—]"), "^(-"),                                               # en dash as a minus sign
    (re.compile(r"(?:\(\s*)?(?:assume\s+that\s+)?\b1\s+([A-Za-zΩ]+)\s+(?:equals|=|is equal to)\s+[\d,.]+\s+"
                r"([A-Za-zΩ/^\d]+(?:\s+per\s+[A-Za-z]+)?)\s*(?:\.\s*)?(?:\)|(?=[.;?]))", re.I), _drop_unit_definition),
    (re.compile(r"(\d)\s*[*x×·]\s*10\s*\^\s*\(\s*([-−+]?\d+)\s*\)"), r"\1e\2"),        # 3.33*10^(-2) -> 3.33e-2
    (re.compile(r"(\d)\s*[*×·x]\s*10\s*\^\s*([-−+]?\d+)"), r"\1e\2"),                  # 1.2*10^4, 1.2·10^4, 1.2 x 10^4
    (re.compile(r"\^\s*\(\s*([-−]?\d+)\s*\)"), r"^\1"),                                  # m/s^(2) -> m/s^2
    (re.compile(r"º"), "°"),                                                              # ordinal sign as degree
    (re.compile(r"°\s+([CF])\b"), r"°\1"),                                                # "1100° C"
    (re.compile(r"(\d)\s*[-–]\s*([A-Za-zμΩ]{1,3})\s*[-–]\s*(long|diameter|thick|wide|high|tall|deep|radius|"
                r"wavelength)\b"), r"\1 \2 \3"),                                         # "1.00-m-long" -> "1.00 m long"
    (re.compile(r"(\d)\s+([A-Za-zμ]{1,3})\s*[–]\s*(long|diameter|thick|wide|high|tall|deep)\b"), r"\1 \2 \3"),
    (re.compile(r"\bmm\s+Hg\b"), "mmHg"),
    (re.compile(r"\b(Exercises?|Examples?|Problems?|Chapters?|Sections?|Figures?|Tables?|Equations?|Eq\.)\s+"
                r"\d+(?:\.\d+)*(?:\s*(?:and|or|,)\s*\d+(?:\.\d+)*)*(?![\d.])"), _drop_reference),  # "Exercise 9.19" names
    (re.compile(r"\b(in|since|by|during|until|of|from)\s+(1[5-9]\d\d|20\d\d)\b(?!\s*[-–]?\s*(?:[A-Za-z°%/]|\d))",
                re.I), r"\1 that year"),                                                  # "In 1967, ..."
    (re.compile(r"\(\s*(?:about|approximately|roughly|i\.e\.,?|or about)\s+[^()]*?\d[^()]*\)", re.I), " "),   # a restatement
]


def normalize_notation(text: str) -> str:
    for pat, rep in _NOTATION:
        text = pat.sub(rep, text)
    return text


def compile_problem(question: str, pack: Pack) -> Problem:
    if not isinstance(question, str) or not question.strip():
        return Problem("invalid", str(question), message="Expected a question in words.", code="invalid_input")
    if len(question) > 4000:
        return Problem("invalid", question[:80], message="The question is too long.", code="invalid_input")
    # a hint names an intermediate step, never the quantity asked for ("(Hint: first find the angular velocity)")
    text = _HINT.sub(" ", normalize_notation(question)).strip() or question
    terms, low = pack.terms, text.lower()

    for word in terms.get("unsupported_structure", []):
        if _word(word).search(low):
            return Problem("unsupported", text, message=f"The question involves '{word}', a problem structure "
                           "(several bodies, stages or comparisons) this model does not handle yet.",
                           code="unsupported_structure")

    # 1. numbers and the units written after them
    measures, bare = [], []
    for m in _NUM.finditer(text):
        if any(a <= m.start() < b for a, b, *_ in measures):
            continue                                   # a digit inside a unit already read ("m/s^2")
        try:
            value = _number(m.group())
        except ValueError:
            continue
        found = _unit_after(text, m.end())
        if found is None:
            bare.append((m.start(), m.end(), value))
        else:
            unit, spelled, end = found
            measures.append((m.start(), end, value, unit, spelled))

    labels = _occurrences(text, pack.reading_labels())
    claimed: set = set()
    givens: list = []
    pre_tags = context_tags(terms, low)
    sym_given, sym_asked = _symbol_uses(text, terms, [(a, b) for a, b, *_ in measures], pre_tags)

    # 2. a bare number is accepted only as a labelled pure number (a coefficient); otherwise we refuse
    for s, e, value in bare:
        sym = _one_reading([x for x in sym_given.get(s, []) if pack.kinds[x["kind"]].unit == "1"])
        if sym is not None:                            # "zeta=0.5": a declared symbol of a pure number
            givens.append(Given(len(givens), sym["kind"], sym.get("role", "plain"), value, text[s:e]))
            continue
        hit = None
        for i, (ls, le, entry) in enumerate(labels):
            if i in claimed or pack.kinds[entry["kind"]].unit != "1":
                continue
            if 0 <= s - le <= 25 and _CONNECT.match(text[le:s]):
                hit = (i, entry)
        if hit is None:
            return Problem("needs_clarification", text, message=f"The number '{text[s:e]}' has no unit I recognise, "
                           "so I cannot tell what it measures.", code="unexplained_number")
        claimed.add(hit[0])
        givens.append(Given(len(givens), hit[1]["kind"], hit[1].get("role", "plain"), value, text[s:e]))

    # 3a. "from A to B": two values of one kind, the earlier and the later state
    forced: dict = {}
    for i in range(len(measures) - 1):
        (s1, e1, _v1, u1, _), (s2, e2, _v2, u2, _) = measures[i], measures[i + 1]
        if u1.dim != u2.dim or not text[max(0, s1 - 14):s1].lower().rstrip().endswith("from"):
            continue
        if text[e1:s2].strip().lower() not in ("to", "down to", "up to"):
            continue
        for k in pack.kinds_for(u1.dim):
            if {"initial", "final"} <= pack.roles_of(k):
                forced[i], forced[i + 1] = {"kind": k, "role": "initial"}, {"kind": k, "role": "final"}
                break
    for mi, (s, _e, _v, unit, _sp) in enumerate(measures):    # "I 0.01 kg*m^2", "d=1 m": the symbol names it
        sym = _one_reading([x for x in sym_given.get(s, []) if x["kind"] in pack.kinds_for(unit.dim)])
        if sym is not None and mi not in forced:
            forced[mi] = sym
    # 3. each measurement gets a kind (from its unit) and a role (from the nearest free label of that kind)
    pairs = []
    for mi, (s, e, value, unit, spelled) in enumerate(measures):
        kinds = pack.kinds_for(unit.dim)
        if not kinds:
            return Problem("unsupported", text, message=f"'{text[s:e]}' is a kind of quantity this model has no "
                           "relations for.", code="unsupported_quantity")
        a, b = _sentence(text, s)
        for li, (ls, le, entry) in enumerate(labels):
            if entry["kind"] not in kinds or not (a <= ls < b):
                continue
            gap = s - le if le <= s else ls - e
            if gap < 0 or gap > (45 if le <= s else 20):
                continue
            lo, hi = (le, s) if le <= s else (e, ls)
            if any(lo <= ms < hi for ms, *_ in measures if ms != s):
                continue                               # another measurement stands between them
            if le > s and _CLAUSE_BREAK.search(text[e:ls]):
                # a label AFTER the value names it only within its own clause: in "8280 1, what is the skin-friction
                # coefficient" it is the asked quantity, in "49100 1 and a Prandtl number" the next value's label
                continue
            pairs.append((gap + (3 if le > s else 0), mi, li))
    role_of: dict = dict(forced)
    for _gap, mi, li in sorted(pairs):
        if mi not in role_of and li not in claimed:
            role_of[mi] = labels[li][2]
            claimed.add(li)
    for mi, (s, e, value, unit, spelled) in enumerate(measures):
        entry = role_of.get(mi)
        kinds = pack.kinds_for(unit.dim) if entry else pack.kind_by_spelling(unit.dim, spelled)
        if entry is None and set(kinds) == {"temperature", "temperature_difference"}:
            before = text[max(0, s - 14):s].lower()             # "at 300 K" is a state; "by 30 K" is a change
            if re.search(r"\bby\s*$", before):
                kinds = ["temperature_difference"]
            elif re.search(r"\b(at|of|is|to|from|reaches|reach)\s*$", before):
                kinds = ["temperature"]
        if entry is None and len(kinds) > 1:
            return Problem("needs_clarification", text, message=f"'{text[s:e]}' could be " + " or ".join(
                k.replace("_", " ") for k in kinds) + "; say which.", code="ambiguous_quantity")
        kind = entry["kind"] if entry else kinds[0]
        role = entry.get("role", "plain") if entry else "plain"
        if (kind, role) == ("temperature", "plain") and "temperature_difference" in pack.kinds and \
                re.search(r"\bby\s*$", text[max(0, s - 14):s].lower()):
            kind = "temperature_difference"                # "the temperature rises by 30 K": a change, not a state
        si = to_si(value, unit, difference=pack.kinds[kind].difference)
        exact = None
        if unit.pi_factor is not None:
            exact = sp.pi * sp.Rational(str(value)) * sp.Rational(unit.pi_factor.numerator, unit.pi_factor.denominator)
        a, b = _sentence(text, s)
        for mod in terms.get("sign", []):
            if mod["applies_to"] == kind and _word(mod["phrase"], end=False).search(low[a:b]):
                si, exact = si * mod["factor"], (None if exact is None else exact * mod["factor"])
        givens.append(Given(len(givens), kind, role, si, text[s:e], exact=exact))

    # 4. context: tags, and facts the wording implies ("from rest")
    tags = tuple(sorted(context_tags(terms, low)))
    kind_tags = terms.get("kind_tags", {})             # a stated quantity of some kinds is itself a context
    tags = tuple(sorted(set(tags) | {kind_tags[g.kind] for g in givens if g.kind in kind_tags}))
    for imp in terms.get("implied", []):
        if not _word(imp["phrase"]).search(low):
            continue
        if any(g.kind == imp["kind"] and g.role == imp["role"] for g in givens) or set(imp.get("unless", ())) & set(tags):
            continue
        if imp.get("when_tag") and imp["when_tag"] not in tags:          # "from rest" implies no ROTATION only when rotating
            continue
        unit = parse_unit(imp["unit"])
        exact = None
        if unit.pi_factor is not None:
            exact = sp.pi * sp.Rational(str(imp["value"])) * sp.Rational(unit.pi_factor.numerator,
                                                                         unit.pi_factor.denominator)
        givens.append(Given(len(givens), imp["kind"], imp["role"], to_si(imp["value"], unit), imp["phrase"],
                            source="implied", exact=exact, assumption=imp.get("assumption", imp["phrase"])))

    # 5. what is asked: the first unclaimed quantity phrase in the question sentence
    cuts = [0] + [m.end() for m in _SENT.finditer(text)] + [len(text)]
    spans = [(a, b) for a, b in zip(cuts, cuts[1:]) if text[a:b].strip()]
    asking = [sp_ for sp_ in spans if "?" in text[sp_[0]:sp_[1]]] or [sp_ for sp_ in spans if _ASK.search(text[sp_[0]:sp_[1]])]
    if not asking:
        return Problem("needs_clarification", text, givens=givens, tags=tags,
                       message="I could not find what the question asks for.", code="no_question")
    qa, qb = asking[-1]
    target = None
    # a label marked `"asked": false` names a given only ("exhaust 3000 m/s"), never the quantity asked for
    asked = [(ls, -(le - ls), entry, text[ls:le]) for li, (ls, le, entry) in enumerate(labels)
             if li not in claimed and qa <= ls < qb and entry.get("asked", True)]
    idioms = []
    by_pos: dict = {}
    for ss, se, e in sym_asked:                         # "b?", "I in kg*m^2?": the symbol is what is asked
        if qa <= ss < qb:
            by_pos.setdefault((ss, se), []).append(e)
    for (ss, se), entries in by_pos.items():
        sym = _one_reading(entries)
        if sym is not None:
            asked.append((ss, -(se - ss), sym, text[ss:se]))
            idioms.append(ss)
    for idiom in terms.get("ask_idioms", []):          # "how long", "how high": as good a sign as a label
        m = _word(idiom["phrase"]).search(low[qa:qb])
        if m:
            asked.append((qa + m.start(), -len(idiom["phrase"]), idiom, idiom["phrase"]))
            idioms.append(qa + m.start())
    if asked:
        # the quantity the interrogative names ("how much CURRENT", "what is the DISTANCE") outranks an earlier
        # incidental label ("a 120-volt POWER line, how much current ..."); then the earliest; the longer on a tie
        def anchored(pos):
            return pos in idioms or bool(_INTERROGATIVE.search(low[max(qa, pos - 48):pos]))
        _pos, _len, entry, phrase = min(asked, key=lambda x: (not anchored(x[0]), x[0], x[1]))
        target = Target(entry["kind"], entry.get("role", "plain"), phrase)
        conflict = _asked_unit_conflict(low[qa:qb], pack.kinds[target.kind].unit if target.kind in pack.kinds else None,
                                        pack.kinds)
        if conflict:
            return Problem("needs_clarification", text, givens=givens, tags=tags,
                           message=f"The question asks for {conflict}, which is not a {target.kind.replace('_', ' ')}; "
                                   "I will not answer a different quantity.", code="asked_unit_conflict")
    if target is None:
        return Problem("needs_clarification", text, givens=givens, tags=tags,
                       message="I could not recognise the quantity being asked for, so I will not guess one.",
                       code="unknown_target")
    if target.role == "plain":
        roles = pack.roles_of(target.kind)
        if "final" in roles and _FINAL.search(text[qa:qb]):
            target = Target(target.kind, "final", target.phrase)
        elif "initial" in roles and _INITIAL.search(text[qa:qb]):
            target = Target(target.kind, "initial", target.phrase)
    for ref in terms.get("target_refinements", []):   # "how long ... to reach the top": which time is meant
        if (ref.get("when_tag") and ref["when_tag"] not in tags) or (ref.get("unless_tag") and ref["unless_tag"] in tags):
            continue
        if target.kind == ref["kind"] and target.role == ref.get("from", "plain") and any(
                _word(p).search(low[qa:qb]) for p in ref["phrases"]):
            target = Target(target.kind, ref["role"], target.phrase)
            break
    # 6. a pack may say what role an unlabelled quantity plays in a context: with `asked`, how the stated value
    #    and the question relate in time ("moving at 10 m/s, accelerates ... how fast?"); without it, what the
    #    one quantity of that kind must be there ("a ramp at 30 degrees": the angle of the incline)
    for rd in terms.get("role_defaults", []):
        # the one UNLABELLED stated quantity of this kind takes the context's role; labelled ones keep theirs
        same = [g for g in givens if g.kind == rd["kind"] and g.role == "plain"]
        # without `when_tag` the role holds in every context (a stated gravitational constant is the constant G)
        if not ((not rd.get("when_tag") or rd["when_tag"] in tags) and len(same) == 1 and same[0].source == "stated"):
            continue
        if "asked" in rd:
            if not (target.kind == rd["kind"] and target.role == "plain"):
                continue
            target = Target(target.kind, rd["asked"], target.phrase)
        givens[same[0].id] = replace(same[0], role=rd["given"], assumption=rd.get("assumption", ""))
    return Problem("compiled", text, target, givens, tags)
