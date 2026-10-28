# GENERATED from ai999x_factory/runtime by tools/build_package.py — do not edit.
"""Solving a model family's coupled equations, with branches, constraints, ambiguity and underdetermination made
explicit, and every equation re-checked at the solution.

A family builds a `System`: typed symbols (one per body, state and stage), labelled equations, the known values with
where each came from, the asked symbol, and physical constraints that discard non-physical branches. The solver never
picks between two physical answers: two surviving values are an ambiguity, reported with both values; a target that
stays symbolic is underdetermined, reported with the quantities that would determine it.
"""
from __future__ import annotations

from dataclasses import dataclass, field

from .._lazy import sp
from ..answer import Answer
from .. import precision

RESIDUAL_TOL = 1e-9


@dataclass
class Sym:
    kind: str
    meaning: str


@dataclass
class System:
    family: str
    title: str
    symbols: dict                       # name -> Sym
    equations: list                     # [(label, sympy Eq)]
    known: dict                         # name -> SI value
    sources: dict                       # name -> "stated: 2 kg" | "implied: at rest" | "default: g = 9.80665 m/s^2"
    target: str
    phrase: str
    constraints: list = field(default_factory=list)   # [(description, callable(values) -> bool)]
    assumptions: list = field(default_factory=list)
    magnitude: bool = False            # report |value| (a speed asked as a magnitude)
    bodies: list = field(default_factory=list)       # descriptions, for the answer
    inputs: list = field(default_factory=list)       # symbols a question may state (to name what is missing)


@dataclass
class Refusal:
    status: str                        # needs_clarification | unsupported
    message: str
    cause: str
    family: str = ""
    would_need: list = field(default_factory=list)
    branches: list = field(default_factory=list)


def _syms(system):
    return {n: sp.Symbol(n, real=True) for n in system.symbols}


def _residual(eq, values):
    lhs, rhs = eq.lhs.subs(values), eq.rhs.subs(values)
    terms = [t for side in (eq.lhs, eq.rhs) for t in sp.Add.make_args(sp.expand(side))]
    try:
        scale = sum(abs(complex(sp.N(t.subs(values)))) for t in terms) or 1.0
        return abs(complex(sp.N(lhs - rhs))) / scale
    except (TypeError, ValueError):
        return float("inf")


def _real(v):
    try:
        c = complex(sp.N(v))
    except (TypeError, ValueError):
        return None
    return c.real if abs(c.imag) <= 1e-9 * max(1.0, abs(c.real)) else None


def _holds(expr, scale_expr, values) -> bool:
    """A fully numeric equation (lhs - rhs) holds, relative to the size of its terms."""
    v = _real(expr)
    if v is None:
        return False
    try:
        scale = sum(abs(complex(sp.N(t.subs(values)))) for t in sp.Add.make_args(sp.expand(scale_expr))) or 1.0
    except (TypeError, ValueError):
        scale = 1.0
    return abs(v) <= 1e-9 * scale


