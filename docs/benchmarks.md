# Benchmarks

## Purpose

How CI runs **benchmark.yml** as a smoke performance lane—separate from correctness gates in `ci.yml`.

This repository includes a benchmark workflow (`.github/workflows/benchmark.yml`) for smoke performance checks.

## At a glance

- Benchmarks are operational signals, not replacements for determinism checks.
- Use deterministic seeds and stable commands for comparability.
- Track benchmark outcomes alongside contract-readiness outputs.

Suggested local benchmark command:

```bash
sealrun release -- execute ai --model demo --prompt "benchmark" --seed 1
```

## CLI surface

```bash
sealrun execute ai --model demo --prompt "benchmark" --seed 1
sealrun doctor
sealrun measure metrics
```

## Enterprise-readiness

Benchmark practices are enterprise-ready when performance tracking is paired with deterministic replay/governance/measurement evidence.
