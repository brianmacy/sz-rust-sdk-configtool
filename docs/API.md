# API Documentation

Complete reference for all modules and functions in sz_configtool_lib.

## Module Overview

### Core Infrastructure

#### `error`
Custom error types for the library.

**Types:**
- `SzConfigError` - Main error enum
- `Result<T>` - Type alias for `std::result::Result<T, SzConfigError>`

**Error Variants:**
- `JsonParse(String)` - JSON parsing/serialization errors
- `NotFound(String, String)` - Entity not found (type, identifier)
- `AlreadyExists(String, String)` - Entity already exists (type, identifier)
- `InvalidInput(String)` - Invalid input parameters
- `MissingField(String)` - Required field missing in JSON
- `DependencyExists(String)` - Cannot delete due to dependencies
- `InternalError(String)` - Internal processing error

#### `helpers`
Shared utility functions used across modules.

**Functions:**
- `get_next_id(config_json, section, id_field)` - Calculate next available ID
- `find_in_config_array(config_json, section, field, value)` - Find item in config array
- `add_to_config_array(config_json, section, item)` - Add item to config array
- `update_in_config_array(config_json, section, field, value, updates)` - Update existing item
- `delete_from_config_array(config_json, section, field, value)` - Delete item from array

---

## Data Management Modules

### `datasources` (7 functions)

Manage data sources (CFG_DSRC section).

**Functions:**

#### `add_data_source(config_json, dsrc_code, dsrc_id, dsrc_desc, retention_level) -> Result<String>`
Add a new data source to the configuration.

#### `delete_data_source(config_json, dsrc_code) -> Result<String>`
Delete a data source from the configuration.

#### `get_data_source(config_json, dsrc_code) -> Result<Value>`
Get details of a specific data source.

#### `list_data_sources(config_json) -> Result<Vec<Value>>`
List all data sources in the configuration.

#### `set_data_source(config_json, dsrc_code, dsrc_desc, retention_level) -> Result<String>`
Update an existing data source.

#### `set_data_source_id(config_json, old_id, new_id) -> Result<String>`
Change the ID of an existing data source.

#### `get_data_source_by_id(config_json, dsrc_id) -> Result<Value>`
Get data source details by ID.

---

### `attributes` (8 functions)

Manage attributes (CFG_ATTR section).

**Functions:**

#### `add_attribute(config_json, attr_code, feature, element, attr_class, ftype_code, felem_code, felem_required, default_value, advanced) -> Result<(String, Value)>`
Add a new attribute to the configuration.

#### `delete_attribute(config_json, attr_code) -> Result<String>`
Delete an attribute from the configuration.

#### `get_attribute(config_json, attr_code) -> Result<Value>`
Get details of a specific attribute.

#### `list_attributes(config_json) -> Result<Vec<Value>>`
List all attributes in the configuration.

#### `set_attribute(config_json, attr_code, attr_class, feature, element, required, default, advanced, internal) -> Result<String>`
Update an existing attribute.

#### `clone_attribute(config_json, source_attr, new_attr) -> Result<String>`
Clone an existing attribute with a new code.

#### `set_attribute_id(config_json, old_id, new_id) -> Result<String>`
Change the ID of an existing attribute.

#### `get_attribute_by_id(config_json, attr_id) -> Result<Value>`
Get attribute details by ID.

---

## Feature Management Modules

### `features` (24 functions)

Manage features and their configurations.

**Core Operations:**

#### `add_feature(config_json, ftype_code, element_list, fclass, behavior, candidates, comparison_threshold) -> Result<(String, Value)>`
Add a new feature with element list.

#### `delete_feature(config_json, ftype_code) -> Result<String>`
Delete a feature from configuration.

#### `get_feature(config_json, ftype_code) -> Result<Value>`
Get details of a specific feature.

#### `list_features(config_json) -> Result<Vec<Value>>`
List all features in configuration.

#### `set_feature(config_json, ftype_code, fclass, behavior, candidates, anonymize, version, show_in_match_key) -> Result<String>`
Update an existing feature.

**Feature Elements:**

#### `add_feature_element(config_json, ftype_code, felem_code, expressed) -> Result<String>`
Add an element to a feature.

