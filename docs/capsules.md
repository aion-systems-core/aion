# Capsules

A **capsule** is the durable record of a deterministic AI run: model, prompt, seed, emitted tokens, traces, evidence, and explainability payloads (Why report and causal graph).

## At a glance

- Capsule is the canonical run artifact in the deterministic execution engine kernel layer.
- Capsules are replayable, drift-comparable, and policy-verifiable.
- Capsules feed evidence and governance contracts in the enterprise layer.

---

SealRun guarantees deterministic execution, replay symmetry, drift detection and audit‑grade evidence chains.  
SealRun intentionally does not enforce filesystem or network isolation.  
The kernel isolation modules are contract surfaces only; they define the interface but do not restrict access.

This is a deliberate design choice: SealRun is an Execution‑OS, not a Security‑Sandbox‑OS.  
Because SealRun does not modify kernel privileges or intercept syscalls, it is safe to adopt in existing environments without admin rights, without risk to workloads, and without operational friction.

If isolation is required (e.g., for regulated industries), the same contract surfaces can be backed by seccomp/landlock/micro‑VM isolation in a future "SealRun Secure Runtime" module — without breaking compatibility.

---

## Why capsules matter

Capsules are the **unit of audit**: you can archive them, diff them, replay them, and validate them against governance profiles.

## CLI: create a capsule

```bash
sealrun execute ai --model M --prompt "your text" --seed 42
```

The on-disk capsule is typically named `capsule\.sealrunai` inside the output directory.

## CLI: replay artefacts

```bash
sealrun execute ai-replay --capsule /path/to/capsule\.sealrunai
```

## Contract surface

- State-Contract input for replay and identity/finality checks
- Evidence-Contract input for chain verification
- Governance input for policy validation and policy evidence

## CLI surface

```bash
sealrun execute ai --model M --prompt "your text" --seed 42
sealrun execute ai-replay --capsule /path/to/capsule\.sealrunai
sealrun policy validate --capsule /path/to/capsule\.sealrunai --policy examples/governance/dev.policy.json
```

## Conceptual diagram

```
  ┌─────────────┐
  │  Capsule    │
  ├─────────────┤
  │ tokens      │
  │ evidence    │
  │ why + graph │
  └─────────────┘
```

## Related

- [Replay](replay.md)
- [Drift](drift.md)
- [SDK](sdk.md) — `sdk capsule build|load`
- [AI capsule schema](ai-capsule-schema.json)
- [Example capsule JSON](example-capsule.json)

## Enterprise-readiness

Capsules are enterprise-ready when serialization, replay behavior, and evidence linkage remain deterministic across supported versions.
