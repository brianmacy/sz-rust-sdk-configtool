// GENERATED — do not edit. Source: api/manifest/*.yaml; regenerate with `cargo run -p sz-configtool-codegen`.

package io.github.brianmacy.szconfigtool;

/**
 * Typed, stateless operations on Senzing configuration JSON documents
 * (generated from {@code api/manifest}). Every method takes the configuration
 * as an opaque string. A config-changing method returns the modified
 * configuration; when the operation also produces a record (e.g. the new row),
 * the companion {@code <name>Result} (same arguments) returns that record
 * instead. Other methods return JSON text; failures throw
 * {@link SzConfigToolException}.
 */
public final class SzConfigTool {
    private SzConfigTool() {
    }

    /** Optional arguments of {@link #addAttribute}; unset = omitted (library default). */
    public static final class AddAttributeOptions {
        final Args wire = new Args();

        /**
         * <code>default_value</code> (str) Absent stores DEFAULT_VALUE null; any string (including
         * "") is stored verbatim.
         *
         * @param defaultValue the value
         * @return this builder
         */
        public AddAttributeOptions defaultValue(String defaultValue) {
            wire.str("default_value", defaultValue);
            return this;
        }

        /**
         * <code>internal</code> (str) Case-insensitive; normalized to Yes or No, else
         * INVALID_INPUT. Library default when omitted: "No".
         *
         * @param internal the value
         * @return this builder
         */
        public AddAttributeOptions internal(String internal) {
            wire.str("internal", internal);
            return this;
        }

        /**
         * <code>required</code> (str) Case-insensitive; normalized to Yes, No, Any or Desired
         * (stored in FELEM_REQ), else INVALID_INPUT. Library default when omitted: "No".
         *
         * @param required the value
         * @return this builder
         */
        public AddAttributeOptions required(String required) {
            wire.str("required", required);
            return this;
        }

        /**
         * <code>id</code> (int) Requested ATTR_ID. Absent OR &lt;= 0 means auto-allocate (max
         * existing + 1, floor 1000). A taken id &gt; 0 is ALREADY_EXISTS.
         *
         * @param id the value
         * @return this builder
         */
        public AddAttributeOptions id(long id) {
            wire.integer("id", id);
            return this;
        }
    }

    /**
     * Add an attribute (CFG_ATTR row) mapping an input attribute to a feature element.
     *
     * <p>Notes:
     * Returns (modified config, the new CFG_ATTR row). Validation order: class, duplicate
     * attribute, feature, element, required, internal, id. Does not create a CFG_FBOM row.
     *
     * <p>Wire name: {@code add_attribute}; group: {@code attributes}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param attribute <code>attribute</code> (str) Uppercased before storage and duplicate check.
     * @param feature <code>feature</code> (str) Must name an existing CFG_FTYPE (case-insensitive) or
     * NOT_FOUND; stored uppercased in FTYPE_CODE.
     * @param element <code>element</code> (str) Must name an existing CFG_FELEM (case-insensitive) or
     * NOT_FOUND; stored uppercased in FELEM_CODE.
     * @param classValue <code>class</code> (str) CASE-SENSITIVE (not uppercased): must be exactly one of NAME,
     * ATTRIBUTE, IDENTIFIER, ADDRESS, PHONE, RELATIONSHIP, OTHER, else INVALID_INPUT.
     * @return the modified configuration JSON document (opaque); {@link #addAttributeResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, ALREADY_EXISTS, INVALID_INPUT, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addAttribute(String configJson, String attribute, String feature, String element, String classValue) throws SzConfigToolException {
        return addAttribute(configJson, attribute, feature, element, classValue, null);
    }

    /**
     * Add an attribute (CFG_ATTR row) mapping an input attribute to a feature element.
     *
     * <p>Notes:
     * Returns (modified config, the new CFG_ATTR row). Validation order: class, duplicate
     * attribute, feature, element, required, internal, id. Does not create a CFG_FBOM row.
     *
     * <p>Wire name: {@code add_attribute}; group: {@code attributes}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param attribute <code>attribute</code> (str) Uppercased before storage and duplicate check.
     * @param feature <code>feature</code> (str) Must name an existing CFG_FTYPE (case-insensitive) or
     * NOT_FOUND; stored uppercased in FTYPE_CODE.
     * @param element <code>element</code> (str) Must name an existing CFG_FELEM (case-insensitive) or
     * NOT_FOUND; stored uppercased in FELEM_CODE.
     * @param classValue <code>class</code> (str) CASE-SENSITIVE (not uppercased): must be exactly one of NAME,
     * ATTRIBUTE, IDENTIFIER, ADDRESS, PHONE, RELATIONSHIP, OTHER, else INVALID_INPUT.
     * @param options optional arguments ({@code null} = none); see {@link AddAttributeOptions}
     * @return the modified configuration JSON document (opaque); {@link #addAttributeResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, ALREADY_EXISTS, INVALID_INPUT, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addAttribute(String configJson, String attribute, String feature, String element, String classValue, AddAttributeOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("attribute", attribute);
        wire.str("feature", feature);
        wire.str("element", element);
        wire.str("class", classValue);
        return Invoker.call("add_attribute", "config_and_json", configJson, wire)[1];
    }

    /**
     * The record (row / ids) of <code>addAttribute</code>: same arguments and operation, but
     * returns the record instead of the configuration. Operation: Add an attribute (CFG_ATTR
     * row) mapping an input attribute to a feature element.
     *
     * <p>Notes:
     * Returns (modified config, the new CFG_ATTR row). Validation order: class, duplicate
     * attribute, feature, element, required, internal, id. Does not create a CFG_FBOM row.
     *
     * <p>Wire name: {@code add_attribute}; group: {@code attributes}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param attribute <code>attribute</code> (str) Uppercased before storage and duplicate check.
     * @param feature <code>feature</code> (str) Must name an existing CFG_FTYPE (case-insensitive) or
     * NOT_FOUND; stored uppercased in FTYPE_CODE.
     * @param element <code>element</code> (str) Must name an existing CFG_FELEM (case-insensitive) or
     * NOT_FOUND; stored uppercased in FELEM_CODE.
     * @param classValue <code>class</code> (str) CASE-SENSITIVE (not uppercased): must be exactly one of NAME,
     * ATTRIBUTE, IDENTIFIER, ADDRESS, PHONE, RELATIONSHIP, OTHER, else INVALID_INPUT.
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, ALREADY_EXISTS, INVALID_INPUT, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addAttributeResult(String configJson, String attribute, String feature, String element, String classValue) throws SzConfigToolException {
        return addAttributeResult(configJson, attribute, feature, element, classValue, null);
    }

    /**
     * The record (row / ids) of <code>addAttribute</code>: same arguments and operation, but
     * returns the record instead of the configuration. Operation: Add an attribute (CFG_ATTR
     * row) mapping an input attribute to a feature element.
     *
     * <p>Notes:
     * Returns (modified config, the new CFG_ATTR row). Validation order: class, duplicate
     * attribute, feature, element, required, internal, id. Does not create a CFG_FBOM row.
     *
     * <p>Wire name: {@code add_attribute}; group: {@code attributes}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param attribute <code>attribute</code> (str) Uppercased before storage and duplicate check.
     * @param feature <code>feature</code> (str) Must name an existing CFG_FTYPE (case-insensitive) or
     * NOT_FOUND; stored uppercased in FTYPE_CODE.
     * @param element <code>element</code> (str) Must name an existing CFG_FELEM (case-insensitive) or
     * NOT_FOUND; stored uppercased in FELEM_CODE.
     * @param classValue <code>class</code> (str) CASE-SENSITIVE (not uppercased): must be exactly one of NAME,
     * ATTRIBUTE, IDENTIFIER, ADDRESS, PHONE, RELATIONSHIP, OTHER, else INVALID_INPUT.
     * @param options optional arguments ({@code null} = none); see {@link AddAttributeOptions}
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, ALREADY_EXISTS, INVALID_INPUT, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addAttributeResult(String configJson, String attribute, String feature, String element, String classValue, AddAttributeOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("attribute", attribute);
        wire.str("feature", feature);
        wire.str("element", element);
        wire.str("class", classValue);
        return Invoker.result("add_attribute", "config_and_json", configJson, wire);
    }

    /**
     * Delete an attribute by code.
     *
     * <p>Notes:
     * No dependency or system-attribute protection; any attribute can be deleted.
     *
     * <p>Wire name: {@code delete_attribute}; group: {@code attributes}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteAttribute(String configJson, String code) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("code", code);
        return Invoker.config("delete_attribute", configJson, wire);
    }

    /**
     * Get one attribute's raw CFG_ATTR row by code.
     *
     * <p>Notes:
     * Result uses on-disk keys (ATTR_ID, ATTR_CODE, ATTR_CLASS, FTYPE_CODE, FELEM_CODE,
     * FELEM_REQ, DEFAULT_VALUE, INTERNAL).
     *
     * <p>Wire name: {@code get_attribute}; group: {@code attributes}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String getAttribute(String configJson, String code) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("code", code);
        return Invoker.json("get_attribute", configJson, wire);
    }

    /**
     * List all attributes as camelCase summaries.
     *
     * <p>Notes:
     * Result is an array of {id, attribute, class, feature, element, required, default,
     * internal} in config order; feature/element/default may be null.
     *
     * <p>Wire name: {@code list_attributes}; group: {@code attributes}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String listAttributes(String configJson) throws SzConfigToolException {
        Args wire = new Args();
        return Invoker.json("list_attributes", configJson, wire);
    }

    /** Optional arguments of {@link #setAttribute}; unset = omitted (library default). */
    public static final class SetAttributeOptions {
        final Args wire = new Args();

        /**
         * <code>internal</code> (str) Absent leaves INTERNAL unchanged; else case-insensitive,
         * normalized to Yes or No, else INVALID_INPUT.
         *
         * @param internal the value
         * @return this builder
         */
        public SetAttributeOptions internal(String internal) {
            wire.str("internal", internal);
            return this;
        }

        /**
         * <code>required</code> (str) Absent leaves FELEM_REQ unchanged; else normalized to Yes,
         * No, Any or Desired, else INVALID_INPUT.
         *
         * @param required the value
         * @return this builder
         */
        public SetAttributeOptions required(String required) {
            wire.str("required", required);
            return this;
        }

        /**
         * <code>default_value</code> (str) Absent leaves DEFAULT_VALUE unchanged; a string is
         * stored verbatim. NOT tri-state: there is no way to clear DEFAULT_VALUE back to null.
         *
         * @param defaultValue the value
         * @return this builder
         */
        public SetAttributeOptions defaultValue(String defaultValue) {
            wire.str("default_value", defaultValue);
            return this;
        }
    }

    /**
     * Update an attribute's internal / required / default value.
     *
     * <p>Notes:
     * Validation is interleaved with mutation but the input config is never modified on error
     * (a new config string is returned only on success).
     *
     * <p>Wire name: {@code set_attribute}; group: {@code attributes}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param attribute <code>attribute</code> (str) Attribute code; uppercased before lookup.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String setAttribute(String configJson, String attribute) throws SzConfigToolException {
        return setAttribute(configJson, attribute, null);
    }

    /**
     * Update an attribute's internal / required / default value.
     *
     * <p>Notes:
     * Validation is interleaved with mutation but the input config is never modified on error
     * (a new config string is returned only on success).
     *
     * <p>Wire name: {@code set_attribute}; group: {@code attributes}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param attribute <code>attribute</code> (str) Attribute code; uppercased before lookup.
     * @param options optional arguments ({@code null} = none); see {@link SetAttributeOptions}
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String setAttribute(String configJson, String attribute, SetAttributeOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("attribute", attribute);
        return Invoker.config("set_attribute", configJson, wire);
    }

    /**
     * Add a behavior override (CFG_FBOVR row) for a feature and usage type.
     *
     * <p>Notes:
     * Validation order: feature, behavior, CFG_FBOVR present (MISSING_SECTION), duplicate
     * (same FTYPE_ID + uppercased UTYPE_CODE, ALREADY_EXISTS). The row always carries
     * FTYPE_ID, UTYPE_CODE, FTYPE_FREQ, FTYPE_EXCL, FTYPE_STAB.
     *
     * <p>Wire name: {@code add_behavior_override}; group: {@code behavior_overrides}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param feature <code>feature</code> (str) Feature code, matched case-insensitively against CFG_FTYPE
     * (NOT_FOUND); stored as its FTYPE_ID.
     * @param usageType <code>usage_type</code> (str) Uppercased; stored as UTYPE_CODE. Any string is accepted
     * (no domain check).
     * @param behavior <code>behavior</code> (str) Behavior code, case-insensitive: a frequency A1, F1, FF, FM,
     * FVM (with optional E = exclusive and/or S = stable letters in any order) or the bare
     * NAME / NONE; anything else is INVALID_INPUT. Split into FTYPE_FREQ, FTYPE_EXCL (Yes/No),
     * FTYPE_STAB (Yes/No).
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT, MISSING_SECTION, ALREADY_EXISTS; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addBehaviorOverride(String configJson, String feature, String usageType, String behavior) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("feature", feature);
        wire.str("usage_type", usageType);
        wire.str("behavior", behavior);
        return Invoker.config("add_behavior_override", configJson, wire);
    }

    /**
     * Delete the behavior override for a feature and usage type.
     *
     * <p>Notes:
     * No override for the pair is NOT_FOUND; a config without CFG_FBOVR is MISSING_SECTION.
     *
     * <p>Wire name: {@code delete_behavior_override}; group: {@code behavior_overrides}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param feature <code>feature</code> (str) Feature code, case-insensitive; unknown is NOT_FOUND.
     * @param usageType <code>usage_type</code> (str) Uppercased, then matched exactly against UTYPE_CODE (a
     * stored lowercase UTYPE_CODE cannot be matched).
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteBehaviorOverride(String configJson, String feature, String usageType) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("feature", feature);
        wire.str("usage_type", usageType);
        return Invoker.config("delete_behavior_override", configJson, wire);
    }

    /**
     * Get the raw CFG_FBOVR row for a feature and usage type.
     *
     * <p>Notes:
     * Result is the stored row with on-disk keys (FTYPE_ID, UTYPE_CODE, FTYPE_FREQ,
     * FTYPE_EXCL, FTYPE_STAB).
     *
     * <p>Wire name: {@code get_behavior_override}; group: {@code behavior_overrides}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param feature <code>feature</code> (str) Feature code, case-insensitive; unknown is NOT_FOUND.
     * @param usageType <code>usage_type</code> (str) Uppercased, then matched exactly against UTYPE_CODE.
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String getBehaviorOverride(String configJson, String feature, String usageType) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("feature", feature);
        wire.str("usage_type", usageType);
        return Invoker.json("get_behavior_override", configJson, wire);
    }

    /**
     * List the raw CFG_FBOVR rows sorted by FTYPE_ID.
     *
     * <p>Notes:
     * Result is an array of stored rows (on-disk keys), stable-sorted by FTYPE_ID only (rows
     * sharing a FTYPE_ID keep config order). A config without CFG_FBOVR is MISSING_SECTION
     * (not []).
     *
     * <p>Wire name: {@code list_behavior_overrides}; group: {@code behavior_overrides}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String listBehaviorOverrides(String configJson) throws SzConfigToolException {
        Args wire = new Args();
        return Invoker.json("list_behavior_overrides", configJson, wire);
    }

    /**
     * List behavior overrides as {feature, usageType, behavior} display records.
     *
     * <p>Notes:
     * Result is an array of {feature, usageType, behavior} sorted by (FTYPE_ID, UTYPE_CODE).
     * feature is the FTYPE_CODE resolved from FTYPE_ID, or the id as a string when no
     * CFG_FTYPE row matches; behavior is FTYPE_FREQ plus E when FTYPE_EXCL and S when
     * FTYPE_STAB is Y/YES/1 (case-insensitive). A missing G2_CONFIG or CFG_FBOVR is
     * MISSING_SECTION.
     *
     * <p>Wire name: {@code list_behavior_overrides_resolved}; group: {@code behavior_overrides}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String listBehaviorOverridesResolved(String configJson) throws SzConfigToolException {
        Args wire = new Args();
        return Invoker.json("list_behavior_overrides_resolved", configJson, wire);
    }

    /** Optional arguments of {@link #addComparisonCall}; unset = omitted (library default). */
    public static final class AddComparisonCallOptions {
        final Args wire = new Args();

        /**
         * <code>id</code> (int) Requested CFCALL_ID. Absent OR &lt;= 0 means auto-allocate (max
         * existing + 1, floor 1000). A taken id &gt; 0 is ALREADY_EXISTS (checked before any
         * lookup).
         *
         * @param id the value
         * @return this builder
         */
        public AddComparisonCallOptions id(long id) {
            wire.integer("id", id);
            return this;
        }
    }

    /**
     * Add a comparison call (CFG_CFCALL row) binding a comparison function to a feature, with
     * its element list (CFG_CFBOM rows).
     *
     * <p>Notes:
     * Returns (modified config, the new CFG_CFCALL row {CFCALL_ID, FTYPE_ID, CFUNC_ID}).
     * Validation order: id (MISSING_SECTION if CFG_CFCALL is absent or not an array;
     * ALREADY_EXISTS if taken), feature, one-call-per-feature (ALREADY_PRESENT), function,
     * empty list, then per item blank check and element lookup; MISSING_SECTION if CFG_CFBOM
     * is absent. The function's applicability to the feature is not checked.
     *
     * <p>Wire name: {@code add_comparison_call}; group: {@code calls_comparison}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param ftypeCode <code>ftype_code</code> (str) Feature code; case-insensitive lookup in CFG_FTYPE, else
     * NOT_FOUND. Only one comparison call per feature: if any CFG_CFCALL row already has this
     * FTYPE_ID the call fails with ALREADY_PRESENT.
     * @param cfuncCode <code>cfunc_code</code> (str) Comparison function code; case-insensitive lookup in
     * CFG_CFUNC, else NOT_FOUND.
     * @param elementList <code>element_list</code> (str_list) Element codes, each a case-insensitive GLOBAL
     * CFG_FELEM lookup (the element need NOT be in the feature's CFG_FBOM), else NOT_FOUND.
     * Empty list or a blank/whitespace-only item is INVALID_INPUT. One CFG_CFBOM row is
     * written per item with FTYPE_ID = the call's feature and EXEC_ORDER = 1-based list
     * position (outside the exec-order allocation policy). Duplicate items are not rejected.
     * @return the modified configuration JSON document (opaque); {@link #addComparisonCallResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, ALREADY_EXISTS, NOT_FOUND, ALREADY_PRESENT, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addComparisonCall(String configJson, String ftypeCode, String cfuncCode, java.util.List<String> elementList) throws SzConfigToolException {
        return addComparisonCall(configJson, ftypeCode, cfuncCode, elementList, null);
    }

    /**
     * Add a comparison call (CFG_CFCALL row) binding a comparison function to a feature, with
     * its element list (CFG_CFBOM rows).
     *
     * <p>Notes:
     * Returns (modified config, the new CFG_CFCALL row {CFCALL_ID, FTYPE_ID, CFUNC_ID}).
     * Validation order: id (MISSING_SECTION if CFG_CFCALL is absent or not an array;
     * ALREADY_EXISTS if taken), feature, one-call-per-feature (ALREADY_PRESENT), function,
     * empty list, then per item blank check and element lookup; MISSING_SECTION if CFG_CFBOM
     * is absent. The function's applicability to the feature is not checked.
     *
     * <p>Wire name: {@code add_comparison_call}; group: {@code calls_comparison}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param ftypeCode <code>ftype_code</code> (str) Feature code; case-insensitive lookup in CFG_FTYPE, else
     * NOT_FOUND. Only one comparison call per feature: if any CFG_CFCALL row already has this
     * FTYPE_ID the call fails with ALREADY_PRESENT.
     * @param cfuncCode <code>cfunc_code</code> (str) Comparison function code; case-insensitive lookup in
     * CFG_CFUNC, else NOT_FOUND.
     * @param elementList <code>element_list</code> (str_list) Element codes, each a case-insensitive GLOBAL
     * CFG_FELEM lookup (the element need NOT be in the feature's CFG_FBOM), else NOT_FOUND.
     * Empty list or a blank/whitespace-only item is INVALID_INPUT. One CFG_CFBOM row is
     * written per item with FTYPE_ID = the call's feature and EXEC_ORDER = 1-based list
     * position (outside the exec-order allocation policy). Duplicate items are not rejected.
     * @param options optional arguments ({@code null} = none); see {@link AddComparisonCallOptions}
     * @return the modified configuration JSON document (opaque); {@link #addComparisonCallResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, ALREADY_EXISTS, NOT_FOUND, ALREADY_PRESENT, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addComparisonCall(String configJson, String ftypeCode, String cfuncCode, java.util.List<String> elementList, AddComparisonCallOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("ftype_code", ftypeCode);
        wire.str("cfunc_code", cfuncCode);
        wire.strList("element_list", elementList);
        return Invoker.call("add_comparison_call", "config_and_json", configJson, wire)[1];
    }

    /**
     * The record (row / ids) of <code>addComparisonCall</code>: same arguments and operation,
     * but returns the record instead of the configuration. Operation: Add a comparison call
     * (CFG_CFCALL row) binding a comparison function to a feature, with its element list
     * (CFG_CFBOM rows).
     *
     * <p>Notes:
     * Returns (modified config, the new CFG_CFCALL row {CFCALL_ID, FTYPE_ID, CFUNC_ID}).
     * Validation order: id (MISSING_SECTION if CFG_CFCALL is absent or not an array;
     * ALREADY_EXISTS if taken), feature, one-call-per-feature (ALREADY_PRESENT), function,
     * empty list, then per item blank check and element lookup; MISSING_SECTION if CFG_CFBOM
     * is absent. The function's applicability to the feature is not checked.
     *
     * <p>Wire name: {@code add_comparison_call}; group: {@code calls_comparison}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param ftypeCode <code>ftype_code</code> (str) Feature code; case-insensitive lookup in CFG_FTYPE, else
     * NOT_FOUND. Only one comparison call per feature: if any CFG_CFCALL row already has this
     * FTYPE_ID the call fails with ALREADY_PRESENT.
     * @param cfuncCode <code>cfunc_code</code> (str) Comparison function code; case-insensitive lookup in
     * CFG_CFUNC, else NOT_FOUND.
     * @param elementList <code>element_list</code> (str_list) Element codes, each a case-insensitive GLOBAL
     * CFG_FELEM lookup (the element need NOT be in the feature's CFG_FBOM), else NOT_FOUND.
     * Empty list or a blank/whitespace-only item is INVALID_INPUT. One CFG_CFBOM row is
     * written per item with FTYPE_ID = the call's feature and EXEC_ORDER = 1-based list
     * position (outside the exec-order allocation policy). Duplicate items are not rejected.
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, ALREADY_EXISTS, NOT_FOUND, ALREADY_PRESENT, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addComparisonCallResult(String configJson, String ftypeCode, String cfuncCode, java.util.List<String> elementList) throws SzConfigToolException {
        return addComparisonCallResult(configJson, ftypeCode, cfuncCode, elementList, null);
    }

    /**
     * The record (row / ids) of <code>addComparisonCall</code>: same arguments and operation,
     * but returns the record instead of the configuration. Operation: Add a comparison call
     * (CFG_CFCALL row) binding a comparison function to a feature, with its element list
     * (CFG_CFBOM rows).
     *
     * <p>Notes:
     * Returns (modified config, the new CFG_CFCALL row {CFCALL_ID, FTYPE_ID, CFUNC_ID}).
     * Validation order: id (MISSING_SECTION if CFG_CFCALL is absent or not an array;
     * ALREADY_EXISTS if taken), feature, one-call-per-feature (ALREADY_PRESENT), function,
     * empty list, then per item blank check and element lookup; MISSING_SECTION if CFG_CFBOM
     * is absent. The function's applicability to the feature is not checked.
     *
     * <p>Wire name: {@code add_comparison_call}; group: {@code calls_comparison}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param ftypeCode <code>ftype_code</code> (str) Feature code; case-insensitive lookup in CFG_FTYPE, else
     * NOT_FOUND. Only one comparison call per feature: if any CFG_CFCALL row already has this
     * FTYPE_ID the call fails with ALREADY_PRESENT.
     * @param cfuncCode <code>cfunc_code</code> (str) Comparison function code; case-insensitive lookup in
     * CFG_CFUNC, else NOT_FOUND.
     * @param elementList <code>element_list</code> (str_list) Element codes, each a case-insensitive GLOBAL
     * CFG_FELEM lookup (the element need NOT be in the feature's CFG_FBOM), else NOT_FOUND.
     * Empty list or a blank/whitespace-only item is INVALID_INPUT. One CFG_CFBOM row is
     * written per item with FTYPE_ID = the call's feature and EXEC_ORDER = 1-based list
     * position (outside the exec-order allocation policy). Duplicate items are not rejected.
     * @param options optional arguments ({@code null} = none); see {@link AddComparisonCallOptions}
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, ALREADY_EXISTS, NOT_FOUND, ALREADY_PRESENT, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addComparisonCallResult(String configJson, String ftypeCode, String cfuncCode, java.util.List<String> elementList, AddComparisonCallOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("ftype_code", ftypeCode);
        wire.str("cfunc_code", cfuncCode);
        wire.strList("element_list", elementList);
        return Invoker.result("add_comparison_call", "config_and_json", configJson, wire);
    }

    /**
     * Delete a comparison call by CFCALL_ID, cascading to its CFG_CFBOM rows.
     *
     * <p>Notes:
     * Removes the CFG_CFCALL row and every CFG_CFBOM row with that CFCALL_ID. No dependency or
     * system-call protection: template calls (ids &lt; 1000) can be deleted.
     *
     * <p>Wire name: {@code delete_comparison_call}; group: {@code calls_comparison}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param cfcallId <code>cfcall_id</code> (int) CFCALL_ID; must exist in CFG_CFCALL else NOT_FOUND (an
     * absent section is also NOT_FOUND).
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteComparisonCall(String configJson, long cfcallId) throws SzConfigToolException {
        Args wire = new Args();
        wire.integer("cfcall_id", cfcallId);
        return Invoker.config("delete_comparison_call", configJson, wire);
    }

    /**
     * Get one comparison call's raw CFG_CFCALL row, addressed by call id or by feature code.
     *
     * <p>Notes:
     * Result is the stored row with on-disk keys (CFCALL_ID, FTYPE_ID, CFUNC_ID); it does not
     * include the CFBOM elements (codes: list_comparison_calls; raw rows:
     * get_config_section("CFG_CFBOM")).
     *
     * <p>Wire name: {@code get_comparison_call}; group: {@code calls_comparison}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param call <code>call</code> (int_or_str) A JSON integer selects by CFCALL_ID; a JSON string
     * selects the call bound to that feature code (case-insensitive CFG_FTYPE lookup, then a
     * CFG_CFCALL scan by FTYPE_ID). Unknown id, unknown feature, or a feature with no
     * comparison call is NOT_FOUND; a feature matching 2+ calls (malformed config) is
     * INVALID_INPUT. Any other JSON type (or null) is INVALID_INPUT.
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String getComparisonCall(String configJson, long call) throws SzConfigToolException {
        Args wire = new Args();
        wire.integer("call", call);
        return Invoker.json("get_comparison_call", configJson, wire);
    }

    /**
     * Get one comparison call's raw CFG_CFCALL row, addressed by call id or by feature code.
     *
     * <p>Notes:
     * Result is the stored row with on-disk keys (CFCALL_ID, FTYPE_ID, CFUNC_ID); it does not
     * include the CFBOM elements (codes: list_comparison_calls; raw rows:
     * get_config_section("CFG_CFBOM")).
     *
     * <p>Wire name: {@code get_comparison_call}; group: {@code calls_comparison}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param call <code>call</code> (int_or_str) A JSON integer selects by CFCALL_ID; a JSON string
     * selects the call bound to that feature code (case-insensitive CFG_FTYPE lookup, then a
     * CFG_CFCALL scan by FTYPE_ID). Unknown id, unknown feature, or a feature with no
     * comparison call is NOT_FOUND; a feature matching 2+ calls (malformed config) is
     * INVALID_INPUT. Any other JSON type (or null) is INVALID_INPUT.
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String getComparisonCall(String configJson, String call) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("call", call);
        return Invoker.json("get_comparison_call", configJson, wire);
    }

    /**
     * List all comparison calls with feature/function codes resolved and their ordered element
     * lists.
     *
     * <p>Notes:
     * Result is an array of {id, feature, function, elementList} sorted by (FTYPE_ID,
     * CFCALL_ID) — not config order. elementList is the call's CFG_CFBOM element codes
     * ordered by EXEC_ORDER. Unresolvable ids render as the string "unknown". Missing sections
     * are treated as empty (never MISSING_SECTION). LIMITATION: elementList omits the stored
     * CFG_CFBOM columns (FTYPE_ID, EXEC_ORDER); read the raw rows with
     * get_config_section("CFG_CFBOM").
     *
     * <p>Wire name: {@code list_comparison_calls}; group: {@code calls_comparison}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String listComparisonCalls(String configJson) throws SzConfigToolException {
        Args wire = new Args();
        return Invoker.json("list_comparison_calls", configJson, wire);
    }

    /** Optional arguments of {@link #addComparisonCallElement}; unset = omitted (library default). */
    public static final class AddComparisonCallElementOptions {
        final Args wire = new Args();

        /**
         * <code>exec_order</code> (int) Allocated per CFCALL_ID. Absent OR &lt;= 0 means
         * auto-allocate (max EXEC_ORDER on this call + 1, seed 0 so an empty call starts at 1). A
         * taken order &gt; 0 on the same call is ALREADY_EXISTS.
         *
         * @param execOrder the value
         * @return this builder
         */
        public AddComparisonCallElementOptions execOrder(long execOrder) {
            wire.integer("exec_order", execOrder);
            return this;
        }
    }

    /**
     * Add one element (CFG_CFBOM row) to a comparison call, addressed by raw ids.
     *
     * <p>Notes:
     * Returns (modified config, the new CFG_CFBOM row {CFCALL_ID, FTYPE_ID, FELEM_ID,
     * EXEC_ORDER}). Duplicate identity is (CFCALL_ID, FTYPE_ID, FELEM_ID) regardless of
     * EXEC_ORDER -&gt; ALREADY_PRESENT. Order of checks: ftype_id &lt; 0, duplicate,
     * exec_order, then MISSING_SECTION if CFG_CFBOM is absent. The same FELEM_ID may be added
     * under a different ftype_id, which makes a later feature-less
     * delete_comparison_call_element ambiguous.
     *
     * <p>Wire name: {@code add_comparison_call_element}; group: {@code calls_comparison}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param cfcallId <code>cfcall_id</code> (int) CFCALL_ID written verbatim. NOT validated — the call need
     * not exist.
     * @param ftypeId <code>ftype_id</code> (int) The ELEMENT's feature id written to the BOM row's FTYPE_ID.
     * Negative is INVALID_INPUT; otherwise NOT validated against CFG_FTYPE.
     * @param felemId <code>felem_id</code> (int) FELEM_ID written verbatim. NOT validated against CFG_FELEM
     * or CFG_FBOM.
     * @return the modified configuration JSON document (opaque); {@link #addComparisonCallElementResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, INVALID_INPUT, ALREADY_PRESENT, ALREADY_EXISTS, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addComparisonCallElement(String configJson, long cfcallId, long ftypeId, long felemId) throws SzConfigToolException {
        return addComparisonCallElement(configJson, cfcallId, ftypeId, felemId, null);
    }

    /**
     * Add one element (CFG_CFBOM row) to a comparison call, addressed by raw ids.
     *
     * <p>Notes:
     * Returns (modified config, the new CFG_CFBOM row {CFCALL_ID, FTYPE_ID, FELEM_ID,
     * EXEC_ORDER}). Duplicate identity is (CFCALL_ID, FTYPE_ID, FELEM_ID) regardless of
     * EXEC_ORDER -&gt; ALREADY_PRESENT. Order of checks: ftype_id &lt; 0, duplicate,
     * exec_order, then MISSING_SECTION if CFG_CFBOM is absent. The same FELEM_ID may be added
     * under a different ftype_id, which makes a later feature-less
     * delete_comparison_call_element ambiguous.
     *
     * <p>Wire name: {@code add_comparison_call_element}; group: {@code calls_comparison}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param cfcallId <code>cfcall_id</code> (int) CFCALL_ID written verbatim. NOT validated — the call need
     * not exist.
     * @param ftypeId <code>ftype_id</code> (int) The ELEMENT's feature id written to the BOM row's FTYPE_ID.
     * Negative is INVALID_INPUT; otherwise NOT validated against CFG_FTYPE.
     * @param felemId <code>felem_id</code> (int) FELEM_ID written verbatim. NOT validated against CFG_FELEM
     * or CFG_FBOM.
     * @param options optional arguments ({@code null} = none); see {@link AddComparisonCallElementOptions}
     * @return the modified configuration JSON document (opaque); {@link #addComparisonCallElementResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, INVALID_INPUT, ALREADY_PRESENT, ALREADY_EXISTS, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addComparisonCallElement(String configJson, long cfcallId, long ftypeId, long felemId, AddComparisonCallElementOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.integer("cfcall_id", cfcallId);
        wire.integer("ftype_id", ftypeId);
        wire.integer("felem_id", felemId);
        return Invoker.call("add_comparison_call_element", "config_and_json", configJson, wire)[1];
    }

    /**
     * The record (row / ids) of <code>addComparisonCallElement</code>: same arguments and
     * operation, but returns the record instead of the configuration. Operation: Add one
     * element (CFG_CFBOM row) to a comparison call, addressed by raw ids.
     *
     * <p>Notes:
     * Returns (modified config, the new CFG_CFBOM row {CFCALL_ID, FTYPE_ID, FELEM_ID,
     * EXEC_ORDER}). Duplicate identity is (CFCALL_ID, FTYPE_ID, FELEM_ID) regardless of
     * EXEC_ORDER -&gt; ALREADY_PRESENT. Order of checks: ftype_id &lt; 0, duplicate,
     * exec_order, then MISSING_SECTION if CFG_CFBOM is absent. The same FELEM_ID may be added
     * under a different ftype_id, which makes a later feature-less
     * delete_comparison_call_element ambiguous.
     *
     * <p>Wire name: {@code add_comparison_call_element}; group: {@code calls_comparison}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param cfcallId <code>cfcall_id</code> (int) CFCALL_ID written verbatim. NOT validated — the call need
     * not exist.
     * @param ftypeId <code>ftype_id</code> (int) The ELEMENT's feature id written to the BOM row's FTYPE_ID.
     * Negative is INVALID_INPUT; otherwise NOT validated against CFG_FTYPE.
     * @param felemId <code>felem_id</code> (int) FELEM_ID written verbatim. NOT validated against CFG_FELEM
     * or CFG_FBOM.
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, INVALID_INPUT, ALREADY_PRESENT, ALREADY_EXISTS, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addComparisonCallElementResult(String configJson, long cfcallId, long ftypeId, long felemId) throws SzConfigToolException {
        return addComparisonCallElementResult(configJson, cfcallId, ftypeId, felemId, null);
    }

    /**
     * The record (row / ids) of <code>addComparisonCallElement</code>: same arguments and
     * operation, but returns the record instead of the configuration. Operation: Add one
     * element (CFG_CFBOM row) to a comparison call, addressed by raw ids.
     *
     * <p>Notes:
     * Returns (modified config, the new CFG_CFBOM row {CFCALL_ID, FTYPE_ID, FELEM_ID,
     * EXEC_ORDER}). Duplicate identity is (CFCALL_ID, FTYPE_ID, FELEM_ID) regardless of
     * EXEC_ORDER -&gt; ALREADY_PRESENT. Order of checks: ftype_id &lt; 0, duplicate,
     * exec_order, then MISSING_SECTION if CFG_CFBOM is absent. The same FELEM_ID may be added
     * under a different ftype_id, which makes a later feature-less
     * delete_comparison_call_element ambiguous.
     *
     * <p>Wire name: {@code add_comparison_call_element}; group: {@code calls_comparison}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param cfcallId <code>cfcall_id</code> (int) CFCALL_ID written verbatim. NOT validated — the call need
     * not exist.
     * @param ftypeId <code>ftype_id</code> (int) The ELEMENT's feature id written to the BOM row's FTYPE_ID.
     * Negative is INVALID_INPUT; otherwise NOT validated against CFG_FTYPE.
     * @param felemId <code>felem_id</code> (int) FELEM_ID written verbatim. NOT validated against CFG_FELEM
     * or CFG_FBOM.
     * @param options optional arguments ({@code null} = none); see {@link AddComparisonCallElementOptions}
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, INVALID_INPUT, ALREADY_PRESENT, ALREADY_EXISTS, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addComparisonCallElementResult(String configJson, long cfcallId, long ftypeId, long felemId, AddComparisonCallElementOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.integer("cfcall_id", cfcallId);
        wire.integer("ftype_id", ftypeId);
        wire.integer("felem_id", felemId);
        return Invoker.result("add_comparison_call_element", "config_and_json", configJson, wire);
    }

    /** Optional arguments of {@link #deleteComparisonCallElement}; unset = omitted (library default). */
    public static final class DeleteComparisonCallElementOptions {
        final Args wire = new Args();

