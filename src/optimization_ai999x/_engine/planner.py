# GENERATED from ai999x_factory/runtime by tools/build_package.py — do not edit.
"""Dependency planning over typed relations.

Given what is known (kind + role + value) and what is asked (kind + role), find every chain of relations that
produces the asked quantity. Each plan is one consistent reading of the problem: a slot (kind, role) holds one
value throughout the plan, and a stated quantity plays one role throughout. A symbol is filled, in order of
preference, by a slot already known, a stated quantity of exactly its role, a stated quantity of a role it
accepts, a value derived by another relation, or a documented default.

Plans are judged together by the caller: the answer must use every stated quantity, and all plans that do
must agree.
"""
from __future__ import annotations

from dataclasses import dataclass, field

from .pack import Pack
from .quantity import parse_unit, to_si
from .relation import Relation, RelationError

MAX_DEPTH = 2          # sub-derivations below the answering relation (chains of up to three relations deep)
MAX_PLANS = 300
_PENDING = object()    # a slot being derived further up the chain: it may not be consumed below


@dataclass(frozen=True)
class Bound:
    value: float
    exact: object
    source: str                      # given | implied | default | derived
    uses: frozenset = frozenset()    # ids of the stated/implied quantities this value rests on
    note: str = ""


@dataclass(frozen=True)
class Step:
    relation: Relation
    target: str
    bindings: tuple                  # ((symbol name, Bound), ...)
    value: float

    def values(self) -> dict:
        return {n: (b.exact if b.exact is not None else b.value) for n, b in self.bindings}


@dataclass(frozen=True)
class Plan:
    steps: tuple                     # dependency order; the last step yields the answer
    used: frozenset
    assumptions: tuple
    value: float

    @property
    def signature(self) -> tuple:
        return tuple(sorted((s.relation.key, s.target) for s in self.steps))


@dataclass
class Outcome:
    plans: list = field(default_factory=list)
    reasons: list = field(default_factory=list)      # why the answering relations could not be applied


def _default(sym, tags) -> Bound | None:
    d = sym.default
    if not d or set(d.get("unless", ())) & set(tags):
        return None
    unit = parse_unit(d["unit"])
    return Bound(to_si(d["value"], unit), None, "default", frozenset(), d.get("assumption", f"default {sym.name}"))


