# Drift

## Purpose

Define **drift detection** on the **Map** layer: stable categories, tolerances, CLI/SDK entry points, and how drift output feeds governance and CI gates.

**Drift** quantifies **deterministic differences** between two runs or two capsules: which fields diverged, under which categories, and whether the delta is within configured tolerance for your gate.

## At a glance

- Map-layer **comparison contract**: stable labels and categories for automation.
- Used for **CI admission**, **A/B** analysis of prompts/models, and **post-incident** forensics when paired with capsules.
- Complements **replay** (symmetry of one capsule) with **pairwise** comparison.

Drift does not assert **causal** root cause of non-determinism in external dependencies; it reports **observed** structured differences on captured artefacts.

## Comparison targets

| Mode | Typical use |
|------|-------------|
| **Run JSON pair** | `sealrun observe drift left.json right.json` after capture/observe flows. |
| **Capsule pair** | `sealrun sdk drift --a a.aionai --b b.aionai` for sealed-record comparison. |

Outputs land under `sealrun_output/drift/<run-id>/` (observe) or SDK output trees for `sdk drift`.

## Semantics

- **Field-level classification:** tokens, seed, evidence digests, embedded Why/graph projections, etc., per report schema.
- **Exit codes:** use CLI help for the command; a non-zero exit on detected drift is intended for **CI gating** where documented.

## Example drift JSON (illustrative shape)

```json
{
  "changed": true,
  "fields": ["tokens", "seed"],
  "details": ["…"]
}
```

Exact keys are versioned with the tool; pin versions for stable CI parsers.

## Contract surface

- **Drift contract:** categories, tolerances, and deterministic ordering of findings.
- **Governance / measurement:** drift feeds gates and trend reporting ([Governance](governance.md)).

## CLI surface

```bash
sealrun observe drift left.json right.json
sealrun sdk drift --a first.aionai --b second.aionai
sealrun doctor
```

## Related

- [Replay](replay.md)
- [Capsules](capsules.md)
- [CI](ci.md)

## Enterprise-readiness

Define **allowed drift classes** per environment (e.g., documentation-only HTML deltas vs token deltas). Archive drift JSON with the **pair of capsule hashes** and tool version for audit reconstruction.
