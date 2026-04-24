# Evidence model (product view)

## Purpose

Narrative for auditors: how **evidence chain** steps relate to **capsules**, **replay symmetry**, and verification—complementing the [Architecture](../architecture.md) Evidence layer diagram.

This page describes how **capsules**, **hashes**, **signatures**, **replay**, and **verification** fit together for audits and compliance narratives. It is descriptive; the wire formats and algorithms live in the codebase unchanged.

## Flow

**Capsule → Hash → Sign → Replay → Verify**

1. **Capsule** — A persisted AI run (model, prompt, seed, tokens, determinism snapshot, **evidence chain**, Why report, causal graph, drift snapshot).
2. **Hash** — Deterministic digests over canonical capsule bytes and over the **evidence chain** root (rolling chain of step digests).
3. **Sign** — Optional **integrity envelope** (hash-chain style binding of capsule hash + evidence root) and optional **Ed25519** signatures over evidence payloads for external trust anchors.
4. **Replay** — Re-execute the same logical run and compare to the capsule; **replay symmetry** attests that deterministic outputs match.
5. **Verify** — Linear verification of the evidence chain (`parent_digest` / `payload_digest` continuity), signature checks, and governance **policy / determinism / integrity** validation.

## Chaining concepts

- **`previous_signature` → `next_signature`** — Operational pattern for append-only audit logs: each new integrity or evidence step may reference the prior signature so tampering breaks the chain. The capsule’s `IntegritySignature` type includes an optional `previous_signature` field for this style of linkage.
- **`determinism_profile` → `replay_profile`** — The run records a determinism snapshot; replay compares the **replay-facing** profile and flags **replay_profile** mismatches separately from token-level diffs.

## Explainability stack

**Drift → Why → Graph → Evidence**

- **Drift** — What changed between two capsules (or versus a baseline).
- **Why** — Structured explanation artefact tied to the run.
- **Graph** — Causal DAG view over the same run.
- **Evidence** — The underlying linear **evidence chain** that binds execution steps to rolling digests for third-party verification.

## Diagram (SVG)

![Evidence chain schematic](evidence_chain.svg)

## ASCII fallback

```
  Capsule (.aionai)
       │
       ├─► SHA256(capsule JSON) ──► integrity envelope "signature"
       │
       ├─► evidence.records[] ──► leaf_digest + payload_digest chain
       │         │
       │         └─► Ed25519 (optional) over evidence bytes
       │
       ├─► Replay engine ──► replay_symmetry_ok / diff report
       │
       └─► Governance validate ──► policy + determinism + integrity
```

## See also

- [Guided tour: Evidence Model entry point](../guided_tour.md)
- [Pilot: evidence chain walkthrough](../pilot/06_evidence_chain.md)
- [Governance](../governance.md) (policy / integrity profiles)
