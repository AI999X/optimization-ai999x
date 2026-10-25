# GENERATED from ai999x_factory/runtime by tools/build_package.py — do not edit.
"""The small offline runtime a domain package ships with: units, typed relations, problem compiler, planner.

Nothing in this subpackage imports the production side of the factory (teacher, datasets, training), and nothing
here knows a subject — a domain arrives as a data pack.
"""
from .answer import Answer
from .compiler import Given, Problem, Target, compile_problem
from .engine import ask, solve_problem
from .pack import Kind, Pack, PackError
from .relation import Relation, RelationError

__all__ = ["Answer", "Given", "Problem", "Target", "compile_problem", "ask", "solve_problem", "Kind", "Pack",
           "PackError", "Relation", "RelationError"]
