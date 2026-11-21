# GENERATED from ai999x_factory/runtime by tools/build_package.py — do not edit.
"""Typeset the math inside a sentence written as plain text, for notebook (Markdown + LaTeX) output.

    prose_markdown("1.05 <= gamma <= 1.7")              -> "$1.05 \\le \\gamma \\le 1.7$"
    prose_markdown("amounts >= 0 with a positive total") -> "amounts $\\ge 0$ with a positive total"
    prose_markdown("deaths <= cases")                    -> "deaths \u2264 cases"

Assumptions, conditions and warnings are written by each model as short sentences that mix words and math
("E[X^2] >= E[X]^2", "-1000 m <= h <= 47000 m", "0 < zeta < 1/sqrt(2)"). Words stay prose (escaped for Markdown,
with <= >= != +- shown as their symbols); each run of math becomes one inline $...$ span. A run is typeset only when
it really is math (an operator, a power, a subscript, a Greek letter or a function), its brackets balance, and the
LaTeX made from it is well formed; anything else stays readable prose. Nothing is ever dropped.
"""
from __future__ import annotations

import re

BS = "\\"
GREEK = {"alpha", "beta", "gamma", "delta", "epsilon", "eps", "zeta", "eta", "theta", "iota", "kappa", "lambda", "mu",
         "nu", "xi", "pi", "rho", "sigma", "tau", "upsilon", "phi", "chi", "psi", "omega",
         "Gamma", "Delta", "Theta", "Lambda", "Xi", "Pi", "Sigma", "Phi", "Psi", "Omega"}
_GREEK_TEX = {"eps": "epsilon"}
FUNCS = {"sqrt", "exp", "ln", "log", "log10", "sin", "cos", "tan", "asin", "acos", "atan", "sinh", "cosh", "tanh",
         "min", "max", "abs", "arcsin", "arccos", "arctan"}
_PAREN_ONLY = {"min", "max", "abs", "sqrt"}      # function names only when called: "min" alone may be minutes
_FUNC_TEX = {"asin": BS + "arcsin", "acos": BS + "arccos", "atan": BS + "arctan", "log10": BS + "log_{10}",
             "abs": BS + "operatorname{abs}", "sqrt": BS + "sqrt"}
# short words that are English, never a symbol (unless written right next to a math operator: a*x)
STOP = {"a", "I", "an", "as", "at", "be", "by", "do", "if", "in", "is", "it", "no", "of", "on", "or", "so", "to",
        "up", "we", "and", "the", "for", "per", "are", "not", "all", "any", "one", "two", "its", "has", "but", "via",
        "nor", "off", "out", "low", "high", "set", "use", "may", "can", "each", "with", "from", "than", "then", "when",
        "go", "us", "he", "me", "my", "am", "ie", "eg", "ok"}
UNITS = {"m", "km", "cm", "mm", "um", "nm", "K", "s", "ms", "h", "min", "kg", "g", "mol", "Pa", "kPa", "MPa", "bar",
         "atm", "deg", "rad", "Hz", "kHz", "MHz", "GHz", "V", "A", "W", "kW", "MW", "J", "kJ", "N", "kN", "ohm", "dB",
         "L", "mL", "%", "yr", "Myr", "Gyr", "day", "days", "hPa", "dbar", "Mpc", "kpc", "pc", "AU", "mV", "kV", "mA",
         "uF", "nF", "pF", "mH", "uH", "T", "mT", "eV", "keV", "MeV", "ppm", "rpm", "lb", "ft", "degrees", "°C", "°F",
         "mg", "ug", "mmol", "umol", "uL", "dL", "mS", "uS", "Wh", "kWh", "MWh", "GW", "TW", "Gy", "Sv", "Bq"}
