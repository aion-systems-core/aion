# Quickstart

This guide assumes you built the CLI (`cargo build -p aion-cli`) and invoke it with `sealrun …` or put `target/debug` on your `PATH`.

## At a glance

- Execute deterministic capsule
- Verify replay, policy, and doctor outputs
- Continue into enterprise command domains

## 1. Produce an AI capsule

```bash
sealrun execute ai --model demo --prompt "hello" --seed 1
```

**Output:** a timestamped directory under `sealrun_output/ai/<timestamp>/` containing `ai.json`, `ai.html`, `ai.svg`, `why.html`, `why.svg`, `capsule.sealrunai`, and `evidence.sealrunevidence`.

## 2. Replay the capsule

Use the `capsule.sealrunai` path printed in the log line `Output written to: …`:

```bash
sealrun execute ai-replay --capsule sealrun_output/ai/<timestamp>/capsule.sealrunai
```

**Output:** `sealrun_output/ai-replay/<timestamp>/` with replay report JSON/HTML/SVG and why-diff artefacts.

## 3. Validate against a governance policy

```bash
sealrun policy validate \
  --capsule sealrun_output/ai/<timestamp>/capsule.sealrunai \
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
