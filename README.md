# Optimization (optimization-ai999x)

[![tests](https://github.com/AI999X/optimization-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/optimization-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/optimization-ai999x)](https://pypi.org/project/optimization-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/optimization-ai999x)

Optimization covers linear and integer programming, convex and nonlinear optimisation, assignment, transportation and shortest-route problems, scheduling, inventory, queueing, game theory and multi-criteria decisions: 73 calculations in 16 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Optimization desktop app: Transportation problem (minimum shipping cost), with its worked example and live scene](https://www.ai999x.com/?ax_shot=optimization-ai999x/hero.webp)](https://www.ai999x.com/models/optimization-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-optimization-1.6.7-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-optimization-1.6.7-linux-x86_64.tar.gz) |
| **Command line** `optimization` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-cli-1.6.7-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-cli-1.6.7-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/optimization-ai999x/#capabilities) |
| **Python** 3.10+ | `pip install optimization-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## See it

From the Optimization desktop app: every scene and chart is drawn from the computed answer.

[![Optimization desktop app: a slider moves an input and the answer, the scene and the charts follow](https://www.ai999x.com/?ax_shot=optimization-ai999x/clip-loop.webp)](https://www.ai999x.com/?ax_shot=optimization-ai999x/clip.mp4)

*[Watch the full clip (MP4)](https://www.ai999x.com/?ax_shot=optimization-ai999x/clip.mp4)*

| | |
|---|---|
| [![Transportation problem (minimum shipping cost): the live scene in the Optimization desktop app](https://www.ai999x.com/?ax_shot=optimization-ai999x/scene-1.webp)](https://www.ai999x.com/models/optimization-ai999x/#cap-transportation_problem)<br>Transportation problem (minimum shipping cost) | [![Economic order quantity (EOQ): the live scene in the Optimization desktop app](https://www.ai999x.com/?ax_shot=optimization-ai999x/scene-2.webp)](https://www.ai999x.com/models/optimization-ai999x/#cap-eoq)<br>Economic order quantity (EOQ) |
| [![AHP consistency ratio: the live scene in the Optimization desktop app](https://www.ai999x.com/?ax_shot=optimization-ai999x/scene-3.webp)](https://www.ai999x.com/models/optimization-ai999x/#cap-ahp_consistency_ratio)<br>AHP consistency ratio | [![Pure-strategy Nash equilibria of a bimatrix game: the live scene in the Optimization desktop app](https://www.ai999x.com/?ax_shot=optimization-ai999x/scene-4.webp)](https://www.ai999x.com/models/optimization-ai999x/#cap-pure_nash_equilibria)<br>Pure-strategy Nash equilibria of a bimatrix game |
| [![Closed cylinder of least surface area for a given volume: the live scene in the Optimization desktop app](https://www.ai999x.com/?ax_shot=optimization-ai999x/scene-5.webp)](https://www.ai999x.com/models/optimization-ai999x/#cap-can_min_surface)<br>Closed cylinder of least surface area for a given volume | [![Break-even quantity: the live scene in the Optimization desktop app](https://www.ai999x.com/?ax_shot=optimization-ai999x/scene-6.webp)](https://www.ai999x.com/models/optimization-ai999x/#cap-break_even_point)<br>Break-even quantity |
| [![Discounted Markov decision process (optimal values and policy): the live scene in the Optimization desktop app](https://www.ai999x.com/?ax_shot=optimization-ai999x/scene-7.webp)](https://www.ai999x.com/models/optimization-ai999x/#cap-mdp_discounted)<br>Discounted Markov decision process (optimal values and policy) | [![0/1 knapsack problem: the live scene in the Optimization desktop app](https://www.ai999x.com/?ax_shot=optimization-ai999x/scene-8.webp)](https://www.ai999x.com/models/optimization-ai999x/#cap-knapsack_01)<br>0/1 knapsack problem |

Charts:

|   |   |   |   |
|---|---|---|---|
| [![Transportation problem (minimum shipping cost): a chart in the Optimization desktop app](https://www.ai999x.com/?ax_shot=optimization-ai999x/chart-1.webp)](https://www.ai999x.com/models/optimization-ai999x/#cap-transportation_problem)<br>Transportation problem (minimum shipping cost) | [![Economic order quantity (EOQ): a chart in the Optimization desktop app](https://www.ai999x.com/?ax_shot=optimization-ai999x/chart-2.webp)](https://www.ai999x.com/models/optimization-ai999x/#cap-eoq)<br>Economic order quantity (EOQ) | [![Closed cylinder of least surface area for a given volume: a chart in the Optimization desktop app](https://www.ai999x.com/?ax_shot=optimization-ai999x/chart-3.webp)](https://www.ai999x.com/models/optimization-ai999x/#cap-can_min_surface)<br>Closed cylinder of least surface area for a given volume | [![Break-even quantity: a chart in the Optimization desktop app](https://www.ai999x.com/?ax_shot=optimization-ai999x/chart-4.webp)](https://www.ai999x.com/models/optimization-ai999x/#cap-break_even_point)<br>Break-even quantity |

## Ask in words: 15 examples

Each question below was asked on this release; the comment under it is the line it printed.

```python
import optimization_ai999x as optimization

a = optimization.ask(
    "The demand is p = 100 - 2q and the cost is C = 300 + 20q. Find "
    "the profit-maximizing quantity."
)
print(a.outcome, a.values)
# SOLVED {'quantity': 20.0, 'price': 60.0, 'profit': 500.0}

a = optimization.ask(
    "Littlewood's rule: the full fare is 400 and the discount fare is "
    "150; full-fare demand has mean 60 and standard deviation 20. Find "
    "the protection level."
)
print(a.outcome, a.values)
# SOLVED {'protection_level': 66.37278727928751, 'z': 0.31863936396437514}

a = optimization.ask("Find the minimum of f(x) = 2x^2 - 8x + 3.")
print(a.outcome, a.values)
# SOLVED {'x_opt': 2.0, 'f_opt': -5.0, 'type': 'minimum'}

a = optimization.ask(
    "The demand is q = 1000 - 20p. Find the revenue-maximizing price."
)
print(a.outcome, a.values)
# SOLVED {'price': 25.0, 'quantity': 500.0, 'revenue': 12500.0}

a = optimization.ask(
    "In an M/G/1 queue the arrival rate is 0.5 per minute, the mean "
    "service time is 1 minute and its standard deviation is 1 minute. "
    "Find Lq."
)
print(a.outcome, a.values)
# SOLVED {'rho': 0.5, 'Lq': 0.5, 'L': 1.0, 'Wq': 1.0, 'W': 2.0}

a = optimization.ask(
    "Use Johnson's rule: machine 1 times [5, 2, 6, 7] and machine 2 "
    "times [3, 4, 8, 2]. Find the minimum makespan."
)
print(a.outcome, a.values)
# SOLVED {'makespan': 22.0, 'sequence': [2, 3, 1, 4]}

a = optimization.ask(
    "Find and classify the critical point of f(x, y) = 3x^2 + 2xy + "
    "y^2 - 4x + 6y + 1."
)
print(a.outcome, a.values)
# SOLVED {'f_value': -20.5, 'x': 2.5, 'y': -5.5, 'type': 'local minimum'}

a = optimization.ask(
    "A farmer has 200 m of fencing to enclose a rectangular field. "
    "What is the maximum area?"
)
print(a.outcome, a.values)
# SOLVED {'width': 50.0, 'length': 50.0, 'max_area': 2500.0}

a = optimization.ask(
    "Newsvendor: a cake sells for 10 and costs 4; demand values [10, "
    "20, 30, 40] have probabilities [0.1, 0.3, 0.4, 0.2]. How many "
    "should be ordered?"
)
print(a.outcome, a.values)
# SOLVED {'critical_ratio': 0.6, 'Q': 30.0, 'expected_profit': 130.0}

a = optimization.ask(
    "An open box is made from a 12 by 12 sheet by cutting equal "
    "squares from the corners. Find the maximum volume."
)
print(a.outcome, a.values)
# SOLVED {'cut': 2.0, 'max_volume': 128.0}

a = optimization.ask(
    "Solve the set cover problem with incidence matrix [[1, 1, 0, 0], "
    "[0, 1, 1, 0], [0, 0, 1, 1], [1, 0, 0, 1]] and costs [3, 2, 3, 2]."
)
print(a.outcome, a.values)
# SOLVED {'min_cost': 4.0, 'sets_example': [2, 4]}

a = optimization.ask(
    "Newsvendor: an item sells for 10 and costs 4, with uniform demand "
    "between 100 and 300. Find the optimal order quantity."
)
print(a.outcome, a.values)
# SOLVED {'critical_ratio': 0.6, 'Q': 220.0, 'expected_profit': 960.0}

a = optimization.ask(
    "Jobs have processing times [3, 1, 4] and weights [1, 2, 3]. Find "
    "the sequence minimizing the total weighted completion time."
)
print(a.outcome, a.values)
# SOLVED {'sequence': [2, 3, 1], 'weighted_completion_time': 25.0}

a = optimization.ask(
    "Facility location: fixed costs [10, 15, 12] and service cost "
    "matrix [[3, 8, 6, 9], [7, 2, 4, 3], [5, 6, 2, 8]]. Which "
    "facilities should open?"
)
print(a.outcome, a.values)
# SOLVED {'min_cost': 31.0, 'open_example': [2]}

a = optimization.ask(
    "Solve the travelling salesman problem with distance matrix [[0, "
    "10, 15, 20], [10, 0, 35, 25], [15, 35, 0, 30], [20, 25, 30, 0]]."
)
print(a.outcome, a.values)
# SOLVED {'min_tour_length': 80.0, 'tour_example': [1, 3, 4, 2, 1]}
```

## Guides

- Desktop app guide: the window, every calculation, scenes and charts: [PDF](https://huggingface.co/AI999X/optimization-ai999x/blob/main/docs/optimization_ai999x_desktop_guide.pdf) · [HTML](https://huggingface.co/AI999X/optimization-ai999x/blob/main/docs/optimization_ai999x_desktop_guide.html) · [Markdown](https://huggingface.co/AI999X/optimization-ai999x/blob/main/docs/optimization_ai999x_desktop_guide.md)
- Python guide: every calculation with a Python example: [PDF](https://huggingface.co/AI999X/optimization-ai999x/blob/main/docs/optimization_ai999x_capabilities.pdf) · [HTML](https://huggingface.co/AI999X/optimization-ai999x/blob/main/docs/optimization_ai999x_capabilities.html) · [Markdown](https://huggingface.co/AI999X/optimization-ai999x/blob/main/docs/optimization_ai999x_capabilities.md)
- Command-line guide: every calculation with the command and what it printed: [PDF](https://huggingface.co/AI999X/optimization-ai999x/blob/main/docs/optimization_ai999x_rust_guide.pdf) · [HTML](https://huggingface.co/AI999X/optimization-ai999x/blob/main/docs/optimization_ai999x_rust_guide.html) · [Markdown](https://huggingface.co/AI999X/optimization-ai999x/blob/main/docs/optimization_ai999x_rust_guide.md)

Every calculation, with its formula, a worked example and its figures: **[Optimization on ai999x.com](https://www.ai999x.com/models/optimization-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.7 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/optimization-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.7`; the [GitHub release](https://github.com/AI999X/optimization-ai999x/releases/tag/v1.6.7) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `optimization` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/optimization`) and its data (`rust/models/optimization/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p optimization-ai999x
./target/release/optimization --help
cargo test -p optimization-ai999x
```
