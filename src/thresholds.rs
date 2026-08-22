use crate::error::{Result, SzConfigError};
use crate::helpers;
use serde::Serialize;
use serde_json::{Value, json};

// ============================================================================
// Row Structs
// ============================================================================

/// Complete CFG_CFRTN (comparison threshold) row.
///
/// Derives `Serialize` with no `skip_serializing_if`, so every key is always
/// emitted — the seed-then-null fields (EXEC_ORDER and the score fields)
/// serialize as JSON `null` when absent rather than being dropped. The Senzing
/// engine's config loader requires every key to be present, so partial rows
/// must never be written.
#[derive(Debug, Clone, Serialize)]
struct CfrtnRow {
    #[serde(rename = "CFRTN_ID")]
    cfrtn_id: i64,
    #[serde(rename = "CFUNC_ID")]
    cfunc_id: i64,
    #[serde(rename = "FTYPE_ID")]
    ftype_id: i64,
    #[serde(rename = "CFUNC_RTNVAL")]
    cfunc_rtnval: String,
    #[serde(rename = "EXEC_ORDER")]
    exec_order: Option<i64>,
    #[serde(rename = "SAME_SCORE")]
    same_score: Option<i64>,
    #[serde(rename = "CLOSE_SCORE")]
    close_score: Option<i64>,
    #[serde(rename = "LIKELY_SCORE")]
    likely_score: Option<i64>,
    #[serde(rename = "PLAUSIBLE_SCORE")]
    plausible_score: Option<i64>,
    #[serde(rename = "UN_LIKELY_SCORE")]
    un_likely_score: Option<i64>,
}

/// Complete CFG_GENERIC_THRESHOLD row.
///
/// Derives `Serialize` with no `skip_serializing_if`, so every key is always
/// emitted. All fields are always populated by the builder, so none are
/// optional. The Senzing engine's config loader requires every key to be
/// present, so partial rows must never be written.
#[derive(Debug, Clone, Serialize)]
struct GenericThresholdRow {
    #[serde(rename = "GPLAN_ID")]
    gplan_id: i64,
    #[serde(rename = "BEHAVIOR")]
    behavior: String,
    #[serde(rename = "FTYPE_ID")]
    ftype_id: i64,
    #[serde(rename = "CANDIDATE_CAP")]
    candidate_cap: i64,
    #[serde(rename = "SCORING_CAP")]
    scoring_cap: i64,
    #[serde(rename = "SEND_TO_REDO")]
    send_to_redo: String,
}

// ============================================================================
// Parameter Structs
// ============================================================================

/// Parameters for adding a comparison threshold
#[derive(Debug, Clone, Default)]
pub struct AddComparisonThresholdParams<'a> {
    pub cfunc_code: Option<&'a str>,
    pub ftype_code: Option<&'a str>,
    pub cfunc_rtnval: Option<&'a str>,
    pub exec_order: Option<i64>,
    pub same_score: Option<i64>,
    pub close_score: Option<i64>,
    pub likely_score: Option<i64>,
    pub plausible_score: Option<i64>,
    pub un_likely_score: Option<i64>,
}

impl<'a> AddComparisonThresholdParams<'a> {
    pub fn new(cfunc_code: &'a str, ftype_code: &'a str, cfunc_rtnval: &'a str) -> Self {
        Self {
            cfunc_code: Some(cfunc_code),
            ftype_code: Some(ftype_code),
            cfunc_rtnval: Some(cfunc_rtnval),
            ..Default::default()
        }
    }
}

impl<'a> TryFrom<&'a Value> for AddComparisonThresholdParams<'a> {
    type Error = SzConfigError;

    fn try_from(json: &'a Value) -> Result<Self> {
        Ok(Self {
            cfunc_code: json.get("cfuncCode").and_then(|v| v.as_str()),
            ftype_code: json.get("ftypeCode").and_then(|v| v.as_str()),
            cfunc_rtnval: json.get("cfuncRtnval").and_then(|v| v.as_str()),
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
    pub plan: Option<&'a str>,
    pub behavior: Option<&'a str>,
    pub scoring_cap: Option<i64>,
    pub candidate_cap: Option<i64>,
    pub send_to_redo: Option<&'a str>,
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
            plan: Some(plan),
            behavior: Some(behavior),
            scoring_cap: Some(scoring_cap),
            candidate_cap: Some(candidate_cap),
            send_to_redo: Some(send_to_redo),
            feature: None,
        }
    }
}

impl<'a> TryFrom<&'a Value> for AddGenericThresholdParams<'a> {
    type Error = SzConfigError;

    fn try_from(json: &'a Value) -> Result<Self> {
        Ok(Self {
            plan: json.get("plan").and_then(|v| v.as_str()),
            behavior: json.get("behavior").and_then(|v| v.as_str()),
            scoring_cap: json.get("scoringCap").and_then(|v| v.as_i64()),
            candidate_cap: json.get("candidateCap").and_then(|v| v.as_i64()),
            send_to_redo: json.get("sendToRedo").and_then(|v| v.as_str()),
            feature: json.get("feature").and_then(|v| v.as_str()),
        })
    }
}

