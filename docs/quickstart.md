# Quickstart

## Purpose

Shortest path from **clone** to **capsule**, **replay symmetry**, **drift** on `RunResult` JSON, **policy validation**, and **`doctor`**—with paths that match the engine’s on-disk names.

This guide assumes you built the CLI (`cargo build -p aion-cli`) and invoke it with `sealrun …` or put `target/debug` on your `PATH`.

## At a glance

- Execute deterministic capsule
- Verify replay, policy, and doctor outputs
- Continue into enterprise command domains

## 1. Produce an AI capsule

```bash
sealrun execute ai --model demo --prompt "hello" --seed 1
```

**Output:** a directory under `<output_base>/ai/<run_id>/` (default base may be `aion_output`; set `SEALRUN_OUTPUT_BASE` or legacy `AION_OUTPUT_BASE` for `sealrun_output/`) containing `ai.json`, `ai.html`, `ai.svg`, `why.html`, `why.svg`, **`capsule.aionai`**, and **`*.aionevidence`** sidecars.

## 2. Replay the capsule

Use the **`capsule.aionai`** path under the printed output directory (`Output written to: …`):

```bash
sealrun execute ai-replay --capsule <output_base>/ai/<run_id>/capsule.aionai
```

**Output:** `sealrun_output/ai-replay/<timestamp>/` with replay report JSON/HTML/SVG and why-diff artefacts.

## 3. Validate against a governance policy

```bash
sealrun policy validate \
  --capsule <output_base>/ai/<run_id>/capsule.aionai \
  --policy examples/governance/dev.policy.json
```

**Output:** `sealrun_output/policy-validate/<timestamp>/governance.json` (+ HTML/SVG).

## 4. Check deterministic doctor surface

```bash
sealrun doctor
```

## 5. Sample enterprise domain checks

```bash
sealrun reliability status
sealrun ops runbooks
sealrun dist status
sealrun governance status
sealrun ux api
sealrun tests strategy
sealrun measure metrics
```

## Example JSON (truncated capsule)

Capsules are JSON-compatible records. A minimal illustration (fields vary by run):

```json
{
  "version": "1",
  "model": "demo",
  "prompt": "hello",
  "seed": 1,
  "tokens": ["…"]
}
```

## Where to go next

- [Capsules](capsules.md) — what fields mean at a product level  
- [Replay](replay.md) — interpreting replay output  
- [SDK](sdk.md) — automation without the interactive CLI  
- [CLI reference](cli-reference.md) — full deterministic command surface
- [Developer guide](developer-guide.md) — replay, drift, evidence, and identity workflows
