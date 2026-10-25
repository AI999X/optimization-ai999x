# optimization-ai999x — offline, verified optimization solver with step-by-step answers

[![tests](https://github.com/AI999X/optimization-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/optimization-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/optimization-ai999x)](https://pypi.org/project/optimization-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/optimization-ai999x)

**[Install](https://pypi.org/project/optimization-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/optimization-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/optimization-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.0

optimization-ai999x is an offline, deterministic Python solver that answers optimization questions with verified results from 72 certified capabilities, gives 240 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**optimization-ai999x** is a compact, deterministic optimization engine for Python. It reads a question in plain English (or a typed call), picks a certified optimization capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified optimization capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 240 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** linear and integer programming, convex and nonlinear optimisation, assignment, transportation and shortest-route problems, scheduling, inventory, queueing, game theory and multi-criteria decisions.

## Model overview

| Property | Value |
|---|---|
| Package | `optimization-ai999x` 1.0.0 (import `optimization_ai999x`) |
| Type | Deterministic neuro-symbolic optimization engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 72 certified procedures |
| Knowledge pack | version 1.0.1 |
| Golden answers | 240 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Quickstart

```bash
pip install optimization-ai999x
```

```python
import optimization_ai999x as optimization

optimization.ask("Solve the transportation problem with supply [20, 30], demand [10, 25, 15] and cost matrix [[2, 3, 1], [5, 4, 8]].").show()
# -> SOLVED — min_cost = 150, shipments = [[5.0, 0.0, 15.0], [5.0, 25.0, 0.0]], plan_unique = yes, dual = [-3.0, 0.0, 5.0, 4.0, 4.0]  (transportation_problem)
optimization.ask("Find the shortest path from A to D in the undirected graph with edges A-B 4, A-C 2, B-C 1, B-D 5, C-D 8.").show()
# -> SOLVED — distance = 8, path = ['A', 'C', 'B', 'D'], path_unique = yes  (shortest_path)
optimization.solve("eoq", D=1200, S=50, H=3).show()
# -> SOLVED — Q = 200, total_cost = 600, orders_per_period = 6, cycle_time = 0.166667  (eoq)
optimization.ask("Find the AHP consistency ratio for the pairwise judgments [3, 5, 2].").show()
# -> SOLVED — CR = 0.003185, CI = 0.0018473, lambda_max = 3.00369, weights = [0.6483290138222366, 0.2296507940626371, 0.12202019211512621]  (ahp_consistency_ratio)
optimization.ask("Find the pure-strategy Nash equilibria of the bimatrix game with row payoffs [[3, 0], [5, 1]] and column payoffs [[3, 5], [0, 1]].").show()
# -> SOLVED — count = 1, equilibria = [[2.0, 2.0]]  (pure_nash_equilibria)
optimization.ask("Who created you?").message
# -> optimization-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

Command line: `optimization-ai999x "<question>"`, `optimization-ai999x --capabilities`. The outputs above were produced by running this exact code on optimization-ai999x 1.0.0.

## Find the right capability, and draw it

`optimization.search()` finds which certified capability a question is about — across **every** AI999X model, so a question for another model points you to it. It runs **[ai999x-encoder](https://huggingface.co/AI999X/ai999x-encoder)**, AI999X's own capability encoder (4.7M parameters, a 4.9 MB ONNX model, offline; the root `model.safetensors` of this repository): `pip install "optimization-ai999x[search]"`. `ask(question, suggest=True)` adds the same pointers to a question it does not answer; an answer itself never changes.

```python
import optimization_ai999x as optimization

optimization.search("Solve x^2 - 5x + 6 = 0", k=2)
# -> Roots of a quadratic equation (algebra-ai999x, 0.67); Solution of two linear equations in two unknowns (algebra-ai999x, 0.57)
```

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Capabilities

72 certified procedures. A few of them:

- **Procedures:** 0/1 knapsack problem; 2 x 2 zero-sum game (saddle point or mixed-strategy formula); AHP consistency ratio; AHP priority vector (principal eigenvector); AHP weights by the geometric mean method; Absolute extrema of a polynomial on a closed interval; All-pairs shortest distances (Floyd-Warshall); Assignment problem (Hungarian method), maximum total value; Assignment problem (Hungarian method), minimum total cost; Bin packing (minimum number of bins); Break-even quantity; Chromatic number (minimum graph colouring); Closed cylinder of least surface area for a given volume; Critical path method (project duration and slack); Critical point of a two-variable quadratic and its classification; Discounted Markov decision process (optimal values and policy); Dynamic lot sizing (Wagner-Whitin); EOQ with planned backorders; Earliest due date rule (minimum maximum lateness); Economic order quantity (EOQ); Economic production quantity (EPQ); Equality-constrained quadratic program (Lagrange multipliers / KKT); Erlang B blocking probability (M/M/c/c loss system); Integer linear program (branch and bound) …

`optimization_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified optimization capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (linear and integer programming, convex and nonlinear optimisation, assignment, transportation and shortest-route problems, scheduling, inventory, queueing, game theory and multi-criteria decisions); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{optimization_ai999x_1_0_0,
  title  = {optimization-ai999x: an offline, verified optimization solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.0},
  url    = {https://huggingface.co/AI999X/optimization-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/optimization-ai999x/discussions or visit https://ai999x.com.

## Support the project

optimization-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/optimization-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/optimization-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/optimization-ai999x/tree/main/onnx).