#### `delete_feature_element(config_json, ftype_code, felem_code) -> Result<String>`
Remove an element from a feature.

#### `get_feature_element(config_json, ftype_code, felem_code) -> Result<Value>`
Get details of a feature element.

#### `list_feature_elements(config_json, ftype_code) -> Result<Vec<Value>>`
List all elements in a feature.

**Feature Comparisons:**

#### `add_feature_comparison(config_json, ftype_code, cfunc_id) -> Result<String>`
Add a comparison function to a feature.

#### `delete_feature_comparison(config_json, ftype_code, cfunc_id) -> Result<String>`
Remove a comparison function from a feature.

#### `list_feature_comparisons(config_json, ftype_code) -> Result<Vec<Value>>`
List all comparison functions for a feature.

**Feature Distinct Calls:**

#### `add_feature_distinct_call(config_json, ftype_code, dfcall_id) -> Result<String>`
Add a distinct call to a feature.

#### `delete_feature_distinct_call(config_json, ftype_code, dfcall_id) -> Result<String>`
Remove a distinct call from a feature.

#### `list_feature_distinct_calls(config_json, ftype_code) -> Result<Vec<Value>>`
List all distinct calls for a feature.

**Additional Operations:**

#### `clone_feature(config_json, source_ftype, new_ftype) -> Result<String>`
Clone an existing feature with a new code.

#### `set_feature_id(config_json, old_id, new_id) -> Result<String>`
Change the ID of an existing feature.

#### `get_feature_by_id(config_json, ftype_id) -> Result<Value>`
Get feature details by ID.

---

### `elements` (8 functions)

Manage elements (CFG_FELEM section).

#### `add_element(config_json, felem_code, felem_desc, data_type) -> Result<(String, Value)>`
Add a new element to the configuration.

#### `delete_element(config_json, felem_code) -> Result<String>`
Delete an element from the configuration.

#### `get_element(config_json, felem_code) -> Result<Value>`
Get details of a specific element.

#### `list_elements(config_json) -> Result<Vec<Value>>`
List all elements in the configuration.

#### `set_element(config_json, felem_code, felem_desc, data_type) -> Result<String>`
Update an existing element.

#### `clone_element(config_json, source_elem, new_elem) -> Result<String>`
Clone an existing element with a new code.

#### `set_element_id(config_json, old_id, new_id) -> Result<String>`
Change the ID of an existing element.

#### `get_element_by_id(config_json, felem_id) -> Result<Value>`
Get element details by ID.

---

## Configuration Modules

### `thresholds` (6 functions)

Manage comparison and generic thresholds.

#### `add_comparison_threshold(config_json, cfunc_id, same_score, close_score, likely_score, plausible_score, unlikely_score) -> Result<(String, Value)>`
Add a comparison threshold configuration.

#### `delete_comparison_threshold(config_json, cfunc_id) -> Result<String>`
Delete a comparison threshold.

#### `list_comparison_thresholds(config_json) -> Result<Vec<Value>>`
List all comparison thresholds.

#### `add_generic_threshold(config_json, gplan_id, behavior, ftype_code, candidate_cap, scoring_cap, send_to_redo) -> Result<(String, Value)>`
Add a generic threshold configuration.

#### `delete_generic_threshold(config_json, gplan_id, behavior, ftype_code) -> Result<String>`
Delete a generic threshold.

#### `list_generic_thresholds(config_json) -> Result<Vec<Value>>`
List all generic thresholds.

---

### `rules` (5 functions)

Manage entity resolution rules (CFG_ERRULE section).

#### `add_rule(config_json, errule_code, resolve, relate, ref_score, rtype_id, qual_erfrag_code, disq_erfrag_code, errule_tier) -> Result<(String, Value)>`
Add a new resolution rule.

#### `delete_rule(config_json, errule_code) -> Result<String>`
Delete a resolution rule.

#### `get_rule(config_json, errule_code) -> Result<Value>`
Get details of a specific rule.

#### `list_rules(config_json) -> Result<Vec<Value>>`
List all resolution rules.

#### `set_rule(config_json, errule_code, resolve, relate, ref_score, qual_erfrag, disq_erfrag, rtype_id) -> Result<String>`
Update an existing rule.

