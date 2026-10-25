# GENERATED from ai999x_factory/runtime by tools/build_package.py — do not edit.
"""A formula the user types, solved step by step: "Solve E = mc^2 for m", "Rearrange v = u + at for t",
"Make r the subject of A = pi r^2", "E = mc^2 with m = 2 and c = 3, find E".

Safe and exact:
  * the text is normalised (^ -> **, implicit multiplication made explicit: mc -> m*c, 2x -> 2*x, known names such
    as v0 or omega kept whole) and parsed by the allowlisted parser (runtime/expr.py) — never eval or sympify;
  * the asked symbol is isolated one inverse operation at a time (subtract, divide, root, inverse function), each
    step shown as an equation in LaTeX; a symbol that occurs more than once is solved exactly by sympy instead;
  * every result is checked by substituting it back into the original equation; a root that is not the principal
    one, or a solution that does not check, is refused, never shown;
  * numbers are substituted only when every stated value is a pure number (units are the engine's job: a question
    with units goes to the certified capabilities instead).
"""
from __future__ import annotations

import re

import sympy as sp

from .expr import FUNCS, ExpressionError, parse

GREEK = {"alpha", "beta", "gamma", "delta", "epsilon", "zeta", "eta", "theta", "iota", "kappa", "lambda", "lam",
         "mu", "nu", "xi", "rho", "sigma", "tau", "phi", "chi", "psi", "omega"}
_CUE = re.compile(r"\b(?:solve|rearrange|make\s+\w+\s+the\s+subject|isolate|express|find|what\s+is|compute|calculate|"
                  r"evaluate|derive\s+an\s+expression)\b", re.I)
_SOLVE_CUE = re.compile(r"\b(?:solve|rearrange|make\s+\w+\s+the\s+subject|isolate|express|transpose)\b", re.I)
# "for x values between ..." names a range, not the quantity to solve for (calculus surface area, 2026-10-23)
_TARGET = re.compile(r"\b(?:for|make|isolate|express|find|subject\s+of)\s+([A-Za-z][A-Za-z0-9_]*)\b"
                     r"(?!\s+(?:values?|between|from|ranging|in\s+the\s+range|=))"
                     # "For a machine ...": a solve target is never followed by a noun; only by the end, punctuation
                     # or a joining word ("for m with E = ...", "for m step by step")
                     r"(?!\s+(?!(?:in|if|when|given|where|with|using|step|and|by|the|of|to|then)\b)[a-z]{3,})", re.I)
_EQ = re.compile(r"([A-Za-z0-9_ .+\-*/^()√]+?)\s*=\s*([A-Za-z0-9_ .+\-*/^()√]+)")


# an ordinary (lowercase) word in an equation side is sentence text, not part of the formula: "age at redshift z =
# 0.0113", "f(x) = atan(2*x) + 3*x^2 on the interval", "E = mc^2 step by step". Read as letters it became a*g*e*...
# Each side is trimmed to the formula next to "=" (tools/ask_sweep.py, 2026-10-23); symbols are kept (KE, Re, Omega_m).
_TOKEN = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")
_STOP = {"at", "on", "in", "of", "by", "to", "is", "as", "an", "if", "or", "and", "the", "for", "with", "when", "where",
         "from", "per", "be", "it", "its", "we", "use", "then", "than", "that", "this", "find", "solve"}
_FUNCTION_DEF = re.compile(r"\s*[A-Za-z]\w*\s*\(\s*[A-Za-z]\w*\s*\)\s*")


def _text_spans(side: str, known: set) -> list:
    """Spans of sentence words in an equation side: a lowercase token that is not a function, Greek letter or known
    symbol and is a short common word, has four or more letters, or stands next to another lowercase word.
    An isolated run of letters ("mc", "mgh") is a product of symbols and stays."""
    toks = list(_TOKEN.finditer(side))

    def lower_word(m):
        w = m.group(0)
        return w.isalpha() and w.islower() and w not in FUNCS and w not in known and w not in GREEK and w != "pi"
    out = []
    for i, m in enumerate(toks):
        if not lower_word(m) or side[m.end():m.end() + 1] in ("(", "^") and m.group(0) not in _STOP:
            continue
        nb = [toks[j] for j in (i - 1, i + 1) if 0 <= j < len(toks)]
        next_to_word = len(m.group(0)) >= 2 and any(
            lower_word(n) and len(n.group(0)) >= 2 and not side[min(m.end(), n.end()):max(m.start(), n.start())].strip()
            for n in nb)                              # single letters are symbols ("m c^2"), never sentence words
        if m.group(0) in _STOP or len(m.group(0)) >= 4 or next_to_word:
            out.append(m)
    return out


