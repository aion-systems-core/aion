# FAQ

## At a glance

SealRun is an deterministic execution engine with deterministic contracts for replay, drift, evidence, governance, and enterprise readiness.

## What is SealRun?

SealRun is a deterministic AI execution engine and contract layer surface that emits reproducible artefacts for replay, drift, evidence, governance, operations, distribution, testing, and measurement workflows.

## Does SealRun require a hosted service?

No. The repository runs locally and writes artefacts to your filesystem.

## How do I verify determinism?

Run `execute ai`, then `execute ai-replay` on the generated capsule, and validate `sealrun doctor` plus `sealrun tests strategy` outputs.

## How do I enforce policies in CI?

Use policy and governance commands (`sealrun policy packs`, `sealrun policy gates`, `sealrun policy evidence`, `sealrun governance status`) and include them in CI evidence bundles.

## Does telemetry run by default?

No. Telemetry is opt-in; this repository defaults to local-only execution and file outputs.

## Which enterprise command domains are canonical?

Use the seven deterministic domains: `reliability`, `ops`, `dist`, `governance`, `ux`, `tests`, `measure`.
