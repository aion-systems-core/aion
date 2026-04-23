# SDK

The **SDK** is a thin, stable Rust API in the `aion-engine` crate (`aion_engine::sdk`) plus matching **`aion sdk`** CLI commands for scripting.

## At a glance

- SDK is the deterministic automation interface for contract-backed workflows.
- CLI and Rust entry points align on stable output structure.
- JSON outputs are intended as machine contracts in CI/CD.

## Rust API (summary)

| Area | Entry points |
|------|----------------|
| Capsule | `load_capsule`, `save_capsule`, `build_capsule` |
| Replay | `replay_capsule`, `compare_capsules` |
| Drift | `drift_between` |
| Explain | `explain_capsule`, `why_diff` |
| Governance | `validate_capsule` |
| CI | `ci_record_baseline`, `ci_check` |
| Output | `write_output_bundle` (deterministic ordering, no timestamps) |

## CLI: `aion sdk`

Examples:

```bash
cargo run -p aion-cli -- sdk capsule build --model m --prompt "hi" --seed 1
cargo run -p aion-cli -- sdk replay --capsule path/to/capsule.aionai
cargo run -p aion-cli -- sdk drift --a a.aionai --b b.aionai
cargo run -p aion-cli -- sdk explain --capsule path/to/capsule.aionai
cargo run -p aion-cli -- sdk info
```

Each command writes `sdk.json`, `sdk.html`, and `sdk.svg` under `aion_output/<stem>/<timestamp>/`.

Batch + output controls:

```bash
cargo run -p aion-cli -- sdk --output-format jsonl --quiet batch --file batch.json
```

Environment knobs:

- `AION_SDK_VERSION` to override reported SDK version string.
- `AION_SDK_OUTPUT_BASE` for `aion_engine::sdk::write_output_bundle` base path.

## Contract surface

- Replay, drift, explainability, governance, and CI helper contracts
- Deterministic output bundle semantics
- Bridge between kernel-layer execution and enterprise-layer automation

## CLI surface

```bash
aion sdk capsule build --model m --prompt "hi" --seed 1
aion sdk replay --capsule path/to/capsule.aionai
aion sdk drift --a a.aionai --b b.aionai
aion sdk ci check --capsule path/to/candidate.aionai --baseline path/to/baseline-governance.json
```

## Rust examples (cargo)

From the repo root:

```bash
cargo run -p aion-cli --example sdk_capsule_build
```

## Related

- [Governance](governance.md)
- [CI](ci.md)

## Enterprise-readiness

SDK readiness requires stable command semantics, deterministic output envelopes, and compatibility-preserving behavior across supported versions.
