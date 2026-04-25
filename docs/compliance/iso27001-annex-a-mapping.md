# ISO 27001 Annex A Mapping (non-certified)

This is a best-effort Annex A mapping reference. It does not claim ISO 27001 certification.

| Annex A theme | SealRun mapping | Primary artifacts |
|---|---|---|
| Organizational controls | Risk, exceptions, vendor governance | `docs/policies/risk-management-policy.md`, `docs/policies/exceptions-policy.md`, `docs/policies/vendor-third-party-risk-policy.md` |
| People controls | Access provisioning and role segregation | `docs/policies/access-control-policy.md`, RBAC export |
| Physical controls | Out of open-core scope; operator responsibility | `docs/security-guide.md` shared responsibility section |
| Technological controls | OIDC auth, RBAC, tenant isolation, deterministic evidence, SIEM/OTel, release attestation | `docs/oidc-auth.md`, `docs/rbac.md`, `docs/multi-tenancy.md`, `docs/siem-otel.md`, `docs/release-attestation.md` |
| Operations security | Runbooks, lifecycle controls, escalation path | `docs/runbooks/*.md`, `docs/lifecycle-controls.md`, `docs/support-escalation-path.md` |
| Communications security | External call policy and telemetry controls | `docs/policy-engine.md`, `docs/telemetry.md` |
| System acquisition/development | Deterministic contracts and CI gates | `docs/os_contract_spec.md`, CI workflow docs |
| Supplier relationships | Vendor review policy and template controls | `docs/policies/vendor-third-party-risk-policy.md` |
| Incident management | IR policy, SLAs, incident runbooks | `docs/policies/incident-response-policy.md`, `docs/sla.md`, `docs/runbooks/*.md` |
| Business continuity | Status template, operational readiness, replay-based recovery | `docs/status-page-template.md`, `docs/operations-guide.md` |
