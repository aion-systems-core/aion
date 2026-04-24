# Pilot onboarding — Execute capsule

## Purpose

Pilot step producing the first **deterministic capsule** (`capsule.aionai`) and Why/evidence projections—prerequisite for replay ([next step](02_replay_capsule.md)).

Produce a deterministic **AI capsule** and artefacts under `<output_base>/ai/<id>/` (set `AION_OUTPUT_BASE` if you want `sealrun_output/`).

```bash
sealrun execute ai --model demo --prompt "hello pilot" --seed 42 --id pilot_demo
```

You should see paths for `capsule.aionai`, `ai.json`, evidence files, and Why/graph HTML or SVG where enabled.

## What to look for

- **`capsule.aionai`** — Canonical run record for replay and governance.
- **Determinism metadata** — Frozen time / RNG policy used for the run.

## Next

- [02 — Replay capsule](02_replay_capsule.md)  
- [Evidence model](../evidence/evidence_model.md)
