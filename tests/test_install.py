"""The release's own install checks (run by the factory in a clean environment before every release)."""
import math

import install_checks

import optimization_ai999x as P


def same(got, expected):
    # a number is compared to its last floating-point digits, never bit for bit (302361.32499999995 is 302361.325)
    if isinstance(expected, (int, float)) and isinstance(got, (int, float)) and not isinstance(got, bool):
        return math.isclose(got, expected, rel_tol=1e-9, abs_tol=1e-12)
    return got == expected


def test_sample_question():
    q, expected = install_checks.SAMPLE
    a = P.ask(q)
    assert a.solved, a.message
    assert any(same(v, expected) for v in [*(a.values or {}).values(), a.value]), (a.value, a.values)


def test_install_checks():
    failed = [k for k, ok in install_checks.checks(P).items() if not ok]
    assert not failed, failed
