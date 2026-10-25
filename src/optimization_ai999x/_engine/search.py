# GENERATED from ai999x_factory/runtime by tools/build_package.py — do not edit.
"""Capability search: the AI999X shared encoder finds which certified capability a question is about, across EVERY
domain in the index (physics, mechanics, algebra, ...). It never answers: it points to capabilities.

    product.search("How long does a damped spring take to oscillate?")   # -> ranked capabilities with examples
    product.ask(question, suggest=True)                                  # a refusal also lists likely capabilities

Optional (pip install <package>[search]: onnxruntime, tokenizers, numpy). Files in pack/search/: encoder.int8.onnx,
tokenizer.json, index.json ({"model", "dim", "entries": [{"domain", "key", "name", "package", "example", "v"}]}).
Without them, search() raises with the install hint and ask() is unchanged.
"""
from __future__ import annotations

import json
from pathlib import Path


class SearchUnavailable(ImportError):
    pass


class CapabilitySearch:
    def __init__(self, folder: Path, domain: str, package: str):
        self.folder, self.domain, self.package = Path(folder), domain, package
        self._sess = self._tok = self._index = self._V = None

    @staticmethod
    def find(folders, domain: str, package: str) -> "CapabilitySearch | None":
        for f in folders:                                 # installed: pack/search; factory tree: <root>/shared/search
            f = Path(f)
            for cand in (f / "search", f / "shared" / "search", *[p / "shared" / "search" for p in list(f.parents)[:3]]):
                if (cand / "index.json").exists() and (cand / "encoder.int8.onnx").exists():
                    return CapabilitySearch(cand, domain, package)
        return None

    def _load(self):
        if self._sess is not None:
            return
        try:
            import numpy as np
            import onnxruntime as ort
            from tokenizers import Tokenizer
        except ImportError as exc:
            raise SearchUnavailable(f"capability search needs its optional packages: pip install {self.package}[search]") from exc
        opts = ort.SessionOptions()
        opts.intra_op_num_threads = 1                      # small and deterministic
        self._sess = ort.InferenceSession(str(self.folder / "encoder.int8.onnx"), opts, providers=["CPUExecutionProvider"])
        self._tok = Tokenizer.from_file(str(self.folder / "tokenizer.json"))
        self._tok.enable_truncation(96)
        self._index = json.loads((self.folder / "index.json").read_text(encoding="utf-8"))
        self._V = np.array([e["v"] for e in self._index["entries"]], dtype=np.float32)
        self._np = np

    def embed(self, text: str):
        self._load()
        e = self._tok.encode(text)
        np = self._np
        out = self._sess.run(None, {"input_ids": np.array([e.ids], dtype=np.int64),
                                    "attention_mask": np.array([e.attention_mask], dtype=np.int64)})[0][0]
        return out / max(1e-12, float(np.linalg.norm(out)))

    def search(self, text: str, k: int = 5, domain: str | None = None) -> list:
        q = self.embed(text)
        scores = self._V @ q
        order = self._np.argsort(-scores)
        out = []
        for i in order:
            e = self._index["entries"][int(i)]
            if domain and e["domain"] != domain:
                continue
            out.append({"domain": e["domain"], "key": e["key"], "name": e["name"], "score": round(float(scores[i]), 4),
                        "package": e["package"], "here": e["domain"] == self.domain, "example": e.get("example", "")})
            if len(out) >= k:
                break
        return out
