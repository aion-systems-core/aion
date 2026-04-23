"""ctypes smoke test against the AION C-ABI (standalone from the wheel)."""

from __future__ import annotations

import os
import sys
from ctypes import POINTER, Structure, byref, c_char_p, c_int32, c_size_t, c_uint64, c_uint8

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", ".."))


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


def load():
    base = os.environ.get("AION_LIB_PATH", os.path.join(ROOT, "target", "debug"))
    names = ["aion_engine.dll", "libaion_engine.so", "libaion_engine.dylib"]
    for n in names:
        p = os.path.join(base, n)
        if os.path.isfile(p):
            return __import__("ctypes").CDLL(p)
    raise SystemExit("native library not found; set AION_LIB_PATH")


def main() -> int:
    lib = load()
    lib.aion_capsule_save.argtypes = [POINTER(AionCapsule), c_char_p]
    lib.aion_capsule_save.restype = c_int32
    lib.aion_replay_capsule.argtypes = [c_char_p, POINTER(AionRunResult)]
    lib.aion_replay_capsule.restype = c_int32
    lib.aion_free_run_result.argtypes = [POINTER(AionRunResult)]
    lib.aion_free_run_result.restype = None
    lib.aion_last_error.restype = c_char_p

    out = os.path.join(os.path.dirname(__file__), "showcase_capsule_py.aionai")
    cap = AionCapsule(
        c_char_p(b"demo"),
        c_char_p(b"hello"),
        11,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
    code = lib.aion_capsule_save(byref(cap), c_char_p(out.encode()))
    if code != 0:
        print(lib.aion_last_error().decode(), file=sys.stderr)
        return 1
    print("capsule_save ok")
    rr = AionRunResult()
    code = lib.aion_replay_capsule(c_char_p(out.encode()), byref(rr))
    if code != 0:
        print(lib.aion_last_error().decode(), file=sys.stderr)
        return 1
    print("replay ok exit", rr.exit_code)
    lib.aion_free_run_result(byref(rr))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
