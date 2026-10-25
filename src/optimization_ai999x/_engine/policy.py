# GENERATED from ai999x_factory/runtime by tools/build_package.py — do not edit.
"""Decisions that are judgement, not mechanics. Kept in one small file so they are easy to find and change.

`unused_input_policy` decides what happens when the best plan does not use every quantity the question stated.
It never decides by the kind of quantity alone ("time is harmless", "angles are dangerous"): it asks whether
the evidence at hand makes irrelevance defensible, and refuses whenever it does not.

It is not a parser and not a detector of missing relations. A quantity can be unused because the engine lacks
the relation that needs it; the checks below remove the cases where that is likely, and what remains is
answered WITH a warning that names the quantity and says why it was set aside — never silently.
"""
from __future__ import annotations

REFUSE, WARN = "refuse", "warn"


class Decision(str):
    """REFUSE or WARN, carrying the reasons. Compares equal to the plain strings."""

    def __new__(cls, value: str, reasons=()):
        obj = super().__new__(cls, value)
        obj.reasons = tuple(reasons)
        return obj


def unused_input_policy(unused: list, target, plans: list, *, context: dict | None = None) -> Decision:
    """`unused`  the stated quantities (compiler.Given) that the best plans did not consume
    `target`  what was asked (compiler.Target)
    `plans`   the best-covering plans (planner.Plan), all leaving `unused` out
    `context` {"pack", "tags", "all_plans"}: the domain pack (its `relevance` vocabulary), the context tags of
              the question, and every plan found, including those that cover less

    WARN only when every unused quantity passes every test below; otherwise REFUSE with the first reason.

      1. No evidence, no judgement: without the pack's relevance vocabulary nothing can be established.
      2. The plans that remain must agree on the value (otherwise the question is ambiguous anyway).
      3. A quantity stated with a specific ROLE (initial, final, height, radius, gravity ...) describes the
         state, geometry or environment of the problem. Setting it aside changes the problem.
      4. A quantity of a CONDITIONING kind (angle, medium, material constant ...) decides which model applies.
      5. A quantity of a kind the answer already depends on is a competing value: which one applies?
      6. If any other supported reading of the question uses the quantity, it is not irrelevant.
      7. If the wording describes a change (a process tag) and the quantity parameterises that change, the
         asked quantity may refer to the state after it.
    """
    def refuse(reason: str) -> Decision:
        return Decision(REFUSE, (reason,))

    pack = (context or {}).get("pack")
    relevance = getattr(pack, "terms", {}).get("relevance") if pack is not None else None
    if not relevance:
        return refuse("the relevance of a stated quantity cannot be established for this domain")
    if not plans or not unused:
        return refuse("there is no plan to judge the unused quantity against")
    lo, hi = min(p.value for p in plans), max(p.value for p in plans)
    if hi - lo > 1e-9 * max(1.0, abs(lo), abs(hi)):
        return refuse("the readings that remain give different values")

    relations = {s.relation.key: s.relation for p in plans for s in p.steps}
    plan_kinds = {sym.kind for r in relations.values() for sym in r.symbols.values()}
    process = sorted(set((context or {}).get("tags", ())) & set(relevance.get("process_tags", ())))
    all_plans = (context or {}).get("all_plans", plans)

    for u in unused:
        what = f"{u.text} ({u.kind.replace('_', ' ')})"
        if u.role != "plain":
            return refuse(f"{what} is stated as the {u.role.replace('_', ' ')}: it describes the state, geometry or "
                          "environment of the problem, so a model that ignores it may be the wrong model")
        if u.kind in relevance.get("conditioning_kinds", ()):
            return refuse(f"{what} sets a condition of the problem (geometry, medium or material), so a model that "
                          "ignores it may not apply")
        if u.kind in plan_kinds:
            return refuse(f"{what} is a second quantity of a kind the answer depends on, and nothing says which "
                          "one applies")
        if any(u.id in p.used for p in all_plans):
            return refuse(f"another supported reading of the question uses {what}")
        if process and u.kind in relevance.get("process_parameters", ()):
            return refuse(f"the question describes a change ({', '.join(process)}) and {what} is a parameter of it; "
                          "the quantity asked for may refer to the state after that change")

    plan = min(plans, key=lambda p: len(p.steps))
    names = " and ".join(f"'{s.relation.name}'" for s in plan.steps)
    depends = sorted({(s.relation.symbols[n].desc or s.relation.symbols[n].kind)
                      for s in plan.steps for n, b in s.bindings if b.source != "derived"})
    return Decision(WARN, tuple(
        f"{u.text} ({u.kind.replace('_', ' ')}) was not used: under {names}, the {target.phrase} depends only on "
        f"{', '.join(depends)}, and no supported relation connects a {u.kind.replace('_', ' ')} to it from what is "
        "stated. If it matters to your problem, the model above is not the right one." for u in unused))