        /**
         * <code>element_feature</code> (str) The element's feature code (case-insensitive; unknown
         * is NOT_FOUND). Narrows the BOM row match by FTYPE_ID and requires CFG_FBOM membership.
         *
         * @param elementFeature the value
         * @return this builder
         */
        public DeleteComparisonCallElementOptions elementFeature(String elementFeature) {
            wire.str("element_feature", elementFeature);
            return this;
        }
    }

    /**
     * Delete one element (CFG_CFBOM row) from a comparison call, addressed by call id or
     * feature code plus element code.
     *
     * <p>Notes:
     * EXEC_ORDER is derived from the matched BOM row (not supplied). Element not on an
     * existing call is the benign NOT_ON_CALL; an element matching 2+ BOM rows on the call
     * (same FELEM_ID under different FTYPE_IDs) without element_feature is INVALID_INPUT
     * (ambiguous). Only the matched row is removed; remaining rows keep their EXEC_ORDER (no
     * renumbering).
     *
     * <p>Wire name: {@code delete_comparison_call_element}; group: {@code calls_comparison}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param call <code>call</code> (int_or_str) A JSON integer selects by CFCALL_ID (must exist, else
     * NOT_FOUND); a JSON string selects the call bound to that feature code (unknown feature
     * or no call is NOT_FOUND; 2+ calls is INVALID_INPUT). Any other JSON type is
     * INVALID_INPUT.
     * @param elementCode <code>element_code</code> (str) Element code, case-insensitive. Without element_feature:
     * global CFG_FELEM lookup (unknown is NOT_FOUND). With element_feature: an unknown code or
     * one not in that feature's CFG_FBOM is NOT_IN_FEATURE.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT, NOT_IN_FEATURE, NOT_ON_CALL; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteComparisonCallElement(String configJson, long call, String elementCode) throws SzConfigToolException {
        return deleteComparisonCallElement(configJson, call, elementCode, null);
    }

    /**
     * Delete one element (CFG_CFBOM row) from a comparison call, addressed by call id or
     * feature code plus element code.
     *
     * <p>Notes:
     * EXEC_ORDER is derived from the matched BOM row (not supplied). Element not on an
     * existing call is the benign NOT_ON_CALL; an element matching 2+ BOM rows on the call
     * (same FELEM_ID under different FTYPE_IDs) without element_feature is INVALID_INPUT
     * (ambiguous). Only the matched row is removed; remaining rows keep their EXEC_ORDER (no
     * renumbering).
     *
     * <p>Wire name: {@code delete_comparison_call_element}; group: {@code calls_comparison}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param call <code>call</code> (int_or_str) A JSON integer selects by CFCALL_ID (must exist, else
     * NOT_FOUND); a JSON string selects the call bound to that feature code (unknown feature
     * or no call is NOT_FOUND; 2+ calls is INVALID_INPUT). Any other JSON type is
     * INVALID_INPUT.
     * @param elementCode <code>element_code</code> (str) Element code, case-insensitive. Without element_feature:
     * global CFG_FELEM lookup (unknown is NOT_FOUND). With element_feature: an unknown code or
     * one not in that feature's CFG_FBOM is NOT_IN_FEATURE.
     * @param options optional arguments ({@code null} = none); see {@link DeleteComparisonCallElementOptions}
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT, NOT_IN_FEATURE, NOT_ON_CALL; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteComparisonCallElement(String configJson, long call, String elementCode, DeleteComparisonCallElementOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.integer("call", call);
        wire.str("element_code", elementCode);
        return Invoker.config("delete_comparison_call_element", configJson, wire);
    }

    /**
     * Delete one element (CFG_CFBOM row) from a comparison call, addressed by call id or
     * feature code plus element code.
     *
     * <p>Notes:
     * EXEC_ORDER is derived from the matched BOM row (not supplied). Element not on an
     * existing call is the benign NOT_ON_CALL; an element matching 2+ BOM rows on the call
     * (same FELEM_ID under different FTYPE_IDs) without element_feature is INVALID_INPUT
     * (ambiguous). Only the matched row is removed; remaining rows keep their EXEC_ORDER (no
     * renumbering).
     *
     * <p>Wire name: {@code delete_comparison_call_element}; group: {@code calls_comparison}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param call <code>call</code> (int_or_str) A JSON integer selects by CFCALL_ID (must exist, else
     * NOT_FOUND); a JSON string selects the call bound to that feature code (unknown feature
     * or no call is NOT_FOUND; 2+ calls is INVALID_INPUT). Any other JSON type is
     * INVALID_INPUT.
     * @param elementCode <code>element_code</code> (str) Element code, case-insensitive. Without element_feature:
     * global CFG_FELEM lookup (unknown is NOT_FOUND). With element_feature: an unknown code or
     * one not in that feature's CFG_FBOM is NOT_IN_FEATURE.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT, NOT_IN_FEATURE, NOT_ON_CALL; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteComparisonCallElement(String configJson, String call, String elementCode) throws SzConfigToolException {
        return deleteComparisonCallElement(configJson, call, elementCode, null);
    }

    /**
     * Delete one element (CFG_CFBOM row) from a comparison call, addressed by call id or
     * feature code plus element code.
     *
     * <p>Notes:
     * EXEC_ORDER is derived from the matched BOM row (not supplied). Element not on an
     * existing call is the benign NOT_ON_CALL; an element matching 2+ BOM rows on the call
     * (same FELEM_ID under different FTYPE_IDs) without element_feature is INVALID_INPUT
     * (ambiguous). Only the matched row is removed; remaining rows keep their EXEC_ORDER (no
     * renumbering).
     *
     * <p>Wire name: {@code delete_comparison_call_element}; group: {@code calls_comparison}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param call <code>call</code> (int_or_str) A JSON integer selects by CFCALL_ID (must exist, else
     * NOT_FOUND); a JSON string selects the call bound to that feature code (unknown feature
     * or no call is NOT_FOUND; 2+ calls is INVALID_INPUT). Any other JSON type is
     * INVALID_INPUT.
     * @param elementCode <code>element_code</code> (str) Element code, case-insensitive. Without element_feature:
     * global CFG_FELEM lookup (unknown is NOT_FOUND). With element_feature: an unknown code or
     * one not in that feature's CFG_FBOM is NOT_IN_FEATURE.
     * @param options optional arguments ({@code null} = none); see {@link DeleteComparisonCallElementOptions}
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT, NOT_IN_FEATURE, NOT_ON_CALL; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteComparisonCallElement(String configJson, String call, String elementCode, DeleteComparisonCallElementOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("call", call);
        wire.str("element_code", elementCode);
        return Invoker.config("delete_comparison_call_element", configJson, wire);
    }

    /**
     * Add a distinct call (CFG_DFCALL row) binding a distinct function to a feature, with its
     * element list (CFG_DFBOM rows).
     *
     * <p>Notes:
     * Returns (modified config, the new CFG_DFCALL row {DFCALL_ID, FTYPE_ID, DFUNC_ID} — no
     * EXEC_ORDER). DFCALL_ID is ALWAYS auto-allocated (max existing + 1, floor 1000): unlike
     * add_comparison_call there is no <code>id</code> parameter. Validation order: empty list
     * / blank item, id (MISSING_SECTION if G2_CONFIG.CFG_DFCALL is absent), feature,
     * one-call-per-feature (ALREADY_PRESENT), function, element lookups; MISSING_SECTION if
     * CFG_DFCALL is not an array or CFG_DFBOM is absent.
     *
     * <p>Wire name: {@code add_distinct_call}; group: {@code calls_distinct}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param ftypeCode <code>ftype_code</code> (str) Feature code; case-insensitive lookup in CFG_FTYPE, else
     * NOT_FOUND. Only one distinct call per feature: if any CFG_DFCALL row already has this
     * FTYPE_ID the call fails with ALREADY_PRESENT.
     * @param dfuncCode <code>dfunc_code</code> (str) Distinct function code; case-insensitive lookup in
     * CFG_DFUNC, else NOT_FOUND.
     * @param elementList <code>element_list</code> (str_list) Element codes, each a case-insensitive GLOBAL
     * CFG_FELEM lookup (the element need NOT be in the feature's CFG_FBOM), else NOT_FOUND.
     * Empty list or a blank/whitespace-only item is INVALID_INPUT (checked before anything
     * else). One CFG_DFBOM row is written per item with FTYPE_ID = the call's feature and
     * EXEC_ORDER = 1-based list position. Duplicate items are not rejected.
     * @return the modified configuration JSON document (opaque); {@link #addDistinctCallResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, INVALID_INPUT, MISSING_SECTION, NOT_FOUND, ALREADY_PRESENT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addDistinctCall(String configJson, String ftypeCode, String dfuncCode, java.util.List<String> elementList) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("ftype_code", ftypeCode);
        wire.str("dfunc_code", dfuncCode);
        wire.strList("element_list", elementList);
        return Invoker.call("add_distinct_call", "config_and_json", configJson, wire)[1];
    }

    /**
     * The record (row / ids) of <code>addDistinctCall</code>: same arguments and operation,
     * but returns the record instead of the configuration. Operation: Add a distinct call
     * (CFG_DFCALL row) binding a distinct function to a feature, with its element list
     * (CFG_DFBOM rows).
     *
     * <p>Notes:
     * Returns (modified config, the new CFG_DFCALL row {DFCALL_ID, FTYPE_ID, DFUNC_ID} — no
     * EXEC_ORDER). DFCALL_ID is ALWAYS auto-allocated (max existing + 1, floor 1000): unlike
     * add_comparison_call there is no <code>id</code> parameter. Validation order: empty list
     * / blank item, id (MISSING_SECTION if G2_CONFIG.CFG_DFCALL is absent), feature,
     * one-call-per-feature (ALREADY_PRESENT), function, element lookups; MISSING_SECTION if
     * CFG_DFCALL is not an array or CFG_DFBOM is absent.
     *
     * <p>Wire name: {@code add_distinct_call}; group: {@code calls_distinct}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param ftypeCode <code>ftype_code</code> (str) Feature code; case-insensitive lookup in CFG_FTYPE, else
     * NOT_FOUND. Only one distinct call per feature: if any CFG_DFCALL row already has this
     * FTYPE_ID the call fails with ALREADY_PRESENT.
     * @param dfuncCode <code>dfunc_code</code> (str) Distinct function code; case-insensitive lookup in
     * CFG_DFUNC, else NOT_FOUND.
     * @param elementList <code>element_list</code> (str_list) Element codes, each a case-insensitive GLOBAL
     * CFG_FELEM lookup (the element need NOT be in the feature's CFG_FBOM), else NOT_FOUND.
     * Empty list or a blank/whitespace-only item is INVALID_INPUT (checked before anything
     * else). One CFG_DFBOM row is written per item with FTYPE_ID = the call's feature and
     * EXEC_ORDER = 1-based list position. Duplicate items are not rejected.
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, INVALID_INPUT, MISSING_SECTION, NOT_FOUND, ALREADY_PRESENT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addDistinctCallResult(String configJson, String ftypeCode, String dfuncCode, java.util.List<String> elementList) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("ftype_code", ftypeCode);
        wire.str("dfunc_code", dfuncCode);
        wire.strList("element_list", elementList);
        return Invoker.result("add_distinct_call", "config_and_json", configJson, wire);
    }

    /**
     * Delete a distinct call by DFCALL_ID, cascading to its CFG_DFBOM rows.
     *
     * <p>Notes:
     * Removes the CFG_DFCALL row and every CFG_DFBOM row with that DFCALL_ID. No dependency or
     * system-call protection.
     *
     * <p>Wire name: {@code delete_distinct_call}; group: {@code calls_distinct}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param dfcallId <code>dfcall_id</code> (int) DFCALL_ID; must exist in CFG_DFCALL else NOT_FOUND (an
     * absent section is also NOT_FOUND).
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteDistinctCall(String configJson, long dfcallId) throws SzConfigToolException {
        Args wire = new Args();
        wire.integer("dfcall_id", dfcallId);
        return Invoker.config("delete_distinct_call", configJson, wire);
    }

    /**
     * Get one distinct call's raw CFG_DFCALL row, addressed by call id or by feature code.
     *
     * <p>Notes:
     * Result is the stored row with on-disk keys (DFCALL_ID, FTYPE_ID, DFUNC_ID); it does not
     * include the DFBOM elements (codes: list_distinct_calls; raw rows:
     * get_config_section("CFG_DFBOM")).
     *
     * <p>Wire name: {@code get_distinct_call}; group: {@code calls_distinct}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param call <code>call</code> (int_or_str) A JSON integer selects by DFCALL_ID; a JSON string
     * selects the call bound to that feature code (case-insensitive CFG_FTYPE lookup, then a
     * CFG_DFCALL scan by FTYPE_ID). Unknown id, unknown feature, or a feature with no distinct
     * call is NOT_FOUND; a feature matching 2+ calls (malformed config) is INVALID_INPUT. Any
     * other JSON type (or null) is INVALID_INPUT.
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String getDistinctCall(String configJson, long call) throws SzConfigToolException {
        Args wire = new Args();
        wire.integer("call", call);
        return Invoker.json("get_distinct_call", configJson, wire);
    }

    /**
     * Get one distinct call's raw CFG_DFCALL row, addressed by call id or by feature code.
     *
     * <p>Notes:
     * Result is the stored row with on-disk keys (DFCALL_ID, FTYPE_ID, DFUNC_ID); it does not
     * include the DFBOM elements (codes: list_distinct_calls; raw rows:
     * get_config_section("CFG_DFBOM")).
     *
     * <p>Wire name: {@code get_distinct_call}; group: {@code calls_distinct}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param call <code>call</code> (int_or_str) A JSON integer selects by DFCALL_ID; a JSON string
     * selects the call bound to that feature code (case-insensitive CFG_FTYPE lookup, then a
     * CFG_DFCALL scan by FTYPE_ID). Unknown id, unknown feature, or a feature with no distinct
     * call is NOT_FOUND; a feature matching 2+ calls (malformed config) is INVALID_INPUT. Any
     * other JSON type (or null) is INVALID_INPUT.
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String getDistinctCall(String configJson, String call) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("call", call);
        return Invoker.json("get_distinct_call", configJson, wire);
    }

    /**
     * List all distinct calls with feature/function codes resolved and their ordered element
     * lists.
     *
     * <p>Notes:
     * Result is an array of {id, feature, function, execOrder, elementList} sorted by
     * (FTYPE_ID, DFCALL_ID) — not config order. execOrder is the CFG_DFCALL row's
     * EXEC_ORDER, which the v4 schema (and every template / add_distinct_call row) lacks, so
     * it is 1. elementList is the call's CFG_DFBOM element codes ordered by EXEC_ORDER.
     * Unresolvable ids render as "unknown". Missing sections are treated as empty. LIMITATION:
     * elementList omits the stored CFG_DFBOM columns (FTYPE_ID, EXEC_ORDER); read the raw rows
     * with get_config_section("CFG_DFBOM").
     *
     * <p>Wire name: {@code list_distinct_calls}; group: {@code calls_distinct}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String listDistinctCalls(String configJson) throws SzConfigToolException {
        Args wire = new Args();
        return Invoker.json("list_distinct_calls", configJson, wire);
    }

    /** Optional arguments of {@link #addDistinctCallElement}; unset = omitted (library default). */
    public static final class AddDistinctCallElementOptions {
        final Args wire = new Args();

        /**
         * <code>exec_order</code> (int) Allocated per DFCALL_ID. Absent OR &lt;= 0 means
         * auto-allocate (max EXEC_ORDER on this call + 1, seed 0). A taken order &gt; 0 on the
         * same call is ALREADY_EXISTS.
         *
         * @param execOrder the value
         * @return this builder
         */
        public AddDistinctCallElementOptions execOrder(long execOrder) {
            wire.integer("exec_order", execOrder);
            return this;
        }
    }

    /**
     * Add one element (CFG_DFBOM row) to a distinct call, addressed by raw ids.
     *
     * <p>Notes:
     * Returns (modified config, the new CFG_DFBOM row {DFCALL_ID, FTYPE_ID, FELEM_ID,
     * EXEC_ORDER}). Duplicate identity is (DFCALL_ID, FTYPE_ID, FELEM_ID) regardless of
     * EXEC_ORDER -&gt; ALREADY_PRESENT. Order of checks: duplicate, exec_order, then
     * MISSING_SECTION if CFG_DFBOM is absent.
     *
     * <p>Wire name: {@code add_distinct_call_element}; group: {@code calls_distinct}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param dfcallId <code>dfcall_id</code> (int) DFCALL_ID written verbatim. NOT validated — the call need
     * not exist.
     * @param ftypeId <code>ftype_id</code> (int) The ELEMENT's feature id written to the BOM row's FTYPE_ID.
     * NOT validated at all — unlike add_comparison_call_element, a negative id is accepted
     * and stored.
     * @param felemId <code>felem_id</code> (int) FELEM_ID written verbatim. NOT validated against CFG_FELEM
     * or CFG_FBOM.
     * @return the modified configuration JSON document (opaque); {@link #addDistinctCallElementResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, ALREADY_PRESENT, ALREADY_EXISTS, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addDistinctCallElement(String configJson, long dfcallId, long ftypeId, long felemId) throws SzConfigToolException {
        return addDistinctCallElement(configJson, dfcallId, ftypeId, felemId, null);
    }

    /**
     * Add one element (CFG_DFBOM row) to a distinct call, addressed by raw ids.
     *
     * <p>Notes:
     * Returns (modified config, the new CFG_DFBOM row {DFCALL_ID, FTYPE_ID, FELEM_ID,
     * EXEC_ORDER}). Duplicate identity is (DFCALL_ID, FTYPE_ID, FELEM_ID) regardless of
     * EXEC_ORDER -&gt; ALREADY_PRESENT. Order of checks: duplicate, exec_order, then
     * MISSING_SECTION if CFG_DFBOM is absent.
     *
     * <p>Wire name: {@code add_distinct_call_element}; group: {@code calls_distinct}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param dfcallId <code>dfcall_id</code> (int) DFCALL_ID written verbatim. NOT validated — the call need
     * not exist.
     * @param ftypeId <code>ftype_id</code> (int) The ELEMENT's feature id written to the BOM row's FTYPE_ID.
     * NOT validated at all — unlike add_comparison_call_element, a negative id is accepted
     * and stored.
     * @param felemId <code>felem_id</code> (int) FELEM_ID written verbatim. NOT validated against CFG_FELEM
     * or CFG_FBOM.
     * @param options optional arguments ({@code null} = none); see {@link AddDistinctCallElementOptions}
     * @return the modified configuration JSON document (opaque); {@link #addDistinctCallElementResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, ALREADY_PRESENT, ALREADY_EXISTS, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addDistinctCallElement(String configJson, long dfcallId, long ftypeId, long felemId, AddDistinctCallElementOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.integer("dfcall_id", dfcallId);
        wire.integer("ftype_id", ftypeId);
        wire.integer("felem_id", felemId);
        return Invoker.call("add_distinct_call_element", "config_and_json", configJson, wire)[1];
    }

    /**
     * The record (row / ids) of <code>addDistinctCallElement</code>: same arguments and
     * operation, but returns the record instead of the configuration. Operation: Add one
     * element (CFG_DFBOM row) to a distinct call, addressed by raw ids.
     *
     * <p>Notes:
     * Returns (modified config, the new CFG_DFBOM row {DFCALL_ID, FTYPE_ID, FELEM_ID,
     * EXEC_ORDER}). Duplicate identity is (DFCALL_ID, FTYPE_ID, FELEM_ID) regardless of
     * EXEC_ORDER -&gt; ALREADY_PRESENT. Order of checks: duplicate, exec_order, then
     * MISSING_SECTION if CFG_DFBOM is absent.
     *
     * <p>Wire name: {@code add_distinct_call_element}; group: {@code calls_distinct}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param dfcallId <code>dfcall_id</code> (int) DFCALL_ID written verbatim. NOT validated — the call need
     * not exist.
     * @param ftypeId <code>ftype_id</code> (int) The ELEMENT's feature id written to the BOM row's FTYPE_ID.
     * NOT validated at all — unlike add_comparison_call_element, a negative id is accepted
     * and stored.
     * @param felemId <code>felem_id</code> (int) FELEM_ID written verbatim. NOT validated against CFG_FELEM
     * or CFG_FBOM.
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, ALREADY_PRESENT, ALREADY_EXISTS, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addDistinctCallElementResult(String configJson, long dfcallId, long ftypeId, long felemId) throws SzConfigToolException {
        return addDistinctCallElementResult(configJson, dfcallId, ftypeId, felemId, null);
    }

    /**
     * The record (row / ids) of <code>addDistinctCallElement</code>: same arguments and
     * operation, but returns the record instead of the configuration. Operation: Add one
     * element (CFG_DFBOM row) to a distinct call, addressed by raw ids.
     *
     * <p>Notes:
     * Returns (modified config, the new CFG_DFBOM row {DFCALL_ID, FTYPE_ID, FELEM_ID,
     * EXEC_ORDER}). Duplicate identity is (DFCALL_ID, FTYPE_ID, FELEM_ID) regardless of
     * EXEC_ORDER -&gt; ALREADY_PRESENT. Order of checks: duplicate, exec_order, then
     * MISSING_SECTION if CFG_DFBOM is absent.
     *
     * <p>Wire name: {@code add_distinct_call_element}; group: {@code calls_distinct}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param dfcallId <code>dfcall_id</code> (int) DFCALL_ID written verbatim. NOT validated — the call need
     * not exist.
     * @param ftypeId <code>ftype_id</code> (int) The ELEMENT's feature id written to the BOM row's FTYPE_ID.
     * NOT validated at all — unlike add_comparison_call_element, a negative id is accepted
     * and stored.
     * @param felemId <code>felem_id</code> (int) FELEM_ID written verbatim. NOT validated against CFG_FELEM
     * or CFG_FBOM.
     * @param options optional arguments ({@code null} = none); see {@link AddDistinctCallElementOptions}
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, ALREADY_PRESENT, ALREADY_EXISTS, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addDistinctCallElementResult(String configJson, long dfcallId, long ftypeId, long felemId, AddDistinctCallElementOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.integer("dfcall_id", dfcallId);
        wire.integer("ftype_id", ftypeId);
        wire.integer("felem_id", felemId);
        return Invoker.result("add_distinct_call_element", "config_and_json", configJson, wire);
    }

    /** Optional arguments of {@link #deleteDistinctCallElement}; unset = omitted (library default). */
    public static final class DeleteDistinctCallElementOptions {
        final Args wire = new Args();

        /**
         * <code>element_feature</code> (str) The element's feature code (case-insensitive; unknown
         * is NOT_FOUND). Narrows the BOM row match by FTYPE_ID and requires CFG_FBOM membership.
         *
         * @param elementFeature the value
         * @return this builder
         */
        public DeleteDistinctCallElementOptions elementFeature(String elementFeature) {
            wire.str("element_feature", elementFeature);
            return this;
        }
    }

    /**
     * Delete one element (CFG_DFBOM row) from a distinct call, addressed by call id or feature
     * code plus element code.
     *
     * <p>Notes:
     * EXEC_ORDER is derived from the matched BOM row (not supplied). Element not on an
     * existing call is the benign NOT_ON_CALL; an element matching 2+ BOM rows on the call
     * without element_feature is INVALID_INPUT (ambiguous). Only the matched row is removed;
     * no renumbering.
     *
     * <p>Wire name: {@code delete_distinct_call_element}; group: {@code calls_distinct}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param call <code>call</code> (int_or_str) A JSON integer selects by DFCALL_ID (must exist, else
     * NOT_FOUND); a JSON string selects the call bound to that feature code (unknown feature
     * or no call is NOT_FOUND; 2+ calls is INVALID_INPUT). Any other JSON type is
     * INVALID_INPUT.
     * @param elementCode <code>element_code</code> (str) Element code, case-insensitive. Without element_feature:
     * global CFG_FELEM lookup (unknown is NOT_FOUND). With element_feature: an unknown code or
     * one not in that feature's CFG_FBOM is NOT_IN_FEATURE.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT, NOT_IN_FEATURE, NOT_ON_CALL; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteDistinctCallElement(String configJson, long call, String elementCode) throws SzConfigToolException {
        return deleteDistinctCallElement(configJson, call, elementCode, null);
    }

    /**
     * Delete one element (CFG_DFBOM row) from a distinct call, addressed by call id or feature
     * code plus element code.
     *
     * <p>Notes:
     * EXEC_ORDER is derived from the matched BOM row (not supplied). Element not on an
     * existing call is the benign NOT_ON_CALL; an element matching 2+ BOM rows on the call
     * without element_feature is INVALID_INPUT (ambiguous). Only the matched row is removed;
     * no renumbering.
     *
     * <p>Wire name: {@code delete_distinct_call_element}; group: {@code calls_distinct}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param call <code>call</code> (int_or_str) A JSON integer selects by DFCALL_ID (must exist, else
     * NOT_FOUND); a JSON string selects the call bound to that feature code (unknown feature
     * or no call is NOT_FOUND; 2+ calls is INVALID_INPUT). Any other JSON type is
     * INVALID_INPUT.
     * @param elementCode <code>element_code</code> (str) Element code, case-insensitive. Without element_feature:
     * global CFG_FELEM lookup (unknown is NOT_FOUND). With element_feature: an unknown code or
     * one not in that feature's CFG_FBOM is NOT_IN_FEATURE.
     * @param options optional arguments ({@code null} = none); see {@link DeleteDistinctCallElementOptions}
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT, NOT_IN_FEATURE, NOT_ON_CALL; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteDistinctCallElement(String configJson, long call, String elementCode, DeleteDistinctCallElementOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.integer("call", call);
        wire.str("element_code", elementCode);
        return Invoker.config("delete_distinct_call_element", configJson, wire);
    }

    /**
     * Delete one element (CFG_DFBOM row) from a distinct call, addressed by call id or feature
     * code plus element code.
     *
     * <p>Notes:
     * EXEC_ORDER is derived from the matched BOM row (not supplied). Element not on an
     * existing call is the benign NOT_ON_CALL; an element matching 2+ BOM rows on the call
     * without element_feature is INVALID_INPUT (ambiguous). Only the matched row is removed;
     * no renumbering.
     *
     * <p>Wire name: {@code delete_distinct_call_element}; group: {@code calls_distinct}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param call <code>call</code> (int_or_str) A JSON integer selects by DFCALL_ID (must exist, else
     * NOT_FOUND); a JSON string selects the call bound to that feature code (unknown feature
     * or no call is NOT_FOUND; 2+ calls is INVALID_INPUT). Any other JSON type is
     * INVALID_INPUT.
     * @param elementCode <code>element_code</code> (str) Element code, case-insensitive. Without element_feature:
     * global CFG_FELEM lookup (unknown is NOT_FOUND). With element_feature: an unknown code or
     * one not in that feature's CFG_FBOM is NOT_IN_FEATURE.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT, NOT_IN_FEATURE, NOT_ON_CALL; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteDistinctCallElement(String configJson, String call, String elementCode) throws SzConfigToolException {
        return deleteDistinctCallElement(configJson, call, elementCode, null);
    }

    /**
     * Delete one element (CFG_DFBOM row) from a distinct call, addressed by call id or feature
     * code plus element code.
     *
     * <p>Notes:
     * EXEC_ORDER is derived from the matched BOM row (not supplied). Element not on an
     * existing call is the benign NOT_ON_CALL; an element matching 2+ BOM rows on the call
     * without element_feature is INVALID_INPUT (ambiguous). Only the matched row is removed;
     * no renumbering.
     *
     * <p>Wire name: {@code delete_distinct_call_element}; group: {@code calls_distinct}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param call <code>call</code> (int_or_str) A JSON integer selects by DFCALL_ID (must exist, else
     * NOT_FOUND); a JSON string selects the call bound to that feature code (unknown feature
     * or no call is NOT_FOUND; 2+ calls is INVALID_INPUT). Any other JSON type is
     * INVALID_INPUT.
     * @param elementCode <code>element_code</code> (str) Element code, case-insensitive. Without element_feature:
     * global CFG_FELEM lookup (unknown is NOT_FOUND). With element_feature: an unknown code or
     * one not in that feature's CFG_FBOM is NOT_IN_FEATURE.
     * @param options optional arguments ({@code null} = none); see {@link DeleteDistinctCallElementOptions}
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT, NOT_IN_FEATURE, NOT_ON_CALL; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteDistinctCallElement(String configJson, String call, String elementCode, DeleteDistinctCallElementOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("call", call);
        wire.str("element_code", elementCode);
        return Invoker.config("delete_distinct_call_element", configJson, wire);
    }

    /** Optional arguments of {@link #addExpressionCall}; unset = omitted (library default). */
    public static final class AddExpressionCallOptions {
        final Args wire = new Args();

        /**
         * <code>ftype_code</code> (str) Feature code (case-insensitive) or NOT_FOUND; "ALL"
         * (case-insensitive) = absent; absent stores FTYPE_ID -1.
         *
         * @param ftypeCode the value
         * @return this builder
         */
        public AddExpressionCallOptions ftypeCode(String ftypeCode) {
            wire.str("ftype_code", ftypeCode);
            return this;
        }

        /**
         * <code>felem_code</code> (str) Element code (case-insensitive) or NOT_FOUND; "N/A"
         * (case-insensitive) = absent; absent stores FELEM_ID -1. Exactly one of ftype_code /
         * felem_code must resolve, else INVALID_INPUT.
         *
         * @param felemCode the value
         * @return this builder
         */
        public AddExpressionCallOptions felemCode(String felemCode) {
            wire.str("felem_code", felemCode);
            return this;
        }

        /**
         * <code>exec_order</code> (int) CFG_EFCALL EXEC_ORDER scoped per (FTYPE_ID, FELEM_ID).
         * Absent or &lt;= 0 = auto-allocate (max in scope + 1); &gt; 0 and free = verbatim; &gt; 0
         * and taken = ALREADY_EXISTS.
         *
         * @param execOrder the value
         * @return this builder
         */
        public AddExpressionCallOptions execOrder(long execOrder) {
            wire.integer("exec_order", execOrder);
            return this;
        }

        /**
         * <code>expression_feature</code> (str) Feature code stored as EFEAT_FTYPE_ID
         * (case-insensitive) or NOT_FOUND; absent or "N/A" (case-insensitive) stores -1.
         *
         * @param expressionFeature the value
         * @return this builder
         */
        public AddExpressionCallOptions expressionFeature(String expressionFeature) {
            wire.str("expression_feature", expressionFeature);
            return this;
        }
    }

