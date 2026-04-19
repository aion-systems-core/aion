# AION v1.0.0 — Release notes

**Release:** v1.0.0  
**Tag:** `v1.0.0`

---

## Summary

This is the first **product-shaped** public release of **AION**: a command-line entrypoint plus the **repro** tool for **deterministic execution debugging** — capture real runs, compare them, explain environment-linked differences when the data supports it, and replay captured **stdout** without re-executing the original command. Prefer **`aion repro …`** everywhere in docs and automation; the same tool is also available as **`repro`** when invoked directly.

---

## Highlights

- **Run** — Local capture under `./repro_runs/` (stdout, stderr, exit, cwd, environment fingerprint, command line).
- **Diff** — Ordered, repeatable comparison of two runs.
- **Why** — Pair mode with a concise narrative when environment changes align with output change.
- **Replay** — Raw stdout from the stored stream.
- **Streams** — Sidecar event data next to each stored run for auditing and replay.

---

## Deterministic debugging

Given the same stored inputs, comparison and formatting follow fixed rules so output stays stable for scripting and review. Live captures can still differ on time- or machine-specific fields; the **comparison story** stays structured and repeatable.

---

## Commands (overview)

| Command | Role |
|---------|------|
| `aion repro run -- <command>` | Capture a new run |
| `aion repro replay <id \| last>` | Print captured stdout |
| `aion repro diff <a> <b>` | Compare two runs |
| `aion repro why <a> <b>` | Pair explanation (environment vs output) |

Build from source at the repository root:

```bash
cargo build --release -p aion -p repro
export PATH="$PWD/target/release:$PATH"
```

---

## Example

```bash
aion repro run -- echo hello
aion repro replay last
aion repro diff last prev
```

Illustrative scripts: `examples/basic_run.sh`, `examples/diff_example.sh`, `examples/why_analysis.sh`.

---

## Copy-paste: GitHub release body

_Use the sections from **Summary** through **Example** above as the release description. This tree is source-first; attach separate binaries if you distribute them._
