use crate::error::{Result, SzConfigError};
use serde_json::Value;

/// Extract a string field from a config row as an owned `Option<String>`.
///
/// Returns `None` when the key is absent or its value is not a JSON string
/// (including explicit `null`). Used by row builders to carry an existing
/// field value forward when an update does not supply a new one.
pub(crate) fn field_as_string(item: &Value, key: &str) -> Option<String> {
    item.get(key).and_then(|v| v.as_str()).map(String::from)
}

/// Project a config-row field as an owned JSON value, null-preserving.
///
/// Returns the field's value cloned when the key is present (including an
/// explicit JSON `null`), and `Value::Null` when the key is absent. This is the
/// read-side projection used by list/get builders that must emit `null` for a
/// stored-null or missing field rather than coercing it to `""` or `0`.
pub(crate) fn field_or_null(item: &Value, key: &str) -> Value {
    item.get(key).cloned().unwrap_or(Value::Null)
}

/// Parse a tri-state string [`FieldUpdate`] from a JSON object.
///
/// Scans `keys` in order and acts on the first key that is present on `json`:
/// an explicit JSON `null` maps to [`FieldUpdate::Clear`] and a JSON string maps
/// to [`FieldUpdate::Set`]. A key that is absent everywhere (or present only with
/// a non-string, non-null value) leaves the field untouched
/// ([`FieldUpdate::Leave`]). This encodes the JSON write contract where an
/// omitted key means "leave", an explicit `null` means "clear", and a value
/// means "set".
pub(crate) fn field_update_str<'a>(json: &'a Value, keys: &[&str]) -> FieldUpdate<&'a str> {
    for key in keys {
        if let Some(v) = json.get(*key) {
            if v.is_null() {
                return FieldUpdate::Clear;
            }
            if let Some(s) = v.as_str() {
                return FieldUpdate::Set(s);
            }
        }
    }
    FieldUpdate::Leave
}

/// Parse a tri-state integer [`FieldUpdate`] from a JSON object.
///
/// The integer analogue of [`field_update_str`]: an explicit JSON `null` maps to
/// [`FieldUpdate::Clear`], a JSON integer maps to [`FieldUpdate::Set`], and an
/// absent key (or present non-integer, non-null value) leaves the field
/// untouched ([`FieldUpdate::Leave`]).
pub(crate) fn field_update_i64(json: &Value, keys: &[&str]) -> FieldUpdate<i64> {
    for key in keys {
        if let Some(v) = json.get(*key) {
            if v.is_null() {
                return FieldUpdate::Clear;
            }
            if let Some(n) = v.as_i64() {
                return FieldUpdate::Set(n);
            }
        }
    }
    FieldUpdate::Leave
}

/// Tri-state update for an optional field: leave, clear, or set.
///
/// This distinguishes the three intents a partial update can express, which a
/// plain `Option<T>` cannot:
///
/// - [`FieldUpdate::Leave`] — do not touch the field; keep whatever exists.
/// - [`FieldUpdate::Clear`] — remove the field's value (store `null`).
/// - [`FieldUpdate::Set`] — replace the field's value.
///
/// The default is [`FieldUpdate::Leave`], so a field omitted from an update
/// builder carries its existing value forward untouched.
///
/// # Example
///
/// ```
/// use sz_configtool_lib::helpers::FieldUpdate;
///
/// // Leave keeps the existing value.
/// assert_eq!(FieldUpdate::Leave.apply(Some("old")), Some("old"));
/// // Clear drops it.
/// assert_eq!(FieldUpdate::<&str>::Clear.apply(Some("old")), None);
/// // Set overrides it.
/// assert_eq!(FieldUpdate::Set("new").apply(Some("old")), Some("new"));
/// // The default is Leave.
/// assert_eq!(FieldUpdate::<&str>::default(), FieldUpdate::Leave);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FieldUpdate<T> {
    /// Do not modify the field; keep the existing value.
    #[default]
    Leave,
    /// Clear the field (resolve to `None` / stored `null`).
    Clear,
    /// Set the field to the given value.
    Set(T),
}

impl<T> FieldUpdate<T> {
    /// Resolve this update against the current value of the field.
    ///
    /// - [`FieldUpdate::Leave`] returns `existing` unchanged.
    /// - [`FieldUpdate::Clear`] returns `None`.
    /// - [`FieldUpdate::Set`] returns `Some(value)`.
    ///
    /// # Example
    ///
    /// ```
    /// use sz_configtool_lib::helpers::FieldUpdate;
    ///
    /// let update: FieldUpdate<i64> = FieldUpdate::Set(5);
    /// assert_eq!(update.apply(Some(1)), Some(5));
    /// ```
    pub fn apply(self, existing: Option<T>) -> Option<T> {
        match self {
            FieldUpdate::Leave => existing,
            FieldUpdate::Clear => None,
            FieldUpdate::Set(value) => Some(value),
        }
    }
}

