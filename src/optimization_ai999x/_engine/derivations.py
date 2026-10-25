# GENERATED from ai999x_factory/runtime by tools/build_package.py — do not edit.
"""Certified derivations and proofs as capabilities (docs/CONSOLIDATION_PLAN.md, phase C3).

    prove("<derivation id>")                       -> the certified steps, each justified, with what it rests on
    ask("Prove that (a + b)^2 = a^2 + 2ab + b^2.")  -> the same, matched by MATHEMATICAL equivalence of the equation
    ask("Given a*x + b = c, show that x = (c - b)/a.")   a conditional theorem needs its hypotheses stated

Documented forms: "Prove that <eq>.", "Derive <eq>.", "Show that <eq>.", "Prove the <title>.", and for a conditional
theorem "Prove that if <hyps> then <eq>." / "Given <hyps>, show that <eq>." A statement no certified derivation
proves is OUT_OF_SCOPE; a conditional result asked without its hypotheses is NEEDS_INPUT naming them. The steps were
checked mechanically when the derivation was certified (tools/certify_derivations.py); nothing is executed here.
"""
from __future__ import annotations

import json
import re
from functools import lru_cache
from pathlib import Path

import sympy as sp

from . import expr as E
from .answer import Answer

_FORM = re.compile(r"^\s*(?:(?:please\s+)?(?:prove|derive|show))(?:\s+that)?\s*:?\s*(?P<body>.+?)\s*[.?!]?\s*$", re.I | re.S)
_GIVEN = re.compile(r"^\s*given\s+(?P<hyps>.+?),\s*(?:prove|derive|show)(?:\s+that)?\s+(?P<eq>.+?)\s*[.?!]?\s*$", re.I | re.S)
_IF = re.compile(r"^\s*if\s+(?P<hyps>.+?)\s+then\s+(?P<eq>.+)$", re.I | re.S)
_SAFE = re.compile(r"^[\w\s+\-*/^().=,²³]+$")


_LOADED: dict = {}


def load(folder) -> list:
    p = Path(folder) / "capabilities" / "derivations_certified.json"
    key = (str(p), p.stat().st_mtime_ns if p.exists() else 0)
    if key not in _LOADED:
        _LOADED[key] = json.loads(p.read_text(encoding="utf-8"))["derivations"] if p.exists() else []
    return _LOADED[key]


def _sides(text: str, symbols: list):
    return _sides_cached(text, tuple(symbols))


@lru_cache(maxsize=4096)
def _sides_cached(text: str, symbols: tuple):
    """(lhs, rhs) of one equation, as written, read with the given symbols ('2ab' is 2*a*b when a and b are symbols)."""
    t = text.strip().rstrip(".").replace("^", "**").replace("²", "**2").replace("³", "**3").replace("−", "-")
    if t.count("=") != 1 or not _SAFE.match(t):
        return None
    symbols = list(symbols)
    names = sorted(symbols, key=len, reverse=True)

    def split(m):                                         # a run of letters that is not one symbol: a product
        w = m.group(0)
        if w in symbols or w in E.FUNCS or w in E.CONSTS:
            return w
        out, i = [], 0
        while i < len(w):
            hit = next((n for n in names if w.startswith(n, i)), None)
            if hit is None:
                return w                                  # an unknown name: the statement is not about these symbols
            out.append(hit)
            i += len(hit)
        return "*".join(out)
    t = re.sub(r"[A-Za-z_][A-Za-z_0-9]*", split, t)
    t = re.sub(r"(?<=[\d)])\s*(?=[A-Za-z(])", "*", t)     # 2a -> 2*a, (a+b)(a-b) -> (a+b)*(a-b)
    t = re.sub(r"\b([A-Za-z_][A-Za-z_0-9]*)\s*\(", lambda m: m.group(1) + ("(" if m.group(1) in E.FUNCS else "*("), t)
    syms = {n: sp.Symbol(n) for n in set(re.findall(r"[A-Za-z_][A-Za-z_0-9]*", t)) if n not in E.FUNCS and n not in E.CONSTS}
    try:
        lhs, rhs = (E.parse(side, syms, symbolic_powers=True) for side in t.split("="))
    except E.ExpressionError:
        return None
    return lhs, rhs


def _symbols_of(entry: dict) -> list:
    text = " ".join([entry["equation"], *entry.get("hypotheses", [])])
    return sorted({n for n in re.findall(r"[A-Za-z_][A-Za-z_0-9]*", text) if n not in E.FUNCS and n not in E.CONSTS})


