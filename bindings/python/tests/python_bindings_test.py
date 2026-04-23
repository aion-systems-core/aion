"""Smoke tests for the aion Python package (run after `maturin develop` / wheel install)."""

from __future__ import annotations

import sys
import tempfile
from pathlib import Path

import pytest

_bindings_root = Path(__file__).resolve().parents[1]
_py_pkg = _bindings_root / "python"
if _py_pkg.is_dir():
    sys.path.insert(0, str(_py_pkg))


def _import_aion():
    pytest.importorskip(
        "aion._native",
        reason="Build the extension: `cd bindings/python && maturin develop --release`",
    )
    import aion

    return aion


def test_execute_replay_hash_roundtrip():
    aion = _import_aion()

    cap = aion.run("pytest pilot", model="demo", seed=3)
    assert cap.model == "demo"
    assert cap.seed == 3

    path = tempfile.NamedTemporaryFile(suffix=".aionai", delete=False).name
    aion.capsule_save(cap, path)
    h1 = aion.capsule_deterministic_hash(path)
    assert len(h1) == 64

    rep = aion.replay(path)
    assert rep.replay_symmetry_ok is True
    assert rep.deterministic_hash_hex == h1


def test_drift_self():
    aion = _import_aion()

    cap = aion.run("x", model="m", seed=1)
    p = tempfile.NamedTemporaryFile(suffix=".aionai", delete=False).name
    aion.capsule_save(cap, p)
    d = aion.drift(p, p)
    assert d.changed is False
