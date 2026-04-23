---

AION guarantees deterministic execution, replay symmetry, drift detection and audit‑grade evidence chains.  
AION intentionally does not enforce filesystem or network isolation.  
The kernel isolation modules are contract surfaces only; they define the interface but do not restrict access.

This is a deliberate design choice: AION is an Execution‑OS, not a Security‑Sandbox‑OS.  
Because AION does not modify kernel privileges or intercept syscalls, it is safe to adopt in existing environments without admin rights, without risk to workloads, and without operational friction.

If isolation is required (e.g., for regulated industries), the same contract surfaces can be backed by seccomp/landlock/micro‑VM isolation in a future "AION Secure Runtime" module — without breaking compatibility.

---

# AION – Deterministic Execution OS & Evidence Fabric

**AION guarantees deterministic execution, replay symmetry, drift detection, and audit‑grade evidence chains.**

AION does **not** attempt to sandbox or restrict filesystem or network access.  
This is intentional: AION is an **Execution‑OS**, not a Security‑Sandbox‑OS.

## Why AION?

- 🤖 **AI Agents need proof.** When an LLM makes a decision, can you prove *exactly* what the environment looked like?
- 🔁 **Reproducible CI/CD.** The same command on the same commit should produce the same output. AION guarantees it.
- 📋 **Audit‑ready.** Every run produces a cryptographically sealed **Capsule** – a tamper‑proof snapshot of the entire execution context.

## Features

- **Deterministic Subprocess Execution** – Environment, CWD, RNG, and time are frozen.
- **Evidence Chain** – Linear, hash‑linked proof of execution history.
- **Replay & Drift Detection** – Compare runs and pinpoint non‑determinism.
- **Contract‑OS** – Stable, versioned interface with semantic compatibility guarantees.
- **Zero Dependencies on Cloud** – Runs entirely locally. No accounts, no telemetry (opt‑in only).

## Quick Start

```bash
cargo install aion-cli
aion execute --command "python my_script.py"
```

## Enterprise

For regulated industries requiring **filesystem/network isolation**, **compliance exports**, or **SLA‑backed support**, check out **AION Enterprise** at [aion.sh](https://aion.sh).

## License

AION Core is licensed under the **MIT License**.  
Enterprise components are licensed under the **AION Enterprise License v1**.
