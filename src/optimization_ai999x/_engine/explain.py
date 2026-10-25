# GENERATED from ai999x_factory/runtime by tools/build_package.py — do not edit.
"""Conceptual questions answered from the model's own knowledge: "What is the Nernst equation?", "Explain power
iteration", "When does Bernoulli's equation fail?", "What are the assumptions of a t-test?".

Offline and in milliseconds: an in-memory index over the certified capabilities (name, keywords, kind) built once
per product, plus the linked theory of each capability from the shipped knowledge store (explanation records,
tools/capability_theory.py). A capability without written theory is still explained from its certified data (name,
formula or method, assumptions, conditions).

What it never does: compute a number, or answer a question that states measured values (that is the engine's job),
or pick between capabilities that match equally well (it lists them instead). Prose from the theory is tier
`structurally_valid` (well formed, meaning not independently verified); the answer says so.
"""
from __future__ import annotations

import re

_CUE = re.compile(r"^\s*(?:what\s+(?:is|are|does|do)|what's|whats|explain|define|describe|tell\s+me\s+about|"
                  r"why\s+|when\s+(?:does|do|is|should|can)|how\s+does|how\s+do|meaning\s+of|definition\s+of|"
                  r"list\s+the|give\s+the\s+(?:assumptions|limitations)|what\s+assumptions|which\s+assumptions)\b", re.I)
FACETS = (("assumptions", re.compile(r"\bassum|\bcondition|\brequire", re.I)),
          ("when_it_fails", re.compile(r"\bfail|\blimitation|\bnot\s+(?:valid|apply|work)|\bbreak[s]?\s+down|\binvalid", re.I)),
          ("common_mistakes", re.compile(r"\bmistake|\berror[s]?\b|\bpitfall|\bwrong", re.I)))
_STOP = set("a an the of in on for to and or is are was be what whats explain define describe tell me about why when "
            "does do how meaning definition list give which it its this that with by as at from used use using "
            "formula equation law method rule theorem calculate compute find common mistake mistakes error errors "
            "pitfall pitfalls assumption assumptions fail fails failure limitation limitations wrong apply applies "
            "valid condition conditions require requires work works not break breaks down".split())


def _stem(w: str) -> str:
    """Plural and possessive folded the same way on both sides: ohms, ohm's -> ohm; laws -> law."""
    return w[:-1] if len(w) > 3 and w.endswith("s") and not w.endswith("ss") else w


def _words(text: str) -> list:
    t = (text or "").lower().replace("_", " ").replace("'s", "s").replace("’s", "s")
    return [_stem(w) for w in re.findall(r"[a-z0-9]+", t) if w not in _STOP and len(w) > 1]


def is_conceptual(question: str) -> bool:
    """A question about an idea, not a computation: a conceptual opening and no digit at all (a value, a formula
    such as CuSO4 or a matrix means the engine computes)."""
    return isinstance(question, str) and bool(_CUE.search(question)) and not re.search(r"\d", question)


class ConceptIndex:
    def __init__(self, pack, store=None):
        self.entries = {}
        theory = {}
        if store is not None:
            try:
                for r in store.find(kind="explanation", limit=100000):
                    caps = r.get("capabilities") or []
                    if caps:
                        theory[caps[0]] = r.get("payload") or {}
            except Exception:                          # an older store without explanations: certified data only
                theory = {}
        for rel in pack.relations:
            self.entries[rel.key] = {"key": rel.key, "name": rel.name, "kind": "formula", "equation": rel.equation,
                                     "latex": _safe(rel.latex), "assumptions": list(rel.assumptions),
                                     "when_it_fails": list(rel.fails_when), "applies_when": list(rel.applies_when),
                                     "theory": theory.get(rel.key, {})}
        for k, p in pack.procedures.items():
            self.entries[k] = {"key": k, "name": p["name"], "kind": "method", "equation": p.get("method", ""),
                               "latex": p.get("latex", ""), "assumptions": list(p.get("conditions", [])),
                               "when_it_fails": [], "applies_when": [p.get("purpose", "")] if p.get("purpose") else [],
                               "theory": theory.get(k, {})}
        self.terms = {}
        for k, e in self.entries.items():
            name = _words(e["name"]) + _words(k)
            kw = [w for x in e["theory"].get("keywords", []) for w in _words(x)]
            self.terms[k] = {"name": set(name), "kw": set(kw), "key": set(_words(k))}

    def match(self, question: str) -> list:
        """[(score, key)] best first, only capabilities that cover EVERY content word of the question (a partial
        match is not an answer: "area of a circle" is not explained by a cone's lateral area). The name counts
        double, theory keywords once; a key that is exactly the asked words ("ohm") wins."""
        q = set(_words(question))
        if not q:
            return []
        out = []
        for k, t in self.terms.items():
            if not q <= (t["name"] | t["kw"]):
                continue
            s = 2 * len(q & t["name"]) + len(q & t["kw"]) + (5 if t["key"] == q else 0)
            out.append((s, k))
        return sorted(out, key=lambda x: (-x[0], x[1]))


