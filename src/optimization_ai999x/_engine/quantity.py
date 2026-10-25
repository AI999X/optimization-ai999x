# GENERATED from ai999x_factory/runtime by tools/build_package.py — do not edit.
"""Dimensional algebra: compound units, SI prefixes, angles and affine temperatures.

A unit is parsed from its spelling (``kg*m/s^2``, ``N·m``, ``rad/s``, ``J/(kg*K)``, ``km/h``) into a factor to SI
and a dimension. The radian is tracked as a pseudo-base dimension so that ``rad/s`` (angular frequency) and
``1/s`` (frequency) are different things when a quantity is classified; ``Dim.physical()`` drops it where an
equation's dimensional consistency is checked. Nothing here knows any subject: kinds of quantity live in a
domain pack.
"""
from __future__ import annotations

import math
import re
from dataclasses import dataclass
from fractions import Fraction

BASES = ("kg", "m", "s", "A", "K", "mol", "cd", "rad")


class UnitError(ValueError):
    pass


@dataclass(frozen=True)
class Dim:
    exps: tuple

    def __mul__(self, other: "Dim") -> "Dim":
        return Dim(tuple(a + b for a, b in zip(self.exps, other.exps)))

    def __truediv__(self, other: "Dim") -> "Dim":
        return Dim(tuple(a - b for a, b in zip(self.exps, other.exps)))

    def __pow__(self, n) -> "Dim":
        n = Fraction(n)
        return Dim(tuple(a * n for a in self.exps))

    def physical(self) -> "Dim":
        """The same dimension with the angle marker removed (an angle is a ratio of lengths)."""
        return Dim(self.exps[:7] + (Fraction(0),))

    @property
    def dimensionless(self) -> bool:
        return not any(self.exps)

    def __str__(self) -> str:
        parts = [b if e == 1 else f"{b}^{e}" for b, e in zip(BASES, self.exps) if e]
        return "*".join(parts) or "1"


def _dim(**kw) -> Dim:
    return Dim(tuple(Fraction(kw.get(b, 0)) for b in BASES))


ONE = _dim()


@dataclass(frozen=True)
class Unit:
    factor: float                       # value_SI = value * factor (+ offset for a bare affine unit)
    dim: Dim
    offset: float = 0.0
    pi_factor: Fraction | None = None   # when the factor is exactly pi * pi_factor (degrees, revolutions)


# spelling -> (factor, dimension, takes SI prefixes, offset, exact multiple of pi)
_N = _dim(kg=1, m=1, s=-2)
_J = _dim(kg=1, m=2, s=-2)
_W = _dim(kg=1, m=2, s=-3)
_V = _dim(kg=1, m=2, s=-3, A=-1)
_ATOMS = {
    "kg": (1.0, _dim(kg=1), False), "g": (1e-3, _dim(kg=1), True), "t": (1e3, _dim(kg=1), False),
    "m": (1.0, _dim(m=1), True), "s": (1.0, _dim(s=1), True), "A": (1.0, _dim(A=1), True),
    "K": (1.0, _dim(K=1), True), "mol": (1.0, _dim(mol=1), True), "cd": (1.0, _dim(cd=1), False),
    "rad": (1.0, _dim(rad=1), True),
    "min": (60.0, _dim(s=1), False), "h": (3600.0, _dim(s=1), False), "hr": (3600.0, _dim(s=1), False),
    "day": (86400.0, _dim(s=1), False),
    "N": (1.0, _N, True), "J": (1.0, _J, True), "W": (1.0, _W, True),
    "Pa": (1.0, _dim(kg=1, m=-1, s=-2), True), "atm": (101325.0, _dim(kg=1, m=-1, s=-2), False),
    "bar": (1e5, _dim(kg=1, m=-1, s=-2), True),
    "Hz": (1.0, _dim(s=-1), True), "C": (1.0, _dim(A=1, s=1), True), "V": (1.0, _V, True),
    "ohm": (1.0, _dim(kg=1, m=2, s=-3, A=-2), True), "Ω": (1.0, _dim(kg=1, m=2, s=-3, A=-2), True),
    "F": (1.0, _dim(kg=-1, m=-2, s=4, A=2), True), "T": (1.0, _dim(kg=1, s=-2, A=-1), True),
    "Wb": (1.0, _dim(kg=1, m=2, s=-2, A=-1), True), "H": (1.0, _dim(kg=1, m=2, s=-2, A=-2), True),
    "eV": (1.602176634e-19, _J, True), "L": (1e-3, _dim(m=3), True),
    "Wh": (3600.0, _J, True),
    "Nm": (1.0, _J, False),        # newton-metre (torque) as commonly written; case-sensitive: 'nm' is the nanometre
    # customary and practical units (exact definitions)
    "ft": (0.3048, _dim(m=1), False), "mi": (1609.344, _dim(m=1), False), "yd": (0.9144, _dim(m=1), False),
    "mph": (0.44704, _dim(m=1, s=-1), False), "mmHg": (133.322387415, _dim(kg=1, m=-1, s=-2), False),
    "cal": (4.184, _J, True), "Cal": (4184.0, _J, False), "hp": (745.69987158227022, _W, False),
    "yr": (3.15576e7, _dim(s=1), False), "rpm": (2 * math.pi / 60.0, _dim(s=-1, rad=1), False),
    # a percentage is the pure number / 100: "25%" is 0.25, never 25 (thermofluids, 2026-10-22: an efficiency read as 25)
    "%": (0.01, _dim(), False),
    # electrical engineering (2026-10-22): apparent and reactive power have the dimension of power; the siemens is 1/ohm
    "VA": (1.0, _W, True), "var": (1.0, _W, True), "VAr": (1.0, _W, True),
    "S": (1.0, _dim(kg=-1, m=-2, s=3, A=2), True), "mho": (1.0, _dim(kg=-1, m=-2, s=3, A=2), False),
}
_AFFINE = {"°C": (1.0, _dim(K=1), 273.15), "degC": (1.0, _dim(K=1), 273.15)}
_PI_UNITS = {"deg": Fraction(1, 180), "°": Fraction(1, 180), "rev": Fraction(2)}
_PREFIX = {"k": 1e3, "M": 1e6, "G": 1e9, "m": 1e-3, "c": 1e-2, "μ": 1e-6, "µ": 1e-6, "u": 1e-6, "n": 1e-9,
           "p": 1e-12, "f": 1e-15}

