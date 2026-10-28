# optimization-ai999x — offline, verified optimization solver with step-by-step answers

[![tests](https://github.com/AI999X/optimization-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/optimization-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/optimization-ai999x)](https://pypi.org/project/optimization-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/optimization-ai999x)

**[Install](https://pypi.org/project/optimization-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/optimization-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/optimization-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.3

Field: **Mathematics & Statistics** · Tasks: Question Answering, Sentence Similarity, Feature Extraction

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
| Package | `optimization-ai999x` 1.0.3 (import `optimization_ai999x`) |
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

Command line: `optimization-ai999x "<question>"`, `optimization-ai999x --capabilities`. The outputs above were produced by running this exact code on optimization-ai999x 1.0.3.

## Find the right capability, and draw it

`optimization.search()` finds which certified capability a question is about — across **every** AI999X model, so a question for another model points you to it. It runs **[ai999x-encoder](https://huggingface.co/AI999X/ai999x-encoder)**, AI999X's own capability encoder (4.7M parameters, a 4.9 MB ONNX model, offline; the root `model.safetensors` of this repository): `pip install "optimization-ai999x[search]"`. `ask(question, suggest=True)` adds the same pointers to a question it does not answer; an answer itself never changes.

```python
import optimization_ai999x as optimization

optimization.search("Solve x^2 - 5x + 6 = 0", k=2)
# -> Roots of a quadratic equation (math-foundations-ai999x, 0.63); Roots of a cubic equation (math-foundations-ai999x, 0.59)
```

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Worked examples

Each question was put to `optimization-ai999x`; the answer is the one it returned, with the steps it took.

**Transportation problem (minimum shipping cost)** · _Network optimisation_

> Solve the transportation problem with supply [20, 30], demand [10, 25, 15] and cost matrix [[2, 3, 1], [5, 4, 8]].

→ **min_cost = 150, shipments = [[5, 0, 15], [5, 25, 0]], plan_unique = yes, dual = [-3, 0, 5, 4, 4]**

<details><summary>Steps</summary>

- Transportation problem (minimum shipping cost): supply = [20, 30], demand = [10, 25, 15], cost = [2 3 1; 5 4 8] → min_cost = 150, shipments = [[5, 0, 15], [5, 25, 0]], plan_unique = yes, dual = [-3, 0, 5, 4, 4]

</details>

**Shortest path (Dijkstra's algorithm)** · _Network optimisation_

> Find the shortest path from A to D in the undirected graph with edges A-B 4, A-C 2, B-C 1, B-D 5, C-D 8.

→ **distance = 8, path = [A, C, B, D], path_unique = yes**

<details><summary>Steps</summary>

- Shortest path (Dijkstra's algorithm): edges = A-B (4), A-C (2), B-C (1), B-D (5), C-D (8), source = A, target = D, directed = False → distance = 8, path = [A, C, B, D], path_unique = yes

</details>

**Economic order quantity (EOQ)** · _Inventory management_

`m.solve("eoq", D=1200, S=50, H=3)`

→ **Q = 200, total_cost = 600, orders_per_period = 6, cycle_time = 0.1666666667**

<details><summary>Steps</summary>

- Economic order quantity (EOQ): D = 1200, S = 50, H = 3 → Q = 200, total_cost = 600, orders_per_period = 6, cycle_time = 0.1666666667

</details>

**AHP consistency ratio** · _Multi-criteria decision analysis_

> Find the AHP consistency ratio for the pairwise judgments [3, 5, 2].

→ **CR = 0.003184998331, CI = 0.001847299032, lambda_max = 3.003694598, weights = [0.6483290138, 0.2296507941, 0.1220201921], acceptable = yes**

<details><summary>Steps</summary>

- AHP consistency ratio: judgments = [3, 5, 2] → CR = 0.003184998331, CI = 0.001847299032, lambda_max = 3.003694598, weights = [0.6483290138, 0.2296507941, 0.1220201921], acceptable = yes

</details>

**Pure-strategy Nash equilibria of a bimatrix game** · _Game theory_

> Find the pure-strategy Nash equilibria of the bimatrix game with row payoffs [[3, 0], [5, 1]] and column payoffs [[3, 5], [0, 1]].

→ **count = 1, equilibria = [[2, 2]]**

<details><summary>Steps</summary>

- Pure-strategy Nash equilibria of a bimatrix game: A = [3 0; 5 1], B = [3 5; 0 1] → count = 1, equilibria = [[2, 2]]

</details>

**Closed cylinder of least surface area for a given volume** · _Calculus optimisation_

> A closed cylindrical can must hold 500 cm^3. Find the dimensions that minimize the surface area.

→ **radius = 4.301270069, height = 8.602540138, min_area = 348.7342055**

<details><summary>Steps</summary>

- Closed cylinder of least surface area for a given volume: V = 500 → radius = 4.301270069, height = 8.602540138, min_area = 348.7342055

</details>

## Figures

Every point was computed by `optimization-ai999x`.

**A linear programme: feasible region and optimal corner**

![A linear programme: feasible region and optimal corner — computed by optimization-ai999x](https://huggingface.co/AI999X/optimization-ai999x/resolve/main/figures/lp-two-variable-graphical-region.svg)

_Maximise 3x + 5y subject to x ≤ 4; 2y ≤ 12; 3x + 2y ≤ 18, x, y ≥ 0: the feasible region with the corner points the package returns ((0, 0), (0, 6), (2, 6), (4, 0), (4, 3)) and the objective line through the optimum; marked: x = 2, y = 6, objective 36._

**Steepest descent zig-zagging to the minimum**

![Steepest descent zig-zagging to the minimum — computed by optimization-ai999x](https://huggingface.co/AI999X/optimization-ai999x/resolve/main/figures/steepest-descent-step-iteration.svg)

_Steepest descent with exact line search on f(x) = ½ xᵀQx, Q = [[2, 0], [0, 8]], from x₀ = (4, 1): six steps computed by the package, each perpendicular to the level set through its start (f = 20, 7.2, 2.592, 0.93312, 0.335923, 0.120932, 0.0435356); marked: x₆ = (0.186624, 0.046656)._

**Shortest path and minimum spanning tree**

![Shortest path and minimum spanning tree — computed by optimization-ai999x](https://huggingface.co/AI999X/optimization-ai999x/resolve/main/figures/shortest-path-graph.svg)

_A weighted network (A-B 4, A-C 2, B-C 1, B-D 5, C-D 8, A-E 7, E-D 3, C-E 6): the shortest path from A to D the package finds, A → C → B → D of length 8, and its minimum spanning tree (B-C, A-C, E-D, B-D, total weight 11)._

**An optimal transportation plan**

![An optimal transportation plan — computed by optimization-ai999x](https://huggingface.co/AI999X/optimization-ai999x/resolve/main/figures/transportation-problem-heatmap.svg)

_The optimal transportation plan the package computes for supplies [20, 30], demands [10, 25, 15] and unit costs [[2, 3, 1], [5, 4, 8]] (rows: sources, columns: destinations; left the unit costs, right the quantities shipped): minimum total cost 150._

**A two-machine schedule in Johnson's order**

![A two-machine schedule in Johnson's order — computed by optimization-ai999x](https://huggingface.co/AI999X/optimization-ai999x/resolve/main/figures/johnson-two-machine-gantt.svg)

_A two-machine flow shop: processing times [5, 2, 6, 7] on machine 1 and [3, 4, 8, 2] on machine 2. The package's Johnson order is job 2 → job 3 → job 1 → job 4, with makespan 22; each job starts on machine 2 as soon as it leaves machine 1 and machine 2 is free._

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
@software{optimization_ai999x_1_0_3,
  title  = {optimization-ai999x: an offline, verified optimization solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.3},
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
