//! Generic Plan (CFG_GPLAN) operations
//!
//! Functions for managing generic threshold plans in the configuration.
//! Generic plans contain thresholds for entity resolution scoring.

use crate::error::{Result, SzConfigError};
use crate::helpers;
use serde_json::{Value, json};

/// Clone a generic plan with all its thresholds
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
/// * `source_gplan_code` - Source plan code to clone from
/// * `new_gplan_code` - New plan code to create
/// * `new_gplan_desc` - Optional description for new plan (uses code if None)
///
/// # Returns
///
/// Returns `(modified_config, new_plan_id)` tuple on success
///
/// # Example
///
/// ```
/// use sz_configtool_lib::generic_plans;
///
/// let config = r#"{"G2_CONFIG": {"CFG_GPLAN": [{"GPLAN_ID": 1, "GPLAN_CODE": "INGEST"}], "CFG_GENERIC_THRESHOLD": [{"GPLAN_ID": 1, "BEHAVIOR": "NAME"}]}}"#;
/// let (modified, plan_id) = generic_plans::clone_generic_plan(config, "INGEST", "CUSTOM", None).unwrap();
/// ```
pub fn clone_generic_plan(
    config_json: &str,
    source_gplan_code: &str,
    new_gplan_code: &str,
    new_gplan_desc: Option<&str>,
) -> Result<(String, i64)> {
    let source_code = source_gplan_code.to_uppercase();
    let new_code = new_gplan_code.to_uppercase();
    let new_desc = new_gplan_desc.unwrap_or(&new_code);

    // Find source plan
    let source_plan =
        helpers::find_in_config_array(config_json, "CFG_GPLAN", "GPLAN_CODE", &source_code)?
            .ok_or_else(|| {
                SzConfigError::NotFound(format!("Source generic plan not found: {}", source_code))
            })?;

    let source_gplan_id = source_plan
        .get("GPLAN_ID")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| {
            SzConfigError::InvalidConfig("Invalid GPLAN_ID in source plan".to_string())
        })?;

    // Check if new plan already exists
    if helpers::find_in_config_array(config_json, "CFG_GPLAN", "GPLAN_CODE", &new_code)?.is_some() {
        return Err(SzConfigError::AlreadyExists(format!(
            "Generic plan already exists: {}",
            new_code
        )));
    }

    // Get next GPLAN_ID
    let config_data: Value = serde_json::from_str(config_json)?;
    let max_gplan_id = config_data
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_GPLAN"))
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|item| item.get("GPLAN_ID").and_then(|v| v.as_i64()))
                .max()
                .unwrap_or(0)
        })
        .unwrap_or(0);

    let new_gplan_id = max_gplan_id + 1;

    // Create new plan
    let new_plan = json!({
        "GPLAN_ID": new_gplan_id,
        "GPLAN_CODE": new_code,
        "GPLAN_DESC": new_desc
    });

    let mut modified_json = helpers::add_to_config_array(config_json, "CFG_GPLAN", new_plan)?;

    // Clone all thresholds from source plan to new plan
    let config_data: Value = serde_json::from_str(&modified_json)?;
    if let Some(gthresh_array) = config_data
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_GENERIC_THRESHOLD"))
        .and_then(|v| v.as_array())
    {
        let mut cloned_thresholds = Vec::new();
        for item in gthresh_array {
            if item.get("GPLAN_ID").and_then(|v| v.as_i64()) == Some(source_gplan_id) {
                let mut cloned = item.clone();
                if let Some(obj) = cloned.as_object_mut() {
                    obj.insert("GPLAN_ID".to_string(), json!(new_gplan_id));
                }
                cloned_thresholds.push(cloned);
            }
        }

        // Add cloned thresholds
        for threshold in cloned_thresholds {
            modified_json =
                helpers::add_to_config_array(&modified_json, "CFG_GENERIC_THRESHOLD", threshold)?;
        }
    }

    Ok((modified_json, new_gplan_id))
}