_WORDS = {
    "second": "s", "seconds": "s", "sec": "s", "secs": "s", "millisecond": "ms", "milliseconds": "ms",
    "minute": "min", "minutes": "min", "hour": "h", "hours": "h", "days": "day",
    "meter": "m", "meters": "m", "metre": "m", "metres": "m", "kilometer": "km", "kilometers": "km",
    "kilometre": "km", "kilometres": "km", "centimeter": "cm", "centimeters": "cm", "centimetre": "cm",
    "centimetres": "cm", "millimeter": "mm", "millimeters": "mm", "millimetre": "mm", "millimetres": "mm",
    "gram": "g", "grams": "g", "kilogram": "kg", "kilograms": "kg", "tonne": "t", "tonnes": "t",
    "newton": "N", "newtons": "N", "kilonewton": "kN", "kilonewtons": "kN",
    "joule": "J", "joules": "J", "kilojoule": "kJ", "kilojoules": "kJ",
    "watt": "W", "watts": "W", "kilowatt": "kW", "kilowatts": "kW",
    "volt": "V", "volts": "V", "amp": "A", "amps": "A", "ampere": "A", "amperes": "A",
    "ohms": "ohm", "hertz": "Hz", "kilohertz": "kHz", "pascal": "Pa", "pascals": "Pa", "kilopascals": "kPa",
    "coulomb": "C", "coulombs": "C", "kelvin": "K", "kelvins": "K", "mole": "mol", "moles": "mol",
    "turn": "1", "turns": "1", "tesla": "T", "teslas": "T", "farad": "F", "farads": "F", "henry": "H", "weber": "Wb",
    "degree": "deg", "degrees": "deg", "radian": "rad", "radians": "rad", "revolution": "rev",
    "revolutions": "rev", "liter": "L", "liters": "L", "litre": "L", "litres": "L",
    "per": "/", "squared": "^2", "cubed": "^3", "celsius": "°C",
    "foot": "ft", "feet": "ft", "mile": "mi", "miles": "mi", "yard": "yd", "yards": "yd",
    "calorie": "cal", "calories": "cal", "kilocalorie": "kcal", "kilocalories": "kcal",
    "horsepower": "hp", "year": "yr", "years": "yr", "milliliter": "mL", "milliliters": "mL", "millilitre": "mL",
    "milligram": "mg", "milligrams": "mg", "microgram": "µg", "micrograms": "µg", "nanogram": "ng", "nanograms": "ng",
    "micrometer": "µm", "micrometers": "µm", "micrometre": "µm", "micrometres": "µm", "micron": "µm", "microns": "µm",
    "nanometer": "nm", "nanometers": "nm", "nanometre": "nm", "nanometres": "nm",
    "microsecond": "µs", "microseconds": "µs", "nanosecond": "ns", "nanoseconds": "ns",
    "percent": "%", "pct": "%",
}
_SUPER = str.maketrans({"²": "^2", "³": "^3", "·": "*", "⋅": "*", "−": "-"})
# an exponent: ^2, ^-1, ^(1/3), ^(-1/2), ^1.5 (fractional exponents occur in empirical laws: Manning's n is s/m^(1/3))
_TOKEN = re.compile(r"\s*(\^\(\s*-?\d+(?:\s*/\s*\d+)?\s*\)|\^-?\d+(?:\.\d+)?|[*/()]|1(?![\w.])|[A-Za-zμµΩ°]+|%)")


