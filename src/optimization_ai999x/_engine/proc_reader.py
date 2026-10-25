# GENERATED from ai999x_factory/runtime by tools/build_package.py — do not edit.
"""The trained question reader for PROCEDURES (the shared reader's procedure extension). It only proposes a reading;
the engine computes; every gate below must pass or the question is refused.

    from ai999x_factory.runtime.proc_reader import ProcReader
    reader = ProcReader("<folder with reader.int8.onnx, tokenizer.json, labels.json>")
    reading = reader.read(question, domain, pack)      # Reading or Refusal; never a number

What the model decides (training/train_proc_reader.py, our encoder ai999x-encoder-2 plus three heads):
  capability   among this domain's certified procedures (masked to the domain: the package knows its domain);
  asked        which output is asked, among the predicted capability's outputs;
  spans        a BIO tag per token naming the input symbol, among the predicted capability's inputs.
What it never decides: a value. Each tagged span's text is copied EXACTLY as written; a physical value keeps the unit
written with it, so the procedure path converts it once (Product._solve_procedure, the same path as solve()).

The gates (each refusal names its reason; `Product.ask` adds the last two):
  1. confidence (the smallest probability among its decisions) >= the threshold chosen on VALIDATION (labels.json);
  2. a span starts and ends on word boundaries; one span per input; tags form valid B/I runs;
  3. every number written in the question lies inside a tagged span, or is notation (an exponent: m^2, 10^-3; a
     size: 3x3, 2-by-2; a name: x1, H0, CO2): every stated number is bound;
  4. a sign is never dropped (a minus or a sign word just before an unsigned span);
  5. a physical input carries its complete unit (in the span or written right after it, read by parse_unit with the
     input's dimension); a pure number is followed by no unit or scale word other than those seen after that input
     in training ("5 degrees" for an angle in degrees: yes; "5 inches" where the training wrote meters: no);
  6. when an optional input is not read, the question writes no number word or ordinal outside the spans (a
     "second derivative" must not silently become the default order);
  7. the asked output is not contradicted by the wording (asked_supported: an output's distinctive words), and a
     negated or complementary asked phrase ("not", "no", "fail", "lose", "other than") is answered only by an output
     whose symbol or meaning encodes a complement (negation_supported);
  8. [Product] every required input is present, and 9. [Product] the outcome is SOLVED and verified.

Off unless the reader files are present and the model's package enables it (Product._proc_reader). The optional
dependencies (onnxruntime, tokenizers, numpy) are imported in the constructor only. No network.
"""
from __future__ import annotations

import json
import math
import re
from dataclasses import dataclass, field
from pathlib import Path

MAX_LEN = 256
MODEL_FILE = "reader.int8.onnx"


@dataclass
class Reading:
    capability: str
    asked: str
    inputs: dict                         # symbol -> text exactly as the question writes it (a unit appended if written after)
    confidence: float
    spans: list = field(default_factory=list)       # [(start, end, symbol)]
    detail: dict = field(default_factory=dict)


@dataclass
class Refusal:
    reason: str
    confidence: float = 0.0
    detail: dict = field(default_factory=dict)


# --- model inputs (shared with the trainer, so training and inference encode a question identically) -----------------
def build_inputs(tok, domain: str, text: str, max_len: int = MAX_LEN):
    """([CLS] domain words [SEP] question tokens [SEP], index of the first question token, the question tokens'
    character offsets, truncated?). `tok` must have truncation and padding off."""
    cls_id, sep_id = tok.token_to_id("[CLS]"), tok.token_to_id("[SEP]")
    prefix = tok.encode(domain.replace("_", " ")).ids
    e = tok.encode(text)
    room = max_len - len(prefix) - 3
    n = min(len(e.ids), room)
    ids = [cls_id] + prefix + [sep_id] + list(e.ids[:n]) + [sep_id]
    return ids, len(prefix) + 2, [tuple(o) for o in e.offsets[:n]], len(e.ids) > room


