# GENERATED from ai999x_factory/runtime by tools/build_package.py — do not edit.
"""Deterministic plots of certified relations: the data is computed by the engine, the drawing is plain SVG.

    spec = product.plot_relation("escape_speed", x="M", over=(1e23, 1e25), fixed={"r": "6.371e6 m"})
    spec                      # in Jupyter: the figure (SVG, no extra package needed)
    spec.to_svg()             # the same figure as text, byte-identical on every platform
    spec.figure()             # a matplotlib figure (optional: pip install <package>[plot])
    spec.plotly()             # a plotly figure dict (optional, for interactive notebooks)
    spec.points               # [(x, y), ...] in SI units: exactly what was drawn

Nothing is drawn that the relation's own solver did not compute; a point the solver refuses (outside the relation's
domain) is left out and counted, never interpolated. Numbers are rounded to 12 significant digits in the spec and to a
fixed format in the SVG, so the same call gives the same bytes everywhere.
"""
from __future__ import annotations

import html
import json
import math
from dataclasses import dataclass, field


def _r(v: float) -> float:
    return float(f"{v:.12g}")


@dataclass
class PlotSpec:
    title: str
    latex: str
    x: dict                      # {"symbol", "meaning", "unit"}
    y: dict
    fixed: dict                  # {symbol: [value, unit]} held constant
    points: list = field(default_factory=list)
    skipped: int = 0             # x values the solver refused (outside the relation's domain)
    marker: tuple | None = None  # a highlighted point (an answer's own result)

    def to_dict(self) -> dict:
        return {"title": self.title, "latex": self.latex, "x": self.x, "y": self.y, "fixed": self.fixed,
                "points": [[a, b] for a, b in self.points], "skipped": self.skipped,
                "marker": list(self.marker) if self.marker else None}

    def to_json(self) -> str:
        return json.dumps(self.to_dict(), sort_keys=True, ensure_ascii=False)

    # ---------------------------------------------------------------- pure SVG (no dependency)
    def to_svg(self, width: int = 640, height: int = 380) -> str:
        if not self.points:
            raise ValueError("nothing to draw: the solver refused every point")
        L, R, T, B = 78, 20, 46, 56
        xs, ys = [p[0] for p in self.points], [p[1] for p in self.points]
        x0, x1, y0, y1 = min(xs), max(xs), min(ys), max(ys)
        if self.marker:
            y0, y1 = min(y0, self.marker[1]), max(y1, self.marker[1])
        if x1 == x0:
            x1 = x0 + 1.0
        if y1 == y0:
            y1 = y0 + 1.0
        pad = (y1 - y0) * 0.06
        # padding never pushes a non-negative quantity's axis below zero
        y0, y1 = (max(0.0, y0 - pad) if y0 >= 0 else y0 - pad), y1 + pad
        W, H = width - L - R, height - T - B
        sx = lambda v: L + (v - x0) / (x1 - x0) * W          # noqa: E731
        sy = lambda v: T + H - (v - y0) / (y1 - y0) * H      # noqa: E731
        f = lambda v: f"{v:.2f}"                             # noqa: E731
        esc = html.escape
        out = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}" '
               'font-family="system-ui, sans-serif" font-size="12">',
               f'<rect width="{width}" height="{height}" fill="#ffffff"/>',
               f'<text x="{width / 2:.0f}" y="22" text-anchor="middle" font-size="14" font-weight="600">{esc(self.title)}</text>']
        if self.fixed:
            sub = ", ".join(f"{k} = {v[0]:.6g} {v[1]}" for k, v in sorted(self.fixed.items()))
            out.append(f'<text x="{width / 2:.0f}" y="38" text-anchor="middle" fill="#555">{esc(sub)}</text>')
        for i in range(6):                                   # grid and tick labels
            gx, gy = x0 + (x1 - x0) * i / 5, y0 + (y1 - y0) * i / 5
            out.append(f'<line x1="{f(sx(gx))}" y1="{T}" x2="{f(sx(gx))}" y2="{T + H}" stroke="#e6e6e6"/>')
            out.append(f'<line x1="{L}" y1="{f(sy(gy))}" x2="{L + W}" y2="{f(sy(gy))}" stroke="#e6e6e6"/>')
            out.append(f'<text x="{f(sx(gx))}" y="{T + H + 16}" text-anchor="middle" fill="#444">{gx:.4g}</text>')
            out.append(f'<text x="{L - 6}" y="{f(sy(gy) + 4)}" text-anchor="end" fill="#444">{gy:.4g}</text>')
        out.append(f'<rect x="{L}" y="{T}" width="{W}" height="{H}" fill="none" stroke="#999"/>')
        path = " ".join(("M" if i == 0 else "L") + f"{f(sx(a))},{f(sy(b))}" for i, (a, b) in enumerate(self.points))
        out.append(f'<path d="{path}" fill="none" stroke="#2563eb" stroke-width="2"/>')
        if self.marker:
            mx, my = self.marker
            out.append(f'<circle cx="{f(sx(mx))}" cy="{f(sy(my))}" r="4.5" fill="#dc2626"/>')
            out.append(f'<text x="{f(sx(mx) + 8)}" y="{f(sy(my) - 8)}" fill="#dc2626">{my:.4g} {esc(self.y["unit"])}</text>')
        out.append(f'<text x="{L + W / 2:.0f}" y="{height - 14}" text-anchor="middle">'
                   f'{esc(self.x["symbol"])} — {esc(self.x["meaning"])} [{esc(self.x["unit"])}]</text>')
        out.append(f'<text transform="translate(16 {T + H / 2:.0f}) rotate(-90)" text-anchor="middle">'
                   f'{esc(self.y["symbol"])} — {esc(self.y["meaning"])} [{esc(self.y["unit"])}]</text>')
        out.append("</svg>")
        return "\n".join(out)

    def _repr_svg_(self) -> str:                             # Jupyter shows the figure
        return self.to_svg()

    def show(self) -> "PlotSpec":
        try:
            from IPython.display import SVG, display
            display(SVG(self.to_svg()))
        except ImportError:
            print(self.to_svg())
        return self

    # ---------------------------------------------------------------- optional backends
    def figure(self):
        try:
            import matplotlib
            import matplotlib.pyplot as plt
        except ImportError as exc:
            raise ImportError("figure() needs matplotlib: pip install <package>[plot]") from exc
        _ = matplotlib
        fig, ax = plt.subplots(figsize=(6.4, 3.8))
        ax.plot([p[0] for p in self.points], [p[1] for p in self.points])
        if self.marker:
            ax.plot([self.marker[0]], [self.marker[1]], "o", color="#dc2626")
        ax.set_xlabel(f"{self.x['symbol']} ({self.x['meaning']}) [{self.x['unit']}]")
        ax.set_ylabel(f"{self.y['symbol']} ({self.y['meaning']}) [{self.y['unit']}]")
        ax.set_title(f"{self.title}: ${self.latex}$", fontsize=9)
        ax.grid(alpha=0.3)
        fig.tight_layout()
        return fig

    def plotly(self) -> dict:
        """A plotly figure as a plain dict (plotly.graph_objects.Figure(spec.plotly()) or plotly.io.show)."""
        data = [{"type": "scatter", "mode": "lines", "name": self.y["symbol"],
                 "x": [p[0] for p in self.points], "y": [p[1] for p in self.points]}]
        if self.marker:
            data.append({"type": "scatter", "mode": "markers", "name": "answer", "x": [self.marker[0]], "y": [self.marker[1]]})
        return {"data": data, "layout": {"title": {"text": self.title},
                                         "xaxis": {"title": {"text": f"{self.x['symbol']} [{self.x['unit']}]"}},
                                         "yaxis": {"title": {"text": f"{self.y['symbol']} [{self.y['unit']}]"}}}}


