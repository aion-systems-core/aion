"""Pilot API tests (require engine cdylib on AION_LIB_PATH)."""

from __future__ import annotations

import os
import tempfile
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[1]


@pytest.fixture(scope="module")
def pilot():
    os.environ.setdefault("AION_LIB_PATH", str(ROOT.parent.parent / "target" / "debug"))
    from aion import Pilot

    return Pilot()


def test_execute_replay_roundtrip(pilot):
    p = tempfile.NamedTemporaryFile(suffix=".aionai", delete=False)
    path = p.name
    p.close()
    try:
        pilot.execute_ai(model="demo", prompt="pytest-pilot", seed=42, out_path=path)
        r = pilot.replay(path)
        assert r.exit_code in (0, 1)
        assert isinstance(r.stdout, str)
    finally:
        os.unlink(path)


def test_evidence_fixture(pilot):
    ev = ROOT / "examples" / "demo_evidence.json"
    if not ev.is_file():
        pytest.skip("demo evidence fixture missing")
    v = pilot.evidence(str(ev))
    assert v.valid in (True, False)