def tag_names(symbols: list) -> list:
    """O, then B-<symbol>, I-<symbol> for every symbol in order."""
    out = ["O"]
    for s in symbols:
        out += [f"B-{s}", f"I-{s}"]
    return out


def next_word(text: str, end: int) -> str:
    """The word written right after position `end` (a '%' counts as a word), lowercased; '' at a punctuation mark."""
    m = re.match(r"[ \t]*(%|‰|[A-Za-zµ°Ω$€£][\w°µΩ'/^\-]*)", text[end:])
    return m.group(1).lower().rstrip("'-") if m else ""


# --- the gates on text (pure functions; no model needed; tests/test_proc_reader.py) -----------------------------------
_NUMBER = re.compile(r"\d+(?:\.\d+)?(?:[eE][-+]?\d+)?")
_SIZE = re.compile(r"\d+\s*(?:[x×X]|-?\s*by\s*-?)\s*\d+(?:\s*(?:[x×X]|-?\s*by\s*-?)\s*\d+)*")
_SIGN_WORDS = {"minus", "negative", "below", "loss", "losses", "deficit", "decrease", "decreases", "decreased",
               "drop", "drops", "dropped", "fall", "falls", "fell", "down", "downward", "downwards", "lower", "lowered",
               "reduction", "debt", "withdraw", "withdrawal", "withdrawn", "south", "west", "deficit", "outflow",
               "subtract", "subtracted", "less", "under"}
_NUMBER_WORDS = re.compile(
    r"\b(zero|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|twenty|thirty|forty|fifty|hundred|"
    r"thousand|million|billion|trillion|dozen|half|twice|thrice|double|triple|quadruple|first|second|third|fourth|"
    r"fifth|sixth|seventh|eighth|ninth|tenth|\d+(?:st|nd|rd|th))\b", re.I)
_SCALE_WORDS = {"%", "‰", "percent", "per", "pct", "million", "millions", "billion", "billions", "thousand",
                "thousands", "trillion", "hundred", "k", "bn", "mn", "bps", "basis", "dozen"}


def spans_from_tags(tags: list, offsets: list, text: str):
    """[(start, end, symbol)] from per-token tags ('O', 'B-x', 'I-x'), or a Refusal. Tokens with empty offsets are skipped."""
    spans, cur = [], None
    for tag, (s, e) in zip(tags, offsets):
        if e <= s:
            continue
        if tag == "O":
            if cur:
                spans.append(tuple(cur))
            cur = None
            continue
        bio, sym = tag.split("-", 1)
        if bio == "B":
            if cur:
                spans.append(tuple(cur))
            cur = [s, e, sym]
        elif cur is not None and cur[2] == sym:
            cur[1] = e
        else:
            return Refusal(f"the tags do not form a span for '{sym}' (an I tag without its B)")
    if cur:
        spans.append(tuple(cur))
    syms = [x[2] for x in spans]
    dup = sorted({x for x in syms if syms.count(x) > 1})
    if dup:
        return Refusal(f"two spans read as the same input: {', '.join(dup)}")
    for s, e, sym in spans:
        if (s > 0 and text[s - 1].isalnum() and text[s].isalnum()) or \
                (e < len(text) and text[e].isalnum() and text[e - 1].isalnum()):
            return Refusal(f"the span for '{sym}' cuts a word: '{text[max(0, s - 3):e + 3]}'")
    return spans


