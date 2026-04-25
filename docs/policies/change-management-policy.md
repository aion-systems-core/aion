# Change Management Policy

## Purpose

Ensure product changes are planned, reviewed, tested, and traceable.

## Policy

- All changes must be linked to a tracked issue or request.
- Production-impacting changes require peer review.
- Releases require passing CI and deterministic contract checks.
- Release artifacts include changelog, attestation outputs, and rollback notes.
- Emergency changes require incident linkage and retrospective review within 5 business days.

## Evidence

- CI logs
- release notes/changelog
- attestation outputs (`sign`, `verify`, `sbom`)
