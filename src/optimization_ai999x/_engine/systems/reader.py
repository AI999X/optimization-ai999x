# GENERATED from ai999x_factory/runtime by tools/build_package.py — do not edit.
"""Reading a question as a SCENARIO: several bodies with their own identities, quantities in states (initial / final)
and stages, shared environment, and the asked quantity. Deterministic and conservative, like the single-relation
compiler: a number that cannot be placed is reported, never guessed.

A body is anchored by its mass ("a 1200-kg car", "a trash can that is 80.0 kg"). A measured quantity belongs to the
body whose anchor precedes it most closely in the same sentence (or, failing that, the one that follows it). Words
such as "after", "final" or "rebounds" put a velocity in the final state.
"""
from __future__ import annotations

import re
from dataclasses import dataclass, field

from ..compiler import _HINT, _NUM, _number, _unit_after, _word, context_tags, normalize_notation
from ..quantity import to_si

_STOP_NOUN = {"that", "which", "is", "was", "and", "with", "moving", "traveling", "travelling", "at", "of", "in", "on",
              "to", "has", "it", "the", "a", "an", "initially", "going", "rolling", "sliding", "heading", "while", "from"}
FINAL_CUES = re.compile(r"\b(after|afterwards?|final|finally|rebounds?|bounces? back|moves? off|recoils?|"
                        r"just after|immediately after|then moves?|ends? up|leaves?)\b", re.I)
REST = re.compile(r"\b(at rest|stationary|initially at rest|motionless|not moving|parked|from rest|resting|rests)\b", re.I)


@dataclass
class Quantity:
    start: int
    end: int
    value: float              # SI
    kinds: list
    unit: str
    text: str
    kind: str = ""
    body: int | None = None
    state: str = "initial"
    stage: int = 0
    each: bool = False         # stated once for every body ("3 m/s each")


@dataclass
class Body:
    idx: int
    anchor: int               # position of its mass in the text
    mass: Quantity | None
    noun: str = ""


@dataclass
class Scenario:
    question: str
    text: str
    low: str
    quantities: list
    bodies: list
    bare: list                 # numbers without a unit: (start, end, value)
    sentences: list            # (start, end)
    asking: tuple              # (start, end) of the question sentence
    tags: set = field(default_factory=set)
    stages: list = field(default_factory=list)      # (start, end) of each stage segment

    def of_kind(self, kind, state=None, body=None, stage=None):
        return [q for q in self.quantities if q.kind == kind and (state is None or q.state == state)
                and (body is None or q.body == body) and (stage is None or q.stage == stage)]

    def has(self, *patterns) -> bool:
        return any(re.search(p, self.low) for p in patterns)

    def ask_has(self, *patterns) -> bool:
        a, b = self.asking
        return any(re.search(p, self.low[a:b]) for p in patterns)

    def sentence_of(self, pos):
        return next(((a, b) for a, b in self.sentences if a <= pos < b), (0, len(self.text)))


def _kind_for(pack, unit, spelled) -> list:
    """Kinds of this dimension, the domain's preferred one first (its families declare KIND_PREFERENCE)."""
    kinds = pack.kind_by_spelling(unit.dim, spelled)
    prefer = getattr(pack, "kind_preference", None) or []
    return sorted(kinds, key=lambda k: prefer.index(k) if k in prefer else len(prefer))


def _noun_after(text, pos):
    words = re.findall(r"[A-Za-z]+", text[pos:pos + 40])
    out = []
    for w in words:
        if w.lower() in _STOP_NOUN:
            break
        out.append(w.lower())
        if len(out) == 2:
            break
    return " ".join(out)


def _noun_before(text, pos):
    seg = text[max(0, pos - 45):pos]
    m = re.search(r"(?:a|an|the)\s+([A-Za-z]+(?:\s+[A-Za-z]+)?)\s+(?:that|which)\s+(?:is|has a mass of|weighs|of)\s*$", seg, re.I) \
        or re.search(r"(?:a|an|the)\s+([A-Za-z]+(?:\s+[A-Za-z]+)?)\s+(?:of|with a mass of|of mass)\s*$", seg, re.I)
    return m.group(1).lower() if m else ""