/// Get the next available ID for a config array
///
/// Finds the maximum value of the specified ID field and returns max + 1
///
/// # Arguments
/// * `array` - Array of configuration items
/// * `id_field` - Name of the ID field (e.g., "DSRC_ID", "ATTR_ID")
///
/// # Returns
/// Next available ID value
pub fn get_next_id_from_array(array: &[Value], id_field: &str) -> Result<i64> {
    let max_id = array
        .iter()
        .filter_map(|item| item.get(id_field))
        .filter_map(|v| v.as_i64())
        .max()
        .unwrap_or(0);

    Ok(max_id + 1)
}

/// Get the next available ID for a config section with optional seed value
///
/// Navigates to a config section using a path and finds the next available ID.
/// Useful for user-created items that should start at a specific ID (e.g., 1000).
///
/// # Arguments
/// * `config_data` - Parsed configuration JSON Value
/// * `section_path` - Dot-separated path (e.g., "G2_CONFIG.CFG_SFCALL")
/// * `id_field` - Name of the ID field (e.g., "SFCALL_ID")
/// * `seed_value` - Minimum value to return (e.g., 1000 for user items)
///
/// # Returns
/// Next available ID value, at least seed_value
///
/// # Errors
/// Returns error if section path not found
pub fn get_next_id(
    config_data: &Value,
    section_path: &str,
    id_field: &str,
    seed_value: i64,
) -> Result<i64> {
    // Parse section path (e.g., "G2_CONFIG.CFG_SFCALL")
    let parts: Vec<&str> = section_path.split('.').collect();

    let mut current = config_data;
    for part in &parts {
        current = current.get(part).ok_or_else(|| {
            SzConfigError::MissingSection(format!("Section path '{section_path}' not found"))
        })?;
    }

    // Get max ID from array
    let max_id = if let Some(items) = current.as_array() {
        items
            .iter()
            .filter_map(|item| item.get(id_field).and_then(|v| v.as_i64()))
            .max()
            .unwrap_or(seed_value - 1)
    } else {
        seed_value - 1
    };

    Ok(std::cmp::max(max_id + 1, seed_value))
}

/// Get the next available ID for a config array with minimum value
///
/// Finds the maximum value of the specified ID field and returns max(max_id + 1, min_value)
/// This is useful for user-created items that should start at a high ID (e.g., 1000)
///
/// # Arguments
/// * `array` - Array of configuration items
/// * `id_field` - Name of the ID field (e.g., "FTYPE_ID", "FELEM_ID")
/// * `min_value` - Minimum value to return (e.g., 1000 for user items)
///
/// # Returns
/// Next available ID value, at least min_value
pub fn get_next_id_with_min(array: &[Value], id_field: &str, min_value: i64) -> Result<i64> {
    let max_id = array
        .iter()
        .filter_map(|item| item.get(id_field))
        .filter_map(|v| v.as_i64())
        .max()
        .unwrap_or(min_value - 1);

    Ok(std::cmp::max(max_id + 1, min_value))
}

/// Check if an ID is already taken in a config array
///
/// # Arguments
/// * `array` - Array of configuration items
/// * `id_field` - Name of the ID field (e.g., "DSRC_ID", "ATTR_ID")
/// * `id_value` - ID value to check
///
/// # Returns
/// true if ID is taken, false otherwise
pub fn is_id_taken(array: &[Value], id_field: &str, id_value: i64) -> bool {
    array
        .iter()
        .any(|item| item.get(id_field).and_then(|v| v.as_i64()) == Some(id_value))
}

/// Get the next available ID or use desired ID if specified and available
///
/// Matches Python's getDesiredValueOrNext behavior:
/// - If desired_id is Some and available, returns it
/// - If desired_id is Some but taken, returns error
/// - If desired_id is None, returns next available ID
///
/// # Arguments
/// * `array` - Array of configuration items
/// * `id_field` - Name of the ID field (e.g., "DSRC_ID", "ATTR_ID")
/// * `desired_id` - Optional user-specified ID
/// * `min_value` - Minimum value to return (e.g., 1000 for user items)
///
/// # Returns
/// ID to use (either desired_id or next available)
///
/// # Errors
/// Returns error if desired_id is already taken
pub fn get_desired_or_next_id(
    array: &[Value],
    id_field: &str,
    desired_id: Option<i64>,
    min_value: i64,
) -> Result<i64> {
    if let Some(id) = desired_id
        && id > 0
    {
        if is_id_taken(array, id_field, id) {
            return Err(SzConfigError::AlreadyExists(format!(
                "The specified ID {id} is already taken"
            )));
        }
        return Ok(id);
    }

    // No desired ID or invalid, get next available
    get_next_id_with_min(array, id_field, min_value)
}

