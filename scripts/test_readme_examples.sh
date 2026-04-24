#!/usr/bin/env bash
# Smoke commands aligned with README (version, AI capsule, policy validate).
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
SEALRUN=(cargo run -q -p aion-cli --)
POL="$ROOT/examples/governance/dev.policy.json"

"${SEALRUN[@]}" --version
LOG="$("${SEALRUN[@]}" execute ai --model demo --prompt "readme smoke" --seed 4242 2>&1)"
DIR="$(printf '%s\n' "$LOG" | sed -n 's/.*Output written to: //p' | tail -n1 | tr -d '\r')"
test -n "$DIR"
CAP="$(find "$DIR" -name 'capsule.aionai' -print -quit)"
test -n "$CAP"
"${SEALRUN[@]}" policy validate --capsule "$CAP" --policy "$POL"
echo "readme examples: ok"