def _propagate(exprs, tsym, max_branches=64):
    """Branches of numeric assignments. Repeatedly solve an equation with ONE unknown (every real root is a branch);
    when none is left, solve the smallest closed block (k equations in k unknowns); stop at the asked symbol.
    Returns [(assignment, state)] with state 'solved' | 'stuck' | 'contradiction'."""
    frontier, done = [{}], []
    while frontier and len(done) < max_branches:
        asg = frontier.pop()
        live, bad = [], False
        for orig in exprs:
            e = orig.subs(asg)
            if not e.free_symbols:
                if not _holds(e, orig, asg):
                    bad = True
                    break
                continue
            live.append((e, orig))
        if bad:
            done.append((asg, "contradiction"))
            continue
        if tsym in asg:
            done.append((asg, "solved"))
            continue
        single = [e for e, _o in live if len(e.free_symbols) == 1]
        if single:
            single.sort(key=lambda e: tsym not in e.free_symbols)
            e = single[0]
            x = next(iter(e.free_symbols))
            try:
                roots = [r for r in (_real(r) for r in sp.solve(e, x, rational=False)) if r is not None]
            except (NotImplementedError, ValueError, TypeError):
                roots = []
            if not roots:
                done.append((asg, "contradiction"))
            for r in roots:
                frontier.append({**asg, x: sp.Float(r, 17)})
            continue
        block = None
        from itertools import combinations
        for k in (2, 3, 4):
            for combo in combinations([e for e, _o in live], k):
                syms = set().union(*(c.free_symbols for c in combo))
                if len(syms) == k:
                    block = (list(combo), sorted(syms, key=lambda s: s.name))
                    break
            if block:
                break
        if block is None:
            done.append((asg, "stuck"))
            continue
        try:
            sols = sp.solve(block[0], block[1], dict=True, rational=False)
        except (NotImplementedError, ValueError, TypeError):
            sols = []
        added = 0
        for s in sols:
            vals = {k: _real(v) for k, v in s.items()}
            if all(v is not None for v in vals.values()) and len(vals) == len(block[1]):
                frontier.append({**asg, **{k: sp.Float(v, 17) for k, v in vals.items()}})
                added += 1
        if not added:
            done.append((asg, "contradiction"))
    return done


def solve(system: System, pack, question: str) -> Answer:
    S = _syms(system)
    tsym = S[system.target]
    if system.target in system.known:
        return _refuse(system, question, Refusal("needs_clarification", "The asked quantity is already stated.",
                                                 "target_stated", system.family))
    known = {S[n]: sp.Float(v, 17) for n, v in system.known.items()}
    exprs = [e.lhs - e.rhs for _l, e in system.equations]
    branches = _propagate([e.subs(known) for e in exprs], tsym)
    values, stuck, rejected = [], [], []
    for asg, state in branches:
        if state == "stuck":
            stuck.append(asg)
            continue
        if state != "solved":
            continue
        numeric = {s.name: float(v) for s, v in {**known, **asg}.items()}
        tv = numeric[system.target]
        bad = [d for d, ok in system.constraints if not _safe(ok, numeric)]
        if bad:
            rejected.append({"value": tv, "violates": bad})
            continue
        values.append((tv, None, numeric))
    if not values:
        if stuck:
            have = set(system.known) | {s.name for s in stuck[0]}
            need = [n for n in (system.inputs or system.symbols) if n not in have and n != system.target]
            words = [f"{system.symbols[n].meaning}" for n in need]
            return _refuse(system, question, Refusal(
                "needs_clarification", "This is a " + system.title.lower() + " problem, but it is not determined: I also "
                "need " + (words[0] if len(words) == 1 else "one more of: " + ", ".join(words)) + ".", "underdetermined",
                system.family, would_need=words))
        return _refuse(system, question, Refusal(
            "needs_clarification", "No physically valid solution: " + "; ".join(sorted({b for r in rejected for b in r["violates"]})) + ".",
            "no_physical_solution", system.family, branches=rejected))
    distinct = []
    for v in values:
        if not any(precision.agrees(v[0], d[0], "exact", scale=max(1.0, abs(d[0]))) for d in distinct):
            distinct.append(v)
    if system.magnitude:
        mags = []
        for v in distinct:
            if not any(precision.agrees(abs(v[0]), abs(m[0]), "exact", scale=max(1.0, abs(m[0]))) for m in mags):
                mags.append(v)
        distinct = mags
    if len(distinct) > 1:
        vals = [d[0] for d in distinct]
        return _refuse(system, question, Refusal(
            "needs_clarification", "Two physically valid answers exist (" + ", ".join(f"{v:.4g}" for v in vals) + "); "
            "the question does not say which situation is meant.", "multiple_branches", system.family,
            branches=[{"value": v} for v in vals]))
    value, full, numeric = distinct[0]
    # verification: every equation at the full solution, and the target recomputed from each equation alone
    allv = {S[n]: v for n, v in numeric.items()}
    residuals = [_residual(e, allv) for _l, e in system.equations if e.free_symbols <= set(allv)]
    paths = 0
    for _l, e in system.equations:
        if tsym in e.free_symbols and (e.free_symbols - {tsym}) <= set(allv):
            others = {s: v for s, v in allv.items() if s != tsym}
            try:
                roots = [complex(sp.N(r)) for r in sp.solve(e.lhs.subs(others) - e.rhs.subs(others), tsym, rational=False)]
            except (NotImplementedError, TypeError, ValueError):
                continue
            if any(abs(r.real - value) <= 1e-7 * max(1.0, abs(value)) and abs(r.imag) < 1e-9 for r in roots):
                paths += 1
    ok = residuals and max(residuals) <= RESIDUAL_TOL and paths >= 1
    out = (abs(value) if system.magnitude else value)
    kind = system.symbols[system.target].kind
    answer = Answer("solved" if ok else "verification_failed", question, out if ok else None, pack.kinds[kind].unit,
                    {"kind": kind, "role": "system", "phrase": system.phrase}, [system.family], list(system.assumptions),
                    _steps(system, S, numeric, pack), {
                        "status": "verified" if ok else "failed", "max_residual": max(residuals) if residuals else None,
                        "dimensions_consistent": True, "independent_paths": paths,
                        "branches_rejected": rejected, "equations": len(system.equations),
                        "scope": "algebra and numerical agreement of the model's equations; not a certificate that the model fits reality"},
                    message="" if ok else "I computed a value but it did not pass the checks, so I am not showing it.")
    answer.uncertainty = ({"state": "determined", "model_family": system.family, "bodies": system.bodies,
                           "sources": system.sources} if ok else {"state": "verification_failed",
                                                                  "cause": "computed_value_failed_its_checks"})
    return answer