    /**
     * Add an expression call (CFG_EFCALL row) plus its element list (CFG_EFBOM rows).
     *
     * <p>Notes:
     * Returns (modified config, the new CFG_EFCALL row {EFCALL_ID, FTYPE_ID, FELEM_ID,
     * EFUNC_ID, EXEC_ORDER, EFEAT_FTYPE_ID, IS_VIRTUAL}); the created CFG_EFBOM rows are NOT
     * in the record (see list_expression_calls). EFCALL_ID is auto-allocated (max + 1, floor
     * 1000). Check order: EFCALL_ID allocation (MISSING_SECTION if CFG_EFCALL absent), efunc,
     * feature, element, exactly-one rule, exec order, expression_feature, element list, then
     * MISSING_SECTION if CFG_EFBOM absent. BOM FTYPE_ID sentinels (G2 EFBomConfig.cpp): 0 =
     * parent feature link, -1 = any feature. The BOM-feature column is not rendered by
     * get/list_expression_calls; read raw rows with get_config_section("CFG_EFBOM").
     *
     * <p>Wire name: {@code add_expression_call}; group: {@code calls_expression}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param efuncCode <code>efunc_code</code> (str) Expression function code (CFG_EFUNC, case-insensitive) or
     * NOT_FOUND.
     * @param elementList <code>element_list</code> (json) JSON array of {"element": str, "required": str,
     * "feature"?: str} objects (unknown keys, non-objects, non-string values = INVALID_INPUT;
     * missing element/required = MISSING_FIELD). One CFG_EFBOM row per item, EXEC_ORDER =
     * 1-based list position. element: global CFG_FELEM lookup (case-insensitive) or NOT_FOUND.
     * required: stored verbatim in FELEM_REQ (not validated or normalized). feature: absent
     * stores BOM FTYPE_ID -1 (G2 WILDCARDED_FTYPE: any feature in the record carrying the
     * element); "PARENT" (case-insensitive) stores BOM FTYPE_ID 0 (G2
     * PARENT_FEATURE_LINKED_FTYPE: the feature that triggered the call); otherwise a feature
     * code (case-insensitive) or NOT_FOUND. The element is NOT checked for membership in that
     * feature. [] is allowed.
     * @param isVirtual <code>is_virtual</code> (str) Stored verbatim in IS_VIRTUAL (not validated or
     * normalized; the Rust <code>new()</code> default is "No").
     * @param options optional arguments ({@code null} = none); see {@link AddExpressionCallOptions}
     * @return the modified configuration JSON document (opaque); {@link #addExpressionCallResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND, INVALID_INPUT, ALREADY_EXISTS, MISSING_FIELD; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addExpressionCall(String configJson, String efuncCode, String elementList, String isVirtual, AddExpressionCallOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("efunc_code", efuncCode);
        wire.json("element_list", elementList);
        wire.str("is_virtual", isVirtual);
        return Invoker.call("add_expression_call", "config_and_json", configJson, wire)[1];
    }

    /**
     * The record (row / ids) of <code>addExpressionCall</code>: same arguments and operation,
     * but returns the record instead of the configuration. Operation: Add an expression call
     * (CFG_EFCALL row) plus its element list (CFG_EFBOM rows).
     *
     * <p>Notes:
     * Returns (modified config, the new CFG_EFCALL row {EFCALL_ID, FTYPE_ID, FELEM_ID,
     * EFUNC_ID, EXEC_ORDER, EFEAT_FTYPE_ID, IS_VIRTUAL}); the created CFG_EFBOM rows are NOT
     * in the record (see list_expression_calls). EFCALL_ID is auto-allocated (max + 1, floor
     * 1000). Check order: EFCALL_ID allocation (MISSING_SECTION if CFG_EFCALL absent), efunc,
     * feature, element, exactly-one rule, exec order, expression_feature, element list, then
     * MISSING_SECTION if CFG_EFBOM absent. BOM FTYPE_ID sentinels (G2 EFBomConfig.cpp): 0 =
     * parent feature link, -1 = any feature. The BOM-feature column is not rendered by
     * get/list_expression_calls; read raw rows with get_config_section("CFG_EFBOM").
     *
     * <p>Wire name: {@code add_expression_call}; group: {@code calls_expression}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param efuncCode <code>efunc_code</code> (str) Expression function code (CFG_EFUNC, case-insensitive) or
     * NOT_FOUND.
     * @param elementList <code>element_list</code> (json) JSON array of {"element": str, "required": str,
     * "feature"?: str} objects (unknown keys, non-objects, non-string values = INVALID_INPUT;
     * missing element/required = MISSING_FIELD). One CFG_EFBOM row per item, EXEC_ORDER =
     * 1-based list position. element: global CFG_FELEM lookup (case-insensitive) or NOT_FOUND.
     * required: stored verbatim in FELEM_REQ (not validated or normalized). feature: absent
     * stores BOM FTYPE_ID -1 (G2 WILDCARDED_FTYPE: any feature in the record carrying the
     * element); "PARENT" (case-insensitive) stores BOM FTYPE_ID 0 (G2
     * PARENT_FEATURE_LINKED_FTYPE: the feature that triggered the call); otherwise a feature
     * code (case-insensitive) or NOT_FOUND. The element is NOT checked for membership in that
     * feature. [] is allowed.
     * @param isVirtual <code>is_virtual</code> (str) Stored verbatim in IS_VIRTUAL (not validated or
     * normalized; the Rust <code>new()</code> default is "No").
     * @param options optional arguments ({@code null} = none); see {@link AddExpressionCallOptions}
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND, INVALID_INPUT, ALREADY_EXISTS, MISSING_FIELD; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addExpressionCallResult(String configJson, String efuncCode, String elementList, String isVirtual, AddExpressionCallOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("efunc_code", efuncCode);
        wire.json("element_list", elementList);
        wire.str("is_virtual", isVirtual);
        return Invoker.result("add_expression_call", "config_and_json", configJson, wire);
    }

    /**
     * Delete an expression call by EFCALL_ID, cascading its CFG_EFBOM rows.
     *
     * <p>Notes:
     * Removes the CFG_EFCALL row(s) with that id and every CFG_EFBOM row with that EFCALL_ID.
     *
     * <p>Wire name: {@code delete_expression_call}; group: {@code calls_expression}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param efcallId <code>efcall_id</code> (int) Must match an existing EFCALL_ID, else NOT_FOUND.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteExpressionCall(String configJson, long efcallId) throws SzConfigToolException {
        Args wire = new Args();
        wire.integer("efcall_id", efcallId);
        return Invoker.config("delete_expression_call", configJson, wire);
    }

    /**
     * Get one expression call's raw CFG_EFCALL row, by EFCALL_ID or by feature code.
     *
     * <p>Notes:
     * Result uses on-disk keys (EFCALL_ID, FTYPE_ID, FELEM_ID, EFUNC_ID, EXEC_ORDER,
     * EFEAT_FTYPE_ID, IS_VIRTUAL); BOM rows are not included (raw rows:
     * get_config_section("CFG_EFBOM")). Expression calls are many-per-feature (template NAME
     * has 7), so by-feature is often ambiguous.
     *
     * <p>Wire name: {@code get_expression_call}; group: {@code calls_expression}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param call <code>call</code> (int_or_str) Call selector: an integer = EFCALL_ID (NOT_FOUND if
     * absent); a string = feature code (case-insensitive; unknown feature = NOT_FOUND, no call
     * on the feature = NOT_FOUND, more than one call on the feature = INVALID_INPUT —
     * address such calls by id). Any other JSON type = INVALID_INPUT.
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String getExpressionCall(String configJson, long call) throws SzConfigToolException {
        Args wire = new Args();
        wire.integer("call", call);
        return Invoker.json("get_expression_call", configJson, wire);
    }

    /**
     * Get one expression call's raw CFG_EFCALL row, by EFCALL_ID or by feature code.
     *
     * <p>Notes:
     * Result uses on-disk keys (EFCALL_ID, FTYPE_ID, FELEM_ID, EFUNC_ID, EXEC_ORDER,
     * EFEAT_FTYPE_ID, IS_VIRTUAL); BOM rows are not included (raw rows:
     * get_config_section("CFG_EFBOM")). Expression calls are many-per-feature (template NAME
     * has 7), so by-feature is often ambiguous.
     *
     * <p>Wire name: {@code get_expression_call}; group: {@code calls_expression}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param call <code>call</code> (int_or_str) Call selector: an integer = EFCALL_ID (NOT_FOUND if
     * absent); a string = feature code (case-insensitive; unknown feature = NOT_FOUND, no call
     * on the feature = NOT_FOUND, more than one call on the feature = INVALID_INPUT —
     * address such calls by id). Any other JSON type = INVALID_INPUT.
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String getExpressionCall(String configJson, String call) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("call", call);
        return Invoker.json("get_expression_call", configJson, wire);
    }

    /**
     * List all expression calls with resolved codes and element lists.
     *
     * <p>Notes:
     * Array of {id, feature, element, execOrder, function, isVirtual, expressionFeature,
     * elementList}, stably sorted by (FTYPE_ID, FELEM_ID, EXEC_ORDER). feature "all" / element
     * "n/a" / function "unknown" when the id is &lt;= 0 or unresolved; expressionFeature "n/a"
     * when EFEAT_FTYPE_ID &lt;= 0. elementList is the BOM element codes ordered by BOM
     * EXEC_ORDER. Missing sections yield []. LIMITATION: elementList omits the stored
     * CFG_EFBOM columns FTYPE_ID (0 = parent feature, -1 = any feature), EXEC_ORDER and
     * FELEM_REQ, which the engine uses; read the raw rows with
     * get_config_section("CFG_EFBOM").
     *
     * <p>Wire name: {@code list_expression_calls}; group: {@code calls_expression}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String listExpressionCalls(String configJson) throws SzConfigToolException {
        Args wire = new Args();
        return Invoker.json("list_expression_calls", configJson, wire);
    }

    /** Optional arguments of {@link #addExpressionCallElement}; unset = omitted (library default). */
    public static final class AddExpressionCallElementOptions {
        final Args wire = new Args();

        /**
         * <code>exec_order</code> (int) BOM EXEC_ORDER scoped per EFCALL_ID. Absent or &lt;= 0 =
         * auto-allocate (max on the call + 1); &gt; 0 and free = verbatim; &gt; 0 and taken =
         * ALREADY_EXISTS.
         *
         * @param execOrder the value
         * @return this builder
         */
        public AddExpressionCallElementOptions execOrder(long execOrder) {
            wire.integer("exec_order", execOrder);
            return this;
        }
    }

    /**
     * Add one CFG_EFBOM row to an expression call, addressed by raw ids.
     *
     * <p>Notes:
     * Returns (modified config, the new CFG_EFBOM row {EFCALL_ID, FTYPE_ID, FELEM_ID,
     * EXEC_ORDER, FELEM_REQ}). Check order: ftype_id &lt; 0, ALREADY_PRESENT when (EFCALL_ID,
     * FTYPE_ID, FELEM_ID) already exists (EXEC_ORDER ignored), exec order, then
     * MISSING_SECTION if CFG_EFBOM is absent.
     *
     * <p>Wire name: {@code add_expression_call_element}; group: {@code calls_expression}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param efcallId <code>efcall_id</code> (int) Stored as EFCALL_ID. NOT validated — the call need not
     * exist.
     * @param ftypeId <code>ftype_id</code> (int) The ELEMENT's feature id, stored verbatim as BOM FTYPE_ID.
     * &lt; 0 = INVALID_INPUT; 0 is accepted and is the G2 parent feature link (same as
     * add_expression_call's feature "PARENT"); -1 (any feature) is not addable here; NOT
     * validated against CFG_FTYPE.
     * @param felemId <code>felem_id</code> (int) Stored verbatim as FELEM_ID. NOT validated against
     * CFG_FELEM.
     * @param felemReq <code>felem_req</code> (str) Stored verbatim in FELEM_REQ (not validated or normalized).
     * @return the modified configuration JSON document (opaque); {@link #addExpressionCallElementResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, INVALID_INPUT, ALREADY_PRESENT, ALREADY_EXISTS, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addExpressionCallElement(String configJson, long efcallId, long ftypeId, long felemId, String felemReq) throws SzConfigToolException {
        return addExpressionCallElement(configJson, efcallId, ftypeId, felemId, felemReq, null);
    }

    /**
     * Add one CFG_EFBOM row to an expression call, addressed by raw ids.
     *
     * <p>Notes:
     * Returns (modified config, the new CFG_EFBOM row {EFCALL_ID, FTYPE_ID, FELEM_ID,
     * EXEC_ORDER, FELEM_REQ}). Check order: ftype_id &lt; 0, ALREADY_PRESENT when (EFCALL_ID,
     * FTYPE_ID, FELEM_ID) already exists (EXEC_ORDER ignored), exec order, then
     * MISSING_SECTION if CFG_EFBOM is absent.
     *
     * <p>Wire name: {@code add_expression_call_element}; group: {@code calls_expression}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param efcallId <code>efcall_id</code> (int) Stored as EFCALL_ID. NOT validated — the call need not
     * exist.
     * @param ftypeId <code>ftype_id</code> (int) The ELEMENT's feature id, stored verbatim as BOM FTYPE_ID.
     * &lt; 0 = INVALID_INPUT; 0 is accepted and is the G2 parent feature link (same as
     * add_expression_call's feature "PARENT"); -1 (any feature) is not addable here; NOT
     * validated against CFG_FTYPE.
     * @param felemId <code>felem_id</code> (int) Stored verbatim as FELEM_ID. NOT validated against
     * CFG_FELEM.
     * @param felemReq <code>felem_req</code> (str) Stored verbatim in FELEM_REQ (not validated or normalized).
     * @param options optional arguments ({@code null} = none); see {@link AddExpressionCallElementOptions}
     * @return the modified configuration JSON document (opaque); {@link #addExpressionCallElementResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, INVALID_INPUT, ALREADY_PRESENT, ALREADY_EXISTS, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addExpressionCallElement(String configJson, long efcallId, long ftypeId, long felemId, String felemReq, AddExpressionCallElementOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.integer("efcall_id", efcallId);
        wire.integer("ftype_id", ftypeId);
        wire.integer("felem_id", felemId);
        wire.str("felem_req", felemReq);
        return Invoker.call("add_expression_call_element", "config_and_json", configJson, wire)[1];
    }

    /**
     * The record (row / ids) of <code>addExpressionCallElement</code>: same arguments and
     * operation, but returns the record instead of the configuration. Operation: Add one
     * CFG_EFBOM row to an expression call, addressed by raw ids.
     *
     * <p>Notes:
     * Returns (modified config, the new CFG_EFBOM row {EFCALL_ID, FTYPE_ID, FELEM_ID,
     * EXEC_ORDER, FELEM_REQ}). Check order: ftype_id &lt; 0, ALREADY_PRESENT when (EFCALL_ID,
     * FTYPE_ID, FELEM_ID) already exists (EXEC_ORDER ignored), exec order, then
     * MISSING_SECTION if CFG_EFBOM is absent.
     *
     * <p>Wire name: {@code add_expression_call_element}; group: {@code calls_expression}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param efcallId <code>efcall_id</code> (int) Stored as EFCALL_ID. NOT validated — the call need not
     * exist.
     * @param ftypeId <code>ftype_id</code> (int) The ELEMENT's feature id, stored verbatim as BOM FTYPE_ID.
     * &lt; 0 = INVALID_INPUT; 0 is accepted and is the G2 parent feature link (same as
     * add_expression_call's feature "PARENT"); -1 (any feature) is not addable here; NOT
     * validated against CFG_FTYPE.
     * @param felemId <code>felem_id</code> (int) Stored verbatim as FELEM_ID. NOT validated against
     * CFG_FELEM.
     * @param felemReq <code>felem_req</code> (str) Stored verbatim in FELEM_REQ (not validated or normalized).
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, INVALID_INPUT, ALREADY_PRESENT, ALREADY_EXISTS, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addExpressionCallElementResult(String configJson, long efcallId, long ftypeId, long felemId, String felemReq) throws SzConfigToolException {
        return addExpressionCallElementResult(configJson, efcallId, ftypeId, felemId, felemReq, null);
    }

    /**
     * The record (row / ids) of <code>addExpressionCallElement</code>: same arguments and
     * operation, but returns the record instead of the configuration. Operation: Add one
     * CFG_EFBOM row to an expression call, addressed by raw ids.
     *
     * <p>Notes:
     * Returns (modified config, the new CFG_EFBOM row {EFCALL_ID, FTYPE_ID, FELEM_ID,
     * EXEC_ORDER, FELEM_REQ}). Check order: ftype_id &lt; 0, ALREADY_PRESENT when (EFCALL_ID,
     * FTYPE_ID, FELEM_ID) already exists (EXEC_ORDER ignored), exec order, then
     * MISSING_SECTION if CFG_EFBOM is absent.
     *
     * <p>Wire name: {@code add_expression_call_element}; group: {@code calls_expression}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param efcallId <code>efcall_id</code> (int) Stored as EFCALL_ID. NOT validated — the call need not
     * exist.
     * @param ftypeId <code>ftype_id</code> (int) The ELEMENT's feature id, stored verbatim as BOM FTYPE_ID.
     * &lt; 0 = INVALID_INPUT; 0 is accepted and is the G2 parent feature link (same as
     * add_expression_call's feature "PARENT"); -1 (any feature) is not addable here; NOT
     * validated against CFG_FTYPE.
     * @param felemId <code>felem_id</code> (int) Stored verbatim as FELEM_ID. NOT validated against
     * CFG_FELEM.
     * @param felemReq <code>felem_req</code> (str) Stored verbatim in FELEM_REQ (not validated or normalized).
     * @param options optional arguments ({@code null} = none); see {@link AddExpressionCallElementOptions}
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, INVALID_INPUT, ALREADY_PRESENT, ALREADY_EXISTS, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addExpressionCallElementResult(String configJson, long efcallId, long ftypeId, long felemId, String felemReq, AddExpressionCallElementOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.integer("efcall_id", efcallId);
        wire.integer("ftype_id", ftypeId);
        wire.integer("felem_id", felemId);
        wire.str("felem_req", felemReq);
        return Invoker.result("add_expression_call_element", "config_and_json", configJson, wire);
    }

    /** Optional arguments of {@link #deleteExpressionCallElement}; unset = omitted (library default). */
    public static final class DeleteExpressionCallElementOptions {
        final Args wire = new Args();

        /**
         * <code>element_feature</code> (str) The element's feature code (case-insensitive; unknown
         * = NOT_FOUND). Narrows the BOM match to that FTYPE_ID. Without it, an element present
         * under more than one feature on the call is INVALID_INPUT (ambiguous) — e.g. template
         * EFCALL 97 carries TOKENIZED_NM under GROUP_ASSOCIATION and EMPLOYER.
         *
         * @param elementFeature the value
         * @return this builder
         */
        public DeleteExpressionCallElementOptions elementFeature(String elementFeature) {
            wire.str("element_feature", elementFeature);
            return this;
        }
    }

    /**
     * Delete one CFG_EFBOM row from an expression call, addressed by call + element code.
     *
     * <p>Notes:
     * The BOM EXEC_ORDER is derived from the located row; only that row is removed. Check
     * order: call selector, call existence, element_feature, element, BOM row.
     *
     * <p>Wire name: {@code delete_expression_call_element}; group: {@code calls_expression}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param call <code>call</code> (int_or_str) Call selector: an integer = EFCALL_ID; a string = feature
     * code (unknown feature = NOT_FOUND, no call = NOT_FOUND, more than one call =
     * INVALID_INPUT). A call id that does not exist = NOT_FOUND.
     * @param elementCode <code>element_code</code> (str) Element code (case-insensitive). Without
     * element_feature: global lookup, unknown = NOT_FOUND. With element_feature: unknown OR
     * not in that feature's CFG_FBOM = NOT_IN_FEATURE. Known but not on the call =
     * NOT_ON_CALL.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT, NOT_IN_FEATURE, NOT_ON_CALL; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteExpressionCallElement(String configJson, long call, String elementCode) throws SzConfigToolException {
        return deleteExpressionCallElement(configJson, call, elementCode, null);
    }

    /**
     * Delete one CFG_EFBOM row from an expression call, addressed by call + element code.
     *
     * <p>Notes:
     * The BOM EXEC_ORDER is derived from the located row; only that row is removed. Check
     * order: call selector, call existence, element_feature, element, BOM row.
     *
     * <p>Wire name: {@code delete_expression_call_element}; group: {@code calls_expression}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param call <code>call</code> (int_or_str) Call selector: an integer = EFCALL_ID; a string = feature
     * code (unknown feature = NOT_FOUND, no call = NOT_FOUND, more than one call =
     * INVALID_INPUT). A call id that does not exist = NOT_FOUND.
     * @param elementCode <code>element_code</code> (str) Element code (case-insensitive). Without
     * element_feature: global lookup, unknown = NOT_FOUND. With element_feature: unknown OR
     * not in that feature's CFG_FBOM = NOT_IN_FEATURE. Known but not on the call =
     * NOT_ON_CALL.
     * @param options optional arguments ({@code null} = none); see {@link DeleteExpressionCallElementOptions}
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT, NOT_IN_FEATURE, NOT_ON_CALL; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteExpressionCallElement(String configJson, long call, String elementCode, DeleteExpressionCallElementOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.integer("call", call);
        wire.str("element_code", elementCode);
        return Invoker.config("delete_expression_call_element", configJson, wire);
    }

    /**
     * Delete one CFG_EFBOM row from an expression call, addressed by call + element code.
     *
     * <p>Notes:
     * The BOM EXEC_ORDER is derived from the located row; only that row is removed. Check
     * order: call selector, call existence, element_feature, element, BOM row.
     *
     * <p>Wire name: {@code delete_expression_call_element}; group: {@code calls_expression}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param call <code>call</code> (int_or_str) Call selector: an integer = EFCALL_ID; a string = feature
     * code (unknown feature = NOT_FOUND, no call = NOT_FOUND, more than one call =
     * INVALID_INPUT). A call id that does not exist = NOT_FOUND.
     * @param elementCode <code>element_code</code> (str) Element code (case-insensitive). Without
     * element_feature: global lookup, unknown = NOT_FOUND. With element_feature: unknown OR
     * not in that feature's CFG_FBOM = NOT_IN_FEATURE. Known but not on the call =
     * NOT_ON_CALL.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT, NOT_IN_FEATURE, NOT_ON_CALL; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteExpressionCallElement(String configJson, String call, String elementCode) throws SzConfigToolException {
        return deleteExpressionCallElement(configJson, call, elementCode, null);
    }

    /**
     * Delete one CFG_EFBOM row from an expression call, addressed by call + element code.
     *
     * <p>Notes:
     * The BOM EXEC_ORDER is derived from the located row; only that row is removed. Check
     * order: call selector, call existence, element_feature, element, BOM row.
     *
     * <p>Wire name: {@code delete_expression_call_element}; group: {@code calls_expression}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param call <code>call</code> (int_or_str) Call selector: an integer = EFCALL_ID; a string = feature
     * code (unknown feature = NOT_FOUND, no call = NOT_FOUND, more than one call =
     * INVALID_INPUT). A call id that does not exist = NOT_FOUND.
     * @param elementCode <code>element_code</code> (str) Element code (case-insensitive). Without
     * element_feature: global lookup, unknown = NOT_FOUND. With element_feature: unknown OR
     * not in that feature's CFG_FBOM = NOT_IN_FEATURE. Known but not on the call =
     * NOT_ON_CALL.
     * @param options optional arguments ({@code null} = none); see {@link DeleteExpressionCallElementOptions}
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT, NOT_IN_FEATURE, NOT_ON_CALL; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteExpressionCallElement(String configJson, String call, String elementCode, DeleteExpressionCallElementOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("call", call);
        wire.str("element_code", elementCode);
        return Invoker.config("delete_expression_call_element", configJson, wire);
    }

    /** Optional arguments of {@link #addStandardizeCall}; unset = omitted (library default). */
    public static final class AddStandardizeCallOptions {
        final Args wire = new Args();

        /**
         * <code>ftype_code</code> (str) Feature code (case-insensitive) or NOT_FOUND. "ALL"
         * (case-insensitive) is treated as absent. Stored as FTYPE_ID; absent stores FTYPE_ID -1.
         *
         * @param ftypeCode the value
         * @return this builder
         */
        public AddStandardizeCallOptions ftypeCode(String ftypeCode) {
            wire.str("ftype_code", ftypeCode);
            return this;
        }

        /**
         * <code>felem_code</code> (str) Element code (case-insensitive) or NOT_FOUND. "N/A"
         * (case-insensitive) is treated as absent. Stored as FELEM_ID; absent stores FELEM_ID -1.
         * Exactly one of ftype_code / felem_code must resolve, else INVALID_INPUT.
         *
         * @param felemCode the value
         * @return this builder
         */
        public AddStandardizeCallOptions felemCode(String felemCode) {
            wire.str("felem_code", felemCode);
            return this;
        }

        /**
         * <code>exec_order</code> (int) EXEC_ORDER scoped per (FTYPE_ID, FELEM_ID) of the new row
         * (the -1 sentinel is part of the scope). Absent or &lt;= 0 = auto-allocate (max in scope
         * + 1, 1 for an empty scope); &gt; 0 and free = used verbatim; &gt; 0 and taken =
         * ALREADY_EXISTS.
         *
         * @param execOrder the value
         * @return this builder
         */
        public AddStandardizeCallOptions execOrder(long execOrder) {
            wire.integer("exec_order", execOrder);
            return this;
        }
    }

    /**
     * Add a standardize call (CFG_SFCALL row) binding a standardize function to a feature or
     * an element.
     *
     * <p>Notes:
     * Returns (modified config, the new CFG_SFCALL row {SFCALL_ID, FTYPE_ID, FELEM_ID,
     * SFUNC_ID, EXEC_ORDER}). SFCALL_ID is always auto-allocated (max + 1, floor 1000).
     * MISSING_SECTION when CFG_SFCALL is absent. Check order: SFCALL_ID allocation, sfunc,
     * feature, element, exactly-one rule, exec order. TRAP: the exec-order scope does not
     * include SFUNC_ID, so a second call on the same feature continues that feature's order
     * sequence.
     *
     * <p>Wire name: {@code add_standardize_call}; group: {@code calls_standardize}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param sfuncCode <code>sfunc_code</code> (str) Standardize function code (CFG_SFUNC, case-insensitive) or
     * NOT_FOUND. Looked up before the feature/element.
     * @param options optional arguments ({@code null} = none); see {@link AddStandardizeCallOptions}
     * @return the modified configuration JSON document (opaque); {@link #addStandardizeCallResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND, INVALID_INPUT, ALREADY_EXISTS; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addStandardizeCall(String configJson, String sfuncCode, AddStandardizeCallOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("sfunc_code", sfuncCode);
        return Invoker.call("add_standardize_call", "config_and_json", configJson, wire)[1];
    }

    /**
     * The record (row / ids) of <code>addStandardizeCall</code>: same arguments and operation,
     * but returns the record instead of the configuration. Operation: Add a standardize call
     * (CFG_SFCALL row) binding a standardize function to a feature or an element.
     *
     * <p>Notes:
     * Returns (modified config, the new CFG_SFCALL row {SFCALL_ID, FTYPE_ID, FELEM_ID,
     * SFUNC_ID, EXEC_ORDER}). SFCALL_ID is always auto-allocated (max + 1, floor 1000).
     * MISSING_SECTION when CFG_SFCALL is absent. Check order: SFCALL_ID allocation, sfunc,
     * feature, element, exactly-one rule, exec order. TRAP: the exec-order scope does not
     * include SFUNC_ID, so a second call on the same feature continues that feature's order
     * sequence.
     *
     * <p>Wire name: {@code add_standardize_call}; group: {@code calls_standardize}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param sfuncCode <code>sfunc_code</code> (str) Standardize function code (CFG_SFUNC, case-insensitive) or
     * NOT_FOUND. Looked up before the feature/element.
     * @param options optional arguments ({@code null} = none); see {@link AddStandardizeCallOptions}
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND, INVALID_INPUT, ALREADY_EXISTS; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addStandardizeCallResult(String configJson, String sfuncCode, AddStandardizeCallOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("sfunc_code", sfuncCode);
        return Invoker.result("add_standardize_call", "config_and_json", configJson, wire);
    }

    /**
     * Delete a standardize call by SFCALL_ID.
     *
     * <p>Notes:
     * Removes every CFG_SFCALL row with that id; no dependency checks (there is no standardize
     * BOM).
     *
     * <p>Wire name: {@code delete_standardize_call}; group: {@code calls_standardize}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param sfcallId <code>sfcall_id</code> (int) Must match an existing SFCALL_ID, else NOT_FOUND (also
     * NOT_FOUND when CFG_SFCALL is absent).
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteStandardizeCall(String configJson, long sfcallId) throws SzConfigToolException {
        Args wire = new Args();
        wire.integer("sfcall_id", sfcallId);
        return Invoker.config("delete_standardize_call", configJson, wire);
    }

    /**
     * Get one standardize call's raw CFG_SFCALL row, by SFCALL_ID or by feature code.
     *
     * <p>Notes:
     * Result uses on-disk keys (SFCALL_ID, FTYPE_ID, FELEM_ID, SFUNC_ID, EXEC_ORDER).
     * Element-bound calls (FTYPE_ID -1) are never found by feature. In the template NAME has
     * two standardize calls (PARSE_NAME, TOKENIZE_NAME), so by-feature NAME is ambiguous.
     *
     * <p>Wire name: {@code get_standardize_call}; group: {@code calls_standardize}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param call <code>call</code> (int_or_str) Call selector: an integer = SFCALL_ID (NOT_FOUND if
     * absent); a string = feature code (case-insensitive; unknown feature = NOT_FOUND, no call
     * on the feature = NOT_FOUND, more than one call on the feature = INVALID_INPUT —
     * address such calls by id). Any other JSON type = INVALID_INPUT.
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String getStandardizeCall(String configJson, long call) throws SzConfigToolException {
        Args wire = new Args();
        wire.integer("call", call);
        return Invoker.json("get_standardize_call", configJson, wire);
    }

    /**
     * Get one standardize call's raw CFG_SFCALL row, by SFCALL_ID or by feature code.
     *
     * <p>Notes:
     * Result uses on-disk keys (SFCALL_ID, FTYPE_ID, FELEM_ID, SFUNC_ID, EXEC_ORDER).
     * Element-bound calls (FTYPE_ID -1) are never found by feature. In the template NAME has
     * two standardize calls (PARSE_NAME, TOKENIZE_NAME), so by-feature NAME is ambiguous.
     *
     * <p>Wire name: {@code get_standardize_call}; group: {@code calls_standardize}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param call <code>call</code> (int_or_str) Call selector: an integer = SFCALL_ID (NOT_FOUND if
     * absent); a string = feature code (case-insensitive; unknown feature = NOT_FOUND, no call
     * on the feature = NOT_FOUND, more than one call on the feature = INVALID_INPUT —
     * address such calls by id). Any other JSON type = INVALID_INPUT.
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String getStandardizeCall(String configJson, String call) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("call", call);
        return Invoker.json("get_standardize_call", configJson, wire);
    }

    /**
     * List all standardize calls with resolved codes.
     *
     * <p>Notes:
     * Array of {id, feature, element, execOrder, function}, stably sorted by (FTYPE_ID,
     * EXEC_ORDER). feature is "all" when FTYPE_ID &lt;= 0 or unresolved; element is "n/a" when
     * FELEM_ID &lt;= 0 or unresolved; function is "unknown" when unresolved. No elementList
     * key. Missing sections yield [] (no MISSING_SECTION).
     *
     * <p>Wire name: {@code list_standardize_calls}; group: {@code calls_standardize}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String listStandardizeCalls(String configJson) throws SzConfigToolException {
        Args wire = new Args();
        return Invoker.json("list_standardize_calls", configJson, wire);
    }

    /** Optional arguments of {@link #addStandardizeCallElement}; unset = omitted (library default). */
    public static final class AddStandardizeCallElementOptions {
        final Args wire = new Args();

        /**
         * <code>felem_id</code> (int) Stored as FELEM_ID; absent = -1. NOT validated against
         * CFG_FELEM.
         *
         * @param felemId the value
         * @return this builder
         */
        public AddStandardizeCallElementOptions felemId(long felemId) {
            wire.integer("felem_id", felemId);
            return this;
        }

        /**
         * <code>exec_order</code> (int) EXEC_ORDER scoped per (FTYPE_ID, FELEM_ID). Absent or
         * &lt;= 0 = auto-allocate (max in scope + 1); &gt; 0 and free = verbatim; &gt; 0 and taken
         * = ALREADY_EXISTS.
         *
         * @param execOrder the value
         * @return this builder
         */
        public AddStandardizeCallElementOptions execOrder(long execOrder) {
            wire.integer("exec_order", execOrder);
            return this;
        }
    }

    /**
     * Add a CFG_SFCALL row addressed by raw ids (FTYPE_ID, SFUNC_ID, FELEM_ID).
     *
     * <p>Notes:
     * Returns (modified config, the new CFG_SFCALL row). ALREADY_PRESENT when a row with the
     * same (FTYPE_ID, SFUNC_ID, FELEM_ID) exists (checked first). SFCALL_ID auto-allocated
     * (max + 1, floor 1000); MISSING_SECTION when CFG_SFCALL is absent. Unlike
     * add_standardize_call there is no feature-xor-element rule and no id validation.
     *
     * <p>Wire name: {@code add_standardize_call_element}; group: {@code calls_standardize}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param ftypeId <code>ftype_id</code> (int) Stored verbatim as FTYPE_ID. NOT validated against CFG_FTYPE
     * (use -1 for an element-bound row).
     * @param sfuncId <code>sfunc_id</code> (int) Stored verbatim as SFUNC_ID. NOT validated against
     * CFG_SFUNC.
     * @return the modified configuration JSON document (opaque); {@link #addStandardizeCallElementResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, ALREADY_PRESENT, ALREADY_EXISTS; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addStandardizeCallElement(String configJson, long ftypeId, long sfuncId) throws SzConfigToolException {
        return addStandardizeCallElement(configJson, ftypeId, sfuncId, null);
    }

    /**
     * Add a CFG_SFCALL row addressed by raw ids (FTYPE_ID, SFUNC_ID, FELEM_ID).
     *
     * <p>Notes:
     * Returns (modified config, the new CFG_SFCALL row). ALREADY_PRESENT when a row with the
     * same (FTYPE_ID, SFUNC_ID, FELEM_ID) exists (checked first). SFCALL_ID auto-allocated
     * (max + 1, floor 1000); MISSING_SECTION when CFG_SFCALL is absent. Unlike
     * add_standardize_call there is no feature-xor-element rule and no id validation.
     *
     * <p>Wire name: {@code add_standardize_call_element}; group: {@code calls_standardize}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param ftypeId <code>ftype_id</code> (int) Stored verbatim as FTYPE_ID. NOT validated against CFG_FTYPE
     * (use -1 for an element-bound row).
     * @param sfuncId <code>sfunc_id</code> (int) Stored verbatim as SFUNC_ID. NOT validated against
     * CFG_SFUNC.
     * @param options optional arguments ({@code null} = none); see {@link AddStandardizeCallElementOptions}
     * @return the modified configuration JSON document (opaque); {@link #addStandardizeCallElementResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, ALREADY_PRESENT, ALREADY_EXISTS; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addStandardizeCallElement(String configJson, long ftypeId, long sfuncId, AddStandardizeCallElementOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.integer("ftype_id", ftypeId);
        wire.integer("sfunc_id", sfuncId);
        return Invoker.call("add_standardize_call_element", "config_and_json", configJson, wire)[1];
    }

    /**
     * The record (row / ids) of <code>addStandardizeCallElement</code>: same arguments and
     * operation, but returns the record instead of the configuration. Operation: Add a
     * CFG_SFCALL row addressed by raw ids (FTYPE_ID, SFUNC_ID, FELEM_ID).
     *
     * <p>Notes:
     * Returns (modified config, the new CFG_SFCALL row). ALREADY_PRESENT when a row with the
     * same (FTYPE_ID, SFUNC_ID, FELEM_ID) exists (checked first). SFCALL_ID auto-allocated
     * (max + 1, floor 1000); MISSING_SECTION when CFG_SFCALL is absent. Unlike
     * add_standardize_call there is no feature-xor-element rule and no id validation.
     *
     * <p>Wire name: {@code add_standardize_call_element}; group: {@code calls_standardize}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param ftypeId <code>ftype_id</code> (int) Stored verbatim as FTYPE_ID. NOT validated against CFG_FTYPE
     * (use -1 for an element-bound row).
     * @param sfuncId <code>sfunc_id</code> (int) Stored verbatim as SFUNC_ID. NOT validated against
     * CFG_SFUNC.
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, ALREADY_PRESENT, ALREADY_EXISTS; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addStandardizeCallElementResult(String configJson, long ftypeId, long sfuncId) throws SzConfigToolException {
        return addStandardizeCallElementResult(configJson, ftypeId, sfuncId, null);
    }

    /**
     * The record (row / ids) of <code>addStandardizeCallElement</code>: same arguments and
     * operation, but returns the record instead of the configuration. Operation: Add a
     * CFG_SFCALL row addressed by raw ids (FTYPE_ID, SFUNC_ID, FELEM_ID).
     *
     * <p>Notes:
     * Returns (modified config, the new CFG_SFCALL row). ALREADY_PRESENT when a row with the
     * same (FTYPE_ID, SFUNC_ID, FELEM_ID) exists (checked first). SFCALL_ID auto-allocated
     * (max + 1, floor 1000); MISSING_SECTION when CFG_SFCALL is absent. Unlike
     * add_standardize_call there is no feature-xor-element rule and no id validation.
     *
     * <p>Wire name: {@code add_standardize_call_element}; group: {@code calls_standardize}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param ftypeId <code>ftype_id</code> (int) Stored verbatim as FTYPE_ID. NOT validated against CFG_FTYPE
     * (use -1 for an element-bound row).
     * @param sfuncId <code>sfunc_id</code> (int) Stored verbatim as SFUNC_ID. NOT validated against
     * CFG_SFUNC.
     * @param options optional arguments ({@code null} = none); see {@link AddStandardizeCallElementOptions}
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, ALREADY_PRESENT, ALREADY_EXISTS; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addStandardizeCallElementResult(String configJson, long ftypeId, long sfuncId, AddStandardizeCallElementOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.integer("ftype_id", ftypeId);
        wire.integer("sfunc_id", sfuncId);
        return Invoker.result("add_standardize_call_element", "config_and_json", configJson, wire);
    }

    /** Optional arguments of {@link #deleteStandardizeCallElement}; unset = omitted (library default). */
    public static final class DeleteStandardizeCallElementOptions {
        final Args wire = new Args();

        /**
         * <code>felem_id</code> (int) Matched against FELEM_ID; absent = -1.
         *
         * @param felemId the value
         * @return this builder
         */
        public DeleteStandardizeCallElementOptions felemId(long felemId) {
            wire.integer("felem_id", felemId);
            return this;
        }
    }

    /**
     * Delete CFG_SFCALL rows matching raw ids (FTYPE_ID, SFUNC_ID, FELEM_ID).
     *
     * <p>Notes:
     * NOT_FOUND (not NOT_ON_CALL) when no row matches. Removes EVERY matching row.
     *
     * <p>Wire name: {@code delete_standardize_call_element}; group: {@code calls_standardize}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param ftypeId <code>ftype_id</code> (int) Matched against FTYPE_ID.
     * @param sfuncId <code>sfunc_id</code> (int) Matched against SFUNC_ID.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteStandardizeCallElement(String configJson, long ftypeId, long sfuncId) throws SzConfigToolException {
        return deleteStandardizeCallElement(configJson, ftypeId, sfuncId, null);
    }

    /**
     * Delete CFG_SFCALL rows matching raw ids (FTYPE_ID, SFUNC_ID, FELEM_ID).
     *
     * <p>Notes:
     * NOT_FOUND (not NOT_ON_CALL) when no row matches. Removes EVERY matching row.
     *
     * <p>Wire name: {@code delete_standardize_call_element}; group: {@code calls_standardize}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param ftypeId <code>ftype_id</code> (int) Matched against FTYPE_ID.
     * @param sfuncId <code>sfunc_id</code> (int) Matched against SFUNC_ID.
     * @param options optional arguments ({@code null} = none); see {@link DeleteStandardizeCallElementOptions}
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteStandardizeCallElement(String configJson, long ftypeId, long sfuncId, DeleteStandardizeCallElementOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.integer("ftype_id", ftypeId);
        wire.integer("sfunc_id", sfuncId);
        return Invoker.config("delete_standardize_call_element", configJson, wire);
    }

    /**
     * Add a new, empty top-level section (an empty array) to G2_CONFIG.
     *
     * <p>Notes:
     * An existing key of the same (uppercased) name, of ANY JSON type, is ALREADY_EXISTS. A
     * config without a G2_CONFIG key is NOT_FOUND (not MISSING_SECTION). No validation of the
     * name (any string, e.g. not CFG_*, is accepted).
     *
     * <p>Wire name: {@code add_config_section}; group: {@code config_sections}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param sectionName <code>section_name</code> (str) Uppercased before the duplicate check and storage. The
     * new section is always an empty JSON array.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, ALREADY_EXISTS, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addConfigSection(String configJson, String sectionName) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("section_name", sectionName);
        return Invoker.config("add_config_section", configJson, wire);
    }

    /**
     * Remove a top-level section from G2_CONFIG.
     *
     * <p>Notes:
     * TRAP: no protection or dependency check; any section (including core sections such as
     * CFG_DSRC, SETTINGS) can be removed. A missing section or a missing G2_CONFIG is
     * NOT_FOUND.
     *
     * <p>Wire name: {@code remove_config_section}; group: {@code config_sections}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param sectionName <code>section_name</code> (str) Uppercased before lookup.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String removeConfigSection(String configJson, String sectionName) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("section_name", sectionName);
        return Invoker.config("remove_config_section", configJson, wire);
    }

    /** Optional arguments of {@link #getConfigSection}; unset = omitted (library default). */
    public static final class GetConfigSectionOptions {
        final Args wire = new Args();

