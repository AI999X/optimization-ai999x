# GENERATED from ai999x_factory/runtime by tools/build_package.py — do not edit.
"""The optional decision model in SUGGEST MODE: which capability a question needs, and the inputs it reads — offered
to the user to confirm, never computed on (docs/ONNX_CONTRACT.md, docs/DECISION_MODEL_PLAN.md).

    pip install "algebra-ai999x[onnx]"                      # onnxruntime, tokenizers, numpy (no torch)
    hf download AI999X/algebra-ai999x --include "onnx/*" --local-dir algebra_model
    from algebra_ai999x import DecisionReader
    algebra.ask("checking my homework: 3x - 7 = 14, what is x?", reader=DecisionReader("algebra_model/onnx"))

Why suggest mode: measured end to end, the model picks the right capability (27 of 27 questions the rule-based reader
refuses) but copies a wrong input value in 5 of 27 (mostly a lost minus sign). The engine cannot detect a misread
value — it would solve the misread equation correctly — so the model never answers: it names the capability, shows
a pre-filled call with the values marked to check, and the outcome stays a refusal. It runs only when the rule-based
reader refuses, loads nothing unless asked, and makes no network call.
"""
from __future__ import annotations

import json
import re
from fractions import Fraction
from pathlib import Path


class DecisionReader:
    """A compact ONNX encoder (capability + input spans). `threshold` is the zero-wrong cut chosen on validation with
    this very file; below it, or for 'none', nothing is suggested."""

    def __init__(self, folder):
        import numpy as np
        import onnxruntime as ort
        from tokenizers import Tokenizer
        folder = Path(folder)
        self.labels = json.loads((folder / "labels.json").read_text(encoding="utf-8"))
        # a suggestion is checked by the user, so its bar is lower than the zero-wrong cut an automatic answer would
        # need (threshold_compact); the confidence is always shown
        self.threshold = float(self.labels.get("threshold_suggest", 0.5))
        # automatic answering only for a model that passed the end-to-end value check (decision-encoder-3: 0 wrong
        # values on held-out questions the reader refuses); None keeps the model in suggest mode
        self.threshold_auto = self.labels.get("threshold_auto")
        tok_file = folder / "tokenizer.json" if (folder / "tokenizer.json").exists() else folder / "tokenizer" / "tokenizer.json"
        self.tok = Tokenizer.from_file(str(tok_file))
        self.tok.no_padding()
        self.tok.enable_truncation(int(self.labels.get("max_len", 96)))
        model = folder / "decision.int8.onnx"
        self.session = ort.InferenceSession(str(model), providers=["CPUExecutionProvider"])
        self._np = np

    def prepare(self, text: str) -> str:
        """The text the model was trained on: normalize_notation(question) when its labels say so (v7+ datasets)."""
        if self.labels.get("text_normalization") == "notation":
            from .compiler import normalize_notation
            return normalize_notation(text)
        return text

    def read(self, text: str) -> dict | None:
        np = self._np
        text = self.prepare(text)
        e = self.tok.encode(text)
        ids = np.array([e.ids], dtype=np.int64)
        cap, _out, tags = self.session.run(None, {"input_ids": ids, "attention_mask": np.ones_like(ids)})
        p = np.exp(cap[0] - cap[0].max())
        p /= p.sum()
        caps, tagnames = self.labels["capability"], self.labels["tags"]
        best = caps[int(p.argmax())]
        if best == "none" or float(p.max()) < self.threshold:
            return None
        spans, cur = {}, None
        for j, (a, b) in enumerate(e.offsets):
            if a == b:
                continue
            t = tagnames[int(tags[0, j].argmax())]
            if t.startswith("B-") or (t.startswith("I-") and (cur is None or cur[0] != t[2:])):
                cur = [t[2:], a, b]
                spans.setdefault(cur[0], cur)
            elif t.startswith("I-") and cur is not None:
                cur[2] = b
            else:
                cur = None
        inputs, where = {}, {}
        for slot, (_s, a, b) in spans.items():
            span = text[a:b].replace("−", "-").replace(" ", "")      # signed spans: '- 5' is -5
            m = re.search(r"-?\d+(?:\.\d+)?(?:[eE][-+]?\d+)?(?:/\d+)?", span)     # 1.0e1 is 10, never 1.0
            if m:
                inputs[slot] = m.group(0)
                where[slot] = (a, b)
        return {"capability": best, "confidence": round(float(p.max()), 4), "inputs": inputs, "spans": where,
                "text": text}


