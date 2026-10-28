# GENERATED from ai999x_factory/runtime by tools/build_package.py — do not edit.
"""Compiled procedures: a VERIFIED declarative specification turned into the procedure interface the pack exposes.

A specification (models/<domain>/capabilities/procedures_compiled.json, written only by
`tools/compile_procedures.py` after verification) states, as data:
  inputs     [{symbol, meaning, kind ("number" or a pack kind with a unit), required, condition?}]
  outputs    [{symbol, meaning, kind, expr}]       expr: a closed form over the inputs (allowlisted syntax)
  equations  ["lhs = rhs", ...]                    what the outputs must satisfy (the verification, and the
                                                   independent reference: certification solves THESE, not expr)
  for_all    [variable, ...]                       variables an equation must hold for (an identity)
  conditions ["a != 0", "a*e - b*d != 0", ...]     violated -> ValueError -> INVALID_INPUT
  templates  ["Solve {a}x + {b} = {c}.", ...]      documented question forms; {name} is a number slot
Nothing here evaluates text as code: expressions go through `expr.parse` (an allowlist) and are compiled from the
resulting sympy tree.
"""
from __future__ import annotations

import cmath
import math
import random
import re
from fractions import Fraction

from . import expr as E
from ._lazy import sp

_CFUN = {"sqrt": cmath.sqrt, "exp": cmath.exp, "log": cmath.log, "sin": cmath.sin, "cos": cmath.cos, "tan": cmath.tan,
         "asin": cmath.asin, "acos": cmath.acos, "atan": cmath.atan, "pi": math.pi}
_NUM = r"[-+]?\s*(?:\d+(?:\.\d+)?(?:/\d+)?|\.\d+)"
_UNIT = r"[A-Za-zµΩ°][A-Za-z0-9µΩ°/^*.()\-]*"
_COND = re.compile(r"^(.+?)\s*(!=|>=|<=|>|<)\s*(.+)$")


def _real(z):
    if isinstance(z, complex) and abs(z.imag) <= 1e-12 * max(1.0, abs(z.real)):
        return z.real + 0.0
    return z + 0.0 if isinstance(z, float) else z


def _equation(text: str, syms: dict):
    if text.count("=") != 1:
        raise E.ExpressionError(f"an equation needs exactly one '=': {text}")
    lhs, rhs = text.split("=")
    return E.parse(lhs.strip(), syms, symbolic_powers=True), E.parse(rhs.strip(), syms, symbolic_powers=True)


def _number(text: str) -> float:
    t = text.replace(" ", "")
    return float(Fraction(t)) if "/" in t else float(t)


