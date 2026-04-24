# Compatibility Layer

## Purpose

Language and OS **interop boundaries** (C ABI, Rust, Python, …) on top of the same deterministic contracts—without duplicating kernel semantics ([Architecture](architecture.md)).

This document defines language/runtime interoperability boundaries for SealRun.

## At a glance

- Multi-language bindings on top of deterministic core contracts
- Cross-OS and cross-arch compatibility targets
- Universal C ABI as lowest common integration surface

## Supported languages

- C/C++ via universal C ABI (`include/sealrun/sealrun.h`, `include/sealrun/sealrun.hpp`).
- Rust via native `aion-core`/`aion-engine` APIs and optional C ABI feature (`ffi`).
- Python via `pyo3` module in `bindings/python` and `ctypes` fallback in `bindings/python/ctypes`.
- Go via `cgo` bridge in `bindings/go/sealrun.go`.
- Java via JNI wrapper scaffold in `bindings/java`.
- C# via P/Invoke scaffold in `bindings/csharp/SealRunNative.cs`.
- Node.js via `ffi-napi` scaffold in `bindings/node/index.js`.

## Supported operating systems

- Linux (glibc 2.17+ / musl; `~/.sealrun` config defaults)
- macOS 10.15+ (`~/Library/Application Support/SealRun` compatible paths)
- Windows 10+ (`%APPDATA%/SealRun` and `%USERPROFILE%/.sealrun` compatibility)
- FreeBSD 12+ (`~/.sealrun`)

## Supported architectures

- x86_64
- aarch64

## Universal C ABI

- Exported functions are implemented in `engine/src/ffi.rs`.
- Error model uses integer error codes and `aion_last_error()`.
- Struct ABI uses `#[repr(C)]` and primitive C-compatible fields.

## Async API layer

- Rust async wrappers are available under `aion_engine::sdk::*_async` with feature `async`.
- Implementation uses Tokio `spawn_blocking` for sync calls.

## Build notes

- Build shared/static library with:
  - `cargo build -p aion-engine --features ffi`
- Python wheels:
  - `cd bindings/python && maturin build --release --compatibility manylinux2014`

## Contract surface

- FFI contract surfaces map to deterministic error/output contracts
- Compatibility matrix and identity contracts define supported combinations
- Installer/distribution trust chain governs enterprise package confidence

## CLI surface

```bash
sealrun doctor
sealrun dist identity
sealrun dist status
```

## Enterprise-readiness

Compatibility is enterprise-ready when supported language/runtime combinations are explicit, deterministic, and continuously validated.
