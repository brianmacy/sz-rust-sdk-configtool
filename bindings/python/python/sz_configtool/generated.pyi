# GENERATED — do not edit. Source: api/manifest/*.yaml; regenerate with `cargo run -p sz-configtool-codegen`.
"""Typed configuration functions, one per manifest function."""

from typing import Any, NamedTuple

from ._core import UnsetType

REASON_CODES: tuple[str, ...]
"""Every ``SzConfigToolError.reason_code`` value."""

class SetGenericPlanRecord(NamedTuple):
    """Result of ``set_generic_plan_result``; every named field is JSON text."""

    plan_id: str
    """``plan_id`` as JSON text."""
    was_created: str
    """``was_created`` as JSON text."""

class VerifyCompatibilityVersionRecord(NamedTuple):
    """Result of ``verify_compatibility_version``; every named field is JSON text."""

    current_version: str
    """``current_version`` as JSON text."""
    matches: str
    """``matches`` as JSON text."""

def add_attribute(
    config_json: str,
    attribute: str,
    feature: str,
    element: str,
    class_: str,
    *,
    default_value: str | None = None,
    internal: str | None = None,
    required: str | None = None,
    id: int | None = None,
) -> str:
    """Add an attribute (CFG_ATTR row) mapping an input attribute to a feature element.

    Args:
        config_json: The configuration JSON (opaque).
        attribute: Uppercased before storage and duplicate check.
        feature: Must name an existing CFG_FTYPE (case-insensitive) or NOT_FOUND; stored
            uppercased in FTYPE_CODE.
        element: Must name an existing CFG_FELEM (case-insensitive) or NOT_FOUND; stored
            uppercased in FELEM_CODE.
        class_: CASE-SENSITIVE (not uppercased): must be exactly one of NAME, ATTRIBUTE,
            IDENTIFIER, ADDRESS, PHONE, RELATIONSHIP, OTHER, else INVALID_INPUT. Wire
            name ``class``.
        default_value: Absent stores DEFAULT_VALUE null; any string (including "") is
            stored verbatim. ``None`` omits it.
        internal: Case-insensitive; normalized to Yes or No, else INVALID_INPUT.
            ``None`` omits it. Library default when omitted: ``"No"``.
        required: Case-insensitive; normalized to Yes, No, Any or Desired (stored in
            FELEM_REQ), else INVALID_INPUT. ``None`` omits it. Library default when
            omitted: ``"No"``.
        id: Requested ATTR_ID. Absent OR <= 0 means auto-allocate (max existing + 1,
            floor 1000). A taken id > 0 is ALREADY_EXISTS. ``None`` omits it.

    Returns:
        The modified configuration JSON string. ``add_attribute_result`` (same
        arguments) returns the record this operation produces.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, ALREADY_EXISTS,
            INVALID_INPUT, NOT_FOUND; any call may also raise INVALID_INPUT (bad
            arguments) or INTERNAL.

    Notes:
        Returns (modified config, the new CFG_ATTR row). Validation order: class,
        duplicate attribute, feature, element, required, internal, id. Does not create a
        CFG_FBOM row.
    """

def add_attribute_result(
    config_json: str,
    attribute: str,
    feature: str,
    element: str,
    class_: str,
    *,
    default_value: str | None = None,
    internal: str | None = None,
    required: str | None = None,
    id: int | None = None,
) -> str:
    """The record (row / ids) of ``add_attribute``: same arguments and operation, but
    returns the record instead of the configuration. Operation: Add an attribute
    (CFG_ATTR row) mapping an input attribute to a feature element.

    Args:
        config_json: The configuration JSON (opaque).
        attribute: Uppercased before storage and duplicate check.
        feature: Must name an existing CFG_FTYPE (case-insensitive) or NOT_FOUND; stored
            uppercased in FTYPE_CODE.
        element: Must name an existing CFG_FELEM (case-insensitive) or NOT_FOUND; stored
            uppercased in FELEM_CODE.
        class_: CASE-SENSITIVE (not uppercased): must be exactly one of NAME, ATTRIBUTE,
            IDENTIFIER, ADDRESS, PHONE, RELATIONSHIP, OTHER, else INVALID_INPUT. Wire
            name ``class``.
        default_value: Absent stores DEFAULT_VALUE null; any string (including "") is
            stored verbatim. ``None`` omits it.
        internal: Case-insensitive; normalized to Yes or No, else INVALID_INPUT.
            ``None`` omits it. Library default when omitted: ``"No"``.
        required: Case-insensitive; normalized to Yes, No, Any or Desired (stored in
            FELEM_REQ), else INVALID_INPUT. ``None`` omits it. Library default when
            omitted: ``"No"``.
        id: Requested ATTR_ID. Absent OR <= 0 means auto-allocate (max existing + 1,
            floor 1000). A taken id > 0 is ALREADY_EXISTS. ``None`` omits it.

    Returns:
        The record (e.g. the created row or ids) as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, ALREADY_EXISTS,
            INVALID_INPUT, NOT_FOUND; any call may also raise INVALID_INPUT (bad
            arguments) or INTERNAL.

    Notes:
        Returns (modified config, the new CFG_ATTR row). Validation order: class,
        duplicate attribute, feature, element, required, internal, id. Does not create a
        CFG_FBOM row.
    """

def delete_attribute(
    config_json: str,
    code: str,
) -> str:
    """Delete an attribute by code.

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before lookup.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, NOT_FOUND; any call
            may also raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        No dependency or system-attribute protection; any attribute can be deleted.
    """

def get_attribute(
    config_json: str,
    code: str,
) -> str:
    """Get one attribute's raw CFG_ATTR row by code.

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before lookup.

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, NOT_FOUND; any call
            may also raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result uses on-disk keys (ATTR_ID, ATTR_CODE, ATTR_CLASS, FTYPE_CODE,
        FELEM_CODE, FELEM_REQ, DEFAULT_VALUE, INTERNAL).
    """

def list_attributes(
    config_json: str,
) -> str:
    """List all attributes as camelCase summaries.

    Args:
        config_json: The configuration JSON (opaque).

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION; any call may also
            raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result is an array of {id, attribute, class, feature, element, required,
        default, internal} in config order; feature/element/default may be null.
    """

def set_attribute(
    config_json: str,
    attribute: str,
    *,
    internal: str | None = None,
    required: str | None = None,
    default_value: str | None = None,
) -> str:
    """Update an attribute's internal / required / default value.

    Args:
        config_json: The configuration JSON (opaque).
        attribute: Attribute code; uppercased before lookup.
        internal: Absent leaves INTERNAL unchanged; else case-insensitive, normalized to
            Yes or No, else INVALID_INPUT. ``None`` omits it.
        required: Absent leaves FELEM_REQ unchanged; else normalized to Yes, No, Any or
            Desired, else INVALID_INPUT. ``None`` omits it.
        default_value: Absent leaves DEFAULT_VALUE unchanged; a string is stored
            verbatim. NOT tri-state: there is no way to clear DEFAULT_VALUE back to
            null. ``None`` omits it.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, NOT_FOUND,
            INVALID_INPUT; any call may also raise INVALID_INPUT (bad arguments) or
            INTERNAL.

    Notes:
        Validation is interleaved with mutation but the input config is never modified
        on error (a new config string is returned only on success).
    """

def add_behavior_override(
    config_json: str,
    feature: str,
    usage_type: str,
    behavior: str,
) -> str:
    """Add a behavior override (CFG_FBOVR row) for a feature and usage type.

    Args:
        config_json: The configuration JSON (opaque).
        feature: Feature code, matched case-insensitively against CFG_FTYPE (NOT_FOUND);
            stored as its FTYPE_ID.
        usage_type: Uppercased; stored as UTYPE_CODE. Any string is accepted (no domain
            check).
        behavior: Behavior code, case-insensitive: a frequency A1, F1, FF, FM, FVM (with
            optional E = exclusive and/or S = stable letters in any order) or the bare
            NAME / NONE; anything else is INVALID_INPUT. Split into FTYPE_FREQ,
            FTYPE_EXCL (Yes/No), FTYPE_STAB (Yes/No).

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND, INVALID_INPUT,
            MISSING_SECTION, ALREADY_EXISTS; any call may also raise INVALID_INPUT (bad
            arguments) or INTERNAL.

    Notes:
        Validation order: feature, behavior, CFG_FBOVR present (MISSING_SECTION),
        duplicate (same FTYPE_ID + uppercased UTYPE_CODE, ALREADY_EXISTS). The row
        always carries FTYPE_ID, UTYPE_CODE, FTYPE_FREQ, FTYPE_EXCL, FTYPE_STAB.
    """

def delete_behavior_override(
    config_json: str,
    feature: str,
    usage_type: str,
) -> str:
    """Delete the behavior override for a feature and usage type.

    Args:
        config_json: The configuration JSON (opaque).
        feature: Feature code, case-insensitive; unknown is NOT_FOUND.
        usage_type: Uppercased, then matched exactly against UTYPE_CODE (a stored
            lowercase UTYPE_CODE cannot be matched).

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND, MISSING_SECTION; any call
            may also raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        No override for the pair is NOT_FOUND; a config without CFG_FBOVR is
        MISSING_SECTION.
    """

def get_behavior_override(
    config_json: str,
    feature: str,
    usage_type: str,
) -> str:
    """Get the raw CFG_FBOVR row for a feature and usage type.

    Args:
        config_json: The configuration JSON (opaque).
        feature: Feature code, case-insensitive; unknown is NOT_FOUND.
        usage_type: Uppercased, then matched exactly against UTYPE_CODE.

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND, MISSING_SECTION; any call
            may also raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result is the stored row with on-disk keys (FTYPE_ID, UTYPE_CODE, FTYPE_FREQ,
        FTYPE_EXCL, FTYPE_STAB).
    """

def list_behavior_overrides(
    config_json: str,
) -> str:
    """List the raw CFG_FBOVR rows sorted by FTYPE_ID.

    Args:
        config_json: The configuration JSON (opaque).

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION; any call may also
            raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result is an array of stored rows (on-disk keys), stable-sorted by FTYPE_ID only
        (rows sharing a FTYPE_ID keep config order). A config without CFG_FBOVR is
        MISSING_SECTION (not []).
    """

def list_behavior_overrides_resolved(
    config_json: str,
) -> str:
    """List behavior overrides as {feature, usageType, behavior} display records.

    Args:
        config_json: The configuration JSON (opaque).

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION; any call may also
            raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result is an array of {feature, usageType, behavior} sorted by (FTYPE_ID,
        UTYPE_CODE). feature is the FTYPE_CODE resolved from FTYPE_ID, or the id as a
        string when no CFG_FTYPE row matches; behavior is FTYPE_FREQ plus E when
        FTYPE_EXCL and S when FTYPE_STAB is Y/YES/1 (case-insensitive). A missing
        G2_CONFIG or CFG_FBOVR is MISSING_SECTION.
    """

def add_comparison_call(
    config_json: str,
    ftype_code: str,
    cfunc_code: str,
    element_list: list[str],
    *,
    id: int | None = None,
) -> str:
    """Add a comparison call (CFG_CFCALL row) binding a comparison function to a
    feature, with its element list (CFG_CFBOM rows).

    Args:
        config_json: The configuration JSON (opaque).
        ftype_code: Feature code; case-insensitive lookup in CFG_FTYPE, else NOT_FOUND.
            Only one comparison call per feature: if any CFG_CFCALL row already has this
            FTYPE_ID the call fails with ALREADY_PRESENT.
        cfunc_code: Comparison function code; case-insensitive lookup in CFG_CFUNC, else
            NOT_FOUND.
        element_list: Element codes, each a case-insensitive GLOBAL CFG_FELEM lookup
            (the element need NOT be in the feature's CFG_FBOM), else NOT_FOUND. Empty
            list or a blank/whitespace-only item is INVALID_INPUT. One CFG_CFBOM row is
            written per item with FTYPE_ID = the call's feature and EXEC_ORDER = 1-based
            list position (outside the exec-order allocation policy). Duplicate items
            are not rejected.
        id: Requested CFCALL_ID. Absent OR <= 0 means auto-allocate (max existing + 1,
            floor 1000). A taken id > 0 is ALREADY_EXISTS (checked before any lookup).
            ``None`` omits it.

    Returns:
        The modified configuration JSON string. ``add_comparison_call_result`` (same
        arguments) returns the record this operation produces.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, ALREADY_EXISTS,
            NOT_FOUND, ALREADY_PRESENT, INVALID_INPUT; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Returns (modified config, the new CFG_CFCALL row {CFCALL_ID, FTYPE_ID,
        CFUNC_ID}). Validation order: id (MISSING_SECTION if CFG_CFCALL is absent or not
        an array; ALREADY_EXISTS if taken), feature, one-call-per-feature
        (ALREADY_PRESENT), function, empty list, then per item blank check and element
        lookup; MISSING_SECTION if CFG_CFBOM is absent. The function's applicability to
        the feature is not checked.
    """

def add_comparison_call_result(
    config_json: str,
    ftype_code: str,
    cfunc_code: str,
    element_list: list[str],
    *,
    id: int | None = None,
) -> str:
    """The record (row / ids) of ``add_comparison_call``: same arguments and operation,
    but returns the record instead of the configuration. Operation: Add a comparison
    call (CFG_CFCALL row) binding a comparison function to a feature, with its
    element list (CFG_CFBOM rows).

    Args:
        config_json: The configuration JSON (opaque).
        ftype_code: Feature code; case-insensitive lookup in CFG_FTYPE, else NOT_FOUND.
            Only one comparison call per feature: if any CFG_CFCALL row already has this
            FTYPE_ID the call fails with ALREADY_PRESENT.
        cfunc_code: Comparison function code; case-insensitive lookup in CFG_CFUNC, else
            NOT_FOUND.
        element_list: Element codes, each a case-insensitive GLOBAL CFG_FELEM lookup
            (the element need NOT be in the feature's CFG_FBOM), else NOT_FOUND. Empty
            list or a blank/whitespace-only item is INVALID_INPUT. One CFG_CFBOM row is
            written per item with FTYPE_ID = the call's feature and EXEC_ORDER = 1-based
            list position (outside the exec-order allocation policy). Duplicate items
            are not rejected.
        id: Requested CFCALL_ID. Absent OR <= 0 means auto-allocate (max existing + 1,
            floor 1000). A taken id > 0 is ALREADY_EXISTS (checked before any lookup).
            ``None`` omits it.

    Returns:
        The record (e.g. the created row or ids) as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, ALREADY_EXISTS,
            NOT_FOUND, ALREADY_PRESENT, INVALID_INPUT; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Returns (modified config, the new CFG_CFCALL row {CFCALL_ID, FTYPE_ID,
        CFUNC_ID}). Validation order: id (MISSING_SECTION if CFG_CFCALL is absent or not
        an array; ALREADY_EXISTS if taken), feature, one-call-per-feature
        (ALREADY_PRESENT), function, empty list, then per item blank check and element
        lookup; MISSING_SECTION if CFG_CFBOM is absent. The function's applicability to
        the feature is not checked.
    """

def delete_comparison_call(
    config_json: str,
    cfcall_id: int,
) -> str:
    """Delete a comparison call by CFCALL_ID, cascading to its CFG_CFBOM rows.

    Args:
        config_json: The configuration JSON (opaque).
        cfcall_id: CFCALL_ID; must exist in CFG_CFCALL else NOT_FOUND (an absent section
            is also NOT_FOUND).

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Removes the CFG_CFCALL row and every CFG_CFBOM row with that CFCALL_ID. No
        dependency or system-call protection: template calls (ids < 1000) can be
        deleted.
    """

def get_comparison_call(
    config_json: str,
    call: int | str,
) -> str:
    """Get one comparison call's raw CFG_CFCALL row, addressed by call id or by feature
    code.

    Args:
        config_json: The configuration JSON (opaque).
        call: A JSON integer selects by CFCALL_ID; a JSON string selects the call bound
            to that feature code (case-insensitive CFG_FTYPE lookup, then a CFG_CFCALL
            scan by FTYPE_ID). Unknown id, unknown feature, or a feature with no
            comparison call is NOT_FOUND; a feature matching 2+ calls (malformed config)
            is INVALID_INPUT. Any other JSON type (or null) is INVALID_INPUT.

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND, INVALID_INPUT; any call
            may also raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result is the stored row with on-disk keys (CFCALL_ID, FTYPE_ID, CFUNC_ID); it
        does not include the CFBOM elements (codes: list_comparison_calls; raw rows:
        get_config_section("CFG_CFBOM")).
    """

def list_comparison_calls(
    config_json: str,
) -> str:
    """List all comparison calls with feature/function codes resolved and their ordered
    element lists.

    Args:
        config_json: The configuration JSON (opaque).

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result is an array of {id, feature, function, elementList} sorted by (FTYPE_ID,
        CFCALL_ID) — not config order. elementList is the call's CFG_CFBOM element codes
        ordered by EXEC_ORDER. Unresolvable ids render as the string "unknown". Missing
        sections are treated as empty (never MISSING_SECTION). LIMITATION: elementList
        omits the stored CFG_CFBOM columns (FTYPE_ID, EXEC_ORDER); read the raw rows
        with get_config_section("CFG_CFBOM").
    """

def add_comparison_call_element(
    config_json: str,
    cfcall_id: int,
    ftype_id: int,
    felem_id: int,
    *,
    exec_order: int | None = None,
) -> str:
    """Add one element (CFG_CFBOM row) to a comparison call, addressed by raw ids.

    Args:
        config_json: The configuration JSON (opaque).
        cfcall_id: CFCALL_ID written verbatim. NOT validated — the call need not exist.
        ftype_id: The ELEMENT's feature id written to the BOM row's FTYPE_ID. Negative
            is INVALID_INPUT; otherwise NOT validated against CFG_FTYPE.
        felem_id: FELEM_ID written verbatim. NOT validated against CFG_FELEM or
            CFG_FBOM.
        exec_order: Allocated per CFCALL_ID. Absent OR <= 0 means auto-allocate (max
            EXEC_ORDER on this call + 1, seed 0 so an empty call starts at 1). A taken
            order > 0 on the same call is ALREADY_EXISTS. ``None`` omits it.

    Returns:
        The modified configuration JSON string. ``add_comparison_call_element_result``
        (same arguments) returns the record this operation produces.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, INVALID_INPUT, ALREADY_PRESENT,
            ALREADY_EXISTS, MISSING_SECTION; any call may also raise INVALID_INPUT (bad
            arguments) or INTERNAL.

    Notes:
        Returns (modified config, the new CFG_CFBOM row {CFCALL_ID, FTYPE_ID, FELEM_ID,
        EXEC_ORDER}). Duplicate identity is (CFCALL_ID, FTYPE_ID, FELEM_ID) regardless
        of EXEC_ORDER -> ALREADY_PRESENT. Order of checks: ftype_id < 0, duplicate,
        exec_order, then MISSING_SECTION if CFG_CFBOM is absent. The same FELEM_ID may
        be added under a different ftype_id, which makes a later feature-less
        delete_comparison_call_element ambiguous.
    """

def add_comparison_call_element_result(
    config_json: str,
    cfcall_id: int,
    ftype_id: int,
    felem_id: int,
    *,
    exec_order: int | None = None,
) -> str:
    """The record (row / ids) of ``add_comparison_call_element``: same arguments and
    operation, but returns the record instead of the configuration. Operation: Add
    one element (CFG_CFBOM row) to a comparison call, addressed by raw ids.

    Args:
        config_json: The configuration JSON (opaque).
        cfcall_id: CFCALL_ID written verbatim. NOT validated — the call need not exist.
        ftype_id: The ELEMENT's feature id written to the BOM row's FTYPE_ID. Negative
            is INVALID_INPUT; otherwise NOT validated against CFG_FTYPE.
        felem_id: FELEM_ID written verbatim. NOT validated against CFG_FELEM or
            CFG_FBOM.
        exec_order: Allocated per CFCALL_ID. Absent OR <= 0 means auto-allocate (max
            EXEC_ORDER on this call + 1, seed 0 so an empty call starts at 1). A taken
            order > 0 on the same call is ALREADY_EXISTS. ``None`` omits it.

    Returns:
        The record (e.g. the created row or ids) as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, INVALID_INPUT, ALREADY_PRESENT,
            ALREADY_EXISTS, MISSING_SECTION; any call may also raise INVALID_INPUT (bad
            arguments) or INTERNAL.

    Notes:
        Returns (modified config, the new CFG_CFBOM row {CFCALL_ID, FTYPE_ID, FELEM_ID,
        EXEC_ORDER}). Duplicate identity is (CFCALL_ID, FTYPE_ID, FELEM_ID) regardless
        of EXEC_ORDER -> ALREADY_PRESENT. Order of checks: ftype_id < 0, duplicate,
        exec_order, then MISSING_SECTION if CFG_CFBOM is absent. The same FELEM_ID may
        be added under a different ftype_id, which makes a later feature-less
        delete_comparison_call_element ambiguous.
    """

