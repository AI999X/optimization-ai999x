# GENERATED from ai999x_factory/runtime by tools/build_package.py — do not edit.
"""One retrieval interface over a knowledge store: exact lookup, structured filtering, keyword (FTS5), optional vector
retrieval and hybrid ranking — with provenance-aware filtering. Standard library only; a vector backend is injected
(the build-time vector index module), never imported here, so packages need no FAISS or embedding model.

    r = Retriever(KnowledgeStore(path, readonly=True))
    r.exact("algebra.procedure.vertex_form")
    r.keyword("complete the square", tiers={"machine_checked"})
    r.hybrid("how do I find where a parabola turns", k=5)        # keyword only unless a vector backend is attached
"""
from __future__ import annotations

VERIFIED = frozenset({"machine_checked", "source_checked", "expert_reviewed"})


class Retriever:
    def __init__(self, store, vector=None):
        self.store, self.vector = store, vector          # vector: object with .search(text, k) -> [(record_id, score)]

    @staticmethod
    def _allowed(rec, tiers, origins, domain) -> bool:
        return ((tiers is None or rec.get("tier") in tiers) and (origins is None or rec.get("origin") in origins)
                and (domain is None or rec.get("domain") == domain))

    def exact(self, record_id: str):
        return self.store.get(record_id)

    def filter(self, limit: int = 100, **criteria) -> list:
        return self.store.find(limit=limit, **criteria)

    def keyword(self, text: str, k: int = 10, tiers=None, origins=None, domain=None) -> list:
        hits = self.store.search(text, limit=k * 5)
        return [h for h in hits if self._allowed(h, tiers, origins, domain)][:k]

    def semantic(self, text: str, k: int = 10, tiers=None, origins=None, domain=None) -> list:
        if self.vector is None:
            return []
        out = []
        for rid, score in self.vector.search(text, k * 5):
            rec = self.store.get(rid)
            if rec and self._allowed(rec, tiers, origins, domain):
                out.append(dict(rec, score=float(score)))
            if len(out) >= k:
                break
        return out

    def hybrid(self, text: str, k: int = 10, tiers=None, origins=None, domain=None, rrf: int = 60) -> list:
        """Reciprocal-rank fusion of keyword and (if attached) vector results; keyword alone without a backend."""
        lists = [self.keyword(text, k * 2, tiers, origins, domain)]
        if self.vector is not None:
            lists.append(self.semantic(text, k * 2, tiers, origins, domain))
        fused, recs = {}, {}
        for lst in lists:
            for rank, rec in enumerate(lst):
                fused[rec["id"]] = fused.get(rec["id"], 0.0) + 1.0 / (rrf + rank + 1)
                recs[rec["id"]] = rec
        return [dict(recs[i], fused=s) for i, s in sorted(fused.items(), key=lambda kv: -kv[1])[:k]]

    def verified_only(self, text: str, k: int = 10) -> list:
        """What a product may present as checked: machine-checked (or better) records only."""
        return self.hybrid(text, k, tiers=VERIFIED)
