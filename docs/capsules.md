# Capsules

## Purpose

Define the **deterministic capsule** as the unit of audit: what it contains on disk, how **replay symmetry** and **drift detection** consume it, and how **governance policy packs** validate it.

A **capsule** is the durable, versioned record of a deterministic AI run. It is the integration point for **replay**, **drift**, **evidence**, and **governance**.

## At a glance

- Canonical kernel-layer artefact for a sealed run.
- Serializable, diffable, and policy-validatable.
- On-disk companion files (HTML/SVG/JSON) are **projections** of the same logical record for humans and tooling.

For security and isolation scope, see [Security guide](security-guide.md) (SealRun open-core does not substitute for workload sandboxing).

## Logical contents

A capsule aggregates, at minimum:

| Concern | Purpose |
|---------|---------|
| **Identity** | Stable run identity, schema/capsule version, model and prompt (or hash references where used). |
| **Determinism envelope** | Seed, determinism profile inputs, and parameters needed to re-execute or compare. |
| **Emitted sequence** | Token stream (or equivalent) as the primary replay comparison surface. |
| **Evidence** | Digests and chain steps binding the run to integrity checks. |
| **Explainability** | Why report and causal graph projections for audit and regression review. |

Exact field names and required keys are defined in [AI capsule schema](ai-capsule-schema.json) and [Example capsule JSON](example-capsule.json).

## On-disk layout

Typical `sealrun execute ai` output directory:

- **`capsule.aionai`** — AI capsule JSON used by replay and SDK (current on-disk extension).
- `ai.json` / `ai.html` / `ai.svg` — run summary projections.
- `why.html` / `why.svg` — explainability projections.
- Evidence sidecar files as emitted by the engine (names may vary by command; treat directory as one **evidence bundle**).

Paths are rooted under `sealrun_output/<command>/<run-id>/` unless overridden by output base configuration (see [Installation](installation.md)).

## Contract surface

- **State / process:** capsule as input to replay-invariant checks.
- **Evidence:** capsule-bound chain verification inputs.
- **Governance:** capsule as input to `policy validate` and CI baseline/check.

## CLI surface

```bash
sealrun execute ai --model M --prompt "your text" --seed 42
sealrun execute ai-replay --capsule path/to/capsule.aionai
sealrun policy validate --capsule path/to/capsule.aionai --policy examples/governance/dev.policy.json
```

## Related

- [Replay](replay.md)
- [Drift](drift.md)
- [SDK](sdk.md)
- [OS contract spec](os_contract_spec.md)

## Enterprise-readiness

Treat capsule **schema version**, replay symmetry, and evidence linkage as **release invariants**: any change requires explicit compatibility notes and migration guidance ([Migration](migration.md), [Compatibility matrix](compatibility-matrix.md)).
