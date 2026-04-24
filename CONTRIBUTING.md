# Contributing to SealRun

Thank you for helping improve the deterministic execution engine and tooling.

## Build

From the repository root (workspace):

```bash
cargo build -p aion-cli
```

Release binary:

```bash
cargo build -p aion-cli --release
```

## Run tests

```bash
cargo test -p aion-engine
cargo test -p aion-cli
cargo test -p aion-core
cargo test -p aion-kernel
```

### Product / docs checks (optional)

```bash
# Link sanity for docs (Rust integration test `product_docs`)
cargo test -p aion-cli --test product_docs

# Smoke script aligned with README (`scripts/test_readme_examples.sh`; Git Bash / WSL / Linux)
bash scripts/test_readme_examples.sh

# CI-only: compile SDK examples + run Rust examples + basic shell example
export SEALRUN_PRODUCT_TESTS=1
bash scripts/test_examples_run.sh
cargo test -p aion-cli --test product_examples
```

## Adding new modules

1. Prefer small, focused crates under `core`, `kernel`, `engine`, or `crates/`.
2. Export a clear public API from the owning crate’s `lib.rs`.
3. If the CLI should expose behaviour, add a subcommand in `crates/aion-cli` and wire to engine/kernel through existing gateways—avoid duplicating business logic in the CLI.
4. Add or extend tests next to the code you change (`tests/` or inline `#[cfg(test)]`).

## Code style

- Rust edition **2021**, `rustfmt` defaults.
- Prefer explicit error types or `Result` over panics in libraries.
- Keep public types stable when possible; document breaking changes in `CHANGELOG.md`.

## Commit conventions

- Use imperative mood: `Add drift summary to CLI output`.
- Reference issues when applicable: `Fix replay flag (#123)`.
- Separate unrelated changes into multiple commits.

## Release process (checklist)

1. Update `VERSION` at repo root (semantic versioning).
2. Update `CHANGELOG.md` with a dated section for the release.
3. Run full `cargo test` across workspace members.
4. Run product checks if you maintain docs/examples (`SEALRUN_PRODUCT_TESTS=1`; the CI gate script also accepts the pre-rename product-test flag — see `scripts/test_examples_run.sh` and `crates/aion-cli/tests/product_examples.rs`).
5. Tag the repository: `git tag v$(cat VERSION)` and push tags.
6. Publish or attach release artefacts per your distribution policy.

## Documentation changes

- Keep `docs/` product‑level: no engine internal module paths unless necessary for operators.
- When adding links between docs, run `cargo test -p aion-cli --test product_docs`.