class CompiledProcedure:
    def __init__(self, spec: dict):
        self.spec = spec
        self.ins = [i["symbol"] for i in spec["inputs"]]
        self.outs = [o["symbol"] for o in spec["outputs"]]
        self.free = list(spec.get("for_all", []))
        self.syms = {n: sp.Symbol(n) for n in self.ins + self.outs + self.free}
        in_syms = {n: self.syms[n] for n in self.ins}
        self.out_expr = {o["symbol"]: E.parse(o["expr"], in_syms, symbolic_powers=True) for o in spec["outputs"] if o.get("expr")}
        self.equations = [_equation(e, self.syms) for e in spec.get("equations", [])]
        self.conditions = []
        stated = list(spec.get("conditions", []))       # a condition written on an input line binds as well
        stated += [i["condition"] for i in spec["inputs"] if i.get("condition") and i["condition"] not in stated]
        for c in stated:
            m = _COND.match(c)
            if not m:
                raise E.ExpressionError(f"cannot read the condition {c!r}")
            self.conditions.append((c, E.parse(m.group(1), in_syms, symbolic_powers=True) - E.parse(m.group(3), in_syms, symbolic_powers=True), m.group(2)))
        args = [self.syms[n] for n in self.ins]
        self._f = {k: sp.lambdify(args, v, modules=[_CFUN, "math"]) for k, v in self.out_expr.items()}
        allv = [self.syms[n] for n in self.ins + self.outs + self.free]
        self._res = [(sp.lambdify(allv, l - r, modules=[_CFUN, "math"]), sp.lambdify(allv, abs(l) + abs(r), modules=[_CFUN, "math"]))
                     for l, r in self.equations]
        self._cond = [(c, sp.lambdify(args, d, modules=[_CFUN, "math"]), op) for c, d, op in self.conditions]
        self._templates = [self._template_regex(t) for t in spec.get("templates", [])]

    # --- the procedure interface (same keys as a hand-written procedure) ----------------------------------------
    def run(self, **values) -> dict:
        for i in self.spec["inputs"]:
            if i.get("kind") == "integer" and float(values[i["symbol"]]) != int(float(values[i["symbol"]])):
                raise ValueError(f"{i['symbol']} must be a whole number")
        x = [values[n] for n in self.ins]
        for c, f, op in self._cond:
            d = _real(complex(f(*x)))
            ok = {"!=": lambda v: abs(v) > 1e-12, ">": lambda v: v > 0, "<": lambda v: v < 0,
                  ">=": lambda v: v >= 0, "<=": lambda v: v <= 0}[op](d if not isinstance(d, complex) else float("nan"))
            if not ok:
                raise ValueError(f"the condition {c} does not hold for these inputs")
        out = {k: _real(complex(f(*x))) for k, f in self._f.items()}
        for k, v in out.items():
            if isinstance(v, complex) and not self.spec.get("complex_outputs"):
                raise ValueError(f"{k} has no real value for these inputs")
        return out

    def verify(self, inputs: dict, outputs: dict) -> float:
        """Largest relative residual of the defining equations (at three points of each for_all variable)."""
        worst = 0.0
        base = [inputs[n] for n in self.ins] + [outputs[n] for n in self.outs]
        for pt in ((-1.7, 0.3, 2.9) if self.free else (None,)):
            vals = base + ([pt + 0.37 * j for j in range(len(self.free))] if pt is not None else [])
            for res, scale in self._res:
                r, s = abs(complex(res(*vals))), abs(complex(scale(*vals)))
                worst = max(worst, r / max(1.0, s))
        return worst

    def parse(self, question: str):
        ranked = self.parse_ranked(question)
        return None if ranked is None else ranked[1]

    def parse_ranked(self, question: str):
        """(rank, inputs): 0 for an exact documented template, 1 for a recognised out-of-scope form, 2 for the
        'with a = .., b = ..' reading; None when the question is not this procedure's."""
        for rx, slots in self._templates:
            m = rx.fullmatch(question.strip())
            if not m:
                continue
            out = {}
            for s, kinded in slots:
                raw = m.group(s)
                if kinded:
                    out[s] = f"{raw} {m.group(s + '__u')}"
                else:
                    sign = m.groupdict().get(s + "__sign")
                    v = 1.0 if raw in (None, "", "+") else (-1.0 if raw == "-" else _number(raw))
                    out[s] = -v if sign == "-" else v
            return 0, out
        q = question.lower()
        for o in self.spec.get("out_of_scope", []):
            if re.search(o["pattern"][:200], q):
                return 1, {"_out_of_scope": o["why"]}
        words = self.spec.get("keywords", [])
        if words and any(w in q for w in words):
            pairs = {k: _number(v) for k, v in re.findall(r"\b([a-z]\w*)\s*=\s*(" + _NUM + r")", q) if k in self.ins}
            if pairs:
                return 2, pairs
        return None

    def _template_regex(self, template: str):
        """'{a}x + {b} = {c}' -> a regex. A slot written directly before a letter is a coefficient (may be omitted:
        1, or '-': -1); a slot after ' + ' also reads ' - n' (the value negated); a slot of a kinded input reads a
        number and its unit."""
        kinded = {i["symbol"] for i in self.spec["inputs"] if i.get("kind", "number") not in ("number", "integer")}
        parts, slots, pos = [], [], 0
        for m in re.finditer(r"\{(\w+)\}", template):
            s = m.group(1)
            lit = template[pos:m.start()]
            after = template[m.end():m.end() + 1]
            if s in [x for x, _k in slots]:                # the same slot twice: the same value must be written again
                parts.append(re.escape(lit))
                parts.append(rf"(?P={s})")
                pos = m.end()
                continue
            if s in kinded:
                parts.append(re.escape(lit))
                parts.append(rf"(?P<{s}>{_NUM})\s*(?P<{s}__u>{_UNIT})")
            elif lit.endswith("+ ") or lit.endswith("+"):
                parts.append(re.escape(lit.rstrip().rstrip("+").rstrip()))
                opt = "?" if after.isalpha() else ""            # '- y': a coefficient of 1 is not written
                parts.append(rf"\s*(?P<{s}__sign>[+-])\s*(?P<{s}>\d+(?:\.\d+)?(?:/\d+)?){opt}")
            elif after.isalpha():
                parts.append(re.escape(lit))
                parts.append(rf"(?P<{s}>[-+]?\s*(?:\d+(?:\.\d+)?(?:/\d+)?)?)")
            else:
                parts.append(re.escape(lit))
                parts.append(rf"(?P<{s}>{_NUM})")
            slots.append((s, s in kinded))
            pos = m.end()
        parts.append(re.escape(template[pos:]))
        rx = "".join(parts).replace(r"\ ", r"\s*")
        return re.compile(rx, re.I), slots

    # --- certification only (independent of `expr`): solve the EQUATIONS for the outputs ------------------------
    def _ref_equations(self, sub=None) -> list:
        eqs = []
        for l, r in self.equations:
            d = sp.expand((l - r).subs(sub) if sub else l - r)
            if self.free:
                eqs += list(sp.Poly(d, *[self.syms[v] for v in self.free]).coeffs())
            else:
                eqs.append(d)
        return eqs

    def _admissible(self, sols, sub) -> list:
        for c in self.spec.get("output_conditions", []):     # "rad > 0": the equations' admissible solution
            m = re.match(r"(.+?)(>=|<=|!=|>|<)(.+)", c)
            ok = {">": lambda d: d > 0, "<": lambda d: d < 0, ">=": lambda d: d >= 0, "<=": lambda d: d <= 0,
                  "!=": lambda d: d != 0}[m.group(2)]
            gap = E.parse(f"({m.group(1)}) - ({m.group(3)})", self.syms, symbolic_powers=True).subs(sub)
            sols = [s for s in sols if gap.subs(s).is_real and ok(float(sp.N(gap.subs(s))))]
        return sols

    def reference(self, **values) -> dict:
        """The outputs from the EQUATIONS alone. Solved symbolically once per procedure, then evaluated per case to
        30 digits; each candidate must satisfy the equations at the case (an exact-input solve per case took
        minutes on exponential forms: compound_interest, 2026-10-16). Falls back to the per-case solve."""
        # inputs as the decimals they are written as (Fraction(7407.713) is a binary fraction whose exact powers
        # sympy cannot evaluate in reasonable time); symbolic solutions are evaluated with 40-digit floats
        sub = {self.syms[n]: sp.Rational(Fraction(repr(float(values[n]))).limit_denominator(10**12)) for n in self.ins}
        subf = {self.syms[n]: sp.Float(repr(float(values[n])), 40) for n in self.ins}
        outs = [self.syms[o] for o in self.outs]
        if not hasattr(self, "_general"):
            try:
                self._general = sp.solve(self._ref_equations(), outs, dict=True)
            except (NotImplementedError, ValueError, TypeError):
                self._general = []
        sols = []
        if self._general:
            eqs = self._ref_equations(subf)
            for g in self._general:
                cand = {k: sp.N(v.subs(subf), 30) for k, v in g.items()}
                if any(not c.is_finite for c in cand.values()):
                    continue
                res = [abs(complex(sp.N(e.subs(cand), 30))) for e in eqs]
                if all(r <= 1e-15 * max(1.0, *(abs(complex(c)) for c in cand.values())) for r in res):
                    sols.append(cand)
        if not sols:
            sols = sp.solve(self._ref_equations(sub), outs, dict=True)
        sols = self._admissible(sols, subf)
        uniq = []                                          # two symbolic branches may meet at a case
        for c in sols:
            v = {k: complex(sp.N(x, 30)) for k, x in c.items()}
            if not any(v.keys() == u.keys() and all(abs(v[k] - u[k]) <= 1e-12 * max(1.0, abs(u[k])) for k in v)
                       for u in uniq):
                uniq.append(v)
        if len(uniq) != 1:
            raise ValueError(f"the equations do not determine the outputs uniquely ({len(uniq)} solutions)")
        return {o: _real(uniq[0][self.syms[o]]) for o in self.outs if self.syms[o] in uniq[0]}

    def wrong(self, **values) -> dict:
        """A deliberately wrong variant: the first output with the sign of its first term flipped (else doubled)."""
        out = self.run(**values)
        first = self.outs[0]
        e = self.out_expr[first]
        mutated = e.func(-e.args[0], *e.args[1:]) if isinstance(e, sp.Add) else 2 * e
        f = sp.lambdify([self.syms[n] for n in self.ins], mutated, modules=[_CFUN, "math"])
        return dict(out, **{first: _real(complex(f(*[values[n] for n in self.ins])))})

    def random_inputs(self, rng: random.Random) -> dict:
        out = {}
        for i in self.spec["inputs"]:
            lo, hi = i.get("range", [-20, 20])
            whole = rng.randint(int(math.ceil(lo)), int(math.floor(hi)))
            out[i["symbol"]] = whole if i.get("kind") == "integer" else rng.choice([whole, round(rng.uniform(lo, hi), 3)])
        return out


