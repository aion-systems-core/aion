# Pilot onboarding — Drift analysis

## Purpose

Pilot step for **drift detection** between captured runs—ties to the Map-layer contract ([Drift](../drift.md)).

Drift answers: **what changed** between two capsules or between a capsule and a baseline?

Use the CLI **observe** flows (see [Drift](../drift.md) for full syntax). Typical pattern:

1. Capture capsule A (baseline run).
2. Capture capsule B (candidate run).
3. Compare using the CLI/SDK drift commands you already use in CI.

For field-level semantics, prefer **deterministic fields** called out in CLI output or JSON summaries.

## Next

- [04 — Why graph](04_why_graph.md)  
- [Drift reference](../drift.md)
