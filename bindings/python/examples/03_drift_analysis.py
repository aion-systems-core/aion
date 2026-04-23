#!/usr/bin/env python3
"""Pilot example: drift between two capsules (deterministic fields)."""

from __future__ import annotations

import sys

import aion

if len(sys.argv) < 3:
    print("usage: 03_drift_analysis.py <left.aionai> <right.aionai>")
    sys.exit(2)

d = aion.drift(sys.argv[1], sys.argv[2])
print("AION | drift (deterministic subset)")
print("  changed:", d.changed)
print("  fields:", d.fields)
