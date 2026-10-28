# GENERATED from ai999x_factory/runtime by tools/build_package.py — do not edit.
"""The public API every domain package exposes, written once for all domains.

    from ai999x_factory.runtime.product import Product        # a package uses its bundled copy (_engine)
    P = Product("physics", [Path(__file__).parent / "pack", <source-tree folder>])
    P.ask("A 3 kg object moves at 4 m/s. What is its kinetic energy?")
    P.solve("kinetic energy", {"mass": "3 kg", "speed": "4 m/s"})
    P.formula("kinetic_energy"); P.formulas(); P.quantities(); P.conditions(); P.capabilities()

A domain package is a thin layer: its name, where its pack is, and any extras of its own (plots, guided questions,
an optional trained reader). The engine, the structured interface and the refusals are the same for every domain.
"""
from __future__ import annotations

import re
from pathlib import Path

from .answer import Answer
from .compiler import compile_problem
from .engine import ask as _ask
from .pack import Pack


_MUST = {"positive": "greater than zero", "nonnegative": "zero or more"}


def _plain_refusal(rel, p, info: dict, sources: dict) -> str:
    """Why these inputs have no answer, in plain words, with the quantity's meaning and unit (owner, 2026-10-28:
    "no admissible real value for C" told a visitor nothing). "" when the reason is not known (the old text is used)."""
    name = info.get("name")
    if name not in getattr(rel, "symbols", {}):
        return ""
    s = rel.symbols[name]
    what = s.desc or s.kind.replace("_", " ")
    unit = p.kinds[s.kind].unit if s.kind in p.kinds and p.kinds[s.kind].unit not in ("", "1") else ""

    def amount(v: float) -> str:
        return f"{v:.4g}" + (f" {unit}" if unit else "")

    check = "These inputs cannot all hold at once: check each value and its unit."
    reason = info.get("reason")
    if reason == "input_sign":
        given = sources.get(name) or amount(info.get("value", 0.0))
        return f"{what} ({name}) must be {_MUST.get(s.sign, 'valid')}, but it was given as {given}. Check this input."
    if reason == "result_sign" and info.get("values"):
        return (f"with these inputs, {what} ({name}) would come out as {amount(info['values'][0])}, which is impossible: "
                f"it must be {_MUST.get(s.sign, 'valid')}. {check}")
    if reason == "no_real":
        return (f"with these inputs, no real value of {what} ({name}) satisfies the formula (it would need, for example, "
                f"the square root or logarithm of a negative number, or a division by zero). {check}")
    if reason == "branches" and info.get("values"):
        vals = ", ".join(amount(v) for v in info["values"])
        return (f"these inputs allow {len(info['values'])} values of {what} ({name}): {vals}. The formula alone cannot "
                f"choose between them; use a capability or a condition that tells them apart.")
    return ""


class QuantityError(ValueError):
    """A quantity name, value or condition the domain's vocabulary does not know."""


