# Controls Matrix (SOC2-style, non-certified)

This matrix is a control design reference for SealRun. It does not claim SOC 2 certification.

| Control ID | Domain | Control objective | Implementation evidence | Frequency | Owner |
|---|---|---|---|---|---|
| CC-01 | Access | Restrict privileged actions to authorized roles | `sealrun enterprise rbac export`, assignment history | Continuous | Security |
| CC-02 | Access | Authenticate enterprise users via OIDC | `sealrun enterprise auth status`, IdP config records | Continuous | Security |
| CC-03 | Change mgmt | Enforce tracked and test-validated releases | CI logs, `cargo test --workspace --all-targets`, release notes | Per release | Engineering |
| CC-04 | Availability | Detect and triage replay/drift failures | Runbooks, incident tickets, replay/drift outputs | Continuous | SRE |
| CC-05 | Integrity | Ensure deterministic replay and evidence continuity | `sealrun execute ai-replay`, evidence chain outputs | Continuous | Platform |
| CC-06 | Security | Preserve tenant isolation at storage layer | tenant indexes, isolation tests | Continuous | Platform |
| CC-07 | Security | Enforce retention, purge, and legal hold workflows | lifecycle command outputs, audit trail | Daily | SRE |
| CC-08 | Monitoring | Export events to SIEM and OTel pipelines | sink test outputs, OTel export logs | Continuous | SRE |
| CC-09 | Supply chain | Produce signed release attestations and SBOM | Cosign output, SBOM artifact, verify output | Per release | Release Eng |
| CC-10 | Governance | Validate policy constraints before acceptance | `policy-api validate/evaluate` outputs | Continuous | Compliance |
| CC-11 | Vendor risk | Assess third-party dependencies and tooling impact | vendor register, risk log, exception approvals | Quarterly | Security |
| CC-12 | Incident response | Execute defined response process with SLAs | incident runbooks, SLA report, postmortems | Per incident | SRE |
