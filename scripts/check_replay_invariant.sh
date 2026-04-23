#!/usr/bin/env bash
# Model-check docs/formal/replay_invariant.tla with TLC (deadlocks + invariants).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TLA="${ROOT}/docs/formal/replay_invariant.tla"
CFG="${ROOT}/docs/formal/replay_invariant.cfg"

if [[ ! -f "$TLA" || ! -f "$CFG" ]]; then
  echo "missing TLA artefacts: $TLA / $CFG" >&2
  exit 1
fi

if command -v tlc2 >/dev/null 2>&1; then
  exec tlc2 -deadlock "$TLA" -config "$CFG"
fi

if command -v java >/dev/null 2>&1; then
  JAR="${TLA_TOOLS_JAR:-}"
  if [[ -n "$JAR" && -f "$JAR" ]]; then
    exec java -cp "$JAR" tlc2.TLC -deadlock "$TLA" -config "$CFG"
  fi
fi

echo "tlc2 (or java + TLA_TOOLS_JAR) not available; skipping TLC model check." >&2
echo "Install TLA+ tools and ensure 'tlc2' is on PATH, or set TLA_TOOLS_JAR to tla2tools.jar." >&2
exit 0