def delete_comparison_call_element(
    config_json: str,
    call: int | str,
    element_code: str,
    *,
    element_feature: str | None = None,
) -> str:
    """Delete one element (CFG_CFBOM row) from a comparison call, addressed by call id
    or feature code plus element code.

    Args:
        config_json: The configuration JSON (opaque).
        call: A JSON integer selects by CFCALL_ID (must exist, else NOT_FOUND); a JSON
            string selects the call bound to that feature code (unknown feature or no
            call is NOT_FOUND; 2+ calls is INVALID_INPUT). Any other JSON type is
            INVALID_INPUT.
        element_code: Element code, case-insensitive. Without element_feature: global
            CFG_FELEM lookup (unknown is NOT_FOUND). With element_feature: an unknown
            code or one not in that feature's CFG_FBOM is NOT_IN_FEATURE.
        element_feature: The element's feature code (case-insensitive; unknown is
            NOT_FOUND). Narrows the BOM row match by FTYPE_ID and requires CFG_FBOM
            membership. ``None`` omits it.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND, INVALID_INPUT,
            NOT_IN_FEATURE, NOT_ON_CALL; any call may also raise INVALID_INPUT (bad
            arguments) or INTERNAL.

    Notes:
        EXEC_ORDER is derived from the matched BOM row (not supplied). Element not on an
        existing call is the benign NOT_ON_CALL; an element matching 2+ BOM rows on the
        call (same FELEM_ID under different FTYPE_IDs) without element_feature is
        INVALID_INPUT (ambiguous). Only the matched row is removed; remaining rows keep
        their EXEC_ORDER (no renumbering).
    """

def add_distinct_call(
    config_json: str,
    ftype_code: str,
    dfunc_code: str,
    element_list: list[str],
) -> str:
    """Add a distinct call (CFG_DFCALL row) binding a distinct function to a feature,
    with its element list (CFG_DFBOM rows).

    Args:
        config_json: The configuration JSON (opaque).
        ftype_code: Feature code; case-insensitive lookup in CFG_FTYPE, else NOT_FOUND.
            Only one distinct call per feature: if any CFG_DFCALL row already has this
            FTYPE_ID the call fails with ALREADY_PRESENT.
        dfunc_code: Distinct function code; case-insensitive lookup in CFG_DFUNC, else
            NOT_FOUND.
        element_list: Element codes, each a case-insensitive GLOBAL CFG_FELEM lookup
            (the element need NOT be in the feature's CFG_FBOM), else NOT_FOUND. Empty
            list or a blank/whitespace-only item is INVALID_INPUT (checked before
            anything else). One CFG_DFBOM row is written per item with FTYPE_ID = the
            call's feature and EXEC_ORDER = 1-based list position. Duplicate items are
            not rejected.

    Returns:
        The modified configuration JSON string. ``add_distinct_call_result`` (same
        arguments) returns the record this operation produces.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, INVALID_INPUT, MISSING_SECTION,
            NOT_FOUND, ALREADY_PRESENT; any call may also raise INVALID_INPUT (bad
            arguments) or INTERNAL.

    Notes:
        Returns (modified config, the new CFG_DFCALL row {DFCALL_ID, FTYPE_ID, DFUNC_ID}
        — no EXEC_ORDER). DFCALL_ID is ALWAYS auto-allocated (max existing + 1, floor
        1000): unlike add_comparison_call there is no `id` parameter. Validation order:
        empty list / blank item, id (MISSING_SECTION if G2_CONFIG.CFG_DFCALL is absent),
        feature, one-call-per-feature (ALREADY_PRESENT), function, element lookups;
        MISSING_SECTION if CFG_DFCALL is not an array or CFG_DFBOM is absent.
    """

def add_distinct_call_result(
    config_json: str,
    ftype_code: str,
    dfunc_code: str,
    element_list: list[str],
) -> str:
    """The record (row / ids) of ``add_distinct_call``: same arguments and operation,
    but returns the record instead of the configuration. Operation: Add a distinct
    call (CFG_DFCALL row) binding a distinct function to a feature, with its element
    list (CFG_DFBOM rows).

    Args:
        config_json: The configuration JSON (opaque).
        ftype_code: Feature code; case-insensitive lookup in CFG_FTYPE, else NOT_FOUND.
            Only one distinct call per feature: if any CFG_DFCALL row already has this
            FTYPE_ID the call fails with ALREADY_PRESENT.
        dfunc_code: Distinct function code; case-insensitive lookup in CFG_DFUNC, else
            NOT_FOUND.
        element_list: Element codes, each a case-insensitive GLOBAL CFG_FELEM lookup
            (the element need NOT be in the feature's CFG_FBOM), else NOT_FOUND. Empty
            list or a blank/whitespace-only item is INVALID_INPUT (checked before
            anything else). One CFG_DFBOM row is written per item with FTYPE_ID = the
            call's feature and EXEC_ORDER = 1-based list position. Duplicate items are
            not rejected.

    Returns:
        The record (e.g. the created row or ids) as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, INVALID_INPUT, MISSING_SECTION,
            NOT_FOUND, ALREADY_PRESENT; any call may also raise INVALID_INPUT (bad
            arguments) or INTERNAL.

    Notes:
        Returns (modified config, the new CFG_DFCALL row {DFCALL_ID, FTYPE_ID, DFUNC_ID}
        — no EXEC_ORDER). DFCALL_ID is ALWAYS auto-allocated (max existing + 1, floor
        1000): unlike add_comparison_call there is no `id` parameter. Validation order:
        empty list / blank item, id (MISSING_SECTION if G2_CONFIG.CFG_DFCALL is absent),
        feature, one-call-per-feature (ALREADY_PRESENT), function, element lookups;
        MISSING_SECTION if CFG_DFCALL is not an array or CFG_DFBOM is absent.
    """

def delete_distinct_call(
    config_json: str,
    dfcall_id: int,
) -> str:
    """Delete a distinct call by DFCALL_ID, cascading to its CFG_DFBOM rows.

    Args:
        config_json: The configuration JSON (opaque).
        dfcall_id: DFCALL_ID; must exist in CFG_DFCALL else NOT_FOUND (an absent section
            is also NOT_FOUND).

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Removes the CFG_DFCALL row and every CFG_DFBOM row with that DFCALL_ID. No
        dependency or system-call protection.
    """

def get_distinct_call(
    config_json: str,
    call: int | str,
) -> str:
    """Get one distinct call's raw CFG_DFCALL row, addressed by call id or by feature
    code.

    Args:
        config_json: The configuration JSON (opaque).
        call: A JSON integer selects by DFCALL_ID; a JSON string selects the call bound
            to that feature code (case-insensitive CFG_FTYPE lookup, then a CFG_DFCALL
            scan by FTYPE_ID). Unknown id, unknown feature, or a feature with no
            distinct call is NOT_FOUND; a feature matching 2+ calls (malformed config)
            is INVALID_INPUT. Any other JSON type (or null) is INVALID_INPUT.

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND, INVALID_INPUT; any call
            may also raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result is the stored row with on-disk keys (DFCALL_ID, FTYPE_ID, DFUNC_ID); it
        does not include the DFBOM elements (codes: list_distinct_calls; raw rows:
        get_config_section("CFG_DFBOM")).
    """

def list_distinct_calls(
    config_json: str,
) -> str:
    """List all distinct calls with feature/function codes resolved and their ordered
    element lists.

    Args:
        config_json: The configuration JSON (opaque).

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result is an array of {id, feature, function, execOrder, elementList} sorted by
        (FTYPE_ID, DFCALL_ID) — not config order. execOrder is the CFG_DFCALL row's
        EXEC_ORDER, which the v4 schema (and every template / add_distinct_call row)
        lacks, so it is 1. elementList is the call's CFG_DFBOM element codes ordered by
        EXEC_ORDER. Unresolvable ids render as "unknown". Missing sections are treated
        as empty. LIMITATION: elementList omits the stored CFG_DFBOM columns (FTYPE_ID,
        EXEC_ORDER); read the raw rows with get_config_section("CFG_DFBOM").
    """

def add_distinct_call_element(
    config_json: str,
    dfcall_id: int,
    ftype_id: int,
    felem_id: int,
    *,
    exec_order: int | None = None,
) -> str:
    """Add one element (CFG_DFBOM row) to a distinct call, addressed by raw ids.

    Args:
        config_json: The configuration JSON (opaque).
        dfcall_id: DFCALL_ID written verbatim. NOT validated — the call need not exist.
        ftype_id: The ELEMENT's feature id written to the BOM row's FTYPE_ID. NOT
            validated at all — unlike add_comparison_call_element, a negative id is
            accepted and stored.
        felem_id: FELEM_ID written verbatim. NOT validated against CFG_FELEM or
            CFG_FBOM.
        exec_order: Allocated per DFCALL_ID. Absent OR <= 0 means auto-allocate (max
            EXEC_ORDER on this call + 1, seed 0). A taken order > 0 on the same call is
            ALREADY_EXISTS. ``None`` omits it.

    Returns:
        The modified configuration JSON string. ``add_distinct_call_element_result``
        (same arguments) returns the record this operation produces.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, ALREADY_PRESENT, ALREADY_EXISTS,
            MISSING_SECTION; any call may also raise INVALID_INPUT (bad arguments) or
            INTERNAL.

    Notes:
        Returns (modified config, the new CFG_DFBOM row {DFCALL_ID, FTYPE_ID, FELEM_ID,
        EXEC_ORDER}). Duplicate identity is (DFCALL_ID, FTYPE_ID, FELEM_ID) regardless
        of EXEC_ORDER -> ALREADY_PRESENT. Order of checks: duplicate, exec_order, then
        MISSING_SECTION if CFG_DFBOM is absent.
    """

def add_distinct_call_element_result(
    config_json: str,
    dfcall_id: int,
    ftype_id: int,
    felem_id: int,
    *,
    exec_order: int | None = None,
) -> str:
    """The record (row / ids) of ``add_distinct_call_element``: same arguments and
    operation, but returns the record instead of the configuration. Operation: Add
    one element (CFG_DFBOM row) to a distinct call, addressed by raw ids.

    Args:
        config_json: The configuration JSON (opaque).
        dfcall_id: DFCALL_ID written verbatim. NOT validated — the call need not exist.
        ftype_id: The ELEMENT's feature id written to the BOM row's FTYPE_ID. NOT
            validated at all — unlike add_comparison_call_element, a negative id is
            accepted and stored.
        felem_id: FELEM_ID written verbatim. NOT validated against CFG_FELEM or
            CFG_FBOM.
        exec_order: Allocated per DFCALL_ID. Absent OR <= 0 means auto-allocate (max
            EXEC_ORDER on this call + 1, seed 0). A taken order > 0 on the same call is
            ALREADY_EXISTS. ``None`` omits it.

    Returns:
        The record (e.g. the created row or ids) as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, ALREADY_PRESENT, ALREADY_EXISTS,
            MISSING_SECTION; any call may also raise INVALID_INPUT (bad arguments) or
            INTERNAL.

    Notes:
        Returns (modified config, the new CFG_DFBOM row {DFCALL_ID, FTYPE_ID, FELEM_ID,
        EXEC_ORDER}). Duplicate identity is (DFCALL_ID, FTYPE_ID, FELEM_ID) regardless
        of EXEC_ORDER -> ALREADY_PRESENT. Order of checks: duplicate, exec_order, then
        MISSING_SECTION if CFG_DFBOM is absent.
    """

def delete_distinct_call_element(
    config_json: str,
    call: int | str,
    element_code: str,
    *,
    element_feature: str | None = None,
) -> str:
    """Delete one element (CFG_DFBOM row) from a distinct call, addressed by call id or
    feature code plus element code.

    Args:
        config_json: The configuration JSON (opaque).
        call: A JSON integer selects by DFCALL_ID (must exist, else NOT_FOUND); a JSON
            string selects the call bound to that feature code (unknown feature or no
            call is NOT_FOUND; 2+ calls is INVALID_INPUT). Any other JSON type is
            INVALID_INPUT.
        element_code: Element code, case-insensitive. Without element_feature: global
            CFG_FELEM lookup (unknown is NOT_FOUND). With element_feature: an unknown
            code or one not in that feature's CFG_FBOM is NOT_IN_FEATURE.
        element_feature: The element's feature code (case-insensitive; unknown is
            NOT_FOUND). Narrows the BOM row match by FTYPE_ID and requires CFG_FBOM
            membership. ``None`` omits it.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND, INVALID_INPUT,
            NOT_IN_FEATURE, NOT_ON_CALL; any call may also raise INVALID_INPUT (bad
            arguments) or INTERNAL.

    Notes:
        EXEC_ORDER is derived from the matched BOM row (not supplied). Element not on an
        existing call is the benign NOT_ON_CALL; an element matching 2+ BOM rows on the
        call without element_feature is INVALID_INPUT (ambiguous). Only the matched row
        is removed; no renumbering.
    """

def add_expression_call(
    config_json: str,
    efunc_code: str,
    element_list: Any,
    is_virtual: str,
    *,
    ftype_code: str | None = None,
    felem_code: str | None = None,
    exec_order: int | None = None,
    expression_feature: str | None = None,
) -> str:
    """Add an expression call (CFG_EFCALL row) plus its element list (CFG_EFBOM rows).

    Args:
        config_json: The configuration JSON (opaque).
        efunc_code: Expression function code (CFG_EFUNC, case-insensitive) or NOT_FOUND.
        element_list: JSON array of {"element": str, "required": str, "feature"?: str}
            objects (unknown keys, non-objects, non-string values = INVALID_INPUT;
            missing element/required = MISSING_FIELD). One CFG_EFBOM row per item,
            EXEC_ORDER = 1-based list position. element: global CFG_FELEM lookup
            (case-insensitive) or NOT_FOUND. required: stored verbatim in FELEM_REQ (not
            validated or normalized). feature: absent stores BOM FTYPE_ID -1 (G2
            WILDCARDED_FTYPE: any feature in the record carrying the element); "PARENT"
            (case-insensitive) stores BOM FTYPE_ID 0 (G2 PARENT_FEATURE_LINKED_FTYPE:
            the feature that triggered the call); otherwise a feature code
            (case-insensitive) or NOT_FOUND. The element is NOT checked for membership
            in that feature. [] is allowed. Shape: ``[{element: string, required:
            string, feature?: string}]``.
        ftype_code: Feature code (case-insensitive) or NOT_FOUND; "ALL"
            (case-insensitive) = absent; absent stores FTYPE_ID -1. ``None`` omits it.
        felem_code: Element code (case-insensitive) or NOT_FOUND; "N/A"
            (case-insensitive) = absent; absent stores FELEM_ID -1. Exactly one of
            ftype_code / felem_code must resolve, else INVALID_INPUT. ``None`` omits it.
        exec_order: CFG_EFCALL EXEC_ORDER scoped per (FTYPE_ID, FELEM_ID). Absent or <=
            0 = auto-allocate (max in scope + 1); > 0 and free = verbatim; > 0 and taken
            = ALREADY_EXISTS. ``None`` omits it.
        expression_feature: Feature code stored as EFEAT_FTYPE_ID (case-insensitive) or
            NOT_FOUND; absent or "N/A" (case-insensitive) stores -1. ``None`` omits it.
        is_virtual: Stored verbatim in IS_VIRTUAL (not validated or normalized; the Rust
            `new()` default is "No").

    Returns:
        The modified configuration JSON string. ``add_expression_call_result`` (same
        arguments) returns the record this operation produces.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, NOT_FOUND,
            INVALID_INPUT, ALREADY_EXISTS, MISSING_FIELD; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Returns (modified config, the new CFG_EFCALL row {EFCALL_ID, FTYPE_ID, FELEM_ID,
        EFUNC_ID, EXEC_ORDER, EFEAT_FTYPE_ID, IS_VIRTUAL}); the created CFG_EFBOM rows
        are NOT in the record (see list_expression_calls). EFCALL_ID is auto-allocated
        (max + 1, floor 1000). Check order: EFCALL_ID allocation (MISSING_SECTION if
        CFG_EFCALL absent), efunc, feature, element, exactly-one rule, exec order,
        expression_feature, element list, then MISSING_SECTION if CFG_EFBOM absent. BOM
        FTYPE_ID sentinels (G2 EFBomConfig.cpp): 0 = parent feature link, -1 = any
        feature. The BOM-feature column is not rendered by get/list_expression_calls;
        read raw rows with get_config_section("CFG_EFBOM").
    """

def add_expression_call_result(
    config_json: str,
    efunc_code: str,
    element_list: Any,
    is_virtual: str,
    *,
    ftype_code: str | None = None,
    felem_code: str | None = None,
    exec_order: int | None = None,
    expression_feature: str | None = None,
) -> str:
    """The record (row / ids) of ``add_expression_call``: same arguments and operation,
    but returns the record instead of the configuration. Operation: Add an expression
    call (CFG_EFCALL row) plus its element list (CFG_EFBOM rows).

    Args:
        config_json: The configuration JSON (opaque).
        efunc_code: Expression function code (CFG_EFUNC, case-insensitive) or NOT_FOUND.
        element_list: JSON array of {"element": str, "required": str, "feature"?: str}
            objects (unknown keys, non-objects, non-string values = INVALID_INPUT;
            missing element/required = MISSING_FIELD). One CFG_EFBOM row per item,
            EXEC_ORDER = 1-based list position. element: global CFG_FELEM lookup
            (case-insensitive) or NOT_FOUND. required: stored verbatim in FELEM_REQ (not
            validated or normalized). feature: absent stores BOM FTYPE_ID -1 (G2
            WILDCARDED_FTYPE: any feature in the record carrying the element); "PARENT"
            (case-insensitive) stores BOM FTYPE_ID 0 (G2 PARENT_FEATURE_LINKED_FTYPE:
            the feature that triggered the call); otherwise a feature code
            (case-insensitive) or NOT_FOUND. The element is NOT checked for membership
            in that feature. [] is allowed. Shape: ``[{element: string, required:
            string, feature?: string}]``.
        ftype_code: Feature code (case-insensitive) or NOT_FOUND; "ALL"
            (case-insensitive) = absent; absent stores FTYPE_ID -1. ``None`` omits it.
        felem_code: Element code (case-insensitive) or NOT_FOUND; "N/A"
            (case-insensitive) = absent; absent stores FELEM_ID -1. Exactly one of
            ftype_code / felem_code must resolve, else INVALID_INPUT. ``None`` omits it.
        exec_order: CFG_EFCALL EXEC_ORDER scoped per (FTYPE_ID, FELEM_ID). Absent or <=
            0 = auto-allocate (max in scope + 1); > 0 and free = verbatim; > 0 and taken
            = ALREADY_EXISTS. ``None`` omits it.
        expression_feature: Feature code stored as EFEAT_FTYPE_ID (case-insensitive) or
            NOT_FOUND; absent or "N/A" (case-insensitive) stores -1. ``None`` omits it.
        is_virtual: Stored verbatim in IS_VIRTUAL (not validated or normalized; the Rust
            `new()` default is "No").

    Returns:
        The record (e.g. the created row or ids) as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, NOT_FOUND,
            INVALID_INPUT, ALREADY_EXISTS, MISSING_FIELD; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Returns (modified config, the new CFG_EFCALL row {EFCALL_ID, FTYPE_ID, FELEM_ID,
        EFUNC_ID, EXEC_ORDER, EFEAT_FTYPE_ID, IS_VIRTUAL}); the created CFG_EFBOM rows
        are NOT in the record (see list_expression_calls). EFCALL_ID is auto-allocated
        (max + 1, floor 1000). Check order: EFCALL_ID allocation (MISSING_SECTION if
        CFG_EFCALL absent), efunc, feature, element, exactly-one rule, exec order,
        expression_feature, element list, then MISSING_SECTION if CFG_EFBOM absent. BOM
        FTYPE_ID sentinels (G2 EFBomConfig.cpp): 0 = parent feature link, -1 = any
        feature. The BOM-feature column is not rendered by get/list_expression_calls;
        read raw rows with get_config_section("CFG_EFBOM").
    """

def delete_expression_call(
    config_json: str,
    efcall_id: int,
) -> str:
    """Delete an expression call by EFCALL_ID, cascading its CFG_EFBOM rows.

    Args:
        config_json: The configuration JSON (opaque).
        efcall_id: Must match an existing EFCALL_ID, else NOT_FOUND.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Removes the CFG_EFCALL row(s) with that id and every CFG_EFBOM row with that
        EFCALL_ID.
    """

