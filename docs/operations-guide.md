# Operations guide

This guide maps AION-OS contracts to SRE and platform operations workflows.

## At a glance

- Reliability contracts define SLOs, budgets, chaos, and soak readiness.
- Operations contracts define runbooks, incident response, DR, and migration paths.
- Distribution and measurement contracts provide supportability and audit reporting.

---

AION guarantees deterministic execution, replay symmetry, drift detection and audit‑grade evidence chains.  
AION intentionally does not enforce filesystem or network isolation.  
The kernel isolation modules are contract surfaces only; they define the interface but do not restrict access.

This is a deliberate design choice: AION is an Execution‑OS, not a Security‑Sandbox‑OS.  
Because AION does not modify kernel privileges or intercept syscalls, it is safe to adopt in existing environments without admin rights, without risk to workloads, and without operational friction.

If isolation is required (e.g., for regulated industries), the same contract surfaces can be backed by seccomp/landlock/micro‑VM isolation in a future "AION Secure Runtime" module — without breaking compatibility.

---

## Contract surface

- Reliability: `slo_status`, `reliability_status`, `chaos_status`, `soak_status`
- Operations: `runbooks`, `incident_model`, `dr_status`, `upgrade_migration_status`
- Measurement: `metrics_contract`, `kpi_contract`, `audit_reports`, `evidence_export`, `measurement_model`

## CLI surface

```bash
aion reliability status
aion ops runbooks
aion ops incidents
aion ops dr
aion ops upgrade
aion measure kpis
aion measure audits
```

## SRE flows

- **Incident triage:** start with `aion doctor`, then `aion ops incidents`.
- **Rollback/migration:** validate `aion ops upgrade` before release transitions.
- **DR checks:** run `aion ops dr` and track restore-plan status in release sign-off.
- **SLO tracking:** use `aion reliability slo` and `aion measure kpis` for regular reviews.

## Finality rules

- Run-level finality is defined in global consistency contract outputs.
- Operations finality depends on complete runbooks, incident plan, DR readiness, and migration steps.
- Measurement finality depends on no critical gaps in metrics/KPI/audit/export contracts.

## Enterprise readiness

- Make `aion doctor` and `aion ops/reliability/measure` outputs mandatory in operational change reviews.
- Persist JSON envelopes as primary operational evidence artifacts.
