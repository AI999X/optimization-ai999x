# GENERATED from ai999x_factory/runtime by tools/build_package.py — do not edit.
"""Problems with several bodies, states or stages: a scenario reader and one solver, shared by every domain.
A domain supplies its own computational model families (models/<domain>/capabilities/families.py), which the pack
loads; the core holds none.

    solve_text(question, pack) -> Answer | None     None: no model family of the domain recognises the question
"""
from __future__ import annotations

from .reader import read
from .solver import Refusal, Sym, System, refusal_answer, solve

__all__ = ["solve_text", "read", "solve", "System", "Sym", "Refusal"]


def solve_text(question, pack):
    families = getattr(pack, "families", None) or []
    if not families:
        return None
    sc = read(question, pack)
    if sc is None or not sc.quantities:               # a model needs stated quantities ("the half-life of carbon-14"
        return None                                   # is a lookup, not a computation)
    for fam in families:
        try:
            out = fam(sc, pack)
        except (KeyError, IndexError, ValueError, TypeError, AttributeError):
            continue
        if out is None:
            continue
        if isinstance(out, Refusal):
            return refusal_answer(question, out)
        return solve(out, pack, question)
    return None
