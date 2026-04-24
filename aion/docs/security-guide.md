# Security guide

This guide explains deterministic security controls and evidence surfaces in SealRun Execution OS.

## At a glance

- Security behavior is contract-driven and machine-readable.
- Evidence chains and policy gates produce deterministic audit artifacts.
- Identity/distribution/installers provide trust and support boundaries.

---

SealRun guarantees deterministic execution, replay symmetry, drift detection and audit‑grade evidence chains.  
SealRun intentionally does not enforce filesystem or network isolation.  
The kernel isolation modules are contract surfaces only; they define the interface but do not restrict access.

This is a deliberate design choice: SealRun is an Execution‑OS, not a Security‑Sandbox‑OS.  
Because SealRun does not modify kernel privileges or intercept syscalls, it is safe to adopt in existing environments without admin rights, without risk to workloads, and without operational friction.

If isolation is required (e.g., for regulated industries), the same contract surfaces can be backed by seccomp/landlock/micro‑VM isolation in a future "SealRun Secure Runtime" module — without breaking compatibility.

---

## Contract surface

- Threat/compliance/scanning/logging contracts in security model
- Policy gate and policy evidence contracts in governance layer
- Installer trust chain and distribution identity contracts
- Measurement audit report and evidence export contracts

## CLI surface

```bash
aion governance status
aion policy gates
aion policy evidence
aion dist installers
aion dist identity
aion measure audits
aion measure evidence
```

## Deterministic execution guarantees

- Replay, drift, and evidence outputs are deterministic by contract.
- Policy enforcement has explicit decisions and no silent bypasses.
- Audit findings are structured by scope/severity/evidence reference.

## Identity layer and trust chain

- Identity matrix defines supported OS/arch/ABI/contract combinations.
- Installer trust chain defines trusted/untrusted status for distribution artifacts.
- Use both for release admission and rollout controls.

## Enterprise readiness

- Security teams should consume JSON envelopes as primary control evidence.
- Governance and measurement outputs should be archived for external audits.
