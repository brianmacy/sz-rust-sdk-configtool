"""The binding's naming rule, restated independently of the generator."""

import keyword


def py_name(wire: str) -> str:
    """Manifest snake_case name -> Python parameter (keywords get ``_``)."""
    return f"{wire}_" if keyword.iskeyword(wire) or wire == "config_json" else wire
