# GENERATED from ai999x_factory/runtime by tools/build_package.py — do not edit.
"""The answer object every domain package returns from ask()."""
from __future__ import annotations

import json
from dataclasses import asdict, dataclass, field


OUTCOMES = ("SOLVED", "NEEDS_INPUT", "AMBIGUOUS", "INVALID_INPUT", "OUT_OF_SCOPE", "UNRECOGNIZED_FORMAT", "INTERNAL_ERROR")
_CAUSE_OUTCOME = {
    "no_applicable_relation_chain": "NEEDS_INPUT", "underdetermined": "NEEDS_INPUT", "needs_input": "NEEDS_INPUT",
    "ambiguous_quantity": "AMBIGUOUS", "readings_disagree": "AMBIGUOUS", "multiple_branches": "AMBIGUOUS",
    "ambiguous": "AMBIGUOUS", "contradictory_conditions": "AMBIGUOUS",
    "invalid_input": "INVALID_INPUT", "unknown_name": "INVALID_INPUT", "invalid_unit": "INVALID_INPUT",
    "inconsistent_input": "INVALID_INPUT", "target_stated": "INVALID_INPUT", "no_physical_solution": "INVALID_INPUT",
    "no_solution": "INVALID_INPUT", "not_applicable": "INVALID_INPUT",
    "unsupported_structure": "OUT_OF_SCOPE", "unsupported_quantity": "OUT_OF_SCOPE", "trivial": "OUT_OF_SCOPE",
    "unknown_target": "UNRECOGNIZED_FORMAT", "unexplained_number": "UNRECOGNIZED_FORMAT", "no_question": "UNRECOGNIZED_FORMAT",
    "stated_quantity_not_used": "UNRECOGNIZED_FORMAT", "not_compiled": "UNRECOGNIZED_FORMAT",
    "computed_value_failed_its_checks": "INTERNAL_ERROR", "internal_error": "INTERNAL_ERROR",
}
_STATE_OUTCOME = {"missing_information": "NEEDS_INPUT", "ambiguous": "AMBIGUOUS", "unexplained_input": "UNRECOGNIZED_FORMAT",
                  "unsupported": "OUT_OF_SCOPE", "invalid": "INVALID_INPUT", "verification_failed": "INTERNAL_ERROR"}


def _fmtv(v) -> str:
    if v is None:                                     # an output that does not exist here (a tangent's second point)
        return "none"
    if isinstance(v, complex):
        return f"{v.real + 0.0:.6g} {'+' if v.imag >= 0 else '-'} {abs(v.imag):.6g}i"
    return _fmt(v) if isinstance(v, (int, float)) else str(v)


def _tex_name(name: str) -> str:
    """x1 -> x_{1}; a1 -> a_{1}; discriminant -> \\mathrm{discriminant}; v -> v."""
    import re
    m = re.fullmatch(r"([A-Za-z])(\d+|[a-z])", name)
    if m and len(name) <= 3:
        return f"{m.group(1)}_{{{m.group(2)}}}"
    return name if len(name) == 1 else f"\\mathrm{{{name}}}"


def _fmt(x) -> str:
    if isinstance(x, bool) or not isinstance(x, (int, float)):   # a structured input echoed as written ([1 2; 3 4])
        return str(x)
    return f"{x:.6g}"


