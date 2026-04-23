# Pilot onboarding — Execute capsule

Produce a deterministic **AI capsule** and artefacts under `sealrun_output/ai/<id>/`.

```bash
sealrun execute ai --model demo --prompt "hello pilot" --seed 42 --id pilot_demo
```

You should see paths for `capsule\.sealrunai`, `ai.json`, evidence files, and Why/graph HTML or SVG where enabled.

## What to look for

- **`capsule\.sealrunai`** — Canonical run record for replay and governance.
- **Determinism metadata** — Frozen time / RNG policy used for the run.

## Next

- [02 — Replay capsule](02_replay_capsule.md)  
- [Evidence model](../evidence/evidence_model.md)
