#!/usr/bin/env bash
# Observe drift between two RunResult JSON files (kernel run artefacts).
# Usage: drift.sh left.json right.json
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"
L="${1:?left.json}"
R="${2:?right.json}"
AION="${AION:-cargo run -q -p aion-cli --}"
exec $AION observe drift "$L" "$R"