@dataclass
class Answer:
    status: str                       # solved | needs_clarification | unsupported | invalid
    question: str
    value: float | None = None
    unit: str | None = None
    target: dict | None = None        # {"kind", "role", "phrase"}
    models: list = field(default_factory=list)       # relation keys, in the order applied
    assumptions: list = field(default_factory=list)
    steps: list = field(default_factory=list)
    verification: dict = field(default_factory=dict)
    warnings: list = field(default_factory=list)
    message: str = ""
    problem: dict | None = None       # the typed problem the question was compiled to
    # explicit state of knowledge: {"state": determined | missing_information | ambiguous | unexplained_input |
    # unsupported | invalid | verification_failed, ...details}
    uncertainty: dict = field(default_factory=dict)
    values: dict = field(default_factory=dict)        # every output of a capability with several (x1, x2, ...)

    @property
    def solved(self) -> bool:
        return self.status == "solved"

    @property
    def outcome(self) -> str:
        """One of OUTCOMES: what happened, as a fixed word a program can act on."""
        if self.status == "solved":
            return "SOLVED"
        if self.status in ("internal_error", "verification_failed"):
            return "INTERNAL_ERROR"
        u = self.uncertainty or {}
        cause, state = u.get("cause", ""), u.get("state", "")
        if u.get("outcome") in OUTCOMES:
            return u["outcome"]
        if cause in _CAUSE_OUTCOME:
            if cause == "no_applicable_relation_chain" and not u.get("would_need"):
                return "OUT_OF_SCOPE"
            return _CAUSE_OUTCOME[cause]
        return _STATE_OUTCOME.get(state, "INVALID_INPUT" if self.status == "invalid" else "UNRECOGNIZED_FORMAT")

    def __str__(self) -> str:
        if not self.solved:
            out = f"{self.outcome}: {self.message}"
            hint = (self.uncertainty or {}).get("try")
            return out + (f"\nTry: {hint}" if hint else "")
        what = (self.target or {}).get("phrase") or (self.target or {}).get("kind", "answer")
        if self.values:
            lines = [self._values_line()]
        else:
            lines = [f"{what} = {_fmt(self.value)} {self.unit}".rstrip()]
        for i, s in enumerate(self.steps, 1):
            given = ", ".join(f"{k} = {_fmt(v['value'])} {v['unit']}".rstrip() for k, v in s["inputs"].items())
            got = (self._values_line(s["outputs"], s.get("output_units")) if s.get("outputs")
                   else f"{s['target']} = {_fmt(s['result'])} {s['unit']}".rstrip())
            lines.append(f"{i}. {s['name']}: {s['equation']}  with {given}  ->  {got}")
        if self.assumptions:
            lines.append("Assumptions: " + "; ".join(self.assumptions))
        v = self.verification
        lines.append(f"Verification: {v.get('status', 'not run')} (residual {v.get('max_residual', 0):.1e}, "
                     f"{v.get('independent_paths', 1)} derivation path(s))")
        lines += [f"Warning: {w}" for w in self.warnings]
        return "\n".join(lines)

    def latex(self) -> str:
        """The derivation as LaTeX, generated from the canonical expressions."""
        if not self.solved:
            return ""
        rows = []
        for s in self.steps:
            unit = s["unit"].replace("*", "\\,")
            rows.append(s["equation_latex"])
            if s.get("outputs"):                       # a procedure: every output, complex ones as a + bi
                ou = s.get("output_units") or {}
                rows += [f"{_tex_name(k)} = " + _fmtv(v).replace("i", "\\,i")
                         + (f"\\ \\mathrm{{{ou[k].replace('*', chr(92) + ',')}}}" if ou.get(k) else "")
                         for k, v in s["outputs"].items() if not isinstance(v, str)]
                continue
            rows.append(s["solved_latex"] + " = " + _fmt(s["result"]) + "\\ \\mathrm{" + unit + "}")
        return "\\begin{aligned}" + " \\\\ ".join(r.replace(" = ", " &= ", 1) for r in rows) + "\\end{aligned}"

    def _values_line(self, values: dict | None = None, units: dict | None = None) -> str:
        """Every output with its unit: 'x1 = 2, x2 = 3' or 'v = 14 m/s, s = 40 m'."""
        values = self.values if values is None else values
        if units is None:
            units = next((s.get("output_units") for s in self.steps if s.get("output_units")), None) or {}
        return ", ".join(f"{k} = {_fmtv(v)} {units.get(k, '')}".rstrip() for k, v in values.items())

    def _repr_markdown_(self) -> str:
        if not self.solved:
            hint = (self.uncertainty or {}).get("try")
            text = f"**{self.outcome}** — {self.message}" + (f"\n\n*Try:* `{hint}`" if hint else "")
            for s in getattr(self, "suggestions", None) or []:   # ask(..., suggest=True): pointers, never answers
                where = "" if s.get("here") else f" (`pip install {s['package']}`)"
                text += f"\n\n*Perhaps:* **{s['name']}**{where}" + (f" — e.g. “{s['example']}”" if s.get("example") else "")
            return text
        what = (self.target or {}).get("phrase", "answer")
        head = self._values_line() if self.values else f"{what} = {_fmt(self.value)} {self.unit}"
        out = [f"**{head}**", "", f"$$ {self.latex()} $$"]
        for i, s in enumerate(self.steps, 1):
            given = ", ".join(f"{k} = {_fmt(v['value'])} {v['unit']}".rstrip() for k, v in s["inputs"].items())
            out += ["", f"{i}. *{s['name']}* — {s['equation']}; with {given}"]
        if self.assumptions:
            out += ["", "*Assumptions:* " + "; ".join(self.assumptions)]
        out += ["", f"*Verification:* {self.verification.get('status')}"]
        out += [f"*Warning:* {w}" for w in self.warnings]
        return "\n".join(out)

    def show(self) -> "Answer":
        """Render in a notebook (Markdown + LaTeX) or print in a terminal."""
        try:
            from IPython import get_ipython
            from IPython.display import Markdown, display
            if get_ipython() is not None:
                display(Markdown(self._repr_markdown_()))
                return self
        except ImportError:
            pass
        print(self)
        return self

    def to_dict(self) -> dict:
        d = asdict(self)
        d["outcome"] = self.outcome
        if getattr(self, "suggestion", None):          # the optional decision model's suggestion (suggest mode only)
            d["suggestion"] = self.suggestion
        if getattr(self, "suggestions", None):         # capability search (ask(..., suggest=True))
            d["suggestions"] = self.suggestions
        enc = lambda v: {"re": v.real + 0.0, "im": v.imag + 0.0} if isinstance(v, complex) else v
        d["values"] = {k: enc(v) for k, v in self.values.items()}
        for s in d["steps"]:
            if s.get("outputs"):
                s["outputs"] = {k: enc(v) for k, v in s["outputs"].items()}
        return d

    def to_json(self, **kw) -> str:
        return json.dumps(self.to_dict(), ensure_ascii=False, **kw)
