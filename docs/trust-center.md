# Trust Center

This document is the public trust-center baseline for SealRun enterprise programs.

## Scope

- Deterministic execution controls (capsule, replay, drift, evidence)
- Governance controls (policy packs, policy gates, policy evidence)
- Supply-chain controls (signed releases, provenance, SBOM)
- Security controls (threat model, scanning, logging policy, vulnerability SLA)

## Control families

- Access control: SSO, RBAC, tenant isolation
- Auditability: deterministic governance events, evidence chains
- Integrity: release signatures, provenance verification, SBOM verification
- Compliance operations: retention policy, incident and runbook contracts

## Evidence sources

- `sealrun enterprise auth status`
- `sealrun enterprise audit-events`
- `sealrun enterprise trust-center`
- `sealrun enterprise release-attestation sign`
- `sealrun enterprise release-attestation verify`
- `sealrun enterprise release-attestation sbom`
- `sealrun enterprise tenants list`
- `sealrun enterprise lifecycle retention get --tenant <id>`
- `sealrun enterprise rbac export`
- `sealrun enterprise policy-api validate --policy <path>`
- `sealrun governance status`
- `sealrun doctor`

## Enterprise capability map

- Multi-tenancy: `enterprise tenants *` and tenant-scoped capsule/evidence indexes
- Lifecycle controls: retention, purge, legal-hold per tenant
- RBAC: YAML-backed role assignment and permission checks
- OIDC: device-code login/logout/status with local token store
- SIEM + OTel: test pipelines for Splunk/Datadog/Elastic and OTLP export
- Release attestations: Cosign/Sigstore integration plus `cargo sbom`

## Shared responsibility

SealRun is an execution contract layer, not a host isolation layer.
Filesystem, network, and runtime isolation must be enforced by platform operators or enterprise runtime modules.