def _trim_formula(lhs_t: str, rhs_t: str, known: set):
    """(lhs, rhs) cut to the formula next to "=": the left side after its last word, the right side before its first;
    None when a side is then empty or the left side defines a function ("f(x) = ..."), which the model's capabilities
    answer, not the formula solver."""
    words = _text_spans(lhs_t, known)
    if words:
        lhs_t = lhs_t[words[-1].end():]
    words = _text_spans(rhs_t, known)
    if words:
        rhs_t = rhs_t[:words[0].start()]
    lhs_t, rhs_t = lhs_t.strip(), rhs_t.strip().rstrip(".,;:")
    if not lhs_t or not rhs_t or _FUNCTION_DEF.fullmatch(lhs_t):
        return None
    return lhs_t, rhs_t


class FormulaError(ValueError):
    pass


def _identifiers(s: str) -> list:
    return re.findall(r"[A-Za-z][A-Za-z0-9_]*", s)


def normalise(text: str, known: set) -> str:
    """User arithmetic -> Python arithmetic over single symbols (never evaluated here)."""
    s = text.replace("^", "**").replace("√", "sqrt").replace("·", "*").replace("×", "*").replace("−", "-")
    # a multi-letter name written as a call ("log_base_3(x)", "sinh(x)") is a function this solver does not know:
    # refuse it rather than read it as a product (log_base_3 * x); a single letter before "(" is a product: a(b+c)
    for m in re.finditer(r"([A-Za-z][A-Za-z0-9_]+)\s*\(", s):
        if m.group(1).lower() not in FUNCS:
            raise FormulaError(f"'{m.group(1)}(...)' is a function I cannot work with here")
    out = []
    for tok in re.findall(r"[A-Za-z][A-Za-z0-9_]*|\d+(?:\.\d+)?(?:[eE][-+]?\d+)?|\*\*|[+\-*/()]|\s+|.", s):
        if re.fullmatch(r"[A-Za-z][A-Za-z0-9_]*", tok):
            low = tok.lower()
            if tok in known or low in FUNCS or low in GREEK or low == "pi" or re.fullmatch(r"[A-Za-z]\d+|[A-Za-z]_\w+", tok):
                out.append(tok)
            elif tok.isalpha():
                out.extend(" ".join(tok).split())          # mc -> m c (the products are made explicit below)
            else:
                out.append(tok)
        elif not tok.isspace():
            out.append(tok)
    res = []
    for i, tok in enumerate(out):                         # implicit products: 2x, x y, x(, )(, )x, 2(
        if res:
            prev = res[-1]
            left = re.fullmatch(r"[A-Za-z][A-Za-z0-9_]*|\d+(?:\.\d+)?(?:[eE][-+]?\d+)?|\)", prev) and prev.lower() not in FUNCS
            right = re.fullmatch(r"[A-Za-z][A-Za-z0-9_]*|\d+(?:\.\d+)?(?:[eE][-+]?\d+)?|\(", tok)
            if left and right:
                res.append("*")
        res.append(tok)
    return "".join(res)


WORDS = {"solve", "rearrange", "make", "the", "subject", "of", "for", "isolate", "express", "find", "what", "is",
         "compute", "calculate", "evaluate", "equation", "formula", "in", "terms", "derive", "an", "expression", "if",
         "given", "where", "when", "with", "and", "then", "please", "can", "you", "how", "do", "i", "to", "get"}


def find_equation(question: str):
    """(lhs text, rhs text) of the formula in the question, or None. The formula is what stands directly around
    its first '=': instruction words ('solve', 'for m', 'with m = 2') are cut off on both sides."""
    q = question.replace("−", "-").rstrip(" ?.!")
    q = re.split(r"\b(?:with|where|when|if|given)\b", q, maxsplit=1, flags=re.I)[0]
    if q.count("=") != 1:
        return None
    left, right = q.split("=")
    lt = re.findall(r"[^\s]+", left)
    keep = []
    for tok in reversed(lt):                            # the left side ends at the last instruction word
        if tok.lower().strip(",:") in WORDS:
            break
        keep.append(tok)
    lhs = " ".join(reversed(keep)).strip(" ,:")
    rt = re.findall(r"[^\s]+", right)
    keep = []
    for tok in rt:                                      # the right side ends at the first instruction word
        if tok.lower().strip(",:") in WORDS:
            break
        keep.append(tok)
    rhs = " ".join(keep).strip(" ,:")
    if not re.search(r"[A-Za-z]", lhs + rhs) or not lhs or not rhs:
        return None
    return lhs, rhs