        /**
         * <code>filter</code> (str) Absent returns every item. Otherwise keeps items whose
         * json.dumps-spaced rendering (<code>{"K": 1, "J": null}</code>,
         * crate::filter::to_json_dumps_string) contains the filter, case-insensitively.
         *
         * @param filter the value
         * @return this builder
         */
        public GetConfigSectionOptions filter(String filter) {
            wire.str("filter", filter);
            return this;
        }
    }

    /**
     * Get the raw items of a top-level section, optionally filtered by a case-insensitive
     * substring.
     *
     * <p>Notes:
     * Result is an array. An array section returns its rows; a null or empty section returns
     * []; a non-array, non-null section (e.g. SETTINGS, CONFIG_BASE_VERSION objects) returns a
     * one-element array holding the value. An empty result does not distinguish "section
     * empty" from "filter matched nothing": use config_section_is_empty.
     *
     * <p>Wire name: {@code get_config_section}; group: {@code config_sections}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param sectionName <code>section_name</code> (str) TRAP: matched EXACTLY (case-sensitive, NOT uppercased,
     * unlike add/remove_config_section). Unknown name is NOT_FOUND.
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String getConfigSection(String configJson, String sectionName) throws SzConfigToolException {
        return getConfigSection(configJson, sectionName, null);
    }

    /**
     * Get the raw items of a top-level section, optionally filtered by a case-insensitive
     * substring.
     *
     * <p>Notes:
     * Result is an array. An array section returns its rows; a null or empty section returns
     * []; a non-array, non-null section (e.g. SETTINGS, CONFIG_BASE_VERSION objects) returns a
     * one-element array holding the value. An empty result does not distinguish "section
     * empty" from "filter matched nothing": use config_section_is_empty.
     *
     * <p>Wire name: {@code get_config_section}; group: {@code config_sections}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param sectionName <code>section_name</code> (str) TRAP: matched EXACTLY (case-sensitive, NOT uppercased,
     * unlike add/remove_config_section). Unknown name is NOT_FOUND.
     * @param options optional arguments ({@code null} = none); see {@link GetConfigSectionOptions}
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String getConfigSection(String configJson, String sectionName, GetConfigSectionOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("section_name", sectionName);
        return Invoker.json("get_config_section", configJson, wire);
    }

    /**
     * Report whether a top-level section is empty (null or []).
     *
     * <p>Notes:
     * Result is a JSON boolean: true for null or [], false otherwise (any non-array, non-null
     * value such as an object counts as non-empty).
     *
     * <p>Wire name: {@code config_section_is_empty}; group: {@code config_sections}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param sectionName <code>section_name</code> (str) Matched EXACTLY (case-sensitive, not uppercased).
     * Unknown name is NOT_FOUND.
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String configSectionIsEmpty(String configJson, String sectionName) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("section_name", sectionName);
        return Invoker.json("config_section_is_empty", configJson, wire);
    }

    /**
     * List the names of all top-level G2_CONFIG keys.
     *
     * <p>Notes:
     * Result is an array of key names in config order, including non-CFG keys (SETTINGS,
     * SYS_OOM, CONFIG_BASE_VERSION). A missing or non-object G2_CONFIG yields [] rather than
     * an error.
     *
     * <p>Wire name: {@code list_config_sections}; group: {@code config_sections}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String listConfigSections(String configJson) throws SzConfigToolException {
        Args wire = new Args();
        return Invoker.json("list_config_sections", configJson, wire);
    }

    /**
     * Remove a field from every item of an array section, returning how many items had it.
     *
     * <p>Notes:
     * Record is the integer count of items the field was removed from (0 when no item had it;
     * the config is still returned). Non-object items are skipped. A config with no G2_CONFIG
     * key succeeds unchanged with count 0.
     *
     * <p>Wire name: {@code remove_config_section_field}; group: {@code config_sections}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param sectionName <code>section_name</code> (str) Uppercased before lookup. Must name an ARRAY section,
     * else NOT_FOUND.
     * @param fieldName <code>field_name</code> (str) Uppercased before removal (a lowercase key in a row can
     * never be removed).
     * @return the modified configuration JSON document (opaque); {@link #removeConfigSectionFieldResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String removeConfigSectionField(String configJson, String sectionName, String fieldName) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("section_name", sectionName);
        wire.str("field_name", fieldName);
        return Invoker.call("remove_config_section_field", "config_and_json", configJson, wire)[1];
    }

    /**
     * The record (row / ids) of <code>removeConfigSectionField</code>: same arguments and
     * operation, but returns the record instead of the configuration. Operation: Remove a
     * field from every item of an array section, returning how many items had it.
     *
     * <p>Notes:
     * Record is the integer count of items the field was removed from (0 when no item had it;
     * the config is still returned). Non-object items are skipped. A config with no G2_CONFIG
     * key succeeds unchanged with count 0.
     *
     * <p>Wire name: {@code remove_config_section_field}; group: {@code config_sections}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param sectionName <code>section_name</code> (str) Uppercased before lookup. Must name an ARRAY section,
     * else NOT_FOUND.
     * @param fieldName <code>field_name</code> (str) Uppercased before removal (a lowercase key in a row can
     * never be removed).
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String removeConfigSectionFieldResult(String configJson, String sectionName, String fieldName) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("section_name", sectionName);
        wire.str("field_name", fieldName);
        return Invoker.result("remove_config_section_field", "config_and_json", configJson, wire);
    }

    /**
     * Add a field to every item of an array section that lacks it, returning existed/updated
     * counts.
     *
     * <p>Notes:
     * Record is {"existed": n, "updated": n}: items that already had the field (value
     * preserved, never overwritten) vs. items it was inserted into. Non-object items are
     * skipped (counted in neither). A config with no G2_CONFIG key succeeds unchanged with
     * both counts 0.
     *
     * <p>Wire name: {@code add_config_section_field}; group: {@code config_sections}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param sectionName <code>section_name</code> (str) Uppercased before lookup. Must name an ARRAY section,
     * else NOT_FOUND.
     * @param fieldName <code>field_name</code> (str) Uppercased before insertion.
     * @param fieldValue <code>field_value</code> (json) Any JSON value, stored verbatim (cloned) into each item
     * that lacks the field.
     * @return the modified configuration JSON document (opaque); {@link #addConfigSectionFieldResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addConfigSectionField(String configJson, String sectionName, String fieldName, String fieldValue) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("section_name", sectionName);
        wire.str("field_name", fieldName);
        wire.json("field_value", fieldValue);
        return Invoker.call("add_config_section_field", "config_and_json", configJson, wire)[1];
    }

    /**
     * The record (row / ids) of <code>addConfigSectionField</code>: same arguments and
     * operation, but returns the record instead of the configuration. Operation: Add a field
     * to every item of an array section that lacks it, returning existed/updated counts.
     *
     * <p>Notes:
     * Record is {"existed": n, "updated": n}: items that already had the field (value
     * preserved, never overwritten) vs. items it was inserted into. Non-object items are
     * skipped (counted in neither). A config with no G2_CONFIG key succeeds unchanged with
     * both counts 0.
     *
     * <p>Wire name: {@code add_config_section_field}; group: {@code config_sections}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param sectionName <code>section_name</code> (str) Uppercased before lookup. Must name an ARRAY section,
     * else NOT_FOUND.
     * @param fieldName <code>field_name</code> (str) Uppercased before insertion.
     * @param fieldValue <code>field_value</code> (json) Any JSON value, stored verbatim (cloned) into each item
     * that lacks the field.
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addConfigSectionFieldResult(String configJson, String sectionName, String fieldName, String fieldValue) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("section_name", sectionName);
        wire.str("field_name", fieldName);
        wire.json("field_value", fieldValue);
        return Invoker.result("add_config_section_field", "config_and_json", configJson, wire);
    }

    /** Optional arguments of {@link #addDataSource}; unset = omitted (library default). */
    public static final class AddDataSourceOptions {
        final Args wire = new Args();

        /**
         * <code>retention_level</code> (str) Case-insensitive; normalized to Remember or Forget.
         * Any other value is INVALID_INPUT. Library default when omitted: "Remember".
         *
         * @param retentionLevel the value
         * @return this builder
         */
        public AddDataSourceOptions retentionLevel(String retentionLevel) {
            wire.str("retention_level", retentionLevel);
            return this;
        }

        /**
         * <code>id</code> (int) Requested DSRC_ID. Absent OR &lt;= 0 means auto-allocate (next
         * free id, floor 1000). A taken id &gt; 0 is ALREADY_EXISTS.
         *
         * @param id the value
         * @return this builder
         */
        public AddDataSourceOptions id(long id) {
            wire.integer("id", id);
            return this;
        }
    }

    /**
     * Add a data source (CFG_DSRC row) to the configuration.
     *
     * <p>Wire name: {@code add_data_source}; group: {@code datasources}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before storage and duplicate check; DSRC_DESC is set
     * to the same uppercased code.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, ALREADY_EXISTS, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addDataSource(String configJson, String code) throws SzConfigToolException {
        return addDataSource(configJson, code, null);
    }

    /**
     * Add a data source (CFG_DSRC row) to the configuration.
     *
     * <p>Wire name: {@code add_data_source}; group: {@code datasources}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before storage and duplicate check; DSRC_DESC is set
     * to the same uppercased code.
     * @param options optional arguments ({@code null} = none); see {@link AddDataSourceOptions}
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, ALREADY_EXISTS, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addDataSource(String configJson, String code, AddDataSourceOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("code", code);
        return Invoker.config("add_data_source", configJson, wire);
    }

    /**
     * Delete a data source by code.
     *
     * <p>Notes:
     * System data sources (DSRC_ID &lt;= 2, e.g. TEST and SEARCH in the template) are
     * protected and fail with INVALID_INPUT. No dependency check is made against other
     * sections.
     *
     * <p>Wire name: {@code delete_data_source}; group: {@code datasources}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteDataSource(String configJson, String code) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("code", code);
        return Invoker.config("delete_data_source", configJson, wire);
    }

    /**
     * Get one data source's raw CFG_DSRC row by code.
     *
     * <p>Notes:
     * Result is the stored row with on-disk keys (DSRC_ID, DSRC_CODE, DSRC_DESC,
     * RETENTION_LEVEL).
     *
     * <p>Wire name: {@code get_data_source}; group: {@code datasources}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String getDataSource(String configJson, String code) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("code", code);
        return Invoker.json("get_data_source", configJson, wire);
    }

    /**
     * List all data sources as {id, dataSource} summaries.
     *
     * <p>Notes:
     * Result is an array of {"id": DSRC_ID, "dataSource": DSRC_CODE} in config order.
     *
     * <p>Wire name: {@code list_data_sources}; group: {@code datasources}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String listDataSources(String configJson) throws SzConfigToolException {
        Args wire = new Args();
        return Invoker.json("list_data_sources", configJson, wire);
    }

    /** Optional arguments of {@link #setDataSource}; unset = omitted (library default). */
    public static final class SetDataSourceOptions {
        final Args wire = new Args();

        /**
         * <code>retention_level</code> (str) Absent leaves RETENTION_LEVEL unchanged. TRAP: unlike
         * add_data_source the value is written VERBATIM, with no domain validation or case
         * normalization.
         *
         * @param retentionLevel the value
         * @return this builder
         */
        public SetDataSourceOptions retentionLevel(String retentionLevel) {
            wire.str("retention_level", retentionLevel);
            return this;
        }
    }

    /**
     * Update a data source's retention level.
     *
     * <p>Wire name: {@code set_data_source}; group: {@code datasources}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String setDataSource(String configJson, String code) throws SzConfigToolException {
        return setDataSource(configJson, code, null);
    }

    /**
     * Update a data source's retention level.
     *
     * <p>Wire name: {@code set_data_source}; group: {@code datasources}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @param options optional arguments ({@code null} = none); see {@link SetDataSourceOptions}
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String setDataSource(String configJson, String code, SetDataSourceOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("code", code);
        return Invoker.config("set_data_source", configJson, wire);
    }

    /** Optional arguments of {@link #addElement}; unset = omitted (library default). */
    public static final class AddElementOptions {
        final Args wire = new Args();

        /**
         * <code>description</code> (str) FELEM_DESC, stored verbatim; absent defaults to the
         * uppercased code.
         *
         * @param description the value
         * @return this builder
         */
        public AddElementOptions description(String description) {
            wire.str("description", description);
            return this;
        }

        /**
         * <code>data_type</code> (str) Case-insensitive; normalized to lowercase string, number,
         * date, datetime or json, else INVALID_INPUT. Library default when omitted: "string".
         *
         * @param dataType the value
         * @return this builder
         */
        public AddElementOptions dataType(String dataType) {
            wire.str("data_type", dataType);
            return this;
        }

        /**
         * <code>id</code> (int) Requested FELEM_ID. Absent OR &lt;= 0 means auto-allocate (max
         * existing + 1, floor 1000). A taken id &gt; 0 is ALREADY_EXISTS.
         *
         * @param id the value
         * @return this builder
         */
        public AddElementOptions id(long id) {
            wire.integer("id", id);
            return this;
        }
    }

    /**
     * Add an element (CFG_FELEM row).
     *
     * <p>Notes:
     * Validation order: duplicate code, id, data_type.
     *
     * <p>Wire name: {@code add_element}; group: {@code elements}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased; duplicate (exact match on the uppercased code) is
     * ALREADY_EXISTS.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, ALREADY_EXISTS, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addElement(String configJson, String code) throws SzConfigToolException {
        return addElement(configJson, code, null);
    }

    /**
     * Add an element (CFG_FELEM row).
     *
     * <p>Notes:
     * Validation order: duplicate code, id, data_type.
     *
     * <p>Wire name: {@code add_element}; group: {@code elements}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased; duplicate (exact match on the uppercased code) is
     * ALREADY_EXISTS.
     * @param options optional arguments ({@code null} = none); see {@link AddElementOptions}
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, ALREADY_EXISTS, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addElement(String configJson, String code, AddElementOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("code", code);
        return Invoker.config("add_element", configJson, wire);
    }

    /**
     * Delete an element that no feature uses.
     *
     * <p>Notes:
     * INVALID_INPUT when any CFG_FBOM row maps the element to an existing feature ("Element
     * linked to the following feature(s): ..."). TRAP: CFG_ATTR rows naming the element are
     * NOT checked and are left dangling. MISSING_SECTION for an absent CFG_FELEM or CFG_FBOM;
     * MISSING_FIELD when the matched row's FELEM_ID is not an integer.
     *
     * <p>Wire name: {@code delete_element}; group: {@code elements}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup; unknown is NOT_FOUND.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND, INVALID_INPUT, MISSING_FIELD; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteElement(String configJson, String code) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("code", code);
        return Invoker.config("delete_element", configJson, wire);
    }

    /**
     * Get one element as a display summary by code.
     *
     * <p>Notes:
     * Result is exactly {id, element, datatype}; FELEM_DESC is NOT included.
     *
     * <p>Wire name: {@code get_element}; group: {@code elements}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup; unknown is NOT_FOUND.
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String getElement(String configJson, String code) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("code", code);
        return Invoker.json("get_element", configJson, wire);
    }

    /**
     * List all elements as display summaries, sorted by element code.
     *
     * <p>Notes:
     * Array of {id, element, datatype}, sorted alphabetically by element code (not by id).
     *
     * <p>Wire name: {@code list_elements}; group: {@code elements}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String listElements(String configJson) throws SzConfigToolException {
        Args wire = new Args();
        return Invoker.json("list_elements", configJson, wire);
    }

    /** Optional arguments of {@link #setElement}; unset = omitted (library default). */
    public static final class SetElementOptions {
        final Args wire = new Args();

        /**
         * <code>description</code> (str) Absent leaves FELEM_DESC; else stored verbatim.
         *
         * @param description the value
         * @return this builder
         */
        public SetElementOptions description(String description) {
            wire.str("description", description);
            return this;
        }

        /**
         * <code>data_type</code> (str) TRAP — absent leaves DATA_TYPE; else stored VERBATIM with
         * no validation or normalization (unlike add_element).
         *
         * @param dataType the value
         * @return this builder
         */
        public SetElementOptions dataType(String dataType) {
            wire.str("data_type", dataType);
            return this;
        }
    }

    /**
     * Update an element's description and/or data type.
     *
     * <p>Notes:
     * Succeeds unchanged when no update args are given.
     *
     * <p>Wire name: {@code set_element}; group: {@code elements}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup; unknown is NOT_FOUND.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String setElement(String configJson, String code) throws SzConfigToolException {
        return setElement(configJson, code, null);
    }

    /**
     * Update an element's description and/or data type.
     *
     * <p>Notes:
     * Succeeds unchanged when no update args are given.
     *
     * <p>Wire name: {@code set_element}; group: {@code elements}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup; unknown is NOT_FOUND.
     * @param options optional arguments ({@code null} = none); see {@link SetElementOptions}
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String setElement(String configJson, String code, SetElementOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("code", code);
        return Invoker.config("set_element", configJson, wire);
    }

    /** Optional arguments of {@link #setFeatureElement}; unset = omitted (library default). */
    public static final class SetFeatureElementOptions {
        final Args wire = new Args();

        /**
         * <code>exec_order</code> (int) TRAP — stored verbatim; no uniqueness check and no &lt;=
         * 0 auto-allocation.
         *
         * @param execOrder the value
         * @return this builder
         */
        public SetFeatureElementOptions execOrder(long execOrder) {
            wire.integer("exec_order", execOrder);
            return this;
        }

        /**
         * <code>display_level</code> (int) Must be &gt;= 0, else INVALID_INPUT.
         *
         * @param displayLevel the value
         * @return this builder
         */
        public SetFeatureElementOptions displayLevel(long displayLevel) {
            wire.integer("display_level", displayLevel);
            return this;
        }

        /**
         * <code>display_delim</code> (str) Stored verbatim. Not tri-state; cannot be cleared back
         * to null.
         *
         * @param displayDelim the value
         * @return this builder
         */
        public SetFeatureElementOptions displayDelim(String displayDelim) {
            wire.str("display_delim", displayDelim);
            return this;
        }

        /**
         * <code>derived</code> (str) Case-insensitive Yes/No, normalized, else INVALID_INPUT.
         *
         * @param derived the value
         * @return this builder
         */
        public SetFeatureElementOptions derived(String derived) {
            wire.str("derived", derived);
            return this;
        }
    }

    /**
     * Update one feature-element (CFG_FBOM) row's exec order, display level, delimiter or
     * derived flag.
     *
     * <p>Notes:
     * NOT_FOUND when the (feature, element) mapping is absent. Succeeds unchanged when no
     * update args are given. A missing CFG_FTYPE/CFG_FELEM surfaces as NOT_FOUND (code
     * lookup).
     *
     * <p>Wire name: {@code set_feature_element}; group: {@code elements}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param featureCode <code>feature_code</code> (str) Required. Case-insensitive; unknown is NOT_FOUND.
     * @param elementCode <code>element_code</code> (str) Required. Case-insensitive; unknown is NOT_FOUND.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String setFeatureElement(String configJson, String featureCode, String elementCode) throws SzConfigToolException {
        return setFeatureElement(configJson, featureCode, elementCode, null);
    }

    /**
     * Update one feature-element (CFG_FBOM) row's exec order, display level, delimiter or
     * derived flag.
     *
     * <p>Notes:
     * NOT_FOUND when the (feature, element) mapping is absent. Succeeds unchanged when no
     * update args are given. A missing CFG_FTYPE/CFG_FELEM surfaces as NOT_FOUND (code
     * lookup).
     *
     * <p>Wire name: {@code set_feature_element}; group: {@code elements}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param featureCode <code>feature_code</code> (str) Required. Case-insensitive; unknown is NOT_FOUND.
     * @param elementCode <code>element_code</code> (str) Required. Case-insensitive; unknown is NOT_FOUND.
     * @param options optional arguments ({@code null} = none); see {@link SetFeatureElementOptions}
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String setFeatureElement(String configJson, String featureCode, String elementCode, SetFeatureElementOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("feature_code", featureCode);
        wire.str("element_code", elementCode);
        return Invoker.config("set_feature_element", configJson, wire);
    }

    /** Optional arguments of {@link #addElementToFeature}; unset = omitted (library default). */
    public static final class AddElementToFeatureOptions {
        final Args wire = new Args();

        /**
         * <code>display_level</code> (int) Must be &gt;= 0, else INVALID_INPUT. Library default
         * when omitted: 1.
         *
         * @param displayLevel the value
         * @return this builder
         */
        public AddElementToFeatureOptions displayLevel(long displayLevel) {
            wire.integer("display_level", displayLevel);
            return this;
        }

        /**
         * <code>display_delim</code> (str) Stored verbatim; absent stores null.
         *
         * @param displayDelim the value
         * @return this builder
         */
        public AddElementToFeatureOptions displayDelim(String displayDelim) {
            wire.str("display_delim", displayDelim);
            return this;
        }

        /**
         * <code>derived</code> (str) Case-insensitive Yes/No, normalized, else INVALID_INPUT.
         * Library default when omitted: "No".
         *
         * @param derived the value
         * @return this builder
         */
        public AddElementToFeatureOptions derived(String derived) {
            wire.str("derived", derived);
            return this;
        }
    }

    /**
     * Map an existing element to a feature (append a CFG_FBOM row).
     *
     * <p>Notes:
     * EXEC_ORDER is always auto-allocated as max(EXEC_ORDER over the WHOLE CFG_FBOM table) + 1
     * and cannot be requested (use features.add_feature_comparison for an explicit order).
     * Duplicate (FTYPE_ID, FELEM_ID) is ALREADY_EXISTS. Validation order: feature, element,
     * display_level, derived, CFG_FBOM section, duplicate.
     *
     * <p>Wire name: {@code add_element_to_feature}; group: {@code elements}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param featureCode <code>feature_code</code> (str) Case-insensitive; unknown is NOT_FOUND.
     * @param elementCode <code>element_code</code> (str) Case-insensitive; unknown is NOT_FOUND (the element is
     * never auto-created).
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND, INVALID_INPUT, ALREADY_EXISTS; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addElementToFeature(String configJson, String featureCode, String elementCode) throws SzConfigToolException {
        return addElementToFeature(configJson, featureCode, elementCode, null);
    }

    /**
     * Map an existing element to a feature (append a CFG_FBOM row).
     *
     * <p>Notes:
     * EXEC_ORDER is always auto-allocated as max(EXEC_ORDER over the WHOLE CFG_FBOM table) + 1
     * and cannot be requested (use features.add_feature_comparison for an explicit order).
     * Duplicate (FTYPE_ID, FELEM_ID) is ALREADY_EXISTS. Validation order: feature, element,
     * display_level, derived, CFG_FBOM section, duplicate.
     *
     * <p>Wire name: {@code add_element_to_feature}; group: {@code elements}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param featureCode <code>feature_code</code> (str) Case-insensitive; unknown is NOT_FOUND.
     * @param elementCode <code>element_code</code> (str) Case-insensitive; unknown is NOT_FOUND (the element is
     * never auto-created).
     * @param options optional arguments ({@code null} = none); see {@link AddElementToFeatureOptions}
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND, INVALID_INPUT, ALREADY_EXISTS; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addElementToFeature(String configJson, String featureCode, String elementCode, AddElementToFeatureOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("feature_code", featureCode);
        wire.str("element_code", elementCode);
        return Invoker.config("add_element_to_feature", configJson, wire);
    }

    /**
     * Remove one feature-element (CFG_FBOM) mapping.
     *
     * <p>Notes:
     * NOT_FOUND when the mapping is absent. The CFG_FELEM row is kept.
     *
     * <p>Wire name: {@code delete_element_from_feature}; group: {@code elements}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param featureCode <code>feature_code</code> (str) Case-insensitive; unknown is NOT_FOUND.
     * @param elementCode <code>element_code</code> (str) Case-insensitive; unknown is NOT_FOUND.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteElementFromFeature(String configJson, String featureCode, String elementCode) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("feature_code", featureCode);
        wire.str("element_code", elementCode);
        return Invoker.config("delete_element_from_feature", configJson, wire);
    }

    /**
     * Render a config document in canonical export form (recursively key-sorted,
     * pretty-printed).
     *
     * <p>Notes:
     * Result is a JSON STRING holding the rendered text (not the working config envelope):
     * every object's keys sorted recursively (Python json.dumps(sort_keys=True)), array order
     * kept, no trailing newline. The text is itself a valid config and may be passed back in.
     * Any JSON document is accepted (no G2_CONFIG check).
     *
     * <p>Wire name: {@code render_config}; group: {@code export}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param indent <code>indent</code> (int) Spaces per indentation level, applied verbatim (2 = CLI
     * on-disk form, 4 = Python parity; 0 allowed). Negative is INVALID_INPUT (raised by the
     * req_usize converter, before the library is called).
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String renderConfig(String configJson, long indent) throws SzConfigToolException {
        Args wire = new Args();
        wire.integer("indent", indent);
        return Invoker.json("render_config", configJson, wire);
    }

    /** Optional arguments of {@link #addFeature}; unset = omitted (library default). */
    public static final class AddFeatureOptions {
        final Args wire = new Args();

        /**
         * <code>class</code> (str) CFG_FCLASS code, case-insensitive; unknown is NOT_FOUND.
         * Library default when omitted: "OTHER".
         *
         * @param classValue the value
         * @return this builder
         */
        public AddFeatureOptions classValue(String classValue) {
            wire.str("class", classValue);
            return this;
        }

        /**
         * <code>behavior</code> (str) Behavior code (A1, F1, FF, FM, FVM, NONE, NAME; E/S suffixes
         * set FTYPE_EXCL/FTYPE_STAB), case-insensitive; otherwise INVALID_INPUT. Library default
         * when omitted: "FM".
         *
         * @param behavior the value
         * @return this builder
         */
        public AddFeatureOptions behavior(String behavior) {
            wire.str("behavior", behavior);
            return this;
        }

        /**
         * <code>candidates</code> (str) USED_FOR_CAND; case-insensitive Yes/No, normalized, else
         * INVALID_INPUT. Library default when omitted: "No".
         *
         * @param candidates the value
         * @return this builder
         */
        public AddFeatureOptions candidates(String candidates) {
            wire.str("candidates", candidates);
            return this;
        }

        /**
         * <code>anonymize</code> (str) ANONYMIZE; case-insensitive Yes/No, normalized, else
         * INVALID_INPUT. Library default when omitted: "No".
         *
         * @param anonymize the value
         * @return this builder
         */
        public AddFeatureOptions anonymize(String anonymize) {
            wire.str("anonymize", anonymize);
            return this;
        }

        /**
         * <code>derived</code> (str) DERIVED; case-insensitive Yes/No, normalized, else
         * INVALID_INPUT. Library default when omitted: "No".
         *
         * @param derived the value
         * @return this builder
         */
        public AddFeatureOptions derived(String derived) {
            wire.str("derived", derived);
            return this;
        }

        /**
         * <code>history</code> (str) PERSIST_HISTORY; case-insensitive Yes/No, normalized, else
         * INVALID_INPUT. Library default when omitted: "Yes".
         *
         * @param history the value
         * @return this builder
         */
        public AddFeatureOptions history(String history) {
            wire.str("history", history);
            return this;
        }

        /**
         * <code>matchkey</code> (str) SHOW_IN_MATCH_KEY; case-insensitive Yes, No, Confirm or
         * Denial, normalized, else INVALID_INPUT. Absent defaults to Yes when
         * <code>comparison</code> is given, else No.
         *
         * @param matchkey the value
         * @return this builder
         */
        public AddFeatureOptions matchkey(String matchkey) {
            wire.str("matchkey", matchkey);
            return this;
        }

        /**
         * <code>standardize</code> (str) CFG_SFUNC code (case-insensitive) or NOT_FOUND (an empty
         * string is looked up too, so "" is NOT_FOUND). Creates a CFG_SFCALL row (SFCALL_ID max+1
         * floor 1000, EXEC_ORDER 1, FELEM_ID -1).
         *
         * @param standardize the value
         * @return this builder
         */
        public AddFeatureOptions standardize(String standardize) {
            wire.str("standardize", standardize);
            return this;
        }

        /**
         * <code>expression</code> (str) CFG_EFUNC code (case-insensitive) or NOT_FOUND ("" is
         * NOT_FOUND). Requires at least one element_list item with expressed=yes, else
         * INVALID_INPUT. Creates a CFG_EFCALL row (EFCALL_ID max+1 floor 1000, EXEC_ORDER 1) and a
         * CFG_EFBOM row (FELEM_REQ Yes) per expressed element.
         *
         * @param expression the value
         * @return this builder
         */
        public AddFeatureOptions expression(String expression) {
            wire.str("expression", expression);
            return this;
        }

        /**
         * <code>comparison</code> (str) CFG_CFUNC code (case-insensitive) or NOT_FOUND ("" is
         * NOT_FOUND). Requires at least one element_list item with compared=yes, else
         * INVALID_INPUT. Creates a CFG_CFCALL row (CFCALL_ID max+1 floor 1000) and a CFG_CFBOM row
         * per compared element.
         *
         * @param comparison the value
         * @return this builder
         */
        public AddFeatureOptions comparison(String comparison) {
            wire.str("comparison", comparison);
            return this;
        }

        /**
         * <code>version</code> (int) Stored verbatim in VERSION. Library default when omitted: 1.
         *
         * @param version the value
         * @return this builder
         */
        public AddFeatureOptions version(long version) {
            wire.integer("version", version);
            return this;
        }

        /**
         * <code>rtype_id</code> (int) Stored verbatim in RTYPE_ID; not validated against
         * CFG_RTYPE. Library default when omitted: 0.
         *
         * @param rtypeId the value
         * @return this builder
         */
        public AddFeatureOptions rtypeId(long rtypeId) {
            wire.integer("rtype_id", rtypeId);
            return this;
        }

        /**
         * <code>id</code> (int) Requested FTYPE_ID. Absent OR &lt;= 0 means auto-allocate (max
         * existing + 1, floor 1000). A taken id &gt; 0 is ALREADY_EXISTS.
         *
         * @param id the value
         * @return this builder
         */
        public AddFeatureOptions id(long id) {
            wire.integer("id", id);
            return this;
        }
    }

    /**
     * Add a feature (CFG_FTYPE row) with its element list (CFG_FBOM rows) and optional
     * standardize/expression/comparison calls.
     *
     * <p>Notes:
     * Validation order: CFG_FTYPE section, duplicate code, element_list shape,
     * candidates/anonymize/derived/history/matchkey domains, id, behavior, class, function
     * codes, expressed/compared counts, then per-element (display level, derived). The input
     * config is never modified on error.
     *
     * <p>Wire name: {@code add_feature}; group: {@code features}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param feature <code>feature</code> (str) Uppercased; stored as FTYPE_CODE and FTYPE_DESC. Duplicate
     * (exact match on the uppercased code) is ALREADY_EXISTS.
     * @param elementList <code>element_list</code> (json) Must be a non-empty JSON array, else INVALID_INPUT.
     * Each item is either an element-code string, or an object with <code>element</code> (or
     * <code>ELEMENT</code>, required, else INVALID_INPUT) and optional
     * <code>expressed</code>/<code>EXPRESSED</code>,
     * <code>compared</code>/<code>COMPARED</code> ("yes" case-insensitive = true),
     * <code>display</code>/<code>DISPLAY</code> ("yes" = DISPLAY_LEVEL 1, anything else 0) or
     * <code>displaylevel</code>/<code>DISPLAYLEVEL</code>/<code>display_level</code> (int,
     * default 1, negative = INVALID_INPUT),
     * <code>displaydelim</code>/<code>DISPLAYDELIM</code>/<code>display_delim</code>,
     * <code>derived</code>/<code>DERIVED</code> (Yes/No case-insensitive, else INVALID_INPUT;
     * default No). Any other item type is INVALID_INPUT. Element codes are uppercased; a code
     * not in CFG_FELEM is AUTO-CREATED (FELEM_ID max+1 floor 1000, DATA_TYPE string,
     * FELEM_DESC = code). The FBOM EXEC_ORDER is the item's 1-based position (per feature, not
     * whole-table).
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, ALREADY_EXISTS, INVALID_INPUT, NOT_FOUND, INVALID_STRUCTURE; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addFeature(String configJson, String feature, String elementList) throws SzConfigToolException {
        return addFeature(configJson, feature, elementList, null);
    }

    /**
     * Add a feature (CFG_FTYPE row) with its element list (CFG_FBOM rows) and optional
     * standardize/expression/comparison calls.
     *
     * <p>Notes:
     * Validation order: CFG_FTYPE section, duplicate code, element_list shape,
     * candidates/anonymize/derived/history/matchkey domains, id, behavior, class, function
     * codes, expressed/compared counts, then per-element (display level, derived). The input
     * config is never modified on error.
     *
     * <p>Wire name: {@code add_feature}; group: {@code features}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param feature <code>feature</code> (str) Uppercased; stored as FTYPE_CODE and FTYPE_DESC. Duplicate
     * (exact match on the uppercased code) is ALREADY_EXISTS.
     * @param elementList <code>element_list</code> (json) Must be a non-empty JSON array, else INVALID_INPUT.
     * Each item is either an element-code string, or an object with <code>element</code> (or
     * <code>ELEMENT</code>, required, else INVALID_INPUT) and optional
     * <code>expressed</code>/<code>EXPRESSED</code>,
     * <code>compared</code>/<code>COMPARED</code> ("yes" case-insensitive = true),
     * <code>display</code>/<code>DISPLAY</code> ("yes" = DISPLAY_LEVEL 1, anything else 0) or
     * <code>displaylevel</code>/<code>DISPLAYLEVEL</code>/<code>display_level</code> (int,
     * default 1, negative = INVALID_INPUT),
     * <code>displaydelim</code>/<code>DISPLAYDELIM</code>/<code>display_delim</code>,
     * <code>derived</code>/<code>DERIVED</code> (Yes/No case-insensitive, else INVALID_INPUT;
     * default No). Any other item type is INVALID_INPUT. Element codes are uppercased; a code
     * not in CFG_FELEM is AUTO-CREATED (FELEM_ID max+1 floor 1000, DATA_TYPE string,
     * FELEM_DESC = code). The FBOM EXEC_ORDER is the item's 1-based position (per feature, not
     * whole-table).
     * @param options optional arguments ({@code null} = none); see {@link AddFeatureOptions}
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, ALREADY_EXISTS, INVALID_INPUT, NOT_FOUND, INVALID_STRUCTURE; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addFeature(String configJson, String feature, String elementList, AddFeatureOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("feature", feature);
        wire.json("element_list", elementList);
        return Invoker.config("add_feature", configJson, wire);
    }

    /**
     * Delete a feature and cascade-delete its FBOM rows, attributes and
     * standardize/expression/comparison/distinct calls.
     *
     * <p>Notes:
     * Locked features NAME, ADDRESS, PHONE, DOB, REL_LINK, REL_ANCHOR, REL_POINTER are
     * INVALID_INPUT. Cascade removes the feature's CFG_FBOM rows, CFG_ATTR rows whose
     * FTYPE_CODE matches, CFG_SFCALL, CFG_EFCALL (+ their CFG_EFBOM), CFG_CFCALL (+ CFG_CFBOM)
     * and CFG_DFCALL (+ CFG_DFBOM) rows; CFG_FELEM rows are kept. A missing CFG_FTYPE is
     * MISSING_SECTION for an id but NOT_FOUND for a code.
     *
     * <p>Wire name: {@code delete_feature}; group: {@code features}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param feature <code>feature</code> (str) A string that parses (after trim) as an integer is an
     * FTYPE_ID; otherwise a feature code, uppercased. Unknown is NOT_FOUND.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteFeature(String configJson, String feature) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("feature", feature);
        return Invoker.config("delete_feature", configJson, wire);
    }

    /**
     * Get one feature as a display summary including its elementList.
     *
     * <p>Notes:
     * Result {id, feature, class, behavior, anonymize, candidates, standardize, expression,
     * comparison, matchKey, version, elementList}; elementList items {element, expressed,
     * compared, derived, display} sorted by EXEC_ORDER, display = "No" iff DISPLAY_LEVEL is 0.
     * standardize/expression/comparison are "" when absent. DERIVED, PERSIST_HISTORY and
     * RTYPE_ID of the feature are NOT in the result. A missing CFG_FTYPE section is NOT_FOUND
     * (never MISSING_SECTION).
     *
     * <p>Wire name: {@code get_feature}; group: {@code features}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param feature <code>feature</code> (str) Integer string (after trim) = FTYPE_ID; otherwise a code,
     * uppercased.
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String getFeature(String configJson, String feature) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("feature", feature);
        return Invoker.json("get_feature", configJson, wire);
    }

    /**
     * List all features as display summaries, sorted by FTYPE_ID.
     *
     * <p>Notes:
     * Array of the same objects get_feature returns.
     *
     * <p>Wire name: {@code list_features}; group: {@code features}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String listFeatures(String configJson) throws SzConfigToolException {
        Args wire = new Args();
        return Invoker.json("list_features", configJson, wire);
    }

    /** Optional arguments of {@link #setFeature}; unset = omitted (library default). */
    public static final class SetFeatureOptions {
        final Args wire = new Args();