def equivalent(stated: str, certified: str, symbols: list) -> bool:
    """The same statement. An IDENTITY (true for all values: every algebra theorem) is compared side by side in
    its written structure — "(a + b)^2 = a^2 + 2ab + b^2" matches its certified form or the mirror image, and a false
    variant never does (their differences are all zero, so a difference test would match any identity to any other).
    Any other equation matches up to rearrangement and a nonzero constant factor."""
    a, b = _sides(stated, symbols), _sides(certified, symbols)
    if a is None or b is None:
        return False
    if (a[0] - a[1]).free_symbols != (b[0] - b[1]).free_symbols and (a[0].free_symbols | a[1].free_symbols) != (b[0].free_symbols | b[1].free_symbols):
        return False                                      # not about the same quantities: no symbolic work needed
    da, db = sp.simplify(a[0] - a[1]), sp.simplify(b[0] - b[1])
    if db == 0:                                           # the certified statement is an identity
        same = lambda x, y: x == y                       # structural: term order is canonical, form is not
        return da == 0 and ((same(a[0], b[0]) and same(a[1], b[1])) or (same(a[0], b[1]) and same(a[1], b[0])))
    if da == 0:
        return False
    if sp.simplify(da - db) == 0 or sp.simplify(da + db) == 0:
        return True
    ratio = sp.simplify(da / db)
    return bool(ratio.is_number and ratio.is_finite and ratio != 0)


def _hyp_list(text: str) -> list:
    return [h.strip() for h in re.split(r"\s*(?:,|\band\b)\s*", text) if "=" in h]


def match(question: str, entries: list):
    """(entry, missing hypotheses) for a prove-form question, (None, None) when no certified derivation states it,
    or None when the question is not a prove form at all."""
    q = question.strip()
    g = _GIVEN.match(q)
    m = _FORM.match(q)
    if not g and not m:
        return None
    if g:
        stated_hyps, eq = _hyp_list(g.group("hyps")), g.group("eq")
    else:
        body = m.group("body")
        iff = _IF.match(body)
        stated_hyps, eq = (_hyp_list(iff.group("hyps")), iff.group("eq")) if iff else ([], body)
    title = re.sub(r"^the\s+", "", eq.strip().rstrip(".").lower())
    for e in entries:                                   # "Prove the binomial square expansion."
        if "=" not in eq and title == e["title"].lower():
            return e, [h for h in e.get("hypotheses", []) if not stated_hyps]
    if "=" not in eq:
        return None, None
    hits = []
    for e in entries:
        syms = _symbols_of(e)
        if equivalent(eq, e["equation"], syms):
            missing = [h for h in e.get("hypotheses", []) if not any(equivalent(s, h, syms) for s in stated_hyps)]
            hits.append((e, missing))
    if not hits:
        return None, None
    hits.sort(key=lambda h: len(h[1]))                  # the reading whose hypotheses were all stated first
    return hits[0]


def answer(entry: dict, question: str, missing: list | None = None) -> Answer:
    if missing:
        a = Answer("needs_clarification", question, models=[entry["id"]],
                   message="This result is certified only under its hypotheses: " + "; ".join(missing)
                           + f". Ask: {entry['questions'][0]}")
        a.uncertainty = {"state": "missing_information", "cause": "needs_input", "outcome": "NEEDS_INPUT",
                         "needs": missing}
        return a
    steps = [{"equation": s["equation"], "reason": s.get("reason", "")} for s in entry["steps"]]
    rests = [f"{p['equation']} ({p['is'].replace('relation:', 'certified relation ')})" for p in entry.get("premises", [])]
    lines = ([f"Given {', '.join(entry['hypotheses'])}:"] if entry.get("hypotheses") else []) + \
            [f"{i}. {s['equation']}   [{s['reason']}]" if s["reason"] else f"{i}. {s['equation']}" for i, s in enumerate(steps, 1)] + \
            [f"Therefore {entry['equation']}."]
    a = Answer("solved", question, models=[entry["id"]], steps=steps, assumptions=list(entry.get("hypotheses", [])),
               message="\n".join(lines))
    a.verification = {"certified_derivation": entry["id"], "rests_on": rests, "certificate": entry.get("certificate", {})}
    a.uncertainty = {"state": "determined", "capability": entry["id"]}
    return a


def unknown(question: str, entries: list) -> Answer:
    a = Answer("unsupported", question,
               message="No certified derivation proves this statement. Certified: "
                       + "; ".join(e["title"] for e in entries[:8]) + ("; …" if len(entries) > 8 else "")
                       + ". See derivations() for the exact question forms.")
    a.uncertainty = {"state": "unsupported", "cause": "unsupported_structure", "outcome": "OUT_OF_SCOPE"}
    return a
