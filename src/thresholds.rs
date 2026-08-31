use crate::error::{Result, SzConfigError, ValidationFailure, ValidationReason};
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
    /// Requested return-value execution order (tier). This is only consulted
    /// when no all-features tier row already exists for this
    /// `(cfunc, rtnval)` — that tier's order is reused verbatim so per-feature
    /// overrides stay on the same tier as the base row (a load-bearing scoring
    /// invariant). When no tier row exists, `Some(n > 0)` is honoured (or
    /// rejected with `AlreadyExists` if taken within the `(CFUNC_ID,
    /// FTYPE_ID=0)` scope) and `None` auto-allocates the next order. The written
    /// row always carries a concrete order. See the "Execution-order policy" in
    /// [`crate::calls`].
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
            exec_order: optional_i64(json, "execOrder")?,
            same_score: optional_i64(json, "sameScore")?,
            close_score: optional_i64(json, "closeScore")?,
            likely_score: optional_i64(json, "likelyScore")?,
            plausible_score: optional_i64(json, "plausibleScore")?,
            un_likely_score: optional_i64(json, "unlikelyScore")?,
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
            scoring_cap: optional_i64(json, "scoringCap")?,
            candidate_cap: optional_i64(json, "candidateCap")?,
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
            exec_order: optional_i64(json, "execOrder")?,
            same_score: optional_i64(json, "sameScore")?,
            close_score: optional_i64(json, "closeScore")?,
            likely_score: optional_i64(json, "likelyScore")?,
            plausible_score: optional_i64(json, "plausibleScore")?,
            un_likely_score: optional_i64(json, "unlikelyScore")?,
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
            candidate_cap: optional_i64(json, "candidateCap")?,
            scoring_cap: optional_i64(json, "scoringCap")?,
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