        /**
         * <code>candidates</code> (str) USED_FOR_CAND; case-insensitive Yes/No, normalized, else
         * INVALID_INPUT.
         *
         * @param candidates the value
         * @return this builder
         */
        public SetFeatureOptions candidates(String candidates) {
            wire.str("candidates", candidates);
            return this;
        }

        /**
         * <code>anonymize</code> (str) TRAP — stored VERBATIM in ANONYMIZE (no validation or
         * normalization, unlike add_feature).
         *
         * @param anonymize the value
         * @return this builder
         */
        public SetFeatureOptions anonymize(String anonymize) {
            wire.str("anonymize", anonymize);
            return this;
        }

        /**
         * <code>derived</code> (str) TRAP — stored VERBATIM in DERIVED (no validation or
         * normalization).
         *
         * @param derived the value
         * @return this builder
         */
        public SetFeatureOptions derived(String derived) {
            wire.str("derived", derived);
            return this;
        }

        /**
         * <code>history</code> (str) TRAP — stored VERBATIM in PERSIST_HISTORY (no validation or
         * normalization).
         *
         * @param history the value
         * @return this builder
         */
        public SetFeatureOptions history(String history) {
            wire.str("history", history);
            return this;
        }

        /**
         * <code>matchkey</code> (str) SHOW_IN_MATCH_KEY; case-insensitive Yes, No, Confirm or
         * Denial, normalized, else INVALID_INPUT.
         *
         * @param matchkey the value
         * @return this builder
         */
        public SetFeatureOptions matchkey(String matchkey) {
            wire.str("matchkey", matchkey);
            return this;
        }

        /**
         * <code>behavior</code> (str) Behavior code (case-insensitive) parsed into
         * FTYPE_FREQ/FTYPE_EXCL/FTYPE_STAB, else INVALID_INPUT.
         *
         * @param behavior the value
         * @return this builder
         */
        public SetFeatureOptions behavior(String behavior) {
            wire.str("behavior", behavior);
            return this;
        }

        /**
         * <code>class</code> (str) CFG_FCLASS code, case-insensitive; unknown is NOT_FOUND.
         *
         * @param classValue the value
         * @return this builder
         */
        public SetFeatureOptions classValue(String classValue) {
            wire.str("class", classValue);
            return this;
        }

        /**
         * <code>version</code> (int) Stored verbatim in VERSION.
         *
         * @param version the value
         * @return this builder
         */
        public SetFeatureOptions version(long version) {
            wire.integer("version", version);
            return this;
        }

        /**
         * <code>rtype_id</code> (int) Stored verbatim in RTYPE_ID; not validated.
         *
         * @param rtypeId the value
         * @return this builder
         */
        public SetFeatureOptions rtypeId(long rtypeId) {
            wire.integer("rtype_id", rtypeId);
            return this;
        }
    }

    /**
     * Update a feature's flags, behavior, class, version or RTYPE_ID.
     *
     * <p>Notes:
     * If no supplied value differs from the stored row (including when no update args are
     * given) the call fails with INVALID_INPUT "No changes detected". The input config is
     * never modified on error.
     *
     * <p>Wire name: {@code set_feature}; group: {@code features}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param feature <code>feature</code> (str) Integer string (after trim) = FTYPE_ID; otherwise a code,
     * uppercased. Unknown is NOT_FOUND.
     * @param options optional arguments ({@code null} = none); see {@link SetFeatureOptions}
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String setFeature(String configJson, String feature, SetFeatureOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("feature", feature);
        return Invoker.config("set_feature", configJson, wire);
    }

    /** Optional arguments of {@link #addFeatureComparison}; unset = omitted (library default). */
    public static final class AddFeatureComparisonOptions {
        final Args wire = new Args();

        /**
         * <code>exec_order</code> (int) WHOLE-TABLE scope: absent or &lt;= 0 auto-allocates
         * max(EXEC_ORDER over all of CFG_FBOM) + 1; a value &gt; 0 already used by ANY CFG_FBOM
         * row is ALREADY_EXISTS.
         *
         * @param execOrder the value
         * @return this builder
         */
        public AddFeatureComparisonOptions execOrder(long execOrder) {
            wire.integer("exec_order", execOrder);
            return this;
        }

        /**
         * <code>display_level</code> (int) TRAP — stored verbatim, NOT validated, and absent
         * stores DISPLAY_LEVEL null (no default 1, unlike add_element_to_feature).
         *
         * @param displayLevel the value
         * @return this builder
         */
        public AddFeatureComparisonOptions displayLevel(long displayLevel) {
            wire.integer("display_level", displayLevel);
            return this;
        }

        /**
         * <code>display_delim</code> (str) Stored verbatim; absent stores null.
         *
         * @param displayDelim the value
         * @return this builder
         */
        public AddFeatureComparisonOptions displayDelim(String displayDelim) {
            wire.str("display_delim", displayDelim);
            return this;
        }

        /**
         * <code>derived</code> (str) TRAP — stored verbatim, NOT validated; absent stores
         * DERIVED null.
         *
         * @param derived the value
         * @return this builder
         */
        public AddFeatureComparisonOptions derived(String derived) {
            wire.str("derived", derived);
            return this;
        }
    }

    /**
     * Add a feature-element (CFG_FBOM) row with an optional explicit EXEC_ORDER.
     *
     * <p>Notes:
     * Writes the same table as elements.add_element_to_feature. Duplicate (FTYPE_ID, FELEM_ID)
     * is ALREADY_EXISTS. A missing CFG_FTYPE/CFG_FELEM section surfaces as NOT_FOUND (code
     * lookup); a missing CFG_FBOM as MISSING_SECTION.
     *
     * <p>Wire name: {@code add_feature_comparison}; group: {@code features}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param featureCode <code>feature_code</code> (str) Required. Feature code, case-insensitive; unknown is
     * NOT_FOUND.
     * @param elementCode <code>element_code</code> (str) Required. Element code, case-insensitive; unknown is
     * NOT_FOUND.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND, ALREADY_EXISTS; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addFeatureComparison(String configJson, String featureCode, String elementCode) throws SzConfigToolException {
        return addFeatureComparison(configJson, featureCode, elementCode, null);
    }

    /**
     * Add a feature-element (CFG_FBOM) row with an optional explicit EXEC_ORDER.
     *
     * <p>Notes:
     * Writes the same table as elements.add_element_to_feature. Duplicate (FTYPE_ID, FELEM_ID)
     * is ALREADY_EXISTS. A missing CFG_FTYPE/CFG_FELEM section surfaces as NOT_FOUND (code
     * lookup); a missing CFG_FBOM as MISSING_SECTION.
     *
     * <p>Wire name: {@code add_feature_comparison}; group: {@code features}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param featureCode <code>feature_code</code> (str) Required. Feature code, case-insensitive; unknown is
     * NOT_FOUND.
     * @param elementCode <code>element_code</code> (str) Required. Element code, case-insensitive; unknown is
     * NOT_FOUND.
     * @param options optional arguments ({@code null} = none); see {@link AddFeatureComparisonOptions}
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND, ALREADY_EXISTS; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addFeatureComparison(String configJson, String featureCode, String elementCode, AddFeatureComparisonOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("feature_code", featureCode);
        wire.str("element_code", elementCode);
        return Invoker.config("add_feature_comparison", configJson, wire);
    }

    /**
     * Delete one feature-element (CFG_FBOM) row.
     *
     * <p>Notes:
     * Same effect as elements.delete_element_from_feature, except a missing CFG_FBOM section
     * is NOT_FOUND here (MISSING_SECTION there). An absent mapping is NOT_FOUND.
     *
     * <p>Wire name: {@code delete_feature_comparison}; group: {@code features}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param featureCode <code>feature_code</code> (str) Case-insensitive; unknown is NOT_FOUND.
     * @param elementCode <code>element_code</code> (str) Case-insensitive; unknown is NOT_FOUND.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteFeatureComparison(String configJson, String featureCode, String elementCode) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("feature_code", featureCode);
        wire.str("element_code", elementCode);
        return Invoker.config("delete_feature_comparison", configJson, wire);
    }

    /**
     * Get one raw CFG_FBOM row by feature and element code.
     *
     * <p>Notes:
     * Result uses on-disk keys (FTYPE_ID, FELEM_ID, EXEC_ORDER, DISPLAY_LEVEL, DISPLAY_DELIM,
     * DERIVED).
     *
     * <p>Wire name: {@code get_feature_comparison}; group: {@code features}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param featureCode <code>feature_code</code> (str) Required. Case-insensitive; unknown is NOT_FOUND.
     * @param elementCode <code>element_code</code> (str) Required. Case-insensitive; unknown is NOT_FOUND.
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String getFeatureComparison(String configJson, String featureCode, String elementCode) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("feature_code", featureCode);
        wire.str("element_code", elementCode);
        return Invoker.json("get_feature_comparison", configJson, wire);
    }

    /**
     * List all raw CFG_FBOM rows sorted by (FTYPE_ID, EXEC_ORDER).
     *
     * <p>Wire name: {@code list_feature_comparisons}; group: {@code features}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String listFeatureComparisons(String configJson) throws SzConfigToolException {
        Args wire = new Args();
        return Invoker.json("list_feature_comparisons", configJson, wire);
    }

    /** Optional arguments of {@link #addFeatureDistinctCallElement}; unset = omitted (library default). */
    public static final class AddFeatureDistinctCallElementOptions {
        final Args wire = new Args();

        /**
         * <code>element_code</code> (str) TRAP — only validated (unknown is NOT_FOUND); it is
         * NOT stored (CFG_DFCALL has no FELEM_ID column).
         *
         * @param elementCode the value
         * @return this builder
         */
        public AddFeatureDistinctCallElementOptions elementCode(String elementCode) {
            wire.str("element_code", elementCode);
            return this;
        }

        /**
         * <code>exec_order</code> (int) TRAP — IGNORED entirely (CFG_DFCALL has no EXEC_ORDER
         * column).
         *
         * @param execOrder the value
         * @return this builder
         */
        public AddFeatureDistinctCallElementOptions execOrder(long execOrder) {
            wire.integer("exec_order", execOrder);
            return this;
        }
    }

    /**
     * Add a distinct-function call (CFG_DFCALL row) for a feature.
     *
     * <p>Notes:
     * Duplicate (FTYPE_ID, DFUNC_ID) is ALREADY_EXISTS. New row is exactly {DFCALL_ID (max+1
     * floor 1000), FTYPE_ID, DFUNC_ID}; no CFG_DFBOM rows are written.
     *
     * <p>Wire name: {@code add_feature_distinct_call_element}; group: {@code features}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param featureCode <code>feature_code</code> (str) Required. Case-insensitive; unknown is NOT_FOUND.
     * @param distinctFuncCode <code>distinct_func_code</code> (str) Required. CFG_DFUNC code, case-insensitive;
     * unknown is NOT_FOUND.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND, ALREADY_EXISTS; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addFeatureDistinctCallElement(String configJson, String featureCode, String distinctFuncCode) throws SzConfigToolException {
        return addFeatureDistinctCallElement(configJson, featureCode, distinctFuncCode, null);
    }

    /**
     * Add a distinct-function call (CFG_DFCALL row) for a feature.
     *
     * <p>Notes:
     * Duplicate (FTYPE_ID, DFUNC_ID) is ALREADY_EXISTS. New row is exactly {DFCALL_ID (max+1
     * floor 1000), FTYPE_ID, DFUNC_ID}; no CFG_DFBOM rows are written.
     *
     * <p>Wire name: {@code add_feature_distinct_call_element}; group: {@code features}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param featureCode <code>feature_code</code> (str) Required. Case-insensitive; unknown is NOT_FOUND.
     * @param distinctFuncCode <code>distinct_func_code</code> (str) Required. CFG_DFUNC code, case-insensitive;
     * unknown is NOT_FOUND.
     * @param options optional arguments ({@code null} = none); see {@link AddFeatureDistinctCallElementOptions}
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND, ALREADY_EXISTS; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addFeatureDistinctCallElement(String configJson, String featureCode, String distinctFuncCode, AddFeatureDistinctCallElementOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("feature_code", featureCode);
        wire.str("distinct_func_code", distinctFuncCode);
        return Invoker.config("add_feature_distinct_call_element", configJson, wire);
    }

    /**
     * List all raw CFG_FCLASS rows sorted by FCLASS_ID.
     *
     * <p>Wire name: {@code list_feature_classes}; group: {@code features}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String listFeatureClasses(String configJson) throws SzConfigToolException {
        Args wire = new Args();
        return Invoker.json("list_feature_classes", configJson, wire);
    }

    /**
     * Get one raw CFG_FCLASS row by id or code.
     *
     * <p>Wire name: {@code get_feature_class}; group: {@code features}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param featureClass <code>feature_class</code> (str) Integer string (after trim) = FCLASS_ID; otherwise a
     * code, uppercased. Unknown is NOT_FOUND.
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String getFeatureClass(String configJson, String featureClass) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("feature_class", featureClass);
        return Invoker.json("get_feature_class", configJson, wire);
    }

    /**
     * Set G2_CONFIG.CONFIG_BASE_VERSION.COMPATIBILITY_VERSION.FEATURE_VERSION.
     *
     * <p>Notes:
     * MISSING_SECTION when COMPATIBILITY_VERSION is absent or not an object. No manifest
     * function reads FEATURE_VERSION back (versioning reads CONFIG_VERSION).
     *
     * <p>Wire name: {@code update_feature_version}; group: {@code features}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param version <code>version</code> (str) Stored verbatim as a string (inserted or overwritten); not
     * validated.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String updateFeatureVersion(String configJson, String version) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("version", version);
        return Invoker.config("update_feature_version", configJson, wire);
    }

    /**
     * Add a rule fragment (CFG_ERFRAG row), returning the assigned ERFRAG_ID.
     *
     * <p>Notes:
     * Record is the assigned ERFRAG_ID (integer). The row always carries every CFG_ERFRAG key:
     * ERFRAG_DESC is set to the uppercased code, ERFRAG_DEPENDS is the referenced fragments'
     * ids sorted as STRINGS, deduplicated and comma-joined ("11,61"), or null when there are
     * none. ERFRAG_CODE and ERFRAG_SOURCE are checked BEFORE the config is parsed
     * (MISSING_FIELD wins). A config without G2_CONFIG is INVALID_CONFIG; with G2_CONFIG but
     * no CFG_ERFRAG it is MISSING_SECTION.
     *
     * <p>Wire name: {@code add_fragment}; group: {@code fragments}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param fragmentConfig <code>fragment_config</code> (json) Object with on-disk keys. ERFRAG_CODE (string,
     * required, else MISSING_FIELD) is uppercased for storage and the duplicate check
     * (ALREADY_EXISTS). ERFRAG_SOURCE (string, required, else MISSING_FIELD) is stored
     * verbatim; every name referenced inside a FRAGMENT[...] clause (e.g.
     * "./FRAGMENT[./SAME_NAME&gt;0 and ./SAME_STAB&gt;0]") must be an existing ERFRAG_CODE
     * matched EXACTLY (case-sensitive), else INVALID_INPUT. A source without FRAGMENT[
     * (including "") is accepted unvalidated. ERFRAG_ID (integer, optional): absent or &lt;= 0
     * auto-allocates (max + 1, floor 1, so 1000 on the template); a taken id &gt; 0 is
     * ALREADY_EXISTS. Any ERFRAG_DESC key is IGNORED.
     * @return the modified configuration JSON document (opaque); {@link #addFragmentResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_FIELD, ALREADY_EXISTS, INVALID_INPUT, INVALID_CONFIG, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addFragment(String configJson, String fragmentConfig) throws SzConfigToolException {
        Args wire = new Args();
        wire.json("fragment_config", fragmentConfig);
        return Invoker.call("add_fragment", "config_and_json", configJson, wire)[1];
    }

    /**
     * The record (row / ids) of <code>addFragment</code>: same arguments and operation, but
     * returns the record instead of the configuration. Operation: Add a rule fragment
     * (CFG_ERFRAG row), returning the assigned ERFRAG_ID.
     *
     * <p>Notes:
     * Record is the assigned ERFRAG_ID (integer). The row always carries every CFG_ERFRAG key:
     * ERFRAG_DESC is set to the uppercased code, ERFRAG_DEPENDS is the referenced fragments'
     * ids sorted as STRINGS, deduplicated and comma-joined ("11,61"), or null when there are
     * none. ERFRAG_CODE and ERFRAG_SOURCE are checked BEFORE the config is parsed
     * (MISSING_FIELD wins). A config without G2_CONFIG is INVALID_CONFIG; with G2_CONFIG but
     * no CFG_ERFRAG it is MISSING_SECTION.
     *
     * <p>Wire name: {@code add_fragment}; group: {@code fragments}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param fragmentConfig <code>fragment_config</code> (json) Object with on-disk keys. ERFRAG_CODE (string,
     * required, else MISSING_FIELD) is uppercased for storage and the duplicate check
     * (ALREADY_EXISTS). ERFRAG_SOURCE (string, required, else MISSING_FIELD) is stored
     * verbatim; every name referenced inside a FRAGMENT[...] clause (e.g.
     * "./FRAGMENT[./SAME_NAME&gt;0 and ./SAME_STAB&gt;0]") must be an existing ERFRAG_CODE
     * matched EXACTLY (case-sensitive), else INVALID_INPUT. A source without FRAGMENT[
     * (including "") is accepted unvalidated. ERFRAG_ID (integer, optional): absent or &lt;= 0
     * auto-allocates (max + 1, floor 1, so 1000 on the template); a taken id &gt; 0 is
     * ALREADY_EXISTS. Any ERFRAG_DESC key is IGNORED.
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_FIELD, ALREADY_EXISTS, INVALID_INPUT, INVALID_CONFIG, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addFragmentResult(String configJson, String fragmentConfig) throws SzConfigToolException {
        Args wire = new Args();
        wire.json("fragment_config", fragmentConfig);
        return Invoker.result("add_fragment", "config_and_json", configJson, wire);
    }

    /**
     * Delete a fragment by code.
     *
     * <p>Notes:
     * TRAP: no dependency check; a fragment still referenced by a rule (QUAL_ERFRAG_CODE /
     * DISQ_ERFRAG_CODE) or by another fragment's ERFRAG_DEPENDS is deleted anyway, leaving
     * dangling references. A config without CFG_ERFRAG is NOT_FOUND.
     *
     * <p>Wire name: {@code delete_fragment}; group: {@code fragments}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased, then matched exactly against ERFRAG_CODE. Not an id.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteFragment(String configJson, String code) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("code", code);
        return Invoker.config("delete_fragment", configJson, wire);
    }

    /**
     * Get one fragment, by code or ERFRAG_ID, as a summary record.
     *
     * <p>Notes:
     * Result is {id, fragment, source, depends}: id/source/depends are null-preserving
     * projections of ERFRAG_ID/ERFRAG_SOURCE/ERFRAG_DEPENDS; fragment is ERFRAG_CODE or ""
     * when absent. ERFRAG_DESC is not reported.
     *
     * <p>Wire name: {@code get_fragment}; group: {@code fragments}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param codeOrId <code>code_or_id</code> (str) Uppercased, then matched exactly against ERFRAG_CODE
     * first, then numerically against ERFRAG_ID (e.g. "11").
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String getFragment(String configJson, String codeOrId) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("code_or_id", codeOrId);
        return Invoker.json("get_fragment", configJson, wire);
    }

    /**
     * List all fragments as summary records in config order.
     *
     * <p>Notes:
     * Result is an array of the get_fragment record shape, in config order (NOT sorted). A
     * missing CFG_ERFRAG or G2_CONFIG yields [].
     *
     * <p>Wire name: {@code list_fragments}; group: {@code fragments}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String listFragments(String configJson) throws SzConfigToolException {
        Args wire = new Args();
        return Invoker.json("list_fragments", configJson, wire);
    }

    /** Optional arguments of {@link #setFragment}; unset = omitted (library default). */
    public static final class SetFragmentOptions {
        final Args wire = new Args();

        /**
         * <code>source</code> (str) tri-state: leave / clear / set. ERFRAG_SOURCE. Absent = keep
         * source AND ERFRAG_DEPENDS; null = clear BOTH source and ERFRAG_DEPENDS to null; a string
         * is validated exactly as in add_fragment (INVALID_INPUT) and ERFRAG_DEPENDS is
         * recomputed.
         *
         * @param source the value
         * @return this builder
         */
        public SetFragmentOptions source(FieldUpdate<String> source) {
            wire.strUpdate("source", source);
            return this;
        }

        /**
         * <code>description</code> (str) tri-state: leave / clear / set. ERFRAG_DESC. Absent =
         * keep; null = clear to null; a string is stored verbatim.
         *
         * @param description the value
         * @return this builder
         */
        public SetFragmentOptions description(FieldUpdate<String> description) {
            wire.strUpdate("description", description);
            return this;
        }
    }

    /**
     * Update a fragment's source and/or description.
     *
     * <p>Notes:
     * The row is rewritten with every CFG_ERFRAG key; ERFRAG_ID is preserved.
     *
     * <p>Wire name: {@code set_fragment}; group: {@code fragments}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Fragment code; uppercased, then matched exactly. Unknown is
     * NOT_FOUND.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String setFragment(String configJson, String code) throws SzConfigToolException {
        return setFragment(configJson, code, null);
    }

    /**
     * Update a fragment's source and/or description.
     *
     * <p>Notes:
     * The row is rewritten with every CFG_ERFRAG key; ERFRAG_ID is preserved.
     *
     * <p>Wire name: {@code set_fragment}; group: {@code fragments}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Fragment code; uppercased, then matched exactly. Unknown is
     * NOT_FOUND.
     * @param options optional arguments ({@code null} = none); see {@link SetFragmentOptions}
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String setFragment(String configJson, String code, SetFragmentOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("code", code);
        return Invoker.config("set_fragment", configJson, wire);
    }

    /** Optional arguments of {@link #addComparisonFunction}; unset = omitted (library default). */
    public static final class AddComparisonFunctionOptions {
        final Args wire = new Args();

        /**
         * <code>connect_str</code> (str) Absent stores CONNECT_STR null; any string (including "")
         * is stored verbatim.
         *
         * @param connectStr the value
         * @return this builder
         */
        public AddComparisonFunctionOptions connectStr(String connectStr) {
            wire.str("connect_str", connectStr);
            return this;
        }

        /**
         * <code>description</code> (str) Absent stores CFUNC_DESC null; any string is stored
         * verbatim.
         *
         * @param description the value
         * @return this builder
         */
        public AddComparisonFunctionOptions description(String description) {
            wire.str("description", description);
            return this;
        }

        /**
         * <code>language</code> (str) Absent stores LANGUAGE null; any string is stored verbatim.
         *
         * @param language the value
         * @return this builder
         */
        public AddComparisonFunctionOptions language(String language) {
            wire.str("language", language);
            return this;
        }

        /**
         * <code>anon_support</code> (str) Case-insensitive; normalized to Yes or No, else
         * INVALID_INPUT. Library default when omitted: "No".
         *
         * @param anonSupport the value
         * @return this builder
         */
        public AddComparisonFunctionOptions anonSupport(String anonSupport) {
            wire.str("anon_support", anonSupport);
            return this;
        }
    }

    /**
     * Add a comparison function (CFG_CFUNC row).
     *
     * <p>Notes:
     * Returns (modified config, the new complete CFG_CFUNC row: CFUNC_ID, CFUNC_CODE,
     * CONNECT_STR, ANON_SUPPORT, CFUNC_DESC, LANGUAGE). CFUNC_ID is always auto-allocated (max
     * existing + 1, floor 1); no id can be requested. TRAP: a duplicate code is INVALID_INPUT
     * (SzConfigError::validation), NOT ALREADY_EXISTS. Validation order: duplicate code,
     * anon_support, then section. MISSING_SECTION only when CFG_CFUNC is absent.
     *
     * <p>Wire name: {@code add_comparison_function}; group: {@code functions_comparison}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before the duplicate check and storage (CFUNC_CODE).
     * @return the modified configuration JSON document (opaque); {@link #addComparisonFunctionResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addComparisonFunction(String configJson, String code) throws SzConfigToolException {
        return addComparisonFunction(configJson, code, null);
    }

    /**
     * Add a comparison function (CFG_CFUNC row).
     *
     * <p>Notes:
     * Returns (modified config, the new complete CFG_CFUNC row: CFUNC_ID, CFUNC_CODE,
     * CONNECT_STR, ANON_SUPPORT, CFUNC_DESC, LANGUAGE). CFUNC_ID is always auto-allocated (max
     * existing + 1, floor 1); no id can be requested. TRAP: a duplicate code is INVALID_INPUT
     * (SzConfigError::validation), NOT ALREADY_EXISTS. Validation order: duplicate code,
     * anon_support, then section. MISSING_SECTION only when CFG_CFUNC is absent.
     *
     * <p>Wire name: {@code add_comparison_function}; group: {@code functions_comparison}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before the duplicate check and storage (CFUNC_CODE).
     * @param options optional arguments ({@code null} = none); see {@link AddComparisonFunctionOptions}
     * @return the modified configuration JSON document (opaque); {@link #addComparisonFunctionResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addComparisonFunction(String configJson, String code, AddComparisonFunctionOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("code", code);
        return Invoker.call("add_comparison_function", "config_and_json", configJson, wire)[1];
    }

    /**
     * The record (row / ids) of <code>addComparisonFunction</code>: same arguments and
     * operation, but returns the record instead of the configuration. Operation: Add a
     * comparison function (CFG_CFUNC row).
     *
     * <p>Notes:
     * Returns (modified config, the new complete CFG_CFUNC row: CFUNC_ID, CFUNC_CODE,
     * CONNECT_STR, ANON_SUPPORT, CFUNC_DESC, LANGUAGE). CFUNC_ID is always auto-allocated (max
     * existing + 1, floor 1); no id can be requested. TRAP: a duplicate code is INVALID_INPUT
     * (SzConfigError::validation), NOT ALREADY_EXISTS. Validation order: duplicate code,
     * anon_support, then section. MISSING_SECTION only when CFG_CFUNC is absent.
     *
     * <p>Wire name: {@code add_comparison_function}; group: {@code functions_comparison}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before the duplicate check and storage (CFUNC_CODE).
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addComparisonFunctionResult(String configJson, String code) throws SzConfigToolException {
        return addComparisonFunctionResult(configJson, code, null);
    }

    /**
     * The record (row / ids) of <code>addComparisonFunction</code>: same arguments and
     * operation, but returns the record instead of the configuration. Operation: Add a
     * comparison function (CFG_CFUNC row).
     *
     * <p>Notes:
     * Returns (modified config, the new complete CFG_CFUNC row: CFUNC_ID, CFUNC_CODE,
     * CONNECT_STR, ANON_SUPPORT, CFUNC_DESC, LANGUAGE). CFUNC_ID is always auto-allocated (max
     * existing + 1, floor 1); no id can be requested. TRAP: a duplicate code is INVALID_INPUT
     * (SzConfigError::validation), NOT ALREADY_EXISTS. Validation order: duplicate code,
     * anon_support, then section. MISSING_SECTION only when CFG_CFUNC is absent.
     *
     * <p>Wire name: {@code add_comparison_function}; group: {@code functions_comparison}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before the duplicate check and storage (CFUNC_CODE).
     * @param options optional arguments ({@code null} = none); see {@link AddComparisonFunctionOptions}
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addComparisonFunctionResult(String configJson, String code, AddComparisonFunctionOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("code", code);
        return Invoker.result("add_comparison_function", "config_and_json", configJson, wire);
    }

    /**
     * Delete a comparison function's CFG_CFUNC row only (no cascade).
     *
     * <p>Notes:
     * Returns (modified config, the deleted CFG_CFUNC row). Removes ONLY the CFG_CFUNC row;
     * CFG_CFCALL rows referencing it are left dangling (use
     * delete_comparison_function_cascade). A missing CFG_CFUNC section is NOT_FOUND (not
     * MISSING_SECTION).
     *
     * <p>Wire name: {@code delete_comparison_function}; group: {@code functions_comparison}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @return the modified configuration JSON document (opaque); {@link #deleteComparisonFunctionResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteComparisonFunction(String configJson, String code) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("code", code);
        return Invoker.call("delete_comparison_function", "config_and_json", configJson, wire)[1];
    }

    /**
     * The record (row / ids) of <code>deleteComparisonFunction</code>: same arguments and
     * operation, but returns the record instead of the configuration. Operation: Delete a
     * comparison function's CFG_CFUNC row only (no cascade).
     *
     * <p>Notes:
     * Returns (modified config, the deleted CFG_CFUNC row). Removes ONLY the CFG_CFUNC row;
     * CFG_CFCALL rows referencing it are left dangling (use
     * delete_comparison_function_cascade). A missing CFG_CFUNC section is NOT_FOUND (not
     * MISSING_SECTION).
     *
     * <p>Wire name: {@code delete_comparison_function}; group: {@code functions_comparison}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteComparisonFunctionResult(String configJson, String code) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("code", code);
        return Invoker.result("delete_comparison_function", "config_and_json", configJson, wire);
    }

    /**
     * Delete a comparison function and its CFG_CFBOM / CFG_CFCALL / CFG_CFRTN rows.
     *
     * <p>Notes:
     * Returns (modified config, the deleted CFG_CFUNC row). Order: CFG_CFBOM rows whose
     * CFCALL_ID belongs to one of the function's CFG_CFCALL rows; every CFG_CFCALL row with
     * that CFUNC_ID; every CFG_CFRTN row with that CFUNC_ID (well-formed rows via
     * thresholds::delete_comparison_threshold, then a sweep of the rest); finally the
     * CFG_CFUNC row. Absent CFBOM/CFCALL/CFRTN sections are skipped. MISSING_FIELD when the
     * found row has no integer CFUNC_ID. A missing CFG_CFUNC section is NOT_FOUND.
     *
     * <p>Wire name: {@code delete_comparison_function_cascade}; group: {@code functions_comparison}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @return the modified configuration JSON document (opaque); {@link #deleteComparisonFunctionCascadeResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, MISSING_FIELD; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteComparisonFunctionCascade(String configJson, String code) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("code", code);
        return Invoker.call("delete_comparison_function_cascade", "config_and_json", configJson, wire)[1];
    }

    /**
     * The record (row / ids) of <code>deleteComparisonFunctionCascade</code>: same arguments
     * and operation, but returns the record instead of the configuration. Operation: Delete a
     * comparison function and its CFG_CFBOM / CFG_CFCALL / CFG_CFRTN rows.
     *
     * <p>Notes:
     * Returns (modified config, the deleted CFG_CFUNC row). Order: CFG_CFBOM rows whose
     * CFCALL_ID belongs to one of the function's CFG_CFCALL rows; every CFG_CFCALL row with
     * that CFUNC_ID; every CFG_CFRTN row with that CFUNC_ID (well-formed rows via
     * thresholds::delete_comparison_threshold, then a sweep of the rest); finally the
     * CFG_CFUNC row. Absent CFBOM/CFCALL/CFRTN sections are skipped. MISSING_FIELD when the
     * found row has no integer CFUNC_ID. A missing CFG_CFUNC section is NOT_FOUND.
     *
     * <p>Wire name: {@code delete_comparison_function_cascade}; group: {@code functions_comparison}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, MISSING_FIELD; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteComparisonFunctionCascadeResult(String configJson, String code) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("code", code);
        return Invoker.result("delete_comparison_function_cascade", "config_and_json", configJson, wire);
    }

    /**
     * Get one comparison function's raw CFG_CFUNC row by code.
     *
     * <p>Notes:
     * Result uses on-disk keys (CFUNC_ID, CFUNC_CODE, CFUNC_DESC, CONNECT_STR, ANON_SUPPORT,
     * LANGUAGE). A missing CFG_CFUNC section is NOT_FOUND (not MISSING_SECTION).
     *
     * <p>Wire name: {@code get_comparison_function}; group: {@code functions_comparison}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String getComparisonFunction(String configJson, String code) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("code", code);
        return Invoker.json("get_comparison_function", configJson, wire);
    }

    /**
     * List all comparison functions as camelCase summaries.
     *
     * <p>Notes:
     * Result is an array of {id, function, description, connectStr, anonSupport, language} in
     * config order; all but function are null-preserving. A missing CFG_CFUNC section yields
     * [] (no error).
     *
     * <p>Wire name: {@code list_comparison_functions}; group: {@code functions_comparison}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String listComparisonFunctions(String configJson) throws SzConfigToolException {
        Args wire = new Args();
        return Invoker.json("list_comparison_functions", configJson, wire);
    }

    /** Optional arguments of {@link #setComparisonFunction}; unset = omitted (library default). */
    public static final class SetComparisonFunctionOptions {
        final Args wire = new Args();

        /**
         * <code>connect_str</code> (str) tri-state: leave / clear / set. Absent leaves
         * CONNECT_STR; null clears it to null; a string (including "") sets it.
         *
         * @param connectStr the value
         * @return this builder
         */
        public SetComparisonFunctionOptions connectStr(FieldUpdate<String> connectStr) {
            wire.strUpdate("connect_str", connectStr);
            return this;
        }

        /**
         * <code>description</code> (str) Absent leaves CFUNC_DESC; a string is stored verbatim.
         * Cannot be cleared to null.
         *
         * @param description the value
         * @return this builder
         */
        public SetComparisonFunctionOptions description(String description) {
            wire.str("description", description);
            return this;
        }

        /**
         * <code>language</code> (str) Absent leaves LANGUAGE; a string is stored verbatim. Cannot
         * be cleared to null.
         *
         * @param language the value
         * @return this builder
         */
        public SetComparisonFunctionOptions language(String language) {
            wire.str("language", language);
            return this;
        }

        /**
         * <code>anon_support</code> (str) Absent leaves ANON_SUPPORT. TRAP: unlike add, NOT
         * validated or normalized; any string is stored verbatim.
         *
         * @param anonSupport the value
         * @return this builder
         */
        public SetComparisonFunctionOptions anonSupport(String anonSupport) {
            wire.str("anon_support", anonSupport);
            return this;
        }
    }

