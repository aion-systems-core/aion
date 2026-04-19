# AION

AION is a deterministic execution truth layer for debugging, comparison, and reproducible automation.

It captures what actually happens during a command, compares executions deterministically, and explains why they differ.

If the same command behaves differently across machines, environments, or time — AION makes the difference visible.

---

## Why AION exists

Modern execution is not stable.

The same command can behave differently across environments, time, or systems.

- Logs are incomplete  
- Debuggers miss environment drift  
- CI systems hide nondeterminism  
- Failures are often non-reproducible  

AION exists to make execution behavior:

- visible  
- comparable  
- explainable  
- reproducible  

---

## What AION is

AION is a system for deterministic execution analysis.

It is composed of surfaces:

- **Repro** — capture, diff, explain, replay execution  
- **Graph** — causal execution relationships (future)  
- **Envelope** — deterministic execution contracts (future)  
- **Inspect** — execution introspection (future)  

Repro is the first public surface.

---

## 5-second proof

aion repro run -- echo hello  
aion repro diff last prev  
aion repro why last prev  

This captures a run, compares it, and explains differences deterministically.

---

## What you get

- Capture execution output, exit code, environment fingerprint  
- Compare runs deterministically  
- Explain differences via environment alignment  
- Replay stored output without re-execution  

All runs are stored locally under:

./repro_runs/

---

## Installation

cargo build --release -p aion -p repro  
export PATH="$PWD/target/release:$PATH"

This installs the `aion` CLI with the `repro` surface enabled.

---

## Quickstart

aion repro run -- echo hello  
aion repro replay last  
aion repro diff last prev  
aion repro why last prev  

---

## Examples

- examples/basic_run.sh  
- examples/diff_example.sh  
- examples/why_analysis.sh  

---

## Scope

This release includes:

- Repro CLI  
- Deterministic execution capture  
- Execution comparison (diff)  
- Root-cause explanation (why)  
- Replay from stored execution traces  

Future surfaces:

- Graph (causal execution graphs)  
- Envelope (execution contracts)  
- Inspect (execution introspection)  

---

## Stability

- Deterministic execution is enforced  
- Output comparisons are stable across runs  
- Replay produces identical output to original capture  
- Tests verify execution consistency  

AION is designed to behave the same way today, tomorrow, and across machines.

---

## Notes

- All data is local — no external services required  
- Designed for CI, debugging, and reproducible workflows  
- No internal system architecture is exposed  
- Works with any command that can be executed from a shell  

---

## License

MIT
