# SealRun compliance one-pager

## Purpose

One-page bridge for **CISO / compliance** readers: ties business language to **deterministic capsules**, **replay symmetry**, **drift detection**, and **evidence chain**—with pointers to the [Security Guide](../security-guide.md) and [OS Contract Spec](../os_contract_spec.md).

**SealRun = deterministic execution** — Same model, prompt, seed, and frozen runtime contract yields the same token stream and artefacts, so security and compliance teams can reason about *repeatable* AI behaviour.

**SealRun = audit trail** — Each run emits structured outputs (JSON, HTML, SVG) and a **capsule** suitable for retention, e-discovery, and change control.

**SealRun = evidence chain** — A linear chain of digests binds execution steps; verifiers can check **continuity** without trusting a single opaque blob.

**SealRun = replay symmetry** — **Replay** reconstructs the run and compares it to the persisted capsule, surfacing explicit symmetry flags and diffs.

**SealRun = policy enforcement** — **Governance** profiles constrain models, prompts, seeds, evidence presence, and replay outcomes; validation is machine-checkable.

**SealRun = model-agnostic** — The contract is about *recorded* runs and backends that honour the determinism envelope—not a single vendor SDK.

---

**Guided Link: Compliance One-Pager** — Continue with [Evidence model](../evidence/evidence_model.md) or [Pilot onboarding](../pilot/00_install.md).
