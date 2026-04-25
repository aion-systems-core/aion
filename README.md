# SealRun

SealRun is a **deterministic execution engine** for AI and automation workloads: every run is sealed into a **replayable capsule**, compared with **drift** contracts, anchored to an **evidence chain**, and checked against **governance** policies, surfaced as **deterministic JSON envelopes** for machines and auditors.

[![CI](https://github.com/aion-systems-core/sealrun/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/aion-systems-core/sealrun/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org/)

## The problem

Production and regulated teams need to **prove what ran**, **prove it again**, and **prove what changed**. Conventional logs and ad-hoc screenshots are weak evidence: environments drift, models are non-obvious black boxes, and "it worked on my machine" is not a control.

## The solution

SealRun records runs as **structured capsules**, verifies them with **replay symmetry**, measures change with **drift detection**, and binds integrity with a linear **evidence chain**. **Policy validation** and CLI **governance** surfaces make those artefacts usable in **CI gates** and **audit** workflows. The implementation is **Rust-first**: explicit contracts, stable serialization, and reproducible tooling behavior.

## Why SealRun is different

| Typical stack | SealRun |
|---------------|---------|
| Logs as proof | **Capsule + evidence** as the primary audit unit |
| "Re-run and hope" | **Replay symmetry** against the stored record |
| Informal diff | **Drift contract** with deterministic categories |
| Policy in prose | **Machine-readable governance** outputs |
| Implicit isolation | **Execution-OS (contract layer), not a sandbox OS** (see below) |

**Execution-OS vs. security sandbox OS:** In architecture terms, SealRun is an **execution OS**: a deterministic **contract surface** for runs (State, Process, Map, Evidence, Policy layers; see [Architecture](docs/architecture.md)). It is **not** a **sandbox OS**: no root requirement, no kernel modules, no syscall interception for isolation. Filesystem and network isolation remain **operator or enterprise** responsibilities; the same contracts can be backed by stronger isolation where required ([Security Guide](docs/security-guide.md)).

## Architecture overview

SealRun separates the **five-layer deterministic kernel model** (State, Process, Map, Evidence, Policy) from **enterprise contract domains** exposed via the CLI (`governance`, `reliability`, `ops`, `dist`, `ux`, `tests`, `measure`) and aggregated in `sealrun doctor`.

```text
  Prompt + model + seed + profiles
              |
              v
       +------------------+
       |     Capsule      |  canonical run record (tokens, why, graph, evidence)
       +--------+---------+
                |
    +-----------+-----------+---------------+
    |           |           |               |
  Replay       Drift     Evidence       Policy
 (symmetry)  (compare)   (chain)     (validation)
```

| Layer | Role |
|-------|------|
| **State** | Canonical capsule state and replay inputs |
| **Process** | Replay-invariant checks and symmetry |
| **Map** | Drift categories, ordering, tolerances |
| **Evidence** | Digest-linked chain and anchors |
| **Policy** | Governance packs, gates, validation order |

Authoritative definitions: **[OS Contract Specification](docs/os_contract_spec.md)** | [Architecture](docs/architecture.md)

## Key features

- **Deterministic capsules** - versioned run records for archival and comparison
- **Replay symmetry** - re-execution checks against the capsule under the replay invariant
- **Drift detection** - pairwise, field-classified deltas for CI and review
- **Evidence chain** - linear, digest-linked proof surface tied to capsules
- **Governance policy packs** - deterministic pass/fail artefacts for gates
- **Deterministic JSON envelopes** - stable machine contracts across CLI domains
- **Rust workspace** - `core`, `kernel`, `engine`, and `sealrun` CLI crates

## Quickstart

```bash
git clone https://github.com/aion-systems-core/sealrun.git
cd sealrun
cargo build --release
./target/release/sealrun --help
```

On Windows, run `target\release\sealrun.exe` (or use Git Bash for the paths above).

Pin output under **`sealrun_output/`** and fixed run IDs so paths stay stable:

```bash
export SEALRUN_OUTPUT_BASE="$PWD/sealrun_output"   # PowerShell: $env:SEALRUN_OUTPUT_BASE = "$PWD\sealrun_output"
./target/release/sealrun --id quickstart_demo execute ai \
  --model demo --prompt "hello world" --seed 42

./target/release/sealrun --id quickstart_replay execute ai-replay \
  --capsule sealrun_output/ai/quickstart_demo/capsule.aionai

./target/release/sealrun --id quickstart_drift_a observe capture -- echo alpha
./target/release/sealrun --id quickstart_drift_b observe capture -- echo beta
./target/release/sealrun observe drift \
  sealrun_output/capture/quickstart_drift_a/result.json \
  sealrun_output/capture/quickstart_drift_b/result.json

./target/release/sealrun policy validate \
  --capsule sealrun_output/ai/quickstart_demo/capsule.aionai \
  --policy examples/governance/dev.policy.json

./target/release/sealrun doctor
```

**Artefact layout:** `<output_base>/<command>/<run_id>/`. Example: `sealrun_output/ai/quickstart_demo/` holds `ai.json`, **`capsule.aionai`** (AI capsule JSON), **`*.aionevidence`** sidecars, and optional HTML/SVG. Shell capture flows emit **`result.json`** (`RunResult`). Override the base with **`SEALRUN_OUTPUT_BASE`** or **`--output-dir`** (see `engine/src/output/layout.rs` for compatibility with older env names).

## E2E smoke proof

Run one deterministic smoke script with three scenarios (execute/replay/policy, capture/drift, evidence/governance/doctor):

```bash
bash scripts/smoke_e2e.sh
```

The script prints only `PASS` or `FAIL` and writes a single `smoke_report.json` for CI artifacts.

## Golden determinism proof

```bash
cargo test -p aion-cli --test golden_test
bash scripts/smoke_e2e.sh
```

## Use cases

- **Regulated / audit-ready AI** - retain capsules, replay reports, drift JSON, and policy decisions as primary evidence
- **Release and change control** - gate merges on replay, drift against baselines, and governance CLI outputs
- **SRE and platform operations** - fold `doctor` and domain JSON into incident and upgrade reviews ([Operations Guide](docs/operations-guide.md))
- **Vendor and internal evaluation** - reproduce runs from capsules without proprietary hosting
- **Open-source supply chain** - inspect contracts and behaviour in Rust instead of opaque services

## Documentation hub

| Document | Description |
|----------|-------------|
| [Architecture](docs/architecture.md) | Five-layer kernel vs. enterprise domains, diagrams, contract mapping |
| [OS Contract Spec](docs/os_contract_spec.md) | Normative contract text the implementation tracks |
| [CLI Reference](docs/cli-reference.md) | Deterministic CLI surface by domain |
| [Developer Guide](docs/developer-guide.md) | Replay, drift, evidence, identity workflows |
| [Operations Guide](docs/operations-guide.md) | SRE-oriented use of contract outputs |
| [Security Guide](docs/security-guide.md) | Threat model, isolation scope, adoption boundaries |
| [Enterprise Guide](docs/enterprise/README.md) | Enterprise packaging and commercial context |
| [Compliance One-Pager](docs/compliance/sealrun_compliance_onepager.md) | Short compliance-facing summary |
| [Capsules](docs/capsules.md) | Capsule format and on-disk layout |
| [Replay](docs/replay.md) | Replay guarantees and CLI/SDK usage |
| [Drift](docs/drift.md) | Drift semantics and gating |
| [Governance](docs/governance.md) | Policy packs, gates, open-core vs. enterprise |
| [SDK](docs/sdk.md) | Programmatic integration and `sealrun sdk` |
| [FAQ](docs/faq.md) | Developers, operators, and security roles |

## Compliance & enterprise readiness

- **Evidence-first:** deterministic JSON envelopes and capsule-bound artefacts support retention and review
- **Controls, not slogans:** replay, drift, evidence, and governance outputs attach to tickets, CMDBs, and SOC workflows
- **Entry points:** [Compliance One-Pager](docs/compliance/sealrun_compliance_onepager.md) | [Security Guide](docs/security-guide.md) | [Governance](docs/governance.md) | [Enterprise Guide](docs/enterprise/README.md)

## Roadmap

- Deeper replay and drift reporting for large batch fleets
- Stronger evidence export and anchoring options
- Broader governance templates and CI recipes
- Hardened benchmarks and compatibility windows across releases

## Contributing

Issues and pull requests are welcome. Open an issue with **reproduction**, **expected vs. actual behaviour**, **Rust toolchain version**, and **OS** before large changes. Follow `cargo fmt`, `cargo clippy -D warnings`, and `cargo test --workspace` locally. See [.github/workflows/ci.yml](.github/workflows/ci.yml) for the canonical CI matrix. Community expectations: [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md). Security reports: [SECURITY.md](SECURITY.md) (do not use public issues for vulnerabilities).

## License

SealRun is open source under the [MIT License](LICENSE).
