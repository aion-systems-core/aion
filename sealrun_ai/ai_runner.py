import json
import subprocess
from typing import Any, Dict, List
from .config import AIConfig
from .ai_generator import AITestGenerator
from .ai_evaluator import AITestEvaluator
from .fixtures import ensure_fixtures


class AITestRunner:
    """
    Führt AI-generierte Tests deterministisch über SealRun aus.
    """

    COMMAND_MAP = {
        "exec": ["execute"],
        "replay": ["replay"],
        "drift": ["observe", "drift"],
        "evidence": ["evidence", "export"],
        "policy": ["policy", "validate"],
        "doctor": ["doctor"],
    }

    def __init__(self, config: AIConfig = AIConfig()) -> None:
        self.config = config
        self.generator = AITestGenerator(config)
        self.evaluator = AITestEvaluator()

    def _map_command(self, command: str, args: List[str]) -> List[str]:
        """
        Deterministic mapping from allowed high-level commands
        to real SealRun CLI subcommands.
        """
        if command not in self.COMMAND_MAP:
            raise ValueError(f"Invalid command: {command}")

        mapped = self.COMMAND_MAP[command]
        return mapped + args

    def _run_sealrun(self, args: List[str]) -> Dict[str, Any]:
        if not args:
            return {
                "exit_code": 2,
                "stdout": "",
                "stderr": "missing_command",
            }
        mapped_args = self._map_command(args[0], args[1:])
        cmd = [self.config.sandbox.sealrun_bin] + mapped_args
        try:
            proc = subprocess.Popen(
                cmd,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
            )
            stdout, stderr = proc.communicate()
            return {
                "exit_code": proc.returncode,
                "stdout": stdout,
                "stderr": stderr,
            }
        except FileNotFoundError as exc:
            return {
                "exit_code": 127,
                "stdout": "",
                "stderr": str(exc),
            }

    @staticmethod
    def _is_valid_test(test: Dict[str, Any]) -> bool:
        allowed_commands = {"exec", "replay", "drift", "evidence", "policy", "doctor"}
        allowed_flags = {"--cmd", "--capsule", "--left", "--right"}

        command = test.get("command")
        args = test.get("args", [])
        if command not in allowed_commands:
            return False
        if not isinstance(args, list):
            return False

        for arg in args:
            if not isinstance(arg, str):
                return False
            if arg.startswith("--") and arg not in allowed_flags:
                return False
        return True

    def engine_selftest(self, count: int = 20) -> List[Dict[str, Any]]:
        tests = self.generator.generate_engine_tests(count)
        results = []

        for t in tests:
            if not self._is_valid_test(t):
                results.append(
                    {
                        "test": t,
                        "result": {
                            "exit_code": 2,
                            "stdout": "",
                            "stderr": "invalid_test_spec",
                        },
                        "label": "error",
                    }
                )
                continue
            run = self._run_sealrun([t["command"]] + t.get("args", []))
            label = self.evaluator.evaluate(run)
            results.append({"test": t, "result": run, "label": label})

        return results

    def pipeline_test(self, pipeline: Dict[str, Any], count: int = 20) -> List[Dict[str, Any]]:
        fixtures = ensure_fixtures()

        # Deterministic pipeline variants for replay/drift/policy/evidence checks.
        variants: List[Dict[str, Any]] = [
            {
                "name": "pipeline_exec_variant_1",
                "description": "Deterministic exec variant from pipeline payload",
                "command": "exec",
                "args": ["--cmd", f"echo pipeline:{json.dumps(pipeline, sort_keys=True)}"],
            },
            {
                "name": "pipeline_replay_variant_1",
                "description": "Replay deterministic fixture capsule",
                "command": "replay",
                "args": ["--capsule", fixtures["replay_capsule"]],
            },
            {
                "name": "pipeline_drift_variant_1",
                "description": "Drift comparison on deterministic fixture pair",
                "command": "drift",
                "args": ["--left", fixtures["drift_left"], "--right", fixtures["drift_right"]],
            },
            {
                "name": "pipeline_evidence_variant_1",
                "description": "Evidence extraction on deterministic fixture capsule",
                "command": "evidence",
                "args": ["--capsule", fixtures["evidence_capsule"]],
            },
            {
                "name": "pipeline_policy_variant_1",
                "description": "Policy validation on deterministic fixture capsule",
                "command": "policy",
                "args": ["validate", "--capsule", fixtures["policy_capsule"]],
            },
            {
                "name": "pipeline_doctor_variant_1",
                "description": "Doctor deterministic health check",
                "command": "doctor",
                "args": [],
            },
        ]

        selected = variants[: max(0, min(count, len(variants)))]
        results = []
        for t in selected:
            if not self._is_valid_test(t):
                results.append(
                    {
                        "test": t,
                        "result": {
                            "exit_code": 2,
                            "stdout": "",
                            "stderr": "invalid_test_spec",
                        },
                        "label": "error",
                    }
                )
                continue
            run = self._run_sealrun([t["command"]] + t.get("args", []))
            label = self.evaluator.evaluate(run)
            results.append({"test": t, "result": run, "label": label})

        return results

