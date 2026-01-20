use crate::error::{Result, SzConfigError};
use crate::helpers;
use serde_json::{Value, json};

// ============================================================================
// Parameter Structs
// ============================================================================

/// Parameters for adding a comparison threshold
#[derive(Debug, Clone, Default)]
pub struct AddComparisonThresholdParams {
    pub cfunc_id: i64,
    pub cfunc_rtnval: String,
    pub ftype_id: Option<i64>,
    pub exec_order: Option<i64>,
    pub same_score: Option<i64>,
    pub close_score: Option<i64>,
    pub likely_score: Option<i64>,
    pub plausible_score: Option<i64>,
    pub un_likely_score: Option<i64>,
}

impl TryFrom<&Value> for AddComparisonThresholdParams {
    type Error = SzConfigError;

    fn try_from(json: &Value) -> Result<Self> {
        Ok(Self {
            cfunc_id: json
                .get("cfuncId")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| SzConfigError::MissingField("cfuncId".to_string()))?,
            cfunc_rtnval: json
                .get("cfuncRtnval")
                .and_then(|v| v.as_str())
                .ok_or_else(|| SzConfigError::MissingField("cfuncRtnval".to_string()))?
                .to_string(),
            ftype_id: json.get("ftypeId").and_then(|v| v.as_i64()),
            exec_order: json.get("execOrder").and_then(|v| v.as_i64()),
            same_score: json.get("sameScore").and_then(|v| v.as_i64()),
            close_score: json.get("closeScore").and_then(|v| v.as_i64()),
            likely_score: json.get("likelyScore").and_then(|v| v.as_i64()),
            plausible_score: json.get("plausibleScore").and_then(|v| v.as_i64()),
            un_likely_score: json.get("unlikelyScore").and_then(|v| v.as_i64()),
        })
    }
}

/// Parameters for adding a generic threshold
#[derive(Debug, Clone)]
pub struct AddGenericThresholdParams<'a> {
    pub plan: &'a str,
    pub behavior: &'a str,
    pub scoring_cap: i64,
    pub candidate_cap: i64,
    pub send_to_redo: &'a str,
    pub feature: Option<&'a str>,
}

impl<'a> AddGenericThresholdParams<'a> {
    pub fn new(
        plan: &'a str,
        behavior: &'a str,
        scoring_cap: i64,
        candidate_cap: i64,
        send_to_redo: &'a str,
    ) -> Self {
        Self {
            plan,
            behavior,
            scoring_cap,
            candidate_cap,
            send_to_redo,
            feature: None,
        }
    }
}

impl<'a> TryFrom<&'a Value> for AddGenericThresholdParams<'a> {
    type Error = SzConfigError;

    fn try_from(json: &'a Value) -> Result<Self> {
        let plan = json
            .get("plan")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SzConfigError::MissingField("plan".to_string()))?;
        let behavior = json
            .get("behavior")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SzConfigError::MissingField("behavior".to_string()))?;
        let scoring_cap = json
            .get("scoringCap")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| SzConfigError::MissingField("scoringCap".to_string()))?;
        let candidate_cap = json
            .get("candidateCap")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| SzConfigError::MissingField("candidateCap".to_string()))?;
        let send_to_redo = json
            .get("sendToRedo")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SzConfigError::MissingField("sendToRedo".to_string()))?;

        Ok(Self {
            plan,
            behavior,
            scoring_cap,
            candidate_cap,
            send_to_redo,
            feature: json.get("feature").and_then(|v| v.as_str()),
        })
    }
}

/// Parameters for setting (updating) a comparison threshold
#[derive(Debug, Clone, Default)]
pub struct SetComparisonThresholdParams {
    pub cfrtn_id: i64,
    pub same_score: Option<i64>,
    pub close_score: Option<i64>,
    pub likely_score: Option<i64>,
    pub plausible_score: Option<i64>,
    pub un_likely_score: Option<i64>,
}

impl TryFrom<&Value> for SetComparisonThresholdParams {
    type Error = SzConfigError;

    fn try_from(json: &Value) -> Result<Self> {
        let cfrtn_id = json
            .get("cfrtid")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| SzConfigError::MissingField("cfrtnId".to_string()))?;

        Ok(Self {
            cfrtn_id,
            same_score: json.get("sameScore").and_then(|v| v.as_i64()),
            close_score: json.get("closeScore").and_then(|v| v.as_i64()),
            likely_score: json.get("likelyScore").and_then(|v| v.as_i64()),
            plausible_score: json.get("plausibleScore").and_then(|v| v.as_i64()),
            un_likely_score: json.get("unlikelyScore").and_then(|v| v.as_i64()),
        })
    }
}

