# Enterprise Guide

## Purpose

Index of **enterprise-facing** SealRun material (readiness, governance narrative, sales package) with links to normative **OS contract** and **architecture** docs.

This section groups enterprise-facing SealRun documentation, including readiness, governance, and technical contract references.

- [SealRun Enterprise Sales Package](./SealRun_Enterprise_Sales_Package.md)
- [Pricing & commercial support](../pricing.md)
- [OS Contract Specification](../os_contract_spec.md)
- [Architecture](../architecture.md)
- [Security Guide](../security-guide.md)

## Enterprise contract scope

SealRun extends kernel determinism with enterprise-grade contracts across governance, reliability, operations, distribution, UX, testing, and measurement.

### Governance and policy

- Deterministic policy packs, gates, and policy evidence outputs
- Explicit machine-readable decisions with no silent bypass model
- Governance status aggregation through `sealrun governance status` and `sealrun doctor`

### Reliability and operations

- SLO, reliability, chaos, and soak contract surfaces
- Runbooks, incident response, DR, and upgrade/migration contract outputs
- Deterministic JSON envelopes for operational review and audit workflows

### Distribution, identity, and supportability

- Distribution status, identity matrix, LTS policy, and installer trust chain contracts
- Explicit compatibility semantics for controlled rollout and support windows

### UX, tests, and measurement

- API/CLI/admin/golden-path stability contracts
- Test strategy, regression, compatibility, and fuzz/property contract surfaces
- Metrics, KPI, audit report, and evidence export contracts for continuous assurance

## Related CLI domains

- `reliability`
- `ops`
- `dist`
- `governance`
- `ux`
- `tests`
- `measure`
- `enterprise`

## Enterprise CLI (new)

```bash
sealrun enterprise tenants list
sealrun enterprise lifecycle retention get --tenant <id>
sealrun enterprise rbac export
sealrun enterprise auth status
sealrun enterprise sinks send-test --sink splunk --endpoint <url> --token <token>
sealrun enterprise otel export --endpoint <url>
sealrun enterprise release-attestation sbom
sealrun enterprise policy-api validate --policy policy.json
```

Detailed docs:

- [Multi-tenancy](../multi-tenancy.md)
- [Lifecycle controls](../lifecycle-controls.md)
- [RBAC](../rbac.md)
- [OIDC auth](../oidc-auth.md)
- [SIEM and OTel](../siem-otel.md)
- [Release attestation](../release-attestation.md)
- [Policy engine](../policy-engine.md)
