# GENERATED from ai999x_factory/runtime by tools/build_package.py — do not edit.
"""ask(): compile the question, plan, compute, verify — or say precisely why not."""
from __future__ import annotations

import re

from . import policy
from ._lazy import mp, sp
from .answer import Answer
from .compiler import Given, Problem, Target, compile_problem
from .pack import Pack
from .planner import Plan, Step, find_plans
from .quantity import UnitError, parse_unit, to_si

RESIDUAL_TOL = 1e-9
AGREE_TOL = 1e-9


def _same(a: float, b: float) -> bool:
    return abs(a - b) <= AGREE_TOL * max(1.0, abs(a), abs(b))


def _root_check(step: Step):
    """Re-find the step's value with an independent numerical root search on lhs - rhs = 0.
    True / False, or None when the search did not converge (reported, not counted as agreement)."""
    rel = step.relation
    l, r = rel.sides
    x = rel.sp_symbols[step.target]
    expr = (l - r).subs({rel.sp_symbols[n]: v for n, v in step.values().items()})
    # scale by the size of the equation's terms at the solution, so the search's tolerance is relative: an equation
    # whose terms are 1e-27 (a photon's momentum) would otherwise look solved at any starting point
    try:
        scale = sum(abs(complex(sp.N(t.subs(x, step.value), 17))) for t in sp.Add.make_args(sp.expand(expr))) or 1.0
    except (TypeError, ValueError, ZeroDivisionError):
        scale = 1.0
    f = sp.lambdify(x, expr / scale, "mpmath")
    start = step.value * 1.03 + (1e-3 if step.value == 0 else 0.0)
    try:
        root = complex(mp.findroot(f, start, tol=1e-18, maxsteps=60))
    except (ValueError, ZeroDivisionError, TypeError):
        return None
    if abs(root.imag) > 1e-9:
        return None
    return abs(root.real - step.value) <= 1e-6 * max(1.0, abs(step.value))


def _verify(pack: Pack, plan: Plan) -> dict:
    residuals, roots, dims = [], [], []
    for s in plan.steps:
        residuals.append(s.relation.residual({**s.values(), s.target: s.value}))
        roots.append(_root_check(s))
        dims.append(pack.dimensions_consistent(s.relation))
    ok = all(dims) and max(residuals) <= RESIDUAL_TOL and False not in roots
    return {"status": "verified" if ok else "failed", "max_residual": max(residuals),
            "dimensions_consistent": all(dims),
            "independent_root_search": "agrees" if roots and all(r is True for r in roots) else (
                "disagrees" if False in roots else "did not converge"),
            "scope": "algebra, dimensions and numerical agreement; not a certificate that the model fits reality"}


def _describe(pack: Pack, plan: Plan) -> list:
    out = []
    for s in plan.steps:
        rel = s.relation
        inputs = {}
        for name, b in s.bindings:
            inputs[name] = {"value": b.value, "unit": pack.kinds[rel.symbols[name].kind].unit, "source": b.source,
                            "meaning": rel.symbols[name].desc or rel.symbols[name].kind}
        out.append({"relation": rel.key, "name": rel.name, "equation": rel.equation, "equation_latex": rel.latex(),
                    "solved_latex": rel.latex_solved(s.target), "target": s.target, "inputs": inputs,
                    "result": s.value, "unit": pack.kinds[rel.symbols[s.target].kind].unit})
    return out