def given_values(question: str) -> dict:
    """'with m = 2 and c = 3' -> {'m': 2.0, 'c': 3.0} (pure numbers only; a unit makes it None)."""
    tail = re.split(r"\b(?:with|where|when|if|given)\b", question, maxsplit=1, flags=re.I)
    if len(tail) < 2:
        return {}
    out = {}
    for name, num, rest in re.findall(r"([A-Za-z][A-Za-z0-9_]*)\s*=\s*([-+]?\d+(?:\.\d+)?(?:[eE][-+]?\d+)?)(\s*[A-Za-z/°%]*)", tail[1]):
        if rest.strip() and rest.strip().lower() not in ("and", "find", "what", "solve", "for", "compute", "calculate"):
            return None                                    # a unit: not this path
        out[name] = float(num)
    return out


def _tex(e) -> str:
    return sp.latex(e)


def isolate(lhs, rhs, x):
    """Steps [(description, equation latex)], the solution and whether it is ± (an even root), isolating x by
    inverse operations; None if x occurs more than once or on both sides (the caller then solves exactly)."""
    steps, pm = [], False
    if rhs.has(x) and not lhs.has(x):
        a, b = rhs, lhs
        steps.append(("Write the equation with the unknown on the left", f"{_tex(a)} = {_tex(b)}"))
    elif lhs.has(x) and not rhs.has(x):
        a, b = lhs, rhs
    else:
        return None
    if sum(1 for s in sp.preorder_traversal(a) if s == x) != 1:
        return None
    while a != x:
        if a.is_Add:
            rest = sp.Add(*[t for t in a.args if not t.has(x)])
            a, b = a - rest, b - rest
            steps.append((f"Subtract ${_tex(rest)}$ from both sides", f"{_tex(a)} = {_tex(sp.simplify(b))}"))
        elif a.is_Mul:
            rest = sp.Mul(*[t for t in a.args if not t.has(x)])
            a, b = sp.Mul(*[t for t in a.args if t.has(x)]), b / rest
            steps.append((f"Divide both sides by ${_tex(rest)}$", f"{_tex(a)} = {_tex(sp.simplify(b))}"))
        elif a.is_Pow and a.base.has(x) and not a.exp.has(x):
            n = a.exp
            even = n.is_integer and n.is_even
            a, b = a.base, b ** (1 / n)
            what = "square root" if n == 2 else ("reciprocal" if n == -1 else f"power ${_tex(1 / n)}$")
            if even:
                pm = True
                steps.append((f"Take the {what} of both sides (both signs: ±)", f"{_tex(a)} = \\pm {_tex(sp.simplify(b))}"))
            else:
                steps.append((f"Take the {what} of both sides", f"{_tex(a)} = {_tex(sp.simplify(b))}"))
        elif a.is_Pow and a.exp.has(x) and not a.base.has(x):
            base = a.base
            a, b = a.exp, sp.log(b) / sp.log(base) if base != sp.E else sp.log(b)
            steps.append(("Take the logarithm of both sides", f"{_tex(a)} = {_tex(sp.simplify(b))}"))
        elif isinstance(a, sp.exp):
            a, b = a.args[0], sp.log(b)
            steps.append(("Take the natural logarithm of both sides", f"{_tex(a)} = {_tex(sp.simplify(b))}"))
        elif isinstance(a, sp.log):
            a, b = a.args[0], sp.exp(b)
            steps.append(("Exponentiate both sides", f"{_tex(a)} = {_tex(sp.simplify(b))}"))
        elif isinstance(a, (sp.sin, sp.cos, sp.tan)):
            inv = {sp.sin: sp.asin, sp.cos: sp.acos, sp.tan: sp.atan}[type(a)]
            a, b = a.args[0], inv(b)
            steps.append((f"Apply {inv.__name__} to both sides (the principal value)", f"{_tex(a)} = {_tex(sp.simplify(b))}"))
        else:
            return None
    sol = sp.simplify(b)
    return steps, sol, pm


