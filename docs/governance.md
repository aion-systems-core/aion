# Governance

This guide describes AION-OS governance as deterministic policy and evidence contracts.

## At a glance

- Governance is contract-driven (`policy packs`, `policy gates`, `policy evidence`, `governance status`).
- Decisions are deterministic and machine-readable.
- JSON envelopes are the canonical audit surface.

## Contract surface

- **Policy Pack Contract**: signed/versioned policy sets
- **Policy Gate Contract**: mandatory decisions for CI/CD/runtime contexts
- **Policy Evidence Contract**: deterministic decision + evidence chain outputs
- **Governance Model**: aggregated governance status across domains

## CLI surface

### Policy contracts

```bash
cargo run -p aion-cli -- policy packs
cargo run -p aion-cli -- policy gates
cargo run -p aion-cli -- policy evidence
```

### Governance aggregate

```bash
cargo run -p aion-cli -- governance status
```

### Capsule policy validation

```bash
cargo run -p aion-cli -- policy validate \
  --capsule path/to/capsule.aionai \
  --policy examples/governance/dev.policy.json
```

## Enterprise-readiness

- Governance readiness requires explicit policy decisions with no silent bypass paths.
- Use `aion doctor` plus governance commands as release admission evidence.

## Related

- [CLI reference](cli-reference.md)
- [SDK](sdk.md)
- [OS contract spec](os_contract_spec.md)
