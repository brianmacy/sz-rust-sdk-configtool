# Parameter Struct Design Pattern

## Problem

Functions with many optional parameters of the same type are unreadable:

```rust
// Which optional parameter is which? 🤷
features::set_feature(
    config,
    "NAME",
    None,           // candidates?
    Some("Yes"),    // anonymize? derived?
    None,           // ???
    None,           // ???
    Some("No"),     // ???
    Some("NAME"),   // behavior?
    Some("IDENTITY"), // class?
    Some(2),        // version?
    None,           // rtype_id?
)
```

**Current worst offenders:**
- `features::add_feature()` - 15 parameters (10 optional)
- `features::set_feature()` - 11 parameters (10 optional)
- `attributes::add_attribute()` - 8 parameters (4 optional)
- `functions::*::add_*_function()` - 6 parameters (4 optional)

## Solution: Parameter Structs

### Pattern 1: Separate Params Struct (Recommended)

```rust
// In src/features.rs

/// Parameters for setting feature properties
#[derive(Debug, Clone, Default)]
pub struct SetFeatureParams<'a> {
    pub candidates: Option<&'a str>,
    pub anonymize: Option<&'a str>,
    pub derived: Option<&'a str>,
    pub history: Option<&'a str>,
    pub matchkey: Option<&'a str>,
    pub behavior: Option<&'a str>,
    pub class: Option<&'a str>,
    pub version: Option<i64>,
    pub rtype_id: Option<i64>,
}

impl<'a> SetFeatureParams<'a> {
    /// Create params with all fields set to None
    pub fn new() -> Self {
        Self::default()
    }

    /// Builder-style setter for candidates
    pub fn candidates(mut self, value: &'a str) -> Self {
        self.candidates = Some(value);
        self
    }

    /// Builder-style setter for behavior
    pub fn behavior(mut self, value: &'a str) -> Self {
        self.behavior = Some(value);
        self
    }

    // ... more builder methods
}

/// Set (update) a feature's properties
pub fn set_feature(
    config_json: &str,
    feature_code: &str,
    params: SetFeatureParams,
) -> Result<String> {
    // Access params.candidates, params.behavior, etc.
    // ...
}
```

**Usage:**
```rust
// Named fields - clear and explicit!
features::set_feature(config, "NAME", SetFeatureParams {
    candidates: Some("Yes"),
    behavior: Some("NAME"),
    version: Some(2),
    ..Default::default()  // All others = None
})?;

// Or with builder pattern
features::set_feature(config, "NAME",
    SetFeatureParams::new()
        .candidates("Yes")
        .behavior("NAME")
        .version(2)
)?;
```

### Pattern 2: Inline Struct (For Simple Cases)

```rust
/// Parameters for adding an attribute
#[derive(Debug, Clone)]
pub struct AddAttributeParams<'a> {
    pub attribute: &'a str,
    pub feature: &'a str,
    pub element: &'a str,
    pub class: &'a str,
    pub default_value: Option<&'a str>,
    pub internal: Option<&'a str>,
    pub required: Option<&'a str>,
}

pub fn add_attribute(
    config_json: &str,
    params: AddAttributeParams,
) -> Result<(String, Value)> {
    // Access params.attribute, params.feature, etc.
}
```

**Usage:**
```rust
attributes::add_attribute(config, AddAttributeParams {
    attribute: "TEST_ATTR",
    feature: "NAME",
    element: "FULL_NAME",
    class: "OTHER",
    default_value: None,
    internal: Some("No"),
    required: Some("No"),
})?;
```

### Pattern 3: From JSON (For Command Processor)

```rust
impl<'a> TryFrom<&'a Value> for SetFeatureParams<'a> {
    type Error = SzConfigError;

    fn try_from(json: &'a Value) -> Result<Self> {
        Ok(Self {
            candidates: json.get("candidates").and_then(|v| v.as_str()),
            anonymize: json.get("anonymize").and_then(|v| v.as_str()),
            behavior: json.get("behavior").and_then(|v| v.as_str()),
            class: json.get("class").and_then(|v| v.as_str()),
            version: json.get("version").and_then(|v| v.as_i64()),
            // ...
        })
    }
}
```

**Command processor usage:**
```rust
"setFeature" => {
    let feature = get_str_param(params, "feature")?;
    let set_params = SetFeatureParams::try_from(params)?;
    crate::features::set_feature(config, feature, set_params)
}
```

## Migration Strategy

### Phase 1: Add New API (Non-Breaking)

Keep existing functions, add new param-struct versions:

```rust
// OLD API (deprecated but kept for compatibility)
#[deprecated(since = "0.2.0", note = "Use set_feature_v2 with SetFeatureParams")]
pub fn set_feature(/* 11 params */) -> Result<String> {
    // Delegate to new version
    set_feature_v2(config_json, feature_code, SetFeatureParams {
        candidates,
        anonymize,
        // ... map all params
    })
}

// NEW API
pub fn set_feature_v2(
    config_json: &str,
    feature_code: &str,
    params: SetFeatureParams,
) -> Result<String> {
    // Implementation
}
```

### Phase 2: Update Examples and Tests

```rust
// Update examples to use new API
let result = features::set_feature_v2(config, "NAME", SetFeatureParams {
    candidates: Some("Yes"),
    behavior: Some("NAME"),
    ..Default::default()
})?;
```

