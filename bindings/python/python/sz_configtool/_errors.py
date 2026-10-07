"""The single error class raised by every sz_configtool function."""

from __future__ import annotations


class SzConfigToolError(Exception):
    """A configuration operation failed.

    Attributes:
        reason_code: Stable classifier, one of ``REASON_CODES`` (for example
            ``NOT_FOUND`` or ``VALIDATION_ERRORS``). Branch on this, never on
            the message text.
        message: Human-readable description from the library.
        details: For ``VALIDATION_ERRORS``, a JSON string with schema
            ``sz-configtool.validation-errors/v1``; otherwise ``None``.
        kind: The error kind; identical to ``reason_code`` (the reason code
            IS the kind in every binding).
    """

    def __init__(
        self,
        reason_code: str,
        message: str,
        details: str | None = None,
    ) -> None:
        super().__init__(reason_code, message, details)
        self.reason_code = reason_code
        self.message = message
        self.details = details

    @property
    def kind(self) -> str:
        """The error kind: the same value as ``reason_code``."""
        return self.reason_code

    def __str__(self) -> str:
        return f"[{self.reason_code}] {self.message}"
