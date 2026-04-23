# Installers and distribution

This guide maps local install paths to the deterministic distribution and trust model.

## At a glance

- Source install is available via Cargo.
- Packaging scaffolds exist for Homebrew/APT/RPM and container delivery.
- Distribution trust and support status are exposed through contract commands.

## Cargo install

```bash
cargo install --path crates/aion-cli
```

## Homebrew (tap draft)

`packaging/homebrew/aion.rb` contains a starter formula template.

## APT/RPM drafts

- APT metadata scaffold: `packaging/apt/`
- RPM spec scaffold: `packaging/rpm/aion.spec`

## Docker image

```bash
docker build -t aion-os:latest .
docker run --rm aion-os:latest --version
```

## CLI surface

```bash
aion --version
aion dist status
aion dist installers
aion dist lts
```

## Enterprise-readiness

Installer/distribution readiness requires signed/trusted artifact chains and explicit support/LTS status per channel.
