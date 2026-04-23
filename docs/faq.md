# FAQ

## At a glance

AION-OS is an Execution-OS with deterministic contracts for replay, drift, evidence, governance, and enterprise readiness.

## What is AION‑OS?

AION‑OS is a deterministic AI Execution-OS and Contract-OS surface that emits reproducible artefacts for replay, drift, evidence, governance, operations, distribution, testing, and measurement workflows.

## Does AION‑OS require a hosted service?

No. The repository runs locally and writes artefacts to your filesystem.

## How do I verify determinism?

Run `execute ai`, then `execute ai-replay` on the generated capsule, and validate `aion doctor` plus `aion tests strategy` outputs.

## How do I enforce policies in CI?

Use policy and governance commands (`aion policy packs`, `aion policy gates`, `aion policy evidence`, `aion governance status`) and include them in CI evidence bundles.

## Does telemetry run by default?

No. Telemetry is opt-in; this repository defaults to local-only execution and file outputs.

## Which enterprise command domains are canonical?

Use the seven deterministic domains: `reliability`, `ops`, `dist`, `governance`, `ux`, `tests`, `measure`.
