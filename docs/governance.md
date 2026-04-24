# Governance

## Purpose

Clarify **governance policy packs**, **gates**, and **policy evidence** outputs on the **Policy** layer, plus open-core vs. enterprise packaging ([Enterprise Guide](enterprise/README.md)).

SealRun **governance** is the set of **deterministic, machine-readable** policy and evidence contracts applied to capsules and CI baselines.

## At a glance

- **Policy packs**, **gates**, and **evidence** surfaces expose decisions without ambiguous side channels.
- **`sealrun governance status`** aggregates cross-domain governance readiness.
- **`policy validate`** binds capsules to concrete policy, determinism, and integrity profiles.

## Open-core vs enterprise

| Capability | Open-core (this repository) | Enterprise (commercial add-ons) |
|------------|----------------------------|---------------------------------|
| **Policy validation** | Deterministic `policy validate`, CI baseline/check, governance JSON from CLI. | Extended packs, organisational templates, and support workflows as offered by your vendor agreement. |
| **Isolation** | Not enforced by the deterministic engine; workload isolation is the operator’s responsibility. | Optional **secure runtime**-class offerings may back the same contracts with stronger isolation (see [Security guide](security-guide.md)). |
| **Evidence export** | File-based JSON/HTML/SVG bundles suitable for archival. | Extended export, dashboards, and SLA-backed pipelines where provided. |

Technical contract definitions: [OS contract spec](os_contract_spec.md). Sales and packaging context: [Enterprise README](enterprise/README.md).

## Contract surface

- **Policy pack:** signed/versioned policy sets.
- **Policy gate:** mandatory pass/fail rules for a context (e.g., CI).
- **Policy evidence:** deterministic decision records suitable for audit appendices.
- **Governance model:** aggregated status across domains (surfaced via CLI).

## CLI surface

```bash
sealrun policy packs
sealrun policy gates
sealrun policy evidence
sealrun governance status
sealrun policy validate \
  --capsule path/to/capsule.aionai \
  --policy examples/governance/dev.policy.json
```

## Enterprise-readiness

- No **silent bypass**: failed validation and non-zero exits must be observable in CI logs and retained JSON.
- Pair policy checks with **replay** and **drift** gates for defence in depth ([Replay](replay.md), [Drift](drift.md), [CI](ci.md)).

## Related

- [CLI reference](cli-reference.md)
- [SDK](sdk.md)
- [Architecture](architecture.md)
