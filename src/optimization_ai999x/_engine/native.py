# GENERATED from ai999x_factory/runtime by tools/build_package.py — do not edit.
"""The Rust engine under a model's Python package (docs/PYTHON_ON_RUST.md).

A platform wheel of a model carries its Rust engine as one native library next to the package (`_ax_native.so` on
Linux, `_ax_native.dll` on Windows; rust/python, built per model). The package loads it with ctypes and sends JSON, the
same JSON the website's browser module and the desktop app send, so `solve()`, `ask()` and `figures()` give the
answers of the command line, the website and the desktop app. The pure-Python wheel (every other platform) has no
library and computes with the Python engine, as before.

    AI999X_ENGINE=python      use the Python engine even where the library is present
    AI999X_ENGINE=rust        require the library (ImportError when it is absent)
    AI999X_NATIVE_LIB=<path>  load this library instead (development: the factory tree has none)

Nothing here imports the production side or touches the network.
"""
from __future__ import annotations

import ctypes
import json
import os
import re
import sys
from pathlib import Path

ABI = 1
OPS = {"solve": 1, "figures": 2, "ask": 3, "search": 4, "topics": 5, "catalog": 6, "about": 7}
LIB_NAMES = ("_ax_native.dll",) if sys.platform == "win32" else ("_ax_native.so", "_ax_native.dylib")

OUTCOME_STATUS = {"SOLVED": "solved", "NEEDS_INPUT": "needs_clarification", "AMBIGUOUS": "needs_clarification",
                  "UNRECOGNIZED_FORMAT": "needs_clarification", "INVALID_INPUT": "invalid",
                  "OUT_OF_SCOPE": "unsupported", "INTERNAL_ERROR": "internal_error"}
_STATE = {"NEEDS_INPUT": "missing_information", "AMBIGUOUS": "ambiguous", "UNRECOGNIZED_FORMAT": "needs_clarification",
          "INVALID_INPUT": "invalid", "OUT_OF_SCOPE": "unsupported", "INTERNAL_ERROR": "internal_error"}
_CAUSE = {"NEEDS_INPUT": "needs_input", "AMBIGUOUS": "ambiguous", "UNRECOGNIZED_FORMAT": "unrecognized_format",
          "INVALID_INPUT": "invalid_input", "OUT_OF_SCOPE": "unsupported_structure", "INTERNAL_ERROR": "internal_error"}
_NUMBER_UNIT = re.compile(r"\s*([-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?)\s*(.*)$")


class NativeUnavailable(ImportError):
    """This installation has no Rust engine (the pure-Python wheel, or AI999X_ENGINE=python)."""


class NativeEngine:
    """One model's Rust library: JSON text in, JSON text out (rust/python/src/lib.rs)."""

    def __init__(self, path: Path):
        self.path = Path(path)
        lib = ctypes.CDLL(str(self.path))
        lib.ax_abi.restype = ctypes.c_uint32
        lib.ax_abi.argtypes = []
        lib.ax_call.restype = ctypes.c_void_p
        lib.ax_call.argtypes = [ctypes.c_uint32, ctypes.c_char_p, ctypes.c_size_t, ctypes.POINTER(ctypes.c_size_t)]
        lib.ax_release.restype = None
        lib.ax_release.argtypes = [ctypes.c_void_p, ctypes.c_size_t]
        abi = lib.ax_abi()
        if abi != ABI:
            raise NativeUnavailable(f"{self.path.name} speaks interface {abi}, this package {ABI}")
        self._lib = lib
        self.about = self.call_json("about")

    def call(self, op: str, text: str = "") -> str:
        data = text.encode("utf-8")
        n = ctypes.c_size_t(0)
        ptr = self._lib.ax_call(OPS[op], data, len(data), ctypes.byref(n))
        try:
            return ctypes.string_at(ptr, n.value).decode("utf-8")
        finally:
            self._lib.ax_release(ptr, n.value)

    def call_json(self, op: str, payload=None):
        text = payload if isinstance(payload, str) else ("" if payload is None else json.dumps(payload, ensure_ascii=False))
        return json.loads(self.call(op, text))


