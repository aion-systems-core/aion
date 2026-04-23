# CLI reference

This document defines the deterministic CLI surface of SealRun.

## At a glance

- Canonical diagnostics entrypoint: `sealrun doctor`
- Canonical machine output form: deterministic JSON envelope (`status`, `data`, `error`)
- Enterprise command domains: `reliability`, `ops`, `dist`, `governance`, `ux`, `tests`, `measure`

## Contract surface

- Kernel-layer execution: `observe`, `execute`, `policy`, `ci`, `sdk`
- Enterprise-layer contracts: `reliability`, `ops`, `dist`, `governance`, `ux`, `tests`, `measure`

## CLI surface

### Reliability

```bash
sealrun reliability status
sealrun reliability slo
sealrun reliability chaos
sealrun reliability soak
```

### Operations

```bash
sealrun ops runbooks
sealrun ops incidents
sealrun ops dr
sealrun ops upgrade
```

### Distribution

```bash
sealrun dist status
sealrun dist identity
sealrun dist lts
sealrun dist installers
```

### Governance

```bash
sealrun policy packs
sealrun policy gates
sealrun policy evidence
sealrun governance status
```

### UX

```bash
sealrun ux api
sealrun ux cli
sealrun ux admin
sealrun ux golden-paths
```

### Tests

```bash
sealrun tests strategy
sealrun tests regression
sealrun tests compatibility
sealrun tests fuzz-property
```

### Measurement

```bash
sealrun measure metrics
sealrun measure kpis
sealrun measure audits
sealrun measure evidence
```

## Enterprise readiness

- All listed commands map to explicit contracts in `aion-core`.
- `sealrun doctor` aggregates contract state across all enterprise layers.
- Use this surface as the canonical automation/API contract for CI and audit pipelines.
