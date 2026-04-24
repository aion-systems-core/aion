#!/usr/bin/env bash
# Replay a capsule produced by run_ai.sh (pass path to capsule.aionai as $1).
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"
CAP="${1:?usage: $0 path/to/capsule.aionai}"
SEALRUN="${SEALRUN:-cargo run -q -p aion-cli --}"
exec $SEALRUN execute ai-replay --capsule "$CAP"
