# Enterprise Guide

This section groups enterprise-facing AION documentation, including readiness, governance, and technical contract references.

- [AION Enterprise Sales Package](./AION_Enterprise_Sales_Package.md)
- [OS Contract Specification](../os_contract_spec.md)
- [Architecture](../architecture.md)
- [Security Guide](../security-guide.md)

## Enterprise contract scope

AION extends kernel determinism with enterprise-grade contracts across governance, reliability, operations, distribution, UX, testing, and measurement.

### Governance and policy

- Deterministic policy packs, gates, and policy evidence outputs
- Explicit machine-readable decisions with no silent bypass model
- Governance status aggregation through `aion governance status` and `aion doctor`

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
