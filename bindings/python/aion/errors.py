"""Deterministic error types for the pilot API."""

from __future__ import annotations


class AionError(Exception):
    """Structured failure from the native layer."""

    __slots__ = ("code",)

    def __init__(self, code: int, message: str) -> None:
        super().__init__(message)
        self.code = code

    def __str__(self) -> str:
        return f"AionError({self.code}): {self.args[0]}"


# Mirrors include/aion/aion.h
AION_OK = 0
AION_ERR_GENERIC = 1
AION_ERR_CAPSULE_NOT_FOUND = 2
AION_ERR_CAPSULE_CORRUPT = 3
AION_ERR_INVALID_POLICY = 4
AION_ERR_DETERMINISM_FAILURE = 5
AION_ERR_INTEGRITY_FAILURE = 6
AION_ERR_EVIDENCE_INVALID = 7
AION_ERR_IO = 8
AION_ERR_OUT_OF_MEMORY = 9
AION_ERR_UNSUPPORTED_VERSION = 10
