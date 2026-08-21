//! Shared config-row structs.
//!
//! One `pub(crate)` serde struct per CFG_* section that is written by more than
//! one builder module. Centralizing them here guarantees a single source of
//! truth for each section's on-disk shape — previously several sections had two
//! independent struct definitions (one in `features.rs`, one in a `calls/*` or
//! `elements` module) that could drift apart.
//!
//! Every struct derives `Serialize` with **no** `skip_serializing_if`, so every
//! key is always emitted — optional/nullable fields serialize as JSON `null`
//! rather than being dropped. The Senzing engine's config loader requires every
//! key of every row to be present (a missing key yields SENZ9117), so partial
//! rows must never be written.
//!
//! The key set of each struct is exactly the authoritative Senzing v4 column set
//! for that section (from `config/engine/*.data`) — no more, no less.

use serde::Serialize;

/// Complete CFG_FTYPE row (feature type).
#[derive(Debug, Clone, Serialize)]
pub(crate) struct FtypeRow {
    #[serde(rename = "FTYPE_ID")]
    pub(crate) ftype_id: i64,
    #[serde(rename = "FTYPE_CODE")]
    pub(crate) ftype_code: String,
    #[serde(rename = "FTYPE_DESC")]
    pub(crate) ftype_desc: String,
    #[serde(rename = "FCLASS_ID")]
    pub(crate) fclass_id: i64,
    #[serde(rename = "FTYPE_FREQ")]
    pub(crate) ftype_freq: String,
    #[serde(rename = "FTYPE_EXCL")]
    pub(crate) ftype_excl: String,
    #[serde(rename = "FTYPE_STAB")]
    pub(crate) ftype_stab: String,
    #[serde(rename = "ANONYMIZE")]
    pub(crate) anonymize: String,
    #[serde(rename = "DERIVED")]
    pub(crate) derived: String,
    #[serde(rename = "USED_FOR_CAND")]
    pub(crate) used_for_cand: String,
    #[serde(rename = "SHOW_IN_MATCH_KEY")]
    pub(crate) show_in_match_key: String,
    #[serde(rename = "PERSIST_HISTORY")]
    pub(crate) persist_history: String,
    #[serde(rename = "VERSION")]
    pub(crate) version: i64,
    #[serde(rename = "RTYPE_ID")]
    pub(crate) rtype_id: i64,
}

/// Complete CFG_FELEM row (feature element).
///
/// Authoritative columns: `FELEM_ID, FELEM_CODE, FELEM_DESC, DATA_TYPE`. There is
/// no `TOKENIZE`/`TOKENIZED` column in the Senzing v4 schema.
#[derive(Debug, Clone, Serialize)]
pub(crate) struct FelemRow {
    #[serde(rename = "FELEM_ID")]
    pub(crate) felem_id: i64,
    #[serde(rename = "FELEM_CODE")]
    pub(crate) felem_code: String,
    #[serde(rename = "FELEM_DESC")]
    pub(crate) felem_desc: String,
    #[serde(rename = "DATA_TYPE")]
    pub(crate) data_type: String,
}

/// Complete CFG_SFCALL row (standardize call).
///
/// `EXEC_ORDER` is `Option<i64>` because `add_standardize_call_element` uses the
/// seed-then-null pattern (null when not supplied); `add_standardize_call` and
/// `add_feature` always supply a concrete value.
#[derive(Debug, Clone, Serialize)]
pub(crate) struct SfcallRow {
    #[serde(rename = "SFCALL_ID")]
    pub(crate) sfcall_id: i64,
    #[serde(rename = "FTYPE_ID")]
    pub(crate) ftype_id: i64,
    #[serde(rename = "FELEM_ID")]
    pub(crate) felem_id: i64,
    #[serde(rename = "SFUNC_ID")]
    pub(crate) sfunc_id: i64,
    #[serde(rename = "EXEC_ORDER")]
    pub(crate) exec_order: Option<i64>,
}

/// Complete CFG_EFCALL row (expression call).
#[derive(Debug, Clone, Serialize)]
pub(crate) struct EfcallRow {
    #[serde(rename = "EFCALL_ID")]
    pub(crate) efcall_id: i64,
    #[serde(rename = "FTYPE_ID")]
    pub(crate) ftype_id: i64,
    #[serde(rename = "FELEM_ID")]
    pub(crate) felem_id: i64,
    #[serde(rename = "EFUNC_ID")]
    pub(crate) efunc_id: i64,
    #[serde(rename = "EXEC_ORDER")]
    pub(crate) exec_order: i64,
    #[serde(rename = "EFEAT_FTYPE_ID")]
    pub(crate) efeat_ftype_id: i64,
    #[serde(rename = "IS_VIRTUAL")]
    pub(crate) is_virtual: String,
}