def unbound_numbers(text: str, spans: list) -> list:
    """Numbers written in the question that no span holds and that are not notation."""
    inside = lambda a, b: any(s <= a and b <= e for s, e, _ in spans)      # noqa: E731
    notation = set()
    for m in _SIZE.finditer(text):                                          # 3x3, 2 x 4, 3-by-3
        if not (m.start() > 0 and (text[m.start() - 1].isalnum() or text[m.start() - 1] == ".")):
            notation.update(range(m.start(), m.end()))
    out = []
    for m in _NUMBER.finditer(text):
        a, b = m.start(), m.end()
        if inside(a, b) or a in notation:
            continue
        before = text[:a]
        if before[-1:].isalpha() or before[-1:] == "_":                     # a name: x1, H0, CO2, m_2
            continue
        if re.search(r"(\^|\*\*)\s*[-+−(]?\s*$", before) or re.search(r"\^\{\s*[-+−]?\s*$", before):   # an exponent
            continue
        if re.search(r"[A-Za-z]_\{?$", before):
            continue
        out.append(m.group())
    return out


def dropped_sign(text: str, s: int, e: int) -> str:
    """A sign the span leaves outside it: '-5' read as '5', or 'minus 5', 'a loss of 5'. '' when none."""
    span = text[s:e]
    if not span[:1].isdigit() and span[:1] != ".":
        return ""
    if s >= 1 and text[s - 1] in "-−–+" and (s < 2 or not text[s - 2].isalnum()):
        return text[s - 1]
    words = re.findall(r"[A-Za-z]+", text[max(0, s - 40):s].lower())[-2:]
    hit = [w for w in words if w in _SIGN_WORDS]
    return hit[0] if hit else ""


def unit_phrase(text: str, s: int, e: int, dim, parse_unit) -> tuple:
    """(value text with its complete unit, None) or (None, reason) for a physical input.

    The span's number with the unit written in the span, else the longest run of up to five words written right
    after it that parse_unit reads; the result must be a unit of the input's dimension, and no longer reading of the
    words after the span may exist (an incomplete unit: '5 kg' of '5 kg/m^3')."""
    span = text[s:e]
    m = re.match(r"\s*[-+−]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?", span)
    if not m:
        return None, f"'{span}' is not a number"
    number, unit_in_span = span[:m.end()].strip(), span[m.end():].strip()
    if e < len(text) and text[e] in "^/·*²³⁻¹":
        return None, f"the unit of '{span}' continues after it"
    words = re.findall(r"\S+", text[e:e + 80])[:5] if text[e:e + 1].isspace() else []

    def parses(u):
        """(Unit, spelling parse_unit reads) for a unit as written ('kg/m^3') or spelled out ('kilograms'), else None."""
        from .quantity import words_to_unit
        for spelling in (u, words_to_unit(u.split()) if u.strip() else None):
            if spelling:
                try:
                    return parse_unit(spelling), spelling
                except Exception:
                    pass
        return None
    best_k, best = 0, None
    for k in range(1, len(words) + 1):
        if k > 1 and words[k - 2] != words[k - 2].rstrip(",.;:?!)"):
            break                                          # a clause ends before this word
        cand = (unit_in_span + " " + " ".join(words[:k]).rstrip(",.;:?!)")).strip()
        got = parses(cand)
        if got is not None:
            best_k, best = k, got
    follow = words[best_k].lower().strip(",.;:?!()") if best_k < len(words) else ""
    if follow in ("per", "squared", "cubed", "square", "cubic", "times") or follow.startswith("/") or \
            (best_k and words[best_k - 1].endswith(("/", "-"))):
        return None, f"the unit of '{span}' continues in words the reader cannot read"
    if unit_in_span:
        if best_k:
            return None, f"the unit of '{span}' continues after it"
        got = parses(unit_in_span)
        if got is None:
            return None, f"'{unit_in_span}' is not a unit"
        if got[0].dim != dim:
            return None, f"'{unit_in_span}' is not a unit of this input's kind"
        return f"{number} {got[1]}", None
    if best is None:
        return None, f"'{span}' is written without its unit"
    if best[0].dim != dim:
        return None, f"the unit after '{span}' is not a unit of this input's kind"
    return f"{number} {best[1]}", None


_STOP = {"the", "of", "a", "an", "in", "and", "to", "for", "is", "at", "by", "on", "with", "as", "its", "or", "from",
         "be", "this", "that", "each", "value"}