def get_expression_call(
    config_json: str,
    call: int | str,
) -> str:
    """Get one expression call's raw CFG_EFCALL row, by EFCALL_ID or by feature code.

    Args:
        config_json: The configuration JSON (opaque).
        call: Call selector: an integer = EFCALL_ID (NOT_FOUND if absent); a string =
            feature code (case-insensitive; unknown feature = NOT_FOUND, no call on the
            feature = NOT_FOUND, more than one call on the feature = INVALID_INPUT —
            address such calls by id). Any other JSON type = INVALID_INPUT.

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND, INVALID_INPUT; any call
            may also raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result uses on-disk keys (EFCALL_ID, FTYPE_ID, FELEM_ID, EFUNC_ID, EXEC_ORDER,
        EFEAT_FTYPE_ID, IS_VIRTUAL); BOM rows are not included (raw rows:
        get_config_section("CFG_EFBOM")). Expression calls are many-per-feature
        (template NAME has 7), so by-feature is often ambiguous.
    """

def list_expression_calls(
    config_json: str,
) -> str:
    """List all expression calls with resolved codes and element lists.

    Args:
        config_json: The configuration JSON (opaque).

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Array of {id, feature, element, execOrder, function, isVirtual,
        expressionFeature, elementList}, stably sorted by (FTYPE_ID, FELEM_ID,
        EXEC_ORDER). feature "all" / element "n/a" / function "unknown" when the id is
        <= 0 or unresolved; expressionFeature "n/a" when EFEAT_FTYPE_ID <= 0.
        elementList is the BOM element codes ordered by BOM EXEC_ORDER. Missing sections
        yield []. LIMITATION: elementList omits the stored CFG_EFBOM columns FTYPE_ID (0
        = parent feature, -1 = any feature), EXEC_ORDER and FELEM_REQ, which the engine
        uses; read the raw rows with get_config_section("CFG_EFBOM").
    """

def add_expression_call_element(
    config_json: str,
    efcall_id: int,
    ftype_id: int,
    felem_id: int,
    felem_req: str,
    *,
    exec_order: int | None = None,
) -> str:
    """Add one CFG_EFBOM row to an expression call, addressed by raw ids.

    Args:
        config_json: The configuration JSON (opaque).
        efcall_id: Stored as EFCALL_ID. NOT validated — the call need not exist.
        ftype_id: The ELEMENT's feature id, stored verbatim as BOM FTYPE_ID. < 0 =
            INVALID_INPUT; 0 is accepted and is the G2 parent feature link (same as
            add_expression_call's feature "PARENT"); -1 (any feature) is not addable
            here; NOT validated against CFG_FTYPE.
        felem_id: Stored verbatim as FELEM_ID. NOT validated against CFG_FELEM.
        exec_order: BOM EXEC_ORDER scoped per EFCALL_ID. Absent or <= 0 = auto-allocate
            (max on the call + 1); > 0 and free = verbatim; > 0 and taken =
            ALREADY_EXISTS. ``None`` omits it.
        felem_req: Stored verbatim in FELEM_REQ (not validated or normalized).

    Returns:
        The modified configuration JSON string. ``add_expression_call_element_result``
        (same arguments) returns the record this operation produces.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, INVALID_INPUT, ALREADY_PRESENT,
            ALREADY_EXISTS, MISSING_SECTION; any call may also raise INVALID_INPUT (bad
            arguments) or INTERNAL.

    Notes:
        Returns (modified config, the new CFG_EFBOM row {EFCALL_ID, FTYPE_ID, FELEM_ID,
        EXEC_ORDER, FELEM_REQ}). Check order: ftype_id < 0, ALREADY_PRESENT when
        (EFCALL_ID, FTYPE_ID, FELEM_ID) already exists (EXEC_ORDER ignored), exec order,
        then MISSING_SECTION if CFG_EFBOM is absent.
    """

def add_expression_call_element_result(
    config_json: str,
    efcall_id: int,
    ftype_id: int,
    felem_id: int,
    felem_req: str,
    *,
    exec_order: int | None = None,
) -> str:
    """The record (row / ids) of ``add_expression_call_element``: same arguments and
    operation, but returns the record instead of the configuration. Operation: Add
    one CFG_EFBOM row to an expression call, addressed by raw ids.

    Args:
        config_json: The configuration JSON (opaque).
        efcall_id: Stored as EFCALL_ID. NOT validated — the call need not exist.
        ftype_id: The ELEMENT's feature id, stored verbatim as BOM FTYPE_ID. < 0 =
            INVALID_INPUT; 0 is accepted and is the G2 parent feature link (same as
            add_expression_call's feature "PARENT"); -1 (any feature) is not addable
            here; NOT validated against CFG_FTYPE.
        felem_id: Stored verbatim as FELEM_ID. NOT validated against CFG_FELEM.
        exec_order: BOM EXEC_ORDER scoped per EFCALL_ID. Absent or <= 0 = auto-allocate
            (max on the call + 1); > 0 and free = verbatim; > 0 and taken =
            ALREADY_EXISTS. ``None`` omits it.
        felem_req: Stored verbatim in FELEM_REQ (not validated or normalized).

    Returns:
        The record (e.g. the created row or ids) as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, INVALID_INPUT, ALREADY_PRESENT,
            ALREADY_EXISTS, MISSING_SECTION; any call may also raise INVALID_INPUT (bad
            arguments) or INTERNAL.

    Notes:
        Returns (modified config, the new CFG_EFBOM row {EFCALL_ID, FTYPE_ID, FELEM_ID,
        EXEC_ORDER, FELEM_REQ}). Check order: ftype_id < 0, ALREADY_PRESENT when
        (EFCALL_ID, FTYPE_ID, FELEM_ID) already exists (EXEC_ORDER ignored), exec order,
        then MISSING_SECTION if CFG_EFBOM is absent.
    """

def delete_expression_call_element(
    config_json: str,
    call: int | str,
    element_code: str,
    *,
    element_feature: str | None = None,
) -> str:
    """Delete one CFG_EFBOM row from an expression call, addressed by call + element
    code.

    Args:
        config_json: The configuration JSON (opaque).
        call: Call selector: an integer = EFCALL_ID; a string = feature code (unknown
            feature = NOT_FOUND, no call = NOT_FOUND, more than one call =
            INVALID_INPUT). A call id that does not exist = NOT_FOUND.
        element_code: Element code (case-insensitive). Without element_feature: global
            lookup, unknown = NOT_FOUND. With element_feature: unknown OR not in that
            feature's CFG_FBOM = NOT_IN_FEATURE. Known but not on the call =
            NOT_ON_CALL.
        element_feature: The element's feature code (case-insensitive; unknown =
            NOT_FOUND). Narrows the BOM match to that FTYPE_ID. Without it, an element
            present under more than one feature on the call is INVALID_INPUT (ambiguous)
            — e.g. template EFCALL 97 carries TOKENIZED_NM under GROUP_ASSOCIATION and
            EMPLOYER. ``None`` omits it.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND, INVALID_INPUT,
            NOT_IN_FEATURE, NOT_ON_CALL; any call may also raise INVALID_INPUT (bad
            arguments) or INTERNAL.

    Notes:
        The BOM EXEC_ORDER is derived from the located row; only that row is removed.
        Check order: call selector, call existence, element_feature, element, BOM row.
    """

def add_standardize_call(
    config_json: str,
    sfunc_code: str,
    *,
    ftype_code: str | None = None,
    felem_code: str | None = None,
    exec_order: int | None = None,
) -> str:
    """Add a standardize call (CFG_SFCALL row) binding a standardize function to a
    feature or an element.

    Args:
        config_json: The configuration JSON (opaque).
        sfunc_code: Standardize function code (CFG_SFUNC, case-insensitive) or
            NOT_FOUND. Looked up before the feature/element.
        ftype_code: Feature code (case-insensitive) or NOT_FOUND. "ALL"
            (case-insensitive) is treated as absent. Stored as FTYPE_ID; absent stores
            FTYPE_ID -1. ``None`` omits it.
        felem_code: Element code (case-insensitive) or NOT_FOUND. "N/A"
            (case-insensitive) is treated as absent. Stored as FELEM_ID; absent stores
            FELEM_ID -1. Exactly one of ftype_code / felem_code must resolve, else
            INVALID_INPUT. ``None`` omits it.
        exec_order: EXEC_ORDER scoped per (FTYPE_ID, FELEM_ID) of the new row (the -1
            sentinel is part of the scope). Absent or <= 0 = auto-allocate (max in scope
            + 1, 1 for an empty scope); > 0 and free = used verbatim; > 0 and taken =
            ALREADY_EXISTS. ``None`` omits it.

    Returns:
        The modified configuration JSON string. ``add_standardize_call_result`` (same
        arguments) returns the record this operation produces.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, NOT_FOUND,
            INVALID_INPUT, ALREADY_EXISTS; any call may also raise INVALID_INPUT (bad
            arguments) or INTERNAL.

    Notes:
        Returns (modified config, the new CFG_SFCALL row {SFCALL_ID, FTYPE_ID, FELEM_ID,
        SFUNC_ID, EXEC_ORDER}). SFCALL_ID is always auto-allocated (max + 1, floor
        1000). MISSING_SECTION when CFG_SFCALL is absent. Check order: SFCALL_ID
        allocation, sfunc, feature, element, exactly-one rule, exec order. TRAP: the
        exec-order scope does not include SFUNC_ID, so a second call on the same feature
        continues that feature's order sequence.
    """

def add_standardize_call_result(
    config_json: str,
    sfunc_code: str,
    *,
    ftype_code: str | None = None,
    felem_code: str | None = None,
    exec_order: int | None = None,
) -> str:
    """The record (row / ids) of ``add_standardize_call``: same arguments and operation,
    but returns the record instead of the configuration. Operation: Add a standardize
    call (CFG_SFCALL row) binding a standardize function to a feature or an element.

    Args:
        config_json: The configuration JSON (opaque).
        sfunc_code: Standardize function code (CFG_SFUNC, case-insensitive) or
            NOT_FOUND. Looked up before the feature/element.
        ftype_code: Feature code (case-insensitive) or NOT_FOUND. "ALL"
            (case-insensitive) is treated as absent. Stored as FTYPE_ID; absent stores
            FTYPE_ID -1. ``None`` omits it.
        felem_code: Element code (case-insensitive) or NOT_FOUND. "N/A"
            (case-insensitive) is treated as absent. Stored as FELEM_ID; absent stores
            FELEM_ID -1. Exactly one of ftype_code / felem_code must resolve, else
            INVALID_INPUT. ``None`` omits it.
        exec_order: EXEC_ORDER scoped per (FTYPE_ID, FELEM_ID) of the new row (the -1
            sentinel is part of the scope). Absent or <= 0 = auto-allocate (max in scope
            + 1, 1 for an empty scope); > 0 and free = used verbatim; > 0 and taken =
            ALREADY_EXISTS. ``None`` omits it.

    Returns:
        The record (e.g. the created row or ids) as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, NOT_FOUND,
            INVALID_INPUT, ALREADY_EXISTS; any call may also raise INVALID_INPUT (bad
            arguments) or INTERNAL.

    Notes:
        Returns (modified config, the new CFG_SFCALL row {SFCALL_ID, FTYPE_ID, FELEM_ID,
        SFUNC_ID, EXEC_ORDER}). SFCALL_ID is always auto-allocated (max + 1, floor
        1000). MISSING_SECTION when CFG_SFCALL is absent. Check order: SFCALL_ID
        allocation, sfunc, feature, element, exactly-one rule, exec order. TRAP: the
        exec-order scope does not include SFUNC_ID, so a second call on the same feature
        continues that feature's order sequence.
    """

def delete_standardize_call(
    config_json: str,
    sfcall_id: int,
) -> str:
    """Delete a standardize call by SFCALL_ID.

    Args:
        config_json: The configuration JSON (opaque).
        sfcall_id: Must match an existing SFCALL_ID, else NOT_FOUND (also NOT_FOUND when
            CFG_SFCALL is absent).

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Removes every CFG_SFCALL row with that id; no dependency checks (there is no
        standardize BOM).
    """

def get_standardize_call(
    config_json: str,
    call: int | str,
) -> str:
    """Get one standardize call's raw CFG_SFCALL row, by SFCALL_ID or by feature code.

    Args:
        config_json: The configuration JSON (opaque).
        call: Call selector: an integer = SFCALL_ID (NOT_FOUND if absent); a string =
            feature code (case-insensitive; unknown feature = NOT_FOUND, no call on the
            feature = NOT_FOUND, more than one call on the feature = INVALID_INPUT —
            address such calls by id). Any other JSON type = INVALID_INPUT.

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND, INVALID_INPUT; any call
            may also raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result uses on-disk keys (SFCALL_ID, FTYPE_ID, FELEM_ID, SFUNC_ID, EXEC_ORDER).
        Element-bound calls (FTYPE_ID -1) are never found by feature. In the template
        NAME has two standardize calls (PARSE_NAME, TOKENIZE_NAME), so by-feature NAME
        is ambiguous.
    """

def list_standardize_calls(
    config_json: str,
) -> str:
    """List all standardize calls with resolved codes.

    Args:
        config_json: The configuration JSON (opaque).

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Array of {id, feature, element, execOrder, function}, stably sorted by
        (FTYPE_ID, EXEC_ORDER). feature is "all" when FTYPE_ID <= 0 or unresolved;
        element is "n/a" when FELEM_ID <= 0 or unresolved; function is "unknown" when
        unresolved. No elementList key. Missing sections yield [] (no MISSING_SECTION).
    """

def add_standardize_call_element(
    config_json: str,
    ftype_id: int,
    sfunc_id: int,
    *,
    felem_id: int | None = None,
    exec_order: int | None = None,
) -> str:
    """Add a CFG_SFCALL row addressed by raw ids (FTYPE_ID, SFUNC_ID, FELEM_ID).

    Args:
        config_json: The configuration JSON (opaque).
        ftype_id: Stored verbatim as FTYPE_ID. NOT validated against CFG_FTYPE (use -1
            for an element-bound row).
        sfunc_id: Stored verbatim as SFUNC_ID. NOT validated against CFG_SFUNC.
        felem_id: Stored as FELEM_ID; absent = -1. NOT validated against CFG_FELEM.
            ``None`` omits it.
        exec_order: EXEC_ORDER scoped per (FTYPE_ID, FELEM_ID). Absent or <= 0 =
            auto-allocate (max in scope + 1); > 0 and free = verbatim; > 0 and taken =
            ALREADY_EXISTS. ``None`` omits it.

    Returns:
        The modified configuration JSON string. ``add_standardize_call_element_result``
        (same arguments) returns the record this operation produces.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, ALREADY_PRESENT,
            ALREADY_EXISTS; any call may also raise INVALID_INPUT (bad arguments) or
            INTERNAL.

    Notes:
        Returns (modified config, the new CFG_SFCALL row). ALREADY_PRESENT when a row
        with the same (FTYPE_ID, SFUNC_ID, FELEM_ID) exists (checked first). SFCALL_ID
        auto-allocated (max + 1, floor 1000); MISSING_SECTION when CFG_SFCALL is absent.
        Unlike add_standardize_call there is no feature-xor-element rule and no id
        validation.
    """

def add_standardize_call_element_result(
    config_json: str,
    ftype_id: int,
    sfunc_id: int,
    *,
    felem_id: int | None = None,
    exec_order: int | None = None,
) -> str:
    """The record (row / ids) of ``add_standardize_call_element``: same arguments and
    operation, but returns the record instead of the configuration. Operation: Add a
    CFG_SFCALL row addressed by raw ids (FTYPE_ID, SFUNC_ID, FELEM_ID).

    Args:
        config_json: The configuration JSON (opaque).
        ftype_id: Stored verbatim as FTYPE_ID. NOT validated against CFG_FTYPE (use -1
            for an element-bound row).
        sfunc_id: Stored verbatim as SFUNC_ID. NOT validated against CFG_SFUNC.
        felem_id: Stored as FELEM_ID; absent = -1. NOT validated against CFG_FELEM.
            ``None`` omits it.
        exec_order: EXEC_ORDER scoped per (FTYPE_ID, FELEM_ID). Absent or <= 0 =
            auto-allocate (max in scope + 1); > 0 and free = verbatim; > 0 and taken =
            ALREADY_EXISTS. ``None`` omits it.

    Returns:
        The record (e.g. the created row or ids) as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, ALREADY_PRESENT,
            ALREADY_EXISTS; any call may also raise INVALID_INPUT (bad arguments) or
            INTERNAL.

    Notes:
        Returns (modified config, the new CFG_SFCALL row). ALREADY_PRESENT when a row
        with the same (FTYPE_ID, SFUNC_ID, FELEM_ID) exists (checked first). SFCALL_ID
        auto-allocated (max + 1, floor 1000); MISSING_SECTION when CFG_SFCALL is absent.
        Unlike add_standardize_call there is no feature-xor-element rule and no id
        validation.
    """

def delete_standardize_call_element(
    config_json: str,
    ftype_id: int,
    sfunc_id: int,
    *,
    felem_id: int | None = None,
) -> str:
    """Delete CFG_SFCALL rows matching raw ids (FTYPE_ID, SFUNC_ID, FELEM_ID).

    Args:
        config_json: The configuration JSON (opaque).
        ftype_id: Matched against FTYPE_ID.
        sfunc_id: Matched against SFUNC_ID.
        felem_id: Matched against FELEM_ID; absent = -1. ``None`` omits it.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        NOT_FOUND (not NOT_ON_CALL) when no row matches. Removes EVERY matching row.
    """

def add_config_section(
    config_json: str,
    section_name: str,
) -> str:
    """Add a new, empty top-level section (an empty array) to G2_CONFIG.

    Args:
        config_json: The configuration JSON (opaque).
        section_name: Uppercased before the duplicate check and storage. The new section
            is always an empty JSON array.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, ALREADY_EXISTS, NOT_FOUND; any call
            may also raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        An existing key of the same (uppercased) name, of ANY JSON type, is
        ALREADY_EXISTS. A config without a G2_CONFIG key is NOT_FOUND (not
        MISSING_SECTION). No validation of the name (any string, e.g. not CFG_*, is
        accepted).
    """

def remove_config_section(
    config_json: str,
    section_name: str,
) -> str:
    """Remove a top-level section from G2_CONFIG.

    Args:
        config_json: The configuration JSON (opaque).
        section_name: Uppercased before lookup.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        TRAP: no protection or dependency check; any section (including core sections
        such as CFG_DSRC, SETTINGS) can be removed. A missing section or a missing
        G2_CONFIG is NOT_FOUND.
    """

def get_config_section(
    config_json: str,
    section_name: str,
    *,
    filter: str | None = None,
) -> str:
    """Get the raw items of a top-level section, optionally filtered by a
    case-insensitive substring.

    Args:
        config_json: The configuration JSON (opaque).
        section_name: TRAP: matched EXACTLY (case-sensitive, NOT uppercased, unlike
            add/remove_config_section). Unknown name is NOT_FOUND.
        filter: Absent returns every item. Otherwise keeps items whose json.dumps-spaced
            rendering (`{"K": 1, "J": null}`, crate::filter::to_json_dumps_string)
            contains the filter, case-insensitively. ``None`` omits it.

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result is an array. An array section returns its rows; a null or empty section
        returns []; a non-array, non-null section (e.g. SETTINGS, CONFIG_BASE_VERSION
        objects) returns a one-element array holding the value. An empty result does not
        distinguish "section empty" from "filter matched nothing": use
        config_section_is_empty.
    """

def config_section_is_empty(
    config_json: str,
    section_name: str,
) -> str:
    """Report whether a top-level section is empty (null or []).

    Args:
        config_json: The configuration JSON (opaque).
        section_name: Matched EXACTLY (case-sensitive, not uppercased). Unknown name is
            NOT_FOUND.

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result is a JSON boolean: true for null or [], false otherwise (any non-array,
        non-null value such as an object counts as non-empty).
    """

def list_config_sections(
    config_json: str,
) -> str:
    """List the names of all top-level G2_CONFIG keys.

    Args:
        config_json: The configuration JSON (opaque).

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result is an array of key names in config order, including non-CFG keys
        (SETTINGS, SYS_OOM, CONFIG_BASE_VERSION). A missing or non-object G2_CONFIG
        yields [] rather than an error.
    """

def remove_config_section_field(
    config_json: str,
    section_name: str,
    field_name: str,
) -> str:
    """Remove a field from every item of an array section, returning how many items had
    it.

    Args:
        config_json: The configuration JSON (opaque).
        section_name: Uppercased before lookup. Must name an ARRAY section, else
            NOT_FOUND.
        field_name: Uppercased before removal (a lowercase key in a row can never be
            removed).

    Returns:
        The modified configuration JSON string. ``remove_config_section_field_result``
        (same arguments) returns the record this operation produces.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Record is the integer count of items the field was removed from (0 when no item
        had it; the config is still returned). Non-object items are skipped. A config
        with no G2_CONFIG key succeeds unchanged with count 0.
    """

