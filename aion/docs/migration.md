# Migration guide

This guide defines deterministic upgrade and compatibility checks for AION-OS environments.

## At a glance

- Rebuild and smoke-test deterministically after upgrades.
- Validate replay continuity and doctor contract outputs.
- Preserve output path and SDK compatibility expectations.

---

AION guarantees deterministic execution, replay symmetry, drift detection and audit‑grade evidence chains.  
AION intentionally does not enforce filesystem or network isolation.  
The kernel isolation modules are contract surfaces only; they define the interface but do not restrict access.

This is a deliberate design choice: AION is an Execution‑OS, not a Security‑Sandbox‑OS.  
Because AION does not modify kernel privileges or intercept syscalls, it is safe to adopt in existing environments without admin rights, without risk to workloads, and without operational friction.

If isolation is required (e.g., for regulated industries), the same contract surfaces can be backed by seccomp/landlock/micro‑VM isolation in a future "AION Secure Runtime" module — without breaking compatibility.

---

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
