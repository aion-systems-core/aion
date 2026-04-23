# Operations guide

This guide maps SealRun contracts to SRE and platform operations workflows.

## At a glance

- Reliability contracts define SLOs, budgets, chaos, and soak readiness.
- Operations contracts define runbooks, incident response, DR, and migration paths.
- Distribution and measurement contracts provide supportability and audit reporting.

---

SealRun guarantees deterministic execution, replay symmetry, drift detection and audit‑grade evidence chains.  
SealRun intentionally does not enforce filesystem or network isolation.  
The kernel isolation modules are contract surfaces only; they define the interface but do not restrict access.

This is a deliberate design choice: SealRun is an Execution‑OS, not a Security‑Sandbox‑OS.  
Because SealRun does not modify kernel privileges or intercept syscalls, it is safe to adopt in existing environments without admin rights, without risk to workloads, and without operational friction.

If isolation is required (e.g., for regulated industries), the same contract surfaces can be backed by seccomp/landlock/micro‑VM isolation in a future "SealRun Secure Runtime" module — without breaking compatibility.

---

## Contract surface

- Reliability: `slo_status`, `reliability_status`, `chaos_status`, `soak_status`
- Operations: `runbooks`, `incident_model`, `dr_status`, `upgrade_migration_status`
- Measurement: `metrics_contract`, `kpi_contract`, `audit_reports`, `evidence_export`, `measurement_model`

## CLI surface

```bash
sealrun reliability status
sealrun ops runbooks
sealrun ops incidents
sealrun ops dr
sealrun ops upgrade
sealrun measure kpis
sealrun measure audits
```

## SRE flows

- **Incident triage:** start with `sealrun doctor`, then `sealrun ops incidents`.
- **Rollback/migration:** validate `sealrun ops upgrade` before release transitions.
- **DR checks:** run `sealrun ops dr` and track restore-plan status in release sign-off.
- **SLO tracking:** use `sealrun reliability slo` and `sealrun measure kpis` for regular reviews.

## Finality rules

- Run-level finality is defined in global consistency contract outputs.
- Operations finality depends on complete runbooks, incident plan, DR readiness, and migration steps.
- Measurement finality depends on no critical gaps in metrics/KPI/audit/export contracts.

## Enterprise readiness

- Make `sealrun doctor` and `sealrun ops/reliability/measure` outputs mandatory in operational change reviews.
- Persist JSON envelopes as primary operational evidence artifacts.