def _safe(f):
    try:
        return f()
    except Exception:
        return ""


def explain(question: str, index: ConceptIndex, domain: str):
    """(text, key, facet, candidates) or None when nothing in this model matches."""
    ranked = index.match(question)
    if not ranked:
        return None
    best = ranked[0][0]
    top = [k for s, k in ranked if s == best]
    variants = []
    if len(top) > 1:                                   # the closest name wins: fewest name words the question lacks
        q = set(_words(question))
        extra = {k: len(index.terms[k]["name"] - q) for k in top}
        least = min(extra.values())
        top = [k for k in top if extra[k] == least]
    if len(top) > 1:                                   # one law solved for different unknowns ("Ideal gas law: P")
        groups = {}
        for k in top:
            groups.setdefault(re.split(r"[:(]", index.entries[k]["name"])[0].strip().lower(), []).append(k)
        if len(groups) == 1:
            variants = [index.entries[k]["name"] for k in top[1:]]
            top = top[:1]
        elif len(groups) <= 3:                         # a few different ideas fit equally: a short overview of each
            parts = []
            for g in groups.values():
                e = index.entries[g[0]]
                summ = (e["theory"].get("summary") or "").split(". ")[0]
                parts.append(f"**{e['name']}**" + (f": $${e['latex']}$$" if e.get("latex") else "")
                             + (f" {summ.rstrip('.')}." if summ else "")
                             + (f" (also: {'; '.join(index.entries[k]['name'] for k in g[1:])})" if len(g) > 1 else ""))
            text = "This can mean several things in this model:\n\n" + "\n\n".join(parts) + \
                   "\n\nAsk about one of them by name for its assumptions, limits and common mistakes."
            return text, groups[next(iter(groups))][0], "overview_of_several", []
    if len(top) > 1:
        return None, None, None, [index.entries[k]["name"] for k in top[:5]]
    key = top[0]
    e = index.entries[key]
    t = e["theory"]
    facet = next((f for f, rx in FACETS if rx.search(question)), None)
    lines = [f"**{e['name']}**"]
    if e.get("latex"):
        lines.append(f"$${e['latex']}$$")
    if facet:
        items = t.get(facet) or e.get(facet) or []
        title = {"assumptions": "Assumptions", "when_it_fails": "When it does not apply",
                 "common_mistakes": "Common mistakes"}[facet]
        lines.append(f"{title}: " + ("; ".join(items) if items else "none recorded for this capability") + ".")
    else:
        if t.get("summary"):
            lines.append(t["summary"])
        if t.get("statement"):
            lines.append(t["statement"])
        elif e.get("applies_when"):
            lines.append("Applies to: " + "; ".join(x for x in e["applies_when"] if x) + ".")
        if e.get("assumptions") or t.get("assumptions"):
            lines.append("Assumptions: " + "; ".join(t.get("assumptions") or e["assumptions"]) + ".")
        if t.get("when_it_fails") or e.get("when_it_fails"):
            lines.append("Does not apply when: " + "; ".join(t.get("when_it_fails") or e["when_it_fails"]) + ".")
    if variants:
        lines.append("Also available, the same law solved for other quantities: " + "; ".join(variants) + ".")
    lines.append(f"To compute it: `solve(\"{key}\", ...)` or ask with the values stated.")
    return "\n\n".join(lines), key, facet, []