def render(template: str, values: dict, units: dict | None = None) -> str:
    """A documented template with values, written as a person would: '2x - 4 = 6' (not '2x + -4'), 'x' for a
    coefficient of 1, '-x' for -1; a physical value carries its unit (units: symbol -> unit text)."""
    units = units or {}

    def num(v):
        return f"{v:g}"

    def one(m):
        s, after = m.group(1), template[m.end():m.end() + 1]
        v = values[s]
        if s in units:
            return f"{num(v)} {units[s]}"
        if after.isalpha() and v in (1, 1.0):
            return ""
        if after.isalpha() and v in (-1, -1.0):
            return "-"
        return num(v)
    text = re.sub(r"\{(\w+)\}", one, template)
    return re.sub(r"\+\s*-\s*", "- ", text).replace("+ +", "+ ")


def to_procedure(spec: dict) -> dict:
    """The dict a hand-written procedure provides (run, verify, parse, reference, wrong ...), from a compiled spec."""
    c = CompiledProcedure(spec)
    keep = ("name", "topic", "purpose", "inputs", "outputs", "primary", "conditions", "method", "latex", "templates",
            "examples", "questions", "provenance", "keywords", "output_conditions")
    out = {k: spec[k] for k in keep if k in spec}
    eqs = spec.get("equations") or []
    if eqs:                                              # typeset from the equations, as written, symbols in textbook form
        try:
            out["latex"] = ",\\quad ".join(E.latex_written(e) for e in eqs)
        except (E.ExpressionError, SyntaxError):
            pass                                         # keep the stored LaTeX
    return {**out, "run": c.run, "verify": c.verify, "parse": c.parse, "parse_ranked": c.parse_ranked,
            "reference": c.reference, "wrong": c.wrong, "random_inputs": c.random_inputs, "compiled": True}