_DEGREES = ("deg", "degrees")
# dimensionless groups, written as two-letter names (Reynolds, Prandtl, Nusselt, ...)
DIMLESS = {"Re", "Pr", "Nu", "Ra", "Gr", "Ma", "Sc", "Sh", "Bi", "Fo", "St", "Pe", "Da", "We", "Fr", "Le", "Ec", "Kn",
           "Ri", "Ca", "Bo", "Ja", "Ar", "Ga"}

_TOKEN = re.compile(
    r"(?P<num>\d{1,3}(?:,\d{3})+(?![\d,])(?:\.\d+)?|\d+(?:\.\d+)?(?:[eE][+-]?\d+)?)"   # 10,000 is one number
    r"|(?P<grp>\{[A-Za-z0-9+\-*/ ,]*\})"                          # a written subscript/power group: x_{t-1}, e^{at}
    r"|(?P<op><=|>=|!=|==|\+-|->|[<>=+\-*/^,()\[\]|'])"
    r"|(?P<word>°[CF]|[A-Za-zÀ-ÿ][A-Za-zÀ-ÿ0-9]*(?:_[A-Za-zÀ-ÿ0-9]+)*(?:\*(?![\w(]))?)"   # A* (nothing after) = A^{*}
    r"|(?P<sp>\s+)"
    r"|(?P<other>.)")
_MATH_OPS = {"<=", ">=", "!=", "==", "+-", "<", ">", "=", "^", "*", "+", "/", "|"}
_OP_TEX = {"<=": " " + BS + "le ", ">=": " " + BS + "ge ", "!=": " " + BS + "ne ", "==": " = ", "+-": " " + BS + "pm ",
           "->": " " + BS + "to ", "*": " " + BS + "cdot ", "<": " < ", ">": " > ", "=": " = ", "|": "|"}
_PROSE_OPS = (("<=", "\u2264"), (">=", "\u2265"), ("!=", "\u2260"), ("+-", "\u00b1"))


def _is_symbol(w: str, nxt: str) -> bool:
    """Could this word be a math symbol (not an English word)?"""
    base = w.rstrip("*")
    if base in STOP:
        return False
    if base in GREEK or (base in FUNCS and (nxt == "(" or base in ("ln", "log", "exp", "sin", "cos", "tan"))):
        return True
    if "_" in base or re.fullmatch(r"[A-Za-z]{1,2}\d*", base) or re.fullmatch(r"[A-Z]{1,4}\d*", base):
        return True
    if re.fullmatch(r"[A-Za-z](min|max)", base):                 # Vmin, Tmax
        return True
    return re.sub(r"\d+$", "", base) in GREEK                 # theta1


def _one(p: str) -> str:
    """One name: theta -> \\theta, theta1 -> \\theta_{1}, M1 -> M_{1}, max -> \\max, smax -> \\mathrm{smax}."""
    if p in GREEK:
        return BS + _GREEK_TEX.get(p, p)
    g = re.fullmatch(r"([A-Za-z]+?)(\d+)", p)
    if g and g.group(1) in GREEK:
        return BS + _GREEK_TEX.get(g.group(1), g.group(1)) + "_{" + g.group(2) + "}"
    if p in ("max", "min"):
        return BS + p
    m = re.fullmatch(r"([A-Za-z])(\d+)", p)
    if m:
        return f"{m.group(1)}_{{{m.group(2)}}}"
    if len(p) == 1 or p.isdigit():
        return p
    return BS + "mathrm{" + p + "}"