def _words(text: str) -> set:
    out = set()
    for w in re.split(r"[^a-z0-9]+", text.lower()):
        if len(w) < 2 or w in _STOP:
            continue
        out.add(w[:5])                                     # a crude stem: produced / production, kilometres / km...
    return out


def asked_supported(question: str, outputs: list, asked: str) -> str:
    """'' when the wording does not contradict the asked output, else the reason. Each output's DISTINCTIVE words
    (in its symbol and meaning, in no other output's) are looked for in the question: the asked output must have the
    most such words found, and an output with qualifiers none of which is written ('in kilometres', 'erfc',
    'left-tail', 'per day') is refused when a plain variant without qualifiers exists."""
    if len(outputs) < 2:
        return ""
    words = {o["symbol"]: _words(o["symbol"].replace("_", " ") + " " + o.get("meaning", "")) for o in outputs}
    q = _words(question)
    distinct = {s: w - set().union(*(v for t, v in words.items() if t != s)) for s, w in words.items()}
    hits = {s: len(d & q) for s, d in distinct.items()}
    best = max(hits.values())
    if best and (hits.get(asked, 0) < best or sum(h == best for h in hits.values()) > 1):
        other = max((s for s in hits if s != asked), key=hits.get)
        return f"the wording points to the output '{other}' at least as much as to '{asked}'"
    if not hits.get(asked) and distinct.get(asked) and any(not d for s, d in distinct.items() if s != asked):
        return f"'{asked}' is a qualified output ({', '.join(sorted(distinct[asked])[:3])}) the question does not name"
    return ""


_ASK_CUE = re.compile(r"\b(what|find|calculate|compute|determine|how|which|estimate|evaluate|give|obtain|work out)\b",
                      re.I)
_NEGATED = re.compile(r"\b(not|no|none|never|neither|nor|fail|fails|failing|failed|lose|loses|losing|complement|"
                      r"complementary|other\s+than)\b|n't\b", re.I)
_COMPLEMENT_MARK = re.compile(r"\b(not|no|non|none|never|fail\w*|lose|loss\w*|lost|complement\w*|against|other|"
                              r"without|survival|surviv\w*)\b|\b1\s*-\s*|n't\b", re.I)


def asked_clause(question: str) -> str:
    """The part of the question that asks: from the last asking word (what, find, calculate, ...) to the end; the
    last sentence when no such word is written."""
    hits = list(_ASK_CUE.finditer(question))
    if hits:
        return question[hits[-1].start():]
    parts = [p for p in re.split(r"(?<=[.?!])\s+", question.strip()) if p]
    return parts[-1] if parts else question


def negation_supported(question: str, outputs: list, asked: str) -> str:
    """'' unless the ASKED phrase is negated or complemented ('the probability of NOT winning', 'other than', 'fail',
    'lose') while the chosen output encodes no complement in its symbol or meaning; then the reason. The probability
    of winning was once given for 'the probability of not winning' at confidence 0.999999 (reader-2's test split)."""
    clause = asked_clause(question)
    neg = _NEGATED.search(clause)
    if not neg:
        return ""
    out = next((o for o in outputs if o["symbol"] == asked), {"symbol": asked})
    text = out["symbol"].replace("_", " ") + " " + out.get("meaning", "")
    if _COMPLEMENT_MARK.search(text):
        return ""
    return f"the question asks a negated or complementary quantity ('{neg.group().strip()}') and '{asked}' is not one"