    /**
     * Update a comparison function's connect string / description / language / anon support.
     *
     * <p>Notes:
     * Returns (modified config, the updated CFG_CFUNC row). No value validation. The row is
     * deleted and re-appended, so it moves to the END of CFG_CFUNC. A missing CFG_CFUNC
     * section is NOT_FOUND.
     *
     * <p>Wire name: {@code set_comparison_function}; group: {@code functions_comparison}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @return the modified configuration JSON document (opaque); {@link #setComparisonFunctionResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String setComparisonFunction(String configJson, String code) throws SzConfigToolException {
        return setComparisonFunction(configJson, code, null);
    }

    /**
     * Update a comparison function's connect string / description / language / anon support.
     *
     * <p>Notes:
     * Returns (modified config, the updated CFG_CFUNC row). No value validation. The row is
     * deleted and re-appended, so it moves to the END of CFG_CFUNC. A missing CFG_CFUNC
     * section is NOT_FOUND.
     *
     * <p>Wire name: {@code set_comparison_function}; group: {@code functions_comparison}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @param options optional arguments ({@code null} = none); see {@link SetComparisonFunctionOptions}
     * @return the modified configuration JSON document (opaque); {@link #setComparisonFunctionResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String setComparisonFunction(String configJson, String code, SetComparisonFunctionOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("code", code);
        return Invoker.call("set_comparison_function", "config_and_json", configJson, wire)[1];
    }

    /**
     * The record (row / ids) of <code>setComparisonFunction</code>: same arguments and
     * operation, but returns the record instead of the configuration. Operation: Update a
     * comparison function's connect string / description / language / anon support.
     *
     * <p>Notes:
     * Returns (modified config, the updated CFG_CFUNC row). No value validation. The row is
     * deleted and re-appended, so it moves to the END of CFG_CFUNC. A missing CFG_CFUNC
     * section is NOT_FOUND.
     *
     * <p>Wire name: {@code set_comparison_function}; group: {@code functions_comparison}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String setComparisonFunctionResult(String configJson, String code) throws SzConfigToolException {
        return setComparisonFunctionResult(configJson, code, null);
    }

    /**
     * The record (row / ids) of <code>setComparisonFunction</code>: same arguments and
     * operation, but returns the record instead of the configuration. Operation: Update a
     * comparison function's connect string / description / language / anon support.
     *
     * <p>Notes:
     * Returns (modified config, the updated CFG_CFUNC row). No value validation. The row is
     * deleted and re-appended, so it moves to the END of CFG_CFUNC. A missing CFG_CFUNC
     * section is NOT_FOUND.
     *
     * <p>Wire name: {@code set_comparison_function}; group: {@code functions_comparison}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @param options optional arguments ({@code null} = none); see {@link SetComparisonFunctionOptions}
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String setComparisonFunctionResult(String configJson, String code, SetComparisonFunctionOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("code", code);
        return Invoker.result("set_comparison_function", "config_and_json", configJson, wire);
    }

    /** Optional arguments of {@link #addDistinctFunction}; unset = omitted (library default). */
    public static final class AddDistinctFunctionOptions {
        final Args wire = new Args();

        /**
         * <code>connect_str</code> (str) Absent stores CONNECT_STR null; any string (including "")
         * is stored verbatim.
         *
         * @param connectStr the value
         * @return this builder
         */
        public AddDistinctFunctionOptions connectStr(String connectStr) {
            wire.str("connect_str", connectStr);
            return this;
        }

        /**
         * <code>description</code> (str) Stored verbatim in DFUNC_DESC; absent stores null (NOT
         * defaulted to the code).
         *
         * @param description the value
         * @return this builder
         */
        public AddDistinctFunctionOptions description(String description) {
            wire.str("description", description);
            return this;
        }

        /**
         * <code>language</code> (str) Stored verbatim in LANGUAGE; absent stores null.
         *
         * @param language the value
         * @return this builder
         */
        public AddDistinctFunctionOptions language(String language) {
            wire.str("language", language);
            return this;
        }

        /**
         * <code>anon_support</code> (str) Case-insensitive; normalized to Yes or No, any other
         * value is INVALID_INPUT. Library default when omitted: "No".
         *
         * @param anonSupport the value
         * @return this builder
         */
        public AddDistinctFunctionOptions anonSupport(String anonSupport) {
            wire.str("anon_support", anonSupport);
            return this;
        }
    }

    /**
     * Add a distinct function (CFG_DFUNC row) to the configuration.
     *
     * <p>Notes:
     * Returns (modified config, the new complete CFG_DFUNC row: DFUNC_ID, DFUNC_CODE,
     * DFUNC_DESC, CONNECT_STR, ANON_SUPPORT, LANGUAGE; unset optionals are null). DFUNC_ID is
     * auto-allocated as max existing + 1 (floor 1); no id can be requested. TRAP: a duplicate
     * code is INVALID_INPUT, not ALREADY_EXISTS (SzConfigError::validation). The duplicate
     * check runs before anon_support validation. MISSING_SECTION only when G2_CONFIG.CFG_DFUNC
     * is absent.
     *
     * <p>Wire name: {@code add_distinct_function}; group: {@code functions_distinct}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before the duplicate check and storage (DFUNC_CODE).
     * @return the modified configuration JSON document (opaque); {@link #addDistinctFunctionResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addDistinctFunction(String configJson, String code) throws SzConfigToolException {
        return addDistinctFunction(configJson, code, null);
    }

    /**
     * Add a distinct function (CFG_DFUNC row) to the configuration.
     *
     * <p>Notes:
     * Returns (modified config, the new complete CFG_DFUNC row: DFUNC_ID, DFUNC_CODE,
     * DFUNC_DESC, CONNECT_STR, ANON_SUPPORT, LANGUAGE; unset optionals are null). DFUNC_ID is
     * auto-allocated as max existing + 1 (floor 1); no id can be requested. TRAP: a duplicate
     * code is INVALID_INPUT, not ALREADY_EXISTS (SzConfigError::validation). The duplicate
     * check runs before anon_support validation. MISSING_SECTION only when G2_CONFIG.CFG_DFUNC
     * is absent.
     *
     * <p>Wire name: {@code add_distinct_function}; group: {@code functions_distinct}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before the duplicate check and storage (DFUNC_CODE).
     * @param options optional arguments ({@code null} = none); see {@link AddDistinctFunctionOptions}
     * @return the modified configuration JSON document (opaque); {@link #addDistinctFunctionResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addDistinctFunction(String configJson, String code, AddDistinctFunctionOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("code", code);
        return Invoker.call("add_distinct_function", "config_and_json", configJson, wire)[1];
    }

    /**
     * The record (row / ids) of <code>addDistinctFunction</code>: same arguments and
     * operation, but returns the record instead of the configuration. Operation: Add a
     * distinct function (CFG_DFUNC row) to the configuration.
     *
     * <p>Notes:
     * Returns (modified config, the new complete CFG_DFUNC row: DFUNC_ID, DFUNC_CODE,
     * DFUNC_DESC, CONNECT_STR, ANON_SUPPORT, LANGUAGE; unset optionals are null). DFUNC_ID is
     * auto-allocated as max existing + 1 (floor 1); no id can be requested. TRAP: a duplicate
     * code is INVALID_INPUT, not ALREADY_EXISTS (SzConfigError::validation). The duplicate
     * check runs before anon_support validation. MISSING_SECTION only when G2_CONFIG.CFG_DFUNC
     * is absent.
     *
     * <p>Wire name: {@code add_distinct_function}; group: {@code functions_distinct}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before the duplicate check and storage (DFUNC_CODE).
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addDistinctFunctionResult(String configJson, String code) throws SzConfigToolException {
        return addDistinctFunctionResult(configJson, code, null);
    }

    /**
     * The record (row / ids) of <code>addDistinctFunction</code>: same arguments and
     * operation, but returns the record instead of the configuration. Operation: Add a
     * distinct function (CFG_DFUNC row) to the configuration.
     *
     * <p>Notes:
     * Returns (modified config, the new complete CFG_DFUNC row: DFUNC_ID, DFUNC_CODE,
     * DFUNC_DESC, CONNECT_STR, ANON_SUPPORT, LANGUAGE; unset optionals are null). DFUNC_ID is
     * auto-allocated as max existing + 1 (floor 1); no id can be requested. TRAP: a duplicate
     * code is INVALID_INPUT, not ALREADY_EXISTS (SzConfigError::validation). The duplicate
     * check runs before anon_support validation. MISSING_SECTION only when G2_CONFIG.CFG_DFUNC
     * is absent.
     *
     * <p>Wire name: {@code add_distinct_function}; group: {@code functions_distinct}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before the duplicate check and storage (DFUNC_CODE).
     * @param options optional arguments ({@code null} = none); see {@link AddDistinctFunctionOptions}
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addDistinctFunctionResult(String configJson, String code, AddDistinctFunctionOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("code", code);
        return Invoker.result("add_distinct_function", "config_and_json", configJson, wire);
    }

    /**
     * Delete a distinct function by code.
     *
     * <p>Notes:
     * Returns (modified config, the deleted CFG_DFUNC row). No dependency check: CFG_DFCALL
     * rows referencing the DFUNC_ID are left in place. A config without CFG_DFUNC is NOT_FOUND
     * (not MISSING_SECTION).
     *
     * <p>Wire name: {@code delete_distinct_function}; group: {@code functions_distinct}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @return the modified configuration JSON document (opaque); {@link #deleteDistinctFunctionResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteDistinctFunction(String configJson, String code) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("code", code);
        return Invoker.call("delete_distinct_function", "config_and_json", configJson, wire)[1];
    }

    /**
     * The record (row / ids) of <code>deleteDistinctFunction</code>: same arguments and
     * operation, but returns the record instead of the configuration. Operation: Delete a
     * distinct function by code.
     *
     * <p>Notes:
     * Returns (modified config, the deleted CFG_DFUNC row). No dependency check: CFG_DFCALL
     * rows referencing the DFUNC_ID are left in place. A config without CFG_DFUNC is NOT_FOUND
     * (not MISSING_SECTION).
     *
     * <p>Wire name: {@code delete_distinct_function}; group: {@code functions_distinct}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteDistinctFunctionResult(String configJson, String code) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("code", code);
        return Invoker.result("delete_distinct_function", "config_and_json", configJson, wire);
    }

    /**
     * Get one distinct function's raw CFG_DFUNC row by code.
     *
     * <p>Notes:
     * Result is the stored row with on-disk keys. A config without CFG_DFUNC is NOT_FOUND.
     *
     * <p>Wire name: {@code get_distinct_function}; group: {@code functions_distinct}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String getDistinctFunction(String configJson, String code) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("code", code);
        return Invoker.json("get_distinct_function", configJson, wire);
    }

    /**
     * List all distinct functions as {id, function, connectStr, anonSupport, language}
     * summaries.
     *
     * <p>Notes:
     * Result is an array in config order; id/connectStr/anonSupport/language are
     * null-preserving (stored null stays null), DFUNC_DESC is not included. A config without
     * G2_CONFIG.CFG_DFUNC yields [] (no MISSING_SECTION).
     *
     * <p>Wire name: {@code list_distinct_functions}; group: {@code functions_distinct}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String listDistinctFunctions(String configJson) throws SzConfigToolException {
        Args wire = new Args();
        return Invoker.json("list_distinct_functions", configJson, wire);
    }

    /** Optional arguments of {@link #setDistinctFunction}; unset = omitted (library default). */
    public static final class SetDistinctFunctionOptions {
        final Args wire = new Args();

        /**
         * <code>connect_str</code> (str) tri-state: leave / clear / set. Absent leaves
         * CONNECT_STR; null writes null; a string (including "") is written.
         *
         * @param connectStr the value
         * @return this builder
         */
        public SetDistinctFunctionOptions connectStr(FieldUpdate<String> connectStr) {
            wire.strUpdate("connect_str", connectStr);
            return this;
        }

        /**
         * <code>description</code> (str) Absent leaves DFUNC_DESC; a string is written verbatim.
         *
         * @param description the value
         * @return this builder
         */
        public SetDistinctFunctionOptions description(String description) {
            wire.str("description", description);
            return this;
        }

        /**
         * <code>language</code> (str) Absent leaves LANGUAGE; a string is written verbatim.
         *
         * @param language the value
         * @return this builder
         */
        public SetDistinctFunctionOptions language(String language) {
            wire.str("language", language);
            return this;
        }

        /**
         * <code>anon_support</code> (str) Absent leaves ANON_SUPPORT. TRAP: unlike
         * add_distinct_function the value is written VERBATIM, with no Yes/No validation or case
         * normalization.
         *
         * @param anonSupport the value
         * @return this builder
         */
        public SetDistinctFunctionOptions anonSupport(String anonSupport) {
            wire.str("anon_support", anonSupport);
            return this;
        }
    }

    /**
     * Update a distinct function's connect string, description, language or anon support.
     *
     * <p>Notes:
     * Returns (modified config, the updated CFG_DFUNC row). The row is removed and
     * re-appended, so it moves to the END of CFG_DFUNC (list order changes). A config without
     * CFG_DFUNC is NOT_FOUND.
     *
     * <p>Wire name: {@code set_distinct_function}; group: {@code functions_distinct}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @return the modified configuration JSON document (opaque); {@link #setDistinctFunctionResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String setDistinctFunction(String configJson, String code) throws SzConfigToolException {
        return setDistinctFunction(configJson, code, null);
    }

    /**
     * Update a distinct function's connect string, description, language or anon support.
     *
     * <p>Notes:
     * Returns (modified config, the updated CFG_DFUNC row). The row is removed and
     * re-appended, so it moves to the END of CFG_DFUNC (list order changes). A config without
     * CFG_DFUNC is NOT_FOUND.
     *
     * <p>Wire name: {@code set_distinct_function}; group: {@code functions_distinct}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @param options optional arguments ({@code null} = none); see {@link SetDistinctFunctionOptions}
     * @return the modified configuration JSON document (opaque); {@link #setDistinctFunctionResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String setDistinctFunction(String configJson, String code, SetDistinctFunctionOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("code", code);
        return Invoker.call("set_distinct_function", "config_and_json", configJson, wire)[1];
    }

    /**
     * The record (row / ids) of <code>setDistinctFunction</code>: same arguments and
     * operation, but returns the record instead of the configuration. Operation: Update a
     * distinct function's connect string, description, language or anon support.
     *
     * <p>Notes:
     * Returns (modified config, the updated CFG_DFUNC row). The row is removed and
     * re-appended, so it moves to the END of CFG_DFUNC (list order changes). A config without
     * CFG_DFUNC is NOT_FOUND.
     *
     * <p>Wire name: {@code set_distinct_function}; group: {@code functions_distinct}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String setDistinctFunctionResult(String configJson, String code) throws SzConfigToolException {
        return setDistinctFunctionResult(configJson, code, null);
    }

    /**
     * The record (row / ids) of <code>setDistinctFunction</code>: same arguments and
     * operation, but returns the record instead of the configuration. Operation: Update a
     * distinct function's connect string, description, language or anon support.
     *
     * <p>Notes:
     * Returns (modified config, the updated CFG_DFUNC row). The row is removed and
     * re-appended, so it moves to the END of CFG_DFUNC (list order changes). A config without
     * CFG_DFUNC is NOT_FOUND.
     *
     * <p>Wire name: {@code set_distinct_function}; group: {@code functions_distinct}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @param options optional arguments ({@code null} = none); see {@link SetDistinctFunctionOptions}
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String setDistinctFunctionResult(String configJson, String code, SetDistinctFunctionOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("code", code);
        return Invoker.result("set_distinct_function", "config_and_json", configJson, wire);
    }

    /** Optional arguments of {@link #addExpressionFunction}; unset = omitted (library default). */
    public static final class AddExpressionFunctionOptions {
        final Args wire = new Args();

        /**
         * <code>connect_str</code> (str) Absent stores CONNECT_STR null; any string (including "")
         * is stored verbatim.
         *
         * @param connectStr the value
         * @return this builder
         */
        public AddExpressionFunctionOptions connectStr(String connectStr) {
            wire.str("connect_str", connectStr);
            return this;
        }

        /**
         * <code>description</code> (str) Absent stores EFUNC_DESC null; any string is stored
         * verbatim.
         *
         * @param description the value
         * @return this builder
         */
        public AddExpressionFunctionOptions description(String description) {
            wire.str("description", description);
            return this;
        }

        /**
         * <code>language</code> (str) Absent stores LANGUAGE null; any string is stored verbatim.
         *
         * @param language the value
         * @return this builder
         */
        public AddExpressionFunctionOptions language(String language) {
            wire.str("language", language);
            return this;
        }
    }

    /**
     * Add an expression function (CFG_EFUNC row).
     *
     * <p>Notes:
     * Returns (modified config, the new complete CFG_EFUNC row: EFUNC_ID, EFUNC_CODE,
     * CONNECT_STR, EFUNC_DESC, LANGUAGE). EFUNC_ID is always auto-allocated (max existing + 1,
     * floor 1); no id can be requested. TRAP: a duplicate code is INVALID_INPUT
     * (SzConfigError::validation), NOT ALREADY_EXISTS. MISSING_SECTION only when CFG_EFUNC is
     * absent.
     *
     * <p>Wire name: {@code add_expression_function}; group: {@code functions_expression}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before the duplicate check and storage (EFUNC_CODE).
     * @return the modified configuration JSON document (opaque); {@link #addExpressionFunctionResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addExpressionFunction(String configJson, String code) throws SzConfigToolException {
        return addExpressionFunction(configJson, code, null);
    }

    /**
     * Add an expression function (CFG_EFUNC row).
     *
     * <p>Notes:
     * Returns (modified config, the new complete CFG_EFUNC row: EFUNC_ID, EFUNC_CODE,
     * CONNECT_STR, EFUNC_DESC, LANGUAGE). EFUNC_ID is always auto-allocated (max existing + 1,
     * floor 1); no id can be requested. TRAP: a duplicate code is INVALID_INPUT
     * (SzConfigError::validation), NOT ALREADY_EXISTS. MISSING_SECTION only when CFG_EFUNC is
     * absent.
     *
     * <p>Wire name: {@code add_expression_function}; group: {@code functions_expression}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before the duplicate check and storage (EFUNC_CODE).
     * @param options optional arguments ({@code null} = none); see {@link AddExpressionFunctionOptions}
     * @return the modified configuration JSON document (opaque); {@link #addExpressionFunctionResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addExpressionFunction(String configJson, String code, AddExpressionFunctionOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("code", code);
        return Invoker.call("add_expression_function", "config_and_json", configJson, wire)[1];
    }

    /**
     * The record (row / ids) of <code>addExpressionFunction</code>: same arguments and
     * operation, but returns the record instead of the configuration. Operation: Add an
     * expression function (CFG_EFUNC row).
     *
     * <p>Notes:
     * Returns (modified config, the new complete CFG_EFUNC row: EFUNC_ID, EFUNC_CODE,
     * CONNECT_STR, EFUNC_DESC, LANGUAGE). EFUNC_ID is always auto-allocated (max existing + 1,
     * floor 1); no id can be requested. TRAP: a duplicate code is INVALID_INPUT
     * (SzConfigError::validation), NOT ALREADY_EXISTS. MISSING_SECTION only when CFG_EFUNC is
     * absent.
     *
     * <p>Wire name: {@code add_expression_function}; group: {@code functions_expression}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before the duplicate check and storage (EFUNC_CODE).
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addExpressionFunctionResult(String configJson, String code) throws SzConfigToolException {
        return addExpressionFunctionResult(configJson, code, null);
    }

    /**
     * The record (row / ids) of <code>addExpressionFunction</code>: same arguments and
     * operation, but returns the record instead of the configuration. Operation: Add an
     * expression function (CFG_EFUNC row).
     *
     * <p>Notes:
     * Returns (modified config, the new complete CFG_EFUNC row: EFUNC_ID, EFUNC_CODE,
     * CONNECT_STR, EFUNC_DESC, LANGUAGE). EFUNC_ID is always auto-allocated (max existing + 1,
     * floor 1); no id can be requested. TRAP: a duplicate code is INVALID_INPUT
     * (SzConfigError::validation), NOT ALREADY_EXISTS. MISSING_SECTION only when CFG_EFUNC is
     * absent.
     *
     * <p>Wire name: {@code add_expression_function}; group: {@code functions_expression}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before the duplicate check and storage (EFUNC_CODE).
     * @param options optional arguments ({@code null} = none); see {@link AddExpressionFunctionOptions}
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addExpressionFunctionResult(String configJson, String code, AddExpressionFunctionOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("code", code);
        return Invoker.result("add_expression_function", "config_and_json", configJson, wire);
    }

    /**
     * Delete an expression function's CFG_EFUNC row only (no cascade).
     *
     * <p>Notes:
     * Returns (modified config, the deleted CFG_EFUNC row). Removes ONLY the CFG_EFUNC row;
     * CFG_EFCALL rows referencing it are left dangling (use
     * delete_expression_function_cascade). A missing CFG_EFUNC section is NOT_FOUND (not
     * MISSING_SECTION).
     *
     * <p>Wire name: {@code delete_expression_function}; group: {@code functions_expression}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @return the modified configuration JSON document (opaque); {@link #deleteExpressionFunctionResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteExpressionFunction(String configJson, String code) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("code", code);
        return Invoker.call("delete_expression_function", "config_and_json", configJson, wire)[1];
    }

    /**
     * The record (row / ids) of <code>deleteExpressionFunction</code>: same arguments and
     * operation, but returns the record instead of the configuration. Operation: Delete an
     * expression function's CFG_EFUNC row only (no cascade).
     *
     * <p>Notes:
     * Returns (modified config, the deleted CFG_EFUNC row). Removes ONLY the CFG_EFUNC row;
     * CFG_EFCALL rows referencing it are left dangling (use
     * delete_expression_function_cascade). A missing CFG_EFUNC section is NOT_FOUND (not
     * MISSING_SECTION).
     *
     * <p>Wire name: {@code delete_expression_function}; group: {@code functions_expression}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteExpressionFunctionResult(String configJson, String code) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("code", code);
        return Invoker.result("delete_expression_function", "config_and_json", configJson, wire);
    }

    /**
     * Delete an expression function and its CFG_EFCALL / CFG_EFBOM rows.
     *
     * <p>Notes:
     * Returns (modified config, the deleted CFG_EFUNC row). Removes the CFG_EFBOM rows whose
     * EFCALL_ID belongs to one of the function's CFG_EFCALL rows, then every CFG_EFCALL row
     * whose EFUNC_ID matches (each step skipped if its section is absent), then the CFG_EFUNC
     * row. MISSING_FIELD when the found row has no integer EFUNC_ID. A missing CFG_EFUNC
     * section is NOT_FOUND.
     *
     * <p>Wire name: {@code delete_expression_function_cascade}; group: {@code functions_expression}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @return the modified configuration JSON document (opaque); {@link #deleteExpressionFunctionCascadeResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, MISSING_FIELD; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteExpressionFunctionCascade(String configJson, String code) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("code", code);
        return Invoker.call("delete_expression_function_cascade", "config_and_json", configJson, wire)[1];
    }

    /**
     * The record (row / ids) of <code>deleteExpressionFunctionCascade</code>: same arguments
     * and operation, but returns the record instead of the configuration. Operation: Delete an
     * expression function and its CFG_EFCALL / CFG_EFBOM rows.
     *
     * <p>Notes:
     * Returns (modified config, the deleted CFG_EFUNC row). Removes the CFG_EFBOM rows whose
     * EFCALL_ID belongs to one of the function's CFG_EFCALL rows, then every CFG_EFCALL row
     * whose EFUNC_ID matches (each step skipped if its section is absent), then the CFG_EFUNC
     * row. MISSING_FIELD when the found row has no integer EFUNC_ID. A missing CFG_EFUNC
     * section is NOT_FOUND.
     *
     * <p>Wire name: {@code delete_expression_function_cascade}; group: {@code functions_expression}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, MISSING_FIELD; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteExpressionFunctionCascadeResult(String configJson, String code) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("code", code);
        return Invoker.result("delete_expression_function_cascade", "config_and_json", configJson, wire);
    }

    /**
     * Get one expression function's raw CFG_EFUNC row by code.
     *
     * <p>Notes:
     * Result uses on-disk keys (EFUNC_ID, EFUNC_CODE, EFUNC_DESC, CONNECT_STR, LANGUAGE). A
     * missing CFG_EFUNC section is NOT_FOUND (not MISSING_SECTION).
     *
     * <p>Wire name: {@code get_expression_function}; group: {@code functions_expression}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String getExpressionFunction(String configJson, String code) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("code", code);
        return Invoker.json("get_expression_function", configJson, wire);
    }

    /**
     * List all expression functions as camelCase summaries.
     *
     * <p>Notes:
     * Result is an array of {id, function, connectStr, language} in config order (description
     * is NOT included); connectStr/language are null-preserving. A missing CFG_EFUNC section
     * yields [] (no error).
     *
     * <p>Wire name: {@code list_expression_functions}; group: {@code functions_expression}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String listExpressionFunctions(String configJson) throws SzConfigToolException {
        Args wire = new Args();
        return Invoker.json("list_expression_functions", configJson, wire);
    }

    /** Optional arguments of {@link #setExpressionFunction}; unset = omitted (library default). */
    public static final class SetExpressionFunctionOptions {
        final Args wire = new Args();

        /**
         * <code>connect_str</code> (str) tri-state: leave / clear / set. Absent leaves
         * CONNECT_STR; null clears it to null; a string (including "") sets it.
         *
         * @param connectStr the value
         * @return this builder
         */
        public SetExpressionFunctionOptions connectStr(FieldUpdate<String> connectStr) {
            wire.strUpdate("connect_str", connectStr);
            return this;
        }

        /**
         * <code>description</code> (str) Absent leaves EFUNC_DESC; a string is stored verbatim.
         * Cannot be cleared to null.
         *
         * @param description the value
         * @return this builder
         */
        public SetExpressionFunctionOptions description(String description) {
            wire.str("description", description);
            return this;
        }

        /**
         * <code>language</code> (str) Absent leaves LANGUAGE; a string is stored verbatim. Cannot
         * be cleared to null.
         *
         * @param language the value
         * @return this builder
         */
        public SetExpressionFunctionOptions language(String language) {
            wire.str("language", language);
            return this;
        }
    }

    /**
     * Update an expression function's connect string / description / language.
     *
     * <p>Notes:
     * Returns (modified config, the updated CFG_EFUNC row). No value validation. The row is
     * deleted and re-appended, so it moves to the END of CFG_EFUNC. A missing CFG_EFUNC
     * section is NOT_FOUND.
     *
     * <p>Wire name: {@code set_expression_function}; group: {@code functions_expression}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @return the modified configuration JSON document (opaque); {@link #setExpressionFunctionResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String setExpressionFunction(String configJson, String code) throws SzConfigToolException {
        return setExpressionFunction(configJson, code, null);
    }

    /**
     * Update an expression function's connect string / description / language.
     *
     * <p>Notes:
     * Returns (modified config, the updated CFG_EFUNC row). No value validation. The row is
     * deleted and re-appended, so it moves to the END of CFG_EFUNC. A missing CFG_EFUNC
     * section is NOT_FOUND.
     *
     * <p>Wire name: {@code set_expression_function}; group: {@code functions_expression}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @param options optional arguments ({@code null} = none); see {@link SetExpressionFunctionOptions}
     * @return the modified configuration JSON document (opaque); {@link #setExpressionFunctionResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String setExpressionFunction(String configJson, String code, SetExpressionFunctionOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("code", code);
        return Invoker.call("set_expression_function", "config_and_json", configJson, wire)[1];
    }

    /**
     * The record (row / ids) of <code>setExpressionFunction</code>: same arguments and
     * operation, but returns the record instead of the configuration. Operation: Update an
     * expression function's connect string / description / language.
     *
     * <p>Notes:
     * Returns (modified config, the updated CFG_EFUNC row). No value validation. The row is
     * deleted and re-appended, so it moves to the END of CFG_EFUNC. A missing CFG_EFUNC
     * section is NOT_FOUND.
     *
     * <p>Wire name: {@code set_expression_function}; group: {@code functions_expression}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String setExpressionFunctionResult(String configJson, String code) throws SzConfigToolException {
        return setExpressionFunctionResult(configJson, code, null);
    }

    /**
     * The record (row / ids) of <code>setExpressionFunction</code>: same arguments and
     * operation, but returns the record instead of the configuration. Operation: Update an
     * expression function's connect string / description / language.
     *
     * <p>Notes:
     * Returns (modified config, the updated CFG_EFUNC row). No value validation. The row is
     * deleted and re-appended, so it moves to the END of CFG_EFUNC. A missing CFG_EFUNC
     * section is NOT_FOUND.
     *
     * <p>Wire name: {@code set_expression_function}; group: {@code functions_expression}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @param options optional arguments ({@code null} = none); see {@link SetExpressionFunctionOptions}
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String setExpressionFunctionResult(String configJson, String code, SetExpressionFunctionOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("code", code);
        return Invoker.result("set_expression_function", "config_and_json", configJson, wire);
    }

    /** Optional arguments of {@link #addStandardizeFunction}; unset = omitted (library default). */
    public static final class AddStandardizeFunctionOptions {
        final Args wire = new Args();

        /**
         * <code>connect_str</code> (str) Absent stores CONNECT_STR null; any string (including "")
         * is stored verbatim.
         *
         * @param connectStr the value
         * @return this builder
         */
        public AddStandardizeFunctionOptions connectStr(String connectStr) {
            wire.str("connect_str", connectStr);
            return this;
        }

        /**
         * <code>description</code> (str) Absent stores SFUNC_DESC null; any string is stored
         * verbatim.
         *
         * @param description the value
         * @return this builder
         */
        public AddStandardizeFunctionOptions description(String description) {
            wire.str("description", description);
            return this;
        }

        /**
         * <code>language</code> (str) Absent stores LANGUAGE null; any string is stored verbatim.
         *
         * @param language the value
         * @return this builder
         */
        public AddStandardizeFunctionOptions language(String language) {
            wire.str("language", language);
            return this;
        }
    }

    /**
     * Add a standardize function (CFG_SFUNC row).
     *
     * <p>Notes:
     * Returns (modified config, the new complete CFG_SFUNC row: SFUNC_ID, SFUNC_CODE,
     * CONNECT_STR, SFUNC_DESC, LANGUAGE). SFUNC_ID is always auto-allocated (max existing + 1,
     * floor 1); no id can be requested. TRAP: a duplicate code is INVALID_INPUT
     * (SzConfigError::validation), NOT ALREADY_EXISTS. MISSING_SECTION only when CFG_SFUNC is
     * absent.
     *
     * <p>Wire name: {@code add_standardize_function}; group: {@code functions_standardize}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before the duplicate check and storage (SFUNC_CODE).
     * @return the modified configuration JSON document (opaque); {@link #addStandardizeFunctionResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addStandardizeFunction(String configJson, String code) throws SzConfigToolException {
        return addStandardizeFunction(configJson, code, null);
    }

    /**
     * Add a standardize function (CFG_SFUNC row).
     *
     * <p>Notes:
     * Returns (modified config, the new complete CFG_SFUNC row: SFUNC_ID, SFUNC_CODE,
     * CONNECT_STR, SFUNC_DESC, LANGUAGE). SFUNC_ID is always auto-allocated (max existing + 1,
     * floor 1); no id can be requested. TRAP: a duplicate code is INVALID_INPUT
     * (SzConfigError::validation), NOT ALREADY_EXISTS. MISSING_SECTION only when CFG_SFUNC is
     * absent.
     *
     * <p>Wire name: {@code add_standardize_function}; group: {@code functions_standardize}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before the duplicate check and storage (SFUNC_CODE).
     * @param options optional arguments ({@code null} = none); see {@link AddStandardizeFunctionOptions}
     * @return the modified configuration JSON document (opaque); {@link #addStandardizeFunctionResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addStandardizeFunction(String configJson, String code, AddStandardizeFunctionOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("code", code);
        return Invoker.call("add_standardize_function", "config_and_json", configJson, wire)[1];
    }

    /**
     * The record (row / ids) of <code>addStandardizeFunction</code>: same arguments and
     * operation, but returns the record instead of the configuration. Operation: Add a
     * standardize function (CFG_SFUNC row).
     *
     * <p>Notes:
     * Returns (modified config, the new complete CFG_SFUNC row: SFUNC_ID, SFUNC_CODE,
     * CONNECT_STR, SFUNC_DESC, LANGUAGE). SFUNC_ID is always auto-allocated (max existing + 1,
     * floor 1); no id can be requested. TRAP: a duplicate code is INVALID_INPUT
     * (SzConfigError::validation), NOT ALREADY_EXISTS. MISSING_SECTION only when CFG_SFUNC is
     * absent.
     *
     * <p>Wire name: {@code add_standardize_function}; group: {@code functions_standardize}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before the duplicate check and storage (SFUNC_CODE).
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addStandardizeFunctionResult(String configJson, String code) throws SzConfigToolException {
        return addStandardizeFunctionResult(configJson, code, null);
    }

    /**
     * The record (row / ids) of <code>addStandardizeFunction</code>: same arguments and
     * operation, but returns the record instead of the configuration. Operation: Add a
     * standardize function (CFG_SFUNC row).
     *
     * <p>Notes:
     * Returns (modified config, the new complete CFG_SFUNC row: SFUNC_ID, SFUNC_CODE,
     * CONNECT_STR, SFUNC_DESC, LANGUAGE). SFUNC_ID is always auto-allocated (max existing + 1,
     * floor 1); no id can be requested. TRAP: a duplicate code is INVALID_INPUT
     * (SzConfigError::validation), NOT ALREADY_EXISTS. MISSING_SECTION only when CFG_SFUNC is
     * absent.
     *
     * <p>Wire name: {@code add_standardize_function}; group: {@code functions_standardize}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before the duplicate check and storage (SFUNC_CODE).
     * @param options optional arguments ({@code null} = none); see {@link AddStandardizeFunctionOptions}
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addStandardizeFunctionResult(String configJson, String code, AddStandardizeFunctionOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("code", code);
        return Invoker.result("add_standardize_function", "config_and_json", configJson, wire);
    }

    /**
     * Delete a standardize function's CFG_SFUNC row only (no cascade).
     *
     * <p>Notes:
     * Returns (modified config, the deleted CFG_SFUNC row). Removes ONLY the CFG_SFUNC row;
     * CFG_SFCALL rows referencing it are left dangling (use
     * delete_standardize_function_cascade). A missing CFG_SFUNC section is NOT_FOUND (not
     * MISSING_SECTION).
     *
     * <p>Wire name: {@code delete_standardize_function}; group: {@code functions_standardize}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @return the modified configuration JSON document (opaque); {@link #deleteStandardizeFunctionResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteStandardizeFunction(String configJson, String code) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("code", code);
        return Invoker.call("delete_standardize_function", "config_and_json", configJson, wire)[1];
    }

    /**
     * The record (row / ids) of <code>deleteStandardizeFunction</code>: same arguments and
     * operation, but returns the record instead of the configuration. Operation: Delete a
     * standardize function's CFG_SFUNC row only (no cascade).
     *
     * <p>Notes:
     * Returns (modified config, the deleted CFG_SFUNC row). Removes ONLY the CFG_SFUNC row;
     * CFG_SFCALL rows referencing it are left dangling (use
     * delete_standardize_function_cascade). A missing CFG_SFUNC section is NOT_FOUND (not
     * MISSING_SECTION).
     *
     * <p>Wire name: {@code delete_standardize_function}; group: {@code functions_standardize}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteStandardizeFunctionResult(String configJson, String code) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("code", code);
        return Invoker.result("delete_standardize_function", "config_and_json", configJson, wire);
    }

    /**
     * Delete a standardize function and its CFG_SFCALL rows.
     *
     * <p>Notes:
     * Returns (modified config, the deleted CFG_SFUNC row). Removes every CFG_SFCALL row whose
     * SFUNC_ID matches (skipped if CFG_SFCALL is absent), then the CFG_SFUNC row; no other
     * section is touched. MISSING_FIELD when the found row has no integer SFUNC_ID. A missing
     * CFG_SFUNC section is NOT_FOUND.
     *
     * <p>Wire name: {@code delete_standardize_function_cascade}; group: {@code functions_standardize}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @return the modified configuration JSON document (opaque); {@link #deleteStandardizeFunctionCascadeResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, MISSING_FIELD; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteStandardizeFunctionCascade(String configJson, String code) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("code", code);
        return Invoker.call("delete_standardize_function_cascade", "config_and_json", configJson, wire)[1];
    }

    /**
     * The record (row / ids) of <code>deleteStandardizeFunctionCascade</code>: same arguments
     * and operation, but returns the record instead of the configuration. Operation: Delete a
     * standardize function and its CFG_SFCALL rows.
     *
     * <p>Notes:
     * Returns (modified config, the deleted CFG_SFUNC row). Removes every CFG_SFCALL row whose
     * SFUNC_ID matches (skipped if CFG_SFCALL is absent), then the CFG_SFUNC row; no other
     * section is touched. MISSING_FIELD when the found row has no integer SFUNC_ID. A missing
     * CFG_SFUNC section is NOT_FOUND.
     *
     * <p>Wire name: {@code delete_standardize_function_cascade}; group: {@code functions_standardize}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, MISSING_FIELD; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteStandardizeFunctionCascadeResult(String configJson, String code) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("code", code);
        return Invoker.result("delete_standardize_function_cascade", "config_and_json", configJson, wire);
    }

    /**
     * Get one standardize function's raw CFG_SFUNC row by code.
     *
     * <p>Notes:
     * Result uses on-disk keys (SFUNC_ID, SFUNC_CODE, SFUNC_DESC, CONNECT_STR, LANGUAGE). A
     * missing CFG_SFUNC section is NOT_FOUND (not MISSING_SECTION).
     *
     * <p>Wire name: {@code get_standardize_function}; group: {@code functions_standardize}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String getStandardizeFunction(String configJson, String code) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("code", code);
        return Invoker.json("get_standardize_function", configJson, wire);
    }

    /**
     * List all standardize functions as camelCase summaries.
     *
     * <p>Notes:
     * Result is an array of {id, function, connectStr, language} in config order (description
     * is NOT included); connectStr/language are null-preserving. A missing CFG_SFUNC section
     * yields [] (no error).
     *
     * <p>Wire name: {@code list_standardize_functions}; group: {@code functions_standardize}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String listStandardizeFunctions(String configJson) throws SzConfigToolException {
        Args wire = new Args();
        return Invoker.json("list_standardize_functions", configJson, wire);
    }

    /** Optional arguments of {@link #setStandardizeFunction}; unset = omitted (library default). */
    public static final class SetStandardizeFunctionOptions {
        final Args wire = new Args();

