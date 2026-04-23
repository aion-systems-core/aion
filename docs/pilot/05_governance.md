# Pilot onboarding — Governance

Validate a capsule against a **policy JSON** (and optional determinism / integrity profiles on SDK paths).

```bash
sealrun policy validate \
  --capsule sealrun_output/ai/pilot_demo/capsule\.sealrunai \
  --policy examples/governance/dev.policy.json
```

## Built-in presets

```bash
sealrun policy list
sealrun policy show dev
```

## What to look for

- **`governance.json`** — Consolidated policy / determinism / integrity outcome.
- Clear **pass/fail** for pilot gatekeeping.

## Next

- [06 — Evidence chain](06_evidence_chain.md)  
- [Governance reference](../governance.md)
