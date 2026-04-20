# AION

AION — Make Execution Explainable.

Deterministic execution debugging — capture, diff, explain, replay.

AION is a deterministic execution truth layer for debugging, comparison, and reproducible automation.

It captures what actually happened during a command, compares executions deterministically, and explains why they differ.

If the same command behaves differently across machines, environments, or time - AION makes the difference visible.

## About

Suggested GitHub topics for this repository:

`determinism`, `reproducibility`, `devops`, `command-line`, `observability`, `ci-cd`, `debugging`, `rust`, `cli`, `diff-tool`, `testing-tools`

---

## Why AION exists

Commands do not behave consistently.

The same command can produce different results across machines, environments, or time.

Logs are incomplete.  
Debuggers do not capture environment drift.  
CI systems hide nondeterminism instead of explaining it.

Reproducibility is broken in practice.

AION exists to make execution behavior:

* visible  
* comparable  
* explainable  
* reproducible  

## What AION is

AION is a system for deterministic execution analysis.

It is composed of multiple surfaces:

* Repro — deterministic capture, diff, why, replay  
* Graph — execution relationships and causality (future)  
* Envelope — deterministic execution contracts (future)  
* Trace — event-based execution recording (future)  
* Inspect — execution introspection (future)  

Repro is the first available surface.

## 5-second proof

```bash
aion repro run -- echo hello
aion repro diff last prev
aion repro why last prev
```

## What you get

* Capture — see exactly what happened during a run  
* Compare — see what changed between runs  
* Explain — understand why it changed  
* Replay — reproduce output without re-running  

Artifacts are stored locally under `./repro_runs/`.

## Installation

From the repository root:

```bash
cargo build --release -p aion -p repro
export PATH="$PWD/target/release:$PATH"
```

## Quickstart

```bash
aion repro run -- echo hello
aion repro replay last
aion repro diff last prev
aion repro why last prev
```

## Examples

Runnable examples are available in:

* examples/basic_run.sh  
* examples/diff_example.sh  
* examples/why_analysis.sh

## CI Integration Examples

These examples use only the currently available AION CLI commands.

### GitHub Actions Example

```yaml
- name: Install AION
  run: cargo install --git https://github.com/aion-systems-core/aion

- name: Capture baseline (main)
  run: aion repro run -- ./scripts/build.sh

- name: Capture current (PR)
  run: aion repro run -- ./scripts/build.sh

- name: Compare outputs
  run: aion repro diff last prev
```

### Local Debugging Example

```bash
# Capture your environment
aion repro run -- python train.py

# Capture colleague's environment (manually shared JSON)
# Save it under repro_runs/<id>.json
# Then compare:
aion repro diff last prev
aion repro why last prev
```

### Reproducible Benchmarking Example

```bash
for i in {1..5}; do
    aion repro run -- ./benchmark --iterations 1000
done

# Compare the last two runs
aion repro diff last prev
```

## Release

See RELEASE.md for version information and changes.

## Contributing

See CONTRIBUTING.md.

---

## License

MIT
