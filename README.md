# optimization-ai999x: offline, verified optimization solver with step-by-step answers

[![tests](https://github.com/AI999X/optimization-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/optimization-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/optimization-ai999x)](https://pypi.org/project/optimization-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/optimization-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/optimization-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/optimization-ai999x/)** · **[Guides](https://huggingface.co/AI999X/optimization-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/optimization-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.5.0

Field: **Mathematics & Statistics** · Tasks: Question Answering

optimization-ai999x is an offline, deterministic optimization solver written in Rust: a desktop app, a command-line program, a browser module and a Python package. It answers optimization questions with verified results from 72 certified capabilities, gives 240 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**optimization-ai999x** reads a question in plain English (or a typed call), picks a certified optimization capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions, or a refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green) ![Rust](https://img.shields.io/badge/Rust-Windows%20%7C%20Linux%20%7C%20WebAssembly-orange)

**Run it your way.** optimization-ai999x comes as the AI999X desktop app and as the command-line program `optimization` for Windows and Linux, runs in your browser as WebAssembly, and installs as a Python package. The desktop app, the command line and the browser run the same compiled Rust program on your own device: offline, no Python needed.

## Get it

| Way to run it | For | Get it |
|---|---|---|
| **AI999X Optimization desktop app** 1.5.0 | Windows, Linux. This model in its own app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-optimization-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-optimization-1.5.0-linux-x86_64.tar.gz) |
| **AI999X desktop app** 1.5.0 | Windows, Linux. Every AI999X model in one app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-linux-x86_64.tar.gz) |
| **Command line** (`optimization`) | Windows, Linux. One program per model, offline, no Python | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz) |
| **In your browser** | any device. Nothing to install | [Compute on the model page](https://www.ai999x.com/models/optimization-ai999x/#try) |
| **Python** | Python 3.10+ | `pip install optimization-ai999x` |

Checksums: [SHA256SUMS](https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS) · every version: [https://downloads.ai999x.com/desktop/latest/](https://downloads.ai999x.com/desktop/latest/) · install guide: [desktop & command line](https://www.ai999x.com/desktop-cli/)

## Quick start

### Desktop app

Download **AI999X Optimization** for your system (above), unpack it and start it; type a question in words or pick a capability, fill in the form and press Compute. Figures and an explanation of the answer appear beside it. The AI999X desktop app has the same, with Optimization among all AI999X models.

Linux, from a terminal:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-optimization-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-optimization-1.5.0-linux-x86_64.tar.gz
./ai999x-optimization-1.5.0-linux-x86_64/ai999x-optimization
```

### Command line

Linux:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.5.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.5.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
optimization --version
```

Windows (PowerShell):

```powershell
$F = "ai999x-cli-1.5.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.5.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
optimization --version
```

Every command below was run as shown; the text under it is what the program printed.

**Ask in words**: Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```console
$ optimization ask "Solve the transportation problem with supply [20, 30], demand [10, 25, 15] and cost matrix [[2, 3, 1], [5, 4, 8]]."
Read as: Transportation problem (minimum shipping cost)
min_cost = 150
shipments = [[5.0,0.0,15.0],[5.0,25.0,0.0]]
plan_unique = yes
dual = [-3.0,0.0,5.0,4.0,4.0]
  by: Transportation problem (minimum shipping cost)  (the transportation LP solved exactly (rational simplex), certified by its dual (u_i + v_j <= c_ij))
  assumes: total supply >= total demand (surplus stays at the sources at no cost); at most 60 routes
```

**What a capability needs**: The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```console
$ optimization info eoq
Economic order quantity (EOQ)
  minimise D S / Q + H Q / 2 (at the optimum ordering cost equals holding cost)
  in   D        demand per period (units, e.g. per year) [1]
  in   S        fixed cost per order [1]
  in   H        holding cost per unit per period [1]
  out  Q        economic order quantity [1]
  out  total_cost minimum ordering + holding cost per period [1]
  out  orders_per_period number of orders per period [1]
  out  cycle_time time between orders (in periods) [1]
```

**Solve**: Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```console
$ optimization solve eoq D=1200 S=50 H=3
Q = 200
total_cost = 600
orders_per_period = 6
cycle_time = 0.1666666667
  by: Economic order quantity (EOQ)  (minimise D S / Q + H Q / 2 (at the optimum ordering cost equals holding cost))
  D = 1200
  S = 50
  H = 3
  assumes: D, S, H > 0; constant demand, instant replenishment, no shortages
```

**Many questions at once**: One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | optimization batch

`cases.jsonl`:

```json
{"key": "eoq", "inputs": {"D": 1200, "S": 50, "H": 3}}
{"key": "ahp_consistency_ratio", "inputs": {"judgments": "[3, 5, 2]"}}
```

```console
$ optimization batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":200.0,"unit":"","solved_for":"Q","si":[["D",1200.0],["S",50.0],["H",3.0]],"values":[["Q",200.0,""],["total_cost",600.0,""],["orders_per_period",6.0,""],["cycle_time",0.16666666666666666,""]],"target":"Q","phrase":"economic order quantity","assumptions":["D, S, H > 0","constant demand, instant replenishment, no shortages"],"name":"Economic order quantity (EOQ)","equation":"minimise D S / Q + H Q / 2 (at the optimum ordering cost equals holding cost)"}
{"outcome":"SOLVED","message":"","value":0.0031849983307239735,"unit":"","solved_for":"CR","values":[["CR",0.0031849983307239735,""],["CI",0.0018472990318199045,""],["lambda_max",3.00369459806364,""],["weights",[0.6483290138222366,0.2296507940626371,0.12202019211512621],""],["acceptable","yes",""]],"target":"CR","phrase":"consistency ratio","assumptions":["3 to 10 criteria","RI = 0.58, 0.90, 1.12, 1.24, 1.32, 1.41, 1.45, 1.49 for n = 3..10 (Saaty)"],"name":"AHP consistency ratio","equation":"principal eigenpair by power iteration (certified as an eigenpair), then CI and CR"}
```

### In your browser

Open [Optimization on ai999x.com](https://www.ai999x.com/models/optimization-ai999x/#try), choose a capability and press **Compute**: the model runs on your own device as WebAssembly; nothing is sent anywhere. Change any input (any unit of the right kind) and the answer, its explanation and the figures follow.

In your own page: `optimization_ai999x.wasm` and the loader `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js).

### Python

```bash
pip install optimization-ai999x
```

## Worked examples in every form

The same worked example from the command line, from Python and in the browser. Each command-line output is what the program printed.

### Absolute extrema of a polynomial on a closed interval

The absolute minimum and maximum of a polynomial on [lo, hi] (closed-interval method: endpoints and critical points), and where they are attained.

Command line:

```console
$ optimization solve polynomial_extrema_interval "coefficients=[1, -3, 0, 2]" lo=-1 hi=3
min_value = -2
max_value = 2
argmin_example = -1
argmax_example = 0
  by: Absolute extrema of a polynomial on a closed interval  (real roots of f' in [lo, hi] (monotone pieces between the roots of f'' bisected to the last float), compared with the endpoints)
  lo = -1
  hi = 3
  assumes: lo < hi; degree 1 to 5
```

Python:

```python
import optimization_ai999x as m

a = m.solve("polynomial_extrema_interval", coefficients=[1, -3, 0, 2], lo="-1", hi="3")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/optimization-ai999x/#cap-polynomial_extrema_interval) and press **Compute in your browser**.

_What this means:_ This calculation determines the global extrema of a polynomial on the closed interval from -1 to 3. It evaluates the function at the endpoints and all real critical points where the derivative is zero within the interval. The smallest resulting value is reported as the absolute minimum, -2, attained at -1, while the largest is the absolute maximum, 2, found at 0.

### Closed cylinder of least surface area for a given volume

The radius, height and surface area of the closed cylindrical can that holds a volume V with the least material.

Command line:

```console
$ optimization solve can_min_surface V=500
radius = 4.301270069
height = 8.602540138
min_area = 348.7342055
  by: Closed cylinder of least surface area for a given volume  (A(r) = 2 pi r^2 + 2V/r; A'(r) = 0)
  V = 500
  assumes: V > 0; closed top and bottom
```

Python:

```python
import optimization_ai999x as m

a = m.solve("can_min_surface", V="500")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/optimization-ai999x/#cap-can_min_surface) and press **Compute in your browser**.

_What this means:_ This calculation determines the dimensions for a closed cylindrical container that holds a specific volume while using the least amount of material. Given an input volume of 500, the method finds the optimal radius to be 4.30127 and the corresponding height to be 8.60254. The resulting minimum surface area required is 348.734. This ensures efficient packaging design by balancing the top, bottom, and side surfaces.

### Critical point of a two-variable quadratic and its classification

For f(x, y) = a x^2 + b xy + c y^2 + d x + e y + f0: the critical point, its value and whether it is a local minimum, a local maximum or a saddle point (second-derivative test).

Command line:

```console
$ optimization solve critical_point_two_variables a=3 b=2 c=1 d=-4 e=6 f0=1
f_value = -20.5
x = 2.5
y = -5.5
type = local minimum
  by: Critical point of a two-variable quadratic and its classification  (grad f = 0 solved exactly; D = f_xx f_yy - f_xy^2 = 4ac - b^2 classifies it)
  a = 3
  b = 2
  c = 1
  d = -4
  e = 6
  f0 = 1
  assumes: 4ac - b^2 != 0 (an isolated critical point)
```

Python:

```python
import optimization_ai999x as m

a = m.solve("critical_point_two_variables", a="3", b="2", c="1", d="-4", e="6", f0="1")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/optimization-ai999x/#cap-critical_point_two_variables) and press **Compute in your browser**.

_What this means:_ This calculation identifies the stationary point for the quadratic function defined by coefficients 3, 2, 1, -4, 6, and 1. Solving the gradient equal to zero shows that the critical point occurs at coordinates x equals 2.5 and y equals -5.5. The value of the function at this specific location is -20.5. Based on the second derivative test, which examines the curvature using the determinant of the Hessian matrix, the nature of this point is classified as a local minimum. This classification indicates whether the surface curves upward like a bowl, downward like a dome, or forms a saddle shape at this coordinate.

### Largest rectangle for a given length of fencing

The dimensions and maximum area of a rectangle enclosed by a fence of length L, on four sides or on three sides with a wall or river as the fourth.

Command line:

```console
$ optimization solve fence_max_area wall=False L=200
width = 50
length = 50
max_area = 2500
  by: Largest rectangle for a given length of fencing  (area as a function of one side, derivative set to zero)
  L = 200
  assumes: L > 0
```

Python:

```python
import optimization_ai999x as m

a = m.solve("fence_max_area", wall="False", L="200")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/optimization-ai999x/#cap-fence_max_area) and press **Compute in your browser**.

_What this means:_ This calculation finds the largest rectangular area possible for a given fencing length. Using the total fence amount 200, it determines the optimal side dimensions. The resulting maximum area is 2500, achieved when the perpendicular side is 50 and the parallel side is 50. This ensures the most efficient use of boundary materials.

## Documentation

- **Rust user guide**: the command line, every capability with a command and what it printed: [PDF](https://huggingface.co/AI999X/optimization-ai999x/blob/main/docs/optimization_ai999x_rust_guide.pdf) · [Markdown](https://huggingface.co/AI999X/optimization-ai999x/blob/main/docs/optimization_ai999x_rust_guide.md)
- **Python guide**: every capability with a Python example: [PDF](https://huggingface.co/AI999X/optimization-ai999x/blob/main/docs/optimization_ai999x_capabilities.pdf) · [Markdown](https://huggingface.co/AI999X/optimization-ai999x/blob/main/docs/optimization_ai999x_capabilities.md)
- **Desktop app & command line**: downloads, install, ask and search, figures, offline use, checksums: https://www.ai999x.com/desktop-cli/
- **Model page**: compute in your browser, every capability explained: https://www.ai999x.com/models/optimization-ai999x/

## Highlights

- **Verified, not generated.** Every answer is computed by a certified optimization capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes (SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR), never a silent guess. Every one of its 240 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Native and in the browser.** Every capability also computes natively in Rust: the desktop app and the `optimization` command line for Windows and Linux, and WebAssembly on the model page, no Python, no network.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** linear and integer programming, convex and nonlinear optimisation, assignment, transportation and shortest-route problems, scheduling, inventory, queueing, game theory and multi-criteria decisions.

## Model overview

| Property | Value |
|---|---|
| Package | `optimization-ai999x` 1.5.0 (import `optimization_ai999x`) |
| Type | Deterministic neuro-symbolic optimization engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 72 certified procedures |
| Knowledge pack | version 1.0.1 |
| Golden answers | 240 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Python quick start

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

The Python package also installs a Python command, `optimization-ai999x "<question>"`; for the native program use `optimization` (above). The outputs above were produced by running this exact code on optimization-ai999x 1.5.0.

## How every answer is verified

1. **Reading**: the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning**: only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing**: exact symbolic or high-precision numeric evaluation.
4. **Checking**: dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Python worked examples, with their steps

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

![A linear programme: feasible region and optimal corner, computed by optimization-ai999x](https://huggingface.co/AI999X/optimization-ai999x/resolve/main/figures/lp-two-variable-graphical-region.svg)

_Maximise 3x + 5y subject to x ≤ 4; 2y ≤ 12; 3x + 2y ≤ 18, x, y ≥ 0: the feasible region with the corner points the package returns ((0, 0), (0, 6), (2, 6), (4, 0), (4, 3)) and the objective line through the optimum; marked: x = 2, y = 6, objective 36._

**Steepest descent zig-zagging to the minimum**

![Steepest descent zig-zagging to the minimum, computed by optimization-ai999x](https://huggingface.co/AI999X/optimization-ai999x/resolve/main/figures/steepest-descent-step-iteration.svg)

_Steepest descent with exact line search on f(x) = ½ xᵀQx, Q = [[2, 0], [0, 8]], from x₀ = (4, 1): six steps computed by the package, each perpendicular to the level set through its start (f = 20, 7.2, 2.592, 0.93312, 0.335923, 0.120932, 0.0435356); marked: x₆ = (0.186624, 0.046656)._

**Shortest path and minimum spanning tree**

![Shortest path and minimum spanning tree, computed by optimization-ai999x](https://huggingface.co/AI999X/optimization-ai999x/resolve/main/figures/shortest-path-graph.svg)

_A weighted network (A-B 4, A-C 2, B-C 1, B-D 5, C-D 8, A-E 7, E-D 3, C-E 6): the shortest path from A to D the package finds, A → C → B → D of length 8, and its minimum spanning tree (B-C, A-C, E-D, B-D, total weight 11)._

**An optimal transportation plan**

![An optimal transportation plan, computed by optimization-ai999x](https://huggingface.co/AI999X/optimization-ai999x/resolve/main/figures/transportation-problem-heatmap.svg)

_The optimal transportation plan the package computes for supplies [20, 30], demands [10, 25, 15] and unit costs [[2, 3, 1], [5, 4, 8]] (rows: sources, columns: destinations; left the unit costs, right the quantities shipped): minimum total cost 150._

**A two-machine schedule in Johnson's order**

![A two-machine schedule in Johnson's order, computed by optimization-ai999x](https://huggingface.co/AI999X/optimization-ai999x/resolve/main/figures/johnson-two-machine-gantt.svg)

_A two-machine flow shop: processing times [5, 2, 6, 7] on machine 1 and [3, 4, 8, 2] on machine 2. The package's Johnson order is job 2 → job 3 → job 1 → job 4, with makespan 22; each job starts on machine 2 as soon as it leaves machine 1 and machine 2 is free._

## Capabilities

72 certified procedures. A few of them:

- **Procedures:** 0/1 knapsack problem; 2 x 2 zero-sum game (saddle point or mixed-strategy formula); AHP consistency ratio; AHP priority vector (principal eigenvector); AHP weights by the geometric mean method; Absolute extrema of a polynomial on a closed interval; All-pairs shortest distances (Floyd-Warshall); Assignment problem (Hungarian method), maximum total value; Assignment problem (Hungarian method), minimum total cost; Bin packing (minimum number of bins); Break-even quantity; Chromatic number (minimum graph colouring); Closed cylinder of least surface area for a given volume; Critical path method (project duration and slack); Critical point of a two-variable quadratic and its classification; Discounted Markov decision process (optimal values and policy); Dynamic lot sizing (Wagner-Whitin); EOQ with planned backorders; Earliest due date rule (minimum maximum lateness); Economic order quantity (EOQ); Economic production quantity (EPQ); Equality-constrained quadratic program (Lagrange multipliers / KKT); Erlang B blocking probability (M/M/c/c loss system); Integer linear program (branch and bound) …

`optimization_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified optimization capabilities, and any question whose stated numbers it cannot all use: it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (linear and integer programming, convex and nonlinear optimisation, assignment, transportation and shortest-route problems, scheduling, inventory, queueing, game theory and multi-criteria decisions); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused; use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{optimization_ai999x_1_5_0,
  title  = {optimization-ai999x: an offline, verified optimization solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.5.0},
  url    = {https://huggingface.co/AI999X/optimization-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/optimization-ai999x/discussions or visit https://ai999x.com.

## Support the project

optimization-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/optimization-ai999x/) (`pip install .` builds it). Releases are tagged `v1.5.0`; the [GitHub release](https://github.com/AI999X/optimization-ai999x/releases/tag/v1.5.0) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `optimization` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/optimization`) and its data (`rust/models/optimization/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p optimization-ai999x
./target/release/optimization --help
cargo test -p optimization-ai999x
```
