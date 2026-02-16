# sz_configtool_lib API Documentation

**Last Updated:** 2026-02-13 (Sessions 96-99 migration complete)
**Library Version:** 0.1.0
**Authoritative Source:** Run `cargo doc --open` for complete rustdoc with all signatures

---

## Important Notes

**API Uses Parameter Structs** (as of Sessions 96-99):
- All `add_*` and `set_*` functions use typed parameter structs (e.g., `AddDataSourceParams`, `SetFeatureParams`)
- This replaces older flat parameter signatures
- See rustdoc or source code for exact struct definitions

**Migration Complete:**
- This library is the production backend for sz_configtool_rust
- All 120 commands use this library via backend adapter
- Extensive validations, domain checks, and special case handling added

---

## Error Types

### `SzConfigError`

**Module:** `error`

Main error enum with 9 variants:

- `JsonParse(String)` - JSON parsing errors
- `NotFound(String)` - Entity not found
- `AlreadyExists(String)` - Duplicate entity
- `InvalidInput(String)` - Invalid parameters or validation failure
- `MissingSection(String)` - Required config section missing
- `InvalidStructure(String)` - Config structure invalid
- `MissingField(String)` - Required field missing
- `InvalidConfig(String)` - Configuration state invalid
- `NotImplemented(String)` - Feature not yet implemented

**Result Type:** `type Result<T> = std::result::Result<T, SzConfigError>`

---

## Core Data Modules

### Datasources (`datasources`)

**Functions:**
- `add_data_source(config_json, params: AddDataSourceParams) -> Result<String>`
- `delete_data_source(config_json, code: &str) -> Result<String>`
- `get_data_source(config_json, code: &str) -> Result<Value>`
- `list_data_sources(config_json) -> Result<Vec<Value>>`
- `set_data_source(config_json, params: SetDataSourceParams) -> Result<String>`

**Key Structs:**
- `AddDataSourceParams` - Fields: code, retention_level, conversational, reliability
- `SetDataSourceParams` - Fields: code, retention_level, conversational, reliability

**Validations:** retentionLevel domain, conversational domain, system datasource protection (ID ≤ 2)

### Elements (`elements`)

**Functions:**
- `add_element(config_json, params: AddElementParams) -> Result<String>`
- `delete_element(config_json, code: &str) -> Result<String>`
- `get_element(config_json, code: &str) -> Result<Value>`
- `list_elements(config_json) -> Result<Vec<Value>>`
- `set_element(config_json, params: SetElementParams) -> Result<String>`
- `set_feature_element(config_json, params: SetFeatureElementParams) -> Result<String>`

**Key Structs:**
- `AddElementParams` - Fields: code, description, data_type, tokenized
- `SetElementParams` - Fields: code, description, data_type, tokenized
- `SetFeatureElementParams` - Fields: feature, element, display, derived

**Validations:** datatype domain, tokenized domain, linkage checks

### Attributes (`attributes`)

**Functions:**
- `add_attribute(config_json, params: AddAttributeParams) -> Result<(String, Value)>`
- `delete_attribute(config_json, code: &str) -> Result<String>`
- `get_attribute(config_json, code: &str) -> Result<Value>`
- `list_attributes(config_json) -> Result<Vec<Value>>`
- `set_attribute(config_json, params: SetAttributeParams) -> Result<String>`

**Key Structs:**
- `AddAttributeParams` - Fields: code, feature, element, class, default, internal, required
- `SetAttributeParams` - Fields: code, internal, required, default

**Validations:** CLASS default ("OTHER"), feature/element existence checks

### Features (`features`)

**Functions:**
- `add_feature(config_json, params: AddFeatureParams) -> Result<String>`
- `delete_feature(config_json, code: &str) -> Result<String>`
- `get_feature(config_json, code: &str) -> Result<Value>`
- `list_features(config_json) -> Result<Vec<Value>>`
- `set_feature(config_json, params: SetFeatureParams) -> Result<String>` ⭐ Extended Session 98
- `add_feature_comparison(config_json, params: AddFeatureComparisonParams) -> Result<String>` ⭐ Used Session 99

**Key Structs:**
- `AddFeatureParams` - Fields: feature, element_list, class, behavior, etc.
- `SetFeatureParams` ⭐ - Fields: feature, candidates, anonymize, derived, history, matchkey, behavior, class, version, rtype_id
- `AddFeatureComparisonParams` ⭐ - Fields: feature_code, element_code, exec_order, display_level, display_delim, derived

