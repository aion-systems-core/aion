# Pilot onboarding — Replay capsule

## Purpose

Pilot step for **replay symmetry** on an existing `capsule.aionai` ([Replay](../replay.md)).

Replay compares a fresh deterministic run to the persisted capsule.

```bash
sealrun execute ai-replay --capsule sealrun_output/ai/pilot_demo/capsule.aionai
```

(Adjust the path if you used a different `--id` or output base.)

## What to look for

- **`replay_symmetry_ok`** (human or `--json` output) — High-signal pass/fail for pilot reviews.
- Artefacts under `sealrun_output/ai-replay/<timestamp>/` with structured diff data.

## Next

- [03 — Drift analysis](03_drift_analysis.md)  
- [Replay product doc](../replay.md)
