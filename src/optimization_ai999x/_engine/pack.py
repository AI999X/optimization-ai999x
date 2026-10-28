# GENERATED from ai999x_factory/runtime by tools/build_package.py — do not edit.
"""A domain pack: the kinds of quantity a subject uses, its relations and its vocabulary — data, not code."""
from __future__ import annotations

import json
from dataclasses import dataclass
from pathlib import Path

from . import expr as E
from .quantity import Dim, parse_unit
from .relation import Relation


class PackError(ValueError):
    pass


def _exec_or_forget(spec, mod, name: str) -> None:
    """Run a domain module registered in sys.modules; if it fails, unregister it before re-raising, as Python's own
    import does. Otherwise the half-run module (no PROCEDURES) stays cached and every later load sees an empty domain:
    in the browser a first run before SymPy was downloaded left calculus with no capabilities at all (2026-10-28)."""
    import sys
    try:
        spec.loader.exec_module(mod)
    except BaseException:
        if sys.modules.get(name) is mod:
            del sys.modules[name]
        raise


@dataclass(frozen=True)
class Kind:
    name: str
    unit: str                # the SI unit an answer of this kind is reported in
    dim: Dim                 # includes the angle marker, so rad/s and 1/s are different kinds
    difference: bool = False  # a difference of an offset quantity (a temperature change)
    spellings: tuple = ()     # unit spellings that by themselves identify this kind ("J" is an energy)


