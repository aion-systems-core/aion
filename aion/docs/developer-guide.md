# Developer guide

This guide provides deterministic developer onboarding for SealRun Execution OS.

## At a glance

- Build once, run deterministic capsules, validate replay/drift/evidence
- Use contract commands for governance, testing, and measurement
- Treat JSON envelopes as the stable automation interface

---

SealRun guarantees deterministic execution, replay symmetry, drift detection and audit‑grade evidence chains.  
SealRun intentionally does not enforce filesystem or network isolation.  
The kernel isolation modules are contract surfaces only; they define the interface but do not restrict access.

This is a deliberate design choice: SealRun is an Execution‑OS, not a Security‑Sandbox‑OS.  
Because SealRun does not modify kernel privileges or intercept syscalls, it is safe to adopt in existing environments without admin rights, without risk to workloads, and without operational friction.

If isolation is required (e.g., for regulated industries), the same contract surfaces can be backed by seccomp/landlock/micro‑VM isolation in a future "SealRun Secure Runtime" module — without breaking compatibility.

---

## Contract surface

- Replay/Drift/Evidence contracts for run-level determinism
- Governance contracts for policy packs, gates, evidence, and domain status
- Test and measurement contracts for readiness and reporting

## CLI surface

### 1) Run and replay

```bash
cargo run -p aion-cli -- execute ai --model demo --prompt "hello" --seed 1
cargo run -p aion-cli -- execute ai-replay --capsule path/to/capsule.aionai
```

### 2) Drift and policy validation

```bash
cargo run -p aion-cli -- policy validate --capsule path/to/capsule.aionai --policy examples/governance/dev.policy.json
cargo run -p aion-cli -- policy gates
```

### 3) Deterministic diagnostics

```bash
cargo run -p aion-cli -- doctor
cargo run -p aion-cli -- tests strategy
cargo run -p aion-cli -- measure metrics
```

## Replay/Drift/Evidence flows

- Replay flow: capsule -> replay comparison -> replay/evidence finality
- Drift flow: canonical diff categories -> deterministic labels -> tolerance profile
- Evidence flow: sealed chain -> linear verification -> audit/export surfaces

## Identity and distribution flows

```bash
cargo run -p aion-cli -- dist identity
cargo run -p aion-cli -- dist status
```

Use identity/distribution outputs to validate environment support, ABI/contract alignment, and support status.

## Enterprise readiness

- Developers should gate merges on deterministic contracts, not free-form logs.
- Add new features only when contract outputs and tests remain deterministic.
