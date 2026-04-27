import json
from pathlib import Path
from typing import Dict


def ensure_fixtures(base_dir: str = "./sealrun_ai/fixtures") -> Dict[str, str]:
    root = Path(base_dir)
    root.mkdir(parents=True, exist_ok=True)

    exec_capsule = root / "exec_capsule.json"
    replay_capsule = root / "replay_capsule.json"
    drift_left = root / "drift_left.json"
    drift_right = root / "drift_right.json"
    evidence_capsule = root / "evidence_capsule.json"
    policy_capsule = root / "policy_capsule.json"

    capsules = {
        "version": "1",
        "name": "sealrun_ai_fixture",
        "command": "exec",
        "args": ["--cmd", "echo hello"],
        "output": "hello",
        "evidence": {"trace_id": "fixture-trace", "policy_id": "fixture-policy"},
    }

    drift_left_payload = {"version": "1", "value": "left", "seed": 1}
    drift_right_payload = {"version": "1", "value": "right", "seed": 1}

    for pth in [exec_capsule, replay_capsule, evidence_capsule, policy_capsule]:
        pth.write_text(json.dumps(capsules, ensure_ascii=False, sort_keys=True), encoding="utf-8")

    drift_left.write_text(
        json.dumps(drift_left_payload, ensure_ascii=False, sort_keys=True), encoding="utf-8"
    )
    drift_right.write_text(
        json.dumps(drift_right_payload, ensure_ascii=False, sort_keys=True), encoding="utf-8"
    )

    return {
        "exec_capsule": exec_capsule.as_posix(),
        "replay_capsule": replay_capsule.as_posix(),
        "drift_left": drift_left.as_posix(),
        "drift_right": drift_right.as_posix(),
        "evidence_capsule": evidence_capsule.as_posix(),
        "policy_capsule": policy_capsule.as_posix(),
    }