/// Complete CFG_CFCALL row (comparison call).
///
/// Authoritative columns: `CFCALL_ID, FTYPE_ID, CFUNC_ID` — the schema has no
/// `EXEC_ORDER` column for this section.
#[derive(Debug, Clone, Serialize)]
pub(crate) struct CfcallRow {
    #[serde(rename = "CFCALL_ID")]
    pub(crate) cfcall_id: i64,
    #[serde(rename = "FTYPE_ID")]
    pub(crate) ftype_id: i64,
    #[serde(rename = "CFUNC_ID")]
    pub(crate) cfunc_id: i64,
}

/// Complete CFG_EFBOM row (expression bill-of-materials entry).
#[derive(Debug, Clone, Serialize)]
pub(crate) struct EfbomRow {
    #[serde(rename = "EFCALL_ID")]
    pub(crate) efcall_id: i64,
    #[serde(rename = "FTYPE_ID")]
    pub(crate) ftype_id: i64,
    #[serde(rename = "FELEM_ID")]
    pub(crate) felem_id: i64,
    #[serde(rename = "EXEC_ORDER")]
    pub(crate) exec_order: i64,
    #[serde(rename = "FELEM_REQ")]
    pub(crate) felem_req: String,
}

/// Complete CFG_CFBOM row (comparison bill-of-materials entry).
#[derive(Debug, Clone, Serialize)]
pub(crate) struct CfbomRow {
    #[serde(rename = "CFCALL_ID")]
    pub(crate) cfcall_id: i64,
    #[serde(rename = "FTYPE_ID")]
    pub(crate) ftype_id: i64,
    #[serde(rename = "FELEM_ID")]
    pub(crate) felem_id: i64,
    #[serde(rename = "EXEC_ORDER")]
    pub(crate) exec_order: i64,
}

/// Complete CFG_DFCALL row (distinct call).
///
/// Authoritative columns: `DFCALL_ID, FTYPE_ID, DFUNC_ID` — no `FELEM_ID` and no
/// `EXEC_ORDER` on the header row (those belong to CFG_DFBOM).
#[derive(Debug, Clone, Serialize)]
pub(crate) struct DfcallRow {
    #[serde(rename = "DFCALL_ID")]
    pub(crate) dfcall_id: i64,
    #[serde(rename = "FTYPE_ID")]
    pub(crate) ftype_id: i64,
    #[serde(rename = "DFUNC_ID")]
    pub(crate) dfunc_id: i64,
}

/// Complete CFG_DFBOM row (distinct bill-of-materials entry).
#[derive(Debug, Clone, Serialize)]
pub(crate) struct DfbomRow {
    #[serde(rename = "DFCALL_ID")]
    pub(crate) dfcall_id: i64,
    #[serde(rename = "FTYPE_ID")]
    pub(crate) ftype_id: i64,
    #[serde(rename = "FELEM_ID")]
    pub(crate) felem_id: i64,
    #[serde(rename = "EXEC_ORDER")]
    pub(crate) exec_order: i64,
}

/// Complete CFG_FBOM row (feature bill-of-materials entry).
///
/// Shared by `add_feature` (which fills EXEC_ORDER/DISPLAY_LEVEL/DERIVED with
/// concrete values and only DISPLAY_DELIM may be null) and
/// `add_feature_comparison` (which may leave any of them null via seed-then-null).
/// Nullable fields are `Option` so both callers keep every key present.
#[derive(Debug, Clone, Serialize)]
pub(crate) struct FbomRow {
    #[serde(rename = "FTYPE_ID")]
    pub(crate) ftype_id: i64,
    #[serde(rename = "FELEM_ID")]
    pub(crate) felem_id: i64,
    #[serde(rename = "EXEC_ORDER")]
    pub(crate) exec_order: Option<i64>,
    #[serde(rename = "DISPLAY_LEVEL")]
    pub(crate) display_level: Option<i64>,
    #[serde(rename = "DISPLAY_DELIM")]
    pub(crate) display_delim: Option<String>,
    #[serde(rename = "DERIVED")]
    pub(crate) derived: Option<String>,
}
