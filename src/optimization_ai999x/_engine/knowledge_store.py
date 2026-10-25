# GENERATED from ai999x_factory/runtime by tools/build_package.py — do not edit.
"""The universal knowledge store: one domain-neutral SQLite contract for every domain (canonical records, topics,
typed relationships, capabilities, evidence), with FTS5 keyword search and indexed graph traversal.

Part of the runtime (no production imports, standard library only), so a package can ship a compact read-only copy.
Schema versions are applied by MIGRATIONS in order and recorded in `meta.schema_version`.

    store = KnowledgeStore(path)             # creates or migrates
    store.add_records(records)               # batch insert/update (canonical record dicts), deduplicated by content key
    store.get(record_id); store.find(domain=, kind=, tier=, topic=); store.search("quadratic formula")
    store.related(record_id, depth=2, types=("requires",)); store.integrity()

Verification tiers are kept exactly as the judge set them: `machine_checked` (recomputed independently),
`structure_valid` (well formed; meaning NOT verified), `source_checked`, `rejected` ... A structurally valid record
is never reported as a verified fact.
"""
from __future__ import annotations

import hashlib
import json
import re
import sqlite3
import time
from pathlib import Path

SCHEMA_VERSION = 1
MIGRATIONS = {
    1: """
    create table if not exists meta(key text primary key, value text);
    create table if not exists records(
        id text primary key, domain text not null, kind text not null, topic text, tier text not null,
        statement text not null, equation text, latex text, origin text, content_key text not null,
        payload text not null, provenance text not null, validation text not null, created text, updated text);
    create unique index if not exists rec_content on records(domain, content_key);
    create index if not exists rec_domain_kind on records(domain, kind);
    create index if not exists rec_tier on records(tier);
    create index if not exists rec_topic on records(topic);
    create table if not exists topics(id text primary key, domain text, title text, description text);
    create table if not exists edges(source text not null, type text not null, target text not null,
        primary key(source, type, target)) without rowid;
    create index if not exists edge_target on edges(target, type);
    create table if not exists capabilities(record_id text not null, capability text not null,
        primary key(record_id, capability)) without rowid;
    create index if not exists cap_capability on capabilities(capability);
    create table if not exists evidence(record_id text not null, check_name text not null, passed integer, detail text);
    create index if not exists ev_record on evidence(record_id);
    create virtual table if not exists records_fts using fts5(statement, equation, kind unindexed, id unindexed,
        tokenize = 'porter unicode61');
    """,
}
_STOP = frozenset("a an and are as at be by for from how i in is it of on or that the this to was what when where which "
                  "who why with does do its their there these".split())
_TIER_OF = {"machine_checked": "machine_checked", "structure_valid": "structure_valid", "source_checked": "source_checked",
            "expert_reviewed": "expert_reviewed", "generated": "generated", "rejected": "rejected"}


def content_key(rec: dict) -> str:
    """Identity of the CONTENT (domain, kind, normalised statement, payload), blind to id and provenance."""
    statement = re.sub(r"\s+", " ", rec.get("statement", "")).strip().lower()
    body = json.dumps([rec.get("domain"), rec.get("kind"), statement, rec.get("payload", {})], sort_keys=True,
                      ensure_ascii=False, default=str)
    return hashlib.sha256(body.encode()).hexdigest()