def remove_config_section_field_result(
    config_json: str,
    section_name: str,
    field_name: str,
) -> str:
    """The record (row / ids) of ``remove_config_section_field``: same arguments and
    operation, but returns the record instead of the configuration. Operation: Remove
    a field from every item of an array section, returning how many items had it.

    Args:
        config_json: The configuration JSON (opaque).
        section_name: Uppercased before lookup. Must name an ARRAY section, else
            NOT_FOUND.
        field_name: Uppercased before removal (a lowercase key in a row can never be
            removed).

    Returns:
        The record (e.g. the created row or ids) as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Record is the integer count of items the field was removed from (0 when no item
        had it; the config is still returned). Non-object items are skipped. A config
        with no G2_CONFIG key succeeds unchanged with count 0.
    """

def add_config_section_field(
    config_json: str,
    section_name: str,
    field_name: str,
    field_value: Any,
) -> str:
    """Add a field to every item of an array section that lacks it, returning
    existed/updated counts.

    Args:
        config_json: The configuration JSON (opaque).
        section_name: Uppercased before lookup. Must name an ARRAY section, else
            NOT_FOUND.
        field_name: Uppercased before insertion.
        field_value: Any JSON value, stored verbatim (cloned) into each item that lacks
            the field.

    Returns:
        The modified configuration JSON string. ``add_config_section_field_result``
        (same arguments) returns the record this operation produces.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Record is {"existed": n, "updated": n}: items that already had the field (value
        preserved, never overwritten) vs. items it was inserted into. Non-object items
        are skipped (counted in neither). A config with no G2_CONFIG key succeeds
        unchanged with both counts 0.
    """

def add_config_section_field_result(
    config_json: str,
    section_name: str,
    field_name: str,
    field_value: Any,
) -> str:
    """The record (row / ids) of ``add_config_section_field``: same arguments and
    operation, but returns the record instead of the configuration. Operation: Add a
    field to every item of an array section that lacks it, returning existed/updated
    counts.

    Args:
        config_json: The configuration JSON (opaque).
        section_name: Uppercased before lookup. Must name an ARRAY section, else
            NOT_FOUND.
        field_name: Uppercased before insertion.
        field_value: Any JSON value, stored verbatim (cloned) into each item that lacks
            the field.

    Returns:
        The record (e.g. the created row or ids) as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Record is {"existed": n, "updated": n}: items that already had the field (value
        preserved, never overwritten) vs. items it was inserted into. Non-object items
        are skipped (counted in neither). A config with no G2_CONFIG key succeeds
        unchanged with both counts 0.
    """

def add_data_source(
    config_json: str,
    code: str,
    *,
    retention_level: str | None = None,
    id: int | None = None,
) -> str:
    """Add a data source (CFG_DSRC row) to the configuration.

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before storage and duplicate check; DSRC_DESC is set to the
            same uppercased code.
        retention_level: Case-insensitive; normalized to Remember or Forget. Any other
            value is INVALID_INPUT. ``None`` omits it. Library default when omitted:
            ``"Remember"``.
        id: Requested DSRC_ID. Absent OR <= 0 means auto-allocate (next free id, floor
            1000). A taken id > 0 is ALREADY_EXISTS. ``None`` omits it.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, ALREADY_EXISTS,
            INVALID_INPUT; any call may also raise INVALID_INPUT (bad arguments) or
            INTERNAL.
    """

def delete_data_source(
    config_json: str,
    code: str,
) -> str:
    """Delete a data source by code.

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before lookup.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, NOT_FOUND,
            INVALID_INPUT; any call may also raise INVALID_INPUT (bad arguments) or
            INTERNAL.

    Notes:
        System data sources (DSRC_ID <= 2, e.g. TEST and SEARCH in the template) are
        protected and fail with INVALID_INPUT. No dependency check is made against other
        sections.
    """

def get_data_source(
    config_json: str,
    code: str,
) -> str:
    """Get one data source's raw CFG_DSRC row by code.

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before lookup.

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, NOT_FOUND; any call
            may also raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result is the stored row with on-disk keys (DSRC_ID, DSRC_CODE, DSRC_DESC,
        RETENTION_LEVEL).
    """

def list_data_sources(
    config_json: str,
) -> str:
    """List all data sources as {id, dataSource} summaries.

    Args:
        config_json: The configuration JSON (opaque).

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION; any call may also
            raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result is an array of {"id": DSRC_ID, "dataSource": DSRC_CODE} in config order.
    """

def set_data_source(
    config_json: str,
    code: str,
    *,
    retention_level: str | None = None,
) -> str:
    """Update a data source's retention level.

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before lookup.
        retention_level: Absent leaves RETENTION_LEVEL unchanged. TRAP: unlike
            add_data_source the value is written VERBATIM, with no domain validation or
            case normalization. ``None`` omits it.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, NOT_FOUND; any call
            may also raise INVALID_INPUT (bad arguments) or INTERNAL.
    """

def add_element(
    config_json: str,
    code: str,
    *,
    description: str | None = None,
    data_type: str | None = None,
    id: int | None = None,
) -> str:
    """Add an element (CFG_FELEM row).

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased; duplicate (exact match on the uppercased code) is
            ALREADY_EXISTS.
        description: FELEM_DESC, stored verbatim; absent defaults to the uppercased
            code. ``None`` omits it.
        data_type: Case-insensitive; normalized to lowercase string, number, date,
            datetime or json, else INVALID_INPUT. ``None`` omits it. Library default
            when omitted: ``"string"``.
        id: Requested FELEM_ID. Absent OR <= 0 means auto-allocate (max existing + 1,
            floor 1000). A taken id > 0 is ALREADY_EXISTS. ``None`` omits it.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, ALREADY_EXISTS,
            INVALID_INPUT; any call may also raise INVALID_INPUT (bad arguments) or
            INTERNAL.

    Notes:
        Validation order: duplicate code, id, data_type.
    """

def delete_element(
    config_json: str,
    code: str,
) -> str:
    """Delete an element that no feature uses.

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before lookup; unknown is NOT_FOUND.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, NOT_FOUND,
            INVALID_INPUT, MISSING_FIELD; any call may also raise INVALID_INPUT (bad
            arguments) or INTERNAL.

    Notes:
        INVALID_INPUT when any CFG_FBOM row maps the element to an existing feature
        ("Element linked to the following feature(s): ..."). TRAP: CFG_ATTR rows naming
        the element are NOT checked and are left dangling. MISSING_SECTION for an absent
        CFG_FELEM or CFG_FBOM; MISSING_FIELD when the matched row's FELEM_ID is not an
        integer.
    """

def get_element(
    config_json: str,
    code: str,
) -> str:
    """Get one element as a display summary by code.

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before lookup; unknown is NOT_FOUND.

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, NOT_FOUND; any call
            may also raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result is exactly {id, element, datatype}; FELEM_DESC is NOT included.
    """

def list_elements(
    config_json: str,
) -> str:
    """List all elements as display summaries, sorted by element code.

    Args:
        config_json: The configuration JSON (opaque).

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION; any call may also
            raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Array of {id, element, datatype}, sorted alphabetically by element code (not by
        id).
    """

def set_element(
    config_json: str,
    code: str,
    *,
    description: str | None = None,
    data_type: str | None = None,
) -> str:
    """Update an element's description and/or data type.

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before lookup; unknown is NOT_FOUND.
        description: Absent leaves FELEM_DESC; else stored verbatim. ``None`` omits it.
        data_type: TRAP — absent leaves DATA_TYPE; else stored VERBATIM with no
            validation or normalization (unlike add_element). ``None`` omits it.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, NOT_FOUND; any call
            may also raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Succeeds unchanged when no update args are given.
    """

def set_feature_element(
    config_json: str,
    feature_code: str,
    element_code: str,
    *,
    exec_order: int | None = None,
    display_level: int | None = None,
    display_delim: str | None = None,
    derived: str | None = None,
) -> str:
    """Update one feature-element (CFG_FBOM) row's exec order, display level, delimiter
    or derived flag.

    Args:
        config_json: The configuration JSON (opaque).
        feature_code: Required. Case-insensitive; unknown is NOT_FOUND.
        element_code: Required. Case-insensitive; unknown is NOT_FOUND.
        exec_order: TRAP — stored verbatim; no uniqueness check and no <= 0
            auto-allocation. ``None`` omits it.
        display_level: Must be >= 0, else INVALID_INPUT. ``None`` omits it.
        display_delim: Stored verbatim. Not tri-state; cannot be cleared back to null.
            ``None`` omits it.
        derived: Case-insensitive Yes/No, normalized, else INVALID_INPUT. ``None`` omits
            it.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, NOT_FOUND,
            INVALID_INPUT; any call may also raise INVALID_INPUT (bad arguments) or
            INTERNAL.

    Notes:
        NOT_FOUND when the (feature, element) mapping is absent. Succeeds unchanged when
        no update args are given. A missing CFG_FTYPE/CFG_FELEM surfaces as NOT_FOUND
        (code lookup).
    """

def add_element_to_feature(
    config_json: str,
    feature_code: str,
    element_code: str,
    *,
    display_level: int | None = None,
    display_delim: str | None = None,
    derived: str | None = None,
) -> str:
    """Map an existing element to a feature (append a CFG_FBOM row).

    Args:
        config_json: The configuration JSON (opaque).
        feature_code: Case-insensitive; unknown is NOT_FOUND.
        element_code: Case-insensitive; unknown is NOT_FOUND (the element is never
            auto-created).
        display_level: Must be >= 0, else INVALID_INPUT. ``None`` omits it. Library
            default when omitted: ``1``.
        display_delim: Stored verbatim; absent stores null. ``None`` omits it.
        derived: Case-insensitive Yes/No, normalized, else INVALID_INPUT. ``None`` omits
            it. Library default when omitted: ``"No"``.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, NOT_FOUND,
            INVALID_INPUT, ALREADY_EXISTS; any call may also raise INVALID_INPUT (bad
            arguments) or INTERNAL.

    Notes:
        EXEC_ORDER is always auto-allocated as max(EXEC_ORDER over the WHOLE CFG_FBOM
        table) + 1 and cannot be requested (use features.add_feature_comparison for an
        explicit order). Duplicate (FTYPE_ID, FELEM_ID) is ALREADY_EXISTS. Validation
        order: feature, element, display_level, derived, CFG_FBOM section, duplicate.
    """

def delete_element_from_feature(
    config_json: str,
    feature_code: str,
    element_code: str,
) -> str:
    """Remove one feature-element (CFG_FBOM) mapping.

    Args:
        config_json: The configuration JSON (opaque).
        feature_code: Case-insensitive; unknown is NOT_FOUND.
        element_code: Case-insensitive; unknown is NOT_FOUND.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, NOT_FOUND; any call
            may also raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        NOT_FOUND when the mapping is absent. The CFG_FELEM row is kept.
    """

def render_config(
    config_json: str,
    indent: int,
) -> str:
    """Render a config document in canonical export form (recursively key-sorted,
    pretty-printed).

    Args:
        config_json: The configuration JSON (opaque).
        indent: Spaces per indentation level, applied verbatim (2 = CLI on-disk form, 4
            = Python parity; 0 allowed). Negative is INVALID_INPUT (raised by the
            req_usize converter, before the library is called).

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, INVALID_INPUT; any call may also
            raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result is a JSON STRING holding the rendered text (not the working config
        envelope): every object's keys sorted recursively (Python
        json.dumps(sort_keys=True)), array order kept, no trailing newline. The text is
        itself a valid config and may be passed back in. Any JSON document is accepted
        (no G2_CONFIG check).
    """

def add_feature(
    config_json: str,
    feature: str,
    element_list: Any,
    *,
    class_: str | None = None,
    behavior: str | None = None,
    candidates: str | None = None,
    anonymize: str | None = None,
    derived: str | None = None,
    history: str | None = None,
    matchkey: str | None = None,
    standardize: str | None = None,
    expression: str | None = None,
    comparison: str | None = None,
    version: int | None = None,
    rtype_id: int | None = None,
    id: int | None = None,
) -> str:
    """Add a feature (CFG_FTYPE row) with its element list (CFG_FBOM rows) and optional
    standardize/expression/comparison calls.

    Args:
        config_json: The configuration JSON (opaque).
        feature: Uppercased; stored as FTYPE_CODE and FTYPE_DESC. Duplicate (exact match
            on the uppercased code) is ALREADY_EXISTS.
        element_list: Must be a non-empty JSON array, else INVALID_INPUT. Each item is
            either an element-code string, or an object with `element` (or `ELEMENT`,
            required, else INVALID_INPUT) and optional `expressed`/`EXPRESSED`,
            `compared`/`COMPARED` ("yes" case-insensitive = true), `display`/`DISPLAY`
            ("yes" = DISPLAY_LEVEL 1, anything else 0) or
            `displaylevel`/`DISPLAYLEVEL`/`display_level` (int, default 1, negative =
            INVALID_INPUT), `displaydelim`/`DISPLAYDELIM`/`display_delim`,
            `derived`/`DERIVED` (Yes/No case-insensitive, else INVALID_INPUT; default
            No). Any other item type is INVALID_INPUT. Element codes are uppercased; a
            code not in CFG_FELEM is AUTO-CREATED (FELEM_ID max+1 floor 1000, DATA_TYPE
            string, FELEM_DESC = code). The FBOM EXEC_ORDER is the item's 1-based
            position (per feature, not whole-table). Shape: ``[string | {element?:
            string|null, ELEMENT?: string|null, expressed?: string|null, EXPRESSED?:
            string|null, compared?: string|null, COMPARED?: string|null, display?:
            string|null, DISPLAY?: string|null, displaylevel?: int|null, DISPLAYLEVEL?:
            int|null, display_level?: int|null, displaydelim?: string|null,
            DISPLAYDELIM?: string|null, display_delim?: string|null, derived?:
            string|null, DERIVED?: string|null}]``.
        class_: CFG_FCLASS code, case-insensitive; unknown is NOT_FOUND. Wire name
            ``class``. ``None`` omits it. Library default when omitted: ``"OTHER"``.
        behavior: Behavior code (A1, F1, FF, FM, FVM, NONE, NAME; E/S suffixes set
            FTYPE_EXCL/FTYPE_STAB), case-insensitive; otherwise INVALID_INPUT. ``None``
            omits it. Library default when omitted: ``"FM"``.
        candidates: USED_FOR_CAND; case-insensitive Yes/No, normalized, else
            INVALID_INPUT. ``None`` omits it. Library default when omitted: ``"No"``.
        anonymize: ANONYMIZE; case-insensitive Yes/No, normalized, else INVALID_INPUT.
            ``None`` omits it. Library default when omitted: ``"No"``.
        derived: DERIVED; case-insensitive Yes/No, normalized, else INVALID_INPUT.
            ``None`` omits it. Library default when omitted: ``"No"``.
        history: PERSIST_HISTORY; case-insensitive Yes/No, normalized, else
            INVALID_INPUT. ``None`` omits it. Library default when omitted: ``"Yes"``.
        matchkey: SHOW_IN_MATCH_KEY; case-insensitive Yes, No, Confirm or Denial,
            normalized, else INVALID_INPUT. Absent defaults to Yes when `comparison` is
            given, else No. ``None`` omits it.
        standardize: CFG_SFUNC code (case-insensitive) or NOT_FOUND (an empty string is
            looked up too, so "" is NOT_FOUND). Creates a CFG_SFCALL row (SFCALL_ID
            max+1 floor 1000, EXEC_ORDER 1, FELEM_ID -1). ``None`` omits it.
        expression: CFG_EFUNC code (case-insensitive) or NOT_FOUND ("" is NOT_FOUND).
            Requires at least one element_list item with expressed=yes, else
            INVALID_INPUT. Creates a CFG_EFCALL row (EFCALL_ID max+1 floor 1000,
            EXEC_ORDER 1) and a CFG_EFBOM row (FELEM_REQ Yes) per expressed element.
            ``None`` omits it.
        comparison: CFG_CFUNC code (case-insensitive) or NOT_FOUND ("" is NOT_FOUND).
            Requires at least one element_list item with compared=yes, else
            INVALID_INPUT. Creates a CFG_CFCALL row (CFCALL_ID max+1 floor 1000) and a
            CFG_CFBOM row per compared element. ``None`` omits it.
        version: Stored verbatim in VERSION. ``None`` omits it. Library default when
            omitted: ``1``.
        rtype_id: Stored verbatim in RTYPE_ID; not validated against CFG_RTYPE. ``None``
            omits it. Library default when omitted: ``0``.
        id: Requested FTYPE_ID. Absent OR <= 0 means auto-allocate (max existing + 1,
            floor 1000). A taken id > 0 is ALREADY_EXISTS. ``None`` omits it.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, ALREADY_EXISTS,
            INVALID_INPUT, NOT_FOUND, INVALID_STRUCTURE; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Validation order: CFG_FTYPE section, duplicate code, element_list shape,
        candidates/anonymize/derived/history/matchkey domains, id, behavior, class,
        function codes, expressed/compared counts, then per-element (display level,
        derived). The input config is never modified on error.
    """

def delete_feature(
    config_json: str,
    feature: str,
) -> str:
    """Delete a feature and cascade-delete its FBOM rows, attributes and
    standardize/expression/comparison/distinct calls.

    Args:
        config_json: The configuration JSON (opaque).
        feature: A string that parses (after trim) as an integer is an FTYPE_ID;
            otherwise a feature code, uppercased. Unknown is NOT_FOUND.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, NOT_FOUND,
            INVALID_INPUT; any call may also raise INVALID_INPUT (bad arguments) or
            INTERNAL.

    Notes:
        Locked features NAME, ADDRESS, PHONE, DOB, REL_LINK, REL_ANCHOR, REL_POINTER are
        INVALID_INPUT. Cascade removes the feature's CFG_FBOM rows, CFG_ATTR rows whose
        FTYPE_CODE matches, CFG_SFCALL, CFG_EFCALL (+ their CFG_EFBOM), CFG_CFCALL (+
        CFG_CFBOM) and CFG_DFCALL (+ CFG_DFBOM) rows; CFG_FELEM rows are kept. A missing
        CFG_FTYPE is MISSING_SECTION for an id but NOT_FOUND for a code.
    """

def get_feature(
    config_json: str,
    feature: str,
) -> str:
    """Get one feature as a display summary including its elementList.

    Args:
        config_json: The configuration JSON (opaque).
        feature: Integer string (after trim) = FTYPE_ID; otherwise a code, uppercased.

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result {id, feature, class, behavior, anonymize, candidates, standardize,
        expression, comparison, matchKey, version, elementList}; elementList items
        {element, expressed, compared, derived, display} sorted by EXEC_ORDER, display =
        "No" iff DISPLAY_LEVEL is 0. standardize/expression/comparison are "" when
        absent. DERIVED, PERSIST_HISTORY and RTYPE_ID of the feature are NOT in the
        result. A missing CFG_FTYPE section is NOT_FOUND (never MISSING_SECTION).
    """

def list_features(
    config_json: str,
) -> str:
    """List all features as display summaries, sorted by FTYPE_ID.

    Args:
        config_json: The configuration JSON (opaque).

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION; any call may also
            raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Array of the same objects get_feature returns.
    """

def set_feature(
    config_json: str,
    feature: str,
    *,
    candidates: str | None = None,
    anonymize: str | None = None,
    derived: str | None = None,
    history: str | None = None,
    matchkey: str | None = None,
    behavior: str | None = None,
    class_: str | None = None,
    version: int | None = None,
    rtype_id: int | None = None,
) -> str:
    """Update a feature's flags, behavior, class, version or RTYPE_ID.

    Args:
        config_json: The configuration JSON (opaque).
        feature: Integer string (after trim) = FTYPE_ID; otherwise a code, uppercased.
            Unknown is NOT_FOUND.
        candidates: USED_FOR_CAND; case-insensitive Yes/No, normalized, else
            INVALID_INPUT. ``None`` omits it.
        anonymize: TRAP — stored VERBATIM in ANONYMIZE (no validation or normalization,
            unlike add_feature). ``None`` omits it.
        derived: TRAP — stored VERBATIM in DERIVED (no validation or normalization).
            ``None`` omits it.
        history: TRAP — stored VERBATIM in PERSIST_HISTORY (no validation or
            normalization). ``None`` omits it.
        matchkey: SHOW_IN_MATCH_KEY; case-insensitive Yes, No, Confirm or Denial,
            normalized, else INVALID_INPUT. ``None`` omits it.
        behavior: Behavior code (case-insensitive) parsed into
            FTYPE_FREQ/FTYPE_EXCL/FTYPE_STAB, else INVALID_INPUT. ``None`` omits it.
        class_: CFG_FCLASS code, case-insensitive; unknown is NOT_FOUND. Wire name
            ``class``. ``None`` omits it.
        version: Stored verbatim in VERSION. ``None`` omits it.
        rtype_id: Stored verbatim in RTYPE_ID; not validated. ``None`` omits it.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, NOT_FOUND,
            INVALID_INPUT; any call may also raise INVALID_INPUT (bad arguments) or
            INTERNAL.

    Notes:
        If no supplied value differs from the stored row (including when no update args
        are given) the call fails with INVALID_INPUT "No changes detected". The input
        config is never modified on error.
    """

