# AION C-ABI showcase

Minimal examples calling the published C ABI from `include/aion/aion.h`.

## Prerequisite

Build the engine shared library from the repository root:

```text
cargo build -p aion-engine
```

On Windows the artifact is `target/debug/aion_engine.dll` (or `target/release/` with `--release`). On Linux `target/debug/libaion_engine.so`. On macOS `target/debug/libaion_engine.dylib`.

Set `AION_LIB_PATH` to the directory containing that file when running Python examples, or rely on the default search used by each Makefile.

## Build all (POSIX)

```text
make -C examples/c_abi_showcase
```

## Per language

- C: `make -C examples/c_abi_showcase/c`
- C++: `make -C examples/c_abi_showcase/cpp`
- Go: `cd examples/c_abi_showcase/go && go build`
- Python: `python examples/c_abi_showcase/python/demo.py`

Each program writes deterministic lines to stdout and uses non-zero exit status on failure.
