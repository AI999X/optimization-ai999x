"""optimization-ai999x — offline optimization answers with their formula, steps and verification.

    import optimization_ai999x as m
    m.ask("<a question in words>").show()
    m.formulas()                       # every certified capability

72 certified capabilities: linear and integer programming (exact rational simplex with dual certificates, graphical
LPs, shadow prices), calculus and quadratic optimisation (classic max/min word problems, polynomial extrema on an
interval, critical points, unconstrained and equality-constrained quadratic programs, a steepest-descent step),
networks (shortest paths, Bellman-Ford, all pairs, spanning tree, max flow / min cut, min-cost flow, transportation,
assignment, matchings), combinatorial optimisation (knapsack, set cover, facility location, bin packing, travelling
salesman, graph colouring, vertex cover), inventory (EOQ family, reorder point, newsvendor, Littlewood, Wagner-Whitin),
queueing (M/M/1, M/M/c, M/M/c/K, Erlang B, M/G/1, machine repair, staffing), scheduling (SPT, WSPT, EDD,
Moore-Hodgson, Johnson, parallel machines, CPM, PERT), game theory (zero-sum games, pure and mixed Nash equilibria),
multi-criteria decisions (TOPSIS, weighted sum and product models, AHP) and discounted Markov decision processes.
Where an optimum is attained by several solutions the optimal VALUE is the answer and a solution is labelled an
example. Built by the AI999X model machine from the same engine as every domain; a question that is not determined
is refused rather than guessed.

    >>> m.ask("Solve the transportation problem with supply [20, 30], demand [10, 25, 15] and cost matrix [[2, 3, 1], [5, 4, 8]].").values["min_cost"]
    150.0
    >>> m.solve("zero_sum_2x2", A=[[3, -1], [-2, 4]]).values["value"]
    1.0

Creator / engineer: Ibnul Jaif Farabi. Company: AI999X.
"""
from __future__ import annotations

from pathlib import Path

try:                                       # the installed package carries its own copy of the engine
    from ._engine import Answer
    from ._engine.product import Product
except ImportError:                        # the factory's source tree, during development
    from ai999x_factory.runtime import Answer
    from ai999x_factory.runtime.product import Product

__version__ = "1.0.2"
_P = Product("optimization", [Path(__file__).resolve().parent / "pack", Path(__file__).resolve().parents[3]])
pack, ask, solve, compile_problem = _P.pack, _P.ask, _P.solve, _P.compile_problem
formula, formulas, quantities, conditions, capabilities = _P.formula, _P.formulas, _P.quantities, _P.conditions, _P.capabilities
examples, family_examples, structured_example, to_si = _P.examples, _P.family_examples, _P.structured_example, _P.to_si
templates, procedure_examples = _P.templates, _P.procedure_examples
prove, derivations, about = _P.prove, _P.derivations, (lambda: {**_P.about(), "version": __version__})
knowledge, lookup, related = _P.knowledge, _P.lookup, _P.related
plot, plot_relation, search = _P.plot, _P.plot_relation, _P.search   # figures (SVG) and capability search ([search])
__all__ = ["ask", "solve", "prove", "derivations", "about", "formula", "formulas", "quantities", "conditions", "capabilities",
           "compile_problem", "pack", "Answer", "plot", "plot_relation", "search", "__version__"]
