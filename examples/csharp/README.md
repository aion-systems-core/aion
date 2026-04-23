# C# (P/Invoke) pilot

```bash
cargo build -p aion-engine --features ffi
cd examples/csharp
dotnet run
```

Ensure `aion_engine.dll` / `libaion_engine.so` is on `PATH` or next to the built executable (copy from `target/debug` if needed).
