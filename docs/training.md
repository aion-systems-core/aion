# Training materials

## Purpose

Curriculum-style onboarding aligned with [Developer Guide](developer-guide.md) and [Guided tour](guided_tour.md).

This guide defines a deterministic onboarding and enablement path for SealRun teams.

## At a glance

- Fast onboarding path from overview to SDK automation
- Labs mapped to replay, drift, governance, and contracts
- Suitable for developer, ops, and governance onboarding

## Suggested onboarding path

1. [Overview](overview.md)
2. [Quickstart](quickstart.md)
3. [Capsules](capsules.md)
4. [Replay](replay.md)
5. [Governance](governance.md)
6. [SDK](sdk.md)

## Hands-on labs

- Lab 1: deterministic AI run + replay
- Lab 2: drift between seeds
- Lab 3: governance baseline/check
- Lab 4: SDK batch execution

## CLI surface

```bash
sealrun execute ai --model demo --prompt "training" --seed 1
sealrun execute ai-replay --capsule path/to/capsule.aionai
sealrun sdk drift --a first.aionai --b second.aionai
sealrun governance status
```

## Enterprise-readiness

Training is enterprise-ready when participants can execute deterministic workflows and interpret contract outputs without ambiguity.