**Validations (Session 98):**
- CANDIDATES domain ("Yes", "No") with case normalization
- MATCHKEY domain ("Yes", "No", "Confirm", "Denial") with case normalization
- No-changes detection

---

## Rules (`rules`) ⭐ Session 97-98

**Functions:**
- `add_rule(config_json, id: i64, rule_config: &Value) -> Result<(String, i64)>`
- `delete_rule(config_json, code: &str) -> Result<String>`
- `get_rule(config_json, code: &str) -> Result<Value>`
- `list_rules(config_json) -> Result<Vec<Value>>`
- `set_rule(config_json, params: SetRuleParams) -> Result<String>` ⭐ Extended Session 97-98

**Key Structs:**
- `SetRuleParams` ⭐ - Fields: code, resolve, relate, rtype_id, fragment, disqualifier, tier

**Critical Features (Session 98):**
- ID preservation fix (set_rule was losing ERRULE_ID)
- Auto-correction: resolve="Yes" forces rtype_id=1
- RTYPE_ID update logic when resolve is modified

---

## Fragments (`fragments`) ⭐ Session 98

**Functions:**
- `add_fragment(config_json, fragment_config: &Value) -> Result<String>`
- `delete_fragment(config_json, code: &str) -> Result<String>`
- `get_fragment(config_json, code: &str) -> Result<Value>` ⭐ Session 98
- `list_fragments(config_json) -> Result<Vec<Value>>`
- `set_fragment(config_json, code: &str, fragment_config: &Value) -> Result<String>`

**Key Features (Session 98):**
- get_fragment transformation (lowercase keys matching list_fragments)
- Duplicate CODE validation
- Supports ID-based lookup (get by ID or CODE)

---

## Thresholds (`thresholds`) ⭐ Session 98

**Functions:**
- `add_comparison_threshold(config_json, params: AddComparisonThresholdParams) -> Result<String>` ⭐
- `delete_comparison_threshold(config_json, cfunc_code: &str, ftype_code: &str) -> Result<String>`
- `set_comparison_threshold(config_json, params: SetComparisonThresholdParams) -> Result<String>` ⭐
- `list_comparison_thresholds(config_json) -> Result<Vec<Value>>`
- `add_generic_threshold(config_json, params: AddGenericThresholdParams) -> Result<String>`
- `delete_generic_threshold(config_json, params: DeleteGenericThresholdParams) -> Result<String>`
- `set_generic_threshold(config_json, params: SetGenericThresholdParams) -> Result<String>`
- `list_generic_thresholds(config_json) -> Result<Vec<Value>>`

**Key Structs (Session 98):**
- `AddComparisonThresholdParams` - Includes: cfunc_code, ftype_code, cfunc_rtnval, exec_order, score fields
- `SetComparisonThresholdParams` ⭐ - Includes: cfunc_code, ftype_code, **cfunc_rtnval** (for unique identification), **exec_order**, score fields
- `AddGenericThresholdParams` - Includes: gplan_code, behavior_code, feature, send_to_redo, caps
- `SetGenericThresholdParams` - Similar to add

**Critical Features (Session 98):**
- Special case: ftype_code="all" → ftype_id=0
- Unique identification: (function, feature, score_name) for thresholds with same function+feature
- Field order fix (unlikelyScore moved to end)
- exec_order and cfunc_rtnval support added

---

## Generic Plans (`generic_plans`) ⭐ Session 98

**Functions:**
- `clone_generic_plan(config_json, source_code: &str, new_code: &str, new_desc: Option<&str>) -> Result<String>`
- `delete_generic_plan(config_json, code: &str) -> Result<String>` ⭐
- `list_generic_plans(config_json) -> Result<Vec<Value>>`

**Critical Features (Session 98):**
- System plan protection: GPLAN_ID ≤ 2 (SEARCH, INGEST) cannot be deleted
- Cascade delete (automatically handled)
- Duplicate CODE validation

---

## Calls (`calls`) ⭐ Sessions 96-97

### Comparison Calls (`calls::comparison`)

**Functions:**
- `add_comparison_call(config_json, params: AddComparisonCallParams) -> Result<(String, Value)>`
- `delete_comparison_call(config_json, cfcall_id: i64) -> Result<String>`
- `get_comparison_call(config_json, cfcall_id: i64) -> Result<Value>`
- `list_comparison_calls(config_json) -> Result<Vec<Value>>`
- `add_comparison_call_element(config_json, cfcall_id: i64, params: ComparisonCallElementParams) -> Result<(String, Value)>`
- `delete_comparison_call_element(config_json, cfcall_id: i64, params: DeleteComparisonCallElementParams) -> Result<String>` ⭐ Used Session 99

