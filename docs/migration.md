# Migration guide

## Purpose

Checklist for **version upgrades**: rebuild, replay continuity, `doctor` JSON stability, and SDK/env compatibility—without implying format changes ([Compatibility matrix](compatibility-matrix.md)).

This guide defines deterministic upgrade and compatibility checks for SealRun environments.

## At a glance

- Rebuild and smoke-test deterministically after upgrades.
- Validate replay continuity and doctor contract outputs.
- Preserve output path and SDK compatibility expectations.

## Upgrading from previous local build

1. Pull latest repository changes.
2. Rebuild CLI:
   - `cargo build -p aion-cli --release`
3. Validate with smoke commands:
   - `sealrun --version`
   - `sealrun execute ai --model demo --prompt "migration smoke" --seed 1`
   - `sealrun execute ai-replay --capsule <latest capsule>`

## Output layout migration

- Current outputs are deterministic under `<base>/<command>/<run_id>/`.
- `SEALRUN_OUTPUT_BASE` and `SEALRUN_OUTPUT_ID` can be used to control path and run naming.

## SDK migration

- `SEALRUN_SDK_VERSION` and `SEALRUN_SDK_OUTPUT_BASE` are additive.
- Existing `sealrun sdk` commands remain compatible.

## CLI surface

```bash
sealrun --version
sealrun doctor
sealrun execute ai --model demo --prompt "migration smoke" --seed 1
sealrun execute ai-replay --capsule <latest capsule>
```

## Enterprise-readiness

Migration readiness requires compatibility-safe version transitions and deterministic replay/doctor outcomes across supported windows.
