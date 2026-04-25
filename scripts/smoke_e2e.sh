#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

REPORT_PATH="${ROOT}/smoke_report.json"
LOG_DIR="${ROOT}/target/smoke_logs"
OUT_BASE="${ROOT}/target/smoke_output"
BIN="${ROOT}/target/release/sealrun"
if [ -x "${BIN}.exe" ]; then
  BIN="${BIN}.exe"
fi

mkdir -p "$LOG_DIR" "$OUT_BASE"
rm -rf "$OUT_BASE"
mkdir -p "$OUT_BASE"
rm -f "$REPORT_PATH"

export SEALRUN_OUTPUT_BASE="$OUT_BASE"
export AION_OUTPUT_BASE="$OUT_BASE"

if [ ! -x "$BIN" ]; then
  cargo build -p aion-cli --release >/dev/null 2>"$LOG_DIR/build.stderr.log"
  if [ -x "${ROOT}/target/release/sealrun.exe" ]; then
    BIN="${ROOT}/target/release/sealrun.exe"
  else
    BIN="${ROOT}/target/release/sealrun"
  fi
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

  run_cmd "$name" "execute_ai" "$BIN" --id "$id" execute ai --model demo --prompt smoke_one --seed 7 || {
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
  local name="s2_fixture_drift"
  local left="${OUT_BASE}/fixture_left_runresult.json"
  local right="${OUT_BASE}/fixture_right_runresult.json"

  cat >"$left" <<'EOF'
{"schema_version":1,"run_id":"left","command":"echo test","cwd":"/tmp","timestamp":1,"stdout":"alpha\n","stderr":"","exit_code":0,"duration_ms":1,"env_fingerprint":"env"}
EOF
  cat >"$right" <<'EOF'
{"schema_version":1,"run_id":"right","command":"echo test","cwd":"/tmp","timestamp":1,"stdout":"beta gamma\n","stderr":"","exit_code":0,"duration_ms":1,"env_fingerprint":"env"}
EOF

  run_cmd "$name" "drift" "$BIN" --id "smoke_s2_drift" observe drift "$left" "$right" || {
    append_scenario "$name" "FAIL" "observe drift failed"
    overall="FAIL"
    return
  }
  append_scenario "$name" "PASS" "runresult fixtures -> observe drift"
}

scenario_2_live_drift() {
  local name="s2_live_drift"
  local execute_id="smoke_s2_exec"
  local left_id="smoke_s2_live_left"
  local right_id="smoke_s2_live_right"
  local left_capture="${OUT_BASE}/capture/${left_id}/result.json"
  local right_capture="${OUT_BASE}/capture/${right_id}/result.json"
  local left_run="${OUT_BASE}/capture/${left_id}/runresult.json"
  local right_run="${OUT_BASE}/capture/${right_id}/runresult.json"

  run_cmd "$name" "execute_ai" "$BIN" --id "$execute_id" execute ai --model demo --prompt live_drift --seed 9 || {
    append_scenario "$name" "FAIL" "execute ai failed"
    overall="FAIL"
    return
  }
  run_cmd "$name" "capture_left" "$BIN" --id "$left_id" observe capture -- echo alpha || {
    append_scenario "$name" "FAIL" "capture left failed"
    overall="FAIL"
    return
  }
  run_cmd "$name" "capture_right" "$BIN" --id "$right_id" observe capture -- echo beta || {
    append_scenario "$name" "FAIL" "capture right failed"
    overall="FAIL"
    return
  }

  awk '
    /"data":[[:space:]]*\{/ {in_data=1; next}
    in_data && /^  \}/ {in_data=0; next}
    in_data {print}
  ' "$left_capture" | awk 'BEGIN{print "{"} {print} END{print "}"}' >"$left_run"
  awk '
    /"data":[[:space:]]*\{/ {in_data=1; next}
    in_data && /^  \}/ {in_data=0; next}
    in_data {print}
  ' "$right_capture" | awk 'BEGIN{print "{"} {print} END{print "}"}' >"$right_run"

  run_cmd "$name" "drift" "$BIN" --id "smoke_s2_live_drift" observe drift "$left_run" "$right_run" || {
    append_scenario "$name" "FAIL" "observe drift failed"
    overall="FAIL"
    return
  }
  append_scenario "$name" "PASS" "execute -> capture -> drift"
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
scenario_2_live_drift
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
