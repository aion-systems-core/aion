# aion-python (pilot)

Industrial Python pilot over the **SealRun C ABI** (`include/aion/aion.h`). No PyO3 in this wheel: only `ctypes` plus a thin API.

## Build the native library

From the repository root:

```text
cargo build -p aion-engine
```

Windows: `target/debug/aion_engine.dll`. Linux: `target/debug/libaion_engine.so`. macOS: `target/debug/libaion_engine.dylib`.

## Install the wheel

From `bindings/python/`:

```text
pip install build
python -m build --wheel
pip install dist/aion_python-*.whl
```

Set **`SEALRUN_LIB_PATH`** to the directory that contains the shared library (or a direct path to the file). The CLI doctor check still honors the pre-rename library-path variable — see `crates/aion-cli/src/output_bundle.rs`.

## API

- `Pilot().execute_ai(model=..., prompt=..., seed=..., out_path=...)`
- `Pilot().replay(capsule_path)`
- `Pilot().drift(left_path, right_path)`
- `Pilot().evidence(evidence_json_path)`

## CLI

```text
python -m aion execute-ai --model demo --prompt hi --seed 1 --out /tmp/capsule.aionai
python -m aion replay /tmp/capsule.aionai
python -m aion drift /tmp/a.aionai /tmp/b.aionai
python -m aion evidence /path/to/evidence.json
```

Exit code `2` means drift `changed` (deterministic comparison).

## Legacy PyO3 extension

The optional Rust extension in `bindings/python/src/` is not part of this wheel. To build it separately, use `maturin` with `bindings/python/Cargo.toml` (not wired into the workspace `cargo build` graph).

## Examples and notebook

- JSON fixtures: `examples/demo_capsule_*.json`, `examples/demo_evidence.json`
- Notebook: `notebooks/pilot_demo.ipynb`
