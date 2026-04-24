# Replay

## Purpose

Define **replay symmetry**: what the replay report guarantees, how it maps to the **Process** layer, and how CLI/SDK invoke it without changing the capsule binary format.

**Replay** re-executes a workload from a **capsule** and produces a structured **replay report** that states whether the re-run matches the recorded run under the contract’s replay invariant.

## At a glance

- Kernel-layer **determinism control**: pass/fail is machine-checkable.
- Outputs are deterministic **JSON** (with optional HTML/SVG projections).
- Used for regression, release gates, and audit evidence alongside drift and governance.

Replay verifies **contractual symmetry**, not environmental security; combine with your own isolation controls where required ([Security guide](security-guide.md)).

## Guarantees (contract-level)

1. **Same inputs envelope:** replay uses the capsule’s stored determinism inputs (model, prompt, seed, profiles as applicable).
2. **Comparable outputs:** the report states match or identifies a **first differing token** (or equivalent locus) for investigation.
3. **Stable machine contract:** replay JSON is suitable for CI gating; non-zero exit semantics follow CLI help for the command in use.
4. **Version awareness:** reports include **implementation version metadata** so auditors can detect tool/capsule skew (exact field names depend on engine version; treat as opaque identifiers in automation unless pinned).

## CLI: AI replay

```bash
sealrun execute ai-replay --capsule path/to/capsule.aionai
```

Typical output path:

```text
sealrun_output/ai-replay/<run-id>/ai.json
```

## SDK / automation

```bash
sealrun sdk replay --capsule path/to/capsule.aionai
```

Writes `sdk.json` (and projections) under `sealrun_output/sdk-replay/<run-id>/`.

## Contract surface

- **Replay invariant** (process contract): defines what “match” means for the workload class.
- **Global consistency:** replay outcome feeds run-level finality where defined in [OS contract spec](os_contract_spec.md).

## CLI surface

```bash
sealrun execute ai-replay --capsule path/to/capsule.aionai
sealrun sdk replay --capsule path/to/capsule.aionai
sealrun doctor
```

## Related

- [Capsules](capsules.md)
- [Drift](drift.md)
- [SDK](sdk.md)

## Enterprise-readiness

Require **pinned tool versions** and archived **replay JSON** for each production-impacting model or prompt template change. Pair with drift baselines for multi-capsule fleets ([Drift](drift.md), [CI](ci.md)).
