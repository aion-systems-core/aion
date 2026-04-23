# SealRun overview

SealRun is a **deterministic AI execution engine**: it records runs as **capsules**, verifies them with **replay**, compares them with **drift**, explains them with **Why** and **causal graphs**, and enforces rules through machine-readable **contracts** across governance, reliability, operations, distribution, UX, testing, and measurement.

## At a glance

- deterministic execution engine + contract layer model
- 5 kernel layers for deterministic run execution
- Enterprise-layer contracts across phases 1-12
- Canonical diagnostics and readiness output via `sealrun doctor`

---

SealRun guarantees deterministic execution, replay symmetry, drift detection and audit‑grade evidence chains.  
SealRun intentionally does not enforce filesystem or network isolation.  
The kernel isolation modules are contract surfaces only; they define the interface but do not restrict access.

This is a deliberate design choice: SealRun is a deterministic execution engine, not a Security-Sandbox-OS.  
Because SealRun does not modify kernel privileges or intercept syscalls, it is safe to adopt in existing environments without admin rights, without risk to workloads, and without operational friction.

If isolation is required (e.g., for regulated industries), the same contract surfaces can be backed by seccomp/landlock/micro‑VM isolation in a future "SealRun Secure Runtime" module — without breaking compatibility.

---

## Conceptual map

```
  Prompt + model + seed
           │
           ▼
    ┌──────────────┐
    │  AI capsule │  ← serialised run (tokens, evidence, Why, graph)
    └──────┬───────┘
           │
     ┌─────┴─────┬────────────┬──────────────┐
     ▼           ▼            ▼              ▼
  Replay      Drift      Why / Graph   Governance / CI
```

## Who is this for?

- Teams that need **repeatable** AI runs for testing and audits.
- Tooling authors integrating via the **SDK** or **CLI**.
- Pipelines that emit **JSON / HTML / SVG** artefacts for humans and machines.

## Contract surface

- Kernel-layer contracts: replay, replay invariant, drift, evidence chain, policy engine
- Enterprise-layer contracts: governance, reliability, operations, distribution, UX, tests, measurement
- Cross-cutting contracts: identity, finality, compatibility, trust chain, audit export

## CLI surface

```bash
sealrun doctor
sealrun reliability status
sealrun ops runbooks
sealrun dist status
sealrun governance status
sealrun ux api
sealrun tests strategy
sealrun measure metrics
```

## Enterprise-readiness

- Phase 1-12 contracts are implemented and exposed in deterministic JSON envelopes.
- Enterprise automation should treat CLI JSON outputs as the primary machine contract.

## Related docs

- [Architecture](architecture.md)
- [OS Contract Spec](os_contract_spec.md)
- [Installation](installation.md)
- [Quickstart](quickstart.md)
- [Capsules](capsules.md)
- [SDK](sdk.md)
- [Guided tour (trust & pilot)](guided_tour.md) — evidence model, compliance one-pager, pilot path, executive summary
- [CLI reference](cli-reference.md)
- [Developer guide](developer-guide.md)
- [Operations guide](operations-guide.md)
- [Security guide](security-guide.md)
- [Evidence model](evidence/evidence_model.md)
- [Compliance one-pager](compliance/sealrun_compliance_onepager.md)
- [Pilot onboarding](pilot/00_install.md)
- [SealRun in 5 Minuten (executive)](executive/sealrun_in_5_minutes.md)
- [Enterprise sales package (source-anchored)](enterprise/SealRun_Enterprise_Sales_Package.md) · [Specs stub](specs/full.md)

## Enterprise status at a glance

- Phase 1-12 enterprise contracts are implemented in `aion-core` and surfaced in `sealrun doctor`.
- Contract-centric command groups: `reliability`, `ops`, `dist`, `governance`, `ux`, `tests`, `measure`.
- Deterministic JSON envelopes are the default audit interface for automation and compliance tooling.
