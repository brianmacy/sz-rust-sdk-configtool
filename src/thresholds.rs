use crate::error::{Result, SzConfigError};
use crate::helpers;
use serde_json::{Value, json};

// ===== Comparison Thresholds (CFG_CFRTN) =====

/// Add a new comparison threshold (CFG_CFRTN record)
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `cfunc_id` - Comparison function ID
/// * `cfunc_rtnval` - Return value code
/// * `ftype_id` - Optional feature type ID (default: 0 for all features)
/// * `exec_order` - Optional execution order
/// * `same_score` - Optional same score threshold
/// * `close_score` - Optional close score threshold
/// * `likely_score` - Optional likely score threshold
/// * `plausible_score` - Optional plausible score threshold
/// * `un_likely_score` - Optional unlikely score threshold
///
/// # Returns
/// Modified configuration JSON string
#[allow(clippy::too_many_arguments)]
pub fn add_comparison_threshold(
    config_json: &str,
    cfunc_id: i64,
    cfunc_rtnval: &str,
    ftype_id: Option<i64>,
    exec_order: Option<i64>,
    same_score: Option<i64>,
    close_score: Option<i64>,
    likely_score: Option<i64>,
    plausible_score: Option<i64>,
    un_likely_score: Option<i64>,
) -> Result<String> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let ftype = ftype_id.unwrap_or(0);
    let rtnval_upper = cfunc_rtnval.to_uppercase();

    // Check if already exists
    let cfrtn_array = config["G2_CONFIG"]["CFG_CFRTN"]
        .as_array()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_CFRTN".to_string()))?;

    if cfrtn_array.iter().any(|item| {
        item["CFUNC_ID"].as_i64() == Some(cfunc_id)
            && item["FTYPE_ID"].as_i64() == Some(ftype)
            && item["CFUNC_RTNVAL"].as_str() == Some(rtnval_upper.as_str())
    }) {
        return Err(SzConfigError::AlreadyExists(format!(
            "Comparison threshold: CFUNC_ID={}, FTYPE_ID={}, CFUNC_RTNVAL={}",
            cfunc_id, ftype, rtnval_upper
        )));
    }

    // Get next ID
    let cfrtn_id = helpers::get_next_id_from_array(cfrtn_array, "CFRTN_ID")?;

    // Build record
    let mut record = json!({
        "CFRTN_ID": cfrtn_id,
        "CFUNC_ID": cfunc_id,
        "FTYPE_ID": ftype,
        "CFUNC_RTNVAL": rtnval_upper,
    });

    if let Some(order) = exec_order {
        record["EXEC_ORDER"] = json!(order);
    }
    if let Some(score) = same_score {
        record["SAME_SCORE"] = json!(score);
    }
    if let Some(score) = close_score {
        record["CLOSE_SCORE"] = json!(score);
    }
    if let Some(score) = likely_score {
        record["LIKELY_SCORE"] = json!(score);
    }
    if let Some(score) = plausible_score {
        record["PLAUSIBLE_SCORE"] = json!(score);
    }
    if let Some(score) = un_likely_score {
        record["UN_LIKELY_SCORE"] = json!(score);
    }

    helpers::add_to_config_array(config_json, "CFG_CFRTN", record)
}