def check_reading(text: str, spans: list, inputs_spec: list, pack, unit_words: dict | None = None) -> tuple:
    """The text gates 3-6 on a decoded reading: ({symbol: value text}, None) or (None, reason)."""
    from .inputs import STRUCTURED
    from .quantity import parse_unit
    loose = unbound_numbers(text, spans)
    if loose:
        return None, "a number in the question is not read as any input: " + ", ".join(loose[:5])
    by = {i["symbol"]: i for i in inputs_spec}
    values = {}
    for s, e, sym in spans:
        spec = by.get(sym)
        if spec is None:
            return None, f"'{sym}' is not an input of this capability"
        sign = dropped_sign(text, s, e)
        if sign:
            return None, f"a sign ('{sign}') stands before '{text[s:e]}' outside what was read"
        typ, kind = spec.get("type", "number"), spec.get("kind", "number")
        physical = typ not in STRUCTURED and kind != "number" and kind in pack.kinds and pack.kinds[kind].unit not in ("1", "")
        if physical:
            v, why = unit_phrase(text, s, e, pack.kinds[kind].dim, parse_unit)
            if v is None:
                return None, why
            values[sym] = v
            continue
        if typ in ("number", "integer"):
            w = next_word(text, e)
            seen = set((unit_words or {}).get(sym, ()))
            if w and w not in seen:
                try:
                    is_unit = parse_unit(w) is not None
                except Exception:
                    is_unit = False
                if is_unit or w in _SCALE_WORDS:
                    return None, f"'{text[s:e]} {w}': a unit or scale this input was never read with"
        values[sym] = text[s:e]
    read = set(values)
    rest = "".join(" " if any(s <= k < e for s, e, _ in spans) else ch for k, ch in enumerate(text))
    for i in inputs_spec:                     # an unread optional choice whose option is written ("... of a proton")
        if not i.get("required") and i["symbol"] not in read and i.get("type") in ("choice", "boolean", "text"):
            opts = [str(c) for c in i.get("choices", [])] if i.get("type") == "choice" else []
            hit = [c for c in opts if re.search(r"(?<![A-Za-z])" + re.escape(c) + r"(?:e?s)?(?![A-Za-z])", rest, re.I)]
            if hit or i.get("type") != "choice":
                return None, (f"the optional input '{i['symbol']}' is not read and the question says '{hit[0]}'" if hit
                              else f"the optional input '{i['symbol']}' ({i.get('type')}) is not read")
    if any(not i.get("required") and i["symbol"] not in read for i in inputs_spec):
        w = _NUMBER_WORDS.search(rest)
        if w:
            return None, f"an optional input is not read and the question says '{w.group()}'"
    return values, None


