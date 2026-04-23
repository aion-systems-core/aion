# Installation

AION‑OS is built from source as a Rust workspace. There is no separate installer in this repository slice.

## At a glance

- Build from source using stable Rust toolchain.
- Validate installation with deterministic CLI commands.
- Continue with quickstart and contract-domain checks.

## Prerequisites

- **Rust toolchain** (stable), including `cargo` and `rustfmt`.
- A shell capable of running the CLI (Windows: PowerShell, or Git Bash for bundled shell examples).

## Clone and build

```bash
git clone <your-fork-or-upstream-url> aion-os
cd aion-os
cargo build -p aion-cli
```

The `aion` binary is produced under `target/debug/aion` (or `target/release/aion` with `--release`).

## Verify

```bash
cargo run -p aion-cli -- --version
```

## CLI surface

```bash
cargo run -p aion-cli -- --version
cargo run -p aion-cli -- doctor
cargo run -p aion-cli -- execute ai --model demo --prompt "install check" --seed 1
```

## Output location

- By default, artefacts go under `<current working directory>/aion_output/<command>/<timestamp>/`.
- Set **`AION_OUTPUT_BASE`** to an absolute or relative directory to redirect the `<command>/<timestamp>/` tree under that base instead of `cwd/aion_output`.
- Each run folder includes **`meta.json`** (`aion_version`, optional `AION_GIT_COMMIT`, `command`, timestamps, and reserved optional policy fields).

## Next steps

- [Quickstart](quickstart.md)
- [Governance](governance.md) if you plan policy checks in CI

## Enterprise-readiness

Installation readiness means reproducible builds plus successful deterministic doctor output on target environments.
