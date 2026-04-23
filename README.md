# SealRun ? seal your run

SealRun is a deterministic execution engine that seals every run into a verifiable capsule.

**GitHub description:** SealRun is a deterministic execution engine that seals every run into a verifiable capsule.

**Suggested GitHub topics:** `compliance`, `audit`, `evidence`, `governance`, `reproducibility`, `determinism`

## Problem -> Solution

Automation and AI runs are often hard to prove after the fact. Logs can be incomplete, environments drift silently, and non-deterministic behavior breaks confidence in results.

SealRun captures execution into deterministic capsules, supports replay and diff, detects drift, and emits evidence artifacts that can be reviewed in engineering and compliance workflows.

## Key features

- Deterministic capsules
- Replay and diff
- Drift detection
- Evidence artifacts

## Quickstart

Installation (placeholder): install or build the CLI and expose it as `sealrun` in your PATH.

```bash
# Deterministic execution
sealrun execute ai --model demo --prompt "hello world" --seed 42

# Replay capsule
sealrun execute ai-replay --capsule path/to/capsule\.sealrunai

# Diff / drift analysis
sealrun observe drift left.json right.json

# Governance policy validation
sealrun policy validate --capsule path/to/capsule\.sealrunai --policy examples/governance/dev.policy.json

# Deterministic diagnostics
sealrun doctor
```

Artifacts are written under `sealrun_output/...`.

## Use cases

- **Debugging:** replay failed or flaky runs and compare deterministic diffs.
- **Compliance / audit:** produce machine-readable execution evidence chains.
- **Reproducible automation:** enforce deterministic run behavior in CI/CD and ops pipelines.

## Roadmap

- Harden replay and drift reporting for larger pipelines
- Extend evidence export and anchoring workflows
- Expand policy and governance templates
- Improve deterministic CI and benchmark surfaces
- Publish migration and compatibility guides for broader adoption

## Documentation

- [Architecture](docs/architecture.md)
- [OS Contract Specification](docs/os_contract_spec.md)
- [CLI Reference](docs/cli-reference.md)
- [Developer Guide](docs/developer-guide.md)
- [Operations Guide](docs/operations-guide.md)
- [Security Guide](docs/security-guide.md)
- [Enterprise Guide](docs/enterprise/README.md)

## Compliance & Audit Readiness

- Deterministic evidence surfaces: capsule-bound artefacts and machine-readable CLI JSON envelopes.
- Replay and drift for reproducible comparisons suitable for audit trails and release gates.
- Governance and policy validation for admission control in CI and operations.
- Summary for security and compliance reviews: [Compliance one-pager](docs/compliance/sealrun_compliance_onepager.md).
- Isolation scope and threat assumptions: [Security Guide](docs/security-guide.md).
- Policy contracts and enforcement surfaces: [Governance](docs/governance.md).

Additional references: [Capsules](docs/capsules.md), [Replay](docs/replay.md), [Drift](docs/drift.md).

## Contributing

Issues and pull requests are welcome. Open a focused issue with reproduction details, expected behavior, and environment context before larger changes.