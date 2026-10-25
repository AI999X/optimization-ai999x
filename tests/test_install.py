"""The release's own install checks (run by the factory in a clean environment before every release)."""
import install_checks

import optimization_ai999x as P


def test_sample_question():
    q, expected = install_checks.SAMPLE
    a = P.ask(q)
    assert a.solved, a.message
    assert expected in [v for v in (a.values or {}).values()] or a.value == expected


def test_install_checks():
    failed = [k for k, ok in install_checks.checks(P).items() if not ok]
    assert not failed, failed
