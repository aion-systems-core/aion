# TRACE — CLI specification (Tool 2)

**Status:** specification only. No implementation in this repository revision.

This document defines the intended `aion trace` command surface. It assumes existing AION Repro storage and artifact shapes; it does not extend the COS kernel or change Repro semantics.

---

## 1. Purpose

- **Read-only.** Inspect and visualize execution timelines derived from artifacts already produced by the stack (e.g. Repro capture output).
- **No new kernel semantics.** Trace does not define new execution truth; it projects existing records.
- **No new file formats.** All inputs are files that Repro (and mirrored CI paths, where applicable) already write. Output is either plain text to stdout or JSON to stdout as specified below—no new on-disk schema for export.

---

## 2. Commands

### 2.a `aion trace <run-id>`

- **Behavior:** Prints one **linear timeline** of execution events for the resolved run, in a single deterministic total order.
- **Ordering:** Strictly ascending by the canonical event sequence already stored (e.g. artifact `trace.events` ordering). No reordering for display convenience.
- **Side effects:** None. Read artifact / event stream from existing paths only.

### 2.b `aion trace graph <run-id>`

- **Behavior:** Prints an **ASCII-only** dependency graph of the same logical event set (nodes and directed edges implied by the causal projection used today for `why` / graph-style views).
- **Layout:** Deterministic: stable node id ordering, stable edge ordering, fixed tie-breaks documented in the implementation plan (not here). Printable ASCII only (space through `~`, plus newlines); no ANSI escapes, no Unicode box-drawing or other non-ASCII glyphs.
- **Side effects:** None.

### 2.c `aion trace export <run-id>`

- **Behavior:** Writes to **stdout** a JSON document whose structure is a **subset or direct embedding** of existing serialized types (e.g. `ExecutionTrace` / event list as already serialized by serde in artifacts). No new top-level schema version field introduced by this command.
- **Side effects:** None beyond stdout.

---

## 3. Input and output rules

- **Run IDs:** Accepts the same identifiers as Repro today: literal `run_id` strings and aliases `last` / `prev` resolved against `./repro_runs/INDEX` from the process current working directory (same rule stack as `repro`).
- **Reads:** Only files already defined by Repro (artifact JSON, companion event stream JSON if present). Optional read of CI mirror paths only if explicitly documented to mirror Repro layout; default scope is `./repro_runs/`.
- **Writes:** Forbidden. No creation or modification of files or directories. No new cache directories.

---

## 4. Error handling

- **Missing run / alias resolution failure:** Clear message on stderr; exit code non-zero (documented constant in implementation, e.g. `1`).
- **Invalid JSON or schema mismatch:** Message names the file path and failure kind; non-zero exit.
- **No panics** on user-controlled inputs; parse and I/O errors map to `Result`-style handling at the tool boundary.

---

## 5. Implementation constraints (for implementers)

- Implement as a routed tool under `aion`, analogous to `repro`, without changing existing `repro` or `cos_core` public APIs unless done in a separate, explicitly versioned change set.
- Do not add flags, subcommands, or storage locations beyond this spec in the initial implementation pass.
- Determinism: same `(cwd, run-id, command)` → identical stdout/stderr for success paths.

---

## 6. Data flow (informative)

```text
./repro_runs/INDEX  →  resolve run-id
./repro_runs/<id>.json  →  load ExecutionArtifact (or equivalent)
./repro_runs/<id>.events.json  →  optional canonical stream; if present, timeline must match artifact trace contract
stdout  →  human text or JSON per subcommand
stderr  →  errors only
```

---

## 7. Non-goals

- Live tracing of processes not already captured.
- Modifying INDEX, artifacts, or event streams.
- New COS record types or kernel entrypoints introduced solely for Trace.
