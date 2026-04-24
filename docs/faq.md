# FAQ

## Purpose

Fast answers for **developers**, **operators**, and **security/compliance** roles: sandbox scope, capsule vs. replay vs. drift, evidence, CI, and where to read authoritative contracts.

## At a glance

SealRun is a deterministic execution engine: runs are sealed into **capsules**, checked with **replay**, compared with **drift**, and anchored with **evidence** and **governance** contracts. Authoritative contract definitions live in [OS contract spec](os_contract_spec.md) and [Architecture](architecture.md).

## What is SealRun?

A system for **recording**, **replaying**, and **comparing** AI runs under explicit inputs (model, prompt, seed, determinism profile) so outputs are **machine-checkable**, not only human-readable logs.

## Is SealRun a security sandbox?

No. SealRun does **not** rely on syscall interception or mandatory filesystem/network isolation in the open-core path. Isolation is a **separate contract surface**; for regulated deployments, enforce isolation at the workload boundary (containers, seccomp, micro-VMs, enterprise runtime). See [Security guide](security-guide.md).

## What is a capsule?

A **versioned, serializable record** of a run (tokens, evidence digests, explainability payloads, metadata). It is the **unit of audit** for replay, drift, and policy validation. See [Capsules](capsules.md).

## How do replay and drift differ?

- **Replay** re-executes from a capsule and reports **symmetry** against the stored record (pass/fail, mismatch locus).
- **Drift** compares **two artefacts** (runs or capsules) and classifies **field-level** differences for admission control.

See [Replay](replay.md) and [Drift](drift.md).

## How do I verify determinism operationally?

1. Produce a capsule: `sealrun execute ai …`
2. Replay it: `sealrun execute ai-replay --capsule …`
3. Inspect `sealrun doctor` and domain contracts (`sealrun tests strategy`, governance/CI commands as needed).

Treat **deterministic JSON envelopes** from the CLI as the primary machine contract in CI.

## How do I enforce policies in CI?

Record a baseline, then gate candidates:

- `sealrun ci baseline …` / `sealrun ci check …` (see [CI](ci.md))
- `sealrun policy validate …` and `sealrun governance status` for policy surface coverage

Archive emitted JSON (and linked HTML/SVG if required by your evidence policy).

## What is the evidence chain?

A **linear, digest-linked** representation of execution proof attached to capsules and export surfaces. It supports **integrity checks** and audit workflows; exact fields are defined in the evidence model. See [Evidence model](evidence/evidence_model.md).

## Does telemetry run by default?

No. Execution defaults to **local filesystem outputs**; telemetry is **opt-in** where implemented.

## Which CLI domains are canonical for enterprise readiness?

The seven deterministic domains: `reliability`, `ops`, `dist`, `governance`, `ux`, `tests`, `measure`. Use them together with `sealrun doctor` for release and operational sign-off. See [CLI reference](cli-reference.md) and [Operations guide](operations-guide.md).

## Where should a CISO or compliance officer start?

- [Compliance One-Pager](compliance/sealrun_compliance_onepager.md)
- [Security guide](security-guide.md)
- [Governance](governance.md)
- [Enterprise materials](enterprise/README.md)

## Where should an integrator start?

- [SDK](sdk.md)
- [Developer guide](developer-guide.md)
- [OS contract spec](os_contract_spec.md)