---

### `fragments` (5 functions)

Manage rule fragments (CFG_ERFRAG section).

#### `add_fragment(config_json, erfrag_code, erfrag_source, erfrag_depends) -> Result<(String, Value)>`
Add a new rule fragment.

#### `delete_fragment(config_json, erfrag_code) -> Result<String>`
Delete a rule fragment.

#### `get_fragment(config_json, erfrag_code) -> Result<Value>`
Get details of a specific fragment.

#### `list_fragments(config_json) -> Result<Vec<Value>>`
List all rule fragments.

#### `set_fragment(config_json, erfrag_code, erfrag_source, erfrag_depends) -> Result<String>`
Update an existing fragment.

---

### `generic_plans` (4 functions)

Manage generic plans (CFG_GPLAN section).

#### `set_generic_plan(config_json, gplan_id, max_candidates, max_scoring_candidates) -> Result<String>`
Update generic plan settings.

#### `clone_generic_plan(config_json, source_id, new_id) -> Result<String>`
Clone an existing generic plan.

#### `delete_generic_plan(config_json, gplan_id) -> Result<String>`
Delete a generic plan.

#### `get_generic_plan(config_json, gplan_id) -> Result<Value>`
Get generic plan details.

---

### `hashes` (4 functions)

Manage name and SSN hash configurations.

#### `add_name_hash(config_json, hash_code, hash_desc) -> Result<(String, Value)>`
Add a name hash configuration.

#### `delete_name_hash(config_json, hash_code) -> Result<String>`
Delete a name hash configuration.

#### `add_ssn_hash(config_json, hash_code, hash_desc) -> Result<(String, Value)>`
Add an SSN hash configuration.

#### `delete_ssn_hash(config_json, hash_code) -> Result<String>`
Delete an SSN hash configuration.

---

## Function Modules

### `functions/standardize` (6 functions)

Manage standardization functions (CFG_SFUNC).

#### `add_standardize_function(config_json, sfunc_code, connect_str, sfunc_desc, language) -> Result<(String, Value)>`
Add a standardization function.

#### `delete_standardize_function(config_json, sfunc_code) -> Result<String>`
Delete a standardization function.

#### `get_standardize_function(config_json, sfunc_code) -> Result<Value>`
Get standardization function details.

#### `list_standardize_functions(config_json) -> Result<Vec<Value>>`
List all standardization functions.

#### `set_standardize_function(config_json, sfunc_code, updates_json) -> Result<String>`
Update standardization function settings.

#### `get_standardize_function_by_id(config_json, sfunc_id) -> Result<Value>`
Get standardization function by ID.

---

### `functions/expression` (6 functions)

Manage expression functions (CFG_EFUNC).

#### `add_expression_function(config_json, efunc_code, connect_str, efunc_desc, language) -> Result<(String, Value)>`
Add an expression function.

#### `delete_expression_function(config_json, efunc_code) -> Result<String>`
Delete an expression function.

#### `get_expression_function(config_json, efunc_code) -> Result<Value>`
Get expression function details.

#### `list_expression_functions(config_json) -> Result<Vec<Value>>`
List all expression functions.

#### `set_expression_function(config_json, efunc_code, updates_json) -> Result<String>`
Update expression function settings.

#### `get_expression_function_by_id(config_json, efunc_id) -> Result<Value>`
Get expression function by ID.

---

### `functions/comparison` (7 functions)

Manage comparison functions (CFG_CFUNC).

#### `add_comparison_function(config_json, cfunc_code, connect_str, cfunc_desc, language, anon_support) -> Result<(String, Value)>`
Add a comparison function.

#### `delete_comparison_function(config_json, cfunc_code) -> Result<String>`
Delete a comparison function.

#### `get_comparison_function(config_json, cfunc_code) -> Result<Value>`
Get comparison function details.

#### `list_comparison_functions(config_json) -> Result<Vec<Value>>`
List all comparison functions.

#### `set_comparison_function(config_json, cfunc_code, updates_json) -> Result<String>`
Update comparison function settings.

#### `get_comparison_function_by_id(config_json, cfunc_id) -> Result<Value>`
Get comparison function by ID.

