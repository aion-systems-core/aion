# Go (cgo) pilot

`aion_run` is exercised in `engine/tests/c_abi_smoke_test.rs` (same linkage as this sample).

This directory demonstrates **capsule save → replay → symmetry → hash** via cgo.

```bash
cargo build -p aion-engine --features ffi
cd examples/go
go run .
```

Adjust `CGO_LDFLAGS` if your `target/debug` layout differs (see `examples/c_abi/README.md`).
