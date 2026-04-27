# Release zip — new customer ship test (CST) checklist

## Overview

Use this checklist when producing a **deterministic** release bundle (zip or tarball) for a pilot or customer ship test (CST). Align bundle contents with your internal release governance.

## Preconditions

- Release tag or commit hash is recorded in your change ticket.  
- `cargo test --workspace --all-targets` passed on the same commit.  
- SBOM and attestation steps (if applicable) are defined in [release-attestation.md](../../release-attestation.md).

## Bundle contents (minimum)

| Item | Included (Y/N) | Notes |
|------|----------------|-------|
| `sealrun` / CLI binary for target platform | | |
| Version string and build metadata | | |
| License file | | |
| README or pointer to `README_DEPLOY.md` (repository root) | | |
| Example capsules or scripts (if contractually allowed) | | |

## Determinism checks

| Check | Pass (Y/N) | Notes |
|-------|------------|-------|
| Same inputs produce same capsule hash on two machines | | |
| Replay matches baseline evidence | | |
| Drift report stable for negative control | | |

## Signing and integrity

| Step | Pass (Y/N) | Notes |
|------|------------|-------|
| Checksums published alongside bundle | | |
| Cosign / Sigstore signature (if in scope) | | |

## Handoff

- Recipient acknowledged secure transfer channel.  
- Recipient verified checksums before install.

## Next steps

After CST sign-off, attach this completed checklist to the pilot evaluation report.
