# Migration guide

This guide defines deterministic upgrade and compatibility checks for AION-OS environments.

## At a glance

- Rebuild and smoke-test deterministically after upgrades.
- Validate replay continuity and doctor contract outputs.
- Preserve output path and SDK compatibility expectations.

## Upgrading from previous local build

1. Pull latest repository changes.
2. Rebuild CLI:
   - `cargo build -p aion-cli --release`
3. Validate with smoke commands:
   - `aion --version`
   - `aion execute ai --model demo --prompt "migration smoke" --seed 1`
   - `aion execute ai-replay --capsule <latest capsule>`

## Output layout migration

- Current outputs are deterministic under `<base>/<command>/<run_id>/`.
- `AION_OUTPUT_BASE` and `AION_OUTPUT_ID` can be used to control path and run naming.

## SDK migration

- `AION_SDK_VERSION` and `AION_SDK_OUTPUT_BASE` are additive.
- Existing `aion sdk` commands remain compatible.

## CLI surface

```bash
aion --version
aion doctor
aion execute ai --model demo --prompt "migration smoke" --seed 1
aion execute ai-replay --capsule <latest capsule>
```

## Enterprise-readiness

Migration readiness requires compatibility-safe version transitions and deterministic replay/doctor outcomes across supported windows.
