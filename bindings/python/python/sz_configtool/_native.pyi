"""Type stub for the Rust extension (``bindings/python/src/lib.rs``)."""

LIBRARY_VERSION: str
ABI_VERSION: int

class NativeError(Exception):
    """args = (reason_code, message, details_json | None)."""

def invoke(
    name: str, config: str, args_json: str
) -> tuple[str, str | None, str | None]: ...
