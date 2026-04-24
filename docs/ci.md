# CI

## Purpose

Describe **`sealrun ci baseline`** / **`sealrun ci check`** as machine-readable **governance** gates over capsules, with exit semantics suitable for pipelines ([Governance](governance.md)).

SealRun supports **governance CI** workflows: record a **baseline** (capsule + profiles), then **check** new capsules against that baseline with drift, replay, and governance gates.

## At a glance

- CI uses deterministic capsules and governance baselines.
- Checks are machine-readable and suitable for gating decisions.
- Doctor and domain contracts should be included in release pipelines.

## Governance CI (capsules)

### Record baseline

```bash
sealrun ci baseline \
  --capsule path/to/capsule.aionai \
  --policy examples/governance/dev.policy.json \
  --determinism examples/governance/dev.determinism.json \
  --integrity examples/governance/dev.integrity.json
```

Writes `governance.json` (+ HTML/SVG) under `sealrun_output/ci-baseline/<timestamp>/`. Keep the JSON as your baseline file for checks.

### Check against baseline

```bash
sealrun ci check \
  --capsule path/to/candidate.aionai \
  --baseline path/to/baseline-governance.json
```

Non‑zero exit when checks fail (see CLI help).

## SDK equivalents

```bash
sealrun sdk ci baseline …
sealrun sdk ci check …
```

## Contract surface

- Policy/Governance contracts for gate decisions
- Replay/Drift/Evidence contracts for determinism verification
- Test/Measurement contracts for readiness trend tracking

## CLI surface

```bash
sealrun ci baseline --capsule path/to/capsule.aionai --policy examples/governance/dev.policy.json --determinism examples/governance/dev.determinism.json --integrity examples/governance/dev.integrity.json
sealrun ci check --capsule path/to/candidate.aionai --baseline path/to/baseline-governance.json
sealrun doctor
```

## Related

- [Governance](governance.md)
- [Replay](replay.md)
- [Drift](drift.md)

## Enterprise-readiness

CI is enterprise-ready when deterministic contract outputs are enforced as mandatory release gates with archived evidence.
