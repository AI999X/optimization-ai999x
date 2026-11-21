# GENERATED from ai999x_factory/runtime by tools/build_package.py — do not edit.
"""The answer object every domain package returns from ask()."""
from __future__ import annotations

import json
from dataclasses import asdict, dataclass, field

from .prose_math import _sym_tex, expr_tex, prose_markdown


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
    """x1 -> x_{1}; a1 -> a_{1}; discriminant -> \\mathrm{discriminant}; v -> v; AUC_0_t -> \\mathrm{AUC}_{0,t};
    break_even_revenue -> \\mathrm{break\\_even\\_revenue} (several underscores used to make a double subscript,
    which stops the whole formula block from rendering)."""
    import re
    m = re.fullmatch(r"([A-Za-z])(\d+|[a-z])", name)
    if m and len(name) <= 3:
        return f"{m.group(1)}_{{{m.group(2)}}}"
    if "_" in name:
        return _sym_tex(name)
    return name if len(name) == 1 else f"\\mathrm{{{name}}}"


def _tex_e(s: str) -> str:
    """Numbers written with an exponent, inside LaTeX: 2.55061e+10 -> 2.55061 \\times 10^{10}, 4e-06 -> 4 \\times 10^{-6}."""
    import re
    return re.sub(r"(\d)e([+-]?)0*(\d+)", lambda m: f"{m.group(1)} \\times 10^{{{'-' if m.group(2) == '-' else ''}"
                  f"{m.group(3)}}}", s)


def _fmt(x) -> str:
    if isinstance(x, bool) or not isinstance(x, (int, float)):   # a structured input echoed as written ([1 2; 3 4])
        return str(x)
    return f"{x:.6g}"


def _tex_unit(unit: str) -> str:
    """'Pa*m^6/mol^2' -> \\mathrm{Pa\\,m^{6}/mol^{2}}; '' -> ''."""
    import re
    if not unit:
        return ""
    u = re.sub(r"\^\(?(-?\d+(?:\.\d+)?)\)?", r"^{\1}", unit.replace("%", "\\%")).replace("*", "\\,")
    return f"\\mathrm{{{u}}}"


def _tex_text(s: str) -> str:
    """A value written as text (an expression, a matrix, a formula): typeset as math when it reads as math
    ("1 + cos(theta)" -> 1 + \\cos(\\theta)), otherwise upright text. Characters with no text-mode form in every
    renderer (MathJax in JupyterLab, KaTeX elsewhere) are set as math symbols between the text pieces."""
    import re
    tex = expr_tex(s)
    if tex is not None:
        return tex
    for a, b in (("{", "\\{"), ("}", "\\}"), ("_", "\\_"), ("%", "\\%"), ("#", "\\#"), ("&", "\\&"), ("$", "\\$")):
        s = s.replace(a, b)
    sym = {"\\": "\\backslash ", "^": "\\hat{\\ }", "~": "\\sim ", "·": "\\cdot "}
    pieces = re.split(r"([\\^~·])", s)
    return "".join(sym[p] if p in sym else (f"\\text{{{p}}}" if p else "") for p in pieces)


def _brace_subs(tex: str) -> str:
    """x_ab -> x_{a}b: a one-character subscript directly followed by a letter gets braces. Same meaning everywhere;
    MathJax (JupyterLab) otherwise reads \\mathrm{C_xH_yO_z} as a double subscript and shows an error."""
    import re
    return re.sub(r"(?<!\\)_([A-Za-z0-9])(?=[A-Za-z])", r"_{\1}", tex)   # an escaped \_ is a literal underscore


def _align_at_first_equals(row: str) -> str:
    """'x = 1' -> 'x &= 1' for an aligned block: the first ' = ' outside any braces (a subscript such as
    \\max_{|S| = p} keeps its own sign)."""
    depth = 0
    for i, c in enumerate(row):
        depth += {"{": 1, "}": -1}.get(c, 0)
        if depth == 0 and row.startswith(" = ", i):
            return row[:i] + " &= " + row[i + 3:]
    return row


def _md_text(s: str) -> str:
    """Prose for Markdown: characters that Markdown would turn into emphasis or math are shown as themselves."""
    for ch in ("\\", "*", "_", "$", "`"):
        s = s.replace(ch, "\\" + ch)
    return s


def _tex_given(inputs: dict) -> str:
    """The inputs of one step as typeset math: L_{p} = 1\\ \\mathrm{m},\\; g = 9.80665\\ \\mathrm{m/s^{2}}."""
    parts = []
    for k, v in inputs.items():
        val = v["value"]
        num = _tex_e(_fmt(val)) if isinstance(val, (int, float)) and not isinstance(val, bool) else _tex_text(str(val))
        unit = _tex_unit(v.get("unit") or "")
        parts.append(f"{_tex_name(k)} = {num}" + (f"\\ {unit}" if unit else ""))
    return ",\\; ".join(parts)


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
        if "max_residual" in v:
            lines.append(f"Verification: {v.get('status', 'not run')} (residual {v.get('max_residual', 0):.1e}, "
                         f"{v.get('independent_paths', 1)} derivation path(s))")
        else:                                         # the Rust engine checks before it answers; it reports no residual
            lines.append(f"Verification: {v.get('status', 'not run')} ({v.get('scope', 'checked')})")
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
                rows += [f"{_tex_name(k)} = " + _tex_e(_fmtv(v).replace("i", "\\,i"))
                         + (f"\\ \\mathrm{{{ou[k].replace('*', chr(92) + ',')}}}" if ou.get(k) else "")
                         for k, v in s["outputs"].items() if not isinstance(v, str)]
                continue
            rows.append(s["solved_latex"] + " = " + _tex_e(_fmt(s["result"])) + "\\ \\mathrm{" + unit + "}")
        return "\\begin{aligned}" + " \\\\ ".join(_align_at_first_equals(_brace_subs(r)) for r in rows) + "\\end{aligned}"

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
        for i, s in enumerate(self.steps, 1):     # each step typeset: its equation and its inputs (never plain text)
            eq = s.get("equation_latex") or ""
            eq = f"${_brace_subs(eq)}$" if eq else _md_text(s["equation"])
            given = _tex_given(s.get("inputs") or {})
            out += ["", f"{i}. *{prose_markdown(s['name'])}* — {eq}" + (f"; with ${given}$" if given else "")]
        if self.assumptions:                     # the math inside each sentence typeset ("1.05 <= gamma <= 1.7")
            out += ["", "*Assumptions:* " + "; ".join(prose_markdown(a) for a in self.assumptions)]
        out += ["", f"*Verification:* {self.verification.get('status')}"]
        out += [f"*Warning:* {prose_markdown(w)}" for w in self.warnings]
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