def solve_formula(question: str, known: set):
    """{"equation", "target", "steps", "solution", "value", "checked"} or None (not a typed-formula question).
    Raises FormulaError with a reason a user can act on."""
    if "=" not in question or not _CUE.search(question):
        return None
    found = find_equation(question)
    if found is None:
        return None
    lhs_t, rhs_t = found
    trimmed = _trim_formula(lhs_t, rhs_t, known)
    if trimmed is None:
        return None
    lhs_t, rhs_t = trimmed
    try:
        lt, rt = normalise(lhs_t, known), normalise(rhs_t, known)
        names = {n for n in _identifiers(lt) + _identifiers(rt) if n.lower() not in FUNCS and n != "pi"}
        syms = {n: sp.Symbol(n, real=True) for n in names}
        lhs, rhs = parse(lt, syms, symbolic_powers=True), parse(rt, syms, symbolic_powers=True)
    except ExpressionError as exc:
        raise FormulaError(f"I could not read the formula ({exc}); write it with *, / and ^ (e.g. E = m*c^2)") from exc
    # "x = 85" or "alpha = 0.05" states a value, it is not a formula: a formula relates at least two quantities
    if len(names) < 2 or (lhs.is_Symbol and rhs.is_number) or (rhs.is_Symbol and lhs.is_number):
        return None
    vals = given_values(question)
    if vals is None:
        return None                                       # values with units: the certified capabilities answer those
    m = _TARGET.search(re.sub(r"\b(?:with|where|when|if|given)\b.*$", "", question, flags=re.I))
    target = m.group(1) if m and m.group(1) in syms else None
    if target is None:
        # the unknown is chosen for the question only when it asks to SOLVE the formula; "find the Gaussian curvature K
        # of z = ..." or "a b = 34 bit" asks for something else, and the lone free symbol was answered instead
        # (tools/ask_sweep.py, 2026-10-23)
        if not _SOLVE_CUE.search(question):
            return None
        free = [n for n in sorted(syms) if n not in vals]
        target = free[0] if len(free) == 1 else None
    if target is None:
        raise FormulaError("say which quantity to solve for (for example 'solve E = m*c^2 for m')")
    x = syms[target]
    iso = isolate(lhs, rhs, x)
    if iso is None:
        sols = [s for s in sp.solve(sp.Eq(lhs, rhs), x) if s.is_real is not False]
        if not sols:
            raise FormulaError(f"this formula has no real solution for {target}")
        steps = [("Collect the terms in the unknown and solve exactly",
                  ", ".join(f"{sp.latex(x)} = {sp.latex(s)}" for s in sols))]
    else:
        steps, sol, pm = iso
        sols = [sol, -sol] if pm else [sol]
    for s in sols:                                        # every solution shown is checked by substitution
        if sp.simplify((lhs - rhs).subs(x, s)) != 0:
            raise FormulaError("the rearranged formula does not check when substituted back, so I will not show it")
    if len(sols) == 2 and iso is not None and iso[2]:
        sol_tex = f"{sp.latex(x)} = \\pm {sp.latex(sols[0])}"
    else:
        sol_tex = ", ".join(f"{sp.latex(x)} = {sp.latex(s)}" for s in sols)
    out = {"equation": f"{sp.latex(lhs)} = {sp.latex(rhs)}", "target": target, "steps": steps,
           "solution": sol_tex, "solutions": [sp.latex(s) for s in sols], "value": None, "values": [], "checked": True}
    if vals:
        sub = {syms[k]: v for k, v in vals.items() if k in syms}
        missing = sorted({f.name for s in sols for f in s.free_symbols if f.name not in vals})
        if missing:
            raise FormulaError("give a value for " + ", ".join(missing))
        nums = []
        for s in sols:
            num = complex(sp.N(s.subs(sub), 15))
            if abs(num.imag) > 1e-12 * max(1.0, abs(num.real)):
                continue                                  # not a real value for these numbers
            v = num.real
            res = complex(sp.N((lhs - rhs).subs({**sub, x: v}), 15))
            if abs(res) > 1e-9 * max(1.0, abs(v), float(sp.N(abs(lhs.subs({**sub, x: v})), 15))):
                raise FormulaError("the computed value does not satisfy the formula, so I will not show it")
            nums.append(v)
        if not nums:
            raise FormulaError(f"there is no real value of {target} for these numbers")
        out["values"] = nums
        out["value"] = nums[0] if len(nums) == 1 else None
        out["substituted"] = ", ".join(f"{sp.latex(x)} = {sp.latex(s.subs({k: sp.Float(v, 6) for k, v in sub.items()}))}" for s in sols)
    return out
