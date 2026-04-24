# Installation

## Purpose

From **Rust toolchain** to a verified **`sealrun` binary**, artefact base directory (`SEALRUN_OUTPUT_BASE` or `--output-dir`; older env names are still read by the engine — see `engine/src/output/layout.rs`), and smoke commands—so later docs ([Quickstart](quickstart.md)) match your environment.

SealRun is built from source as a Rust workspace. There is no separate installer in this repository slice.

## At a glance

- Build from source using stable Rust toolchain.
- Validate installation with deterministic CLI commands.
- Continue with quickstart and contract-domain checks.

## Prerequisites

- **Rust toolchain** (stable), including `cargo` and `rustfmt`.
- A shell capable of running the CLI (Windows: PowerShell, or Git Bash for bundled shell examples).

## Clone and build

```bash
git clone <your-fork-or-upstream-url> SealRun
cd SealRun
cargo build -p aion-cli
```

The `sealrun` binary is produced under `target/debug/sealrun` (or `target/release/sealrun` with `--release`).

## Verify

```bash
sealrun --version
```

## CLI surface

```bash
sealrun --version
sealrun doctor
sealrun execute ai --model demo --prompt "install check" --seed 1
```

## Output location

- By default, artefacts go under `<current working directory>/sealrun_output/<command>/<timestamp>/`.
- Set **`SEALRUN_OUTPUT_BASE`** to an absolute or relative directory to redirect the `<command>/<timestamp>/` tree under that base instead of `cwd/sealrun_output`.
- Each run folder includes **`meta.json`** (`sealrun_version`, optional `SEALRUN_GIT_COMMIT`, `command`, timestamps, and reserved optional policy fields).

## Next steps

- [Quickstart](quickstart.md)
- [Governance](governance.md) if you plan policy checks in CI

## Enterprise-readiness

Installation readiness means reproducible builds plus successful deterministic doctor output on target environments.
