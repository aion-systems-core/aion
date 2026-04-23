# Replay

**Replay** reconstructs a run from a capsule and compares it to the stored record, producing a structured **replay report** (JSON/HTML/SVG).

## At a glance

- Replay is a kernel-layer determinism control.
- Outputs are deterministic and machine-readable.
- Replay status contributes to finality and doctor readiness.

---

AION guarantees deterministic execution, replay symmetry, drift detection and audit‑grade evidence chains.  
AION intentionally does not enforce filesystem or network isolation.  
The kernel isolation modules are contract surfaces only; they define the interface but do not restrict access.

This is a deliberate design choice: AION is an Execution‑OS, not a Security‑Sandbox‑OS.  
Because AION does not modify kernel privileges or intercept syscalls, it is safe to adopt in existing environments without admin rights, without risk to workloads, and without operational friction.

If isolation is required (e.g., for regulated industries), the same contract surfaces can be backed by seccomp/landlock/micro‑VM isolation in a future "AION Secure Runtime" module — without breaking compatibility.

---

## CLI: AI replay

```bash
cargo run -p aion-cli -- execute ai-replay --capsule path/to/capsule.aionai
```

### Example output location

```
aion_output/ai-replay/<timestamp>/ai.json
```

The JSON summarises whether replay succeeded and lists comparison flags.
Replay reports include metadata fields such as:

- `replay_timestamp`
- `replay_aion_version`
- `replay_duration_ms`
- `first_differing_token`
- `warnings` (version mismatch / missing evidence notices)

## SDK / automation

For headless workflows:

```bash
cargo run -p aion-cli -- sdk replay --capsule path/to/capsule.aionai
```

This writes `sdk.json` (+ HTML/SVG) under `aion_output/sdk-replay/<timestamp>/`.

## Contract surface

- Process-Contract (Replay-Invariant)
- Global Consistency finality (`replay_finality`, `run_finality`)
- Determinism/compatibility checks across version windows

## CLI surface

```bash
aion execute ai-replay --capsule path/to/capsule.aionai
aion sdk replay --capsule path/to/capsule.aionai
aion doctor
```

## Related

- [Capsules](capsules.md)
- [Drift](drift.md)
- [SDK](sdk.md)

## Enterprise-readiness

Replay is enterprise-ready when invariant/symmetry checks and mismatch outputs stay stable across supported environments and versions.
