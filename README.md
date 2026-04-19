# AION

**AION** is a small command-line surface for **deterministic execution debugging**: capture what a command actually did, compare captures, reason about differences, and replay recorded output without running the command again.

The **repro** tool is the first capability on that surface. Day to day you invoke it as **`aion repro …`** so scripts, docs, and muscle memory stay on one entrypoint.

---

## What you get

| Area | What it gives you |
|------|-------------------|
| **Capture** | Records stdout, stderr, exit code, working directory, a compact environment fingerprint, and a persistent event stream next to each run under `./repro_runs/`. |
| **Compare** | A stable, ordered diff between two stored runs. |
| **Explain** | In two-run mode, a short deterministic narrative when environment changes line up with output changes. |
| **Replay** | Prints the stored stdout stream as it was recorded. |

Same command on different machines or days often diverges because of **environment and output**, not a single obvious log line. AION is built to make those differences **visible**, **comparable**, and **repeatable** in the terminal.

---

## Build and install (from source)

At the repository root:

```bash
cargo build --release -p aion -p repro
export PATH="$PWD/target/release:$PATH"
```

A typical session:

```bash
aion repro run -- echo hello
aion repro replay last
aion repro diff last prev
aion repro why <run_a> <run_b>
```

Use the run ids printed after each capture, or aliases like `last` / `prev` where supported. Pair **why** always takes two ids (or aliases).

---

## Quick flow

1. Capture two runs that should differ in a controlled way (for example, change one variable between runs with the same command).
2. **`aion repro diff`** the two ids and confirm what moved.
3. **`aion repro why`** the same pair when you want the short causal readout.

Runnable illustrations: `examples/basic_run.sh`, `examples/diff_example.sh`, `examples/why_analysis.sh`.

---

## More

- **Release notes:** `RELEASE.md` (v1.0.0)
- **Contributing:** `CONTRIBUTING.md`
- **License:** `LICENSE` (MIT)
