# Engine integration tests

Rust integration tests for `aion-engine`: deterministic AI capsules, replay symmetry, drift contracts, governance and SDK surfaces, syscall policy, C ABI smoke, and formal replay invariants.

Run from the repository root:

```bash
cargo test -p aion-engine
```

Artefact directories under `sealrun_output/` are created at runtime and must not be committed (see root `.gitignore`).