def relation_plot(pack, key: str, x: str, over: tuple, fixed: dict, y: str | None = None, points: int = 101,
                  marker=None) -> PlotSpec:
    """`y` (default: the relation's left-hand symbol) against `x` over `over` (SI), the other symbols `fixed` (SI)."""
    r = pack.by_key[key]
    y = y or r.equation.split("=")[0].strip()
    if x not in r.symbols or y not in r.symbols or x == y:
        raise ValueError(f"{key} relates {sorted(r.symbols)}; choose two different symbols of it")
    missing = [n for n in r.symbols if n not in (x, y) and n not in fixed and r.symbols[n].default is None]
    if missing:
        raise ValueError(f"give a value for {', '.join(missing)} (fixed=...)")
    lo, hi = float(over[0]), float(over[1])
    pts, skipped = [], 0
    for i in range(points):
        xv = lo + (hi - lo) * i / (points - 1)
        try:
            yv = r.solve(y, {**fixed, x: xv})
            if not math.isfinite(yv):
                raise ValueError
            pts.append((_r(xv), _r(yv)))
        except Exception:                                    # outside the relation's domain: left out, counted
            skipped += 1
    unit = lambda s: pack.kinds[r.symbols[s].kind].unit      # noqa: E731
    meaning = lambda s: r.symbols[s].desc or r.symbols[s].kind.replace("_", " ")   # noqa: E731
    return PlotSpec(title=r.name, latex=r.latex(), x={"symbol": x, "meaning": meaning(x), "unit": unit(x)},
                    y={"symbol": y, "meaning": meaning(y), "unit": unit(y)},
                    fixed={k: [_r(float(v)), unit(k)] for k, v in sorted(fixed.items())}, points=pts, skipped=skipped,
                    marker=(_r(marker[0]), _r(marker[1])) if marker else None)
