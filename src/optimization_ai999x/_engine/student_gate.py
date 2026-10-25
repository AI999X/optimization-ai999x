# GENERATED from ai999x_factory/runtime by tools/build_package.py — do not edit.
"""The gate between a trained reader and the engine.

A learned model reads a question into a typed problem. It is never allowed to answer: its reading goes through
deterministic checks, then through the planner and the engine's own verification, exactly like a typed problem a
user supplied. A reading is used only if ALL of these hold:

  well formed      it decodes to a target and givens with known kinds and units of the right dimension
  grounded         every given is a number that stands in the question with a unit of that dimension (or a fact
                   a phrase of the question implies); each number is used once; its exact written value replaces
                   the model's rounded copy. A sign differs from the written number only where a sign phrase of the
                   pack ("decelerates") says so
  complete         every measured number in the question is read
  kind             a number whose unit names one kind ("J" is an energy) is read as that kind, and a number the
                   rule-based reader types is read as the same kind
  role             where the wording fixes a role (a label such as "initial speed", "from A to B"), the reading
                   gives that role — an exchange of initial and final values is refused
  target           the asked quantity is the one the wording asks for, kind and (where the wording fixes it) role
  conditions       every context tag is produced by a phrase of the question, and every condition the question
                   states is kept
  confident        the model's mean token log-probability is at least the threshold chosen on validation data
  solved           the engine solves and verifies the typed problem (its own applicability rules apply)
  consistent       if the rule-based compiler also answers, both answers agree

Every accepted reading carries its provenance: which values were stated, which a phrase implied, which sign a
convention fixed, and which defaults the engine assumed.

What the gate cannot see: a role the wording does not fix (two unlabelled speeds) read the wrong way round and
still solvable. The agreement check and the threshold reduce that risk; only held-out measurement shows how far.

Pure Python over the runtime: no model library is imported here.
"""
from __future__ import annotations

import re
from dataclasses import dataclass, field

from .answer import Answer
from .compiler import _NUM, _number, _unit_after, compile_problem, context_tags
from .engine import ask
from .pack import Pack
from .quantity import UnitError, parse_unit, to_si

DEFAULT_THRESHOLD = None          # no reading is trusted until a threshold has been chosen on validation data


def decode(text: str) -> dict | None:
    """`ask <kind> <role> | <kind> <role> <value> <unit> | ... | tags a b` -> typed problem, or None."""
    try:
        parts = [p.strip() for p in (text or "").strip().split("|")]
        head = parts[0].split()
        if len(head) != 3 or head[0] != "ask":
            return None
        out = {"target": {"kind": head[1], "role": head[2]}, "given": [], "tags": []}
        for p in parts[1:]:
            tok = p.split()
            if not tok:
                return None
            if tok[0] == "tags":
                out["tags"] = tok[1:]
            elif len(tok) == 4:
                out["given"].append({"kind": tok[0], "role": tok[1], "value": float(tok[2]), "unit": tok[3].replace("**", "^")})
            else:
                return None
        return out
    except (ValueError, IndexError, AttributeError):
        return None


def measurement_spans(question: str) -> list:
    """Every number in the question with the unit written after it, with its position:
    [(start, end, SI value, dimension or None, text, spelled, written value, unit or None)]. A lone digit without a
    unit ("2 carts") is not a measurement."""
    out = []
    for m in _NUM.finditer(question):
        try:
            value = _number(m.group())
        except ValueError:
            continue
        found = _unit_after(question, m.end())
        if found is None:
            if not re.fullmatch(r"\d", m.group()):
                out.append((m.start(), m.end(), value, None, m.group(), "", value, None))
        else:
            unit, spelled, end = found
            out.append((m.start(), end, to_si(value, unit), unit.dim, question[m.start():end], spelled, value, unit))
    return out


def measurements(question: str) -> list:
    """Every number in the question with the unit written after it: [(SI value, dimension or None, text, spelled)]."""
    return [(s[2], s[3], s[4], s[5]) for s in measurement_spans(question)]


def _close(a: float, b: float, rtol: float = 1e-5) -> bool:
    return abs(a - b) <= rtol * max(1.0, abs(a), abs(b))


def _has(low: str, phrase: str) -> bool:
    return re.search(r"(?<![a-z])" + re.escape(phrase.lower()) + r"(?![a-z])", low) is not None


def text_tags(question: str, pack: Pack) -> set:
    return context_tags(pack.terms, question.lower())


@dataclass
class GateResult:
    passed: bool
    reason: str
    problem: dict | None = None
    answer: Answer | None = None
    checks: dict = field(default_factory=dict)
    provenance: list = field(default_factory=list)


def _fail(check, reason, checks, problem=None):
    checks[check] = False
    return GateResult(False, reason, problem=problem, checks=checks)


