import unittest

from sealrun_ai.config import AIConfig, OllamaConfig, SandboxConfig
from sealrun_ai.ai_generator import AITestGenerator
from sealrun_ai.fixtures import ensure_fixtures


class SealrunAITests(unittest.TestCase):
    def test_engine_tests_fallback_is_deterministic(self) -> None:
        cfg = AIConfig(
            ollama=OllamaConfig(base_url="http://127.0.0.1:9", model="qwen2.5:7b"),
            sandbox=SandboxConfig(sealrun_bin="sealrun"),
        )
        gen = AITestGenerator(cfg)
        tests = gen.generate_engine_tests(2)
        self.assertEqual(len(tests), 2)
        self.assertEqual(tests[0]["command"], "exec")
        self.assertEqual(tests[1]["command"], "doctor")

    def test_fixture_generation_creates_files(self) -> None:
        fixtures = ensure_fixtures()
        for _, path in fixtures.items():
            with open(path, "r", encoding="utf-8") as fh:
                self.assertTrue(len(fh.read()) > 0)


if __name__ == "__main__":
    unittest.main()
