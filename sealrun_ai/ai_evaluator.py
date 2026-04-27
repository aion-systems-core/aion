from typing import Any, Dict, Literal

EvaluationLabel = Literal["ok", "drift", "error", "policy_violation", "unknown"]


class AITestEvaluator:
    """
    Bewertet SealRun-Ausführungen.
    Später durch ONNX-Modell ersetzbar.
    """

    def evaluate(self, result: Dict[str, Any]) -> EvaluationLabel:
        exit_code = result.get("exit_code", 0)
        stderr = (result.get("stderr") or "").lower()
        stdout = (result.get("stdout") or "").lower()
        drift = result.get("drift_score")
        policy = result.get("policy_violations", 0)
        evidence_count = result.get("evidence_count")

        # Hard error conditions first.
        if isinstance(evidence_count, int) and evidence_count < 0:
            return "error"

        if exit_code != 0 or "error" in stderr or "exception" in stderr:
            return "error"

        # Policy-related conditions.
        if "policy" in stderr and "violation" in stderr:
            return "policy_violation"

        if policy and policy > 0:
            return "policy_violation"

        if "policy_violation" in stdout:
            return "policy_violation"

        # Drift-related conditions.
        if "drift" in stderr and "violation" in stderr:
            return "drift"

        if drift is not None and drift > 0.1:
            return "drift"

        if "drift" in stdout and "score" in stdout:
            return "drift"

        # Evidence extraction anomalies.
        if "evidence" in stderr and ("missing" in stderr or "invalid" in stderr):
            return "error"

        return "ok"
