#!/usr/bin/env python3
"""Pilot example: causal Why + graph JSON for one capsule."""

from __future__ import annotations

import json
import sys

import aion

if len(sys.argv) < 2:
    print("usage: 04_why_graph.py <path/to/capsule.aionai>")
    sys.exit(2)

path = sys.argv[1]
why = aion.why(path)
graph = json.loads(aion.graph_causal(path))
print("AION | deterministic causal view")
print("  why summary:", why.get("summary", "")[:200])
print("  graph edges:", len(graph.get("edges", [])))
