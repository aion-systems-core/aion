#!/usr/bin/env python3
"""Pilot example: run deterministic AI and optionally save capsule."""

from __future__ import annotations

import tempfile

import aion

cap = aion.run("hello from AION pilot", model="demo", seed=7)
print("AION | execute (pilot API)")
print("  model:", cap.model)
print("  seed:", cap.seed)
print("  determinism_profile:", cap.determinism_profile[:120], "...")

path = tempfile.NamedTemporaryFile(suffix=".aionai", delete=False).name
aion.capsule_save(cap, path)
print("  saved:", path)

h = aion.capsule_deterministic_hash(path)
print("  deterministic_hash_hex:", h[:32], "...")
