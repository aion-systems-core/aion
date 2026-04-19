# Contributing to AION Repro

Thanks for helping improve **AION Repro**. Keep changes focused on the **CLI experience**, documentation, and tests unless you are explicitly working on a scoped maintenance task.

## How to try it

From the repository root:

```bash
cargo build -p aion -p repro
export PATH="$PWD/target/debug:$PATH"
aion repro run -- echo hello
```

Run the **Repro** test suite:

```bash
cargo test -p repro
```

For a full workspace check (optional, slower):

```bash
cargo test --workspace
```

## Guidelines

- Prefer **user-facing** language in `README.md` (repo root), `RELEASE.md`, and `examples/` — no internal platform vocabulary.
- Scripts and docs should use **`aion repro …`** as the primary command form.
- If you change the one-line product description, update it in **one source of truth** inside the Repro crate (`CATEGORY_DEFINITION`) and keep the **root `README.md`** in sync — the integration tests enforce alignment with `--help` and `repro eval`.

---

## Maintainer checklist (internal — not user-facing)

Use this before merging doc or packaging PRs:

- [ ] Root **`README.md`** contains none of: `cos`, `cos_core`, `kernel`, `ExecutionArtifact`, `ExecutionTrace`, internal module paths as “architecture.”
- [ ] **`examples/*.sh`** only use `aion repro …` and contain no internal terminology.
- [ ] **`RELEASE.md`** is self-contained enough to paste into GitHub Releases as-is.
- [ ] A new reader can understand **what to install and which three commands to run** in under **two minutes** using only the root README + one example script.
- [ ] `cargo test -p repro` passes after your change.
