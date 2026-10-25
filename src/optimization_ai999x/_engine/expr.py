# GENERATED from ai999x_factory/runtime by tools/build_package.py — do not edit.
"""Safe mathematical expressions: an allowlisted parser (never eval/sympify on text), dimension inference and
LaTeX generated from the canonical expression."""
from __future__ import annotations

import ast
from fractions import Fraction

import sympy as sp

from .quantity import ONE, Dim

FUNCS = {"sqrt": sp.sqrt, "sin": sp.sin, "cos": sp.cos, "tan": sp.tan, "exp": sp.exp, "log": sp.log,
         "asin": sp.asin, "acos": sp.acos, "atan": sp.atan}
CONSTS = {"pi": sp.pi}
_DIMENSIONLESS_ARG = (sp.sin, sp.cos, sp.tan, sp.exp, sp.log, sp.asin, sp.acos, sp.atan)


class ExpressionError(ValueError):
    pass


def parse(text: str, symbols: dict, symbolic_powers: bool = False) -> sp.Expr:
    """Parse arithmetic over the given symbols. Only numbers, the named symbols, pi, + - * / **, and the
    functions in FUNCS are accepted; anything else raises ExpressionError."""
    if not isinstance(text, str) or not text.strip() or len(text) > 400:
        raise ExpressionError("expression missing or too long")
    try:
        tree = ast.parse(text.strip(), mode="eval")
    except SyntaxError as exc:
        raise ExpressionError(f"cannot parse: {text}") from exc
    if sum(1 for _ in ast.walk(tree)) > 120:
        raise ExpressionError("expression too complex")

    def walk(n):
        if isinstance(n, ast.Expression):
            return walk(n.body)
        if isinstance(n, ast.Constant) and type(n.value) in (int, float):
            return sp.Integer(n.value) if isinstance(n.value, int) else sp.Rational(str(n.value))
        if isinstance(n, ast.Name):
            if n.id in symbols:
                return symbols[n.id]
            if n.id in CONSTS:
                return CONSTS[n.id]
            raise ExpressionError(f"unknown name: {n.id}")
        if isinstance(n, ast.UnaryOp) and isinstance(n.op, (ast.USub, ast.UAdd)):
            v = walk(n.operand)
            return -v if isinstance(n.op, ast.USub) else v
        if isinstance(n, ast.BinOp) and isinstance(n.op, (ast.Add, ast.Sub, ast.Mult, ast.Div, ast.Pow)):
            a, b = walk(n.left), walk(n.right)
            if isinstance(n.op, ast.Add):
                return a + b
            if isinstance(n.op, ast.Sub):
                return a - b
            if isinstance(n.op, ast.Mult):
                return a * b
            if isinstance(n.op, ast.Div):
                return a / b
            if symbolic_powers and not b.is_number:      # procedures: r**(n - 1) (still allowlisted syntax)
                return a ** b
            if not b.is_Rational or abs(b) > 8:
                raise ExpressionError("only small rational exponents are allowed")
            return a ** b
        if (isinstance(n, ast.Call) and isinstance(n.func, ast.Name) and n.func.id in FUNCS
                and len(n.args) == 1 and not n.keywords):
            return FUNCS[n.func.id](walk(n.args[0]))
        raise ExpressionError("unsupported syntax")

    return walk(tree)


def dims(expr: sp.Expr, symbol_dims: dict) -> Dim:
    """Physical dimension of an expression (angles count as dimensionless). Raises ExpressionError when a
    sum mixes dimensions or a transcendental function receives a dimensioned argument."""
    if expr.is_Number or expr.is_NumberSymbol:
        return ONE
    if expr.is_Symbol:
        return symbol_dims[str(expr)].physical()
    if expr.is_Mul:
        out = ONE
        for a in expr.args:
            out = out * dims(a, symbol_dims)
        return out
    if expr.is_Pow:
        if not expr.exp.is_Rational:
            raise ExpressionError("exponent must be a rational number")
        return dims(expr.base, symbol_dims) ** Fraction(int(expr.exp.p), int(expr.exp.q))
    if expr.is_Add:
        seen = {dims(a, symbol_dims) for a in expr.args}
        if len(seen) != 1:
            raise ExpressionError("a sum mixes different dimensions")
        return seen.pop()
    if expr.func in _DIMENSIONLESS_ARG:
        if not dims(expr.args[0], symbol_dims).dimensionless:
            raise ExpressionError(f"{expr.func.__name__} needs a dimensionless argument")
        return ONE
    raise ExpressionError(f"no dimension rule for {expr.func}")