/// Parameters for setting (updating) a generic threshold
#[derive(Debug, Clone, Default)]
pub struct SetGenericThresholdParams {
    pub gplan_id: i64,
    pub behavior: String,
    pub ftype_id: Option<i64>,
    pub candidate_cap: Option<i64>,
    pub scoring_cap: Option<i64>,
    pub send_to_redo: Option<String>,
}

impl TryFrom<&Value> for SetGenericThresholdParams {
    type Error = SzConfigError;

    fn try_from(json: &Value) -> Result<Self> {
        let gplan_id = json
            .get("gplanId")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| SzConfigError::MissingField("gplanId".to_string()))?;
        let behavior = json
            .get("behavior")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SzConfigError::MissingField("behavior".to_string()))?
            .to_string();

        Ok(Self {
            gplan_id,
            behavior,
            ftype_id: json.get("ftypeId").and_then(|v| v.as_i64()),
            candidate_cap: json.get("candidateCap").and_then(|v| v.as_i64()),
            scoring_cap: json.get("scoringCap").and_then(|v| v.as_i64()),
            send_to_redo: json
                .get("sendToRedo")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
        })
    }
}

/// Parameters for deleting a generic threshold
#[derive(Debug, Clone)]
pub struct DeleteGenericThresholdParams<'a> {
    pub gplan_id: i64,
    pub behavior: &'a str,
    pub feature: Option<&'a str>,
}

/// Parameters for setting a threshold (stub - not yet implemented)
#[derive(Debug, Clone, Default)]
pub struct SetThresholdParams {
    pub threshold_id: i64,
}

impl<'a> TryFrom<&'a Value> for DeleteGenericThresholdParams<'a> {
    type Error = SzConfigError;

    fn try_from(json: &'a Value) -> Result<Self> {
        let gplan_id = json
            .get("gplanId")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| SzConfigError::MissingField("gplanId".to_string()))?;
        let behavior = json
            .get("behavior")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SzConfigError::MissingField("behavior".to_string()))?;

        Ok(Self {
            gplan_id,
            behavior,
            feature: json.get("feature").and_then(|v| v.as_str()),
        })
    }
}

// ===== Comparison Thresholds (CFG_CFRTN) =====