def load(package_dir: Path | None) -> NativeEngine | None:
    """The library beside the package (or AI999X_NATIVE_LIB), or None; AI999X_ENGINE chooses (module docstring)."""
    choice = os.environ.get("AI999X_ENGINE", "").strip().lower()
    if choice == "python":
        return None
    override = os.environ.get("AI999X_NATIVE_LIB", "").strip()
    cands = [Path(override)] if override else [Path(package_dir) / n for n in LIB_NAMES] if package_dir else []
    path = next((c for c in cands if c.is_file()), None)
    if path is None:
        if choice == "rust":
            raise NativeUnavailable("AI999X_ENGINE=rust, but this installation has no Rust engine "
                                    "(platform wheels: Windows x86_64 and Linux x86_64)")
        return None
    try:
        return NativeEngine(path)
    except (OSError, NativeUnavailable, ValueError, AttributeError):
        if choice == "rust":
            raise
        return None                                    # an unloadable library: the Python engine answers, as before


# --- what is sent -----------------------------------------------------------------------------------------------
def input_json(v):
    """A Python input value as the engine reads it: text as written, a number, a list (vector, matrix), true/false."""
    if isinstance(v, (bool, int, float, str)) or v is None:
        return v
    if isinstance(v, tuple) and len(v) == 2 and isinstance(v[0], (int, float)) and isinstance(v[1], str):
        return f"{v[0]!r} {v[1]}".strip()                # (3, "kg"): a number and its unit, as solve() documents
    if isinstance(v, complex):
        return f"{v.real!r}{v.imag:+}j"
    if hasattr(v, "tolist"):                             # a numpy array or number
        return input_json(v.tolist())
    if isinstance(v, (list, tuple)):
        return [input_json(x) for x in v]
    if isinstance(v, dict):
        return {str(k): input_json(x) for k, x in v.items()}
    return str(v)                                        # a Fraction, a Decimal: as written


def request(key: str, given: dict, target: str | None = None) -> dict:
    return {"key": key, "inputs": {str(k): input_json(v) for k, v in given.items()}, "target": target}


# --- answers ------------------------------------------------------------------------------------------------------
def _value(v):
    """The engine's JSON value as the Python engine gives it: {"re", "im"} is a complex number (also inside lists)."""
    if isinstance(v, dict) and set(v) == {"re", "im"}:
        return complex(v["re"], v["im"])
    if isinstance(v, list):
        return [_value(x) for x in v]
    return v


def _shown(v):
    """An input as stated, for the steps: '10 kohm' -> (10.0, 'kohm'); a number -> (n, ''); a list as written."""
    if isinstance(v, str):
        m = _NUMBER_UNIT.match(v)
        if m and not re.search(r"[\[,;(]", m.group(2)):
            return float(m.group(1)), m.group(2).strip()
        return v, ""
    if isinstance(v, (int, float)) and not isinstance(v, bool):
        return float(v), ""
    return v, ""


def _needs(product, key: str, given: dict, target=None) -> list:
    """The inputs a NEEDS_INPUT answer lacks (the engine names them in its message; the package also lists them)."""
    p = product.pack()
    if key in p.procedures:
        return [i["symbol"] for i in p.procedures[key]["inputs"] if i.get("required") and i["symbol"] not in given]
    rel = p.by_key.get(key)
    if rel is None:
        return []
    stated = {product._symbol_of(rel, n) or n for n in given}
    unknown = [n for n, s in rel.symbols.items() if n not in stated and not s.default]
    lhs = rel.equation.split("=")[0].strip()
    if target is None:
        target = lhs if lhs in unknown else (unknown[0] if len(unknown) == 1 else None)
    return [n for n in unknown if n != target]


