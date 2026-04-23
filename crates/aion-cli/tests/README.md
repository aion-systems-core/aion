# CLI integration tests

Integration tests for the `sealrun` binary (`aion-cli` crate): help output determinism, execute flows, JSON envelopes, `doctor`/domain contract writers, and product layout checks.

Run from the repository root:

```bash
cargo test -p aion-cli
```

Tests invoke the built binary via `CARGO_BIN_EXE_sealrun`. Do not commit `sealrun_output/` or other generated run directories.