/// Delete a comparison threshold by ID
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `cfrtn_id` - Comparison threshold ID
///
/// # Returns
/// Modified configuration JSON string
pub fn delete_comparison_threshold(config_json: &str, cfrtn_id: i64) -> Result<String> {
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let mut found = false;

    if let Some(cfrtn_array) = config["G2_CONFIG"]["CFG_CFRTN"].as_array_mut() {
        cfrtn_array.retain(|item| {
            let matches = item["CFRTN_ID"].as_i64() == Some(cfrtn_id);
            if matches {
                found = true;
            }
            !matches
        });
    }

    if !found {
        return Err(SzConfigError::NotFound(format!(
            "Comparison threshold: {}",
            cfrtn_id
        )));
    }

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// Set (update) a comparison threshold
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `cfrtn_id` - Comparison threshold ID
/// * `updates` - JSON object with fields to update
///
/// # Returns
/// Modified configuration JSON string
pub fn set_comparison_threshold(
    config_json: &str,
    cfrtn_id: i64,
    updates: &Value,
) -> Result<String> {
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let cfrtn_array = config["G2_CONFIG"]["CFG_CFRTN"]
        .as_array_mut()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_CFRTN".to_string()))?;

    let cfrtn = cfrtn_array
        .iter_mut()
        .find(|item| item["CFRTN_ID"].as_i64() == Some(cfrtn_id))
        .ok_or_else(|| SzConfigError::NotFound(format!("Comparison threshold: {}", cfrtn_id)))?;

    // Update fields from updates object
    if let Some(src_obj) = updates.as_object() {
        if let Some(dest_obj) = cfrtn.as_object_mut() {
            for (key, value) in src_obj {
                let upper_key = key.to_uppercase();
                // Skip the ID field
                if upper_key != "CFRTN_ID" {
                    dest_obj.insert(upper_key, value.clone());
                }
            }
        }
    }

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// List all comparison thresholds with resolved names
///
/// # Arguments
/// * `config_json` - JSON configuration string
///
/// # Returns
/// Vector of JSON Values with id, function, returnOrder, scoreName, feature, and score fields
pub fn list_comparison_thresholds(config_json: &str) -> Result<Vec<Value>> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let cfrtn_array = config["G2_CONFIG"]["CFG_CFRTN"]
        .as_array()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_CFRTN".to_string()))?;

    let cfunc_array = config["G2_CONFIG"]["CFG_CFUNC"]
        .as_array()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_CFUNC".to_string()))?;

    let ftype_array = config["G2_CONFIG"]["CFG_FTYPE"]
        .as_array()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_FTYPE".to_string()))?;

    let mut result: Vec<Value> = cfrtn_array
        .iter()
        .map(|item| {
            let cfunc_id = item["CFUNC_ID"].as_i64().unwrap_or(0);
            let ftype_id = item["FTYPE_ID"].as_i64().unwrap_or(0);
            let cfrtn_id = item["CFRTN_ID"].as_i64().unwrap_or(0);

            // Resolve function name
            let function = cfunc_array
                .iter()
                .find(|cf| cf["CFUNC_ID"].as_i64() == Some(cfunc_id))
                .and_then(|cf| cf["CFUNC_CODE"].as_str())
                .unwrap_or("unknown")
                .to_string();

            // Resolve feature name
            let feature = if ftype_id == 0 {
                "all".to_string()
            } else {
                ftype_array
                    .iter()
                    .find(|ft| ft["FTYPE_ID"].as_i64() == Some(ftype_id))
                    .and_then(|ft| ft["FTYPE_CODE"].as_str())
                    .unwrap_or("unknown")
                    .to_string()
            };

            json!({
                "id": cfrtn_id,
                "cfunc_id": cfunc_id,  // Keep for sorting
                "function": function,
                "returnOrder": item["EXEC_ORDER"].as_i64().unwrap_or(0),
                "scoreName": item["CFUNC_RTNVAL"].as_str().unwrap_or(""),
                "feature": feature,
                "sameScore": item["SAME_SCORE"].as_i64().unwrap_or(0),
                "closeScore": item["CLOSE_SCORE"].as_i64().unwrap_or(0),
                "likelyScore": item["LIKELY_SCORE"].as_i64().unwrap_or(0),
                "plausibleScore": item["PLAUSIBLE_SCORE"].as_i64().unwrap_or(0),
                "unlikelyScore": item["UN_LIKELY_SCORE"].as_i64().unwrap_or(0)
            })
        })
        .collect();

    // Sort by CFUNC_ID and CFRTN_ID (like Python) - not by function name
    result.sort_by_key(|e| {
        (
            e["cfunc_id"].as_i64().unwrap_or(0),
            e["id"].as_i64().unwrap_or(0),
        )
    });

    // Remove cfunc_id from output (it was only for sorting)
    for item in &mut result {
        if let Some(obj) = item.as_object_mut() {
            obj.remove("cfunc_id");
        }
    }

    Ok(result)
}

// ===== Generic Thresholds (CFG_GENERIC_THRESHOLD) =====

/// Add a new generic threshold (CFG_GENERIC_THRESHOLD record)
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `plan` - Generic plan name
/// * `behavior` - Behavior code
/// * `scoring_cap` - Scoring cap threshold
/// * `candidate_cap` - Candidate cap threshold
/// * `send_to_redo` - Send to redo flag ("Yes" or "No")
/// * `feature` - Optional feature name (default: "ALL" for all features)
///
/// # Returns
/// Modified configuration JSON string
pub fn add_generic_threshold(
    config_json: &str,
    plan: &str,
    behavior: &str,
    scoring_cap: i64,
    candidate_cap: i64,
    send_to_redo: &str,
    feature: Option<&str>,
) -> Result<String> {
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let plan_upper = plan.to_uppercase();
    let behavior_upper = behavior.to_uppercase();
    let redo_upper = send_to_redo.to_uppercase();
    let feature_upper = feature.unwrap_or("ALL").to_uppercase();

    // Validate sendToRedo
    if redo_upper != "YES" && redo_upper != "NO" {
        return Err(SzConfigError::InvalidInput(format!(
            "Invalid sendToRedo value '{}'. Must be 'Yes' or 'No'",
            send_to_redo
        )));
    }

    // Lookup plan ID
    let gplan_array = config["G2_CONFIG"]["CFG_GPLAN"]
        .as_array()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_GPLAN".to_string()))?;

    let gplan_id = gplan_array
        .iter()
        .find(|p| p["GPLAN_CODE"].as_str() == Some(plan_upper.as_str()))
        .and_then(|p| p["GPLAN_ID"].as_i64())
        .ok_or_else(|| SzConfigError::NotFound(format!("Generic plan: {}", plan_upper.clone())))?;

    // Lookup feature ID (0 for "all")
    let ftype_id = if feature_upper == "ALL" {
        0
    } else {
        let ftype_array = config["G2_CONFIG"]["CFG_FTYPE"]
            .as_array()
            .ok_or_else(|| SzConfigError::MissingSection("CFG_FTYPE".to_string()))?;

        ftype_array
            .iter()
            .find(|f| f["FTYPE_CODE"].as_str() == Some(feature_upper.as_str()))
            .and_then(|f| f["FTYPE_ID"].as_i64())
            .ok_or_else(|| SzConfigError::NotFound(format!("Feature: {}", feature_upper.clone())))?
    };

    // Check if threshold already exists
    let gthresh_array = config["G2_CONFIG"]["CFG_GENERIC_THRESHOLD"]
        .as_array()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_GENERIC_THRESHOLD".to_string()))?;

    if gthresh_array.iter().any(|record| {
        record["GPLAN_ID"].as_i64() == Some(gplan_id)
            && record["BEHAVIOR"].as_str() == Some(behavior_upper.as_str())
            && record["FTYPE_ID"].as_i64() == Some(ftype_id)
    }) {
        return Err(SzConfigError::AlreadyExists(format!(
            "Generic threshold: plan={}, behavior={}, feature={}",
            plan_upper, behavior_upper, feature_upper
        )));
    }

    // Create new threshold record
    let new_threshold = json!({
        "GPLAN_ID": gplan_id,
        "BEHAVIOR": behavior_upper,
        "FTYPE_ID": ftype_id,
        "CANDIDATE_CAP": candidate_cap,
        "SCORING_CAP": scoring_cap,
        "SEND_TO_REDO": redo_upper
    });

    if let Some(threshold_array) = config["G2_CONFIG"]["CFG_GENERIC_THRESHOLD"].as_array_mut() {
        threshold_array.push(new_threshold);
    }

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// Delete a generic threshold
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `plan` - Generic plan name
/// * `behavior` - Behavior code
/// * `feature` - Optional feature name (default: "ALL")
///
/// # Returns
/// Modified configuration JSON string
pub fn delete_generic_threshold(
    config_json: &str,
    plan: &str,
    behavior: &str,
    feature: Option<&str>,
) -> Result<String> {
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let plan_upper = plan.to_uppercase();
    let behavior_upper = behavior.to_uppercase();
    let feature_upper = feature.unwrap_or("ALL").to_uppercase();

    // Lookup plan ID
    let gplan_array = config["G2_CONFIG"]["CFG_GPLAN"]
        .as_array()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_GPLAN".to_string()))?;

    let gplan_id = gplan_array
        .iter()
        .find(|p| p["GPLAN_CODE"].as_str() == Some(plan_upper.as_str()))
        .and_then(|p| p["GPLAN_ID"].as_i64())
        .ok_or_else(|| SzConfigError::NotFound(format!("Generic plan: {}", plan_upper.clone())))?;

    // Lookup feature ID (0 for "all")
    let ftype_id = if feature_upper == "ALL" {
        0
    } else {
        let ftype_array = config["G2_CONFIG"]["CFG_FTYPE"]
            .as_array()
            .ok_or_else(|| SzConfigError::MissingSection("CFG_FTYPE".to_string()))?;

        ftype_array
            .iter()
            .find(|f| f["FTYPE_CODE"].as_str() == Some(feature_upper.as_str()))
            .and_then(|f| f["FTYPE_ID"].as_i64())
            .ok_or_else(|| SzConfigError::NotFound(format!("Feature: {}", feature_upper.clone())))?
    };

    // Find and delete threshold record
    let mut found = false;
    if let Some(threshold_array) = config["G2_CONFIG"]["CFG_GENERIC_THRESHOLD"].as_array_mut() {
        threshold_array.retain(|record| {
            let matches = record["GPLAN_ID"].as_i64() == Some(gplan_id)
                && record["BEHAVIOR"].as_str() == Some(behavior_upper.as_str())
                && record["FTYPE_ID"].as_i64() == Some(ftype_id);
            if matches {
                found = true;
            }
            !matches
        });
    }

    if !found {
        return Err(SzConfigError::NotFound(format!(
            "Generic threshold not found: plan={}, behavior={}, feature={}",
            plan_upper, behavior_upper, feature_upper
        )));
    }

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// Set (update) a generic threshold
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `gplan_id` - Generic plan ID
/// * `behavior` - Behavior code
/// * `updates` - JSON object with fields to update
///
/// # Returns
/// Modified configuration JSON string
pub fn set_generic_threshold(
    config_json: &str,
    gplan_id: i64,
    behavior: &str,
    updates: &Value,
) -> Result<String> {
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let behavior_upper = behavior.to_uppercase();

    let gthresh_array = config["G2_CONFIG"]["CFG_GENERIC_THRESHOLD"]
        .as_array_mut()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_GENERIC_THRESHOLD".to_string()))?;

    let gthresh = gthresh_array
        .iter_mut()
        .find(|item| {
            item["GPLAN_ID"].as_i64() == Some(gplan_id)
                && item["BEHAVIOR"].as_str() == Some(behavior_upper.as_str())
        })
        .ok_or_else(|| {
            SzConfigError::NotFound(format!(
                "Generic threshold not found: GPLAN_ID={}, BEHAVIOR={}",
                gplan_id, behavior_upper
            ))
        })?;

    // Update fields from updates object
    if let Some(src_obj) = updates.as_object() {
        if let Some(dest_obj) = gthresh.as_object_mut() {
            for (key, value) in src_obj {
                let upper_key = key.to_uppercase();
                // Skip ID and BEHAVIOR fields
                if upper_key != "GPLAN_ID" && upper_key != "BEHAVIOR" {
                    dest_obj.insert(upper_key, value.clone());
                }
            }
        }
    }

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// List all generic thresholds with resolved names
///
/// # Arguments
/// * `config_json` - JSON configuration string
///
/// # Returns
/// Vector of JSON Values with plan, behavior, feature, candidateCap, scoringCap, and sendToRedo fields
pub fn list_generic_thresholds(config_json: &str) -> Result<Vec<Value>> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let gthresh_array = config["G2_CONFIG"]["CFG_GENERIC_THRESHOLD"]
        .as_array()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_GENERIC_THRESHOLD".to_string()))?;

    let gplan_array = config["G2_CONFIG"]["CFG_GPLAN"]
        .as_array()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_GPLAN".to_string()))?;

    let ftype_array = config["G2_CONFIG"]["CFG_FTYPE"]
        .as_array()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_FTYPE".to_string()))?;

    let result: Vec<Value> = gthresh_array
        .iter()
        .map(|item| {
            let gplan_id = item["GPLAN_ID"].as_i64().unwrap_or(0);
            let ftype_id = item["FTYPE_ID"].as_i64().unwrap_or(0);

            // Resolve plan name
            let plan = gplan_array
                .iter()
                .find(|gp| gp["GPLAN_ID"].as_i64() == Some(gplan_id))
                .and_then(|gp| gp["GPLAN_CODE"].as_str())
                .unwrap_or("unknown")
                .to_string();

            // Resolve feature name
            let feature = if ftype_id == 0 {
                "all".to_string()
            } else {
                ftype_array
                    .iter()
                    .find(|ft| ft["FTYPE_ID"].as_i64() == Some(ftype_id))
                    .and_then(|ft| ft["FTYPE_CODE"].as_str())
                    .unwrap_or("unknown")
                    .to_string()
            };

            json!({
                "plan": plan,
                "behavior": item["BEHAVIOR"].as_str().unwrap_or(""),
                "feature": feature,
                "candidateCap": item["CANDIDATE_CAP"].as_i64().unwrap_or(0),
                "scoringCap": item["SCORING_CAP"].as_i64().unwrap_or(0),
                "sendToRedo": item["SEND_TO_REDO"].as_str().unwrap_or("")
            })
        })
        .collect();

    Ok(result)
}

/// Get threshold level by ID
///
/// This is a placeholder for get_threshold() functionality.
/// TODO: Determine exact requirements for this function.
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `threshold_id` - Threshold ID
///
/// # Returns
/// JSON Value representing the threshold
pub fn get_threshold(_config_json: &str, _threshold_id: i64) -> Result<Value> {
    Err(SzConfigError::InvalidInput(
        "get_threshold not yet implemented".to_string(),
    ))
}

/// Set threshold level by ID
///
/// This is a placeholder for set_threshold() functionality.
/// TODO: Determine exact requirements for this function.
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `threshold_id` - Threshold ID
/// * `updates` - JSON object with fields to update
///
/// # Returns
/// Modified configuration JSON string
pub fn set_threshold(_config_json: &str, _threshold_id: i64, _updates: &Value) -> Result<String> {
    Err(SzConfigError::InvalidInput(
        "set_threshold not yet implemented".to_string(),
    ))
}
