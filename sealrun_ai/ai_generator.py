import json
import time
import requests
from typing import Any, Dict, List
from .config import AIConfig


FALLBACK_ENGINE_TESTS: List[Dict[str, Any]] = [
    {
        "name": "dummy_exec",
        "description": "Deterministic fallback exec smoke test",
        "command": "exec",
        "args": ["--cmd", "echo hello"],
    },
    {
        "name": "dummy_doctor",
        "description": "Deterministic fallback doctor smoke test",
        "command": "doctor",
        "args": [],
    },
]


class AITestGenerator:
    """
    LLM-basierter Testfall-Generator.
    - Engine-Selftests
    - Kunden-Pipeline-Tests
    """

    def __init__(self, config: AIConfig) -> None:
        self.config = config

    def healthcheck(self) -> bool:
        tags_url = f"{self.config.ollama.base_url}/api/tags"
        try:
            resp = requests.get(tags_url, timeout=5)
            return resp.status_code == 200
        except requests.exceptions.RequestException:
            return False

    def _ollama(self, prompt: str) -> str:
        if not self.healthcheck():
            return "[]"

        url = f"{self.config.ollama.base_url}/api/generate"
        payload = {
            "model": self.config.ollama.model,
            "prompt": prompt,
            "temperature": 0.0,
            "stream": False,
        }

        # Deterministic retry/backoff profile.
        # attempts: [180s, 30s, 5s]
        for idx, timeout_s in enumerate([180, 30, 5]):
            try:
                r = requests.post(url, json=payload, timeout=timeout_s)
                r.raise_for_status()
                return r.json().get("response", "")
            except requests.exceptions.ReadTimeout:
                if idx == 2:
                    return "[]"
                time.sleep(1)
            except requests.exceptions.RequestException:
                if idx == 2:
                    return "[]"
                time.sleep(1)
        return "[]"

    def generate_engine_tests(self, count: int = 20) -> List[Dict[str, Any]]:
        prompt = f"""
You are a JSON-only generator.
You MUST output ONLY valid JSON.
NO text, NO markdown, NO explanation.

Generate EXACTLY {count} test cases for the SealRun deterministic execution engine.

VALID SealRun commands are ONLY:
- exec
- replay
- drift
- evidence
- policy
- doctor

For each test case, output a JSON object with:
- "name": string
- "description": string
- "command": one of the VALID commands above
- "args": array of strings (ONLY valid SealRun flags)

VALID FLAG EXAMPLES:
- exec: ["--cmd", "echo hello"]
- replay: ["--capsule", "./capsules/sample.json"]
- drift: ["--left", "./capsules/a.json", "--right", "./capsules/b.json"]
- evidence: ["--capsule", "./capsules/sample.json"]
- policy: ["validate", "--capsule", "./capsules/sample.json"]
- doctor: []

Constraints:
- Use only relative example paths shown above.
- Do not reference absolute system paths.
- Do not include unsupported commands or flags.

Output format:
[
  {{"name": "...", "description": "...", "command": "...", "args": ["..."]}},
  ...
]

Return ONLY the JSON array.
"""
        tests = self._parse(self._ollama(prompt))
        if tests:
            return tests
        return self._fallback_engine_tests(count)

    def generate_pipeline_tests(self, pipeline: Dict[str, Any], count: int = 20) -> List[Dict[str, Any]]:
        prompt = f"""
Given this pipeline spec:

{json.dumps(pipeline, indent=2)}

Generate {count} JSON test cases that:
- simulate drift
- create edge cases
- break inputs
- violate policies

Each test must include:
- name
- description
- input_overrides (JSON object)
- expected_risk ("ok"|"drift"|"error"|"policy_violation")

Return ONLY a JSON array.
"""
        return self._parse(self._ollama(prompt))

    @staticmethod
    def _fallback_engine_tests(count: int) -> List[Dict[str, Any]]:
        if count <= 0:
            return []
        out: List[Dict[str, Any]] = []
        idx = 0
        while len(out) < count:
            template = FALLBACK_ENGINE_TESTS[idx % len(FALLBACK_ENGINE_TESTS)]
            out.append(
                {
                    "name": f"{template['name']}_{len(out)+1}",
                    "description": template["description"],
                    "command": template["command"],
                    "args": list(template["args"]),
                }
            )
            idx += 1
        return out

    @staticmethod
    def _parse(raw: str) -> List[Dict[str, Any]]:
        try:
            data = json.loads(raw)
            return data if isinstance(data, list) else []
        except:
            return []