### Phase 3: Deprecation Notice

After one minor version, consider removing old API in next major version.

## Benefits

✅ **Self-documenting** - Field names at call site
✅ **Type-safe** - Still get compile-time checking
✅ **Extensible** - Easy to add new optional fields
✅ **Default handling** - `..Default::default()` for unset fields
✅ **JSON integration** - `TryFrom<&Value>` for command processor
✅ **IDE support** - Auto-completion shows field names

## Recommended Functions to Refactor

**High Priority (10+ params):**
1. `features::add_feature()` - 15 params → `AddFeatureParams`
2. `features::set_feature()` - 11 params → `SetFeatureParams`

**Medium Priority (6-9 params):**
3. `attributes::add_attribute()` - 8 params → `AddAttributeParams`
4. `calls::expression::add_expression_call()` - 8 params → `AddExpressionCallParams`
5. `thresholds::add_comparison_threshold()` - 11 params → `AddComparisonThresholdParams`
6. `functions::comparison::add_comparison_function()` - 6 params → `AddComparisonFunctionParams`

**Low Priority (3-5 params):**
- Most other functions are fine with positional params

## Example: Complete Refactored API

```rust
// src/features.rs

#[derive(Debug, Clone, Default)]
pub struct SetFeatureParams<'a> {
    pub candidates: Option<&'a str>,
    pub anonymize: Option<&'a str>,
    pub derived: Option<&'a str>,
    pub history: Option<&'a str>,
    pub matchkey: Option<&'a str>,
    pub behavior: Option<&'a str>,
    pub class: Option<&'a str>,
    pub version: Option<i64>,
    pub rtype_id: Option<i64>,
}

impl<'a> SetFeatureParams<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    // Builder-style methods (optional, but nice)
    pub fn with_candidates(mut self, val: &'a str) -> Self {
        self.candidates = Some(val);
        self
    }

    pub fn with_behavior(mut self, val: &'a str) -> Self {
        self.behavior = Some(val);
        self
    }

    // ... etc
}

impl<'a> TryFrom<&'a Value> for SetFeatureParams<'a> {
    type Error = SzConfigError;

    fn try_from(json: &'a Value) -> Result<Self> {
        Ok(Self {
            candidates: json.get("candidates").and_then(|v| v.as_str()),
            anonymize: json.get("anonymize").and_then(|v| v.as_str()),
            derived: json.get("derived").and_then(|v| v.as_str()),
            history: json.get("history").and_then(|v| v.as_str()),
            matchkey: json.get("matchKey").and_then(|v| v.as_str()),
            behavior: json.get("behavior").and_then(|v| v.as_str()),
            class: json.get("class").and_then(|v| v.as_str()),
            version: json.get("version").and_then(|v| v.as_i64()),
            rtype_id: json.get("rtypeId").and_then(|v| v.as_i64()),
        })
    }
}

pub fn set_feature(
    config_json: &str,
    feature_code: &str,
    params: SetFeatureParams,
) -> Result<String> {
    let mut config: Value = serde_json::from_str(config_json)?;

    let ftype_id = if let Ok(id) = feature_code.trim().parse::<i64>() {
        id
    } else {
        lookup_feature_id(&config, feature_code)?
    };

    let ftypes = config["G2_CONFIG"]["CFG_FTYPE"]
        .as_array_mut()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_FTYPE".to_string()))?;

    let ftype = ftypes
        .iter_mut()
        .find(|f| f["FTYPE_ID"].as_i64() == Some(ftype_id))
        .ok_or_else(|| SzConfigError::NotFound(format!("Feature: {}", ftype_id)))?;

    // Now super clear what's being set!
    if let Some(val) = params.candidates {
        ftype["USED_FOR_CAND"] = json!(val);
    }
    if let Some(val) = params.anonymize {
        ftype["ANONYMIZE"] = json!(val);
    }
    if let Some(val) = params.behavior {
        let (freq, excl, stab) = parse_behavior_code(val)?;
        ftype["FTYPE_FREQ"] = json!(freq);
        ftype["FTYPE_EXCL"] = json!(excl);
        ftype["FTYPE_STAB"] = json!(stab);
    }
    // ... etc

    serde_json::to_string(&config)
}
```

## Comparison: Before vs After

### Before (Current)
```rust
// Unreadable - need to count params and check docs
features::set_feature(
    config, "NAME",
    None, Some("Yes"), None, None, Some("No"),
    Some("NAME"), Some("IDENTITY"), Some(2), None
)?;
```

### After (With Param Struct)
```rust
// Self-documenting!
features::set_feature(config, "NAME", SetFeatureParams {
    anonymize: Some("Yes"),
    matchkey: Some("No"),
    behavior: Some("NAME"),
    class: Some("IDENTITY"),
    version: Some(2),
    ..Default::default()
})?;
```

## Implementation Effort

**Per function:**
- Define param struct: 10 minutes
- Add `Default` derive: included
- Add `TryFrom<&Value>`: 10 minutes
- Update function signature: 5 minutes
- Update callers: 10-20 minutes
- **Total: 35-45 minutes per function**

**For 6 high-priority functions: ~4 hours**

---

Would you like me to refactor the high-priority functions to use this pattern?