**Key Structs:**
- `AddComparisonCallParams` - Fields: ftype_code, cfunc_code, element_list
- `ComparisonCallElementParams` - Fields: ftype_id, felem_id, exec_order
- `DeleteComparisonCallElementParams` ⭐ - Fields: ftype_id, felem_id, exec_order

**Critical Features (Session 97):**
- FBOM lookup bug fixed (FELEM_CODE → FELEM_ID)
- Duplicate detection excludes EXEC_ORDER
- ftype_id=-1 validation (rejects invalid sentinel)

### Expression Calls (`calls::expression`)

**Functions:**
- `add_expression_call(config_json, params: AddExpressionCallParams) -> Result<(String, Value)>`
- `delete_expression_call(config_json, efcall_id: i64) -> Result<String>`
- `get_expression_call(config_json, efcall_id: i64) -> Result<Value>`
- `list_expression_calls(config_json) -> Result<Vec<Value>>`
- `add_expression_call_element(config_json, efcall_id: i64, params: ExpressionCallElementParams) -> Result<(String, Value)>`
- `delete_expression_call_element(config_json, efcall_id: i64, key: ExpressionCallElementKey) -> Result<String>` ⭐ Used Session 99

**Key Structs:**
- `AddExpressionCallParams` - Fields: ftype_code, efunc_code, element_list, expression_feature, is_virtual
- `ExpressionCallElementParams` - Fields: ftype_id, felem_id, exec_order, required
- `ExpressionCallElementKey` ⭐ - Fields: ftype_id, felem_id, exec_order

### Distinct Calls (`calls::distinct`)

**Functions:**
- `add_distinct_call(config_json, params: AddDistinctCallParams) -> Result<(String, Value)>`
- `delete_distinct_call(config_json, dfcall_id: i64) -> Result<String>`
- `get_distinct_call(config_json, dfcall_id: i64) -> Result<Value>`
- `list_distinct_calls(config_json) -> Result<Vec<Value>>`
- `add_distinct_call_element(config_json, dfcall_id: i64, params: DistinctCallElementParams) -> Result<(String, Value)>`
- `delete_distinct_call_element(config_json, params: DeleteDistinctCallElementParams) -> Result<String>` ⭐ Used Session 99

**Key Structs:**
- `AddDistinctCallParams` - Fields: ftype_code, dfunc_code, element_list
- `DistinctCallElementParams` - Fields: ftype_id, felem_id, exec_order
- `DeleteDistinctCallElementParams` ⭐ - Fields: dfcall_id, ftype_id, felem_id, exec_order

### Standardize Calls (`calls::standardize`)

**Functions:**
- `add_standardize_call(config_json, params: AddStandardizeCallParams) -> Result<(String, Value)>`
- `delete_standardize_call(config_json, sfcall_id: i64) -> Result<String>`
- `get_standardize_call(config_json, sfcall_id: i64) -> Result<Value>`
- `list_standardize_calls(config_json) -> Result<Vec<Value>>`

---

## Functions (`functions`)

### Standardize Functions (`functions::standardize`)
- `add_standardize_function(config_json, code: &str, params: AddStandardizeFunctionParams) -> Result<(String, Value)>`
- `delete_standardize_function(config_json, code: &str) -> Result<(String, Value)>`
- `list_standardize_functions(config_json) -> Result<Vec<Value>>`

### Comparison Functions (`functions::comparison`)
- `add_comparison_function(config_json, code: &str, params: AddComparisonFunctionParams) -> Result<(String, Value)>`
- `delete_comparison_function(config_json, code: &str) -> Result<(String, Value)>`
- `list_comparison_functions(config_json) -> Result<Vec<Value>>`

### Expression Functions (`functions::expression`)
- `add_expression_function(config_json, code: &str, params: AddExpressionFunctionParams) -> Result<String>`
- `list_expression_functions(config_json) -> Result<Vec<Value>>`

### Distinct Functions (`functions::distinct`)
- `add_distinct_function(config_json, code: &str, params: AddDistinctFunctionParams) -> Result<(String, Value)>`
- `list_distinct_functions(config_json) -> Result<Vec<Value>>`

### Matching Functions (`functions::matching`) ⚠️ Not Yet Implemented

**Module:** `functions::matching`

**Status:** Placeholder module - all functions return `NotImplemented` error

**Functions:**
- `list_matching_functions(config_json) -> Result<Vec<Value>>`

