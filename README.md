```
    ___    ____ ___  _   _
   / _ \  / ___|_ _|/ \ | |
  | |_| || |    | || | || |
  |  _  || |___ | || |_| |
  |_| |_| \____|___|\___/
```

# AION‑OS

**Deterministic AI Execution OS**

AION‑OS is a deterministic execution stack for AI workloads. It captures each run as a verifiable **capsule**, evaluates replay and drift deterministically, emits evidence chains, and applies policy/governance contracts through machine-readable outputs.

## At a glance

- Product type: Execution-OS with a Contract-OS control plane
- Kernel view: 5 deterministic kernel layers
- Enterprise view: 12 contract phases surfaced through 7 CLI domains
- Canonical audit output: deterministic JSON envelope (`status`, `data`, `error`)

---

AION guarantees deterministic execution, replay symmetry, drift detection and audit‑grade evidence chains.  
AION intentionally does not enforce filesystem or network isolation.  
The kernel isolation modules are contract surfaces only; they define the interface but do not restrict access.

This is a deliberate design choice: AION is an Execution‑OS, not a Security‑Sandbox‑OS.  
Because AION does not modify kernel privileges or intercept syscalls, it is safe to adopt in existing environments without admin rights, without risk to workloads, and without operational friction.

If isolation is required (e.g., for regulated industries), the same contract surfaces can be backed by seccomp/landlock/micro‑VM isolation in a future "AION Secure Runtime" module — without breaking compatibility.

---

## Kernel-layer architecture

AION-OS is structured as five deterministic execution layers:

- **State-Layer** (`Replay-Contract`) — canonical capsule + profile state for replay.
- **Process-Layer** (`Replay-Invariant`) — fixed replay check order with tokenized errors.
- **Map-Layer** (`Drift-Contract`) — deterministic drift labels, categories, and thresholds.
- **Evidence-Layer** (`Evidence-Chain`) — rolling evidence hashes and replay anchors.
- **Policy-Layer** (`Policy-Engine`) — deterministic policy/profile validation and outcomes.

See [Architecture](docs/architecture.md) for the simplified industrial diagram.

## Enterprise-layer architecture

The kernel-layer model is extended by enterprise-layer domains:

- Governance and policy hardening
- Reliability and operations
- Distribution and identity/LTS
- Developer and enterprise UX stability
- Test strategy and compatibility
- Measurement, KPI, audit, and evidence export

See [OS contract spec](docs/os_contract_spec.md) for the full contract catalog.

## Contract surface

- Determinism: replay, drift, evidence, policy, global consistency, identity/finality
- Governance: policy packs, policy gates, policy evidence, governance model
- Operations: reliability, runbooks, incidents, DR, upgrade/migration
- Distribution: channel status, compatibility matrix, LTS policy, installer trust chain
- UX/Test/Measurement: stability contracts, test contracts, metrics/KPI/audit/export

## CLI surface (7 domains)

```bash
cargo run -p aion-cli -- reliability status
cargo run -p aion-cli -- ops runbooks
cargo run -p aion-cli -- dist status
cargo run -p aion-cli -- governance status
cargo run -p aion-cli -- ux api
cargo run -p aion-cli -- tests strategy
cargo run -p aion-cli -- measure metrics
```

## Why deterministic AI?

When the same inputs produce the same outputs, you can **audit**, **diff**, and **ship** with confidence. AION‑OS treats determinism as a first‑class property: capsules, replay reports, and governance checks are designed to be **stable artefacts** for engineering and compliance workflows.

## Key features

| Area | What you get |
|------|----------------|
| **Deterministic capsules** | Serialized AI run records (model, prompt, seed, tokens, evidence, Why, graph). |
| **Replay** | Reconstruct and compare runs to prove fidelity. |
| **Drift** | See what changed between two capsules or baselines. |
| **Why / graph** | Structured explanations and causal DAG views (HTML/SVG). |
| **Governance** | Policy packs, gates, evidence trails, and governance model contracts. |
| **Enterprise contracts** | Distribution, reliability, operations, UX, test strategy, and measurement contracts. |
| **SDK** | Minimal Rust API and CLI `sdk` commands for tools and automation. |

## Quickstart (three commands)

```bash
# 1) Run a deterministic AI capsule (writes artefacts under aion_output/ai/<timestamp>/)
cargo run -p aion-cli -- execute ai --model demo --prompt "hello world" --seed 42

# 2) Replay against the saved capsule (path from previous step: .../capsule.aionai)
cargo run -p aion-cli -- execute ai-replay --capsule path/to/capsule.aionai

# 3) Governance policy validate (use example policy JSON in-repo)
cargo run -p aion-cli -- policy validate --capsule path/to/capsule.aionai --policy examples/governance/dev.policy.json
```

