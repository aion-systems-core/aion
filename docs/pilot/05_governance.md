# Pilot onboarding — Governance

Validate a capsule against a **policy JSON** (and optional determinism / integrity profiles on SDK paths).

```bash
cargo run -p aion-cli -- policy validate \
  --capsule aion_output/ai/pilot_demo/capsule.aionai \
  --policy examples/governance/dev.policy.json
```

## Built-in presets

```bash
cargo run -p aion-cli -- policy list
cargo run -p aion-cli -- policy show dev
```

## What to look for

- **`governance.json`** — Consolidated policy / determinism / integrity outcome.
- Clear **pass/fail** for pilot gatekeeping.

## Next

- [06 — Evidence chain](06_evidence_chain.md)  
- [Governance reference](../governance.md)