class Pack:
    def __init__(self, domain: str, kinds: dict, relations: list, terms: dict, version: str = "0"):
        self.domain, self.kinds, self.relations, self.terms, self.version = domain, kinds, relations, terms, version
        self.by_key = {r.key: r for r in relations}
        if len(self.by_key) != len(relations):
            raise PackError("duplicate relation key")
        for r in relations:
            for s in r.symbols.values():
                if s.kind not in kinds:
                    raise PackError(f"{r.key}.{s.name}: unknown kind '{s.kind}'")
        self._dims_ok: dict = {}

    @staticmethod
    def load(folder, version: str | None = None) -> "Pack":
        """The domain's pack; with `version`, the pack exactly as it was at that version: the snapshot
        `knowledge/packs/<version>/` (relations, kinds, terms; tools/pack_snapshot.py), e.g. to re-judge a finished
        run. Procedures always follow the current certified set (a blocked record waits for them)."""
        folder = Path(folder)
        read = lambda p: json.loads(Path(p).read_text(encoding="utf-8"))
        current = folder / "knowledge" / "relations.json"
        snap = folder / "knowledge" / "packs" / str(version) if version else None
        if version and read(current).get("version") != version and not (snap and (snap / "relations.json").exists()):
            raise FileNotFoundError(f"pack {version} is not kept (knowledge/packs/{version}/ missing); cannot reproduce it")
        where = lambda name, sub: (snap / name if snap and (snap / name).exists() else folder / sub / name)
        raw = read(where("kinds.json", "dictionary"))
        kinds = {k: Kind(k, v["unit"], parse_unit(v["unit"]).dim, bool(v.get("difference")),
                         tuple(v.get("spellings", ()))) for k, v in raw.items()}
        rel = read(where("relations.json", "knowledge"))
        pack = Pack(rel["domain"], kinds, [Relation.from_dict(r) for r in rel["relations"]],
                    read(where("terms.json", "dictionary")), rel.get("version", "0"))
        pack._attach_families(folder / "capabilities" / "families.py")
        pack._attach_procedures(folder / "capabilities")
        return pack

    # --- procedures: a domain's deterministic algorithms (several outputs); only CERTIFIED ones are exposed ------
    _DOMAIN_PROCEDURES: dict = {}

    @property
    def procedures(self) -> dict:
        return dict(Pack._DOMAIN_PROCEDURES.get(self.domain, {}))

    def _attach_procedures(self, folder: Path) -> None:
        """Hand-written procedures (procedures.py) and compiled ones (procedures_compiled.json, written by
        tools/compile_procedures.py from verified specifications); only those certified are exposed."""
        certified = folder / "procedures_certified.json"
        ok = set(json.loads(certified.read_text(encoding="utf-8")).get("certified", [])) if certified.exists() else set()
        found = self._defined_procedures(folder, cached=True)
        if found or certified.exists():
            Pack._DOMAIN_PROCEDURES[self.domain] = {k: v for k, v in found.items() if k in ok}

    def _defined_procedures(self, folder: Path, cached: bool) -> dict:
        import importlib.util
        import sys
        out = {}
        path = folder / "procedures.py"
        if path.exists():
            name = f"{__package__}._procedures_{self.domain}" if cached else f"_candidate_procedures_{self.domain}"
            mod = sys.modules.get(name) if cached else None
            if mod is None or getattr(mod, "__file__", None) != str(path):
                spec = importlib.util.spec_from_file_location(name, path)
                mod = importlib.util.module_from_spec(spec)
                if cached:
                    sys.modules[name] = mod
                _exec_or_forget(spec, mod, name)
            out.update(getattr(mod, "PROCEDURES", {}))
        compiled = folder / "procedures_compiled.json"
        if compiled.exists():
            from .procedure_spec import to_procedure
            for spec in json.loads(compiled.read_text(encoding="utf-8")).get("procedures", []):
                if spec["key"] in out:
                    raise PackError(f"procedure {spec['key']} is defined twice")
                out[spec["key"]] = to_procedure(spec)
        return out

    def all_procedures(self, folder) -> dict:
        """Every procedure the domain defines, certified or not (for certification only)."""
        return self._defined_procedures(Path(folder) / "capabilities", cached=False)

    # --- model families: a domain's own code for problems with several bodies, states or stages ---------------
    # registered per DOMAIN, so every pack of the domain (a working pack, a pack plus a candidate) has them
    _DOMAIN_FAMILIES: dict = {}

    @property
    def families(self) -> list:
        return list(getattr(Pack._DOMAIN_FAMILIES.get(self.domain), "FAMILIES", []))

    @property
    def family_uses(self) -> dict:
        return dict(getattr(Pack._DOMAIN_FAMILIES.get(self.domain), "FAMILY_USES", {}))

    @property
    def kind_preference(self) -> list:
        return list(getattr(Pack._DOMAIN_FAMILIES.get(self.domain), "KIND_PREFERENCE", []))

    def _attach_families(self, path: Path) -> None:
        """Load the domain's model families (optional). The module is loaded INSIDE this runtime's `systems`
        package, so `from .solver import ...` resolves to the same engine in the factory and in an installed
        package; the core itself holds no domain's families."""
        if not path.exists():
            return
        import importlib.util
        import sys
        from . import systems  # noqa: F401  (the parent package must be imported first)
        name = f"{__package__}.systems._families_{self.domain}"
        mod = sys.modules.get(name)
        if mod is None or getattr(mod, "__file__", None) != str(path):
            spec = importlib.util.spec_from_file_location(name, path)
            mod = importlib.util.module_from_spec(spec)
            sys.modules[name] = mod
            _exec_or_forget(spec, mod, name)
        Pack._DOMAIN_FAMILIES[self.domain] = mod

    def family_index(self) -> list:
        titles = getattr(Pack._DOMAIN_FAMILIES.get(self.domain), "FAMILY_TITLES", {})
        return [{"key": k, "title": titles.get(k, k.replace("_", " ")), "uses": sorted(v)}
                for k, v in sorted(self.family_uses.items())]

    def reading_labels(self) -> list:
        """The phrases the reader binds a value or the asked quantity with: the dictionary's labels and, when the
        dictionary opts in (`"labels_from_descriptions": true`), each relation symbol's own description ("wavelength
        of maximum spectral emission", "temperature at the mid-plane") — the words the domain's questions were
        written from. A description becomes a label only when it is at least two words, is not already a label, and
        names ONE kind and role across the whole pack; otherwise it is left out, never guessed between."""
        if getattr(self, "_reading_labels", None) is not None:
            return self._reading_labels
        labels = list(self.terms.get("labels", []))
        if self.terms.get("labels_from_descriptions"):
            have = {e["phrase"].lower() for e in labels}
            seen: dict = {}
            for r in self.relations:
                for s in r.symbols.values():
                    d = " ".join((s.desc or "").lower().split())
                    if len(d.split()) >= 2 and d not in have:
                        seen.setdefault(d, set()).add((s.kind, s.role))
            labels += [{"phrase": d, "kind": k, "role": role, "from": "description"}
                       for d, readings in sorted(seen.items()) if len(readings) == 1 for k, role in readings]
        self._reading_labels = labels
        return labels

    def kinds_for(self, dim: Dim) -> list:
        return [k.name for k in self.kinds.values() if k.dim == dim]

    def kind_by_spelling(self, dim: Dim, spelled: str) -> list:
        """Kinds of this dimension, narrowed to one when the unit's spelling names it ("kJ" -> energy)."""
        kinds = self.kinds_for(dim)
        named = [k for k in kinds if spelled in self.kinds[k].spellings or spelled[1:] in self.kinds[k].spellings]
        return named if len(named) == 1 else kinds

    def roles_of(self, kind: str) -> set:
        return {s.role for r in self.relations for s in r.symbols.values() if s.kind == kind}

    def dimensions_consistent(self, relation: Relation) -> bool:
        """Both sides of the relation carry the same physical dimension under this pack's kinds."""
        if relation.key not in self._dims_ok:
            sd = {n: self.kinds[s.kind].dim for n, s in relation.symbols.items()}
            try:
                l, r = relation.sides
                self._dims_ok[relation.key] = E.dims(l, sd) == E.dims(r, sd)
            except (E.ExpressionError, ValueError):
                self._dims_ok[relation.key] = False
        return self._dims_ok[relation.key]
