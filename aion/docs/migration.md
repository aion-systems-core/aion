# Migration guide

This guide defines deterministic upgrade and compatibility checks for SealRun Execution OS environments.

## At a glance

- Rebuild and smoke-test deterministically after upgrades.
- Validate replay continuity and doctor contract outputs.
- Preserve output path and SDK compatibility expectations.

---

SealRun guarantees deterministic execution, replay symmetry, drift detection and audit‑grade evidence chains.  
SealRun intentionally does not enforce filesystem or network isolation.  
The kernel isolation modules are contract surfaces only; they define the interface but do not restrict access.

This is a deliberate design choice: SealRun is an Execution‑OS, not a Security‑Sandbox‑OS.  
Because SealRun does not modify kernel privileges or intercept syscalls, it is safe to adopt in existing environments without admin rights, without risk to workloads, and without operational friction.

If isolation is required (e.g., for regulated industries), the same contract surfaces can be backed by seccomp/landlock/micro‑VM isolation in a future "SealRun Secure Runtime" module — without breaking compatibility.

---

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
- **`SEALRUN_OUTPUT_BASE`** / **`SEALRUN_OUTPUT_ID`** control path and run naming (see `engine/src/output/layout.rs` in the main workspace for alternate env names).

## SDK migration

- SDK version and output overrides read by `engine/src/sdk/output.rs` remain additive for programmatic callers.
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
