"""High-level pilot API (dataclasses, deterministic JSON)."""

from __future__ import annotations

import json
from dataclasses import dataclass
from ctypes import byref, c_char_p, c_uint8, string_at

from . import c_abi
from .errors import AION_OK, AionError


@dataclass(frozen=True)
class RunResultView:
    stdout: str
    stderr: str
    exit_code: int
    duration_ms: int
    capsule_id: str


@dataclass(frozen=True)
class DriftView:
    changed: bool
    fields: list[str]


@dataclass(frozen=True)
class EvidenceView:
    valid: bool


class Pilot:
    """Root handle: loads the C-ABI once."""

    def __init__(self, lib_path: str | None = None) -> None:
        self._lib = c_abi.load_library(lib_path)
        c_abi.bind(self._lib)

    def execute_ai(self, *, model: str, prompt: str, seed: int, out_path: str) -> None:
        """Persist a deterministic AI capsule via ``aion_capsule_save`` (model, prompt, seed)."""
        m = c_char_p(model.encode("utf-8"))
        p = c_char_p(prompt.encode("utf-8"))
        cap = c_abi.AionCapsule(m, p, seed, None, None, None, None, None, None, None)
        path = c_char_p(out_path.encode("utf-8"))
        code = self._lib.aion_capsule_save(byref(cap), path)
        c_abi.check(code, self._lib)

    def replay(self, capsule_path: str) -> RunResultView:
        out = c_abi.AionRunResult()
        code = self._lib.aion_replay_capsule(c_char_p(capsule_path.encode("utf-8")), byref(out))
        try:
            c_abi.check(code, self._lib)
            stdout = (
                string_at(out.stdout_data, out.stdout_len).decode("utf-8", errors="replace")
                if out.stdout_data
                else ""
            )
            stderr = (
                string_at(out.stderr_data, out.stderr_len).decode("utf-8", errors="replace")
                if out.stderr_data
                else ""
            )
            cid = out.capsule_id.decode("utf-8", errors="replace") if out.capsule_id else ""
            return RunResultView(
                stdout=stdout,
                stderr=stderr,
                exit_code=int(out.exit_code),
                duration_ms=int(out.duration_ms),
                capsule_id=cid,
            )
        finally:
            self._lib.aion_free_run_result(byref(out))

    def drift(self, left_path: str, right_path: str) -> DriftView:
        rep = c_abi.AionDriftReport()
        code = self._lib.aion_drift_between_capsules(
            c_char_p(left_path.encode("utf-8")),
            c_char_p(right_path.encode("utf-8")),
            byref(rep),
        )
        try:
            c_abi.check(code, self._lib)
            fields: list[str] = []
            if rep.fields_json:
                fields = json.loads(rep.fields_json.decode("utf-8"))
            changed = bool(rep.changed)
            return DriftView(changed=changed, fields=fields)
        finally:
            if rep.fields_json:
                self._lib.aion_free_string(rep.fields_json)

    def evidence(self, evidence_path: str) -> EvidenceView:
        """Verify linear evidence chain JSON (``aion_evidence_verify``)."""
        ok = c_uint8(0)
        code = self._lib.aion_evidence_verify(c_char_p(evidence_path.encode("utf-8")), byref(ok))
        if code == AION_OK:
            return EvidenceView(valid=bool(ok.value))
        raise AionError(code, "evidence verify failed")
