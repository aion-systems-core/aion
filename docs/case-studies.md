# Case studies

## Purpose

Illustrative deployment patterns; verify any claim against **deterministic JSON** outputs from your build.

This document highlights representative enterprise usage patterns for SealRun.

## At a glance

- Deterministic replay and governance checks as merge gates
- Evidence-centric operations for audit and compliance
- Contract outputs used as machine-readable proof

## CI reproducibility guardrails

Teams use capsule replay + governance baseline checks to block nondeterministic changes before merge.

## Audit-focused workflows

Security and compliance teams use immutable output bundles (`.sealrun.zip`) and audit records for run traceability.

## Contract surface

- Replay/Drift/Evidence contracts in delivery pipelines
- Governance and policy evidence contracts in release approvals
- Measurement/audit contracts for post-release assurance

## CLI surface

```bash
sealrun ci baseline --capsule path/to/capsule.aionai --policy examples/governance/dev.policy.json --determinism examples/governance/dev.determinism.json --integrity examples/governance/dev.integrity.json
sealrun ci check --capsule path/to/candidate.aionai --baseline path/to/baseline-governance.json
sealrun governance status
sealrun measure audits
```

## Enterprise-readiness

Case-study outcomes are enterprise-grade when teams can prove deterministic behavior and governance decisions with reproducible artifacts.