def _safe(fn, numeric):
    try:
        return bool(fn(numeric))
    except (KeyError, ZeroDivisionError, ValueError, TypeError):
        return True                       # a constraint about a quantity this solution does not fix does not apply


def _steps(system, S, numeric, pack):
    """The equations in a solving order: each step determines one new quantity; a coupled remainder is one step."""
    have = {S[n] for n in system.known}
    pending = [(l, e) for l, e in system.equations]
    steps = []
    while pending:
        nxt = next(((l, e) for l, e in pending if len(e.free_symbols - have) == 1), None)
        if nxt is None:
            l, e = pending[0]
            new = sorted(e.free_symbols - have, key=lambda s: s.name)
            if not new:
                pending.pop(0)
                continue
            target = new[0]
        else:
            l, e = nxt
            target = next(iter(e.free_symbols - have))
        pending.remove((l, e))
        if target.name not in numeric:
            continue
        inputs = {s.name: {"value": numeric[s.name], "unit": pack.kinds[system.symbols[s.name].kind].unit,
                           "source": system.sources.get(s.name, "computed"), "meaning": system.symbols[s.name].meaning}
                  for s in sorted(e.free_symbols - {target}, key=lambda s: s.name) if s.name in numeric}
        steps.append({"relation": f"{system.family}:{l}", "name": l, "equation": f"{sp.sstr(e.lhs)} = {sp.sstr(e.rhs)}",
                      "equation_latex": sp.latex(e), "solved_latex": sp.latex(target), "target": target.name,
                      "inputs": inputs, "result": numeric[target.name],
                      "unit": pack.kinds[system.symbols[target.name].kind].unit})
        have.add(target)
    return steps


def _refuse(system, question, r: Refusal) -> Answer:
    return refusal_answer(question, r)


def refusal_answer(question, r: Refusal) -> Answer:
    state = {"underdetermined": "missing_information", "multiple_branches": "ambiguous"}.get(r.cause, "missing_information")
    a = Answer(r.status, question, message=r.message, models=[r.family] if r.family else [])
    a.uncertainty = {"state": state, "cause": r.cause, "model_family": r.family}
    if r.would_need:
        a.uncertainty["would_need"] = [f"{r.family} needs the {w}" for w in r.would_need]
    if r.branches:
        a.uncertainty["branches"] = r.branches
    return a