def read(question: str, pack) -> Scenario | None:
    if not isinstance(question, str) or not question.strip() or len(question) > 4000:
        return None
    text = _HINT.sub(" ", normalize_notation(question)).strip()
    low = text.lower()
    cuts = [0] + [m.end() for m in re.finditer(r"[.?!;](?=\s|$)", text)] + [len(text)]
    sentences = [(a, b) for a, b in zip(cuts, cuts[1:]) if text[a:b].strip()]
    asking = next((s for s in reversed(sentences) if "?" in text[s[0]:s[1]]), None) or next(
        (s for s in reversed(sentences) if re.search(r"\b(what|how|find|calculate|determine|compute)\b", low[s[0]:s[1]])),
        sentences[-1] if sentences else (0, len(text)))
    quantities, bare = [], []
    for m in _NUM.finditer(text):
        if any(q.start <= m.start() < q.end for q in quantities):
            continue
        try:
            value = _number(m.group())
        except ValueError:
            continue
        found = _unit_after(text, m.end())
        if found is None:
            bare.append((m.start(), m.end(), value))
            continue
        unit, spelled, end = found
        kinds = _kind_for(pack, unit, spelled)
        if not kinds:
            continue
        kind = kinds[0]
        if kind == "temperature" and "temperature_difference" in kinds and re.search(r"\bby\s*$", low[max(0, m.start() - 6):m.start()]):
            kind = "temperature_difference"
        si = to_si(value, unit, difference=pack.kinds[kind].difference)
        quantities.append(Quantity(m.start(), end, si, kinds, spelled, text[m.start():end], kind))
    # a quantity repeated in the question sentence ("the 1 kg ball", "the 2 uF capacitor") names something already
    # stated: it is a reference, not a new given
    qa, qb = asking
    quantities = [q for q in quantities if not (qa <= q.start < qb and any(
        o.kind == q.kind and o.start < qa and abs(o.value - q.value) <= 1e-12 * max(1.0, abs(q.value)) for o in quantities))]
    tags = context_tags(pack.terms, low)
    sc = Scenario(question, text, low, quantities, [], bare, sentences, asking, tags)
    # bodies: one per stated mass; "two 2 kg balls" (or "two identical 2 kg balls") is two bodies of that mass
    for q in quantities:
        if q.kind == "mass":
            noun = _noun_after(text, q.end) or _noun_before(text, q.start)
            sc.bodies.append(Body(len(sc.bodies), q.start, q, noun))
            q.body = sc.bodies[-1].idx
            if re.search(r"\b(two|both)\s+(?:identical\s+|equal\s+|similar\s+)?$", low[max(0, q.start - 20):q.start]):
                sc.bodies.append(Body(len(sc.bodies), q.start, q, noun))
    _bind(sc)
    for q in quantities:                                   # "3 m/s each", "each moving at 3 m/s": every body
        if re.match(r"\s*each\b", low[q.end:q.end + 8]) or re.search(r"\beach (?:moving |traveling |travelling |going )?(?:at|of|with)\s*$", low[max(0, q.start - 30):q.start]):
            q.each = True
    return sc


_ORDINAL = re.compile(r"(?<!\bat )\b(first|second)\s+(?!(?:time|times|later|half|stage|stages|collision|interval|part|"
                      r"phase|attempt|trial|run|case|second|seconds|minute|minutes|hour|hours|of|and|or|to)\b)[a-z]+", re.I)


def _ordinal_body(sc: Scenario, seg: str) -> int | None:
    """The body a stretch of wording names by its ordinal ("the final velocity of the first body", "a stationary second
    cart"): 0 or 1, or None when it names none or both. Ordinal names outrank position: "the first body" is body 0
    wherever the sentence mentions it."""
    found = {m.group(1).lower() for m in _ORDINAL.finditer(seg)}
    if len(sc.bodies) < 2 or len(found) != 1:
        return None
    return 0 if found == {"first"} else 1