        /**
         * <code>connect_str</code> (str) tri-state: leave / clear / set. Absent leaves
         * CONNECT_STR; null clears it to null; a string (including "") sets it.
         *
         * @param connectStr the value
         * @return this builder
         */
        public SetStandardizeFunctionOptions connectStr(FieldUpdate<String> connectStr) {
            wire.strUpdate("connect_str", connectStr);
            return this;
        }

        /**
         * <code>description</code> (str) Absent leaves SFUNC_DESC; a string is stored verbatim.
         * Cannot be cleared to null.
         *
         * @param description the value
         * @return this builder
         */
        public SetStandardizeFunctionOptions description(String description) {
            wire.str("description", description);
            return this;
        }

        /**
         * <code>language</code> (str) Absent leaves LANGUAGE; a string is stored verbatim. Cannot
         * be cleared to null.
         *
         * @param language the value
         * @return this builder
         */
        public SetStandardizeFunctionOptions language(String language) {
            wire.str("language", language);
            return this;
        }
    }

    /**
     * Update a standardize function's connect string / description / language.
     *
     * <p>Notes:
     * Returns (modified config, the updated CFG_SFUNC row). No value validation. The row is
     * deleted and re-appended, so it moves to the END of CFG_SFUNC. A missing CFG_SFUNC
     * section is NOT_FOUND.
     *
     * <p>Wire name: {@code set_standardize_function}; group: {@code functions_standardize}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @return the modified configuration JSON document (opaque); {@link #setStandardizeFunctionResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String setStandardizeFunction(String configJson, String code) throws SzConfigToolException {
        return setStandardizeFunction(configJson, code, null);
    }

    /**
     * Update a standardize function's connect string / description / language.
     *
     * <p>Notes:
     * Returns (modified config, the updated CFG_SFUNC row). No value validation. The row is
     * deleted and re-appended, so it moves to the END of CFG_SFUNC. A missing CFG_SFUNC
     * section is NOT_FOUND.
     *
     * <p>Wire name: {@code set_standardize_function}; group: {@code functions_standardize}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @param options optional arguments ({@code null} = none); see {@link SetStandardizeFunctionOptions}
     * @return the modified configuration JSON document (opaque); {@link #setStandardizeFunctionResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String setStandardizeFunction(String configJson, String code, SetStandardizeFunctionOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("code", code);
        return Invoker.call("set_standardize_function", "config_and_json", configJson, wire)[1];
    }

    /**
     * The record (row / ids) of <code>setStandardizeFunction</code>: same arguments and
     * operation, but returns the record instead of the configuration. Operation: Update a
     * standardize function's connect string / description / language.
     *
     * <p>Notes:
     * Returns (modified config, the updated CFG_SFUNC row). No value validation. The row is
     * deleted and re-appended, so it moves to the END of CFG_SFUNC. A missing CFG_SFUNC
     * section is NOT_FOUND.
     *
     * <p>Wire name: {@code set_standardize_function}; group: {@code functions_standardize}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String setStandardizeFunctionResult(String configJson, String code) throws SzConfigToolException {
        return setStandardizeFunctionResult(configJson, code, null);
    }

    /**
     * The record (row / ids) of <code>setStandardizeFunction</code>: same arguments and
     * operation, but returns the record instead of the configuration. Operation: Update a
     * standardize function's connect string / description / language.
     *
     * <p>Notes:
     * Returns (modified config, the updated CFG_SFUNC row). No value validation. The row is
     * deleted and re-appended, so it moves to the END of CFG_SFUNC. A missing CFG_SFUNC
     * section is NOT_FOUND.
     *
     * <p>Wire name: {@code set_standardize_function}; group: {@code functions_standardize}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased before lookup.
     * @param options optional arguments ({@code null} = none); see {@link SetStandardizeFunctionOptions}
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String setStandardizeFunctionResult(String configJson, String code, SetStandardizeFunctionOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("code", code);
        return Invoker.result("set_standardize_function", "config_and_json", configJson, wire);
    }

    /** Optional arguments of {@link #cloneGenericPlan}; unset = omitted (library default). */
    public static final class CloneGenericPlanOptions {
        final Args wire = new Args();

        /**
         * <code>new_gplan_desc</code> (str) Stored verbatim in GPLAN_DESC; absent = the uppercased
         * new code.
         *
         * @param newGplanDesc the value
         * @return this builder
         */
        public CloneGenericPlanOptions newGplanDesc(String newGplanDesc) {
            wire.str("new_gplan_desc", newGplanDesc);
            return this;
        }
    }

    /**
     * Clone a generic plan, copying every CFG_GENERIC_THRESHOLD row of the source to the new
     * plan.
     *
     * <p>Notes:
     * Returns (modified config, new GPLAN_ID); the record is the integer id. The new id is
     * always max existing GPLAN_ID + 1 (no floor, no id arg). Cloned threshold rows are
     * verbatim copies with GPLAN_ID rewritten, appended after existing rows; an absent
     * CFG_GENERIC_THRESHOLD section is skipped silently. INVALID_CONFIG when the source row's
     * GPLAN_ID is not an integer.
     *
     * <p>Wire name: {@code clone_generic_plan}; group: {@code generic_plans}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param sourceGplanCode <code>source_gplan_code</code> (str) Uppercased, then matched exactly against
     * GPLAN_CODE; unknown = NOT_FOUND.
     * @param newGplanCode <code>new_gplan_code</code> (str) Uppercased before the duplicate check and storage; an
     * existing code = ALREADY_EXISTS.
     * @return the modified configuration JSON document (opaque); {@link #cloneGenericPlanResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, ALREADY_EXISTS, INVALID_CONFIG; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String cloneGenericPlan(String configJson, String sourceGplanCode, String newGplanCode) throws SzConfigToolException {
        return cloneGenericPlan(configJson, sourceGplanCode, newGplanCode, null);
    }

    /**
     * Clone a generic plan, copying every CFG_GENERIC_THRESHOLD row of the source to the new
     * plan.
     *
     * <p>Notes:
     * Returns (modified config, new GPLAN_ID); the record is the integer id. The new id is
     * always max existing GPLAN_ID + 1 (no floor, no id arg). Cloned threshold rows are
     * verbatim copies with GPLAN_ID rewritten, appended after existing rows; an absent
     * CFG_GENERIC_THRESHOLD section is skipped silently. INVALID_CONFIG when the source row's
     * GPLAN_ID is not an integer.
     *
     * <p>Wire name: {@code clone_generic_plan}; group: {@code generic_plans}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param sourceGplanCode <code>source_gplan_code</code> (str) Uppercased, then matched exactly against
     * GPLAN_CODE; unknown = NOT_FOUND.
     * @param newGplanCode <code>new_gplan_code</code> (str) Uppercased before the duplicate check and storage; an
     * existing code = ALREADY_EXISTS.
     * @param options optional arguments ({@code null} = none); see {@link CloneGenericPlanOptions}
     * @return the modified configuration JSON document (opaque); {@link #cloneGenericPlanResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, ALREADY_EXISTS, INVALID_CONFIG; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String cloneGenericPlan(String configJson, String sourceGplanCode, String newGplanCode, CloneGenericPlanOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("source_gplan_code", sourceGplanCode);
        wire.str("new_gplan_code", newGplanCode);
        return Invoker.call("clone_generic_plan", "config_and_json", configJson, wire)[1];
    }

    /**
     * The record (row / ids) of <code>cloneGenericPlan</code>: same arguments and operation,
     * but returns the record instead of the configuration. Operation: Clone a generic plan,
     * copying every CFG_GENERIC_THRESHOLD row of the source to the new plan.
     *
     * <p>Notes:
     * Returns (modified config, new GPLAN_ID); the record is the integer id. The new id is
     * always max existing GPLAN_ID + 1 (no floor, no id arg). Cloned threshold rows are
     * verbatim copies with GPLAN_ID rewritten, appended after existing rows; an absent
     * CFG_GENERIC_THRESHOLD section is skipped silently. INVALID_CONFIG when the source row's
     * GPLAN_ID is not an integer.
     *
     * <p>Wire name: {@code clone_generic_plan}; group: {@code generic_plans}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param sourceGplanCode <code>source_gplan_code</code> (str) Uppercased, then matched exactly against
     * GPLAN_CODE; unknown = NOT_FOUND.
     * @param newGplanCode <code>new_gplan_code</code> (str) Uppercased before the duplicate check and storage; an
     * existing code = ALREADY_EXISTS.
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, ALREADY_EXISTS, INVALID_CONFIG; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String cloneGenericPlanResult(String configJson, String sourceGplanCode, String newGplanCode) throws SzConfigToolException {
        return cloneGenericPlanResult(configJson, sourceGplanCode, newGplanCode, null);
    }

    /**
     * The record (row / ids) of <code>cloneGenericPlan</code>: same arguments and operation,
     * but returns the record instead of the configuration. Operation: Clone a generic plan,
     * copying every CFG_GENERIC_THRESHOLD row of the source to the new plan.
     *
     * <p>Notes:
     * Returns (modified config, new GPLAN_ID); the record is the integer id. The new id is
     * always max existing GPLAN_ID + 1 (no floor, no id arg). Cloned threshold rows are
     * verbatim copies with GPLAN_ID rewritten, appended after existing rows; an absent
     * CFG_GENERIC_THRESHOLD section is skipped silently. INVALID_CONFIG when the source row's
     * GPLAN_ID is not an integer.
     *
     * <p>Wire name: {@code clone_generic_plan}; group: {@code generic_plans}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param sourceGplanCode <code>source_gplan_code</code> (str) Uppercased, then matched exactly against
     * GPLAN_CODE; unknown = NOT_FOUND.
     * @param newGplanCode <code>new_gplan_code</code> (str) Uppercased before the duplicate check and storage; an
     * existing code = ALREADY_EXISTS.
     * @param options optional arguments ({@code null} = none); see {@link CloneGenericPlanOptions}
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, ALREADY_EXISTS, INVALID_CONFIG; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String cloneGenericPlanResult(String configJson, String sourceGplanCode, String newGplanCode, CloneGenericPlanOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("source_gplan_code", sourceGplanCode);
        wire.str("new_gplan_code", newGplanCode);
        return Invoker.result("clone_generic_plan", "config_and_json", configJson, wire);
    }

    /**
     * Delete a generic plan and all of its generic thresholds.
     *
     * <p>Notes:
     * System plans (GPLAN_ID &lt;= 2, i.e. INGEST and SEARCH in the template) are protected:
     * INVALID_INPUT. Removes the CFG_GPLAN row and every CFG_GENERIC_THRESHOLD row with that
     * GPLAN_ID. An absent CFG_GPLAN section is NOT_FOUND (not MISSING_SECTION).
     *
     * <p>Wire name: {@code delete_generic_plan}; group: {@code generic_plans}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param gplanCode <code>gplan_code</code> (str) Uppercased, then matched exactly against GPLAN_CODE;
     * unknown = NOT_FOUND.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT, INVALID_CONFIG; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteGenericPlan(String configJson, String gplanCode) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("gplan_code", gplanCode);
        return Invoker.config("delete_generic_plan", configJson, wire);
    }

    /** Optional arguments of {@link #listGenericPlans}; unset = omitted (library default). */
    public static final class ListGenericPlansOptions {
        final Args wire = new Args();

        /**
         * <code>filter</code> (str) Case-insensitive SUBSTRING match against the raw row
         * serialized as JSON text — keys and numbers included (so e.g. "gplan" matches every
         * row). Absent = no filtering.
         *
         * @param filter the value
         * @return this builder
         */
        public ListGenericPlansOptions filter(String filter) {
            wire.str("filter", filter);
            return this;
        }
    }

    /**
     * List generic plans as {id, plan, description}, optionally filtered.
     *
     * <p>Notes:
     * Array of {id (GPLAN_ID), plan (GPLAN_CODE), description (GPLAN_DESC)} sorted by id;
     * missing values become 0 / "". An absent CFG_GPLAN (or G2_CONFIG) yields an empty array,
     * never MISSING_SECTION.
     *
     * <p>Wire name: {@code list_generic_plans}; group: {@code generic_plans}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String listGenericPlans(String configJson) throws SzConfigToolException {
        return listGenericPlans(configJson, null);
    }

    /**
     * List generic plans as {id, plan, description}, optionally filtered.
     *
     * <p>Notes:
     * Array of {id (GPLAN_ID), plan (GPLAN_CODE), description (GPLAN_DESC)} sorted by id;
     * missing values become 0 / "". An absent CFG_GPLAN (or G2_CONFIG) yields an empty array,
     * never MISSING_SECTION.
     *
     * <p>Wire name: {@code list_generic_plans}; group: {@code generic_plans}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param options optional arguments ({@code null} = none); see {@link ListGenericPlansOptions}
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String listGenericPlans(String configJson, ListGenericPlansOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        return Invoker.json("list_generic_plans", configJson, wire);
    }

    /**
     * Result of {@link #setGenericPlanResult} (named by the manifest's {@code tuple_names}).
     *
     * @param planId the {@code plan_id} value as JSON text
     * @param wasCreated the {@code was_created} value as JSON text
     */
    public record SetGenericPlanRecord(String planId, String wasCreated) {
    }

    /**
     * Create a generic plan, or update the description of an existing one (upsert).
     *
     * <p>Notes:
     * Returns (config, {plan_id, was_created}). Existing code: only GPLAN_DESC is replaced
     * (other keys kept), was_created false. New code: a row with GPLAN_ID = max + 1 is
     * appended, was_created true; an absent CFG_GPLAN section is MISSING_SECTION on this
     * create path.
     *
     * <p>Wire name: {@code set_generic_plan}; group: {@code generic_plans}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param gplanCode <code>gplan_code</code> (str) Uppercased, then matched exactly against GPLAN_CODE.
     * @param gplanDesc <code>gplan_desc</code> (str) Written verbatim to GPLAN_DESC.
     * @return the modified configuration JSON document (opaque); {@link #setGenericPlanResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String setGenericPlan(String configJson, String gplanCode, String gplanDesc) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("gplan_code", gplanCode);
        wire.str("gplan_desc", gplanDesc);
        return Invoker.call("set_generic_plan", "config_and_json", configJson, wire)[1];
    }

    /**
     * The record (row / ids) of <code>setGenericPlan</code>: same arguments and operation, but
     * returns the record instead of the configuration. Operation: Create a generic plan, or
     * update the description of an existing one (upsert).
     *
     * <p>Notes:
     * Returns (config, {plan_id, was_created}). Existing code: only GPLAN_DESC is replaced
     * (other keys kept), was_created false. New code: a row with GPLAN_ID = max + 1 is
     * appended, was_created true; an absent CFG_GPLAN section is MISSING_SECTION on this
     * create path.
     *
     * <p>Wire name: {@code set_generic_plan}; group: {@code generic_plans}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param gplanCode <code>gplan_code</code> (str) Uppercased, then matched exactly against GPLAN_CODE.
     * @param gplanDesc <code>gplan_desc</code> (str) Written verbatim to GPLAN_DESC.
     * @return the named result values, each as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static SetGenericPlanRecord setGenericPlanResult(String configJson, String gplanCode, String gplanDesc) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("gplan_code", gplanCode);
        wire.str("gplan_desc", gplanDesc);
        String[] f = Invoker.fields(Invoker.result("set_generic_plan", "config_and_json", configJson, wire), "plan_id", "was_created");
        return new SetGenericPlanRecord(f[0], f[1]);
    }

    /**
     * Add an entity resolution rule (CFG_ERRULE row), returning the assigned ERRULE_ID.
     *
     * <p>Notes:
     * Record is the assigned ERRULE_ID (integer). The written row always carries every
     * CFG_ERRULE key (ERRULE_ID, ERRULE_CODE, RESOLVE, RELATE, RTYPE_ID, QUAL_ERFRAG_CODE,
     * DISQ_ERFRAG_CODE, ERRULE_TIER; optional ones as null). ERRULE_CODE is checked BEFORE the
     * config is parsed, so a missing code is MISSING_FIELD even for invalid config JSON. A
     * config without CFG_ERRULE is MISSING_SECTION (after validation). Validation order:
     * fragment, disqualifier, duplicate code, RESOLVE, RELATE, exclusivity, tier, RTYPE_ID.
     *
     * <p>Wire name: {@code add_rule}; group: {@code rules}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param id <code>id</code> (int) Requested ERRULE_ID. 0 or any negative value means auto-allocate
     * (max existing + 1, floor 1000, so 1000 on the template). A taken id &gt; 0 is
     * ALREADY_EXISTS. Any ERRULE_ID key inside rule_config is IGNORED.
     * @param ruleConfig <code>rule_config</code> (json) Object with on-disk keys. ERRULE_CODE (string) is
     * required, else MISSING_FIELD; uppercased for storage and the case-insensitive duplicate
     * check (ALREADY_EXISTS). QUAL_ERFRAG_CODE (the fragment) is required: absent/non-string
     * is MISSING_FIELD, "" or an unknown code is NOT_FOUND (existence is case-insensitive).
     * DISQ_ERFRAG_CODE is optional: "" is accepted and stored as "", an unknown code is
     * NOT_FOUND. TRAP: both fragment codes are stored VERBATIM (not uppercased). RESOLVE /
     * RELATE default "No", must be Yes/No case-insensitively (stored title-case) else
     * INVALID_INPUT, and may not both be Yes (INVALID_INPUT). RESOLVE=Yes requires a non-zero
     * ERRULE_TIER (INVALID_INPUT) and forces RTYPE_ID to 1; RELATE=Yes requires RTYPE_ID in
     * 2,3,4 (INVALID_INPUT). RTYPE_ID defaults to 1; ERRULE_TIER defaults to null. A
     * non-string / non-integer value for any of these keys is treated as absent.
     * @return the modified configuration JSON document (opaque); {@link #addRuleResult} (same arguments) returns the record this operation produces
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_FIELD, ALREADY_EXISTS, NOT_FOUND, INVALID_INPUT, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addRule(String configJson, long id, String ruleConfig) throws SzConfigToolException {
        Args wire = new Args();
        wire.integer("id", id);
        wire.json("rule_config", ruleConfig);
        return Invoker.call("add_rule", "config_and_json", configJson, wire)[1];
    }

    /**
     * The record (row / ids) of <code>addRule</code>: same arguments and operation, but
     * returns the record instead of the configuration. Operation: Add an entity resolution
     * rule (CFG_ERRULE row), returning the assigned ERRULE_ID.
     *
     * <p>Notes:
     * Record is the assigned ERRULE_ID (integer). The written row always carries every
     * CFG_ERRULE key (ERRULE_ID, ERRULE_CODE, RESOLVE, RELATE, RTYPE_ID, QUAL_ERFRAG_CODE,
     * DISQ_ERFRAG_CODE, ERRULE_TIER; optional ones as null). ERRULE_CODE is checked BEFORE the
     * config is parsed, so a missing code is MISSING_FIELD even for invalid config JSON. A
     * config without CFG_ERRULE is MISSING_SECTION (after validation). Validation order:
     * fragment, disqualifier, duplicate code, RESOLVE, RELATE, exclusivity, tier, RTYPE_ID.
     *
     * <p>Wire name: {@code add_rule}; group: {@code rules}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param id <code>id</code> (int) Requested ERRULE_ID. 0 or any negative value means auto-allocate
     * (max existing + 1, floor 1000, so 1000 on the template). A taken id &gt; 0 is
     * ALREADY_EXISTS. Any ERRULE_ID key inside rule_config is IGNORED.
     * @param ruleConfig <code>rule_config</code> (json) Object with on-disk keys. ERRULE_CODE (string) is
     * required, else MISSING_FIELD; uppercased for storage and the case-insensitive duplicate
     * check (ALREADY_EXISTS). QUAL_ERFRAG_CODE (the fragment) is required: absent/non-string
     * is MISSING_FIELD, "" or an unknown code is NOT_FOUND (existence is case-insensitive).
     * DISQ_ERFRAG_CODE is optional: "" is accepted and stored as "", an unknown code is
     * NOT_FOUND. TRAP: both fragment codes are stored VERBATIM (not uppercased). RESOLVE /
     * RELATE default "No", must be Yes/No case-insensitively (stored title-case) else
     * INVALID_INPUT, and may not both be Yes (INVALID_INPUT). RESOLVE=Yes requires a non-zero
     * ERRULE_TIER (INVALID_INPUT) and forces RTYPE_ID to 1; RELATE=Yes requires RTYPE_ID in
     * 2,3,4 (INVALID_INPUT). RTYPE_ID defaults to 1; ERRULE_TIER defaults to null. A
     * non-string / non-integer value for any of these keys is treated as absent.
     * @return the record (e.g. the created row or ids) as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_FIELD, ALREADY_EXISTS, NOT_FOUND, INVALID_INPUT, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addRuleResult(String configJson, long id, String ruleConfig) throws SzConfigToolException {
        Args wire = new Args();
        wire.integer("id", id);
        wire.json("rule_config", ruleConfig);
        return Invoker.result("add_rule", "config_and_json", configJson, wire);
    }

    /**
     * Delete a rule by code.
     *
     * <p>Notes:
     * No dependency or system-rule protection; any rule can be deleted. A config without
     * CFG_ERRULE is NOT_FOUND.
     *
     * <p>Wire name: {@code delete_rule}; group: {@code rules}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Uppercased, then matched EXACTLY against ERRULE_CODE (a stored
     * lowercase code cannot be deleted). Not an id.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteRule(String configJson, String code) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("code", code);
        return Invoker.config("delete_rule", configJson, wire);
    }

    /**
     * Get one rule, by code or ERRULE_ID, as a summary record.
     *
     * <p>Notes:
     * Result is {id, rule, resolve, relate, rtype_id, fragment, disqualifier, tier} projected
     * from ERRULE_ID, ERRULE_CODE, RESOLVE, RELATE, RTYPE_ID, QUAL_ERFRAG_CODE,
     * DISQ_ERFRAG_CODE (null-preserving). TRAP: <code>tier</code> is the stored ERRULE_TIER
     * only when RESOLVE is exactly "Yes", otherwise null.
     *
     * <p>Wire name: {@code get_rule}; group: {@code rules}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param codeOrId <code>code_or_id</code> (str) Uppercased, then matched exactly against ERRULE_CODE
     * first, then numerically against ERRULE_ID (e.g. "100").
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String getRule(String configJson, String codeOrId) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("code_or_id", codeOrId);
        return Invoker.json("get_rule", configJson, wire);
    }

    /**
     * List all rules as summary records, sorted by ERRULE_ID.
     *
     * <p>Notes:
     * Result is an array of the get_rule record shape, sorted by id ascending (null/absent id
     * sorts as 0). A missing CFG_ERRULE or G2_CONFIG yields [].
     *
     * <p>Wire name: {@code list_rules}; group: {@code rules}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String listRules(String configJson) throws SzConfigToolException {
        Args wire = new Args();
        return Invoker.json("list_rules", configJson, wire);
    }

    /** Optional arguments of {@link #setRule}; unset = omitted (library default). */
    public static final class SetRuleOptions {
        final Args wire = new Args();

        /**
         * <code>resolve</code> (str) Absent keeps the stored RESOLVE. Must be Yes/No
         * case-insensitively (stored title-case), else INVALID_INPUT.
         *
         * @param resolve the value
         * @return this builder
         */
        public SetRuleOptions resolve(String resolve) {
            wire.str("resolve", resolve);
            return this;
        }

        /**
         * <code>relate</code> (str) Absent keeps the stored RELATE. Must be Yes/No
         * case-insensitively, else INVALID_INPUT.
         *
         * @param relate the value
         * @return this builder
         */
        public SetRuleOptions relate(String relate) {
            wire.str("relate", relate);
            return this;
        }

        /**
         * <code>rtype_id</code> (int) Absent keeps the stored RTYPE_ID. Forced to 1 when the
         * merged RESOLVE is Yes; must be 2, 3 or 4 when RELATE is Yes.
         *
         * @param rtypeId the value
         * @return this builder
         */
        public SetRuleOptions rtypeId(long rtypeId) {
            wire.integer("rtype_id", rtypeId);
            return this;
        }

        /**
         * <code>fragment</code> (str) tri-state: leave / clear / set. QUAL_ERFRAG_CODE. Absent =
         * keep (never re-validated); null = clear to null (TRAP: allowed here although add_rule
         * requires a fragment); a string must name an existing fragment (case-insensitive; "" is
         * NOT_FOUND) and is stored UPPERCASED (unlike add_rule).
         *
         * @param fragment the value
         * @return this builder
         */
        public SetRuleOptions fragment(FieldUpdate<String> fragment) {
            wire.strUpdate("fragment", fragment);
            return this;
        }

        /**
         * <code>disqualifier</code> (str) tri-state: leave / clear / set. DISQ_ERFRAG_CODE. Absent
         * = keep; null = clear to null; "" is accepted and stored ""; another string must name an
         * existing fragment (NOT_FOUND) and is stored uppercased.
         *
         * @param disqualifier the value
         * @return this builder
         */
        public SetRuleOptions disqualifier(FieldUpdate<String> disqualifier) {
            wire.strUpdate("disqualifier", disqualifier);
            return this;
        }

        /**
         * <code>tier</code> (int) tri-state: leave / clear / set. ERRULE_TIER. Absent = keep; null
         * = clear; value = set. The merged rule with RESOLVE=Yes must have a non-zero tier, else
         * INVALID_INPUT.
         *
         * @param tier the value
         * @return this builder
         */
        public SetRuleOptions tier(FieldUpdate<Long> tier) {
            wire.intUpdate("tier", tier);
            return this;
        }
    }

    /**
     * Update a rule's resolve/relate/relationship type, fragment, disqualifier or tier.
     *
     * <p>Notes:
     * Merges the update into the stored row and re-applies add_rule's
     * RESOLVE/RELATE/exclusivity/tier/RTYPE_ID rules to the merged row (no duplicate-code
     * check); the row is rewritten with every CFG_ERRULE key and ERRULE_ID preserved.
     *
     * <p>Wire name: {@code set_rule}; group: {@code rules}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Rule code; uppercased, then matched exactly. Unknown is
     * NOT_FOUND.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String setRule(String configJson, String code) throws SzConfigToolException {
        return setRule(configJson, code, null);
    }

    /**
     * Update a rule's resolve/relate/relationship type, fragment, disqualifier or tier.
     *
     * <p>Notes:
     * Merges the update into the stored row and re-applies add_rule's
     * RESOLVE/RELATE/exclusivity/tier/RTYPE_ID rules to the merged row (no duplicate-code
     * check); the row is rewritten with every CFG_ERRULE key and ERRULE_ID preserved.
     *
     * <p>Wire name: {@code set_rule}; group: {@code rules}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Rule code; uppercased, then matched exactly. Unknown is
     * NOT_FOUND.
     * @param options optional arguments ({@code null} = none); see {@link SetRuleOptions}
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String setRule(String configJson, String code, SetRuleOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("code", code);
        return Invoker.config("set_rule", configJson, wire);
    }

    /** Optional arguments of {@link #addSearchProfile}; unset = omitted (library default). */
    public static final class AddSearchProfileOptions {
        final Args wire = new Args();

        /**
         * <code>candidates</code> (str) DEFAULT_USED_FOR_CAND; trimmed, case-insensitive,
         * normalized to Normal or Off. Absent or blank = Normal. Anything else is
         * VALIDATION_ERRORS (field "candidates", OUT_OF_DOMAIN). Library default when omitted:
         * "Normal".
         *
         * @param candidates the value
         * @return this builder
         */
        public AddSearchProfileOptions candidates(String candidates) {
            wire.str("candidates", candidates);
            return this;
        }

        /**
         * <code>description</code> (str) SPROFILE_DESC, stored verbatim; absent = "". Library
         * default when omitted: "".
         *
         * @param description the value
         * @return this builder
         */
        public AddSearchProfileOptions description(String description) {
            wire.str("description", description);
            return this;
        }

        /**
         * <code>elements</code> (json) Feature candidate overrides: an array of {"feature":
         * FTYPE_CODE, "flag": Yes|No|Y|N} objects (only those two keys, both strings, else
         * INVALID_INPUT; a missing key = MISSING_FIELD); absent = none. Each feature is resolved
         * case-insensitively against CFG_FTYPE (NOT_FOUND); a feature listed twice is
         * VALIDATION_ERRORS (field "overrides", DUPLICATE); a flag other than Yes/Y/No/N (trimmed,
         * case-insensitive) is VALIDATION_ERRORS (field "overrides", OUT_OF_DOMAIN). Stored in
         * FTYPE_OVERRIDES as "[{&lt;ftypeId&gt;,&lt;Y|N&gt;},...]" sorted by FTYPE_ID, or "[]".
         *
         * @param elements the value
         * @return this builder
         */
        public AddSearchProfileOptions elements(String elements) {
            wire.json("elements", elements);
            return this;
        }
    }

    /**
     * Add a search profile (CFG_SPROFILE row) tying a generic plan and feature candidate
     * overrides to a code.
     *
     * <p>Notes:
     * SPROFILE_ID is always auto-allocated (max + 1, floor 1; 3 on the template, whose only
     * profile is SEARCH = 2); no explicit id can be requested. CFG_SPROFILE is created when
     * absent. Validation order: code, candidates, generic plan, overrides (per element:
     * feature, duplicate, flag), duplicate code. A config without G2_CONFIG fails the
     * generic-plan lookup (NOT_FOUND), so the library's MISSING_SECTION branch is unreachable;
     * a non-array CFG_SPROFILE is INVALID_STRUCTURE.
     *
     * <p>Wire name: {@code add_search_profile}; group: {@code search_profiles}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) SPROFILE_CODE; trimmed and uppercased. Empty after trimming is
     * INVALID_INPUT; an existing code (case-insensitive) is ALREADY_EXISTS.
     * @param genericPlan <code>generic_plan</code> (str) GPLAN_CODE, matched case-insensitively against CFG_GPLAN
     * (NOT_FOUND); stored as GPLAN_ID.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, INVALID_INPUT, VALIDATION_ERRORS, NOT_FOUND, ALREADY_EXISTS, INVALID_STRUCTURE, MISSING_FIELD; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addSearchProfile(String configJson, String code, String genericPlan) throws SzConfigToolException {
        return addSearchProfile(configJson, code, genericPlan, null);
    }

    /**
     * Add a search profile (CFG_SPROFILE row) tying a generic plan and feature candidate
     * overrides to a code.
     *
     * <p>Notes:
     * SPROFILE_ID is always auto-allocated (max + 1, floor 1; 3 on the template, whose only
     * profile is SEARCH = 2); no explicit id can be requested. CFG_SPROFILE is created when
     * absent. Validation order: code, candidates, generic plan, overrides (per element:
     * feature, duplicate, flag), duplicate code. A config without G2_CONFIG fails the
     * generic-plan lookup (NOT_FOUND), so the library's MISSING_SECTION branch is unreachable;
     * a non-array CFG_SPROFILE is INVALID_STRUCTURE.
     *
     * <p>Wire name: {@code add_search_profile}; group: {@code search_profiles}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) SPROFILE_CODE; trimmed and uppercased. Empty after trimming is
     * INVALID_INPUT; an existing code (case-insensitive) is ALREADY_EXISTS.
     * @param genericPlan <code>generic_plan</code> (str) GPLAN_CODE, matched case-insensitively against CFG_GPLAN
     * (NOT_FOUND); stored as GPLAN_ID.
     * @param options optional arguments ({@code null} = none); see {@link AddSearchProfileOptions}
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, INVALID_INPUT, VALIDATION_ERRORS, NOT_FOUND, ALREADY_EXISTS, INVALID_STRUCTURE, MISSING_FIELD; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addSearchProfile(String configJson, String code, String genericPlan, AddSearchProfileOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("code", code);
        wire.str("generic_plan", genericPlan);
        return Invoker.config("add_search_profile", configJson, wire);
    }

    /**
     * Get one search profile's raw CFG_SPROFILE row by code.
     *
     * <p>Notes:
     * Result is the stored row (SPROFILE_ID, SPROFILE_CODE, SPROFILE_DESC, GPLAN_ID,
     * DEFAULT_USED_FOR_CAND, FTYPE_OVERRIDES). A missing section is NOT_FOUND.
     *
     * <p>Wire name: {@code get_search_profile}; group: {@code search_profiles}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param code <code>code</code> (str) Case-insensitive match against SPROFILE_CODE. Not an id.
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String getSearchProfile(String configJson, String code) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("code", code);
        return Invoker.json("get_search_profile", configJson, wire);
    }

    /** Optional arguments of {@link #listSearchProfiles}; unset = omitted (library default). */
    public static final class ListSearchProfilesOptions {
        final Args wire = new Args();

        /**
         * <code>filter</code> (str) Absent returns all. Otherwise keeps rows whose COMPACT raw-row
         * JSON (serde_json::to_string, no spaces, on-disk keys and ids, e.g. "GPLAN_ID":2)
         * contains the filter case-insensitively; it is applied to the raw row, not the projected
         * record.
         *
         * @param filter the value
         * @return this builder
         */
        public ListSearchProfilesOptions filter(String filter) {
            wire.str("filter", filter);
            return this;
        }
    }

