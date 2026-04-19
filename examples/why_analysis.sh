#!/usr/bin/env bash
# AION Repro — env change → why (Unix-style env for the child process)
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
export PATH="${ROOT}/target/release:${ROOT}/target/debug:${PATH}"

WORKDIR="${WORKDIR:-$(mktemp -d 2>/dev/null || echo "${TMPDIR:-/tmp}/aion-repro-why-demo")}"
mkdir -p "${WORKDIR}"
cd "${WORKDIR}"

echo "== Capture with DEMO_VAR=foo =="
DEMO_VAR=foo aion repro run -- sh -c 'printf "%s\n" "${DEMO_VAR:-}"'

echo "== Capture with DEMO_VAR=bar =="
DEMO_VAR=bar aion repro run -- sh -c 'printf "%s\n" "${DEMO_VAR:-}"'

# Resolve the two run ids (oldest capture first, then the next)
LIST="${WORKDIR}/repro_runs/INDEX"
mapfile -t IDS < <(grep -v '^[[:space:]]*$' "${LIST}" || true)
ID_A="${IDS[0]:-}"
ID_B="${IDS[1]:-}"
if [[ -z "${ID_A}" || -z "${ID_B}" || "${ID_A}" == "${ID_B}" ]]; then
  echo "Could not read two distinct run ids:"
  cat "${LIST}" 2>/dev/null || true
  exit 1
fi

echo "== Diff ${ID_A} vs ${ID_B} =="
aion repro diff "${ID_A}" "${ID_B}"

echo "== Why ${ID_A} vs ${ID_B} =="
aion repro why "${ID_A}" "${ID_B}"
