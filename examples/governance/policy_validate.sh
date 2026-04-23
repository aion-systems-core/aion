#!/usr/bin/env bash
# Validate a capsule against a governance policy JSON.
# Usage: policy_validate.sh path/to/capsule.aionai
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"
CAP="${1:?usage: $0 path/to/capsule.aionai}"
POL="$ROOT/examples/governance/dev.policy.json"
AION="${AION:-cargo run -q -p aion-cli --}"
exec $AION policy validate --capsule "$CAP" --policy "$POL"