def add_feature_comparison(
    config_json: str,
    feature_code: str,
    element_code: str,
    *,
    exec_order: int | None = None,
    display_level: int | None = None,
    display_delim: str | None = None,
    derived: str | None = None,
) -> str:
    """Add a feature-element (CFG_FBOM) row with an optional explicit EXEC_ORDER.

    Args:
        config_json: The configuration JSON (opaque).
        feature_code: Required. Feature code, case-insensitive; unknown is NOT_FOUND.
        element_code: Required. Element code, case-insensitive; unknown is NOT_FOUND.
        exec_order: WHOLE-TABLE scope: absent or <= 0 auto-allocates max(EXEC_ORDER over
            all of CFG_FBOM) + 1; a value > 0 already used by ANY CFG_FBOM row is
            ALREADY_EXISTS. ``None`` omits it.
        display_level: TRAP — stored verbatim, NOT validated, and absent stores
            DISPLAY_LEVEL null (no default 1, unlike add_element_to_feature). ``None``
            omits it.
        display_delim: Stored verbatim; absent stores null. ``None`` omits it.
        derived: TRAP — stored verbatim, NOT validated; absent stores DERIVED null.
            ``None`` omits it.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, NOT_FOUND,
            ALREADY_EXISTS; any call may also raise INVALID_INPUT (bad arguments) or
            INTERNAL.

    Notes:
        Writes the same table as elements.add_element_to_feature. Duplicate (FTYPE_ID,
        FELEM_ID) is ALREADY_EXISTS. A missing CFG_FTYPE/CFG_FELEM section surfaces as
        NOT_FOUND (code lookup); a missing CFG_FBOM as MISSING_SECTION.
    """

def delete_feature_comparison(
    config_json: str,
    feature_code: str,
    element_code: str,
) -> str:
    """Delete one feature-element (CFG_FBOM) row.

    Args:
        config_json: The configuration JSON (opaque).
        feature_code: Case-insensitive; unknown is NOT_FOUND.
        element_code: Case-insensitive; unknown is NOT_FOUND.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Same effect as elements.delete_element_from_feature, except a missing CFG_FBOM
        section is NOT_FOUND here (MISSING_SECTION there). An absent mapping is
        NOT_FOUND.
    """

def get_feature_comparison(
    config_json: str,
    feature_code: str,
    element_code: str,
) -> str:
    """Get one raw CFG_FBOM row by feature and element code.

    Args:
        config_json: The configuration JSON (opaque).
        feature_code: Required. Case-insensitive; unknown is NOT_FOUND.
        element_code: Required. Case-insensitive; unknown is NOT_FOUND.

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, NOT_FOUND; any call
            may also raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result uses on-disk keys (FTYPE_ID, FELEM_ID, EXEC_ORDER, DISPLAY_LEVEL,
        DISPLAY_DELIM, DERIVED).
    """

def list_feature_comparisons(
    config_json: str,
) -> str:
    """List all raw CFG_FBOM rows sorted by (FTYPE_ID, EXEC_ORDER).

    Args:
        config_json: The configuration JSON (opaque).

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION; any call may also
            raise INVALID_INPUT (bad arguments) or INTERNAL.
    """

def add_feature_distinct_call_element(
    config_json: str,
    feature_code: str,
    distinct_func_code: str,
    *,
    element_code: str | None = None,
    exec_order: int | None = None,
) -> str:
    """Add a distinct-function call (CFG_DFCALL row) for a feature.

    Args:
        config_json: The configuration JSON (opaque).
        feature_code: Required. Case-insensitive; unknown is NOT_FOUND.
        distinct_func_code: Required. CFG_DFUNC code, case-insensitive; unknown is
            NOT_FOUND.
        element_code: TRAP — only validated (unknown is NOT_FOUND); it is NOT stored
            (CFG_DFCALL has no FELEM_ID column). ``None`` omits it.
        exec_order: TRAP — IGNORED entirely (CFG_DFCALL has no EXEC_ORDER column).
            ``None`` omits it.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, NOT_FOUND,
            ALREADY_EXISTS; any call may also raise INVALID_INPUT (bad arguments) or
            INTERNAL.

    Notes:
        Duplicate (FTYPE_ID, DFUNC_ID) is ALREADY_EXISTS. New row is exactly {DFCALL_ID
        (max+1 floor 1000), FTYPE_ID, DFUNC_ID}; no CFG_DFBOM rows are written.
    """

def list_feature_classes(
    config_json: str,
) -> str:
    """List all raw CFG_FCLASS rows sorted by FCLASS_ID.

    Args:
        config_json: The configuration JSON (opaque).

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION; any call may also
            raise INVALID_INPUT (bad arguments) or INTERNAL.
    """

def get_feature_class(
    config_json: str,
    feature_class: str,
) -> str:
    """Get one raw CFG_FCLASS row by id or code.

    Args:
        config_json: The configuration JSON (opaque).
        feature_class: Integer string (after trim) = FCLASS_ID; otherwise a code,
            uppercased. Unknown is NOT_FOUND.

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, NOT_FOUND; any call
            may also raise INVALID_INPUT (bad arguments) or INTERNAL.
    """

def update_feature_version(
    config_json: str,
    version: str,
) -> str:
    """Set G2_CONFIG.CONFIG_BASE_VERSION.COMPATIBILITY_VERSION.FEATURE_VERSION.

    Args:
        config_json: The configuration JSON (opaque).
        version: Stored verbatim as a string (inserted or overwritten); not validated.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION; any call may also
            raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        MISSING_SECTION when COMPATIBILITY_VERSION is absent or not an object. No
        manifest function reads FEATURE_VERSION back (versioning reads CONFIG_VERSION).
    """

def add_fragment(
    config_json: str,
    fragment_config: Any,
) -> str:
    """Add a rule fragment (CFG_ERFRAG row), returning the assigned ERFRAG_ID.

    Args:
        config_json: The configuration JSON (opaque).
        fragment_config: Object with on-disk keys. ERFRAG_CODE (string, required, else
            MISSING_FIELD) is uppercased for storage and the duplicate check
            (ALREADY_EXISTS). ERFRAG_SOURCE (string, required, else MISSING_FIELD) is
            stored verbatim; every name referenced inside a FRAGMENT[...] clause (e.g.
            "./FRAGMENT[./SAME_NAME>0 and ./SAME_STAB>0]") must be an existing
            ERFRAG_CODE matched EXACTLY (case-sensitive), else INVALID_INPUT. A source
            without FRAGMENT[ (including "") is accepted unvalidated. ERFRAG_ID
            (integer, optional): absent or <= 0 auto-allocates (max + 1, floor 1, so
            1000 on the template); a taken id > 0 is ALREADY_EXISTS. Any ERFRAG_DESC key
            is IGNORED. Shape: ``{ERFRAG_CODE: string, ERFRAG_SOURCE: string,
            ERFRAG_ID?: int|null, ERFRAG_DESC?: any, ERFRAG_DEPENDS?: any}``.

    Returns:
        The modified configuration JSON string. ``add_fragment_result`` (same arguments)
        returns the record this operation produces.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_FIELD, ALREADY_EXISTS,
            INVALID_INPUT, INVALID_CONFIG, MISSING_SECTION; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Record is the assigned ERFRAG_ID (integer). The row always carries every
        CFG_ERFRAG key: ERFRAG_DESC is set to the uppercased code, ERFRAG_DEPENDS is the
        referenced fragments' ids sorted as STRINGS, deduplicated and comma-joined
        ("11,61"), or null when there are none. ERFRAG_CODE and ERFRAG_SOURCE are
        checked BEFORE the config is parsed (MISSING_FIELD wins). A config without
        G2_CONFIG is INVALID_CONFIG; with G2_CONFIG but no CFG_ERFRAG it is
        MISSING_SECTION.
    """

def add_fragment_result(
    config_json: str,
    fragment_config: Any,
) -> str:
    """The record (row / ids) of ``add_fragment``: same arguments and operation, but
    returns the record instead of the configuration. Operation: Add a rule fragment
    (CFG_ERFRAG row), returning the assigned ERFRAG_ID.

    Args:
        config_json: The configuration JSON (opaque).
        fragment_config: Object with on-disk keys. ERFRAG_CODE (string, required, else
            MISSING_FIELD) is uppercased for storage and the duplicate check
            (ALREADY_EXISTS). ERFRAG_SOURCE (string, required, else MISSING_FIELD) is
            stored verbatim; every name referenced inside a FRAGMENT[...] clause (e.g.
            "./FRAGMENT[./SAME_NAME>0 and ./SAME_STAB>0]") must be an existing
            ERFRAG_CODE matched EXACTLY (case-sensitive), else INVALID_INPUT. A source
            without FRAGMENT[ (including "") is accepted unvalidated. ERFRAG_ID
            (integer, optional): absent or <= 0 auto-allocates (max + 1, floor 1, so
            1000 on the template); a taken id > 0 is ALREADY_EXISTS. Any ERFRAG_DESC key
            is IGNORED. Shape: ``{ERFRAG_CODE: string, ERFRAG_SOURCE: string,
            ERFRAG_ID?: int|null, ERFRAG_DESC?: any, ERFRAG_DEPENDS?: any}``.

    Returns:
        The record (e.g. the created row or ids) as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_FIELD, ALREADY_EXISTS,
            INVALID_INPUT, INVALID_CONFIG, MISSING_SECTION; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Record is the assigned ERFRAG_ID (integer). The row always carries every
        CFG_ERFRAG key: ERFRAG_DESC is set to the uppercased code, ERFRAG_DEPENDS is the
        referenced fragments' ids sorted as STRINGS, deduplicated and comma-joined
        ("11,61"), or null when there are none. ERFRAG_CODE and ERFRAG_SOURCE are
        checked BEFORE the config is parsed (MISSING_FIELD wins). A config without
        G2_CONFIG is INVALID_CONFIG; with G2_CONFIG but no CFG_ERFRAG it is
        MISSING_SECTION.
    """

def delete_fragment(
    config_json: str,
    code: str,
) -> str:
    """Delete a fragment by code.

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased, then matched exactly against ERFRAG_CODE. Not an id.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        TRAP: no dependency check; a fragment still referenced by a rule
        (QUAL_ERFRAG_CODE / DISQ_ERFRAG_CODE) or by another fragment's ERFRAG_DEPENDS is
        deleted anyway, leaving dangling references. A config without CFG_ERFRAG is
        NOT_FOUND.
    """

def get_fragment(
    config_json: str,
    code_or_id: str,
) -> str:
    """Get one fragment, by code or ERFRAG_ID, as a summary record.

    Args:
        config_json: The configuration JSON (opaque).
        code_or_id: Uppercased, then matched exactly against ERFRAG_CODE first, then
            numerically against ERFRAG_ID (e.g. "11").

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result is {id, fragment, source, depends}: id/source/depends are null-preserving
        projections of ERFRAG_ID/ERFRAG_SOURCE/ERFRAG_DEPENDS; fragment is ERFRAG_CODE
        or "" when absent. ERFRAG_DESC is not reported.
    """

def list_fragments(
    config_json: str,
) -> str:
    """List all fragments as summary records in config order.

    Args:
        config_json: The configuration JSON (opaque).

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result is an array of the get_fragment record shape, in config order (NOT
        sorted). A missing CFG_ERFRAG or G2_CONFIG yields [].
    """

def set_fragment(
    config_json: str,
    code: str,
    *,
    source: str | None | UnsetType = ...,
    description: str | None | UnsetType = ...,
) -> str:
    """Update a fragment's source and/or description.

    Args:
        config_json: The configuration JSON (opaque).
        code: Fragment code; uppercased, then matched exactly. Unknown is NOT_FOUND.
        source: ERFRAG_SOURCE. Absent = keep source AND ERFRAG_DEPENDS; null = clear
            BOTH source and ERFRAG_DEPENDS to null; a string is validated exactly as in
            add_fragment (INVALID_INPUT) and ERFRAG_DEPENDS is recomputed. ``UNSET``
            leaves it, ``None`` clears it.
        description: ERFRAG_DESC. Absent = keep; null = clear to null; a string is
            stored verbatim. ``UNSET`` leaves it, ``None`` clears it.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND, INVALID_INPUT; any call
            may also raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        The row is rewritten with every CFG_ERFRAG key; ERFRAG_ID is preserved.
    """

def add_comparison_function(
    config_json: str,
    code: str,
    *,
    connect_str: str | None = None,
    description: str | None = None,
    language: str | None = None,
    anon_support: str | None = None,
) -> str:
    """Add a comparison function (CFG_CFUNC row).

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before the duplicate check and storage (CFUNC_CODE).
        connect_str: Absent stores CONNECT_STR null; any string (including "") is stored
            verbatim. ``None`` omits it.
        description: Absent stores CFUNC_DESC null; any string is stored verbatim.
            ``None`` omits it.
        language: Absent stores LANGUAGE null; any string is stored verbatim. ``None``
            omits it.
        anon_support: Case-insensitive; normalized to Yes or No, else INVALID_INPUT.
            ``None`` omits it. Library default when omitted: ``"No"``.

    Returns:
        The modified configuration JSON string. ``add_comparison_function_result`` (same
        arguments) returns the record this operation produces.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, INVALID_INPUT; any
            call may also raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Returns (modified config, the new complete CFG_CFUNC row: CFUNC_ID, CFUNC_CODE,
        CONNECT_STR, ANON_SUPPORT, CFUNC_DESC, LANGUAGE). CFUNC_ID is always
        auto-allocated (max existing + 1, floor 1); no id can be requested. TRAP: a
        duplicate code is INVALID_INPUT (SzConfigError::validation), NOT ALREADY_EXISTS.
        Validation order: duplicate code, anon_support, then section. MISSING_SECTION
        only when CFG_CFUNC is absent.
    """

def add_comparison_function_result(
    config_json: str,
    code: str,
    *,
    connect_str: str | None = None,
    description: str | None = None,
    language: str | None = None,
    anon_support: str | None = None,
) -> str:
    """The record (row / ids) of ``add_comparison_function``: same arguments and
    operation, but returns the record instead of the configuration. Operation: Add a
    comparison function (CFG_CFUNC row).

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before the duplicate check and storage (CFUNC_CODE).
        connect_str: Absent stores CONNECT_STR null; any string (including "") is stored
            verbatim. ``None`` omits it.
        description: Absent stores CFUNC_DESC null; any string is stored verbatim.
            ``None`` omits it.
        language: Absent stores LANGUAGE null; any string is stored verbatim. ``None``
            omits it.
        anon_support: Case-insensitive; normalized to Yes or No, else INVALID_INPUT.
            ``None`` omits it. Library default when omitted: ``"No"``.

    Returns:
        The record (e.g. the created row or ids) as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, INVALID_INPUT; any
            call may also raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Returns (modified config, the new complete CFG_CFUNC row: CFUNC_ID, CFUNC_CODE,
        CONNECT_STR, ANON_SUPPORT, CFUNC_DESC, LANGUAGE). CFUNC_ID is always
        auto-allocated (max existing + 1, floor 1); no id can be requested. TRAP: a
        duplicate code is INVALID_INPUT (SzConfigError::validation), NOT ALREADY_EXISTS.
        Validation order: duplicate code, anon_support, then section. MISSING_SECTION
        only when CFG_CFUNC is absent.
    """

def delete_comparison_function(
    config_json: str,
    code: str,
) -> str:
    """Delete a comparison function's CFG_CFUNC row only (no cascade).

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before lookup.

    Returns:
        The modified configuration JSON string. ``delete_comparison_function_result``
        (same arguments) returns the record this operation produces.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Returns (modified config, the deleted CFG_CFUNC row). Removes ONLY the CFG_CFUNC
        row; CFG_CFCALL rows referencing it are left dangling (use
        delete_comparison_function_cascade). A missing CFG_CFUNC section is NOT_FOUND
        (not MISSING_SECTION).
    """

def delete_comparison_function_result(
    config_json: str,
    code: str,
) -> str:
    """The record (row / ids) of ``delete_comparison_function``: same arguments and
    operation, but returns the record instead of the configuration. Operation: Delete
    a comparison function's CFG_CFUNC row only (no cascade).

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before lookup.

    Returns:
        The record (e.g. the created row or ids) as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Returns (modified config, the deleted CFG_CFUNC row). Removes ONLY the CFG_CFUNC
        row; CFG_CFCALL rows referencing it are left dangling (use
        delete_comparison_function_cascade). A missing CFG_CFUNC section is NOT_FOUND
        (not MISSING_SECTION).
    """

def delete_comparison_function_cascade(
    config_json: str,
    code: str,
) -> str:
    """Delete a comparison function and its CFG_CFBOM / CFG_CFCALL / CFG_CFRTN rows.

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before lookup.

    Returns:
        The modified configuration JSON string.
        ``delete_comparison_function_cascade_result`` (same arguments) returns the
        record this operation produces.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND, MISSING_FIELD; any call
            may also raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Returns (modified config, the deleted CFG_CFUNC row). Order: CFG_CFBOM rows
        whose CFCALL_ID belongs to one of the function's CFG_CFCALL rows; every
        CFG_CFCALL row with that CFUNC_ID; every CFG_CFRTN row with that CFUNC_ID
        (well-formed rows via thresholds::delete_comparison_threshold, then a sweep of
        the rest); finally the CFG_CFUNC row. Absent CFBOM/CFCALL/CFRTN sections are
        skipped. MISSING_FIELD when the found row has no integer CFUNC_ID. A missing
        CFG_CFUNC section is NOT_FOUND.
    """

def delete_comparison_function_cascade_result(
    config_json: str,
    code: str,
) -> str:
    """The record (row / ids) of ``delete_comparison_function_cascade``: same arguments
    and operation, but returns the record instead of the configuration. Operation:
    Delete a comparison function and its CFG_CFBOM / CFG_CFCALL / CFG_CFRTN rows.

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before lookup.

    Returns:
        The record (e.g. the created row or ids) as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND, MISSING_FIELD; any call
            may also raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Returns (modified config, the deleted CFG_CFUNC row). Order: CFG_CFBOM rows
        whose CFCALL_ID belongs to one of the function's CFG_CFCALL rows; every
        CFG_CFCALL row with that CFUNC_ID; every CFG_CFRTN row with that CFUNC_ID
        (well-formed rows via thresholds::delete_comparison_threshold, then a sweep of
        the rest); finally the CFG_CFUNC row. Absent CFBOM/CFCALL/CFRTN sections are
        skipped. MISSING_FIELD when the found row has no integer CFUNC_ID. A missing
        CFG_CFUNC section is NOT_FOUND.
    """

def get_comparison_function(
    config_json: str,
    code: str,
) -> str:
    """Get one comparison function's raw CFG_CFUNC row by code.

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before lookup.

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result uses on-disk keys (CFUNC_ID, CFUNC_CODE, CFUNC_DESC, CONNECT_STR,
        ANON_SUPPORT, LANGUAGE). A missing CFG_CFUNC section is NOT_FOUND (not
        MISSING_SECTION).
    """

def list_comparison_functions(
    config_json: str,
) -> str:
    """List all comparison functions as camelCase summaries.

    Args:
        config_json: The configuration JSON (opaque).

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result is an array of {id, function, description, connectStr, anonSupport,
        language} in config order; all but function are null-preserving. A missing
        CFG_CFUNC section yields [] (no error).
    """

def set_comparison_function(
    config_json: str,
    code: str,
    *,
    connect_str: str | None | UnsetType = ...,
    description: str | None = None,
    language: str | None = None,
    anon_support: str | None = None,
) -> str:
    """Update a comparison function's connect string / description / language / anon
    support.

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before lookup.
        connect_str: Absent leaves CONNECT_STR; null clears it to null; a string
            (including "") sets it. ``UNSET`` leaves it, ``None`` clears it.
        description: Absent leaves CFUNC_DESC; a string is stored verbatim. Cannot be
            cleared to null. ``None`` omits it.
        language: Absent leaves LANGUAGE; a string is stored verbatim. Cannot be cleared
            to null. ``None`` omits it.
        anon_support: Absent leaves ANON_SUPPORT. TRAP: unlike add, NOT validated or
            normalized; any string is stored verbatim. ``None`` omits it.

    Returns:
        The modified configuration JSON string. ``set_comparison_function_result`` (same
        arguments) returns the record this operation produces.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Returns (modified config, the updated CFG_CFUNC row). No value validation. The
        row is deleted and re-appended, so it moves to the END of CFG_CFUNC. A missing
        CFG_CFUNC section is NOT_FOUND.
    """