def latex(expr, names: dict | None = None) -> str:
    """LaTeX from the canonical expression; `names` maps a symbol to its display form."""
    return sp.latex(expr, symbol_names={k: v for k, v in (names or {}).items()}, ln_notation=True)


# ----------------------------------------------------------------------------------- textbook typesetting
_GREEK = ["alpha", "beta", "gamma", "delta", "epsilon", "varepsilon", "zeta", "eta", "theta", "vartheta", "iota",
          "kappa", "lambda", "lam", "mu", "nu", "xi", "pi", "rho", "sigma", "tau", "upsilon", "phi", "varphi", "chi",
          "psi", "omega"]
_GREEK_TEX = {g: ("\\lambda" if g == "lam" else "\\" + g) for g in _GREEK}
_GREEK_TEX["eps"] = "\\varepsilon"
_GREEK_TEX.update({g.capitalize(): "\\" + g.capitalize() for g in
                   ("gamma", "delta", "theta", "lambda", "xi", "pi", "sigma", "upsilon", "phi", "psi", "omega")})
_UPRIGHT = {"Re", "Fr", "Ma", "Nu", "Pr", "Gr", "We", "Bi", "Sc", "Sh", "Pe", "St", "Kn", "Ra", "Eu", "Ca"}  # numbers


def _sub(text: str) -> str:
    """A subscript: greek letters, short labels italic, words upright."""
    t = text.strip("_")
    if not t:
        return ""
    if t in _GREEK_TEX:
        return _GREEK_TEX[t]
    if len(t) > 2 and t.isalpha():
        return "\\text{" + t.replace("_", " ") + "}"
    return t.replace("_", ",")


def symbol_latex(name: str, kind: str = "", role: str = "") -> str:
    """Textbook form of a symbol name: rho -> \\rho, P0 -> P_{0}, v1f -> v_{1f}, R_h -> R_{h}, delta_p -> \\Delta p,
    rho_f -> \\rho_{f}, wd -> \\omega_{d} for an angular frequency, Re -> \\mathrm{Re}, left_limit -> \\text{left limit}."""
    import re
    if role.startswith("delta") and len(name) == 2 and name[0] == "d":   # dv for a change in v
        return "\\Delta " + name[1]
    base, _, sub = name.partition("_")
    if base.lower() == "delta" and sub:                  # delta_p: a change
        return "\\Delta " + symbol_latex(sub, kind, "")
    if base in _UPRIGHT and not sub:
        return "\\mathrm{" + base + "}"
    if base in ("re", "im") and not sub:                 # real and imaginary parts
        return "\\operatorname{" + base.capitalize() + "}"
    w = re.match(r"^([A-Za-z]{3,})(\d+)$", base)          # num1 -> num_1, a word with a number
    if w and w.group(1) not in _GREEK_TEX:
        return "\\text{" + w.group(1) + "}_{" + w.group(2) + (("," + _sub(sub)) if sub else "") + "}"
    # a Greek name alone or with a number (omega0, theta1); "num1" is not nu + m1
    m = re.match(r"^(" + "|".join(sorted(_GREEK_TEX, key=len, reverse=True)) + r")(\d*)$", base)
    if m:                                                # rho, omega0, theta1
        head, rest = _GREEK_TEX[m.group(1)], m.group(2) + (("_" + sub) if sub else "")
        return head + ("_{" + _sub(rest) + "}" if rest.strip("_") else "")
    tail = re.match(r"^([A-Za-z])(rms|max|min|net|avg|eff|tot|ext|int|cm|out|in|ref|crit|sat)$", base)
    if tail and not sub:                                 # Irms -> I_rms, vmax -> v_max
        return tail.group(1) + "_{\\text{" + tail.group(2) + "}}"
    if len(base) > 2 and base.isalpha() and (not sub or len(sub) > 2):   # a word: left_limit, total
        return "\\text{" + name.replace("_", " ") + "}"
    head, rest = base[0], base[1:]
    if kind == "angular_frequency" and head in "wW":    # w is the usual ASCII stand-in for omega
        head = "\\omega" if head == "w" else "\\Omega"
    rest = rest + (("_" + sub) if sub else "") if rest else sub
    return head + ("_{" + _sub(rest) + "}" if rest else "")


