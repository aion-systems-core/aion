#!/usr/bin/env bash
# CI-only: compile and run SDK examples (set AION_PRODUCT_TESTS=1).
set -euo pipefail
if [ "${AION_PRODUCT_TESTS:-}" != "1" ]; then
  echo "skip test_examples_run (set AION_PRODUCT_TESTS=1 to run)"
  exit 0
fi
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
cargo check -p aion-cli --examples
for ex in sdk_capsule_build sdk_replay sdk_drift sdk_governance; do
  cargo run -q -p aion-cli --example "$ex"
done
bash "$ROOT/examples/basic/run_ai.sh"
echo "examples run: ok"
