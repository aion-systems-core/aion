#!/usr/bin/env bash
# Check a capsule against a saved baseline governance.json.
# Usage: ci_check.sh path/to/capsule.aionai path/to/baseline-governance.json
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"
CAP="${1:?usage: $0 capsule.aionai baseline-governance.json}"
BASE="${2:?baseline json}"
AION="${AION:-cargo run -q -p aion-cli --}"
exec $AION ci check --capsule "$CAP" --baseline "$BASE"