def check_reading(question: str, reading: str, pack: Pack, confidence: float | None = None,
                  threshold: float | None = DEFAULT_THRESHOLD, *, structural_only: bool = False) -> GateResult:
    """`structural_only` runs every check except confidence and the engine (used to audit training labels)."""
    checks: dict = {}
    p = decode(reading) if isinstance(reading, str) else reading
    checks["well_formed"] = p is not None and p["target"]["kind"] in pack.kinds and all(g["kind"] in pack.kinds for g in p["given"])
    if not checks["well_formed"]:
        return GateResult(False, "the reading is not a well-formed typed problem", checks=checks)
    p = {"target": dict(p["target"]), "given": [dict(g) for g in p["given"]], "tags": list(p.get("tags", []))}
    low = question.lower()
    found = measurements(question)
    try:
        comp = compile_problem(question, pack)
    except Exception:                                  # the compiler never decides for the gate; it only informs it
        comp = None
    comp_roles = {g.text: (g.kind, g.role) for g in (comp.givens if comp else []) if g.source == "stated"}
    signed = [s for s in pack.terms.get("sign", [])]
    used, provenance = set(), []
    for g in p["given"]:
        try:
            unit = parse_unit(g["unit"])
        except UnitError:
            return _fail("grounded", f"unknown unit {g['unit']}", checks)
        kind = pack.kinds[g["kind"]]
        if unit.dim != kind.dim:
            return _fail("grounded", f"{g['unit']} is not a unit of {g['kind']}", checks)
        si = to_si(g["value"], unit, difference=kind.difference)
        hit, how = None, "stated"
        for i, (v, dim, _t, _s) in enumerate(found):
            if i in used or not (dim == kind.dim or (dim is None and kind.unit == "1")):
                continue
            if _close(v, si):
                hit = i
                break
            if _close(v, -si) and any(s["applies_to"] == g["kind"] and _has(low, s["phrase"]) for s in signed):
                hit, how = i, "sign_convention"
                break
        if hit is not None:
            used.add(hit)
            v, dim, t, spelled = found[hit]
            named = pack.kind_by_spelling(dim, spelled) if dim is not None and spelled else []
            if len(named) == 1 and named[0] != g["kind"]:
                return _fail("kind", f"'{t}' is a {named[0].replace('_', ' ')}, not a {g['kind'].replace('_', ' ')}", checks)
            if t in comp_roles:
                ck, cr = comp_roles[t]
                if ck != g["kind"]:
                    return _fail("kind", f"the wording makes '{t}' a {ck.replace('_', ' ')}, the reading a {g['kind'].replace('_', ' ')}", checks)
                if cr != "plain" and g["role"] != cr:
                    return _fail("role", f"the wording makes '{t}' the {cr} {ck.replace('_', ' ')}; the reading says {g['role']}", checks)
            g["value"], g["unit"] = (v if how == "stated" else -v), kind.unit
            provenance.append({"quantity": f"{g['kind']}/{g['role']}", "from": how, "text": t})
            continue
        implied = [e for e in pack.terms.get("implied", []) if e["kind"] == g["kind"] and _has(low, e["phrase"])
                   and _close(to_si(e["value"], parse_unit(e["unit"])), si)]
        if not implied:
            return _fail("grounded", f"the reading uses {g['value']} {g['unit']}, which the question does not state", checks)
        if implied[0]["role"] != g["role"]:
            return _fail("role", f"'{implied[0]['phrase']}' fixes the {implied[0]['role']} {g['kind']}; the reading says {g['role']}", checks)
        provenance.append({"quantity": f"{g['kind']}/{g['role']}", "from": "implied", "text": implied[0]["phrase"]})
    checks["grounded"] = checks["kind"] = checks["role"] = True
    unread = [t for i, (_v, _d, t, _s) in enumerate(found) if i not in used]
    checks["complete"] = not unread
    if unread:
        return _fail("complete", f"the reading leaves out what the question states: {', '.join(unread)}", checks)
    if comp is not None and comp.target is not None:
        ct = comp.target
        if ct.kind != p["target"]["kind"] or (ct.role != "plain" and p["target"]["role"] != ct.role):
            return _fail("target", f"the question asks for the {ct.role} {ct.kind}; the reading asks for the "
                                   f"{p['target']['role']} {p['target']['kind']}".replace("plain ", ""), checks)
    checks["target"] = True
    stated_tags = text_tags(question, pack)
    kind_tags = {(pack.terms.get("kind_tags") or {}).get(g["kind"]) for g in p["given"]} - {None}
    unsupported = [t for t in p["tags"] if t not in stated_tags and t not in kind_tags]
    if unsupported:
        return _fail("conditions", f"no phrase of the question supports the conditions {unsupported}", checks)
    ignored = sorted(stated_tags - set(p["tags"]))
    if ignored:
        return _fail("conditions", f"the reading ignores conditions the question states: {ignored}", checks)
    checks["conditions"] = True
    if structural_only:
        return GateResult(True, "", problem=p, checks=checks, provenance=provenance)
    checks["confident"] = threshold is not None and confidence is not None and confidence >= threshold
    if not checks["confident"]:
        return GateResult(False, "below the confidence threshold" if threshold is not None else "no threshold has been set",
                          problem=p, checks=checks, provenance=provenance)
    a = ask(p, pack)
    checks["solved"] = a.solved
    if not a.solved:
        return GateResult(False, f"the engine does not solve this reading: {a.message}", problem=p, checks=checks, provenance=provenance)
    defaults = list((a.uncertainty or {}).get("assumed_defaults", []))
    a.uncertainty = {**(a.uncertainty or {}), "reading": "trained reader through the gate",
                     "reading_provenance": provenance + [{"quantity": d, "from": "default"} for d in defaults]}
    return GateResult(True, "", problem=p, answer=a, checks=checks, provenance=provenance)


def gated_ask(question: str, reading: str, pack: Pack, confidence: float | None, threshold: float | None) -> tuple:
    """(Answer or None, source, reason). The rule-based reading first; the trained reading only through the gate,
    and never against a rule-based answer that differs."""
    rule = ask(question, pack)
    g = check_reading(question, reading, pack, confidence, threshold) if reading is not None else None
    if rule.solved:
        if g is not None and g.passed and not _close(g.answer.value, rule.value, 1e-6):
            return None, "refused", "the trained reading and the rule-based reading disagree"
        return rule, "rule_based", ""
    if g is not None and g.passed:
        return g.answer, "student", ""
    return None, "refused", (g.reason if g is not None else rule.message)