#### `get_comparison_function_call(config_json, cfunc_code) -> Result<Value>`
Get comparison function call details.

---

### `functions/distinct` (6 functions)

Manage distinct functions (CFG_DFUNC).

#### `add_distinct_function(config_json, dfunc_code, connect_str, dfunc_desc, language) -> Result<(String, Value)>`
Add a distinct function.

#### `delete_distinct_function(config_json, dfunc_code) -> Result<String>`
Delete a distinct function.

#### `get_distinct_function(config_json, dfunc_code) -> Result<Value>`
Get distinct function details.

#### `list_distinct_functions(config_json) -> Result<Vec<Value>>`
List all distinct functions.

#### `set_distinct_function(config_json, dfunc_code, updates_json) -> Result<String>`
Update distinct function settings.

#### `get_distinct_function_by_id(config_json, dfunc_id) -> Result<Value>`
Get distinct function by ID.

---

### `functions/matching` (1 function)

Manage matching functions (CFG_RTYPE).

#### `list_matching_functions(config_json) -> Result<Vec<Value>>`
List all matching functions.

---

## Call Modules

### `calls/standardize` (8 functions)

Manage standardize calls and BOM (CFG_SFCALL, CFG_SBOM).

#### `add_standardize_call(config_json, ftype_code, felem_code, exec_order, sfunc_id) -> Result<(String, Value)>`
Add a standardize call.

#### `delete_standardize_call(config_json, ftype_code, felem_code, sfunc_id) -> Result<String>`
Delete a standardize call.

#### `get_standardize_call(config_json, ftype_code, felem_code, sfunc_id) -> Result<Value>`
Get standardize call details.

#### `list_standardize_calls(config_json) -> Result<Vec<Value>>`
List all standardize calls.

#### `add_standardize_call_bom(config_json, ftype_code, felem_code, exec_order, standardize_function) -> Result<String>`
Add standardize call by function name.

#### `delete_standardize_call_bom(config_json, ftype_code, felem_code, standardize_function) -> Result<String>`
Delete standardize call by function name.

#### `list_standardize_call_boms(config_json) -> Result<Vec<Value>>`
List standardize call BOMs with resolved names.

#### `get_standardize_call_bom(config_json, ftype_code, felem_code, standardize_function) -> Result<Value>`
Get standardize call BOM details.

---

### `calls/expression` (8 functions)

Manage expression calls and BOM (CFG_EFCALL, CFG_EFBOM).

#### `add_expression_call(config_json, ftype_code, felem_code, exec_order, efunc_id) -> Result<(String, Value)>`
Add an expression call.

#### `delete_expression_call(config_json, ftype_code, felem_code, efunc_id) -> Result<String>`
Delete an expression call.

#### `get_expression_call(config_json, ftype_code, felem_code, efunc_id) -> Result<Value>`
Get expression call details.

#### `list_expression_calls(config_json) -> Result<Vec<Value>>`
List all expression calls.

#### `add_expression_call_bom(config_json, ftype_code, exec_order, expression_function) -> Result<String>`
Add expression call by function name.

#### `delete_expression_call_bom(config_json, ftype_code, expression_function) -> Result<String>`
Delete expression call by function name.

#### `list_expression_call_boms(config_json) -> Result<Vec<Value>>`
List expression call BOMs with resolved names.

#### `get_expression_call_bom(config_json, ftype_code, expression_function) -> Result<Value>`
Get expression call BOM details.

---

### `calls/comparison` (8 functions)

Manage comparison calls and BOM (CFG_CFCALL, CFG_CFBOM).

#### `add_comparison_call(config_json, ftype_code, cfunc_id) -> Result<(String, Value)>`
Add a comparison call.

#### `delete_comparison_call(config_json, ftype_code, cfunc_id) -> Result<String>`
Delete a comparison call.

#### `get_comparison_call(config_json, ftype_code, cfunc_id) -> Result<Value>`
Get comparison call details.

#### `list_comparison_calls(config_json) -> Result<Vec<Value>>`
List all comparison calls.

#### `add_comparison_call_bom(config_json, ftype_code, comparison_function) -> Result<String>`
Add comparison call by function name.