def _atom(name: str) -> Unit:
    if name in _ATOMS:
        f, d, _ = _ATOMS[name]
        return Unit(f, d)
    if name in _AFFINE:
        f, d, off = _AFFINE[name]
        return Unit(f, d, off)
    if name in _PI_UNITS:
        return Unit(math.pi * float(_PI_UNITS[name]), _dim(rad=1), 0.0, _PI_UNITS[name])
    if len(name) > 1 and name[0] in _PREFIX and name[1:] in _ATOMS and _ATOMS[name[1:]][2]:
        f, d, _ = _ATOMS[name[1:]]
        return Unit(f * _PREFIX[name[0]], d)
    raise UnitError(f"unknown unit: {name}")


def parse_unit(text: str) -> Unit:
    """Parse a unit expression. Raises UnitError for anything that is not entirely a unit."""
    src = (text or "").translate(_SUPER).replace("**", "^").strip()
    if not src or len(src) > 60:
        raise UnitError("empty or oversized unit")
    tokens, pos = [], 0
    while pos < len(src):
        m = _TOKEN.match(src, pos)
        if not m:
            raise UnitError(f"not a unit: {text}")
        tokens.append(m.group(1))
        pos = m.end()
    i = 0

    def factor() -> Unit:
        nonlocal i
        if i >= len(tokens):
            raise UnitError(f"not a unit: {text}")
        tok = tokens[i]
        i += 1
        if tok == "(":
            u = expr()
            if i >= len(tokens) or tokens[i] != ")":
                raise UnitError("unbalanced parenthesis")
            i += 1
        elif tok == "1":
            u = Unit(1.0, ONE)
        elif tok[0] in "^*/)":
            raise UnitError(f"not a unit: {text}")
        else:
            u = _atom(tok)
        if i < len(tokens) and tokens[i].startswith("^"):
            raw = tokens[i][1:].strip("() ").replace(" ", "")
            n = Fraction(raw) if ("/" in raw or "." in raw) else int(raw)
            i += 1
            if u.offset:
                raise UnitError("an offset unit cannot be raised to a power")
            u = Unit(u.factor ** n, u.dim ** n)
        return u

    def expr() -> Unit:
        nonlocal i
        u = factor()
        while i < len(tokens) and tokens[i] in "*/":
            op = tokens[i]
            i += 1
            v = factor()
            if u.offset or v.offset:
                raise UnitError("an offset unit cannot be combined with another unit")
            u = Unit(u.factor * v.factor, u.dim * v.dim) if op == "*" else Unit(u.factor / v.factor, u.dim / v.dim)
        return u

    unit = expr()
    if i != len(tokens):
        raise UnitError(f"not a unit: {text}")
    return unit


def words_to_unit(tokens: list[str]) -> str | None:
    """Turn spelled-out unit words ('meters per second squared') into a unit spelling, or None."""
    out = []
    expanded = []
    for tok in tokens:                                 # "kilogram-meters squared": a hyphen joins two unit words
        parts = tok.lower().split("-")
        expanded.extend(parts if len(parts) > 1 and all(p in _WORDS for p in parts) else [tok])
    for tok in expanded:
        low = tok.lower()
        if low in _WORDS:
            out.append(_WORDS[low])
        else:
            out.append(tok)
    for i in range(len(out) - 1):                      # "per cent" is one word: a percentage, not "/cent"
        if out[i] == "/" and out[i + 1].lower() == "cent":
            out[i:i + 2] = ["%"]
            break
    for i in range(len(out) - 1):                      # "degrees Celsius" is one unit, not an angle
        if out[i] == "deg" and out[i + 1] in ("°C", "C"):
            out[i:i + 2] = ["°C"]
            break
    text = ""
    for part in out:
        if part.startswith("^") or part == "/" or text.endswith("/") or not text:
            text += part
        else:
            text += "*" + part
    return text or None


def stated_tolerance(value) -> float:
    """The relative agreement a stated number can be held to: half a unit in its last significant figure, but
    never looser than 0.2% (so a different formula cannot slip through) and never tighter than 1e-6."""
    text = repr(float(value)) if not isinstance(value, str) else value
    mantissa = text.lower().split("e")[0].replace("-", "").replace("+", "")
    digits = mantissa.replace(".", "").lstrip("0")
    if "." in mantissa:
        sig = len(digits)
        if mantissa.endswith(".0"):                    # 192.0 is the integer 192 as Python prints it
            sig = len(digits.rstrip("0")) or 1
    else:
        sig = len(digits.rstrip("0")) or 1
    return max(1e-6, min(2e-3, 0.5 * 10.0 ** (1 - sig)))


def to_si(value: float, unit: Unit, *, difference: bool = False) -> float:
    """Convert a number in `unit` to SI. A temperature DIFFERENCE ignores the unit's offset."""
    return value * unit.factor + (0.0 if difference else unit.offset)
