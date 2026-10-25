"""Optimization procedures: deterministic algorithms with typed inputs and several outputs (domain code, loaded by the
pack; enabled only once tools/certify_procedures.py has certified them).

Every procedure states its inputs, outputs, conditions, method and LaTeX, the documented question forms it reads,
worked examples, an INDEPENDENT reference computation (scipy, ortools, networkx, nashpy, pymcdm, sympy: used only by
certification, imported inside the reference) and a deliberately wrong variant that the checks must reject.

The shipped code is plain Python: linear programs are solved EXACTLY in rational arithmetic (two-phase simplex,
Bland's rule), and every optimum carries a certificate (a dual solution, potentials, a cut) that `verify` checks.
Where an optimum may be attained by several solutions, the optimal VALUE is the answer; a solution vector is
reported as the solution only when it is proved unique, otherwise it is labelled as one optimal solution.

Fixed conventions: quadratic objectives are f = 1/2 x^T Q x + c^T x and multipliers satisfy Q x + c + A^T lambda = 0;
a minimum cut is the source side reachable in the final residual network; MCDA weights are normalised to sum 1 and
the best alternative on a tie is the first listed; AHP judgments are the upper triangle a12, a13, ..., a23, ...
with Saaty's random indices; an MDP policy takes the lowest-numbered optimal action; a polynomial's argmin/argmax
example is the smallest attaining point. Written edge lists are read sign-preserving here ('B-A -3' is weight -3).
"""
from __future__ import annotations

import heapq
import itertools
import math
import re
from fractions import Fraction as F
from statistics import NormalDist

_STD = NormalDist()


# --- small helpers ------------------------------------------------------------------------------------------------
def _fr(v):
    """The exact rational value of a number as written (0.1 -> 1/10)."""
    if isinstance(v, F):
        return v
    if isinstance(v, bool):
        raise ValueError("expected a number")
    if isinstance(v, int):
        return F(v)
    v = float(v)
    if not math.isfinite(v):
        raise ValueError("every number must be finite")
    return F(repr(v))


def _rel(a, b):
    return abs(a - b) / max(1.0, abs(a), abs(b))


def _rr(a, b):
    """Purely relative difference (a probability of 1e-12 is not 'equal' to 2e-12); exact zeros match below 1e-15."""
    d = abs(a - b)
    return 0.0 if d <= 1e-15 else d / max(abs(a), abs(b))


def _finite(*vals):
    for v in vals:
        if not (isinstance(v, (int, float)) and not isinstance(v, bool) and math.isfinite(v)):
            raise ValueError("every input must be a finite number")


def _fl(x):
    return [float(v) for v in x]


_N = r"[-+]?(?:\d+(?:\.\d+)?|\.\d+)(?:[eE][-+]?\d+)?"

# a negator that governs the word right after it: "no wall", "without a river", "not along a wall", "no wall or river"
# (the second noun of "no X or Y" is negated too), "neither a wall nor a river"; "no fence along the river" negates the
# FENCE, not the river (a word other than an article or preposition stands between)
_NEGATED = re.compile(r"\b(?:no|not|without|neither|nor|never|cannot)\s+(?:(?:a|an|any|the|along|beside|by|against|"
                      r"next\s+to|use|using|of|be|been|is|are|allowed|permitted)\s+){0,3}(?:[a-z]+\s+(?:or|nor)\s+(?:(?:a|an|any|the)\s+)?)?$")


def _affirmed(ql, pattern):
    """True when `pattern` occurs at least once in the lower-cased question WITHOUT a negator governing it ("no wall or
    river" does not affirm a wall). A flag read from a keyword must never be set by its own negation."""
    return any(not _NEGATED.search(ql[max(0, m.start() - 60):m.start()]) for m in re.finditer(pattern, ql))


def _num_after(q, words, pct=False, gap=40):
    """The first number written after one of the phrases `words` (a regex alternation), or None. With pct, '95%'
    and '0.95' both give 0.95."""
    m = re.search(r"(?:" + words + r")[^0-9\-+.]{0,%d}?(%s)\s*(%%|percent)?" % (gap, _N), q)
    if not m:
        return None
    v = float(m.group(1))
    if pct and (m.group(2) or v > 1):
        v = v / 100.0
    return v


def _bracket_groups(q):
    """Every top-level [...] group of a question with the text before it: [(prefix, inner_text)]."""
    out, depth, start = [], 0, None
    for i, ch in enumerate(q):
        if ch == "[":
            if depth == 0:
                start = i
            depth += 1
        elif ch == "]" and depth:
            depth -= 1
            if depth == 0:
                out.append((q[:start], q[start:i + 1]))
    return out


def _br_value(t):
    """'[1, 2, 3]' -> [1.0, 2.0, 3.0]; '[[1, 2], [3, 4]]' or '[1 2; 3 4]' -> rows. A value that is not numbers is
    returned as written, so the input reader refuses it with its reason (never a guess)."""
    inner = t.strip()[1:-1].strip()
    try:
        if inner.startswith("["):
            rows = re.findall(r"\[([^\[\]]*)\]", inner)
            return [[float(x) for x in re.split(r"[,\s]+", r.strip()) if x] for r in rows]
        if ";" in inner:
            return [[float(x) for x in re.split(r"[,\s]+", r.strip()) if x] for r in inner.split(";") if r.strip()]
        return [float(x) for x in re.split(r"[,\s]+", inner) if x]
    except ValueError:
        return inner


def _labelled(q, labels):
    """{name: value} for bracket groups whose preceding words end with one of labels {name: regex}."""
    found = {}
    for prefix, text in _bracket_groups(q):
        tail = prefix.lower()[-60:]
        for name, rx in labels.items():
            if name not in found and re.search(r"(?:" + rx + r")[^\[\]]{0,25}$", tail):
                found[name] = _br_value(text)
                break
    return found


def _edges_text(q):
    """The edge list written after 'edges' (up to the end of the sentence): 'A-B 4, B-C 1'."""
    m = re.search(r"\bedges?\s*(?:list\s*)?:?\s*(.+?)\s*[.?]?\s*$", q, re.S)
    if not m:
        return None
    # read here, sign-preserving: the shared reader splits 'B-A -3' at every '-' and would lose the minus sign
    out = []
    for part in re.split(r"[;,]\s*", m.group(1).strip()):
        if not part.strip():
            continue
        e = re.fullmatch(r"\s*([A-Za-z0-9_]+)\s*-\s*([A-Za-z0-9_]+)(?:\s+(" + _N + r"))?\s*", part)
        if not e:
            return [("unreadable edge list",)]              # refused by the input reader (never guessed)
        out.append((e.group(1), e.group(2), float(e.group(3))) if e.group(3) is not None else (e.group(1), e.group(2)))
    return out or None


def _I(sym, meaning, typ="number", required=True, **kw):
    d = {"symbol": sym, "kind": "number", "meaning": meaning, "required": required}
    if typ != "number":
        d["type"] = typ
    d.update(kw)
    return d


def _O(sym, meaning, kind="number"):
    return {"symbol": sym, "kind": kind, "meaning": meaning}


PROCEDURES: dict = {}


def _add(key, **spec):
    spec.setdefault("conditions", [])
    spec.setdefault("questions", [])
    PROCEDURES[key] = spec


# --- exact linear programming: two-phase simplex with Bland's rule, in rational arithmetic -------------------------
def _simplex_eq(A, b, c, max_iter=20000):
    """min c.x s.t. A x = b, x >= 0 (lists of Fractions). Returns (status, x, y, basis, redundant) where y is an
    optimal dual (c - A^T y >= 0, b.y = c.x) and redundant counts equality rows dropped as linear combinations."""
    m, n = len(A), len(c)
    sign = [1] * m
    A = [list(r) for r in A]
    b = list(b)
    for i in range(m):
        if b[i] < 0:
            A[i], b[i], sign[i] = [-v for v in A[i]], -b[i], -1
    T = [A[i] + [F(int(k == i)) for k in range(m)] + [b[i]] for i in range(m)]
    basis = [n + i for i in range(m)]
    N = n + m

    def pivot(r, col):
        pv = T[r][col]
        T[r] = [v / pv for v in T[r]]
        for i in range(len(T)):
            if i != r and T[i][col] != 0:
                f = T[i][col]
                T[i] = [a - f * bb for a, bb in zip(T[i], T[r])]
        basis[r] = col

    def phase(cost, allowed):
        for _ in range(max_iter):
            cb = [cost[j] for j in basis]
            enter = None
            inb = set(basis)
            for j in range(allowed):
                if j not in inb:
                    rc = cost[j] - sum(cb[i] * T[i][j] for i in range(len(T)) if T[i][j] != 0)
                    if rc < 0:
                        enter = j
                        break
            if enter is None:
                return "optimal"
            best = None
            for i in range(len(T)):
                if T[i][enter] > 0:
                    ratio = T[i][-1] / T[i][enter]
                    if best is None or ratio < best[0] or (ratio == best[0] and basis[i] < basis[best[1]]):
                        best = (ratio, i)
            if best is None:
                return "unbounded"
            pivot(best[1], enter)
        raise ValueError("the simplex method did not finish within its iteration limit")

    phase([F(0)] * n + [F(1)] * m, N)
    if sum(T[i][-1] for i in range(len(T)) if basis[i] >= n) > 0:
        return "infeasible", None, None, None, 0
    redundant = 0
    i = 0
    while i < len(T):
        if basis[i] >= n:
            col = next((j for j in range(n) if T[i][j] != 0), None)
            if col is None:
                T.pop(i)
                basis.pop(i)
                redundant += 1
                continue
            pivot(i, col)
        i += 1
    cost = list(c) + [F(0)] * m
    if phase(cost, n) == "unbounded":
        return "unbounded", None, None, None, redundant
    x = [F(0)] * n
    for i, j in enumerate(basis):
        x[j] = T[i][-1]
    cb = [cost[j] for j in basis]
    y = [sign[k] * sum(cb[i] * T[i][n + k] for i in range(len(T))) for k in range(m)]
    return "optimal", x, y, list(basis), redundant


def _standard(sense, c, A, b, rel):
    """min-form standard equality data of: optimise c.x s.t. A_i x (rel_i) b_i, x >= 0."""
    m, n = len(A), len(c)
    slack_cols = [i for i in range(m) if rel[i] in ("<=", ">=")]
    As = []
    for i in range(m):
        row = [_fr(v) for v in A[i]]
        for k in slack_cols:
            row.append(F(0) if k != i else (F(1) if rel[i] == "<=" else F(-1)))
        As.append(row)
    cs = [(_fr(v) if sense == "minimize" else -_fr(v)) for v in c] + [F(0)] * len(slack_cols)
    return As, [_fr(v) for v in b], cs


def _check_lp(sense, c, A, b, rel):
    if sense not in ("maximize", "minimize"):
        raise ValueError("the objective sense must be maximize or minimize")
    n, m = len(c), len(A)
    if n == 0 or m == 0:
        raise ValueError("give at least one variable and one constraint")
    if any(len(r) != n for r in A):
        raise ValueError(f"every constraint row must have {n} coefficients (one per variable)")
    if len(b) != m:
        raise ValueError(f"the right-hand side must have {m} entries (one per constraint)")
    if len(rel) != m or any(r not in ("<=", ">=", "=") for r in rel):
        raise ValueError("each constraint relation must be <=, >= or =")
    if n > 12 or m > 12:
        raise ValueError("at most 12 variables and 12 constraints")


def _relations(text, m):
    if text is None or str(text).strip() == "":
        return ["<="] * m
    toks = [t for t in re.split(r"[,\s;]+", str(text).replace("≤", "<=").replace("≥", ">=").strip()) if t]
    toks = [{"=<": "<=", "=>": ">=", "==": "="}.get(t, t) for t in toks]
    if len(toks) == 1 and m > 1:
        toks = toks * m
    return toks


def _lp(sense, c, A, b, rel, certify=True):
    """Exact solution of a linear program with x >= 0. Raises ValueError when infeasible or unbounded."""
    _check_lp(sense, c, A, b, rel)
    As, bs, cs = _standard(sense, c, A, b, rel)
    status, x, y, basis, redundant = _simplex_eq(As, bs, cs)
    if status == "infeasible":
        raise ValueError("the constraints are infeasible: no point satisfies all of them")
    if status == "unbounded":
        raise ValueError("the objective is unbounded on the feasible region: there is no finite optimum")
    n, m = len(c), len(A)
    zmin = sum(ci * xi for ci, xi in zip(cs, x))
    out = {"value": zmin if sense == "minimize" else -zmin, "x": x[:n],
           "p": [yi if sense == "minimize" else -yi for yi in y]}
    if certify:
        N = len(cs)
        nonbasic = [j for j in range(N) if j not in basis]
        st, xa, _, _, _ = _simplex_eq(As + [cs], bs + [zmin], [F(-1) if j in nonbasic else F(0) for j in range(N)])
        out["x_unique"] = st == "optimal" and sum(xa[j] for j in nonbasic) == 0
        if redundant:
            out["p_unique"] = False
        else:
            rows = []                                   # variables u (m), v (m), r (N)
            for j in range(N):
                rows.append([As[i][j] for i in range(m)] + [-As[i][j] for i in range(m)] + [F(int(k == j)) for k in range(N)])
            rows.append(bs + [-v for v in bs] + [F(0)] * N)
            cost = [F(0)] * (2 * m) + [F(-1) if j in basis else F(0) for j in range(N)]
            st, sol, _, _, _ = _simplex_eq(rows, cs + [zmin], cost)
            out["p_unique"] = st == "optimal" and sum(sol[2 * m + j] for j in basis) == 0
    return out


def _lp_certificate_residual(inputs, x, p, value):
    """Residual of an LP optimality certificate: primal feasibility, dual feasibility and equal objectives."""
    sense, c, A, b = inputs["sense"], inputs["c"], inputs["A"], inputs["b"]
    rel = _relations(inputs.get("relations"), len(A))
    if len(p) != len(A) or len(x) != len(c):
        return 1.0                                       # a certificate of the wrong shape certifies nothing
    s = 1.0 if sense == "minimize" else -1.0
    cm = [s * v for v in c]
    y = [s * v for v in p]
    scale = max([1.0] + [abs(v) for v in c] + [abs(v) for v in b] + [abs(v) for r in A for v in r])
    xs = max([1.0] + [abs(v) for v in x])
    worst = 0.0
    for i, row in enumerate(A):
        ax = sum(a * xi for a, xi in zip(row, x))
        viol = {"<=": max(0.0, ax - b[i]), ">=": max(0.0, b[i] - ax), "=": abs(ax - b[i])}[rel[i]]
        worst = max(worst, viol / (scale * xs))
        sv = {"<=": max(0.0, y[i]), ">=": max(0.0, -y[i]), "=": 0.0}[rel[i]]
        worst = max(worst, sv / scale)
    worst = max([worst] + [max(0.0, -xi) / xs for xi in x])
    for j in range(len(c)):
        rc = cm[j] - sum(A[i][j] * y[i] for i in range(len(A)))
        worst = max(worst, max(0.0, -rc) / (scale * max(1.0, max(abs(v) for v in y) if y else 1.0)))
    zx = sum(ci * xi for ci, xi in zip(c, x))
    zy = s * sum(bi * yi for bi, yi in zip(b, y))
    big = max(1.0, abs(value), abs(zx), abs(zy))
    return max(worst, abs(zx - value) / big, abs(zy - value) / big)


def _lp_outputs(r):
    out = {"optimal_value": float(r["value"])}
    out["x" if r["x_unique"] else "x_one_optimal"] = _fl(r["x"])
    out["solution_unique"] = "yes" if r["x_unique"] else "no"
    out["shadow_prices" if r["p_unique"] else "dual_one_optimal"] = _fl(r["p"])
    return out


def _out_x(out):
    return out.get("x", out.get("x_one_optimal"))


def _out_p(out):
    return out.get("shadow_prices", out.get("dual_one_optimal"))


def linear_program(sense, c, A, b, relations=None):
    rel = _relations(relations, len(A))
    return _lp_outputs(_lp(sense, c, A, b, rel))


def verify_lp(inputs, outputs):
    x, p = _out_x(outputs), _out_p(outputs)
    if x is None or p is None:
        return 1.0
    return _lp_certificate_residual(inputs, x, p, outputs["optimal_value"])


def _scipy_lp(sense, c, A, b, rel, integer=False):
    import numpy as np
    from scipy.optimize import linprog, milp, LinearConstraint, Bounds
    c = np.array(c, float)
    cm = c if sense == "minimize" else -c
    A = np.array(A, float)
    b = np.array(b, float)
    lo = np.array([b[i] if r in (">=", "=") else -np.inf for i, r in enumerate(rel)])
    hi = np.array([b[i] if r in ("<=", "=") else np.inf for i, r in enumerate(rel)])
    if integer:
        res = milp(cm, constraints=LinearConstraint(A, lo, hi), integrality=np.ones(len(c)),
                   bounds=Bounds(0, np.inf), options={"mip_rel_gap": 0, "presolve": True})
        if res.status != 0:
            raise ValueError("milp: " + str(res.message))
        x = np.round(res.x)
        return float(c @ x)
    ub = [i for i, r in enumerate(rel) if r != "="]
    A_ub = np.array([A[i] * (1 if rel[i] == "<=" else -1) for i in ub]) if ub else None
    b_ub = np.array([b[i] * (1 if rel[i] == "<=" else -1) for i in ub]) if ub else None
    eq = [i for i, r in enumerate(rel) if r == "="]
    res = linprog(cm, A_ub=A_ub, b_ub=b_ub, A_eq=A[eq] if eq else None, b_eq=b[eq] if eq else None,
                  bounds=[(0, None)] * len(c), method="highs")
    if res.status != 0:
        raise ValueError("linprog: " + str(res.message))
    return float(c @ res.x), res


def reference_lp(sense, c, A, b, relations=None):
    """INDEPENDENT reference: scipy.optimize.linprog (HiGHS)."""
    return {"optimal_value": _scipy_lp(sense, c, A, b, _relations(relations, len(A)))[0]}


def wrong_lp(sense, c, A, b, relations=None):
    """A deliberately wrong variant: optimises in the opposite sense."""
    other = "minimize" if sense == "maximize" else "maximize"
    try:
        r = _lp(other, c, A, b, _relations(relations, len(A)), certify=False)
    except ValueError:
        r = {"value": F(0), "x": [F(0)] * len(c), "p": [F(0)] * len(A)}
    return {"optimal_value": float(r["value"]), "x": _fl(r["x"]), "shadow_prices": _fl(r["p"])}


def _random_lp(rng, nvar=None, mixed=True):
    n = nvar or rng.randint(2, 5)
    m = rng.randint(2, 5)
    kind = rng.choice(["max", "min", "mixed"] if mixed else ["max", "min"])
    if kind == "max":
        A = [[rng.randint(0, 9) for _ in range(n)] for _ in range(m)]
        A.append([rng.randint(1, 5) for _ in range(n)])
        b = [rng.randint(10, 120) for _ in A]
        c = [rng.randint(1, 20) for _ in range(n)]
        return {"sense": "maximize", "c": c, "A": A, "b": b, "relations": ",".join(["<="] * len(A))}
    if kind == "min":
        A = [[rng.randint(0, 9) for _ in range(n)] for _ in range(m)]
        for r in A:
            if not any(r):
                r[rng.randrange(n)] = rng.randint(1, 9)
        b = [rng.randint(5, 80) for _ in A]
        c = [rng.randint(1, 20) for _ in range(n)]
        return {"sense": "minimize", "c": c, "A": A, "b": b, "relations": ",".join([">="] * len(A))}
    x0 = [rng.randint(0, 6) for _ in range(n)]
    A, b, rel = [], [], []
    for _ in range(m):
        row = [rng.randint(-4, 9) for _ in range(n)]
        r = rng.choice(["<=", "<=", ">="] + ([] if nvar else ["="]))
        ax = sum(a * v for a, v in zip(row, x0))
        A.append(row)
        rel.append(r)
        b.append(ax + rng.randint(0, 15) if r == "<=" else (ax - rng.randint(0, 15) if r == ">=" else ax))
    A.append([1] * n)
    b.append(sum(x0) + rng.randint(1, 20))
    rel.append("<=")
    c = [rng.randint(-10, 20) for _ in range(n)]
    return {"sense": rng.choice(["maximize", "minimize"]), "c": c, "A": A, "b": b, "relations": ",".join(rel)}


# --- reading a linear program written in words ---------------------------------------------------------------------
_VAR = r"[a-z](?:_?\d{1,2})?"


def _lin_expr(s):
    s = s.replace("−", "-").replace("*", "").replace("·", "").replace(" ", "")
    if not s:
        return None
    if s[0] not in "+-":
        s = "+" + s
    terms = re.findall(r"[+-][^+-]+", s)
    if "".join(terms) != s:
        return None
    out = {}
    for t in terms:
        m = re.fullmatch(r"([+-])(\d+(?:\.\d+)?(?:/\d+)?)?(" + _VAR + ")", t)
        if not m:
            return None
        coef = F(m.group(2)) if m.group(2) else F(1)
        out[m.group(3)] = out.get(m.group(3), F(0)) + (-coef if m.group(1) == "-" else coef)
    return out


def _read_lp(question):
    """'Maximize 3x + 5y subject to x + 2y <= 14, 3x - y >= 0, x, y >= 0.' -> LP inputs and variable names, a
    dict with _out_of_scope, or None. Non-negativity must be stated for every variable."""
    q = question.lower().replace("≤", "<=").replace("≥", ">=").replace("=<", "<=").replace("=>", ">=").replace("−", "-")
    m = re.search(r"\b(maximi[sz]e|minimi[sz]e)\b\s*:?\s*(?:[a-z]\s*=\s*)?(.+?)\s*(?:,\s*)?(?:subject to|s\.\s?t\.|such that|"
                  r"with constraints|under the constraints|constrained by)\s*:?\s*(.+?)\s*[.?]?\s*$", q, re.S)
    if not m:
        return None
    sense = "maximize" if m.group(1).startswith("max") else "minimize"
    obj = _lin_expr(m.group(2))
    if obj is None:
        return None
    body = re.sub(r"\b(?:and\s+)?(?:all\s+)?(?:(?:the\s+)?variables\s+)?(?:are\s+|is\s+)?(?:integers?|integer[- ]valued|whole numbers?)\b",
                  "", m.group(3))
    nonneg = set()
    allnn = re.search(r"\ball (?:the )?variables (?:are )?non-?negative\b|\ball variables\s*>=\s*0\b", body)
    body = re.sub(r"\ball (?:the )?variables (?:are )?non-?negative\b|\ball variables\s*>=\s*0\b", "", body)

    def grab(mm):
        nonneg.update(v.strip() for v in re.split(r",|\band\b", mm.group(1)) if v.strip())
        return ""
    body = re.sub(r"((?:" + _VAR + r")(?:\s*(?:,|\band\b)\s*" + _VAR + r")+)\s*>=\s*0(?![\d.])", grab, body)
    body = re.sub(r"((?:" + _VAR + r")(?:\s*(?:,|\band\b)\s*" + _VAR + r")*)\s+(?:are |is )?non-?negative\b", grab, body)
    parts = [p.strip() for p in re.split(r",|;|\band\b", body) if p.strip()]
    rows = []
    for p in parts:
        mm = re.fullmatch(r"(.+?)\s*(<=|>=|=)\s*(" + _N + r"(?:/\d+)?)", p)
        if not mm:
            return None
        lhs = _lin_expr(mm.group(1))
        if lhs is None:
            return None
        rhs = F(mm.group(3))
        if mm.group(2) == ">=" and rhs == 0 and len(lhs) == 1 and list(lhs.values())[0] > 0:
            nonneg.add(list(lhs)[0])
            continue
        rows.append((lhs, mm.group(2), rhs))
    names = []
    for d in [obj] + [r[0] for r in rows]:
        for v in d:
            if v not in names:
                names.append(v)
    if not rows:
        return None
    if not allnn and not set(names) <= nonneg:
        return {"_out_of_scope": "variables without a stated sign (state x >= 0 for every variable; free variables are not supported)"}
    c = [float(obj.get(v, 0)) for v in names]
    A = [[float(r[0].get(v, 0)) for v in names] for r in rows]
    return {"sense": sense, "c": c, "A": A, "b": [float(r[2]) for r in rows],
            "relations": ",".join(r[1] for r in rows)}, names


def _structured_lp(q):
    """'... with objective [3, 5], constraints [[1, 2], [3, -1]], right-hand side [14, 0] ...'"""
    g = _labelled(q, {"c": r"objective(?: coefficients)?|c\s*=", "A": r"constraint matrix|constraints|matrix|a\s*=",
                      "b": r"right-hand sides?|right hand sides?|rhs|b\s*="})
    if not {"c", "A", "b"} <= set(g):
        return None
    ql = q.lower()
    sense = "maximize" if re.search(r"\bmaximi[sz]", ql) else ("minimize" if re.search(r"\bminimi[sz]", ql) else None)
    out = dict(g)
    if sense:
        out["sense"] = sense
    m = re.search(r"relations?\s*:?\s*\(?((?:<=|>=|=)(?:\s*,\s*(?:<=|>=|=))*)", ql.replace("≤", "<=").replace("≥", ">="))
    if m:
        out["relations"] = m.group(1)
    return out


def _lp_question(q, need_words=None, forbid=r"\b(integer|whole[- ]number|graphical(?:ly)?|corner[- ]points?|vertex|vertices|shadow prices?|dual values?|knapsack|quadratic|hessian)\b"):
    ql = q.lower()
    if need_words and not re.search(need_words, ql):
        return None
    if forbid and re.search(forbid, ql):
        return None
    r = _read_lp(q)
    if r is None:
        return _structured_lp(q) if re.search(r"\b(linear program|lp|maximi[sz]e|minimi[sz]e)\b", ql) else None
    if "_out_of_scope" in r:
        return r
    return r[0]


_LP_IN = [_I("sense", "maximize or minimize", "choice", choices=["maximize", "minimize"]),
          _I("c", "objective coefficients, one per variable", "vector", min_length=1, max_length=12),
          _I("A", "constraint coefficients, one row per constraint", "matrix"),
          _I("b", "right-hand sides, one per constraint", "vector", min_length=1, max_length=12),
          _I("relations", "relation of each constraint: <=, >= or = (default all <=)", "text", required=False)]

_add("linear_program",
     name="Linear program (simplex method)", topic="linear programming",
     purpose="Optimise a linear objective subject to linear constraints with non-negative variables: the optimal value, "
             "an optimal solution (stated as unique only when it is) and the shadow prices.",
     inputs=_LP_IN,
     outputs=[_O("optimal_value", "optimal value of the objective"),
              _O("x", "the optimal solution, variables in order of first appearance (only when unique; otherwise x_one_optimal)"),
              _O("solution_unique", "yes when the optimal solution is unique", "text"),
              _O("shadow_prices", "change of the optimal value per unit increase of each right-hand side (only when unique; otherwise dual_one_optimal)")],
     primary="optimal_value",
     conditions=["all variables >= 0", "the LP is feasible and bounded (otherwise INVALID_INPUT says which)", "at most 12 variables and constraints"],
     method="two-phase simplex in exact rational arithmetic with Bland's anti-cycling rule; optimality certified by a dual solution "
            "(primal feasibility, dual feasibility, equal objective values); uniqueness proved by an auxiliary LP over the optimal face",
     latex=r"\max / \min \; c^\top x \quad \text{s.t. } A x \;(\le, \ge, =)\; b,\; x \ge 0",
     run=linear_program, verify=verify_lp, parse=lambda q: _lp_question(q), reference=reference_lp, wrong=wrong_lp,
     random_inputs=lambda rng: _random_lp(rng),
     keywords=["linear program", "simplex", "maximize", "minimize", "subject to"],
     templates=["Maximize 3x + 5y subject to x <= 4, 2y <= 12, 3x + 2y <= 18, x, y >= 0.",
                "Minimize 2x1 + 3x2 subject to x1 + x2 >= 4, x1 + 3x2 >= 6, x1, x2 >= 0.",
                "Solve the linear program: maximize with objective [3, 5], constraint matrix [[1, 0], [0, 2], [3, 2]], right-hand side [4, 12, 18]."],
     examples=[{"inputs": {"sense": "maximize", "c": "3, 5", "A": "1 0; 0 2; 3 2", "b": "4, 12, 18"},
                "outputs": {"optimal_value": 36.0, "x": [2.0, 6.0], "solution_unique": "yes"}},
               {"inputs": {"sense": "minimize", "c": [2, 3], "A": [[1, 1], [1, 3]], "b": [4, 6], "relations": ">=, >="},
                "outputs": {"optimal_value": 9.0, "x": [3.0, 1.0]}},
               {"inputs": {"sense": "maximize", "c": [2, 4], "A": [[1, 2], [1, 0]], "b": [8, 5]},
                "outputs": {"optimal_value": 16.0, "solution_unique": "no"}}],
     questions=[{"q": "Maximize 3x + 5y subject to x <= 4, 2y <= 12, 3x + 2y <= 18, x, y >= 0.", "outputs": {"optimal_value": 36.0, "x": [2.0, 6.0]}},
                {"q": "Minimize 2x1 + 3x2 subject to x1 + x2 >= 4, x1 + 3x2 >= 6, x1, x2 >= 0.", "outputs": {"optimal_value": 9.0}},
                {"q": "Maximize x + y subject to x - y >= 1, x, y >= 0.", "outcome": "INVALID_INPUT"},
                {"q": "Maximize x + y subject to x + y <= 2, x + y >= 5, x, y >= 0.", "outcome": "INVALID_INPUT"},
                {"q": "Maximize 3x + 5y subject to x + 2y <= 14.", "outcome": "OUT_OF_SCOPE"}])


# --- two variables: the graphical (corner-point) method -----------------------------------------------------------
def _lp2_vertices(c, A, b, rel):
    lines = [(F(row[0]), F(row[1]), F(bi)) for row, bi in zip([[_fr(v) for v in r] for r in A], [_fr(v) for v in b])]
    lines += [(F(1), F(0), F(0)), (F(0), F(1), F(0))]
    cons = [(_fr(r[0]), _fr(r[1]), _fr(bi), rl) for r, bi, rl in zip(A, b, rel)]

    def feasible(x, y):
        if x < 0 or y < 0:
            return False
        for a1, a2, bb, rl in cons:
            v = a1 * x + a2 * y
            if (rl == "<=" and v > bb) or (rl == ">=" and v < bb) or (rl == "=" and v != bb):
                return False
        return True
    pts = set()
    for (a1, a2, b1), (c1, c2, b2) in itertools.combinations(lines, 2):
        det = a1 * c2 - a2 * c1
        if det == 0:
            continue
        x, y = (b1 * c2 - a2 * b2) / det, (a1 * b2 - b1 * c1) / det
        if feasible(x, y):
            pts.add((x, y))
    # extreme rays of the recession cone {d >= 0, a.d (rel) 0}: on an axis or along a constraint line
    rays = []
    for d in [(F(1), F(0)), (F(0), F(1))] + [(a2, -a1) for a1, a2, _, _ in cons] + [(-a2, a1) for a1, a2, _, _ in cons]:
        if d == (0, 0) or d[0] < 0 or d[1] < 0:
            continue
        ok = all(not ((rl == "<=" and a1 * d[0] + a2 * d[1] > 0) or (rl == ">=" and a1 * d[0] + a2 * d[1] < 0)
                      or (rl == "=" and a1 * d[0] + a2 * d[1] != 0)) for a1, a2, _, rl in cons)
        if ok:
            rays.append(d)
    return sorted(pts), rays


def lp_two_variable(sense, c, A, b, relations=None):
    rel = _relations(relations, len(A))
    if len(c) != 2:
        raise ValueError("the graphical method takes exactly 2 variables")
    _check_lp(sense, c, A, b, rel)
    pts, rays = _lp2_vertices(c, A, b, rel)
    if not pts:
        raise ValueError("the constraints are infeasible: no point satisfies all of them")
    s = 1 if sense == "maximize" else -1
    c0, c1 = _fr(c[0]), _fr(c[1])
    if any(s * (c0 * d[0] + c1 * d[1]) > 0 for d in rays):
        raise ValueError("the objective is unbounded on the feasible region: there is no finite optimum")
    vals = [(c0 * x + c1 * y, (x, y)) for x, y in pts]
    best = max(v for v, _ in vals) if s == 1 else min(v for v, _ in vals)
    opt = [p for v, p in vals if v == best]
    flat_ray = any(c0 * d[0] + c1 * d[1] == 0 for d in rays)
    out = {"optimal_value": float(best), "optimal_corners": [[float(x), float(y)] for x, y in opt],
           "corner_points": [[float(x), float(y)] for x, y in pts]}
    if len(opt) == 1 and not flat_ray:
        out["x"], out["y"], out["solution_unique"] = float(opt[0][0]), float(opt[0][1]), "yes"
    else:
        out["solution_unique"] = "no"
    return out


def verify_lp2(inputs, outputs):
    c, A, b = inputs["c"], inputs["A"], inputs["b"]
    rel = _relations(inputs.get("relations"), len(A))
    try:
        pts, _ = _lp2_vertices(c, A, b, rel)
    except Exception:
        return 1.0
    vals = [float(c[0] * x + c[1] * y) for x, y in pts]
    if not vals:
        return 1.0
    best = max(vals) if inputs["sense"] == "maximize" else min(vals)
    worst = _rel(best, outputs["optimal_value"])
    for x, y in outputs.get("optimal_corners", []):
        worst = max(worst, _rel(c[0] * x + c[1] * y, outputs["optimal_value"]))
        for row, bi, rl in zip(A, b, rel):
            v = row[0] * x + row[1] * y
            viol = {"<=": max(0.0, v - bi), ">=": max(0.0, bi - v), "=": abs(v - bi)}[rl]
            worst = max(worst, viol / max(1.0, abs(bi)))
    return worst


def _lp2_question(q):
    ql = q.lower()
    if not re.search(r"\b(graphical(?:ly)?|corner[- ]points?|vertex|vertices)\b", ql) or re.search(r"\binteger\b", ql):
        return None
    r = _read_lp(q)
    if r is None:
        return None
    if "_out_of_scope" in r:
        return r
    if len(r[1]) != 2:
        return {"_out_of_scope": "the graphical method takes exactly two variables (use the simplex method)"}
    return r[0]


_add("lp_two_variable_graphical",
     name="Two-variable linear program (graphical corner-point method)", topic="linear programming",
     purpose="Solve a linear program in two variables by evaluating the objective at every corner point of the feasible region.",
     inputs=_LP_IN,
     outputs=[_O("optimal_value", "optimal value of the objective"),
              _O("x", "first variable at the optimum (only when the optimum is unique)"),
              _O("y", "second variable at the optimum (only when the optimum is unique)"),
              _O("optimal_corners", "every corner point at which the optimum is attained"),
              _O("corner_points", "every corner point of the feasible region"),
              _O("solution_unique", "yes when exactly one point is optimal", "text")],
     primary="optimal_value", conditions=["exactly two variables, both >= 0", "a feasible region and a finite optimum"],
     method="intersect every pair of boundary lines (exact fractions), keep the feasible corners, evaluate the objective; "
            "unboundedness detected from the extreme rays of the recession cone",
     latex=r"z^* = \max_{v \in \text{corners}} c^\top v",
     run=lp_two_variable, verify=verify_lp2, parse=_lp2_question, reference=reference_lp, wrong=wrong_lp,
     random_inputs=lambda rng: _random_lp(rng, nvar=2),
     keywords=["graphical method", "corner point", "feasible region"],
     templates=["Use the graphical method to maximize 3x + 5y subject to x <= 4, 2y <= 12, 3x + 2y <= 18, x, y >= 0.",
                "Find the corner points and minimize 2x + 3y subject to x + y >= 4, x + 3y >= 6, x, y >= 0."],
     examples=[{"inputs": {"sense": "maximize", "c": [3, 5], "A": [[1, 0], [0, 2], [3, 2]], "b": [4, 12, 18]},
                "outputs": {"optimal_value": 36.0, "x": 2.0, "y": 6.0}},
               {"inputs": {"sense": "minimize", "c": [2, 3], "A": [[1, 1], [1, 3]], "b": [4, 6], "relations": ">=,>="},
                "outputs": {"optimal_value": 9.0, "x": 3.0, "y": 1.0}}],
     questions=[{"q": "Use the graphical method to maximize 3x + 5y subject to x <= 4, 2y <= 12, 3x + 2y <= 18, x, y >= 0.",
                 "outputs": {"optimal_value": 36.0, "x": 2.0, "y": 6.0}},
                {"q": "Graphically maximize 2x + 4y subject to x + 2y <= 8, x <= 5, x, y >= 0.", "outputs": {"optimal_value": 16.0, "solution_unique": "no"}},
                {"q": "Use the graphical method to maximize x + y + z subject to x + y + z <= 3, x, y, z >= 0.", "outcome": "OUT_OF_SCOPE"}])


# --- integer linear programming: branch and bound on exact LP relaxations ---------------------------------------
def integer_program(sense, c, A, b, relations=None, _last=False):
    rel = _relations(relations, len(A))
    _check_lp(sense, c, A, b, rel)
    n = len(c)
    best = [None, None]
    nodes = [0]
    better = (lambda u, v: u > v) if sense == "maximize" else (lambda u, v: u < v)

    def solve(extraA, extrab, extrar):
        nodes[0] += 1
        if nodes[0] > 4000:
            raise ValueError("branch and bound exceeded 4000 nodes; this problem is too large for this capability")
        try:
            r = _lp(sense, c, list(A) + extraA, list(b) + extrab, rel + extrar, certify=False)
        except ValueError as exc:
            if "unbounded" in str(exc) and not extraA:
                raise ValueError("the LP relaxation is unbounded; this capability needs a bounded relaxation") from None
            if "unbounded" in str(exc):
                raise
            return
        if best[0] is not None and not better(r["value"], best[0]):
            return
        fracs = [j for j in range(n) if r["x"][j].denominator != 1]
        frac = (fracs[-1] if _last else fracs[0]) if fracs else None
        if frac is None:
            best[0], best[1] = r["value"], r["x"]
            return
        v = r["x"][frac]
        row = [0] * n
        row[frac] = 1
        fl = v.numerator // v.denominator
        down = (extraA + [row], extrab + [fl], extrar + ["<="])
        up = (extraA + [row], extrab + [fl + 1], extrar + [">="])
        for br in ((up, down) if _last else (down, up)):
            solve(*br)
    solve([], [], [])
    if best[0] is None:
        raise ValueError("there is no integer point satisfying all the constraints")
    return {"optimal_value": float(best[0]), "x_one_optimal": _fl(best[1]), "nodes": nodes[0]}


_IP_MEMO: dict = {}


def verify_ip(inputs, outputs):
    c, A, b = inputs["c"], inputs["A"], inputs["b"]
    rel = _relations(inputs.get("relations"), len(A))
    x = outputs.get("x_one_optimal")
    if x is None:
        return 1.0
    worst = max(abs(v - round(v)) for v in x)
    worst = max(worst, max(0.0, -min(x)))
    for row, bi, rl in zip(A, b, rel):
        v = sum(a * xi for a, xi in zip(row, x))
        worst = max(worst, {"<=": max(0.0, v - bi), ">=": max(0.0, bi - v), "=": abs(v - bi)}[rl] / max(1.0, abs(bi)))
    worst = max(worst, _rel(sum(ci * xi for ci, xi in zip(c, x)), outputs["optimal_value"]))
    key = repr((inputs["sense"], c, A, b, inputs.get("relations")))
    if key not in _IP_MEMO:  # optimality: a second branch and bound with the opposite branching order must agree
        try:
            _IP_MEMO[key] = integer_program(inputs["sense"], c, A, b, inputs.get("relations"), _last=True)["optimal_value"]
        except ValueError:
            _IP_MEMO[key] = None
        if len(_IP_MEMO) > 256:
            _IP_MEMO.pop(next(iter(_IP_MEMO)))
    again = _IP_MEMO[key]
    if again is None:
        return 1.0
    return max(worst, _rel(again, outputs["optimal_value"]))


def wrong_ip(sense, c, A, b, relations=None):
    """A plausible mistake: report the LP relaxation (integrality ignored)."""
    r = _lp(sense, c, A, b, _relations(relations, len(A)), certify=False)
    return {"optimal_value": float(r["value"]), "x_one_optimal": _fl(r["x"])}


def _random_ip(rng):
    n = rng.randint(2, 3)
    if rng.random() < 0.6:
        A = [[rng.randint(1, 9) for _ in range(n)] for _ in range(rng.randint(1, 3))]
        b = [rng.randint(10, 40) for _ in A]
        return {"sense": "maximize", "c": [rng.randint(1, 15) for _ in range(n)], "A": A, "b": b,
                "relations": ",".join(["<="] * len(A))}
    A = [[rng.randint(1, 9) for _ in range(n)] for _ in range(rng.randint(1, 3))]
    b = [rng.randint(5, 40) for _ in A]
    return {"sense": "minimize", "c": [rng.randint(1, 15) for _ in range(n)], "A": A, "b": b,
            "relations": ",".join([">="] * len(A))}


_add("integer_program",
     name="Integer linear program (branch and bound)", topic="integer programming",
     purpose="Optimise a linear objective over non-negative INTEGER points satisfying linear constraints: the optimal value "
             "and one optimal integer solution.",
     inputs=_LP_IN,
     outputs=[_O("optimal_value", "optimal value over integer points"),
              _O("x_one_optimal", "one optimal integer solution (the first found: depth-first, lowest-index fractional variable, round-down branch first)"),
              _O("nodes", "branch-and-bound nodes explored")],
     primary="optimal_value", conditions=["all variables integer and >= 0", "a bounded LP relaxation", "at most 4000 branch-and-bound nodes"],
     method="LP-based branch and bound; every relaxation solved exactly (rational simplex), so pruning and integrality tests are exact",
     latex=r"\max / \min \; c^\top x \quad \text{s.t. } A x \;(\le,\ge,=)\; b,\; x \in \mathbb{Z}_{\ge 0}^n",
     run=integer_program, verify=verify_ip, parse=lambda q: _lp_question(q, r"\b(integer|whole[- ]number)", None),
     reference=lambda sense, c, A, b, relations=None: {"optimal_value": _scipy_lp(sense, c, A, b, _relations(relations, len(A)), integer=True)},
     wrong=wrong_ip, random_inputs=_random_ip,
     keywords=["integer program", "integer programming", "branch and bound"],
     templates=["Maximize 5x + 4y subject to 6x + 4y <= 24, x + 2y <= 6, x, y >= 0 and integer.",
                "Solve the integer program: minimize 3x + 2y subject to x + y >= 3.5, x, y >= 0, integer."],
     examples=[{"inputs": {"sense": "maximize", "c": [5, 4], "A": [[6, 4], [1, 2]], "b": [24, 6]},
                "outputs": {"optimal_value": 20.0}},
               {"inputs": {"sense": "minimize", "c": [3, 2], "A": [[1, 1]], "b": [3.5], "relations": ">="},
                "outputs": {"optimal_value": 8.0}}],
     questions=[{"q": "Maximize 5x + 4y subject to 6x + 4y <= 24, x + 2y <= 6, x, y >= 0 and integer.", "outputs": {"optimal_value": 20.0}},
                {"q": "Solve the integer program: minimize 3x + 2y subject to x + y >= 3.5, x, y >= 0, integer.", "outputs": {"optimal_value": 8.0}},
                {"q": "Maximize x + y subject to 2x + 2y = 3, x, y >= 0, integer.", "outcome": "INVALID_INPUT"}])


# --- shadow prices (dual values), reported only when unique -----------------------------------------------------
def lp_shadow_prices(sense, c, A, b, relations=None):
    r = _lp(sense, c, A, b, _relations(relations, len(A)))
    if not r["p_unique"]:
        raise ValueError("the shadow prices are not unique at this optimum (a degenerate optimal solution); "
                         "only one-sided rates of change exist")
    return _lp_outputs(r)


def reference_shadow(sense, c, A, b, relations=None):
    """INDEPENDENT reference: HiGHS marginals (dual values) from scipy.optimize.linprog."""
    rel = _relations(relations, len(A))
    val, res = _scipy_lp(sense, c, A, b, rel)
    s = 1.0 if sense == "minimize" else -1.0
    ub = [i for i, r in enumerate(rel) if r != "="]
    eq = [i for i, r in enumerate(rel) if r == "="]
    p = [0.0] * len(A)
    for k, i in enumerate(ub):
        p[i] = s * float(res.ineqlin.marginals[k]) * (1 if rel[i] == "<=" else -1)
    for k, i in enumerate(eq):
        p[i] = s * float(res.eqlin.marginals[k])
    return {"optimal_value": val, "shadow_prices": p}


_add("lp_shadow_prices",
     name="Shadow prices of a linear program", topic="linear programming / sensitivity analysis",
     purpose="The shadow price (dual value) of each constraint: the change of the optimal value per unit increase of its "
             "right-hand side, when it is uniquely defined.",
     inputs=_LP_IN,
     outputs=[_O("shadow_prices", "shadow price of each constraint, in order"),
              _O("optimal_value", "optimal value of the objective"),
              _O("x", "the optimal solution (only when unique; otherwise x_one_optimal)"),
              _O("solution_unique", "yes when the optimal solution is unique", "text")],
     primary="optimal_value", conditions=["all variables >= 0", "a finite optimum", "unique dual solution (otherwise INVALID_INPUT)"],
     method="exact simplex; the dual solution y = c_B B^{-1}; uniqueness of the dual proved by an auxiliary LP over the dual optimal face",
     latex=r"y^\top = c_B^\top B^{-1}, \qquad \frac{\partial z^*}{\partial b_i} = y_i",
     run=lp_shadow_prices, verify=verify_lp, parse=lambda q: _lp_question(q, r"\b(shadow prices?|dual values?|dual prices?)\b", None),
     reference=reference_shadow, wrong=wrong_lp, random_inputs=lambda rng: _random_lp(rng),
     keywords=["shadow price", "dual value"],
     templates=["Find the shadow prices: maximize 3x + 5y subject to x <= 4, 2y <= 12, 3x + 2y <= 18, x, y >= 0."],
     examples=[{"inputs": {"sense": "maximize", "c": [3, 5], "A": [[1, 0], [0, 2], [3, 2]], "b": [4, 12, 18]},
                "outputs": {"shadow_prices": [0.0, 1.5, 1.0], "optimal_value": 36.0}}],
     questions=[{"q": "Find the shadow prices: maximize 3x + 5y subject to x <= 4, 2y <= 12, 3x + 2y <= 18, x, y >= 0.",
                 "outputs": {"shadow_prices": [0.0, 1.5, 1.0]}},
                {"q": "What are the dual values when you minimize 2x + 3y subject to x + y >= 4, x + 3y >= 6, x, y >= 0?",
                 "outputs": {"shadow_prices": [1.5, 0.5]}}])


# =================================================================================================================
# closed-form optima (calculus): references solve the first-order condition symbolically with sympy
# =================================================================================================================
def _q(v):
    import sympy as sp
    f = _fr(v)
    return sp.Rational(f.numerator, f.denominator)


def _sym_opt(expr_fn, lo_ok=lambda s: True):
    """sympy: stationary points of a one-variable expression (positive real roots by default)."""
    import sympy as sp
    t = sp.Symbol("t", real=True)
    e = expr_fn(t)
    roots = [r for r in sp.solve(sp.diff(e, t), t) if r.is_real and lo_ok(r)]
    return t, e, roots


def _quad_side(expr, var):
    s = expr.replace("²", "^2").replace("**", "^").replace("·", "*").replace("−", "-").replace(" ", "")
    s = re.sub(r"(?<=\d)\*(?=" + var + ")", "", s)
    if not s:
        return None
    if s[0] not in "+-":
        s = "+" + s
    terms = re.findall(r"[+-][^+-]+", s)
    if "".join(terms) != s:
        return None
    a = b = c = 0.0
    num = r"([+-](?:\d+(?:\.\d+)?(?:/\d+)?|\.\d+)?)"
    for t in terms:
        m = re.fullmatch(num + var + r"\^2", t)
        if m:
            a += float(F(m.group(1) + ("1" if m.group(1) in "+-" else "")))
            continue
        m = re.fullmatch(num + var, t)
        if m:
            b += float(F(m.group(1) + ("1" if m.group(1) in "+-" else "")))
            continue
        m = re.fullmatch(r"([+-](?:\d+(?:\.\d+)?(?:/\d+)?|\.\d+))", t)
        if m:
            c += float(F(m.group(1)))
            continue
        return None
    return a, b, c


def quadratic_optimum(a, b, c, goal="optimum"):
    _finite(a, b, c)
    if a == 0:
        raise ValueError("a must not be 0: a linear function has no interior optimum")
    if goal == "minimum" and a < 0:
        raise ValueError("with a < 0 the parabola opens downward: it has a maximum, no minimum")
    if goal == "maximum" and a > 0:
        raise ValueError("with a > 0 the parabola opens upward: it has a minimum, no maximum")
    A, B, Cc = _fr(a), _fr(b), _fr(c)
    x = -B / (2 * A)
    return {"x_opt": float(x), "f_opt": float(Cc - B * B / (4 * A)), "type": "minimum" if a > 0 else "maximum"}


def _parse_quadratic_opt(q):
    ql = q.lower().replace("−", "-")
    goal = "minimum" if re.search(r"\bminim(?:um|ise|ize)\b", ql) else ("maximum" if re.search(r"\bmaxim(?:um|ise|ize)\b", ql) else None)
    if goal is None and not re.search(r"\b(optimum|extreme value|turning point|stationary point)\b", ql):
        return None
    if re.search(r"\b(subject to|s\.t\.|constraint|interval|on \[|between|demand|cost|revenue|profit)\b", ql):
        return None
    m = re.search(r"(?:f\s*\(\s*([a-z])\s*\)|\by)\s*=\s*([-+0-9a-z.\s/^²*]+?)\s*(?:[?.,;]\s*|$)", ql)
    if not m:
        return None
    expr = m.group(2)
    var = m.group(1) or (re.search(r"([a-z])(?:\^2|²|\*\*2)", expr).group(1) if re.search(r"([a-z])(?:\^2|²|\*\*2)", expr) else None)
    if var is None or re.search(r"(?:\^|\*\*)\s*[3-9]", expr):
        return None
    co = _quad_side(expr, var)
    if co is None:
        return None
    return {"a": co[0], "b": co[1], "c": co[2], "goal": goal or "optimum"}


def _ref_quadratic_opt(a, b, c, goal="optimum"):
    import sympy as sp
    t, e, roots = _sym_opt(lambda t: _q(a) * t ** 2 + _q(b) * t + _q(c), lambda r: True)
    return {"x_opt": float(roots[0]), "f_opt": float(e.subs(t, roots[0]))}


_add("quadratic_optimum",
     name="Optimum of a quadratic function", topic="unconstrained optimisation / one variable",
     purpose="The minimum or maximum of f(x) = a x^2 + b x + c: where it occurs and its value.",
     inputs=[_I("a", "coefficient of x^2"), _I("b", "coefficient of x"), _I("c", "constant term"),
             _I("goal", "minimum, maximum or optimum (default optimum)", "choice", required=False, choices=["minimum", "maximum", "optimum"])],
     outputs=[_O("x_opt", "x at the optimum"), _O("f_opt", "optimal value f(x*)"), _O("type", "minimum (a > 0) or maximum (a < 0)", "text")],
     primary="f_opt", conditions=["a != 0", "a minimum asked needs a > 0, a maximum a < 0"],
     method="first-order condition f'(x) = 2ax + b = 0, exact fractions", latex=r"x^* = -\frac{b}{2a}, \quad f^* = c - \frac{b^2}{4a}",
     run=quadratic_optimum,
     verify=lambda i, o: max(abs(2 * i["a"] * o["x_opt"] + i["b"]) / max(1.0, abs(i["b"]), abs(2 * i["a"] * o["x_opt"])),
                             _rel(o["f_opt"], i["a"] * o["x_opt"] ** 2 + i["b"] * o["x_opt"] + i["c"])),
     parse=_parse_quadratic_opt, reference=_ref_quadratic_opt,
     wrong=lambda a, b, c, goal="optimum": {"x_opt": b / (2 * a), "f_opt": a * (b / (2 * a)) ** 2 + b * b / (2 * a) + c},
     random_inputs=lambda rng: {"a": rng.choice([-1, 1]) * rng.randint(1, 12) / rng.choice([1, 2, 4]),
                                "b": rng.randint(-40, 40), "c": rng.randint(-50, 50)},
     keywords=["minimum of", "maximum of", "quadratic"],
     templates=["Find the minimum of f(x) = 2x^2 - 8x + 3.", "What is the maximum value of y = -x^2 + 6x + 1?"],
     examples=[{"inputs": {"a": 2, "b": -8, "c": 3}, "outputs": {"x_opt": 2.0, "f_opt": -5.0, "type": "minimum"}},
               {"inputs": {"a": -1, "b": 6, "c": 1}, "outputs": {"x_opt": 3.0, "f_opt": 10.0, "type": "maximum"}}],
     questions=[{"q": "Find the minimum of f(x) = 2x^2 - 8x + 3.", "outputs": {"x_opt": 2.0, "f_opt": -5.0}},
                {"q": "What is the maximum value of y = -x^2 + 6x + 1?", "outputs": {"f_opt": 10.0}},
                {"q": "Find the minimum of f(x) = -x^2 + 4x.", "outcome": "INVALID_INPUT"}])


# --- break-even ---------------------------------------------------------------------------------------------------
def break_even(F_, p, v):
    _finite(F_, p, v)
    if F_ < 0:
        raise ValueError("fixed cost must be >= 0")
    if p <= v:
        raise ValueError("the price must exceed the variable cost per unit, otherwise no break-even point exists")
    q = _fr(F_) / (_fr(p) - _fr(v))
    return {"quantity": float(q), "revenue": float(q * _fr(p)), "contribution_margin": float(_fr(p) - _fr(v))}


def _parse_break_even(q):
    ql = q.lower()
    if not re.search(r"\bbreak[- ]?even\b", ql):
        return None
    out = {}
    f = _num_after(ql, r"fixed costs?(?: of| are| is| =|:)?")
    p = _num_after(ql, r"(?:selling |sale |unit )?price(?: per unit)?(?: of| is| =|:)?|sells? (?:each unit |it )?(?:for|at)")
    v = _num_after(ql, r"variable costs?(?: per unit)?(?: of| is| are| =|:)?")
    for k, val in (("F_", f), ("p", p), ("v", v)):
        if val is not None:
            out[k] = val
    return out or None


def _ref_break_even(F_, p, v):
    import sympy as sp
    Q = sp.Symbol("Q", positive=True)
    s = sp.solve(sp.Eq(_q(p) * Q, _q(F_) + _q(v) * Q), Q)
    return {"quantity": float(s[0]) if s else 0.0, "revenue": float(s[0] * _q(p)) if s else 0.0}


_add("break_even_point",
     name="Break-even quantity", topic="cost-volume-profit analysis",
     purpose="The number of units at which revenue equals total cost (fixed plus variable), and the revenue there.",
     inputs=[_I("F_", "fixed cost (currency)"), _I("p", "selling price per unit"), _I("v", "variable cost per unit")],
     outputs=[_O("quantity", "break-even quantity (units)"), _O("revenue", "revenue at break-even"), _O("contribution_margin", "price minus variable cost per unit")],
     primary="quantity", conditions=["fixed cost >= 0", "price > variable cost"],
     method="solve p Q = F + v Q", latex=r"Q_{BE} = \frac{F}{p - v}",
     run=break_even, verify=lambda i, o: _rel(i["p"] * o["quantity"], i["F_"] + i["v"] * o["quantity"]),
     parse=_parse_break_even, reference=_ref_break_even,
     wrong=lambda F_, p, v: {"quantity": F_ / p, "revenue": F_},
     random_inputs=lambda rng: (lambda v: {"F_": rng.randint(100, 100000), "v": v, "p": v + rng.randint(1, 60)})(rng.randint(1, 80)),
     keywords=["break-even", "break even"],
     templates=["A firm has fixed costs of 5000, a selling price of 25 per unit and a variable cost of 15 per unit. Find the break-even quantity."],
     examples=[{"inputs": {"F_": 5000, "p": 25, "v": 15}, "outputs": {"quantity": 500.0, "revenue": 12500.0}},
               {"inputs": {"F_": 1200, "p": 7.5, "v": 4.5}, "outputs": {"quantity": 400.0}}],
     questions=[{"q": "A firm has fixed costs of 5000, a selling price of 25 per unit and a variable cost of 15 per unit. Find the break-even quantity.",
                 "outputs": {"quantity": 500.0}},
                {"q": "Find the break-even point if fixed costs are 1200, the price is 7.5 and the variable cost is 4.5 per unit.", "outputs": {"quantity": 400.0}},
                {"q": "Find the break-even point if fixed costs are 1200, the price is 4 and the variable cost is 4.5 per unit.", "outcome": "INVALID_INPUT"},
                {"q": "Find the break-even quantity with fixed costs of 1200 and a price of 7.5.", "outcome": "NEEDS_INPUT"}])


# --- linear demand: profit and revenue maximisation ---------------------------------------------------------------
def profit_max_linear(a, b, cv, F_=0.0):
    _finite(a, b, cv, F_)
    if b <= 0:
        raise ValueError("the demand slope b must be > 0 (price falls as quantity rises)")
    if a <= cv:
        raise ValueError("the demand intercept must exceed the marginal cost, otherwise no positive output is profitable")
    A, B, C, Fx = _fr(a), _fr(b), _fr(cv), _fr(F_)
    qq = (A - C) / (2 * B)
    return {"quantity": float(qq), "price": float(A - B * qq), "profit": float((A - B * qq) * qq - C * qq - Fx)}


def _parse_profit_max(q):
    ql = re.sub(r"\s*([=+\-*])\s*", r"\1", q.lower().replace("−", "-"))
    if "profit" not in ql:
        return None
    m = re.search(r"\bp(?:\(q\))?=(" + _N + r")-(" + _N + r")?\*?q(?![a-z0-9^])", ql)
    k = re.search(r"\bc(?:\(q\))?=(" + _N + r")\+(" + _N + r")?\*?q(?![a-z0-9^])", ql) or re.search(r"\bc(?:\(q\))?=(" + _N + r")?\*?q\+(" + _N + r")", ql)
    if not m:
        return None
    out = {"a": float(m.group(1)), "b": float(m.group(2) or 1)}
    if k:
        if re.search(r"\bc(?:\(q\))?=(" + _N + r")\+", ql):
            out["F_"], out["cv"] = float(k.group(1)), float(k.group(2) or 1)
        else:
            out["cv"], out["F_"] = float(k.group(1) or 1), float(k.group(2))
    return out


def _ref_profit_max(a, b, cv, F_=0.0):
    t, e, roots = _sym_opt(lambda t: (_q(a) - _q(b) * t) * t - _q(cv) * t - _q(F_), lambda r: r > 0)
    return {"quantity": float(roots[0]), "profit": float(e.subs(t, roots[0]))}


_add("profit_max_linear_demand",
     name="Profit-maximising output with linear demand", topic="economic optimisation / monopoly pricing",
     purpose="With inverse demand p = a - b q and cost C = F + c q, the output, price and profit at the profit maximum.",
     inputs=[_I("a", "demand intercept (price at zero quantity)"), _I("b", "demand slope (price drop per unit)"),
             _I("cv", "marginal (variable) cost per unit"), _I("F_", "fixed cost (default 0)", required=False)],
     outputs=[_O("quantity", "profit-maximising quantity"), _O("price", "price at that quantity"), _O("profit", "maximum profit")],
     primary="quantity", conditions=["b > 0", "a > c"],
     method="marginal revenue equals marginal cost: a - 2 b q = c", latex=r"q^* = \frac{a - c}{2b}, \quad p^* = \frac{a + c}{2}",
     run=profit_max_linear,
     verify=lambda i, o: max(abs(i["a"] - 2 * i["b"] * o["quantity"] - i["cv"]) / max(1.0, abs(i["a"])),
                             _rel(o["profit"], (i["a"] - i["b"] * o["quantity"]) * o["quantity"] - i["cv"] * o["quantity"] - i.get("F_", 0.0))),
     parse=_parse_profit_max, reference=_ref_profit_max,
     wrong=lambda a, b, cv, F_=0.0: {"quantity": (a - cv) / b, "price": cv, "profit": -F_},
     random_inputs=lambda rng: (lambda cv: {"a": cv + rng.randint(5, 200), "b": rng.choice([0.5, 1, 2, 3, 4, 0.25]), "cv": cv,
                                            "F_": rng.randint(0, 2000)})(rng.randint(1, 100)),
     keywords=["profit-maximizing", "profit maximizing", "maximum profit"],
     templates=["The demand is p = 100 - 2q and the cost is C = 300 + 20q. Find the profit-maximizing quantity."],
     examples=[{"inputs": {"a": 100, "b": 2, "cv": 20, "F_": 300}, "outputs": {"quantity": 20.0, "price": 60.0, "profit": 500.0}},
               {"inputs": {"a": 50, "b": 0.5, "cv": 10}, "outputs": {"quantity": 40.0, "price": 30.0, "profit": 800.0}}],
     questions=[{"q": "The demand is p = 100 - 2q and the cost is C = 300 + 20q. Find the profit-maximizing quantity.",
                 "outputs": {"quantity": 20.0, "profit": 500.0}},
                {"q": "With demand p = 50 - 0.5q and cost C = 10q + 0, what output maximizes profit?", "outputs": {"quantity": 40.0}},
                {"q": "With demand p = 10 - 2q and cost C = 100 + 20q, what output maximizes profit?", "outcome": "INVALID_INPUT"}])


def revenue_max_linear(alpha, beta):
    _finite(alpha, beta)
    if alpha <= 0 or beta <= 0:
        raise ValueError("demand q = alpha - beta p needs alpha > 0 and beta > 0")
    A, B = _fr(alpha), _fr(beta)
    p = A / (2 * B)
    return {"price": float(p), "quantity": float(A - B * p), "revenue": float(p * (A - B * p))}


def _parse_revenue_max(q):
    ql = re.sub(r"\s*([=+\-*])\s*", r"\1", q.lower().replace("−", "-"))
    if "revenue" not in ql or "cost" in ql:
        return None
    m = re.search(r"\bq(?:\(p\))?=(" + _N + r")-(" + _N + r")?\*?p(?![a-z0-9^])", ql)
    return {"alpha": float(m.group(1)), "beta": float(m.group(2) or 1)} if m else None


def _ref_revenue_max(alpha, beta):
    t, e, roots = _sym_opt(lambda t: t * (_q(alpha) - _q(beta) * t), lambda r: r > 0)
    return {"price": float(roots[0]), "revenue": float(e.subs(t, roots[0]))}


_add("revenue_max_linear_demand",
     name="Revenue-maximising price with linear demand", topic="economic optimisation / pricing",
     purpose="With demand q = alpha - beta p, the price that maximises revenue p q, the quantity sold and the revenue.",
     inputs=[_I("alpha", "demand at a price of zero"), _I("beta", "drop in demand per unit increase in price")],
     outputs=[_O("price", "revenue-maximising price"), _O("quantity", "quantity sold at that price"), _O("revenue", "maximum revenue")],
     primary="price", conditions=["alpha > 0", "beta > 0"],
     method="dR/dp = alpha - 2 beta p = 0 (unit elasticity)", latex=r"p^* = \frac{\alpha}{2\beta}, \quad R^* = \frac{\alpha^2}{4\beta}",
     run=revenue_max_linear,
     verify=lambda i, o: max(abs(i["alpha"] - 2 * i["beta"] * o["price"]) / max(1.0, i["alpha"]),
                             _rel(o["revenue"], o["price"] * (i["alpha"] - i["beta"] * o["price"]))),
     parse=_parse_revenue_max, reference=_ref_revenue_max,
     wrong=lambda alpha, beta: {"price": alpha / beta, "quantity": 0.0, "revenue": 0.0},
     random_inputs=lambda rng: {"alpha": rng.randint(10, 5000), "beta": rng.choice([0.5, 1, 2, 5, 10, 20, 0.2])},
     keywords=["revenue-maximizing", "maximize revenue"],
     templates=["The demand is q = 1000 - 20p. Find the revenue-maximizing price."],
     examples=[{"inputs": {"alpha": 1000, "beta": 20}, "outputs": {"price": 25.0, "quantity": 500.0, "revenue": 12500.0}}],
     questions=[{"q": "The demand is q = 1000 - 20p. Find the revenue-maximizing price.", "outputs": {"price": 25.0, "revenue": 12500.0}},
                {"q": "If demand is q = 600 - 3p, what price maximizes revenue?", "outputs": {"price": 100.0}}])


# --- classic geometric optimisation ------------------------------------------------------------------------------
def fence_max_area(L, wall=False):
    _finite(L)
    if L <= 0:
        raise ValueError("the fence length must be > 0")
    Lf = _fr(L)
    if wall:
        x = Lf / 4
        return {"width": float(x), "length": float(Lf - 2 * x), "max_area": float(x * (Lf - 2 * x))}
    return {"width": float(Lf / 4), "length": float(Lf / 4), "max_area": float(Lf * Lf / 16)}


def _parse_fence(q):
    ql = q.lower()
    if not re.search(r"\bfenc(?:e|ing)\b", ql) or not re.search(r"\b(maxim|largest|greatest)", ql) or "area" not in ql:
        return None
    m = re.search(r"(" + _N + r")\s*(?:m|meters?|metres?|ft|feet|yards?|yd|units?)?\s+of\s+fenc", ql) or \
        re.search(r"fenc\w*\s+(?:of\s+)?(?:length\s+)?(" + _N + r")", ql)
    # one side free along a wall/river/barn/building, or three fenced sides; negated ("no wall or river") it is not.
    # "all four sides", "every side", "completely" say the opposite; both stated at once is not one reading: refuse
    wall = _affirmed(ql, r"\b(?:rivers?|walls?|barns?|buildings?|three sides)\b")
    four = bool(re.search(r"\b(?:all\s+(?:four|4)\s+sides|(?:four|4)\s+sides|every\s+side|each\s+side|all\s+sides|"
                          r"completely|entirely)\b", ql))
    if wall and four:
        return None
    out = {"wall": wall}
    if m:
        out["L"] = float(m.group(1))
    return out


def _ref_fence(L, wall=False):
    k = 2 if wall else 2
    t, e, roots = _sym_opt((lambda t: t * (_q(L) - 2 * t)) if wall else (lambda t: t * (_q(L) / 2 - t)), lambda r: r > 0)
    return {"max_area": float(e.subs(t, roots[0]))}


_add("fence_max_area",
     name="Largest rectangle for a given length of fencing", topic="calculus optimisation / geometry",
     purpose="The dimensions and maximum area of a rectangle enclosed by a fence of length L, on four sides or on three "
             "sides with a wall or river as the fourth.",
     inputs=[_I("L", "total length of fencing (any length unit)"),
             _I("wall", "true when one side lies along a wall or river and needs no fence", "boolean", required=False)],
     outputs=[_O("width", "side perpendicular to the wall (or one side of the square)"), _O("length", "side parallel to the wall"),
              _O("max_area", "maximum enclosed area (square of the length unit)")],
     primary="max_area", conditions=["L > 0"],
     method="area as a function of one side, derivative set to zero", latex=r"A_{\max} = \frac{L^2}{16} \text{ (4 sides)}, \quad \frac{L^2}{8} \text{ (3 sides)}",
     run=fence_max_area,
     verify=lambda i, o: max(_rel(o["max_area"], o["width"] * o["length"]),
                             _rel((2 * o["width"] + o["length"]) if i.get("wall") else 2 * (o["width"] + o["length"]), i["L"]),
                             abs((i["L"] - 4 * o["width"]) if i.get("wall") else (o["width"] - o["length"])) / max(1.0, i["L"])),
     parse=_parse_fence, reference=_ref_fence,
     wrong=lambda L, wall=False: {"width": L / 2, "length": L / 2, "max_area": L * L / 4},
     random_inputs=lambda rng: {"L": rng.randint(4, 5000) / rng.choice([1, 2, 10]), "wall": rng.random() < 0.5},
     keywords=["fencing", "maximum area"],
     templates=["A farmer has 200 m of fencing to enclose a rectangular field. What is the maximum area?",
                "A farmer has 200 m of fencing to enclose a rectangular field along a river, with no fence along the river. Find the maximum area."],
     examples=[{"inputs": {"L": 200}, "outputs": {"max_area": 2500.0, "width": 50.0}},
               {"inputs": {"L": 200, "wall": True}, "outputs": {"max_area": 5000.0, "width": 50.0, "length": 100.0}}],
     questions=[{"q": "A farmer has 200 m of fencing to enclose a rectangular field. What is the maximum area?", "outputs": {"max_area": 2500.0}},
                {"q": "A farmer has 200 m of fencing to enclose a rectangular field along a river, with no fence along the river. Find the maximum area.",
                 "outputs": {"max_area": 5000.0}},
                {"q": "A farmer has 200 m of fencing to enclose a rectangular field on all four sides, with no wall or river to use as a boundary. What is the maximum area?",
                 "outputs": {"max_area": 2500.0}},
                {"q": "A farmer has 200 m of fencing to enclose a rectangular field along a river, fencing all four sides. What is the maximum area?",
                 "outcome": "UNRECOGNIZED_FORMAT"},
                {"q": "What is the largest area of a rectangular pen made with fencing?", "outcome": "NEEDS_INPUT"}])


def open_box(L, W):
    _finite(L, W)
    if L <= 0 or W <= 0:
        raise ValueError("the sheet dimensions must be > 0")
    x = ((L + W) - math.sqrt(L * L - L * W + W * W)) / 6
    return {"cut": x, "max_volume": x * (L - 2 * x) * (W - 2 * x)}


def _parse_box(q):
    ql = q.lower()
    if "box" not in ql or not re.search(r"\b(corners?|squares?)\b", ql) or not re.search(r"\b(maxim|largest|greatest)", ql):
        return None
    m = re.search(r"(" + _N + r")\s*(?:cm|m|in|inches|ft|units?)?\s*(?:by|x|×)\s*(" + _N + r")", ql)
    return {"L": float(m.group(1)), "W": float(m.group(2))} if m else {}


def _ref_box(L, W):
    lim = min(_fr(L), _fr(W)) / 2
    import sympy as sp
    t, e, roots = _sym_opt(lambda t: t * (_q(L) - 2 * t) * (_q(W) - 2 * t), lambda r: 0 < r < sp.Rational(lim.numerator, lim.denominator))
    return {"cut": float(roots[0]), "max_volume": float(e.subs(t, roots[0]))}


_add("open_box_max_volume",
     name="Open box of maximum volume from a rectangular sheet", topic="calculus optimisation / geometry",
     purpose="Squares of side x are cut from the corners of an L by W sheet and the sides folded up: the cut that "
             "maximises the volume, and that volume.",
     inputs=[_I("L", "sheet length"), _I("W", "sheet width")],
     outputs=[_O("cut", "side of the corner squares at the maximum"), _O("max_volume", "maximum volume")],
     primary="max_volume", conditions=["L > 0 and W > 0"],
     method="V(x) = x (L - 2x)(W - 2x); V'(x) = 0, the root in (0, min(L, W)/2)",
     latex=r"x^* = \frac{(L+W) - \sqrt{L^2 - LW + W^2}}{6}",
     run=open_box,
     verify=lambda i, o: max(abs(12 * o["cut"] ** 2 - 4 * (i["L"] + i["W"]) * o["cut"] + i["L"] * i["W"]) / max(1.0, i["L"] * i["W"]),
                             _rel(o["max_volume"], o["cut"] * (i["L"] - 2 * o["cut"]) * (i["W"] - 2 * o["cut"])),
                             0.0 if 0 < o["cut"] < min(i["L"], i["W"]) / 2 else 1.0),
     parse=_parse_box, reference=_ref_box,
     wrong=lambda L, W: (lambda x: {"cut": x, "max_volume": x * (L - 2 * x) * (W - 2 * x)})((L + W) / 12),
     random_inputs=lambda rng: {"L": rng.randint(5, 200), "W": rng.randint(5, 200)},
     keywords=["open box", "maximum volume"],
     templates=["An open box is made from a 30 by 20 sheet by cutting equal squares from the corners. Find the maximum volume."],
     examples=[{"inputs": {"L": 12, "W": 12}, "outputs": {"cut": 2.0, "max_volume": 128.0}},
               {"inputs": {"L": 8, "W": 5}, "outputs": {"cut": 1.0, "max_volume": 18.0}}],
     questions=[{"q": "An open box is made from a 12 by 12 sheet by cutting equal squares from the corners. Find the maximum volume.",
                 "outputs": {"max_volume": 128.0}},
                {"q": "Squares are cut from the corners of an 8 cm by 5 cm card to make an open box. What cut gives the largest volume?",
                 "outputs": {"cut": 1.0}}])


def can_min_surface(V):
    _finite(V)
    if V <= 0:
        raise ValueError("the volume must be > 0")
    r = (V / (2 * math.pi)) ** (1 / 3)
    r = r - (4 * math.pi * r ** 3 - 2 * V) / (12 * math.pi * r ** 2)          # one Newton polish of r^3 = V/(2 pi)
    h = V / (math.pi * r * r)
    return {"radius": r, "height": h, "min_area": 2 * math.pi * r * r + 2 * V / r}


def _parse_can(q):
    ql = q.lower()
    if not re.search(r"\bcylind", ql) or not re.search(r"\b(surface|material|metal|area)\b", ql) or not re.search(r"\b(minim|least|smallest)", ql):
        return None
    if re.search(r"\bopen\b|\bno top\b|\bwithout (?:a )?(?:top|lid)\b", ql):
        return {"_out_of_scope": "an open cylinder (this capability is the closed can)"}
    m = re.search(r"(" + _N + r")\s*(?:cm\^?3|cm³|cubic|ml|m\^?3|m³|in\^?3|units?)", ql) or re.search(r"volume (?:of )?(" + _N + r")", ql)
    return {"V": float(m.group(1))} if m else {}


def _ref_can(V):
    import sympy as sp
    t, e, roots = _sym_opt(lambda t: 2 * sp.pi * t ** 2 + 2 * _q(V) / t, lambda r: r > 0)
    return {"radius": float(roots[0]), "min_area": float(e.subs(t, roots[0]))}


_add("can_min_surface",
     name="Closed cylinder of least surface area for a given volume", topic="calculus optimisation / geometry",
     purpose="The radius, height and surface area of the closed cylindrical can that holds a volume V with the least material.",
     inputs=[_I("V", "volume (cube of the length unit)")],
     outputs=[_O("radius", "optimal radius"), _O("height", "optimal height (equal to the diameter)"), _O("min_area", "least total surface area")],
     primary="min_area", conditions=["V > 0", "closed top and bottom"],
     method="A(r) = 2 pi r^2 + 2V/r; A'(r) = 0", latex=r"r^* = \left(\frac{V}{2\pi}\right)^{1/3}, \quad h^* = 2r^*",
     run=can_min_surface,
     verify=lambda i, o: max(abs(4 * math.pi * o["radius"] - 2 * i["V"] / o["radius"] ** 2) / max(1.0, 4 * math.pi * o["radius"]),
                             _rel(o["height"] * math.pi * o["radius"] ** 2, i["V"]),
                             _rel(o["min_area"], 2 * math.pi * o["radius"] ** 2 + 2 * math.pi * o["radius"] * o["height"])),
     parse=_parse_can, reference=_ref_can,
     wrong=lambda V: (lambda r: {"radius": r, "height": V / (math.pi * r * r), "min_area": 2 * math.pi * r * r + 2 * V / r})((V / math.pi) ** (1 / 3)),
     random_inputs=lambda rng: {"V": rng.randint(1, 100000) / rng.choice([1, 10])},
     keywords=["cylindrical can", "least surface area"],
     templates=["A closed cylindrical can must hold 500 cm^3. Find the dimensions that minimize the surface area."],
     examples=[{"inputs": {"V": 2 * math.pi}, "outputs": {"radius": 1.0, "height": 2.0, "min_area": 6 * math.pi}}],
     questions=[{"q": "A closed cylindrical can must hold 500 cm^3. Find the dimensions that minimize the surface area.",
                 "outputs": {"radius": (500 / (2 * math.pi)) ** (1 / 3)}},
                {"q": "An open cylindrical tank must hold 10 m^3. Find the radius that minimizes the surface area.", "outcome": "OUT_OF_SCOPE"}])


# =================================================================================================================
# inventory
# =================================================================================================================
def eoq(D, S, H):
    _finite(D, S, H)
    if D <= 0 or S <= 0 or H <= 0:
        raise ValueError("demand, ordering cost and holding cost must all be > 0")
    Q = math.sqrt(2 * D * S / H)
    return {"Q": Q, "total_cost": math.sqrt(2 * D * S * H), "orders_per_period": D / Q, "cycle_time": Q / D}


def _inv_numbers(ql):
    out = {}
    d = _num_after(ql, r"(?:annual |yearly |monthly |weekly )?demand(?: rate)?(?: of| is| =|:)?")
    s = _num_after(ql, r"(?:ordering|order|setup|set-up) costs?(?: of| is| =|:)?|costs? per order(?: of| is)?|costs? per setup(?: of| is)?")
    h = _num_after(ql, r"(?:holding|carrying|storage) costs?(?: of| is| =|:)?")
    for k, v in (("D", d), ("S", s), ("H", h)):
        if v is not None:
            out[k] = v
    if "H" in out and re.search(r"(?:holding|carrying) costs?[^.]{0,20}?" + _N + r"\s*(?:%|percent)", ql):
        return None                                   # a holding RATE needs the unit price: not this reading
    return out


def _parse_eoq(q):
    ql = q.lower()
    if not re.search(r"\b(eoq|economic order quantity|optimal order quantity)\b", ql):
        return None
    if re.search(r"\b(shortages?|backorders?|back-orders?|backlog|production rate|produc\w+ rate|discounts?|price breaks?|"
                 r"newsvendor|news vendor|newsboy|single[- ]period)\b", ql):
        return None
    return _inv_numbers(ql)


def _ref_eoq(D, S, H):
    import sympy as sp
    Q = sp.Symbol("Q", positive=True)
    tc = _q(D) * _q(S) / Q + _q(H) * Q / 2
    r = sp.solve(sp.diff(tc, Q), Q)[0]
    return {"Q": float(r), "total_cost": float(tc.subs(Q, r))}


_add("eoq",
     name="Economic order quantity (EOQ)", topic="inventory management",
     purpose="The order size that minimises ordering plus holding cost for a steady demand, the minimum cost, the number "
             "of orders per period and the time between orders.",
     inputs=[_I("D", "demand per period (units, e.g. per year)"), _I("S", "fixed cost per order"),
             _I("H", "holding cost per unit per period")],
     outputs=[_O("Q", "economic order quantity"), _O("total_cost", "minimum ordering + holding cost per period"),
              _O("orders_per_period", "number of orders per period"), _O("cycle_time", "time between orders (in periods)")],
     primary="Q", conditions=["D, S, H > 0", "constant demand, instant replenishment, no shortages"],
     method="minimise D S / Q + H Q / 2 (at the optimum ordering cost equals holding cost)",
     latex=r"Q^* = \sqrt{\frac{2 D S}{H}}, \quad TC^* = \sqrt{2 D S H}",
     run=eoq,
     verify=lambda i, o: max(_rel(i["D"] * i["S"] / o["Q"], i["H"] * o["Q"] / 2),
                             _rel(o["total_cost"], i["D"] * i["S"] / o["Q"] + i["H"] * o["Q"] / 2),
                             _rel(o["orders_per_period"] * o["Q"], i["D"])),
     parse=_parse_eoq, reference=_ref_eoq,
     wrong=lambda D, S, H: {"Q": math.sqrt(D * S / H), "total_cost": math.sqrt(D * S * H), "orders_per_period": D / math.sqrt(D * S / H)},
     random_inputs=lambda rng: {"D": rng.randint(50, 100000), "S": rng.randint(1, 500), "H": rng.randint(1, 100) / rng.choice([1, 4, 10])},
     keywords=["economic order quantity", "eoq"],
     templates=["Find the EOQ for an annual demand of 1200 units, an ordering cost of 50 per order and a holding cost of 3 per unit per year."],
     examples=[{"inputs": {"D": 1200, "S": 50, "H": 3}, "outputs": {"Q": 200.0, "total_cost": 600.0, "orders_per_period": 6.0}},
               {"inputs": {"D": 1000, "S": 10, "H": 0.5}, "outputs": {"Q": 200.0, "total_cost": 100.0}}],
     questions=[{"q": "Find the EOQ for an annual demand of 1200 units, an ordering cost of 50 per order and a holding cost of 3 per unit per year.",
                 "outputs": {"Q": 200.0, "total_cost": 600.0}},
                {"q": "What is the economic order quantity if demand is 1000 units per year, the setup cost is 10 and the holding cost is 0.5 per unit per year?",
                 "outputs": {"Q": 200.0}},
                {"q": "Find the EOQ for an annual demand of 1200 units and an ordering cost of 50 per order.", "outcome": "NEEDS_INPUT"}])


def eoq_backorders(D, S, H, B):
    _finite(D, S, H, B)
    if min(D, S, H, B) <= 0:
        raise ValueError("demand, ordering, holding and shortage costs must all be > 0")
    Q = math.sqrt(2 * D * S / H * (H + B) / B)
    s = Q * H / (H + B)
    return {"Q": Q, "max_backorder": s, "max_inventory": Q - s, "total_cost": math.sqrt(2 * D * S * H * B / (H + B))}


def _verify_eoq_b(i, o):
    D, S, H, B = i["D"], i["S"], i["H"], i["B"]
    Q, s = o["Q"], o["max_backorder"]
    tc = D * S / Q + H * (Q - s) ** 2 / (2 * Q) + B * s * s / (2 * Q)
    g_s = (-H * (Q - s) + B * s) / Q                       # partial derivatives of the cost: zero at the optimum
    g_q = -D * S / Q ** 2 + H * (Q * Q - s * s) / (2 * Q * Q) - B * s * s / (2 * Q * Q)
    return max(_rel(o["total_cost"], tc), abs(g_s) / max(1.0, H + B), abs(g_q) * Q / max(1.0, tc), _rel(o["max_inventory"], Q - s))


def _ref_eoq_b(D, S, H, B):
    import sympy as sp
    Q, s = sp.symbols("Q s", positive=True)
    tc = _q(D) * _q(S) / Q + _q(H) * (Q - s) ** 2 / (2 * Q) + _q(B) * s ** 2 / (2 * Q)
    sol = sp.solve([sp.diff(tc, Q), sp.diff(tc, s)], [Q, s], dict=True)[0]
    return {"Q": float(sol[Q]), "max_backorder": float(sol[s]), "total_cost": float(tc.subs(sol))}


_add("eoq_planned_shortages",
     name="EOQ with planned backorders", topic="inventory management",
     purpose="The order quantity and the planned maximum backorder that minimise ordering, holding and shortage cost when "
             "unmet demand is backordered.",
     inputs=[_I("D", "demand per period"), _I("S", "cost per order"), _I("H", "holding cost per unit per period"),
             _I("B", "backorder (shortage) cost per unit per period")],
     outputs=[_O("Q", "optimal order quantity"), _O("max_backorder", "planned maximum backorder"),
              _O("max_inventory", "maximum inventory level"), _O("total_cost", "minimum total cost per period")],
     primary="Q", conditions=["D, S, H, B > 0", "backorders filled when the order arrives"],
     method="minimise D S/Q + H (Q-s)^2/(2Q) + B s^2/(2Q) over Q and s",
     latex=r"Q^* = \sqrt{\frac{2DS}{H}\cdot\frac{H+B}{B}}, \quad s^* = Q^* \frac{H}{H+B}",
     run=eoq_backorders, verify=_verify_eoq_b, parse=lambda q: (_inv_numbers(q.lower()) or {}) | (
         {"B": v} if (v := _num_after(q.lower(), r"(?:shortage|backorder|back-order|stockout) costs?(?: of| is| =|:)?")) is not None else {})
     if re.search(r"\b(eoq|economic order quantity|order quantity)\b", q.lower()) and re.search(r"\b(shortages?|backorders?|back-orders?)\b", q.lower()) else None,
     reference=_ref_eoq_b,
     wrong=lambda D, S, H, B: (lambda Q: {"Q": Q, "max_backorder": 0.0, "max_inventory": Q, "total_cost": math.sqrt(2 * D * S * H)})(math.sqrt(2 * D * S / H)),
     random_inputs=lambda rng: {"D": rng.randint(100, 50000), "S": rng.randint(5, 400), "H": rng.randint(1, 40), "B": rng.randint(1, 80)},
     keywords=["backorder", "planned shortages"],
     templates=["Find the EOQ with backorders for an annual demand of 1200 units, an ordering cost of 50, a holding cost of 3 and a shortage cost of 6 per unit per year."],
     examples=[{"inputs": {"D": 1200, "S": 50, "H": 3, "B": 6}, "outputs": {"Q": 200 * math.sqrt(1.5), "max_backorder": 200 * math.sqrt(1.5) / 3}}],
     questions=[{"q": "Find the EOQ with backorders for an annual demand of 1200 units, an ordering cost of 50, a holding cost of 3 and a shortage cost of 6 per unit per year.",
                 "outputs": {"Q": 200 * math.sqrt(1.5)}}])


def epq(D, P, S, H):
    _finite(D, P, S, H)
    if min(D, P, S, H) <= 0:
        raise ValueError("demand, production rate, setup and holding costs must be > 0")
    if P <= D:
        raise ValueError("the production rate must exceed the demand rate")
    k = 1 - D / P
    Q = math.sqrt(2 * D * S / (H * k))
    return {"Q": Q, "max_inventory": Q * k, "total_cost": math.sqrt(2 * D * S * H * k), "runs_per_period": D / Q}


def _ref_epq(D, P, S, H):
    import sympy as sp
    Q = sp.Symbol("Q", positive=True)
    tc = _q(D) * _q(S) / Q + _q(H) * Q * (1 - _q(D) / _q(P)) / 2
    r = sp.solve(sp.diff(tc, Q), Q)[0]
    return {"Q": float(r), "total_cost": float(tc.subs(Q, r))}


def _parse_epq(q):
    ql = q.lower()
    if not re.search(r"\b(epq|economic production quantity|production lot size|economic lot size|optimal production (?:run|lot))\b", ql):
        return None
    out = _inv_numbers(ql) or {}
    p = _num_after(ql, r"production rate(?: of| is| =|:)?|produc\w+ (?:at a rate of|at)")
    if p is not None:
        out["P"] = p
    return out


_add("epq_production_lot",
     name="Economic production quantity (EPQ)", topic="inventory management",
     purpose="The production lot size that minimises setup plus holding cost when stock builds up gradually at a finite "
             "production rate.",
     inputs=[_I("D", "demand rate per period"), _I("P", "production rate per period (> D)"), _I("S", "setup cost per run"),
             _I("H", "holding cost per unit per period")],
     outputs=[_O("Q", "economic production quantity"), _O("max_inventory", "maximum inventory Q (1 - D/P)"),
              _O("total_cost", "minimum setup + holding cost per period"), _O("runs_per_period", "production runs per period")],
     primary="Q", conditions=["P > D > 0", "S, H > 0"],
     method="minimise D S/Q + H Q (1 - D/P)/2", latex=r"Q^* = \sqrt{\frac{2DS}{H(1 - D/P)}}",
     run=epq,
     verify=lambda i, o: max(_rel(i["D"] * i["S"] / o["Q"], i["H"] * o["Q"] * (1 - i["D"] / i["P"]) / 2),
                             _rel(o["total_cost"], i["D"] * i["S"] / o["Q"] + i["H"] * o["max_inventory"] / 2),
                             _rel(o["max_inventory"], o["Q"] * (1 - i["D"] / i["P"]))),
     parse=_parse_epq, reference=_ref_epq,
     wrong=lambda D, P, S, H: (lambda Q: {"Q": Q, "max_inventory": Q, "total_cost": math.sqrt(2 * D * S * H)})(math.sqrt(2 * D * S / H)),
     random_inputs=lambda rng: (lambda D: {"D": D, "P": D + rng.randint(1, 5 * D), "S": rng.randint(5, 500), "H": rng.randint(1, 50)})(rng.randint(100, 20000)),
     keywords=["economic production quantity", "epq", "production lot"],
     templates=["Find the EPQ for an annual demand of 1000 units, a production rate of 2000 units per year, a setup cost of 100 and a holding cost of 4 per unit per year."],
     examples=[{"inputs": {"D": 1000, "P": 2000, "S": 100, "H": 4}, "outputs": {"Q": math.sqrt(100000), "max_inventory": math.sqrt(100000) / 2}}],
     questions=[{"q": "Find the EPQ for an annual demand of 1000 units, a production rate of 2000 units per year, a setup cost of 100 and a holding cost of 4 per unit per year.",
                 "outputs": {"Q": math.sqrt(100000)}},
                {"q": "Find the economic production quantity for a demand of 3000 units, a production rate of 2000 units, a setup cost of 100 and a holding cost of 4.",
                 "outcome": "INVALID_INPUT"}])


def eoq_discount(D, S, i_rate, breaks, prices):
    _finite(D, S, i_rate)
    if min(D, S, i_rate) <= 0:
        raise ValueError("demand, ordering cost and holding rate must be > 0")
    if len(breaks) != len(prices) or not breaks:
        raise ValueError("give one price per quantity break")
    if breaks[0] != 0 and breaks[0] > 1:
        raise ValueError("the first price break must start at 0 or 1 unit")
    if any(b2 <= b1 for b1, b2 in zip(breaks, breaks[1:])) or any(p <= 0 for p in prices):
        raise ValueError("quantity breaks must increase and prices must be > 0")
    best = None
    for k, (q0, p) in enumerate(zip(breaks, prices)):
        hi = breaks[k + 1] if k + 1 < len(breaks) else math.inf
        Q = math.sqrt(2 * D * S / (i_rate * p))
        if Q >= hi:
            continue                                   # cheaper tiers are reachable: this tier's optimum lies beyond it
        Q = max(Q, q0, 1e-300)
        tc = D * p + D * S / Q + i_rate * p * Q / 2
        if best is None or tc < best[0] - 1e-12 * tc:
            best = (tc, Q, p)
    return {"order_quantity": best[1], "total_cost": best[0], "unit_price": best[2]}


def _ref_discount(D, S, i_rate, breaks, prices):
    """INDEPENDENT reference: bounded scalar minimisation of the total cost (purchase + ordering + holding) in every tier."""
    from scipy.optimize import minimize_scalar
    best = math.inf
    for k, (q0, p) in enumerate(zip(breaks, prices)):
        hi = breaks[k + 1] if k + 1 < len(breaks) else 1e3 * math.sqrt(2 * D * S / (i_rate * p)) + q0 + 1
        f = lambda Q: D * p + D * S / Q + i_rate * p * Q / 2
        r = minimize_scalar(f, bounds=(max(q0, 1e-9), hi), method="bounded", options={"xatol": 1e-12, "maxiter": 2000})
        best = min(best, float(r.fun), f(max(q0, 1e-9)), f(hi))
    return {"total_cost": best}


def _verify_discount(i, o):
    D, S, ir = i["D"], i["S"], i["i_rate"]
    Q, p = o["order_quantity"], o["unit_price"]
    k = i["prices"].index(p) if p in i["prices"] else -1
    if k < 0 or Q < i["breaks"][k] - 1e-9:
        return 1.0
    worst = _rel(o["total_cost"], D * p + D * S / Q + ir * p * Q / 2)
    for q0, pk in zip(i["breaks"], i["prices"]):           # no tier's best feasible point is cheaper
        Qk = max(math.sqrt(2 * D * S / (ir * pk)), q0, 1e-300)
        nxt = [b for b in i["breaks"] if b > q0]
        if nxt and Qk >= nxt[0]:
            Qk = nxt[0]                                   # the boundary point belongs to the next (cheaper) tier
            pk = i["prices"][i["breaks"].index(nxt[0])]
        worst = max(worst, max(0.0, o["total_cost"] - (D * pk + D * S / Qk + ir * pk * Qk / 2)) / o["total_cost"])
    return worst


def _parse_discount(q):
    ql = q.lower()
    if not re.search(r"\b(quantity discounts?|price breaks?|discount schedule)\b", ql):
        return None
    out = _inv_numbers(ql.replace("holding rate", "holding_rate")) or {}
    out.pop("H", None)
    r = _num_after(ql, r"holding (?:cost )?rate(?: of| is| =|:)?|carrying (?:cost )?rate(?: of| is| =|:)?|holding costs? (?:is |of )?", pct=True)
    if r is not None:
        out["i_rate"] = r
    out.update(_labelled(q, {"breaks": r"breaks?(?: at)?|quantities|from|minimum quantities", "prices": r"prices?|unit costs?"}))
    return out


_add("eoq_quantity_discount",
     name="Order quantity with quantity discounts (all-units)", topic="inventory management",
     purpose="With all-units price breaks, the order quantity that minimises purchase + ordering + holding cost, the price "
             "paid and the minimum total cost per period.",
     inputs=[_I("D", "demand per period"), _I("S", "cost per order"), _I("i_rate", "holding cost as a fraction of the unit price per period (0.2 = 20%)"),
             _I("breaks", "minimum order quantity of each price tier, increasing, the first 0", "vector", min_length=1, max_length=10),
             _I("prices", "unit price of each tier", "vector", min_length=1, max_length=10)],
     outputs=[_O("order_quantity", "optimal order quantity"), _O("total_cost", "minimum total cost per period (purchase + ordering + holding)"),
              _O("unit_price", "unit price paid")],
     primary="total_cost", conditions=["D, S, i > 0", "breaks increasing from 0", "ties broken toward the lower-quantity tier"],
     method="EOQ per tier at its price, raised to the tier minimum when below it; tiers whose EOQ exceeds their upper limit are dominated; least total cost",
     latex=r"TC(Q) = D p_k + \frac{D S}{Q} + \frac{i p_k Q}{2}",
     run=eoq_discount, verify=_verify_discount, parse=_parse_discount, reference=_ref_discount,
     wrong=lambda D, S, i_rate, breaks, prices: (lambda Q, p: {"order_quantity": Q, "unit_price": p,
                                                              "total_cost": D * p + D * S / Q + i_rate * p * Q / 2})(
         math.sqrt(2 * D * S / (i_rate * prices[0])), prices[-1]),
     random_inputs=lambda rng: (lambda k, p0: {"D": rng.randint(500, 50000), "S": rng.randint(10, 300), "i_rate": rng.choice([0.1, 0.15, 0.2, 0.25, 0.3]),
                                               "breaks": [0] + sorted(rng.sample(range(100, 5000, 50), k - 1)),
                                               "prices": [round(p0 * (1 - 0.03 * j), 4) for j in range(k)]})(rng.randint(2, 4), rng.randint(2, 50)),
     keywords=["quantity discount", "price breaks"],
     templates=["Quantity discounts: annual demand 5000, ordering cost 49, holding rate 20%, price breaks at [0, 1000, 2000] with prices [5, 4.8, 4.75]. Find the best order quantity."],
     examples=[{"inputs": {"D": 5000, "S": 49, "i_rate": 0.2, "breaks": "0, 1000, 2000", "prices": "5, 4.8, 4.75"},
                "outputs": {"order_quantity": 1000.0, "total_cost": 24725.0, "unit_price": 4.8}}],
     questions=[{"q": "Quantity discounts: annual demand 5000, ordering cost 49, holding rate 20%, price breaks at [0, 1000, 2000] with prices [5, 4.8, 4.75]. Find the best order quantity.",
                 "outputs": {"order_quantity": 1000.0, "total_cost": 24725.0}}])


def reorder_point(d, L, sigma, SL):
    _finite(d, L, sigma, SL)
    if d < 0 or L <= 0 or sigma < 0:
        raise ValueError("demand must be >= 0, lead time > 0 and the standard deviation >= 0")
    if not 0.5 <= SL < 1:
        raise ValueError("the service level must be at least 0.5 and below 1 (e.g. 0.95)")
    z = _STD.inv_cdf(SL)
    ss = z * sigma * math.sqrt(L)
    return {"reorder_point": d * L + ss, "safety_stock": ss, "z": z, "lead_time_demand": d * L}


def _parse_rop(q):
    ql = q.lower()
    if not re.search(r"\b(reorder point|re-order point|rop)\b", ql):
        return None
    out = {}
    d = _num_after(ql, r"(?:average |mean |expected )?(?:daily |weekly |monthly )?demand(?: per (?:day|week|month|period))?(?: of| is| averages| averaging| =|:)?")
    sd = _num_after(ql, r"standard deviation(?: of)?(?: (?:daily |weekly )?demand)?(?: of| is| =|:)?|sd(?: of| =)?")
    L = _num_after(ql, r"lead time(?: of| is| =|:)?")
    sl = _num_after(ql, r"service level(?: of| is| =|:)?|in-stock probability(?: of)?", pct=True)
    for k, v in (("d", d), ("sigma", sd), ("L", L), ("SL", sl)):
        if v is not None:
            out[k] = v
    return out


_add("reorder_point",
     name="Reorder point with safety stock", topic="inventory management",
     purpose="The stock level at which to reorder so that demand during the lead time is covered with a given cycle "
             "service level, for normally distributed demand per period.",
     inputs=[_I("d", "average demand per period"), _I("L", "lead time in periods"), _I("sigma", "standard deviation of demand per period"),
             _I("SL", "cycle service level (probability of no stockout per cycle, e.g. 0.95)")],
     outputs=[_O("reorder_point", "reorder point"), _O("safety_stock", "safety stock z sigma sqrt(L)"), _O("z", "standard normal quantile of the service level"),
              _O("lead_time_demand", "expected demand during the lead time")],
     primary="reorder_point", conditions=["0.5 <= SL < 1", "independent normal demand per period"],
     method="z = Phi^{-1}(SL); ROP = d L + z sigma sqrt(L)", latex=r"ROP = \bar d L + z_{SL}\,\sigma_d \sqrt{L}",
     run=reorder_point,
     verify=lambda i, o: max(_rel(o["reorder_point"], i["d"] * i["L"] + o["safety_stock"]),
                             abs(_STD.cdf(o["z"]) - i["SL"]),
                             _rel(o["safety_stock"], o["z"] * i["sigma"] * math.sqrt(i["L"]))),
     parse=_parse_rop,
     reference=lambda d, L, sigma, SL: {"reorder_point": d * L + float(__import__("scipy.stats", fromlist=["norm"]).norm.ppf(SL)) * sigma * math.sqrt(L)},
     wrong=lambda d, L, sigma, SL: (lambda z: {"reorder_point": d * L + z * sigma * L, "safety_stock": z * sigma * L, "z": z})(_STD.inv_cdf(SL)),
     random_inputs=lambda rng: {"d": rng.randint(1, 500), "L": rng.randint(2, 30), "sigma": rng.randint(1, 100),
                                "SL": rng.choice([0.8, 0.85, 0.9, 0.95, 0.975, 0.98, 0.99, 0.999])},
     keywords=["reorder point", "safety stock"],
     templates=["Daily demand averages 40 units with a standard deviation of 8 units, the lead time is 9 days and the service level is 95%. Find the reorder point."],
     examples=[{"inputs": {"d": 40, "L": 9, "sigma": 8, "SL": 0.95}, "outputs": {"reorder_point": 360 + 24 * _STD.inv_cdf(0.95)}},
               {"inputs": {"d": 100, "L": 4, "sigma": 0, "SL": 0.9}, "outputs": {"reorder_point": 400.0, "safety_stock": 0.0}}],
     questions=[{"q": "Daily demand averages 40 units with a standard deviation of 8 units, the lead time is 9 days and the service level is 95%. Find the reorder point.",
                 "outputs": {"reorder_point": 360 + 24 * _STD.inv_cdf(0.95)}},
                {"q": "Find the reorder point with demand of 40 per day, standard deviation 8, lead time 9 days and service level 120%.", "outcome": "INVALID_INPUT"}])


def order_up_to(d, sigma, R, L, SL):
    _finite(d, sigma, R, L, SL)
    if d < 0 or sigma < 0 or R <= 0 or L < 0:
        raise ValueError("demand and its standard deviation must be >= 0, the review period > 0 and the lead time >= 0")
    if not 0.5 <= SL < 1:
        raise ValueError("the service level must be at least 0.5 and below 1")
    z = _STD.inv_cdf(SL)
    ss = z * sigma * math.sqrt(R + L)
    return {"order_up_to_level": d * (R + L) + ss, "safety_stock": ss, "z": z}


_add("order_up_to_level",
     name="Periodic-review order-up-to level", topic="inventory management",
     purpose="The target inventory position S of a periodic-review system that covers demand over the review period "
             "plus the lead time with a given cycle service level.",
     inputs=[_I("d", "average demand per period"), _I("sigma", "standard deviation of demand per period"),
             _I("R", "review period (periods)"), _I("L", "lead time (periods)"), _I("SL", "cycle service level, e.g. 0.95")],
     outputs=[_O("order_up_to_level", "order-up-to level S"), _O("safety_stock", "safety stock z sigma sqrt(R + L)"), _O("z", "standard normal quantile")],
     primary="order_up_to_level", conditions=["0.5 <= SL < 1", "independent normal demand per period"],
     method="S = d (R + L) + z sigma sqrt(R + L)", latex=r"S = \bar d (R + L) + z_{SL}\,\sigma_d\sqrt{R + L}",
     run=order_up_to,
     verify=lambda i, o: max(_rel(o["order_up_to_level"], i["d"] * (i["R"] + i["L"]) + o["safety_stock"]), abs(_STD.cdf(o["z"]) - i["SL"]),
                             _rel(o["safety_stock"], o["z"] * i["sigma"] * math.sqrt(i["R"] + i["L"]))),
     parse=lambda q: (lambda ql: None if not re.search(r"\b(order[- ]up[- ]to|periodic review|target inventory)\b", ql) else {
         k: v for k, v in (("d", _num_after(ql, r"(?:average |mean )?(?:daily |weekly )?demand(?: per (?:day|week|period))?(?: of| is| averages| =|:)?")),
                           ("sigma", _num_after(ql, r"standard deviation(?: of)?(?: demand)?(?: of| is| =|:)?")),
                           ("R", _num_after(ql, r"review (?:period|interval)(?: of| is| =|:)?")),
                           ("L", _num_after(ql, r"lead time(?: of| is| =|:)?")),
                           ("SL", _num_after(ql, r"service level(?: of| is| =|:)?", pct=True))) if v is not None})(q.lower()),
     reference=lambda d, sigma, R, L, SL: {"order_up_to_level": d * (R + L) + float(__import__("scipy.stats", fromlist=["norm"]).norm.ppf(SL)) * sigma * math.sqrt(R + L)},
     wrong=lambda d, sigma, R, L, SL: (lambda z: {"order_up_to_level": d * L + z * sigma * math.sqrt(L), "safety_stock": z * sigma * math.sqrt(L), "z": z})(_STD.inv_cdf(SL)),
     random_inputs=lambda rng: {"d": rng.randint(1, 300), "sigma": rng.randint(1, 60), "R": rng.randint(1, 14), "L": rng.randint(1, 14),
                                "SL": rng.choice([0.8, 0.9, 0.95, 0.98, 0.99])},
     keywords=["order-up-to level", "periodic review"],
     templates=["In a periodic review system, demand averages 20 units per day with a standard deviation of 4, the review period is 7 days, the lead time is 2 days and the service level is 95%. Find the order-up-to level."],
     examples=[{"inputs": {"d": 20, "sigma": 4, "R": 7, "L": 2, "SL": 0.95}, "outputs": {"order_up_to_level": 180 + 12 * _STD.inv_cdf(0.95)}}],
     questions=[{"q": "In a periodic review system, demand averages 20 units per day with a standard deviation of 4, the review period is 7 days, the lead time is 2 days and the service level is 95%. Find the order-up-to level.",
                 "outputs": {"order_up_to_level": 180 + 12 * _STD.inv_cdf(0.95)}}])


# --- newsvendor ---------------------------------------------------------------------------------------------------
def _nv_check(p, c, s):
    _finite(p, c, s)
    if not s < c < p:
        raise ValueError("the salvage value, unit cost and price must satisfy salvage < cost < price")


def newsvendor_normal(p, c, mu, sigma, s=0.0):
    _nv_check(p, c, s)
    _finite(mu, sigma)
    if sigma <= 0:
        raise ValueError("the standard deviation of demand must be > 0")
    cr = (p - c) / (p - s)
    z = _STD.inv_cdf(cr)
    Q = mu + z * sigma
    loss = sigma * (_STD.pdf(z) - z * (1 - cr))                # expected shortage E[(D - Q)^+]
    sales = mu - loss
    return {"critical_ratio": cr, "Q": Q, "expected_profit": (p - s) * sales - (c - s) * Q, "expected_shortage": loss}


def _ref_nv_normal(p, c, mu, sigma, s=0.0):
    """INDEPENDENT reference: scipy's normal quantile and numerical integration of the profit over the demand density."""
    from scipy import integrate, stats
    cr = (p - c) / (p - s)
    Q = float(stats.norm.ppf(cr, mu, sigma))
    f = lambda x: (p * min(x, Q) + s * max(Q - x, 0.0) - c * Q) * stats.norm.pdf(x, mu, sigma)
    lo, hi = mu - 40 * sigma, mu + 40 * sigma
    val = integrate.quad(f, lo, Q, epsabs=1e-12, epsrel=1e-13, limit=500)[0] + integrate.quad(f, Q, hi, epsabs=1e-12, epsrel=1e-13, limit=500)[0]
    return {"critical_ratio": cr, "Q": Q, "expected_profit": val}


def _nv_words(ql):
    out = {}
    for k, rx in (("p", r"(?:selling |retail |sale )?price(?: of| is| =|:)?|sells? (?:each |them |it )?(?:for|at)"),
                  ("c", r"(?:unit |purchase |wholesale |production )?cost(?: per unit)?(?: of| is| =|:)?|costs?\s|buys? (?:each |them |it )?(?:for|at)"),
                  ("s", r"salvage(?: value)?(?: of| is| =|:)?|leftovers? (?:are |is )?sold (?:for|at)|clearance price(?: of| is)?")):
        v = _num_after(ql, rx)
        if v is not None:
            out[k] = v
    return out


def _parse_nv(q, dist):
    ql = q.lower()
    if not re.search(r"\b(newsvendor|news vendor|newsboy|single[- ]period)\b", ql):
        return None
    if dist == "normal" and not re.search(r"\bnormal", ql):
        return None
    if dist == "uniform" and not re.search(r"\buniform", ql):
        return None
    if dist == "discrete" and not _bracket_groups(q):
        return None
    if dist != "discrete" and _bracket_groups(q):
        return None
    out = _nv_words(ql)
    if dist == "normal":
        mu = _num_after(ql, r"mean(?: of)?(?: demand)?(?: is| =|:)?")
        sd = _num_after(ql, r"standard deviation(?: of)?(?: demand)?(?: is| =|:)?")
        out.update({k: v for k, v in (("mu", mu), ("sigma", sd)) if v is not None})
    elif dist == "uniform":
        m = re.search(r"(?:between|from)\s*(" + _N + r")\s*(?:and|to)\s*(" + _N + r")", ql)
        if m:
            out["lo"], out["hi"] = float(m.group(1)), float(m.group(2))
    else:
        out.update(_labelled(q, {"demands": r"demands?(?: values)?|values|outcomes", "probs": r"probabilit(?:y|ies)"}))
    return out


_NV_IN = [_I("p", "selling price per unit"), _I("c", "cost per unit")]

_add("newsvendor_normal",
     name="Newsvendor order quantity (normal demand)", topic="inventory management / single-period",
     purpose="The single-period order quantity that maximises expected profit for normally distributed demand: the "
             "critical ratio, the optimal quantity and the expected profit.",
     inputs=_NV_IN + [_I("mu", "mean demand"), _I("sigma", "standard deviation of demand"), _I("s", "salvage value per unsold unit (default 0)", required=False)],
     outputs=[_O("critical_ratio", "critical ratio (p - c)/(p - s)"), _O("Q", "optimal order quantity"),
              _O("expected_profit", "expected profit at Q"), _O("expected_shortage", "expected unmet demand at Q")],
     primary="Q", conditions=["salvage < cost < price", "sigma > 0", "continuous normal demand"],
     method="critical fractile: F(Q) = (p - c)/(p - s); expected profit from the standard normal loss function",
     latex=r"F(Q^*) = \frac{p - c}{p - s}, \quad Q^* = \mu + z\,\sigma",
     run=newsvendor_normal,
     verify=lambda i, o: max(abs(_STD.cdf((o["Q"] - i["mu"]) / i["sigma"]) - (i["p"] - i["c"]) / (i["p"] - i.get("s", 0.0))),
                             _rel(o["expected_profit"], (i["p"] - i.get("s", 0.0)) * (i["mu"] - o["expected_shortage"]) - (i["c"] - i.get("s", 0.0)) * o["Q"])),
     parse=lambda q: _parse_nv(q, "normal"), reference=_ref_nv_normal,
     wrong=lambda p, c, mu, sigma, s=0.0: (lambda cr: {"critical_ratio": cr, "Q": mu + _STD.inv_cdf(cr) * sigma, "expected_profit": 0.0, "expected_shortage": 0.0})((p - c) / p if s else c / p),
     random_inputs=lambda rng: (lambda c: {"p": c + rng.randint(1, 40), "c": c, "s": rng.choice([0, 0, round(c * 0.3, 2)]),
                                           "mu": rng.randint(50, 2000), "sigma": rng.randint(5, 300)})(rng.randint(2, 40)),
     keywords=["newsvendor", "single-period", "critical ratio"],
     templates=["Newsvendor problem: a paper sells for 2 and costs 0.5, leftovers are sold for salvage of 0.2, and demand is normal with mean 100 and standard deviation 20. Find the optimal order quantity."],
     examples=[{"inputs": {"p": 2, "c": 0.5, "s": 0.2, "mu": 100, "sigma": 20}, "outputs": {"critical_ratio": 1.5 / 1.8, "Q": 100 + 20 * _STD.inv_cdf(1.5 / 1.8)}},
               {"inputs": {"p": 10, "c": 5, "mu": 500, "sigma": 50}, "outputs": {"critical_ratio": 0.5, "Q": 500.0}}],
     questions=[{"q": "Newsvendor problem: a paper sells for 2 and costs 0.5, leftovers are sold for salvage of 0.2, and demand is normal with mean 100 and standard deviation 20. Find the optimal order quantity.",
                 "outputs": {"Q": 100 + 20 * _STD.inv_cdf(1.5 / 1.8)}},
                {"q": "Newsvendor: price 5, cost 6, demand normal with mean 100 and standard deviation 20. Find the order quantity.", "outcome": "INVALID_INPUT"}])


def newsvendor_uniform(p, c, lo, hi, s=0.0):
    _nv_check(p, c, s)
    _finite(lo, hi)
    if not 0 <= lo < hi:
        raise ValueError("uniform demand needs 0 <= lower < upper")
    cr = (p - c) / (p - s)
    Q = lo + cr * (hi - lo)
    loss = (hi - Q) ** 2 / (2 * (hi - lo))                     # E[(D - Q)^+]
    sales = (lo + hi) / 2 - loss
    return {"critical_ratio": cr, "Q": Q, "expected_profit": (p - s) * sales - (c - s) * Q}


def _ref_nv_uniform(p, c, lo, hi, s=0.0):
    """INDEPENDENT reference: maximise the integrated expected profit numerically (scipy quad + bounded search) and
    report the profit there."""
    from scipy import integrate, optimize
    def prof(Q):
        f = lambda x: (p * min(x, Q) + s * max(Q - x, 0.0) - c * Q) / (hi - lo)
        return integrate.quad(f, lo, Q, epsabs=1e-13, epsrel=1e-13)[0] + integrate.quad(f, Q, hi, epsabs=1e-13, epsrel=1e-13)[0]
    r = optimize.minimize_scalar(lambda Q: -prof(Q), bounds=(lo, hi), method="bounded", options={"xatol": 1e-11 * max(1.0, hi)})
    return {"expected_profit": prof(float(r.x))}


_add("newsvendor_uniform",
     name="Newsvendor order quantity (uniform demand)", topic="inventory management / single-period",
     purpose="The single-period order quantity that maximises expected profit when demand is uniform on [a, b].",
     inputs=_NV_IN + [_I("lo", "lowest possible demand a"), _I("hi", "highest possible demand b"), _I("s", "salvage value per unsold unit (default 0)", required=False)],
     outputs=[_O("critical_ratio", "critical ratio (p - c)/(p - s)"), _O("Q", "optimal order quantity a + CR (b - a)"), _O("expected_profit", "expected profit at Q")],
     primary="expected_profit", conditions=["salvage < cost < price", "0 <= a < b"],
     method="critical fractile of the uniform distribution", latex=r"Q^* = a + \frac{p - c}{p - s}(b - a)",
     run=newsvendor_uniform,
     verify=lambda i, o: max(_rel((o["Q"] - i["lo"]) / (i["hi"] - i["lo"]), (i["p"] - i["c"]) / (i["p"] - i.get("s", 0.0))),
                             _rel(o["expected_profit"], (i["p"] - i.get("s", 0.0)) * ((i["lo"] + i["hi"]) / 2 - (i["hi"] - o["Q"]) ** 2 / (2 * (i["hi"] - i["lo"])))
                                  - (i["c"] - i.get("s", 0.0)) * o["Q"])),
     parse=lambda q: _parse_nv(q, "uniform"), reference=_ref_nv_uniform,
     wrong=lambda p, c, lo, hi, s=0.0: newsvendor_uniform(p, c, lo, hi, s) | {"Q": lo + c / p * (hi - lo), "expected_profit": (p - c) * (lo + hi) / 2},
     random_inputs=lambda rng: (lambda c, lo: {"p": c + rng.randint(1, 30), "c": c, "s": rng.choice([0, 0, round(c / 4, 2)]), "lo": lo,
                                               "hi": lo + rng.randint(10, 1000)})(rng.randint(2, 30), rng.randint(0, 300)),
     keywords=["newsvendor", "uniform demand"],
     templates=["Newsvendor: an item sells for 10 and costs 4, with uniform demand between 100 and 300. Find the optimal order quantity."],
     examples=[{"inputs": {"p": 10, "c": 4, "lo": 100, "hi": 300}, "outputs": {"critical_ratio": 0.6, "Q": 220.0, "expected_profit": 960.0}}],
     questions=[{"q": "Newsvendor: an item sells for 10 and costs 4, with uniform demand between 100 and 300. Find the optimal order quantity.",
                 "outputs": {"Q": 220.0, "expected_profit": 960.0}}])


def newsvendor_discrete(p, c, demands, probs, s=0.0):
    _nv_check(p, c, s)
    if len(demands) != len(probs) or not demands:
        raise ValueError("give one probability per demand value")
    if any(pr < 0 for pr in probs) or abs(sum(probs) - 1) > 1e-9:
        raise ValueError("the probabilities must be >= 0 and sum to 1")
    if any(d2 <= d1 for d1, d2 in zip(demands, demands[1:])) or demands[0] < 0:
        raise ValueError("demand values must be >= 0 and increasing")
    P = [_fr(v) for v in probs]
    tot = sum(P)
    P = [v / tot for v in P]
    cr = (_fr(p) - _fr(c)) / (_fr(p) - _fr(s))
    cum, Q = F(0), demands[-1]
    for d, pr in zip(demands, P):
        cum += pr
        if cum >= cr:
            Q = d
            break
    Df = [_fr(d) for d in demands]
    Qf = _fr(Q)
    prof = sum(pr * (_fr(p) * min(d, Qf) + _fr(s) * max(Qf - d, 0) - _fr(c) * Qf) for d, pr in zip(Df, P))
    return {"critical_ratio": float(cr), "Q": float(Q), "expected_profit": float(prof)}


def _nv_disc_profit(p, c, s, demands, probs, Q):
    return sum(pr * (p * min(d, Q) + s * max(Q - d, 0) - c * Q) for d, pr in zip(demands, probs))


_add("newsvendor_discrete",
     name="Newsvendor order quantity (discrete demand)", topic="inventory management / single-period",
     purpose="For demand taking listed values with given probabilities, the order quantity that maximises expected "
             "profit (the smallest demand value whose cumulative probability reaches the critical ratio) and that profit.",
     inputs=_NV_IN + [_I("demands", "possible demand values, increasing", "vector", min_length=1, max_length=50),
                      _I("probs", "probability of each demand value (sum 1)", "vector", min_length=1, max_length=50),
                      _I("s", "salvage value per unsold unit (default 0)", required=False)],
     outputs=[_O("critical_ratio", "critical ratio (p - c)/(p - s)"), _O("Q", "optimal order quantity (smallest optimal value)"), _O("expected_profit", "maximum expected profit")],
     primary="expected_profit", conditions=["salvage < cost < price", "probabilities sum to 1", "order quantities restricted to the listed demand values"],
     method="cumulative distribution against the critical ratio, in exact fractions",
     latex=r"Q^* = \min\{ d : F(d) \ge \tfrac{p - c}{p - s} \}",
     run=newsvendor_discrete,
     verify=lambda i, o: max(_rel(o["expected_profit"], _nv_disc_profit(i["p"], i["c"], i.get("s", 0.0), i["demands"], i["probs"], o["Q"])),
                             max(max(0.0, _nv_disc_profit(i["p"], i["c"], i.get("s", 0.0), i["demands"], i["probs"], d) - o["expected_profit"])
                                 for d in i["demands"]) / max(1.0, abs(o["expected_profit"]))),
     parse=lambda q: _parse_nv(q, "discrete"),
     reference=lambda p, c, demands, probs, s=0.0: {"expected_profit": float(max(
         __import__("numpy").dot(probs, [p * min(d, Q) + s * max(Q - d, 0) - c * Q for d in demands]) for Q in demands))},
     wrong=lambda p, c, demands, probs, s=0.0: (lambda Q: {"Q": Q, "expected_profit": _nv_disc_profit(p, c, s, demands, probs, Q)})(
         sum(d * pr for d, pr in zip(demands, probs))),
     random_inputs=lambda rng: (lambda c, k: (lambda w: {"p": c + rng.randint(1, 30), "c": c, "s": rng.choice([0, 0, round(c / 3, 2)]),
                                                         "demands": sorted(rng.sample(range(0, 400, 10), k)),
                                                         "probs": [x / sum(w) for x in w]})([rng.randint(1, 10) for _ in range(k)]))(rng.randint(2, 20), rng.randint(3, 8)),
     keywords=["newsvendor", "discrete demand"],
     templates=["Newsvendor: a cake sells for 10 and costs 4; demand values [10, 20, 30, 40] have probabilities [0.1, 0.3, 0.4, 0.2]. How many should be ordered?"],
     examples=[{"inputs": {"p": 10, "c": 4, "demands": "10, 20, 30, 40", "probs": "0.1, 0.3, 0.4, 0.2"},
                "outputs": {"critical_ratio": 0.6, "Q": 30.0, "expected_profit": 130.0}}],
     questions=[{"q": "Newsvendor: a cake sells for 10 and costs 4; demand values [10, 20, 30, 40] have probabilities [0.1, 0.3, 0.4, 0.2]. How many should be ordered?",
                 "outputs": {"Q": 30.0, "expected_profit": 130.0}},
                {"q": "Newsvendor: a cake sells for 10 and costs 4; demand values [10, 20, 30, 40] have probabilities [0.1, 0.3, 0.4, 0.4]. How many should be ordered?",
                 "outcome": "INVALID_INPUT"}])


def littlewood(p1, p2, mu, sigma, capacity=None):
    _finite(p1, p2, mu, sigma)
    if not 0 < p2 < p1:
        raise ValueError("the discount fare must be positive and below the full fare")
    if sigma <= 0:
        raise ValueError("the standard deviation of full-fare demand must be > 0")
    z = _STD.inv_cdf(1 - p2 / p1)
    y = mu + z * sigma
    out = {"protection_level": y, "z": z}
    if capacity is not None:
        _finite(capacity)
        if capacity <= 0:
            raise ValueError("capacity must be > 0")
        out["booking_limit"] = max(0.0, capacity - max(y, 0.0))
    return out


_add("littlewood_protection",
     name="Littlewood's rule (two-fare protection level)", topic="revenue management",
     purpose="How many seats (units) to protect for full-fare demand when a discount fare is also sold: the protection "
             "level and, with a capacity, the booking limit for the discount class.",
     inputs=[_I("p1", "full fare"), _I("p2", "discount fare"), _I("mu", "mean full-fare demand"), _I("sigma", "standard deviation of full-fare demand"),
             _I("capacity", "total capacity (optional)", required=False)],
     outputs=[_O("protection_level", "units protected for full-fare demand"), _O("z", "standard normal quantile of 1 - p2/p1"),
              _O("booking_limit", "discount booking limit = capacity - protection (when a capacity is given)")],
     primary="protection_level", conditions=["0 < p2 < p1", "normally distributed full-fare demand", "continuous approximation (no rounding)"],
     method="protect y with P(D1 > y) = p2/p1", latex=r"P(D_1 > y^*) = \frac{p_2}{p_1}, \quad y^* = \mu + z\,\sigma",
     run=littlewood,
     verify=lambda i, o: max(abs((1 - _STD.cdf((o["protection_level"] - i["mu"]) / i["sigma"])) - i["p2"] / i["p1"]),
                             _rel(o["protection_level"], i["mu"] + o["z"] * i["sigma"])),
     parse=lambda q: (lambda ql: None if not re.search(r"\b(littlewood|protection level|protect|booking limit)\b", ql) else {
         k: v for k, v in (("p1", _num_after(ql, r"(?:full|high|regular)[- ]fare(?: of| is| =|:)?|full price(?: of| is)?")),
                           ("p2", _num_after(ql, r"(?:discount(?:ed)?|low)[- ]fare(?: of| is| =|:)?|discount price(?: of| is)?")),
                           ("mu", _num_after(ql, r"mean(?: of)?(?: full[- ]fare)?(?: demand)?(?: is| =|:)?")),
                           ("sigma", _num_after(ql, r"standard deviation(?: of)?(?: demand)?(?: is| =|:)?")),
                           ("capacity", _num_after(ql, r"capacity(?: of| is| =|:)?|(?:has|with) (?=\d)"))) if v is not None})(q.lower()),
     reference=lambda p1, p2, mu, sigma, capacity=None: {"protection_level": float(__import__("scipy.stats", fromlist=["norm"]).norm.isf(p2 / p1, mu, sigma))},
     wrong=lambda p1, p2, mu, sigma, capacity=None: (lambda z: {"protection_level": mu + z * sigma, "z": z})(_STD.inv_cdf(p2 / p1)),
     random_inputs=lambda rng: (lambda p1: {"p1": p1, "p2": round(p1 * rng.uniform(0.2, 0.9), 2), "mu": rng.randint(10, 300),
                                            "sigma": rng.randint(2, 80)})(rng.randint(100, 900)),
     keywords=["littlewood", "protection level", "booking limit"],
     templates=["Littlewood's rule: the full fare is 400 and the discount fare is 150; full-fare demand has mean 60 and standard deviation 20. Find the protection level."],
     examples=[{"inputs": {"p1": 400, "p2": 150, "mu": 60, "sigma": 20}, "outputs": {"protection_level": 60 + 20 * _STD.inv_cdf(1 - 150 / 400)}},
               {"inputs": {"p1": 200, "p2": 100, "mu": 50, "sigma": 10, "capacity": 120}, "outputs": {"protection_level": 50.0, "booking_limit": 70.0}}],
     questions=[{"q": "Littlewood's rule: the full fare is 400 and the discount fare is 150; full-fare demand has mean 60 and standard deviation 20. Find the protection level.",
                 "outputs": {"protection_level": 60 + 20 * _STD.inv_cdf(1 - 150 / 400)}}])


# =================================================================================================================
# queueing: closed forms in the shipped code; references solve the birth-death Markov chain numerically
# =================================================================================================================
def _bd_reference(birth, death, c=1):
    """INDEPENDENT reference: stationary distribution of a birth-death chain (states 0..N) by a sparse linear solve of
    pi Q = 0, sum pi = 1; returns its queue metrics."""
    import numpy as np
    from scipy.sparse import lil_matrix
    from scipy.sparse.linalg import spsolve
    N = len(birth)
    Q = lil_matrix((N, N))
    for n in range(N):
        out = 0.0
        if n + 1 < N and birth[n] > 0:
            Q[n, n + 1] = birth[n]
            out += birth[n]
        if n > 0:
            Q[n, n - 1] = death[n]
            out += death[n]
        Q[n, n] = -out
    A = Q.T.tolil()
    A[N - 1, :] = np.ones(N)
    rhs = np.zeros(N)
    rhs[-1] = 1.0
    pi = spsolve(A.tocsc(), rhs)
    n = np.arange(N)
    L = float(n @ pi)
    Lq = float(np.maximum(n - c, 0) @ pi)
    lam_eff = float(np.array([birth[k] if k + 1 < N else 0.0 for k in range(N)]) @ pi)
    return {"pi": pi, "L": L, "Lq": Lq, "lam_eff": lam_eff, "W": L / lam_eff, "Wq": Lq / lam_eff, "P0": float(pi[0]),
            "P_wait": float(pi[c:].sum()), "P_last": float(pi[-1])}


def _trunc(rho, c=1):
    return int(min(6000, max(200, math.ceil(math.log(1e-26) / math.log(rho)) + c + 20)))


def _rates(ql):
    out = {}
    lam = _num_after(ql, r"arrival rate(?: of| is| =|:)?|λ\s*=|lambda\s*=|arriv(?:e|es|ing|als)(?: occur)?(?: on average)?(?: at)?(?: a rate of)?(?: an average of)?")
    mu = _num_after(ql, r"service rate(?: of| is| =|:)?(?: each server)?|μ\s*=|mu\s*=|served at(?: a rate of)?|serves?(?: customers)? at(?: a rate of)?|"
                        r"can serve(?: an average of)?|service capacity of|(?:each|every) (?:server|teller|agent|clerk|operator) (?:can )?(?:serves?|handles?)")
    if lam is not None:
        out["lam"] = lam
    if mu is not None:
        out["mu"] = mu
    return out


def _servers(ql):
    m = re.search(r"\bm/m/(\d+)\b(?!\s*/)", ql) or re.search(r"\b(\d+)\s+(?:identical |parallel )?(?:servers|tellers|channels|agents|clerks|windows|pumps|counters|doctors|operators)\b", ql) \
        or re.search(r"\bc\s*=\s*(\d+)", ql)
    return int(m.group(1)) if m else None


def _no_service_time(ql):
    return not re.search(r"\bservice times?\b|\bminutes? per customer\b|\binterarrival\b", ql)


def mm1(lam, mu):
    _finite(lam, mu)
    if lam <= 0 or mu <= 0:
        raise ValueError("the arrival and service rates must be > 0")
    if lam >= mu:
        raise ValueError("the queue is unstable: the arrival rate must be below the service rate")
    rho = lam / mu
    return {"rho": rho, "L": rho / (1 - rho), "Lq": rho * rho / (1 - rho), "W": 1 / (mu - lam), "Wq": rho / (mu - lam),
            "P0": 1 - rho}


def _verify_mm1(i, o):
    lam, mu = i["lam"], i["mu"]
    return max(_rel(o["L"], lam * o["W"]), _rel(o["Lq"], lam * o["Wq"]), _rel(o["W"] - o["Wq"], 1 / mu),
               abs(o["W"] * (mu - lam) - 1), _rel(o["P0"], 1 - lam / mu))


def _parse_mm1(q):
    ql = q.lower()
    if not (re.search(r"\bm/m/1\b(?!\s*/)", ql) or re.search(r"\bsingle[- ]server\b|\bone server\b", ql)):
        return None
    if re.search(r"\b(capacity|at most \d+|room for|maximum of \d+ customers|finite)\b", ql) or not _no_service_time(ql):
        return None
    if not re.search(r"\b(queue|queu|waiting line|customers?|arriv)", ql):
        return None
    return _rates(ql)


_add("mm1_queue",
     name="M/M/1 queue", topic="queueing theory",
     purpose="Steady-state measures of a single-server queue with Poisson arrivals and exponential service: utilisation, "
             "mean numbers in system and in queue, mean times in system and in queue, probability of an empty system.",
     inputs=[_I("lam", "arrival rate (customers per unit time)"), _I("mu", "service rate (customers per unit time)")],
     outputs=[_O("rho", "utilisation lambda/mu"), _O("L", "mean number in the system"), _O("Lq", "mean number waiting"),
              _O("W", "mean time in the system (time unit of the rates)"), _O("Wq", "mean waiting time in the queue"), _O("P0", "probability the system is empty")],
     primary="L", conditions=["lambda < mu (stable)", "Poisson arrivals, exponential service, one server, unlimited capacity"],
     method="geometric stationary distribution; Little's law", latex=r"L = \frac{\rho}{1-\rho},\; W = \frac{1}{\mu - \lambda},\; W_q = \frac{\rho}{\mu-\lambda}",
     run=mm1, verify=_verify_mm1, parse=_parse_mm1,
     reference=lambda lam, mu: (lambda r: {k: r[k] for k in ("L", "Lq", "W", "Wq", "P0")})(
         _bd_reference([lam] * _trunc(lam / mu) + [0.0], [0.0] + [mu] * _trunc(lam / mu))),
     wrong=lambda lam, mu: (lambda rho: {"rho": rho, "L": rho / (1 - rho) ** 2, "Lq": rho / (1 - rho), "W": rho / (1 - rho) ** 2 / lam,
                                         "Wq": rho / (1 - rho) / lam, "P0": 1 - rho})(lam / mu),
     random_inputs=lambda rng: (lambda mu: {"mu": mu, "lam": round(mu * rng.uniform(0.05, 0.95), 2) or 0.01})(rng.randint(2, 100)),
     keywords=["m/m/1", "single server queue"],
     templates=["In an M/M/1 queue customers arrive at 8 per hour and are served at 10 per hour. Find L, Lq, W and Wq."],
     examples=[{"inputs": {"lam": 8, "mu": 10}, "outputs": {"rho": 0.8, "L": 4.0, "Lq": 3.2, "W": 0.5, "Wq": 0.4, "P0": 0.2}},
               {"inputs": {"lam": 2, "mu": 3}, "outputs": {"L": 2.0, "W": 1.0}}],
     questions=[{"q": "In an M/M/1 queue customers arrive at 8 per hour and are served at 10 per hour. Find L, Lq, W and Wq.",
                 "outputs": {"L": 4.0, "Lq": 3.2, "W": 0.5, "Wq": 0.4}},
                {"q": "A single-server queue has an arrival rate of 2 per minute and a service rate of 3 per minute. What is the mean time in the system?",
                 "outputs": {"W": 1.0}},
                {"q": "In an M/M/1 queue customers arrive at 12 per hour and are served at 10 per hour. Find L.", "outcome": "INVALID_INPUT"}])


def _erlang_b(a, c):
    B = 1.0
    for k in range(1, c + 1):
        B = a * B / (k + a * B)
    return B


def mmc(lam, mu, c):
    _finite(lam, mu)
    if lam <= 0 or mu <= 0:
        raise ValueError("the arrival and service rates must be > 0")
    if c < 1 or c > 500:
        raise ValueError("the number of servers must be between 1 and 500")
    a, rho = lam / mu, lam / (c * mu)
    if rho >= 1:
        raise ValueError("the queue is unstable: lambda must be below c * mu")
    B = _erlang_b(a, c)
    C = B / (1 - rho * (1 - B))
    Lq = C * rho / (1 - rho)
    Wq = Lq / lam
    W = Wq + 1 / mu
    log_t = c * math.log(a) - math.lgamma(c + 1) - math.log(1 - rho)
    P0 = C / math.exp(log_t)
    return {"rho": rho, "P_wait": C, "Lq": Lq, "Wq": Wq, "W": W, "L": lam * W, "P0": P0}


def _verify_mmc(i, o):
    lam, mu, c = i["lam"], i["mu"], int(i["c"])
    a, rho = lam / mu, lam / (c * mu)
    terms, t = 0.0, 1.0
    for n in range(c):                                   # P0 from its defining series (not from Erlang B)
        terms += t
        t *= a / (n + 1)
    p0 = 1 / (terms + t / (1 - rho))
    return max(_rr(o["L"], lam * o["W"]), _rr(o["Lq"], lam * o["Wq"]), _rr(o["W"] - o["Wq"], 1 / mu),
               _rr(o["P0"], p0), _rr(o["P_wait"], p0 * t / (1 - rho)), _rr(o["Lq"], o["P_wait"] * rho / (1 - rho)))


def _parse_mmc(q):
    ql = q.lower()
    c = _servers(ql)
    if c is None or re.search(r"\bm/m/1\b", ql) or re.search(r"\bm/m/\d+/\d+", ql):
        return None
    if re.search(r"\b(capacity|at most \d+|room for|blocked|lost|erlang b|finite|machines?|repair)\b", ql) or not _no_service_time(ql):
        return None
    if re.search(r"\bhow many (?:servers|tellers|agents|clerks|operators)\b|\bminimum number of\b|\bstaff", ql):
        return None
    if not re.search(r"\b(queue|queu|waiting|customers?|arriv)", ql):
        return None
    out = _rates(ql)
    out["c"] = c
    return out


_add("mmc_queue",
     name="M/M/c queue (Erlang C)", topic="queueing theory",
     purpose="Steady-state measures of a queue with c parallel identical servers, Poisson arrivals and exponential "
             "service: probability of waiting (Erlang C), mean queue length and waiting times.",
     inputs=[_I("lam", "arrival rate"), _I("mu", "service rate of each server"), _I("c", "number of servers", "integer", min=1, max=500)],
     outputs=[_O("rho", "utilisation lambda/(c mu)"), _O("P_wait", "probability an arrival waits (Erlang C)"), _O("Lq", "mean number waiting"),
              _O("Wq", "mean waiting time"), _O("W", "mean time in the system"), _O("L", "mean number in the system"), _O("P0", "probability the system is empty")],
     primary="Lq", conditions=["lambda < c mu (stable)", "unlimited queue capacity"],
     method="Erlang B by its stable recursion, then Erlang C = B / (1 - rho (1 - B)); Little's law",
     latex=r"C(c, a) = \frac{B(c,a)}{1 - \rho(1 - B(c,a))}, \quad L_q = C\,\frac{\rho}{1-\rho}",
     run=mmc, verify=_verify_mmc, parse=_parse_mmc,
     reference=lambda lam, mu, c: (lambda r: {"P_wait": r["P_wait"], "Lq": r["Lq"], "Wq": r["Wq"], "W": r["W"], "L": r["L"], "P0": r["P0"]})(
         (lambda N: _bd_reference([lam] * N + [0.0], [0.0] + [min(n, c) * mu for n in range(1, N + 1)], c))(_trunc(lam / (c * mu), c))),
     wrong=lambda lam, mu, c: (lambda r: r | {"Lq": r["P_wait"] * r["rho"], "Wq": r["P_wait"] * r["rho"] / lam})(mmc(lam, mu, c)),
     random_inputs=lambda rng: (lambda c, mu: {"c": c, "mu": mu, "lam": round(c * mu * rng.uniform(0.45, 0.92), 2)})(rng.randint(1, 12), rng.randint(1, 30)),
     keywords=["m/m/c", "erlang c", "servers"],
     templates=["Customers arrive at 24 per hour at a bank with 3 tellers; each teller serves 10 per hour. Find the probability of waiting and Lq for this M/M/c queue."],
     examples=[{"inputs": {"lam": 2, "mu": 3, "c": 2}, "outputs": {"Lq": 1 / 12, "P0": 0.5}},
               {"inputs": {"lam": 2, "mu": 3, "c": 1}, "outputs": {"L": 2.0, "Lq": 4 / 3}}],
     questions=[{"q": "Customers arrive at 24 per hour at a bank with 3 tellers; each teller serves 10 per hour. Find the probability of waiting and Lq for this M/M/c queue.",
                 "outputs": {"rho": 0.8}},
                {"q": "An M/M/2 queue has arrival rate 2 per hour and service rate 3 per hour. Find Lq.", "outputs": {"Lq": 1 / 12}},
                {"q": "An M/M/2 queue has arrival rate 8 per hour and service rate 3 per hour. Find Lq.", "outcome": "INVALID_INPUT"}])


def mmc_finite(lam, mu, c, K):
    _finite(lam, mu)
    if lam <= 0 or mu <= 0:
        raise ValueError("the arrival and service rates must be > 0")
    if not 1 <= c <= K <= 2000:
        raise ValueError("need 1 <= servers <= system capacity <= 2000")
    a = lam / mu
    logs = []
    for n in range(K + 1):
        if n <= c:
            logs.append(n * math.log(a) - math.lgamma(n + 1))
        else:
            logs.append(c * math.log(a) - math.lgamma(c + 1) + (n - c) * math.log(a / c))
    mx = max(logs)
    w = [math.exp(v - mx) for v in logs]
    tot = math.fsum(w)
    pi = [v / tot for v in w]
    L = math.fsum(n * p for n, p in enumerate(pi))
    Lq = math.fsum(max(n - c, 0) * p for n, p in enumerate(pi))
    lam_eff = lam * (1 - pi[K])
    return {"P_block": pi[K], "lam_eff": lam_eff, "L": L, "Lq": Lq, "W": L / lam_eff, "Wq": Lq / lam_eff, "P0": pi[0]}


def _verify_mmck(i, o):
    lam, mu = i["lam"], i["mu"]
    return max(_rel(o["lam_eff"], lam * (1 - o["P_block"])), _rel(o["L"], o["lam_eff"] * o["W"]), _rel(o["Lq"], o["lam_eff"] * o["Wq"]),
               _rel(o["L"] - o["Lq"], o["lam_eff"] / mu), _rel(o["W"] - o["Wq"], 1 / mu))


def _parse_mmck(q):
    ql = q.lower()
    m = re.search(r"\bm/m/(\d+)/(\d+)\b", ql)
    if m:
        c, K = int(m.group(1)), int(m.group(2))
    else:
        mk = re.search(r"(?:capacity of|at most|room for|maximum of|holds? at most|limit of)\s*(\d+)", ql)
        if not mk or not re.search(r"\b(queue|customers?|arriv)", ql) or re.search(r"\b(machines?|repair|erlang b)\b", ql):
            return None
        c = _servers(ql) or (1 if re.search(r"\bsingle[- ]server\b|\bone server\b|\bm/m/1\b", ql) else None)
        K = int(mk.group(1))
        if c is None:
            return None
    if not _no_service_time(ql):
        return None
    out = _rates(ql)
    out["c"], out["K"] = c, K
    return out


_add("mmck_finite_queue",
     name="M/M/c/K queue (finite capacity)", topic="queueing theory",
     purpose="Steady-state measures of a queue with c servers and room for at most K customers in the system (arrivals "
             "finding it full are lost): blocking probability, effective arrival rate, mean numbers and times.",
     inputs=[_I("lam", "arrival rate"), _I("mu", "service rate of each server"), _I("c", "number of servers", "integer", min=1),
             _I("K", "system capacity (customers in service plus waiting)", "integer", min=1)],
     outputs=[_O("P_block", "probability an arrival is lost (system full)"), _O("lam_eff", "effective arrival rate"),
              _O("L", "mean number in the system"), _O("Lq", "mean number waiting"), _O("W", "mean time in the system"),
              _O("Wq", "mean waiting time"), _O("P0", "probability the system is empty")],
     primary="L", conditions=["1 <= c <= K", "any utilisation (finite capacity is always stable)"],
     method="truncated birth-death distribution computed in log space; Little's law with the effective arrival rate",
     latex=r"\pi_n \propto \frac{a^n}{n!}\ (n \le c),\quad \frac{a^c}{c!}\left(\frac{a}{c}\right)^{n-c}\ (c < n \le K)",
     run=mmc_finite, verify=_verify_mmck, parse=_parse_mmck,
     reference=lambda lam, mu, c, K: (lambda r: {"P_block": r["P_last"], "lam_eff": r["lam_eff"], "L": r["L"], "Lq": r["Lq"], "W": r["W"], "Wq": r["Wq"], "P0": r["P0"]})(
         _bd_reference([lam] * K + [0.0], [0.0] + [min(n, c) * mu for n in range(1, K + 1)], c)),
     wrong=lambda lam, mu, c, K: (lambda r: r | {"W": r["L"] / lam, "Wq": r["Lq"] / lam, "lam_eff": lam})(mmc_finite(lam, mu, c, K)),
     random_inputs=lambda rng: (lambda c: {"c": c, "K": c + rng.randint(0, 30), "mu": rng.randint(1, 20), "lam": rng.randint(1, 60)})(rng.randint(1, 6)),
     keywords=["finite capacity", "m/m/1/k", "m/m/c/k", "blocking"],
     templates=["An M/M/1/3 queue has arrival rate 2 per hour and service rate 2 per hour. Find the blocking probability and L.",
                "A single-server queue with a capacity of 5 customers has arrival rate 4 per hour and service rate 5 per hour. Find L."],
     examples=[{"inputs": {"lam": 2, "mu": 2, "c": 1, "K": 3}, "outputs": {"P_block": 0.25, "L": 1.5, "lam_eff": 1.5}},
               {"inputs": {"lam": 3, "mu": 1, "c": 2, "K": 2}, "outputs": {"P_block": 4.5 / 8.5}}],
     questions=[{"q": "An M/M/1/3 queue has arrival rate 2 per hour and service rate 2 per hour. Find the blocking probability and L.",
                 "outputs": {"P_block": 0.25, "L": 1.5}},
                {"q": "An M/M/2/2 queue has arrival rate 3 per hour and service rate 1 per hour. Find the probability an arrival is lost.",
                 "outputs": {"P_block": 4.5 / 8.5}}])


def erlang_b(A, c):
    _finite(A)
    if A <= 0:
        raise ValueError("the offered traffic must be > 0")
    if not 1 <= c <= 5000:
        raise ValueError("the number of servers must be between 1 and 5000")
    B = _erlang_b(A, c)
    return {"blocking_probability": B, "carried_traffic": A * (1 - B), "lost_traffic": A * B}


def _verify_erlang_b(i, o):
    A, c = i["A"], int(i["c"])
    logs = [n * math.log(A) - math.lgamma(n + 1) for n in range(c + 1)]   # B from its defining series
    mx = max(logs)
    B = math.exp(logs[-1] - mx) / math.fsum(math.exp(v - mx) for v in logs)
    return max(_rr(o["blocking_probability"], B), _rr(o["carried_traffic"], A * (1 - o["blocking_probability"])))


def _parse_erlang_b(q):
    ql = q.lower()
    if not re.search(r"\b(erlang b|erlang-b|loss system|blocking probability|probability (?:that )?a call is blocked|blocked calls?)\b", ql) \
            or re.search(r"\bm/m/\d+/\d+\b|capacity of", ql):
        return None
    out = {}
    a = _num_after(ql, r"offered (?:traffic|load)(?: of| is| =|:)?|traffic (?:intensity )?of|load of")
    if a is not None:
        out["A"] = a
    m = re.search(r"\b(\d+)\s+(?:servers|lines|trunks|channels|circuits|operators)\b", ql)
    if m:
        out["c"] = int(m.group(1))
    return out


_add("erlang_b",
     name="Erlang B blocking probability (M/M/c/c loss system)", topic="queueing theory / teletraffic",
     purpose="The probability that a call (customer) is blocked in a loss system with c servers and no waiting room, "
             "for an offered traffic of A erlangs, with the carried and lost traffic.",
     inputs=[_I("A", "offered traffic in erlangs (arrival rate x mean holding time)"), _I("c", "number of servers (lines, trunks)", "integer", min=1, max=5000)],
     outputs=[_O("blocking_probability", "Erlang B blocking probability"), _O("carried_traffic", "carried traffic A (1 - B), erlangs"),
              _O("lost_traffic", "lost traffic A B, erlangs")],
     primary="blocking_probability", conditions=["blocked customers are lost (no waiting)", "A > 0"],
     method="stable recursion B(k) = A B(k-1) / (k + A B(k-1))", latex=r"B(c, A) = \frac{A^c / c!}{\sum_{k=0}^{c} A^k / k!}",
     run=erlang_b, verify=_verify_erlang_b, parse=_parse_erlang_b,
     reference=lambda A, c: {"blocking_probability": _bd_reference([A] * c + [0.0], [0.0] + [float(n) for n in range(1, c + 1)], c)["P_last"]},
     wrong=lambda A, c: (lambda B: {"blocking_probability": B, "carried_traffic": A * (1 - B), "lost_traffic": A * B})(_erlang_b(A, c - 1) if c > 1 else 1.0),
     random_inputs=lambda rng: (lambda c: {"A": round(c * rng.uniform(0.4, 1.4), 2), "c": c})(rng.randint(1, 80)),
     keywords=["erlang b", "blocking probability", "loss system"],
     templates=["A trunk group of 5 lines is offered traffic of 3 erlangs. What is the Erlang B blocking probability?"],
     examples=[{"inputs": {"A": 3, "c": 5}, "outputs": {"blocking_probability": (243 / 120) / (1 + 3 + 4.5 + 4.5 + 81 / 24 + 243 / 120)}},
               {"inputs": {"A": 1, "c": 1}, "outputs": {"blocking_probability": 0.5}}],
     questions=[{"q": "A trunk group of 5 lines is offered traffic of 3 erlangs. What is the Erlang B blocking probability?",
                 "outputs": {"blocking_probability": (243 / 120) / (1 + 3 + 4.5 + 4.5 + 81 / 24 + 243 / 120)}}])


def machine_repair(N, lam, mu, c=1):
    _finite(lam, mu)
    if lam <= 0 or mu <= 0:
        raise ValueError("the failure and repair rates must be > 0")
    if not 1 <= c <= N <= 1000:
        raise ValueError("need 1 <= repairers <= machines <= 1000")
    r = lam / mu
    logs = []
    for n in range(N + 1):
        base = math.lgamma(N + 1) - math.lgamma(N - n + 1) + n * math.log(r)
        logs.append(base - (math.lgamma(n + 1) if n <= c else math.lgamma(c + 1) + (n - c) * math.log(c)))
    mx = max(logs)
    w = [math.exp(v - mx) for v in logs]
    tot = math.fsum(w)
    pi = [v / tot for v in w]
    L = math.fsum(n * p for n, p in enumerate(pi))
    Lq = math.fsum(max(n - c, 0) * p for n, p in enumerate(pi))
    lam_eff = lam * (N - L)
    return {"L": L, "Lq": Lq, "lam_eff": lam_eff, "W": L / lam_eff, "Wq": Lq / lam_eff, "P0": pi[0], "working": N - L}


def _parse_repair(q):
    ql = q.lower()
    if not re.search(r"\b(machines?|looms?|robots?|computers?)\b", ql) or not re.search(r"\b(repair|break(?:s|down)?|fail)", ql):
        return None
    out = {}
    m = re.search(r"\b(\d+)\s+(?:identical )?(?:machines|looms|robots|computers)\b", ql)
    if m:
        out["N"] = int(m.group(1))
    lam = _num_after(ql, r"(?:breaks? down|fails?|failure rate)(?: at)?(?: a rate of)?(?: on average)?(?: of)?")
    mu = _num_after(ql, r"repairs? at(?: a rate of)?|repair rate(?: of| is)?|fix(?:es)? at(?: a rate of)?")
    if lam is not None:
        out["lam"] = lam
    if mu is not None:
        out["mu"] = mu
    m = re.search(r"\b(\d+)\s+(?:repair(?:ers|men|people)|mechanics|technicians)\b", ql)
    if m:
        out["c"] = int(m.group(1))
    elif re.search(r"\b(?:one|a single) (?:repair(?:er|man|person)|mechanic|technician)\b", ql):
        out["c"] = 1
    return out


_add("machine_repair_queue",
     name="Machine-repair (finite-source) queue", topic="queueing theory",
     purpose="N machines each fail at rate lambda while running and c repairers fix them at rate mu each: mean number "
             "of machines down and waiting, failure rate, downtime and the mean number working.",
     inputs=[_I("N", "number of machines", "integer", min=1, max=1000), _I("lam", "failure rate of one running machine"),
             _I("mu", "repair rate of one repairer"), _I("c", "number of repairers (default 1)", "integer", required=False, min=1)],
     outputs=[_O("L", "mean number of machines down"), _O("Lq", "mean number waiting for repair"), _O("lam_eff", "mean failure rate lambda (N - L)"),
              _O("W", "mean downtime per failure"), _O("Wq", "mean wait for a repairer"), _O("P0", "probability all machines run"), _O("working", "mean number running")],
     primary="L", conditions=["1 <= c <= N", "exponential failure and repair times"],
     method="finite birth-death distribution (birth (N - n) lambda, death min(n, c) mu) in log space; Little's law",
     latex=r"\pi_n \propto \frac{N!}{(N-n)!}\,\frac{(\lambda/\mu)^n}{n!\ \text{or}\ c!\,c^{n-c}}",
     run=machine_repair,
     verify=lambda i, o: max(_rel(o["lam_eff"], i["lam"] * (i["N"] - o["L"])), _rel(o["L"], o["lam_eff"] * o["W"]), _rel(o["Lq"], o["lam_eff"] * o["Wq"]),
                             _rel(o["L"] - o["Lq"], o["lam_eff"] / i["mu"]), _rel(o["working"], i["N"] - o["L"])),
     parse=_parse_repair,
     reference=lambda N, lam, mu, c=1: (lambda r: {"L": r["L"], "Lq": r["Lq"], "lam_eff": r["lam_eff"], "W": r["W"], "P0": r["P0"]})(
         _bd_reference([(N - n) * lam for n in range(N)] + [0.0], [0.0] + [min(n, c) * mu for n in range(1, N + 1)], c)),
     wrong=lambda N, lam, mu, c=1: (lambda r: r | {"lam_eff": lam * N, "W": r["L"] / (lam * N), "Wq": r["Lq"] / (lam * N)})(machine_repair(N, lam, mu, c)),
     random_inputs=lambda rng: (lambda N: {"N": N, "c": rng.randint(1, min(N, 4)), "lam": round(rng.uniform(0.01, 2), 3), "mu": round(rng.uniform(0.2, 10), 2)})(rng.randint(1, 30)),
     keywords=["machine repair", "finite source", "machine interference"],
     templates=["There are 5 machines; each machine breaks down at a rate of 0.2 per hour and one repairer repairs at a rate of 1 per hour. Find the mean number of machines down."],
     examples=[{"inputs": {"N": 2, "lam": 1, "mu": 1, "c": 1}, "outputs": {"L": 1.2, "P0": 0.2}},
               {"inputs": {"N": 5, "lam": 0.2, "mu": 1, "c": 1}, "outputs": {"P0": 1 / (1 + 1 + 0.8 + 0.48 + 0.192 + 0.0384)}}],
     questions=[{"q": "There are 5 machines; each machine breaks down at a rate of 0.2 per hour and one repairer repairs at a rate of 1 per hour. Find the mean number of machines down.",
                 "outputs": {"P0": 1 / (1 + 1 + 0.8 + 0.48 + 0.192 + 0.0384)}}])


def mg1(lam, ES, sd):
    _finite(lam, ES, sd)
    if lam <= 0 or ES <= 0 or sd < 0:
        raise ValueError("the arrival rate and mean service time must be > 0 and the standard deviation >= 0")
    rho = lam * ES
    if rho >= 1:
        raise ValueError("the queue is unstable: lambda times the mean service time must be below 1")
    Lq = (lam * lam * sd * sd + rho * rho) / (2 * (1 - rho))
    Wq = Lq / lam
    return {"rho": rho, "Lq": Lq, "L": Lq + rho, "Wq": Wq, "W": Wq + ES}


def _ref_mg1(lam, ES, sd):
    """INDEPENDENT reference: the M/G/1 embedded Markov chain at departures, with a gamma service time of the given
    mean and standard deviation (deterministic when sd = 0): arrivals per service a_k from scipy (Poisson, or the
    gamma-Poisson negative binomial), tails from sf; the chain solved by the stable all-positive recursion
    a_0 pi_j = pi_0 abar_j + sum_{i=1}^{j-1} pi_i abar_{j-i+1}, pi_0 = 1 - rho; L is its mean (departure-epoch and
    time-average distributions coincide for M/G/1)."""
    import numpy as np
    from scipy import stats
    N = 20000
    k = np.arange(N + 2)
    if sd == 0:
        dist = stats.poisson(lam * ES)
    else:
        alpha, theta = (ES / sd) ** 2, sd * sd / ES
        dist = stats.nbinom(alpha, 1 / (1 + lam * theta))
    a0 = float(dist.pmf(0))
    abar = dist.sf(k - 1)                                 # abar[k] = P(A >= k)
    pi = [1.0 - lam * ES]
    tot, L = pi[0], 0.0
    for j in range(1, N):
        v = (pi[0] * abar[j] + (float(np.dot(pi[1:j], abar[j:1:-1])) if j > 1 else 0.0)) / a0
        pi.append(v)
        tot += v
        L += j * v
        if v < 1e-19 * max(1.0, L) and j > 5:
            break
    return {"L": L, "Lq": L - lam * ES, "W": L / lam}


def _parse_mg1(q):
    ql = q.lower()
    if not re.search(r"\bm/g/1\b|\bm/d/1\b|pollaczek|general service", ql):
        return None
    out = {}
    lam = _num_after(ql, r"arrival rate(?: of| is| =|:)?|λ\s*=|arriv(?:e|es|als)(?: at)?(?: a rate of)?")
    es = _num_after(ql, r"(?:mean |average |constant )?service time(?: of| is| =|:)?")
    sd = 0.0 if re.search(r"\bm/d/1\b|constant service|deterministic", ql) else _num_after(ql, r"standard deviation(?: of)?(?: the service time)?(?: of| is| =|:)?")
    for kk, v in (("lam", lam), ("ES", es), ("sd", sd)):
        if v is not None:
            out[kk] = v
    return out


_add("mg1_pollaczek_khinchine",
     name="M/G/1 queue (Pollaczek-Khinchine formula)", topic="queueing theory",
     purpose="Mean queue length and waiting times of a single-server queue with Poisson arrivals and a GENERAL service "
             "time distribution given by its mean and standard deviation (M/D/1 when the standard deviation is 0).",
     inputs=[_I("lam", "arrival rate"), _I("ES", "mean service time (time unit of 1/lambda)"), _I("sd", "standard deviation of the service time")],
     outputs=[_O("rho", "utilisation lambda E[S]"), _O("Lq", "mean number waiting"), _O("L", "mean number in the system"),
              _O("Wq", "mean waiting time"), _O("W", "mean time in the system")],
     primary="Lq", conditions=["lambda E[S] < 1", "Poisson arrivals, one server, first come first served"],
     method="Pollaczek-Khinchine mean-value formula; Little's law", latex=r"L_q = \frac{\lambda^2 \sigma_S^2 + \rho^2}{2(1-\rho)}",
     run=mg1,
     verify=lambda i, o: max(_rel(2 * (1 - o["rho"]) * o["Lq"], i["lam"] ** 2 * i["sd"] ** 2 + o["rho"] ** 2), _rel(o["Lq"], i["lam"] * o["Wq"]),
                             _rel(o["W"] - o["Wq"], i["ES"]), _rel(o["L"], i["lam"] * o["W"]), _rel(o["rho"], i["lam"] * i["ES"])),
     parse=_parse_mg1, reference=_ref_mg1,
     wrong=lambda lam, ES, sd: (lambda r: r | {"Lq": 2 * r["Lq"], "Wq": 2 * r["Wq"]})(mg1(lam, ES, sd)),
     random_inputs=lambda rng: (lambda ES: {"ES": ES, "lam": round(rng.uniform(0.2, 0.9) / ES, 4), "sd": rng.choice([0.0, round(ES * rng.uniform(0.1, 1.2), 3)])})(rng.choice([0.5, 1, 2, 4, 5, 10])),
     keywords=["m/g/1", "pollaczek-khinchine", "m/d/1"],
     templates=["In an M/G/1 queue the arrival rate is 0.5 per minute, the mean service time is 1 minute and its standard deviation is 1 minute. Find Lq."],
     examples=[{"inputs": {"lam": 0.5, "ES": 1, "sd": 1}, "outputs": {"Lq": 0.5, "L": 1.0, "W": 2.0}},
               {"inputs": {"lam": 0.5, "ES": 1, "sd": 0}, "outputs": {"Lq": 0.25, "Wq": 0.5}}],
     questions=[{"q": "In an M/G/1 queue the arrival rate is 0.5 per minute, the mean service time is 1 minute and its standard deviation is 1 minute. Find Lq.",
                 "outputs": {"Lq": 0.5}},
                {"q": "An M/D/1 queue has an arrival rate of 0.5 per minute and a constant service time of 1 minute. Find Wq.", "outputs": {"Wq": 0.5}},
                {"q": "In an M/G/1 queue the arrival rate is 2 per minute, the mean service time is 1 minute and its standard deviation is 1 minute. Find Lq.",
                 "outcome": "INVALID_INPUT"}])


def mmc_staffing(lam, mu, target, criterion="wait_probability"):
    _finite(lam, mu, target)
    if lam <= 0 or mu <= 0 or target <= 0:
        raise ValueError("rates and the target must be > 0")
    if criterion == "wait_probability" and target >= 1:
        raise ValueError("a target probability of waiting must be below 1")
    a = lam / mu
    if a > 400:
        raise ValueError("offered load too large (at most 400)")
    c = int(math.floor(a)) + 1
    while True:
        r = mmc(lam, mu, c)
        val = r["P_wait"] if criterion == "wait_probability" else r["Wq"]
        if val <= target:
            return {"servers": float(c), "P_wait": r["P_wait"], "Wq": r["Wq"], "utilisation": r["rho"]}
        c += 1


def _verify_staffing(i, o):
    c = o["servers"]
    if c != int(c):
        return 1.0
    c = int(c)
    try:
        r = mmc(i["lam"], i["mu"], c)
    except ValueError:
        return 1.0
    crit = i.get("criterion", "wait_probability")
    val = r["P_wait"] if crit == "wait_probability" else r["Wq"]
    worst = max(_rel(o["P_wait"], r["P_wait"]), max(0.0, val - i["target"]) / i["target"])
    if c - 1 > i["lam"] / i["mu"]:
        r2 = mmc(i["lam"], i["mu"], c - 1)
        v2 = r2["P_wait"] if crit == "wait_probability" else r2["Wq"]
        worst = max(worst, 0.0 if v2 > i["target"] else 1.0)
    return worst


def _ref_staffing(lam, mu, target, criterion="wait_probability"):
    """INDEPENDENT reference: Erlang C from the Poisson distribution (scipy.stats), B = pmf(c)/cdf(c)."""
    from scipy import stats
    a = lam / mu
    c = int(math.floor(a)) + 1
    while True:
        B = float(stats.poisson.pmf(c, a) / stats.poisson.cdf(c, a))
        rho = a / c
        C = B / (1 - rho * (1 - B))
        val = C if criterion == "wait_probability" else C / (c * mu - lam)
        if val <= target:
            return {"servers": float(c), "P_wait": C}
        c += 1


def _parse_staffing(q):
    ql = q.lower()
    if not re.search(r"\bhow many (?:servers|tellers|agents|clerks|operators|staff)\b|\bminimum number of (?:servers|tellers|agents|clerks|operators)\b", ql):
        return None
    out = _rates(ql)
    v = _num_after(ql, r"probability (?:of |that a customer (?:has to )?)?wait(?:ing|s)?(?: is)?(?: at most| below| under| no more than| <=?)?", pct=True)
    if v is not None:
        out.update(target=v, criterion="wait_probability")
    else:
        w = _num_after(ql, r"(?:mean|average) wait(?:ing time)?(?: in the queue)?(?: is)?(?: at most| below| under| no more than| <=?)?")
        if w is not None:
            out.update(target=w, criterion="mean_wait")
    return out


_add("mmc_staffing",
     name="Minimum number of servers for a service target (Erlang C)", topic="queueing theory / staffing",
     purpose="The smallest number of servers c of an M/M/c queue for which the probability of waiting, or the mean "
             "waiting time, meets a target.",
     inputs=[_I("lam", "arrival rate"), _I("mu", "service rate of each server"), _I("target", "target value (a probability, or a mean wait in the time unit of the rates)"),
             _I("criterion", "wait_probability (default) or mean_wait", "choice", required=False, choices=["wait_probability", "mean_wait"])],
     outputs=[_O("servers", "minimum number of servers"), _O("P_wait", "probability of waiting with that many servers"),
              _O("Wq", "mean waiting time with that many servers"), _O("utilisation", "server utilisation")],
     primary="servers", conditions=["Poisson arrivals, exponential service", "target > 0"],
     method="increase c from floor(lambda/mu) + 1 until the Erlang C measure meets the target",
     latex=r"c^* = \min\{c > \lambda/\mu : C(c, \lambda/\mu) \le \alpha\}",
     run=mmc_staffing, verify=_verify_staffing, parse=_parse_staffing, reference=_ref_staffing,
     wrong=lambda lam, mu, target, criterion="wait_probability": (lambda c: {"servers": float(c), "P_wait": mmc(lam, mu, c)["P_wait"], "Wq": 0.0})(int(math.floor(lam / mu)) + 1),
     random_inputs=lambda rng: (lambda crit: {"lam": rng.randint(1, 200), "mu": rng.randint(1, 20), "criterion": crit,
                                              "target": rng.choice([0.01, 0.05, 0.1, 0.2]) if crit == "wait_probability" else rng.choice([0.001, 0.005, 0.01, 0.05])})(
         rng.choice(["wait_probability", "mean_wait"])),
     keywords=["how many servers", "staffing", "erlang c"],
     templates=["Calls arrive at 24 per hour and each agent serves 10 per hour. How many agents are needed so that the probability of waiting is at most 0.2?"],
     examples=[{"inputs": {"lam": 24, "mu": 10, "target": 0.2}, "outputs": {"servers": 5.0}}],
     questions=[{"q": "Calls arrive at 24 per hour and each agent serves 10 per hour. How many agents are needed so that the probability of waiting is at most 0.2?",
                 "outputs": {"servers": 5.0}}])


# =================================================================================================================
# scheduling
# =================================================================================================================
def _jobs(p, name="processing times"):
    if not p or len(p) > 12:
        raise ValueError(f"give between 1 and 12 {name}")
    if any(v < 0 for v in p):
        raise ValueError(f"{name} must be >= 0")


def _completion(seq, p):
    t, C = F(0), {}
    for j in seq:
        t += _fr(p[j])
        C[j] = t
    return C


def _verify_seq(i, o, key_fn, value_fn, value_key, n_key="p"):
    seq = [int(round(v)) - 1 for v in o["sequence"]]
    if sorted(seq) != list(range(len(i[n_key]))):
        return 1.0
    keys = [key_fn(i, j) for j in seq]
    if any(k2 < k1 for k1, k2 in zip(keys, keys[1:])):
        return 1.0
    return _rel(o[value_key], value_fn(i, seq))


def _labelled_jobs(q, extra=None):
    labels = {"p": r"processing times?|durations?|job times?"}
    labels.update(extra or {})
    return _labelled(q, labels)


def _brute_seq(n, value):
    return min(value(seq) for seq in itertools.permutations(range(n)))


def spt(p):
    _jobs(p)
    seq = sorted(range(len(p)), key=lambda j: (p[j], j))
    tot = sum(_completion(seq, p).values())
    return {"sequence": [j + 1 for j in seq], "total_completion_time": float(tot), "mean_flow_time": float(tot / len(p))}


def _ref_spt(p):
    """INDEPENDENT reference: position-based assignment problem (job j in position k contributes p_j (n - k))."""
    import numpy as np
    from scipy.optimize import linear_sum_assignment
    n = len(p)
    cost = np.array([[p[j] * (n - k) for k in range(n)] for j in range(n)], float)
    r, c = linear_sum_assignment(cost)
    return {"mean_flow_time": float(cost[r, c].sum()) / n}


_add("spt_mean_flow_time",
     name="Shortest processing time rule (minimum mean flow time)", topic="scheduling / single machine",
     purpose="The job order that minimises the mean flow (completion) time on one machine, and that minimum.",
     inputs=[_I("p", "processing time of each job (jobs numbered 1, 2, ...)", "vector", min_length=1, max_length=12)],
     outputs=[_O("mean_flow_time", "minimum mean completion time"), _O("total_completion_time", "minimum total completion time"),
              _O("sequence", "optimal job order (shortest first; equal times in job-number order)")],
     primary="mean_flow_time", conditions=["all jobs available at time 0", "processing times >= 0"],
     method="sort by processing time (SPT); optimal by the exchange argument", latex=r"\min \tfrac{1}{n}\sum_j C_j:\; p_{[1]} \le p_{[2]} \le \dots",
     run=spt,
     verify=lambda i, o: _verify_seq(i, o, lambda i, j: i["p"][j], lambda i, seq: float(sum(_completion(seq, i["p"]).values())) / len(i["p"]), "mean_flow_time"),
     parse=lambda q: (_labelled_jobs(q) or None) if re.search(r"\b(spt|shortest processing time|mean flow time|average flow time|mean completion time|average completion time)\b", q.lower())
     and not re.search(r"\b(due dates?|weights?|machine 2|two machines|machines)\b", q.lower()) else None,
     reference=_ref_spt,
     wrong=lambda p: {"sequence": list(range(1, len(p) + 1)), "mean_flow_time": float(sum(_completion(range(len(p)), p).values())) / len(p)},
     random_inputs=lambda rng: {"p": [rng.randint(1, 30) for _ in range(rng.randint(3, 10))]},
     keywords=["shortest processing time", "spt", "mean flow time"],
     templates=["Jobs with processing times [4, 2, 6, 3] are done on one machine. Find the sequence that minimizes the mean flow time."],
     examples=[{"inputs": {"p": "4, 2, 6, 3"}, "outputs": {"sequence": [2, 4, 1, 3], "total_completion_time": 31.0, "mean_flow_time": 7.75}}],
     questions=[{"q": "Jobs with processing times [4, 2, 6, 3] are done on one machine. Find the sequence that minimizes the mean flow time.",
                 "outputs": {"sequence": [2, 4, 1, 3], "mean_flow_time": 7.75}}])


def wspt(p, w):
    _jobs(p)
    if len(w) != len(p) or any(v <= 0 for v in w):
        raise ValueError("give one positive weight per job")
    seq = sorted(range(len(p)), key=lambda j: (_fr(p[j]) / _fr(w[j]), j))
    C = _completion(seq, p)
    return {"sequence": [j + 1 for j in seq], "weighted_completion_time": float(sum(_fr(w[j]) * C[j] for j in seq))}


_add("wspt_weighted_completion",
     name="Weighted shortest processing time (Smith's rule)", topic="scheduling / single machine",
     purpose="The job order that minimises the total weighted completion time on one machine, and that minimum.",
     inputs=[_I("p", "processing time of each job", "vector", min_length=1, max_length=12), _I("w", "weight (importance) of each job", "vector", min_length=1, max_length=12)],
     outputs=[_O("weighted_completion_time", "minimum total weighted completion time"), _O("sequence", "optimal order (ratio p/w ascending; ties in job-number order)")],
     primary="weighted_completion_time", conditions=["weights > 0", "all jobs available at time 0"],
     method="sort by p_j / w_j (Smith's ratio rule)", latex=r"\min \sum_j w_j C_j:\; \frac{p_{[1]}}{w_{[1]}} \le \frac{p_{[2]}}{w_{[2]}} \le \dots",
     run=wspt,
     verify=lambda i, o: _verify_seq(i, o, lambda i, j: _fr(i["p"][j]) / _fr(i["w"][j]),
                                     lambda i, seq: float(sum(_fr(i["w"][j]) * c for j, c in _completion(seq, i["p"]).items())), "weighted_completion_time"),
     parse=lambda q: (_labelled_jobs(q, {"w": r"weights?|priorities|importance"}) or None)
     if re.search(r"\b(weighted (?:completion|flow) time|wspt|smith'?s rule)\b", q.lower()) else None,
     reference=lambda p, w: {"weighted_completion_time": float(_brute_seq(len(p), lambda s: sum(w[j] * c for j, c in zip(s, itertools.accumulate(p[k] for k in s)))))},
     wrong=lambda p, w: (lambda seq: {"sequence": [j + 1 for j in seq], "weighted_completion_time": float(sum(_fr(w[j]) * c for j, c in _completion(seq, p).items()))})(
         sorted(range(len(p)), key=lambda j: (p[j], j))),
     random_inputs=lambda rng: (lambda n: {"p": [rng.randint(1, 20) for _ in range(n)], "w": [rng.randint(1, 10) for _ in range(n)]})(rng.randint(3, 7)),
     keywords=["weighted completion time", "smith's rule"],
     templates=["Jobs have processing times [3, 1, 4] and weights [1, 2, 3]. Find the sequence minimizing the total weighted completion time."],
     examples=[{"inputs": {"p": [3, 1, 4], "w": [1, 2, 3]}, "outputs": {"sequence": [2, 3, 1], "weighted_completion_time": 25.0}}],
     questions=[{"q": "Jobs have processing times [3, 1, 4] and weights [1, 2, 3]. Find the sequence minimizing the total weighted completion time.",
                 "outputs": {"weighted_completion_time": 25.0}}])


def edd(p, d):
    _jobs(p)
    if len(d) != len(p):
        raise ValueError("give one due date per job")
    seq = sorted(range(len(p)), key=lambda j: (d[j], j))
    C = _completion(seq, p)
    late = [C[j] - _fr(d[j]) for j in seq]
    Lmax = max(late)
    return {"sequence": [j + 1 for j in seq], "max_lateness": float(Lmax), "max_tardiness": float(max(Lmax, 0)),
            "tardy_jobs_in_sequence": float(sum(1 for v in late if v > 0))}


_add("edd_max_lateness",
     name="Earliest due date rule (minimum maximum lateness)", topic="scheduling / single machine",
     purpose="The job order that minimises the maximum lateness (and so the maximum tardiness) on one machine.",
     inputs=[_I("p", "processing time of each job", "vector", min_length=1, max_length=12), _I("d", "due date of each job", "vector", min_length=1, max_length=12)],
     outputs=[_O("max_lateness", "minimum possible maximum lateness"), _O("max_tardiness", "minimum possible maximum tardiness"),
              _O("sequence", "optimal order (earliest due date first; ties in job-number order)"), _O("tardy_jobs_in_sequence", "number of late jobs in this sequence (not minimised)")],
     primary="max_lateness", conditions=["all jobs available at time 0"],
     method="sort by due date (Jackson's rule)", latex=r"\min L_{\max} = \min \max_j (C_j - d_j):\; d_{[1]} \le d_{[2]} \le \dots",
     run=edd,
     verify=lambda i, o: _verify_seq(i, o, lambda i, j: i["d"][j],
                                     lambda i, seq: float(max(c - _fr(i["d"][j]) for j, c in _completion(seq, i["p"]).items())), "max_lateness"),
     parse=lambda q: (_labelled_jobs(q, {"d": r"due dates?|deadlines?"}) or None)
     if re.search(r"\b(edd|earliest due date|maximum lateness|max(?:imum)? tardiness)\b", q.lower()) else None,
     reference=lambda p, d: {"max_lateness": float(_brute_seq(len(p), lambda s: max(c - d[j] for j, c in zip(s, itertools.accumulate(p[k] for k in s)))))},
     wrong=lambda p, d: (lambda seq: {"sequence": [j + 1 for j in seq], "max_lateness": float(max(c - _fr(d[j]) for j, c in _completion(seq, p).items()))})(
         sorted(range(len(p)), key=lambda j: (p[j], j))),
     random_inputs=lambda rng: (lambda n: {"p": [rng.randint(1, 15) for _ in range(n)], "d": [rng.randint(1, 60) for _ in range(n)]})(rng.randint(3, 7)),
     keywords=["earliest due date", "maximum lateness"],
     templates=["Jobs have processing times [2, 4, 1, 3] and due dates [5, 6, 3, 9]. Find the sequence that minimizes the maximum lateness."],
     examples=[{"inputs": {"p": [2, 4, 1, 3], "d": [5, 6, 3, 9]}, "outputs": {"sequence": [3, 1, 2, 4], "max_lateness": 1.0}}],
     questions=[{"q": "Jobs have processing times [2, 4, 1, 3] and due dates [5, 6, 3, 9]. Find the sequence that minimizes the maximum lateness.",
                 "outputs": {"max_lateness": 1.0}}])


def moore_hodgson(p, d):
    _jobs(p)
    if len(d) != len(p):
        raise ValueError("give one due date per job")
    order = sorted(range(len(p)), key=lambda j: (d[j], j))
    sched, t, tardy = [], F(0), []
    for j in order:
        sched.append(j)
        t += _fr(p[j])
        if t > _fr(d[j]):
            k = max(sched, key=lambda x: (p[x], x))
            sched.remove(k)
            tardy.append(k)
            t -= _fr(p[k])
    return {"min_tardy_jobs": float(len(tardy)), "on_time_sequence": [j + 1 for j in sched], "tardy_jobs": sorted(j + 1 for j in tardy)}


def _verify_moore(i, o):
    on = [int(round(v)) - 1 for v in o["on_time_sequence"]]
    tardy = [int(round(v)) - 1 for v in o["tardy_jobs"]]
    if sorted(on + tardy) != list(range(len(i["p"]))) or o["min_tardy_jobs"] != len(tardy):
        return 1.0
    for j, c in _completion(on, i["p"]).items():
        if c > _fr(i["d"][j]):
            return 1.0
    return 0.0 if moore_hodgson(i["p"], i["d"])["min_tardy_jobs"] == o["min_tardy_jobs"] else 1.0


def _ref_moore(p, d):
    """INDEPENDENT reference: the largest subset of jobs that all meet their due dates (checked in due-date order),
    by enumeration of every subset."""
    n, best = len(p), 0
    for mask in range(1 << n):
        js = sorted((j for j in range(n) if mask >> j & 1), key=lambda j: d[j])
        t, ok = 0, True
        for j in js:
            t += p[j]
            if t > d[j] + 1e-12:
                ok = False
                break
        if ok:
            best = max(best, len(js))
    return {"min_tardy_jobs": float(n - best)}


_add("moore_hodgson_tardy_jobs",
     name="Moore-Hodgson algorithm (minimum number of tardy jobs)", topic="scheduling / single machine",
     purpose="The smallest possible number of late jobs on one machine, with an order of the on-time jobs and the set of late jobs.",
     inputs=[_I("p", "processing time of each job", "vector", min_length=1, max_length=12), _I("d", "due date of each job", "vector", min_length=1, max_length=12)],
     outputs=[_O("min_tardy_jobs", "minimum number of tardy jobs"), _O("on_time_sequence", "on-time jobs in due-date order"),
              _O("tardy_jobs", "jobs scheduled late (one optimal choice: the longest job removed, ties the higher job number)")],
     primary="min_tardy_jobs", conditions=["all jobs available at time 0"],
     method="EDD order; whenever a job is late remove the longest job scheduled so far", latex=r"\min \sum_j U_j",
     run=moore_hodgson, verify=_verify_moore,
     parse=lambda q: (_labelled_jobs(q, {"d": r"due dates?|deadlines?"}) or None)
     if re.search(r"\b(moore|hodgson|number of (?:tardy|late) jobs|tardy jobs|late jobs)\b", q.lower()) and not re.search(r"\bmaximum lateness\b", q.lower()) else None,
     reference=_ref_moore,
     wrong=lambda p, d: {"min_tardy_jobs": edd(p, d)["tardy_jobs_in_sequence"], "on_time_sequence": [], "tardy_jobs": []},
     random_inputs=lambda rng: (lambda n: {"p": [rng.randint(1, 12) for _ in range(n)], "d": [rng.randint(3, 40) for _ in range(n)]})(rng.randint(3, 10)),
     keywords=["moore-hodgson", "number of tardy jobs"],
     templates=["Jobs have processing times [4, 3, 2, 5, 6] and due dates [5, 6, 8, 12, 15]. Find the minimum number of tardy jobs."],
     examples=[{"inputs": {"p": [4, 3, 2, 5, 6], "d": [5, 6, 8, 12, 15]}, "outputs": {"min_tardy_jobs": 2.0}}],
     questions=[{"q": "Jobs have processing times [4, 3, 2, 5, 6] and due dates [5, 6, 8, 12, 15]. Find the minimum number of tardy jobs.",
                 "outputs": {"min_tardy_jobs": 2.0}}])


def _flow2(seq, a, b):
    t1 = t2 = F(0)
    for j in seq:
        t1 += _fr(a[j])
        t2 = max(t2, t1) + _fr(b[j])
    return t2


def johnson(a, b):
    _jobs(a, "machine-1 times")
    if len(b) != len(a) or any(v < 0 for v in b):
        raise ValueError("give one machine-2 time >= 0 per job")
    first = sorted((j for j in range(len(a)) if a[j] < b[j]), key=lambda j: (a[j], j))
    last = sorted((j for j in range(len(a)) if a[j] >= b[j]), key=lambda j: (-b[j], j))
    seq = first + last
    return {"makespan": float(_flow2(seq, a, b)), "sequence": [j + 1 for j in seq]}


def _verify_johnson(i, o):
    seq = [int(round(v)) - 1 for v in o["sequence"]]
    if sorted(seq) != list(range(len(i["a"]))):
        return 1.0
    return max(_rel(o["makespan"], float(_flow2(seq, i["a"], i["b"]))), _rel(o["makespan"], johnson(i["a"], i["b"])["makespan"]))


def _ref_johnson(a, b):
    """INDEPENDENT reference: every job order enumerated, the two-machine makespan of each computed."""
    def mk(s):
        t1 = t2 = 0
        for j in s:
            t1 += a[j]
            t2 = max(t2, t1) + b[j]
        return t2
    return {"makespan": float(_brute_seq(len(a), mk))}


_add("johnson_two_machine",
     name="Johnson's rule (two-machine flow shop)", topic="scheduling / flow shop",
     purpose="The job order that minimises the makespan when every job is processed first on machine 1 and then on machine 2.",
     inputs=[_I("a", "processing time of each job on machine 1", "vector", min_length=1, max_length=12),
             _I("b", "processing time of each job on machine 2", "vector", min_length=1, max_length=12)],
     outputs=[_O("makespan", "minimum makespan"), _O("sequence", "Johnson order (a < b by increasing a, then the rest by decreasing b; ties by job number)")],
     primary="makespan", conditions=["same job order on both machines", "all jobs available at time 0"],
     method="Johnson's rule", latex=r"\min C_{\max} \text{ for } F2 \| C_{\max}",
     run=johnson, verify=_verify_johnson,
     parse=lambda q: (_labelled(q, {"a": r"machine (?:1|one|a)(?: times?)?|first machine(?: times?)?", "b": r"machine (?:2|two|b)(?: times?)?|second machine(?: times?)?"}) or None)
     if re.search(r"\b(johnson|two[- ]machine|2[- ]machine|flow ?shop)\b", q.lower()) else None,
     reference=_ref_johnson,
     wrong=lambda a, b: {"makespan": float(_flow2(range(len(a)), a, b)), "sequence": list(range(1, len(a) + 1))},
     random_inputs=lambda rng: (lambda n: {"a": [rng.randint(1, 20) for _ in range(n)], "b": [rng.randint(1, 20) for _ in range(n)]})(rng.randint(3, 7)),
     keywords=["johnson's rule", "two-machine flow shop", "makespan"],
     templates=["Use Johnson's rule: machine 1 times [5, 2, 6, 7] and machine 2 times [3, 4, 8, 2]. Find the minimum makespan."],
     examples=[{"inputs": {"a": [5, 2, 6, 7], "b": [3, 4, 8, 2]}, "outputs": {"sequence": [2, 3, 1, 4], "makespan": 22.0}}],
     questions=[{"q": "Use Johnson's rule: machine 1 times [5, 2, 6, 7] and machine 2 times [3, 4, 8, 2]. Find the minimum makespan.",
                 "outputs": {"makespan": 22.0}}])


def parallel_makespan(p, m):
    _jobs(p)
    if not 1 <= m <= 6:
        raise ValueError("the number of machines must be between 1 and 6")
    P = [_fr(v) for v in p]
    order = sorted(range(len(P)), key=lambda j: (-P[j], j))
    loads = [F(0)] * m                                # LPT start as the incumbent
    assign = [0] * len(P)
    for j in order:
        k = min(range(m), key=lambda i: (loads[i], i))
        loads[k] += P[j]
        assign[j] = k
    best = [max(loads), list(assign)]
    lb = max(max(P), sum(P) / m)
    cur = [F(0)] * m
    a = [0] * len(P)

    def dfs(t):
        if best[0] == lb:
            return
        if t == len(order):
            mk = max(cur)
            if mk < best[0]:
                best[0], best[1] = mk, list(a)
            return
        j = order[t]
        seen = set()
        for k in range(m):
            if cur[k] in seen or cur[k] + P[j] >= best[0]:
                continue
            seen.add(cur[k])
            cur[k] += P[j]
            a[j] = k
            dfs(t + 1)
            cur[k] -= P[j]
    dfs(0)
    return {"makespan": float(best[0]), "machine_of_job": [k + 1 for k in best[1]], "lower_bound": float(lb)}


def _verify_parallel(i, o):
    m = int(i["m"])
    ks = [int(round(v)) - 1 for v in o["machine_of_job"]]
    if len(ks) != len(i["p"]) or any(not 0 <= k < m for k in ks):
        return 1.0
    loads = [0.0] * m
    for j, k in enumerate(ks):
        loads[k] += i["p"][j]
    return max(_rel(max(loads), o["makespan"]), _rel(parallel_makespan(i["p"], m)["makespan"], o["makespan"]))


def _ref_parallel(p, m):
    """INDEPENDENT reference: mixed-integer program (scipy.optimize.milp, HiGHS, zero gap)."""
    import numpy as np
    from scipy.optimize import milp, LinearConstraint, Bounds
    n = len(p)
    nv = n * m + 1
    c = np.zeros(nv)
    c[-1] = 1
    rows, lo, hi = [], [], []
    for j in range(n):
        r = np.zeros(nv)
        r[j * m:(j + 1) * m] = 1
        rows.append(r); lo.append(1); hi.append(1)
    for k in range(m):
        r = np.zeros(nv)
        for j in range(n):
            r[j * m + k] = p[j]
        r[-1] = -1
        rows.append(r); lo.append(-np.inf); hi.append(0)
    integ = np.ones(nv)
    integ[-1] = 0
    res = milp(c, constraints=LinearConstraint(np.array(rows), lo, hi), integrality=integ,
               bounds=Bounds(np.zeros(nv), np.r_[np.ones(n * m), np.inf]), options={"mip_rel_gap": 0})
    x = np.round(res.x[:-1]).reshape(n, m)
    return {"makespan": float(max((x[:, k] * np.array(p, float)).sum() for k in range(m)))}


def _parse_parallel(q):
    ql = q.lower()
    if not re.search(r"\b(identical|parallel)\s+machines\b", ql) or not re.search(r"\b(makespan|finish|complete)", ql):
        return None
    out = _labelled_jobs(q) or {}
    mm = re.search(r"\b(\d+)\s+(?:identical|parallel)\s+machines\b", ql)
    if mm:
        out["m"] = int(mm.group(1))
    return out


_add("parallel_machine_makespan",
     name="Minimum makespan on identical parallel machines", topic="scheduling / parallel machines",
     purpose="Assign jobs to m identical machines so that the last machine finishes as early as possible (P||Cmax), exactly.",
     inputs=[_I("p", "processing time of each job", "vector", min_length=1, max_length=12), _I("m", "number of identical machines", "integer", min=1, max=6)],
     outputs=[_O("makespan", "minimum makespan"), _O("machine_of_job", "one optimal assignment: machine number of each job"),
              _O("lower_bound", "the bound max(longest job, total / m)")],
     primary="makespan", conditions=["at most 12 jobs and 6 machines", "jobs are not split"],
     method="exact depth-first branch and bound (longest job first, symmetric machines skipped), started from the LPT schedule",
     latex=r"\min_{\text{assignment}} \max_k \sum_{j \to k} p_j",
     run=parallel_makespan, verify=_verify_parallel, parse=_parse_parallel, reference=_ref_parallel,
     wrong=lambda p, m: (lambda loads: {"makespan": float(max(loads)), "machine_of_job": [1] * len(p)})(
         [sum(p[j] for j in range(len(p)) if j % m == k) for k in range(m)]),
     random_inputs=lambda rng: {"p": [rng.randint(1, 30) for _ in range(rng.randint(3, 10))], "m": rng.randint(2, 4)},
     keywords=["parallel machines", "makespan"],
     templates=["Jobs with processing times [7, 5, 4, 3, 3, 2] are scheduled on 2 identical machines. Find the minimum makespan."],
     examples=[{"inputs": {"p": [7, 5, 4, 3, 3, 2], "m": 2}, "outputs": {"makespan": 12.0}},
               {"inputs": {"p": [3, 3, 2, 2, 2], "m": 2}, "outputs": {"makespan": 6.0}}],
     questions=[{"q": "Jobs with processing times [7, 5, 4, 3, 3, 2] are scheduled on 2 identical machines. Find the minimum makespan.",
                 "outputs": {"makespan": 12.0}}])


# --- project scheduling: critical path method and PERT -------------------------------------------------------------
def _activity_index(n):
    """Activities are named A, B, C, ... (or 1, 2, 3, ...) in the order of the durations."""
    idx = {}
    for k in range(n):
        idx[chr(ord("A") + k)] = k
        idx[str(k + 1)] = k
    return idx


def _project(dur, precedences):
    n = len(dur)
    if not 1 <= n <= 26:
        raise ValueError("give between 1 and 26 activities")
    if any(v < 0 for v in dur):
        raise ValueError("durations must be >= 0")
    idx = _activity_index(n)
    preds = [set() for _ in range(n)]
    for e in precedences or []:
        if len(e) == 3:
            raise ValueError("a precedence is a pair 'A-B' (A before B) without a number")
        u, v = str(e[0]).strip().upper(), str(e[1]).strip().upper()
        if u not in idx or v not in idx:
            raise ValueError(f"precedence {u}-{v} names an activity that does not exist (activities are A..{chr(ord('A') + n - 1)} or 1..{n})")
        preds[idx[v]].add(idx[u])
    order, done = [], set()
    while len(order) < n:                               # topological order, smallest index first
        ready = [k for k in range(n) if k not in done and preds[k] <= done]
        if not ready:
            raise ValueError("the precedences contain a cycle")
        order.append(ready[0])
        done.add(ready[0])
    return preds, order


def cpm(durations, precedences=None):
    D = [_fr(v) for v in durations]
    preds, order = _project(durations, precedences)
    n = len(D)
    ES = [F(0)] * n
    for k in order:
        ES[k] = max((ES[j] + D[j] for j in preds[k]), default=F(0))
    T = max(ES[k] + D[k] for k in range(n))
    succ = [[k for k in range(n) if j in preds[k]] for j in range(n)]
    LF = [T] * n
    for k in reversed(order):
        LF[k] = min((LF[s] - D[s] for s in succ[k]), default=T)
    LS = [LF[k] - D[k] for k in range(n)]
    slack = [LS[k] - ES[k] for k in range(n)]
    return {"project_duration": float(T), "critical_activities": [chr(65 + k) for k in range(n) if slack[k] == 0],
            "slack": _fl(slack), "earliest_start": _fl(ES), "latest_start": _fl(LS)}


def _verify_cpm(i, o):
    try:
        r = cpm(i["durations"], i.get("precedences"))
    except ValueError:
        return 1.0
    worst = _rel(o["project_duration"], r["project_duration"])
    for k in ("slack", "earliest_start", "latest_start"):
        worst = max([worst] + [_rel(a, b) for a, b in zip(o[k], r[k])])
    return worst if o["critical_activities"] == r["critical_activities"] else 1.0


def _ref_cpm(durations, precedences=None):
    """INDEPENDENT reference: networkx longest paths in the activity-on-node graph (source S, sink T)."""
    import networkx as nx
    n = len(durations)
    idx = _activity_index(n)
    G = nx.DiGraph()
    for k in range(n):
        G.add_edge("S", k, weight=0.0)
        G.add_edge(k, "T", weight=float(durations[k]))
    for e in precedences or []:
        u = idx[str(e[0]).upper()]
        G.add_edge(u, idx[str(e[1]).upper()], weight=float(durations[u]))
    T = nx.dag_longest_path_length(G)
    slack = []
    for k in range(n):
        es = nx.dag_longest_path_length(G.subgraph(nx.ancestors(G, k) | {k}))
        tail = nx.dag_longest_path_length(G.subgraph(nx.descendants(G, k) | {k}))
        slack.append(float(T - es - tail))
    return {"project_duration": float(T), "slack": slack}


def _precedence_text(q):
    m = re.search(r"\bprecedences?(?: relations?)?(?: are)?\s*:?\s*((?:[A-Za-z0-9]+\s*(?:-|->|→)\s*[A-Za-z0-9]+\s*[,;]?\s*(?:and\s+)?)+)", q)
    if not m:
        return None
    return re.sub(r"\s*(?:->|→)\s*", "-", m.group(1)).replace(" and ", " ").strip(" ,;")


def _parse_cpm(q):
    ql = q.lower()
    if not re.search(r"\b(critical path|cpm|project duration|slack|total float)\b", ql) or re.search(r"\b(pert|optimistic|pessimistic|most likely)\b", ql):
        return None
    out = _labelled(q, {"durations": r"durations?|times?"})
    pt = _precedence_text(q)
    if pt is not None:
        out["precedences"] = pt
    return out or None


def _random_project(rng):
    n = rng.randint(3, 10)
    edges = []
    for v in range(1, n):
        for u in rng.sample(range(v), min(v, rng.randint(0, 2))):
            edges.append((chr(65 + u), chr(65 + v)))
    return n, edges


_add("critical_path_method",
     name="Critical path method (project duration and slack)", topic="project scheduling",
     purpose="For activities with durations and precedences, the minimum project duration, the critical activities and "
             "each activity's earliest start, latest start and slack (total float).",
     inputs=[_I("durations", "duration of each activity; activities are named A, B, C, ... (or 1, 2, 3, ...) in this order", "vector", min_length=1, max_length=26),
             _I("precedences", "precedence pairs 'A-B' meaning A must finish before B starts (none if omitted)", "edges", required=False)],
     outputs=[_O("project_duration", "minimum project duration"), _O("critical_activities", "activities with zero slack"),
              _O("slack", "total slack of each activity"), _O("earliest_start", "earliest start of each activity"), _O("latest_start", "latest start of each activity")],
     primary="project_duration", conditions=["finish-to-start precedences without cycles", "deterministic durations"],
     method="forward pass (earliest starts) and backward pass (latest finishes) on the activity-on-node network, exact fractions",
     latex=r"ES_k = \max_{j \to k}(ES_j + d_j),\quad LS_k = \min_{k \to s} LS_s - d_k,\quad \text{slack}_k = LS_k - ES_k",
     run=cpm, verify=_verify_cpm, parse=_parse_cpm, reference=_ref_cpm,
     wrong=lambda durations, precedences=None: cpm(durations, precedences) | {"project_duration": float(sum(_fr(v) for v in durations))},
     random_inputs=lambda rng: (lambda ne: {"durations": [rng.randint(1, 15) for _ in range(ne[0])], "precedences": ne[1]})(_random_project(rng)),
     keywords=["critical path", "project duration", "slack"],
     templates=["Activities A, B, C, D have durations [3, 4, 2, 5]; precedences: A-B, A-C, B-D, C-D. Find the critical path and the project duration."],
     examples=[{"inputs": {"durations": "3, 4, 2, 5", "precedences": "A-B, A-C, B-D, C-D"},
                "outputs": {"project_duration": 12.0, "critical_activities": ["A", "B", "D"], "slack": [0.0, 0.0, 2.0, 0.0]}}],
     questions=[{"q": "Activities A, B, C, D have durations [3, 4, 2, 5]; precedences: A-B, A-C, B-D, C-D. Find the critical path and the project duration.",
                 "outputs": {"project_duration": 12.0, "critical_activities": ["A", "B", "D"]}},
                {"q": "Find the critical path: activities have durations [3, 4, 2]; precedences: A-B, B-C, C-A.", "outcome": "INVALID_INPUT"}])


def pert(times, precedences=None, deadline=None):
    if any(len(r) != 3 for r in times):
        raise ValueError("each activity needs three times: optimistic, most likely, pessimistic")
    for a, m, b in times:
        if not 0 <= a <= m <= b:
            raise ValueError("each activity needs 0 <= optimistic <= most likely <= pessimistic")
    te = [(_fr(a) + 4 * _fr(m) + _fr(b)) / 6 for a, m, b in times]
    var = [((_fr(b) - _fr(a)) / 6) ** 2 for a, m, b in times]
    preds, order = _project([float(v) for v in te], precedences)
    n = len(te)
    ES, cnt = [F(0)] * n, [1] * n
    for k in order:
        if preds[k]:
            best = max(ES[j] + te[j] for j in preds[k])
            ES[k], cnt[k] = best, sum(cnt[j] for j in preds[k] if ES[j] + te[j] == best)
    finish = [ES[k] + te[k] for k in range(n)]
    T = max(finish)
    ends = [k for k in range(n) if finish[k] == T]
    if sum(cnt[k] for k in ends) > 1:
        raise ValueError("there is more than one critical path, so the PERT variance of the project is not determined")
    path, k = [], ends[0]
    while True:                                          # walk the unique critical path back
        path.append(k)
        if not preds[k]:
            break
        k = next(j for j in preds[k] if ES[j] + te[j] == ES[k])
    path.reverse()
    V = sum(var[k] for k in path)
    out = {"expected_duration": float(T), "variance": float(V), "std_dev": math.sqrt(V), "critical_path": [chr(65 + k) for k in path],
           "expected_times": _fl(te)}
    if deadline is not None:
        _finite(deadline)
        out["probability"] = (1.0 if deadline >= T else 0.0) if V == 0 else _STD.cdf((deadline - float(T)) / math.sqrt(V))
    return out


def _verify_pert(i, o):
    times = i["times"]
    te = [(a + 4 * m + b) / 6 for a, m, b in times]
    names = [ord(c) - 65 for c in o["critical_path"]]
    worst = _rel(o["expected_duration"], sum(te[k] for k in names))
    worst = max(worst, _rel(o["variance"], sum(((times[k][2] - times[k][0]) / 6) ** 2 for k in names)))
    try:
        r = pert(times, i.get("precedences"), i.get("deadline"))
    except ValueError:
        return 1.0
    worst = max(worst, _rel(o["expected_duration"], r["expected_duration"]))
    if "probability" in r:
        worst = max(worst, abs(o.get("probability", -1) - r["probability"]))
    return worst


def _ref_pert(times, precedences=None, deadline=None):
    """INDEPENDENT reference: networkx longest path on expected times, variance along that path, scipy normal cdf."""
    import networkx as nx
    from scipy import stats
    n = len(times)
    idx = _activity_index(n)
    te = [(a + 4 * m + b) / 6 for a, m, b in times]
    G = nx.DiGraph()
    for k in range(n):
        G.add_edge("S", k, weight=0.0)
        G.add_edge(k, "T", weight=te[k])
    for e in precedences or []:
        u = idx[str(e[0]).upper()]
        G.add_edge(u, idx[str(e[1]).upper()], weight=te[u])
    path = [v for v in nx.dag_longest_path(G) if v not in ("S", "T")]
    T = nx.dag_longest_path_length(G)
    V = sum(((times[k][2] - times[k][0]) / 6) ** 2 for k in path)
    out = {"expected_duration": float(T), "variance": float(V)}
    if deadline is not None and V > 0:
        out["probability"] = float(stats.norm.cdf(deadline, T, math.sqrt(V)))
    return out


def _parse_pert(q):
    ql = q.lower()
    if not re.search(r"\b(pert|three[- ]time estimates?|optimistic)\b", ql):
        return None
    out = _labelled(q, {"times": r"times?|estimates?"})
    pt = _precedence_text(q)
    if pt is not None:
        out["precedences"] = pt
    m = re.search(r"\bprobability\b[^.?]*?\b(?:within|by|in)\s+(" + _N + r")", ql)
    if m:
        out["deadline"] = float(m.group(1))
    return out or None


_add("pert_project",
     name="PERT project duration and completion probability", topic="project scheduling",
     purpose="With optimistic, most likely and pessimistic times per activity, the expected project duration, its "
             "variance along the critical path and the probability of finishing by a deadline (normal approximation).",
     inputs=[_I("times", "rows (optimistic, most likely, pessimistic), one per activity A, B, C, ...", "matrix"),
             _I("precedences", "precedence pairs 'A-B' (A before B)", "edges", required=False),
             _I("deadline", "deadline for the completion probability (optional)", required=False)],
     outputs=[_O("expected_duration", "expected project duration"), _O("variance", "variance of the critical path"), _O("std_dev", "standard deviation"),
              _O("critical_path", "the critical path (must be unique)"), _O("expected_times", "expected time of each activity (a + 4m + b)/6"),
              _O("probability", "probability of completion by the deadline")],
     primary="expected_duration", conditions=["a unique critical path (otherwise INVALID_INPUT)", "independent activity times; normal approximation"],
     method="beta-PERT expected times, longest path in exact fractions with a count of longest paths, variance summed along it",
     latex=r"t_e = \frac{a + 4m + b}{6},\; \sigma^2 = \left(\frac{b - a}{6}\right)^2,\; P(T \le d) = \Phi\!\left(\frac{d - \mu}{\sigma}\right)",
     run=pert, verify=_verify_pert, parse=_parse_pert, reference=_ref_pert,
     wrong=lambda times, precedences=None, deadline=None: (lambda r: r | {"variance": math.sqrt(r["variance"]) * 6 + 1, "probability": 0.5 if r.get("probability") != 0.5 else 0.6})(pert(times, precedences, deadline)),
     random_inputs=lambda rng: (lambda ne: {"times": [(lambda a: (lambda m: [a, m, m + rng.randint(1, 10)])(a + rng.randint(0, 6)))(rng.randint(1, 8)) for _ in range(ne[0])],
                                            "precedences": ne[1], "deadline": rng.randint(10, 60)})(_random_project(rng)),
     keywords=["pert", "optimistic", "pessimistic", "probability of completion"],
     templates=["PERT: activities A, B, C have times [[2, 4, 6], [3, 5, 13], [1, 2, 3]]; precedences: A-B, A-C. Find the expected project duration."],
     examples=[{"inputs": {"times": "2 4 6; 3 5 13; 1 2 3", "precedences": "A-B, A-C", "deadline": 10},
                "outputs": {"expected_duration": 10.0, "variance": 4 / 9 + 25 / 9, "critical_path": ["A", "B"], "probability": 0.5}}],
     questions=[{"q": "PERT: activities A, B, C have times [[2, 4, 6], [3, 5, 13], [1, 2, 3]]; precedences: A-B, A-C. Find the expected project duration.",
                 "outputs": {"expected_duration": 10.0, "critical_path": ["A", "B"]}},
                {"q": "PERT: activities A, B have times [[2, 4, 6], [2, 4, 6]]. Find the expected project duration.", "outcome": "INVALID_INPUT"}])


# =================================================================================================================
# networks
# =================================================================================================================
def _nodes(edges):
    out = []
    for e in edges:
        for v in e[:2]:
            if v not in out:
                out.append(v)
    return out


def _wedges(edges, need_weight=True):
    if not edges:
        raise ValueError("give at least one edge")
    if need_weight and any(len(e) != 3 for e in edges):
        raise ValueError("every edge needs a weight: 'A-B 4'")
    return [(str(e[0]), str(e[1]), _fr(e[2])) for e in edges] if need_weight else [(str(e[0]), str(e[1])) for e in edges]


def _adj(E, directed):
    adj = {}
    for u, v, w in E:
        adj.setdefault(u, []).append((v, w))
        adj.setdefault(v, [])
        if not directed:
            adj[v].append((u, w))
    return adj


def _dijkstra(E, s, directed):
    adj = _adj(E, directed)
    dist, done, h, cnt = {s: F(0)}, set(), [(F(0), 0, s)], 1
    while h:
        d, _, u = heapq.heappop(h)
        if u in done:
            continue
        done.add(u)
        for v, w in adj[u]:
            if v not in dist or d + w < dist[v]:
                dist[v] = d + w
                heapq.heappush(h, (d + w, cnt, v))
                cnt += 1
    return dist, adj


def _bellman(E, s, directed):
    nodes = _nodes(E)
    arcs = [(u, v, w) for u, v, w in E] + ([] if directed else [(v, u, w) for u, v, w in E])
    dist = {s: F(0)}
    for _ in range(len(nodes)):
        changed = False
        for u, v, w in arcs:
            if u in dist and (v not in dist or dist[u] + w < dist[v]):
                dist[v] = dist[u] + w
                changed = True
        if not changed:
            return dist
    raise ValueError("the graph has a negative cycle reachable from the source: shortest paths are undefined")


def _sp_result(E, s, t, directed, dist, adj):
    if t not in dist:
        raise ValueError(f"there is no path from {s} to {t}")
    nodes = sorted(dist, key=lambda v: (dist[v], _nodes(E).index(v)))
    tight = {v: [(u, w) for u in nodes for (x, w) in adj.get(u, []) if x == v and dist[u] + w == dist[v]] for v in nodes}
    zero = any(w == 0 for v in nodes for _, w in tight[v])
    count = {s: 1}
    for v in nodes:
        if v != s:
            count[v] = sum(count.get(u, 0) for u, _ in tight[v])
    path, v = [t], t
    while v != s:
        v = min((u for u, _ in tight[v]), key=lambda u: (dist[u], _nodes(E).index(u)))
        path.append(v)
    path.reverse()
    unique = count[t] == 1 and not zero
    return {"distance": float(dist[t]), ("path" if unique else "path_example"): path, "path_unique": "yes" if unique else "no"}


def shortest_path(edges, source, target, directed):
    E = _wedges(edges)
    if any(w < 0 for _, _, w in E):
        raise ValueError("negative weights: use the Bellman-Ford capability")
    s, t = str(source).strip(), str(target).strip()
    if s not in _nodes(E) or t not in _nodes(E):
        raise ValueError("the source and the target must be nodes of the graph")
    dist, adj = _dijkstra(E, s, directed)
    return _sp_result(E, s, t, directed, dist, adj)


def _verify_sp(i, o):
    try:
        E = _wedges(i["edges"])
        dist = _bellman(E, str(i["source"]).strip(), i["directed"])
    except ValueError:
        return 1.0
    t = str(i["target"]).strip()
    path = o.get("path", o.get("path_example"))
    if t not in dist or not path or path[0] != str(i["source"]).strip() or path[-1] != t:
        return 1.0
    adj = _adj(E, i["directed"])
    tot = F(0)
    for u, v in zip(path, path[1:]):
        ws = [w for x, w in adj[u] if x == v]
        if not ws:
            return 1.0
        tot += min(ws)
    return max(_rel(o["distance"], float(dist[t])), _rel(o["distance"], float(tot)))


def _nx_graph(edges, directed):
    import networkx as nx
    G = nx.DiGraph() if directed else nx.Graph()
    for u, v, w in edges:
        u, v = str(u), str(v)
        if G.has_edge(u, v):
            G[u][v]["weight"] = min(G[u][v]["weight"], float(w))
        else:
            G.add_edge(u, v, weight=float(w))
    return G


def _random_graph(rng, n=None, neg=False, directed=None, connected=True):
    n = n or rng.randint(4, 9)
    names = [chr(65 + k) for k in range(n)]
    edges = []
    for k in range(1, n):
        edges.append((names[rng.randrange(k)], names[k], rng.randint(1, 20)))
    for _ in range(rng.randint(0, 2 * n)):
        u, v = rng.sample(names, 2)
        edges.append((u, v, rng.randint(1, 20)))
    rng.shuffle(edges)
    return names, edges


def _graph_q(q):
    ql = q.lower()
    out = {}
    et = _edges_text(q)
    if et:
        out["edges"] = et
    m = re.search(r"\bfrom\s+(?:node\s+|vertex\s+)?([A-Za-z0-9_]+)\s+to\s+(?:node\s+|vertex\s+)?([A-Za-z0-9_]+)", q)
    if m:
        out["source"], out["target"] = m.group(1), m.group(2)
    if re.search(r"\bundirected\b", ql):
        out["directed"] = False
    elif _affirmed(ql, r"\bdirected\b|\bdigraph\b|\barcs\b"):
        out["directed"] = True
    return out


_GRAPH_IN = [_I("edges", "weighted edges 'A-B 4, B-C 1, ...'", "edges", min_length=1, max_length=200),
             _I("source", "start node", "text"), _I("target", "end node", "text"),
             _I("directed", "true for a directed graph (edge u-v goes from u to v), false for undirected", "boolean")]

_add("shortest_path",
     name="Shortest path (Dijkstra's algorithm)", topic="network optimisation / shortest routes",
     purpose="The length of the shortest route between two nodes of a graph with non-negative edge weights, and the "
             "route itself (stated as the route only when it is the only shortest one).",
     inputs=_GRAPH_IN,
     outputs=[_O("distance", "shortest distance"), _O("path", "the shortest path (only when unique; otherwise path_example)"),
              _O("path_unique", "yes when exactly one shortest path exists", "text")],
     primary="distance", conditions=["weights >= 0", "a path from source to target exists", "parallel edges: the lightest counts"],
     method="Dijkstra's algorithm in exact arithmetic; shortest paths counted on the tight-edge subgraph",
     latex=r"d(v) = \min_{(u,v)} \big(d(u) + w_{uv}\big)",
     run=shortest_path, verify=_verify_sp,
     parse=lambda q: (lambda g: g or None)(_graph_q(q)) if re.search(r"\bshortest (?:path|route|distance)\b", q.lower())
     and not re.search(r"\b(bellman|negative|all[- ]pairs|floyd|every pair)\b", q.lower()) else None,
     reference=lambda edges, source, target, directed: {"distance": float(__import__("networkx").dijkstra_path_length(_nx_graph(edges, directed), str(source), str(target)))},
     wrong=lambda edges, source, target, directed: (lambda d: {"distance": float(d[0][str(target)]), "path": [str(source), str(target)]})(   # hops, not weights
         _dijkstra([(str(u), str(v), F(1)) for u, v, *_ in edges], str(source), directed)),
     random_inputs=lambda rng: (lambda ne: {"edges": ne[1], "source": ne[0][0], "target": ne[0][-1], "directed": False})(_random_graph(rng)),
     keywords=["shortest path", "dijkstra", "shortest route"],
     templates=["Find the shortest path from A to D in the undirected graph with edges A-B 4, A-C 2, B-C 1, B-D 5, C-D 8."],
     examples=[{"inputs": {"edges": "A-B 4, A-C 2, B-C 1, B-D 5, C-D 8", "source": "A", "target": "D", "directed": False},
                "outputs": {"distance": 8.0, "path": ["A", "C", "B", "D"]}},
               {"inputs": {"edges": "A-B 1, B-C 1, A-C 2", "source": "A", "target": "C", "directed": True},
                "outputs": {"distance": 2.0, "path_unique": "no"}}],
     questions=[{"q": "Find the shortest path from A to D in the undirected graph with edges A-B 4, A-C 2, B-C 1, B-D 5, C-D 8.",
                 "outputs": {"distance": 8.0, "path": ["A", "C", "B", "D"]}},
                {"q": "Find the shortest path from A to D in the graph with edges A-B 4, A-C 2, B-C 1, B-D 5, C-D 8.", "outcome": "NEEDS_INPUT"},
                {"q": "Find the shortest path from A to E in the directed graph with edges A-B 4, B-C 1.", "outcome": "INVALID_INPUT"}])


def bellman_ford(edges, source, target):
    E = _wedges(edges)
    s, t = str(source).strip(), str(target).strip()
    if s not in _nodes(E) or t not in _nodes(E):
        raise ValueError("the source and the target must be nodes of the graph")
    dist = _bellman(E, s, True)
    if t not in dist:
        raise ValueError(f"there is no path from {s} to {t}")
    return {"distance": float(dist[t]), "distances_from_source": {v: float(d) for v, d in sorted(dist.items(), key=lambda kv: _nodes(E).index(kv[0]))}}


def _verify_bf(i, o):
    E = _wedges(i["edges"])
    d = {k: _fr(v) for k, v in o["distances_from_source"].items()}
    s, t = str(i["source"]).strip(), str(i["target"]).strip()
    if d.get(s) != 0 or t not in d:
        return 1.0
    worst = _rel(o["distance"], float(d[t]))
    for u, v, w in E:                                    # potentials certificate: no edge can improve a label
        if u in d and (v not in d or d[u] + w < d[v]):
            return 1.0
    for v in d:                                          # every label is attained by some edge (or is the source)
        if v != s and not any(u in d and d[u] + w == d[v] for u, x, w in E if x == v):
            return 1.0
    return worst


def _random_neg(rng):
    names, edges = _random_graph(rng)
    phi = {v: rng.randint(-25, 25) for v in names}
    E = [(u, v, w + phi[v] - phi[u]) for u, v, w in edges]   # reduced weights: no negative cycles
    return {"edges": E, "source": names[0], "target": names[-1]}


_add("bellman_ford_shortest_path",
     name="Shortest path with negative weights (Bellman-Ford)", topic="network optimisation / shortest routes",
     purpose="Shortest distance in a directed graph whose edge weights may be negative, with all distances from the source; "
             "a negative cycle reachable from the source is reported instead of a number.",
     inputs=[_I("edges", "directed weighted edges 'A-B 4, B-C -2, ...'", "edges", min_length=1, max_length=200),
             _I("source", "start node", "text"), _I("target", "end node", "text")],
     outputs=[_O("distance", "shortest distance from source to target"), _O("distances_from_source", "shortest distance to every reachable node")],
     primary="distance", conditions=["directed graph", "no negative cycle reachable from the source (otherwise INVALID_INPUT)"],
     method="Bellman-Ford relaxation in exact arithmetic; distances certified as feasible, tight potentials",
     latex=r"d(v) \le d(u) + w_{uv} \ \forall (u,v)",
     run=bellman_ford, verify=_verify_bf,
     parse=lambda q: (lambda g: (g.pop("directed", None), g)[1] or None)(_graph_q(q)) if re.search(r"\b(bellman|negative (?:weights?|costs?|edges?))\b", q.lower())
     and re.search(r"\bshortest\b", q.lower()) else None,
     reference=lambda edges, source, target: {"distance": float(__import__("networkx").bellman_ford_path_length(_nx_graph(edges, True), str(source), str(target)))},
     wrong=lambda edges, source, target: (lambda d: {"distance": float(d.get(str(target), 0)), "distances_from_source": {k: float(v) for k, v in d.items()}})(   # |w|: signs dropped
         _dijkstra([(str(u), str(v), abs(_fr(w))) for u, v, w in edges], str(source), True)[0]),
     random_inputs=_random_neg,
     keywords=["bellman-ford", "negative weights"],
     templates=["With negative weights allowed, find the shortest path from S to T in the directed graph with edges S-A 4, S-B 5, A-T 3, B-A -3, B-T 6."],
     examples=[{"inputs": {"edges": [("S", "A", 4), ("S", "B", 5), ("A", "T", 3), ("B", "A", -3), ("B", "T", 6)], "source": "S", "target": "T"}, "outputs": {"distance": 5.0}}],
     questions=[{"q": "With negative weights allowed, find the shortest path from S to T in the directed graph with edges S-A 4, S-B 5, A-T 3, B-A -3, B-T 6.",
                 "outputs": {"distance": 5.0}},
                {"q": "With negative weights allowed, find the shortest path from S to T in the directed graph with edges S-A 1, A-B -2, B-A 1, A-T 1.",
                 "outcome": "INVALID_INPUT"}])


def all_pairs(edges, directed):
    E = _wedges(edges)
    nodes = _nodes(E)
    if len(nodes) > 40:
        raise ValueError("at most 40 nodes")
    n = len(nodes)
    ix = {v: k for k, v in enumerate(nodes)}
    INF = None
    D = [[F(0) if a == b else INF for b in range(n)] for a in range(n)]
    for u, v, w in E:
        for a, b in ((u, v),) + (() if directed else ((v, u),)):
            if D[ix[a]][ix[b]] is None or w < D[ix[a]][ix[b]]:
                D[ix[a]][ix[b]] = w
    for k in range(n):
        for a in range(n):
            if D[a][k] is None:
                continue
            for b in range(n):
                if D[k][b] is not None and (D[a][b] is None or D[a][k] + D[k][b] < D[a][b]):
                    D[a][b] = D[a][k] + D[k][b]
    if any(D[a][a] < 0 for a in range(n)):
        raise ValueError("the graph has a negative cycle")
    if any(x is None for r in D for x in r):
        raise ValueError("some node cannot reach another (the distance would be infinite); give a connected graph")
    return {"nodes": nodes, "distances": [[float(x) for x in r] for r in D], "diameter": float(max(max(r) for r in D))}


def _verify_apsp(i, o):
    E = _wedges(i["edges"])
    try:
        if sorted(o["nodes"]) != sorted(_nodes(E)):
            return 1.0
        rows = {s: _bellman(E, s, i["directed"]) for s in o["nodes"]}
        if any(rows[s].get(t) is None for s in o["nodes"] for t in o["nodes"]):
            return 1.0
        worst = max(_rel(a, float(rows[s][t])) for s, ra in zip(o["nodes"], o["distances"]) for a, t in zip(ra, o["nodes"]))
        return max(worst, _rel(o["diameter"], max(float(rows[s][t]) for s in o["nodes"] for t in o["nodes"])))
    except ValueError:
        return 1.0


_add("all_pairs_shortest_paths",
     name="All-pairs shortest distances (Floyd-Warshall)", topic="network optimisation / shortest routes",
     purpose="The table of shortest distances between every pair of nodes, and the graph's diameter.",
     inputs=[_I("edges", "weighted edges 'A-B 4, ...'", "edges", min_length=1, max_length=300),
             _I("directed", "true for a directed graph, false for undirected", "boolean")],
     outputs=[_O("distances", "shortest-distance matrix (rows and columns in the order of 'nodes')"), _O("nodes", "node order (first appearance)"),
              _O("diameter", "largest shortest distance")],
     primary="diameter", conditions=["every node reaches every other", "no negative cycle", "at most 40 nodes"],
     method="Floyd-Warshall in exact arithmetic", latex=r"d_{ij} \leftarrow \min(d_{ij},\, d_{ik} + d_{kj})",
     run=all_pairs, verify=_verify_apsp,
     parse=lambda q: (lambda g: {k: v for k, v in g.items() if k in ("edges", "directed")} or None)(_graph_q(q))
     if re.search(r"\b(all[- ]pairs|floyd|every pair|between all pairs|distance matrix|diameter)\b", q.lower()) else None,
     reference=lambda edges, directed: (lambda G, nodes: {"distances": __import__("networkx").floyd_warshall_numpy(G, nodelist=nodes).tolist(),
                                                           "diameter": float(__import__("networkx").floyd_warshall_numpy(G, nodelist=nodes).max())})(
         _nx_graph(edges, directed), _nodes([(str(e[0]), str(e[1])) for e in edges])),
     wrong=lambda edges, directed: (lambda r: r | {"diameter": float(sum(float(e[2]) for e in edges))})(all_pairs(edges, directed)),
     random_inputs=lambda rng: {"edges": _random_graph(rng)[1], "directed": False},
     keywords=["all pairs shortest path", "floyd-warshall", "distance matrix"],
     templates=["Find the all-pairs shortest distances in the undirected graph with edges A-B 3, B-C 4, A-C 9, C-D 2."],
     examples=[{"inputs": {"edges": "A-B 3, B-C 4, A-C 9, C-D 2", "directed": False},
                "outputs": {"distances": [[0.0, 3.0, 7.0, 9.0], [3.0, 0.0, 4.0, 6.0], [7.0, 4.0, 0.0, 2.0], [9.0, 6.0, 2.0, 0.0]], "diameter": 9.0}}],
     questions=[{"q": "Find the all-pairs shortest distances in the undirected graph with edges A-B 3, B-C 4, A-C 9, C-D 2.", "outputs": {"diameter": 9.0}}])


def mst(edges):
    E = _wedges(edges)
    nodes = _nodes(E)
    parent = {v: v for v in nodes}

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x
    tree = []
    for k in sorted(range(len(E)), key=lambda k: (E[k][2], k)):
        u, v, w = E[k]
        ru, rv = find(u), find(v)
        if ru != rv:
            parent[ru] = rv
            tree.append(k)
    if len(tree) != len(nodes) - 1:
        raise ValueError("the graph is not connected: no spanning tree exists")
    tset = set(tree)
    adj = {v: [] for v in nodes}
    for k in tree:
        u, v, w = E[k]
        adj[u].append((v, w))
        adj[v].append((u, w))

    def maxw(a, b):                                      # heaviest edge on the tree path a..b
        st, seen = [(a, F(-10 ** 18))], {a}
        while st:
            x, m = st.pop()
            if x == b:
                return m
            for y, w in adj[x]:
                if y not in seen:
                    seen.add(y)
                    st.append((y, max(m, w)))
    unique = all(maxw(u, v) < w for k, (u, v, w) in enumerate(E) if k not in tset and u != v)
    out = {"total_weight": float(sum(E[k][2] for k in tree)), "tree_unique": "yes" if unique else "no"}
    out["tree_edges" if unique else "tree_edges_example"] = [f"{E[k][0]}-{E[k][1]}" for k in tree]
    return out


def _verify_mst(i, o):
    E = _wedges(i["edges"])
    nodes = _nodes(E)
    names = o.get("tree_edges", o.get("tree_edges_example", []))
    if len(names) != len(nodes) - 1:
        return 1.0
    best = {}
    for u, v, w in E:
        for key in (f"{u}-{v}", f"{v}-{u}"):
            best[key] = min(best.get(key, w), w)
    if any(nm not in best for nm in names):
        return 1.0
    tot = sum(best[nm] for nm in names)
    inn, frontier = {nodes[0]}, F(0)                     # Prim, an independent recomputation of the minimum
    while len(inn) < len(nodes):
        cand = [(w, v) for u, v, w in E if u in inn and v not in inn] + [(w, u) for u, v, w in E if v in inn and u not in inn]
        if not cand:
            return 1.0
        w, v = min(cand)
        frontier += w
        inn.add(v)
    return max(_rel(o["total_weight"], float(tot)), _rel(o["total_weight"], float(frontier)))


_add("minimum_spanning_tree",
     name="Minimum spanning tree (Kruskal's algorithm)", topic="network optimisation / spanning trees",
     purpose="The least total weight of edges connecting all nodes of an undirected graph, and the tree (stated as THE "
             "tree only when the minimum spanning tree is unique).",
     inputs=[_I("edges", "undirected weighted edges 'A-B 4, ...'", "edges", min_length=1, max_length=300)],
     outputs=[_O("total_weight", "weight of a minimum spanning tree"), _O("tree_edges", "the minimum spanning tree (only when unique; otherwise tree_edges_example)"),
              _O("tree_unique", "yes when the minimum spanning tree is unique", "text")],
     primary="total_weight", conditions=["connected undirected graph"],
     method="Kruskal (edges by weight, union-find); uniqueness by the cycle property; total re-checked by Prim",
     latex=r"\min_{T \text{ spanning tree}} \sum_{e \in T} w_e",
     run=mst, verify=_verify_mst,
     parse=lambda q: ({"edges": et} if (et := _edges_text(q)) else {}) or None if re.search(r"\b(minimum spanning tree|minimal spanning tree|mst|minimum cost to connect)\b", q.lower()) else None,
     reference=lambda edges: {"total_weight": float(__import__("networkx").minimum_spanning_tree(_nx_graph(edges, False)).size(weight="weight"))},
     wrong=lambda edges: (lambda E, n: {"total_weight": float(sum(sorted(w for _, _, w in E)[:n - 1])), "tree_edges": []})(_wedges(edges), len(_nodes(edges))),
     random_inputs=lambda rng: {"edges": _random_graph(rng)[1]},
     keywords=["minimum spanning tree", "kruskal", "prim"],
     templates=["Find the minimum spanning tree of the graph with edges A-B 4, A-C 1, B-C 2, B-D 5, C-D 8."],
     examples=[{"inputs": {"edges": "A-B 4, A-C 1, B-C 2, B-D 5, C-D 8"}, "outputs": {"total_weight": 8.0, "tree_edges": ["A-C", "B-C", "B-D"]}}],
     questions=[{"q": "Find the minimum spanning tree of the graph with edges A-B 4, A-C 1, B-C 2, B-D 5, C-D 8.", "outputs": {"total_weight": 8.0}},
                {"q": "Find the minimum spanning tree of the graph with edges A-B 4, C-D 1.", "outcome": "INVALID_INPUT"}])


def max_flow(edges, source, target):
    E = _wedges(edges)
    if any(w < 0 for _, _, w in E):
        raise ValueError("capacities must be >= 0")
    s, t = str(source).strip(), str(target).strip()
    nodes = _nodes(E)
    if s not in nodes or t not in nodes or s == t:
        raise ValueError("the source and the sink must be two different nodes of the network")
    cap = {}
    for u, v, w in E:
        cap[(u, v)] = cap.get((u, v), F(0)) + w
        cap.setdefault((v, u), F(0))
    flow = {k: F(0) for k in cap}
    nbr = {}
    for u, v in cap:
        nbr.setdefault(u, []).append(v)
    value = F(0)
    while True:                                          # Edmonds-Karp: shortest augmenting paths
        prev, q = {s: None}, [s]
        for x in q:
            for y in nbr.get(x, []):
                if y not in prev and cap[(x, y)] - flow[(x, y)] > 0:
                    prev[y] = x
                    q.append(y)
        if t not in prev:
            break
        path, y = [], t
        while prev[y] is not None:
            path.append((prev[y], y))
            y = prev[y]
        aug = min(cap[e] - flow[e] for e in path)
        for x, y in path:
            flow[(x, y)] += aug
            flow[(y, x)] -= aug
        value += aug
    side = [v for v in nodes if v in prev]
    left, per = {k: max(flow[k], F(0)) for k in cap}, []
    for u, v, w in E:                                    # split the net flow over parallel edges in input order
        f = min(w, left[(u, v)])
        left[(u, v)] -= f
        per.append(float(f))
    return {"max_flow": float(value), "min_cut_source_side": side, "edge_flows": per}


def _verify_maxflow(i, o):
    E = _wedges(i["edges"])
    s, t = str(i["source"]).strip(), str(i["target"]).strip()
    f = [_fr(v) for v in o["edge_flows"]]
    if len(f) != len(E) or any(x < 0 or x > w for x, (_, _, w) in zip(f, E)):
        return 1.0
    bal = {}
    for x, (u, v, _) in zip(f, E):
        bal[u] = bal.get(u, 0) - x
        bal[v] = bal.get(v, 0) + x
    if any(b != 0 for k, b in bal.items() if k not in (s, t)):
        return 1.0
    S = set(o["min_cut_source_side"])
    if s not in S or t in S:
        return 1.0
    cut = sum(w for u, v, w in E if u in S and v not in S)
    val = -bal.get(s, 0)
    return max(_rel(o["max_flow"], float(val)), _rel(o["max_flow"], float(cut)))


def _random_flow(rng):
    names, edges = _random_graph(rng)
    edges = [(u, v, w) if names.index(u) < names.index(v) else (v, u, w) for u, v, w in edges]
    return {"edges": edges, "source": names[0], "target": names[-1]}


_add("max_flow",
     name="Maximum flow and minimum cut (Edmonds-Karp)", topic="network optimisation / flows",
     purpose="The largest flow that can be sent from a source to a sink through a network of capacitated arcs, a flow "
             "achieving it and the minimum cut that proves it optimal.",
     inputs=[_I("edges", "directed arcs with capacities 'S-A 10, A-T 5, ...'", "edges", min_length=1, max_length=300),
             _I("source", "source node", "text"), _I("target", "sink node", "text")],
     outputs=[_O("max_flow", "maximum flow value"), _O("min_cut_source_side", "nodes on the source side of a minimum cut (those reachable in the final residual network)"),
              _O("edge_flows", "flow on each arc, in input order (one maximum flow)")],
     primary="max_flow", conditions=["directed arcs", "capacities >= 0"],
     method="Edmonds-Karp (breadth-first augmenting paths) in exact arithmetic; certified by the max-flow min-cut theorem",
     latex=r"\max |f| = \min_{(S, \bar S)} c(S, \bar S)",
     run=max_flow, verify=_verify_maxflow,
     parse=lambda q: (lambda g: {k: v for k, v in g.items() if k != "directed"} or None)(_graph_q(q))
     if re.search(r"\b(maximum flow|max flow|max-flow|maximal flow|minimum cut|min cut)\b", q.lower()) and not re.search(r"\bundirected\b", q.lower()) else None,
     reference=lambda edges, source, target: {"max_flow": float(__import__("networkx").maximum_flow_value(
         (lambda G: [G.add_edge(str(u), str(v), capacity=G.get_edge_data(str(u), str(v), {"capacity": 0})["capacity"] + float(w)) for u, v, w in edges] and G)(__import__("networkx").DiGraph()),
         str(source), str(target)))},
     wrong=lambda edges, source, target: {"max_flow": float(sum(_fr(w) for u, v, w in edges if str(u) == str(source))), "min_cut_source_side": [str(source)],
                                          "edge_flows": [float(w) for u, v, w in edges]},
     random_inputs=_random_flow,
     keywords=["maximum flow", "minimum cut", "max flow"],
     templates=["Find the maximum flow from S to T in the network with edges S-A 10, S-B 5, A-B 15, A-T 5, B-T 10."],
     examples=[{"inputs": {"edges": "S-A 10, S-B 5, A-B 15, A-T 5, B-T 10", "source": "S", "target": "T"},
                "outputs": {"max_flow": 15.0, "min_cut_source_side": ["S"]}}],
     questions=[{"q": "Find the maximum flow from S to T in the network with edges S-A 10, S-B 5, A-B 15, A-T 5, B-T 10.", "outputs": {"max_flow": 15.0}}])


# --- LP-based network problems (exact simplex + dual certificate) ------------------------------------------------
def _cert_from_lp(sense, c, A, b, rel, x, p, value):
    return _lp_certificate_residual({"sense": sense, "c": c, "A": A, "b": b, "relations": ",".join(rel)}, x, p, value)


def _transport_lp(supply, demand, cost):
    m, n = len(supply), len(demand)
    if len(cost) != m or any(len(r) != n for r in cost):
        raise ValueError(f"the cost matrix must have {m} rows (sources) and {n} columns (destinations)")
    if any(v < 0 for v in supply) or any(v < 0 for v in demand):
        raise ValueError("supplies and demands must be >= 0")
    if sum(_fr(v) for v in supply) < sum(_fr(v) for v in demand):
        raise ValueError("total supply is less than total demand: the demand cannot be met")
    if m * n > 60:
        raise ValueError("at most 60 routes (sources x destinations)")
    c = [cost[i][j] for i in range(m) for j in range(n)]
    A, b, rel = [], [], []
    for i in range(m):
        A.append([1 if k // n == i else 0 for k in range(m * n)]); b.append(supply[i]); rel.append("<=")
    for j in range(n):
        A.append([1 if k % n == j else 0 for k in range(m * n)]); b.append(demand[j]); rel.append("=")
    return "minimize", c, A, b, rel


def transportation(supply, demand, cost):
    sense, c, A, b, rel = _transport_lp(supply, demand, cost)
    r = _lp(sense, c, A, b, rel)
    n = len(demand)
    plan = [[float(r["x"][i * n + j]) for j in range(n)] for i in range(len(supply))]
    out = {"min_cost": float(r["value"]), ("shipments" if r["x_unique"] else "shipments_example"): plan,
           "plan_unique": "yes" if r["x_unique"] else "no", "dual": _fl(r["p"])}
    return out


def _verify_transport(i, o):
    try:
        sense, c, A, b, rel = _transport_lp(i["supply"], i["demand"], i["cost"])
    except ValueError:
        return 1.0
    plan = o.get("shipments", o.get("shipments_example"))
    x = [v for r in plan for v in r]
    return _cert_from_lp(sense, c, A, b, rel, x, o["dual"], o["min_cost"])


def _ref_transport(supply, demand, cost):
    """INDEPENDENT reference: scipy.optimize.linprog (HiGHS) on the transportation LP."""
    sense, c, A, b, rel = _transport_lp(supply, demand, cost)
    return {"min_cost": _scipy_lp(sense, c, A, b, rel)[0]}


def _random_transport(rng):
    m, n = rng.randint(2, 4), rng.randint(2, 5)
    demand = [rng.randint(5, 50) for _ in range(n)]
    supply = [rng.randint(5, 50) for _ in range(m)]
    extra = sum(demand) - sum(supply)
    supply[-1] += max(0, extra) + rng.choice([0, 0, rng.randint(1, 20)])
    return {"supply": supply, "demand": demand, "cost": [[rng.randint(1, 20) for _ in range(n)] for _ in range(m)]}


_add("transportation_problem",
     name="Transportation problem (minimum shipping cost)", topic="network optimisation / transportation",
     purpose="Ship from sources with given supplies to destinations with given demands at least total cost: the minimum "
             "cost and a shipping plan (stated as THE plan only when it is unique).",
     inputs=[_I("supply", "supply of each source", "vector", min_length=1, max_length=10), _I("demand", "demand of each destination", "vector", min_length=1, max_length=10),
             _I("cost", "unit shipping cost, rows = sources, columns = destinations", "matrix")],
     outputs=[_O("min_cost", "minimum total shipping cost"), _O("shipments", "optimal plan, rows = sources (only when unique; otherwise shipments_example)"),
              _O("plan_unique", "yes when the optimal plan is unique", "text"), _O("dual", "dual values (u for sources, v for destinations): the optimality certificate")],
     primary="min_cost", conditions=["total supply >= total demand (surplus stays at the sources at no cost)", "at most 60 routes"],
     method="the transportation LP solved exactly (rational simplex), certified by its dual (u_i + v_j <= c_ij)",
     latex=r"\min \sum_{ij} c_{ij} x_{ij}\ \text{s.t.}\ \sum_j x_{ij} \le s_i,\ \sum_i x_{ij} = d_j,\ x \ge 0",
     run=transportation, verify=_verify_transport, reference=_ref_transport,
     parse=lambda q: (_labelled(q, {"supply": r"suppl(?:y|ies)|capacit(?:y|ies)", "demand": r"demands?|requirements?", "cost": r"costs?|cost matrix|unit costs?"}) or None)
     if re.search(r"\b(transportation problem|shipping cost|transport(?:ation)? cost)\b", q.lower()) else None,
     wrong=lambda supply, demand, cost: {"min_cost": float(sum(min(r) for r in cost) * sum(demand) / len(cost)), "shipments": [[0.0] * len(demand) for _ in supply], "dual": [0.0] * (len(supply) + len(demand))},
     random_inputs=_random_transport,
     keywords=["transportation problem", "supply", "demand"],
     templates=["Solve the transportation problem with supply [20, 30, 25], demand [10, 35, 30] and cost matrix [[8, 6, 10], [9, 12, 13], [14, 9, 16]]."],
     examples=[{"inputs": {"supply": [20, 30], "demand": [10, 25, 15], "cost": [[2, 3, 1], [5, 4, 8]]},
                "outputs": {"min_cost": 150.0}}],
     questions=[{"q": "Solve the transportation problem with supply [20, 30], demand [10, 25, 15] and cost matrix [[2, 3, 1], [5, 4, 8]].",
                 "outputs": {"min_cost": 150.0}},
                {"q": "Solve the transportation problem with supply [5, 5], demand [10, 25, 15] and cost matrix [[2, 3, 1], [5, 4, 8]].", "outcome": "INVALID_INPUT"}])


def _mcf_lp(capacity, cost, supply):
    n = len(supply)
    if len(capacity) != n or len(cost) != n or any(len(r) != n for r in capacity + cost):
        raise ValueError(f"capacity and cost must be {n} x {n} matrices (one row and column per node)")
    if sum(_fr(v) for v in supply) != 0:
        raise ValueError("supplies (positive) and demands (negative) must sum to 0")
    arcs = [(i, j) for i in range(n) for j in range(n) if i != j and capacity[i][j] > 0]
    if not arcs or len(arcs) > 60:
        raise ValueError("give between 1 and 60 arcs (positive capacities)")
    c = [cost[i][j] for i, j in arcs]
    A, b, rel = [], [], []
    for v in range(n):
        A.append([(1 if i == v else 0) - (1 if j == v else 0) for i, j in arcs]); b.append(supply[v]); rel.append("=")
    for k, (i, j) in enumerate(arcs):
        A.append([1 if q == k else 0 for q in range(len(arcs))]); b.append(capacity[i][j]); rel.append("<=")
    return arcs, ("minimize", c, A, b, rel)


def min_cost_flow(capacity, cost, supply):
    arcs, lp = _mcf_lp(capacity, cost, supply)
    r = _lp(*lp)
    n = len(supply)
    fl = [[0.0] * n for _ in range(n)]
    for (i, j), x in zip(arcs, r["x"]):
        fl[i][j] = float(x)
    return {"min_cost": float(r["value"]), ("flows" if r["x_unique"] else "flows_example"): fl, "flow_unique": "yes" if r["x_unique"] else "no",
            "dual": _fl(r["p"])}


def _verify_mcf(i, o):
    try:
        arcs, lp = _mcf_lp(i["capacity"], i["cost"], i["supply"])
    except ValueError:
        return 1.0
    fl = o.get("flows", o.get("flows_example"))
    return _cert_from_lp(*lp, [fl[a][b] for a, b in arcs], o["dual"], o["min_cost"])


def _ref_mcf(capacity, cost, supply):
    """INDEPENDENT reference: networkx network simplex (min_cost_flow_cost)."""
    import networkx as nx
    G = nx.DiGraph()
    for v, s in enumerate(supply):
        G.add_node(v, demand=-s)
    for i in range(len(supply)):
        for j in range(len(supply)):
            if i != j and capacity[i][j] > 0:
                G.add_edge(i, j, capacity=capacity[i][j], weight=cost[i][j])
    return {"min_cost": float(nx.min_cost_flow_cost(G))}


def _random_mcf(rng):
    n = rng.randint(3, 6)
    cap = [[0] * n for _ in range(n)]
    cost = [[0] * n for _ in range(n)]
    for i in range(n):
        for j in range(n):
            if i != j and rng.random() < 0.6:
                cap[i][j], cost[i][j] = rng.randint(5, 30), rng.randint(1, 15)
    for i in range(n - 1):                              # a path 0 -> ... -> n-1 with spare capacity keeps it feasible
        cap[i][i + 1], cost[i][i + 1] = max(cap[i][i + 1], 40), cost[i][i + 1] or rng.randint(1, 15)
    s = rng.randint(1, 20)
    supply = [0] * n
    supply[0], supply[-1] = s, -s
    if n > 3 and rng.random() < 0.5:
        k = rng.randint(1, s)
        supply[-1] += k
        supply[rng.randint(1, n - 2)] -= k
    return {"capacity": cap, "cost": cost, "supply": supply}


_add("min_cost_flow",
     name="Minimum-cost flow (transshipment)", topic="network optimisation / flows",
     purpose="Send flow from supply nodes to demand nodes through capacitated arcs at least total cost (transshipment "
             "allowed): the minimum cost and a flow achieving it.",
     inputs=[_I("capacity", "arc capacities, n x n (row = from node, column = to node, 0 = no arc)", "matrix"),
             _I("cost", "unit cost of each arc, n x n", "matrix"),
             _I("supply", "net supply of each node (positive = supply, negative = demand), summing to 0", "vector", min_length=2, max_length=12)],
     outputs=[_O("min_cost", "minimum total cost"), _O("flows", "optimal flow matrix (only when unique; otherwise flows_example)"),
              _O("flow_unique", "yes when the optimal flow is unique", "text"), _O("dual", "dual values: node potentials then arc-capacity prices")],
     primary="min_cost", conditions=["supplies sum to 0", "a feasible flow exists", "at most 60 arcs"],
     method="the min-cost-flow LP solved exactly (rational simplex) and certified by its dual",
     latex=r"\min \sum c_{ij} x_{ij}\ \text{s.t.}\ \sum_j x_{vj} - \sum_i x_{iv} = b_v,\ 0 \le x_{ij} \le u_{ij}",
     run=min_cost_flow, verify=_verify_mcf, reference=_ref_mcf,
     parse=lambda q: (_labelled(q, {"capacity": r"capacit(?:y|ies)(?: matrix)?", "cost": r"costs?(?: matrix)?", "supply": r"suppl(?:y|ies)|net supplies|balances?"}) or None)
     if re.search(r"\b(minimum[- ]cost flow|min[- ]cost flow|transshipment)\b", q.lower()) else None,
     wrong=lambda capacity, cost, supply: {"min_cost": 0.0, "flows": [[0.0] * len(supply) for _ in supply], "dual": []},
     random_inputs=_random_mcf,
     keywords=["minimum cost flow", "transshipment"],
     templates=["Find the minimum cost flow with capacity [[0, 10, 5], [0, 0, 10], [0, 0, 0]], cost [[0, 1, 4], [0, 0, 2], [0, 0, 0]] and supply [8, 0, -8]."],
     examples=[{"inputs": {"capacity": [[0, 10, 5], [0, 0, 10], [0, 0, 0]], "cost": [[0, 1, 4], [0, 0, 2], [0, 0, 0]], "supply": [8, 0, -8]},
                "outputs": {"min_cost": 24.0}}],
     questions=[{"q": "Find the minimum cost flow with capacity [[0, 10, 5], [0, 0, 10], [0, 0, 0]], cost [[0, 1, 4], [0, 0, 2], [0, 0, 0]] and supply [8, 0, -8].",
                 "outputs": {"min_cost": 24.0}}])


# --- assignment (Hungarian method with dual potentials) ------------------------------------------------------------
def _hungarian(C):
    n = len(C)
    INF = float("inf")
    u, v, p, way = [F(0)] * (n + 1), [F(0)] * (n + 1), [0] * (n + 1), [0] * (n + 1)
    for i in range(1, n + 1):
        p[0], j0 = i, 0
        minv, used = [INF] * (n + 1), [False] * (n + 1)
        while True:
            used[j0] = True
            i0, delta, j1 = p[j0], INF, 0
            for j in range(1, n + 1):
                if not used[j]:
                    cur = C[i0 - 1][j - 1] - u[i0] - v[j]
                    if cur < minv[j]:
                        minv[j], way[j] = cur, j0
                    if minv[j] < delta:
                        delta, j1 = minv[j], j
            for j in range(n + 1):
                if used[j]:
                    u[p[j]] += delta
                    v[j] -= delta
                else:
                    minv[j] -= delta
            j0 = j1
            if p[j0] == 0:
                break
        while True:
            j1 = way[j0]
            p[j0] = p[j1]
            j0 = j1
            if j0 == 0:
                break
    a = [0] * n
    for j in range(1, n + 1):
        a[p[j] - 1] = j - 1
    return a, u[1:], v[1:]


def _assignment(C, maximize):
    n = len(C)
    if n == 0 or n > 12 or any(len(r) != n for r in C):
        raise ValueError("give a square matrix with at most 12 rows (add dummy rows or columns of zeros for an unbalanced problem)")
    M = [[(-_fr(x) if maximize else _fr(x)) for x in r] for r in C]
    a, u, v = _hungarian(M)
    best = sum(M[i][a[i]] for i in range(n))
    unique = True
    for i in range(n):                                   # unique iff forbidding any chosen cell raises the cost
        big = sum(abs(x) for r in M for x in r) + 1
        M2 = [list(r) for r in M]
        M2[i][a[i]] += big
        a2, _, _ = _hungarian(M2)
        if sum(M2[k][a2[k]] for k in range(n)) == best:
            unique = False
            break
    s = -1 if maximize else 1
    out = {("max_value" if maximize else "min_cost"): float(s * best), ("assignment" if unique else "assignment_example"): [j + 1 for j in a],
           "assignment_unique": "yes" if unique else "no", "dual_u": [float(s * x) for x in u], "dual_v": [float(s * x) for x in v]}
    return out


def _verify_assign(i, o, maximize):
    C = i["C"]
    n = len(C)
    a = [int(round(x)) - 1 for x in o.get("assignment", o.get("assignment_example"))]
    if sorted(a) != list(range(n)):
        return 1.0
    key = "max_value" if maximize else "min_cost"
    s = -1 if maximize else 1
    u, v = [s * x for x in o["dual_u"]], [s * x for x in o["dual_v"]]
    worst = _rel(o[key], sum(C[k][a[k]] for k in range(n)))
    scale = max(1.0, max(abs(x) for r in C for x in r))
    for r in range(n):
        for c in range(n):
            worst = max(worst, max(0.0, u[r] + v[c] - s * C[r][c]) / scale)
    return max(worst, _rel(o[key], s * (sum(u) + sum(v))))


def _ref_assign(C, maximize):
    import numpy as np
    from scipy.optimize import linear_sum_assignment
    M = np.array(C, float)
    r, c = linear_sum_assignment(M, maximize=maximize)
    return {("max_value" if maximize else "min_cost"): float(M[r, c].sum())}


def _parse_assign(q, maximize):
    ql = q.lower()
    if not re.search(r"\b(assignment problem|hungarian|assign (?:each|the|\d+) \w+ to)\b", ql):
        return None
    if _affirmed(ql, r"\b(maximi[sz]e|maximum|profit|revenue|score)\b") != maximize:
        return None
    g = _labelled(q, {"C": r"matrix|costs?|times?|profits?|values?|scores?|table"})
    return g or None


for _mx in (False, True):
    _k = "assignment_max_profit" if _mx else "assignment_min_cost"
    _add(_k,
         name="Assignment problem (Hungarian method), " + ("maximum total value" if _mx else "minimum total cost"),
         topic="network optimisation / assignment",
         purpose=("Assign each row (worker) to a different column (task) so that the total " + ("value is as large" if _mx else "cost is as small")
                  + " as possible: the optimum and an optimal assignment (stated as THE assignment only when unique)."),
         inputs=[_I("C", ("value" if _mx else "cost") + " matrix, rows = workers, columns = tasks (square)", "matrix")],
         outputs=[_O("max_value" if _mx else "min_cost", "optimal total " + ("value" if _mx else "cost")),
                  _O("assignment", "column assigned to each row, 1-based (only when unique; otherwise assignment_example)"),
                  _O("assignment_unique", "yes when the optimal assignment is unique", "text"),
                  _O("dual_u", "row potentials"), _O("dual_v", "column potentials (u_i + v_j bounds every cell: the optimality certificate)")],
         primary="max_value" if _mx else "min_cost", conditions=["square matrix, at most 12 x 12 (pad with zeros for unbalanced problems)"],
         method="Hungarian method (shortest augmenting paths with potentials) in exact arithmetic; uniqueness by forbidding each chosen cell",
         latex=r"\min_{\sigma} \sum_i c_{i\sigma(i)} = \max \Big\{ \sum_i u_i + \sum_j v_j : u_i + v_j \le c_{ij} \Big\}",
         run=(lambda C, _m=_mx: _assignment(C, _m)), verify=(lambda i, o, _m=_mx: _verify_assign(i, o, _m)),
         parse=(lambda q, _m=_mx: _parse_assign(q, _m)), reference=(lambda C, _m=_mx: _ref_assign(C, _m)),
         wrong=(lambda C, _m=_mx: {("max_value" if _m else "min_cost"): float(sum(C[k][k] for k in range(len(C)))), "assignment": list(range(1, len(C) + 1)),
                                   "dual_u": [0.0] * len(C), "dual_v": [0.0] * len(C)}),
         random_inputs=lambda rng: (lambda n: {"C": [[rng.randint(1, 30) for _ in range(n)] for _ in range(n)]})(rng.randint(2, 7)),
         keywords=["assignment problem", "hungarian method"],
         templates=[("Solve the assignment problem to maximize total profit with matrix [[9, 2, 7], [6, 4, 3], [5, 8, 1]]." if _mx
                     else "Solve the assignment problem with cost matrix [[9, 2, 7], [6, 4, 3], [5, 8, 1]].")],
         examples=[{"inputs": {"C": "9 2 7; 6 4 3; 5 8 1"}, "outputs": ({"max_value": 23.0, "assignment": [1, 3, 2]} if False else
                                                                       ({"max_value": 21.0, "assignment": [3, 1, 2]} if _mx else {"min_cost": 9.0, "assignment": [2, 1, 3]}))}],
         questions=[{"q": ("Solve the assignment problem to maximize total profit with matrix [[9, 2, 7], [6, 4, 3], [5, 8, 1]]." if _mx
                           else "Solve the assignment problem with cost matrix [[9, 2, 7], [6, 4, 3], [5, 8, 1]]."),
                     "outputs": ({"max_value": 21.0} if _mx else {"min_cost": 9.0})},
                    {"q": "Solve the assignment problem with cost matrix [[9, 2, 7], [6, 4, 3]]." if not _mx else
                     "Solve the assignment problem to maximize total profit with matrix [[9, 2, 7], [6, 4, 3]].", "outcome": "INVALID_INPUT"}])


def bipartite_matching(edges):
    E = _wedges(edges, need_weight=False)
    left = _nodes([(u, u) for u, _ in E])
    right = _nodes([(v, v) for _, v in E])
    if set(left) & set(right):
        raise ValueError("a node appears on both sides: give edges as left-right pairs of a bipartite graph")
    adj = {u: [] for u in left}
    for u, v in E:
        if v not in adj[u]:
            adj[u].append(v)
    match_r = {}

    def aug(u, seen):
        for v in adj[u]:
            if v not in seen:
                seen.add(v)
                if v not in match_r or aug(match_r[v], seen):
                    match_r[v] = u
                    return True
        return False
    for u in left:
        aug(u, set())
    match_l = {u: v for v, u in match_r.items()}
    Z, stack = set(), [u for u in left if u not in match_l]   # Konig: alternating reachability from free left nodes
    Z.update(stack)
    while stack:
        u = stack.pop()
        for v in adj[u]:
            if v not in Z:
                Z.add(v)
                w = match_r.get(v)
                if w is not None and w not in Z:
                    Z.add(w)
                    stack.append(w)
    cover = [u for u in left if u not in Z] + [v for v in right if v in Z]
    return {"max_matching": float(len(match_l)), "matching_example": [f"{u}-{match_l[u]}" for u in left if u in match_l], "vertex_cover": cover}


def _verify_bm(i, o):
    E = _wedges(i["edges"], need_weight=False)
    pairs = [tuple(s.split("-", 1)) for s in o["matching_example"]]
    es = set(E)
    if any(p not in es for p in pairs) or len({p[0] for p in pairs}) != len(pairs) or len({p[1] for p in pairs}) != len(pairs):
        return 1.0
    cov = set(o["vertex_cover"])
    if any(u not in cov and v not in cov for u, v in E):
        return 1.0
    return max(_rel(o["max_matching"], len(pairs)), _rel(o["max_matching"], len(cov)))


def _ref_bm(edges):
    import networkx as nx
    G = nx.Graph()
    left = {("L", str(u)) for u, v in edges}
    G.add_edges_from((("L", str(u)), ("R", str(v))) for u, v in edges)
    return {"max_matching": float(len(nx.bipartite.hopcroft_karp_matching(G, top_nodes=left)) // 2)}


_add("bipartite_matching",
     name="Maximum bipartite matching", topic="network optimisation / matching",
     purpose="The largest number of pairs that can be formed between two groups (workers and jobs, students and "
             "projects) using only allowed pairs, with one maximum matching and a vertex cover proving it maximum.",
     inputs=[_I("edges", "allowed pairs 'left-right', e.g. 'W1-J1, W1-J2, W2-J1'", "edges", min_length=1, max_length=300)],
     outputs=[_O("max_matching", "size of a maximum matching"), _O("matching_example", "one maximum matching"),
              _O("vertex_cover", "a minimum vertex cover of the same size (Konig's theorem: the certificate)")],
     primary="max_matching", conditions=["bipartite: the first node of each pair is on the left, the second on the right"],
     method="augmenting paths (Kuhn); minimum vertex cover from alternating reachability (Konig)",
     latex=r"\nu(G) = \tau(G) \text{ for bipartite } G",
     run=bipartite_matching, verify=_verify_bm, reference=_ref_bm,
     parse=lambda q: ({"edges": et} if (et := _edges_text(q)) else {}) or None
     if re.search(r"\b(maximum (?:bipartite |cardinality )?matching|bipartite matching|maximum number of pairs)\b", q.lower()) and not re.search(r"\bweight", q.lower()) else None,
     wrong=lambda edges: {"max_matching": float(len({str(e[0]) for e in edges})), "matching_example": [], "vertex_cover": []},
     random_inputs=lambda rng: (lambda nl: (lambda nr: {"edges": sorted({(f"L{l}", f"R{rng.randint(1, nr)}") for l in range(1, nl + 1) for _ in range(rng.randint(1, 3))})})(rng.randint(2, nl)))(rng.randint(4, 8)),
     keywords=["bipartite matching", "maximum matching"],
     templates=["Find the maximum bipartite matching with edges W1-J1, W1-J2, W2-J1, W3-J2, W3-J3."],
     examples=[{"inputs": {"edges": "W1-J1, W1-J2, W2-J1, W3-J2, W3-J3"}, "outputs": {"max_matching": 3.0}},
               {"inputs": {"edges": "A-X, B-X, C-X"}, "outputs": {"max_matching": 1.0}}],
     questions=[{"q": "Find the maximum bipartite matching with edges W1-J1, W1-J2, W2-J1, W3-J2, W3-J3.", "outputs": {"max_matching": 3.0}}])


def max_weight_matching(edges):
    E = _wedges(edges)
    nodes = _nodes(E)
    if len(nodes) > 16:
        raise ValueError("at most 16 nodes")
    ix = {v: k for k, v in enumerate(nodes)}
    W = {}
    for u, v, w in E:
        if u != v:
            a, b = sorted((ix[u], ix[v]))
            W[(a, b)] = max(W.get((a, b), w), w)
    n = len(nodes)
    memo = {}

    def f(mask):                                         # best weight using the nodes NOT in mask
        if mask == (1 << n) - 1:
            return F(0), ()
        if mask in memo:
            return memo[mask]
        i = next(k for k in range(n) if not mask >> k & 1)
        best = f(mask | 1 << i)
        for j in range(i + 1, n):
            if not mask >> j & 1 and (i, j) in W and W[(i, j)] > 0:
                val, pairs = f(mask | 1 << i | 1 << j)
                if val + W[(i, j)] > best[0]:
                    best = (val + W[(i, j)], ((i, j),) + pairs)
        memo[mask] = best
        return best
    val, pairs = f(0)
    return {"max_weight": float(val), "matching_example": [f"{nodes[a]}-{nodes[b]}" for a, b in pairs]}


def _verify_mwm(i, o):
    E = _wedges(i["edges"])
    W = {}
    for u, v, w in E:
        W[(u, v)] = W[(v, u)] = max(W.get((u, v), w), w)
    pairs = [tuple(s.split("-", 1)) for s in o["matching_example"]]
    used = [x for p in pairs for x in p]
    if len(used) != len(set(used)) or any(p not in W for p in pairs):
        return 1.0
    return max(_rel(o["max_weight"], float(sum(W[p] for p in pairs))), _rel(o["max_weight"], max_weight_matching(i["edges"])["max_weight"]))


_add("max_weight_matching",
     name="Maximum-weight matching (general graph)", topic="network optimisation / matching",
     purpose="Pair up nodes of a graph (each node at most once) so that the total weight of the chosen edges is as "
             "large as possible: the maximum weight and one optimal matching.",
     inputs=[_I("edges", "undirected weighted edges 'A-B 4, ...'", "edges", min_length=1, max_length=120)],
     outputs=[_O("max_weight", "maximum total weight of a matching"), _O("matching_example", "one maximum-weight matching")],
     primary="max_weight", conditions=["at most 16 nodes", "edges of weight <= 0 are never worth choosing"],
     method="exact dynamic programming over node subsets", latex=r"\max_{M \text{ matching}} \sum_{e \in M} w_e",
     run=max_weight_matching, verify=_verify_mwm,
     reference=lambda edges: (lambda G: {"max_weight": float(sum(G[u][v]["weight"] for u, v in __import__("networkx").max_weight_matching(G)))})(
         (lambda G: G)(_nx_max_graph(edges))),
     parse=lambda q: ({"edges": et} if (et := _edges_text(q)) else {}) or None if re.search(r"\b(maximum[- ]weight(?:ed)? matching|max(?:imum)? weight matching)\b", q.lower()) else None,
     wrong=lambda edges: (lambda E: {"max_weight": float(max(w for _, _, w in E)), "matching_example": []})(_wedges(edges)),
     random_inputs=lambda rng: {"edges": _random_graph(rng, n=rng.randint(4, 10))[1]},
     keywords=["maximum weight matching"],
     templates=["Find the maximum weight matching of the graph with edges A-B 6, B-C 5, C-D 6, A-D 2, A-C 4."],
     examples=[{"inputs": {"edges": "A-B 6, B-C 5, C-D 6, A-D 2, A-C 4"}, "outputs": {"max_weight": 12.0}}],
     questions=[{"q": "Find the maximum weight matching of the graph with edges A-B 6, B-C 5, C-D 6, A-D 2, A-C 4.", "outputs": {"max_weight": 12.0}}])


def _nx_max_graph(edges):
    import networkx as nx
    G = nx.Graph()
    for u, v, w in edges:
        u, v = str(u), str(v)
        if u != v and (not G.has_edge(u, v) or G[u][v]["weight"] < float(w)):
            G.add_edge(u, v, weight=float(w))
    return G


def tsp(D):
    n = len(D)
    if n < 2 or n > 11 or any(len(r) != n for r in D):
        raise ValueError("give a square distance matrix with 2 to 11 cities")
    M = [[_fr(x) for x in r] for r in D]
    full = 1 << (n - 1)
    dp = {(1 << k, k): (M[0][k + 1], 0) for k in range(n - 1)}
    for mask in range(1, full):
        for k in range(n - 1):
            if not mask >> k & 1 or (mask, k) not in dp:
                continue
            base = dp[(mask, k)][0]
            for j in range(n - 1):
                if mask >> j & 1:
                    continue
                key = (mask | 1 << j, j)
                cand = base + M[k + 1][j + 1]
                if key not in dp or cand < dp[key][0]:
                    dp[key] = (cand, k)
    best = min((dp[(full - 1, k)][0] + M[k + 1][0], k) for k in range(n - 1))
    tour, mask, k = [], full - 1, best[1]
    while True:
        tour.append(k + 1)
        prev = dp[(mask, k)][1]
        if mask == 1 << k:
            break
        mask, k = mask ^ (1 << k), prev
    tour = [1] + [c + 1 for c in reversed(tour)]
    return {"min_tour_length": float(best[0]), "tour_example": tour + [1]}


_TSP_MEMO: dict = {}


def _verify_tsp(i, o):
    D = i["D"]
    t = [int(round(x)) - 1 for x in o["tour_example"]]
    n = len(D)
    if t[0] != 0 or t[-1] != 0 or sorted(t[:-1]) != list(range(n)):
        return 1.0
    worst = _rel(o["min_tour_length"], sum(D[a][b] for a, b in zip(t, t[1:])))
    key = repr(D)
    if key not in _TSP_MEMO:
        _TSP_MEMO.clear() if len(_TSP_MEMO) > 64 else None
        _TSP_MEMO[key] = tsp(D)["min_tour_length"]
    return max(worst, _rel(o["min_tour_length"], _TSP_MEMO[key]))


def _ref_tsp(D):
    """INDEPENDENT reference: Miller-Tucker-Zemlin integer program (scipy.optimize.milp, zero gap)."""
    import numpy as np
    from scipy.optimize import milp, LinearConstraint, Bounds
    n = len(D)
    X = lambda i, j: i * n + j
    nv = n * n + n
    c = np.zeros(nv)
    for i in range(n):
        for j in range(n):
            c[X(i, j)] = D[i][j] if i != j else 0
    rows, lo, hi = [], [], []
    for i in range(n):
        r = np.zeros(nv); r[[X(i, j) for j in range(n) if j != i]] = 1; rows.append(r); lo.append(1); hi.append(1)
        r = np.zeros(nv); r[[X(j, i) for j in range(n) if j != i]] = 1; rows.append(r); lo.append(1); hi.append(1)
    for i in range(1, n):
        for j in range(1, n):
            if i != j:
                r = np.zeros(nv); r[n * n + i] = 1; r[n * n + j] = -1; r[X(i, j)] = n
                rows.append(r); lo.append(-np.inf); hi.append(n - 1)
    ub = np.r_[np.array([0 if i == j else 1 for i in range(n) for j in range(n)], float), np.full(n, n)]
    res = milp(c, constraints=LinearConstraint(np.array(rows), lo, hi), integrality=np.r_[np.ones(n * n), np.zeros(n)],
               bounds=Bounds(np.zeros(nv), ub), options={"mip_rel_gap": 0})
    x = np.round(res.x[:n * n])
    return {"min_tour_length": float(sum(D[i][j] * x[X(i, j)] for i in range(n) for j in range(n)))}


_add("traveling_salesman",
     name="Travelling salesman tour (exact, Held-Karp)", topic="network optimisation / routing",
     purpose="The shortest round trip visiting every city exactly once and returning to the start, for a distance "
             "matrix (symmetric or not): the minimum tour length and one optimal tour.",
     inputs=[_I("D", "distance matrix, D[i][j] = distance from city i to city j", "matrix")],
     outputs=[_O("min_tour_length", "length of a shortest tour"), _O("tour_example", "one optimal tour starting and ending at city 1")],
     primary="min_tour_length", conditions=["2 to 11 cities"],
     method="Held-Karp dynamic programming over city subsets, exact arithmetic", latex=r"g(S, k) = \min_{m \in S \setminus \{k\}} \big(g(S \setminus \{k\}, m) + d_{mk}\big)",
     run=tsp, verify=_verify_tsp, reference=_ref_tsp,
     parse=lambda q: (_labelled(q, {"D": r"matrix|distances?|costs?|times?"}) or None) if re.search(r"\b(travel(?:l)?ing salesman|tsp|shortest tour|round trip visiting)\b", q.lower()) else None,
     wrong=lambda D: (lambda t: {"min_tour_length": float(sum(D[a][b] for a, b in zip(t, t[1:]))), "tour_example": [x + 1 for x in t]})(list(range(len(D))) + [0]),
     random_inputs=lambda rng: (lambda n: (lambda P: {"D": [[0 if i == j else abs(P[i][0] - P[j][0]) + abs(P[i][1] - P[j][1]) + rng.randint(0, 3) for j in range(n)] for i in range(n)]})(
         [(rng.randint(0, 30), rng.randint(0, 30)) for _ in range(n)]))(rng.randint(4, 8)),
     keywords=["travelling salesman", "tsp", "shortest tour"],
     templates=["Solve the travelling salesman problem with distance matrix [[0, 10, 15, 20], [10, 0, 35, 25], [15, 35, 0, 30], [20, 25, 30, 0]]."],
     examples=[{"inputs": {"D": [[0, 10, 15, 20], [10, 0, 35, 25], [15, 35, 0, 30], [20, 25, 30, 0]]}, "outputs": {"min_tour_length": 80.0}}],
     questions=[{"q": "Solve the travelling salesman problem with distance matrix [[0, 10, 15, 20], [10, 0, 35, 25], [15, 35, 0, 30], [20, 25, 30, 0]].",
                 "outputs": {"min_tour_length": 80.0}}])


# =================================================================================================================
# combinatorial optimisation (exact; references are mixed-integer programs)
# =================================================================================================================
def _milp_ref(c, A_rows, lo, hi, ub, maximize=False, integral=None):
    import numpy as np
    from scipy.optimize import milp, LinearConstraint, Bounds
    c = np.array(c, float)
    res = milp(-c if maximize else c, constraints=LinearConstraint(np.array(A_rows, float), lo, hi),
               integrality=np.ones(len(c)) if integral is None else np.array(integral), bounds=Bounds(np.zeros(len(c)), np.array(ub, float)),
               options={"mip_rel_gap": 0})
    x = np.round(res.x)
    return float(c @ x)


def knapsack01(values, weights, capacity):
    _finite(capacity)
    n = len(values)
    if n == 0 or n > 40 or len(weights) != n:
        raise ValueError("give between 1 and 40 items, one weight per value")
    if any(w < 0 for w in weights) or any(v < 0 for v in values) or capacity < 0:
        raise ValueError("values, weights and capacity must be >= 0")
    V, Wt, cap = [_fr(v) for v in values], [_fr(w) for w in weights], _fr(capacity)
    states = {F(0): (F(0), ())}                           # weight -> (best value, items); Pareto-pruned
    for k in range(n):
        new = dict(states)
        for w, (v, it) in states.items():
            nw = w + Wt[k]
            if nw <= cap and (nw not in new or v + V[k] > new[nw][0]):
                new[nw] = (v + V[k], it + (k,))
        front, best = {}, F(-1)
        for w in sorted(new):
            if new[w][0] > best:
                front[w], best = new[w], new[w][0]
        states = front
    w, (v, it) = max(states.items(), key=lambda kv: (kv[1][0], -kv[0]))
    return {"max_value": float(v), "items_example": [k + 1 for k in it], "total_weight": float(w)}


def _verify_knap(i, o):
    it = [int(round(x)) - 1 for x in o["items_example"]]
    if len(set(it)) != len(it) or any(not 0 <= k < len(i["values"]) for k in it):
        return 1.0
    w = sum(i["weights"][k] for k in it)
    if w > i["capacity"] + 1e-9:
        return 1.0
    return max(_rel(o["max_value"], sum(i["values"][k] for k in it)), _rel(o["max_value"], knapsack01(i["values"], i["weights"], i["capacity"])["max_value"]))


def _knap_q(q, unbounded):
    ql = q.lower()
    if "knapsack" not in ql or _affirmed(ql, r"\bunbounded|\bunlimited|any number of copies|\brepeat") != unbounded:
        return None
    out = _labelled(q, {"values": r"values?|profits?|benefits?", "weights": r"weights?|sizes?|costs?"})
    cap = _num_after(ql, r"capacity(?: of| is| =|:)?|limit(?: of| is)?")
    if cap is not None:
        out["capacity"] = cap
    return out or None


_add("knapsack_01",
     name="0/1 knapsack problem", topic="integer programming / knapsack",
     purpose="Choose items (each at most once) to maximise total value without exceeding the capacity: the maximum "
             "value, one optimal choice and its weight.",
     inputs=[_I("values", "value of each item", "vector", min_length=1, max_length=40), _I("weights", "weight of each item", "vector", min_length=1, max_length=40),
             _I("capacity", "knapsack capacity")],
     outputs=[_O("max_value", "maximum total value"), _O("items_example", "one optimal set of items (1-based; ties to the lighter set)"), _O("total_weight", "weight of that set")],
     primary="max_value", conditions=["values, weights, capacity >= 0", "at most 40 items"],
     method="exact dynamic programming over (weight, value) states with Pareto pruning, rational arithmetic",
     latex=r"\max \sum_i v_i x_i \ \text{s.t.}\ \sum_i w_i x_i \le W,\ x_i \in \{0,1\}",
     run=knapsack01, verify=_verify_knap, parse=lambda q: _knap_q(q, False),
     reference=lambda values, weights, capacity: {"max_value": _milp_ref(values, [weights], [-1e30], [capacity], [1] * len(values), True)},
     wrong=lambda values, weights, capacity: (lambda order: (lambda pick: {"max_value": float(sum(values[k] for k in pick)), "items_example": [k + 1 for k in pick]})(
         [k for k in order if sum(weights[j] for j in order[:order.index(k) + 1]) <= capacity]))(sorted(range(len(values)), key=lambda k: -values[k])),
     random_inputs=lambda rng: (lambda n: {"values": [rng.randint(1, 60) for _ in range(n)], "weights": [rng.randint(1, 30) for _ in range(n)], "capacity": rng.randint(10, 15 * n)})(rng.randint(3, 15)),
     keywords=["knapsack", "0/1 knapsack"],
     templates=["Solve the knapsack problem with values [60, 100, 120], weights [10, 20, 30] and capacity 50."],
     examples=[{"inputs": {"values": "60, 100, 120", "weights": "10, 20, 30", "capacity": 50}, "outputs": {"max_value": 220.0, "items_example": [2, 3]}}],
     questions=[{"q": "Solve the knapsack problem with values [60, 100, 120], weights [10, 20, 30] and capacity 50.", "outputs": {"max_value": 220.0}},
                {"q": "Solve the knapsack problem with values [60, 100, 120], weights [10, 20, 30] and capacity 50; items cannot be repeated.",
                 "outputs": {"max_value": 220.0}}])


def knapsack_unbounded(values, weights, capacity):
    n = len(values)
    if n == 0 or n > 40 or len(weights) != n:
        raise ValueError("give between 1 and 40 item types, one weight per value")
    if any(w <= 0 or w != int(w) for w in weights) or capacity != int(capacity) or not 0 <= capacity <= 100000:
        raise ValueError("weights must be positive whole numbers and the capacity a whole number up to 100000")
    if any(v < 0 for v in values):
        raise ValueError("values must be >= 0")
    cap, W, V = int(capacity), [int(w) for w in weights], [_fr(v) for v in values]
    best, choice = [F(0)] * (cap + 1), [-1] * (cap + 1)
    for c in range(1, cap + 1):
        best[c], choice[c] = best[c - 1], -1
        for k in range(n):
            if W[k] <= c and best[c - W[k]] + V[k] > best[c]:
                best[c], choice[c] = best[c - W[k]] + V[k], k
    counts, c = [0] * n, cap
    while c > 0:
        if choice[c] < 0:
            c -= 1
        else:
            counts[choice[c]] += 1
            c -= W[choice[c]]
    return {"max_value": float(best[cap]), "copies_example": [float(x) for x in counts]}


_add("knapsack_unbounded",
     name="Unbounded knapsack problem", topic="integer programming / knapsack",
     purpose="Choose any number of copies of each item type to maximise total value within the capacity.",
     inputs=[_I("values", "value of each item type", "vector", min_length=1, max_length=40), _I("weights", "weight of each item type (whole numbers)", "vector", min_length=1, max_length=40),
             _I("capacity", "capacity (a whole number)")],
     outputs=[_O("max_value", "maximum total value"), _O("copies_example", "number of copies of each type in one optimal choice")],
     primary="max_value", conditions=["positive whole-number weights", "capacity a whole number <= 100000"],
     method="dynamic programming over capacities", latex=r"V(c) = \max\big(V(c-1),\ \max_{i: w_i \le c} V(c - w_i) + v_i\big)",
     run=knapsack_unbounded,
     verify=lambda i, o: (1.0 if sum(i["weights"][k] * o["copies_example"][k] for k in range(len(i["values"]))) > i["capacity"] or any(x < 0 or x != int(x) for x in o["copies_example"])
                          else max(_rel(o["max_value"], sum(i["values"][k] * o["copies_example"][k] for k in range(len(i["values"])))),
                                   _rel(o["max_value"], knapsack_unbounded(i["values"], i["weights"], i["capacity"])["max_value"]))),
     parse=lambda q: _knap_q(q, True),
     reference=lambda values, weights, capacity: {"max_value": _milp_ref(values, [weights], [-1e30], [capacity], [capacity // max(1, int(w)) + 1 for w in weights], True)},
     wrong=lambda values, weights, capacity: knapsack01(values, weights, capacity) | {"copies_example": [0.0] * len(values)},
     random_inputs=lambda rng: (lambda n: {"values": [rng.randint(1, 60) for _ in range(n)], "weights": [rng.randint(2, 30) for _ in range(n)], "capacity": rng.randint(10, 200)})(rng.randint(2, 8)),
     keywords=["unbounded knapsack"],
     templates=["Solve the unbounded knapsack problem with values [10, 40, 50, 70], weights [1, 3, 4, 5] and capacity 8."],
     examples=[{"inputs": {"values": [10, 40, 50, 70], "weights": [1, 3, 4, 5], "capacity": 8}, "outputs": {"max_value": 110.0}}],
     questions=[{"q": "Solve the unbounded knapsack problem with values [10, 40, 50, 70], weights [1, 3, 4, 5] and capacity 8.", "outputs": {"max_value": 110.0}}])


def set_cover(sets, costs):
    m = len(sets)
    if m == 0 or m > 18 or len(costs) != m:
        raise ValueError("give between 1 and 18 sets, one cost each")
    if any(x not in (0, 1) for r in sets for x in r):
        raise ValueError("the incidence matrix must contain only 0 and 1")
    if any(c < 0 for c in costs):
        raise ValueError("costs must be >= 0")
    n = len(sets[0])
    masks = [sum(1 << j for j in range(n) if sets[i][j]) for i in range(m)]
    full = (1 << n) - 1
    C = [_fr(c) for c in costs]
    best = None
    for pick in range(1 << m):
        cov, cost = 0, F(0)
        for i in range(m):
            if pick >> i & 1:
                cov |= masks[i]
                cost += C[i]
        if cov == full and (best is None or cost < best[0]):
            best = (cost, pick)
    if best is None:
        raise ValueError("some element is in no set: no cover exists")
    return {"min_cost": float(best[0]), "sets_example": [i + 1 for i in range(m) if best[1] >> i & 1]}


_add("set_cover",
     name="Minimum-cost set cover", topic="integer programming / covering",
     purpose="Choose sets (each with a cost) that together contain every element, at least total cost.",
     inputs=[_I("sets", "incidence matrix: row i lists with 0/1 which elements set i contains", "matrix"), _I("costs", "cost of each set", "vector", min_length=1, max_length=18)],
     outputs=[_O("min_cost", "minimum total cost of a cover"), _O("sets_example", "one optimal choice of sets (1-based)")],
     primary="min_cost", conditions=["at most 18 sets", "every element in some set"],
     method="exhaustive enumeration of set combinations (exact)", latex=r"\min \sum_i c_i x_i\ \text{s.t.}\ \sum_{i \ni e} x_i \ge 1\ \forall e,\ x \in \{0,1\}",
     run=set_cover,
     verify=lambda i, o: (lambda pick: 1.0 if any(not any(i["sets"][k][j] for k in pick) for j in range(len(i["sets"][0])))
                          else max(_rel(o["min_cost"], sum(i["costs"][k] for k in pick)), _rel(o["min_cost"], set_cover(i["sets"], i["costs"])["min_cost"])))(
         [int(round(x)) - 1 for x in o["sets_example"]]),
     parse=lambda q: (_labelled(q, {"sets": r"incidence matrix|sets?|matrix", "costs": r"costs?"}) or None) if re.search(r"\bset cover(?:ing)?\b", q.lower()) else None,
     reference=lambda sets, costs: {"min_cost": _milp_ref(costs, [list(col) for col in zip(*sets)], [1] * len(sets[0]), [1e30] * len(sets[0]), [1] * len(costs))},
     wrong=lambda sets, costs: {"min_cost": float(sum(costs)), "sets_example": list(range(1, len(costs) + 1))},
     random_inputs=lambda rng: (lambda m, n: (lambda S: {"sets": S, "costs": [rng.randint(1, 20) for _ in range(m)]})(
         [[1 if rng.random() < 0.35 else 0 for _ in range(n)] for _ in range(m - 1)] + [[1] * n]))(rng.randint(3, 10), rng.randint(3, 8)),
     keywords=["set cover"],
     templates=["Solve the set cover problem with incidence matrix [[1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 1, 1], [1, 0, 0, 1]] and costs [3, 2, 3, 2]."],
     examples=[{"inputs": {"sets": [[1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 1, 1], [1, 0, 0, 1]], "costs": [3, 2, 3, 2]}, "outputs": {"min_cost": 4.0}}],
     questions=[{"q": "Solve the set cover problem with incidence matrix [[1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 1, 1], [1, 0, 0, 1]] and costs [3, 2, 3, 2].",
                 "outputs": {"min_cost": 4.0, "sets_example": [2, 4]}}])


def facility_location(fixed, cost):
    m = len(fixed)
    if m == 0 or m > 15 or len(cost) != m:
        raise ValueError("give between 1 and 15 facilities: one fixed cost and one row of service costs each")
    n = len(cost[0])
    Fx, C = [_fr(f) for f in fixed], [[_fr(x) for x in r] for r in cost]
    best = None
    for pick in range(1, 1 << m):
        op = [i for i in range(m) if pick >> i & 1]
        tot = sum(Fx[i] for i in op) + sum(min(C[i][j] for i in op) for j in range(n))
        if best is None or tot < best[0]:
            best = (tot, op)
    return {"min_cost": float(best[0]), "open_example": [i + 1 for i in best[1]]}


def _ref_ufl(fixed, cost):
    """INDEPENDENT reference: the strong MILP formulation (y_i open, x_ij serve) with scipy milp."""
    m, n = len(fixed), len(cost[0])
    nv = m + m * n
    c = list(fixed) + [cost[i][j] for i in range(m) for j in range(n)]
    rows, lo, hi = [], [], []
    for j in range(n):
        r = [0] * nv
        for i in range(m):
            r[m + i * n + j] = 1
        rows.append(r); lo.append(1); hi.append(1)
    for i in range(m):
        for j in range(n):
            r = [0] * nv
            r[m + i * n + j], r[i] = 1, -1
            rows.append(r); lo.append(-1e30); hi.append(0)
    return {"min_cost": _milp_ref(c, rows, lo, hi, [1] * nv)}


_add("facility_location",
     name="Uncapacitated facility location", topic="integer programming / location",
     purpose="Choose which facilities to open (each with a fixed cost) and serve every customer from its cheapest open "
             "facility, minimising fixed plus service cost.",
     inputs=[_I("fixed", "fixed opening cost of each facility", "vector", min_length=1, max_length=15),
             _I("cost", "service cost matrix, rows = facilities, columns = customers", "matrix")],
     outputs=[_O("min_cost", "minimum total cost"), _O("open_example", "one optimal set of facilities to open (1-based)")],
     primary="min_cost", conditions=["at most 15 facilities", "unlimited capacity"],
     method="exact enumeration of the facilities to open; each customer assigned to its cheapest open facility",
     latex=r"\min_{S \neq \emptyset} \sum_{i \in S} f_i + \sum_j \min_{i \in S} c_{ij}",
     run=facility_location,
     verify=lambda i, o: (lambda op: 1.0 if not op or any(not 0 <= k < len(i["fixed"]) for k in op)
                          else max(_rel(o["min_cost"], sum(i["fixed"][k] for k in op) + sum(min(i["cost"][k][j] for k in op) for j in range(len(i["cost"][0])))),
                                   _rel(o["min_cost"], facility_location(i["fixed"], i["cost"])["min_cost"])))([int(round(x)) - 1 for x in o["open_example"]]),
     parse=lambda q: (_labelled(q, {"fixed": r"fixed(?: opening)? costs?|opening costs?", "cost": r"service costs?|costs? matrix|transport(?:ation)? costs?|assignment costs?"}) or None)
     if re.search(r"\bfacility location|warehouse location|plant location\b", q.lower()) else None,
     reference=_ref_ufl,
     wrong=lambda fixed, cost: (lambda i: {"min_cost": float(fixed[i] + sum(cost[i])), "open_example": [i + 1]})(min(range(len(fixed)), key=lambda i: fixed[i])),
     random_inputs=lambda rng: (lambda m, n: {"fixed": [rng.randint(5, 60) for _ in range(m)], "cost": [[rng.randint(1, 30) for _ in range(n)] for _ in range(m)]})(rng.randint(2, 7), rng.randint(2, 8)),
     keywords=["facility location", "warehouse location"],
     templates=["Facility location: fixed costs [10, 15, 12] and service cost matrix [[3, 8, 6, 9], [7, 2, 4, 3], [5, 6, 2, 8]]. Which facilities should open?"],
     examples=[{"inputs": {"fixed": [10, 15, 12], "cost": [[3, 8, 6, 9], [7, 2, 4, 3], [5, 6, 2, 8]]}, "outputs": {"min_cost": 31.0}}],
     questions=[{"q": "Facility location: fixed costs [10, 15, 12] and service cost matrix [[3, 8, 6, 9], [7, 2, 4, 3], [5, 6, 2, 8]]. Which facilities should open?",
                 "outputs": {"min_cost": 31.0}}])


def bin_packing(sizes, capacity):
    _finite(capacity)
    n = len(sizes)
    if n == 0 or n > 14:
        raise ValueError("give between 1 and 14 items")
    if capacity <= 0 or any(s <= 0 or s > capacity for s in sizes):
        raise ValueError("every item size must be > 0 and fit in a bin")
    S, cap = [_fr(s) for s in sizes], _fr(capacity)
    order = sorted(range(n), key=lambda j: (-S[j], j))
    best = [n + 1, None]
    lb = math.ceil(sum(S) / cap)
    loads, a = [], [0] * n

    def dfs(t):
        if best[0] == lb or len(loads) >= best[0]:
            return
        if t == n:
            best[0], best[1] = len(loads), list(a)
            return
        j = order[t]
        seen = set()
        for k in range(len(loads)):
            if loads[k] + S[j] <= cap and loads[k] not in seen:
                seen.add(loads[k])
                loads[k] += S[j]
                a[j] = k
                dfs(t + 1)
                loads[k] -= S[j]
        loads.append(S[j])
        a[j] = len(loads) - 1
        dfs(t + 1)
        loads.pop()
    dfs(0)
    return {"min_bins": float(best[0]), "bin_of_item": [k + 1 for k in best[1]], "lower_bound": float(lb)}


def _next_fit(sizes, capacity):
    b, load, where = 1, 0, []
    for x in sizes:
        if load + x > capacity:
            b, load = b + 1, 0
        load += x
        where.append(b)
    return {"min_bins": float(b), "bin_of_item": where}


def _verify_bins(i, o):
    ks = [int(round(x)) - 1 for x in o["bin_of_item"]]
    loads = {}
    for j, k in enumerate(ks):
        loads[k] = loads.get(k, 0) + i["sizes"][j]
    if any(v > i["capacity"] + 1e-9 for v in loads.values()):
        return 1.0
    return max(_rel(o["min_bins"], len(loads)), _rel(o["min_bins"], bin_packing(i["sizes"], i["capacity"])["min_bins"]))


def _ref_bins(sizes, capacity):
    """INDEPENDENT reference: assignment MILP (x_jk item j in bin k, y_k bin used), scipy milp."""
    n = len(sizes)
    nv = n * n + n
    c = [0] * (n * n) + [1] * n
    rows, lo, hi = [], [], []
    for j in range(n):
        r = [0] * nv
        for k in range(n):
            r[j * n + k] = 1
        rows.append(r); lo.append(1); hi.append(1)
    for k in range(n):
        r = [0] * nv
        for j in range(n):
            r[j * n + k] = sizes[j]
        r[n * n + k] = -capacity
        rows.append(r); lo.append(-1e30); hi.append(0)
    return {"min_bins": _milp_ref(c, rows, lo, hi, [1] * nv)}


_add("bin_packing",
     name="Bin packing (minimum number of bins)", topic="integer programming / packing",
     purpose="Pack items of given sizes into the fewest bins of a given capacity: the minimum number and one packing.",
     inputs=[_I("sizes", "size of each item", "vector", min_length=1, max_length=14), _I("capacity", "capacity of each bin")],
     outputs=[_O("min_bins", "minimum number of bins"), _O("bin_of_item", "one optimal packing: bin number of each item"), _O("lower_bound", "ceil(total size / capacity)")],
     primary="min_bins", conditions=["at most 14 items", "every item fits in a bin"],
     method="exact depth-first search (largest item first, symmetric bins skipped) with the volume bound",
     latex=r"\min \#\text{bins} \ \ge\ \left\lceil \sum_j s_j / C \right\rceil",
     run=bin_packing, verify=_verify_bins, reference=_ref_bins,
     parse=lambda q: (lambda ql: None if not re.search(r"\bbin[- ]packing|fewest bins|minimum number of bins\b", ql) else (
         (_labelled(q, {"sizes": r"sizes?|items?|weights?|lengths?"}) or {}) | ({"capacity": v} if (v := _num_after(ql, r"capacity(?: of| is| =|:)?")) is not None else {})))(q.lower()),
     wrong=lambda sizes, capacity: _next_fit(sizes, capacity),        # a heuristic count taken for the optimum
     random_inputs=lambda rng: (lambda c: {"sizes": [rng.randint(max(1, int(c * 0.15)), int(c * 0.75)) for _ in range(rng.randint(7, 13))], "capacity": c})(rng.randint(10, 20)),
     keywords=["bin packing"],
     templates=["Bin packing: item sizes [4, 8, 1, 4, 2, 1] and bin capacity 10. Find the minimum number of bins."],
     examples=[{"inputs": {"sizes": [4, 8, 1, 4, 2, 1], "capacity": 10}, "outputs": {"min_bins": 2.0}},
               {"inputs": {"sizes": [6, 6, 6], "capacity": 10}, "outputs": {"min_bins": 3.0}}],
     questions=[{"q": "Bin packing: item sizes [4, 8, 1, 4, 2, 1] and bin capacity 10. Find the minimum number of bins.", "outputs": {"min_bins": 2.0}}])


# =================================================================================================================
# game theory
# =================================================================================================================
def _payoff(Am, name="payoff matrix", max_dim=12):
    if not Am or not Am[0] or any(len(r) != len(Am[0]) for r in Am):
        raise ValueError(f"the {name} must be a rectangular matrix")
    if len(Am) > max_dim or len(Am[0]) > max_dim:
        raise ValueError(f"at most {max_dim} strategies per player")
    return [[_fr(v) for v in r] for r in Am]


def _game_value(M):
    """Value and one optimal mixed strategy for each player of the zero-sum game M (row player maximises), from two
    exact LPs on the shifted game (all entries >= 1)."""
    m, n = len(M), len(M[0])
    k = 1 - min(min(r) for r in M)
    S = [[v + k for v in r] for r in M]
    ru = _lp("minimize", [1] * m, [[S[i][j] for i in range(m)] for j in range(n)], [1] * n, [">="] * n, certify=False)
    cw = _lp("maximize", [1] * n, [list(S[i]) for i in range(m)], [1] * m, ["<="] * m, certify=False)
    vs = 1 / ru["value"]
    return vs - k, [u * vs for u in ru["x"]], [w * vs for w in cw["x"]]


def zero_sum_game(A):
    M = _payoff(A)
    v, x, y = _game_value(M)
    return {"value": float(v), "row_strategy_example": _fl(x), "col_strategy_example": _fl(y)}


def _verify_game(i, o):
    M = [[float(v) for v in r] for r in i["A"]]
    m, n = len(M), len(M[0])
    x, y, v = o["row_strategy_example"], o["col_strategy_example"], o["value"]
    if len(x) != m or len(y) != n:
        return 1.0
    sc = max([1.0] + [abs(a) for r in M for a in r])
    bad = max([abs(sum(x) - 1), abs(sum(y) - 1)] + [max(0.0, -t) for t in x + y])
    lo = min(sum(x[r] * M[r][j] for r in range(m)) for j in range(n))      # x guarantees at least lo
    hi = max(sum(M[r][j] * y[j] for j in range(n)) for r in range(m))      # y concedes at most hi
    return max(bad, max(0.0, v - lo) / sc, max(0.0, hi - v) / sc)


def _ref_game(A):
    """INDEPENDENT reference: scipy linprog (HiGHS) on the unshifted game LP: max v s.t. v <= sum_i a_ij x_i for every
    column, sum x = 1, x >= 0, v free."""
    import numpy as np
    from scipy.optimize import linprog
    M = np.array(A, float)
    m, n = M.shape
    c = np.zeros(m + 1)
    c[-1] = -1.0
    A_ub = np.hstack([-M.T, np.ones((n, 1))])
    res = linprog(c, A_ub=A_ub, b_ub=np.zeros(n), A_eq=np.array([[1.0] * m + [0.0]]), b_eq=[1.0],
                  bounds=[(0, None)] * m + [(None, None)], method="highs")
    return {"value": float(-res.fun)}


def _game_matrix_q(q, need):
    ql = q.lower()
    if not re.search(r"\bzero[- ]sum (?:matrix )?game\b", ql) or re.search(r"\b(bimatrix|nash)\b", ql):
        return None
    g = _labelled(q, {"A": r"payoff matrix|matrix|payoffs?"})
    if "A" in g and isinstance(g["A"], list) and g["A"] and isinstance(g["A"][0], list):
        two = len(g["A"]) == 2 and all(len(r) == 2 for r in g["A"])
        if two != need:
            return None
    return g or None


_add("zero_sum_game",
     name="Two-person zero-sum game (value and optimal mixed strategies)", topic="game theory / zero-sum games",
     purpose="The value of a matrix game (row player maximises the payoff in the matrix, column player minimises it) and "
             "one optimal mixed strategy for each player.",
     inputs=[_I("A", "payoff matrix to the row player, rows = row strategies, columns = column strategies", "matrix")],
     outputs=[_O("value", "value of the game"), _O("row_strategy_example", "one optimal mixed strategy of the row player"),
              _O("col_strategy_example", "one optimal mixed strategy of the column player")],
     primary="value", conditions=["at most 12 strategies per player"],
     method="the game as a pair of dual linear programs on the shifted matrix, solved exactly (rational simplex); the pair "
            "of strategies is the certificate (each guarantees the value)",
     latex=r"v = \max_{x \in \Delta_m} \min_{j} \sum_i x_i a_{ij} = \min_{y \in \Delta_n} \max_i \sum_j a_{ij} y_j",
     run=zero_sum_game, verify=_verify_game, reference=_ref_game,
     parse=lambda q: _game_matrix_q(q, False),
     wrong=lambda A: (lambda v: {"value": float(v), "row_strategy_example": [1.0] + [0.0] * (len(A) - 1), "col_strategy_example": [1.0] + [0.0] * (len(A[0]) - 1)})(
         max(min(r) for r in A)),                                        # the pure maximin taken for the value
     random_inputs=lambda rng: (lambda m, n: {"A": [[rng.randint(-9, 9) for _ in range(n)] for _ in range(m)]})(rng.randint(2, 5), rng.randint(3, 5)),
     keywords=["zero-sum game", "matrix game", "minimax", "mixed strategy"],
     templates=["Solve the zero-sum game with payoff matrix [[3, -1, 2], [-2, 4, 1], [0, 1, -1]]."],
     examples=[{"inputs": {"A": [[3, -1, 2], [-2, 4, 1], [0, 1, -1]]}, "outputs": {"value": 1.0}},
               {"inputs": {"A": [[1, -1], [-1, 1]]}, "outputs": {"value": 0.0, "row_strategy_example": [0.5, 0.5]}}],
     questions=[{"q": "Solve the zero-sum game with payoff matrix [[3, -1, 2], [-2, 4, 1], [0, 1, -1]].", "outputs": {"value": 1.0}},
                {"q": "Solve the zero-sum game with payoff matrix [[3, -1, 2], [-2, 4]].", "outcome": "INVALID_INPUT"}])


def zero_sum_2x2(A):
    M = _payoff(A)
    if len(M) != 2 or len(M[0]) != 2:
        raise ValueError("give a 2 x 2 payoff matrix (use the general zero-sum game capability otherwise)")
    (a, b), (c, d) = M
    lo, hi = max(min(a, b), min(c, d)), min(max(a, c), max(b, d))
    if lo == hi:
        return {"value": float(lo), "saddle_point": "yes"}
    D = a + d - b - c
    return {"value": float((a * d - b * c) / D), "p_row1": float((d - c) / D), "q_col1": float((d - b) / D), "saddle_point": "no"}


def _verify_2x2(i, o):
    (a, b), (c, d) = [[float(v) for v in r] for r in i["A"]]
    v = o["value"]
    lo, hi = max(min(a, b), min(c, d)), min(max(a, c), max(b, d))
    if o.get("saddle_point") == "yes":
        return max(_rel(v, lo), _rel(v, hi)) if lo == hi else 1.0
    if lo == hi or "p_row1" not in o:
        return 1.0
    p, q = o["p_row1"], o["q_col1"]
    if not (0 < p < 1 and 0 < q < 1):
        return 1.0
    return max(_rel(p * a + (1 - p) * c, v), _rel(p * b + (1 - p) * d, v), _rel(q * a + (1 - q) * b, v), _rel(q * c + (1 - q) * d, v))


_add("zero_sum_2x2",
     name="2 x 2 zero-sum game (saddle point or mixed-strategy formula)", topic="game theory / zero-sum games",
     purpose="The value of a 2 x 2 matrix game: the saddle-point value when one exists, otherwise the mixed-strategy "
             "solution (probability of each player's first strategy, unique in that case).",
     inputs=[_I("A", "2 x 2 payoff matrix to the row player [[a, b], [c, d]]", "matrix")],
     outputs=[_O("value", "value of the game"), _O("saddle_point", "yes when a pure saddle point exists", "text"),
              _O("p_row1", "probability the row player plays row 1 (only without a saddle point)"),
              _O("q_col1", "probability the column player plays column 1 (only without a saddle point)")],
     primary="value", conditions=["a 2 x 2 matrix"],
     method="maximin = minimax gives a saddle point; otherwise the indifference formulas in exact arithmetic",
     latex=r"v = \frac{ad - bc}{a + d - b - c},\quad p = \frac{d - c}{a + d - b - c},\quad q = \frac{d - b}{a + d - b - c}",
     run=zero_sum_2x2, verify=_verify_2x2, reference=_ref_game,
     parse=lambda q: _game_matrix_q(q, True),
     wrong=lambda A: (lambda a, b, c, d: {"value": float((a * d - b * c) / (a + b + c + d)) if a + b + c + d else 1e3 + abs(a),
                                          "p_row1": 0.5, "q_col1": 0.5, "saddle_point": "no"})(A[0][0], A[0][1], A[1][0], A[1][1]),
     random_inputs=lambda rng: {"A": [[rng.randint(-9, 9) for _ in range(2)] for _ in range(2)]},
     keywords=["2x2 game", "saddle point", "mixed strategy"],
     templates=["Solve the zero-sum game with payoff matrix [[3, -1], [-2, 4]]."],
     examples=[{"inputs": {"A": [[3, -1], [-2, 4]]}, "outputs": {"value": 1.0, "p_row1": 0.6, "q_col1": 0.5, "saddle_point": "no"}},
               {"inputs": {"A": [[4, 2], [1, 3]]}, "outputs": {"value": 2.5, "p_row1": 0.5, "q_col1": 0.25}},
               {"inputs": {"A": [[5, 3], [4, 1]]}, "outputs": {"value": 3.0, "saddle_point": "yes"}}],
     questions=[{"q": "Solve the zero-sum game with payoff matrix [[3, -1], [-2, 4]].", "outputs": {"value": 1.0, "p_row1": 0.6}},
                {"q": "Solve the zero-sum game with payoff matrix [[5, 3], [4, 1]].", "outputs": {"value": 3.0, "saddle_point": "yes"}}])


def _bimatrix(A, B, max_dim=8):
    MA, MB = _payoff(A, "row payoff matrix", max_dim), _payoff(B, "column payoff matrix", max_dim)
    if len(MA) != len(MB) or len(MA[0]) != len(MB[0]):
        raise ValueError("the row and column payoff matrices must have the same shape")
    return MA, MB


def _pure_ne(MA, MB):
    m, n = len(MA), len(MA[0])
    return [[r + 1, s + 1] for r in range(m) for s in range(n)
            if MA[r][s] == max(MA[k][s] for k in range(m)) and MB[r][s] == max(MB[r][k] for k in range(n))]


def pure_nash(A, B):
    MA, MB = _bimatrix(A, B)
    eq = _pure_ne(MA, MB)
    return {"count": float(len(eq)), "equilibria": [[float(a), float(b)] for a, b in eq]}


def _verify_pure_ne(i, o):
    try:
        eq = _pure_ne(*_bimatrix(i["A"], i["B"]))
    except ValueError:
        return 1.0
    same = [[int(round(a)), int(round(b))] for a, b in o["equilibria"]] == eq
    return 0.0 if same and o["count"] == len(eq) else max(1.0, abs(o["count"] - len(eq)))


def _ref_pure_ne(A, B):
    """INDEPENDENT reference: nashpy support enumeration; the equilibria whose supports are single strategies."""
    import nashpy as nash
    import numpy as np
    G = nash.Game(np.array(A, float), np.array(B, float))
    found = set()
    for s1, s2 in G.support_enumeration(non_degenerate=False):
        if np.isclose(s1.max(), 1.0) and np.isclose(s2.max(), 1.0):
            found.add((int(np.argmax(s1)) + 1, int(np.argmax(s2)) + 1))
    eq = sorted(found)
    return {"count": float(len(eq)), "equilibria": [[float(a), float(b)] for a, b in eq]}


def _random_bimatrix(rng):
    m, n = rng.randint(2, 4), rng.randint(2, 4)
    va, vb = rng.sample(range(-30, 30), m * n), rng.sample(range(-30, 30), m * n)   # distinct payoffs: non-degenerate
    return {"A": [va[r * n:(r + 1) * n] for r in range(m)], "B": [vb[r * n:(r + 1) * n] for r in range(m)]}


_BIMAT_LABELS = {"A": r"row (?:player'?s? )?payoffs?(?: matrix)?|row player", "B": r"column (?:player'?s? )?payoffs?(?: matrix)?|column player"}

_add("pure_nash_equilibria",
     name="Pure-strategy Nash equilibria of a bimatrix game", topic="game theory / non-zero-sum games",
     purpose="Every pure-strategy Nash equilibrium (a cell where each player's strategy is a best reply to the other's) of "
             "a two-player game given by the two players' payoff matrices.",
     inputs=[_I("A", "row player's payoffs (rows = row strategies)", "matrix"), _I("B", "column player's payoffs (same shape)", "matrix")],
     outputs=[_O("count", "number of pure equilibria"), _O("equilibria", "all pure equilibria as [row, column], 1-based, in row-major order")],
     primary="count", conditions=["at most 8 strategies per player", "weak best replies count (ties are equilibria)"],
     method="mutual best replies, cell by cell, in exact arithmetic",
     latex=r"a_{rs} = \max_k a_{ks},\quad b_{rs} = \max_k b_{rk}",
     run=pure_nash, verify=_verify_pure_ne, reference=_ref_pure_ne,
     parse=lambda q: (_labelled(q, _BIMAT_LABELS) or None) if re.search(r"\bpure[- ]strategy nash|pure nash\b", q.lower()) else None,
     wrong=lambda A, B: (lambda MA: {"count": float(sum(1 for r in range(len(MA)) for s in range(len(MA[0])) if MA[r][s] == max(MA[k][s] for k in range(len(MA))))),
                                     "equilibria": []})([[_fr(v) for v in r] for r in A]),     # the row player's best replies only
     random_inputs=_random_bimatrix,
     keywords=["nash equilibrium", "bimatrix game", "pure strategy"],
     templates=["Find the pure-strategy Nash equilibria of the bimatrix game with row payoffs [[3, 0], [5, 1]] and column payoffs [[3, 5], [0, 1]]."],
     examples=[{"inputs": {"A": [[3, 0], [5, 1]], "B": [[3, 5], [0, 1]]}, "outputs": {"count": 1.0, "equilibria": [[2.0, 2.0]]}},
               {"inputs": {"A": [[2, 0], [0, 1]], "B": [[1, 0], [0, 2]]}, "outputs": {"count": 2.0, "equilibria": [[1.0, 1.0], [2.0, 2.0]]}}],
     questions=[{"q": "Find the pure-strategy Nash equilibria of the bimatrix game with row payoffs [[3, 0], [5, 1]] and column payoffs [[3, 5], [0, 1]].",
                 "outputs": {"count": 1.0}},
                {"q": "Find the pure-strategy Nash equilibria of the bimatrix game with row payoffs [[3, 0], [5, 1]] and column payoffs [[3, 5, 1], [0, 1, 2]].",
                 "outcome": "INVALID_INPUT"}])


def mixed_nash_2x2(A, B):
    MA, MB = _bimatrix(A, B)
    if len(MA) != 2 or len(MA[0]) != 2:
        raise ValueError("give 2 x 2 payoff matrices")
    if _pure_ne(MA, MB):
        raise ValueError("the game has a pure-strategy equilibrium: its mixed equilibrium is not the only one (ask for the pure equilibria)")
    (a11, a12), (a21, a22) = MA
    (b11, b12), (b21, b22) = MB
    DA, DB = a11 - a12 - a21 + a22, b11 - b12 - b21 + b22
    if DA == 0 or DB == 0:
        raise ValueError("degenerate game: no unique mixed equilibrium")
    q, p = (a22 - a12) / DA, (b22 - b21) / DB
    if not (0 < p < 1 and 0 < q < 1):
        raise ValueError("no completely mixed equilibrium")
    return {"p_row1": float(p), "q_col1": float(q), "row_payoff": float(q * a11 + (1 - q) * a12), "col_payoff": float(p * b11 + (1 - p) * b21)}


def _verify_mixed(i, o):
    (a11, a12), (a21, a22) = [[float(v) for v in r] for r in i["A"]]
    (b11, b12), (b21, b22) = [[float(v) for v in r] for r in i["B"]]
    p, q = o["p_row1"], o["q_col1"]
    if not (0 < p < 1 and 0 < q < 1):
        return 1.0
    u, w = o["row_payoff"], o["col_payoff"]
    return max(_rel(q * a11 + (1 - q) * a12, u), _rel(q * a21 + (1 - q) * a22, u),
               _rel(p * b11 + (1 - p) * b21, w), _rel(p * b12 + (1 - p) * b22, w))


def _ref_mixed(A, B):
    """INDEPENDENT reference: nashpy support enumeration (the game has exactly one equilibrium, fully mixed)."""
    import nashpy as nash
    import numpy as np
    MA, MB = np.array(A, float), np.array(B, float)
    eqs = list(nash.Game(MA, MB).support_enumeration())
    s1, s2 = eqs[0]
    return {"p_row1": float(s1[0]), "q_col1": float(s2[0]), "row_payoff": float(s1 @ MA @ s2), "col_payoff": float(s1 @ MB @ s2)}


def _random_mixed(rng):
    a = rng.sample(range(-20, 21), 4)
    b = rng.sample(range(-20, 21), 4)
    a.sort()
    b.sort()
    # row player wants to match (a11 > a21, a22 > a12), column player wants to mismatch (b12 > b11, b21 > b22)
    A = [[a[3], a[0]], [a[1], a[2]]]
    B = [[b[0], b[3]], [b[2], b[1]]]
    return {"A": A, "B": B}


_add("mixed_nash_2x2",
     name="Mixed-strategy Nash equilibrium of a 2 x 2 game", topic="game theory / non-zero-sum games",
     purpose="The equilibrium of a 2 x 2 two-player game without a pure equilibrium: each player's probability of the "
             "first strategy (chosen to make the OTHER player indifferent) and the expected payoffs.",
     inputs=[_I("A", "row player's 2 x 2 payoffs", "matrix"), _I("B", "column player's 2 x 2 payoffs", "matrix")],
     outputs=[_O("p_row1", "probability the row player plays row 1"), _O("q_col1", "probability the column player plays column 1"),
              _O("row_payoff", "row player's expected payoff"), _O("col_payoff", "column player's expected payoff")],
     primary="p_row1", conditions=["2 x 2 matrices", "no pure-strategy equilibrium (then the mixed one is unique)"],
     method="indifference conditions in exact arithmetic: p from the column player's payoffs, q from the row player's",
     latex=r"q = \frac{a_{22} - a_{12}}{a_{11} - a_{12} - a_{21} + a_{22}},\quad p = \frac{b_{22} - b_{21}}{b_{11} - b_{12} - b_{21} + b_{22}}",
     run=mixed_nash_2x2, verify=_verify_mixed, reference=_ref_mixed,
     parse=lambda q: (_labelled(q, _BIMAT_LABELS) or None) if re.search(r"\bmixed[- ]strategy nash|mixed nash\b", q.lower()) else None,
     wrong=lambda A, B: (lambda r: r | {"p_row1": float((_fr(A[1][1]) - _fr(A[1][0])) / (_fr(A[0][0]) - _fr(A[0][1]) - _fr(A[1][0]) + _fr(A[1][1])))})(mixed_nash_2x2(A, B)),
     random_inputs=_random_mixed,
     keywords=["mixed strategy", "nash equilibrium", "2x2 game"],
     templates=["Find the mixed-strategy Nash equilibrium of the game with row payoffs [[3, -1], [-2, 1]] and column payoffs [[-1, 2], [1, -1]]."],
     examples=[{"inputs": {"A": [[3, -1], [-2, 1]], "B": [[-1, 2], [1, -1]]},
                "outputs": {"p_row1": 0.4, "q_col1": 2 / 7, "row_payoff": 1 / 7, "col_payoff": 0.2}}],
     questions=[{"q": "Find the mixed-strategy Nash equilibrium of the game with row payoffs [[3, -1], [-2, 1]] and column payoffs [[-1, 2], [1, -1]].",
                 "outputs": {"p_row1": 0.4}},
                {"q": "Find the mixed-strategy Nash equilibrium of the game with row payoffs [[3, 0], [5, 1]] and column payoffs [[3, 5], [0, 1]].",
                 "outcome": "INVALID_INPUT"}])


# =================================================================================================================
# multi-criteria decision analysis
# =================================================================================================================
def _mcda_in(X, w, types):
    if not X or any(len(r) != len(X[0]) for r in X):
        raise ValueError("the decision matrix must be rectangular (rows = alternatives, columns = criteria)")
    m, k = len(X), len(X[0])
    if m < 2 or m > 40 or k > 15:
        raise ValueError("give 2 to 40 alternatives and at most 15 criteria")
    if len(w) != k or len(types) != k:
        raise ValueError(f"give one weight and one type per criterion ({k})")
    if any(v <= 0 for r in X for v in r):
        raise ValueError("every value of the decision matrix must be > 0")
    if any(v <= 0 for v in w):
        raise ValueError("every weight must be > 0")
    if any(t not in (1, -1) for t in types):
        raise ValueError("criterion types are 1 (benefit: larger is better) or -1 (cost: smaller is better)")
    W = [_fr(v) for v in w]
    s = sum(W)
    return [[float(v) for v in r] for r in X], [float(v / s) for v in W], [int(t) for t in types]


def _best(scores):
    top = max(scores)
    return next(k for k, v in enumerate(scores) if v >= top - 1e-12 * max(1.0, abs(top))) + 1


def _mcda_out(scores):
    return {"scores": scores, "best_score": max(scores), "best": float(_best(scores))}


def topsis(X, w, types):
    X, w, types = _mcda_in(X, w, types)
    m, k = len(X), len(X[0])
    norms = [math.sqrt(math.fsum(X[i][j] ** 2 for i in range(m))) for j in range(k)]
    V = [[w[j] * X[i][j] / norms[j] for j in range(k)] for i in range(m)]
    best = [(max if types[j] == 1 else min)(V[i][j] for i in range(m)) for j in range(k)]
    worst = [(min if types[j] == 1 else max)(V[i][j] for i in range(m)) for j in range(k)]
    sc = []
    for i in range(m):
        dp = math.sqrt(math.fsum((V[i][j] - best[j]) ** 2 for j in range(k)))
        dm = math.sqrt(math.fsum((V[i][j] - worst[j]) ** 2 for j in range(k)))
        if dp + dm == 0:
            raise ValueError("every alternative is identical: no ranking")
        sc.append(dm / (dp + dm))
    return _mcda_out(sc)


def _verify_mcda(fn):
    def v(i, o):
        try:
            ref = fn(i["X"], i["w"], i["types"])["scores"]
        except ValueError:
            return 1.0
        if len(o["scores"]) != len(ref):
            return 1.0
        return max([_rr(a, b) for a, b in zip(o["scores"], ref)] + [_rr(o["best_score"], max(o["scores"])),
                   0.0 if int(o["best"]) == _best(o["scores"]) else 1.0])
    return v


def _topsis_check(X, w, types):
    """The verifier's own arrangement: cost columns flipped to 1 - r (as distances, identical to the textbook ideal)."""
    X, w, types = _mcda_in(X, w, types)
    m, k = len(X), len(X[0])
    R = [[(X[i][j] / math.sqrt(math.fsum(X[t][j] ** 2 for t in range(m)))) for j in range(k)] for i in range(m)]
    V = [[w[j] * (R[i][j] if types[j] == 1 else 1 - R[i][j]) for j in range(k)] for i in range(m)]
    hi = [max(V[i][j] for i in range(m)) for j in range(k)]
    lo = [min(V[i][j] for i in range(m)) for j in range(k)]
    sc = [math.dist(V[i], lo) / (math.dist(V[i], lo) + math.dist(V[i], hi)) for i in range(m)]
    return {"scores": sc}


def _ref_pymcdm(method, norm):
    def ref(X, w, types):
        import numpy as np
        from pymcdm import methods, normalizations
        W = np.array(w, float)
        sc = getattr(methods, method)(getattr(normalizations, norm))(np.array(X, float), W / W.sum(), np.array(types, int))
        return {"scores": [float(v) for v in sc], "best_score": float(max(sc))}
    return ref


def _random_mcda(rng):
    m, k = rng.randint(3, 8), rng.randint(2, 6)
    return {"X": [[rng.randint(1, 100) for _ in range(k)] for _ in range(m)], "w": [rng.randint(1, 10) for _ in range(k)],
            "types": [rng.choice([1, -1]) for _ in range(k)]}


def _mcda_q(rx):
    labels = {"X": r"decision matrix|performance matrix|matrix", "w": r"weights?", "types": r"criteri(?:on|a) types?|types?|directions?"}
    return lambda q: (_labelled(q, labels) or None) if re.search(rx, q.lower()) else None


_MCDA_IN = [_I("X", "decision matrix, rows = alternatives, columns = criteria (all values > 0)", "matrix"),
            _I("w", "criterion weights (> 0; normalised to sum 1)", "vector", min_length=1, max_length=15),
            _I("types", "criterion types: 1 = benefit (larger is better), -1 = cost (smaller is better)", "vector", min_length=1, max_length=15)]
_MCDA_OUT = [_O("scores", "score of each alternative"), _O("best_score", "the highest score"),
             _O("best", "best alternative, 1-based (on a tie the first listed)")]

_add("topsis",
     name="TOPSIS ranking (closeness to the ideal solution)", topic="multi-criteria decision analysis",
     purpose="Rank alternatives scored on several criteria by their relative closeness to the ideal and distance from the "
             "anti-ideal solution (vector normalisation, weighted Euclidean distances).",
     inputs=_MCDA_IN, outputs=_MCDA_OUT, primary="best_score",
     conditions=["values > 0", "weights > 0 (normalised to sum 1)", "types 1 or -1"],
     method="vector normalisation r = x / ||x_col||, weighted; ideal = best per criterion, anti-ideal = worst; C = D- / (D+ + D-)",
     latex=r"C_i = \frac{D_i^-}{D_i^+ + D_i^-},\quad D_i^\pm = \Big\| w \odot r_i - v^\pm \Big\|_2",
     run=topsis, verify=_verify_mcda(_topsis_check), reference=_ref_pymcdm("TOPSIS", "vector_normalization"),
     parse=_mcda_q(r"\btopsis\b"),
     wrong=lambda X, w, types: (lambda Xf, wf, t: (lambda V: (lambda b, a: _mcda_out([math.dist(r, a) / (math.dist(r, a) + math.dist(r, b)) if math.dist(r, a) + math.dist(r, b) else 0.5 for r in V]))(
         [(max if t[j] == 1 else min)(r[j] for r in V) for j in range(len(wf))], [(min if t[j] == 1 else max)(r[j] for r in V) for j in range(len(wf))]))(
         [[wf[j] * r[j] for j in range(len(wf))] for r in Xf]))(*_mcda_in(X, w, types)),          # no normalisation
     random_inputs=_random_mcda,
     keywords=["topsis", "multi-criteria", "ideal solution"],
     templates=["Rank with TOPSIS the alternatives with decision matrix [[250, 16, 12, 5], [200, 16, 8, 3], [300, 32, 16, 4], [275, 32, 8, 4], [225, 16, 16, 2]], weights [0.25, 0.25, 0.25, 0.25] and criteria types [-1, 1, 1, 1]."],
     examples=[{"inputs": {"X": [[250, 16, 12, 5], [200, 16, 8, 3], [300, 32, 16, 4], [275, 32, 8, 4], [225, 16, 16, 2]], "w": [0.25, 0.25, 0.25, 0.25], "types": [-1, 1, 1, 1]},
                "outputs": {"best": 3.0}},
               {"inputs": {"X": [[1, 1], [2, 2]], "w": [1, 1], "types": [1, 1]}, "outputs": {"scores": [0.0, 1.0], "best": 2.0, "best_score": 1.0}}],
     questions=[{"q": "Rank with TOPSIS the alternatives with decision matrix [[1, 1], [2, 2]], weights [1, 1] and criteria types [1, 1].",
                 "outputs": {"best": 2.0, "best_score": 1.0}},
                {"q": "Rank with TOPSIS the alternatives with decision matrix [[1, 1], [2, 2]], weights [1, 1] and criteria types [1, 2].", "outcome": "INVALID_INPUT"}])


def _linear_norm(X, types):
    m, k = len(X), len(X[0])
    cols = [[X[i][j] for i in range(m)] for j in range(k)]
    return [[(X[i][j] / max(cols[j])) if types[j] == 1 else (min(cols[j]) / X[i][j]) for j in range(k)] for i in range(m)]


def weighted_sum(X, w, types):
    X, w, types = _mcda_in(X, w, types)
    N = _linear_norm(X, types)
    return _mcda_out([math.fsum(wj * v for wj, v in zip(w, r)) for r in N])


def weighted_product(X, w, types):
    X, w, types = _mcda_in(X, w, types)
    N = _linear_norm(X, types)
    return _mcda_out([math.exp(math.fsum(wj * math.log(v) for wj, v in zip(w, r))) for r in N])


def _wrong_norm(fn):                                       # cost criteria treated as benefits
    return lambda X, w, types: fn(X, w, [1] * len(types))


_add("weighted_sum_model",
     name="Weighted sum model (simple additive weighting)", topic="multi-criteria decision analysis",
     purpose="Score alternatives by the weighted sum of linearly normalised criterion values (benefit x / max, cost "
             "min / x) and pick the best.",
     inputs=_MCDA_IN, outputs=_MCDA_OUT, primary="best_score",
     conditions=["values > 0", "weights > 0 (normalised to sum 1)", "types 1 or -1"],
     method="linear normalisation (x / max for benefits, min / x for costs), then S_i = sum_j w_j n_ij",
     latex=r"S_i = \sum_j w_j n_{ij},\quad n_{ij} = \frac{x_{ij}}{\max_i x_{ij}}\ \text{or}\ \frac{\min_i x_{ij}}{x_{ij}}",
     run=weighted_sum, verify=_verify_mcda(lambda X, w, t: {"scores": [math.fsum(a * b for a, b in zip(_mcda_in(X, w, t)[1], r)) for r in _linear_norm(*_mcda_in(X, w, t)[0:3:2])]}),
     reference=_ref_pymcdm("WSM", "linear_normalization"),
     parse=_mcda_q(r"\b(weighted sum model|simple additive weighting|saw method)\b"),
     wrong=_wrong_norm(weighted_sum), random_inputs=_random_mcda,
     keywords=["weighted sum model", "simple additive weighting", "SAW"],
     templates=["Score with the weighted sum model the alternatives with decision matrix [[250, 16, 12], [200, 16, 8], [300, 32, 16]], weights [0.4, 0.3, 0.3] and criteria types [-1, 1, 1]."],
     examples=[{"inputs": {"X": [[250, 16, 12], [200, 16, 8], [300, 32, 16]], "w": [0.4, 0.3, 0.3], "types": [-1, 1, 1]},
                "outputs": {"scores": [0.4 * 0.8 + 0.3 * 0.5 + 0.3 * 0.75, 0.4 + 0.15 + 0.15, 0.4 * 2 / 3 + 0.3 + 0.3], "best": 3.0}}],
     questions=[{"q": "Score with the weighted sum model the alternatives with decision matrix [[250, 16, 12], [200, 16, 8], [300, 32, 16]], weights [0.4, 0.3, 0.3] and criteria types [-1, 1, 1].",
                 "outputs": {"best": 3.0}}])

_add("weighted_product_model",
     name="Weighted product model", topic="multi-criteria decision analysis",
     purpose="Score alternatives by the product of linearly normalised criterion values raised to the criterion weights.",
     inputs=_MCDA_IN, outputs=_MCDA_OUT, primary="best_score",
     conditions=["values > 0", "weights > 0 (normalised to sum 1)", "types 1 or -1"],
     method="linear normalisation (x / max for benefits, min / x for costs), then P_i = prod_j n_ij^{w_j}",
     latex=r"P_i = \prod_j n_{ij}^{\,w_j}",
     run=weighted_product, verify=_verify_mcda(lambda X, w, t: {"scores": [math.prod(b ** a for a, b in zip(_mcda_in(X, w, t)[1], r)) for r in _linear_norm(*_mcda_in(X, w, t)[0:3:2])]}),
     reference=_ref_pymcdm("WPM", "linear_normalization"),
     parse=_mcda_q(r"\b(weighted product model|wpm method)\b"),
     wrong=_wrong_norm(weighted_product), random_inputs=_random_mcda,
     keywords=["weighted product model", "WPM"],
     templates=["Score with the weighted product model the alternatives with decision matrix [[250, 16, 12], [200, 16, 8], [300, 32, 16]], weights [0.4, 0.3, 0.3] and criteria types [-1, 1, 1]."],
     examples=[{"inputs": {"X": [[4, 2], [2, 4]], "w": [0.5, 0.5], "types": [1, 1]}, "outputs": {"scores": [math.sqrt(0.5), math.sqrt(0.5)], "best": 1.0}}],
     questions=[{"q": "Score with the weighted product model the alternatives with decision matrix [[4, 2], [2, 4]], weights [0.5, 0.5] and criteria types [1, 1].",
                 "outputs": {"best": 1.0, "best_score": math.sqrt(0.5)}}])


# --- analytic hierarchy process -------------------------------------------------------------------------------------
_RI = {3: 0.58, 4: 0.90, 5: 1.12, 6: 1.24, 7: 1.32, 8: 1.41, 9: 1.45, 10: 1.49}     # Saaty's random indices


def _ahp_matrix(judgments):
    L = len(judgments)
    n = next((k for k in range(2, 11) if k * (k - 1) // 2 == L), None)
    if n is None:
        raise ValueError("give the upper-triangle judgments a12, a13, ..., a1n, a23, ... (3, 6, 10, ... values for 3, 4, 5, ... criteria; at most 10 criteria)")
    if any(v <= 0 for v in judgments):
        raise ValueError("every pairwise judgment must be > 0")
    P = [[1.0] * n for _ in range(n)]
    t = 0
    for i in range(n):
        for j in range(i + 1, n):
            P[i][j], P[j][i] = float(judgments[t]), 1.0 / float(judgments[t])
            t += 1
    return P


def _perron(P):
    n = len(P)
    w = [1.0 / n] * n
    calm = 0
    for _ in range(20000):
        z = [math.fsum(P[i][j] * w[j] for j in range(n)) for i in range(n)]
        s = math.fsum(z)
        z = [v / s for v in z]
        calm = calm + 1 if max(abs(a - b) / b for a, b in zip(z, w)) <= 1e-15 else 0
        w = z
        if calm >= 3:                                    # stable at rounding level for three iterations
            break
    lam = math.fsum(math.fsum(P[i][j] * w[j] for j in range(n)) for i in range(n))
    return w, lam


def ahp_priority(judgments):
    P = _ahp_matrix(judgments)
    w, lam = _perron(P)
    return {"weights": w, "lambda_max": lam, "best": float(_best(w))}


def ahp_consistency(judgments):
    P = _ahp_matrix(judgments)
    n = len(P)
    if n < 3:
        raise ValueError("the consistency ratio needs at least 3 criteria")
    w, lam = _perron(P)
    ci = (lam - n) / (n - 1)
    cr = ci / _RI[n]
    return {"CR": cr, "CI": ci, "lambda_max": lam, "weights": w, "acceptable": "yes" if cr < 0.1 else "no"}


def _verify_eigen(i, o):
    try:
        P = _ahp_matrix(i["judgments"])
    except ValueError:
        return 1.0
    w, lam = o["weights"], o["lambda_max"]
    if len(w) != len(P) or min(w) <= 0:
        return 1.0                                       # a positive eigenvector of a positive matrix is its Perron vector
    r = max(_rr(math.fsum(P[a][b] * w[b] for b in range(len(w))), lam * w[a]) for a in range(len(w)))
    return max(r, abs(math.fsum(w) - 1.0))


def _verify_cr(i, o):
    n = len(o["weights"])
    if n < 3:
        return 1.0
    ci = (o["lambda_max"] - n) / (n - 1)
    return max(_verify_eigen(i, o), _rr(o["CI"], ci), _rr(o["CR"], ci / _RI[n]), 0.0 if o["acceptable"] == ("yes" if o["CR"] < 0.1 else "no") else 1.0)


def _ref_eig(judgments):
    """INDEPENDENT reference: numpy.linalg.eig of the reciprocal matrix (largest real eigenvalue, its eigenvector)."""
    import numpy as np
    P = np.array(_ahp_matrix(judgments))
    vals, vecs = np.linalg.eig(P)
    k = int(np.argmax(vals.real))
    v = np.abs(vecs[:, k].real)
    return float(vals[k].real), [float(x) for x in v / v.sum()]


_SAATY = [1 / 9, 1 / 8, 1 / 7, 1 / 6, 1 / 5, 1 / 4, 1 / 3, 1 / 2, 1, 2, 3, 4, 5, 6, 7, 8, 9]


def _random_ahp(rng):
    n = rng.randint(3, 7)
    base = [rng.uniform(1, 9) for _ in range(n)]
    out = []
    for i in range(n):
        for j in range(i + 1, n):
            k = min(range(len(_SAATY)), key=lambda t: abs(math.log(_SAATY[t]) - math.log(base[i] / base[j])))
            k = max(0, min(len(_SAATY) - 1, k + rng.choice([-1, 0, 0, 1])))
            out.append(round(_SAATY[k], 4))
    return {"judgments": out}


_AHP_IN = [_I("judgments", "upper-triangle pairwise judgments a12, a13, ..., a23, ... (Saaty scale; a_ji = 1/a_ij)", "vector", min_length=1, max_length=45)]


def _ahp_q(rx, forbid):
    return lambda q: ((_labelled(q, {"judgments": r"judg(?:e)?ments?|comparisons?"}) or None)
                      if re.search(rx, q.lower()) and not re.search(forbid, q.lower()) else None)


_add("ahp_priority_vector",
     name="AHP priority vector (principal eigenvector)", topic="multi-criteria decision analysis / AHP",
     purpose="The priority weights of criteria compared in pairs (analytic hierarchy process): the normalised principal "
             "eigenvector of the reciprocal comparison matrix, and its eigenvalue lambda_max.",
     inputs=_AHP_IN,
     outputs=[_O("weights", "priority weights (sum 1)"), _O("lambda_max", "principal eigenvalue"), _O("best", "highest-priority criterion, 1-based")],
     primary="lambda_max", conditions=["judgments > 0", "2 to 10 criteria"],
     method="power iteration on the positive reciprocal matrix; certified as an eigenpair (P w = lambda w, w > 0: the Perron vector)",
     latex=r"P w = \lambda_{\max} w,\quad \sum_i w_i = 1",
     run=ahp_priority, verify=_verify_eigen,
     reference=lambda judgments: (lambda r: {"lambda_max": r[0], "weights": r[1]})(_ref_eig(judgments)),
     parse=_ahp_q(r"\b(ahp|analytic hierarchy)\b.*\b(priorit|weights?|eigenvector)", r"\b(consistency|geometric mean)\b"),
     wrong=lambda judgments: (lambda P: {"weights": [sum(r) / sum(map(sum, P)) for r in P], "lambda_max": float(len(P)), "best": 1.0})(_ahp_matrix(judgments)),
     random_inputs=_random_ahp,
     keywords=["AHP", "analytic hierarchy process", "priority vector", "pairwise comparison"],
     templates=["Find the AHP priority vector for the pairwise judgments [3, 5, 2]."],
     examples=[{"inputs": {"judgments": [2, 4, 2]}, "outputs": {"weights": [4 / 7, 2 / 7, 1 / 7], "lambda_max": 3.0, "best": 1.0}},
               {"inputs": {"judgments": [3]}, "outputs": {"weights": [0.75, 0.25], "lambda_max": 2.0}}],
     questions=[{"q": "Find the AHP priority vector for the pairwise judgments [2, 4, 2].", "outputs": {"lambda_max": 3.0}},
                {"q": "Find the AHP priority vector for the pairwise judgments [2, 4].", "outcome": "INVALID_INPUT"}])

_add("ahp_consistency_ratio",
     name="AHP consistency ratio", topic="multi-criteria decision analysis / AHP",
     purpose="How consistent a set of pairwise comparisons is: CI = (lambda_max - n)/(n - 1) and CR = CI / RI (Saaty's "
             "random index); CR below 0.10 is conventionally acceptable.",
     inputs=_AHP_IN,
     outputs=[_O("CR", "consistency ratio"), _O("CI", "consistency index"), _O("lambda_max", "principal eigenvalue"),
              _O("weights", "priority weights"), _O("acceptable", "yes when CR < 0.10", "text")],
     primary="CR", conditions=["3 to 10 criteria", "RI = 0.58, 0.90, 1.12, 1.24, 1.32, 1.41, 1.45, 1.49 for n = 3..10 (Saaty)"],
     method="principal eigenpair by power iteration (certified as an eigenpair), then CI and CR",
     latex=r"CI = \frac{\lambda_{\max} - n}{n - 1},\quad CR = \frac{CI}{RI_n}",
     run=ahp_consistency, verify=_verify_cr,
     reference=lambda judgments: (lambda r, n: {"CR": (r[0] - n) / (n - 1) / _RI[n], "lambda_max": r[0]})(_ref_eig(judgments), len(_ahp_matrix(judgments))),
     parse=_ahp_q(r"\bconsistency (?:ratio|index)\b", r"\bgeometric mean\b"),
     wrong=lambda judgments: (lambda r: r | {"CR": r["CI"], "acceptable": "yes" if r["CI"] < 0.1 else "no"})(ahp_consistency(judgments)),   # RI forgotten
     random_inputs=_random_ahp,
     keywords=["consistency ratio", "AHP", "consistency index"],
     templates=["Find the AHP consistency ratio for the pairwise judgments [3, 5, 2]."],
     examples=[{"inputs": {"judgments": [2, 4, 2]}, "outputs": {"CR": 0.0, "CI": 0.0, "acceptable": "yes"}}],
     questions=[{"q": "Find the AHP consistency ratio for the pairwise judgments [2, 4, 2].", "outputs": {"CR": 0.0}},
                {"q": "Find the AHP consistency ratio for the pairwise judgments [3].", "outcome": "INVALID_INPUT"}])


def ahp_geometric_mean(judgments):
    P = _ahp_matrix(judgments)
    n = len(P)
    g = [math.exp(math.fsum(math.log(v) for v in r) / n) for r in P]
    s = math.fsum(g)
    w = [v / s for v in g]
    return {"weights": w, "max_weight": max(w), "best": float(_best(w))}


def _verify_gm(i, o):
    """Optimality of the logarithmic least-squares fit: for every i, sum_j (ln a_ij - ln w_i + ln w_j) = 0."""
    try:
        P = _ahp_matrix(i["judgments"])
    except ValueError:
        return 1.0
    w = o["weights"]
    if len(w) != len(P) or min(w) <= 0:
        return 1.0
    n = len(w)
    res = max(abs(math.fsum(math.log(P[a][b]) - math.log(w[a]) + math.log(w[b]) for b in range(n))) for a in range(n))
    return max(res / n, abs(math.fsum(w) - 1), _rr(o["max_weight"], max(w)), 0.0 if int(o["best"]) == _best(w) else 1.0)


def _ref_llsm(judgments):
    """INDEPENDENT reference: the logarithmic least-squares problem min sum (ln a_ij - y_i + y_j)^2 solved by
    numpy.linalg.lstsq (with sum y = 0); w = exp(y) normalised."""
    import numpy as np
    P = np.array(_ahp_matrix(judgments))
    n = len(P)
    rows, rhs = [], []
    for i in range(n):
        for j in range(n):
            if i != j:
                r = np.zeros(n)
                r[i], r[j] = 1.0, -1.0
                rows.append(r)
                rhs.append(np.log(P[i, j]))
    rows.append(np.ones(n))
    rhs.append(0.0)
    y = np.linalg.lstsq(np.array(rows), np.array(rhs), rcond=None)[0]
    w = np.exp(y) / np.exp(y).sum()
    return {"weights": [float(v) for v in w], "max_weight": float(w.max())}


_add("ahp_geometric_mean",
     name="AHP weights by the geometric mean method", topic="multi-criteria decision analysis / AHP",
     purpose="Priority weights from pairwise comparisons as normalised row geometric means (the logarithmic "
             "least-squares solution).",
     inputs=_AHP_IN,
     outputs=[_O("weights", "priority weights (sum 1)"), _O("max_weight", "largest weight"), _O("best", "highest-priority criterion, 1-based")],
     primary="max_weight", conditions=["judgments > 0", "2 to 10 criteria"],
     method="row geometric means, normalised", latex=r"w_i = \frac{(\prod_j a_{ij})^{1/n}}{\sum_k (\prod_j a_{kj})^{1/n}}",
     run=ahp_geometric_mean, verify=_verify_gm, reference=_ref_llsm,
     parse=_ahp_q(r"\bgeometric mean\b.*\b(ahp|weights?|priorit)|\b(ahp|analytic hierarchy)\b.*\bgeometric mean\b", r"\bconsistency\b"),
     wrong=lambda judgments: (lambda P: (lambda a: {"weights": [v / sum(a) for v in a], "max_weight": max(a) / sum(a), "best": 1.0})([sum(r) / len(r) for r in P]))(_ahp_matrix(judgments)),
     random_inputs=_random_ahp,
     keywords=["geometric mean method", "AHP", "row geometric mean"],
     templates=["Find the AHP weights by the geometric mean method for the pairwise judgments [3, 5, 2]."],
     examples=[{"inputs": {"judgments": [2, 4, 2]}, "outputs": {"weights": [4 / 7, 2 / 7, 1 / 7], "max_weight": 4 / 7, "best": 1.0}}],
     questions=[{"q": "Find the AHP weights by the geometric mean method for the pairwise judgments [2, 4, 2].", "outputs": {"max_weight": 4 / 7}}])


# =================================================================================================================
# quadratic optimisation (exact)
# =================================================================================================================
def _sym_pd(Q, name="Q"):
    n = len(Q)
    if n == 0 or any(len(r) != n for r in Q) or n > 10:
        raise ValueError(f"{name} must be a square matrix (at most 10 x 10)")
    M = [[_fr(v) for v in r] for r in Q]
    if any(M[i][j] != M[j][i] for i in range(n) for j in range(n)):
        raise ValueError(f"{name} must be symmetric")
    W = [r[:] for r in M]
    for k in range(n):                                   # symmetric elimination: positive pivots <=> positive definite
        if W[k][k] <= 0:
            raise ValueError(f"{name} must be positive definite (otherwise there is no unique minimum)")
        for i in range(k + 1, n):
            f = W[i][k] / W[k][k]
            for j in range(k, n):
                W[i][j] -= f * W[k][j]
    return M


def _solve_exact(M, rhs):
    n = len(M)
    A = [list(r) + [v] for r, v in zip(M, rhs)]
    for k in range(n):
        p = next((i for i in range(k, n) if A[i][k] != 0), None)
        if p is None:
            raise ValueError("the system is singular")
        A[k], A[p] = A[p], A[k]
        for i in range(n):
            if i != k and A[i][k] != 0:
                f = A[i][k] / A[k][k]
                A[i] = [a - f * b for a, b in zip(A[i], A[k])]
    return [A[i][n] / A[i][i] for i in range(n)]


def _qf(M, c, x):
    return sum(x[i] * M[i][j] * x[j] for i in range(len(x)) for j in range(len(x))) / 2 + sum(ci * xi for ci, xi in zip(c, x))


def quadratic_min(Q, c):
    M = _sym_pd(Q)
    if len(c) != len(M):
        raise ValueError(f"c must have {len(M)} entries")
    cf = [_fr(v) for v in c]
    x = _solve_exact(M, [-v for v in cf])
    return {"f_min": float(_qf(M, cf, x)), "x_star": _fl(x)}


def _grad_res(Q, c, x, extra=None):
    n = len(x)
    g = [math.fsum(float(Q[i][j]) * x[j] for j in range(n)) + float(c[i]) + (extra[i] if extra else 0.0) for i in range(n)]
    sc = max([1.0] + [abs(float(v)) for r in Q for v in r] + [abs(float(v)) for v in c]) * max([1.0] + [abs(v) for v in x])
    return max(abs(v) for v in g) / sc


def _verify_qmin(i, o):
    try:
        M = _sym_pd(i["Q"])
    except ValueError:
        return 1.0
    x = o["x_star"]
    if len(x) != len(M):
        return 1.0
    f = 0.5 * math.fsum(x[a] * float(M[a][b]) * x[b] for a in range(len(x)) for b in range(len(x))) + math.fsum(float(v) * t for v, t in zip(i["c"], x))
    return max(_grad_res(i["Q"], i["c"], x), _rel(o["f_min"], f))


def _ref_qmin(Q, c):
    """INDEPENDENT reference: sympy — the gradient of the symbolic objective set to zero and solved exactly."""
    import sympy as sp
    n = len(Q)
    X = sp.symbols(f"x0:{n}")
    R = lambda v: sp.Rational(repr(float(v)))
    f = sp.Rational(1, 2) * sum(X[a] * R(Q[a][b]) * X[b] for a in range(n) for b in range(n)) + sum(R(c[a]) * X[a] for a in range(n))
    sol = sp.solve([sp.diff(f, v) for v in X], X, dict=True)[0]
    return {"f_min": float(f.subs(sol)), "x_star": [float(sol[v]) for v in X]}


def _random_spd(rng, n):
    B = [[rng.randint(-3, 3) for _ in range(n)] for _ in range(n)]
    return [[sum(B[k][i] * B[k][j] for k in range(n)) + (rng.randint(1, 4) if i == j else 0) for j in range(n)] for i in range(n)]


def _qp_q(rx, forbid, extra=None):
    labels = {"Q": r"hessian(?: matrix)?(?: q)?\s*=?|\bq\s*=", "c": r"linear term(?: c)?\s*=?|\bc\s*="}
    labels.update(extra or {})

    def p(q):
        ql = q.lower()
        if not re.search(rx, ql) or (forbid and re.search(forbid, ql)):
            return None
        return _labelled(q, labels) or None
    return p


_add("unconstrained_quadratic_min",
     name="Unconstrained quadratic minimisation", topic="nonlinear optimisation / quadratic",
     purpose="Minimise f(x) = 1/2 x^T Q x + c^T x for a symmetric positive definite Q: the minimiser and the minimum.",
     inputs=[_I("Q", "symmetric positive definite Hessian Q (n x n)", "matrix"), _I("c", "linear term c (n entries)", "vector", min_length=1, max_length=10)],
     outputs=[_O("f_min", "minimum value"), _O("x_star", "the minimiser x* = -Q^{-1} c (unique)")],
     primary="f_min", conditions=["Q symmetric positive definite (convention: f = 1/2 x^T Q x + c^T x)", "n <= 10"],
     method="positive definiteness by symmetric elimination, then Q x = -c solved exactly; certified by the gradient",
     latex=r"x^* = -Q^{-1} c,\quad f^* = \tfrac12 c^\top x^*",
     run=quadratic_min, verify=_verify_qmin, reference=_ref_qmin,
     parse=_qp_q(r"\bunconstrained quadratic\b", r"\bsubject to\b|\bsteepest\b"),
     wrong=lambda Q, c: (lambda r: r | {"f_min": float(sum(float(a) * b for a, b in zip(c, r["x_star"])))})(quadratic_min(Q, c)),   # the 1/2 dropped
     random_inputs=lambda rng: (lambda n: {"Q": _random_spd(rng, n), "c": [rng.randint(-9, 9) or 1 for _ in range(n)]})(rng.randint(2, 5)),
     keywords=["quadratic minimization", "positive definite", "unconstrained"],
     templates=["Minimise the unconstrained quadratic with Hessian Q = [[4, 1], [1, 3]] and linear term c = [-1, -2]."],
     examples=[{"inputs": {"Q": [[4, 1], [1, 3]], "c": [-1, -2]}, "outputs": {"x_star": [1 / 11, 7 / 11], "f_min": -15 / 22}},
               {"inputs": {"Q": [[2, 0], [0, 2]], "c": [-4, 2]}, "outputs": {"x_star": [2.0, -1.0], "f_min": -5.0}}],
     questions=[{"q": "Minimise the unconstrained quadratic with Hessian Q = [[4, 1], [1, 3]] and linear term c = [-1, -2].", "outputs": {"f_min": -15 / 22}},
                {"q": "Minimise the unconstrained quadratic with Hessian Q = [[1, 2], [2, 1]] and linear term c = [-1, -2].", "outcome": "INVALID_INPUT"}])


def eq_qp(Q, c, A, b):
    M = _sym_pd(Q)
    n = len(M)
    if len(c) != n:
        raise ValueError(f"c must have {n} entries")
    if not A or any(len(r) != n for r in A) or len(A) != len(b) or len(A) >= n + 1:
        raise ValueError(f"A must have {n} columns and one row per entry of b (at most {n} rows)")
    Af, bf, cf = [[_fr(v) for v in r] for r in A], [_fr(v) for v in b], [_fr(v) for v in c]
    p = len(Af)
    K = [M[i] + [Af[k][i] for k in range(p)] for i in range(n)] + [Af[k] + [F(0)] * p for k in range(p)]
    try:
        z = _solve_exact(K, [-v for v in cf] + bf)
    except ValueError:
        raise ValueError("the constraint rows must be linearly independent")
    x, lam = z[:n], z[n:]
    return {"f_min": float(_qf(M, cf, x)), "x_star": _fl(x), "multipliers": _fl(lam)}


def _verify_eqqp(i, o):
    try:
        M = _sym_pd(i["Q"])
    except ValueError:
        return 1.0
    x, lam, A = o["x_star"], o["multipliers"], i["A"]
    if len(x) != len(M) or len(lam) != len(A):
        return 1.0
    atl = [math.fsum(float(A[k][j]) * lam[k] for k in range(len(A))) for j in range(len(x))]
    feas = max(abs(math.fsum(float(A[k][j]) * x[j] for j in range(len(x))) - float(i["b"][k])) for k in range(len(A))) / max([1.0] + [abs(float(v)) for v in i["b"]] + [abs(float(v)) for r in A for v in r])
    f = 0.5 * math.fsum(x[a] * float(M[a][b]) * x[b] for a in range(len(x)) for b in range(len(x))) + math.fsum(float(v) * t for v, t in zip(i["c"], x))
    return max(_grad_res(i["Q"], i["c"], x, atl), feas, _rel(o["f_min"], f))


def _ref_eqqp(Q, c, A, b):
    """INDEPENDENT reference: sympy — the Lagrangian L = f + lambda^T (A x - b), every partial derivative set to zero."""
    import sympy as sp
    n, p = len(Q), len(A)
    X, Lm = sp.symbols(f"x0:{n}"), sp.symbols(f"l0:{p}")
    R = lambda v: sp.Rational(repr(float(v)))
    f = sp.Rational(1, 2) * sum(X[a] * R(Q[a][bb]) * X[bb] for a in range(n) for bb in range(n)) + sum(R(c[a]) * X[a] for a in range(n))
    L = f + sum(Lm[k] * (sum(R(A[k][j]) * X[j] for j in range(n)) - R(b[k])) for k in range(p))
    sol = sp.solve([sp.diff(L, v) for v in X + Lm], X + Lm, dict=True)[0]
    return {"f_min": float(f.subs(sol)), "x_star": [float(sol[v]) for v in X], "multipliers": [float(sol[v]) for v in Lm]}


def _random_eqqp(rng):
    n = rng.randint(2, 5)
    p = rng.randint(1, n - 1)
    return {"Q": _random_spd(rng, n), "c": [rng.randint(-9, 9) for _ in range(n)],
            "A": [[rng.randint(-4, 4) for _ in range(n)] for _ in range(p)], "b": [rng.randint(-9, 9) for _ in range(p)]}


_add("equality_constrained_qp",
     name="Equality-constrained quadratic program (Lagrange multipliers / KKT)", topic="nonlinear optimisation / quadratic",
     purpose="Minimise 1/2 x^T Q x + c^T x subject to A x = b: the minimiser, the minimum and the Lagrange multipliers.",
     inputs=[_I("Q", "symmetric positive definite Hessian Q", "matrix"), _I("c", "linear term c", "vector", min_length=1, max_length=10),
             _I("A", "constraint matrix A (rows = constraints)", "matrix"), _I("b", "right-hand side b", "vector", min_length=1, max_length=10)],
     outputs=[_O("f_min", "minimum value"), _O("x_star", "the minimiser (unique)"),
              _O("multipliers", "Lagrange multipliers lambda (convention Q x + c + A^T lambda = 0)")],
     primary="f_min", conditions=["Q symmetric positive definite", "rows of A linearly independent"],
     method="the KKT system [Q A^T; A 0][x; lambda] = [-c; b] solved exactly; certified by stationarity and feasibility",
     latex=r"\begin{bmatrix} Q & A^\top \\ A & 0 \end{bmatrix}\begin{bmatrix} x \\ \lambda \end{bmatrix} = \begin{bmatrix} -c \\ b \end{bmatrix}",
     run=eq_qp, verify=_verify_eqqp, reference=_ref_eqqp,
     parse=_qp_q(r"\bsubject to\b.*\bequality|\bequality[- ]constrained quadratic\b", None, {"A": r"constraint matrix(?: a)?\s*=?|\ba\s*=", "b": r"right[- ]hand side(?: b)?\s*=?|\bb\s*="}),
     wrong=lambda Q, c, A, b: (lambda r: {"f_min": r["f_min"], "x_star": r["x_star"], "multipliers": [0.0] * len(A)})(quadratic_min(Q, c)),   # constraints ignored
     random_inputs=_random_eqqp,
     keywords=["equality constrained", "lagrange multipliers", "KKT", "quadratic program"],
     templates=["Minimise the equality-constrained quadratic with Hessian Q = [[2, 0], [0, 2]], linear term c = [0, 0], constraint matrix A = [[1, 1]] and right-hand side b = [2]."],
     examples=[{"inputs": {"Q": [[2, 0], [0, 2]], "c": [0, 0], "A": [[1, 1]], "b": [2]}, "outputs": {"x_star": [1.0, 1.0], "f_min": 2.0, "multipliers": [-2.0]}}],
     questions=[{"q": "Minimise the equality-constrained quadratic with Hessian Q = [[2, 0], [0, 2]], linear term c = [0, 0], constraint matrix A = [[1, 1]] and right-hand side b = [2].",
                 "outputs": {"f_min": 2.0}},
                {"q": "Minimise the equality-constrained quadratic with Hessian Q = [[2, 0], [0, 2]], linear term c = [0, 0], constraint matrix A = [[1, 1], [2, 2]] and right-hand side b = [2, 4].",
                 "outcome": "INVALID_INPUT"}])


def steepest_step(Q, c, x0):
    M = _sym_pd(Q)
    n = len(M)
    if len(c) != n or len(x0) != n:
        raise ValueError(f"c and x0 must have {n} entries")
    cf, x = [_fr(v) for v in c], [_fr(v) for v in x0]
    g = [sum(M[i][j] * x[j] for j in range(n)) + cf[i] for i in range(n)]
    gg = sum(v * v for v in g)
    if gg == 0:
        raise ValueError("x0 is already the minimiser (the gradient is zero)")
    alpha = gg / sum(g[i] * M[i][j] * g[j] for i in range(n) for j in range(n))
    x1 = [a - alpha * b for a, b in zip(x, g)]
    return {"alpha": float(alpha), "x1": _fl(x1), "f1": float(_qf(M, cf, x1)), "gradient": _fl(g)}


def _verify_sd(i, o):
    try:
        M = _sym_pd(i["Q"])
    except ValueError:
        return 1.0
    n = len(M)
    Qf, c, x0 = [[float(v) for v in r] for r in M], [float(v) for v in i["c"]], [float(v) for v in i["x0"]]
    g = [math.fsum(Qf[a][b] * x0[b] for b in range(n)) + c[a] for a in range(n)]
    x1 = o["x1"]
    g1 = [math.fsum(Qf[a][b] * x1[b] for b in range(n)) + c[a] for a in range(n)]
    gn = math.fsum(v * v for v in g)
    orth = abs(math.fsum(a * b for a, b in zip(g, g1))) / gn             # exact line search: new gradient orthogonal
    step = max(_rel(x1[a], x0[a] - o["alpha"] * g[a]) for a in range(n))
    f1 = 0.5 * math.fsum(x1[a] * Qf[a][b] * x1[b] for a in range(n) for b in range(n)) + math.fsum(a * b for a, b in zip(c, x1))
    return max(orth, step, _rel(o["f1"], f1), max(_rel(a, b) for a, b in zip(o["gradient"], g)))


def _ref_sd(Q, c, x0):
    """INDEPENDENT reference: sympy — gradient by symbolic differentiation, then the exact minimiser of
    phi(t) = f(x0 - t g) from phi'(t) = 0."""
    import sympy as sp
    n = len(Q)
    X, t = sp.symbols(f"x0:{n}"), sp.Symbol("t")
    R = lambda v: sp.Rational(repr(float(v)))
    f = sp.Rational(1, 2) * sum(X[a] * R(Q[a][b]) * X[b] for a in range(n) for b in range(n)) + sum(R(c[a]) * X[a] for a in range(n))
    at = {X[a]: R(x0[a]) for a in range(n)}
    g = [sp.diff(f, v).subs(at) for v in X]
    phi = f.subs({X[a]: R(x0[a]) - t * g[a] for a in range(n)}, simultaneous=True)
    ts = sp.solve(sp.diff(phi, t), t)[0]
    return {"alpha": float(ts), "f1": float(phi.subs(t, ts)), "x1": [float(R(x0[a]) - ts * g[a]) for a in range(n)]}


_add("steepest_descent_step",
     name="Steepest descent step with exact line search (quadratic)", topic="nonlinear optimisation / descent methods",
     purpose="One iteration of steepest descent on f(x) = 1/2 x^T Q x + c^T x from x0: the gradient, the exact step "
             "length, the new point and its function value.",
     inputs=[_I("Q", "symmetric positive definite Hessian Q", "matrix"), _I("c", "linear term c", "vector", min_length=1, max_length=10),
             _I("x0", "starting point x0", "vector", min_length=1, max_length=10)],
     outputs=[_O("alpha", "exact step length g^T g / g^T Q g"), _O("x1", "new point x0 - alpha g"), _O("f1", "f at the new point"), _O("gradient", "gradient at x0")],
     primary="alpha", conditions=["Q symmetric positive definite", "x0 is not the minimiser"],
     method="g = Q x0 + c; alpha = g^T g / g^T Q g in exact arithmetic; certified by g^T grad f(x1) = 0",
     latex=r"\alpha = \frac{g^\top g}{g^\top Q g},\quad x_1 = x_0 - \alpha g",
     run=steepest_step, verify=_verify_sd, reference=_ref_sd,
     parse=_qp_q(r"\bsteepest descent\b", None, {"x0": r"starting point(?: x0)?\s*=?|\bx0\s*=|\bfrom(?: the point)?"}),
     wrong=lambda Q, c, x0: (lambda r: (lambda x1: r | {"alpha": 1.0, "x1": x1})([a - b for a, b in zip(map(float, x0), r["gradient"])]))(steepest_step(Q, c, x0)),   # unit step
     random_inputs=lambda rng: (lambda n: {"Q": _random_spd(rng, n), "c": [rng.randint(-9, 9) for _ in range(n)], "x0": [rng.randint(-5, 5) for _ in range(n)]})(rng.randint(2, 4)),
     keywords=["steepest descent", "exact line search", "gradient descent"],
     templates=["Take one steepest descent step with exact line search for Hessian Q = [[2, 0], [0, 8]] and linear term c = [0, 0] from x0 = [4, 1]."],
     examples=[{"inputs": {"Q": [[2, 0], [0, 8]], "c": [0, 0], "x0": [4, 1]}, "outputs": {"gradient": [8.0, 8.0], "alpha": 0.2, "x1": [2.4, -0.6], "f1": 5.76 + 1.44}}],
     questions=[{"q": "Take one steepest descent step with exact line search for Hessian Q = [[2, 0], [0, 8]] and linear term c = [0, 0] from x0 = [4, 1].",
                 "outputs": {"alpha": 0.2}},
                {"q": "Take one steepest descent step with exact line search for Hessian Q = [[2, 0], [0, 8]] and linear term c = [0, 0] from x0 = [0, 0].",
                 "outcome": "INVALID_INPUT"}])


# --- calculus: critical points of a two-variable quadratic; extrema of a polynomial on an interval -----------------
def critical_point_2var(a, b, c, d=0.0, e=0.0, f0=0.0):
    _finite(a, b, c, d, e, f0)
    A, B, C, D, E, G = (_fr(v) for v in (a, b, c, d, e, f0))
    det = 4 * A * C - B * B
    if det == 0:
        raise ValueError("4ac - b^2 = 0: no isolated critical point (the second-derivative test is inconclusive)")
    x, y = _solve_exact([[2 * A, B], [B, 2 * C]], [-D, -E])
    kind = "saddle point" if det < 0 else ("local minimum" if A > 0 else "local maximum")
    val = A * x * x + B * x * y + C * y * y + D * x + E * y + G
    return {"f_value": float(val), "x": float(x), "y": float(y), "type": kind}


def _verify_cp(i, o):
    a, b, c = (float(i[k]) for k in ("a", "b", "c"))
    d, e, g = (float(i.get(k, 0.0)) for k in ("d", "e", "f0"))
    x, y = o["x"], o["y"]
    sc = max(1.0, abs(a), abs(b), abs(c), abs(d), abs(e)) * max(1.0, abs(x), abs(y))
    det = 4 * a * c - b * b
    kind = "saddle point" if det < 0 else ("local minimum" if a > 0 else "local maximum")
    val = a * x * x + b * x * y + c * y * y + d * x + e * y + g
    return max(abs(2 * a * x + b * y + d) / sc, abs(b * x + 2 * c * y + e) / sc, _rel(o["f_value"], val), 0.0 if o["type"] == kind else 1.0)


def _ref_cp(a, b, c, d=0.0, e=0.0, f0=0.0):
    """INDEPENDENT reference: sympy — solve grad f = 0 and classify by the signs of the Hessian's eigenvalues."""
    import sympy as sp
    x, y = sp.symbols("x y")
    R = lambda v: sp.Rational(repr(float(v)))
    f = R(a) * x ** 2 + R(b) * x * y + R(c) * y ** 2 + R(d) * x + R(e) * y + R(f0)
    s = sp.solve([sp.diff(f, x), sp.diff(f, y)], [x, y], dict=True)[0]
    ev = list(sp.hessian(f, (x, y)).eigenvals())
    kind = "local minimum" if all(v > 0 for v in ev) else ("local maximum" if all(v < 0 for v in ev) else "saddle point")
    return {"f_value": float(f.subs(s)), "x": float(s[x]), "y": float(s[y]), "type": kind}


_TERM2 = re.compile(r"([+-]?)(\d+(?:\.\d+)?)?\*?(x\^2|y\^2|x\*?y|y\*?x|x|y)?")


def _poly2(text):
    """'3x^2 + 2xy - y^2 + 4x + 1' -> {a, b, c, d, e, f0}; None for anything else."""
    s = text.replace(" ", "").replace("**", "^").replace("²", "^2")
    if not s or re.search(r"[^0-9.xy^*+\-]", s):
        return None
    out = {"a": 0.0, "b": 0.0, "c": 0.0, "d": 0.0, "e": 0.0, "f0": 0.0}
    key = {"x^2": "a", "y^2": "c", "xy": "b", "x*y": "b", "yx": "b", "y*x": "b", "x": "d", "y": "e", None: "f0"}
    pos, seen = 0, False
    while pos < len(s):
        m = _TERM2.match(s, pos)
        if not m or m.end() == pos or (m.group(2) is None and m.group(3) is None) or (seen and not m.group(1)):
            return None
        coef = float(m.group(2)) if m.group(2) else 1.0
        out[key[m.group(3)]] += -coef if m.group(1) == "-" else coef
        pos, seen = m.end(), True
    return out


def _parse_cp(q):
    if not re.search(r"\bcritical point\b", q.lower()):
        return None
    m = re.search(r"f\s*\(\s*x\s*,\s*y\s*\)\s*=\s*(.+?)\s*[.?]?\s*$", q)
    return _poly2(m.group(1)) if m else None


_add("critical_point_two_variables",
     name="Critical point of a two-variable quadratic and its classification", topic="calculus optimisation / several variables",
     purpose="For f(x, y) = a x^2 + b xy + c y^2 + d x + e y + f0: the critical point, its value and whether it is a "
             "local minimum, a local maximum or a saddle point (second-derivative test).",
     inputs=[_I("a", "coefficient of x^2"), _I("b", "coefficient of xy"), _I("c", "coefficient of y^2"),
             _I("d", "coefficient of x", required=False), _I("e", "coefficient of y", required=False), _I("f0", "constant term", required=False)],
     outputs=[_O("f_value", "value of f at the critical point"), _O("x", "x of the critical point"), _O("y", "y of the critical point"),
              _O("type", "local minimum, local maximum or saddle point", "text")],
     primary="f_value", conditions=["4ac - b^2 != 0 (an isolated critical point)"],
     method="grad f = 0 solved exactly; D = f_xx f_yy - f_xy^2 = 4ac - b^2 classifies it",
     latex=r"D = f_{xx} f_{yy} - f_{xy}^2 = 4ac - b^2",
     run=critical_point_2var, verify=_verify_cp, reference=_ref_cp, parse=_parse_cp,
     wrong=lambda a, b, c, d=0.0, e=0.0, f0=0.0: (lambda r: r | {"f_value": r["f_value"] + 1.0} if a * c - b * b == 0 else
         (lambda s: r | {"x": float(s[0]), "y": float(s[1]), "f_value": float(a * s[0] ** 2 + b * s[0] * s[1] + c * s[1] ** 2 + d * s[0] + e * s[1] + f0)})(
         _solve_exact([[_fr(a), _fr(b)], [_fr(b), _fr(c)]], [-_fr(d), -_fr(e)])))(critical_point_2var(a, b, c, d, e, f0)),   # f_x taken as a x + b y + d
     random_inputs=lambda rng: {"a": rng.choice([-5, -4, -3, -2, -1, 1, 2, 3, 4, 5]), "b": rng.randint(-6, 6), "c": rng.choice([-5, -4, -3, -2, -1, 1, 2, 3, 4, 5]),
                                "d": rng.randint(-9, 9), "e": rng.randint(-9, 9), "f0": rng.randint(-9, 9)},
     keywords=["critical point", "second derivative test", "saddle point"],
     templates=["Find and classify the critical point of f(x, y) = 3x^2 + 2xy + y^2 - 4x + 6y + 1."],
     examples=[{"inputs": {"a": 3, "b": 2, "c": 1, "d": -4, "e": 6, "f0": 1}, "outputs": {"x": 2.5, "y": -5.5, "type": "local minimum", "f_value": -20.5}},
               {"inputs": {"a": 1, "b": 0, "c": -1}, "outputs": {"x": 0.0, "y": 0.0, "f_value": 0.0, "type": "saddle point"}}],
     questions=[{"q": "Find and classify the critical point of f(x, y) = 3x^2 + 2xy + y^2 - 4x + 6y + 1.", "outputs": {"f_value": -20.5, "type": "local minimum"}},
                {"q": "Find and classify the critical point of f(x, y) = x^2 + 2xy + y^2.", "outcome": "INVALID_INPUT"}])


def _peval(cs, x):
    v = 0.0
    for a in cs:
        v = v * x + a
    return v


def _pder(cs):
    n = len(cs) - 1
    return [a * (n - k) for k, a in enumerate(cs[:-1])]


def _roots_in(cs, lo, hi):
    """Real roots of the polynomial cs (highest power first) in [lo, hi], by splitting at the roots of its derivative
    and bisecting each monotone piece to the last float."""
    while len(cs) > 1 and cs[0] == 0:
        cs = cs[1:]
    if len(cs) <= 1:
        return []
    if len(cs) == 2:
        r = -cs[1] / cs[0]
        return [r] if lo <= r <= hi else []
    pts = [lo] + _roots_in(_pder(cs), lo, hi) + [hi]
    out = []
    for a, b in zip(pts, pts[1:]):
        fa, fb = _peval(cs, a), _peval(cs, b)
        if fa == 0:
            out.append(a)
        if fa * fb < 0:
            for _ in range(2000):
                mid = (a + b) / 2
                if mid in (a, b):
                    break
                fm = _peval(cs, mid)
                if fm == 0:
                    a = b = mid
                    break
                if (fm < 0) == (fa < 0):
                    a, fa = mid, fm
                else:
                    b = mid
            out.append((a + b) / 2)
    if _peval(cs, hi) == 0:
        out.append(hi)
    return sorted(set(out))


def poly_extrema(coefficients, lo, hi):
    _finite(lo, hi, *coefficients)
    if not lo < hi:
        raise ValueError("the interval needs lo < hi")
    cs = [float(v) for v in coefficients]
    if len(cs) < 2 or len(cs) > 6:
        raise ValueError("give 2 to 6 coefficients (degree 1 to 5), highest power first")
    cand = [lo, hi] + _roots_in(_pder(cs), lo, hi)
    vals = [(_peval(cs, x), x) for x in cand]
    mn, mx = min(v for v, _ in vals), max(v for v, _ in vals)
    tol = lambda t: 1e-12 * max(1.0, abs(t))
    return {"min_value": mn, "max_value": mx, "argmin_example": min(x for v, x in vals if v <= mn + tol(mn)),
            "argmax_example": min(x for v, x in vals if v >= mx - tol(mx))}


def _verify_pe(i, o):
    cs = [float(v) for v in i["coefficients"]]
    lo, hi = float(i["lo"]), float(i["hi"])
    cand = [lo, hi] + _roots_in(_pder(cs), lo, hi)
    vals = [_peval(cs, x) for x in cand]
    out = [_rel(o["min_value"], min(vals)), _rel(o["max_value"], max(vals))]
    for key, v in (("argmin_example", o["min_value"]), ("argmax_example", o["max_value"])):
        x = o[key]
        out.append(1.0 if not lo <= x <= hi else _rel(_peval(cs, x), v))
    return max(out)


def _ref_pe(coefficients, lo, hi):
    """INDEPENDENT reference: sympy — the exact real roots of f' (algebraic numbers) in [lo, hi] and the endpoints,
    f evaluated to 30 digits."""
    import sympy as sp
    x = sp.Symbol("x")
    R = lambda v: sp.Rational(repr(float(v)))
    f = sp.Poly([R(v) for v in coefficients], x)
    a, b = R(lo), R(hi)
    pts = [a, b] + [r for r in sp.Poly(f.diff(x), x).real_roots() if a <= r <= b] if f.degree() > 1 else [a, b]
    vals = [float(sp.N(f.as_expr().subs(x, p), 30)) for p in pts]
    return {"min_value": min(vals), "max_value": max(vals)}


def _random_poly(rng):
    lo = rng.randint(-5, 1)
    hi = lo + rng.randint(2, 7)
    k = rng.choice([-3, -2, -1, 1, 2, 3])
    roots = [round(rng.uniform(lo + 0.2, hi - 0.2), 1) for _ in range(rng.randint(2, 3))]
    d = [float(k)]
    for r in roots:                                       # f' = k prod (x - r)
        d = [a - r * b for a, b in zip(d + [0.0], [0.0] + d)]
    n = len(d)
    cs = [d[t] / (n - t) for t in range(n)] + [float(rng.randint(-9, 9))]
    return {"coefficients": [round(v, 6) for v in cs], "lo": lo, "hi": hi}


_add("polynomial_extrema_interval",
     name="Absolute extrema of a polynomial on a closed interval", topic="calculus optimisation / one variable",
     purpose="The absolute minimum and maximum of a polynomial on [lo, hi] (closed-interval method: endpoints and "
             "critical points), and where they are attained.",
     inputs=[_I("coefficients", "polynomial coefficients, highest power first", "vector", min_length=2, max_length=6),
             _I("lo", "left end of the interval"), _I("hi", "right end of the interval")],
     outputs=[_O("min_value", "absolute minimum"), _O("max_value", "absolute maximum"),
              _O("argmin_example", "a point attaining the minimum (the smallest one on a tie)"),
              _O("argmax_example", "a point attaining the maximum (the smallest one on a tie)")],
     primary="min_value", conditions=["lo < hi", "degree 1 to 5"],
     method="real roots of f' in [lo, hi] (monotone pieces between the roots of f'' bisected to the last float), "
            "compared with the endpoints",
     latex=r"\min_{[a,b]} f = \min\{f(a), f(b), f(c) : f'(c) = 0,\ a < c < b\}",
     run=poly_extrema, verify=_verify_pe, reference=_ref_pe,
     parse=lambda q: (lambda g: ({"coefficients": g["coefficients"], "lo": g["iv"][0], "hi": g["iv"][1]}
                                 if isinstance(g.get("iv"), list) and len(g["iv"]) == 2 and not isinstance(g["iv"][0], list) and "coefficients" in g
                                 else ({k: v for k, v in g.items() if k == "coefficients"} or None)))(
         _labelled(q, {"coefficients": r"coefficients?", "iv": r"interval|on"}))
     if re.search(r"\b(absolute|global) (?:minimum|maximum|extrema)\b", q.lower()) and re.search(r"\bpolynomial\b", q.lower()) else None,
     wrong=lambda coefficients, lo, hi: (lambda cs: {"min_value": min(_peval(cs, lo), _peval(cs, hi)), "max_value": max(_peval(cs, lo), _peval(cs, hi)),
                                                     "argmin_example": lo, "argmax_example": hi})([float(v) for v in coefficients]),   # endpoints only
     random_inputs=_random_poly,
     keywords=["absolute extrema", "closed interval method", "global minimum"],
     templates=["Find the absolute minimum and maximum of the polynomial with coefficients [1, -3, 0, 2] on the interval [-1, 3]."],
     examples=[{"inputs": {"coefficients": [1, -3, 0, 2], "lo": -1, "hi": 3}, "outputs": {"min_value": -2.0, "max_value": 2.0, "argmin_example": -1.0, "argmax_example": 0.0}}],
     questions=[{"q": "Find the absolute minimum and maximum of the polynomial with coefficients [1, -3, 0, 2] on the interval [-1, 3].",
                 "outputs": {"min_value": -2.0, "max_value": 2.0}},
                {"q": "Find the absolute minimum and maximum of the polynomial with coefficients [1, -3, 0, 2] on the interval [3, -1].", "outcome": "INVALID_INPUT"}])


# =================================================================================================================
# dynamic programming: lot sizing and Markov decision processes
# =================================================================================================================
def wagner_whitin(demand, K, h):
    _finite(K, h)
    d = [_fr(v) for v in demand]
    T = len(d)
    if T == 0 or T > 52:
        raise ValueError("give 1 to 52 periods of demand")
    if any(v < 0 for v in d) or K < 0 or h < 0:
        raise ValueError("demands, the setup cost and the holding cost must be >= 0")
    Kf, hf = _fr(K), _fr(h)
    best, arg = [F(0)] + [None] * T, [None] * (T + 1)
    for t in range(1, T + 1):
        if d[t - 1] == 0 and best[t - 1] is not None:
            best[t], arg[t] = best[t - 1], None              # nothing needed in period t
        for s in range(1, t + 1):                            # one order in period s covers s..t
            cost = best[s - 1] + Kf + hf * sum((u - s) * d[u - 1] for u in range(s, t + 1))
            if best[t] is None or cost < best[t]:
                best[t], arg[t] = cost, s
    q, t = [F(0)] * T, T
    while t > 0:
        s = arg[t]
        if s is None:
            t -= 1
            continue
        q[s - 1] = sum(d[s - 1:t])
        t = s - 1
    return {"min_cost": float(best[T]), "order_quantities_example": _fl(q), "orders": float(sum(1 for v in q if v > 0))}


def _verify_ww(i, o):
    q, d = o["order_quantities_example"], [float(v) for v in i["demand"]]
    if len(q) != len(d) or min(q) < 0:
        return 1.0
    inv, cost = 0.0, 0.0
    for qt, dt in zip(q, d):
        inv += qt - dt
        if inv < -1e-9 * max(1.0, dt):
            return 1.0
        cost += (float(i["K"]) if qt > 0 else 0.0) + float(i["h"]) * inv
    return max(_rel(o["min_cost"], cost), _rel(o["min_cost"], wagner_whitin(i["demand"], i["K"], i["h"])["min_cost"]),
               0.0 if o["orders"] == sum(1 for v in q if v > 0) else 1.0)


def _ref_ww(demand, K, h):
    """INDEPENDENT reference: the lot-sizing MILP (order q_t <= M y_t, inventory balance, y binary) by scipy milp."""
    T = len(demand)
    M = float(sum(demand))
    nv = 3 * T                                            # q_1..q_T, I_1..I_T, y_1..y_T
    c = [0.0] * T + [float(h)] * T + [float(K)] * T
    rows, lo, hi = [], [], []
    for t in range(T):
        r = [0.0] * nv
        r[t], r[T + t] = 1.0, -1.0
        if t:
            r[T + t - 1] = 1.0
        rows.append(r); lo.append(float(demand[t])); hi.append(float(demand[t]))
        r = [0.0] * nv
        r[t], r[2 * T + t] = 1.0, -M
        rows.append(r); lo.append(-float("inf")); hi.append(0.0)
    return {"min_cost": _milp_ref(c, rows, lo, hi, [M] * (2 * T) + [1.0] * T)}


_add("wagner_whitin_lot_sizing",
     name="Dynamic lot sizing (Wagner-Whitin)", topic="dynamic programming / inventory",
     purpose="The cheapest ordering plan for known demands over several periods with a fixed cost per order and a "
             "holding cost per unit carried to the next period (no shortages).",
     inputs=[_I("demand", "demand in each period", "vector", min_length=1, max_length=52), _I("K", "setup (ordering) cost per order"),
             _I("h", "holding cost per unit per period")],
     outputs=[_O("min_cost", "minimum total setup plus holding cost"), _O("order_quantities_example", "one optimal plan: quantity ordered in each period"),
              _O("orders", "number of orders in that plan")],
     primary="min_cost", conditions=["demands, K, h >= 0", "no initial stock, no shortages, holding charged on end-of-period stock"],
     method="Wagner-Whitin dynamic programme (each order covers whole periods) in exact arithmetic",
     latex=r"C_t = \min_{1 \le s \le t} \Big( C_{s-1} + K + h \sum_{u=s}^{t} (u - s)\, d_u \Big)",
     run=wagner_whitin, verify=_verify_ww, reference=_ref_ww,
     parse=lambda q: (lambda ql: None if not re.search(r"\b(wagner[- ]whitin|lot[- ]sizing|dynamic lot)\b", ql) else (
         (_labelled(q, {"demand": r"demands?"}) or {}) | ({"K": v} if (v := _num_after(ql, r"setup cost|ordering cost|order cost")) is not None else {})
         | ({"h": v} if (v := _num_after(ql, r"holding cost")) is not None else {}) or None))(q.lower()),
     wrong=lambda demand, K, h: {"min_cost": float(K) * sum(1 for v in demand if v > 0), "order_quantities_example": [float(v) for v in demand],
                                 "orders": float(sum(1 for v in demand if v > 0))},           # lot-for-lot taken as optimal
     random_inputs=lambda rng: {"demand": [rng.choice([0, rng.randint(5, 80), rng.randint(5, 80)]) for _ in range(rng.randint(4, 12))],
                                "K": rng.randint(40, 300), "h": rng.randint(1, 4)},
     keywords=["wagner-whitin", "lot sizing", "dynamic programming"],
     templates=["Wagner-Whitin lot sizing with demands [20, 50, 10, 50], setup cost 100 and holding cost 1 per unit per period."],
     examples=[{"inputs": {"demand": [20, 50, 10, 50], "K": 100, "h": 1}, "outputs": {"min_cost": 270.0}},
               {"inputs": {"demand": [10, 0, 10], "K": 50, "h": 5}, "outputs": {"min_cost": 100.0}}],
     questions=[{"q": "Wagner-Whitin lot sizing with demands [20, 50, 10, 50], setup cost 100 and holding cost 1 per unit per period.", "outputs": {"min_cost": 270.0}},
                {"q": "Wagner-Whitin lot sizing with demands [20, 50, 10, 50] and setup cost 100.", "outcome": "NEEDS_INPUT"}])


def _mdp_in(R, T, gamma):
    _finite(gamma)
    if not 0 < gamma < 1:
        raise ValueError("the discount factor must be between 0 and 1 (exclusive)")
    S, A = len(R), len(R[0]) if R else 0
    if S < 1 or S > 10 or A < 1 or A > 5 or any(len(r) != A for r in R):
        raise ValueError("rewards: one row per state (at most 10), one column per action (at most 5)")
    if len(T) != S * A or any(len(r) != S for r in T):
        raise ValueError(f"transitions: {S * A} rows (state 1 action 1, state 1 action 2, ...) of {S} probabilities")
    P = []
    for r in T:
        fr = [_fr(v) for v in r]
        s = sum(fr)
        if any(v < 0 for v in fr) or abs(s - 1) > F(1, 10 ** 9):
            raise ValueError("every transition row must be probabilities summing to 1")
        P.append([v / s for v in fr])
    return [[_fr(v) for v in r] for r in R], P, _fr(gamma), S, A


def mdp_discounted(R, T, gamma):
    Rf, P, g, S, A = _mdp_in(R, T, gamma)
    pol = [0] * S
    for _ in range(1000):
        V = _solve_exact([[(1 if s == t else 0) - g * P[s * A + pol[s]][t] for t in range(S)] for s in range(S)], [Rf[s][pol[s]] for s in range(S)])
        Qv = [[Rf[s][a] + g * sum(P[s * A + a][t] * V[t] for t in range(S)) for a in range(A)] for s in range(S)]
        new = [pol[s] if Qv[s][pol[s]] == max(Qv[s]) else Qv[s].index(max(Qv[s])) for s in range(S)]
        if new == pol:
            break
        pol = new
    best = [Qv[s].index(max(Qv[s])) + 1 for s in range(S)]          # lowest optimal action
    return {"value_state_1": float(V[0]), "values": _fl(V), "policy": [float(a) for a in best]}


def _verify_mdp(i, o):
    try:
        Rf, P, g, S, A = _mdp_in(i["R"], i["T"], i["gamma"])
    except ValueError:
        return 1.0
    V = o["values"]
    if len(V) != S:
        return 1.0
    g = float(g)
    sc = max([1.0] + [abs(v) for v in V])
    Qv = [[float(Rf[s][a]) + g * math.fsum(float(P[s * A + a][t]) * V[t] for t in range(S)) for a in range(A)] for s in range(S)]
    bell = max(abs(V[s] - max(Qv[s])) for s in range(S)) / sc                  # the Bellman optimality equation
    pol = max(max(Qv[s]) - Qv[s][int(o["policy"][s]) - 1] for s in range(S)) / sc
    return max(bell, pol, _rel(o["value_state_1"], V[0]))


def _ref_mdp(R, T, gamma):
    """INDEPENDENT reference: the primal LP of a discounted MDP (min sum V s.t. V_s >= R(s,a) + gamma P V) by scipy
    linprog (HiGHS)."""
    import numpy as np
    from scipy.optimize import linprog
    S, A = len(R), len(R[0])
    Tm = np.array(T, float)
    Tm = Tm / Tm.sum(axis=1, keepdims=True)
    A_ub, b_ub = [], []
    for s in range(S):
        for a in range(A):
            row = gamma * Tm[s * A + a].copy()
            row[s] -= 1.0
            A_ub.append(row)
            b_ub.append(-float(R[s][a]))
    res = linprog(np.ones(S), A_ub=np.array(A_ub), b_ub=np.array(b_ub), bounds=[(None, None)] * S, method="highs")
    return {"value_state_1": float(res.x[0]), "values": [float(v) for v in res.x]}


def _random_mdp(rng):
    S, A = rng.randint(2, 5), rng.randint(2, 3)
    T = []
    for _ in range(S * A):
        w = [rng.randint(0, 4) for _ in range(S)]
        if sum(w) == 0:
            w[rng.randrange(S)] = 1
        tot = sum(w)
        T.append([v / tot for v in w])
    return {"R": [[rng.randint(-5, 10) for _ in range(A)] for _ in range(S)], "T": T, "gamma": rng.choice([0.5, 0.8, 0.9, 0.95])}


_add("mdp_discounted",
     name="Discounted Markov decision process (optimal values and policy)", topic="dynamic programming / Markov decision processes",
     purpose="The optimal expected discounted reward from every state of a finite Markov decision process and an "
             "optimal stationary policy.",
     inputs=[_I("R", "rewards, rows = states, columns = actions", "matrix"),
             _I("T", "transition probabilities: one row per (state, action) in the order s1a1, s1a2, ..., s2a1, ...; columns = next state", "matrix"),
             _I("gamma", "discount factor (0 < gamma < 1)")],
     outputs=[_O("value_state_1", "optimal value of state 1"), _O("values", "optimal value of every state"),
              _O("policy", "optimal action in each state, 1-based (the lowest-numbered one on a tie)")],
     primary="value_state_1", conditions=["0 < gamma < 1", "rows of T sum to 1", "at most 10 states and 5 actions"],
     method="policy iteration in exact arithmetic; certified by the Bellman optimality equation",
     latex=r"V^*(s) = \max_a \Big( R(s,a) + \gamma \sum_{s'} P(s' \mid s,a)\, V^*(s') \Big)",
     run=mdp_discounted, verify=_verify_mdp, reference=_ref_mdp,
     parse=lambda q: (lambda ql: None if not re.search(r"\b(markov decision|mdp)\b", ql) else (
         (_labelled(q, {"R": r"rewards?(?: matrix)?", "T": r"transitions?(?: matrix| probabilities)?"}) or {})
         | ({"gamma": v} if (v := _num_after(ql, r"discount(?: factor| rate)?")) is not None else {}) or None))(q.lower()),
     wrong=lambda R, T, gamma: (lambda V: {"value_state_1": V[0], "values": V, "policy": [1.0] * len(R)})([max(float(v) for v in r) / (1 - float(gamma)) for r in R]),
     random_inputs=_random_mdp,
     keywords=["markov decision process", "policy iteration", "bellman equation"],
     templates=["Solve the discounted Markov decision process with rewards [[1, 0], [0, 2]], transitions [[1, 0], [0, 1], [0, 1], [1, 0]] and discount factor 0.9."],
     examples=[{"inputs": {"R": [[1, 0], [0, 2]], "T": [[1, 0], [0, 1], [0, 1], [1, 0]], "gamma": 0.9},
                "outputs": {"values": [10.0, 11.0], "value_state_1": 10.0, "policy": [1.0, 2.0]}}],
     questions=[{"q": "Solve the discounted Markov decision process with rewards [[1, 0], [0, 2]], transitions [[1, 0], [0, 1], [0, 1], [1, 0]] and discount factor 0.9.",
                 "outputs": {"value_state_1": 10.0}},
                {"q": "Solve the discounted Markov decision process with rewards [[1, 0], [0, 2]], transitions [[1, 0], [0, 1], [0, 1], [1, 0]] and discount factor 1.",
                 "outcome": "INVALID_INPUT"}])


# =================================================================================================================
# combinatorial: graph colouring and vertex cover (exact, small graphs)
# =================================================================================================================
def _simple_graph(edges, max_nodes):
    E = _wedges(edges, need_weight=False)
    if any(u == v for u, v in E):
        raise ValueError("a node cannot be joined to itself")
    nodes = _nodes(E)
    if len(nodes) > max_nodes:
        raise ValueError(f"at most {max_nodes} nodes")
    adj = {v: set() for v in nodes}
    for u, v in E:
        adj[u].add(v)
        adj[v].add(u)
    return nodes, adj


def chromatic(edges):
    nodes, adj = _simple_graph(edges, 14)
    order = sorted(nodes, key=lambda v: (-len(adj[v]), nodes.index(v)))
    for k in range(1, len(nodes) + 1):
        col = {}

        def bt(t):
            if t == len(order):
                return True
            v = order[t]
            used = {col[u] for u in adj[v] if u in col}
            for cc in range(1, min(k, max(col.values(), default=0) + 1) + 1):   # symmetric colours skipped
                if cc not in used:
                    col[v] = cc
                    if bt(t + 1):
                        return True
                    del col[v]
            return False
        if bt(0):
            return {"chromatic_number": float(k), "coloring_example": [float(col[v]) for v in nodes], "nodes": nodes}
    raise ValueError("no colouring found")


def _verify_col(i, o):
    nodes, adj = _simple_graph(i["edges"], 14)
    cl = o["coloring_example"]
    if o.get("nodes") != nodes or len(cl) != len(nodes):
        return 1.0
    c = dict(zip(nodes, cl))
    if any(c[u] == c[v] for u in nodes for v in adj[u]) or len(set(cl)) != o["chromatic_number"]:
        return 1.0
    return _rel(o["chromatic_number"], chromatic(i["edges"])["chromatic_number"])


def _ref_col(edges):
    """INDEPENDENT reference: the colouring MILP (x_vc node v gets colour c, w_c colour used) by scipy milp."""
    nodes = _nodes([(str(u), str(v)) for u, v in edges])
    n = len(nodes)
    ix = {v: k for k, v in enumerate(nodes)}
    nv = n * n + n
    c = [0.0] * (n * n) + [1.0] * n
    rows, lo, hi = [], [], []
    for v in range(n):
        r = [0.0] * nv
        for cc in range(n):
            r[v * n + cc] = 1.0
        rows.append(r); lo.append(1.0); hi.append(1.0)
        for cc in range(n):
            r = [0.0] * nv
            r[v * n + cc], r[n * n + cc] = 1.0, -1.0
            rows.append(r); lo.append(-float("inf")); hi.append(0.0)
    for u, v in edges:
        for cc in range(n):
            r = [0.0] * nv
            r[ix[str(u)] * n + cc] += 1.0
            r[ix[str(v)] * n + cc] += 1.0
            rows.append(r); lo.append(-float("inf")); hi.append(1.0)
    return {"chromatic_number": _milp_ref(c, rows, lo, hi, [1.0] * nv)}


def _random_simple(rng, lo_n, hi_n, p_lo, p_hi):
    n = rng.randint(lo_n, hi_n)
    p = rng.uniform(p_lo, p_hi)
    names = [f"V{k}" for k in range(1, n + 1)]
    E = [(names[a], names[b]) for a in range(n) for b in range(a + 1, n) if rng.random() < p]
    return {"edges": E or [(names[0], names[1])]}


_add("graph_coloring",
     name="Chromatic number (minimum graph colouring)", topic="integer programming / graph colouring",
     purpose="The fewest colours needed so that adjacent nodes get different colours (scheduling exams, assigning "
             "frequencies), and one such colouring.",
     inputs=[_I("edges", "edges 'A-B, B-C, ...' (undirected)", "edges", min_length=1, max_length=120)],
     outputs=[_O("chromatic_number", "minimum number of colours"), _O("coloring_example", "one optimal colouring, in the order of 'nodes'"),
              _O("nodes", "node order (first appearance)", "text")],
     primary="chromatic_number", conditions=["at most 14 nodes", "no self-loops"],
     method="exact backtracking for k = 1, 2, ... colours (largest degree first, symmetric colours skipped)",
     latex=r"\chi(G) = \min\{k : \exists\, c: V \to \{1..k\},\ c(u) \neq c(v)\ \forall uv \in E\}",
     run=chromatic, verify=_verify_col, reference=_ref_col,
     parse=lambda q: ({"edges": et} if (et := _edges_text(q)) else {}) or None
     if re.search(r"\b(chromatic number|graph colou?ring|minimum colou?ring|fewest colou?rs)\b", q.lower()) else None,
     wrong=lambda edges: (lambda nodes, adj: {"chromatic_number": float(max(len(a) for a in adj.values()) + 1), "coloring_example": [], "nodes": nodes})(*_simple_graph(edges, 14)),   # max degree + 1
     random_inputs=lambda rng: _random_simple(rng, 5, 10, 0.25, 0.6),
     keywords=["graph coloring", "chromatic number"],
     templates=["Find the chromatic number of the graph with edges A-B, B-C, C-A, C-D."],
     examples=[{"inputs": {"edges": "A-B, B-C, C-A, C-D"}, "outputs": {"chromatic_number": 3.0}},
               {"inputs": {"edges": "A-B, B-C, C-D, D-A"}, "outputs": {"chromatic_number": 2.0}}],
     questions=[{"q": "Find the chromatic number of the graph with edges A-B, B-C, C-A, C-D.", "outputs": {"chromatic_number": 3.0}},
                {"q": "Find the chromatic number of the graph with edges A-A, A-B.", "outcome": "INVALID_INPUT"}])


def vertex_cover(edges):
    nodes, adj = _simple_graph(edges, 30)
    best = [set(nodes)]

    def rec(cover, rem):
        if len(cover) >= len(best[0]):
            return
        live = {v: {u for u in rem[v]} for v in rem if rem[v]}
        if not live:
            best[0] = set(cover)
            return
        v = max(live, key=lambda x: (len(live[x]), -nodes.index(x)))
        if len(live[v]) == 1:                                # a leaf edge: taking the neighbour is never worse
            u = next(iter(live[v]))
            take = [u]
        else:
            take = None
        for choice in ([take] if take else [[v], sorted(live[v], key=nodes.index)]):
            r2 = {x: set(s) for x, s in live.items()}
            for w in choice:
                for x in r2.pop(w, set()):
                    r2.get(x, set()).discard(w)
            rec(cover | set(choice), r2)
    rec(set(), {v: set(adj[v]) for v in nodes})
    cov = [v for v in nodes if v in best[0]]
    return {"min_cover_size": float(len(cov)), "cover_example": cov, "max_independent_set_size": float(len(nodes) - len(cov))}


def _verify_vc(i, o):
    nodes, adj = _simple_graph(i["edges"], 30)
    cov = set(o["cover_example"])
    if not cov <= set(nodes) or any(u not in cov and v not in cov for u in nodes for v in adj[u]) or len(cov) != o["min_cover_size"]:
        return 1.0
    return max(_rel(o["min_cover_size"], vertex_cover(i["edges"])["min_cover_size"]), _rel(o["max_independent_set_size"], len(nodes) - len(cov)))


def _ref_vc(edges):
    """INDEPENDENT reference: the vertex-cover MILP (x_u + x_v >= 1 per edge) by scipy milp."""
    nodes = _nodes([(str(u), str(v)) for u, v in edges])
    ix = {v: k for k, v in enumerate(nodes)}
    rows = []
    for u, v in edges:
        r = [0.0] * len(nodes)
        r[ix[str(u)]] += 1.0
        r[ix[str(v)]] += 1.0
        rows.append(r)
    k = _milp_ref([1.0] * len(nodes), rows, [1.0] * len(rows), [float("inf")] * len(rows), [1.0] * len(nodes))
    return {"min_cover_size": k, "max_independent_set_size": len(nodes) - k}


def _wrong_vc(edges):                                     # both ends of a greedy maximal matching (the 2-approximation)
    nodes, adj = _simple_graph(edges, 30)
    cov = set()
    for u, v in _wedges(edges, need_weight=False):
        if u not in cov and v not in cov:
            cov |= {u, v}
    return {"min_cover_size": float(len(cov)), "cover_example": [v for v in nodes if v in cov], "max_independent_set_size": float(len(nodes) - len(cov))}


_add("minimum_vertex_cover",
     name="Minimum vertex cover (and maximum independent set)", topic="integer programming / covering",
     purpose="The fewest nodes that touch every edge of a graph (placing guards or sensors), one such set, and the "
             "size of a largest set of pairwise non-adjacent nodes.",
     inputs=[_I("edges", "edges 'A-B, B-C, ...' (undirected)", "edges", min_length=1, max_length=200)],
     outputs=[_O("min_cover_size", "size of a minimum vertex cover"), _O("cover_example", "one minimum vertex cover", "text"),
              _O("max_independent_set_size", "size of a maximum independent set (nodes minus cover)")],
     primary="min_cover_size", conditions=["at most 30 nodes", "no self-loops", "only nodes that appear in an edge count"],
     method="exact branch and bound (a node of largest degree in the cover, or all its neighbours)",
     latex=r"\tau(G) = \min\{|C| : C \cap \{u, v\} \neq \emptyset\ \forall uv \in E\},\quad \alpha(G) = |V| - \tau(G)",
     run=vertex_cover, verify=_verify_vc, reference=_ref_vc,
     parse=lambda q: ({"edges": et} if (et := _edges_text(q)) else {}) or None
     if re.search(r"\b(vertex cover|node cover|independent set)\b", q.lower()) else None,
     wrong=_wrong_vc,
     random_inputs=lambda rng: _random_simple(rng, 6, 14, 0.15, 0.45),
     keywords=["vertex cover", "independent set"],
     templates=["Find the minimum vertex cover of the graph with edges A-B, A-C, A-D, B-C, D-E."],
     examples=[{"inputs": {"edges": "A-B, A-C, A-D, B-C, D-E"}, "outputs": {"min_cover_size": 3.0, "max_independent_set_size": 2.0}},
               {"inputs": {"edges": "A-B, A-C, A-D"}, "outputs": {"min_cover_size": 1.0, "cover_example": ["A"]}}],
     questions=[{"q": "Find the minimum vertex cover of the graph with edges A-B, A-C, A-D, B-C, D-E.", "outputs": {"min_cover_size": 3.0}},
                {"q": "Find the minimum vertex cover of the graph with edges A-B, B-B.", "outcome": "INVALID_INPUT"}])


# ==================================================================================================================
# the reading guard: every parser above is wrapped by the same rules (ask_sweep 2026-10-23: 8 wrong answers and 2
# exceptions, among them a finite-buffer queue read as an unlimited M/M/c, a price schedule read as a plain EOQ, a
# savings matrix read as a cost matrix, an unread number of repairers defaulting to 1, and a one-row constraint
# matrix written [-2 3 -1] that crashed the input reader)
#   1. every stated number is bound: a reading that leaves a number of the question unused is refused. Numbers that
#      are part of a name or a notation (M/M/1, 0-1 knapsack, x0, p1, a12, an exponent ^2, "machine 1", "state 2",
#      "2x2") are not stated values; a percentage is its fraction ("20%" = 0.2);
#   2. wording that names a different capability's task is refused (`_O_FOREIGN`): an M/M/c reading does not read a
#      buffer or a capacity limit, an EOQ reading does not read a price schedule, a cost assignment does not read
#      savings or profits;
#   3. a flat list given for a table input is one row ("[-2 3 -1]" is a 1 x 3 matrix), and a single number given for
#      a list input is a list of one, so the shared input reader gets the shapes it expects; an exception inside a
#      parser is refused.
# ==================================================================================================================
_O_UNUM = r"(?:\d+(?:\.\d*)?|\.\d+)(?:e[-+]?\d+)?"
_O_QNUM = re.compile(r"(?<![\w.^/])(" + _O_UNUM + r")(\s*%|\s*percent\b|\s*per cent\b)?(?![a-z_\d^])")
_O_NOTATION = re.compile(
    r"\bm\s*/\s*[mgd]\s*/\s*1\b(?!\s*/)|\b0\s*[-/]\s*1\b(?=\s*(?:knapsack|integer|binary|programm|variables?|decision))|"
    r"\b\d+\s*[x×]\s*\d+(?:\s*[x×]\s*\d+)*\b|\b\d+\s*by\s*\d+\b|\^\s*\(?\s*-?\d+|"
    r"\b(?:machines?|states?|actions?|jobs?|nodes?|workers?|tasks?|rows?|columns?|players?|items?|activit(?:y|ies)|"
    r"stages?|periods?|tiers?|alternatives?|criteri(?:on|a)|facilit(?:y|ies)|cit(?:y|ies)|plants?|warehouses?|"
    r"steps?|iterations?|phases?|constraints?|products?|resources?|sources?|sinks?|projects?|options?|strateg(?:y|ies)|"
    r"suppliers?|customers? (?=\d+\s*(?:and|,|to|\)|$)))\s*#?\s*\d+(?:\s*(?:,|and|or|to)\s*\d+)*\b(?!\s*(?:=|:|units?|hours?|"
    r"minutes?|days?|weeks?|per\b|%))|\bphase (?:1|2|one|two)\b|\b(?:1st|2nd|3rd|\d+th)\b|"
    # the nonnegativity of the variables ("x, y >= 0"), the standing convention of a linear program
    r"(?<![\w.])(?:[a-z]\w{0,3}\s*,\s*)*[a-z]\w{0,3}\s*(?:>=|≥)\s*0(?!\.?\d)(?=\s*(?:,|;|\.|and\b|\?|$))")
_O_FOREIGN = {
    "mmc_queue": r"\bbuffer|\brestrict|\blimit|\bmaximum number|\bat most\b|\bcapacity|\bwaiting room|\bfinite|"
                 r"\bblock|\bturned away|\bbalk|\blost\b|\bno more than\b|\bup to \d+ customers",
    "mm1_queue": r"\bbuffer|\brestrict|\blimit|\bmaximum number|\bat most\b|\bcapacity|\bwaiting room|\bfinite|"
                 r"\bblock|\bturned away|\bbalk|\blost\b|\bno more than\b|\bservers\b|\bmachines?\b|\brepair",
    "eoq": r"\bprices?\b|\bschedule\b|\btiers?\b|\bunit costs?\b|\bdiscount",
    "assignment_min_cost": r"\bsavings?\b|\bprofits?\b|\bvalues?\b|\bmaximi[sz]|\bmaximum\b|\brevenues?\b|\bbenefits?\b|"
                           r"\bgains?\b|\bscores?\b|\bpreferences?\b|\bratings?\b|\bproductivity|\befficienc",
    "assignment_max_profit": r"\bminimi[sz]|\bminimum (?:total )?cost\b",
}
_O_FOREIGN_RE = {k: re.compile(v) for k, v in _O_FOREIGN.items()}


def _o_stated(q: str) -> list:
    q = _O_NOTATION.sub(lambda m: " " * len(m.group(0)), q)
    out = []
    for m in _O_QNUM.finditer(q):
        v = float(m.group(1))
        out.append((v, bool(m.group(2))))
    return out


def _o_bound(parsed) -> list:
    nums = []

    def walk(x):
        if isinstance(x, bool) or x is None:
            return
        if isinstance(x, (int, float)):
            nums.append(abs(float(x)))
        elif isinstance(x, F):
            nums.append(abs(float(x)))
        elif isinstance(x, str):
            nums.extend(abs(float(t)) for t in re.findall(_O_UNUM, x.lower()))
        elif isinstance(x, dict):
            for v in x.values():
                walk(v)
        elif isinstance(x, (list, tuple)):
            for v in x:
                walk(v)
    walk(parsed)
    return nums


def _o_same(a, b):
    return abs(a - b) <= 1e-12 * max(1.0, abs(a), abs(b))


def _o_shapes(spec, p):
    """Rule 3: a flat list for a table input is one row; a single number for a list input is a list of one."""
    types = {i["symbol"]: i.get("type", "number") for i in spec["inputs"]}
    out = dict(p)
    for k, v in p.items():
        if types.get(k) == "matrix" and isinstance(v, list) and v and all(
                isinstance(x, (int, float)) and not isinstance(x, bool) for x in v):
            out[k] = [list(v)]
        elif types.get(k) == "vector" and isinstance(v, (int, float)) and not isinstance(v, bool):
            out[k] = [v]
    return out


def _o_guarded(key, spec, parse):
    foreign = _O_FOREIGN_RE.get(key)

    def guarded(question):
        if not isinstance(question, str):
            return None
        q = question.lower()
        if foreign is not None and foreign.search(q):        # rule 2: the question asks for another task
            return None
        try:
            p = parse(question)
        except (ValueError, TypeError, ZeroDivisionError, OverflowError, IndexError, KeyError, ArithmeticError):
            return None
        if not p or "_out_of_scope" in p:
            return p
        bound = _o_bound(p)                                  # rule 1: every stated number is bound
        for v, pct in _o_stated(q):
            if any(_o_same(v, b) for b in bound):
                continue
            if pct and any(_o_same(v / 100.0, b) for b in bound):
                continue                                     # "20%", "a 95 percent service level"
            if 1.0 < v <= 100.0 and any(_o_same(v / 100.0, b) for b in bound if 0.0 < b < 1.0):
                continue                                     # "a service level of 95" read as 0.95
            return None
        return _o_shapes(spec, p)
    return guarded


for _k, _spec in PROCEDURES.items():
    _spec["parse"] = _o_guarded(_k, _spec, _spec["parse"])