#### `delete_comparison_call_bom(config_json, ftype_code, comparison_function) -> Result<String>`
Delete comparison call by function name.

#### `list_comparison_call_boms(config_json) -> Result<Vec<Value>>`
List comparison call BOMs with resolved names.

#### `get_comparison_call_bom(config_json, ftype_code, comparison_function) -> Result<Value>`
Get comparison call BOM details.

---

### `calls/distinct` (8 functions)

Manage distinct calls and BOM (CFG_DFCALL, CFG_DFBOM).

#### `add_distinct_call(config_json, ftype_code, dfunc_id) -> Result<(String, Value)>`
Add a distinct call.

#### `delete_distinct_call(config_json, ftype_code, dfunc_id) -> Result<String>`
Delete a distinct call.

#### `get_distinct_call(config_json, ftype_code, dfunc_id) -> Result<Value>`
Get distinct call details.

#### `list_distinct_calls(config_json) -> Result<Vec<Value>>`
List all distinct calls.

#### `add_distinct_call_bom(config_json, ftype_code, distinct_function) -> Result<String>`
Add distinct call by function name.

#### `delete_distinct_call_bom(config_json, ftype_code, distinct_function) -> Result<String>`
Delete distinct call by function name.

#### `list_distinct_call_boms(config_json) -> Result<Vec<Value>>`
List distinct call BOMs with resolved names.

#### `get_distinct_call_bom(config_json, ftype_code, distinct_function) -> Result<Value>`
Get distinct call BOM details.

---

## System Management Modules

### `config_sections` (6 functions)

Manage G2_CONFIG sections.

#### `get_config_section(config_json, section_name) -> Result<Value>`
Get a specific configuration section.

#### `set_config_section(config_json, section_name, section_data) -> Result<String>`
Update a configuration section.

#### `list_config_sections(config_json) -> Result<Vec<String>>`
List all configuration section names.

#### `delete_config_section(config_json, section_name) -> Result<String>`
Delete a configuration section.

#### `add_config_section(config_json, section_name, section_data) -> Result<String>`
Add a new configuration section.

#### `clone_config_section(config_json, source_section, new_section) -> Result<String>`
Clone a configuration section.

---

### `system_params` (2 functions)

Manage system parameters.

#### `set_system_parameter(config_json, param_name, param_value) -> Result<String>`
Set a system parameter value.

#### `get_system_parameter(config_json, param_name) -> Result<String>`
Get a system parameter value.

---

### `versioning` (4 functions)

Manage configuration versioning.

#### `update_config_version(config_json) -> Result<String>`
Increment the configuration version.

#### `get_config_version(config_json) -> Result<i64>`
Get the current configuration version.

#### `set_config_version(config_json, version) -> Result<String>`
Set the configuration version.

#### `verify_version_compatibility(config_json, required_version) -> Result<bool>`
Check if configuration meets version requirement.

---

## Usage Patterns

### Basic CRUD Operations

```rust
// Add
let config = module::add_entity(&config, "CODE", param1, param2)?;

// Get
let entity = module::get_entity(&config, "CODE")?;

// List
let entities = module::list_entities(&config)?;

// Update
let config = module::set_entity(&config, "CODE", new_param1, new_param2)?;

// Delete
let config = module::delete_entity(&config, "CODE")?;
```

### Working with Tuples

Some functions return tuples `(String, Value)`:

```rust
let (modified_config, created_entity) = module::add_entity(&config, params)?;

// Use the modified config for next operation
let config = modified_config;

// Access the created entity details
println!("Created ID: {}", created_entity["ENTITY_ID"]);
```

### Error Handling

```rust
match module::operation(&config, params) {
    Ok(result) => {
        // Success
    }
    Err(SzConfigError::NotFound(entity_type, id)) => {
        eprintln!("{} '{}' not found", entity_type, id);
    }
    Err(SzConfigError::AlreadyExists(entity_type, id)) => {
        eprintln!("{} '{}' already exists", entity_type, id);
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}
```

---

## See Also

- [FFI Guide](FFI_GUIDE.md) - C FFI interface documentation
- [Contributing](CONTRIBUTING.md) - Contribution guidelines
- [README](../README.md) - Quick start and examples
