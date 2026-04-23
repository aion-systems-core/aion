# Pilot onboarding — Install

**Goal:** Build the CLI and run your first command in minutes.

## Prerequisites

- Rust toolchain (`cargo`, stable recommended).
- Clone this repository.

## Build

```bash
cargo build -p aion-cli
```

Invoke via `sealrun <subcommand>` or add `target/debug` to your `PATH`.

## Sanity checks

```bash
sealrun doctor
sealrun setup
```

## Next

- [01 — Execute capsule](01_execute_capsule.md)

**Guided Link: Pilot Onboarding** — start of sequence; see also [Guided tour](../guided_tour.md).