**Note:** These functions manage matching functions (CFG_RTYPE) but are not yet implemented. Awaiting CLI command completion.

---

## Configuration Management

### Config Sections (`config_sections`)

**Module:** `config_sections`

Manage top-level G2_CONFIG sections (add, remove, query configuration sections).

**Functions:**
- `add_config_section(config_json, section_name) -> Result<String>`
- `delete_config_section(config_json, section_name) -> Result<String>`
- `get_config_section(config_json, section_name) -> Result<Value>`
- `list_config_sections(config_json) -> Result<Vec<String>>`
- `set_config_section(config_json, section_name, section_data) -> Result<String>`
- `clone_config_section(config_json, source_section, new_section) -> Result<String>`

**Operations:** Add/delete/get/list/update/clone configuration sections within G2_CONFIG.

---

## Behavior & Versioning

### Behavior Overrides (`behavior_overrides`) ⭐ Session 98

**Functions:**
- `add_behavior_override(config_json, params: AddBehaviorOverrideParams) -> Result<String>`

**Struct:**
- `AddBehaviorOverrideParams` - Fields: feature_code, usage_type, behavior

### Versioning (`versioning`) ⭐ Session 98

**Functions:**
- `get_compatibility_version(config_json) -> Result<String>`
- `update_compatibility_version(config_json, new_version: &str) -> Result<String>`
- `verify_compatibility_version(config_json, expected_version: &str) -> Result<(String, bool)>`

---

## Helpers (`helpers`)

**Commonly Used:**
- `get_next_id(config: &Value, section: &str, id_field: &str, seed: i64) -> Result<i64>`
- `get_next_id_with_min(array: &[Value], id_field: &str, min_value: i64) -> Result<i64>`
- `find_in_config_array(config_json, section, field, value) -> Result<Option<Value>>`
- `add_to_config_array(config_json, section, record: Value) -> Result<String>`
- `update_in_config_array(config_json, section, criteria, updates) -> Result<String>`
- `delete_from_config_array(config_json, section, field, value) -> Result<String>`

**Lookup Helpers:**
- `lookup_feature_id(config_json, code: &str) -> Result<i64>`
- `lookup_element_id(config_json, code: &str) -> Result<i64>`

---

## Key API Changes (Sessions 96-99)

### Session 96: Foundation
- Converted to parameter struct pattern
- Added domain validations (retentionLevel, conversational, datatype, tokenized)
- Added system datasource protection (ID ≤ 2)

### Session 97: Calls & Rules
- Extended SetRuleParams (fragment, disqualifier, tier fields)
- Changed add_rule signature (ID required parameter)
- Fixed FBOM lookup (FELEM_CODE → FELEM_ID)
- Added ftype_id validation (rejects < 0)

### Session 98: Advanced Modules
- SetFeatureParams: Added CANDIDATES, MATCHKEY domain validation with case normalization
- SetComparisonThresholdParams: Added cfunc_rtnval, exec_order for unique identification
- Special case handling: ftype_code="all" → ftype_id=0
- get_fragment transformation (consistent with list_fragments)
- System plan protection (GPLAN_ID ≤ 2)
- ID preservation fixes (set_rule, set_fragment)

### Session 99: Quality & Completion
- Suppressed 4 dead_code warnings (3 unused lookup functions, 1 unused variable)
- Applied 28 clippy inline format fixes in examples
- Strict clippy passing (-D warnings)
- All tests passing (with updated expectations)

---

## Usage Pattern

```rust
use sz_configtool_lib::datasources::{self, AddDataSourceParams};

// Add a data source
let config = datasources::add_data_source(
    &config,
    AddDataSourceParams {
        code: "CUSTOMERS",
        retention_level: Some("Remember"),
        conversational: Some("Yes"),
        reliability: None,
    }
)?;

// List all data sources
let sources = datasources::list_data_sources(&config)?;

// Each source has lowercase transformed keys:
// {"id": 1, "dataSource": "CUSTOMERS", "retentionLevel": "Remember", ...}
```

---

## For Complete Details

**Run:**
```bash
cargo doc --open
```

This will generate complete rustdoc documentation with:
- All function signatures
- All parameter struct field definitions
- Return types
- Error conditions
- Examples

**Updated:** 2026-02-13 to reflect Sessions 96-99 migration work

---

## See Also

- [FFI Guide](FFI_GUIDE.md) - C FFI interface documentation
- [Contributing](CONTRIBUTING.md) - Contribution guidelines
- [README](../README.md) - Quick start and examples
