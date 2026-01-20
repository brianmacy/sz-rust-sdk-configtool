# Parameter Struct Refactoring Progress

## Rule
ANY function with more than 2 total parameters (config + 1 other) must be refactored to use a parameter struct.

## Completed ✅

### 1. features.rs (Partial)
- `add_feature`: 16 params → 3 params (config + feature_code + params)
- `set_feature`: 12 params → 3 params (config + feature_code + params)

### 2. attributes.rs
- `add_attribute`: 9 params → 3 params (config + code + params)

### 3. datasources.rs
- `add_data_source`: 6 params → 3 params (config + code + params)

### 4. elements.rs
- `set_feature_element`: 8 params → 4 params (config + felem_code + ftype_code + params)

### 5. thresholds.rs
- `add_comparison_threshold`: 11 params → 4 params (config + cfunc_id + cfunc_rtnval + params)
- `add_generic_threshold`: 7 params → 3 params (config + plan + params) ✅ FIXED

### 6. calls/expression.rs ✅ COMPLETE
- `add_expression_call`: 13 params → 2 params (config + params)
- `add_expression_call_element`: 6 params → 3 params (config + efcall_id + params)
- `delete_expression_call_element`: 5 params → 3 params (config + efcall_id + key)
- `set_expression_call_element`: 6 params → 3 params (config + efcall_id + key + updates)

## Remaining - Critical Priority (8+ params)

### features.rs
- `add_feature_comparison`: 7 params → 4 params
  - Signature: `(config, ftype_id, felem_id, params: AddFeatureComparisonParams)`
  - Params struct needs: exec_order, display_level, display_delim, derived

- `add_feature_comparison_element`: 8 params → 4 params
  - Same as above (appears to be duplicate function)

## Remaining - High Priority (6-7 params)

### functions/comparison.rs
- `add_comparison_function`: 7 params → 3 params
  - Signature: `(config, cfunc_code, params)`
  - Params: cfunc_rtnval, ref_score, lib_feat_id, lib_func_id, anon_func_id

- `set_comparison_function`: 7 params → 3 params
  - Same param struct as add

### functions/standardize.rs
- `add_standardize_function`: 6 params → 3 params
  - Signature: `(config, sfunc_code, params)`
  - Params: connect_str, language, plugin

- `set_standardize_function`: 6 params → 3 params
  - Same param struct as add

### functions/expression.rs
- `add_expression_function`: 6 params → 3 params
  - Signature: `(config, efunc_code, params)`
  - Params: efunc_code, connect_str, language, plugin

- `set_expression_function`: 6 params → 3 params
  - Same param struct as add

### functions/distinct.rs
- `add_distinct_function`: 6 params → 3 params
  - Signature: `(config, dfunc_code, params)`
  - Params: connect_str, language, plugin

- `set_distinct_function`: 6 params → 3 params
  - Same param struct as add

### features.rs
- `add_feature_distinct_call_element`: 6 params → 4 params
  - Signature: `(config, felem_code, ftype_code, params)`
  - Params: dfunc_code, exec_order

## Remaining - Medium Priority (6 params, element operations)

### calls/comparison.rs
- `add_comparison_call`: 7 params → 3 params
- `add_comparison_call_element`: 6 params → 3 params
- `set_comparison_call_element`: 6 params → 3 params
- `delete_comparison_call_element`: 5 params → 3 params

### calls/distinct.rs
- `add_distinct_call`: 6 params → 3 params
- `add_distinct_call_element`: 6 params → 3 params
- `set_distinct_call_element`: 6 params → 3 params
- `delete_distinct_call_element`: 5 params → 3 params

### calls/standardize.rs
- `add_standardize_call`: 7 params → 3 params
- `add_standardize_call_element`: 6 params → 3 params
- `set_standardize_call_element`: 6 params → 3 params
- `delete_standardize_call_element`: 5 params → 3 params

## Remaining - Low Priority (5 params)

### behavior_overrides.rs
- `add_behavior_override`: 5 params → 3 params
  - Signature: `(config, ftype_code, params)`
  - Params: behavior, exec_order, same_score

## Refactoring Pattern

### 1. Create Parameter Struct