def solve_problem(problem: Problem, pack: Pack) -> Answer:
    q = problem.question
    if problem.status != "compiled":
        state = {"unsupported_structure": "unsupported", "unsupported_quantity": "unsupported",
                 "ambiguous_quantity": "ambiguous", "invalid_input": "invalid"}.get(problem.code, "missing_information")
        return Answer(problem.status, q, message=problem.message, problem=problem.to_dict(),
                      uncertainty={"state": state, "cause": problem.code or "not_compiled"})
    t = problem.target
    target = {"kind": t.kind, "role": t.role, "phrase": t.phrase}
    found = find_plans(pack, t.kind, t.role, problem.givens, problem.tags)
    if not found.plans:
        why = "; ".join(list(dict.fromkeys(found.reasons))[:3])
        return Answer("needs_clarification", q, target=target, problem=problem.to_dict(),
                      message=f"No supported relation chain gives the {t.phrase} from what is stated."
                              + (f" For example: {why}." if why else ""),
                      uncertainty={"state": "missing_information", "cause": "no_applicable_relation_chain",
                                   "would_need": list(dict.fromkeys(found.reasons))[:6]})
    stated = {g.id for g in problem.givens if g.source == "stated"}
    implied = {g.id for g in problem.givens if g.source == "implied"}
    conditioning = set((pack.terms.get("relevance") or {}).get("conditioning_kinds", ()))
    warnings, unused_note = [], []
    full = [p for p in found.plans if stated <= p.used]
    if full:                                           # of those, the plans that use most of what the wording implies
        top = max(len(p.used & implied) for p in full)
        candidates = [p for p in full if len(p.used & implied) == top]
    else:                                              # evidence is everything stated AND implied
        best = max(len(p.used & (stated | implied)) for p in found.plans)
        candidates = [p for p in found.plans if len(p.used & (stated | implied)) == best]
    left = stated - set().union(*(p.used for p in candidates))
    if not full and not left:                          # every quantity is used by some plan, none uses all
        left = stated - candidates[0].used
    # a CONDITION the wording fixes (an angle, say) may not be ignored any more than a stated number
    left |= {g.id for g in problem.givens if g.id in implied and g.kind in conditioning
             and not all(g.id in p.used for p in candidates)}
    if left:
        unused = [g for g in problem.givens if g.id in left]
        listing = ", ".join(f"{g.text} ({g.kind.replace('_', ' ')})" for g in unused)
        decision = policy.unused_input_policy(unused, t, candidates, context={
            "pack": pack, "tags": problem.tags, "givens": problem.givens, "all_plans": found.plans})
        reasons = list(getattr(decision, "reasons", ()))
        if decision != policy.WARN:
            return Answer("needs_clarification", q, target=target, problem=problem.to_dict(),
                          message=f"I can relate the {t.phrase} to some of what is stated, but not to: {listing}. "
                                  + (f"Reason: {reasons[0]}. " if reasons else "")
                                  + "I will not drop a stated quantity without knowing why it is there.",
                          uncertainty={"state": "unexplained_input", "cause": "stated_quantity_not_used",
                                       "unused": [g.text for g in unused], "reasons": reasons})
        warnings.extend(reasons or [f"Not used in this answer: {listing}."])
        unused_note = [{"text": g.text, "kind": g.kind, "role": g.role} for g in unused]
    groups: list = []
    for p in candidates:
        for g in groups:
            if _same(g[0].value, p.value):
                g.append(p)
                break
        else:
            groups.append([p])
    if len(groups) > 1:
        readings = "; ".join(f"{' then '.join(s.relation.name for s in g[0].steps)} gives "
                             f"{g[0].value:.6g} {pack.kinds[t.kind].unit}" for g in groups[:4])
        return Answer("needs_clarification", q, target=target, problem=problem.to_dict(),
                      message=f"The question can be read in more than one way: {readings}. Say which is meant.",
                      uncertainty={"state": "ambiguous", "cause": "readings_disagree",
                                   "readings": [{"relations": [s.relation.key for s in g[0].steps], "value": g[0].value}
                                                for g in groups[:6]]})
    group = groups[0]
    plan = min(group, key=lambda p: (len(p.steps), len(p.assumptions)))
    check = _verify(pack, plan)
    # two plans are independent only if they rest on different laws: a relation derived from others counts as them
    check["independent_paths"] = len({frozenset(k for s in p.steps for k in (s.relation.derived_from or (s.relation.key,)))
                                      for p in group})
    check["unused_inputs"] = unused_note
    assumptions = list(dict.fromkeys(a for s in plan.steps for a in s.relation.assumptions)) + list(plan.assumptions)
    answer = Answer("solved" if check["status"] == "verified" else "invalid", q, plan.value, pack.kinds[t.kind].unit,
                    target, [s.relation.key for s in plan.steps], assumptions, _describe(pack, plan), check,
                    warnings, "" if check["status"] == "verified" else "The computed value failed verification.",
                    problem.to_dict())
    if not answer.solved:
        answer.value = None
        answer.uncertainty = {"state": "verification_failed", "cause": "computed_value_failed_its_checks"}
    else:
        defaults = [b.note for s in plan.steps for _n, b in s.bindings if b.source == "default"]
        answer.uncertainty = {"state": "determined", "assumed_defaults": list(dict.fromkeys(defaults)),
                              "unused_inputs": [u["text"] for u in unused_note],
                              "conventions": list(dict.fromkeys(c for s in plan.steps for c in s.relation.conventions))}
    return answer