def _bind(sc: Scenario) -> None:
    prev_end = 0
    for q in sc.quantities:
        start_seg, prev_end = prev_end, q.end
        if q.kind == "mass" or not sc.bodies:
            continue
        a, b = sc.sentence_of(q.start)
        before = [bd for bd in sc.bodies if bd.anchor < q.start]
        same = [bd for bd in before if a <= bd.anchor]
        named = _ordinal_body(sc, sc.low[max(a, start_seg):q.start])
        if named is not None:
            q.body = named                                # "... of the first body is 1.22 m/s": named, not nearest
        elif same:
            q.body = same[-1].idx
        elif q.start < sc.asking[0] and before and re.search(r"\b(it|its|the [a-z]+)\b", sc.low[a:q.start]):
            q.body = before[-1].idx                       # "It is moving at ..." refers back
        else:
            after = [bd for bd in sc.bodies if bd.anchor > q.start and a <= bd.anchor < b]
            q.body = after[0].idx if after else (before[-1].idx if before else None)
        clause = sc.low[max(a, q.start - 60):q.start]
        after = sc.low[q.end:min(b, q.end + 30)]
        if re.search(r"\b(initially|originally|at first|before)\b[^.;]{0,25}$", clause):
            continue                                     # "is initially moving at 25 m/s just after it hits ...": initial
        if FINAL_CUES.search(clause) or re.match(r"\s*,?\s*(?:immediately |just |right |shortly )?after\b", after) \
                or re.search(r"\b(combined|together|both|they|the pair)\b[^.;]*$", clause):
            q.state = "final"


def rest_bodies(sc: Scenario) -> set:
    """Bodies the wording says are at rest initially ("a stationary 500 kg car", "a trash can ... initially at rest")."""
    out = set()
    for m in REST.finditer(sc.low):
        a, b = sc.sentence_of(m.start())
        nxt = re.search(r"\d", sc.low[m.end():b])
        named = _ordinal_body(sc, sc.low[m.end():m.end() + nxt.start() if nxt else b][:40])
        if named is not None and re.match(r"\s*(?:the|a|an)?\s*(?:first|second)\b", sc.low[m.end():]):
            out.add(named)                                 # "a stationary second body": the body the ordinal names
            continue
        ahead = [bd for bd in sc.bodies if m.end() <= bd.anchor <= m.end() + 25 and a <= bd.anchor < b]
        behind = [bd for bd in sc.bodies if bd.anchor < m.start() and a <= bd.anchor]
        if ahead and not re.search(r"\d", sc.low[m.end():ahead[0].anchor]):
            out.add(ahead[0].idx)
        elif behind:
            out.add(behind[-1].idx)
        elif sc.bodies and not ahead:
            prev = [bd for bd in sc.bodies if bd.anchor < m.start()]
            if prev:
                out.add(prev[-1].idx)
    return out


def body_named(sc: Scenario, span) -> int | None:
    """The body a phrase of the question refers to: by its noun, its mass, or first/second."""
    a, b = span
    seg = sc.low[a:b]
    if re.search(r"\b(first|former|incoming)\b", seg):
        return 0
    if re.search(r"\b(second|latter|target)\b", seg) and len(sc.bodies) >= 2:
        return 1
    by_mass = [bd.idx for bd in sc.bodies if bd.mass is not None
               and re.search(r"(?<![\d.])" + re.escape(bd.mass.text.lower().replace(" ", "")), seg.replace(" ", ""))]
    if len(by_mass) == 1:                                  # "the 1 kg ball": the stated mass decides
        return by_mass[0]
    hits = []
    for bd in sc.bodies:
        if bd.mass is not None and bd.mass.text.lower().replace(" ", "") in seg.replace(" ", ""):
            hits.append(bd.idx)
        elif bd.noun and re.search(r"\b" + re.escape(bd.noun.split()[-1]) + r"s?\b", seg):
            hits.append(bd.idx)
    nouns = [bd.noun.split()[-1] for bd in sc.bodies if bd.noun]
    hits = sorted(set(hits))
    if len(hits) == 1 and (len(set(nouns)) == len(nouns) or any(bd.mass and bd.mass.text.lower().replace(" ", "") in seg.replace(" ", "") for bd in sc.bodies)):
        return hits[0]
    return None


def percent_of(sc: Scenario, pattern) -> float | None:
    """A percentage stated next to a phrase ("90.0% of its final voltage") as a fraction."""
    m = re.search(r"(\d+(?:\.\d+)?)\s*%\s*(?:of\s+)?" + pattern, sc.low)
    return float(m.group(1)) / 100.0 if m else None
