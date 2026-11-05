# GENERATED from ai999x_factory/runtime by tools/build_package.py — do not edit.
"""Explanation templates filled with the engine's own values (offline, no network, no model at run time).

A capability's explanation (`knowledge/explanations.json`, written once by `tools/explanation_templates.py` and checked
by `knowledge/explanation_check.py`) is plain prose with placeholders; every number a reader sees comes from the answer:

    {in:<symbol>}    an input's value with its unit          (algorithms)
    {out:<symbol>}   an output's value with its unit         (algorithms)
    {val:<symbol>}   a quantity's value with its unit, given or computed (formulas: any symbol may be asked)
    {unit:<symbol>}  the unit symbol alone

The same rules are implemented in Rust (`rust/ax-engine/src/explain.rs`) and JavaScript (`AXCompute.explain` in
`publish/website/compute/compute.js`); `tests/fixtures/explanations_render.json` holds the cases all three must render
to identical text.

A view (what a template is filled from) is JSON-shaped:
    {"inputs": {"<symbol>": {"value": <v>, "unit": "<unit>"}}, "outputs": {"<symbol>": {"value": <v>, "unit": "..."}}}
Values: an integer (a JSON integer) in full; any other number as Python's format(x, ".6g"); text as written; true/false
as "yes"/"no"; a list as "[a, b]"; {"re", "im"} as "a + bi"; another object as "{k: v, ...}". A unit "" or "1" is a pure
number and is not written. A placeholder whose symbol has no value (or no unit, for {unit:}) makes the whole text
unavailable: render() returns None and the caller shows another level or nothing, never a gap.
"""
from __future__ import annotations

import json
import math
import re

PLACEHOLDER = re.compile(r"\{(in|out|val|unit):([A-Za-z_][A-Za-z0-9_]*)\}")
LEVELS = ("short", "standard", "deep", "reading")


def g6(x: float) -> str:
    """Python's format(x, ".6g") (the website's number format)."""
    return format(float(x), ".6g")


def show(v) -> str | None:
    """One value as prose writes it; None for a missing value."""
    if v is None:
        return None
    if isinstance(v, bool):
        return "yes" if v else "no"
    if isinstance(v, int):
        return str(v)
    if isinstance(v, float):
        return g6(v)
    if isinstance(v, str):
        return v
    if isinstance(v, complex):
        v = {"re": v.real, "im": v.imag}
    if isinstance(v, (list, tuple)):
        parts = [show(x) for x in v]
        return None if any(p is None for p in parts) else "[" + ", ".join(parts) + "]"
    if isinstance(v, dict):
        if set(v) == {"re", "im"}:
            re_, im = show(float(v["re"])), float(v["im"])
            neg = im < 0 or (im == 0 and math.copysign(1.0, im) < 0)
            return f"{re_} {'-' if neg else '+'} {g6(abs(im))}i"
        parts = [(str(k), show(x)) for k, x in v.items()]
        return None if any(p is None for _, p in parts) else "{" + ", ".join(f"{k}: {p}" for k, p in parts) + "}"
    try:                                             # a Fraction, numpy scalar, ...: by its float value
        return g6(float(v))
    except (TypeError, ValueError):
        return str(v)


def _unit(u) -> str:
    u = "" if u is None else str(u)
    return "" if u in ("", "1") else u


def render(text: str, view) -> str | None:
    """`text` with every placeholder filled from `view` (a dict, or its JSON text); None if one cannot be filled."""
    if isinstance(view, str):
        view = json.loads(view)
    ins, outs = (view or {}).get("inputs") or {}, (view or {}).get("outputs") or {}
    missing = []

    def entry(kind, sym):
        if kind == "in":
            return ins.get(sym)
        if kind == "out":
            return outs.get(sym)
        return outs.get(sym) if sym in outs else ins.get(sym)

    def fill(m):
        kind, sym = m.group(1), m.group(2)
        e = entry("val" if kind == "unit" else kind, sym)
        if not isinstance(e, dict):
            missing.append(sym)
            return ""
        u = _unit(e.get("unit"))
        if kind == "unit":
            if not u:
                missing.append(sym)
            return u
        s = show(e.get("value"))
        if s is None:
            missing.append(sym)
            return ""
        return f"{s} {u}" if u else s

    out = PLACEHOLDER.sub(fill, text)
    return None if missing else out


def placeholders(text: str) -> list:
    """[(kind, symbol)] in the order written."""
    return [(m.group(1), m.group(2)) for m in PLACEHOLDER.finditer(text or "")]


# --- from an Answer of this runtime --------------------------------------------------------------------------------

def view_of(answer) -> dict | None:
    """The view of a solved Answer (relation or procedure), from its first step; None when it has none."""
    if not getattr(answer, "solved", False) or not answer.steps:
        return None
    st = answer.steps[0]
    ins = {k: {"value": v.get("value"), "unit": v.get("unit") or ""} for k, v in (st.get("inputs") or {}).items()}
    if "outputs" in st:                               # an algorithm: every output with its unit
        units = st.get("output_units") or {}
        outs = {k: {"value": v, "unit": units.get(k, "")} for k, v in st["outputs"].items()}
    else:                                             # a formula: the solved symbol
        outs = {st["target"]: {"value": st["result"], "unit": st.get("unit") or ""}}
    return {"inputs": ins, "outputs": outs}


def explain(entry: dict, answer_or_view, level: str = "standard") -> str | None:
    """The text of one level of a capability's explanation entry, filled from an Answer or a view. Falls back to
    shorter levels when the asked one cannot be filled (a value the answer does not carry)."""
    view = answer_or_view if isinstance(answer_or_view, (dict, str)) else view_of(answer_or_view)
    if view is None or not entry:
        return None
    order = {"deep": ("deep", "standard", "short"), "standard": ("standard", "short"), "short": ("short",),
             "reading": ("reading",)}.get(level, (level,))
    for lv in order:
        t = entry.get(lv)
        if t:
            s = render(t, view)
            if s is not None:
                return s
    return None
