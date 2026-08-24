//! Rule (CFG_ERRULE) operations
//!
//! Functions for managing entity resolution rules in the configuration.
//! Rules define matching and relationship logic based on fragments.

use crate::error::{Result, SzConfigError};
use crate::helpers::{self, FieldUpdate};
use serde::Serialize;
use serde_json::{Value, json};

// ============================================================================
// Row Structs
// ============================================================================

/// Complete CFG_ERRULE row.
///
/// This struct is the single source of truth for the on-disk shape of a rule.
/// It derives `Serialize` with no `skip_serializing_if`, so every key is always
/// emitted — optional fields serialize as JSON `null` rather than being dropped.
/// The Senzing engine's config loader requires every key to be present (a
/// missing key yields SENZ9117), so partial rows must never be written.
#[derive(Debug, Clone, Serialize)]
pub(crate) struct ErruleRow {
    #[serde(rename = "ERRULE_ID")]
    errule_id: i64,
    #[serde(rename = "ERRULE_CODE")]
    errule_code: String,
    #[serde(rename = "RESOLVE")]
    resolve: String,
    #[serde(rename = "RELATE")]
    relate: String,
    #[serde(rename = "RTYPE_ID")]
    rtype_id: i64,
    #[serde(rename = "QUAL_ERFRAG_CODE")]
    qual_erfrag_code: Option<String>,
    #[serde(rename = "DISQ_ERFRAG_CODE")]
    disq_erfrag_code: Option<String>,
    #[serde(rename = "ERRULE_TIER")]
    errule_tier: Option<i64>,
}

impl ErruleRow {
    /// Build a complete row from a caller-provided JSON object plus a resolved
    /// id and code. Missing scalar fields fall back to Senzing defaults and
    /// missing optional fields become `None` (serialized as `null`), so the
    /// resulting row always carries every CFG_ERRULE key.
    fn from_config(id: i64, code: String, cfg: &Value) -> Self {
        Self {
            errule_id: id,
            errule_code: code,
            resolve: cfg
                .get("RESOLVE")
                .and_then(|v| v.as_str())
                .unwrap_or("No")
                .to_string(),
            relate: cfg
                .get("RELATE")
                .and_then(|v| v.as_str())
                .unwrap_or("No")
                .to_string(),
            rtype_id: cfg.get("RTYPE_ID").and_then(|v| v.as_i64()).unwrap_or(1),
            qual_erfrag_code: helpers::field_as_string(cfg, "QUAL_ERFRAG_CODE"),
            disq_erfrag_code: helpers::field_as_string(cfg, "DISQ_ERFRAG_CODE"),
            errule_tier: cfg.get("ERRULE_TIER").and_then(|v| v.as_i64()),
        }
    }
}

/// Check whether a fragment code exists in `CFG_ERFRAG` (case-insensitive).
fn fragment_code_exists(config: &Value, frag_upper: &str) -> bool {
    config
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_ERFRAG"))
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter().any(|item| {
                item.get("ERFRAG_CODE")
                    .and_then(|v| v.as_str())
                    .map(|c| c.eq_ignore_ascii_case(frag_upper))
                    .unwrap_or(false)
            })
        })
        .unwrap_or(false)
}

/// Validate that a single fragment/disqualifier code names an existing
/// `CFG_ERFRAG` row.
///
/// `required` selects the reference's nullability, matching Python's
/// `validateRule`:
/// - **`required` (the fragment / `QUAL_ERFRAG_CODE`)** — Python calls
///   `lookupFragment(...)` unconditionally, so a blank `""` fails lookup and is
///   rejected here too (`Fragment "" not found`). `None` is skipped (the caller,
///   e.g. `set_rule`, only validates a code it is actually changing).
/// - **not `required` (the disqualifier / `DISQ_ERFRAG_CODE`)** — Python guards
///   its lookup with `if record.get("DISQ_ERFRAG_CODE"):`, so a blank clears the
///   nullable reference and is accepted.
///
/// `label` distinguishes the message (`"Fragment"` vs `"Disqualifier"`). The
/// message uses double quotes for Python parity (D13).
///
/// This is the single-code building block so callers can validate *only* the
/// codes they are changing (as `set_rule` does), while `validate_rule_row`
/// composes it to validate every code present on a new row.
fn validate_fragment_code(
    config: &Value,
    code: Option<&str>,
    label: &str,
    required: bool,
) -> Result<()> {
    match code {
        Some(c) if !c.is_empty() => {
            let upper = c.to_uppercase();
            if !fragment_code_exists(config, &upper) {
                return Err(SzConfigError::NotFound(format!(
                    "{label} \"{upper}\" not found"
                )));
            }
        }
        // A blank *required* reference (the fragment) is rejected, matching
        // Python's unconditional `lookupFragment("")`; a blank *nullable*
        // reference (the disqualifier) is accepted (clears it).
        Some(c) if c.is_empty() && required => {
            return Err(SzConfigError::NotFound(format!("{label} \"\" not found")));
        }
        // An *absent* required reference is also rejected: Python's `do_addRule`
        // lists FRAGMENT as a required parameter (`{attr} is required`). Only the
        // add path reaches this (set_rule validates only codes it is Setting, so
        // it never passes `None` for a required code).
        None if required => {
            return Err(SzConfigError::MissingField(format!("{label} is required")));
        }
        _ => {}
    }
    Ok(())
}

