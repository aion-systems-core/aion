# CLI reference

This document defines the deterministic CLI surface of AION-OS.

## At a glance

- Canonical diagnostics entrypoint: `aion doctor`
- Canonical machine output form: deterministic JSON envelope (`status`, `data`, `error`)
- Enterprise command domains: `reliability`, `ops`, `dist`, `governance`, `ux`, `tests`, `measure`

## Contract surface

- Kernel-layer execution: `observe`, `execute`, `policy`, `ci`, `sdk`
- Enterprise-layer contracts: `reliability`, `ops`, `dist`, `governance`, `ux`, `tests`, `measure`

## CLI surface

### Reliability

```bash
aion reliability status
aion reliability slo
aion reliability chaos
aion reliability soak
```

### Operations

```bash
aion ops runbooks
aion ops incidents
aion ops dr
aion ops upgrade
```

### Distribution

```bash
aion dist status
aion dist identity
aion dist lts
aion dist installers
```

### Governance

```bash
aion policy packs
aion policy gates
aion policy evidence
aion governance status
```

### UX

```bash
aion ux api
aion ux cli
aion ux admin
aion ux golden-paths
```

### Tests

```bash
aion tests strategy
aion tests regression
aion tests compatibility
aion tests fuzz-property
```

### Measurement

```bash
aion measure metrics
aion measure kpis
aion measure audits
aion measure evidence
```

## Enterprise readiness

- All listed commands map to explicit contracts in `aion-core`.
- `aion doctor` aggregates contract state across all enterprise layers.
- Use this surface as the canonical automation/API contract for CI and audit pipelines.
