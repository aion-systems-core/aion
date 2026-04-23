# Legacy gold import audit

## Scope and limits

- **Scanned roots that exist on disk:** `Documents/aion-os`, `Documents/aion`.
- **Missing at `Documents/` top level:** `repro/`, `cos/`, `cognitive_os_v14/`, `cos-v1/`, `aion-guard/`, `aion-repro/`, `aion-old/` (those trees appear **nested under `aion/`** instead, e.g. `aion/cos/tools/repro`, `aion/cognitive_os_v14`, `aion/cos-v1`).
- **Wildcard `Documents/*`:** not exhaustively enumerated (1000+ Rust files). This pass imports **curated high-value subtrees** judged **≥9/10** as cohesive product logic, not every `.rs` in every repo.

## Method (quality ≥9)

Copied **entire source subtrees** where the codebase already treats the layer as reusable library-quality code (core engine, CI ledger, COS kernel core, adapters, v14 evidence engine, v1 kernel/audit).

## Copied trees (`legacy_gold/…`)

| Source | Files (approx) | Role |
|--------|----------------|------|
| `aion-os/repro/src/core` | 17 | Repro deterministic core |
| `aion-os/repro/src/ci` | 11 | CI schema, baseline, orchestration |
| `aion-os/repro/src/engine` | 2 | COS adapter boundary |
| `aion-os/repro/src/lib.rs` | 1 | Crate root |
| `aion-os/cos/core/src` | 18 | Evidence, audit, replay engine |
| `aion-os/crates/aion-core/src` | 3 | Shared contracts + capsule (branch) |
| `aion-os/scripts` | 1 | Kernel integrity verifier |
| `aion/cos/tools/repro/src/{core,ci,engine}` | 30 | Parallel repro stack inside monorepo |
| `aion/cos/core/src` | 18 | Duplicate cos core (historical) |
| `aion/cognitive_os_v14/src/{evidence_engine,kernel}` | 16 | v14 kernel + evidence |
| `aion/cos-v1/src/{kernel,evidence,audit}` | 15 | v1 kernel / audit surface |
| `aion/cos/adapters/src` | 7 | Bridge layers (repro/v1/v14) |
| `aion/cos/governance/src` | 4 | Policy interfaces |
| `aion/aion/src/core` | 3 | Tool contract + repro wiring |
| `aion/scripts` | 1 | Integrity script |

## Potentially valuable (7–8) — not bulk-copied

- `repro/src/cli/*.rs`, `repro/src/analysis/*.rs` — good UX / product logic but **CLI-coupled**.
- `repro/tests/**` — high value for **regression**; belong in `aion-os-v2/tests/` when ported, not `legacy_gold`.
- `aion-os/crates/aion-cli`, `aion-repro`, `aion-guard` binaries — superseded by v2 `aion-cli` surface.
- `aion/cos/runtime/**`, large `cognitive_os_v14` beyond `kernel`+`evidence_engine` — broader scope; review before merge.

## Integration hints (v2 architecture)

| Gold region | Target in aion-os-v2 |
|-------------|----------------------|
| `repro/src/core/*` | `engine/` + `core/contracts` extensions |
| `repro/src/ci/*` | `engine/ci.rs` + typed ledger in `core/` |
| `cos/core/src/*` | `kernel/` integrity/evidence or separate `cos-adapter` crate |
| `cognitive_os_v14/src/kernel` | `kernel/` (replace stubs) |
| `cos/adapters/*` | boundary crate between engine and COS |
| `cos/governance/*` | `engine/policy.rs` / `core` policy types |

## Verdict

**Mostly complete for “library-grade” layers** under the two reachable monorepos; **not** a byte-for-byte mirror of every `Documents/**/*.rs`. Expand with per-file scoring when you narrow the next integration milestone.