/// Validate and normalise the non-fragment invariants of a proposed rule row.
///
/// This is the shared core that both `add_rule` and `set_rule` delegate to so
/// their RESOLVE/RELATE/RTYPE_ID handling can never drift. It performs, in
/// order:
///
/// 1. **Duplicate-code check** (only when `is_new`).
/// 2. **RESOLVE / RELATE domain** — each must be `Yes`/`No` (case-insensitive);
///    the returned row is normalised to title-case.
/// 3. **Mutual exclusivity** — a rule may not both resolve and relate.
/// 4. **RESOLVE=Yes requires a non-zero tier**, then **RTYPE_ID coherence** —
///    `RESOLVE=Yes` auto-corrects `RTYPE_ID` to `1`; `RELATE=Yes` requires
///    `RTYPE_ID` in `[2, 3, 4]`.
///
/// Fragment/disqualifier *existence* is intentionally NOT checked here: `add_rule`
/// validates every code on the new row via [`validate_rule_row`], while `set_rule`
/// validates only the codes it is changing via [`validate_fragment_code`], never
/// re-validating a code carried over unchanged.
fn validate_rule_row_core(config: &Value, row: &ErruleRow, is_new: bool) -> Result<ErruleRow> {
    let mut out = row.clone();

    // 1. Duplicate ERRULE_CODE (adds only; an update targets an existing code).
    if is_new
        && config
            .get("G2_CONFIG")
            .and_then(|g| g.get("CFG_ERRULE"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter().any(|item| {
                    item.get("ERRULE_CODE")
                        .and_then(|v| v.as_str())
                        .map(|c| c.eq_ignore_ascii_case(&out.errule_code))
                        .unwrap_or(false)
                })
            })
            .unwrap_or(false)
    {
        return Err(SzConfigError::AlreadyExists(format!(
            "Rule '{}' already exists",
            out.errule_code
        )));
    }

    // 2. RESOLVE domain + normalise.
    let resolve_upper = out.resolve.to_uppercase();
    if resolve_upper != "YES" && resolve_upper != "NO" {
        return Err(SzConfigError::InvalidInput(
            "resolve value must be in [\"Yes\", \"No\"]".to_string(),
        ));
    }
    out.resolve = if resolve_upper == "YES" { "Yes" } else { "No" }.to_string();

    // 2. RELATE domain + normalise.
    let relate_upper = out.relate.to_uppercase();
    if relate_upper != "YES" && relate_upper != "NO" {
        return Err(SzConfigError::InvalidInput(
            "relate value must be in [\"Yes\", \"No\"]".to_string(),
        ));
    }
    out.relate = if relate_upper == "YES" { "Yes" } else { "No" }.to_string();

    // 3. Mutual exclusivity.
    if out.resolve == "Yes" && out.relate == "Yes" {
        return Err(SzConfigError::InvalidInput(
            "A rule must either resolve or relate, please set the other to No".to_string(),
        ));
    }

    // 4. RESOLVE=Yes requires a tier, then RTYPE_ID coherence / auto-correct.
    if out.resolve == "Yes" {
        // Python `validateRule`: `if not tier` — an absent tier OR 0 fails.
        if out.errule_tier.unwrap_or(0) == 0 {
            return Err(SzConfigError::InvalidInput(
                "A tier matching other rules that could be considered ambiguous to this one must be specified"
                    .to_string(),
            ));
        }
        if out.rtype_id != 1 {
            out.rtype_id = 1;
        }
    }
    if out.relate == "Yes" && ![2, 3, 4].contains(&out.rtype_id) {
        return Err(SzConfigError::InvalidInput(
            "Relationship type (RTYPE_ID) must be 2 (Possible match), 3 (Possibly related), or 4"
                .to_string(),
        ));
    }

    Ok(out)
}

/// Validate and normalise a proposed `CFG_ERRULE` row against the config.
///
/// This is the single, shared rule validator so that `add_rule` and `set_rule`
/// enforce exactly the same invariants and cannot drift apart. It performs, in
/// order:
///
/// 1. **Duplicate-code check** (only when `is_new`): rejects a rule whose
///    `ERRULE_CODE` already exists.
/// 2. **Fragment / disqualifier existence**: any non-empty `QUAL_ERFRAG_CODE`
///    or `DISQ_ERFRAG_CODE` must name an existing `CFG_ERFRAG` row.
/// 3. **RESOLVE / RELATE domain**: each must be `Yes`/`No` (case-insensitive);
///    the returned row is normalised to title-case.
/// 4. **Mutual exclusivity**: a rule may not both resolve and relate.
/// 5. **RTYPE_ID coherence**: `RESOLVE=Yes` auto-corrects `RTYPE_ID` to `1`;
///    `RELATE=Yes` requires `RTYPE_ID` in `[2, 3, 4]`.
///
/// On success it returns a normalised copy of the row (title-cased
/// RESOLVE/RELATE and any auto-corrected RTYPE_ID). It never mutates the config.
///
/// This validates the existence of **every** fragment/disqualifier code present
/// on the row, so it is the entry point used by `add_rule` (which validates the
/// whole new row). `set_rule` instead validates only the codes it is changing
/// via [`validate_fragment_code`] and composes [`validate_rule_row_core`] for the
/// remaining invariants, so a code carried over unchanged is never re-validated.
pub(crate) fn validate_rule_row(
    config: &Value,
    row: &ErruleRow,
    is_new: bool,
) -> Result<ErruleRow> {
    // Fragment / disqualifier existence for every code on the row.
    validate_fragment_code(config, row.qual_erfrag_code.as_deref(), "Fragment", true)?;
    validate_fragment_code(
        config,
        row.disq_erfrag_code.as_deref(),
        "Disqualifier",
        false,
    )?;

    // Remaining invariants (dup code, domains, exclusivity, RTYPE_ID coherence).
    validate_rule_row_core(config, row, is_new)
}

// ============================================================================
// Parameter Structs
// ============================================================================

/// Parameters for setting (updating) a rule.
///
/// `resolve`, `relate` and `rtype_id` are plain `Option`s (a `None` leaves the
/// stored value untouched). `fragment`, `disqualifier` and `tier` are tri-state
/// [`FieldUpdate`]s so an update can distinguish "leave unchanged" from
/// "clear to null": `Leave` carries the stored value forward, `Clear` writes
/// JSON `null`, and `Set` writes the new value.
#[derive(Debug, Clone)]
pub struct SetRuleParams<'a> {
    pub code: &'a str,
    pub resolve: Option<&'a str>,
    pub relate: Option<&'a str>,
    pub rtype_id: Option<i64>,
    pub fragment: FieldUpdate<&'a str>,
    pub disqualifier: FieldUpdate<&'a str>,
    pub tier: FieldUpdate<i64>,
}

impl<'a> TryFrom<&'a Value> for SetRuleParams<'a> {
    type Error = SzConfigError;

    fn try_from(json: &'a Value) -> Result<Self> {
        let code = json
            .get("code")
            .and_then(|v| v.as_str())
            .or_else(|| json.get("rule").and_then(|v| v.as_str()))
            .ok_or_else(|| SzConfigError::MissingField("code or rule".to_string()))?;

        Ok(Self {
            code,
            resolve: json
                .get("resolve")
                .and_then(|v| v.as_str())
                .or_else(|| json.get("RESOLVE").and_then(|v| v.as_str())),
            relate: json
                .get("relate")
                .and_then(|v| v.as_str())
                .or_else(|| json.get("RELATE").and_then(|v| v.as_str())),
            rtype_id: json
                .get("rtypeId")
                .and_then(|v| v.as_i64())
                .or_else(|| json.get("RTYPE_ID").and_then(|v| v.as_i64())),
            // Tri-state: an absent key -> Leave, an explicit JSON null -> Clear,
            // a value -> Set.
            fragment: helpers::field_update_str(json, &["fragment", "FRAGMENT"]),
            disqualifier: helpers::field_update_str(json, &["disqualifier", "DISQUALIFIER"]),
            tier: helpers::field_update_i64(json, &["tier", "TIER"]),
        })
    }
}