def set_comparison_function_result(
    config_json: str,
    code: str,
    *,
    connect_str: str | None | UnsetType = ...,
    description: str | None = None,
    language: str | None = None,
    anon_support: str | None = None,
) -> str:
    """The record (row / ids) of ``set_comparison_function``: same arguments and
    operation, but returns the record instead of the configuration. Operation: Update
    a comparison function's connect string / description / language / anon support.

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before lookup.
        connect_str: Absent leaves CONNECT_STR; null clears it to null; a string
            (including "") sets it. ``UNSET`` leaves it, ``None`` clears it.
        description: Absent leaves CFUNC_DESC; a string is stored verbatim. Cannot be
            cleared to null. ``None`` omits it.
        language: Absent leaves LANGUAGE; a string is stored verbatim. Cannot be cleared
            to null. ``None`` omits it.
        anon_support: Absent leaves ANON_SUPPORT. TRAP: unlike add, NOT validated or
            normalized; any string is stored verbatim. ``None`` omits it.

    Returns:
        The record (e.g. the created row or ids) as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Returns (modified config, the updated CFG_CFUNC row). No value validation. The
        row is deleted and re-appended, so it moves to the END of CFG_CFUNC. A missing
        CFG_CFUNC section is NOT_FOUND.
    """

def add_distinct_function(
    config_json: str,
    code: str,
    *,
    connect_str: str | None = None,
    description: str | None = None,
    language: str | None = None,
    anon_support: str | None = None,
) -> str:
    """Add a distinct function (CFG_DFUNC row) to the configuration.

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before the duplicate check and storage (DFUNC_CODE).
        connect_str: Absent stores CONNECT_STR null; any string (including "") is stored
            verbatim. ``None`` omits it.
        description: Stored verbatim in DFUNC_DESC; absent stores null (NOT defaulted to
            the code). ``None`` omits it.
        language: Stored verbatim in LANGUAGE; absent stores null. ``None`` omits it.
        anon_support: Case-insensitive; normalized to Yes or No, any other value is
            INVALID_INPUT. ``None`` omits it. Library default when omitted: ``"No"``.

    Returns:
        The modified configuration JSON string. ``add_distinct_function_result`` (same
        arguments) returns the record this operation produces.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, INVALID_INPUT; any
            call may also raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Returns (modified config, the new complete CFG_DFUNC row: DFUNC_ID, DFUNC_CODE,
        DFUNC_DESC, CONNECT_STR, ANON_SUPPORT, LANGUAGE; unset optionals are null).
        DFUNC_ID is auto-allocated as max existing + 1 (floor 1); no id can be
        requested. TRAP: a duplicate code is INVALID_INPUT, not ALREADY_EXISTS
        (SzConfigError::validation). The duplicate check runs before anon_support
        validation. MISSING_SECTION only when G2_CONFIG.CFG_DFUNC is absent.
    """

def add_distinct_function_result(
    config_json: str,
    code: str,
    *,
    connect_str: str | None = None,
    description: str | None = None,
    language: str | None = None,
    anon_support: str | None = None,
) -> str:
    """The record (row / ids) of ``add_distinct_function``: same arguments and
    operation, but returns the record instead of the configuration. Operation: Add a
    distinct function (CFG_DFUNC row) to the configuration.

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before the duplicate check and storage (DFUNC_CODE).
        connect_str: Absent stores CONNECT_STR null; any string (including "") is stored
            verbatim. ``None`` omits it.
        description: Stored verbatim in DFUNC_DESC; absent stores null (NOT defaulted to
            the code). ``None`` omits it.
        language: Stored verbatim in LANGUAGE; absent stores null. ``None`` omits it.
        anon_support: Case-insensitive; normalized to Yes or No, any other value is
            INVALID_INPUT. ``None`` omits it. Library default when omitted: ``"No"``.

    Returns:
        The record (e.g. the created row or ids) as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, INVALID_INPUT; any
            call may also raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Returns (modified config, the new complete CFG_DFUNC row: DFUNC_ID, DFUNC_CODE,
        DFUNC_DESC, CONNECT_STR, ANON_SUPPORT, LANGUAGE; unset optionals are null).
        DFUNC_ID is auto-allocated as max existing + 1 (floor 1); no id can be
        requested. TRAP: a duplicate code is INVALID_INPUT, not ALREADY_EXISTS
        (SzConfigError::validation). The duplicate check runs before anon_support
        validation. MISSING_SECTION only when G2_CONFIG.CFG_DFUNC is absent.
    """

def delete_distinct_function(
    config_json: str,
    code: str,
) -> str:
    """Delete a distinct function by code.

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before lookup.

    Returns:
        The modified configuration JSON string. ``delete_distinct_function_result``
        (same arguments) returns the record this operation produces.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Returns (modified config, the deleted CFG_DFUNC row). No dependency check:
        CFG_DFCALL rows referencing the DFUNC_ID are left in place. A config without
        CFG_DFUNC is NOT_FOUND (not MISSING_SECTION).
    """

def delete_distinct_function_result(
    config_json: str,
    code: str,
) -> str:
    """The record (row / ids) of ``delete_distinct_function``: same arguments and
    operation, but returns the record instead of the configuration. Operation: Delete
    a distinct function by code.

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before lookup.

    Returns:
        The record (e.g. the created row or ids) as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Returns (modified config, the deleted CFG_DFUNC row). No dependency check:
        CFG_DFCALL rows referencing the DFUNC_ID are left in place. A config without
        CFG_DFUNC is NOT_FOUND (not MISSING_SECTION).
    """

def get_distinct_function(
    config_json: str,
    code: str,
) -> str:
    """Get one distinct function's raw CFG_DFUNC row by code.

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before lookup.

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result is the stored row with on-disk keys. A config without CFG_DFUNC is
        NOT_FOUND.
    """

def list_distinct_functions(
    config_json: str,
) -> str:
    """List all distinct functions as {id, function, connectStr, anonSupport, language}
    summaries.

    Args:
        config_json: The configuration JSON (opaque).

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result is an array in config order; id/connectStr/anonSupport/language are
        null-preserving (stored null stays null), DFUNC_DESC is not included. A config
        without G2_CONFIG.CFG_DFUNC yields [] (no MISSING_SECTION).
    """

def set_distinct_function(
    config_json: str,
    code: str,
    *,
    connect_str: str | None | UnsetType = ...,
    description: str | None = None,
    language: str | None = None,
    anon_support: str | None = None,
) -> str:
    """Update a distinct function's connect string, description, language or anon
    support.

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before lookup.
        connect_str: Absent leaves CONNECT_STR; null writes null; a string (including
            "") is written. ``UNSET`` leaves it, ``None`` clears it.
        description: Absent leaves DFUNC_DESC; a string is written verbatim. ``None``
            omits it.
        language: Absent leaves LANGUAGE; a string is written verbatim. ``None`` omits
            it.
        anon_support: Absent leaves ANON_SUPPORT. TRAP: unlike add_distinct_function the
            value is written VERBATIM, with no Yes/No validation or case normalization.
            ``None`` omits it.

    Returns:
        The modified configuration JSON string. ``set_distinct_function_result`` (same
        arguments) returns the record this operation produces.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Returns (modified config, the updated CFG_DFUNC row). The row is removed and
        re-appended, so it moves to the END of CFG_DFUNC (list order changes). A config
        without CFG_DFUNC is NOT_FOUND.
    """

def set_distinct_function_result(
    config_json: str,
    code: str,
    *,
    connect_str: str | None | UnsetType = ...,
    description: str | None = None,
    language: str | None = None,
    anon_support: str | None = None,
) -> str:
    """The record (row / ids) of ``set_distinct_function``: same arguments and
    operation, but returns the record instead of the configuration. Operation: Update
    a distinct function's connect string, description, language or anon support.

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before lookup.
        connect_str: Absent leaves CONNECT_STR; null writes null; a string (including
            "") is written. ``UNSET`` leaves it, ``None`` clears it.
        description: Absent leaves DFUNC_DESC; a string is written verbatim. ``None``
            omits it.
        language: Absent leaves LANGUAGE; a string is written verbatim. ``None`` omits
            it.
        anon_support: Absent leaves ANON_SUPPORT. TRAP: unlike add_distinct_function the
            value is written VERBATIM, with no Yes/No validation or case normalization.
            ``None`` omits it.

    Returns:
        The record (e.g. the created row or ids) as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Returns (modified config, the updated CFG_DFUNC row). The row is removed and
        re-appended, so it moves to the END of CFG_DFUNC (list order changes). A config
        without CFG_DFUNC is NOT_FOUND.
    """

def add_expression_function(
    config_json: str,
    code: str,
    *,
    connect_str: str | None = None,
    description: str | None = None,
    language: str | None = None,
) -> str:
    """Add an expression function (CFG_EFUNC row).

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before the duplicate check and storage (EFUNC_CODE).
        connect_str: Absent stores CONNECT_STR null; any string (including "") is stored
            verbatim. ``None`` omits it.
        description: Absent stores EFUNC_DESC null; any string is stored verbatim.
            ``None`` omits it.
        language: Absent stores LANGUAGE null; any string is stored verbatim. ``None``
            omits it.

    Returns:
        The modified configuration JSON string. ``add_expression_function_result`` (same
        arguments) returns the record this operation produces.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, INVALID_INPUT; any
            call may also raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Returns (modified config, the new complete CFG_EFUNC row: EFUNC_ID, EFUNC_CODE,
        CONNECT_STR, EFUNC_DESC, LANGUAGE). EFUNC_ID is always auto-allocated (max
        existing + 1, floor 1); no id can be requested. TRAP: a duplicate code is
        INVALID_INPUT (SzConfigError::validation), NOT ALREADY_EXISTS. MISSING_SECTION
        only when CFG_EFUNC is absent.
    """

def add_expression_function_result(
    config_json: str,
    code: str,
    *,
    connect_str: str | None = None,
    description: str | None = None,
    language: str | None = None,
) -> str:
    """The record (row / ids) of ``add_expression_function``: same arguments and
    operation, but returns the record instead of the configuration. Operation: Add an
    expression function (CFG_EFUNC row).

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before the duplicate check and storage (EFUNC_CODE).
        connect_str: Absent stores CONNECT_STR null; any string (including "") is stored
            verbatim. ``None`` omits it.
        description: Absent stores EFUNC_DESC null; any string is stored verbatim.
            ``None`` omits it.
        language: Absent stores LANGUAGE null; any string is stored verbatim. ``None``
            omits it.

    Returns:
        The record (e.g. the created row or ids) as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, INVALID_INPUT; any
            call may also raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Returns (modified config, the new complete CFG_EFUNC row: EFUNC_ID, EFUNC_CODE,
        CONNECT_STR, EFUNC_DESC, LANGUAGE). EFUNC_ID is always auto-allocated (max
        existing + 1, floor 1); no id can be requested. TRAP: a duplicate code is
        INVALID_INPUT (SzConfigError::validation), NOT ALREADY_EXISTS. MISSING_SECTION
        only when CFG_EFUNC is absent.
    """

def delete_expression_function(
    config_json: str,
    code: str,
) -> str:
    """Delete an expression function's CFG_EFUNC row only (no cascade).

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before lookup.

    Returns:
        The modified configuration JSON string. ``delete_expression_function_result``
        (same arguments) returns the record this operation produces.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Returns (modified config, the deleted CFG_EFUNC row). Removes ONLY the CFG_EFUNC
        row; CFG_EFCALL rows referencing it are left dangling (use
        delete_expression_function_cascade). A missing CFG_EFUNC section is NOT_FOUND
        (not MISSING_SECTION).
    """

def delete_expression_function_result(
    config_json: str,
    code: str,
) -> str:
    """The record (row / ids) of ``delete_expression_function``: same arguments and
    operation, but returns the record instead of the configuration. Operation: Delete
    an expression function's CFG_EFUNC row only (no cascade).

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before lookup.

    Returns:
        The record (e.g. the created row or ids) as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Returns (modified config, the deleted CFG_EFUNC row). Removes ONLY the CFG_EFUNC
        row; CFG_EFCALL rows referencing it are left dangling (use
        delete_expression_function_cascade). A missing CFG_EFUNC section is NOT_FOUND
        (not MISSING_SECTION).
    """

def delete_expression_function_cascade(
    config_json: str,
    code: str,
) -> str:
    """Delete an expression function and its CFG_EFCALL / CFG_EFBOM rows.

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before lookup.

    Returns:
        The modified configuration JSON string.
        ``delete_expression_function_cascade_result`` (same arguments) returns the
        record this operation produces.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND, MISSING_FIELD; any call
            may also raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Returns (modified config, the deleted CFG_EFUNC row). Removes the CFG_EFBOM rows
        whose EFCALL_ID belongs to one of the function's CFG_EFCALL rows, then every
        CFG_EFCALL row whose EFUNC_ID matches (each step skipped if its section is
        absent), then the CFG_EFUNC row. MISSING_FIELD when the found row has no integer
        EFUNC_ID. A missing CFG_EFUNC section is NOT_FOUND.
    """

def delete_expression_function_cascade_result(
    config_json: str,
    code: str,
) -> str:
    """The record (row / ids) of ``delete_expression_function_cascade``: same arguments
    and operation, but returns the record instead of the configuration. Operation:
    Delete an expression function and its CFG_EFCALL / CFG_EFBOM rows.

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before lookup.

    Returns:
        The record (e.g. the created row or ids) as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND, MISSING_FIELD; any call
            may also raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Returns (modified config, the deleted CFG_EFUNC row). Removes the CFG_EFBOM rows
        whose EFCALL_ID belongs to one of the function's CFG_EFCALL rows, then every
        CFG_EFCALL row whose EFUNC_ID matches (each step skipped if its section is
        absent), then the CFG_EFUNC row. MISSING_FIELD when the found row has no integer
        EFUNC_ID. A missing CFG_EFUNC section is NOT_FOUND.
    """

def get_expression_function(
    config_json: str,
    code: str,
) -> str:
    """Get one expression function's raw CFG_EFUNC row by code.

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before lookup.

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result uses on-disk keys (EFUNC_ID, EFUNC_CODE, EFUNC_DESC, CONNECT_STR,
        LANGUAGE). A missing CFG_EFUNC section is NOT_FOUND (not MISSING_SECTION).
    """

def list_expression_functions(
    config_json: str,
) -> str:
    """List all expression functions as camelCase summaries.

    Args:
        config_json: The configuration JSON (opaque).

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result is an array of {id, function, connectStr, language} in config order
        (description is NOT included); connectStr/language are null-preserving. A
        missing CFG_EFUNC section yields [] (no error).
    """

def set_expression_function(
    config_json: str,
    code: str,
    *,
    connect_str: str | None | UnsetType = ...,
    description: str | None = None,
    language: str | None = None,
) -> str:
    """Update an expression function's connect string / description / language.

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before lookup.
        connect_str: Absent leaves CONNECT_STR; null clears it to null; a string
            (including "") sets it. ``UNSET`` leaves it, ``None`` clears it.
        description: Absent leaves EFUNC_DESC; a string is stored verbatim. Cannot be
            cleared to null. ``None`` omits it.
        language: Absent leaves LANGUAGE; a string is stored verbatim. Cannot be cleared
            to null. ``None`` omits it.

    Returns:
        The modified configuration JSON string. ``set_expression_function_result`` (same
        arguments) returns the record this operation produces.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Returns (modified config, the updated CFG_EFUNC row). No value validation. The
        row is deleted and re-appended, so it moves to the END of CFG_EFUNC. A missing
        CFG_EFUNC section is NOT_FOUND.
    """

def set_expression_function_result(
    config_json: str,
    code: str,
    *,
    connect_str: str | None | UnsetType = ...,
    description: str | None = None,
    language: str | None = None,
) -> str:
    """The record (row / ids) of ``set_expression_function``: same arguments and
    operation, but returns the record instead of the configuration. Operation: Update
    an expression function's connect string / description / language.

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before lookup.
        connect_str: Absent leaves CONNECT_STR; null clears it to null; a string
            (including "") sets it. ``UNSET`` leaves it, ``None`` clears it.
        description: Absent leaves EFUNC_DESC; a string is stored verbatim. Cannot be
            cleared to null. ``None`` omits it.
        language: Absent leaves LANGUAGE; a string is stored verbatim. Cannot be cleared
            to null. ``None`` omits it.

    Returns:
        The record (e.g. the created row or ids) as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Returns (modified config, the updated CFG_EFUNC row). No value validation. The
        row is deleted and re-appended, so it moves to the END of CFG_EFUNC. A missing
        CFG_EFUNC section is NOT_FOUND.
    """

def add_standardize_function(
    config_json: str,
    code: str,
    *,
    connect_str: str | None = None,
    description: str | None = None,
    language: str | None = None,
) -> str:
    """Add a standardize function (CFG_SFUNC row).

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before the duplicate check and storage (SFUNC_CODE).
        connect_str: Absent stores CONNECT_STR null; any string (including "") is stored
            verbatim. ``None`` omits it.
        description: Absent stores SFUNC_DESC null; any string is stored verbatim.
            ``None`` omits it.
        language: Absent stores LANGUAGE null; any string is stored verbatim. ``None``
            omits it.

    Returns:
        The modified configuration JSON string. ``add_standardize_function_result``
        (same arguments) returns the record this operation produces.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, INVALID_INPUT; any
            call may also raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Returns (modified config, the new complete CFG_SFUNC row: SFUNC_ID, SFUNC_CODE,
        CONNECT_STR, SFUNC_DESC, LANGUAGE). SFUNC_ID is always auto-allocated (max
        existing + 1, floor 1); no id can be requested. TRAP: a duplicate code is
        INVALID_INPUT (SzConfigError::validation), NOT ALREADY_EXISTS. MISSING_SECTION
        only when CFG_SFUNC is absent.
    """

def add_standardize_function_result(
    config_json: str,
    code: str,
    *,
    connect_str: str | None = None,
    description: str | None = None,
    language: str | None = None,
) -> str:
    """The record (row / ids) of ``add_standardize_function``: same arguments and
    operation, but returns the record instead of the configuration. Operation: Add a
    standardize function (CFG_SFUNC row).

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before the duplicate check and storage (SFUNC_CODE).
        connect_str: Absent stores CONNECT_STR null; any string (including "") is stored
            verbatim. ``None`` omits it.
        description: Absent stores SFUNC_DESC null; any string is stored verbatim.
            ``None`` omits it.
        language: Absent stores LANGUAGE null; any string is stored verbatim. ``None``
            omits it.

    Returns:
        The record (e.g. the created row or ids) as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, INVALID_INPUT; any
            call may also raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Returns (modified config, the new complete CFG_SFUNC row: SFUNC_ID, SFUNC_CODE,
        CONNECT_STR, SFUNC_DESC, LANGUAGE). SFUNC_ID is always auto-allocated (max
        existing + 1, floor 1); no id can be requested. TRAP: a duplicate code is
        INVALID_INPUT (SzConfigError::validation), NOT ALREADY_EXISTS. MISSING_SECTION
        only when CFG_SFUNC is absent.
    """

def delete_standardize_function(
    config_json: str,
    code: str,
) -> str:
    """Delete a standardize function's CFG_SFUNC row only (no cascade).

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before lookup.

    Returns:
        The modified configuration JSON string. ``delete_standardize_function_result``
        (same arguments) returns the record this operation produces.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Returns (modified config, the deleted CFG_SFUNC row). Removes ONLY the CFG_SFUNC
        row; CFG_SFCALL rows referencing it are left dangling (use
        delete_standardize_function_cascade). A missing CFG_SFUNC section is NOT_FOUND
        (not MISSING_SECTION).
    """

def delete_standardize_function_result(
    config_json: str,
    code: str,
) -> str:
    """The record (row / ids) of ``delete_standardize_function``: same arguments and
    operation, but returns the record instead of the configuration. Operation: Delete
    a standardize function's CFG_SFUNC row only (no cascade).

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before lookup.

    Returns:
        The record (e.g. the created row or ids) as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Returns (modified config, the deleted CFG_SFUNC row). Removes ONLY the CFG_SFUNC
        row; CFG_SFCALL rows referencing it are left dangling (use
        delete_standardize_function_cascade). A missing CFG_SFUNC section is NOT_FOUND
        (not MISSING_SECTION).
    """