def _from_dict(spec: dict, pack: Pack) -> Problem:
    """A typed problem supplied directly: {"target": {"kind", "role"}, "given": [{"kind", "role", "value",
    "unit"}], "tags": [...]}."""
    q = "typed problem"
    try:
        tk = spec["target"]["kind"]
        if tk not in pack.kinds:
            return Problem("invalid", q, message=f"Unknown kind of quantity: {tk}")
        givens = []
        for g in spec.get("given", []):
            if g["kind"] not in pack.kinds:
                return Problem("invalid", q, message=f"Unknown kind of quantity: {g['kind']}")
            unit = parse_unit(g["unit"])
            if unit.dim != pack.kinds[g["kind"]].dim:
                return Problem("invalid", q, message=f"{g['unit']} is not a unit of {g['kind']}")
            exact = None
            if unit.pi_factor is not None:
                exact = sp.pi * sp.Rational(str(g["value"])) * sp.Rational(unit.pi_factor.numerator,
                                                                           unit.pi_factor.denominator)
            givens.append(Given(len(givens), g["kind"], g.get("role", "plain"),
                                to_si(float(g["value"]), unit, difference=pack.kinds[g["kind"]].difference),
                                f"{g['value']} {g['unit']}", exact=exact))
        role = spec["target"].get("role", "plain")
        return Problem("compiled", q, Target(tk, role, f"{role} {tk}".replace("plain ", "").replace("_", " ")),
                       givens, tuple(spec.get("tags", ())))
    except (KeyError, TypeError, ValueError, UnitError) as exc:
        return Problem("invalid", q, message=f"Malformed typed problem: {exc}")


def _sign_guard(answer: Answer, pack: Pack, question) -> Answer:
    """A quantity defined as positive (its symbol's sign is 'positive' in the relation that gave it) is never answered
    negative: stated values that imply one are inconsistent, and the question is refused with that reason. A question
    that asks for 'the magnitude' gets the magnitude. (classical_physics fold: surface gravity -4.5 m/s^2, an elevator
    'initial speed' of -4 m/s.)"""
    if not answer.solved or answer.value is None or answer.value >= 0 or not answer.target:
        return answer
    kind, role = answer.target.get("kind"), answer.target.get("role")
    rel = pack.by_key.get((answer.models or [None])[-1])
    sign = next((s.sign for s in (rel.symbols.values() if rel else []) if s.kind == kind and (s.role == role or role in (None, "plain"))), None)
    if isinstance(question, str) and re.search(r"\bmagnitude\b|\bhow (?:large|big|strong)\b", question, re.I):
        answer.value = abs(answer.value)
        answer.assumptions = list(answer.assumptions) + ["the magnitude is asked: the sign (direction) is dropped"]
        return answer
    if role == "gravity":                              # g is a magnitude by definition; the sign was the frame's
        answer.value = abs(answer.value)
        answer.assumptions = list(answer.assumptions) + ["gravitational acceleration is reported as a magnitude "
                                                         "(the minus sign came from taking 'up' as positive)"]
        return answer
    phrase = (answer.target.get("phrase") or "").lower()
    named_magnitude = bool(re.search(r"\b(?:speed|gravity|distance|mass|height|length|time|period|radius|wavelength|"
                                     r"frequency|density|magnitude)\b", phrase)) and \
        not re.search(r"\b(?:velocity|displacement|change|net|component|signed)\b", phrase)
    if sign == "positive" or named_magnitude:
        what = (answer.target.get("phrase") or kind or "the asked quantity").replace("_", " ")
        out = Answer("needs_clarification", answer.question, target=answer.target, models=answer.models,
                     message=f"The stated values give a negative {what}, which cannot be negative; the question is "
                             "inconsistent (check the signs or directions of the values), so I will not answer it.")
        out.uncertainty = {"state": "inconsistent", "cause": "negative_magnitude", "outcome": "INVALID_INPUT",
                           "computed": answer.value}
        return out
    return answer


def ask(question, pack: Pack) -> Answer:
    return _sign_guard(_ask_unguarded(question, pack), pack, question)


def _ask_unguarded(question, pack: Pack) -> Answer:
    problem = _from_dict(question, pack) if isinstance(question, dict) else compile_problem(question, pack)
    answer = solve_problem(problem, pack)
    if isinstance(question, dict):
        return answer
    # several bodies, states or stages: the model families (runtime/systems). The single-relation answer stays first;
    # a family answers what it cannot, and two answers that disagree are a refusal, never a choice
    from .systems import solve_text
    system = solve_text(question, pack)
    if system is None:
        return answer
    asked = problem.target.kind if getattr(problem, "target", None) else None
    if system.solved and asked and (system.target or {}).get("kind") != asked:
        return answer                 # the family answered a different quantity than the one the question asks for
    if answer.solved:
        if system.solved and abs(system.value - answer.value) > 1e-6 * max(1.0, abs(answer.value)):
            return Answer("needs_clarification", answer.question, message="Two readings of the question give different "
                          "answers, so I will not choose one.", uncertainty={"state": "ambiguous", "cause": "readings_disagree",
                          "values": [answer.value, system.value], "models": [answer.models, system.models]})
        return answer
    return system
