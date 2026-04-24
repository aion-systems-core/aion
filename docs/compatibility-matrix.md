# Compatibility matrix

## Purpose

Single table of **version anchors** (product, CLI, capsule schema, Why schema, policy) for integrators validating deterministic compatibility.

This matrix summarizes core version anchors for deterministic compatibility in SealRun.

## At a glance

- Compatibility is contract-governed and version-explicit.
- Capsule, Why, and policy versions are stable integration anchors.
- Use doctor and distribution identity outputs for runtime validation.

| Component | Version |
|-----------|---------|
| SealRun product | `VERSION` file |
| CLI binary | `sealrun --version` |
| AI capsule schema | `version = "1"` |
| Why schema | `why_schema_version = "2"` |
| Governance policy version | `policy_version = "1"` |

## Upgrade guidance

- Minor/patch upgrades should preserve capsule `version = "1"` compatibility.
- If capsule version changes in future major versions, migration docs must include conversion tooling.

## CLI surface

```bash
sealrun --version
sealrun doctor
sealrun dist identity
```

## Enterprise-readiness

Compatibility is enterprise-ready when supported version combinations remain deterministic, documented, and test-covered.