def delete_standardize_function_cascade(
    config_json: str,
    code: str,
) -> str:
    """Delete a standardize function and its CFG_SFCALL rows.

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before lookup.

    Returns:
        The modified configuration JSON string.
        ``delete_standardize_function_cascade_result`` (same arguments) returns the
        record this operation produces.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND, MISSING_FIELD; any call
            may also raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Returns (modified config, the deleted CFG_SFUNC row). Removes every CFG_SFCALL
        row whose SFUNC_ID matches (skipped if CFG_SFCALL is absent), then the CFG_SFUNC
        row; no other section is touched. MISSING_FIELD when the found row has no
        integer SFUNC_ID. A missing CFG_SFUNC section is NOT_FOUND.
    """

def delete_standardize_function_cascade_result(
    config_json: str,
    code: str,
) -> str:
    """The record (row / ids) of ``delete_standardize_function_cascade``: same arguments
    and operation, but returns the record instead of the configuration. Operation:
    Delete a standardize function and its CFG_SFCALL rows.

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before lookup.

    Returns:
        The record (e.g. the created row or ids) as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND, MISSING_FIELD; any call
            may also raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Returns (modified config, the deleted CFG_SFUNC row). Removes every CFG_SFCALL
        row whose SFUNC_ID matches (skipped if CFG_SFCALL is absent), then the CFG_SFUNC
        row; no other section is touched. MISSING_FIELD when the found row has no
        integer SFUNC_ID. A missing CFG_SFUNC section is NOT_FOUND.
    """

def get_standardize_function(
    config_json: str,
    code: str,
) -> str:
    """Get one standardize function's raw CFG_SFUNC row by code.

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before lookup.

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result uses on-disk keys (SFUNC_ID, SFUNC_CODE, SFUNC_DESC, CONNECT_STR,
        LANGUAGE). A missing CFG_SFUNC section is NOT_FOUND (not MISSING_SECTION).
    """

def list_standardize_functions(
    config_json: str,
) -> str:
    """List all standardize functions as camelCase summaries.

    Args:
        config_json: The configuration JSON (opaque).

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result is an array of {id, function, connectStr, language} in config order
        (description is NOT included); connectStr/language are null-preserving. A
        missing CFG_SFUNC section yields [] (no error).
    """

def set_standardize_function(
    config_json: str,
    code: str,
    *,
    connect_str: str | None | UnsetType = ...,
    description: str | None = None,
    language: str | None = None,
) -> str:
    """Update a standardize function's connect string / description / language.

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before lookup.
        connect_str: Absent leaves CONNECT_STR; null clears it to null; a string
            (including "") sets it. ``UNSET`` leaves it, ``None`` clears it.
        description: Absent leaves SFUNC_DESC; a string is stored verbatim. Cannot be
            cleared to null. ``None`` omits it.
        language: Absent leaves LANGUAGE; a string is stored verbatim. Cannot be cleared
            to null. ``None`` omits it.

    Returns:
        The modified configuration JSON string. ``set_standardize_function_result``
        (same arguments) returns the record this operation produces.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Returns (modified config, the updated CFG_SFUNC row). No value validation. The
        row is deleted and re-appended, so it moves to the END of CFG_SFUNC. A missing
        CFG_SFUNC section is NOT_FOUND.
    """

def set_standardize_function_result(
    config_json: str,
    code: str,
    *,
    connect_str: str | None | UnsetType = ...,
    description: str | None = None,
    language: str | None = None,
) -> str:
    """The record (row / ids) of ``set_standardize_function``: same arguments and
    operation, but returns the record instead of the configuration. Operation: Update
    a standardize function's connect string / description / language.

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased before lookup.
        connect_str: Absent leaves CONNECT_STR; null clears it to null; a string
            (including "") sets it. ``UNSET`` leaves it, ``None`` clears it.
        description: Absent leaves SFUNC_DESC; a string is stored verbatim. Cannot be
            cleared to null. ``None`` omits it.
        language: Absent leaves LANGUAGE; a string is stored verbatim. Cannot be cleared
            to null. ``None`` omits it.

    Returns:
        The record (e.g. the created row or ids) as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Returns (modified config, the updated CFG_SFUNC row). No value validation. The
        row is deleted and re-appended, so it moves to the END of CFG_SFUNC. A missing
        CFG_SFUNC section is NOT_FOUND.
    """

def clone_generic_plan(
    config_json: str,
    source_gplan_code: str,
    new_gplan_code: str,
    *,
    new_gplan_desc: str | None = None,
) -> str:
    """Clone a generic plan, copying every CFG_GENERIC_THRESHOLD row of the source to
    the new plan.

    Args:
        config_json: The configuration JSON (opaque).
        source_gplan_code: Uppercased, then matched exactly against GPLAN_CODE; unknown
            = NOT_FOUND.
        new_gplan_code: Uppercased before the duplicate check and storage; an existing
            code = ALREADY_EXISTS.
        new_gplan_desc: Stored verbatim in GPLAN_DESC; absent = the uppercased new code.
            ``None`` omits it.

    Returns:
        The modified configuration JSON string. ``clone_generic_plan_result`` (same
        arguments) returns the record this operation produces.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND, ALREADY_EXISTS,
            INVALID_CONFIG; any call may also raise INVALID_INPUT (bad arguments) or
            INTERNAL.

    Notes:
        Returns (modified config, new GPLAN_ID); the record is the integer id. The new
        id is always max existing GPLAN_ID + 1 (no floor, no id arg). Cloned threshold
        rows are verbatim copies with GPLAN_ID rewritten, appended after existing rows;
        an absent CFG_GENERIC_THRESHOLD section is skipped silently. INVALID_CONFIG when
        the source row's GPLAN_ID is not an integer.
    """

def clone_generic_plan_result(
    config_json: str,
    source_gplan_code: str,
    new_gplan_code: str,
    *,
    new_gplan_desc: str | None = None,
) -> str:
    """The record (row / ids) of ``clone_generic_plan``: same arguments and operation,
    but returns the record instead of the configuration. Operation: Clone a generic
    plan, copying every CFG_GENERIC_THRESHOLD row of the source to the new plan.

    Args:
        config_json: The configuration JSON (opaque).
        source_gplan_code: Uppercased, then matched exactly against GPLAN_CODE; unknown
            = NOT_FOUND.
        new_gplan_code: Uppercased before the duplicate check and storage; an existing
            code = ALREADY_EXISTS.
        new_gplan_desc: Stored verbatim in GPLAN_DESC; absent = the uppercased new code.
            ``None`` omits it.

    Returns:
        The record (e.g. the created row or ids) as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND, ALREADY_EXISTS,
            INVALID_CONFIG; any call may also raise INVALID_INPUT (bad arguments) or
            INTERNAL.

    Notes:
        Returns (modified config, new GPLAN_ID); the record is the integer id. The new
        id is always max existing GPLAN_ID + 1 (no floor, no id arg). Cloned threshold
        rows are verbatim copies with GPLAN_ID rewritten, appended after existing rows;
        an absent CFG_GENERIC_THRESHOLD section is skipped silently. INVALID_CONFIG when
        the source row's GPLAN_ID is not an integer.
    """

def delete_generic_plan(
    config_json: str,
    gplan_code: str,
) -> str:
    """Delete a generic plan and all of its generic thresholds.

    Args:
        config_json: The configuration JSON (opaque).
        gplan_code: Uppercased, then matched exactly against GPLAN_CODE; unknown =
            NOT_FOUND.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND, INVALID_INPUT,
            INVALID_CONFIG; any call may also raise INVALID_INPUT (bad arguments) or
            INTERNAL.

    Notes:
        System plans (GPLAN_ID <= 2, i.e. INGEST and SEARCH in the template) are
        protected: INVALID_INPUT. Removes the CFG_GPLAN row and every
        CFG_GENERIC_THRESHOLD row with that GPLAN_ID. An absent CFG_GPLAN section is
        NOT_FOUND (not MISSING_SECTION).
    """

def list_generic_plans(
    config_json: str,
    *,
    filter: str | None = None,
) -> str:
    """List generic plans as {id, plan, description}, optionally filtered.

    Args:
        config_json: The configuration JSON (opaque).
        filter: Case-insensitive SUBSTRING match against the raw row serialized as JSON
            text — keys and numbers included (so e.g. "gplan" matches every row). Absent
            = no filtering. ``None`` omits it.

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Array of {id (GPLAN_ID), plan (GPLAN_CODE), description (GPLAN_DESC)} sorted by
        id; missing values become 0 / "". An absent CFG_GPLAN (or G2_CONFIG) yields an
        empty array, never MISSING_SECTION.
    """

def set_generic_plan(
    config_json: str,
    gplan_code: str,
    gplan_desc: str,
) -> str:
    """Create a generic plan, or update the description of an existing one (upsert).

    Args:
        config_json: The configuration JSON (opaque).
        gplan_code: Uppercased, then matched exactly against GPLAN_CODE.
        gplan_desc: Written verbatim to GPLAN_DESC.

    Returns:
        The modified configuration JSON string. ``set_generic_plan_result`` (same
        arguments) returns the record this operation produces.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION; any call may also
            raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Returns (config, {plan_id, was_created}). Existing code: only GPLAN_DESC is
        replaced (other keys kept), was_created false. New code: a row with GPLAN_ID =
        max + 1 is appended, was_created true; an absent CFG_GPLAN section is
        MISSING_SECTION on this create path.
    """

def set_generic_plan_result(
    config_json: str,
    gplan_code: str,
    gplan_desc: str,
) -> SetGenericPlanRecord:
    """The record (row / ids) of ``set_generic_plan``: same arguments and operation, but
    returns the record instead of the configuration. Operation: Create a generic
    plan, or update the description of an existing one (upsert).

    Args:
        config_json: The configuration JSON (opaque).
        gplan_code: Uppercased, then matched exactly against GPLAN_CODE.
        gplan_desc: Written verbatim to GPLAN_DESC.

    Returns:
        ``SetGenericPlanRecord(plan_id, was_created)``; each named field is that record
        member's JSON text.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION; any call may also
            raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Returns (config, {plan_id, was_created}). Existing code: only GPLAN_DESC is
        replaced (other keys kept), was_created false. New code: a row with GPLAN_ID =
        max + 1 is appended, was_created true; an absent CFG_GPLAN section is
        MISSING_SECTION on this create path.
    """

def add_rule(
    config_json: str,
    id: int,
    rule_config: Any,
) -> str:
    """Add an entity resolution rule (CFG_ERRULE row), returning the assigned ERRULE_ID.

    Args:
        config_json: The configuration JSON (opaque).
        id: Requested ERRULE_ID. 0 or any negative value means auto-allocate (max
            existing + 1, floor 1000, so 1000 on the template). A taken id > 0 is
            ALREADY_EXISTS. Any ERRULE_ID key inside rule_config is IGNORED.
        rule_config: Object with on-disk keys. ERRULE_CODE (string) is required, else
            MISSING_FIELD; uppercased for storage and the case-insensitive duplicate
            check (ALREADY_EXISTS). QUAL_ERFRAG_CODE (the fragment) is required:
            absent/non-string is MISSING_FIELD, "" or an unknown code is NOT_FOUND
            (existence is case-insensitive). DISQ_ERFRAG_CODE is optional: "" is
            accepted and stored as "", an unknown code is NOT_FOUND. TRAP: both fragment
            codes are stored VERBATIM (not uppercased). RESOLVE / RELATE default "No",
            must be Yes/No case-insensitively (stored title-case) else INVALID_INPUT,
            and may not both be Yes (INVALID_INPUT). RESOLVE=Yes requires a non-zero
            ERRULE_TIER (INVALID_INPUT) and forces RTYPE_ID to 1; RELATE=Yes requires
            RTYPE_ID in 2,3,4 (INVALID_INPUT). RTYPE_ID defaults to 1; ERRULE_TIER
            defaults to null. A non-string / non-integer value for any of these keys is
            treated as absent. Shape: ``{ERRULE_CODE: string, QUAL_ERFRAG_CODE: string,
            DISQ_ERFRAG_CODE?: string|null, RESOLVE?: string|null, RELATE?: string|null,
            RTYPE_ID?: int|null, ERRULE_TIER?: int|null, ERRULE_ID?: int|null}``.

    Returns:
        The modified configuration JSON string. ``add_rule_result`` (same arguments)
        returns the record this operation produces.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_FIELD, ALREADY_EXISTS,
            NOT_FOUND, INVALID_INPUT, MISSING_SECTION; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Record is the assigned ERRULE_ID (integer). The written row always carries every
        CFG_ERRULE key (ERRULE_ID, ERRULE_CODE, RESOLVE, RELATE, RTYPE_ID,
        QUAL_ERFRAG_CODE, DISQ_ERFRAG_CODE, ERRULE_TIER; optional ones as null).
        ERRULE_CODE is checked BEFORE the config is parsed, so a missing code is
        MISSING_FIELD even for invalid config JSON. A config without CFG_ERRULE is
        MISSING_SECTION (after validation). Validation order: fragment, disqualifier,
        duplicate code, RESOLVE, RELATE, exclusivity, tier, RTYPE_ID.
    """

def add_rule_result(
    config_json: str,
    id: int,
    rule_config: Any,
) -> str:
    """The record (row / ids) of ``add_rule``: same arguments and operation, but returns
    the record instead of the configuration. Operation: Add an entity resolution rule
    (CFG_ERRULE row), returning the assigned ERRULE_ID.

    Args:
        config_json: The configuration JSON (opaque).
        id: Requested ERRULE_ID. 0 or any negative value means auto-allocate (max
            existing + 1, floor 1000, so 1000 on the template). A taken id > 0 is
            ALREADY_EXISTS. Any ERRULE_ID key inside rule_config is IGNORED.
        rule_config: Object with on-disk keys. ERRULE_CODE (string) is required, else
            MISSING_FIELD; uppercased for storage and the case-insensitive duplicate
            check (ALREADY_EXISTS). QUAL_ERFRAG_CODE (the fragment) is required:
            absent/non-string is MISSING_FIELD, "" or an unknown code is NOT_FOUND
            (existence is case-insensitive). DISQ_ERFRAG_CODE is optional: "" is
            accepted and stored as "", an unknown code is NOT_FOUND. TRAP: both fragment
            codes are stored VERBATIM (not uppercased). RESOLVE / RELATE default "No",
            must be Yes/No case-insensitively (stored title-case) else INVALID_INPUT,
            and may not both be Yes (INVALID_INPUT). RESOLVE=Yes requires a non-zero
            ERRULE_TIER (INVALID_INPUT) and forces RTYPE_ID to 1; RELATE=Yes requires
            RTYPE_ID in 2,3,4 (INVALID_INPUT). RTYPE_ID defaults to 1; ERRULE_TIER
            defaults to null. A non-string / non-integer value for any of these keys is
            treated as absent. Shape: ``{ERRULE_CODE: string, QUAL_ERFRAG_CODE: string,
            DISQ_ERFRAG_CODE?: string|null, RESOLVE?: string|null, RELATE?: string|null,
            RTYPE_ID?: int|null, ERRULE_TIER?: int|null, ERRULE_ID?: int|null}``.

    Returns:
        The record (e.g. the created row or ids) as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_FIELD, ALREADY_EXISTS,
            NOT_FOUND, INVALID_INPUT, MISSING_SECTION; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Record is the assigned ERRULE_ID (integer). The written row always carries every
        CFG_ERRULE key (ERRULE_ID, ERRULE_CODE, RESOLVE, RELATE, RTYPE_ID,
        QUAL_ERFRAG_CODE, DISQ_ERFRAG_CODE, ERRULE_TIER; optional ones as null).
        ERRULE_CODE is checked BEFORE the config is parsed, so a missing code is
        MISSING_FIELD even for invalid config JSON. A config without CFG_ERRULE is
        MISSING_SECTION (after validation). Validation order: fragment, disqualifier,
        duplicate code, RESOLVE, RELATE, exclusivity, tier, RTYPE_ID.
    """

def delete_rule(
    config_json: str,
    code: str,
) -> str:
    """Delete a rule by code.

    Args:
        config_json: The configuration JSON (opaque).
        code: Uppercased, then matched EXACTLY against ERRULE_CODE (a stored lowercase
            code cannot be deleted). Not an id.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        No dependency or system-rule protection; any rule can be deleted. A config
        without CFG_ERRULE is NOT_FOUND.
    """

def get_rule(
    config_json: str,
    code_or_id: str,
) -> str:
    """Get one rule, by code or ERRULE_ID, as a summary record.

    Args:
        config_json: The configuration JSON (opaque).
        code_or_id: Uppercased, then matched exactly against ERRULE_CODE first, then
            numerically against ERRULE_ID (e.g. "100").

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result is {id, rule, resolve, relate, rtype_id, fragment, disqualifier, tier}
        projected from ERRULE_ID, ERRULE_CODE, RESOLVE, RELATE, RTYPE_ID,
        QUAL_ERFRAG_CODE, DISQ_ERFRAG_CODE (null-preserving). TRAP: `tier` is the stored
        ERRULE_TIER only when RESOLVE is exactly "Yes", otherwise null.
    """

def list_rules(
    config_json: str,
) -> str:
    """List all rules as summary records, sorted by ERRULE_ID.

    Args:
        config_json: The configuration JSON (opaque).

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result is an array of the get_rule record shape, sorted by id ascending
        (null/absent id sorts as 0). A missing CFG_ERRULE or G2_CONFIG yields [].
    """

def set_rule(
    config_json: str,
    code: str,
    *,
    resolve: str | None = None,
    relate: str | None = None,
    rtype_id: int | None = None,
    fragment: str | None | UnsetType = ...,
    disqualifier: str | None | UnsetType = ...,
    tier: int | None | UnsetType = ...,
) -> str:
    """Update a rule's resolve/relate/relationship type, fragment, disqualifier or tier.

    Args:
        config_json: The configuration JSON (opaque).
        code: Rule code; uppercased, then matched exactly. Unknown is NOT_FOUND.
        resolve: Absent keeps the stored RESOLVE. Must be Yes/No case-insensitively
            (stored title-case), else INVALID_INPUT. ``None`` omits it.
        relate: Absent keeps the stored RELATE. Must be Yes/No case-insensitively, else
            INVALID_INPUT. ``None`` omits it.
        rtype_id: Absent keeps the stored RTYPE_ID. Forced to 1 when the merged RESOLVE
            is Yes; must be 2, 3 or 4 when RELATE is Yes. ``None`` omits it.
        fragment: QUAL_ERFRAG_CODE. Absent = keep (never re-validated); null = clear to
            null (TRAP: allowed here although add_rule requires a fragment); a string
            must name an existing fragment (case-insensitive; "" is NOT_FOUND) and is
            stored UPPERCASED (unlike add_rule). ``UNSET`` leaves it, ``None`` clears
            it.
        disqualifier: DISQ_ERFRAG_CODE. Absent = keep; null = clear to null; "" is
            accepted and stored ""; another string must name an existing fragment
            (NOT_FOUND) and is stored uppercased. ``UNSET`` leaves it, ``None`` clears
            it.
        tier: ERRULE_TIER. Absent = keep; null = clear; value = set. The merged rule
            with RESOLVE=Yes must have a non-zero tier, else INVALID_INPUT. ``UNSET``
            leaves it, ``None`` clears it.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND, INVALID_INPUT; any call
            may also raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Merges the update into the stored row and re-applies add_rule's
        RESOLVE/RELATE/exclusivity/tier/RTYPE_ID rules to the merged row (no
        duplicate-code check); the row is rewritten with every CFG_ERRULE key and
        ERRULE_ID preserved.
    """

def add_search_profile(
    config_json: str,
    code: str,
    generic_plan: str,
    *,
    candidates: str | None = None,
    description: str | None = None,
    elements: Any | None = None,
) -> str:
    """Add a search profile (CFG_SPROFILE row) tying a generic plan and feature
    candidate overrides to a code.

    Args:
        config_json: The configuration JSON (opaque).
        code: SPROFILE_CODE; trimmed and uppercased. Empty after trimming is
            INVALID_INPUT; an existing code (case-insensitive) is ALREADY_EXISTS.
        generic_plan: GPLAN_CODE, matched case-insensitively against CFG_GPLAN
            (NOT_FOUND); stored as GPLAN_ID.
        candidates: DEFAULT_USED_FOR_CAND; trimmed, case-insensitive, normalized to
            Normal or Off. Absent or blank = Normal. Anything else is VALIDATION_ERRORS
            (field "candidates", OUT_OF_DOMAIN). ``None`` omits it. Library default when
            omitted: ``"Normal"``.
        description: SPROFILE_DESC, stored verbatim; absent = "". ``None`` omits it.
            Library default when omitted: ``""``.
        elements: Feature candidate overrides: an array of {"feature": FTYPE_CODE,
            "flag": Yes|No|Y|N} objects (only those two keys, both strings, else
            INVALID_INPUT; a missing key = MISSING_FIELD); absent = none. Each feature
            is resolved case-insensitively against CFG_FTYPE (NOT_FOUND); a feature
            listed twice is VALIDATION_ERRORS (field "overrides", DUPLICATE); a flag
            other than Yes/Y/No/N (trimmed, case-insensitive) is VALIDATION_ERRORS
            (field "overrides", OUT_OF_DOMAIN). Stored in FTYPE_OVERRIDES as
            "[{<ftypeId>,<Y|N>},...]" sorted by FTYPE_ID, or "[]". Shape: ``[{feature:
            string, flag: "Yes"|"No"|"Y"|"N"}]``. ``None`` omits it.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, INVALID_INPUT, VALIDATION_ERRORS,
            NOT_FOUND, ALREADY_EXISTS, INVALID_STRUCTURE, MISSING_FIELD; any call may
            also raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        SPROFILE_ID is always auto-allocated (max + 1, floor 1; 3 on the template, whose
        only profile is SEARCH = 2); no explicit id can be requested. CFG_SPROFILE is
        created when absent. Validation order: code, candidates, generic plan, overrides
        (per element: feature, duplicate, flag), duplicate code. A config without
        G2_CONFIG fails the generic-plan lookup (NOT_FOUND), so the library's
        MISSING_SECTION branch is unreachable; a non-array CFG_SPROFILE is
        INVALID_STRUCTURE.
    """