/// Add a new comparison threshold (CFG_CFRTN record)
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `params` - Threshold parameters (cfunc_id, cfunc_rtnval required; others optional)
///
/// # Returns
/// Modified configuration JSON string
pub fn add_comparison_threshold(
    config_json: &str,
    params: AddComparisonThresholdParams,
) -> Result<String> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let ftype = params.ftype_id.unwrap_or(0);
    let rtnval_upper = params.cfunc_rtnval.to_uppercase();

    // Check if already exists
    let cfrtn_array = config["G2_CONFIG"]["CFG_CFRTN"]
        .as_array()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_CFRTN".to_string()))?;

    if cfrtn_array.iter().any(|item| {
        item["CFUNC_ID"].as_i64() == Some(params.cfunc_id)
            && item["FTYPE_ID"].as_i64() == Some(ftype)
            && item["CFUNC_RTNVAL"].as_str() == Some(rtnval_upper.as_str())
    }) {
        return Err(SzConfigError::AlreadyExists(format!(
            "Comparison threshold: CFUNC_ID={}, FTYPE_ID={}, CFUNC_RTNVAL={}",
            params.cfunc_id, ftype, rtnval_upper
        )));
    }

    // Get next ID
    let cfrtn_id = helpers::get_next_id_from_array(cfrtn_array, "CFRTN_ID")?;

    // Build record
    let mut record = json!({
        "CFRTN_ID": cfrtn_id,
        "CFUNC_ID": params.cfunc_id,
        "FTYPE_ID": ftype,
        "CFUNC_RTNVAL": rtnval_upper,
    });

    if let Some(order) = params.exec_order {
        record["EXEC_ORDER"] = json!(order);
    }
    if let Some(score) = params.same_score {
        record["SAME_SCORE"] = json!(score);
    }
    if let Some(score) = params.close_score {
        record["CLOSE_SCORE"] = json!(score);
    }
    if let Some(score) = params.likely_score {
        record["LIKELY_SCORE"] = json!(score);
    }
    if let Some(score) = params.plausible_score {
        record["PLAUSIBLE_SCORE"] = json!(score);
    }
    if let Some(score) = params.un_likely_score {
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
/// * `params` - Threshold parameters (cfrtn_id required; score fields optional)
///
/// # Returns
/// Modified configuration JSON string
pub fn set_comparison_threshold(
    config_json: &str,
    params: SetComparisonThresholdParams,
) -> Result<String> {
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let cfrtn_array = config["G2_CONFIG"]["CFG_CFRTN"]
        .as_array_mut()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_CFRTN".to_string()))?;

    let cfrtn = cfrtn_array
        .iter_mut()
        .find(|item| item["CFRTN_ID"].as_i64() == Some(params.cfrtn_id))
        .ok_or_else(|| {
            SzConfigError::NotFound(format!("Comparison threshold: {}", params.cfrtn_id))
        })?;

    // Update fields from params
    if let Some(dest_obj) = cfrtn.as_object_mut() {
        if let Some(score) = params.same_score {
            dest_obj.insert("SAME_SCORE".to_string(), json!(score));
        }
        if let Some(score) = params.close_score {
            dest_obj.insert("CLOSE_SCORE".to_string(), json!(score));
        }
        if let Some(score) = params.likely_score {
            dest_obj.insert("LIKELY_SCORE".to_string(), json!(score));
        }
        if let Some(score) = params.plausible_score {
            dest_obj.insert("PLAUSIBLE_SCORE".to_string(), json!(score));
        }
        if let Some(score) = params.un_likely_score {
            dest_obj.insert("UN_LIKELY_SCORE".to_string(), json!(score));
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
/// * `params` - Generic threshold parameters (plan, behavior, caps required; feature optional)
///
/// # Returns
/// Modified configuration JSON string
pub fn add_generic_threshold(
    config_json: &str,
    params: AddGenericThresholdParams,
) -> Result<String> {
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let plan_upper = params.plan.to_uppercase();
    let behavior_upper = params.behavior.to_uppercase();
    let redo_upper = params.send_to_redo.to_uppercase();
    let feature_upper = params.feature.unwrap_or("ALL").to_uppercase();

    // Validate sendToRedo
    if redo_upper != "YES" && redo_upper != "NO" {
        return Err(SzConfigError::InvalidInput(format!(
            "Invalid sendToRedo value '{}'. Must be 'Yes' or 'No'",
            params.send_to_redo
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
        "CANDIDATE_CAP": params.candidate_cap,
        "SCORING_CAP": params.scoring_cap,
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
/// * `params` - Delete parameters (gplan_id, behavior required; feature optional)
///
/// # Returns
/// Modified configuration JSON string
pub fn delete_generic_threshold(
    config_json: &str,
    params: DeleteGenericThresholdParams,
) -> Result<String> {
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let behavior_upper = params.behavior.to_uppercase();
    let feature_upper = params.feature.unwrap_or("ALL").to_uppercase();

    // Use the gplan_id directly from params
    let gplan_id = params.gplan_id;

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
            "Generic threshold not found: GPLAN_ID={}, behavior={}, feature={}",
            gplan_id, behavior_upper, feature_upper
        )));
    }

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// Set (update) a generic threshold
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `params` - Threshold parameters (gplan_id, behavior required; caps/redo optional)
///
/// # Returns
/// Modified configuration JSON string
pub fn set_generic_threshold(
    config_json: &str,
    params: SetGenericThresholdParams,
) -> Result<String> {
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let behavior_upper = params.behavior.to_uppercase();

    let gthresh_array = config["G2_CONFIG"]["CFG_GENERIC_THRESHOLD"]
        .as_array_mut()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_GENERIC_THRESHOLD".to_string()))?;

    let gthresh = gthresh_array
        .iter_mut()
        .find(|item| {
            item["GPLAN_ID"].as_i64() == Some(params.gplan_id)
                && item["BEHAVIOR"].as_str() == Some(behavior_upper.as_str())
        })
        .ok_or_else(|| {
            SzConfigError::NotFound(format!(
                "Generic threshold not found: GPLAN_ID={}, BEHAVIOR={}",
                params.gplan_id, behavior_upper
            ))
        })?;

    // Update fields from params
    if let Some(dest_obj) = gthresh.as_object_mut() {
        if let Some(ftype_id) = params.ftype_id {
            dest_obj.insert("FTYPE_ID".to_string(), json!(ftype_id));
        }
        if let Some(cap) = params.candidate_cap {
            dest_obj.insert("CANDIDATE_CAP".to_string(), json!(cap));
        }
        if let Some(cap) = params.scoring_cap {
            dest_obj.insert("SCORING_CAP".to_string(), json!(cap));
        }
        if let Some(redo) = params.send_to_redo {
            dest_obj.insert("SEND_TO_REDO".to_string(), json!(redo));
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
/// * `params` - Threshold parameters (threshold_id required to identify, others optional to update)
///
/// # Returns
/// Modified configuration JSON string
pub fn set_threshold(_config_json: &str, _params: SetThresholdParams) -> Result<String> {
    Err(SzConfigError::InvalidInput(
        "set_threshold not yet implemented".to_string(),
    ))
}