class Product:
    def __init__(self, domain: str, folders):
        self.domain, self._folders, self._pack = domain, [Path(f) for f in folders], None

    # --- the pack ----------------------------------------------------------------------------------------------
    def pack(self) -> Pack:
        if self._pack is None:
            for folder in self._folders:
                if (folder / "knowledge" / "relations.json").exists():
                    self._pack = Pack.load(folder)
                    break
            else:
                raise FileNotFoundError(f"the {self.domain} knowledge pack is missing from this installation")
        return self._pack

    # --- derivations and proofs (certified capabilities; runtime/derivations.py) -------------------------------
    def _pack_folder(self) -> Path | None:
        return next((f for f in self._folders if (f / "knowledge" / "relations.json").exists()), None)

    def about(self) -> dict:
        """Who made this package (runtime/identity.py): creator, role, company — the same answer every time."""
        from .identity import about as _about
        return _about(self.domain)

    def derivations(self) -> list:
        """Every certified derivation or proof: id, title, equation, hypotheses, the exact question forms."""
        from .derivations import load
        folder = self._pack_folder()
        return [{k: e[k] for k in ("id", "title", "equation", "hypotheses", "questions") if k in e}
                for e in (load(folder) if folder else [])]

    def prove(self, what: str) -> Answer:
        """The certified derivation with this id, or a prove-form question ("Prove that ..."): its checked steps."""
        from .derivations import answer, load, unknown
        entries = load(self._pack_folder()) if self._pack_folder() else []
        by_id = {e["id"]: e for e in entries}
        if what in by_id:
            return answer(by_id[what], f"prove {what}")
        got = self._derivation(what if re.match(r"^\s*(prove|derive|show|given)\b", what, re.I) else f"Prove that {what}")
        return got if got is not None else unknown(what, entries)

    def _derivation(self, question: str):
        from .derivations import answer, load, match, unknown
        if not re.match(r"^\s*(?:please\s+)?(prove|derive|show|given)\b", question, re.I):
            return None
        entries = load(self._pack_folder()) if self._pack_folder() else []
        found = match(question, entries)
        if found is None:
            return None
        entry, missing = found
        return unknown(question, entries) if entry is None else answer(entry, question, missing)

    # --- answering ----------------------------------------------------------------------------------------------
    def search(self, question: str, k: int = 5, domain: str | None = None) -> list:
        """The certified capabilities a question is most likely about, across every domain in the shared index
        (runtime/search.py). It never answers; with `pip install <package>[search]`."""
        from .search import CapabilitySearch, SearchUnavailable
        if not hasattr(self, "_search"):
            pkg = self.domain.replace("_", "-") + "-ai999x"
            self._search = CapabilitySearch.find(self._folders, self.domain, pkg)
        if self._search is None:
            raise SearchUnavailable("this installation has no capability search index")
        return self._search.search(question, k=k, domain=domain)

    def ask(self, question, suggest: bool = False) -> Answer:
        """As _ask; with suggest=True a question that is not answered also carries the capabilities it is most likely
        about (`answer.suggestions`), from the shared search index when it is installed. The answer itself never
        changes: a suggestion is a pointer, not a result."""
        a = self._ask(question)
        if not a.solved and isinstance(question, str) and a.outcome == "UNRECOGNIZED_FORMAT":
            # the optional trained reader (runtime/proc_reader.py), only where every rule-based parser refused and
            # only when this package enables it; it proposes a reading, the engine computes, its gates decide
            read = self._trained_reading(question, a)
            if read is not None:
                return read
        # never over a capability that recognised the question and declined it (OUT_OF_SCOPE, AMBIGUOUS, INVALID_INPUT:
        # "Solve x^5 - 1 = 0" is out of scope for algebra's procedures, not an invitation to answer it partly)
        if not a.solved and isinstance(question, str) and a.outcome in ("UNRECOGNIZED_FORMAT", "NEEDS_INPUT"):
            # fallbacks, only where the certified capabilities did not answer: a formula the user typed
            # ("Solve E = mc^2 for m"), then a conceptual question ("What is ...?", "When does ... fail?")
            typed = self._typed_formula(question)
            if typed is not None and typed.solved:
                return typed
            explained = self._explain(question)
            if explained is not None and (explained.solved or a.outcome == "UNRECOGNIZED_FORMAT"):
                return explained
        if suggest and not a.solved and isinstance(question, str):
            try:
                a.suggestions = self.search(question, k=3)
            except ImportError:
                a.suggestions = []
        return a

    def _ask(self, question) -> Answer:
        """Answer a question in words (or a typed problem as a dictionary), within the documented question formats.
        Every answer carries an `outcome` (SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE,
        UNRECOGNIZED_FORMAT, INTERNAL_ERROR); a format the reader does not recognise comes back with a copyable
        supported template and the structured call, never a guess."""
        if isinstance(question, str):
            from .identity import about as _about, is_identity_question
            if is_identity_question(question):             # "Who created you?": one fixed, deterministic answer
                info = _about(self.domain)
                a = Answer("solved", question, message=info["statement"], models=["identity"])
                a.uncertainty = {"state": "determined", "capability": "identity"}
                a.values = {"creator": info["creator"], "company": info["company"]}
                return a
            proved = self._derivation(question)            # "Prove that ...", "Derive ...", "Show that ..."
            if proved is not None:
                return proved
            # a domain's documented forms for its algorithms. Every parser is asked; an exact documented template
            # (rank 0) outranks a hand-written parser (rank 1), which outranks a 'with a = .., b = ..' reading
            # (rank 2). Two different capabilities at the best rank is an AMBIGUOUS question, never a guess.
            found = []
            for key, spec in self.pack().procedures.items():
                try:
                    ranked = spec["parse_ranked"](question) if "parse_ranked" in spec else \
                        (lambda p: None if p is None else (1, p))(spec["parse"](question))
                except Exception as exc:
                    return self._internal_error(question, exc)
                if ranked is None:
                    continue
                found.append((ranked[0], key, ranked[1]))
            if found:
                best = min(r for r, _k, _p in found)
                top = [(k, p) for r, k, p in found if r == best]
                if len(top) > 1 and best == 2:
                    # keyword readings: the most SPECIFIC keyword wins — "absolute value" and "inequality" are both
                    # contained in "absolute value inequality less than", which names the intended capability
                    q = question.lower()
                    hit = {k: max((w for w in self.pack().procedures[k].get("keywords", []) if w in q), key=len, default="")
                           for k, _p in top}
                    top = [(k, p) for k, p in top if not any(hit[k] != hit[o] and hit[k] in hit[o] for o, _ in top)]
                if len(top) > 1:
                    a = Answer("needs_clarification", question, models=[k for k, _p in top],
                               message="This question matches several capabilities: " + ", ".join(k for k, _p in top)
                                       + ". Use solve() with one of them.")
                    a.uncertainty = {"state": "ambiguous", "cause": "ambiguous", "outcome": "AMBIGUOUS",
                                     "candidates": [k for k, _p in top]}
                    return a
                key, parsed = top[0]
                if "_out_of_scope" in parsed:
                    a = Answer("unsupported", question, message=f"Out of scope: {parsed['_out_of_scope']}.", models=[key])
                    a.uncertainty = {"state": "unsupported", "cause": "unsupported_structure", "outcome": "OUT_OF_SCOPE"}
                    return a
                return self._solve_procedure(key, parsed, question)
        if isinstance(question, str) and not self.pack().relations:
            # a domain of procedures only (algebra): the physical-quantity reader has nothing to read, and its wording
            # ("the number '3' has no unit") would only confuse — say what is supported instead
            a = Answer("needs_clarification", question,
                       message="I could not match this question to a documented form, so I will not guess.")
            a.uncertainty = {"state": "needs_clarification", "cause": "unrecognized_format", "outcome": "UNRECOGNIZED_FORMAT"}
            hint = self._nearest_template(question)
            if hint:
                a.uncertainty["try"], a.uncertainty["structured"] = hint["question"], hint["structured"]
                a.message += f" A supported way to ask: \"{hint['question']}\" — or {hint['structured']}"
            a.message += f" Every form: {self.domain}_ai999x.templates(); every capability: {self.domain}_ai999x.help()."
            return a
        try:
            a = _ask(question, self.pack())
        except Exception as exc:                       # a defect of the engine, never disguised as a refusal
            return self._internal_error(question, exc)
        if a.outcome == "NEEDS_INPUT" and isinstance(question, str) and (a.uncertainty or {}).get("cause") == "no_applicable_relation_chain":
            fit = self._fitting_needs(question)
            if fit is None:                            # no certified capability computes this in this situation
                a.uncertainty["outcome"] = "OUT_OF_SCOPE"
                a.message = ("No certified capability computes the asked quantity in this situation, so there is no "
                             "missing input to ask for. See capabilities() for what this version computes.")
            else:
                a.uncertainty["needs"] = fit["needs"]
                a.uncertainty["capability"] = fit["key"]
                a.message = (f"{fit['name']} applies, but it needs: " + ", ".join(fit["needs_words"]) +
                             ". Add it to the question with its unit, or use " + self.structured_example(fit["key"]) + ".")
        if a.outcome == "UNRECOGNIZED_FORMAT" and isinstance(question, str):
            hint = self._nearest_template(question)
            if hint:
                a.uncertainty["try"] = hint["question"]
                a.uncertainty["structured"] = hint["structured"]
                a.message += f" A supported way to ask: \"{hint['question']}\" — or {hint['structured']}"
        return a

    def compile_problem(self, question: str):
        return compile_problem(question, self.pack())

    def solve(self, find: str, given: dict | None = None, conditions=(), *, target: str | None = None, **inputs) -> Answer:
        """Solve without writing a sentence. Two forms:

            solve("kinetic_energy", mass="3 kg", speed="4 m/s")          # a capability by its id: the contract path
            solve("kinetic energy", {"mass": "3 kg", "speed": "4 m/s"})   # a quantity to find, planned over relations

        With a capability id, inputs are named by symbol ("m"), kind ("mass"), kind/role ("speed/final") or a
        vocabulary phrase; `target=` picks the symbol to compute when it is not the equation's left side. Missing
        inputs return NEEDS_INPUT naming each one; a wrong unit or an unknown input returns INVALID_INPUT."""
        given = {**(given or {}), **inputs}
        try:
            if find in self.pack().procedures:
                # an algorithm may have an input named `target` (the end node of a path): it is that input, not the
                # symbol-to-compute switch of a formula
                if target is not None and any(i["symbol"] == "target" for i in self.pack().procedures[find]["inputs"]):
                    given["target"] = target
                return self._solve_procedure(find, given)
            if find in self.pack().by_key:
                return self._solve_capability(find, given, conditions, target)
            typed = self.problem(find, given, conditions)
        except QuantityError as exc:
            return Answer("invalid", "structured problem", message=str(exc), uncertainty={"state": "invalid", "cause": "unknown_name"})
        except Exception as exc:
            return self._internal_error(f"solve({find!r})", exc)
        return self.ask(typed)

    # --- the capability contract path: one certified relation, no language involved ----------------------------
    def _solve_capability(self, key: str, given: dict, conditions, target) -> Answer:
        from .quantity import UnitError, parse_unit, to_si
        p = self.pack()
        rel = p.by_key[key]
        q = f"solve({key!r})"
        takes = ", ".join(f"{n} ({s.kind.replace('_', ' ')}{'/' + s.role if s.role != 'plain' else ''}, {p.kinds[s.kind].unit})"
                          for n, s in rel.symbols.items())

        def refuse(outcome, cause, message, **extra):
            return Answer("invalid" if outcome == "INVALID_INPUT" else "needs_clarification", q, message=message, models=[key],
                          uncertainty={"state": {"NEEDS_INPUT": "missing_information"}.get(outcome, "invalid"), "cause": cause,
                                       "outcome": outcome, **extra})
        bad = set(conditions) & set(rel.excludes)
        if bad:
            return refuse("INVALID_INPUT", "not_applicable", f"{rel.name} does not apply when: {', '.join(sorted(bad))}.")
        values, sources = {}, {}
        for name, v in given.items():
            sym = self._symbol_of(rel, name)
            if sym is None:
                return refuse("INVALID_INPUT", "unknown_name", f"'{name}' is not an input of {key}. It takes: {takes}.")
            kind = p.kinds[rel.symbols[sym].kind]
            try:
                number, unit = self._value(kind.name, v)
                u = parse_unit(unit)
            except (QuantityError, UnitError, ValueError):
                return refuse("INVALID_INPUT", "invalid_unit", f"cannot read {name}={v!r}: give a number and a unit of "
                              f"{kind.name.replace('_', ' ')} (e.g. '{3} {kind.unit}').")
            if u.dim != kind.dim:
                return refuse("INVALID_INPUT", "invalid_unit", f"{name} is a {kind.name.replace('_', ' ')}: its unit must be "
                              f"like {kind.unit}, not '{unit}'.")
            values[sym] = to_si(number, u, difference=kind.difference)
            sources[sym] = f"{number:g} {unit}".strip()
        tgt = None
        if target is not None:
            tgt = self._symbol_of(rel, target)
            if tgt is None:
                return refuse("INVALID_INPUT", "unknown_name", f"target '{target}' is not a quantity of {key}. It has: {takes}.")
            if tgt in values:
                return refuse("INVALID_INPUT", "target_stated", f"{tgt} is given as an input and asked for as the target.")
        assumptions = list(rel.assumptions)
        for n, s in rel.symbols.items():                 # a declared constant (g, k, h ...) takes its stated default
            if n not in values and n != tgt and s.default:
                values[n] = to_si(float(s.default["value"]), parse_unit(s.default["unit"]))
                sources[n] = f"default: {s.default.get('assumption', '')}"
                assumptions.append(s.default.get("assumption", f"default {n}"))
        unknown = [n for n in rel.symbols if n not in values]
        lhs = rel.equation.split("=")[0].strip()
        if tgt is None:
            tgt = unknown[0] if len(unknown) == 1 else (lhs if lhs in unknown else None)
        if not unknown:
            return refuse("INVALID_INPUT", "target_stated", f"every quantity of {key} is given; leave out the one to compute "
                          f"(or pass target=).")
        needs = [n for n in unknown if n != tgt]
        if tgt is None or needs:
            example = ", ".join(f"{n}='... {p.kinds[rel.symbols[n].kind].unit}'" for n in rel.symbols if n != (tgt or lhs))
            missing = ", ".join(f"{n} ({rel.symbols[n].desc or rel.symbols[n].kind}, {p.kinds[rel.symbols[n].kind].unit})" for n in needs)
            return refuse("NEEDS_INPUT", "needs_input", f"{key} needs {missing or 'to know which quantity to compute (pass target=)'}. "
                          f"Example: solve({key!r}, {example}).", needs=needs, target=tgt)
        try:
            value = rel.solve(tgt, values)
        except Exception as exc:                          # no admissible real value / several branches: the inputs
            text = str(exc)
            info = getattr(exc, "info", None) or {}
            outcome = "AMBIGUOUS" if info.get("reason") == "branches" or "branch" in text else "INVALID_INPUT"
            plain = _plain_refusal(rel, p, info, sources)
            return refuse(outcome, "multiple_branches" if outcome == "AMBIGUOUS" else "no_solution",
                          f"{rel.name}: {plain or text.split(': ', 1)[-1] + '.'}")
        residual = rel.residual({**values, tgt: value})
        dims = p.dimensions_consistent(rel)
        ok = residual <= 1e-9 and dims
        if rel.requires:
            assumptions.append("the situation is: " + ", ".join(t.replace("_", " ") for t in rel.requires))
        kind = rel.symbols[tgt].kind
        step = {"relation": key, "name": rel.name, "equation": rel.equation, "equation_latex": rel.latex(),
                "solved_latex": rel.latex_solved(tgt), "target": tgt, "result": value, "unit": p.kinds[kind].unit,
                "inputs": {n: {"value": values[n], "unit": p.kinds[rel.symbols[n].kind].unit, "source": sources.get(n, ""),
                               "meaning": rel.symbols[n].desc or rel.symbols[n].kind} for n in values}}
        a = Answer("solved" if ok else "verification_failed", q, value if ok else None, p.kinds[kind].unit,
                   {"kind": kind, "role": rel.symbols[tgt].role, "phrase": rel.symbols[tgt].desc or kind.replace("_", " ")},
                   [key], assumptions, [step], {"status": "verified" if ok else "failed", "max_residual": residual,
                                                "dimensions_consistent": dims, "independent_paths": 1,
                                                "scope": "algebra and dimensions of the certified relation"})
        a.uncertainty = {"state": "determined", "capability": key} if ok else {"state": "verification_failed",
                                                                                "cause": "computed_value_failed_its_checks"}
        return a

    # --- the optional trained reader for procedures (runtime/proc_reader.py; docs/ONNX_CONTRACT.md) ---------------
    def _proc_reader(self):
        """The trained reader, or None. Off unless its files are present AND this model enables it: in the factory
        tree `models/<d>/package/release.json` "trained_reader": true; installed, the build ships pack/reader only for
        a package that enables it. AI999X_PROC_READER=off|on overrides the switch (on: for measurement; the files must
        still be present); AI999X_PROC_READER_DIR names another folder. Missing optional packages: None."""
        if getattr(self, "_preader", None) is None:
            self._preader = self._load_proc_reader() or False
        return self._preader or None

    def _load_proc_reader(self):
        import json
        import os
        from .proc_reader import MODEL_FILE
        env = os.environ.get("AI999X_PROC_READER", "").strip().lower()
        if env in ("off", "0", "false", "no"):
            return None
        enabled, folder = env in ("on", "1", "true", "yes"), None
        override = os.environ.get("AI999X_PROC_READER_DIR", "").strip()
        for f in self._folders:
            rel = f / "package" / "release.json"            # the factory tree: the model's own switch
            if rel.exists():
                try:
                    enabled = enabled or bool(json.loads(rel.read_text(encoding="utf-8")).get("trained_reader"))
                except ValueError:
                    pass
            for cand in ([Path(override)] if override else []) + [f / "reader", f / "shared" / "reader"] + \
                    [p / "shared" / "reader" for p in list(f.parents)[:3]]:
                if folder is None and (cand / "labels.json").exists() and (cand / MODEL_FILE).exists():
                    folder = cand
                    if cand == f / "reader":                # installed: shipped only in a package that enables it
                        enabled = True
        if folder is None or not enabled:
            return None
        try:
            from .proc_reader import ProcReader
            return ProcReader(folder)
        except ImportError:                                 # pip install <package>[reader] was not chosen
            return None
        except Exception:                                   # unreadable reader files: ask() stays as without them
            return None

    def _trained_reading(self, question: str, refused: Answer):
        """A SOLVED answer from the trained reader's reading, or None (then `refused` notes why, when it ran)."""
        reader = self._proc_reader()
        if reader is None:
            return None
        from .proc_reader import Refusal
        try:
            got = reader.read(question, self.domain, self.pack())
        except Exception as exc:                            # the optional reader never breaks ask()
            got = Refusal(f"the trained reader failed: {type(exc).__name__}: {exc}"[:200])

        def note(reason: str, conf: float):
            refused.uncertainty = {**(refused.uncertainty or {}),
                                   "trained_reader": {"refused_because": reason, "confidence": float(conf)}}
            return None
        if isinstance(got, Refusal):
            return note(got.reason, got.confidence)
        spec = self.pack().procedures[got.capability]
        needs = [i["symbol"] for i in spec["inputs"] if i.get("required") and i["symbol"] not in got.inputs]
        if needs:
            return note("a required input is not in the question: " + ", ".join(needs), got.confidence)
        a = self._solve_procedure(got.capability, dict(got.inputs), question)     # the path of solve(): input checks
        if not a.solved:
            return note(f"the engine did not solve the reading ({a.outcome}: {a.message[:160]})", got.confidence)
        if got.asked in a.values:                           # the asked output is the answer's value
            v = a.values[got.asked]
            out = next((o for o in spec["outputs"] if o["symbol"] == got.asked), {})
            a.value = v if isinstance(v, (int, float)) and not isinstance(v, bool) else None
            a.unit = ((a.steps[0].get("output_units") or {}).get(got.asked, "")) if a.steps else ""
            a.target = {"kind": out.get("kind", "number"), "role": "procedure", "phrase": out.get("meaning", got.asked)}
        a.uncertainty = {**a.uncertainty, "reading_provenance": "trained_reader",
                         "trained_reader": {"confidence": float(got.confidence), "capability": got.capability,
                                            "asked": got.asked, "inputs": dict(got.inputs)}}
        a.warnings.append("Read by the trained reader (no documented question form matched): each value was copied "
                          "from the question as written, and the engine computed and verified the result. The reading "
                          "is in uncertainty['trained_reader']; solve() with it gives the same answer.")
        return a

    # --- a formula the user types, solved step by step (runtime/formula_solver.py) --------------------------------
    def _typed_formula(self, question: str):
        from .formula_solver import FormulaError, solve_formula
        known = {s for r in self.pack().relations for s in r.symbols}
        try:
            r = solve_formula(question, known)
        except FormulaError as exc:
            a = Answer("needs_clarification", question, message=f"{exc}.", models=["formula"])
            a.uncertainty = {"state": "missing_information", "cause": "formula", "outcome": "NEEDS_INPUT"}
            return a
        except Exception:                              # anything unexpected: let the other readers try, never guess
            return None
        if r is None:
            return None
        lines = [f"Solve for ${r['target']}$ in $${r['equation']}$$"]
        lines += [f"{i}. {d}: $${eq}$$" for i, (d, eq) in enumerate(r["steps"], 1)]
        lines.append(f"Result: $${r['solution']}$$")
        if r.get("substituted"):
            lines.append(f"With the stated values: $${r['substituted']}$$ = " + ", ".join(f"{v:.6g}" for v in r["values"]))
        lines.append("Checked: the result was substituted back into the original equation.")
        a = Answer("solved", question, value=r["value"], message="\n\n".join(lines), models=["formula"])
        if r.get("values") and len(r["values"]) > 1:
            a.values = {f"{r['target']}{i}": v for i, v in enumerate(r["values"], 1)}
        a.uncertainty = {"state": "determined", "capability": "typed_formula", "solution_latex": r["solution"],
                         "scope": "exact algebra on the formula as written; units are not checked on this path"}
        return a

    # --- conceptual questions: explained from the model's own knowledge, never computed ---------------------------
    def _explain(self, question: str):
        from .explain import ConceptIndex, explain, is_conceptual
        if not is_conceptual(question):
            return None
        if getattr(self, "_concepts", None) is None:
            self._concepts = ConceptIndex(self.pack(), self._store())
        got = explain(question, self._concepts, self.domain)
        if got is None:
            return None
        text, key, facet, candidates = got
        if key is None:                                # several capabilities fit equally: list them, choose none
            a = Answer("needs_clarification", question, message="This could be about: " + ", ".join(candidates)
                       + ". Name the one you mean.")
            a.uncertainty = {"state": "ambiguous", "cause": "ambiguous", "outcome": "AMBIGUOUS", "candidates": candidates}
            return a
        a = Answer("solved", question, message=text, models=[key])
        a.uncertainty = {"state": "explained", "capability": key, "facet": facet or "overview",
                         "tier": "explanation: written from the certified capability; prose not independently verified"}
        return a

    # --- the procedure path: a certified algorithm with several outputs ------------------------------------------
    def _solve_procedure(self, key: str, given: dict, question: str | None = None) -> Answer:
        from fractions import Fraction
        spec = self.pack().procedures[key]
        q = question or f"solve({key!r})"
        names = [i["symbol"] for i in spec["inputs"]]

        def refuse(outcome, cause, message, **extra):
            a = Answer("invalid" if outcome == "INVALID_INPUT" else "needs_clarification", q, message=message, models=[key])
            a.uncertainty = {"state": {"NEEDS_INPUT": "missing_information"}.get(outcome, "invalid"), "cause": cause,
                             "outcome": outcome, **extra}
            return a
        from .inputs import STRUCTURED, InputError, coerce, shown as show_input
        from .quantity import UnitError, parse_unit, to_si
        kinds = {i["symbol"]: i.get("kind", "number") for i in spec["inputs"]}
        by_name = {i["symbol"]: i for i in spec["inputs"]}
        values, shown = {}, {}
        for name, v in given.items():
            if name not in names:
                return refuse("INVALID_INPUT", "unknown_name", f"'{name}' is not an input of {key}. It takes: "
                              + ", ".join(f"{i['symbol']} ({i['meaning']})" for i in spec["inputs"]) + ".")
            if by_name[name].get("type", "number") in STRUCTURED:    # a list, matrix, graph, whole number, text, choice
                try:
                    values[name] = coerce(by_name[name], v)
                except InputError as exc:
                    return refuse("INVALID_INPUT", "invalid_input", str(exc) + ".")
                except (TypeError, ValueError):            # a matrix given a flat list or a number, and the like:
                    t = by_name[name].get("type")          # a refusal with a reason, never an escaping exception
                    return refuse("INVALID_INPUT", "invalid_input", f"{name}: expected a {t}, got {v!r}.")
                shown[name] = (show_input(values[name]), "")
                continue
            kind = self.pack().kinds.get(kinds[name]) if kinds[name] != "number" else None
            if kind is not None:                         # a physical input: a number with a unit of its kind
                try:
                    number, unit = self._value(kind.name, v)
                    u = parse_unit(unit)
                except (QuantityError, UnitError, ValueError):
                    return refuse("INVALID_INPUT", "invalid_unit", f"cannot read {name}={v!r}: give a number and a unit of "
                                  f"{kind.name.replace('_', ' ')} (e.g. '3 {kind.unit}').")
                if u.dim != kind.dim:
                    return refuse("INVALID_INPUT", "invalid_unit", f"{name} is a {kind.name.replace('_', ' ')}: its unit "
                                  f"must be like {kind.unit}, not '{unit}'.")
                values[name], shown[name] = to_si(number, u, difference=kind.difference), (number, unit)
                continue
            try:
                values[name] = float(Fraction(v.replace(" ", ""))) if isinstance(v, str) else float(v)
            except (ValueError, TypeError, ZeroDivisionError):
                return refuse("INVALID_INPUT", "invalid_input", f"{name}={v!r} is not a number.")
            shown[name] = (values[name], "")
        needs = [i["symbol"] for i in spec["inputs"] if i.get("required") and i["symbol"] not in values]
        if needs:
            return refuse("NEEDS_INPUT", "needs_input", f"{key} needs {', '.join(needs)} "
                          f"({'; '.join(i['meaning'] for i in spec['inputs'] if i['symbol'] in needs)}). "
                          f"Example: {self.structured_example(key)}.", needs=needs)
        try:
            outputs = spec["run"](**values)
        except ValueError as exc:                        # a stated condition is violated (a = 0 ...)
            return refuse("INVALID_INPUT", "not_applicable", f"{spec['name']}: {exc}.")
        except Exception as exc:
            return self._internal_error(q, exc)
        residual = float(spec["verify"](values, outputs))
        ok = residual <= 1e-9
        primary = outputs.get(spec.get("primary"))
        out_kind = {o["symbol"]: o.get("kind", "number") for o in spec["outputs"]}
        unit_of = lambda s: (self.pack().kinds[out_kind[s]].unit if out_kind.get(s, "number") not in ("number", None)
                             and out_kind[s] in self.pack().kinds else "")     # a pure number carries no unit
        pk = out_kind.get(spec.get("primary"), "number")
        a = Answer("solved" if ok else "verification_failed", q,
                   (primary if ok and isinstance(primary, (int, float)) else None), unit_of(spec.get("primary")),
                   {"kind": pk, "role": "procedure", "phrase": spec["outputs"][0]["meaning"]}, [key],
                   list(spec.get("conditions", [])),
                   [{"relation": key, "name": spec["name"], "equation": spec["method"], "equation_latex": spec["latex"],
                     "solved_latex": ", ".join(outputs), "target": ", ".join(outputs),
                     "result": primary if isinstance(primary, (int, float)) else None, "unit": unit_of(spec.get("primary")),
                     "outputs": dict(outputs), "output_units": {s: unit_of(s) for s in outputs},
                     "inputs": {n: {"value": shown[n][0], "unit": shown[n][1], "source": "stated",
                                    "meaning": next(i["meaning"] for i in spec["inputs"] if i["symbol"] == n)}
                                for n in values}}],
                   {"status": "verified" if ok else "failed", "max_residual": residual, "independent_paths": 1,
                    "scope": "the result substituted back into the defining equation"},
                   values=dict(outputs) if ok else {})
        if not ok:
            a.message = "I computed a value but it did not pass the checks, so I am not showing it."
        a.uncertainty = {"state": "determined", "capability": key} if ok else {"state": "verification_failed",
                                                                                "cause": "computed_value_failed_its_checks"}
        return a

    # --- knowledge lookup: the domain's compact knowledge store (runtime/knowledge_store.py), shipped read-only -----
    def _store(self):
        if getattr(self, "_kstore", None) is None:
            from .knowledge_store import KnowledgeStore
            names = [f"{self.domain}_knowledge_public.sqlite", f"{self.domain}_knowledge.sqlite"]
            cands = [f / n for f in self._folders for n in names] + \
                    [f / "knowledge" / "store" / n for f in self._folders for n in names]   # installed pack, or source tree
            path = next((p for p in cands if p.exists()), None)
            self._kstore = KnowledgeStore(path, readonly=True) if path else False
        return self._kstore or None

    @staticmethod
    def _brief(r: dict) -> dict:
        return {"id": r["id"], "kind": r["kind"], "topic": r.get("topic"), "checked": r["tier"] == "machine_checked",
                "tier": r["tier"], "statement": r["statement"], "equation": r.get("equation") or ""}

    def knowledge(self, query: str, k: int = 5, verified_only: bool = False) -> list:
        """Search the domain's knowledge (keyword search, ranked). `checked` is True only for records the factory
        recomputed independently; other records are well formed but their meaning is NOT verified."""
        st = self._store()
        if st is None:
            return []
        from .retrieval import VERIFIED, Retriever
        hits = Retriever(st).keyword(query, k, tiers=VERIFIED if verified_only else None)
        return [self._brief(h) for h in hits]

    def lookup(self, record_id: str) -> dict | None:
        st = self._store()
        r = st.get(record_id if record_id.startswith(self.domain + ".") else f"{self.domain}.{record_id}") if st else None
        return None if r is None else {**self._brief(r), "latex": r.get("latex") or "", "origin": r.get("origin")}

    def related(self, record_id: str, depth: int = 1) -> list:
        """Concepts, laws and procedures linked to a record (both directions), nearest first."""
        st = self._store()
        if st is None:
            return []
        rid = record_id if record_id.startswith(self.domain + ".") else f"{self.domain}.{record_id}"
        seen, out = set(), []
        for direction in ("out", "in"):
            for hit in st.related(rid, depth=depth, direction=direction):
                r = st.get(hit["id"])
                if r and r["id"] not in seen:
                    seen.add(r["id"])
                    out.append({**self._brief(r), "depth": hit["depth"], "direction": direction})
        return sorted(out, key=lambda x: x["depth"])

    def procedure_examples(self) -> dict:
        return {k: list(v.get("questions", [])) for k, v in self.pack().procedures.items()}

    def _symbol_of(self, rel, name):
        if name in rel.symbols:
            return name
        low = name.strip().lower().replace("_", " ")
        p = self.pack()
        if "/" in name:
            k, r = name.split("/", 1)
            hit = [n for n, s in rel.symbols.items() if s.kind == k and s.role == (r or "plain")]
            return hit[0] if len(hit) == 1 else None
        by_kind = [n for n, s in rel.symbols.items() if s.kind == low.replace(" ", "_")]
        if len(by_kind) == 1:
            return by_kind[0]
        lab = self._labels().get(low)
        if lab:
            exact = [n for n, s in rel.symbols.items() if (s.kind, s.role) == lab]
            same_kind = [n for n, s in rel.symbols.items() if s.kind == lab[0]]
            hit = exact or same_kind
            return hit[0] if len(hit) == 1 else None
        by_desc = [n for n, s in rel.symbols.items() if s.desc and low == s.desc.lower()]
        return by_desc[0] if len(by_desc) == 1 and p else None

    def _internal_error(self, question, exc) -> Answer:
        import traceback
        a = Answer("internal_error", str(question), message="Internal error: this is a defect in the engine, not a property "
                   "of your question. Please report it with the question.")
        a.uncertainty = {"state": "internal_error", "cause": "internal_error", "outcome": "INTERNAL_ERROR",
                         "error": f"{type(exc).__name__}: {exc}", "where": traceback.format_exc().strip().splitlines()[-3:]}
        return a

    # --- documented examples -------------------------------------------------------------------------------------
    def examples(self) -> dict:
        """Verified question templates per capability: {key: [{"q", "value", "unit"}]}."""
        for folder in self._folders:
            path = folder / "knowledge" / "question_examples.json"
            if path.exists():
                import json
                return json.loads(path.read_text(encoding="utf-8"))
        return {}

    def templates(self, key: str | None = None) -> list:
        """Documented question formats per capability: the verified phrasings, each with its structured call."""
        out = []
        for k, rows in self.examples().items():
            if key not in (None, k) or k not in self.pack().by_key:
                continue
            for r in rows:
                out.append({"relation": k, "name": self.pack().by_key[k].name, "example": r["q"],
                            "answer": f"{float(r['value']):.6g} {r['unit']}", "structured": self.structured_example(k)})
        for k, spec in self.pack().procedures.items():
            if key not in (None, k):
                continue
            unit = {i["symbol"]: self.pack().kinds[i["kind"]].unit for i in spec["inputs"]
                    if i.get("kind", "number") not in ("number", "integer") and i["kind"] in self.pack().kinds}
            for t in spec.get("templates", []):            # documented forms only; each with ITS structured call
                if "{" in t:                               # a slotted form, shown with the first worked example
                    from .procedure_spec import render
                    t = render(t, spec["examples"][0]["inputs"], unit)
                inputs = spec["parse"](t) or {}
                # a list, matrix, boolean or text input is shown as Python writes it; a number compactly (a list
                # input once made templates() raise, 2026-10-23)
                show = lambda v: f"{v:g}" if isinstance(v, (int, float)) and not isinstance(v, bool) else repr(v)  # noqa: E731
                out.append({"relation": k, "name": spec["name"], "example": t, "answer": str(self.ask(t)).splitlines()[0],
                            "structured": f"solve({k!r}, " + ", ".join(f"{n}={show(v)}" for n, v in inputs.items()) + ")"})
        return out

    def family_examples(self) -> dict:
        from .pack import Pack as _P
        mod = _P._DOMAIN_FAMILIES.get(self.pack().domain)
        return dict(getattr(mod, "FAMILY_EXAMPLES", {}))

    def structured_example(self, key: str) -> str:
        """A copyable solve() call for a capability, from its first hand-worked case."""
        if key in self.pack().procedures:
            ex = self.pack().procedures[key]["examples"][0]["inputs"]
            spec = self.pack().procedures[key]
            unit = {i["symbol"]: self.pack().kinds[i["kind"]].unit for i in spec["inputs"]
                    if i.get("kind", "number") != "number" and i["kind"] in self.pack().kinds}
            show = lambda n, v: repr(f"{v:g} {unit[n]}") if n in unit else (f"{v:g}" if isinstance(v, (int, float)) else repr(v))
            return f"solve({key!r}, " + ", ".join(f"{n}={show(n, v)}" for n, v in ex.items()) + ")"
        rel = self.pack().by_key[key]
        lhs = rel.equation.split("=")[0].strip()
        inputs = [n for n in rel.symbols if n != lhs and not rel.symbols[n].default]
        unit = lambda n: self.pack().kinds[rel.symbols[n].kind].unit   # noqa: E731
        return f"solve({key!r}, " + ", ".join(f"{n}='1 {unit(n)}'" for n in inputs) + ")"

    def _si_value(self, v):
        """'6.371e6 m' or [6.371e6, 'm'] -> SI float; a bare number is taken as SI."""
        from .quantity import parse_unit, to_si
        if isinstance(v, (int, float)):
            return float(v)
        if isinstance(v, (list, tuple)):
            return to_si(float(v[0]), parse_unit(str(v[1])))
        num, _, unit = str(v).strip().partition(" ")
        return to_si(float(num), parse_unit(unit.strip())) if unit.strip() else float(num)

    def plot_relation(self, key: str, x: str, over, fixed: dict | None = None, y: str | None = None, points: int = 101):
        """A deterministic plot of one certified relation (see runtime/visual.py): `y` against `x` over `over`,
        the other symbols `fixed` (values with units, or SI numbers); constants take their declared values."""
        from .visual import relation_plot
        p = self.pack()
        r = p.by_key[key]
        fx = {k: self._si_value(v) for k, v in (fixed or {}).items()}
        for n, s in r.symbols.items():                     # a declared constant (G, g) fills itself in
            if n not in fx and n != x and n != (y or r.equation.split("=")[0].strip()) and s.default:
                fx[n] = self._si_value([s.default["value"], s.default["unit"]])
        return relation_plot(p, key, x, (self._si_value(over[0]), self._si_value(over[1])), fx, y=y, points=points)

    def plot(self, answer, span: float = 1.0, points: int = 101):
        """The answer's last step around its own result: its result against its first input, ±span × that input."""
        from .visual import relation_plot
        if not answer.solved or not answer.steps:
            raise ValueError("only a solved answer can be plotted")
        s = answer.steps[-1]
        if s.get("relation") not in self.pack().by_key:
            raise ValueError("this answer comes from an algorithm, not a formula: there is no curve to draw around it")
        numeric = {k: v for k, v in s["inputs"].items() if isinstance(v.get("value"), (int, float))}
        stated = [k for k, v in numeric.items() if v.get("source") not in ("default", "implied")]
        if not (stated or numeric):
            raise ValueError("this answer has no numeric input to plot against")
        x = (stated or list(numeric))[0]                # a stated value, never a declared constant like G
        xv = float(s["inputs"][x]["value"])
        fixed = {k: float(v["value"]) for k, v in s["inputs"].items() if k != x}
        lo, hi = (xv - span * abs(xv), xv + span * abs(xv)) if xv else (-1.0, 1.0)
        lo = max(lo, xv * 1e-3) if xv > 0 else lo
        return relation_plot(self.pack(), s["relation"], x, (lo, hi), fixed, y=s["target"], points=points,
                             marker=(xv, float(s["result"])))

    def to_si(self, value, unit, contract=None, symbol=None) -> float:
        from .quantity import parse_unit, to_si
        kind = None
        if contract and symbol:
            kind = next((pp["kind"] for pp in contract["inputs"] if pp["symbol"] == symbol), None)
        diff = bool(kind and kind in self.pack().kinds and self.pack().kinds[kind].difference)
        return to_si(float(value), parse_unit(unit), difference=diff)

    def _fitting_needs(self, question: str):
        """The certified relation that FITS the question: it produces the asked quantity, its context conditions are
        stated, it uses every stated quantity, and it lacks at most two inputs (declared defaults fill constants).
        None when no relation fits: then nothing is asked (an unrelated formula is never a reason to ask)."""
        p = self.pack()
        prob = compile_problem(question, p)
        if prob.target is None:
            return None
        tags, givens, best = set(prob.tags), list(prob.givens), None
        for rel in p.relations:
            if not set(rel.requires) <= tags or set(rel.excludes) & tags:
                continue
            tname = next((n for n, s in rel.symbols.items() if s.kind == prob.target.kind and s.takes(prob.target.role)), None)
            if tname is None:
                continue
            free, missing = list(givens), []
            for n, s in rel.symbols.items():
                if n == tname:
                    continue
                g = next((g for g in free if g.kind == s.kind and s.takes(g.role)), None)
                if g is not None:
                    free.remove(g)
                elif not (s.default and not set(s.default.get("unless", ())) & tags):
                    missing.append(n)
            if any(g.source == "stated" for g in free) or not missing or len(missing) > 2:
                continue
            if best is None or len(missing) < len(best[1]):
                best = (rel, missing)
        if best is None:
            return None
        rel, missing = best
        return {"key": rel.key, "name": rel.name, "needs": missing,
                "needs_words": [f"{rel.symbols[n].desc or rel.symbols[n].kind.replace('_', ' ')} ({p.kinds[rel.symbols[n].kind].unit})"
                                for n in missing]}

    def _nearest_template(self, question: str):
        words = set(re.findall(r"[a-z]{4,}", question.lower()))
        best, score = None, 0
        for key, rows in self.examples().items():
            rel = self.pack().by_key.get(key)
            if rel is None or not rows:
                continue
            vocab = set(re.findall(r"[a-z]{4,}", (rel.name + " " + " ".join(s.desc for s in rel.symbols.values())).lower()))
            s = len(words & vocab)
            if s > score:
                best, score = key, s
        if best is None:
            return None
        return {"capability": best, "question": self.examples()[best][0]["q"], "structured": self.structured_example(best)}

    def problem(self, find: str, given: dict, conditions=()) -> dict:
        tk, tr = self._resolve(find)
        out = {"target": {"kind": tk, "role": tr}, "given": [], "tags": sorted({self._tag(c) for c in conditions})}
        for name, v in given.items():
            k, r = self._resolve(name)
            value, unit = self._value(k, v)
            out["given"].append({"kind": k, "role": r, "value": value, "unit": unit})
        return out

    # --- discovery ---------------------------------------------------------------------------------------------
    def quantities(self) -> list:
        p, by = self.pack(), {}
        for e in p.terms.get("labels", []):
            by.setdefault(e["kind"], set()).add(e["phrase"])
        return [{"kind": k, "unit": v.unit, "roles": sorted(p.roles_of(k)), "names": sorted(by.get(k, ()))}
                for k, v in sorted(p.kinds.items())]

    def conditions(self) -> list:
        by = {}
        for x in self.pack().terms.get("context", []):
            by.setdefault(x["tag"], []).append(x["phrase"])
        return [{"condition": t, "phrases": sorted(set(v))} for t, v in sorted(by.items())]

    def formulas(self, family: str | None = None) -> list:
        return [self.formula(r.key) for r in self.pack().relations if family is None or r.family == family]

    def formula(self, key: str) -> dict:
        p = self.pack()
        r = p.by_key.get(key)
        if r is None:
            raise KeyError(f"no relation '{key}'; see {self.domain}_ai999x.formulas()")
        return {"key": r.key, "name": r.name, "family": r.family, "equation": r.equation, "latex": r.latex(),
                "symbols": {n: {"kind": s.kind, "role": s.role, "unit": p.kinds[s.kind].unit, "meaning": s.desc,
                                "default": s.default} for n, s in r.symbols.items()},
                "assumptions": list(r.assumptions), "requires": list(r.requires), "excludes": list(r.excludes),
                "applies_when": list(r.applies_when), "fails_when": list(r.fails_when), "frame": r.frame,
                "conventions": list(r.conventions), "derived_from": list(r.derived_from), "shortcut": r.shortcut}

    def capabilities(self) -> list:
        """One entry per certified relation and per model family of the domain."""
        p = self.pack()
        out = [{"type": "relation", "family": r.family, "relation": r.key, "name": r.name, "equation": r.equation,
                "assumptions": list(r.assumptions)} for r in p.relations]
        out += [{"type": "model_family", "family": f["key"], "name": f["title"], "relations_used": sorted(f["uses"])}
                for f in p.family_index()]
        out += [{"type": "procedure", "family": v.get("topic", ""), "procedure": k, "name": v["name"], "method": v["method"]}
                for k, v in p.procedures.items()]
        return out

    # --- help: the guide entry of a capability, offline, from the pack itself --------------------------------------
    def capability_ids(self) -> list:
        """Every capability `solve()` takes by id: certified relations and procedures, in a stable order."""
        p = self.pack()
        return sorted(set(p.by_key) | set(getattr(p, "procedures", {}) or {}))

    def help(self, name: str | None = None) -> str:
        """What a capability computes, its formula, every input with its meaning, unit and condition, its outputs and
        a ready call — or, without a name, how to find one. A misspelt name gets the closest matches."""
        import difflib
        p = self.pack()
        dist = f"{self.domain}_ai999x"
        if not name:
            return (f"{dist}: {len(self.capability_ids())} capabilities. {dist}.capabilities() lists them by topic; "
                    f"{dist}.help('<id>') explains one; {dist}.templates() shows every question form ask() reads; "
                    f"tab completion offers each as {dist}.<id>(...).")
        key = name.strip().replace(" ", "_").replace("-", "_")
        procs = getattr(p, "procedures", {}) or {}
        if key not in p.by_key and key not in procs:
            names = {i: i for i in self.capability_ids()}
            names.update({(r.name or "").lower(): r.key for r in p.relations})
            names.update({(v.get("name") or "").lower(): k for k, v in procs.items()})
            close = difflib.get_close_matches(name.strip().lower().replace("_", " "), [n.replace("_", " ") for n in names], n=5, cutoff=0.45)
            hits = list(dict.fromkeys(names[c.replace(" ", "_")] if c.replace(" ", "_") in names else names[c] for c in close))
            return (f"'{name}' is not a capability of {dist}."
                    + (f" Did you mean: {', '.join(hits)}?" if hits else f" See {dist}.capabilities()."))
        lines = []
        if key in procs:
            v = procs[key]
            lines += [f"{v['name']} — {key}", "", v.get("purpose", ""), ""]
            if v.get("latex"):
                lines += [f"Formula (LaTeX): {v['latex']}", ""]
            lines.append("Inputs:")
            for i in v["inputs"]:
                lines.append(f"  {i['symbol']}: {i.get('meaning', '')}" + (f"  [condition: {i['condition']}]" if i.get("condition") else "")
                             + ("" if i.get("required", True) else "  (optional)"))
            lines.append("Outputs:")
            lines += [f"  {o['symbol']}: {o.get('meaning', '')}" for o in v["outputs"]]
            if v.get("method"):
                lines += ["", f"Method: {v['method']}"]
        else:
            f = self.formula(key)
            lines += [f"{f['name']} — {key}", "", f"Equation: {f['equation']}", f"Formula (LaTeX): {f['latex']}", "",
                      "Quantities (give any but the one to find; name them by symbol or by kind, with units):"]
            for s, d in f["symbols"].items():
                lines.append(f"  {s}: {d['meaning'] or d['kind'].replace('_', ' ')} — {d['kind'].replace('_', ' ')}"
                             + (f"/{d['role']}" if d["role"] != "plain" else "") + f" [{d['unit']}]")
            for label, items in (("Applies when", f["applies_when"]), ("Assumes", f["assumptions"]), ("Fails when", f["fails_when"])):
                if items:
                    lines.append(f"{label}: {'; '.join(items)}")
        try:
            lines += ["", "Example:", f"  {dist}.{self.structured_example(key).replace(f'{self.domain}_ai999x.', '')}"]
        except Exception:
            pass
        return "\n".join(x for x in lines if x is not None)

    # --- helpers -----------------------------------------------------------------------------------------------
    def _labels(self) -> dict:
        out = {}
        for e in self.pack().terms.get("labels", []):
            out.setdefault(e["phrase"].lower(), (e["kind"], e.get("role", "plain")))
        for idiom in self.pack().terms.get("ask_idioms", []):
            out.setdefault(idiom["phrase"].lower(), (idiom["kind"], idiom.get("role", "plain")))
        return out

    def _resolve(self, name: str) -> tuple:
        p = self.pack()
        key = name.strip().lower().replace("_", " ")
        raw = name.strip()
        if "/" in raw:
            kind, role = raw.split("/", 1)
            if kind in p.kinds:
                return kind, role or "plain"
        if raw in p.kinds:
            return raw, "plain"
        if key.replace(" ", "_") in p.kinds:
            return key.replace(" ", "_"), "plain"
        labels = self._labels()
        if key in labels:
            return labels[key]
        close = sorted(k for k in labels if key in k or k in key)[:5]
        raise QuantityError(f"unknown quantity '{name}'" + (f"; did you mean: {', '.join(close)}?" if close else
                                                           f"; see {self.domain}_ai999x.quantities()"))

    def _value(self, kind: str, v) -> tuple:
        unit_si = self.pack().kinds[kind].unit
        if isinstance(v, (int, float)):
            return float(v), unit_si
        if isinstance(v, (tuple, list)) and len(v) == 2:
            return float(v[0]), str(v[1])
        if isinstance(v, str):
            m = re.match(r"\s*([-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?)\s*(.*)$", v)
            if m:
                return float(m.group(1)), (m.group(2).strip() or unit_si)
        raise QuantityError(f"cannot read the value {v!r}: write a number with its unit, e.g. '3 kg' or (3, 'kg')")

    def _tag(self, c: str) -> str:
        p = self.pack()
        tags = {x["tag"] for x in p.terms.get("context", [])} | set((p.terms.get("kind_tags") or {}).values())
        if c in tags:
            return c
        low = c.strip().lower()
        for x in p.terms.get("context", []):
            if x["phrase"].lower() == low:
                return x["tag"]
        raise QuantityError(f"unknown condition '{c}'; see {self.domain}_ai999x.conditions()")
