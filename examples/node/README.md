# Node (ffi-napi) pilot

Build the shared library:

```bash
cargo build -p aion-engine --features ffi
```

Install and run (expects an existing capsule path):

```bash
cd examples/node
npm install
node index.js path/to/capsule.aionai
```

For a full **save + replay** flow via C structs, use the Rust `c_abi_smoke_test` or the Go / C# samples.