/// Parameters for setting (updating) a comparison threshold
#[derive(Debug, Clone, Default)]
pub struct SetComparisonThresholdParams<'a> {
    pub cfunc_code: Option<&'a str>,
    pub ftype_code: Option<&'a str>,
    pub cfunc_rtnval: Option<&'a str>,
    pub exec_order: Option<i64>,
    pub same_score: Option<i64>,
    pub close_score: Option<i64>,
    pub likely_score: Option<i64>,
    pub plausible_score: Option<i64>,
    pub un_likely_score: Option<i64>,
}

impl<'a> TryFrom<&'a Value> for SetComparisonThresholdParams<'a> {
    type Error = SzConfigError;

    fn try_from(json: &'a Value) -> Result<Self> {
        Ok(Self {
            cfunc_code: json.get("cfuncCode").and_then(|v| v.as_str()),
            ftype_code: json.get("ftypeCode").and_then(|v| v.as_str()),
            cfunc_rtnval: json.get("cfuncRtnval").and_then(|v| v.as_str()),
            exec_order: json.get("execOrder").and_then(|v| v.as_i64()),
            same_score: json.get("sameScore").and_then(|v| v.as_i64()),
            close_score: json.get("closeScore").and_then(|v| v.as_i64()),
            likely_score: json.get("likelyScore").and_then(|v| v.as_i64()),
            plausible_score: json.get("plausibleScore").and_then(|v| v.as_i64()),
            un_likely_score: json.get("unlikelyScore").and_then(|v| v.as_i64()),
        })
    }
}

/// Parameters for setting (updating) a generic threshold
#[derive(Debug, Clone)]
pub struct SetGenericThresholdParams<'a> {
    pub plan: Option<&'a str>,
    pub behavior: Option<&'a str>,
    pub feature: Option<&'a str>,
    pub candidate_cap: Option<i64>,
    pub scoring_cap: Option<i64>,
    pub send_to_redo: Option<&'a str>,
}

impl<'a> TryFrom<&'a Value> for SetGenericThresholdParams<'a> {
    type Error = SzConfigError;

    fn try_from(json: &'a Value) -> Result<Self> {
        Ok(Self {
            plan: json.get("plan").and_then(|v| v.as_str()),
            behavior: json.get("behavior").and_then(|v| v.as_str()),
            feature: json.get("feature").and_then(|v| v.as_str()),
            candidate_cap: json.get("candidateCap").and_then(|v| v.as_i64()),
            scoring_cap: json.get("scoringCap").and_then(|v| v.as_i64()),
            send_to_redo: json.get("sendToRedo").and_then(|v| v.as_str()),
        })
    }
}

/// Parameters for deleting a generic threshold
#[derive(Debug, Clone, Default)]
pub struct DeleteGenericThresholdParams<'a> {
    pub plan: Option<&'a str>,
    pub behavior: Option<&'a str>,
    pub feature: Option<&'a str>,
}

impl<'a> DeleteGenericThresholdParams<'a> {
    pub fn new(plan: &'a str, behavior: &'a str) -> Self {
        Self {
            plan: Some(plan),
            behavior: Some(behavior),
            feature: None,
        }
    }

    pub fn with_feature(mut self, feature: &'a str) -> Self {
        self.feature = Some(feature);
        self
    }
}

/// Parameters for setting a threshold (stub - not yet implemented)
#[derive(Debug, Clone, Default)]
pub struct SetThresholdParams {
    pub threshold_id: i64,
}

impl<'a> TryFrom<&'a Value> for DeleteGenericThresholdParams<'a> {
    type Error = SzConfigError;

    fn try_from(json: &'a Value) -> Result<Self> {
        Ok(Self {
            plan: json.get("plan").and_then(|v| v.as_str()),
            behavior: json.get("behavior").and_then(|v| v.as_str()),
            feature: json.get("feature").and_then(|v| v.as_str()),
        })
    }
}

// ============================================================================
// Shared local helpers
// ============================================================================

/// Resolve a feature code to its `FTYPE_ID`, treating `"all"` (case-insensitive)
/// as the `0` sentinel that means "all features".
///
/// Used as a *lookup* key by the threshold add/set/delete paths.
fn resolve_ftype_id_or_all(config_json: &str, feature: &str) -> Result<i64> {
    if feature.eq_ignore_ascii_case("all") {
        Ok(0)
    } else {
        helpers::lookup_feature_id(config_json, feature)
    }
}

