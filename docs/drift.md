# Drift

**Drift** answers: “what changed between two runs?” For AI capsules, drift is computed over stable fields (tokens, evidence digests, embedded Why/graph projections, etc.).

## At a glance

- Drift classifies deterministic differences between runs.
- Output categories and labels are tokenized and machine-readable.
- Drift contributes to finality and release admission decisions.

---

SealRun guarantees deterministic execution, replay symmetry, drift detection and audit‑grade evidence chains.  
SealRun intentionally does not enforce filesystem or network isolation.  
The kernel isolation modules are contract surfaces only; they define the interface but do not restrict access.

This is a deliberate design choice: SealRun is an Execution‑OS, not a Security‑Sandbox‑OS.  
Because SealRun does not modify kernel privileges or intercept syscalls, it is safe to adopt in existing environments without admin rights, without risk to workloads, and without operational friction.

If isolation is required (e.g., for regulated industries), the same contract surfaces can be backed by seccomp/landlock/micro‑VM isolation in a future "SealRun Secure Runtime" module — without breaking compatibility.

---

## CLI: drift between two run JSON files (observe)

```bash
sealrun observe drift left.json right.json
```

Produces drift JSON/HTML/SVG under `sealrun_output/drift/<timestamp>/`.

## CLI: drift between two capsules (SDK)

```bash
sealrun sdk drift --a first\.sealrunai --b second\.sealrunai
```

Exit code **2** when drift is detected (useful in CI).

## Example drift JSON (shape)

```json
{
  "changed": true,
  "fields": ["tokens", "seed"],
  "details": ["…"]
}
```

## Contract surface

- Map-Contract (Drift-Contract) with fixed categories and tolerance profile
- Global Consistency integration via run finality
- Measurement inputs for trend and KPI reporting

## CLI surface

```bash
sealrun observe drift left.json right.json
sealrun sdk drift --a first\.sealrunai --b second\.sealrunai
sealrun doctor
```

## Related

- [Replay](replay.md)
- [Governance](governance.md)

## Enterprise-readiness

Drift is enterprise-ready when labels/categories and tolerance outcomes remain deterministic across CI and production comparisons.