class ProcReader:
    def __init__(self, folder, model: str = MODEL_FILE):
        import numpy as np
        import onnxruntime as ort
        from tokenizers import Tokenizer
        folder = Path(folder)
        self.folder = folder
        self.labels = json.loads((folder / "labels.json").read_text(encoding="utf-8"))
        self.threshold = self.labels.get("threshold")
        self.capabilities = self.labels["capabilities"]
        self.outputs = self.labels["outputs"]
        self.tags = tag_names(self.labels["symbols"])
        self._tag_index = {t: i for i, t in enumerate(self.tags)}
        self._out_index = {o: i for i, o in enumerate(self.outputs)}
        self.max_len = int(self.labels.get("max_len", MAX_LEN))
        self.tok = Tokenizer.from_file(str(folder / "tokenizer.json"))
        self.tok.no_padding()
        self.tok.no_truncation()
        opts = ort.SessionOptions()
        opts.intra_op_num_threads = int(self.labels.get("threads", 2))
        self.session = ort.InferenceSession(str(folder / model), opts, providers=["CPUExecutionProvider"])
        self._np = np

    @staticmethod
    def _softmax(row, allowed):
        vals = [float(row[i]) for i in allowed]
        m = max(vals)
        ex = [math.exp(v - m) for v in vals]
        s = sum(ex)
        return [x / s for x in ex]

    def logits(self, question: str, domain: str):
        np = self._np
        ids, first, offsets, truncated = build_inputs(self.tok, domain, question, self.max_len)
        arr = np.array([ids], dtype=np.int64)
        cap, asked, tags = self.session.run(None, {"input_ids": arr, "attention_mask": np.ones_like(arr)})
        return cap[0], asked[0], tags[0][first:first + len(offsets)], offsets, truncated

    def read(self, question: str, domain: str, pack, threshold=None):
        """A Reading that passed gates 1-6, or a Refusal with its reason."""
        thr = self.threshold if threshold is None else threshold
        procs = pack.procedures
        allowed_c = [i for i, c in enumerate(self.capabilities)
                     if c.split("/", 1)[0] == domain and c.split("/", 1)[1] in procs]
        if not allowed_c:
            return Refusal("the trained reader knows no capability of this domain")
        # the reader answers for a domain only when it was trained on EVERY certified capability of it: a capability it
        # never saw cannot be chosen, so its questions were read as a known neighbour with full confidence ("strictly
        # greater than" answered by exponential_probability_below while _above was untrained; statistics, 2026-10-24)
        known = {self.capabilities[i].split("/", 1)[1] for i in allowed_c}
        unseen = sorted(set(procs) - known)
        if unseen:
            return Refusal(f"the trained reader was not trained on {len(unseen)} capabilities of this domain "
                           f"(e.g. {unseen[0]}); it stays silent until retrained")
        cap_l, asked_l, tag_l, offsets, truncated = self.logits(question, domain)
        if truncated:
            return Refusal(f"the question is longer than the {self.max_len} tokens the reader can see")
        p = self._softmax(cap_l, allowed_c)
        j = max(range(len(p)), key=p.__getitem__)
        cap = self.capabilities[allowed_c[j]]
        key = cap.split("/", 1)[1]
        spec = procs[key]
        conf, detail = p[j], {"capability": round(p[j], 4)}
        outs = [o["symbol"] for o in spec["outputs"] if o["symbol"] in self._out_index]
        if not outs:
            return Refusal(f"the reader knows no output of {key}", conf, detail)
        allowed_o = [self._out_index[o] for o in outs]
        pa = self._softmax(asked_l, allowed_o)
        k = max(range(len(pa)), key=pa.__getitem__)
        asked = outs[k]
        conf = min(conf, pa[k])
        detail["asked"] = round(pa[k], 4)
        ins = [i["symbol"] for i in spec["inputs"] if f"B-{i['symbol']}" in self._tag_index]
        allowed_t = [0] + [self._tag_index[f"{b}-{s}"] for s in ins for b in ("B", "I")]
        tags, tag_conf = [], 1.0
        for row, (s, e) in zip(tag_l, offsets):
            pt = self._softmax(row, allowed_t)
            t = max(range(len(pt)), key=pt.__getitem__)
            tags.append(self.tags[allowed_t[t]])
            if e > s and (tags[-1] != "O" or any(ch.isdigit() for ch in question[s:e])):
                tag_conf = min(tag_conf, pt[t])
        conf = min(conf, tag_conf)
        detail["tags"] = round(tag_conf, 4)
        spans = spans_from_tags(tags, offsets, question)
        if isinstance(spans, Refusal):
            spans.confidence, spans.detail = conf, detail
            return spans
        if thr is None or conf < thr:
            return Refusal(f"confidence {conf:.4f} is below the threshold {thr}", conf,
                           {**detail, "capability_key": key, "asked_symbol": asked,
                            "spans": {sym: question[s:e] for s, e, sym in spans}})
        values, why = check_reading(question, spans, spec["inputs"], pack,
                                    (self.labels.get("unit_words") or {}).get(cap))
        if values is None:
            return Refusal(why, conf, {**detail, "capability_key": key, "asked_symbol": asked,
                                       "spans": {sym: question[s:e] for s, e, sym in spans}})
        why = asked_supported(question, spec["outputs"], asked) or negation_supported(question, spec["outputs"], asked)
        if why:
            return Refusal(why, conf, {**detail, "capability_key": key, "asked_symbol": asked})
        return Reading(key, asked, values, conf, spans, detail)