/// Canonicalise a `sendToRedo` value to the title-case form stored on disk.
///
/// Accepts `"Yes"`/`"No"` in any case and returns the canonical `"Yes"`/`"No"`.
/// Any other value is rejected as invalid input.
fn send_to_redo_canonical(value: &str) -> Result<&'static str> {
    match value.to_uppercase().as_str() {
        "YES" => Ok("Yes"),
        "NO" => Ok("No"),
        _ => Err(SzConfigError::InvalidInput(format!(
            "Invalid sendToRedo value '{value}'. Must be 'Yes' or 'No'"
        ))),
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
    // Extract and validate required fields
    let cfunc_code = params
        .cfunc_code
        .ok_or_else(|| SzConfigError::MissingField("cfunc_code".to_string()))?;
    let ftype_code = params
        .ftype_code
        .ok_or_else(|| SzConfigError::MissingField("ftype_code".to_string()))?;
    let cfunc_rtnval = params
        .cfunc_rtnval
        .ok_or_else(|| SzConfigError::MissingField("cfunc_rtnval".to_string()))?;

    // Lookup IDs from codes (special case: "all" = ftype_id 0)
    let cfunc_id = helpers::lookup_cfunc_id(config_json, cfunc_code)?;
    let ftype_id = if ftype_code.eq_ignore_ascii_case("all") {
        0 // Special case: "all" means ftype_id=0 (all features)
    } else {
        helpers::lookup_feature_id(config_json, ftype_code)?
    };

    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let rtnval_upper = cfunc_rtnval.to_uppercase();

    // Check if already exists
    let cfrtn_array = config["G2_CONFIG"]["CFG_CFRTN"]
        .as_array()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_CFRTN".to_string()))?;

    if cfrtn_array.iter().any(|item| {
        item["CFUNC_ID"].as_i64() == Some(cfunc_id)
            && item["FTYPE_ID"].as_i64() == Some(ftype_id)
            && item["CFUNC_RTNVAL"].as_str() == Some(rtnval_upper.as_str())
    }) {
        return Err(SzConfigError::AlreadyExists(format!(
            "Comparison threshold: {cfunc_code}+{ftype_code}+{rtnval_upper}"
        )));
    }

    // Get next ID
    let cfrtn_id = helpers::get_next_id_from_array(cfrtn_array, "CFRTN_ID")?;

    // Build a complete row via CfrtnRow so every CFG_CFRTN key is present
    // (unset seed-then-null fields serialize as null).
    let row = CfrtnRow {
        cfrtn_id,
        cfunc_id,
        ftype_id,
        cfunc_rtnval: rtnval_upper,
        exec_order: params.exec_order,
        same_score: params.same_score,
        close_score: params.close_score,
        likely_score: params.likely_score,
        plausible_score: params.plausible_score,
        un_likely_score: params.un_likely_score,
    };
    let record = serde_json::to_value(&row)?;

    helpers::add_to_config_array(config_json, "CFG_CFRTN", record)
}

/// Internal: Add comparison threshold by ID (for FFI use)
#[allow(clippy::too_many_arguments)]
pub(crate) fn add_comparison_threshold_by_id(
    config_json: &str,
    cfunc_id: i64,
    ftype_id: Option<i64>,
    cfunc_rtnval: &str,
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
        return Err(SzConfigError::AlreadyExists(
            "Comparison threshold already exists".to_string(),
        ));
    }

    // Get next ID
    let cfrtn_id = crate::helpers::get_next_id_from_array(cfrtn_array, "CFRTN_ID")?;

    // Build a complete row via CfrtnRow so every CFG_CFRTN key is present
    // (unset seed-then-null fields serialize as null).
    let row = CfrtnRow {
        cfrtn_id,
        cfunc_id,
        ftype_id: ftype,
        cfunc_rtnval: rtnval_upper,
        exec_order,
        same_score,
        close_score,
        likely_score,
        plausible_score,
        un_likely_score,
    };
    let record = serde_json::to_value(&row)?;

    crate::helpers::add_to_config_array(config_json, "CFG_CFRTN", record)
}

