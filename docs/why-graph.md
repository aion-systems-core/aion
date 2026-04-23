# Why & causal graph

AION‑OS attaches a structured **Why** report and a **causal graph** to AI capsules so runs are explainable without opening proprietary model weights.

## At a glance

- Why/graph artifacts provide deterministic explainability surfaces.
- Artifacts are capsule-bound and replay-comparable.
- Outputs support audit and review workflows.

---

AION guarantees deterministic execution, replay symmetry, drift detection and audit‑grade evidence chains.  
AION intentionally does not enforce filesystem or network isolation.  
The kernel isolation modules are contract surfaces only; they define the interface but do not restrict access.

This is a deliberate design choice: AION is an Execution‑OS, not a Security‑Sandbox‑OS.  
Because AION does not modify kernel privileges or intercept syscalls, it is safe to adopt in existing environments without admin rights, without risk to workloads, and without operational friction.

If isolation is required (e.g., for regulated industries), the same contract surfaces can be backed by seccomp/landlock/micro‑VM isolation in a future "AION Secure Runtime" module — without breaking compatibility.

---

## Outputs

After `execute ai`, you typically receive:

- `why.html` — tables for prompt, token, seed/determinism influence plus embedded graph SVG  
- `why.svg` — standalone causal graph  
- `ai.html` / capsule JSON — includes the same structures for tooling  

Replay adds **why diff** artefacts when you run `execute ai-replay`.

## CLI

```bash
cargo run -p aion-cli -- execute ai --model m --prompt "a b" --seed 1
# open aion_output/ai/<ts>/why.html in a browser
```

`observe graph` supports format/depth controls:

```bash
cargo run -p aion-cli -- observe graph path/to/run.json --format dot --depth 20
```

## Explain bundle (SDK)

```bash
cargo run -p aion-cli -- sdk explain --capsule path/to/capsule.aionai
```

## Contract surface

- Explainability payloads are part of capsule/evidence-facing outputs
- Replay and drift consume Why/graph projections for deterministic comparisons
- Governance and audit processes can reference explainability artifacts

## CLI surface

```bash
aion execute ai --model m --prompt "a b" --seed 1
aion observe graph path/to/run.json --format dot --depth 20
aion sdk explain --capsule path/to/capsule.aionai
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
