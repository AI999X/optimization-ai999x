# GENERATED from ai999x_factory/runtime by tools/build_package.py — do not edit.
"""SymPy and mpmath, imported on first use instead of when the engine is imported.

    from ._lazy import sp, mp        # then sp.Symbol(...), mp.mpf(...) exactly as with `import sympy as sp`

Importing SymPy takes most of a package's start-up time (3.5 s of 4 s natively, several times that in a browser, plus
its download there), and most capabilities never use it: an algorithm written in plain Python answers without it.
Each module of the engine therefore names SymPy through these stand-ins; the real module is imported the first time
one of its attributes is read, and the attribute is then kept on the stand-in, so later reads cost a dictionary lookup.

Rule for the engine: nothing at module level may read `sp.<name>` or `mp.<name>` (that would import SymPy on import);
`tests/test_lazy_sympy.py` checks that importing a package and running a plain-Python capability leaves SymPy unloaded.
"""
from __future__ import annotations

import importlib


class LazyModule:
    """A module imported on the first attribute read. `loaded` says whether that has happened."""

    def __init__(self, name: str):
        self.__dict__["_name"] = name
        self.__dict__["_module"] = None

    @property
    def loaded(self) -> bool:
        return self.__dict__["_module"] is not None

    def __getattr__(self, attr: str):               # only called for names not yet kept on the stand-in
        if attr.startswith("__") and attr.endswith("__"):
            raise AttributeError(attr)              # copy/pickle/inspection probes never trigger the import
        module = self.__dict__["_module"]
        if module is None:
            module = importlib.import_module(self.__dict__["_name"])
            self.__dict__["_module"] = module
        value = getattr(module, attr)
        self.__dict__[attr] = value
        return value

    def __setattr__(self, attr: str, value) -> None:
        raise AttributeError(f"{self.__dict__['_name']} is read-only through its lazy stand-in")

    def __repr__(self) -> str:
        return f"<lazy module {self.__dict__['_name']!r}{' (loaded)' if self.loaded else ''}>"


sp = LazyModule("sympy")
mp = LazyModule("mpmath")