def to_answer(product, reply: dict, question: str, key: str | None, given: dict | None = None, extra: dict | None = None):
    """The package's Answer for an engine reply (solve_json's answer), with the steps the Python engine writes."""
    from .answer import Answer
    outcome = reply.get("outcome") or "INTERNAL_ERROR"
    status = OUTCOME_STATUS.get(outcome, "internal_error")
    a = Answer(status, question, message=reply.get("message") or "", models=[key] if key else [])
    a.value, a.unit = reply.get("value"), reply.get("unit")
    a.assumptions = list(reply.get("assumptions") or [])
    a.values = {s: _value(v) for s, v, _u in reply.get("values") or []}
    if outcome != "SOLVED":
        a.uncertainty = {"state": _STATE.get(outcome, "invalid"), "cause": _CAUSE.get(outcome, "internal_error"),
                         "outcome": outcome, "engine": "rust", **({"capability": key} if key else {}), **(extra or {})}
        if outcome == "NEEDS_INPUT" and key and "needs" not in a.uncertainty and given is not None:
            target = ((extra or {}).get("request") or {}).get("target")
            a.uncertainty["needs"] = _needs(product, key, given, target)   # as the Python engine lists them
        return a
    a.uncertainty = {"state": "determined", "capability": key, "engine": "rust", **(extra or {})}
    p = product.pack()
    units = {s: u for s, _v, u in reply.get("values") or []}
    given = given or {}
    if key in p.procedures:
        spec = p.procedures[key]
        out_kind = {o["symbol"]: o.get("kind", "number") for o in spec["outputs"]}
        meaning = {i["symbol"]: i.get("meaning", "") for i in spec["inputs"]}
        a.target = {"kind": out_kind.get(spec.get("primary"), "number"), "role": "procedure",
                    "phrase": reply.get("phrase") or spec["outputs"][0]["meaning"]}
        shown = {n: _shown(v) for n, v in given.items()}
        a.steps = [{"relation": key, "name": reply.get("name") or spec["name"], "equation": reply.get("equation") or spec["method"],
                    "equation_latex": spec.get("latex", ""), "solved_latex": ", ".join(a.values), "target": ", ".join(a.values),
                    "result": a.value if isinstance(a.value, (int, float)) else None, "unit": a.unit or "",
                    "outputs": dict(a.values), "output_units": units,
                    "inputs": {n: {"value": shown[n][0], "unit": shown[n][1], "source": "stated", "meaning": meaning.get(n, "")}
                               for n in given}}]
        scope = "the result substituted back into the defining equation"
    elif key in p.by_key:
        rel = p.by_key[key]
        tgt = reply.get("solved_for") or reply.get("target")
        sym = rel.symbols.get(tgt)
        kind = sym.kind if sym else "number"
        a.target = {"kind": kind, "role": sym.role if sym else "plain",
                    "phrase": reply.get("phrase") or (sym.desc if sym else "") or kind.replace("_", " ")}
        stated = set()
        for name in given:
            s = product._symbol_of(rel, name)
            if s:
                stated.add(s)
        inputs = {}
        for n, v in reply.get("si") or []:
            s = rel.symbols.get(n)
            src = "stated" if n in stated or s is None or not s.default else f"default: {s.default.get('assumption', '')}"
            inputs[n] = {"value": v, "unit": p.kinds[s.kind].unit if s else "", "source": src,
                         "meaning": (s.desc or s.kind) if s else ""}
        try:
            solved_latex = rel.latex_solved(tgt)
        except Exception:                               # display only: never a reason to lose the answer
            solved_latex = tgt or ""
        a.steps = [{"relation": key, "name": rel.name, "equation": rel.equation, "equation_latex": rel.latex(),
                    "solved_latex": solved_latex, "target": tgt, "result": a.value, "unit": a.unit or "", "inputs": inputs}]
        scope = "algebra and dimensions of the certified relation"
    else:
        scope = "the engine's own checks"
    a.verification = {"status": "verified", "independent_paths": 1, "scope": scope, "engine": "rust"}
    return a


def ask_answer(product, reply: dict, question: str):
    """The package's Answer for an ask() reply (ask.rs): the engine's answer when it computed one, the reader's
    outcome and message otherwise; the reading itself is in uncertainty["reading"]."""
    reading = {k: reply[k] for k in ("capability", "name", "kind", "inputs", "target", "request", "reading",
                                     "alternatives", "needs") if k in reply}
    key = reply.get("capability")
    extra = {"reading": reading}
    if reply.get("needs"):
        extra["needs"] = list(reply["needs"])
    if reply.get("alternatives"):
        extra["candidates"] = [x.get("capability") for x in reply["alternatives"]]
    ans = reply.get("answer")
    if isinstance(ans, dict):
        a = to_answer(product, ans, question, key, (reply.get("request") or {}).get("inputs") or {}, extra)
        if a.outcome != "SOLVED":
            a.message = reply.get("message") or a.message
        return a
    return to_answer(product, {"outcome": reply.get("outcome"), "message": reply.get("message"), "value": None,
                               "unit": None}, question, key, None, extra)
