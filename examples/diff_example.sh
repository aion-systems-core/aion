#!/usr/bin/env bash
# AION Repro — diff two runs (requires two successful captures in the same cwd)
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
export PATH="${ROOT}/target/release:${ROOT}/target/debug:${PATH}"

WORKDIR="${WORKDIR:-$(mktemp -d 2>/dev/null || echo "${TMPDIR:-/tmp}/aion-repro-diff-demo")}"
mkdir -p "${WORKDIR}"
cd "${WORKDIR}"

echo "== Run 1 =="
aion repro run -- echo one

echo "== Run 2 =="
aion repro run -- echo two

echo "== Diff last vs previous =="
aion repro diff last prev
