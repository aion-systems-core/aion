# Pilot onboarding — Replay capsule

Replay compares a fresh deterministic run to the persisted capsule.

```bash
cargo run -p aion-cli -- execute ai-replay --capsule aion_output/ai/pilot_demo/capsule.aionai
```

(Adjust the path if you used a different `--id` or output base.)

## What to look for

- **`replay_symmetry_ok`** (human or `--json` output) — High-signal pass/fail for pilot reviews.
- Artefacts under `aion_output/ai-replay/<timestamp>/` with structured diff data.

## Next

- [03 — Drift analysis](03_drift_analysis.md)  
- [Replay product doc](../replay.md)
