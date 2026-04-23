"""ctypes bindings to the AION C-ABI (no PyO3)."""

from __future__ import annotations

import ctypes
import os
import sys
from ctypes import POINTER, Structure, c_char_p, c_int32, c_size_t, c_uint8, c_uint64

from .errors import AION_OK, AionError


class AionRunResult(Structure):
    _fields_ = [
        ("stdout_data", c_char_p),
        ("stdout_len", c_size_t),
        ("stderr_data", c_char_p),
        ("stderr_len", c_size_t),
        ("exit_code", c_int32),
        ("duration_ms", c_uint64),
        ("capsule_id", c_char_p),
    ]


class AionCapsule(Structure):
    _fields_ = [
        ("model", c_char_p),
        ("prompt", c_char_p),
        ("seed", c_uint64),
        ("determinism_profile_json", c_char_p),
        ("token_trace_json", c_char_p),
        ("events_json", c_char_p),
        ("graph_json", c_char_p),
        ("why_report_json", c_char_p),
        ("drift_report_json", c_char_p),
        ("evidence_path", c_char_p),
    ]


class AionReplayComparison(Structure):
    _fields_ = [
        ("tokens_equal", c_uint8),
        ("trace_equal", c_uint8),
        ("events_equal", c_uint8),
        ("graph_equal", c_uint8),
        ("why_equal", c_uint8),
        ("drift_equal", c_uint8),
        ("capsule_equal", c_uint8),
        ("evidence_equal", c_uint8),
        ("differences", POINTER(c_char_p)),
        ("differences_count", c_size_t),
    ]


class AionDriftReport(Structure):
    _fields_ = [
        ("changed", c_uint8),
        ("fields_json", c_char_p),
    ]


def _default_lib_names() -> list[str]:
    if sys.platform == "win32":
        return ["aion_engine.dll", "libaion_engine.dll"]
    if sys.platform == "darwin":
        return ["libaion_engine.dylib"]
    return ["libaion_engine.so"]


def load_library(explicit: str | None = None) -> ctypes.CDLL:
    """Load the engine cdylib. Set AION_LIB_PATH to a directory or file path."""
    if explicit:
        return ctypes.CDLL(explicit)
    env = os.environ.get("AION_LIB_PATH", "").strip()
    if env:
        p = os.path.abspath(env)
        if os.path.isdir(p):
            for name in _default_lib_names():
                fp = os.path.join(p, name)
                if os.path.isfile(fp):
                    return ctypes.CDLL(fp)
        if os.path.isfile(p):
            return ctypes.CDLL(p)
    here = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", ".."))
    candidates = [
        os.path.join(here, "target", "debug"),
        os.path.join(here, "target", "release"),
    ]
    for base in candidates:
        for name in _default_lib_names():
            fp = os.path.join(base, name)
            if os.path.isfile(fp):
                return ctypes.CDLL(fp)
    names = ", ".join(_default_lib_names())
    raise OSError(f"AION native library not found ({names}). Set AION_LIB_PATH.")


def bind(lib: ctypes.CDLL) -> None:
    lib.aion_last_error.argtypes = []
    lib.aion_last_error.restype = c_char_p

    lib.aion_free_string.argtypes = [c_char_p]
    lib.aion_free_string.restype = None

    lib.aion_free_run_result.argtypes = [POINTER(AionRunResult)]
    lib.aion_free_run_result.restype = None

    lib.aion_capsule_save.argtypes = [POINTER(AionCapsule), c_char_p]
    lib.aion_capsule_save.restype = c_int32

    lib.aion_replay_capsule.argtypes = [c_char_p, POINTER(AionRunResult)]
    lib.aion_replay_capsule.restype = c_int32

    lib.aion_drift_between_capsules.argtypes = [c_char_p, c_char_p, POINTER(AionDriftReport)]
    lib.aion_drift_between_capsules.restype = c_int32

    lib.aion_evidence_verify.argtypes = [c_char_p, POINTER(c_uint8)]
    lib.aion_evidence_verify.restype = c_int32


def check(code: int, lib: ctypes.CDLL) -> None:
    if code == AION_OK:
        return
    raw = lib.aion_last_error()
    msg = raw.decode("utf-8", errors="replace") if raw else "unknown error"
    raise AionError(code, msg)
