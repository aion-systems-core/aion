# AION v1.1.0 — Deterministic Execution Tools

**Release:** v1.1.0  
**Tag:** `v1.1.0`  
**Scope:** Workspace release (`aion`, `aion-repro`, `aion-guard`)

---

## Summary

AION v1.1.0 delivers a unified deterministic execution toolchain for CI/CD, debugging, and automation.
This release includes the full public workspace:

- **aion-repro** — deterministic run capture, replay, diff, and why-analysis
- **aion-guard** — deterministic CI drift detection
- **aion-cli** — unified entry point for all AION tools

All tools run on top of the AION Execution Kernel (distributed separately).
The release focuses on determinism, reproducibility, and audit-ready execution artifacts.

---

## Highlights

### aion-repro — Deterministic Run Capture and Replay

- Freeze command execution into reproducible artifacts
- Replay without re-running the original command
- Deterministic diff (`stdout`, `stderr`, `exit_code`)
- Why-analysis for environment-linked differences
- Stable, auditable run directories

### aion-guard — Deterministic CI Drift Detection

- Baseline recording for CI pipelines
- Drift detection across `stdout`, `stderr`, `exit_code`
- Optional duration tolerance
- Stable deterministic CI exit codes
- Reproducible comparisons for automation and audits

### aion-cli — Unified Command Interface

- Single entry point: `aion repro ...`, `aion guard ...`
- Consistent UX across tools
- Workspace-wide versioning

---

## Kernel Boundary

All AION tools dynamically load the AION Execution Kernel at runtime.
If the kernel is missing:

```text
AION Kernel not found. Install aion-kernel or set AION_KERNEL_PATH.
```

The kernel is distributed separately and is not part of this repository.

---

## Commands (overview)

| Area | Command | Role |
|------|---------|------|
| repro | `aion repro run -- <command>` | Capture a run |
| repro | `aion repro replay <id \| last>` | Replay captured stdout |
| repro | `aion repro diff <id-a> <id-b>` | Deterministic run diff |
| repro | `aion repro why <id-a> <id-b>` | Environment-vs-output explanation |
| guard | `aion guard record --cmd "<command>"` | Record baseline |
| guard | `aion guard check --cmd "<command>"` | Detect drift |

---

## Build

```bash
cargo build --workspace --release
```

---

## Example

```bash
aion repro run -- echo hello
aion repro replay last
aion guard record --cmd "echo hello"
aion guard check --cmd "echo hello"
```

---

## Copy-paste: GitHub release body

Use the sections from **Summary** through **Example** as the release description.