/// Add a new rule to the configuration
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
/// * `id` - Desired `ERRULE_ID`: `0` (or any non-positive value) auto-assigns
///   the next available id (seeded at 1000 for user rules); a positive value is
///   honoured, or rejected with `AlreadyExists` if already taken.
/// * `rule_config` - JSON configuration for the rule (must include ERRULE_CODE)
///
/// # Returns
///
/// Returns `(modified_config, new_rule_id)` on success, where `new_rule_id` is
/// the id actually assigned.
///
/// The proposed row is run through the shared rule validator, so `add_rule`
/// rejects a duplicate code, a missing or unknown fragment, an unknown
/// disqualifier, an invalid RESOLVE/RELATE value, a rule that both resolves and
/// relates, a `RESOLVE=Yes` rule without a (non-zero) tier, and an incoherent
/// RTYPE_ID — exactly as `set_rule` does.
///
/// # Example
///
/// ```
/// use sz_configtool_lib::rules;
/// use serde_json::json;
///
/// let config = r#"{"G2_CONFIG": {"CFG_ERRULE": [],
///     "CFG_ERFRAG": [{"ERFRAG_ID": 1, "ERFRAG_CODE": "MY_FRAGMENT"}]}}"#;
/// // A rule requires a fragment (matches Python's addRule); RESOLVE="Yes"
/// // additionally requires a non-zero tier.
/// let rule_config = json!({
///     "ERRULE_CODE": "CUSTOM_RULE",
///     "RESOLVE": "No",
///     "RELATE": "No",
///     "RTYPE_ID": 1,
///     "QUAL_ERFRAG_CODE": "MY_FRAGMENT"
/// });
/// // Pass 0 to auto-assign the next id, or a positive value for a specific id.
/// let (_modified, _rule_id) = rules::add_rule(config, 0, &rule_config).unwrap();
/// ```
pub fn add_rule(config_json: &str, id: i64, rule_config: &Value) -> Result<(String, i64)> {
    let code = rule_config
        .get("ERRULE_CODE")
        .and_then(|v| v.as_str())
        .ok_or_else(|| SzConfigError::MissingField("ERRULE_CODE".to_string()))?;

    let config_data: Value = serde_json::from_str(config_json)?;

    // Resolve the ERRULE_ID: id 0 (or non-positive) auto-assigns the next id
    // (seeded at 1000 for user-created rules, matching the other add_* paths);
    // a specific positive id is honoured, or rejected if already taken.
    let empty: Vec<Value> = Vec::new();
    let errule_array = config_data
        .get("G2_CONFIG")
        .and_then(|g| g.get("CFG_ERRULE"))
        .and_then(|v| v.as_array())
        .unwrap_or(&empty);
    let desired = if id > 0 { Some(id) } else { None };
    let assigned_id = helpers::get_desired_or_next_id(errule_array, "ERRULE_ID", desired, 1000)?;

    // Build a complete row via ErruleRow so every CFG_ERRULE key is present
    // (optional fields serialize as null) regardless of what the caller passed.
    let row = ErruleRow::from_config(assigned_id, code.to_uppercase(), rule_config);

    // Validate the whole new row through the shared validator (dup code,
    // fragment/disqualifier existence, RESOLVE/RELATE domain + exclusivity,
    // RTYPE_ID coherence). Returns the normalised row.
    let validated = validate_rule_row(&config_data, &row, true)?;
    let new_item = serde_json::to_value(&validated)?;

    // Add to config
    let modified_json = helpers::add_to_config_array(config_json, "CFG_ERRULE", new_item)?;

    Ok((modified_json, assigned_id))
}

/// Delete a rule from the configuration
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
/// * `rule_code` - Rule code to delete
///
/// # Returns
///
/// Returns modified configuration JSON on success
///
/// # Example
///
/// ```
/// use sz_configtool_lib::rules;
///
/// let config = r#"{"G2_CONFIG": {"CFG_ERRULE": [{"ERRULE_ID": 1, "ERRULE_CODE": "TEST"}]}}"#;
/// let modified = rules::delete_rule(config, "TEST").unwrap();
/// ```
pub fn delete_rule(config_json: &str, rule_code: &str) -> Result<String> {
    let rule_code = rule_code.to_uppercase();

    // Verify rule exists before deletion
    let _ = helpers::find_in_config_array(config_json, "CFG_ERRULE", "ERRULE_CODE", &rule_code)?
        .ok_or_else(|| SzConfigError::NotFound(format!("Rule not found: {rule_code}")))?;

    // Remove from config
    helpers::remove_from_config_array(config_json, "CFG_ERRULE", "ERRULE_CODE", &rule_code)
}

/// Get a rule by code or ID
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
/// * `code_or_id` - Rule code or ID to search for
///
/// # Returns
///
/// Returns the rule JSON object on success
///
/// # Example
///
/// ```
/// use sz_configtool_lib::rules;
///
/// let config = r#"{"G2_CONFIG": {"CFG_ERRULE": [{"ERRULE_ID": 1, "ERRULE_CODE": "TEST"}]}}"#;
/// let rule = rules::get_rule(config, "TEST").unwrap();
/// ```
pub fn get_rule(config_json: &str, code_or_id: &str) -> Result<Value> {
    let search_value = code_or_id.to_uppercase();

    // Try to find by CODE first, then by ID
    let item = if let Some(item) =
        helpers::find_in_config_array(config_json, "CFG_ERRULE", "ERRULE_CODE", &search_value)?
    {
        item
    } else if let Some(item) =
        helpers::find_in_config_array(config_json, "CFG_ERRULE", "ERRULE_ID", &search_value)?
    {
        item
    } else {
        return Err(SzConfigError::NotFound(format!(
            "Rule not found: {search_value}"
        )));
    };

    // Transform to lowercase format (matching list_rules for consistency).
    // Stored-nullable columns are projected null-preserving (stored null stays
    // null, stored "" stays "", absent -> null) via helpers::field_or_null. The
    // computed `tier` keeps its business rule: it is the stored ERRULE_TIER only
    // when RESOLVE == "Yes", otherwise null.
    let resolve_is_yes = item.get("RESOLVE").and_then(|v| v.as_str()) == Some("Yes");
    let tier = if resolve_is_yes {
        helpers::field_or_null(&item, "ERRULE_TIER")
    } else {
        Value::Null
    };

    Ok(json!({
        "id": helpers::field_or_null(&item, "ERRULE_ID"),
        "rule": helpers::field_or_null(&item, "ERRULE_CODE"),
        "resolve": helpers::field_or_null(&item, "RESOLVE"),
        "relate": helpers::field_or_null(&item, "RELATE"),
        "rtype_id": helpers::field_or_null(&item, "RTYPE_ID"),
        "fragment": helpers::field_or_null(&item, "QUAL_ERFRAG_CODE"),
        "disqualifier": helpers::field_or_null(&item, "DISQ_ERFRAG_CODE"),
        "tier": tier
    }))
}

