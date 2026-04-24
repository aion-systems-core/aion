#!/usr/bin/env bash
# Record a governance CI baseline JSON (keep output governance.json for ci_check).
# Usage: ci_baseline.sh path/to/capsule.aionai
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"
CAP="${1:?usage: $0 path/to/capsule.aionai}"
SEALRUN="${SEALRUN:-cargo run -q -p aion-cli --}"
exec $SEALRUN ci baseline \
  --capsule "$CAP" \
  --policy "$ROOT/examples/governance/dev.policy.json" \
  --determinism "$ROOT/examples/governance/dev.determinism.json" \
  --integrity "$ROOT/examples/governance/dev.integrity.json"