/// Resolve an execution-order value within a scope, honouring a desired value or
/// allocating the next available one.
///
/// This is the order-field analogue of [`get_desired_or_next_id`] and mirrors
/// Python `getDesiredValueOrNext` with its default `seed_order` of `0`. A row is
/// *in scope* when every `(field, value)` pair in `scope` matches that row's
/// field (compared as `i64`); `scope` is the list of *senior* key fields that
/// partition the order space (e.g. `[("CFCALL_ID", 42)]` numbers `EXEC_ORDER`
/// independently per call, and an empty scope numbers `order_field` across the
/// whole table).
///
/// Over the in-scope rows:
/// - `desired == Some(d)` with `d > 0` and `d` free -> returns `d`.
/// - `desired == Some(d)` with `d > 0` but `d` already taken -> `AlreadyExists`
///   (the reject-if-taken policy; Python instead silently reallocates, but this
///   SDK surfaces the collision).
/// - otherwise (`None`, or a non-positive `d`) -> `(max in scope, seed 0) + 1`.
///
/// Never returns `null`/absent: an order is always resolved to a concrete value.
///
/// # Arguments
/// * `array` - The section's rows to scan
/// * `order_field` - The order column being allocated (e.g. `"EXEC_ORDER"`)
/// * `scope` - Senior `(field, value)` keys that partition the order space
/// * `desired` - Caller-requested order, or `None` to auto-allocate
///
/// # Errors
/// Returns `AlreadyExists` when a positive `desired` value is already taken
/// within the scope.
pub(crate) fn get_desired_or_next_order(
    array: &[Value],
    order_field: &str,
    scope: &[(&str, i64)],
    desired: Option<i64>,
) -> Result<i64> {
    // seed_order default is 0 (Python getDesiredValueOrNext default).
    let mut last: i64 = 0;
    let mut taken = false;

    for row in array {
        let in_scope = scope
            .iter()
            .all(|(field, value)| row.get(*field).and_then(|v| v.as_i64()) == Some(*value));
        if !in_scope {
            continue;
        }
        if let Some(order) = row.get(order_field).and_then(|v| v.as_i64()) {
            if let Some(d) = desired
                && d > 0
                && order == d
            {
                taken = true;
            }
            if order > last {
                last = order;
            }
        }
    }

    if let Some(d) = desired
        && d > 0
    {
        if taken {
            return Err(SzConfigError::AlreadyExists(format!(
                "The specified {order_field} {d} is already taken"
            )));
        }
        return Ok(d);
    }

    Ok(last + 1)
}

/// Get the next available ID or use desired ID (for config sections)
///
/// Same as get_desired_or_next_id but works with section paths
///
/// # Arguments
/// * `config_data` - Parsed configuration JSON Value
/// * `section_path` - Dot-separated path (e.g., "G2_CONFIG.CFG_SFCALL")
/// * `id_field` - Name of the ID field (e.g., "SFCALL_ID")
/// * `desired_id` - Optional user-specified ID
/// * `seed_value` - Minimum value to return (e.g., 1000 for user items)
///
/// # Returns
/// ID to use (either desired_id or next available)
///
/// # Errors
/// Returns error if section not found or desired_id is already taken
pub fn get_desired_or_next_id_from_section(
    config_data: &Value,
    section_path: &str,
    id_field: &str,
    desired_id: Option<i64>,
    seed_value: i64,
) -> Result<i64> {
    // Parse section path
    let parts: Vec<&str> = section_path.split('.').collect();

    let mut current = config_data;
    for part in &parts {
        current = current.get(part).ok_or_else(|| {
            SzConfigError::MissingSection(format!("Section path '{section_path}' not found"))
        })?;
    }

    let array = current.as_array().ok_or_else(|| {
        SzConfigError::MissingSection(format!("Section '{section_path}' is not an array"))
    })?;

    get_desired_or_next_id(array, id_field, desired_id, seed_value)
}

/// Find item in config array by field value
///
/// # Arguments
/// * `array` - Array of configuration items
/// * `field` - Field name to search
/// * `value` - Value to match
///
/// # Returns
/// Reference to matching item, or None if not found
pub fn find_in_array<'a>(array: &'a [Value], field: &str, value: &str) -> Option<&'a Value> {
    array.iter().find(|item| {
        item.get(field)
            .and_then(|v| v.as_str())
            .map(|s| s == value)
            .unwrap_or(false)
    })
}

/// Get mutable reference to item in config array
///
/// # Arguments
/// * `array` - Mutable array of configuration items
/// * `field` - Field name to search
/// * `value` - Value to match
///
/// # Returns
/// Mutable reference to matching item, or None if not found
pub fn find_in_array_mut<'a>(
    array: &'a mut [Value],
    field: &str,
    value: &str,
) -> Option<&'a mut Value> {
    array.iter_mut().find(|item| {
        item.get(field)
            .and_then(|v| v.as_str())
            .map(|s| s == value)
            .unwrap_or(false)
    })
}