/// List all rules in the configuration
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
///
/// The rows are sorted inside the SDK by `ERRULE_ID` ascending. This matches the
/// Python `sz_configtool` reference (`/opt/senzing/er/bin/sz_configtool`, 4.4.0),
/// whose `do_listRules` sorts with `key=lambda k: k["ERRULE_ID"]`.
///
/// # Returns
///
/// Returns a vector of rule objects in Python sz_configtool format, sorted by id
///
/// # Example
///
/// ```
/// use sz_configtool_lib::rules;
///
/// let config = r#"{"G2_CONFIG": {"CFG_ERRULE": [{"ERRULE_ID": 1, "ERRULE_CODE": "TEST", "RESOLVE": "Yes", "RELATE": "No", "RTYPE_ID": 1, "QUAL_ERFRAG_CODE": "", "DISQ_ERFRAG_CODE": "", "ERRULE_TIER": 10}]}}"#;
/// let rules = rules::list_rules(config).unwrap();
/// assert_eq!(rules.len(), 1);
/// ```
pub fn list_rules(config_json: &str) -> Result<Vec<Value>> {
    let config_data: Value = serde_json::from_str(config_json)?;

    // Extract rules and transform to Python format
    let items: Vec<Value> = if let Some(g2_config) = config_data.get("G2_CONFIG") {
        if let Some(array) = g2_config.get("CFG_ERRULE").and_then(|v| v.as_array()) {
            array
                .iter()
                .map(|item| {
                    // Null-preserving projection for every stored column; the
                    // computed `tier` keeps its business rule (stored ERRULE_TIER
                    // only when RESOLVE == "Yes", otherwise null).
                    let resolve_is_yes =
                        item.get("RESOLVE").and_then(|v| v.as_str()) == Some("Yes");
                    let tier = if resolve_is_yes {
                        helpers::field_or_null(item, "ERRULE_TIER")
                    } else {
                        Value::Null
                    };

                    json!({
                        "id": helpers::field_or_null(item, "ERRULE_ID"),
                        "rule": helpers::field_or_null(item, "ERRULE_CODE"),
                        "resolve": helpers::field_or_null(item, "RESOLVE"),
                        "relate": helpers::field_or_null(item, "RELATE"),
                        "rtype_id": helpers::field_or_null(item, "RTYPE_ID"),
                        "fragment": helpers::field_or_null(item, "QUAL_ERFRAG_CODE"),
                        "disqualifier": helpers::field_or_null(item, "DISQ_ERFRAG_CODE"),
                        "tier": tier
                    })
                })
                .collect()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    // SDK-owned default sort by ERRULE_ID ascending, matching Python
    // sz_configtool `do_listRules` (`key=lambda k: k["ERRULE_ID"]`). Rows
    // carrying a null/absent id sort first (id 0).
    let mut items = items;
    items.sort_by_key(|item| item.get("id").and_then(|v| v.as_i64()).unwrap_or(0));

    Ok(items)
}

/// Update an existing rule in the configuration
///
/// # Arguments
///
/// * `config_json` - Configuration JSON string
/// * `rule_code` - Rule code to update
/// * `rule_config` - New configuration for the rule
///
/// # Returns
///
/// Returns modified configuration JSON on success
///
/// # Example
///
/// ```
/// use sz_configtool_lib::rules;
///
/// use sz_configtool_lib::helpers::FieldUpdate;
///
/// let config = r#"{"G2_CONFIG": {"CFG_ERRULE": [{"ERRULE_ID": 1, "ERRULE_CODE": "TEST", "RESOLVE": "No"}], "CFG_ERFRAG": []}}"#;
/// let params = rules::SetRuleParams {
///     code: "TEST",
///     resolve: Some("Yes"),
///     relate: Some("No"),
///     rtype_id: None,
///     fragment: FieldUpdate::Leave,
///     disqualifier: FieldUpdate::Leave,
///     tier: FieldUpdate::Set(10), // RESOLVE="Yes" requires a non-zero tier
/// };
/// let modified = rules::set_rule(config, params).unwrap();
/// ```
pub fn set_rule(config_json: &str, params: SetRuleParams) -> Result<String> {
    let code = params.code.to_uppercase();

    let config_data: Value = serde_json::from_str(config_json)?;

    // Get existing rule to validate and merge updates
    let existing_rule =
        helpers::find_in_config_array(config_json, "CFG_ERRULE", "ERRULE_CODE", &code)?
            .ok_or_else(|| SzConfigError::NotFound(format!("Rule not found: {code}")))?;

    // Validate ONLY the fragment/disqualifier being Set. A code carried over
    // unchanged (Leave) or cleared (Clear) is never validated, preserving the
    // historical set_rule contract (do not newly reject previously-accepted
    // input). add_rule, by contrast, validates every code on the new row.
    if let FieldUpdate::Set(frag) = params.fragment {
        validate_fragment_code(&config_data, Some(frag), "Fragment", true)?;
    }
    if let FieldUpdate::Set(disq) = params.disqualifier {
        validate_fragment_code(&config_data, Some(disq), "Disqualifier", false)?;
    }

    // Extract ERRULE_ID from existing rule to preserve it
    let errule_id = existing_rule
        .get("ERRULE_ID")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);

    // Build the merged row from params (or the existing value where a field is
    // not being updated). RESOLVE/RELATE domain, exclusivity and RTYPE_ID
    // coherence are enforced and normalised by the shared validator core below,
    // so add_rule and set_rule cannot drift on those rules. Because ErruleRow
    // always serializes every key (None -> null), the resulting object can never
    // be missing a key the engine config loader requires.
    let row = ErruleRow {
        errule_id,
        errule_code: code.clone(),
        resolve: params
            .resolve
            .or_else(|| existing_rule.get("RESOLVE").and_then(|v| v.as_str()))
            .unwrap_or("No")
            .to_string(),
        relate: params
            .relate
            .or_else(|| existing_rule.get("RELATE").and_then(|v| v.as_str()))
            .unwrap_or("No")
            .to_string(),
        rtype_id: params
            .rtype_id
            .or_else(|| existing_rule.get("RTYPE_ID").and_then(|v| v.as_i64()))
            .unwrap_or(1),
        // Tri-state: Leave carries the stored value forward, Clear writes null,
        // Set writes the (upper-cased) new code.
        qual_erfrag_code: match params.fragment {
            FieldUpdate::Leave => helpers::field_as_string(&existing_rule, "QUAL_ERFRAG_CODE"),
            FieldUpdate::Clear => None,
            FieldUpdate::Set(frag) => Some(frag.to_uppercase()),
        },
        disq_erfrag_code: match params.disqualifier {
            FieldUpdate::Leave => helpers::field_as_string(&existing_rule, "DISQ_ERFRAG_CODE"),
            FieldUpdate::Clear => None,
            FieldUpdate::Set(disq) => Some(disq.to_uppercase()),
        },
        errule_tier: match params.tier {
            FieldUpdate::Leave => existing_rule.get("ERRULE_TIER").and_then(|v| v.as_i64()),
            FieldUpdate::Clear => None,
            FieldUpdate::Set(tier) => Some(tier),
        },
    };

    // Enforce/normalise the non-fragment invariants (is_new=false skips the
    // duplicate-code check, since an update targets an existing code).
    let validated = validate_rule_row_core(&config_data, &row, false)?;
    let updated_item = serde_json::to_value(&validated)?;

    // Update the item in the config
    helpers::update_in_config_array(
        config_json,
        "CFG_ERRULE",
        "ERRULE_CODE",
        &code,
        updated_item,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Regression: updating a rule must preserve all existing keys, including
    /// null-valued ones like DISQ_ERFRAG_CODE. The engine config loader rejects
    /// a CFG_ERRULE row missing that key (SENZ9117), so set_rule must never drop
    /// it when the disqualifier is not part of the update.
    #[test]
    fn test_set_rule_preserves_null_disqualifier() {
        let config = r#"{"G2_CONFIG": {"CFG_ERRULE": [
            {"ERRULE_ID": 100, "ERRULE_CODE": "SAME_A1", "RESOLVE": "Yes",
             "RELATE": "No", "RTYPE_ID": 1, "QUAL_ERFRAG_CODE": "SAME_A1",
             "DISQ_ERFRAG_CODE": null, "ERRULE_TIER": 10}
        ], "CFG_ERFRAG": []}}"#;

        let params = SetRuleParams {
            code: "SAME_A1",
            resolve: Some("No"),
            relate: None,
            rtype_id: None,
            fragment: FieldUpdate::Leave,
            disqualifier: FieldUpdate::Leave,
            tier: FieldUpdate::Leave,
        };

        let modified = set_rule(config, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let rule = &value["G2_CONFIG"]["CFG_ERRULE"][0];
        let obj = rule.as_object().unwrap();

        // Every CFG_ERRULE key must always be present, even when null.
        for key in [
            "ERRULE_ID",
            "ERRULE_CODE",
            "RESOLVE",
            "RELATE",
            "RTYPE_ID",
            "QUAL_ERFRAG_CODE",
            "DISQ_ERFRAG_CODE",
            "ERRULE_TIER",
        ] {
            assert!(obj.contains_key(key), "{key} key must be present");
        }

        // The updated field applied; unprovided fields preserved (incl. null).
        assert_eq!(rule["RESOLVE"], json!("No"));
        assert_eq!(rule["DISQ_ERFRAG_CODE"], Value::Null);
        assert_eq!(rule["RELATE"], json!("No"));
        assert_eq!(rule["QUAL_ERFRAG_CODE"], json!("SAME_A1"));
        assert_eq!(rule["ERRULE_TIER"], json!(10));
    }

    /// A brand-new-style update that provides no optional fields at all must
    /// still emit every key (as null where nothing exists), so the engine
    /// config loader never sees a missing CFG_ERRULE key.
    #[test]
    fn test_set_rule_emits_all_keys_when_optionals_absent() {
        let config = r#"{"G2_CONFIG": {"CFG_ERRULE": [
            {"ERRULE_ID": 42, "ERRULE_CODE": "MINIMAL", "RESOLVE": "No",
             "QUAL_ERFRAG_CODE": "F", "DISQ_ERFRAG_CODE": null, "ERRULE_TIER": null}
        ], "CFG_ERFRAG": [{"ERFRAG_ID": 1, "ERFRAG_CODE": "F"}]}}"#;

        let params = SetRuleParams {
            code: "MINIMAL",
            resolve: Some("No"),
            relate: None,
            rtype_id: None,
            fragment: FieldUpdate::Leave,
            disqualifier: FieldUpdate::Leave,
            tier: FieldUpdate::Leave,
        };

        let modified = set_rule(config, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let obj = value["G2_CONFIG"]["CFG_ERRULE"][0].as_object().unwrap();

        for key in [
            "ERRULE_ID",
            "ERRULE_CODE",
            "RESOLVE",
            "RELATE",
            "RTYPE_ID",
            "QUAL_ERFRAG_CODE",
            "DISQ_ERFRAG_CODE",
            "ERRULE_TIER",
        ] {
            assert!(obj.contains_key(key), "{key} key must be present");
        }
        // Fragment is required so it is present; the truly optional disqualifier
        // and tier were absent, so they surface as null.
        assert_eq!(obj["QUAL_ERFRAG_CODE"], json!("F"));
        assert_eq!(obj["DISQ_ERFRAG_CODE"], Value::Null);
        assert_eq!(obj["ERRULE_TIER"], Value::Null);
    }

    /// add_rule must write a complete row even when the caller supplies only a
    /// subset of fields — the omitted optionals become null, never dropped.
    #[test]
    fn test_add_rule_emits_all_keys() {
        let config = r#"{"G2_CONFIG": {"CFG_ERRULE": [],
            "CFG_ERFRAG": [{"ERFRAG_ID": 1, "ERFRAG_CODE": "MYFRAG"}]}}"#;
        let rule_config = json!({
            "ERRULE_CODE": "custom_rule",
            "RESOLVE": "No",
            "RELATE": "No",
            "RTYPE_ID": 1,
            "QUAL_ERFRAG_CODE": "MYFRAG"
        });

        let (modified, _id) = add_rule(config, 0, &rule_config).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let obj = value["G2_CONFIG"]["CFG_ERRULE"][0].as_object().unwrap();

        for key in [
            "ERRULE_ID",
            "ERRULE_CODE",
            "RESOLVE",
            "RELATE",
            "RTYPE_ID",
            "QUAL_ERFRAG_CODE",
            "DISQ_ERFRAG_CODE",
            "ERRULE_TIER",
        ] {
            assert!(obj.contains_key(key), "{key} key must be present");
        }
        assert_eq!(obj["ERRULE_CODE"], json!("CUSTOM_RULE"));
        assert_eq!(obj["QUAL_ERFRAG_CODE"], json!("MYFRAG"));
        assert_eq!(obj["DISQ_ERFRAG_CODE"], Value::Null);
        assert_eq!(obj["ERRULE_TIER"], Value::Null);
    }

    // ------------------------------------------------------------------
    // #33 null-preserving read projection
    // ------------------------------------------------------------------

    /// list_rules / get_rule must render a stored null as JSON null (not ""), a
    /// stored "" as "", and an absent column as null; the computed `tier` keeps
    /// its business rule (present only when RESOLVE == "Yes").
    #[test]
    fn test_list_rules_null_preserving_projection() {
        // Row 1: RESOLVE=Yes, DISQ null, QUAL "" (empty), tier present.
        // Row 2: RELATE=Yes, tier present but must be suppressed (RESOLVE != Yes).
        // Row 3: QUAL_ERFRAG_CODE absent entirely -> projects as null.
        let config = r#"{"G2_CONFIG": {"CFG_ERRULE": [
            {"ERRULE_ID": 1, "ERRULE_CODE": "A", "RESOLVE": "Yes", "RELATE": "No",
             "RTYPE_ID": 1, "QUAL_ERFRAG_CODE": "", "DISQ_ERFRAG_CODE": null,
             "ERRULE_TIER": 10},
            {"ERRULE_ID": 2, "ERRULE_CODE": "B", "RESOLVE": "No", "RELATE": "Yes",
             "RTYPE_ID": 2, "QUAL_ERFRAG_CODE": "Q", "DISQ_ERFRAG_CODE": "D",
             "ERRULE_TIER": 20},
            {"ERRULE_ID": 3, "ERRULE_CODE": "C", "RESOLVE": "No", "RELATE": "No",
             "RTYPE_ID": 1, "DISQ_ERFRAG_CODE": null}
        ]}}"#;

        let rules = list_rules(config).unwrap();

        // Row 1: stored null stays null, stored "" stays "", tier computed.
        assert_eq!(rules[0]["disqualifier"], Value::Null);
        assert_eq!(rules[0]["fragment"], json!(""));
        assert_eq!(rules[0]["tier"], json!(10));

        // Row 2: RESOLVE != "Yes" -> tier suppressed to null despite stored 20.
        assert_eq!(rules[1]["tier"], Value::Null);
        assert_eq!(rules[1]["fragment"], json!("Q"));

        // Row 3: absent QUAL_ERFRAG_CODE -> null (present as a key).
        assert!(rules[2].as_object().unwrap().contains_key("fragment"));
        assert_eq!(rules[2]["fragment"], Value::Null);

        // get_rule agrees with list_rules for row 1.
        let one = get_rule(config, "A").unwrap();
        assert_eq!(one["disqualifier"], Value::Null);
        assert_eq!(one["fragment"], json!(""));
        assert_eq!(one["tier"], json!(10));
    }

    // ------------------------------------------------------------------
    // validate_rule_row (shared validator, created in Wave 1)
    // ------------------------------------------------------------------

    fn validator_config() -> Value {
        serde_json::from_str(
            r#"{"G2_CONFIG": {
                "CFG_ERRULE": [
                    {"ERRULE_ID": 100, "ERRULE_CODE": "EXISTING", "RESOLVE": "Yes",
                     "RELATE": "No", "RTYPE_ID": 1, "QUAL_ERFRAG_CODE": "FRAG_A",
                     "DISQ_ERFRAG_CODE": null, "ERRULE_TIER": 10}
                ],
                "CFG_ERFRAG": [
                    {"ERFRAG_ID": 1, "ERFRAG_CODE": "FRAG_A"},
                    {"ERFRAG_ID": 2, "ERFRAG_CODE": "FRAG_B"}
                ]
            }}"#,
        )
        .unwrap()
    }

    fn base_row() -> ErruleRow {
        // A valid baseline: a required fragment (#45) is present and, so
        // RESOLVE=Yes cases pass the tier check, a tier is set. Tests that
        // exercise the fragment/tier rules override these explicitly.
        ErruleRow {
            errule_id: 1000,
            errule_code: "NEW_RULE".to_string(),
            resolve: "No".to_string(),
            relate: "No".to_string(),
            rtype_id: 1,
            qual_erfrag_code: Some("FRAG_A".to_string()),
            disq_erfrag_code: None,
            errule_tier: Some(10),
        }
    }

    #[test]
    fn test_validate_rule_row_accepts_valid_new() {
        let cfg = validator_config();
        let out = validate_rule_row(&cfg, &base_row(), true).unwrap();
        assert_eq!(out.resolve, "No");
        assert_eq!(out.relate, "No");
    }

    #[test]
    fn test_validate_rule_row_rejects_duplicate_code_when_new() {
        let cfg = validator_config();
        let mut row = base_row();
        row.errule_code = "existing".to_string(); // case-insensitive clash
        let err = validate_rule_row(&cfg, &row, true).unwrap_err();
        assert_eq!(err.kind(), crate::error::SzErrorKind::AlreadyExists);
    }

    #[test]
    fn test_validate_rule_row_allows_existing_code_when_not_new() {
        let cfg = validator_config();
        let mut row = base_row();
        row.errule_code = "EXISTING".to_string();
        // is_new = false -> dup check skipped.
        assert!(validate_rule_row(&cfg, &row, false).is_ok());
    }

    #[test]
    fn test_validate_rule_row_bad_fragment() {
        let cfg = validator_config();
        let mut row = base_row();
        row.qual_erfrag_code = Some("NOPE".to_string());
        let err = validate_rule_row(&cfg, &row, true).unwrap_err();
        assert_eq!(err.kind(), crate::error::SzErrorKind::NotFound);
        // Double-quoted wording (D13, Python parity).
        assert!(err.to_string().contains("Fragment \"NOPE\" not found"));
    }

    #[test]
    fn test_validate_rule_row_bad_disqualifier() {
        let cfg = validator_config();
        let mut row = base_row();
        row.disq_erfrag_code = Some("NOPE".to_string());
        let err = validate_rule_row(&cfg, &row, true).unwrap_err();
        assert_eq!(err.kind(), crate::error::SzErrorKind::NotFound);
        // Disqualifier uses its own label, double-quoted.
        assert!(err.to_string().contains("Disqualifier \"NOPE\" not found"));
    }

    #[test]
    fn test_validate_rule_row_existing_fragment_ok() {
        let cfg = validator_config();
        let mut row = base_row();
        row.qual_erfrag_code = Some("frag_b".to_string()); // case-insensitive
        assert!(validate_rule_row(&cfg, &row, true).is_ok());
    }

    #[test]
    fn test_validate_rule_row_resolve_domain() {
        let cfg = validator_config();
        let mut row = base_row();
        row.resolve = "maybe".to_string();
        let err = validate_rule_row(&cfg, &row, true).unwrap_err();
        assert_eq!(err.kind(), crate::error::SzErrorKind::InvalidInput);
    }

    #[test]
    fn test_validate_rule_row_relate_domain() {
        let cfg = validator_config();
        let mut row = base_row();
        row.relate = "perhaps".to_string();
        assert!(validate_rule_row(&cfg, &row, true).is_err());
    }

    #[test]
    fn test_validate_rule_row_mutual_exclusivity() {
        let cfg = validator_config();
        let mut row = base_row();
        row.resolve = "Yes".to_string();
        row.relate = "Yes".to_string();
        let err = validate_rule_row(&cfg, &row, true).unwrap_err();
        assert!(err.to_string().contains("either resolve or relate"));
    }

    #[test]
    fn test_validate_rule_row_resolve_autocorrects_rtype() {
        let cfg = validator_config();
        let mut row = base_row();
        row.resolve = "yes".to_string();
        row.rtype_id = 3; // incoherent with RESOLVE=Yes
        let out = validate_rule_row(&cfg, &row, true).unwrap();
        assert_eq!(out.resolve, "Yes"); // normalised
        assert_eq!(out.rtype_id, 1); // auto-corrected
    }

    #[test]
    fn test_validate_rule_row_relate_requires_valid_rtype() {
        let cfg = validator_config();
        let mut row = base_row();
        row.relate = "Yes".to_string();
        row.rtype_id = 1; // invalid for RELATE
        assert!(validate_rule_row(&cfg, &row, true).is_err());

        row.rtype_id = 2; // valid
        assert!(validate_rule_row(&cfg, &row, true).is_ok());
    }

    #[test]
    fn test_validate_rule_row_relate_accepts_rtype_4() {
        // The RELATE domain is [2, 3, 4]; 4 must be accepted (reconciled wording).
        let cfg = validator_config();
        let mut row = base_row();
        row.relate = "Yes".to_string();
        row.rtype_id = 4;
        assert!(validate_rule_row(&cfg, &row, true).is_ok());
    }

    // ------------------------------------------------------------------
    // add_rule / set_rule wiring + parity (#39, Wave 2c)
    // ------------------------------------------------------------------

    fn add_rule_config() -> &'static str {
        r#"{"G2_CONFIG": {
            "CFG_ERRULE": [
                {"ERRULE_ID": 100, "ERRULE_CODE": "EXISTING", "RESOLVE": "Yes",
                 "RELATE": "No", "RTYPE_ID": 1, "QUAL_ERFRAG_CODE": "FRAG_A",
                 "DISQ_ERFRAG_CODE": null, "ERRULE_TIER": 10}
            ],
            "CFG_ERFRAG": [
                {"ERFRAG_ID": 1, "ERFRAG_CODE": "FRAG_A"},
                {"ERFRAG_ID": 2, "ERFRAG_CODE": "FRAG_B"}
            ]
        }}"#
    }

    #[test]
    fn test_add_rule_rejects_duplicate_code() {
        let cfg = add_rule_config();
        let rule = json!({"ERRULE_CODE": "existing", "RESOLVE": "No", "RELATE": "No",
            "QUAL_ERFRAG_CODE": "FRAG_A"});
        let err = add_rule(cfg, 0, &rule).unwrap_err();
        assert_eq!(err.kind(), crate::error::SzErrorKind::AlreadyExists);
    }

    #[test]
    fn test_add_rule_rejects_bad_fragment() {
        let cfg = add_rule_config();
        let rule = json!({"ERRULE_CODE": "NEW", "RESOLVE": "No", "RELATE": "No",
            "QUAL_ERFRAG_CODE": "NOPE"});
        let err = add_rule(cfg, 0, &rule).unwrap_err();
        assert_eq!(err.kind(), crate::error::SzErrorKind::NotFound);
        assert!(err.to_string().contains("Fragment \"NOPE\" not found"));
    }

    #[test]
    fn test_add_rule_rejects_bad_disqualifier() {
        let cfg = add_rule_config();
        let rule = json!({"ERRULE_CODE": "NEW", "RESOLVE": "No", "RELATE": "No",
            "QUAL_ERFRAG_CODE": "FRAG_A", "DISQ_ERFRAG_CODE": "NOPE"});
        let err = add_rule(cfg, 0, &rule).unwrap_err();
        assert_eq!(err.kind(), crate::error::SzErrorKind::NotFound);
        assert!(err.to_string().contains("Disqualifier \"NOPE\" not found"));
    }

    // #45: a blank fragment code is a required-reference error (matches Python's
    // unconditional lookupFragment("")), while a blank disqualifier is accepted
    // (nullable). Regression guard for the v0.6.0 empty-skip.
    #[test]
    fn test_add_rule_rejects_blank_fragment() {
        let cfg = add_rule_config();
        let rule = json!({"ERRULE_CODE": "NEW", "RESOLVE": "No", "RELATE": "No",
            "QUAL_ERFRAG_CODE": ""});
        let err = add_rule(cfg, 0, &rule).unwrap_err();
        assert_eq!(err.kind(), crate::error::SzErrorKind::NotFound);
        assert!(
            err.to_string().contains("Fragment \"\" not found"),
            "got: {err}"
        );
    }

    // add_rule requires a fragment (Python `do_addRule` lists FRAGMENT as a
    // required param). An absent fragment is a MissingField error.
    #[test]
    fn test_add_rule_rejects_missing_fragment() {
        let cfg = add_rule_config();
        let rule = json!({"ERRULE_CODE": "NEW", "RESOLVE": "No", "RELATE": "No"});
        let err = add_rule(cfg, 0, &rule).unwrap_err();
        assert_eq!(err.kind(), crate::error::SzErrorKind::MissingField);
        assert!(
            err.to_string().contains("Fragment is required"),
            "got: {err}"
        );
    }

    // RESOLVE=Yes requires a non-zero tier (Python `validateRule`: `if not tier`,
    // so both an absent tier and 0 fail).
    #[test]
    fn test_rule_resolve_yes_requires_nonzero_tier() {
        let cfg = add_rule_config();
        let base = |tier: Option<i64>| {
            let mut r = json!({"ERRULE_CODE": "NEW", "RESOLVE": "Yes", "RELATE": "No",
                "RTYPE_ID": 1, "QUAL_ERFRAG_CODE": "FRAG_A"});
            if let Some(t) = tier {
                r["ERRULE_TIER"] = json!(t);
            }
            r
        };
        // Absent tier -> rejected.
        let err = add_rule(cfg, 0, &base(None)).unwrap_err();
        assert_eq!(err.kind(), crate::error::SzErrorKind::InvalidInput);
        assert!(err.to_string().contains("tier"), "got: {err}");
        // Tier 0 -> also rejected.
        assert_eq!(
            add_rule(cfg, 0, &base(Some(0))).unwrap_err().kind(),
            crate::error::SzErrorKind::InvalidInput
        );
        // Non-zero tier -> accepted.
        assert!(add_rule(cfg, 0, &base(Some(5))).is_ok());
    }

    #[test]
    fn test_set_rule_rejects_blank_fragment() {
        let config = r#"{"G2_CONFIG": {"CFG_ERRULE": [
            {"ERRULE_ID": 100, "ERRULE_CODE": "R", "RESOLVE": "No", "RELATE": "No",
             "RTYPE_ID": 0, "QUAL_ERFRAG_CODE": "F", "DISQ_ERFRAG_CODE": null,
             "ERRULE_TIER": null}
        ], "CFG_ERFRAG": [{"ERFRAG_ID": 1, "ERFRAG_CODE": "F"}]}}"#;
        let params = SetRuleParams {
            code: "R",
            resolve: None,
            relate: None,
            rtype_id: None,
            fragment: FieldUpdate::Set(""),
            disqualifier: FieldUpdate::Leave,
            tier: FieldUpdate::Leave,
        };
        let err = set_rule(config, params).unwrap_err();
        assert_eq!(err.kind(), crate::error::SzErrorKind::NotFound);
        assert!(
            err.to_string().contains("Fragment \"\" not found"),
            "got: {err}"
        );
    }

    #[test]
    fn test_set_rule_accepts_blank_disqualifier() {
        // A blank disqualifier is a nullable reference and must NOT be rejected
        // (Python guards its lookup with `if record.get("DISQ_ERFRAG_CODE"):`).
        let config = r#"{"G2_CONFIG": {"CFG_ERRULE": [
            {"ERRULE_ID": 100, "ERRULE_CODE": "R", "RESOLVE": "No", "RELATE": "No",
             "RTYPE_ID": 0, "QUAL_ERFRAG_CODE": "F", "DISQ_ERFRAG_CODE": "D",
             "ERRULE_TIER": null}
        ], "CFG_ERFRAG": [{"ERFRAG_ID": 1, "ERFRAG_CODE": "F"},
                          {"ERFRAG_ID": 2, "ERFRAG_CODE": "D"}]}}"#;
        let params = SetRuleParams {
            code: "R",
            resolve: None,
            relate: None,
            rtype_id: None,
            fragment: FieldUpdate::Leave,
            disqualifier: FieldUpdate::Set(""),
            tier: FieldUpdate::Leave,
        };
        let modified = set_rule(config, params).expect("blank disqualifier accepted");
        let value: Value = serde_json::from_str(&modified).unwrap();
        assert_eq!(
            value["G2_CONFIG"]["CFG_ERRULE"][0]["DISQ_ERFRAG_CODE"],
            json!("")
        );
    }

    #[test]
    fn test_add_rule_rejects_resolve_and_relate_both_yes() {
        let cfg = add_rule_config();
        let rule = json!({"ERRULE_CODE": "NEW", "RESOLVE": "Yes", "RELATE": "Yes",
            "RTYPE_ID": 2, "QUAL_ERFRAG_CODE": "FRAG_A"});
        let err = add_rule(cfg, 0, &rule).unwrap_err();
        assert_eq!(err.kind(), crate::error::SzErrorKind::InvalidInput);
        assert!(err.to_string().contains("either resolve or relate"));
    }

    #[test]
    fn test_add_rule_rejects_incoherent_rtype_for_relate() {
        let cfg = add_rule_config();
        // RELATE=Yes with RTYPE_ID=1 is incoherent.
        let rule = json!({"ERRULE_CODE": "NEW", "RESOLVE": "No", "RELATE": "Yes",
            "RTYPE_ID": 1, "QUAL_ERFRAG_CODE": "FRAG_A"});
        let err = add_rule(cfg, 0, &rule).unwrap_err();
        assert_eq!(err.kind(), crate::error::SzErrorKind::InvalidInput);
    }

    /// Drift guard: add_rule and set_rule must reject the same invalid row. Here
    /// a bad *new* fragment is supplied to both entry points.
    #[test]
    fn test_add_and_set_rule_reject_same_invalid_row() {
        let cfg = add_rule_config();

        // add_rule: new rule naming a non-existent fragment.
        let add_row = json!({"ERRULE_CODE": "NEW", "RESOLVE": "No", "RELATE": "No",
            "QUAL_ERFRAG_CODE": "GHOST"});
        let add_err = add_rule(cfg, 0, &add_row).unwrap_err();

        // set_rule: existing rule updated to name the same non-existent fragment.
        let set_params = SetRuleParams {
            code: "EXISTING",
            resolve: None,
            relate: None,
            rtype_id: None,
            fragment: FieldUpdate::Set("GHOST"),
            disqualifier: FieldUpdate::Leave,
            tier: FieldUpdate::Leave,
        };
        let set_err = set_rule(cfg, set_params).unwrap_err();

        assert_eq!(add_err.kind(), crate::error::SzErrorKind::NotFound);
        assert_eq!(set_err.kind(), crate::error::SzErrorKind::NotFound);
        assert_eq!(add_err.to_string(), set_err.to_string());
    }

    /// Regression: set_rule must NOT re-validate a fragment/disqualifier carried
    /// over unchanged. Here the existing rule points at a fragment that has since
    /// been removed from CFG_ERFRAG; a no-fragment update must still succeed.
    #[test]
    fn test_set_rule_does_not_revalidate_carried_over_fragment() {
        let config = r#"{"G2_CONFIG": {
            "CFG_ERRULE": [
                {"ERRULE_ID": 100, "ERRULE_CODE": "ORPHAN", "RESOLVE": "Yes",
                 "RELATE": "No", "RTYPE_ID": 1, "QUAL_ERFRAG_CODE": "GONE",
                 "DISQ_ERFRAG_CODE": "ALSO_GONE", "ERRULE_TIER": 10}
            ],
            "CFG_ERFRAG": []
        }}"#;

        // Change only RESOLVE; fragment/disqualifier are carried over unchanged
        // and must not be re-validated (they no longer exist).
        let params = SetRuleParams {
            code: "ORPHAN",
            resolve: Some("No"),
            relate: None,
            rtype_id: None,
            fragment: FieldUpdate::Leave,
            disqualifier: FieldUpdate::Leave,
            tier: FieldUpdate::Leave,
        };
        let modified = set_rule(config, params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let rule = &value["G2_CONFIG"]["CFG_ERRULE"][0];
        assert_eq!(rule["RESOLVE"], json!("No"));
        assert_eq!(rule["QUAL_ERFRAG_CODE"], json!("GONE"));
        assert_eq!(rule["DISQ_ERFRAG_CODE"], json!("ALSO_GONE"));
    }

    #[test]
    fn test_add_rule_auto_assigns_id() {
        // Empty rule table -> first user id seeds at 1000.
        let config = r#"{"G2_CONFIG": {"CFG_ERRULE": [],
            "CFG_ERFRAG": [{"ERFRAG_ID": 1, "ERFRAG_CODE": "F"}]}}"#;
        let rule = json!({"ERRULE_CODE": "R1", "RESOLVE": "No", "RELATE": "No",
            "QUAL_ERFRAG_CODE": "F"});
        let (modified, id) = add_rule(config, 0, &rule).unwrap();
        assert_eq!(id, 1000);
        let value: Value = serde_json::from_str(&modified).unwrap();
        assert_eq!(
            value["G2_CONFIG"]["CFG_ERRULE"][0]["ERRULE_ID"],
            json!(1000)
        );

        // A specific positive id is honoured.
        let rule2 = json!({"ERRULE_CODE": "R2", "RESOLVE": "No", "RELATE": "No",
            "QUAL_ERFRAG_CODE": "F"});
        let (_m2, id2) = add_rule(&modified, 2000, &rule2).unwrap();
        assert_eq!(id2, 2000);
    }

    /// Tri-state on set_rule: Clear writes an explicit null, Set writes a value
    /// (validating existence), and Leave preserves the stored value.
    #[test]
    fn test_set_rule_disqualifier_tri_state() {
        let cfg = add_rule_config();

        // Clear: existing disqualifier (null already) stays null; and clearing a
        // populated fragment writes null.
        let clear_params = SetRuleParams {
            code: "EXISTING",
            resolve: None,
            relate: None,
            rtype_id: None,
            fragment: FieldUpdate::Clear,
            disqualifier: FieldUpdate::Leave,
            tier: FieldUpdate::Leave,
        };
        let modified = set_rule(cfg, clear_params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let rule = &value["G2_CONFIG"]["CFG_ERRULE"][0];
        assert_eq!(rule["QUAL_ERFRAG_CODE"], Value::Null);

        // Set: point the disqualifier at an existing fragment.
        let set_params = SetRuleParams {
            code: "EXISTING",
            resolve: None,
            relate: None,
            rtype_id: None,
            fragment: FieldUpdate::Leave,
            disqualifier: FieldUpdate::Set("frag_b"),
            tier: FieldUpdate::Leave,
        };
        let modified = set_rule(cfg, set_params).unwrap();
        let value: Value = serde_json::from_str(&modified).unwrap();
        let rule = &value["G2_CONFIG"]["CFG_ERRULE"][0];
        assert_eq!(rule["DISQ_ERFRAG_CODE"], json!("FRAG_B"));
        // The carried-over fragment (Leave) is untouched.
        assert_eq!(rule["QUAL_ERFRAG_CODE"], json!("FRAG_A"));
    }

    #[test]
    fn test_add_rule_rejects_taken_id() {
        let cfg = add_rule_config();
        let rule = json!({"ERRULE_CODE": "NEW", "RESOLVE": "No", "RELATE": "No"});
        // id 100 is already taken by EXISTING.
        let err = add_rule(cfg, 100, &rule).unwrap_err();
        assert_eq!(err.kind(), crate::error::SzErrorKind::AlreadyExists);
    }

    #[test]
    fn test_list_rules_default_sort_by_id() {
        // Stored order deliberately out of ERRULE_ID order.
        let config = r#"{"G2_CONFIG": {"CFG_ERRULE": [
            {"ERRULE_ID": 30, "ERRULE_CODE": "C", "RESOLVE": "No"},
            {"ERRULE_ID": 10, "ERRULE_CODE": "A", "RESOLVE": "No"},
            {"ERRULE_ID": 20, "ERRULE_CODE": "B", "RESOLVE": "No"}
        ]}}"#;

        let rules = list_rules(config).unwrap();
        // SDK-owned default sort: ERRULE_ID ascending.
        assert_eq!(rules[0]["id"], json!(10));
        assert_eq!(rules[1]["id"], json!(20));
        assert_eq!(rules[2]["id"], json!(30));
    }
}