_NUM = re.compile(r"(?<![\w.^])-?\d+(?:\.\d+)?(?:[eE][-+]?\d+)?(?![\w.]*\d)")      # e-notation is one number


def _all_numbers_used(question: str, inputs: dict) -> bool:
    """Every number written in the question is one of the read inputs (by value, sign-insensitive: a model that ignored
    a number must not answer)."""
    vals = [abs(float(v)) for v in inputs.values()]
    for m in _NUM.finditer(question.replace("−", "-")):
        x = abs(float(m.group(0)))
        if not any(abs(x - v) <= 1e-9 * max(1.0, v) for v in vals):
            return False
    return True


def _symbol_for(rel, kind: str, role: str | None):
    """The relation's symbol for a (kind, role): exact match, else the only symbol of that kind; None if ambiguous."""
    exact = [n for n, s in rel.symbols.items() if s.kind == kind and getattr(s, "role", None) == role]
    if len(exact) == 1:
        return exact[0]
    same = [n for n, s in rel.symbols.items() if s.kind == kind]
    return same[0] if len(same) == 1 and role in (None, "plain") else None


# direction and opposition words: a copied number carries no sign, so a question that sets directions against each other
# ("downward speed of 2.0 m/s ... upward acceleration") is never answered from the model's reading (it answered 8 m/s
# for -4 m/s on a held-out question before this check)
_DIRECTION = re.compile(r"\b(?:up(?:ward)?s?|down(?:ward)?s?|opposite|oppos\w+|against|backwards?|reverse\w*|negative|"
                        r"decelerat\w*|slow\w*|brak\w*|retard\w*|toward\w*|away|north\w*|south\w*|east\w*|west\w*|left|right|"
                        r"falls?|falling|dropp?\w*|rises?|rising|climb\w*|descend\w*|ascend\w*|sink\w*|lower\w*|rais\w*)\b")


def relation_reading(product, question: str, s: dict):
    """A physics-style reading the engine can compute and check, or None (then the model only suggests).
    Every condition is a reason the rule-based reader would also need: the asked quantity is READ FROM THE QUESTION by
    the rule-based compiler (never inferred from which symbol lacks a value); it is exactly one symbol of the named
    relation; every other symbol has a copied value with the unit written after it; every number in the question is
    used; no sign word ("decelerates", copied digits carry no sign); the relation's required context is in the wording
    and none of its excluded context."""
    from .compiler import _unit_after, compile_problem
    from .student_gate import text_tags
    pack = product.pack()
    rel = pack.by_key.get(s["capability"])
    if rel is None:
        return None
    text = s.get("text") or question
    prob = compile_problem(text, pack)
    if prob.target is None:
        return None
    target = _symbol_for(rel, prob.target.kind, prob.target.role)
    if target is None:
        return None
    low = text.lower()
    if any(w["phrase"] in low for w in (pack.terms or {}).get("sign", [])) or _DIRECTION.search(low):
        return None
    tags = text_tags(text, pack)
    if not set(rel.requires or ()) <= set(tags) or set(rel.excludes or ()) & set(tags):
        return None
    given = {}
    for slot, value in s["inputs"].items():
        kind, _, role = slot.partition("/")
        sym = _symbol_for(rel, kind, role or None)
        if sym is None or sym == target or sym in given:
            return None
        unit = _unit_after(text, s["spans"][slot][1])
        if unit is None:
            return None
        given[sym] = f"{value} {unit[1]}"
    if set(given) != set(rel.symbols) - {target} or not _all_numbers_used(text, s["inputs"]):
        return None
    return target, given