/// Add item to config array (generic)
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `section` - Section name (e.g., "CFG_DSRC", "CFG_ATTR")
/// * `item` - JSON Value to add
///
/// # Returns
/// Modified configuration JSON string
///
/// # Errors
/// - `JsonParse` if config_json is invalid
/// - `MissingSection` if section doesn't exist
pub fn add_to_config_array(config_json: &str, section: &str, item: Value) -> Result<String> {
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let array = config
        .get_mut("G2_CONFIG")
        .and_then(|g| g.get_mut(section))
        .and_then(|v| v.as_array_mut())
        .ok_or_else(|| SzConfigError::MissingSection(section.to_string()))?;

    array.push(item);

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// Delete item from config array by field value
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `section` - Section name (e.g., "CFG_DSRC", "CFG_ATTR")
/// * `field` - Field name to match (e.g., "DSRC_CODE", "ATTR_CODE")
/// * `value` - Value to match for deletion
///
/// # Returns
/// Modified configuration JSON string
///
/// # Errors
/// - `JsonParse` if config_json is invalid
/// - `MissingSection` if section doesn't exist
/// - `NotFound` if no item matches the criteria
pub fn delete_from_config_array(
    config_json: &str,
    section: &str,
    field: &str,
    value: &str,
) -> Result<String> {
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let array = config
        .get_mut("G2_CONFIG")
        .and_then(|g| g.get_mut(section))
        .and_then(|v| v.as_array_mut())
        .ok_or_else(|| SzConfigError::MissingSection(section.to_string()))?;

    let original_len = array.len();
    array.retain(|item| {
        item.get(field)
            .and_then(|v| v.as_str())
            .map(|s| s != value)
            .unwrap_or(true)
    });

    if array.len() == original_len {
        return Err(SzConfigError::NotFound(format!(
            "{section} '{value}' not found"
        )));
    }

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// Find item in config array by field value (returns owned value)
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `section` - Section name (e.g., "CFG_DSRC", "CFG_ATTR")
/// * `field` - Field name to match (e.g., "DSRC_CODE", "ATTR_CODE")
/// * `value` - Value to match
///
/// # Returns
/// Cloned item if found, None otherwise
///
/// # Errors
/// - `JsonParse` if config_json is invalid
pub fn find_in_config_array(
    config_json: &str,
    section: &str,
    field: &str,
    value: &str,
) -> Result<Option<Value>> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let array = config
        .get("G2_CONFIG")
        .and_then(|g| g.get(section))
        .and_then(|v| v.as_array());

    if let Some(arr) = array {
        let item = arr.iter().find(|item| {
            item.get(field)
                .and_then(|v| v.as_str())
                .map(|s| s == value)
                .or_else(|| {
                    // Also try numeric comparison
                    item.get(field)
                        .and_then(|v| v.as_i64())
                        .and_then(|id| value.parse::<i64>().ok().map(|val| id == val))
                })
                .unwrap_or(false)
        });
        Ok(item.cloned())
    } else {
        Ok(None)
    }
}

/// Alias for delete_from_config_array for compatibility
pub fn remove_from_config_array(
    config_json: &str,
    section: &str,
    field: &str,
    value: &str,
) -> Result<String> {
    delete_from_config_array(config_json, section, field, value)
}

/// Update item in config array (complete replacement)
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `section` - Section name (e.g., "CFG_DSRC", "CFG_ATTR")
/// * `field` - Field name to match (e.g., "DSRC_CODE", "ATTR_CODE")
/// * `value` - Value to match
/// * `new_item` - Complete new item value (replaces old item entirely)
///
/// # Returns
/// Modified configuration JSON string
pub fn update_in_config_array(
    config_json: &str,
    section: &str,
    field: &str,
    value: &str,
    new_item: Value,
) -> Result<String> {
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let array = config
        .get_mut("G2_CONFIG")
        .and_then(|g| g.get_mut(section))
        .and_then(|v| v.as_array_mut())
        .ok_or_else(|| SzConfigError::MissingSection(section.to_string()))?;

    let item = find_in_array_mut(array, field, value)
        .ok_or_else(|| SzConfigError::NotFound(format!("{section} '{value}' not found")))?;

    // Replace the entire item
    *item = new_item;

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// List all items from a config array
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `section` - Section name (e.g., "CFG_DSRC", "CFG_ATTR")
///
/// # Returns
/// Vector of all items in the section
///
/// # Errors
/// - `JsonParse` if config_json is invalid
pub fn list_from_config_array(config_json: &str, section: &str) -> Result<Vec<Value>> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let items = if let Some(g2_config) = config.get("G2_CONFIG") {
        if let Some(array) = g2_config.get(section).and_then(|v| v.as_array()) {
            array.clone()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    Ok(items)
}

/// Lookup feature ID by feature code
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `feature_code` - Feature code to look up (case-insensitive)
///
/// # Returns
/// Feature ID (FTYPE_ID)
///
/// # Errors
/// Returns error if feature not found or JSON is invalid
pub fn lookup_feature_id(config_json: &str, feature_code: &str) -> Result<i64> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    config
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_FTYPE"))
        .and_then(|v| v.as_array())
        .and_then(|arr| {
            arr.iter()
                .find(|f| {
                    f.get("FTYPE_CODE")
                        .and_then(|v| v.as_str())
                        .map(|s| s.eq_ignore_ascii_case(feature_code))
                        .unwrap_or(false)
                })
                .and_then(|f| f.get("FTYPE_ID"))
                .and_then(|v| v.as_i64())
        })
        .ok_or_else(|| SzConfigError::NotFound(format!("Feature '{feature_code}' not found")))
}

/// Lookup element ID by element code
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `element_code` - Element code to look up (case-insensitive)
///
/// # Returns
/// Element ID (FELEM_ID)
///
/// # Errors
/// Returns error if element not found or JSON is invalid
pub fn lookup_element_id(config_json: &str, element_code: &str) -> Result<i64> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    config
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_FELEM"))
        .and_then(|v| v.as_array())
        .and_then(|arr| {
            arr.iter()
                .find(|e| {
                    e.get("FELEM_CODE")
                        .and_then(|v| v.as_str())
                        .map(|s| s.eq_ignore_ascii_case(element_code))
                        .unwrap_or(false)
                })
                .and_then(|e| e.get("FELEM_ID"))
                .and_then(|v| v.as_i64())
        })
        .ok_or_else(|| SzConfigError::NotFound(format!("Element '{element_code}' not found")))
}

/// Lookup standardize function ID by function code
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `func_code` - Function code to look up (case-insensitive)
///
/// # Returns
/// Function ID (SFUNC_ID)
///
/// # Errors
/// Returns error if function not found or JSON is invalid
pub fn lookup_sfunc_id(config_json: &str, func_code: &str) -> Result<i64> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    config
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_SFUNC"))
        .and_then(|v| v.as_array())
        .and_then(|arr| {
            arr.iter()
                .find(|f| {
                    f.get("SFUNC_CODE")
                        .and_then(|v| v.as_str())
                        .map(|s| s.eq_ignore_ascii_case(func_code))
                        .unwrap_or(false)
                })
                .and_then(|f| f.get("SFUNC_ID"))
                .and_then(|v| v.as_i64())
        })
        .ok_or_else(|| {
            SzConfigError::NotFound(format!("Standardize function '{func_code}' not found"))
        })
}

