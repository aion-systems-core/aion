#!/usr/bin/env bash
# Build a deterministic AI capsule (writes under aion_output/ai/<timestamp>/).
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"
AION="${AION:-cargo run -q -p aion-cli --}"
exec $AION execute ai --model demo --prompt "hello from basic example" --seed 1001
