#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

REPORT_PATH="${ROOT}/smoke_report.json"
LOG_DIR="${ROOT}/target/smoke_logs"
OUT_BASE="${ROOT}/target/smoke_output"
BIN="${ROOT}/target/release/sealrun"

mkdir -p "$LOG_DIR" "$OUT_BASE"
rm -f "$REPORT_PATH"

export SEALRUN_OUTPUT_BASE="$OUT_BASE"
export AION_OUTPUT_BASE="$OUT_BASE"

if [ ! -x "$BIN" ]; then
  cargo build -p aion-cli --release >/dev/null 2>"$LOG_DIR/build.stderr.log"
fi

overall="PASS"
scenario_records=""

run_cmd() {
  local scenario="$1"
  local step="$2"
  shift 2
  local log_prefix="${LOG_DIR}/${scenario}_${step}"
  if "$@" >"${log_prefix}.stdout.log" 2>"${log_prefix}.stderr.log"; then
    return 0
  fi
  return 1
}

append_scenario() {
  local name="$1"
  local status="$2"
  local details="$3"
  local record
  record="{\"name\":\"${name}\",\"status\":\"${status}\",\"details\":\"${details}\"}"
  if [ -z "$scenario_records" ]; then
    scenario_records="$record"
  else
    scenario_records="${scenario_records},${record}"
  fi
}

scenario_1() {
  local name="s1_execute_replay_policy"
  local id="smoke_s1_ai"
  local capsule="${OUT_BASE}/ai/${id}/capsule.aionai"

  run_cmd "$name" "execute_ai" "$BIN" --id "$id" execute ai --model demo --prompt "smoke one" --seed 7 || {
    append_scenario "$name" "FAIL" "execute ai failed"
    overall="FAIL"
    return
  }
  run_cmd "$name" "execute_ai_replay" "$BIN" --id "${id}_replay" execute ai-replay --capsule "$capsule" || {
    append_scenario "$name" "FAIL" "execute ai-replay failed"
    overall="FAIL"
    return
  }
  run_cmd "$name" "policy_evidence" "$BIN" --id "${id}_policy" policy evidence || {
    append_scenario "$name" "FAIL" "policy evidence failed"
    overall="FAIL"
    return
  }
  append_scenario "$name" "PASS" "execute -> replay -> policy evidence"
}

scenario_2() {
  local name="s2_capture_drift"
  local left_id="smoke_s2_left"
  local right_id="smoke_s2_right"
  local left="${OUT_BASE}/capture/${left_id}/result.json"
  local right="${OUT_BASE}/capture/${right_id}/result.json"

  run_cmd "$name" "capture_left" "$BIN" --id "$left_id" observe capture -- echo alpha || {
    append_scenario "$name" "FAIL" "left capture failed"
    overall="FAIL"
    return
  }
  run_cmd "$name" "capture_right" "$BIN" --id "$right_id" observe capture -- echo beta || {
    append_scenario "$name" "FAIL" "right capture failed"
    overall="FAIL"
    return
  }
  run_cmd "$name" "drift" "$BIN" --id "smoke_s2_drift" observe drift "$left" "$right" || {
    append_scenario "$name" "FAIL" "observe drift failed"
    overall="FAIL"
    return
  }
  append_scenario "$name" "PASS" "capture left/right -> observe drift"
}

scenario_3() {
  local name="s3_evidence_governance_doctor"

  run_cmd "$name" "policy_evidence" "$BIN" --id "smoke_s3_pol_evidence" policy evidence || {
    append_scenario "$name" "FAIL" "policy evidence failed"
    overall="FAIL"
    return
  }
  run_cmd "$name" "governance_status" "$BIN" --id "smoke_s3_gov" governance status || {
    append_scenario "$name" "FAIL" "governance status failed"
    overall="FAIL"
    return
  }
  run_cmd "$name" "doctor" "$BIN" --id "smoke_s3_doc" doctor || {
    append_scenario "$name" "FAIL" "doctor failed"
    overall="FAIL"
    return
  }
  append_scenario "$name" "PASS" "policy evidence -> governance status -> doctor"
}

scenario_1
scenario_2
scenario_3

timestamp="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
printf '{\n  "status": "%s",\n  "generated_at_utc": "%s",\n  "scenarios": [%s]\n}\n' \
  "$overall" "$timestamp" "$scenario_records" >"$REPORT_PATH"

if [ "$overall" = "PASS" ]; then
  echo "PASS"
  exit 0
fi

echo "FAIL"
exit 1
