# Compatibility matrix

This matrix summarizes core version anchors for deterministic compatibility in AION-OS.

## At a glance

- Compatibility is contract-governed and version-explicit.
- Capsule, Why, and policy versions are stable integration anchors.
- Use doctor and distribution identity outputs for runtime validation.

---

AION guarantees deterministic execution, replay symmetry, drift detection and audit‑grade evidence chains.  
AION intentionally does not enforce filesystem or network isolation.  
The kernel isolation modules are contract surfaces only; they define the interface but do not restrict access.

This is a deliberate design choice: AION is an Execution‑OS, not a Security‑Sandbox‑OS.  
Because AION does not modify kernel privileges or intercept syscalls, it is safe to adopt in existing environments without admin rights, without risk to workloads, and without operational friction.

If isolation is required (e.g., for regulated industries), the same contract surfaces can be backed by seccomp/landlock/micro‑VM isolation in a future "AION Secure Runtime" module — without breaking compatibility.

---

| Component | Version |
|-----------|---------|
| AION‑OS product | `VERSION` file |
| CLI binary | `aion --version` |
| AI capsule schema | `version = "1"` |
| Why schema | `why_schema_version = "2"` |
| Governance policy version | `policy_version = "1"` |

## Upgrade guidance

- Minor/patch upgrades should preserve capsule `version = "1"` compatibility.
- If capsule version changes in future major versions, migration docs must include conversion tooling.

## CLI surface

```bash
aion --version
aion doctor
aion dist identity
```

## Enterprise-readiness

Compatibility is enterprise-ready when supported version combinations remain deterministic, documented, and test-covered.