_PREC = {ast.Add: 1, ast.Sub: 1, ast.Mult: 2, ast.Div: 2, ast.Pow: 4}
_FN = {"sqrt": None, "sin": "\\sin", "cos": "\\cos", "tan": "\\tan", "asin": "\\arcsin", "acos": "\\arccos",
       "atan": "\\arctan", "log": "\\ln", "exp": None}


def _num(v) -> str:
    if isinstance(v, int) or float(v).is_integer() and abs(v) < 1e15:
        return str(int(v))
    s = repr(float(v))
    if "e" in s:
        mant, ex = s.split("e")
        return f"{mant} \\times 10^{{{int(ex)}}}"
    return s


def latex_written(text: str, names: dict | None = None) -> str:
    """Typeset an allowlisted expression or equation IN THE ORDER IT IS WRITTEN (rho*g*h stays rho g h; sympy's
    printer would sort it to g h rho): fractions, roots, functions, minimal parentheses. `names` maps a symbol name
    to its LaTeX (symbol_latex by default). Only the allowlisted syntax is accepted, as in parse()."""
    names = names or {}
    if "=" in text:
        lhs, rhs = text.split("=", 1)
        return latex_written(lhs, names) + " = " + latex_written(rhs, names)
    tree = ast.parse(text.strip(), mode="eval").body

    def prec(n):
        if isinstance(n, ast.BinOp):
            return _PREC.get(type(n.op), 5)
        if isinstance(n, ast.UnaryOp):
            return 3
        return 5

    def wrap(n, need):
        s = walk(n)
        return f"\\left({s}\\right)" if prec(n) < need else s

    def walk(n):
        if isinstance(n, ast.Constant) and type(n.value) in (int, float):
            return _num(n.value)
        if isinstance(n, ast.Name):
            if n.id in CONSTS:
                return "\\pi"
            return names.get(n.id) or symbol_latex(n.id)
        if isinstance(n, ast.UnaryOp) and isinstance(n.op, (ast.USub, ast.UAdd)):
            return ("-" if isinstance(n.op, ast.USub) else "") + wrap(n.operand, 3)
        if isinstance(n, ast.BinOp):
            op = type(n.op)
            if op is ast.Div:
                return f"\\frac{{{walk(n.left)}}}{{{walk(n.right)}}}"
            if op is ast.Pow:
                if isinstance(n.right, ast.Constant) and n.right.value == 0.5:
                    return f"\\sqrt{{{walk(n.left)}}}"
                base = wrap(n.left, 5) if not (isinstance(n.left, ast.Call) and n.left.func.id == "sqrt") else f"\\left({walk(n.left)}\\right)"
                return f"{base}^{{{walk(n.right)}}}"
            if op is ast.Mult:
                l, r = wrap(n.left, 2), wrap(n.right, 3)
                numeric = isinstance(n.right, ast.Constant) or (isinstance(n.right, ast.UnaryOp))
                return f"{l} \\cdot {r}" if numeric else f"{l} {r}"
            sym = " + " if op is ast.Add else " - "
            return walk(n.left) + sym + (wrap(n.right, 2) if op is ast.Sub else walk(n.right))
        if isinstance(n, ast.Call) and isinstance(n.func, ast.Name) and n.func.id in _FN and len(n.args) == 1:
            arg = walk(n.args[0])
            if n.func.id == "sqrt":
                return f"\\sqrt{{{arg}}}"
            if n.func.id == "exp":
                return f"e^{{{arg}}}"
            return f"{_FN[n.func.id]}\\left({arg}\\right)"
        raise ExpressionError("unsupported syntax")

    return walk(tree)