def _sym_tex(w: str) -> str:
    """theta_max -> \\theta_{\\max}; n_F -> n_{F}; A* -> A^{*}; Mw -> M_{w}; coupon_rate -> \\mathrm{coupon\\_rate}."""
    star = "^{*}" if w.endswith("*") else ""
    w = w.rstrip("*")
    parts = w.split("_")
    # an input name (mean_demand, sd_service_time, coupon_rate) is shown as written; a symbol with a subscript
    # (theta_max, p_bar, n_F, V_th) is typeset as one
    word_parts = [p for p in parts if len(p) >= 3 and p.islower() and p not in GREEK | {"max", "min", "bar", "inf"}]
    if len(parts) > 1 and parts[0].islower() and len(parts[0]) >= 2 and parts[0] not in GREEK and word_parts:
        return BS + "mathrm{" + w.replace("_", BS + "_") + "}" + star
    if len(parts) > 1:
        head, subs = parts[0], [_one(r) for r in parts[1:] if r]
        d = re.fullmatch(r"([A-Za-z]+?)(\d+)", head)              # C0_mgL -> C_{0,mgL}: one subscript, never two
        if d and (len(d.group(1)) == 1 or d.group(1) in GREEK):
            head, subs = d.group(1), [d.group(2)] + subs
        return _one(head) + "_{" + ",".join(subs) + "}" + star
    if w in DIMLESS:                                          # Re, Pr, Nu: one upright name
        return BS + "mathrm{" + w + "}" + star
    mm = re.fullmatch(r"([A-Za-z])(min|max)", w)              # Vmin -> V_{\min}
    if mm:
        return f"{mm.group(1)}_{{{BS}{mm.group(2)}}}" + star
    m = re.fullmatch(r"([A-Z])([a-z])", w)          # capital + letter: Mw -> M_{w}, Td -> T_{d}; sd, cm stay upright
    return (f"{m.group(1)}_{{{m.group(2)}}}" if m and w not in GREEK else _one(w)) + star


def _close_sqrt(s: str) -> str:
    """\\sqrt{(a + b) ... -> \\sqrt{a + b} ... (the matching parenthesis closes the root)."""
    key = BS + "sqrt{("
    out, i = "", 0
    while True:
        j = s.find(key, i)
        if j < 0:
            return out + s[i:]
        out += s[i:j] + BS + "sqrt{"
        k, depth = j + len(key), 1
        while k < len(s) and depth:
            depth += {"(": 1, ")": -1}.get(s[k], 0)
            k += 1
        if depth:                                           # unbalanced: the caller's safety check refuses it
            return out + s[j + len(BS + "sqrt{"):]
        out += s[j + len(key):k - 1] + "}"
        i = k