/// Delete a generic plan and all its thresholds
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
/// * `gplan_code` - Plan code to delete
///
/// # Returns
///
/// Returns modified configuration JSON on success
///
/// # Example
///
/// ```
/// use sz_configtool_lib::generic_plans;
///
/// let config = r#"{"G2_CONFIG": {"CFG_GPLAN": [{"GPLAN_ID": 1, "GPLAN_CODE": "TEST"}], "CFG_GENERIC_THRESHOLD": []}}"#;
/// let modified = generic_plans::delete_generic_plan(config, "TEST").unwrap();
/// ```
pub fn delete_generic_plan(config_json: &str, gplan_code: &str) -> Result<String> {
    let gplan_code = gplan_code.to_uppercase();

    // Find the plan
    let plan = helpers::find_in_config_array(config_json, "CFG_GPLAN", "GPLAN_CODE", &gplan_code)?
        .ok_or_else(|| {
            SzConfigError::NotFound(format!("Generic plan not found: {}", gplan_code))
        })?;

    let gplan_id = plan
        .get("GPLAN_ID")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| SzConfigError::InvalidConfig("Invalid GPLAN_ID".to_string()))?;

    // Parse and modify config
    let mut config_data: Value = serde_json::from_str(config_json)?;

    // Delete the plan
    if let Some(g2_config) = config_data.get_mut("G2_CONFIG") {
        if let Some(gplan_array) = g2_config
            .get_mut("CFG_GPLAN")
            .and_then(|v| v.as_array_mut())
        {
            gplan_array
                .retain(|item| item.get("GPLAN_ID").and_then(|v| v.as_i64()) != Some(gplan_id));
        }

        // Delete all associated thresholds
        if let Some(gthresh_array) = g2_config
            .get_mut("CFG_GENERIC_THRESHOLD")
            .and_then(|v| v.as_array_mut())
        {
            gthresh_array
                .retain(|item| item.get("GPLAN_ID").and_then(|v| v.as_i64()) != Some(gplan_id));
        }
    }

    Ok(serde_json::to_string(&config_data)?)
}

/// List all generic plans in the configuration
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
/// * `filter` - Optional filter string to search in records
///
/// # Returns
///
/// Returns a vector of plan objects in Python sz_configtool format
///
/// # Example
///
/// ```
/// use sz_configtool_lib::generic_plans;
///
/// let config = r#"{"G2_CONFIG": {"CFG_GPLAN": [{"GPLAN_ID": 1, "GPLAN_CODE": "INGEST", "GPLAN_DESC": "Ingest Plan"}]}}"#;
/// let plans = generic_plans::list_generic_plans(config, None).unwrap();
/// assert_eq!(plans.len(), 1);
/// ```
pub fn list_generic_plans(config_json: &str, filter: Option<&str>) -> Result<Vec<Value>> {
    // Get all items from CFG_GPLAN
    let items = helpers::list_from_config_array(config_json, "CFG_GPLAN")?;

    // Transform and filter items
    let mut result: Vec<Value> = items
        .into_iter()
        .filter(|item| {
            if let Some(f) = filter {
                // Filter if search term appears anywhere in the record
                let item_str = item.to_string().to_lowercase();
                item_str.contains(&f.to_lowercase())
            } else {
                true
            }
        })
        .map(|item| {
            json!({
                "id": item.get("GPLAN_ID").and_then(|v| v.as_i64()).unwrap_or(0),
                "plan": item.get("GPLAN_CODE").and_then(|v| v.as_str()).unwrap_or(""),
                "description": item.get("GPLAN_DESC").and_then(|v| v.as_str()).unwrap_or("")
            })
        })
        .collect();

    // Sort by ID
    result.sort_by_key(|item| item.get("id").and_then(|v| v.as_i64()).unwrap_or(0));

    Ok(result)
}

/// Set (create or update) a generic plan
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
/// * `gplan_code` - Plan code
/// * `gplan_desc` - Plan description
///
/// # Returns
///
/// Returns `(modified_config, plan_id, was_created)` tuple on success
///
/// # Example
///
/// ```
/// use sz_configtool_lib::generic_plans;
///
/// let config = r#"{"G2_CONFIG": {"CFG_GPLAN": []}}"#;
/// let (modified, plan_id, was_created) = generic_plans::set_generic_plan(config, "CUSTOM", "Custom Plan").unwrap();
/// assert!(was_created);
/// ```
pub fn set_generic_plan(
    config_json: &str,
    gplan_code: &str,
    gplan_desc: &str,
) -> Result<(String, i64, bool)> {
    let code = gplan_code.to_uppercase();

    // Check if plan already exists
    if let Some(existing) =
        helpers::find_in_config_array(config_json, "CFG_GPLAN", "GPLAN_CODE", &code)?
    {
        // Update existing plan
        let plan_id = existing
            .get("GPLAN_ID")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);
        let mut updated = existing.clone();
        if let Some(obj) = updated.as_object_mut() {
            obj.insert("GPLAN_DESC".to_string(), json!(gplan_desc));
        }
        let modified = helpers::update_in_config_array(
            config_json,
            "CFG_GPLAN",
            "GPLAN_CODE",
            &code,
            updated,
        )?;
        Ok((modified, plan_id, false))
    } else {
        // Create new plan
        let config_data: Value = serde_json::from_str(config_json)?;
        let max_id = config_data
            .get("G2_CONFIG")
            .and_then(|g| g.get("CFG_GPLAN"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|item| item.get("GPLAN_ID").and_then(|v| v.as_i64()))
                    .max()
                    .unwrap_or(0)
            })
            .unwrap_or(0);

        let new_id = max_id + 1;
        let new_plan = json!({
            "GPLAN_ID": new_id,
            "GPLAN_CODE": code,
            "GPLAN_DESC": gplan_desc
        });

        let modified = helpers::add_to_config_array(config_json, "CFG_GPLAN", new_plan)?;
        Ok((modified, new_id, true))
    }
}
