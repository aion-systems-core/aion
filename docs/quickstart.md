# Quickstart

This guide assumes you built the CLI (`cargo build -p aion-cli`) and invoke it with `cargo run -p aion-cli -- …` or put `target/debug` on your `PATH`.

## At a glance

- Execute deterministic capsule
- Verify replay, policy, and doctor outputs
- Continue into enterprise command domains

## 1. Produce an AI capsule

```bash
cargo run -p aion-cli -- execute ai --model demo --prompt "hello" --seed 1
```

**Output:** a timestamped directory under `aion_output/ai/<timestamp>/` containing `ai.json`, `ai.html`, `ai.svg`, `why.html`, `why.svg`, `capsule.aionai`, and `evidence.aionevidence`.

## 2. Replay the capsule

Use the `capsule.aionai` path printed in the log line `Output written to: …`:

```bash
cargo run -p aion-cli -- execute ai-replay --capsule aion_output/ai/<timestamp>/capsule.aionai
```

**Output:** `aion_output/ai-replay/<timestamp>/` with replay report JSON/HTML/SVG and why-diff artefacts.

## 3. Validate against a governance policy

```bash
cargo run -p aion-cli -- policy validate \
  --capsule aion_output/ai/<timestamp>/capsule.aionai \
  --policy examples/governance/dev.policy.json
```

**Output:** `aion_output/policy-validate/<timestamp>/governance.json` (+ HTML/SVG).

## 4. Check deterministic doctor surface

```bash
cargo run -p aion-cli -- doctor
```

## 5. Sample enterprise domain checks

```bash
cargo run -p aion-cli -- reliability status
cargo run -p aion-cli -- ops runbooks
cargo run -p aion-cli -- dist status
cargo run -p aion-cli -- governance status
cargo run -p aion-cli -- ux api
cargo run -p aion-cli -- tests strategy
cargo run -p aion-cli -- measure metrics
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
