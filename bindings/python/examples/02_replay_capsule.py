#!/usr/bin/env python3
"""Pilot example: replay a capsule and read symmetry + hash."""

from __future__ import annotations

import sys

import aion

if len(sys.argv) < 2:
    print("usage: 02_replay_capsule.py <path/to/capsule.aionai>")
    sys.exit(2)

rep = aion.replay(sys.argv[1])
print("AION | replay")
print("  exit_code:", rep.exit_code)
print("  replay_symmetry_ok:", rep.replay_symmetry_ok)
print("  deterministic_hash_hex:", rep.deterministic_hash_hex)