/// Internal: Set comparison threshold by ID (for FFI use)
pub(crate) fn set_comparison_threshold_by_id(
    config_json: &str,
    cfrtn_id: i64,
    same_score: Option<i64>,
    close_score: Option<i64>,
    likely_score: Option<i64>,
    plausible_score: Option<i64>,
    un_likely_score: Option<i64>,
) -> Result<String> {
    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let cfrtn_array = config["G2_CONFIG"]["CFG_CFRTN"]
        .as_array_mut()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_CFRTN".to_string()))?;

    let cfrtn = cfrtn_array
        .iter_mut()
        .find(|item| item["CFRTN_ID"].as_i64() == Some(cfrtn_id))
        .ok_or_else(|| SzConfigError::NotFound(format!("Comparison threshold ID: {cfrtn_id}")))?;

    // In-place update of a complete existing row; all keys preserved.
    // Update fields from params
    if let Some(dest_obj) = cfrtn.as_object_mut() {
        if let Some(score) = same_score {
            dest_obj.insert("SAME_SCORE".to_string(), json!(score));
        }
        if let Some(score) = close_score {
            dest_obj.insert("CLOSE_SCORE".to_string(), json!(score));
        }
        if let Some(score) = likely_score {
            dest_obj.insert("LIKELY_SCORE".to_string(), json!(score));
        }
        if let Some(score) = plausible_score {
            dest_obj.insert("PLAUSIBLE_SCORE".to_string(), json!(score));
        }
        if let Some(score) = un_likely_score {
            dest_obj.insert("UN_LIKELY_SCORE".to_string(), json!(score));
        }
    }

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// Internal: Delete comparison threshold by ID (for FFI use)
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `cfrtn_id` - Comparison threshold ID
pub(crate) fn delete_comparison_threshold_by_id(
    config_json: &str,
    cfrtn_id: i64,
) -> Result<String> {
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
            "Comparison threshold ID: {cfrtn_id}"
        )));
    }

    serde_json::to_string(&config).map_err(|e| SzConfigError::JsonParse(e.to_string()))
}

