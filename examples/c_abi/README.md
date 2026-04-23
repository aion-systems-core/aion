# C ABI showcase (pilot)

These examples call `aion_engine` as a **cdylib** with the `ffi` feature enabled.

## Build the library

From the repository root:

```bash
cargo build -p aion-engine --features ffi
```

- **Windows:** `target/debug/aion_engine.dll`
- **Linux / macOS:** `target/debug/libaion_engine.so` or `libaion_engine.dylib`

Point `LD_LIBRARY_PATH` / `PATH` at `target/debug` (or copy the library next to each example).

## Symbols used

- `aion_run` — capture a subprocess (demo)
- `aion_capsule_save` — persist a synthetic AI capsule
- `aion_replay_capsule` — replay from path
- `aion_replay_symmetry_ok` — 1 if replay symmetry passed
- `aion_capsule_deterministic_hash_hex` — Blake3 content hash (hex)
- `aion_free_string`, `aion_free_run_result`, `aion_last_error`

See `engine/src/ffi.rs` for full API.
