"""
AION-OS — Deterministic AI Execution Layer (pilot API).

This package wraps the Rust extension `aion._native`. Install with::

    pip install maturin
    maturin develop --release -m bindings/python/pyproject.toml

(from repo root), or ``pip install aion`` once a wheel is published.
"""

from __future__ import annotations

import json
from typing import Any, Optional

from ._native import (
    AICapsule,
    DriftReport,
    GovernanceReport,
    ReplayComparison,
    RunResult,
    capsule_deterministic_hash,
    capsule_load,
    capsule_save,
    compare_capsules,
    drift_between,
    execute_ai,
    graph_causal,
    replay_capsule,
    validate as _governance_validate,
    why_explain,
)


def run(
    prompt: str,
    *,
    model: str = "demo",
    seed: int = 42,
    backend: Optional[str] = None,
) -> AICapsule:
    """Run deterministic AI execution; returns a capsule view (same as ``execute_ai``)."""
    return execute_ai(model, prompt, int(seed), backend)


def replay(capsule_path: str) -> RunResult:
    """Replay a saved ``.aionai`` capsule; inspect ``replay_symmetry_ok`` and ``deterministic_hash_hex``."""
    return replay_capsule(capsule_path)


def drift(a: str, b: str) -> DriftReport:
    """Deterministic-field drift between two capsule files."""
    return drift_between(a, b)


def why(capsule_path: str) -> Any:
    """Return Why v2 JSON object for a capsule."""
    return json.loads(why_explain(capsule_path))


def validate_capsule(capsule_path: str, policy_path: str) -> GovernanceReport:
    """Validate capsule against a governance policy JSON file."""
    return _governance_validate(capsule_path, policy_path)


__all__ = [
    "AICapsule",
    "DriftReport",
    "GovernanceReport",
    "ReplayComparison",
    "RunResult",
    "run",
    "replay",
    "drift",
    "why",
    "validate_capsule",
    "capsule_load",
    "capsule_save",
    "capsule_deterministic_hash",
    "compare_capsules",
    "graph_causal",
    "execute_ai",
]