/// Delete a comparison threshold (CFG_CFRTN record)
///
/// Matches on the full three-key identity of a comparison threshold:
/// `(CFUNC_ID, FTYPE_ID, CFUNC_RTNVAL)`. `cfunc_rtnval` is matched
/// case-insensitively (mirroring [`set_comparison_threshold`]), and
/// `ftype_code = "all"` resolves to the `FTYPE_ID` `0` sentinel.
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `cfunc_code` - Comparison function code
/// * `ftype_code` - Feature code, or `"all"` for the all-features (0) sentinel
/// * `cfunc_rtnval` - Return value / score name (matched case-insensitively)
///
/// # Returns
/// Modified configuration JSON string
pub fn delete_comparison_threshold(
    config_json: &str,
    cfunc_code: &str,
    ftype_code: &str,
    cfunc_rtnval: &str,
) -> Result<String> {
    let cfunc_id = helpers::lookup_cfunc_id(config_json, cfunc_code)?;
    let ftype_id = resolve_ftype_id_or_all(config_json, ftype_code)?;

    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let cfrtn_array = config["G2_CONFIG"]["CFG_CFRTN"]
        .as_array_mut()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_CFRTN".to_string()))?;

    let original_len = cfrtn_array.len();
    cfrtn_array.retain(|item| {
        let matches = item["CFUNC_ID"].as_i64() == Some(cfunc_id)
            && item["FTYPE_ID"].as_i64() == Some(ftype_id)
            && item["CFUNC_RTNVAL"]
                .as_str()
                .map(|s| s.eq_ignore_ascii_case(cfunc_rtnval))
                .unwrap_or(false);
        !matches
    });

    if cfrtn_array.len() == original_len {
        return Err(SzConfigError::NotFound(format!(
            "Comparison threshold for cfunc='{cfunc_code}', ftype='{ftype_code}', rtnval='{cfunc_rtnval}'"
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
    // Extract and validate required fields
    let cfunc_code = params
        .cfunc_code
        .ok_or_else(|| SzConfigError::MissingField("cfunc_code".to_string()))?;
    let ftype_code = params
        .ftype_code
        .ok_or_else(|| SzConfigError::MissingField("ftype_code".to_string()))?;
    let cfunc_rtnval = params
        .cfunc_rtnval
        .ok_or_else(|| SzConfigError::MissingField("cfunc_rtnval".to_string()))?;

    // Lookup IDs from codes (special case: "all" = ftype_id 0)
    let cfunc_id = helpers::lookup_cfunc_id(config_json, cfunc_code)?;
    let ftype_id = if ftype_code.eq_ignore_ascii_case("all") {
        0 // Special case: "all" means ftype_id=0 (all features)
    } else {
        helpers::lookup_feature_id(config_json, ftype_code)?
    };

    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let cfrtn_array = config["G2_CONFIG"]["CFG_CFRTN"]
        .as_array_mut()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_CFRTN".to_string()))?;

    // Find threshold by (CFUNC_ID, FTYPE_ID, CFUNC_RTNVAL) - all 3 needed for uniqueness
    let cfrtn = cfrtn_array
        .iter_mut()
        .find(|item| {
            item["CFUNC_ID"].as_i64() == Some(cfunc_id)
                && item["FTYPE_ID"].as_i64() == Some(ftype_id)
                && item["CFUNC_RTNVAL"]
                    .as_str()
                    .map(|s| s.eq_ignore_ascii_case(cfunc_rtnval))
                    .unwrap_or(false)
        })
        .ok_or_else(|| {
            SzConfigError::NotFound(format!(
                "Comparison threshold: {cfunc_code}+{ftype_code}+{cfunc_rtnval}"
            ))
        })?;

    // In-place update of a complete existing row; all keys preserved.
    // Update fields from params
    if let Some(dest_obj) = cfrtn.as_object_mut() {
        if let Some(order) = params.exec_order {
            dest_obj.insert("EXEC_ORDER".to_string(), json!(order));
        }
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

    // Rebuild output with correct field order (remove cfunc_id and ensure proper order)
    let final_result: Vec<Value> = result
        .iter()
        .map(|item| {
            json!({
                "id": item["id"],
                "function": item["function"],
                "returnOrder": item["returnOrder"],
                "scoreName": item["scoreName"],
                "feature": item["feature"],
                "sameScore": item["sameScore"],
                "closeScore": item["closeScore"],
                "likelyScore": item["likelyScore"],
                "plausibleScore": item["plausibleScore"],
                "unlikelyScore": item["unlikelyScore"]
            })
        })
        .collect();

    Ok(final_result)
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
    // Extract and validate required fields
    let plan = params
        .plan
        .ok_or_else(|| SzConfigError::MissingField("plan".to_string()))?;
    let behavior = params
        .behavior
        .ok_or_else(|| SzConfigError::MissingField("behavior".to_string()))?;
    let scoring_cap = params
        .scoring_cap
        .ok_or_else(|| SzConfigError::MissingField("scoring_cap".to_string()))?;
    let candidate_cap = params
        .candidate_cap
        .ok_or_else(|| SzConfigError::MissingField("candidate_cap".to_string()))?;
    let send_to_redo = params
        .send_to_redo
        .ok_or_else(|| SzConfigError::MissingField("send_to_redo".to_string()))?;

    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let plan_upper = plan.to_uppercase();
    let behavior_upper = behavior.to_uppercase();
    let feature_upper = params.feature.unwrap_or("ALL").to_uppercase();

    // Validate sendToRedo and store the canonical title-case form ("Yes"/"No").
    let redo_canonical = send_to_redo_canonical(send_to_redo)?;

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
            "Generic threshold: plan={plan_upper}, behavior={behavior_upper}, feature={feature_upper}"
        )));
    }

    // Build a complete row via GenericThresholdRow so every
    // CFG_GENERIC_THRESHOLD key is present.
    let row = GenericThresholdRow {
        gplan_id,
        behavior: behavior_upper,
        ftype_id,
        candidate_cap,
        scoring_cap,
        send_to_redo: redo_canonical.to_string(),
    };
    let new_threshold = serde_json::to_value(&row)?;

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
    // Extract and validate required fields
    let plan = params
        .plan
        .ok_or_else(|| SzConfigError::MissingField("plan".to_string()))?;
    let behavior = params
        .behavior
        .ok_or_else(|| SzConfigError::MissingField("behavior".to_string()))?;

    let gplan_id = helpers::lookup_gplan_id(config_json, plan)?;

    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let behavior_upper = behavior.to_uppercase();
    let feature_upper = params.feature.unwrap_or("ALL").to_uppercase();

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
            "Generic threshold not found: GPLAN_ID={gplan_id}, behavior={behavior_upper}, feature={feature_upper}"
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
    // Extract and validate required fields
    let plan = params
        .plan
        .ok_or_else(|| SzConfigError::MissingField("plan".to_string()))?;
    let behavior = params
        .behavior
        .ok_or_else(|| SzConfigError::MissingField("behavior".to_string()))?;

    let gplan_id = helpers::lookup_gplan_id(config_json, plan)?;

    // The feature selects WHICH per-feature row to edit; it is a lookup key,
    // never a value to write. Defaults to the all-features (0) sentinel.
    let ftype_id = resolve_ftype_id_or_all(config_json, params.feature.unwrap_or("ALL"))?;

    // Canonicalise sendToRedo up front so an invalid value is rejected before
    // any mutation (case-insensitive validation, title-case storage).
    let redo_canonical = match params.send_to_redo {
        Some(v) => Some(send_to_redo_canonical(v)?),
        None => None,
    };

    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let behavior_upper = behavior.to_uppercase();

    let gthresh_array = config["G2_CONFIG"]["CFG_GENERIC_THRESHOLD"]
        .as_array_mut()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_GENERIC_THRESHOLD".to_string()))?;

    // Match on the FULL triple (GPLAN_ID, BEHAVIOR, FTYPE_ID) so per-feature
    // rows that share a (plan, behaviour) are distinguished by feature.
    let gthresh = gthresh_array
        .iter_mut()
        .find(|item| {
            item["GPLAN_ID"].as_i64() == Some(gplan_id)
                && item["BEHAVIOR"].as_str() == Some(behavior_upper.as_str())
                && item["FTYPE_ID"].as_i64() == Some(ftype_id)
        })
        .ok_or_else(|| {
            SzConfigError::NotFound(format!(
                "Generic threshold not found: GPLAN_ID={gplan_id}, BEHAVIOR={behavior_upper}, FTYPE_ID={ftype_id}"
            ))
        })?;

    // In-place update of a complete existing row; all keys preserved.
    // FTYPE_ID is a lookup key and must never be overwritten here.
    if let Some(dest_obj) = gthresh.as_object_mut() {
        if let Some(cap) = params.candidate_cap {
            dest_obj.insert("CANDIDATE_CAP".to_string(), json!(cap));
        }
        if let Some(cap) = params.scoring_cap {
            dest_obj.insert("SCORING_CAP".to_string(), json!(cap));
        }
        if let Some(redo) = redo_canonical {
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

#[cfg(test)]
mod tests {
    use super::*;

    const CFRTN_KEYS: [&str; 10] = [
        "CFRTN_ID",
        "CFUNC_ID",
        "FTYPE_ID",
        "CFUNC_RTNVAL",
        "EXEC_ORDER",
        "SAME_SCORE",
        "CLOSE_SCORE",
        "LIKELY_SCORE",
        "PLAUSIBLE_SCORE",
        "UN_LIKELY_SCORE",
    ];

    const GENERIC_THRESHOLD_KEYS: [&str; 6] = [
        "GPLAN_ID",
        "BEHAVIOR",
        "FTYPE_ID",
        "CANDIDATE_CAP",
        "SCORING_CAP",
        "SEND_TO_REDO",
    ];

    fn assert_all_keys(row: &Value, keys: &[&str]) {
        let obj = row.as_object().unwrap();
        for key in keys {
            assert!(obj.contains_key(*key), "{key} key must be present");
        }
    }

    #[test]
    fn test_add_comparison_threshold_emits_all_keys() {
        let config = r#"{"G2_CONFIG": {
            "CFG_CFRTN": [],
            "CFG_CFUNC": [{"CFUNC_ID": 5, "CFUNC_CODE": "GNR_COMP"}],
            "CFG_FTYPE": [{"FTYPE_ID": 3, "FTYPE_CODE": "NAME"}]
        }}"#;

        // Only the required fields supplied; the seed-then-null fields must
        // surface as null, never dropped.
        let params = AddComparisonThresholdParams::new("GNR_COMP", "NAME", "FULL_SCORE");
        let modified = add_comparison_threshold(config, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let row = &value["G2_CONFIG"]["CFG_CFRTN"][0];

        assert_all_keys(row, &CFRTN_KEYS);
        assert_eq!(row["CFUNC_ID"], json!(5));
        assert_eq!(row["FTYPE_ID"], json!(3));
        assert_eq!(row["CFUNC_RTNVAL"], json!("FULL_SCORE"));
        assert_eq!(row["EXEC_ORDER"], Value::Null);
        assert_eq!(row["SAME_SCORE"], Value::Null);
        assert_eq!(row["UN_LIKELY_SCORE"], Value::Null);
    }

    #[test]
    fn test_add_comparison_threshold_by_id_emits_all_keys() {
        let config = r#"{"G2_CONFIG": {"CFG_CFRTN": []}}"#;

        let modified = add_comparison_threshold_by_id(
            config,
            5,
            Some(3),
            "full_score",
            Some(1),
            Some(100),
            None,
            None,
            None,
            None,
        )
        .unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let row = &value["G2_CONFIG"]["CFG_CFRTN"][0];

        assert_all_keys(row, &CFRTN_KEYS);
        assert_eq!(row["CFUNC_RTNVAL"], json!("FULL_SCORE")); // uppercased
        assert_eq!(row["EXEC_ORDER"], json!(1));
        assert_eq!(row["SAME_SCORE"], json!(100));
        assert_eq!(row["CLOSE_SCORE"], Value::Null);
        assert_eq!(row["PLAUSIBLE_SCORE"], Value::Null);
    }

    #[test]
    fn test_add_generic_threshold_emits_all_keys() {
        let config = r#"{"G2_CONFIG": {
            "CFG_GPLAN": [{"GPLAN_ID": 1, "GPLAN_CODE": "INGEST"}],
            "CFG_GENERIC_THRESHOLD": [],
            "CFG_FTYPE": [{"FTYPE_ID": 3, "FTYPE_CODE": "NAME"}]
        }}"#;

        let params = AddGenericThresholdParams::new("INGEST", "NAME", 20, 10, "No");
        let modified = add_generic_threshold(config, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let row = &value["G2_CONFIG"]["CFG_GENERIC_THRESHOLD"][0];

        assert_all_keys(row, &GENERIC_THRESHOLD_KEYS);
        assert_eq!(row["GPLAN_ID"], json!(1));
        assert_eq!(row["BEHAVIOR"], json!("NAME"));
        // feature defaults to "ALL" -> ftype_id 0.
        assert_eq!(row["FTYPE_ID"], json!(0));
        assert_eq!(row["CANDIDATE_CAP"], json!(10));
        assert_eq!(row["SCORING_CAP"], json!(20));
        // Canonical storage is title-case "No", not "NO".
        assert_eq!(row["SEND_TO_REDO"], json!("No"));
    }

    #[test]
    fn test_add_generic_threshold_send_to_redo_yes_title_case() {
        let config = r#"{"G2_CONFIG": {
            "CFG_GPLAN": [{"GPLAN_ID": 1, "GPLAN_CODE": "INGEST"}],
            "CFG_GENERIC_THRESHOLD": [],
            "CFG_FTYPE": [{"FTYPE_ID": 3, "FTYPE_CODE": "NAME"}]
        }}"#;

        // Lower-case input must be accepted and stored title-case.
        let params = AddGenericThresholdParams::new("INGEST", "NAME", 20, 10, "yes");
        let modified = add_generic_threshold(config, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let row = &value["G2_CONFIG"]["CFG_GENERIC_THRESHOLD"][0];
        assert_eq!(row["SEND_TO_REDO"], json!("Yes"));
    }

    #[test]
    fn test_add_generic_threshold_rejects_unknown_send_to_redo() {
        let config = r#"{"G2_CONFIG": {
            "CFG_GPLAN": [{"GPLAN_ID": 1, "GPLAN_CODE": "INGEST"}],
            "CFG_GENERIC_THRESHOLD": [],
            "CFG_FTYPE": [{"FTYPE_ID": 3, "FTYPE_CODE": "NAME"}]
        }}"#;

        let params = AddGenericThresholdParams::new("INGEST", "NAME", 20, 10, "maybe");
        let err = add_generic_threshold(config, params).unwrap_err();
        assert_eq!(err.kind(), crate::error::SzErrorKind::InvalidInput);
    }

    /// Rows sharing (GPLAN_ID, BEHAVIOR) but differing by FTYPE_ID must be
    /// distinguished by the feature key; the matched row's FTYPE_ID must never
    /// be overwritten.
    #[test]
    fn test_set_generic_threshold_edits_correct_per_feature_row() {
        let config = r#"{"G2_CONFIG": {
            "CFG_GPLAN": [{"GPLAN_ID": 1, "GPLAN_CODE": "INGEST"}],
            "CFG_FTYPE": [{"FTYPE_ID": 3, "FTYPE_CODE": "NAME"}],
            "CFG_GENERIC_THRESHOLD": [
                {"GPLAN_ID": 1, "BEHAVIOR": "FF", "FTYPE_ID": 0,
                 "CANDIDATE_CAP": 10, "SCORING_CAP": 20, "SEND_TO_REDO": "No"},
                {"GPLAN_ID": 1, "BEHAVIOR": "FF", "FTYPE_ID": 3,
                 "CANDIDATE_CAP": 11, "SCORING_CAP": 21, "SEND_TO_REDO": "No"}
            ]
        }}"#;

        let params = SetGenericThresholdParams {
            plan: Some("INGEST"),
            behavior: Some("FF"),
            feature: Some("NAME"),
            candidate_cap: Some(99),
            scoring_cap: None,
            send_to_redo: Some("yes"),
        };
        let modified = set_generic_threshold(config, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let rows = value["G2_CONFIG"]["CFG_GENERIC_THRESHOLD"]
            .as_array()
            .unwrap();

        // The FTYPE_ID=0 row is untouched.
        assert_eq!(rows[0]["CANDIDATE_CAP"], json!(10));
        assert_eq!(rows[0]["SEND_TO_REDO"], json!("No"));
        assert_eq!(rows[0]["FTYPE_ID"], json!(0));

        // The NAME (FTYPE_ID=3) row is edited; FTYPE_ID preserved, redo canonical.
        assert_eq!(rows[1]["FTYPE_ID"], json!(3));
        assert_eq!(rows[1]["CANDIDATE_CAP"], json!(99));
        assert_eq!(rows[1]["SCORING_CAP"], json!(21));
        assert_eq!(rows[1]["SEND_TO_REDO"], json!("Yes"));
    }

    #[test]
    fn test_set_generic_threshold_defaults_to_all_sentinel() {
        let config = r#"{"G2_CONFIG": {
            "CFG_GPLAN": [{"GPLAN_ID": 1, "GPLAN_CODE": "INGEST"}],
            "CFG_FTYPE": [{"FTYPE_ID": 3, "FTYPE_CODE": "NAME"}],
            "CFG_GENERIC_THRESHOLD": [
                {"GPLAN_ID": 1, "BEHAVIOR": "FF", "FTYPE_ID": 0,
                 "CANDIDATE_CAP": 10, "SCORING_CAP": 20, "SEND_TO_REDO": "No"}
            ]
        }}"#;

        // No feature supplied -> matches the FTYPE_ID=0 row.
        let params = SetGenericThresholdParams {
            plan: Some("INGEST"),
            behavior: Some("FF"),
            feature: None,
            candidate_cap: Some(5),
            scoring_cap: None,
            send_to_redo: None,
        };
        let modified = set_generic_threshold(config, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let row = &value["G2_CONFIG"]["CFG_GENERIC_THRESHOLD"][0];
        assert_eq!(row["FTYPE_ID"], json!(0));
        assert_eq!(row["CANDIDATE_CAP"], json!(5));
    }

    #[test]
    fn test_set_generic_threshold_rejects_unknown_send_to_redo() {
        let config = r#"{"G2_CONFIG": {
            "CFG_GPLAN": [{"GPLAN_ID": 1, "GPLAN_CODE": "INGEST"}],
            "CFG_FTYPE": [],
            "CFG_GENERIC_THRESHOLD": [
                {"GPLAN_ID": 1, "BEHAVIOR": "FF", "FTYPE_ID": 0,
                 "CANDIDATE_CAP": 10, "SCORING_CAP": 20, "SEND_TO_REDO": "No"}
            ]
        }}"#;

        let params = SetGenericThresholdParams {
            plan: Some("INGEST"),
            behavior: Some("FF"),
            feature: None,
            candidate_cap: None,
            scoring_cap: None,
            send_to_redo: Some("nope"),
        };
        let err = set_generic_threshold(config, params).unwrap_err();
        assert_eq!(err.kind(), crate::error::SzErrorKind::InvalidInput);
    }

    /// delete_comparison_threshold must match the full 3-key identity and leave
    /// rows that differ in any key intact. "all" resolves to FTYPE_ID 0.
    #[test]
    fn test_delete_comparison_threshold_three_key_match() {
        let config = r#"{"G2_CONFIG": {
            "CFG_CFUNC": [{"CFUNC_ID": 5, "CFUNC_CODE": "GNR_COMP"}],
            "CFG_FTYPE": [{"FTYPE_ID": 3, "FTYPE_CODE": "NAME"}],
            "CFG_CFRTN": [
                {"CFRTN_ID": 1, "CFUNC_ID": 5, "FTYPE_ID": 3, "CFUNC_RTNVAL": "FULL_SCORE"},
                {"CFRTN_ID": 2, "CFUNC_ID": 5, "FTYPE_ID": 3, "CFUNC_RTNVAL": "CLOSE_SCORE"},
                {"CFRTN_ID": 3, "CFUNC_ID": 5, "FTYPE_ID": 0, "CFUNC_RTNVAL": "FULL_SCORE"}
            ]
        }}"#;

        // Case-insensitive rtnval; only the exact 3-key row is removed.
        let modified =
            delete_comparison_threshold(config, "GNR_COMP", "NAME", "full_score").unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let rows = value["G2_CONFIG"]["CFG_CFRTN"].as_array().unwrap();
        assert_eq!(rows.len(), 2);
        let ids: Vec<i64> = rows
            .iter()
            .map(|r| r["CFRTN_ID"].as_i64().unwrap())
            .collect();
        assert_eq!(ids, vec![2, 3]);

        // "all" -> FTYPE_ID 0 sentinel.
        let modified2 =
            delete_comparison_threshold(&modified, "GNR_COMP", "all", "FULL_SCORE").unwrap();
        let value2: Value = serde_json::from_str(&modified2).unwrap();
        let rows2 = value2["G2_CONFIG"]["CFG_CFRTN"].as_array().unwrap();
        assert_eq!(rows2.len(), 1);
        assert_eq!(rows2[0]["CFRTN_ID"], json!(2));
    }

    #[test]
    fn test_delete_comparison_threshold_not_found() {
        let config = r#"{"G2_CONFIG": {
            "CFG_CFUNC": [{"CFUNC_ID": 5, "CFUNC_CODE": "GNR_COMP"}],
            "CFG_FTYPE": [{"FTYPE_ID": 3, "FTYPE_CODE": "NAME"}],
            "CFG_CFRTN": [
                {"CFRTN_ID": 1, "CFUNC_ID": 5, "FTYPE_ID": 3, "CFUNC_RTNVAL": "FULL_SCORE"}
            ]
        }}"#;

        // Right cfunc+ftype, wrong rtnval -> NotFound (3-key addressing).
        let err =
            delete_comparison_threshold(config, "GNR_COMP", "NAME", "CLOSE_SCORE").unwrap_err();
        assert_eq!(err.kind(), crate::error::SzErrorKind::NotFound);
    }
}