```rust
/// Parameters for adding XYZ
#[derive(Debug, Clone)]
pub struct AddXyzParams<'a> {
    pub required_field: &'a str,
    pub optional_field: Option<i64>,
}

impl<'a> AddXyzParams<'a> {
    pub fn new(required_field: &'a str) -> Self {
        Self {
            required_field,
            optional_field: None,
        }
    }
}

impl<'a> TryFrom<&'a Value> for AddXyzParams<'a> {
    type Error = SzConfigError;

    fn try_from(json: &'a Value) -> Result<Self> {
        let required_field = json
            .get("requiredField")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SzConfigError::MissingField("requiredField".to_string()))?;

        Ok(Self {
            required_field,
            optional_field: json.get("optionalField").and_then(|v| v.as_i64()),
        })
    }
}
```

### 2. Update Function Signature

```rust
// BEFORE:
pub fn add_xyz(
    config: &str,
    key: &str,
    field1: &str,
    field2: Option<i64>,
    field3: Option<&str>,
) -> Result<String> {
    // ...
}

// AFTER:
pub fn add_xyz(
    config: &str,
    key: &str,
    params: AddXyzParams,
) -> Result<String> {
    // Reference params.field1, params.field2, etc.
}
```

### 3. Update Callers in command_processor.rs

```rust
// BEFORE:
"addXyz" => {
    let key = get_str_param(params, "key")?;
    let field1 = get_str_param(params, "field1")?;
    let field2 = params.get("field2").and_then(|v| v.as_i64());
    let field3 = get_opt_str_param(params, "field3");

    crate::module::add_xyz(config, key, field1, field2, field3)
}

// AFTER:
"addXyz" => {
    let key = get_str_param(params, "key")?;
    let xyz_params = crate::module::AddXyzParams::try_from(params)?;

    crate::module::add_xyz(config, key, xyz_params)
}
```

### 4. Update FFI Wrappers

```rust
// BEFORE:
#[no_mangle]
pub extern "C" fn SzConfigTool_addXyz(
    config: *const c_char,
    key: *const c_char,
    field1: *const c_char,
    field2: i64,
    field3: *const c_char,
) -> SzConfigTool_result {
    // ... extract all params ...
    handle_result!(crate::module::add_xyz(
        config_str,
        key_str,
        field1_str,
        Some(field2),
        field3_opt,
    ))
}

// AFTER:
#[no_mangle]
pub extern "C" fn SzConfigTool_addXyz(
    config: *const c_char,
    key: *const c_char,
    field1: *const c_char,
    field2: i64,
    field3: *const c_char,
) -> SzConfigTool_result {
    // ... extract all params ...
    handle_result!(crate::module::add_xyz(
        config_str,
        key_str,
        crate::module::AddXyzParams {
            required_field: field1_str,
            optional_field: if field2 >= 0 { Some(field2) } else { None },
        }
    ))
}
```

## Testing After Refactoring

After each module:
1. `cargo build --lib` - ensure compilation
2. `cargo test` - ensure tests pass
3. `cargo clippy --all-targets --all-features -- -D warnings` - no warnings

## Completion Checklist

- [x] thresholds::add_generic_threshold
- [x] calls/expression::add_expression_call + element functions (4 functions)
- [ ] features::add_feature_comparison
- [ ] features::add_feature_comparison_element
- [ ] features::add_feature_distinct_call_element
- [ ] functions/comparison::add_comparison_function
- [ ] functions/comparison::set_comparison_function
- [ ] functions/standardize::add_standardize_function
- [ ] functions/standardize::set_standardize_function
- [ ] functions/expression::add_expression_function
- [ ] functions/expression::set_expression_function
- [ ] functions/distinct::add_distinct_function
- [ ] functions/distinct::set_distinct_function
- [ ] calls/comparison::{add_comparison_call, add/set/delete_comparison_call_element} (4 functions)
- [ ] calls/distinct::{add_distinct_call, add/set/delete_distinct_call_element} (4 functions)
- [ ] calls/standardize::{add_standardize_call, add/set/delete_standardize_call_element} (4 functions)
- [ ] behavior_overrides::add_behavior_override
- [ ] Final: cargo test
- [ ] Final: cargo clippy

## Estimated Remaining Work

- ~35-40 more functions to refactor
- Each function requires:
  - 1 param struct definition (~15-30 lines)
  - 1 function signature change (~5 lines)
  - 1 function body update (~5-20 lines)
  - 1 command_processor update (~5-10 lines)
  - 1 FFI wrapper update (~5-15 lines)

Total estimated: ~50-100 lines per function × 40 functions = 2000-4000 lines of changes