/// Read an optional integer field from a JSON object with strict typing.
///
/// This exists so a present-but-wrong-type numeric field (a JSON string, float,
/// or bool) is *rejected* rather than silently coerced to `None` — the trap the
/// old `.and_then(Value::as_i64)` parsing fell into, where
/// `{"candidateCap": "500"}` was quietly dropped and the update became a no-op.
///
/// - missing key or JSON `null` -> `Ok(None)` (the field is genuinely absent)
/// - integer value -> `Ok(Some(n))`
/// - any other present value (string / float / bool / array / object) ->
///   `Err(SzConfigError::InvalidInput)`
fn optional_i64(json: &Value, key: &str) -> Result<Option<i64>> {
    match json.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(v) => v
            .as_i64()
            .map(Some)
            .ok_or_else(|| SzConfigError::InvalidInput(format!("{key} must be an integer"))),
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

/// Resolve the `EXEC_ORDER` for a new `CFG_CFRTN` (comparison threshold) row.
///
/// Mirrors Python `do_addComparisonThreshold`'s three-step logic, which is
/// load-bearing for scoring — it keeps every return value of one comparison
/// function on the same tier across features:
///
/// 1. **Tier reuse.** If a return-value tier row already exists for this
///    `(CFUNC_ID, CFUNC_RTNVAL)` at the all-features level (`FTYPE_ID = 0`), its
///    `EXEC_ORDER` is reused verbatim — a per-feature override must land on the
///    same tier as the base row. This takes precedence over any `desired` value,
///    and a naive max+1 here would be a silent scoring regression.
/// 2. **Honour desired.** Otherwise, if `desired` is `Some(n > 0)`, it is
///    honoured — unless already taken within the `(CFUNC_ID, FTYPE_ID = 0)`
///    scope, in which case `AlreadyExists` is returned (reject-if-taken policy).
/// 3. **Next available.** Otherwise the next free order within
///    `(CFUNC_ID, FTYPE_ID = 0)` is allocated.
///
/// Never returns `null`: an order is always resolved to a concrete value.
fn resolve_cfrtn_exec_order(
    cfrtn_array: &[Value],
    cfunc_id: i64,
    rtnval_upper: &str,
    desired: Option<i64>,
) -> Result<i64> {
    // Step 1: reuse the all-features tier row's EXEC_ORDER when present.
    let tier_order = cfrtn_array.iter().find_map(|row| {
        let is_tier = row["CFUNC_ID"].as_i64() == Some(cfunc_id)
            && row["FTYPE_ID"].as_i64() == Some(0)
            && row["CFUNC_RTNVAL"]
                .as_str()
                .map(|s| s.eq_ignore_ascii_case(rtnval_upper))
                .unwrap_or(false);
        if is_tier {
            row["EXEC_ORDER"].as_i64()
        } else {
            None
        }
    });
    if let Some(order) = tier_order {
        return Ok(order);
    }

    // Steps 2 & 3: honour desired (reject if taken) else next-available, scoped
    // to (CFUNC_ID, FTYPE_ID = 0) — exactly Python's getDesiredValueOrNext
    // over ["CFUNC_ID", "FTYPE_ID", "EXEC_ORDER"] with [cfuncID, 0, ...].
    helpers::get_desired_or_next_order(
        cfrtn_array,
        "EXEC_ORDER",
        &[("CFUNC_ID", cfunc_id), ("FTYPE_ID", 0)],
        desired,
    )
}

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

    // Resolve EXEC_ORDER: reuse the all-features tier row when present, else
    // honour/allocate within (CFUNC_ID, FTYPE_ID=0). Never null.
    let exec_order =
        resolve_cfrtn_exec_order(cfrtn_array, cfunc_id, &rtnval_upper, params.exec_order)?;

    // Build a complete row via CfrtnRow so every CFG_CFRTN key is present
    // (unset seed-then-null score fields serialize as null).
    let row = CfrtnRow {
        cfrtn_id,
        cfunc_id,
        ftype_id,
        cfunc_rtnval: rtnval_upper,
        exec_order: Some(exec_order),
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

    // Resolve EXEC_ORDER: reuse the all-features tier row when present, else
    // honour/allocate within (CFUNC_ID, FTYPE_ID=0). Never null.
    let resolved_exec_order =
        resolve_cfrtn_exec_order(cfrtn_array, cfunc_id, &rtnval_upper, exec_order)?;

    // Build a complete row via CfrtnRow so every CFG_CFRTN key is present
    // (unset seed-then-null score fields serialize as null).
    let row = CfrtnRow {
        cfrtn_id,
        cfunc_id,
        ftype_id: ftype,
        cfunc_rtnval: rtnval_upper,
        exec_order: Some(resolved_exec_order),
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

/// Which reference lookup failed while staging a generic-threshold validation.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenericThresholdRef {
    /// The generic plan (`CFG_GPLAN`) was not found.
    Plan,
    /// The feature (`CFG_FTYPE`) was not found.
    Feature,
}

/// The staged outcome of [`validate_generic_threshold`] for the ADD path.
///
/// Every variant is returned as `Ok(..)` DATA — only a genuinely internal error
/// (e.g. unparseable config JSON) is an `Err`. This mirrors Python
/// `do_addGenericThreshold`'s staging order: a fatal-first plan lookup, then a
/// fatal-first feature lookup, then a warning-success duplicate check, and only
/// then the aggregated behaviour + `sendToRedo` field validation.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenericThresholdCheck {
    /// A plan or feature reference was not found (fatal-first): the command must
    /// stop here (Python emits only "Plan/Feature X not found" and never runs
    /// the field validator).
    NotFound {
        /// Which reference lookup failed.
        which: GenericThresholdRef,
        /// The (upper-cased) code that was not found.
        value: String,
    },
    /// The `(plan, behavior, feature)` tuple already exists. Python treats this
    /// as a warning and the command still *succeeds* (a no-op add), so this is
    /// deliberately NOT an error.
    Duplicate,
    /// One or more of the aggregated fields (`behavior`, `sendToRedo`) failed
    /// validation, in canonical order `[behavior, sendToRedo]`.
    Invalid(Vec<ValidationFailure>),
    /// All checks passed; the add would succeed.
    Ok,
}

/// Build the aggregated behaviour + `sendToRedo` validation failures for a
/// generic-threshold ADD, in canonical order `[behavior, sendToRedo]`.
///
/// Shared by [`validate_generic_threshold`] and [`add_generic_threshold`] so the
/// two never drift. Mirrors Python `validateGenericThreshold`'s two SDK-relevant
/// checks (the two cap checks are enforced strictly upstream as scalar
/// [`SzConfigError::InvalidInput`], never folded in here):
///
///   1. `behavior_upper` must be one of the canonical behaviour codes
///      (`behavior_domain::behavior_position`, the exact set Python's
///      `lookupBehaviorCode` checks) -> `UnknownReferenceCode`.
///   2. `send_to_redo` must canonicalise to `"Yes"`/`"No"` -> `OutOfDomain`.
///
/// `behavior_upper` must already be upper-cased by the caller; its verbatim
/// (upper-cased) value is echoed as the offending value.
fn collect_generic_threshold_failures(
    behavior_upper: &str,
    send_to_redo: &str,
) -> Vec<ValidationFailure> {
    let mut failures = Vec::new();
    if crate::behavior_domain::behavior_position(behavior_upper).is_none() {
        failures.push(ValidationFailure::new(
            "behavior",
            ValidationReason::UnknownReferenceCode,
            Some(behavior_upper.to_string()),
        ));
    }
    if send_to_redo_canonical(send_to_redo).is_err() {
        failures.push(ValidationFailure::new(
            "sendToRedo",
            ValidationReason::OutOfDomain,
            Some(send_to_redo.to_string()),
        ));
    }
    failures
}

/// Validate a generic-threshold ADD without mutating the config.
///
/// This is the CLI's orchestration surface: it stages the checks in Python
/// `do_addGenericThreshold` order and returns every staged outcome as
/// [`GenericThresholdCheck`] DATA (`Ok(..)`), reserving `Err` for genuinely
/// internal failures (e.g. unparseable config JSON):
///
/// 1. **plan lookup** (fatal-first) -> [`GenericThresholdCheck::NotFound`]
///    with [`GenericThresholdRef::Plan`].
/// 2. **feature lookup** (fatal-first, unless the feature is absent or `"all"`)
///    -> [`GenericThresholdCheck::NotFound`] with [`GenericThresholdRef::Feature`].
/// 3. **duplicate** `(plan, behavior, feature)` -> [`GenericThresholdCheck::Duplicate`]
///    (warning-success; the command still succeeds as a no-op).
/// 4. **field aggregate** (`behavior` + `sendToRedo`) -> [`GenericThresholdCheck::Invalid`],
///    else [`GenericThresholdCheck::Ok`].
///
/// Caps are deliberately not taken here: they are typed `i64` at the API/FFI
/// boundary and rejected strictly upstream, so they never reach this staging.
pub fn validate_generic_threshold(
    config_json: &str,
    plan: &str,
    behavior: &str,
    send_to_redo: &str,
    feature: Option<&str>,
) -> Result<GenericThresholdCheck> {
    let config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let plan_upper = plan.to_uppercase();
    let behavior_upper = behavior.to_uppercase();
    let feature_upper = feature.unwrap_or("ALL").to_uppercase();

    // Stage 1: plan lookup (fatal-first).
    let gplan_id = match config["G2_CONFIG"]["CFG_GPLAN"]
        .as_array()
        .and_then(|rows| {
            rows.iter()
                .find(|p| p["GPLAN_CODE"].as_str() == Some(plan_upper.as_str()))
        })
        .and_then(|p| p["GPLAN_ID"].as_i64())
    {
        Some(id) => id,
        None => {
            return Ok(GenericThresholdCheck::NotFound {
                which: GenericThresholdRef::Plan,
                value: plan_upper,
            });
        }
    };

    // Stage 2: feature lookup (fatal-first); "ALL" is the 0 sentinel.
    let ftype_id = if feature_upper == "ALL" {
        0
    } else {
        match config["G2_CONFIG"]["CFG_FTYPE"]
            .as_array()
            .and_then(|rows| {
                rows.iter()
                    .find(|f| f["FTYPE_CODE"].as_str() == Some(feature_upper.as_str()))
                    .and_then(|f| f["FTYPE_ID"].as_i64())
            }) {
            Some(id) => id,
            None => {
                return Ok(GenericThresholdCheck::NotFound {
                    which: GenericThresholdRef::Feature,
                    value: feature_upper,
                });
            }
        }
    };

    // Stage 3: duplicate (warning-success).
    let is_duplicate = config["G2_CONFIG"]["CFG_GENERIC_THRESHOLD"]
        .as_array()
        .map(|rows| {
            rows.iter().any(|record| {
                record["GPLAN_ID"].as_i64() == Some(gplan_id)
                    && record["BEHAVIOR"].as_str() == Some(behavior_upper.as_str())
                    && record["FTYPE_ID"].as_i64() == Some(ftype_id)
            })
        })
        .unwrap_or(false);
    if is_duplicate {
        return Ok(GenericThresholdCheck::Duplicate);
    }

    // Stage 4: aggregated field validation (behavior + sendToRedo).
    let failures = collect_generic_threshold_failures(&behavior_upper, send_to_redo);
    if failures.is_empty() {
        Ok(GenericThresholdCheck::Ok)
    } else {
        Ok(GenericThresholdCheck::Invalid(failures))
    }
}

/// Add a new generic threshold (CFG_GENERIC_THRESHOLD record)
///
/// # Arguments
/// * `config_json` - JSON configuration string
/// * `params` - Generic threshold parameters (plan, behavior, caps required; feature optional)
///
/// # Returns
/// Modified configuration JSON string
///
/// # Duplicates
/// A duplicate `(plan, behavior, feature)` tuple is rejected here with
/// [`SzConfigError::AlreadyExists`]. This differs from Python's
/// `do_addGenericThreshold`, which treats a duplicate as a warning and succeeds
/// without writing. Callers wanting that Python-parity behaviour should call
/// [`validate_generic_threshold`] first and skip the add when it returns
/// [`GenericThresholdCheck::Duplicate`].
pub fn add_generic_threshold(
    config_json: &str,
    params: AddGenericThresholdParams,
) -> Result<String> {
    // Aggregate ALL missing required fields into a single MissingField error,
    // mirroring Python `do_addGenericThreshold`'s up-front `validate_parms`
    // (which reports every absent parameter at once rather than one at a time).
    // Field order matches the Python required list: PLAN, BEHAVIOR, SCORINGCAP,
    // CANDIDATECAP, SENDTOREDO.
    let mut missing: Vec<&str> = Vec::new();
    if params.plan.is_none() {
        missing.push("plan");
    }
    if params.behavior.is_none() {
        missing.push("behavior");
    }
    if params.scoring_cap.is_none() {
        missing.push("scoring_cap");
    }
    if params.candidate_cap.is_none() {
        missing.push("candidate_cap");
    }
    if params.send_to_redo.is_none() {
        missing.push("send_to_redo");
    }
    if !missing.is_empty() {
        return Err(SzConfigError::MissingField(missing.join(", ")));
    }
    let plan = params.plan.expect("checked present above");
    let behavior = params.behavior.expect("checked present above");
    let scoring_cap = params.scoring_cap.expect("checked present above");
    let candidate_cap = params.candidate_cap.expect("checked present above");
    let send_to_redo = params.send_to_redo.expect("checked present above");

    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let plan_upper = plan.to_uppercase();
    let behavior_upper = behavior.to_uppercase();
    let feature_upper = params.feature.unwrap_or("ALL").to_uppercase();

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

    // Aggregated validity block, mirroring Python `validateGenericThreshold`
    // (which appends every failure to one `errorList` and reports them together
    // AFTER the plan/feature/duplicate checks). The two SDK-relevant checks —
    // BEHAVIOR against the canonical domain and SEND_TO_REDO against [Yes, No] —
    // are built by the shared `collect_generic_threshold_failures` so this
    // producer and `validate_generic_threshold` never drift. Each failure is
    // carried as structured DATA in `SzConfigError::ValidationErrors`
    // (canonical order [behavior, sendToRedo]) rather than a lossy joined
    // string. The caps are already `i64` (typed at the API/FFI boundary via
    // `optional_i64`), so Python's `isinstance(int)` cap checks are enforced
    // upstream and need no runtime check here.
    let failures = collect_generic_threshold_failures(&behavior_upper, send_to_redo);
    if !failures.is_empty() {
        return Err(SzConfigError::ValidationErrors(failures));
    }
    let redo_canonical = send_to_redo_canonical(send_to_redo)
        .expect("no validity errors implies redo canonicalised");

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

    let mut config: Value =
        serde_json::from_str(config_json).map_err(|e| SzConfigError::JsonParse(e.to_string()))?;

    let behavior_upper = behavior.to_uppercase();

    let gthresh_array = config["G2_CONFIG"]["CFG_GENERIC_THRESHOLD"]
        .as_array_mut()
        .ok_or_else(|| SzConfigError::MissingSection("CFG_GENERIC_THRESHOLD".to_string()))?;

    // Match on the FULL triple (GPLAN_ID, BEHAVIOR, FTYPE_ID) so per-feature
    // rows that share a (plan, behaviour) are distinguished by feature. BEHAVIOR
    // is part of this lookup KEY, so an unknown/non-canonical behaviour surfaces
    // here as NotFound (row absent) — it is NOT re-validated as a reference
    // code. This mirrors Python `do_setGenericThreshold`, which looks the row up
    // by (plan, behavior, feature) BEFORE running `validateGenericThreshold` on
    // the merged record (whose BEHAVIOR is copied from the found row and is
    // therefore always canonical). Only `sendToRedo` is genuinely re-validated.
    let idx = gthresh_array
        .iter()
        .position(|item| {
            item["GPLAN_ID"].as_i64() == Some(gplan_id)
                && item["BEHAVIOR"].as_str() == Some(behavior_upper.as_str())
                && item["FTYPE_ID"].as_i64() == Some(ftype_id)
        })
        .ok_or_else(|| {
            SzConfigError::NotFound(format!(
                "Generic threshold not found: GPLAN_ID={gplan_id}, BEHAVIOR={behavior_upper}, FTYPE_ID={ftype_id}"
            ))
        })?;

    // Validate sendToRedo AFTER the row lookup (Python ordering: a missing row
    // wins over a bad sendToRedo). A bad value aggregates into ValidationErrors
    // — one uniform structured surface across ADD and SET for the CLI — rather
    // than the old scalar InvalidInput.
    let redo_canonical = match params.send_to_redo {
        Some(v) => match send_to_redo_canonical(v) {
            Ok(canonical) => Some(canonical),
            Err(_) => {
                return Err(SzConfigError::ValidationErrors(vec![
                    ValidationFailure::new(
                        "sendToRedo",
                        ValidationReason::OutOfDomain,
                        Some(v.to_string()),
                    ),
                ]));
            }
        },
        None => None,
    };

    let gthresh = &mut gthresh_array[idx];

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
/// The rows are sorted inside the SDK by `(GPLAN_ID, behaviour-code position)` —
/// the behaviour order coming from [`crate::behavior_domain::behavior_position`]
/// — so callers never need to re-sort or reverse-map the plan code to its id. The
/// projection carries `id` (the `GPLAN_ID`), previously absent.
///
/// # Arguments
/// * `config_json` - JSON configuration string
///
/// # Returns
/// Vector of JSON Values with id, plan, behavior, feature, candidateCap, scoringCap, and sendToRedo fields
///
/// # Example
/// ```
/// use sz_configtool_lib::thresholds::list_generic_thresholds;
/// let config = r#"{"G2_CONFIG": {
///     "CFG_GPLAN": [{"GPLAN_ID": 1, "GPLAN_CODE": "INGEST"}],
///     "CFG_FTYPE": [],
///     "CFG_GENERIC_THRESHOLD": [
///         {"GPLAN_ID": 1, "BEHAVIOR": "F1", "FTYPE_ID": 0, "CANDIDATE_CAP": 10,
///          "SCORING_CAP": -1, "SEND_TO_REDO": "Yes"},
///         {"GPLAN_ID": 1, "BEHAVIOR": "NAME", "FTYPE_ID": 0, "CANDIDATE_CAP": 10,
///          "SCORING_CAP": -1, "SEND_TO_REDO": "Yes"}
///     ]
/// }}"#;
/// let rows = list_generic_thresholds(config).unwrap();
/// // NAME sorts before F1 despite being stored second.
/// assert_eq!(rows[0]["behavior"], "NAME");
/// assert_eq!(rows[0]["id"], 1);
/// ```
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

    // Sort the raw rows by (GPLAN_ID, behaviour-code position) before projection
    // so the numeric sort key is never lost. The behaviour-code order comes from
    // the SDK-owned canonical domain (behavior_domain::behavior_position); an
    // unrecognised behaviour sorts last. sort_by_key is stable, so per-feature
    // rows sharing a (plan, behaviour) keep their stored relative order.
    let mut sorted: Vec<&Value> = gthresh_array.iter().collect();
    sorted.sort_by_key(|item| {
        let gplan_id = item["GPLAN_ID"].as_i64().unwrap_or(0);
        let behavior = item["BEHAVIOR"].as_str().unwrap_or("");
        let pos = crate::behavior_domain::behavior_position(behavior).unwrap_or(usize::MAX);
        (gplan_id, pos)
    });

    let result: Vec<Value> = sorted
        .into_iter()
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

            // `id` carries the GPLAN_ID: generic-threshold rows have no single
            // surrogate key, and the plan id is the numeric key callers need to
            // sort/round-trip (previously reverse-mapped from `plan`).
            json!({
                "id": gplan_id,
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

        // Only the required fields supplied. EXEC_ORDER is now auto-allocated
        // (never null): with no tier row and an empty (CFUNC_ID 5, FTYPE_ID 0)
        // scope, the first order is 1. The score fields remain seed-then-null.
        let params = AddComparisonThresholdParams::new("GNR_COMP", "NAME", "FULL_SCORE");
        let modified = add_comparison_threshold(config, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let row = &value["G2_CONFIG"]["CFG_CFRTN"][0];

        assert_all_keys(row, &CFRTN_KEYS);
        assert_eq!(row["CFUNC_ID"], json!(5));
        assert_eq!(row["FTYPE_ID"], json!(3));
        assert_eq!(row["CFUNC_RTNVAL"], json!("FULL_SCORE"));
        assert_eq!(row["EXEC_ORDER"], json!(1));
        assert_eq!(row["SAME_SCORE"], Value::Null);
        assert_eq!(row["UN_LIKELY_SCORE"], Value::Null);
    }

    // Tier reuse (synthetic): a per-feature override reuses the all-features
    // tier row's EXEC_ORDER verbatim, even when a different order is requested.
    #[test]
    fn test_add_comparison_threshold_reuses_all_features_tier_order() {
        let config = r#"{"G2_CONFIG": {
            "CFG_CFUNC": [{"CFUNC_ID": 5, "CFUNC_CODE": "GNR_COMP"}],
            "CFG_FTYPE": [{"FTYPE_ID": 3, "FTYPE_CODE": "NAME"}],
            "CFG_CFRTN": [
                {"CFRTN_ID": 1, "CFUNC_ID": 5, "FTYPE_ID": 0, "CFUNC_RTNVAL": "GNR_SN", "EXEC_ORDER": 4}
            ]
        }}"#;

        // Adding a NAME-specific GNR_SN row must reuse the tier order (4), NOT
        // honour the requested 99 (tier reuse takes precedence — scoring
        // invariant).
        let mut params = AddComparisonThresholdParams::new("GNR_COMP", "NAME", "GNR_SN");
        params.exec_order = Some(99);
        let modified = add_comparison_threshold(config, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let new_row = value["G2_CONFIG"]["CFG_CFRTN"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["FTYPE_ID"].as_i64() == Some(3))
            .unwrap();
        assert_eq!(new_row["EXEC_ORDER"], json!(4));
    }

    // Reject-if-taken (Q1): with no tier row, an explicit order already used in
    // the (CFUNC_ID, FTYPE_ID=0) scope is rejected rather than reallocated.
    #[test]
    fn test_add_comparison_threshold_rejects_taken_explicit_order() {
        let config = r#"{"G2_CONFIG": {
            "CFG_CFUNC": [{"CFUNC_ID": 5, "CFUNC_CODE": "GNR_COMP"}],
            "CFG_FTYPE": [{"FTYPE_ID": 3, "FTYPE_CODE": "NAME"}],
            "CFG_CFRTN": [
                {"CFRTN_ID": 1, "CFUNC_ID": 5, "FTYPE_ID": 0, "CFUNC_RTNVAL": "GNR_FN", "EXEC_ORDER": 1}
            ]
        }}"#;

        // New rtnval (no tier row), explicit order 1 already taken in the
        // (5, 0) scope -> AlreadyExists.
        let mut params = AddComparisonThresholdParams::new("GNR_COMP", "all", "GNR_SN");
        params.exec_order = Some(1);
        let err = add_comparison_threshold(config, params).unwrap_err();
        assert_eq!(err.kind(), crate::error::SzErrorKind::AlreadyExists);
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
        assert_eq!(err.kind(), crate::error::SzErrorKind::ValidationErrors);
        let failures = err.validation_failures().unwrap();
        assert_eq!(failures.len(), 1);
        assert_eq!(failures[0].field, "sendToRedo");
        assert_eq!(
            failures[0].reason_code,
            crate::error::ValidationReason::OutOfDomain
        );
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
        assert_eq!(err.kind(), crate::error::SzErrorKind::ValidationErrors);
        let failures = err.validation_failures().unwrap();
        assert_eq!(failures.len(), 1);
        assert_eq!(failures[0].field, "sendToRedo");
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

    #[test]
    fn test_list_generic_thresholds_carries_id_and_sorts_by_behavior() {
        // Stored order deliberately differs from (GPLAN_ID, behaviour-position):
        // within plan 1, FF is stored before NAME/F1; plan 2 stored before plan 1.
        let config = r#"{"G2_CONFIG": {
            "CFG_GPLAN": [
                {"GPLAN_ID": 1, "GPLAN_CODE": "INGEST"},
                {"GPLAN_ID": 2, "GPLAN_CODE": "SEARCH"}
            ],
            "CFG_FTYPE": [{"FTYPE_ID": 3, "FTYPE_CODE": "NAME"}],
            "CFG_GENERIC_THRESHOLD": [
                {"GPLAN_ID": 2, "BEHAVIOR": "F1", "FTYPE_ID": 0, "CANDIDATE_CAP": 5, "SCORING_CAP": 5, "SEND_TO_REDO": "Yes"},
                {"GPLAN_ID": 1, "BEHAVIOR": "FF", "FTYPE_ID": 0, "CANDIDATE_CAP": 20, "SCORING_CAP": 20, "SEND_TO_REDO": "No"},
                {"GPLAN_ID": 1, "BEHAVIOR": "NAME", "FTYPE_ID": 0, "CANDIDATE_CAP": 10, "SCORING_CAP": -1, "SEND_TO_REDO": "Yes"},
                {"GPLAN_ID": 1, "BEHAVIOR": "F1", "FTYPE_ID": 0, "CANDIDATE_CAP": 5, "SCORING_CAP": 5, "SEND_TO_REDO": "Yes"}
            ]
        }}"#;

        let rows = list_generic_thresholds(config).unwrap();
        // Plan 1 first (all its rows), each carrying id = GPLAN_ID, in behaviour
        // order NAME < F1 < FF; then plan 2.
        assert_eq!(rows[0]["id"], json!(1));
        assert_eq!(rows[0]["behavior"], json!("NAME"));
        assert_eq!(rows[1]["behavior"], json!("F1"));
        assert_eq!(rows[2]["behavior"], json!("FF"));
        assert_eq!(rows[3]["id"], json!(2));
        assert_eq!(rows[3]["behavior"], json!("F1"));
    }
}
