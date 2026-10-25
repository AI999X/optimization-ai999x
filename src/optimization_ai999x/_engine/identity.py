# GENERATED from ai999x_factory/runtime by tools/build_package.py — do not edit.
"""Who made this package: one source of truth, answered deterministically by every domain package.

    about()                      -> {"name", "creator", "role", "company", "version", ...}
    ask("Who created you?")      -> SOLVED, the same sentence every time

Documented forms: who created / made / built / developed / engineered / designed you (or this, or it); who is your
creator / engineer / developer / author; which (or what) company / organization made you; who owns you; what are you.
"""
from __future__ import annotations

import re

CREATOR = "Ibnul Jaif Farabi"
ROLE = "creator and engineer"
COMPANY = "AI999X"

_FORMS = re.compile(
    r"^\s*(?:"
    r"who\s+(?:created|made|built|developed|engineered|designed|wrote|owns)\s+(?:you|this|it|this\s+(?:package|model|engine|library))"
    r"|who\s+(?:is|are)\s+(?:your|the)\s+(?:creator|engineer|developer|author|maker|owner)s?"
    r"|(?:which|what)\s+(?:company|organi[sz]ation|firm)\s+(?:made|created|built|developed|owns|is\s+behind)\s+(?:you|this|it)"
    r"|(?:what|who)\s+are\s+you"
    r")\s*[?.!]*\s*$", re.I)


def is_identity_question(question: str) -> bool:
    return bool(_FORMS.match(question or ""))


def distribution(domain: str) -> str:
    """The package name of a domain: classical_physics -> classical-physics-ai999x (the import keeps the underscore:
    classical_physics_ai999x)."""
    return f"{domain.replace('_', '-')}-ai999x"


def about(domain: str, version: str = "") -> dict:
    name = distribution(domain)
    return {"name": name, "domain": domain, "version": version, "creator": CREATOR, "role": ROLE, "company": COMPANY,
            "statement": f"{name} was created and engineered by {CREATOR} at {COMPANY}. It answers {domain} questions "
                         "offline and deterministically, from certified capabilities only, and refuses what it cannot "
                         "verify."}