class KnowledgeStore:
    def __init__(self, path, readonly: bool = False):
        self.path = Path(path)
        if readonly:
            self.db = sqlite3.connect(f"file:{self.path.as_posix()}?mode=ro", uri=True, check_same_thread=False)
        else:
            self.path.parent.mkdir(parents=True, exist_ok=True)
            self.db = sqlite3.connect(self.path, check_same_thread=False)
            self.db.execute("pragma journal_mode=wal")
            self.db.execute("pragma synchronous=normal")
            self.db.execute("pragma cache_size=-131072")      # 128 MB page cache while building (index pages stay hot)
            self.db.execute("pragma temp_store=memory")
            self.migrate()
        self.db.row_factory = sqlite3.Row

    # --- schema ------------------------------------------------------------------------------------------------
    def version(self) -> int:
        try:
            row = self.db.execute("select value from meta where key='schema_version'").fetchone()
            return int(row[0]) if row else 0
        except sqlite3.OperationalError:
            return 0

    def migrate(self) -> int:
        v = self.version()
        for n in sorted(k for k in MIGRATIONS if k > v):
            self.db.executescript(MIGRATIONS[n])
            self.db.execute("insert or replace into meta values('schema_version', ?)", (str(n),))
            self.db.commit()
        return self.version()

    def close(self):
        self.db.close()

    # --- writing (build time) ----------------------------------------------------------------------------------
    def add_topics(self, domain: str, topics: list) -> int:
        self.db.executemany("insert or replace into topics values(?,?,?,?)",
                            [(t["id"], domain, t.get("title", ""), t.get("description", "")) for t in topics])
        self.db.commit()
        return len(topics)

    def add_records(self, records, batch: int = 5000) -> dict:
        """Insert or update canonical record dicts in batches. A record whose CONTENT is already held under another
        id is counted as a duplicate and skipped (the first id wins)."""
        stats = {"inserted": 0, "updated": 0, "duplicates": 0}
        now = time.strftime("%Y-%m-%dT%H:%M:%S")
        cur = self.db.cursor()
        buf = []

        def held_by(column, values):
            out = {}
            vals = list(values)
            for i in range(0, len(vals), 900):              # SQLite's bound-parameter limit
                chunk = vals[i:i + 900]
                q = f"select {column}, id from records where {column} in ({','.join('?' * len(chunk))})"
                out.update({row[0]: row[1] for row in cur.execute(q, chunk)})
            return out

        def flush():
            keys = {r["id"]: content_key(r) for r in buf}
            by_key = held_by("content_key", set(keys.values()))   # one query per batch, not per record
            by_id = held_by("id", keys.keys())
            seen_in_batch, ids_in_batch = {}, set()
            for r in buf:
                if r["id"] in ids_in_batch:                 # the same id twice in one batch: the first one wins
                    stats["duplicates"] += 1
                    continue
                ids_in_batch.add(r["id"])
                ck = content_key(r)
                owner = by_key.get(ck) or seen_in_batch.get((r["domain"], ck))
                if owner and owner != r["id"]:
                    stats["duplicates"] += 1
                    continue
                seen_in_batch[(r["domain"], ck)] = r["id"]
                exists = r["id"] in by_id
                val, prov, pay = r.get("validation", {}), r.get("provenance", {}), r.get("payload", {})
                eq = _text(r.get("equation") or pay.get("equation") or (pay.get("relation") or {}).get("equation") or "")
                topic = r.get("topic") if isinstance(r.get("topic"), str) else None
                row = (r["id"], r["domain"], r["kind"], topic, _TIER_OF.get(val.get("status"), val.get("status", "generated")),
                       _text(r.get("statement", "")), eq, _text(r.get("latex") or pay.get("latex") or ""), _text(prov.get("origin") or ""), ck,
                       json.dumps(pay, ensure_ascii=False, default=str), json.dumps(prov, ensure_ascii=False, default=str),
                       json.dumps(val, ensure_ascii=False, default=str), now, now)
                if exists:
                    cur.execute("delete from records_fts where id=?", (r["id"],))
                    cur.execute("delete from edges where source=?", (r["id"],))
                    cur.execute("delete from capabilities where record_id=?", (r["id"],))
                    cur.execute("delete from evidence where record_id=?", (r["id"],))
                    cur.execute("""update records set domain=?, kind=?, topic=?, tier=?, statement=?, equation=?, latex=?,
                                   origin=?, content_key=?, payload=?, provenance=?, validation=?, updated=? where id=?""",
                                row[1:13] + (now, r["id"]))
                    stats["updated"] += 1
                else:
                    cur.execute("insert into records values(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)", row)
                    stats["inserted"] += 1
                cur.execute("insert into records_fts(statement, equation, kind, id) values(?,?,?,?)", (row[5], row[6], row[2], row[0]))
                for rel in r.get("relationships", []):
                    cur.execute("insert or ignore into edges values(?,?,?)", (r["id"], rel.get("type", "related"), rel.get("to", "")))
                for cap in r.get("capabilities", []) or _capabilities_of(r):
                    cur.execute("insert or ignore into capabilities values(?,?)", (r["id"], cap))
                for e in val.get("evidence", []) or []:
                    if isinstance(e, dict):
                        cur.execute("insert into evidence values(?,?,?,?)", (r["id"], e.get("check", ""), int(bool(e.get("passed"))), str(e.get("detail", ""))[:500]))
            buf.clear()

        for r in records:
            buf.append(r)
            if len(buf) >= batch:
                flush()
        flush()
        self.db.commit()
        return stats

    # --- reading (runtime) -------------------------------------------------------------------------------------
    def get(self, record_id: str) -> dict | None:
        row = self.db.execute("select * from records where id=?", (record_id,)).fetchone()
        return _row(row) if row else None

    def find(self, domain=None, kind=None, tier=None, topic=None, origin=None, capability=None, limit: int = 100) -> list:
        """Exact metadata filtering (indexed columns only)."""
        where, args = [], []
        for col, v in (("domain", domain), ("kind", kind), ("tier", tier), ("topic", topic), ("origin", origin)):
            if v is not None:
                where.append(f"r.{col} = ?")
                args.append(v)
        join = ""
        if capability is not None:
            join = " join capabilities c on c.record_id = r.id"
            where.append("c.capability = ?")
            args.append(capability)
        sql = f"select r.* from records r{join}" + (" where " + " and ".join(where) if where else "") + " limit ?"
        return [_row(x) for x in self.db.execute(sql, args + [limit])]

    def search(self, text: str, limit: int = 20, **filters) -> list:
        """Keyword retrieval (FTS5, BM25 ranking), then exact filters. Words are matched as terms, not as syntax."""
        terms = [t for t in re.findall(r"[A-Za-z0-9_]+", text) if t.lower() not in _STOP]
        if not terms:
            return []
        sql = "select id, rank from records_fts where records_fts match ? order by rank limit ?"   # ranked inside FTS5
        hits = self.db.execute(sql, (" AND ".join(f'"{t}"' for t in terms), limit * 4)).fetchall()
        if len(hits) < limit and len(terms) > 1:           # all terms first; any term only when too few match
            hits = self.db.execute(sql, (" OR ".join(f'"{t}"' for t in terms), limit * 4)).fetchall()
        out = []
        for rid, rank in hits:
            row = self.db.execute("select * from records where id=?", (rid,)).fetchone()
            if row is not None:
                out.append(dict(_row(row), score=-rank))
        for k, v in filters.items():
            out = [r for r in out if r.get(k) == v]
        return out[:limit]

    def related(self, record_id: str, depth: int = 2, types: tuple | None = None, direction: str = "out") -> list:
        """Indexed graph traversal (recursive CTE): records reachable within `depth` typed edges."""
        tfilter = f"and e.type in ({','.join('?' * len(types))})" if types else ""
        a, b = ("source", "target") if direction == "out" else ("target", "source")
        sql = f"""with recursive walk(id, d) as (select ?, 0 union
                    select e.{b}, w.d + 1 from edges e join walk w on e.{a} = w.id where w.d < ? {tfilter})
                  select id, min(d) as depth from walk where id != ? group by id order by depth, id"""
        args = [record_id, depth] + list(types or []) + [record_id]
        return [{"id": r[0], "depth": r[1]} for r in self.db.execute(sql, args)]

    def stats(self) -> dict:
        q = lambda s: self.db.execute(s).fetchall()
        return {"schema_version": self.version(), "records": q("select count(*) from records")[0][0],
                "by_tier": dict(q("select tier, count(*) from records group by tier")),
                "by_kind": dict(q("select kind, count(*) from records group by kind")),
                "edges": q("select count(*) from edges")[0][0], "capabilities": q("select count(distinct capability) from capabilities")[0][0],
                "topics": q("select count(*) from topics")[0][0],
                "bytes": self.path.stat().st_size if self.path.exists() else 0}

    def integrity(self) -> dict:
        """SQLite integrity, FTS consistency, dangling edges, duplicate content."""
        q = lambda s: self.db.execute(s).fetchall()
        ids = q("select count(*) from records")[0][0]
        fts = q("select count(*) from records_fts")[0][0]
        dangling = q("""select count(*) from edges e where e.target not in (select id from records)
                        and e.target not in (select id from topics)""")[0][0]
        dup = q("select count(*) from (select domain, content_key from records group by 1, 2 having count(*) > 1)")[0][0]
        ok = q("pragma integrity_check")[0][0]
        return {"sqlite": ok, "records": ids, "fts_rows": fts, "fts_consistent": fts == ids,
                "dangling_edges": dangling, "duplicate_content": dup, "ok": ok == "ok" and fts == ids and dup == 0}


def _text(v) -> str:
    """A column value as text: structured values (a LaTeX map per equation) are stored as JSON."""
    return v if isinstance(v, str) else json.dumps(v, ensure_ascii=False, default=str)


def _capabilities_of(r: dict) -> list:
    pay = r.get("payload", {})
    caps = []
    if r.get("kind") == "procedure":
        caps.append(r["id"].split(".")[-1])
    task = pay.get("task") or {}
    if task.get("procedure"):
        caps.append(task["procedure"])
    caps += list(task.get("models") or [])
    return sorted(set(caps))


def _row(row) -> dict:
    d = dict(row)
    for k in ("payload", "provenance", "validation"):
        if k in d and isinstance(d[k], str):
            d[k] = json.loads(d[k])
    return d