/// Lookup expression function ID by function code
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `func_code` - Function code to look up (case-insensitive)
///
/// # Returns
/// Function ID (EFUNC_ID)
///
/// # Errors
/// Returns error if function not found or JSON is invalid
pub fn lookup_efunc_id(config_json: &str, func_code: &str) -> Result<i64> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    config
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_EFUNC"))
        .and_then(|v| v.as_array())
        .and_then(|arr| {
            arr.iter()
                .find(|f| {
                    f.get("EFUNC_CODE")
                        .and_then(|v| v.as_str())
                        .map(|s| s.eq_ignore_ascii_case(func_code))
                        .unwrap_or(false)
                })
                .and_then(|f| f.get("EFUNC_ID"))
                .and_then(|v| v.as_i64())
        })
        .ok_or_else(|| {
            SzConfigError::NotFound(format!("Expression function '{func_code}' not found"))
        })
}

/// Lookup comparison function ID by function code
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `func_code` - Function code to look up (case-insensitive)
///
/// # Returns
/// Function ID (CFUNC_ID)
///
/// # Errors
/// Returns error if function not found or JSON is invalid
pub fn lookup_cfunc_id(config_json: &str, func_code: &str) -> Result<i64> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    config
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_CFUNC"))
        .and_then(|v| v.as_array())
        .and_then(|arr| {
            arr.iter()
                .find(|f| {
                    f.get("CFUNC_CODE")
                        .and_then(|v| v.as_str())
                        .map(|s| s.eq_ignore_ascii_case(func_code))
                        .unwrap_or(false)
                })
                .and_then(|f| f.get("CFUNC_ID"))
                .and_then(|v| v.as_i64())
        })
        .ok_or_else(|| {
            SzConfigError::NotFound(format!("Comparison function '{func_code}' not found"))
        })
}

/// Lookup distinct function ID by function code
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `func_code` - Function code to look up (case-insensitive)
///
/// # Returns
/// Function ID (DFUNC_ID)
///
/// # Errors
/// Returns error if function not found or JSON is invalid
pub fn lookup_dfunc_id(config_json: &str, func_code: &str) -> Result<i64> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    config
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_DFUNC"))
        .and_then(|v| v.as_array())
        .and_then(|arr| {
            arr.iter()
                .find(|f| {
                    f.get("DFUNC_CODE")
                        .and_then(|v| v.as_str())
                        .map(|s| s.eq_ignore_ascii_case(func_code))
                        .unwrap_or(false)
                })
                .and_then(|f| f.get("DFUNC_ID"))
                .and_then(|v| v.as_i64())
        })
        .ok_or_else(|| {
            SzConfigError::NotFound(format!("Distinct function '{func_code}' not found"))
        })
}