More detail: [Quickstart](docs/quickstart.md) · [Installation](docs/installation.md)

Product ops:

```bash
cargo run -p aion-cli -- setup
cargo run -p aion-cli -- doctor
cargo run -p aion-cli -- stats
cargo run -p aion-cli -- telemetry status
```

## 12-phase model

AION-OS enterprise readiness is organized as a fixed 12-phase contract program (OS meta contracts, contract stability, release/supply chain, security/compliance, production determinism, reliability, operations, distribution/identity, governance, UX, testing, measurement).

## Developer onboarding

- Start: [Installation](docs/installation.md) -> [Quickstart](docs/quickstart.md)
- Build and execution flows: [Developer guide](docs/developer-guide.md)
- Contract and command map: [CLI reference](docs/cli-reference.md)
- Pilot and adoption path: [Guided tour](docs/guided_tour.md)

## Enterprise-readiness and sales

- External story and buyer mapping: [Enterprise sales package](docs/enterprise/AION_Enterprise_Sales_Package.md)
- Presentation edition: [Enterprise sales HTML](docs/enterprise/AION_Enterprise_Sales_Package.html)

## Example: capsule + replay

```bash
cargo run -p aion-cli -- execute ai --model m --prompt "one two" --seed 7
# Note printed output path; use its capsule.aionai with:
cargo run -p aion-cli -- execute ai-replay --capsule aion_output/ai/<timestamp>/capsule.aionai
```

## Example: governance check

```bash
cargo run -p aion-cli -- policy validate \
  --capsule path/to/capsule.aionai \
  --policy examples/governance/dev.policy.json
```

CI baseline/check (governance v1) is documented in [Governance](docs/governance.md) and [CI](docs/ci.md).

## Documentation

| Doc | Topic |
|-----|--------|
| [Overview](docs/overview.md) | Product map and concepts |
| [Architecture](docs/architecture.md) | Core execution OS layers and deterministic contracts |
| [OS contract spec](docs/os_contract_spec.md) | Formal contract specification for all enterprise layers |
| [CLI reference](docs/cli-reference.md) | Deterministic command surface and examples |
| [Developer guide](docs/developer-guide.md) | Deterministic developer onboarding and flows |
| [Operations guide](docs/operations-guide.md) | SRE and operations contract workflows |
| [Security guide](docs/security-guide.md) | Security, trust chain, policy evidence flows |
| [Installation](docs/installation.md) | Build from source, prerequisites |
| [Quickstart](docs/quickstart.md) | First commands and artefacts |
| [Capsules](docs/capsules.md) | AI capsule format (conceptual) |
| [Replay](docs/replay.md) | Replay and comparison |
| [Drift](docs/drift.md) | Drift between runs or capsules |
| [Why & graph](docs/why-graph.md) | Explainability outputs |
| [Governance](docs/governance.md) | Policy, determinism, integrity |
| [SDK](docs/sdk.md) | Programmatic API and `aion sdk` |
| [CI](docs/ci.md) | Baselines and checks |
| [FAQ](docs/faq.md) | Common operational questions |
| [Compatibility matrix](docs/compatibility-matrix.md) | Version/scheme guarantees |
| [Migration](docs/migration.md) | Upgrade workflow |
| [Training](docs/training.md) | Onboarding path and labs |
| [Installers](docs/installers.md) | Cargo/Homebrew/APT/RPM and Docker |
| [Telemetry](docs/telemetry.md) | Opt-in telemetry controls |
| [Case studies](docs/case-studies.md) | Applied usage patterns |
| [Whitepaper](docs/whitepaper.md) | Architecture/design draft |
| [Community](docs/community.md) | Support channels |
| [Benchmarks](docs/benchmarks.md) | Benchmark workflow |
| [Enterprise license](docs/enterprise-license.md) | Commercial licensing note |
| [Enterprise sales package](docs/enterprise/AION_Enterprise_Sales_Package.md) | Enterprise positioning, readiness, and rollout model |
| [Videos](docs/videos.md) | Video roadmap |
| [Feedback survey](docs/feedback-survey.md) | Structured feedback prompts |

## Version

The repo root `VERSION` file is the product version string surfaced by `aion --version`.

See [CHANGELOG.md](CHANGELOG.md) for release notes.

## License

MIT (see repository files).