def suggest(product, question: str, answer, reader):
    """Help a refused answer (never a solved one). A model that passed the end-to-end check answers through the gate
    (confidence >= its zero-wrong cut, every required input read, every number used, engine-verified); otherwise the
    answer carries a suggestion to check. Returns the answer."""
    if reader is None or not isinstance(question, str) or answer.outcome == "SOLVED":
        return answer
    try:
        s = reader.read(question)
    except Exception as exc:                          # an optional helper must never break ask()
        answer.uncertainty = {**(answer.uncertainty or {}), "decision_model_error": str(exc)[:200]}
        return answer
    if not s or s["capability"] not in product.capability_ids():
        return answer
    procs = getattr(product.pack(), "procedures", {}) or {}
    auto = getattr(reader, "threshold_auto", None)
    if auto is not None and s["confidence"] >= float(auto) and s["capability"] in procs:
        need = [i["symbol"] for i in procs[s["capability"]]["inputs"] if i.get("required", True)]
        seen = reader.prepare(question) if hasattr(reader, "prepare") else question
        if all(k in s["inputs"] for k in need) and _all_numbers_used(seen, s["inputs"]):
            try:
                vals = {k: float(Fraction(v)) for k, v in s["inputs"].items()}
                solved = product.solve(s["capability"], **vals)
            except Exception:
                solved = None
            if solved is not None and solved.outcome == "SOLVED":
                solved.question = question
                solved.warnings = list(getattr(solved, "warnings", []) or []) + [
                    f"Read by the decision model (confidence {s['confidence']:.4f}): {s['capability']} with "
                    f"{vals}; computed and verified by the engine."]
                solved.uncertainty = {**(solved.uncertainty or {}), "reading": "decision_model",
                                      "decision_model": {"capability": s["capability"], "confidence": s["confidence"],
                                                         "inputs": s["inputs"]}}
                return solved
    rel_reading = relation_reading(product, question, s) if s["capability"] in product.pack().by_key else None
    if auto is not None and s["confidence"] >= float(auto) and rel_reading is not None:
        target, given = rel_reading
        try:
            solved = product.solve(s["capability"], target=target, **given)
        except Exception:
            solved = None
        if solved is not None and solved.outcome == "SOLVED":
            solved.question = question
            solved.warnings = list(getattr(solved, "warnings", []) or []) + [
                f"Read by the decision model (confidence {s['confidence']:.4f}): {s['capability']} for {target} with "
                f"{given}; the asked quantity read by the rule-based reader; computed and verified by the engine."]
            solved.uncertainty = {**(solved.uncertainty or {}), "reading": "decision_model",
                                  "decision_model": {"capability": s["capability"], "confidence": s["confidence"],
                                                     "inputs": s["inputs"], "target": target}}
            return solved
    dist = f"{product.domain}_ai999x"
    rel = product.pack().by_key.get(s["capability"])
    if rel_reading is not None:                       # a relation: the call takes symbols with their units
        target, given = rel_reading
        call = f"{dist}.{s['capability']}(" + ", ".join(f"{k}='{v}'" for k, v in given.items()) + f", target='{target}')"
    elif rel is not None:                             # the gate declined: still a call the user can run after checking
        from .compiler import _unit_after
        text, args = s.get("text") or question, []
        for slot, value in s["inputs"].items():
            kind, _, role = slot.partition("/")
            sym = _symbol_for(rel, kind, role or None) or slot.replace("/", "_")
            unit = _unit_after(text, s["spans"][slot][1]) if slot in s.get("spans", {}) else None
            args.append(f"{sym}='{value} {unit[1]}'" if unit else f"{sym}={value}")
        call = f"{dist}.{s['capability']}(" + ", ".join(args) + ")"
    else:
        call = f"{dist}.{s['capability']}(" + ", ".join(f"{k}={v}" for k, v in s["inputs"].items()) + ")"
    answer.suggestion = {**{k: v for k, v in s.items() if k not in ("spans", "text")}, "call": call, "check": "values were read by a model: check each one (signs especially) "
                                                     "before running the call"}
    answer.message = ((answer.message or "").rstrip() + f"\nSuggestion (decision model, confidence {s['confidence']:.2f}, "
                      f"not verified): this looks like `{s['capability']}`. Read from your question: "
                      f"{s['inputs'] or 'no values'}. Check them (signs especially), then run "
                      f"{call} — see {dist}.help('{s['capability']}') for what each input means.")
    return answer
