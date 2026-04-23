# SDK

The **SDK** is the programmatic surface for the same contracts exposed by the `sealrun` CLI: build/load capsules, replay, drift, explain, validate, and CI baseline/check — with **deterministic output bundles** for automation.

## At a glance

- **Rust API** lives in the internal engine crate (see workspace `Cargo.toml`); stable entry points mirror CLI semantics.
- **`sealrun sdk`** provides a **scriptable**, JSON-first path without linking Rust.
- Outputs use **canonical ordering** rules suitable for content-addressed storage and CI caches.

Authoritative behaviour and invariants: [OS contract spec](os_contract_spec.md).

## Integration points

| Layer | How to integrate |
|-------|------------------|
| **Shell / CI** | Invoke `sealrun sdk …`; parse `sdk.json` and command-specific JSON under `sealrun_output/`. |
| **Rust services** | Call SDK functions for capsule/replay/drift/governance; use `write_output_bundle` for deterministic artefact trees. |
| **Cross-language** | Prefer subprocess + JSON contracts; optional native bindings are described in [Compatibility layer](compatibility-layer.md) where present. |

## Rust API (summary)

| Area | Entry points |
|------|----------------|
| Capsule | `load_capsule`, `save_capsule`, `build_capsule` |
| Replay | `replay_capsule`, `compare_capsules` |
| Drift | `drift_between` |
| Explain | `explain_capsule`, `why_diff` |
| Governance | `validate_capsule` |
| CI | `ci_record_baseline`, `ci_check` |
| Output | `write_output_bundle` (deterministic ordering) |

## CLI: `sealrun sdk`

```bash
sealrun sdk capsule build --model m --prompt "hi" --seed 1
sealrun sdk replay --capsule path/to/capsule.sealrunai
sealrun sdk drift --a a.sealrunai --b b.sealrunai
sealrun sdk explain --capsule path/to/capsule.sealrunai
sealrun sdk info
```

Each command writes `sdk.json`, `sdk.html`, and `sdk.svg` under `sealrun_output/<stem>/<run-id>/`.

Batch:

```bash
sealrun sdk --output-format jsonl --quiet batch --file batch.json
```

## Environment variables (documentation)

- `SEALRUN_SDK_VERSION` — override reported SDK version string in outputs.
- `SEALRUN_SDK_OUTPUT_BASE` — base path for SDK bundle writes (aligns with organisational artefact layout policies).

## Contract surface

- Parity with kernel and enterprise contracts: replay, drift, explainability, governance, CI helpers.
- **Deterministic envelopes** are the stable integration boundary; HTML/SVG are optional human annexes.

## Rust examples (cargo)

```bash
cargo run -p aion-cli --example sdk_capsule_build
```

## Related

- [Governance](governance.md)
- [CI](ci.md)
- [Developer guide](developer-guide.md)

## Enterprise-readiness

Pin **SDK/engine versions**, validate **output schema** in CI, and store bundles with **capsule hash + git commit + policy version** for traceability.