def find_plans(pack: Pack, kind: str, role: str, givens: list, tags: tuple) -> Outcome:
    out = Outcome()
    count = [0]
    # quantities that describe the environment, the material or the geometry of a problem: they may be stated,
    # defaulted with a declared assumption, or asked for — never inferred on the way to something else
    never = [tuple(x) for x in (pack.terms.get("relevance") or {}).get("never_derive", ())]

    def protected(kind_, role_) -> bool:
        return any(k == kind_ and r in ("*", role_) for k, r in never)

    # a quantity that may NOT stand in for a symbol of another role in some context: thrown upward, gravity is
    # not the acceleration along the motion (it opposes it), so g may fill only a symbol that is itself gravity
    no_stand_in = {(x["kind"], x["role"]) for x in (pack.terms.get("relevance") or {}).get("no_stand_in", ())
                   if x["when_tag"] in tags}

    def produce(want_kind, role_ok, depth, banned, slots, assigned):
        """Yield (steps, assumptions, bound, slots, assigned) for every way to derive a new slot of `want_kind`
        whose producing symbol satisfies `role_ok`."""
        for rel in pack.relations:
            if rel.key in banned or set(rel.requires) - set(tags) or set(rel.excludes) & set(tags):
                continue
            if rel.shortcut and depth < MAX_DEPTH:
                continue                               # a derived shortcut answers; the laws it came from do the chaining
            for tname, tsym in rel.symbols.items():
                slot = (tsym.kind, tsym.role)
                if tsym.kind != want_kind or not role_ok(tsym) or slot in slots:
                    continue
                if depth < MAX_DEPTH and protected(tsym.kind, tsym.role):
                    continue
                if any(g.kind == tsym.kind and g.role == tsym.role for g in givens):
                    continue                           # that quantity is stated; it is not derived
                others = [s for n, s in rel.symbols.items() if n != tname]
                for bound, steps, assume, slots2, assigned2 in fill(
                        # a shortcut works from what is stated (or defaulted) only: it never sits on a derived value
                        rel, others, 0, {}, (), (), 0 if rel.shortcut else depth, banned | {rel.key},
                        {**slots, slot: _PENDING}, assigned,
                        frozenset({slot})):
                    try:
                        value = rel.solve(tname, {n: (b.exact if b.exact is not None else b.value)
                                                  for n, b in bound.items()})
                    except RelationError as exc:
                        if depth == MAX_DEPTH:
                            out.reasons.append(str(exc))
                        continue
                    uses = frozenset().union(*(b.uses for b in bound.values())) if bound else frozenset()
                    b = Bound(value, None, "derived", uses)
                    step = Step(rel, tname, tuple(bound.items()), value)
                    yield steps + (step,), assume, b, {**slots2, slot: b}, assigned2

    def fill(rel, others, i, bound, steps, assume, depth, banned, slots, assigned, taken=frozenset()):
        """`taken` holds the slots already consumed by another symbol of THIS relation: two symbols of the same
        kind and role (the two works in W1 + W2 = ...) are different quantities and may never share a value."""
        count[0] += 1
        if count[0] > 60000:
            return
        if i == len(others):
            yield bound, steps, assume, slots, assigned
            return
        sym = others[i]
        own = (sym.kind, sym.role)
        twin = own in taken                            # a sibling symbol already holds this slot
        options = []                                   # (Bound, extra steps, extra assumptions, slots, assigned, slot used)
        known = slots.get(own)
        if known is _PENDING:
            return
        if known is not None and not twin:
            options.append((known, (), (), slots, assigned, own))
        else:
            for r in sym.accepts:                      # a slot of an accepted role that the plan already holds
                other = slots.get((sym.kind, r))
                if (sym.kind, r) in no_stand_in:
                    continue
                if other is not None and other is not _PENDING and (sym.kind, r) not in taken:
                    options.append((other, (), (), slots, assigned, (sym.kind, r)))
            free = [g for g in givens if g.kind == sym.kind and g.id not in assigned]
            exact = [g for g in free if g.role == sym.role]
            loose = [] if exact else [g for g in free if sym.takes(g.role) and (g.kind, g.role) not in no_stand_in]
            for g in exact or loose:
                b = Bound(g.value, g.exact, "implied" if g.source == "implied" else "given", frozenset({g.id}),
                          g.assumption)
                options.append((b, (), (g.assumption,) if g.assumption else (), slots if twin else {**slots, own: b},
                                {**assigned, g.id: sym.role}, own))
            if not exact and not twin:
                if depth > 0:
                    for sub_steps, sub_assume, b, slots2, assigned2 in produce(
                            sym.kind, lambda p, c=sym: c.takes(p.role), depth - 1, banned, slots, assigned):
                        options.append((b, sub_steps, sub_assume, slots2, assigned2, own))
                d = _default(sym, tags)
                if d is not None and not options:
                    options.append((d, (), (d.note,), {**slots, own: d}, assigned, own))
        if not options and depth == MAX_DEPTH:
            out.reasons.append(f"{rel.name} needs the {sym.desc or sym.name}")
        for b, sub_steps, sub_assume, slots2, assigned2, used_slot in options:
            if any(b is held for held in bound.values()):
                continue        # one value standing in for two symbols of one relation (v = v0 = d/t gave a = 0): never
            yield from fill(rel, others, i + 1, {**bound, sym.name: b}, steps + sub_steps, assume + sub_assume,
                            depth, banned, slots2, assigned2, taken | {used_slot})

    seen = set()
    for steps, assume, b, _slots, _assigned in produce(kind, lambda s: s.takes(role), MAX_DEPTH, frozenset(), {}, {}):
        unique = tuple(dict.fromkeys(steps))           # a slot reused by two symbols contributes its steps once
        plan = Plan(unique, b.uses, tuple(dict.fromkeys(assume)), b.value)
        key = (plan.signature, tuple(sorted(plan.used)), round(plan.value, 12))
        if key not in seen:
            seen.add(key)
            out.plans.append(plan)
            if len(out.plans) >= MAX_PLANS:
                break
    return out