def _span_tex(toks: list) -> str:
    """LaTeX for one run of math tokens."""
    out, i = [], 0
    while i < len(toks):
        kind, t = toks[i]
        nxt = toks[i + 1][1] if i + 1 < len(toks) else ""
        if kind == "op":
            out.append(_OP_TEX.get(t, t))
        elif kind == "num":
            e = re.fullmatch(r"([\d.]+)[eE]([+-]?)0*(\d+)", t)      # 1.2566e-6 -> 1.2566 \times 10^{-6}
            out.append(f"{e.group(1)} {BS}times 10^{{{'-' if e.group(2) == '-' else ''}{e.group(3)}}}" if e
                       else t.replace(",", "{,}"))                    # 10,000 without a space after the comma
            # a unit straight after a number, compound ones too: 47000 m, 150 K, 90 deg, 70 km/s/Mpc, 2 kg/m^3
            if i + 2 < len(toks) and toks[i + 1][0] == "sp" and toks[i + 2][1] in UNITS \
                    and (i + 3 >= len(toks) or toks[i + 3][1] not in ("(", "_")):
                u, k = toks[i + 2][1], i + 3
                while k + 1 < len(toks):
                    if toks[k][1] in ("/", "*", "^") and (
                            toks[k + 1][1] in UNITS or (toks[k][1] == "^" and toks[k + 1][0] == "num")):
                        u += (BS + "," if toks[k][1] == "*" else toks[k][1]) + toks[k + 1][1]
                        k += 2
                        continue
                    # a bracketed unit group: J/(mol*K), W/(m^2*K^4)
                    if toks[k][1] == "/" and toks[k + 1][1] == "(":
                        e = k + 2
                        while e < len(toks) and (toks[e][1] in UNITS or toks[e][1] in ("*", "^")
                                                 or toks[e][0] == "num"):
                            e += 1
                        if e < len(toks) and toks[e][1] == ")" and e > k + 2:
                            u += "/(" + "".join(BS + "," if t == "*" else t for _, t in toks[k + 2:e]) + ")"
                            k = e + 1
                            continue
                    break
                u = re.sub(r"\^(-?\d+)", r"^{\1}", u).replace("%", BS + "%")
                if u in _DEGREES:
                    out.append("^{" + BS + "circ}")
                elif u in ("°C", "°F"):
                    out.append(BS + " ^{" + BS + "circ}" + BS + "mathrm{" + u[1] + "}")
                else:
                    out.append(BS + " " + BS + "mathrm{" + u + "}")
                i = k
                continue
        elif kind == "word":
            if t in FUNCS and (nxt == "(" or t not in _PAREN_ONLY):     # ln C, exp(x): operator names
                out.append(_FUNC_TEX.get(t, BS + t) + ("" if nxt == "(" else " "))
            else:
                out.append(_sym_tex(t))
        elif kind == "sp":
            out.append(" ")
        elif kind == "grp":
            out.append("{" + t[1:-1].replace("*", " " + BS + "cdot ") + "}")
        elif t == "_":
            out.append("_")
        elif t == "%":
            out.append(BS + "%")
        else:
            out.append({"{": BS + "{", "}": BS + "}", "#": BS + "#", "&": BS + "&", "~": BS + "sim "}.get(t, t))
        i += 1
    s = "".join(out)
    s = re.sub(r"\\sqrt\s*\(", lambda m: BS + "sqrt{(", s)
    s = _close_sqrt(s)
    s = re.sub(r"\^\s*\(([^()]*)\)", r"^{\1}", s)                       # x^(m t) -> x^{m t}
    s = re.sub(r"\^\s*(-?[A-Za-z0-9.]+(?:_\{[^{}]*\})?)", r"^{\1}", s)   # x^2, x^-1, x^p1 -> x^{2}, x^{-1}, x^{p_{1}}
    return re.sub(r"\s+", " ", s).strip()


def _safe(tex: str) -> bool:
    """Typeset only what will render: braces balance, no power or subscript left without its argument."""
    depth = 0
    for c in tex:
        depth += {"{": 1, "}": -1}.get(c, 0)
        if depth < 0:
            return False
    return depth == 0 and "$" not in tex and not re.search(r"[\^_](\s|$|[,)\]=<>+\-/])", tex)


def expr_tex(text: str) -> str | None:
    """A whole value written as an expression ("1 + cos(theta)", "(A | B) & ~C", "p -> q") as LaTeX, or None when it
    cannot be typeset safely (the caller then shows it as text)."""
    toks = [(m.lastgroup, m.group()) for m in _TOKEN.finditer(text)]
    if not toks or sum({"(": 1, ")": -1, "[": 1, "]": -1}.get(t, 0) for _, t in toks) != 0:
        return None
    tex = _span_tex([(k, ("~" if t == "~" else t)) for k, t in toks])
    return tex if _safe(tex) else None


def _md(s: str) -> str:
    """Prose for Markdown: comparison operators as their symbols; emphasis and math characters escaped."""
    for a, b in _PROSE_OPS:
        s = s.replace(a, b)
    for ch in (BS, "*", "_", "$", "`"):
        s = s.replace(ch, BS + ch)
    return s


