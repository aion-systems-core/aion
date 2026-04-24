# SealRun Enterprise Sales Package

## Purpose

Enterprise narrative and **12-phase contract roadmap** aligned with the normative [OS Contract Spec](../os_contract_spec.md) and [Architecture](../architecture.md).

> Deterministic AI execution engine for enterprise-grade replay, evidence, governance, and auditability.
>  
> Status: up to date with the 12-phase enterprise contract roadmap.

---

## Executive summary

SealRun provides deterministic AI run execution with machine-readable contracts across the full enterprise lifecycle:

- deterministic execution and replay
- drift and evidence contracts
- governance, security, compliance, and release contracts
- reliability, operations, distribution, and identity contracts
- UX stability and test strategy contracts
- measurement, KPI, audit, and evidence export contracts

The core product truth is emitted via deterministic JSON outputs and aggregated by `sealrun doctor`.

---

## At a glance

- Category: deterministic AI deterministic execution engine with contract layer governance
- Readiness model: phase 1-12 contract program
- Buyer proof anchor: deterministic CLI + doctor contract outputs

---

SealRun guarantees deterministic execution, replay symmetry, drift detection and audit‑grade evidence chains.  
SealRun intentionally does not enforce filesystem or network isolation.  
The kernel isolation modules are contract surfaces only; they define the interface but do not restrict access.

This is a deliberate design choice: SealRun is an Execution‑OS, not a Security‑Sandbox‑OS.  
Because SealRun does not modify kernel privileges or intercept syscalls, it is safe to adopt in existing environments without admin rights, without risk to workloads, and without operational friction.

If isolation is required (e.g., for regulated industries), the same contract surfaces can be backed by seccomp/landlock/micro‑VM isolation in a future "SealRun Secure Runtime" module — without breaking compatibility.

---

## Product positioning

### What SealRun is

- A deterministic deterministic execution engine for AI workloads.
- A contract-driven evidence and governance layer over replayable AI runs.
- A machine-readable audit surface for engineering, compliance, and security teams.

### What SealRun is not

- Not a foundation model provider.
- Not a replacement for customer SIEM/GRC tooling.
- Not a promise of external certifications by default.

---

## Enterprise-ready capabilities (phase 1-12)

| Domain | Contract surface | Outcome |
|---|---|---|
| Core determinism | Replay, Drift, Evidence, Policy, Global consistency | Stable and reproducible run behavior |
| Versioning and compatibility | Upgrade replay, ABI, contract stability, identity matrix | Predictable N/N-1/N-2 behavior |
| Supply chain and security | Release signing, provenance, SBOM, vulnerability SLA, security model | Auditable software delivery |
| Reliability and operations | SLO, reliability, chaos/soak, runbooks, incidents, DR, migration | Production operations control |
| Distribution and support | Distribution contract, LTS policy, installer trust chain | Clear support and package trust story |
| Developer and enterprise UX | API/CLI stability, admin docs, golden paths | Safe adoption and onboarding |
| Test and measurement | Test strategy, regression, compatibility, fuzz/property, KPI/audit/evidence export | Ongoing measurable confidence |

---

## Contract surface

- Kernel-layer contracts: replay, drift, evidence, policy, global consistency
- Enterprise-layer contracts: reliability, operations, distribution, governance, UX, tests, measurement
- Cross-cutting contracts: identity, compatibility, trust chain, audit report/export

---

## Core proof points for buyers

### Determinism and evidence

- Capsules are canonical and replayable.
- Replay symmetry/invariant and drift are deterministic.
- Evidence chain is machine-readable and verifiable.

### Governance and compliance

- Policy packs, policy gates, and policy evidence are first-class contracts.
- No silent bypass model: decisions are explicit and machine-readable.
- Governance model aggregates policy, security, compliance, release, and operations domains.

### Operations and supportability

- Reliability and operations contracts model incident, rollback, DR, and migration behavior.
- Distribution model defines support, identity compatibility, LTS windowing, and installer trust.
- Measurement model defines KPIs, metrics, audit findings, and evidence exports.

---

## Buyer-relevant command surface

### Core diagnostics

```bash
sealrun doctor
sealrun version --full
```

### 7-domain enterprise CLI surface

`reliability`, `ops`, `dist`, `governance`, `ux`, `tests`, `measure`

### Governance and policy

```bash
sealrun policy packs
sealrun policy gates
sealrun policy evidence
sealrun governance status
```

### Reliability and operations

```bash
sealrun reliability status
sealrun ops runbooks
sealrun ops incidents
sealrun ops dr
sealrun ops upgrade
```

### Distribution and UX

```bash
sealrun dist status
sealrun dist identity
sealrun dist lts
sealrun dist installers
sealrun ux api
sealrun ux cli
sealrun ux admin
sealrun ux golden-paths
```

### Test and measurement

```bash
sealrun tests strategy
sealrun tests regression
sealrun tests compatibility
sealrun tests fuzz-property
sealrun measure metrics
sealrun measure kpis
sealrun measure audits
sealrun measure evidence
```

---

## Pilot framework (contract-first)

1. Validate environment and doctor outputs.
2. Execute and replay deterministic capsules.
3. Validate policy gates and policy evidence.
4. Validate evidence chain and governance status.
5. Validate reliability and operations contracts.
6. Validate distribution and identity/LTS contracts.
7. Validate UX, test, and measurement contracts.
8. Freeze baseline and run CI regression checks.

Deliverable: deterministic artifact bundle + doctor report + policy/governance/measurement proofs.

---

## Messaging kit

### One sentence

SealRun turns AI execution into deterministic, replayable, auditable contracts for enterprise engineering and compliance.

### 30-second pitch

SealRun is a deterministic AI deterministic execution engine: runs are sealed as capsules, replay and drift are verifiable, policy and governance are machine-readable, and operations/distribution/test/measurement readiness is exposed through one deterministic contract surface (`sealrun doctor` + CLI contracts).

### 2-minute walkthrough

Start with deterministic run lifecycle (capsule/replay/drift/evidence), then governance/security/release, then reliability/operations/distribution, and close with UX/test/measurement contracts to show complete enterprise control from build to audit.

---

## Commercial guidance

Pricing and legal terms are intentionally not hard-coded in repository docs. Use this package as technical due-diligence substrate and layer commercial terms in your standard proposal process.

---

## Enterprise-readiness

- SealRun is enterprise-ready when contracts remain deterministic, versioned, and auditable across releases.
- Production rollout should require passing doctor, governance, reliability, operations, distribution, testing, and measurement checks.

---

## Related docs

- [OS Contract Spec](../os_contract_spec.md)
- [Architecture](../architecture.md)
- [Overview](../overview.md)
- [Guided tour](../guided_tour.md)
- [Compliance one-pager](../compliance/sealrun_compliance_onepager.md)

---

## Technical documentation

- [OS Contract Spec](../os_contract_spec.md)
- [Architecture](../architecture.md)
- [Compliance one-pager](../compliance/sealrun_compliance_onepager.md)
