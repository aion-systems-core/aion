# Release attestation

SealRun enterprise release attestation integrates Cosign and cargo-sbom.

## Commands

```bash
sealrun enterprise release-attestation sign --artifact target/release/sealrun
sealrun enterprise release-attestation verify \
  --artifact target/release/sealrun \
  --signature sealrun.sig \
  --public-key cosign.pub
sealrun enterprise release-attestation sbom
```

## Notes

- `sign` and `verify` require `cosign` available in PATH.
- `sbom` requires `cargo-sbom` (`cargo sbom` command).
