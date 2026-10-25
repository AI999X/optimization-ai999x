# GENERATED from ai999x_factory/runtime by tools/build_package.py — do not edit.
"""A relation: one equation declared once, with every symbol typed by kind of quantity and semantic role.

The same declaration serves every unknown: `solve(target, values)` rearranges for the asked symbol, rejects
inputs and roots that break a symbol's sign constraint, and refuses when more than one admissible root remains.
"""
from __future__ import annotations

import math
from dataclasses import dataclass, field
from functools import cached_property

import sympy as sp

from . import expr as E

SIGNS = ("any", "nonnegative", "positive")


class RelationError(ValueError):
    pass


@dataclass(frozen=True)
class Sym:
    name: str
    kind: str
    role: str = "plain"
    accepts: tuple = ()
    sign: str = "any"
    latex: str = ""
    desc: str = ""
    default: dict | None = None      # {"value", "unit", "unless": [tags], "assumption"}

    def takes(self, role: str) -> bool:
        return role == self.role or role in self.accepts


@dataclass(frozen=True, eq=False)
class Relation:
    key: str
    name: str
    equation: str
    symbols: dict
    family: str = ""
    assumptions: tuple = ()
    requires: tuple = ()             # context tags that must be present
    excludes: tuple = ()             # context tags that rule the relation out
    frame: str = ""                  # the reference frame or geometry the symbols are defined in
    conventions: tuple = ()          # sign and direction conventions ("magnitudes only", "up is positive")
    applies_when: tuple = ()         # stated conditions of applicability, for the reader
    fails_when: tuple = ()           # stated limits of the model, for the reader
    derived_from: tuple = ()         # keys of the relations this one was derived from: not independent of them
    shortcut: bool = False           # a convenience that adds no knowledge: it may answer, never link a chain
    _cache: dict = field(default_factory=dict, compare=False, repr=False)

    @staticmethod
    def from_dict(d: dict) -> "Relation":
        for k in ("key", "name", "equation", "symbols"):
            if k not in d:
                raise RelationError(f"relation is missing '{k}'")
        syms = {}
        for name, s in d["symbols"].items():
            if not name.isidentifier() or "kind" not in s:
                raise RelationError(f"{d['key']}: bad symbol {name}")
            if s.get("sign", "any") not in SIGNS:
                raise RelationError(f"{d['key']}.{name}: unknown sign constraint")
            syms[name] = Sym(name, s["kind"], s.get("role", "plain"), tuple(s.get("accepts", ())),
                             s.get("sign", "any"), s.get("latex", ""), s.get("desc", ""), s.get("default"))
        return Relation(d["key"], d["name"], d["equation"], syms, d.get("family", ""),
                        tuple(d.get("assumptions", ())), tuple(d.get("requires", ())), tuple(d.get("excludes", ())),
                        d.get("frame", ""), tuple(d.get("conventions", ())), tuple(d.get("applies_when", ())),
                        tuple(d.get("fails_when", ())), tuple(d.get("derived_from", ())), bool(d.get("shortcut", False)))

    @cached_property
    def sp_symbols(self) -> dict:
        out = {}
        for name, s in self.symbols.items():
            flags = {"real": True}
            if s.sign == "positive":
                flags["positive"] = True
            elif s.sign == "nonnegative":
                flags["nonnegative"] = True
            out[name] = sp.Symbol(name, **flags)
        return out

    @cached_property
    def sides(self) -> tuple:
        if self.equation.count("=") != 1:
            raise RelationError(f"{self.key}: an equation needs exactly one '='")
        lhs, rhs = self.equation.split("=")
        try:
            l, r = E.parse(lhs, self.sp_symbols), E.parse(rhs, self.sp_symbols)
        except E.ExpressionError as exc:
            raise RelationError(f"{self.key}: {exc}") from exc
        used = {str(x) for x in (l.free_symbols | r.free_symbols)}
        if used != set(self.symbols):
            raise RelationError(f"{self.key}: declared symbols {sorted(self.symbols)} differ from used {sorted(used)}")
        return l, r

    @property
    def names(self) -> dict:
        return {self.sp_symbols[n]: (s.latex or E.symbol_latex(n, s.kind, s.role)) for n, s in self.symbols.items()}

    def latex(self) -> str:
        """The equation typeset as written (rho*g*h stays rho g h), symbols in textbook form."""
        try:
            return E.latex_written(self.equation, {n: self.names[self.sp_symbols[n]] for n in self.symbols})
        except (E.ExpressionError, SyntaxError, KeyError):
            l, r = self.sides
            return f"{E.latex(l, self.names)} = {E.latex(r, self.names)}"

    def solved_forms(self, target: str) -> list:
        if target not in self._cache:
            l, r = self.sides
            self._cache[target] = sp.solve(sp.Eq(l, r), self.sp_symbols[target])
        return self._cache[target]

    def latex_solved(self, target: str) -> str:
        forms = self.solved_forms(target)
        return f"{self.names[self.sp_symbols[target]]} = {E.latex(forms[0], self.names)}" if forms else ""

    def _admissible(self, name: str, value: float) -> bool:
        sign = self.symbols[name].sign
        return not ((sign == "positive" and value <= 0) or (sign == "nonnegative" and value < 0))

    def solve(self, target: str, values: dict) -> float:
        """The single admissible value of `target` given every other symbol. `values` maps a symbol name to a
        float or an exact sympy number. Raises RelationError otherwise — never picks a branch silently."""
        if target not in self.symbols or target in values:
            raise RelationError(f"{self.key}: cannot solve for {target}")
        missing = set(self.symbols) - set(values) - {target}
        if missing:
            raise RelationError(f"{self.key}: missing {sorted(missing)}")
        for name, v in values.items():
            if not self._admissible(name, float(v)):
                raise RelationError(f"{self.key}: {name} = {float(v):g} violates '{self.symbols[name].sign}'")
        subs = {self.sp_symbols[n]: v for n, v in values.items()}
        roots, terms = [], []
        for form in self.solved_forms(target):
            try:
                z = complex(sp.N(form.subs(subs), 17))
            except (TypeError, ValueError, ZeroDivisionError):
                continue
            if not (math.isfinite(z.real) and math.isfinite(z.imag)):
                continue
            scale = max(1.0, abs(z.real))
            if abs(z.imag) > 1e-9 * scale or not self._admissible(target, z.real):
                continue
            if not any(abs(z.real - r) <= 1e-9 * max(1.0, abs(r)) for r in roots):
                roots.append(z.real)
                try:                                     # the size of the terms that were added to give this value; a
                    # bounded factor (cos 90 deg = 6e-17) is taken at its bound of 1, so F*d*cos(theta) is judged
                    # against F*d, not against its own rounding noise
                    bounded = form.replace(sp.cos, lambda *a: sp.Integer(1)).replace(sp.sin, lambda *a: sp.Integer(1))
                    terms.append(max([abs(complex(sp.N(t.subs(subs), 17))) for t in sp.Add.make_args(sp.expand(form))] +
                                     [abs(complex(sp.N(t.subs(subs), 17))) for t in sp.Add.make_args(sp.expand(bounded))]))
                except (TypeError, ValueError, ZeroDivisionError):
                    terms.append(abs(z.real))
        if not roots:
            raise RelationError(f"{self.key}: no admissible real value for {target}")
        if len(roots) > 1:
            raise RelationError(f"{self.key}: {len(roots)} admissible values for {target} "
                                f"({', '.join(f'{r:g}' for r in roots)}); the branch is not determined")
        value = roots[0]
        # floating-point noise left by a cancellation (0 = v0 + a*t) is set to zero: tiny relative to the TERMS that
        # produced it, never relative to unrelated inputs (a proton's force is 1e-13 N while its speed is 1e6 m/s)
        return 0.0 if abs(value) < 1e-13 * terms[0] else value

    def residual(self, values: dict) -> float:
        """|lhs - rhs| relative to the size of the equation's TERMS, with every symbol given. Scaling by the
        terms, not by the two sides, keeps the measure meaningful when a side is zero (0 = v0 + a*t)."""
        l, r = self.sides
        subs = {self.sp_symbols[n]: v for n, v in values.items()}
        terms = [complex(sp.N(t.subs(subs), 17)) for t in sp.Add.make_args(sp.expand(l - r))]
        # bounded factors at their bound (cos 90 deg leaves 6e-17 of rounding): the scale is the size the terms can take
        bounded = (l - r).replace(sp.cos, lambda *a: sp.Integer(1)).replace(sp.sin, lambda *a: sp.Integer(1))
        scale = max(sum(abs(t) for t in terms),
                    sum(abs(complex(sp.N(t.subs(subs), 17))) for t in sp.Add.make_args(sp.expand(bounded))))
        return abs(sum(terms)) / scale if scale > 0 else 0.0
