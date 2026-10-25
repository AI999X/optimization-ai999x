# GENERATED from ai999x_factory/runtime by tools/build_package.py — do not edit.
"""Structured inputs of a procedure: not only numbers, but lists, matrices, graphs, whole numbers, text and choices.

A procedure input declares `type` (default "number", read as before: a pure number, or a number with a unit of a
pack kind). The other types:

  integer   a whole number                         7, "7"
  vector    a list of numbers                      [1, 2.5, 3], "1, 2.5, 3", "1 2.5 3", "[1, 2.5, 3]"
  matrix    a rectangular list of rows             [[1, 2], [3, 4]], "1 2; 3 4", "[[1, 2], [3, 4]]"
  edges     a graph as edges (u, v) or (u, v, w)   [("A", "B", 4), ("B", "C", 1)], "A-B 4, B-C 1", "A B 4; B C 1"
  text      a string                               "ACGTTGCA"
  choice    one of the input's `choices`           "two-sided"
  boolean   true or false                          True, "yes", "false"

Optional limits on an input: `min_length`, `max_length` (vector, text, edges), `min`, `max` (integer, number).
A value that does not fit raises InputError with a message saying what was expected; the procedure path turns that
into INVALID_INPUT. Nothing here evaluates text: numbers are read with float(), never eval().
"""
from __future__ import annotations

import re

STRUCTURED = ("integer", "vector", "matrix", "edges", "text", "choice", "boolean")


class InputError(ValueError):
    pass


def _number(x, name):
    if isinstance(x, bool):
        raise InputError(f"{name}: expected a number, got {x!r}")
    try:
        v = float(str(x).replace("−", "-").replace(",", "").strip()) if isinstance(x, str) else float(x)
    except (TypeError, ValueError):
        raise InputError(f"{name}: {x!r} is not a number") from None
    if v != v or v in (float("inf"), float("-inf")):
        raise InputError(f"{name}: {x!r} is not a finite number")
    return v


def _split_list(text: str) -> list:
    t = text.strip()
    if t.startswith("[") and t.endswith("]"):
        t = t[1:-1]
    parts = [p for p in re.split(r"[,\s;]+", t) if p]
    return parts


def _length(name, n, spec):
    lo, hi = spec.get("min_length"), spec.get("max_length")
    if lo is not None and n < lo:
        raise InputError(f"{name}: needs at least {lo} entries, got {n}")
    if hi is not None and n > hi:
        raise InputError(f"{name}: allows at most {hi} entries, got {n}")


def _range(name, v, spec):
    if spec.get("min") is not None and v < spec["min"]:
        raise InputError(f"{name}: must be at least {spec['min']}, got {v:g}")
    if spec.get("max") is not None and v > spec["max"]:
        raise InputError(f"{name}: must be at most {spec['max']}, got {v:g}")


def coerce(spec: dict, value):
    """The input's value in the form the procedure receives, or InputError."""
    t, name = spec.get("type", "number"), spec["symbol"]
    if t == "integer":
        # read exactly: through float, a whole number above 2**53 would be rounded without a word (discrete_math)
        if isinstance(value, bool):
            raise InputError(f"{name}: expected a whole number, got {value!r}")
        if isinstance(value, int):
            v = value
        elif isinstance(value, str) and re.fullmatch(r"\s*[-+−]?\d[\d,_]*\s*", value):
            v = int(value.strip().replace("−", "-").replace(",", "").replace("_", ""))
        else:
            f = _number(value, name)
            if f != int(f) or abs(f) > 2 ** 53:
                raise InputError(f"{name}: expected a whole number written exactly, got {value!r}")
            v = int(f)
        _range(name, v, spec)
        return v
    if t == "vector":
        items = _split_list(value) if isinstance(value, str) else list(value)
        out = [_number(x, name) for x in items]
        _length(name, len(out), spec)
        return out
    if t == "matrix":
        if isinstance(value, str):
            s = value.strip()
            if s.startswith("[["):
                rows = [r for r in re.split(r"\]\s*,?\s*\[", s.strip("[]")) if r.strip()]
            else:
                rows = [r for r in s.split(";") if r.strip()]
            rows = [_split_list(r) for r in rows]
        else:
            rows = [list(r) for r in value]
        out = [[_number(x, name) for x in r] for r in rows]
        if not out or not out[0] or any(len(r) != len(out[0]) for r in out):
            raise InputError(f"{name}: expected a rectangular matrix (rows of equal length)")
        return out
    if t == "edges":
        if isinstance(value, str):
            items = []
            for part in re.split(r"[;,]\s*", value.strip()):
                if not part.strip():
                    continue
                # one edge: "A-B", "A-B -3", "A B" or "A B -3". The dash between names is a separator only there;
                # a weight keeps its sign ("B-A -3" is weight -3, never +3: optimization, 2026-10-22)
                m = (re.fullmatch(r"\s*([^\s\-–]+)\s*[-–]\s*([^\s\-–]+)(?:\s+([-+−]?[\d.]+(?:[eE][-+]?\d+)?))?\s*", part)
                     or re.fullmatch(r"\s*(\S+)\s+(\S+)(?:\s+([-+−]?[\d.]+(?:[eE][-+]?\d+)?))?\s*", part))
                if not m:
                    raise InputError(f"{name}: cannot read the edge {part.strip()!r}; write 'A-B 4' or 'A B 4'")
                items.append([g for g in m.groups() if g is not None])
        else:
            items = [list(e) for e in value]
        out = []
        for e in items:
            if len(e) not in (2, 3):
                raise InputError(f"{name}: each edge is (u, v) or (u, v, weight); got {e!r}")
            u, v = str(e[0]).strip(), str(e[1]).strip()
            if not u or not v:
                raise InputError(f"{name}: an edge needs two node names; got {e!r}")
            out.append((u, v, _number(e[2], name)) if len(e) == 3 else (u, v))
        if len({len(e) for e in out}) > 1:
            raise InputError(f"{name}: give a weight for every edge or for none")
        _length(name, len(out), spec)
        return out
    if t == "text":
        if not isinstance(value, str):
            raise InputError(f"{name}: expected text, got {value!r}")
        _length(name, len(value), spec)
        return value
    if t == "choice":
        v = str(value).strip().lower()
        choices = [str(c).lower() for c in spec.get("choices", [])]
        if v not in choices:
            raise InputError(f"{name}: one of {', '.join(spec.get('choices', []))}; got {value!r}")
        return spec["choices"][choices.index(v)]
    if t == "boolean":
        if isinstance(value, bool):
            return value
        v = str(value).strip().lower()
        if v in ("true", "yes", "1"):
            return True
        if v in ("false", "no", "0"):
            return False
        raise InputError(f"{name}: expected true or false, got {value!r}")
    raise InputError(f"{name}: unknown input type {t!r}")


def shown(value) -> str:
    """A short display form of a structured value (inputs echoed in an answer)."""
    if isinstance(value, list) and value and isinstance(value[0], tuple):
        return ", ".join("-".join(str(x) for x in e[:2]) + (f" ({e[2]:g})" if len(e) == 3 else "") for e in value)
    if isinstance(value, list) and value and isinstance(value[0], list):
        return "[" + "; ".join(" ".join(f"{x:g}" for x in r) for r in value) + "]"
    if isinstance(value, list):
        return "[" + ", ".join(f"{x:g}" for x in value) + "]"
    return str(value)