def prose_markdown(text: str) -> str:
    """The sentence for Markdown: words as prose, each run of math as one $...$ span."""
    toks =[(m.lastgroup, m.group()) for m in _TOKEN.finditer(text)]
    mathy = []
    for i, (k, t) in enumerate(toks):
        nxt = next((u for kk, u in toks[i + 1:] if kk != "sp"), "")
        prv = next((u for kk, u in reversed(toks[:i]) if kk != "sp"), "")
        if k == "word" and t in STOP:
            near_op = nxt in _MATH_OPS - {"|"} or prv in _MATH_OPS - {"|"}
            mathy.append(near_op and len(t) == 1)               # a*x: the single letter is a symbol
        elif k == "word" and i and i + 1 < len(toks) and toks[i - 1][1] == "|" and toks[i + 1][1] == "|":
            mathy.append(True)                                  # |latitude|: an absolute value of a named quantity
        elif k == "word":
            unit_after_num = t in UNITS and i >= 2 and toks[i - 1][0] == "sp" and toks[i - 2][0] == "num"
            unit_chain = t in UNITS and i >= 2 and toks[i - 1][1] in ("/", "*") and toks[i - 2][1] in UNITS \
                and bool(mathy[i - 2])                          # km/s/Mpc: each piece after a unit is a unit
            unit_chain = unit_chain or (t in UNITS and prv in ("(", "*", "/")    # J/(mol*K): inside a unit group
                                        and any(kk == "num" for kk, _ in toks[max(0, i - 8):i]))
            mathy.append(_is_symbol(t, nxt) or unit_after_num or unit_chain)
        elif k == "num":
            mathy.append(True)
        elif k == "op":
            mathy.append(t not in (",", "'") or bool(prv and nxt and prv not in STOP))
        elif k == "grp":
            mathy.append(True)
        elif k == "other" and t == "_":
            mathy.append(i + 1 < len(toks) and toks[i + 1][0] == "grp")
        elif k == "other" and t == "%":
            mathy.append(i > 0 and toks[i - 1][0] == "num")
        else:
            mathy.append(None)                                  # spaces join a run, never start or end one
    out, i = [], 0
    while i < len(toks):
        if not mathy[i]:
            out.append(_md(toks[i][1]))
            i += 1
            continue
        j = i
        while j + 1 < len(toks) and (mathy[j + 1] or (mathy[j + 1] is None and toks[j + 1][0] == "sp"
                                                     and j + 2 < len(toks) and mathy[j + 2])):
            j += 1
        # a run never starts with a closing bracket, a comma or a binary operator: those stay with the prose before it
        if toks[i][1] in (")", "]", ",", "/", "*", "+", "^", "="):
            out.append(_md(toks[i][1]))
            i += 1
            continue
        run = toks[i:j + 1]
        depth = lambda r: sum({"(": 1, ")": -1, "[": 1, "]": -1}.get(t, 0) for _, t in r)  # noqa: E731
        # trailing commas, opening brackets and spaces belong to the prose; an unbalanced run is cut back to its
        # longest balanced start ("c*x^n (n rational)" -> "c*x^n" typeset, "(n rational)" prose)
        while run and (run[-1][1] in (",", "(", "[") or run[-1][0] == "sp" or depth(run) > 0
                       or (depth(run) < 0 and run[-1][1] in (")", "]"))):   # "(at most 2 kg/m^3)": ")" is prose
            run, j = run[:-1], j - 1
        if not run:
            out.append(_md(toks[i][1]))
            i += 1
            continue
        words = [t for k, t in run if k != "sp"]
        single = len(words) == 1
        symbol = single and run[0][0] == "word" and ("_" in run[0][1] or run[0][1].rstrip("*") in GREEK
                                                     or run[0][1].endswith("*"))      # c*, Gi*
        is_math = any(k == "op" and t in _MATH_OPS for k, t in run) or any(
            k == "word" and (t.rstrip("*") in GREEK or "_" in t or t in FUNCS or t.endswith("*")) for k, t in run)
        balanced = sum({"(": 1, ")": -1, "[": 1, "]": -1}.get(t, 0) for _, t in run) == 0
        tex = _span_tex(run)
        if is_math and (not single or symbol) and balanced and _safe(tex):
            out.append("$" + tex + "$")
        else:
            out.append(_md("".join(t for _, t in run)))
        i = j + 1
    return "".join(out)
