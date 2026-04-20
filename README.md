# AION - Deterministic Execution Tools

AION provides deterministic execution tools for CI/CD, debugging, and automation.  
This repository contains the public AION tooling:

- **aion-repro** - deterministic run capture, replay, diff, and why-analysis  
- **aion-guard** - deterministic CI drift detection  
- **aion-cli** - unified command-line interface for all AION tools  

AION tools run on top of the AION Execution Kernel, which is distributed separately.

---

## Tools

### aion-repro
Deterministic run capture and replay.

- Freeze command execution
- Replay without re-running
- Diff and why-analysis
- Reproducible artifacts

See: [crates/aion-repro/README.md](crates/aion-repro/README.md)

---

### aion-guard
Deterministic CI drift detection.

- Baseline recording
- Drift detection (stdout, stderr, exit code)
- Optional duration tolerance
- Stable CI exit codes

See: [crates/aion-guard/README.md](crates/aion-guard/README.md)

---

### aion-cli
Unified entry point for all AION tools.

```bash
aion repro ...
aion guard ...
```

---

## Kernel Boundary

AION tools dynamically load the AION Execution Kernel at runtime.  
The kernel is distributed separately and is not part of this repository.

If the kernel is not installed, tools will report:

```text
AION Kernel not found. Install aion-kernel or set AION_KERNEL_PATH.
```

---

## Build

```bash
cargo build --workspace --release
```
---
## License

MIT (tools only)
