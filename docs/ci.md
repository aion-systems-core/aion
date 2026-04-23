# CI

AION‑OS supports **governance CI** workflows: record a **baseline** (capsule + profiles), then **check** new capsules against that baseline with drift, replay, and governance gates.

## At a glance

- CI uses deterministic capsules and governance baselines.
- Checks are machine-readable and suitable for gating decisions.
- Doctor and domain contracts should be included in release pipelines.

## Governance CI (capsules)

### Record baseline

```bash
cargo run -p aion-cli -- ci baseline \
  --capsule path/to/capsule.aionai \
  --policy examples/governance/dev.policy.json \
  --determinism examples/governance/dev.determinism.json \
  --integrity examples/governance/dev.integrity.json
```

Writes `governance.json` (+ HTML/SVG) under `aion_output/ci-baseline/<timestamp>/`. Keep the JSON as your baseline file for checks.

### Check against baseline

```bash
cargo run -p aion-cli -- ci check \
  --capsule path/to/candidate.aionai \
  --baseline path/to/baseline-governance.json
```

Non‑zero exit when checks fail (see CLI help).

## SDK equivalents

```bash
cargo run -p aion-cli -- sdk ci baseline …
cargo run -p aion-cli -- sdk ci check …
```

## Contract surface

- Policy/Governance contracts for gate decisions
- Replay/Drift/Evidence contracts for determinism verification
- Test/Measurement contracts for readiness trend tracking

## CLI surface

```bash
aion ci baseline --capsule path/to/capsule.aionai --policy examples/governance/dev.policy.json --determinism examples/governance/dev.determinism.json --integrity examples/governance/dev.integrity.json
aion ci check --capsule path/to/candidate.aionai --baseline path/to/baseline-governance.json
aion doctor
```

## Related

- [Governance](governance.md)
- [Replay](replay.md)
- [Drift](drift.md)

## Enterprise-readiness

CI is enterprise-ready when deterministic contract outputs are enforced as mandatory release gates with archived evidence.
