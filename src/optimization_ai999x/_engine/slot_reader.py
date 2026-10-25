# GENERATED from ai999x_factory/runtime by tools/build_package.py — do not edit.
"""The shared slot reader (reader v2): our own encoder reads which quantity each written number states and which
quantity is asked; the reading goes only through the gate (`student_gate.check_reading`) and the engine.

    from ai999x_factory.runtime.slot_reader import SlotReader
    reader = SlotReader("<folder with reader.int8.onnx, tokenizer.json, labels.json>")
    answer = reader.ask(question, pack)          # None when the reading does not pass the gate

What the model decides: a label `kind/role` (or `none`) for every number the runtime's own finder locates
(`student_gate.measurement_spans`), chosen only among kinds of the number's dimension in this pack, and the asked
`kind/role` among this pack's kinds. What it never decides: a value (copied from the text), a condition (the
question's own phrases, `text_tags`), an implied fact (the pack's phrases), or an answer (the engine).

Confidence is the smallest probability among its decisions; the threshold is the one chosen on validation data with
zero wrong answers through the gate (`labels.json` `threshold`; None means it never answers).
Optional dependencies (onnxruntime, tokenizers, numpy) are imported in the constructor only; no network call.
"""
from __future__ import annotations

import json
from pathlib import Path

from .pack import Pack
from .student_gate import _has, check_reading, measurement_spans, text_tags


class SlotReader:
    def __init__(self, folder, model: str = "reader.int8.onnx"):
        import numpy as np
        import onnxruntime as ort
        from tokenizers import Tokenizer
        folder = Path(folder)
        self.labels = json.loads((folder / "labels.json").read_text(encoding="utf-8"))
        self.number_labels, self.target_labels = self.labels["number_labels"], self.labels["target_labels"]
        self.threshold = self.labels.get("threshold")
        self.tok = Tokenizer.from_file(str(folder / "tokenizer.json"))
        self.tok.no_padding()
        self.tok.enable_truncation(int(self.labels.get("max_len", 128)))
        self.session = ort.InferenceSession(str(folder / model), providers=["CPUExecutionProvider"])
        self._np = np

    @staticmethod
    def _softmax(x, allowed):
        import math
        vals = [x[i] for i in allowed]
        m = max(vals)
        ex = [math.exp(v - m) for v in vals]
        s = sum(ex)
        return [e / s for e in ex]

    def read(self, question: str, pack: Pack):
        """(typed problem, confidence, detail) or (None, 0.0, reason)."""
        np = self._np
        e = self.tok.encode(question)
        ids = np.array([e.ids], dtype=np.int64)
        num, tgt = self.session.run(None, {"input_ids": ids, "attention_mask": np.ones_like(ids)})
        num, tgt = num[0], tgt[0]
        conf, given, detail = 1.0, [], []
        for start, end, si, dim, text, _sp, _w, _u in measurement_spans(question):
            pos = [i for i, (s, t) in enumerate(e.offsets) if t > s and s < end and t > start]
            if not pos:
                return None, 0.0, f"'{text}' lies beyond what the reader can see"
            kinds = {k for k, kd in pack.kinds.items() if (kd.dim == dim if dim is not None else kd.unit == "1")}
            allowed = [i for i, lab in enumerate(self.number_labels) if lab == "none" or lab.split("/")[0] in kinds]
            if not allowed:
                return None, 0.0, f"no quantity of this domain has the unit of '{text}'"
            p = self._softmax(num[pos].mean(0), allowed)
            j = max(range(len(p)), key=p.__getitem__)
            conf = min(conf, p[j])
            lab = self.number_labels[allowed[j]]
            detail.append((text, lab, round(p[j], 4)))
            if lab == "none":
                continue
            kind, role = lab.split("/", 1)
            given.append({"kind": kind, "role": role, "value": si, "unit": pack.kinds[kind].unit})
        allowed_t = [i for i, lab in enumerate(self.target_labels) if lab.split("/")[0] in pack.kinds]
        if not allowed_t:
            return None, 0.0, "no asked quantity of this domain is known to the reader"
        p = self._softmax(tgt, allowed_t)
        j = max(range(len(p)), key=p.__getitem__)
        conf = min(conf, p[j])
        kind, role = self.target_labels[allowed_t[j]].split("/", 1)
        detail.append(("asked", f"{kind}/{role}", round(p[j], 4)))
        have = {(g["kind"], g["role"]) for g in given}
        low = question.lower()
        implied = [{"kind": x["kind"], "role": x["role"], "value": x["value"], "unit": x["unit"]}
                   for x in pack.terms.get("implied", []) if _has(low, x["phrase"]) and (x["kind"], x["role"]) not in have]
        kind_tags = {(pack.terms.get("kind_tags") or {}).get(g["kind"]) for g in given} - {None}
        problem = {"target": {"kind": kind, "role": role}, "given": given,
                   "tags": sorted(text_tags(question, pack) | kind_tags), "implied": implied}
        return problem, conf, detail

    def gate(self, question: str, pack: Pack, threshold=None):
        """The gate's verdict on this reading: without implied facts first, then with them."""
        problem, conf, detail = self.read(question, pack)
        if problem is None:
            return None, conf, detail
        thr = self.threshold if threshold is None else threshold
        implied = problem.pop("implied")
        g = check_reading(question, problem, pack, conf, thr)
        if not g.passed and implied and "does not solve" in g.reason:
            g2 = check_reading(question, {**problem, "given": problem["given"] + implied}, pack, conf, thr)
            if g2.passed:
                g = g2
        return g, conf, detail

    def ask(self, question: str, pack: Pack, threshold=None):
        g, _conf, _detail = self.gate(question, pack, threshold)
        return g.answer if g is not None and g.passed else None
