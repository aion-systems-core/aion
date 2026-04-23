import ctypes
from ctypes import c_char_p, c_int32, c_size_t, c_uint8, c_uint64, Structure


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


lib = ctypes.CDLL("libaion.so")
lib.aion_last_error.restype = c_char_p


def _check(code: int):
    if code != 0:
        err = lib.aion_last_error()
        raise RuntimeError(err.decode("utf-8") if err else "aion error")

