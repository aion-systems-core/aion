#!/usr/bin/env python3
"""Pilot example: governance validate (policy JSON vs capsule)."""

from __future__ import annotations

import sys
from pathlib import Path

import aion

if len(sys.argv) < 3:
    repo = Path(__file__).resolve().parents[3]
    default_pol = repo / "examples" / "governance" / "dev.policy.json"
    print(
        "usage: 05_governance_validate.py <capsule.aionai> <policy.json>\n"
        f"example policy: {default_pol}"
    )
    sys.exit(2)

r = aion.validate_capsule(sys.argv[1], sys.argv[2])
print("AION | governance validate")
print("  policy_ok:", r.policy_ok)
print("  determinism_ok:", r.determinism_ok)
print("  integrity_ok:", r.integrity_ok)