/// Lookup generic plan ID by plan code
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `plan_code` - Plan code to look up (case-insensitive, e.g., "INGEST", "SEARCH")
///
/// # Returns
/// Plan ID (GPLAN_ID)
///
/// # Errors
/// Returns error if plan not found or JSON is invalid
pub fn lookup_gplan_id(config_json: &str, plan_code: &str) -> Result<i64> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    config
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_GPLAN"))
        .and_then(|v| v.as_array())
        .and_then(|arr| {
            arr.iter()
                .find(|p| {
                    p.get("GPLAN_CODE")
                        .and_then(|v| v.as_str())
                        .map(|s| s.eq_ignore_ascii_case(plan_code))
                        .unwrap_or(false)
                })
                .and_then(|p| p.get("GPLAN_ID"))
                .and_then(|v| v.as_i64())
        })
        .ok_or_else(|| SzConfigError::NotFound(format!("Generic plan '{plan_code}' not found")))
}

/// Internal: Lookup generic plan code by plan ID (for FFI use)
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `gplan_id` - Plan ID to look up
///
/// # Returns
/// Plan code (GPLAN_CODE)
///
/// # Errors
/// Returns error if plan not found or JSON is invalid
pub(crate) fn lookup_gplan_code(config_json: &str, gplan_id: i64) -> Result<String> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    config
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_GPLAN"))
        .and_then(|v| v.as_array())
        .and_then(|arr| {
            arr.iter()
                .find(|p| p.get("GPLAN_ID").and_then(|v| v.as_i64()) == Some(gplan_id))
                .and_then(|p| p.get("GPLAN_CODE"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        })
        .ok_or_else(|| SzConfigError::NotFound(format!("Generic plan ID: {gplan_id}")))
}

// ============================================================================
// By-feature call-id resolvers
// ============================================================================
//
// A call row (CFG_?CALL) binds a function to a feature via FTYPE_ID. These
// resolvers find the call *id* for a given feature by scanning the relevant
// call section for rows whose FTYPE_ID matches.
//
// Multiplicity policy (see decision D22): all four resolvers require an
// unambiguous result.
//   * 0 matching rows  -> NotFound
//   * exactly 1 row    -> that row's call id
//   * 2+ matching rows -> InvalidInput (ambiguous; the caller must address the
//                         call by id instead)
// Comparison (CFCALL) and distinct (DFCALL) calls are expected to be 0-or-1
// per feature, so ambiguity there indicates malformed config. Standardize
// (SFCALL) and expression (EFCALL) calls *can* legitimately be many-per-feature,
// so the ambiguity error is the safety net that prevents silently picking the
// wrong call.

/// Resolve the single call id for a feature within one call section.
///
/// Scans `config["G2_CONFIG"][section]` for rows whose `FTYPE_ID` equals
/// `ftype_id` and returns the value of `id_field` when exactly one such row
/// exists. `label` names the call family for error messages.
fn resolve_call_id_for_feature(
    config: &Value,
    section: &str,
    id_field: &str,
    ftype_id: i64,
    label: &str,
) -> Result<i64> {
    let empty: Vec<Value> = Vec::new();
    let rows = config
        .get("G2_CONFIG")
        .and_then(|g| g.get(section))
        .and_then(|v| v.as_array())
        .unwrap_or(&empty);

    let matches: Vec<i64> = rows
        .iter()
        .filter(|row| row.get("FTYPE_ID").and_then(|v| v.as_i64()) == Some(ftype_id))
        .filter_map(|row| row.get(id_field).and_then(|v| v.as_i64()))
        .collect();

    match matches.as_slice() {
        [] => Err(SzConfigError::NotFound(format!(
            "No {label} call found for feature id {ftype_id}"
        ))),
        [id] => Ok(*id),
        many => Err(SzConfigError::InvalidInput(format!(
            "Ambiguous {label} call for feature id {ftype_id}: {} calls match; address the call by id instead",
            many.len()
        ))),
    }
}

/// Resolve the comparison call id (`CFCALL_ID`) bound to a feature.
///
/// See the module policy: returns `NotFound` when the feature has no comparison
/// call and `InvalidInput` when more than one matches.
///
/// # Example
///
/// ```
/// use serde_json::json;
/// use sz_configtool_lib::helpers::resolve_cfcall_id_for_feature;
///
/// let config = json!({"G2_CONFIG": {"CFG_CFCALL": [
///     {"CFCALL_ID": 7, "FTYPE_ID": 3, "CFUNC_ID": 1}
/// ]}});
/// assert_eq!(resolve_cfcall_id_for_feature(&config, 3).unwrap(), 7);
/// assert!(resolve_cfcall_id_for_feature(&config, 99).is_err());
/// ```
pub fn resolve_cfcall_id_for_feature(config: &Value, ftype_id: i64) -> Result<i64> {
    resolve_call_id_for_feature(config, "CFG_CFCALL", "CFCALL_ID", ftype_id, "comparison")
}

/// Resolve the distinct call id (`DFCALL_ID`) bound to a feature.
///
/// See the module policy: returns `NotFound` when the feature has no distinct
/// call and `InvalidInput` when more than one matches.
///
/// # Example
///
/// ```
/// use serde_json::json;
/// use sz_configtool_lib::helpers::resolve_dfcall_id_for_feature;
///
/// let config = json!({"G2_CONFIG": {"CFG_DFCALL": [
///     {"DFCALL_ID": 11, "FTYPE_ID": 3, "DFUNC_ID": 1}
/// ]}});
/// assert_eq!(resolve_dfcall_id_for_feature(&config, 3).unwrap(), 11);
/// ```
pub fn resolve_dfcall_id_for_feature(config: &Value, ftype_id: i64) -> Result<i64> {
    resolve_call_id_for_feature(config, "CFG_DFCALL", "DFCALL_ID", ftype_id, "distinct")
}

/// Resolve the standardize call id (`SFCALL_ID`) bound to a feature.
///
/// Standardize calls can legitimately be many-per-feature; this returns
/// `InvalidInput` when more than one matches (address by id instead) and
/// `NotFound` when none do.
///
/// # Example
///
/// ```
/// use serde_json::json;
/// use sz_configtool_lib::helpers::resolve_sfcall_id_for_feature;
///
/// let config = json!({"G2_CONFIG": {"CFG_SFCALL": [
///     {"SFCALL_ID": 5, "FTYPE_ID": 3, "SFUNC_ID": 1}
/// ]}});
/// assert_eq!(resolve_sfcall_id_for_feature(&config, 3).unwrap(), 5);
/// ```
pub fn resolve_sfcall_id_for_feature(config: &Value, ftype_id: i64) -> Result<i64> {
    resolve_call_id_for_feature(config, "CFG_SFCALL", "SFCALL_ID", ftype_id, "standardize")
}

/// Resolve the expression call id (`EFCALL_ID`) bound to a feature.
///
/// Expression calls can legitimately be many-per-feature; this returns
/// `InvalidInput` when more than one matches (address by id instead) and
/// `NotFound` when none do.
///
/// # Example
///
/// ```
/// use serde_json::json;
/// use sz_configtool_lib::helpers::resolve_efcall_id_for_feature;
///
/// let config = json!({"G2_CONFIG": {"CFG_EFCALL": [
///     {"EFCALL_ID": 9, "FTYPE_ID": 3, "EFUNC_ID": 1}
/// ]}});
/// assert_eq!(resolve_efcall_id_for_feature(&config, 3).unwrap(), 9);
/// ```
pub fn resolve_efcall_id_for_feature(config: &Value, ftype_id: i64) -> Result<i64> {
    resolve_call_id_for_feature(config, "CFG_EFCALL", "EFCALL_ID", ftype_id, "expression")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_field_or_null_present_empty_null_absent() {
        let item = json!({"a": "value", "b": "", "c": null});
        // Present, non-empty string -> cloned value.
        assert_eq!(field_or_null(&item, "a"), json!("value"));
        // Present, empty string -> preserved as "".
        assert_eq!(field_or_null(&item, "b"), json!(""));
        // Present, explicit null -> null.
        assert_eq!(field_or_null(&item, "c"), Value::Null);
        // Absent key -> null.
        assert_eq!(field_or_null(&item, "missing"), Value::Null);
    }

    #[test]
    fn test_field_or_null_non_string_values() {
        let item = json!({"n": 42, "arr": [1, 2]});
        assert_eq!(field_or_null(&item, "n"), json!(42));
        assert_eq!(field_or_null(&item, "arr"), json!([1, 2]));
    }

    #[test]
    fn test_field_update_apply() {
        assert_eq!(FieldUpdate::Leave.apply(Some(1)), Some(1));
        assert_eq!(FieldUpdate::Leave.apply(None::<i64>), None);
        assert_eq!(FieldUpdate::<i64>::Clear.apply(Some(1)), None);
        assert_eq!(FieldUpdate::<i64>::Clear.apply(None), None);
        assert_eq!(FieldUpdate::Set(9).apply(Some(1)), Some(9));
        assert_eq!(FieldUpdate::Set(9).apply(None), Some(9));
    }

    #[test]
    fn test_field_update_default_is_leave() {
        let d: FieldUpdate<String> = FieldUpdate::default();
        assert_eq!(d, FieldUpdate::Leave);
    }

    fn resolver_config() -> Value {
        json!({"G2_CONFIG": {
            "CFG_CFCALL": [
                {"CFCALL_ID": 100, "FTYPE_ID": 1, "CFUNC_ID": 1},
                {"CFCALL_ID": 101, "FTYPE_ID": 2, "CFUNC_ID": 1}
            ],
            "CFG_DFCALL": [
                {"DFCALL_ID": 200, "FTYPE_ID": 1, "DFUNC_ID": 1}
            ],
            "CFG_SFCALL": [
                {"SFCALL_ID": 300, "FTYPE_ID": 1, "SFUNC_ID": 1},
                {"SFCALL_ID": 301, "FTYPE_ID": 1, "SFUNC_ID": 2}
            ],
            "CFG_EFCALL": [
                {"EFCALL_ID": 400, "FTYPE_ID": 5, "EFUNC_ID": 1}
            ]
        }})
    }

    #[test]
    fn test_resolvers_single_match() {
        let cfg = resolver_config();
        assert_eq!(resolve_cfcall_id_for_feature(&cfg, 1).unwrap(), 100);
        assert_eq!(resolve_cfcall_id_for_feature(&cfg, 2).unwrap(), 101);
        assert_eq!(resolve_dfcall_id_for_feature(&cfg, 1).unwrap(), 200);
        assert_eq!(resolve_efcall_id_for_feature(&cfg, 5).unwrap(), 400);
    }

    #[test]
    fn test_resolvers_zero_match_not_found() {
        let cfg = resolver_config();
        let err = resolve_cfcall_id_for_feature(&cfg, 999).unwrap_err();
        assert_eq!(err.kind(), SzConfigError::not_found("").kind());
        assert!(resolve_dfcall_id_for_feature(&cfg, 999).is_err());
        assert!(resolve_sfcall_id_for_feature(&cfg, 999).is_err());
        assert!(resolve_efcall_id_for_feature(&cfg, 999).is_err());
    }

    #[test]
    fn test_resolvers_ambiguous_many_match() {
        let cfg = resolver_config();
        // Feature 1 has two standardize calls -> ambiguous.
        let err = resolve_sfcall_id_for_feature(&cfg, 1).unwrap_err();
        assert_eq!(err.kind(), SzConfigError::validation("").kind());
        assert!(err.to_string().contains("Ambiguous"));
    }

    #[test]
    fn test_resolvers_missing_section_is_not_found() {
        // Absent section behaves like zero matches.
        let cfg = json!({"G2_CONFIG": {}});
        assert!(resolve_cfcall_id_for_feature(&cfg, 1).is_err());
    }

    #[test]
    fn test_get_desired_or_next_order_empty_and_scoped() {
        let rows = vec![
            json!({"CALL_ID": 1, "EXEC_ORDER": 1}),
            json!({"CALL_ID": 1, "EXEC_ORDER": 3}),
            json!({"CALL_ID": 2, "EXEC_ORDER": 9}),
        ];

        // Empty scope -> whole-table max (9) + 1.
        assert_eq!(
            get_desired_or_next_order(&rows, "EXEC_ORDER", &[], None).unwrap(),
            10
        );

        // Scoped to CALL_ID 1 -> max (3) + 1.
        assert_eq!(
            get_desired_or_next_order(&rows, "EXEC_ORDER", &[("CALL_ID", 1)], None).unwrap(),
            4
        );

        // Scoped to a fresh CALL_ID with no rows -> seed 0 + 1.
        assert_eq!(
            get_desired_or_next_order(&rows, "EXEC_ORDER", &[("CALL_ID", 99)], None).unwrap(),
            1
        );
    }

    #[test]
    fn test_get_desired_or_next_order_honour_and_reject() {
        let rows = vec![
            json!({"CALL_ID": 1, "EXEC_ORDER": 1}),
            json!({"CALL_ID": 1, "EXEC_ORDER": 2}),
        ];

        // Desired free within scope -> honoured.
        assert_eq!(
            get_desired_or_next_order(&rows, "EXEC_ORDER", &[("CALL_ID", 1)], Some(5)).unwrap(),
            5
        );

        // Desired taken within scope -> AlreadyExists.
        let err =
            get_desired_or_next_order(&rows, "EXEC_ORDER", &[("CALL_ID", 1)], Some(2)).unwrap_err();
        assert_eq!(err.kind(), SzConfigError::already_exists("").kind());

        // The same value is free under a different scope key -> honoured there.
        assert_eq!(
            get_desired_or_next_order(&rows, "EXEC_ORDER", &[("CALL_ID", 7)], Some(2)).unwrap(),
            2
        );

        // Non-positive desired falls through to auto-allocation.
        assert_eq!(
            get_desired_or_next_order(&rows, "EXEC_ORDER", &[("CALL_ID", 1)], Some(0)).unwrap(),
            3
        );
    }
}
