#!/usr/bin/env bash
# AION Repro — first capture + replay (build: cargo build -p aion -p repro from repo root)
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
export PATH="${ROOT}/target/release:${ROOT}/target/debug:${PATH}"

echo "== Capture a run =="
aion repro run -- echo "hello from AION Repro"

echo "== Replay stdout from the latest run =="
aion repro replay last
