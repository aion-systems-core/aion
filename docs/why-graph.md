# Why & causal graph

## Purpose

Document **explainability artefacts** (`why.html`, `why.svg`, replay **why-diff** projections), how they tie to **replay symmetry** and **drift detection**, and how to regenerate them via **`sealrun observe graph`** and **`sealrun sdk explain`**.

SealRun attaches a structured **Why** report and a **causal graph** to AI capsules so runs are explainable without opening proprietary model weights.

## Explainability artefacts

| Artefact | Produced by | Role |
|----------|----------------|------|
| `why.html` | `execute ai` | Tabular Why + embedded causal graph for review in a browser |
| `why.svg` | `execute ai` | Standalone causal graph vector |
| `why_diff.html` / `why_diff.svg` | `execute ai-replay` | Deterministic diff of Why payloads between original and replayed capsule views |
| `sealrun observe graph` | CLI | Renders graph projections from a `RunResult` JSON path (format/depth flags) |
| `sealrun sdk explain` | CLI/SDK | Emits explain bundle JSON/HTML/SVG from a **`capsule.aionai`** path |

Drift JSON/HTML may reference graph/Why slices as part of deterministic field classification ([Drift](drift.md)).

## At a glance

- Why/graph artifacts provide deterministic explainability surfaces.
- Artifacts are capsule-bound and replay-comparable.
- Outputs support audit and review workflows.

---

SealRun guarantees deterministic execution, replay symmetry, drift detection and audit‑grade evidence chains.  
SealRun intentionally does not enforce filesystem or network isolation.  
The kernel isolation modules are contract surfaces only; they define the interface but do not restrict access.

This is a deliberate design choice: SealRun is a deterministic execution engine, not a Security-Sandbox-OS.  
Because SealRun does not modify kernel privileges or intercept syscalls, it is safe to adopt in existing environments without admin rights, without risk to workloads, and without operational friction.

If isolation is required (e.g., for regulated industries), the same contract surfaces can be backed by seccomp/landlock/micro‑VM isolation in a future "SealRun Secure Runtime" module — without breaking compatibility.

---

## CLI

```bash
sealrun execute ai --model m --prompt "a b" --seed 1
# open sealrun_output/ai/<ts>/why.html in a browser
```

`observe graph` supports format/depth controls:

```bash
sealrun observe graph path/to/run.json --format dot --depth 20
```

## Explain bundle (SDK)

```bash
sealrun sdk explain --capsule path/to/capsule.aionai
```

## Contract surface

- Explainability payloads are part of capsule/evidence-facing outputs
- Replay and drift consume Why/graph projections for deterministic comparisons
- Governance and audit processes can reference explainability artifacts

## CLI surface

```bash
sealrun execute ai --model m --prompt "a b" --seed 1
sealrun observe graph path/to/run.json --format dot --depth 20
sealrun sdk explain --capsule path/to/capsule.aionai
```

## ASCII sketch

```
  prompt segments ──┐
                    ├──► token_0 ──► token_1 ──► …
  seed ─────────────┤
  determinism ──────┘
```

## Related

- [Capsules](capsules.md)
- [SDK](sdk.md)
- [Why schema](why-schema.json)
- [Example Why report](example-why-report.json)

## Enterprise-readiness

Why/graph outputs are enterprise-ready when artifact structure and replay comparison behavior remain deterministic and referenceable.
