# GENERATED from ai999x_factory/runtime by tools/build_package.py — do not edit.
"""One numerical comparison policy for the whole factory and product. Every "do these numbers agree?" goes through
`agrees(computed, reference, mode, ...)`; modules do not invent tolerances of their own.

Modes, and why they differ:

  exact         unit conversions, symbolic identities, an inverse solved from an exactly computed value, residuals.
                Relative 1e-9; nothing about rounding applies.
  presentation  a computed answer against a reference ANSWER (a key, a benchmark value, a user's expectation). A key
                like "2.04 s" for 2.0408 s is right. Tolerance: half a unit in the reference's last stated digit, but
                at least 1% (keys are worked with rounded intermediates and g = 9.8) and at most 2%; a precision the
                problem explicitly asks for replaces it.
  validity      a hand-worked reference CASE used as evidence that a formula is right (certification). Each case is
                held to half a unit in its last stated digit (at most 2%), and `validity_check` adds what a single
                tolerance cannot: at least one case must pin the formula to 0.5% or better (or agree exactly), and deviations that all
                lean the same way by more than 0.2% are a systematic error, not rounding. A formula off by a constant
                factor of 1.01 therefore fails even though each case is "within 2%".

Near zero: a reference of exactly 0 is compared against a SCALE (the size of the terms that produced the value),
never a fixed absolute tolerance, so 1e-27 kg*m/s is not mistaken for zero.
"""
from __future__ import annotations

EXACT_RTOL = 1e-9
PRESENTATION_FLOOR, PRESENTATION_CAP = 0.01, 0.02
VALIDITY_CAP, VALIDITY_PIN, SYSTEMATIC = 0.02, 0.005, 0.002


def significant_figures(value) -> int:
    """Significant figures of a stated number; a float is read as Python prints it (192.0 is the integer 192)."""
    text = value if isinstance(value, str) else repr(float(value))
    mantissa = text.strip().lower().split("e")[0].replace("-", "").replace("+", "").replace(",", "")
    digits = mantissa.replace(".", "").lstrip("0")
    if "." in mantissa and not mantissa.endswith(".0"):
        return max(1, len(digits))
    return max(1, len(digits.rstrip("0")))


def half_unit(value) -> float:
    """Relative half-unit in the last stated digit (2.04 -> 0.25%, 2.0408 -> 0.0025%)."""
    return 0.5 * 10.0 ** (1 - significant_figures(value))


def tolerance(reference, mode: str = "presentation", requested: float | None = None) -> float:
    if mode == "exact":
        return EXACT_RTOL
    if requested is not None:                      # "to within 0.1%", "to four significant figures"
        return float(requested)
    if mode == "presentation":
        return min(PRESENTATION_CAP, max(PRESENTATION_FLOOR, half_unit(reference)))
    if mode == "validity":
        return max(1e-6, min(VALIDITY_CAP, half_unit(reference)))
    raise ValueError(f"unknown precision mode: {mode}")


def agrees(computed: float, reference, mode: str = "presentation", *, scale: float | None = None,
           requested: float | None = None, stated=None) -> bool:
    """Whether `computed` agrees with `reference` under the mode's policy. `stated` is the reference as written
    (its significant figures), when `reference` is already converted to SI; `scale` is the size of the terms that
    produced the value, used when the reference is zero."""
    if computed is None or reference is None:
        return False
    ref = float(reference)
    rtol = tolerance(stated if stated is not None else reference, mode, requested)
    if ref == 0.0:
        return computed == 0.0 or (scale is not None and abs(computed) <= rtol * abs(scale))
    return abs(computed - ref) <= rtol * abs(ref)


def within(computed, reference, rtol: float, *, scale: float | None = None) -> bool:
    """`computed` within relative `rtol` of `reference`. A zero reference is compared against `scale` (the size of the
    terms that produced it), never against a fixed absolute floor: `rtol * max(1e-12, |want|)` accepted ANY answer
    smaller than 1e-12 in SI — 3.2e-19 C for 3.2e-18 C, 3.0e-19 kg*m/s for 6.0e-22 kg*m/s (physics-questions-3,
    2026-10-17: three wrong teacher answers were machine-checked that way)."""
    if computed is None or reference is None:
        return False
    ref = float(reference)
    if ref == 0.0:
        return computed == 0.0 or (scale is not None and abs(computed) <= rtol * abs(scale))
    return abs(computed - ref) <= rtol * abs(ref)


def deviation(computed: float, reference: float) -> float:
    return (computed - reference) / reference if reference else 0.0


def validity_check(cases: list) -> tuple[bool, str]:
    """Evidence from hand-worked cases: [(computed, reference, stated[, scale])], `stated` the reference as written,
    `scale` the size of the terms for a zero reference. Each within its stated precision; at least one nonzero case
    pinning the formula to 0.5% (or agreeing exactly); no systematic lean. A zero case says nothing about scale."""
    if not cases:
        return False, "no hand-worked case"
    devs, pins = [], []
    for computed, reference, stated, *rest in cases:
        scale = rest[0] if rest else None
        if not agrees(computed, reference, "validity", stated=stated, scale=scale):
            return False, f"computed {computed:.9g}, worked by hand {float(reference):.9g}"
        if float(reference) != 0.0:
            devs.append(deviation(computed, float(reference)))
            pins.append(tolerance(stated, "validity"))
    if not devs:
        return False, "every case has a zero reference: nothing pins the formula's scale"
    exact = any(abs(d) <= 1e-9 for d in devs)          # exact agreement pins the formula whatever the figures
    if not exact and min(pins) > VALIDITY_PIN:
        return False, "every case is stated too coarsely to pin the formula (need one to 3 significant figures)"
    if len(devs) >= 2 and (all(d > SYSTEMATIC for d in devs) or all(d < -SYSTEMATIC for d in devs)):
        return False, "every case deviates the same way by more than 0.2%: a systematic error, not rounding"
    return True, f"{len(cases)} case(s) agree at their stated precision"
