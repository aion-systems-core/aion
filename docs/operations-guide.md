# Operations guide

## Purpose

Map **deterministic JSON envelopes** from reliability, operations, and measurement domains to **SRE workflows** (incident, change, DR, upgrades) and evidence retention—without restating kernel contracts ([Architecture](architecture.md)).

This guide maps SealRun **contract outputs** to platform and SRE workflows: change management, incidents, upgrades, and evidence retention.

## At a glance

- **Reliability** contracts expose SLO, chaos, and soak readiness as structured JSON.
- **Operations** contracts cover runbooks, incidents, DR, and upgrade/migration status.
- **Measurement** contracts cover metrics, KPIs, audits, and evidence export surfaces.
- **Admission control:** treat `sealrun doctor` plus domain JSON as mandatory artefacts in release pipelines.

Isolation and security posture are **not** implied by deterministic execution alone; align workload boundaries with [Security guide](security-guide.md) and your organisational policy.

## Contract surface (operations-relevant)

- Reliability: `reliability_status`, SLO/chaos/soak projections (see CLI reference for exact command mapping).
- Operations: runbooks, incident model, DR status, upgrade/migration status.
- Measurement: metrics, KPIs, audit reports, evidence export hooks.

## CLI surface

```bash
sealrun doctor
sealrun reliability status
sealrun ops runbooks
sealrun ops incidents
sealrun ops dr
sealrun ops upgrade
sealrun measure kpis
sealrun measure audits
```

## SRE flows

| Scenario | Suggested sequence |
|----------|-------------------|
| **Incident triage** | `sealrun doctor` → `sealrun ops incidents` → attach latest capsule/replay JSON from the affected run. |
| **Change / release** | Baseline `doctor` + domain checks → execute smoke capsule + replay → archive JSON under change record. |
| **DR / restore** | `sealrun ops dr` → verify contract outputs against last known good snapshots. |
| **Upgrade** | `sealrun ops upgrade` → re-run replay/drift on reference capsules before traffic shift. |

## Evidence retention

- Persist **CLI JSON envelopes** as the canonical machine-readable record; retain HTML/SVG only if your audit programme requires human-readable annexes.
- Tie each retention object to a **capsule path** or hash referenced in your CMDB or ticket system.
- For governance baselines, store the **baseline JSON** produced by `ci baseline` (see [CI](ci.md)).

## Finality and readiness

Operational **readiness** is the conjunction of: stable contract snapshots, successful replay on reference workloads, acceptable drift against baselines, and no critical gaps in `doctor` and measurement outputs. Exact finality fields are defined in [OS contract spec](os_contract_spec.md).

## Related

- [Architecture](architecture.md)
- [CLI reference](cli-reference.md)
- [Governance](governance.md)
- [Enterprise README](enterprise/README.md)