def get_search_profile(
    config_json: str,
    code: str,
) -> str:
    """Get one search profile's raw CFG_SPROFILE row by code.

    Args:
        config_json: The configuration JSON (opaque).
        code: Case-insensitive match against SPROFILE_CODE. Not an id.

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result is the stored row (SPROFILE_ID, SPROFILE_CODE, SPROFILE_DESC, GPLAN_ID,
        DEFAULT_USED_FOR_CAND, FTYPE_OVERRIDES). A missing section is NOT_FOUND.
    """

def list_search_profiles(
    config_json: str,
    *,
    filter: str | None = None,
) -> str:
    """List search profiles as display records with ids resolved to codes, sorted by id.

    Args:
        config_json: The configuration JSON (opaque).
        filter: Absent returns all. Otherwise keeps rows whose COMPACT raw-row JSON
            (serde_json::to_string, no spaces, on-disk keys and ids, e.g. "GPLAN_ID":2)
            contains the filter case-insensitively; it is applied to the raw row, not
            the projected record. ``None`` omits it.

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result is an array of {id, profile, description, genericPlan, candidates,
        overrides: [{feature, flag: Yes|No}], overridesRaw} sorted by id. genericPlan /
        feature fall back to the numeric id as a string when unresolvable. A missing
        CFG_SPROFILE yields [].
    """

def delete_search_profile(
    config_json: str,
    search_value: str,
) -> str:
    """Delete a search profile by code or SPROFILE_ID.

    Args:
        config_json: The configuration JSON (opaque).
        search_value: Matched case-insensitively against SPROFILE_CODE, or (when it
            parses as an integer after trimming) against SPROFILE_ID.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND, INVALID_INPUT; any call
            may also raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        The shipped profiles INGEST and SEARCH are protected: deleting one that exists
        is INVALID_INPUT. Existence is checked FIRST, so an absent reserved code (INGEST
        is not in the template) is NOT_FOUND.
    """

def set_setting(
    config_json: str,
    name: str,
    value: Any,
) -> str:
    """Create or overwrite a named setting in the G2_CONFIG.SETTINGS object.

    Args:
        config_json: The configuration JSON (opaque).
        name: Uppercased; an existing setting of that name is overwritten silently.
        value: Stored VERBATIM as its typed JSON value (an integer stays an integer, a
            string a string); no validation.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION; any call may also
            raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        SETTINGS is created when absent, and a non-object SETTINGS value (null, string,
        array) is REPLACED by a fresh object (prior content lost). A missing or
        non-object G2_CONFIG is MISSING_SECTION.
    """

def list_system_parameters(
    config_json: str,
) -> str:
    """List system parameters as a name -> string-value map.

    Args:
        config_json: The configuration JSON (opaque).

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result is an object. The only parameter is relationshipsBreakMatches, read from
        BREAK_RES of the FIRST CFG_RTYPE row with RCLASS_ID 2 and reported as a decimal
        STRING. TRAP: it is reported only when BREAK_RES is a JSON integer; the
        template's DISCLOSED row stores the string "No", so the template yields {}. A
        missing CFG_RTYPE/G2_CONFIG also yields {}.
    """

def set_system_parameter(
    config_json: str,
    parameter_name: str,
    parameter_value: Any,
) -> str:
    """Set a system parameter (relationshipsBreakMatches).

    Args:
        config_json: The configuration JSON (opaque).
        parameter_name: Case-insensitive; only relationshipsBreakMatches (or
            relationships_break_matches) is known. Any other name is INVALID_CONFIG (not
            INVALID_INPUT).
        parameter_value: Written VERBATIM (any JSON value, no validation) to BREAK_RES
            of the first CFG_RTYPE row with RCLASS_ID 2. Only an integer value is
            visible to list_system_parameters afterwards.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, INVALID_CONFIG, NOT_FOUND; any call
            may also raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        NOT_FOUND when no CFG_RTYPE row has RCLASS_ID 2 (or CFG_RTYPE/G2_CONFIG is
        absent). The parameter name is checked AFTER the config is parsed.
    """

def add_comparison_threshold(
    config_json: str,
    cfunc_code: str,
    ftype_code: str,
    cfunc_rtnval: str,
    *,
    exec_order: int | None = None,
    same_score: int | None = None,
    close_score: int | None = None,
    likely_score: int | None = None,
    plausible_score: int | None = None,
    un_likely_score: int | None = None,
) -> str:
    """Add a comparison threshold (CFG_CFRTN row) for a comparison function, feature and
    return value.

    Args:
        config_json: The configuration JSON (opaque).
        cfunc_code: REQUIRED by the library (absent = MISSING_FIELD). Comparison
            function code (CFG_CFUNC), matched case-insensitively; unknown = NOT_FOUND.
        ftype_code: REQUIRED by the library (absent = MISSING_FIELD). Feature code
            matched case-insensitively (unknown = NOT_FOUND), or "all" (any case) for
            the all-features FTYPE_ID 0 sentinel.
        cfunc_rtnval: REQUIRED by the library (absent = MISSING_FIELD). Return value /
            score name; uppercased before storage and duplicate check.
        exec_order: Resolved in three steps: (1) if an all-features (FTYPE_ID 0) row
            already exists for this (cfunc, rtnval), its EXEC_ORDER is REUSED and this
            arg is ignored; (2) else a value > 0 is honoured, or ALREADY_EXISTS if taken
            within (CFUNC_ID, FTYPE_ID 0); (3) else (absent or <= 0) the next order
            within (CFUNC_ID, FTYPE_ID 0) is auto-allocated (max + 1). Never null.
            ``None`` omits it.
        same_score: Stored verbatim (no range check); absent stores SAME_SCORE null.
            ``None`` omits it.
        close_score: Stored verbatim; absent stores CLOSE_SCORE null. ``None`` omits it.
        likely_score: Stored verbatim; absent stores LIKELY_SCORE null. ``None`` omits
            it.
        plausible_score: Stored verbatim; absent stores PLAUSIBLE_SCORE null. ``None``
            omits it.
        un_likely_score: Stored verbatim; absent stores UN_LIKELY_SCORE null. ``None``
            omits it.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_FIELD, NOT_FOUND,
            MISSING_SECTION, ALREADY_EXISTS; any call may also raise INVALID_INPUT (bad
            arguments) or INTERNAL.

    Notes:
        CFRTN_ID is always auto-allocated (max existing + 1, no floor); there is no id
        arg. Duplicate key is (CFUNC_ID, FTYPE_ID, uppercased rtnval) = ALREADY_EXISTS.
        Order: missing fields, cfunc lookup, feature lookup, then CFG_CFRTN section,
        duplicate, exec order.
    """

def delete_comparison_threshold(
    config_json: str,
    cfunc_code: str,
    ftype_code: str,
    cfunc_rtnval: str,
) -> str:
    """Delete a comparison threshold identified by (comparison function, feature, return
    value).

    Args:
        config_json: The configuration JSON (opaque).
        cfunc_code: Comparison function code, case-insensitive; unknown = NOT_FOUND.
        ftype_code: Feature code (case-insensitive) or "all" for FTYPE_ID 0; unknown =
            NOT_FOUND.
        cfunc_rtnval: Return value, matched case-insensitively.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND, MISSING_SECTION; any call
            may also raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        No matching row = NOT_FOUND. No tier/dependency protection: deleting the
        all-features tier row leaves per-feature rows in place.
    """

def set_comparison_threshold(
    config_json: str,
    cfunc_code: str,
    ftype_code: str,
    cfunc_rtnval: str,
    *,
    exec_order: int | None = None,
    same_score: int | None = None,
    close_score: int | None = None,
    likely_score: int | None = None,
    plausible_score: int | None = None,
    un_likely_score: int | None = None,
) -> str:
    """Update the exec order and/or scores of an existing comparison threshold.

    Args:
        config_json: The configuration JSON (opaque).
        cfunc_code: REQUIRED by the library (absent = MISSING_FIELD). Case-insensitive
            lookup; unknown = NOT_FOUND.
        ftype_code: REQUIRED (absent = MISSING_FIELD). Feature code (case-insensitive)
            or "all" for FTYPE_ID 0.
        cfunc_rtnval: REQUIRED (absent = MISSING_FIELD). Matched case-insensitively; no
            row = NOT_FOUND.
        exec_order: Absent leaves EXEC_ORDER unchanged; a value is written VERBATIM (no
            uniqueness / tier check, any integer incl. <= 0). ``None`` omits it.
        same_score: Absent leaves unchanged; else written verbatim. ``None`` omits it.
        close_score: Absent leaves unchanged; else written verbatim. ``None`` omits it.
        likely_score: Absent leaves unchanged; else written verbatim. ``None`` omits it.
        plausible_score: Absent leaves unchanged; else written verbatim. ``None`` omits
            it.
        un_likely_score: Absent leaves unchanged; else written verbatim. ``None`` omits
            it.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_FIELD, NOT_FOUND,
            MISSING_SECTION; any call may also raise INVALID_INPUT (bad arguments) or
            INTERNAL.

    Notes:
        NOT tri-state: a score cannot be cleared back to null.
    """

def list_comparison_thresholds(
    config_json: str,
) -> str:
    """List all comparison thresholds with resolved function and feature names.

    Args:
        config_json: The configuration JSON (opaque).

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION; any call may also
            raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Array of {id (CFRTN_ID), function (CFUNC_CODE or "unknown"), returnOrder
        (EXEC_ORDER), scoreName, feature ("all" for FTYPE_ID 0, else FTYPE_CODE or
        "unknown"), sameScore, closeScore, likelyScore, plausibleScore, unlikelyScore},
        sorted by (CFUNC_ID, CFRTN_ID). TRAP: null/absent EXEC_ORDER and scores are
        reported as 0, not null. Requires CFG_CFRTN, CFG_CFUNC and CFG_FTYPE (else
        MISSING_SECTION).
    """

def add_generic_threshold(
    config_json: str,
    plan: str,
    behavior: str,
    scoring_cap: int,
    candidate_cap: int,
    send_to_redo: str,
    *,
    feature: str | None = None,
) -> str:
    """Add a generic threshold (CFG_GENERIC_THRESHOLD row) for a plan, behavior and
    optional feature.

    Args:
        config_json: The configuration JSON (opaque).
        plan: REQUIRED (absent = MISSING_FIELD). Uppercased, then matched EXACTLY
            against GPLAN_CODE (so effectively case-insensitive for upper-case stored
            codes); unknown = NOT_FOUND.
        behavior: REQUIRED (absent = MISSING_FIELD). Uppercased; must be a canonical
            behavior code (behavior_domain, e.g. NAME, F1, FM, A1) else
            VALIDATION_ERRORS (field "behavior", UnknownReferenceCode).
        scoring_cap: REQUIRED (absent = MISSING_FIELD). Stored verbatim (e.g. -1).
        candidate_cap: REQUIRED (absent = MISSING_FIELD). Stored verbatim.
        send_to_redo: REQUIRED (absent = MISSING_FIELD). Case-insensitive Yes/No, stored
            canonical "Yes"/"No"; else VALIDATION_ERRORS (field "sendToRedo",
            OutOfDomain).
        feature: Absent or "all" (any case) = FTYPE_ID 0. Else uppercased and matched
            EXACTLY against FTYPE_CODE; unknown = NOT_FOUND. ``None`` omits it. Library
            default when omitted: ``"ALL"``.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_FIELD, MISSING_SECTION,
            NOT_FOUND, ALREADY_EXISTS, VALIDATION_ERRORS; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        All absent required fields are reported in ONE MISSING_FIELD (order plan,
        behavior, scoring_cap, candidate_cap, send_to_redo), checked before the config
        is parsed. Then: plan lookup, feature lookup, duplicate (plan, behavior,
        feature) = ALREADY_EXISTS (Python treats it as a warning-success; the root
        library does not), then behavior + sendToRedo are validated TOGETHER into one
        VALIDATION_ERRORS (details schema sz-configtool.validation-errors/v1, order
        [behavior, sendToRedo]).
    """

def delete_generic_threshold(
    config_json: str,
    plan: str,
    behavior: str,
    *,
    feature: str | None = None,
) -> str:
    """Delete a generic threshold identified by (plan, behavior, feature).

    Args:
        config_json: The configuration JSON (opaque).
        plan: REQUIRED (absent = MISSING_FIELD). Case-insensitive GPLAN_CODE lookup;
            unknown = NOT_FOUND.
        behavior: REQUIRED (absent = MISSING_FIELD). Uppercased and matched exactly; NOT
            validated against the behavior domain (no row = NOT_FOUND).
        feature: Absent or "all" = FTYPE_ID 0; else uppercased exact FTYPE_CODE match,
            unknown = NOT_FOUND. ``None`` omits it. Library default when omitted:
            ``"ALL"``.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_FIELD, NOT_FOUND,
            MISSING_SECTION; any call may also raise INVALID_INPUT (bad arguments) or
            INTERNAL.

    Notes:
        No matching row = NOT_FOUND (also when CFG_GENERIC_THRESHOLD is absent). (Before
        the Unreleased fix it ignored `plan` and always deleted from INGEST.)
    """

def set_generic_threshold(
    config_json: str,
    plan: str,
    behavior: str,
    *,
    feature: str | None = None,
    candidate_cap: int | None = None,
    scoring_cap: int | None = None,
    send_to_redo: str | None = None,
) -> str:
    """Update the caps and/or send-to-redo flag of an existing generic threshold.

    Args:
        config_json: The configuration JSON (opaque).
        plan: REQUIRED (absent = MISSING_FIELD). Case-insensitive GPLAN_CODE lookup;
            unknown = NOT_FOUND.
        behavior: REQUIRED (absent = MISSING_FIELD). Lookup KEY only: uppercased and
            matched exactly, never validated against the domain (no row = NOT_FOUND).
        feature: Lookup KEY selecting the per-feature row (never written). Absent or
            "all" = FTYPE_ID 0; else case-insensitive feature lookup, unknown =
            NOT_FOUND. ``None`` omits it. Library default when omitted: ``"ALL"``.
        candidate_cap: Absent leaves CANDIDATE_CAP unchanged; else written verbatim.
            ``None`` omits it.
        scoring_cap: Absent leaves SCORING_CAP unchanged; else written verbatim.
            ``None`` omits it.
        send_to_redo: Absent leaves unchanged; else case-insensitive Yes/No stored
            canonical, otherwise VALIDATION_ERRORS (field "sendToRedo", OutOfDomain) —
            checked AFTER the row lookup, so a missing row wins. ``None`` omits it.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_FIELD, NOT_FOUND,
            MISSING_SECTION, VALIDATION_ERRORS; any call may also raise INVALID_INPUT
            (bad arguments) or INTERNAL.
    """

def list_generic_thresholds(
    config_json: str,
) -> str:
    """List all generic thresholds with resolved plan and feature names.

    Args:
        config_json: The configuration JSON (opaque).

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION; any call may also
            raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Array of {id (GPLAN_ID), plan, behavior, feature ("all" for FTYPE_ID 0),
        candidateCap, scoringCap, sendToRedo}, sorted by (GPLAN_ID, canonical behavior
        position; unknown behaviors last), stable within ties. Requires
        CFG_GENERIC_THRESHOLD, CFG_GPLAN and CFG_FTYPE (else MISSING_SECTION).
    """

def validate_generic_threshold(
    config_json: str,
    plan: str,
    behavior: str,
    send_to_redo: str,
    *,
    feature: str | None = None,
) -> str:
    """Stage the checks of a generic-threshold add without mutating the config,
    returning the outcome as data.

    Args:
        config_json: The configuration JSON (opaque).
        plan: Uppercased, matched exactly against GPLAN_CODE; unknown = result
            notFound/plan (data, not an error).
        behavior: Uppercased; checked against the canonical behavior codes only in the
            last stage.
        send_to_redo: Case-insensitive Yes/No; checked only in the last stage.
        feature: Absent or "all" (any case) = FTYPE_ID 0. Else uppercased exact
            FTYPE_CODE match; unknown = result notFound/feature. ``None`` omits it.
            Library default when omitted: ``"ALL"``.

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result is the versioned object (schema sz-configtool.generic-threshold-check/v1,
        root-library Serialize for GenericThresholdCheck): {"schema", "result": "ok" |
        "duplicate" | "notFound" (+ "which": "plan"|"feature", "value": uppercased code)
        | "invalid" (+ "failures": [{"field", "reasonCode", "offendingValue"}], order
        [behavior, sendToRedo])}. Stages stop at the first hit: plan, feature, duplicate
        (plan, behavior, feature), then field validation. Missing sections are NOT
        errors (a missing CFG_GPLAN is notFound/plan). Caps are not taken.
    """

def validate_config(
    config_json: str,
) -> None:
    """Check that a document has the top-level shape of a config (structure only).

    Args:
        config_json: The configuration JSON (opaque).

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, MISSING_SECTION, INVALID_STRUCTURE;
            any call may also raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Checks, in order: valid JSON (JSON_PARSE); a G2_CONFIG key (MISSING_SECTION)
        that is an object (INVALID_STRUCTURE); every recognised CFG_* section
        (validation::EXPECTED_SECTIONS, 27 names) that is PRESENT is an array
        (INVALID_STRUCTURE). Absent sections, other keys and cross-references are not
        checked.
    """

def get_version(
    config_json: str,
) -> str:
    """Get the configuration VERSION string (G2_CONFIG.CONFIG_BASE_VERSION.VERSION).

    Args:
        config_json: The configuration JSON (opaque).

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result is a JSON string (e.g. "4.4.0" in the template). Absent or non-string
        VERSION (or any missing parent) = NOT_FOUND.
    """

def get_compatibility_version(
    config_json: str,
) -> str:
    """Get COMPATIBILITY_VERSION.CONFIG_VERSION.

    Args:
        config_json: The configuration JSON (opaque).

    Returns:
        The result as a JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Result is a JSON string (e.g. "11" in the template). Absent or non-string
        CONFIG_VERSION (or any missing parent) = NOT_FOUND.
    """

def update_compatibility_version(
    config_json: str,
    new_version: str,
) -> str:
    """Set COMPATIBILITY_VERSION.CONFIG_VERSION.

    Args:
        config_json: The configuration JSON (opaque).
        new_version: Stored verbatim as a JSON string; no format validation.

    Returns:
        The modified configuration JSON string.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND, INVALID_CONFIG; any call
            may also raise INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Absent CONFIG_BASE_VERSION or COMPATIBILITY_VERSION = NOT_FOUND;
        COMPATIBILITY_VERSION not an object = INVALID_CONFIG. TRAP: an absent G2_CONFIG
        is NOT an error — the config is returned unchanged (re-serialized).
    """

def verify_compatibility_version(
    config_json: str,
    expected_version: str,
) -> VerifyCompatibilityVersionRecord:
    """Compare COMPATIBILITY_VERSION.CONFIG_VERSION with an expected value.

    Args:
        config_json: The configuration JSON (opaque).
        expected_version: Compared by exact, case-sensitive string equality.

    Returns:
        ``VerifyCompatibilityVersionRecord(current_version, matches)``; each named field
        is that record member's JSON text.

    Raises:
        SzConfigToolError: reason codes JSON_PARSE, NOT_FOUND; any call may also raise
            INVALID_INPUT (bad arguments) or INTERNAL.

    Notes:
        Rust returns (current_version, matches) where the String is NOT a config, so it
        is `json`; tuple_names makes the result the object {"current_version": "11",
        "matches": true}. Absent CONFIG_VERSION = NOT_FOUND.
    """