    /**
     * List search profiles as display records with ids resolved to codes, sorted by id.
     *
     * <p>Notes:
     * Result is an array of {id, profile, description, genericPlan, candidates, overrides:
     * [{feature, flag: Yes|No}], overridesRaw} sorted by id. genericPlan / feature fall back
     * to the numeric id as a string when unresolvable. A missing CFG_SPROFILE yields [].
     *
     * <p>Wire name: {@code list_search_profiles}; group: {@code search_profiles}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String listSearchProfiles(String configJson) throws SzConfigToolException {
        return listSearchProfiles(configJson, null);
    }

    /**
     * List search profiles as display records with ids resolved to codes, sorted by id.
     *
     * <p>Notes:
     * Result is an array of {id, profile, description, genericPlan, candidates, overrides:
     * [{feature, flag: Yes|No}], overridesRaw} sorted by id. genericPlan / feature fall back
     * to the numeric id as a string when unresolvable. A missing CFG_SPROFILE yields [].
     *
     * <p>Wire name: {@code list_search_profiles}; group: {@code search_profiles}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param options optional arguments ({@code null} = none); see {@link ListSearchProfilesOptions}
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String listSearchProfiles(String configJson, ListSearchProfilesOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        return Invoker.json("list_search_profiles", configJson, wire);
    }

    /**
     * Delete a search profile by code or SPROFILE_ID.
     *
     * <p>Notes:
     * The shipped profiles INGEST and SEARCH are protected: deleting one that exists is
     * INVALID_INPUT. Existence is checked FIRST, so an absent reserved code (INGEST is not in
     * the template) is NOT_FOUND.
     *
     * <p>Wire name: {@code delete_search_profile}; group: {@code search_profiles}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param searchValue <code>search_value</code> (str) Matched case-insensitively against SPROFILE_CODE, or
     * (when it parses as an integer after trimming) against SPROFILE_ID.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, INVALID_INPUT; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteSearchProfile(String configJson, String searchValue) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("search_value", searchValue);
        return Invoker.config("delete_search_profile", configJson, wire);
    }

    /**
     * Create or overwrite a named setting in the G2_CONFIG.SETTINGS object.
     *
     * <p>Notes:
     * SETTINGS is created when absent, and a non-object SETTINGS value (null, string, array)
     * is REPLACED by a fresh object (prior content lost). A missing or non-object G2_CONFIG is
     * MISSING_SECTION.
     *
     * <p>Wire name: {@code set_setting}; group: {@code settings}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param name <code>name</code> (str) Uppercased; an existing setting of that name is overwritten
     * silently.
     * @param value <code>value</code> (json) Stored VERBATIM as its typed JSON value (an integer stays an
     * integer, a string a string); no validation.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String setSetting(String configJson, String name, String value) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("name", name);
        wire.json("value", value);
        return Invoker.config("set_setting", configJson, wire);
    }

    /**
     * List system parameters as a name -&gt; string-value map.
     *
     * <p>Notes:
     * Result is an object. The only parameter is relationshipsBreakMatches, read from
     * BREAK_RES of the FIRST CFG_RTYPE row with RCLASS_ID 2 and reported as a decimal STRING.
     * TRAP: it is reported only when BREAK_RES is a JSON integer; the template's DISCLOSED row
     * stores the string "No", so the template yields {}. A missing CFG_RTYPE/G2_CONFIG also
     * yields {}.
     *
     * <p>Wire name: {@code list_system_parameters}; group: {@code system_params}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String listSystemParameters(String configJson) throws SzConfigToolException {
        Args wire = new Args();
        return Invoker.json("list_system_parameters", configJson, wire);
    }

    /**
     * Set a system parameter (relationshipsBreakMatches).
     *
     * <p>Notes:
     * NOT_FOUND when no CFG_RTYPE row has RCLASS_ID 2 (or CFG_RTYPE/G2_CONFIG is absent). The
     * parameter name is checked AFTER the config is parsed.
     *
     * <p>Wire name: {@code set_system_parameter}; group: {@code system_params}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param parameterName <code>parameter_name</code> (str) Case-insensitive; only relationshipsBreakMatches (or
     * relationships_break_matches) is known. Any other name is INVALID_CONFIG (not
     * INVALID_INPUT).
     * @param parameterValue <code>parameter_value</code> (json) Written VERBATIM (any JSON value, no validation) to
     * BREAK_RES of the first CFG_RTYPE row with RCLASS_ID 2. Only an integer value is visible
     * to list_system_parameters afterwards.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, INVALID_CONFIG, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String setSystemParameter(String configJson, String parameterName, String parameterValue) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("parameter_name", parameterName);
        wire.json("parameter_value", parameterValue);
        return Invoker.config("set_system_parameter", configJson, wire);
    }

    /** Optional arguments of {@link #addComparisonThreshold}; unset = omitted (library default). */
    public static final class AddComparisonThresholdOptions {
        final Args wire = new Args();

        /**
         * <code>exec_order</code> (int) Resolved in three steps: (1) if an all-features (FTYPE_ID
         * 0) row already exists for this (cfunc, rtnval), its EXEC_ORDER is REUSED and this arg is
         * ignored; (2) else a value &gt; 0 is honoured, or ALREADY_EXISTS if taken within
         * (CFUNC_ID, FTYPE_ID 0); (3) else (absent or &lt;= 0) the next order within (CFUNC_ID,
         * FTYPE_ID 0) is auto-allocated (max + 1). Never null.
         *
         * @param execOrder the value
         * @return this builder
         */
        public AddComparisonThresholdOptions execOrder(long execOrder) {
            wire.integer("exec_order", execOrder);
            return this;
        }

        /**
         * <code>same_score</code> (int) Stored verbatim (no range check); absent stores SAME_SCORE
         * null.
         *
         * @param sameScore the value
         * @return this builder
         */
        public AddComparisonThresholdOptions sameScore(long sameScore) {
            wire.integer("same_score", sameScore);
            return this;
        }

        /**
         * <code>close_score</code> (int) Stored verbatim; absent stores CLOSE_SCORE null.
         *
         * @param closeScore the value
         * @return this builder
         */
        public AddComparisonThresholdOptions closeScore(long closeScore) {
            wire.integer("close_score", closeScore);
            return this;
        }

        /**
         * <code>likely_score</code> (int) Stored verbatim; absent stores LIKELY_SCORE null.
         *
         * @param likelyScore the value
         * @return this builder
         */
        public AddComparisonThresholdOptions likelyScore(long likelyScore) {
            wire.integer("likely_score", likelyScore);
            return this;
        }

        /**
         * <code>plausible_score</code> (int) Stored verbatim; absent stores PLAUSIBLE_SCORE null.
         *
         * @param plausibleScore the value
         * @return this builder
         */
        public AddComparisonThresholdOptions plausibleScore(long plausibleScore) {
            wire.integer("plausible_score", plausibleScore);
            return this;
        }

        /**
         * <code>un_likely_score</code> (int) Stored verbatim; absent stores UN_LIKELY_SCORE null.
         *
         * @param unLikelyScore the value
         * @return this builder
         */
        public AddComparisonThresholdOptions unLikelyScore(long unLikelyScore) {
            wire.integer("un_likely_score", unLikelyScore);
            return this;
        }
    }

    /**
     * Add a comparison threshold (CFG_CFRTN row) for a comparison function, feature and return
     * value.
     *
     * <p>Notes:
     * CFRTN_ID is always auto-allocated (max existing + 1, no floor); there is no id arg.
     * Duplicate key is (CFUNC_ID, FTYPE_ID, uppercased rtnval) = ALREADY_EXISTS. Order:
     * missing fields, cfunc lookup, feature lookup, then CFG_CFRTN section, duplicate, exec
     * order.
     *
     * <p>Wire name: {@code add_comparison_threshold}; group: {@code thresholds}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param cfuncCode <code>cfunc_code</code> (str) required by the library. REQUIRED by the library (absent =
     * MISSING_FIELD). Comparison function code (CFG_CFUNC), matched case-insensitively;
     * unknown = NOT_FOUND.
     * @param ftypeCode <code>ftype_code</code> (str) required by the library. REQUIRED by the library (absent =
     * MISSING_FIELD). Feature code matched case-insensitively (unknown = NOT_FOUND), or "all"
     * (any case) for the all-features FTYPE_ID 0 sentinel.
     * @param cfuncRtnval <code>cfunc_rtnval</code> (str) required by the library. REQUIRED by the library (absent
     * = MISSING_FIELD). Return value / score name; uppercased before storage and duplicate
     * check.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_FIELD, NOT_FOUND, MISSING_SECTION, ALREADY_EXISTS; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addComparisonThreshold(String configJson, String cfuncCode, String ftypeCode, String cfuncRtnval) throws SzConfigToolException {
        return addComparisonThreshold(configJson, cfuncCode, ftypeCode, cfuncRtnval, null);
    }

    /**
     * Add a comparison threshold (CFG_CFRTN row) for a comparison function, feature and return
     * value.
     *
     * <p>Notes:
     * CFRTN_ID is always auto-allocated (max existing + 1, no floor); there is no id arg.
     * Duplicate key is (CFUNC_ID, FTYPE_ID, uppercased rtnval) = ALREADY_EXISTS. Order:
     * missing fields, cfunc lookup, feature lookup, then CFG_CFRTN section, duplicate, exec
     * order.
     *
     * <p>Wire name: {@code add_comparison_threshold}; group: {@code thresholds}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param cfuncCode <code>cfunc_code</code> (str) required by the library. REQUIRED by the library (absent =
     * MISSING_FIELD). Comparison function code (CFG_CFUNC), matched case-insensitively;
     * unknown = NOT_FOUND.
     * @param ftypeCode <code>ftype_code</code> (str) required by the library. REQUIRED by the library (absent =
     * MISSING_FIELD). Feature code matched case-insensitively (unknown = NOT_FOUND), or "all"
     * (any case) for the all-features FTYPE_ID 0 sentinel.
     * @param cfuncRtnval <code>cfunc_rtnval</code> (str) required by the library. REQUIRED by the library (absent
     * = MISSING_FIELD). Return value / score name; uppercased before storage and duplicate
     * check.
     * @param options optional arguments ({@code null} = none); see {@link AddComparisonThresholdOptions}
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_FIELD, NOT_FOUND, MISSING_SECTION, ALREADY_EXISTS; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addComparisonThreshold(String configJson, String cfuncCode, String ftypeCode, String cfuncRtnval, AddComparisonThresholdOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("cfunc_code", cfuncCode);
        wire.str("ftype_code", ftypeCode);
        wire.str("cfunc_rtnval", cfuncRtnval);
        return Invoker.config("add_comparison_threshold", configJson, wire);
    }

    /**
     * Delete a comparison threshold identified by (comparison function, feature, return
     * value).
     *
     * <p>Notes:
     * No matching row = NOT_FOUND. No tier/dependency protection: deleting the all-features
     * tier row leaves per-feature rows in place.
     *
     * <p>Wire name: {@code delete_comparison_threshold}; group: {@code thresholds}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param cfuncCode <code>cfunc_code</code> (str) Comparison function code, case-insensitive; unknown =
     * NOT_FOUND.
     * @param ftypeCode <code>ftype_code</code> (str) Feature code (case-insensitive) or "all" for FTYPE_ID 0;
     * unknown = NOT_FOUND.
     * @param cfuncRtnval <code>cfunc_rtnval</code> (str) Return value, matched case-insensitively.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteComparisonThreshold(String configJson, String cfuncCode, String ftypeCode, String cfuncRtnval) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("cfunc_code", cfuncCode);
        wire.str("ftype_code", ftypeCode);
        wire.str("cfunc_rtnval", cfuncRtnval);
        return Invoker.config("delete_comparison_threshold", configJson, wire);
    }

    /** Optional arguments of {@link #setComparisonThreshold}; unset = omitted (library default). */
    public static final class SetComparisonThresholdOptions {
        final Args wire = new Args();

        /**
         * <code>exec_order</code> (int) Absent leaves EXEC_ORDER unchanged; a value is written
         * VERBATIM (no uniqueness / tier check, any integer incl. &lt;= 0).
         *
         * @param execOrder the value
         * @return this builder
         */
        public SetComparisonThresholdOptions execOrder(long execOrder) {
            wire.integer("exec_order", execOrder);
            return this;
        }

        /**
         * <code>same_score</code> (int) Absent leaves unchanged; else written verbatim.
         *
         * @param sameScore the value
         * @return this builder
         */
        public SetComparisonThresholdOptions sameScore(long sameScore) {
            wire.integer("same_score", sameScore);
            return this;
        }

        /**
         * <code>close_score</code> (int) Absent leaves unchanged; else written verbatim.
         *
         * @param closeScore the value
         * @return this builder
         */
        public SetComparisonThresholdOptions closeScore(long closeScore) {
            wire.integer("close_score", closeScore);
            return this;
        }

        /**
         * <code>likely_score</code> (int) Absent leaves unchanged; else written verbatim.
         *
         * @param likelyScore the value
         * @return this builder
         */
        public SetComparisonThresholdOptions likelyScore(long likelyScore) {
            wire.integer("likely_score", likelyScore);
            return this;
        }

        /**
         * <code>plausible_score</code> (int) Absent leaves unchanged; else written verbatim.
         *
         * @param plausibleScore the value
         * @return this builder
         */
        public SetComparisonThresholdOptions plausibleScore(long plausibleScore) {
            wire.integer("plausible_score", plausibleScore);
            return this;
        }

        /**
         * <code>un_likely_score</code> (int) Absent leaves unchanged; else written verbatim.
         *
         * @param unLikelyScore the value
         * @return this builder
         */
        public SetComparisonThresholdOptions unLikelyScore(long unLikelyScore) {
            wire.integer("un_likely_score", unLikelyScore);
            return this;
        }
    }

    /**
     * Update the exec order and/or scores of an existing comparison threshold.
     *
     * <p>Notes:
     * NOT tri-state: a score cannot be cleared back to null.
     *
     * <p>Wire name: {@code set_comparison_threshold}; group: {@code thresholds}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param cfuncCode <code>cfunc_code</code> (str) required by the library. REQUIRED by the library (absent =
     * MISSING_FIELD). Case-insensitive lookup; unknown = NOT_FOUND.
     * @param ftypeCode <code>ftype_code</code> (str) required by the library. REQUIRED (absent =
     * MISSING_FIELD). Feature code (case-insensitive) or "all" for FTYPE_ID 0.
     * @param cfuncRtnval <code>cfunc_rtnval</code> (str) required by the library. REQUIRED (absent =
     * MISSING_FIELD). Matched case-insensitively; no row = NOT_FOUND.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_FIELD, NOT_FOUND, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String setComparisonThreshold(String configJson, String cfuncCode, String ftypeCode, String cfuncRtnval) throws SzConfigToolException {
        return setComparisonThreshold(configJson, cfuncCode, ftypeCode, cfuncRtnval, null);
    }

    /**
     * Update the exec order and/or scores of an existing comparison threshold.
     *
     * <p>Notes:
     * NOT tri-state: a score cannot be cleared back to null.
     *
     * <p>Wire name: {@code set_comparison_threshold}; group: {@code thresholds}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param cfuncCode <code>cfunc_code</code> (str) required by the library. REQUIRED by the library (absent =
     * MISSING_FIELD). Case-insensitive lookup; unknown = NOT_FOUND.
     * @param ftypeCode <code>ftype_code</code> (str) required by the library. REQUIRED (absent =
     * MISSING_FIELD). Feature code (case-insensitive) or "all" for FTYPE_ID 0.
     * @param cfuncRtnval <code>cfunc_rtnval</code> (str) required by the library. REQUIRED (absent =
     * MISSING_FIELD). Matched case-insensitively; no row = NOT_FOUND.
     * @param options optional arguments ({@code null} = none); see {@link SetComparisonThresholdOptions}
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_FIELD, NOT_FOUND, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String setComparisonThreshold(String configJson, String cfuncCode, String ftypeCode, String cfuncRtnval, SetComparisonThresholdOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("cfunc_code", cfuncCode);
        wire.str("ftype_code", ftypeCode);
        wire.str("cfunc_rtnval", cfuncRtnval);
        return Invoker.config("set_comparison_threshold", configJson, wire);
    }

    /**
     * List all comparison thresholds with resolved function and feature names.
     *
     * <p>Notes:
     * Array of {id (CFRTN_ID), function (CFUNC_CODE or "unknown"), returnOrder (EXEC_ORDER),
     * scoreName, feature ("all" for FTYPE_ID 0, else FTYPE_CODE or "unknown"), sameScore,
     * closeScore, likelyScore, plausibleScore, unlikelyScore}, sorted by (CFUNC_ID, CFRTN_ID).
     * TRAP: null/absent EXEC_ORDER and scores are reported as 0, not null. Requires CFG_CFRTN,
     * CFG_CFUNC and CFG_FTYPE (else MISSING_SECTION).
     *
     * <p>Wire name: {@code list_comparison_thresholds}; group: {@code thresholds}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String listComparisonThresholds(String configJson) throws SzConfigToolException {
        Args wire = new Args();
        return Invoker.json("list_comparison_thresholds", configJson, wire);
    }

    /** Optional arguments of {@link #addGenericThreshold}; unset = omitted (library default). */
    public static final class AddGenericThresholdOptions {
        final Args wire = new Args();

        /**
         * <code>feature</code> (str) Absent or "all" (any case) = FTYPE_ID 0. Else uppercased and
         * matched EXACTLY against FTYPE_CODE; unknown = NOT_FOUND. Library default when omitted:
         * "ALL".
         *
         * @param feature the value
         * @return this builder
         */
        public AddGenericThresholdOptions feature(String feature) {
            wire.str("feature", feature);
            return this;
        }
    }

    /**
     * Add a generic threshold (CFG_GENERIC_THRESHOLD row) for a plan, behavior and optional
     * feature.
     *
     * <p>Notes:
     * All absent required fields are reported in ONE MISSING_FIELD (order plan, behavior,
     * scoring_cap, candidate_cap, send_to_redo), checked before the config is parsed. Then:
     * plan lookup, feature lookup, duplicate (plan, behavior, feature) = ALREADY_EXISTS
     * (Python treats it as a warning-success; the root library does not), then behavior +
     * sendToRedo are validated TOGETHER into one VALIDATION_ERRORS (details schema
     * sz-configtool.validation-errors/v1, order [behavior, sendToRedo]).
     *
     * <p>Wire name: {@code add_generic_threshold}; group: {@code thresholds}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param plan <code>plan</code> (str) required by the library. REQUIRED (absent = MISSING_FIELD).
     * Uppercased, then matched EXACTLY against GPLAN_CODE (so effectively case-insensitive for
     * upper-case stored codes); unknown = NOT_FOUND.
     * @param behavior <code>behavior</code> (str) required by the library. REQUIRED (absent = MISSING_FIELD).
     * Uppercased; must be a canonical behavior code (behavior_domain, e.g. NAME, F1, FM, A1)
     * else VALIDATION_ERRORS (field "behavior", UnknownReferenceCode).
     * @param scoringCap <code>scoring_cap</code> (int) required by the library. REQUIRED (absent =
     * MISSING_FIELD). Stored verbatim (e.g. -1).
     * @param candidateCap <code>candidate_cap</code> (int) required by the library. REQUIRED (absent =
     * MISSING_FIELD). Stored verbatim.
     * @param sendToRedo <code>send_to_redo</code> (str) required by the library. REQUIRED (absent =
     * MISSING_FIELD). Case-insensitive Yes/No, stored canonical "Yes"/"No"; else
     * VALIDATION_ERRORS (field "sendToRedo", OutOfDomain).
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_FIELD, MISSING_SECTION, NOT_FOUND, ALREADY_EXISTS, VALIDATION_ERRORS; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addGenericThreshold(String configJson, String plan, String behavior, long scoringCap, long candidateCap, String sendToRedo) throws SzConfigToolException {
        return addGenericThreshold(configJson, plan, behavior, scoringCap, candidateCap, sendToRedo, null);
    }

    /**
     * Add a generic threshold (CFG_GENERIC_THRESHOLD row) for a plan, behavior and optional
     * feature.
     *
     * <p>Notes:
     * All absent required fields are reported in ONE MISSING_FIELD (order plan, behavior,
     * scoring_cap, candidate_cap, send_to_redo), checked before the config is parsed. Then:
     * plan lookup, feature lookup, duplicate (plan, behavior, feature) = ALREADY_EXISTS
     * (Python treats it as a warning-success; the root library does not), then behavior +
     * sendToRedo are validated TOGETHER into one VALIDATION_ERRORS (details schema
     * sz-configtool.validation-errors/v1, order [behavior, sendToRedo]).
     *
     * <p>Wire name: {@code add_generic_threshold}; group: {@code thresholds}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param plan <code>plan</code> (str) required by the library. REQUIRED (absent = MISSING_FIELD).
     * Uppercased, then matched EXACTLY against GPLAN_CODE (so effectively case-insensitive for
     * upper-case stored codes); unknown = NOT_FOUND.
     * @param behavior <code>behavior</code> (str) required by the library. REQUIRED (absent = MISSING_FIELD).
     * Uppercased; must be a canonical behavior code (behavior_domain, e.g. NAME, F1, FM, A1)
     * else VALIDATION_ERRORS (field "behavior", UnknownReferenceCode).
     * @param scoringCap <code>scoring_cap</code> (int) required by the library. REQUIRED (absent =
     * MISSING_FIELD). Stored verbatim (e.g. -1).
     * @param candidateCap <code>candidate_cap</code> (int) required by the library. REQUIRED (absent =
     * MISSING_FIELD). Stored verbatim.
     * @param sendToRedo <code>send_to_redo</code> (str) required by the library. REQUIRED (absent =
     * MISSING_FIELD). Case-insensitive Yes/No, stored canonical "Yes"/"No"; else
     * VALIDATION_ERRORS (field "sendToRedo", OutOfDomain).
     * @param options optional arguments ({@code null} = none); see {@link AddGenericThresholdOptions}
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_FIELD, MISSING_SECTION, NOT_FOUND, ALREADY_EXISTS, VALIDATION_ERRORS; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String addGenericThreshold(String configJson, String plan, String behavior, long scoringCap, long candidateCap, String sendToRedo, AddGenericThresholdOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("plan", plan);
        wire.str("behavior", behavior);
        wire.integer("scoring_cap", scoringCap);
        wire.integer("candidate_cap", candidateCap);
        wire.str("send_to_redo", sendToRedo);
        return Invoker.config("add_generic_threshold", configJson, wire);
    }

    /** Optional arguments of {@link #deleteGenericThreshold}; unset = omitted (library default). */
    public static final class DeleteGenericThresholdOptions {
        final Args wire = new Args();

        /**
         * <code>feature</code> (str) Absent or "all" = FTYPE_ID 0; else uppercased exact
         * FTYPE_CODE match, unknown = NOT_FOUND. Library default when omitted: "ALL".
         *
         * @param feature the value
         * @return this builder
         */
        public DeleteGenericThresholdOptions feature(String feature) {
            wire.str("feature", feature);
            return this;
        }
    }

    /**
     * Delete a generic threshold identified by (plan, behavior, feature).
     *
     * <p>Notes:
     * No matching row = NOT_FOUND (also when CFG_GENERIC_THRESHOLD is absent). (Before the
     * Unreleased fix it ignored <code>plan</code> and always deleted from INGEST.)
     *
     * <p>Wire name: {@code delete_generic_threshold}; group: {@code thresholds}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param plan <code>plan</code> (str) required by the library. REQUIRED (absent = MISSING_FIELD).
     * Case-insensitive GPLAN_CODE lookup; unknown = NOT_FOUND.
     * @param behavior <code>behavior</code> (str) required by the library. REQUIRED (absent = MISSING_FIELD).
     * Uppercased and matched exactly; NOT validated against the behavior domain (no row =
     * NOT_FOUND).
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_FIELD, NOT_FOUND, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteGenericThreshold(String configJson, String plan, String behavior) throws SzConfigToolException {
        return deleteGenericThreshold(configJson, plan, behavior, null);
    }

    /**
     * Delete a generic threshold identified by (plan, behavior, feature).
     *
     * <p>Notes:
     * No matching row = NOT_FOUND (also when CFG_GENERIC_THRESHOLD is absent). (Before the
     * Unreleased fix it ignored <code>plan</code> and always deleted from INGEST.)
     *
     * <p>Wire name: {@code delete_generic_threshold}; group: {@code thresholds}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param plan <code>plan</code> (str) required by the library. REQUIRED (absent = MISSING_FIELD).
     * Case-insensitive GPLAN_CODE lookup; unknown = NOT_FOUND.
     * @param behavior <code>behavior</code> (str) required by the library. REQUIRED (absent = MISSING_FIELD).
     * Uppercased and matched exactly; NOT validated against the behavior domain (no row =
     * NOT_FOUND).
     * @param options optional arguments ({@code null} = none); see {@link DeleteGenericThresholdOptions}
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_FIELD, NOT_FOUND, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String deleteGenericThreshold(String configJson, String plan, String behavior, DeleteGenericThresholdOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("plan", plan);
        wire.str("behavior", behavior);
        return Invoker.config("delete_generic_threshold", configJson, wire);
    }

    /** Optional arguments of {@link #setGenericThreshold}; unset = omitted (library default). */
    public static final class SetGenericThresholdOptions {
        final Args wire = new Args();

        /**
         * <code>feature</code> (str) Lookup KEY selecting the per-feature row (never written).
         * Absent or "all" = FTYPE_ID 0; else case-insensitive feature lookup, unknown = NOT_FOUND.
         * Library default when omitted: "ALL".
         *
         * @param feature the value
         * @return this builder
         */
        public SetGenericThresholdOptions feature(String feature) {
            wire.str("feature", feature);
            return this;
        }

        /**
         * <code>candidate_cap</code> (int) Absent leaves CANDIDATE_CAP unchanged; else written
         * verbatim.
         *
         * @param candidateCap the value
         * @return this builder
         */
        public SetGenericThresholdOptions candidateCap(long candidateCap) {
            wire.integer("candidate_cap", candidateCap);
            return this;
        }

        /**
         * <code>scoring_cap</code> (int) Absent leaves SCORING_CAP unchanged; else written
         * verbatim.
         *
         * @param scoringCap the value
         * @return this builder
         */
        public SetGenericThresholdOptions scoringCap(long scoringCap) {
            wire.integer("scoring_cap", scoringCap);
            return this;
        }

        /**
         * <code>send_to_redo</code> (str) Absent leaves unchanged; else case-insensitive Yes/No
         * stored canonical, otherwise VALIDATION_ERRORS (field "sendToRedo", OutOfDomain) —
         * checked AFTER the row lookup, so a missing row wins.
         *
         * @param sendToRedo the value
         * @return this builder
         */
        public SetGenericThresholdOptions sendToRedo(String sendToRedo) {
            wire.str("send_to_redo", sendToRedo);
            return this;
        }
    }

    /**
     * Update the caps and/or send-to-redo flag of an existing generic threshold.
     *
     * <p>Wire name: {@code set_generic_threshold}; group: {@code thresholds}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param plan <code>plan</code> (str) required by the library. REQUIRED (absent = MISSING_FIELD).
     * Case-insensitive GPLAN_CODE lookup; unknown = NOT_FOUND.
     * @param behavior <code>behavior</code> (str) required by the library. REQUIRED (absent = MISSING_FIELD).
     * Lookup KEY only: uppercased and matched exactly, never validated against the domain (no
     * row = NOT_FOUND).
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_FIELD, NOT_FOUND, MISSING_SECTION, VALIDATION_ERRORS; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String setGenericThreshold(String configJson, String plan, String behavior) throws SzConfigToolException {
        return setGenericThreshold(configJson, plan, behavior, null);
    }

    /**
     * Update the caps and/or send-to-redo flag of an existing generic threshold.
     *
     * <p>Wire name: {@code set_generic_threshold}; group: {@code thresholds}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param plan <code>plan</code> (str) required by the library. REQUIRED (absent = MISSING_FIELD).
     * Case-insensitive GPLAN_CODE lookup; unknown = NOT_FOUND.
     * @param behavior <code>behavior</code> (str) required by the library. REQUIRED (absent = MISSING_FIELD).
     * Lookup KEY only: uppercased and matched exactly, never validated against the domain (no
     * row = NOT_FOUND).
     * @param options optional arguments ({@code null} = none); see {@link SetGenericThresholdOptions}
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_FIELD, NOT_FOUND, MISSING_SECTION, VALIDATION_ERRORS; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String setGenericThreshold(String configJson, String plan, String behavior, SetGenericThresholdOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("plan", plan);
        wire.str("behavior", behavior);
        return Invoker.config("set_generic_threshold", configJson, wire);
    }

    /**
     * List all generic thresholds with resolved plan and feature names.
     *
     * <p>Notes:
     * Array of {id (GPLAN_ID), plan, behavior, feature ("all" for FTYPE_ID 0), candidateCap,
     * scoringCap, sendToRedo}, sorted by (GPLAN_ID, canonical behavior position; unknown
     * behaviors last), stable within ties. Requires CFG_GENERIC_THRESHOLD, CFG_GPLAN and
     * CFG_FTYPE (else MISSING_SECTION).
     *
     * <p>Wire name: {@code list_generic_thresholds}; group: {@code thresholds}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String listGenericThresholds(String configJson) throws SzConfigToolException {
        Args wire = new Args();
        return Invoker.json("list_generic_thresholds", configJson, wire);
    }

    /** Optional arguments of {@link #validateGenericThreshold}; unset = omitted (library default). */
    public static final class ValidateGenericThresholdOptions {
        final Args wire = new Args();

        /**
         * <code>feature</code> (str) Absent or "all" (any case) = FTYPE_ID 0. Else uppercased
         * exact FTYPE_CODE match; unknown = result notFound/feature. Library default when omitted:
         * "ALL".
         *
         * @param feature the value
         * @return this builder
         */
        public ValidateGenericThresholdOptions feature(String feature) {
            wire.str("feature", feature);
            return this;
        }
    }

    /**
     * Stage the checks of a generic-threshold add without mutating the config, returning the
     * outcome as data.
     *
     * <p>Notes:
     * Result is the versioned object (schema sz-configtool.generic-threshold-check/v1,
     * root-library Serialize for GenericThresholdCheck): {"schema", "result": "ok" |
     * "duplicate" | "notFound" (+ "which": "plan"|"feature", "value": uppercased code) |
     * "invalid" (+ "failures": [{"field", "reasonCode", "offendingValue"}], order [behavior,
     * sendToRedo])}. Stages stop at the first hit: plan, feature, duplicate (plan, behavior,
     * feature), then field validation. Missing sections are NOT errors (a missing CFG_GPLAN is
     * notFound/plan). Caps are not taken.
     *
     * <p>Wire name: {@code validate_generic_threshold}; group: {@code thresholds}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param plan <code>plan</code> (str) Uppercased, matched exactly against GPLAN_CODE; unknown = result
     * notFound/plan (data, not an error).
     * @param behavior <code>behavior</code> (str) Uppercased; checked against the canonical behavior codes
     * only in the last stage.
     * @param sendToRedo <code>send_to_redo</code> (str) Case-insensitive Yes/No; checked only in the last stage.
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String validateGenericThreshold(String configJson, String plan, String behavior, String sendToRedo) throws SzConfigToolException {
        return validateGenericThreshold(configJson, plan, behavior, sendToRedo, null);
    }

    /**
     * Stage the checks of a generic-threshold add without mutating the config, returning the
     * outcome as data.
     *
     * <p>Notes:
     * Result is the versioned object (schema sz-configtool.generic-threshold-check/v1,
     * root-library Serialize for GenericThresholdCheck): {"schema", "result": "ok" |
     * "duplicate" | "notFound" (+ "which": "plan"|"feature", "value": uppercased code) |
     * "invalid" (+ "failures": [{"field", "reasonCode", "offendingValue"}], order [behavior,
     * sendToRedo])}. Stages stop at the first hit: plan, feature, duplicate (plan, behavior,
     * feature), then field validation. Missing sections are NOT errors (a missing CFG_GPLAN is
     * notFound/plan). Caps are not taken.
     *
     * <p>Wire name: {@code validate_generic_threshold}; group: {@code thresholds}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param plan <code>plan</code> (str) Uppercased, matched exactly against GPLAN_CODE; unknown = result
     * notFound/plan (data, not an error).
     * @param behavior <code>behavior</code> (str) Uppercased; checked against the canonical behavior codes
     * only in the last stage.
     * @param sendToRedo <code>send_to_redo</code> (str) Case-insensitive Yes/No; checked only in the last stage.
     * @param options optional arguments ({@code null} = none); see {@link ValidateGenericThresholdOptions}
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String validateGenericThreshold(String configJson, String plan, String behavior, String sendToRedo, ValidateGenericThresholdOptions options) throws SzConfigToolException {
        Args wire = options == null ? new Args() : options.wire.copy();
        wire.str("plan", plan);
        wire.str("behavior", behavior);
        wire.str("send_to_redo", sendToRedo);
        return Invoker.json("validate_generic_threshold", configJson, wire);
    }

    /**
     * Check that a document has the top-level shape of a config (structure only).
     *
     * <p>Notes:
     * Checks, in order: valid JSON (JSON_PARSE); a G2_CONFIG key (MISSING_SECTION) that is an
     * object (INVALID_STRUCTURE); every recognised CFG_* section
     * (validation::EXPECTED_SECTIONS, 27 names) that is PRESENT is an array
     * (INVALID_STRUCTURE). Absent sections, other keys and cross-references are not checked.
     *
     * <p>Wire name: {@code validate_config}; group: {@code validation}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, MISSING_SECTION, INVALID_STRUCTURE; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static void validateConfig(String configJson) throws SzConfigToolException {
        Args wire = new Args();
        Invoker.unit("validate_config", configJson, wire);
    }

    /**
     * Get the configuration VERSION string (G2_CONFIG.CONFIG_BASE_VERSION.VERSION).
     *
     * <p>Notes:
     * Result is a JSON string (e.g. "4.4.0" in the template). Absent or non-string VERSION (or
     * any missing parent) = NOT_FOUND.
     *
     * <p>Wire name: {@code get_version}; group: {@code versioning}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String getVersion(String configJson) throws SzConfigToolException {
        Args wire = new Args();
        return Invoker.json("get_version", configJson, wire);
    }

    /**
     * Get COMPATIBILITY_VERSION.CONFIG_VERSION.
     *
     * <p>Notes:
     * Result is a JSON string (e.g. "11" in the template). Absent or non-string CONFIG_VERSION
     * (or any missing parent) = NOT_FOUND.
     *
     * <p>Wire name: {@code get_compatibility_version}; group: {@code versioning}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @return the result as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String getCompatibilityVersion(String configJson) throws SzConfigToolException {
        Args wire = new Args();
        return Invoker.json("get_compatibility_version", configJson, wire);
    }

    /**
     * Set COMPATIBILITY_VERSION.CONFIG_VERSION.
     *
     * <p>Notes:
     * Absent CONFIG_BASE_VERSION or COMPATIBILITY_VERSION = NOT_FOUND; COMPATIBILITY_VERSION
     * not an object = INVALID_CONFIG. TRAP: an absent G2_CONFIG is NOT an error — the config
     * is returned unchanged (re-serialized).
     *
     * <p>Wire name: {@code update_compatibility_version}; group: {@code versioning}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param newVersion <code>new_version</code> (str) Stored verbatim as a JSON string; no format validation.
     * @return the modified configuration JSON document (opaque)
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND, INVALID_CONFIG; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static String updateCompatibilityVersion(String configJson, String newVersion) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("new_version", newVersion);
        return Invoker.config("update_compatibility_version", configJson, wire);
    }

    /**
     * Result of {@link #verifyCompatibilityVersion} (named by the manifest's {@code tuple_names}).
     *
     * @param currentVersion the {@code current_version} value as JSON text
     * @param matches the {@code matches} value as JSON text
     */
    public record VerifyCompatibilityVersionRecord(String currentVersion, String matches) {
    }

    /**
     * Compare COMPATIBILITY_VERSION.CONFIG_VERSION with an expected value.
     *
     * <p>Notes:
     * Rust returns (current_version, matches) where the String is NOT a config, so it is
     * <code>json</code>; tuple_names makes the result the object {"current_version": "11",
     * "matches": true}. Absent CONFIG_VERSION = NOT_FOUND.
     *
     * <p>Wire name: {@code verify_compatibility_version}; group: {@code versioning}.
     *
     * @param configJson the configuration JSON document (opaque; never parsed here)
     * @param expectedVersion <code>expected_version</code> (str) Compared by exact, case-sensitive string equality.
     * @return the named result values, each as JSON text
     * @throws SzConfigToolException library reason codes: JSON_PARSE, NOT_FOUND; plus the universal INVALID_INPUT / MISSING_FIELD / INTERNAL wire errors
     */
    public static VerifyCompatibilityVersionRecord verifyCompatibilityVersion(String configJson, String expectedVersion) throws SzConfigToolException {
        Args wire = new Args();
        wire.str("expected_version", expectedVersion);
        String[] f = Invoker.fields(Invoker.result("verify_compatibility_version", "json", configJson, wire), "current_version", "matches");
        return new VerifyCompatibilityVersionRecord(f[0], f[1]);
    }
}
