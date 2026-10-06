//! Command script processor for Senzing .gtc files
//!
//! Processes line-based command scripts to transform Senzing configuration JSON.
//! Use cases include configuration upgrades, batch changes, templates, and
//! automated testing.
//!
//! # Format
//!
//! Commands follow the pattern:
//! ```text
//! commandName {"param": "value", ...}
//! commandName {"param": "value"}
//!
//! save
//! ```
//!
//! # Example
//!
//! ```no_run
//! use sz_configtool_lib::command_processor::CommandProcessor;
//!
//! let config = std::fs::read_to_string("g2config.json")?;
//! let mut processor = CommandProcessor::new(config);
//!
//! let upgraded = processor.process_file("upgrade-10-to-11.gtc")?;
//! std::fs::write("g2config_v11.json", upgraded)?;
//!
//! println!("{}", processor.summary());
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use crate::error::{Result, SzConfigError};
use serde_json::Value;
use std::fs;
use std::path::Path;

/// Processes Senzing command scripts (.gtc files)
pub struct CommandProcessor {
    config: String,
    commands_executed: Vec<String>,
    dry_run: bool,
}

impl CommandProcessor {
    /// Create a new processor with initial configuration
    ///
    /// # Arguments
    /// * `config_json` - Initial configuration JSON string
    pub fn new(config_json: String) -> Self {
        Self {
            config: config_json,
            commands_executed: Vec::new(),
            dry_run: false,
        }
    }

    /// Enable or disable dry-run mode
    ///
    /// In dry-run mode, commands are validated but not applied to the config.
    ///
    /// # Arguments
    /// * `enabled` - true to enable dry-run mode
    pub fn dry_run(mut self, enabled: bool) -> Self {
        self.dry_run = enabled;
        self
    }

    /// Process a command script from a file
    ///
    /// # Arguments
    /// * `path` - Path to .gtc script file
    ///
    /// # Returns
    /// Modified configuration JSON string
    pub fn process_file<P: AsRef<Path>>(&mut self, path: P) -> Result<String> {
        let content = fs::read_to_string(path.as_ref()).map_err(|e| {
            SzConfigError::InvalidConfig(format!("Failed to read script file: {e}"))
        })?;
        self.process_script(&content)
    }

    /// Process a command script from a string
    ///
    /// # Arguments
    /// * `script` - Script content with line-based commands
    ///
    /// # Returns
    /// Modified configuration JSON string
    pub fn process_script(&mut self, script: &str) -> Result<String> {
        for (line_num, line) in script.lines().enumerate() {
            let trimmed = line.trim();

            // Skip blank lines and comments
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            // Process command
            if let Err(e) = self.process_command(trimmed) {
                return Err(SzConfigError::InvalidConfig(format!(
                    "Line {}: {} - Error: {}",
                    line_num + 1,
                    trimmed,
                    e
                )));
            }

            // Track executed command (skip "save" which is a no-op)
            if trimmed != "save" {
                self.commands_executed
                    .push(format!("Line {}: {}", line_num + 1, trimmed));
            }
        }

        Ok(self.config.clone())
    }

    /// Process a single command line
    fn process_command(&mut self, line: &str) -> Result<()> {
        // Handle save command (no-op in library context)
        if line == "save" {
            return Ok(());
        }

        let (cmd, params) = parse_command_line(line)?;

        // Execute command
        let new_config = execute_command(&self.config, &cmd, &params)?;

        // Update config unless dry-run
        if !self.dry_run {
            self.config = new_config;
        }

        Ok(())
    }

    /// Get execution summary
    pub fn summary(&self) -> String {
        format!(
            "Executed {} commands{}",
            self.commands_executed.len(),
            if self.dry_run { " (DRY RUN)" } else { "" }
        )
    }

    /// Get list of executed commands
    pub fn get_executed_commands(&self) -> &[String] {
        &self.commands_executed
    }

    /// Get current configuration
    pub fn get_config(&self) -> &str {
        &self.config
    }
}

/// Parse a command line into (command_name, parameters)
fn parse_command_line(line: &str) -> Result<(String, Value)> {
    let parts: Vec<&str> = line.splitn(2, ' ').collect();

    if parts.is_empty() {
        return Err(SzConfigError::InvalidInput("Empty command".to_string()));
    }

    let cmd = parts[0].to_string();

    let params = if parts.len() > 1 {
        serde_json::from_str(parts[1])
            .map_err(|e| SzConfigError::JsonParse(format!("Invalid JSON in '{cmd}': {e}")))?
    } else {
        Value::Null
    };

    Ok((cmd, params))
}

/// Execute a command and return updated config
fn execute_command(config: &str, cmd: &str, params: &Value) -> Result<String> {
    match cmd {
        // ===== Versioning Commands =====
        "verifyCompatibilityVersion" => {
            let expected = get_str_param(params, "expectedVersion")?;
            crate::versioning::verify_compatibility_version(config, expected)?;
            Ok(config.to_string()) // Verification only, no modification
        }

        "updateCompatibilityVersion" => {
            // Note: Function only takes new version, ignores fromVersion
            let to = get_str_param(params, "toVersion")?;
            crate::versioning::update_compatibility_version(config, to)
        }

        // ===== Config Section Commands =====
        "removeConfigSection" => {
            let section = get_str_param(params, "section")?;
            crate::config_sections::remove_config_section(config, section)
        }

        "removeConfigSectionField" => {
            let section = get_str_param(params, "section")?;
            let field = get_str_param(params, "field")?;
            crate::config_sections::remove_config_section_field(config, section, field)
                .map(|(cfg, _)| cfg)
        }

        "addConfigSection" => {
            let section = get_str_param(params, "section")?;
            // add_config_section only takes section name, creates empty array
            crate::config_sections::add_config_section(config, section)
        }

        "addConfigSectionField" => {
            let section = get_str_param(params, "section")?;
            let field = get_str_param(params, "field")?;
            let value = &params["value"];
            crate::config_sections::add_config_section_field(config, section, field, value)
                .map(|(cfg, _)| cfg)
        }

        // ===== Attribute Commands =====
        "addAttribute" => {
            let attr = get_str_param(params, "attribute")?;
            let class = get_str_param(params, "class")?;
            let feature = get_str_param(params, "feature")?;
            let element = get_str_param(params, "element")?;
            let required = get_opt_str_param(params, "required");
            let internal = get_opt_str_param(params, "internal");
            let default_value = get_opt_str_param(params, "default");

            crate::attributes::add_attribute(
                config,
                crate::attributes::AddAttributeParams {
                    attribute: attr,
                    feature,
                    element,
                    class,
                    default_value,
                    internal,
                    required,
                    id: params.get("id").and_then(|v| v.as_i64()),
                },
            )
            .map(|(cfg, _)| cfg)
        }

        "deleteAttribute" => {
            let attr = get_str_param(params, "attribute")?;
            crate::attributes::delete_attribute(config, attr)
        }

        "setAttribute" => {
            let set_params = crate::attributes::SetAttributeParams::try_from(params)?;
            crate::attributes::set_attribute(config, set_params)
        }

        // ===== Element Commands =====
        "addElement" => {
            let element = get_str_param(params, "element")?;
            let datatype = get_opt_str_param(params, "datatype");

            let add_params = crate::elements::AddElementParams {
                code: element,
                description: None, // Will default to code
                data_type: datatype,
                id: params.get("id").and_then(|v| v.as_i64()),
            };

            crate::elements::add_element(config, add_params)
        }

        "setFeatureElement" => {
            let feature = get_str_param(params, "feature")?;
            let element = get_str_param(params, "element")?;

            // Check which property to set
            if let Some(derived) = get_opt_str_param(params, "derived") {
                crate::elements::set_feature_element_derived(config, feature, element, derived)
            } else if let Some(display_level) = params.get("displayLevel").and_then(|v| v.as_i64())
            {
                crate::elements::set_feature_element_display_level(
                    config,
                    feature,
                    element,
                    display_level,
                )
            } else {
                Err(SzConfigError::InvalidInput(
                    "setFeatureElement requires 'derived' or 'displayLevel'".to_string(),
                ))
            }
        }

        // ===== Feature Commands =====
        "addFeature" => {
            let feature = get_str_param(params, "feature")?;
            let element_list = params
                .get("elementList")
                .ok_or_else(|| SzConfigError::MissingField("elementList".to_string()))?;

            crate::features::add_feature(
                config,
                crate::features::AddFeatureParams {
                    feature,
                    element_list,
                    class: get_opt_str_param(params, "class"),
                    behavior: get_opt_str_param(params, "behavior"),
                    candidates: get_opt_str_param(params, "candidates"),
                    anonymize: get_opt_str_param(params, "anonymize"),
                    derived: get_opt_str_param(params, "derived"),
                    history: get_opt_str_param(params, "history"),
                    matchkey: get_opt_str_param(params, "matchKey"),
                    standardize: get_opt_str_param(params, "standardize").filter(|s| !s.is_empty()),
                    expression: get_opt_str_param(params, "expression").filter(|s| !s.is_empty()),
                    comparison: get_opt_str_param(params, "comparison").filter(|s| !s.is_empty()),
                    version: params.get("version").and_then(|v| v.as_i64()),
                    rtype_id: params.get("rtypeId").and_then(|v| v.as_i64()),
                    id: params.get("id").and_then(|v| v.as_i64()),
                },
            )
        }

        "setFeature" => {
            let feature = get_str_param(params, "feature")?;

            crate::features::set_feature(
                config,
                crate::features::SetFeatureParams {
                    feature,
                    candidates: get_opt_str_param(params, "candidates"),
                    anonymize: get_opt_str_param(params, "anonymize"),
                    derived: get_opt_str_param(params, "derived"),
                    history: get_opt_str_param(params, "history"),
                    matchkey: get_opt_str_param(params, "matchKey"),
                    behavior: get_opt_str_param(params, "behavior"),
                    class: get_opt_str_param(params, "class"),
                    version: params.get("version").and_then(|v| v.as_i64()),
                    rtype_id: params.get("rtypeId").and_then(|v| v.as_i64()),
                },
            )
        }

        // ===== Behavior Override Commands =====
        "addBehaviorOverride" => {
            let feature = get_str_param(params, "feature")?;
            let usage_type = get_str_param(params, "usageType")?;
            let behavior = get_str_param(params, "behavior")?;

            crate::behavior_overrides::add_behavior_override(
                config,
                crate::behavior_overrides::AddBehaviorOverrideParams::new(
                    feature, usage_type, behavior,
                ),
            )
        }

        // ===== Fragment Commands =====
        "deleteFragment" => {
            let fragment = get_str_param(params, "fragment").or_else(|_| {
                // Support old format: deleteFragment FRAGMENT_NAME
                params
                    .as_str()
                    .ok_or_else(|| SzConfigError::MissingField("fragment".to_string()))
            })?;
            crate::fragments::delete_fragment(config, fragment)
        }

        "setFragment" => {
            let fragment = get_str_param(params, "fragment")?;
            let source = get_str_param(params, "source")?;

            crate::fragments::set_fragment(
                config,
                fragment,
                crate::fragments::SetFragmentParams {
                    source: crate::helpers::FieldUpdate::Set(source),
                    description: crate::helpers::FieldUpdate::Leave,
                },
            )
        }

        "addFragment" => {
            // add_fragment takes a full fragment config as Value
            crate::fragments::add_fragment(config, params).map(|(cfg, _)| cfg)
        }

        // ===== Rule Commands =====
        "addRule" => {
            let id = params
                .get("ERRULE_ID")
                .and_then(|v| v.as_i64())
                .unwrap_or(0);
            crate::rules::add_rule(config, id, params).map(|(cfg, _)| cfg)
        }

        "setRule" => {
            let set_params = crate::rules::SetRuleParams::try_from(params)?;
            crate::rules::set_rule(config, set_params)
        }

        // ===== System Parameter Commands =====
        "setSetting" => {
            let name = get_str_param(params, "name")?;
            let value = &params["value"];
            crate::system_params::set_system_parameter(config, name, value)
        }

        // ===== Function Commands - Standardize =====
        "removeStandardizeFunction" | "deleteStandardizeFunction" => {
            let func = get_str_param(params, "function")?;
            crate::functions::standardize::delete_standardize_function(config, func)
                .map(|(cfg, _)| cfg)
        }

        "addStandardizeFunction" => {
            let func = get_str_param(params, "function")?;
            let connect = get_str_param(params, "connectStr")?;
            let desc = get_opt_str_param(params, "description");
            let language = get_opt_str_param(params, "language");

            crate::functions::standardize::add_standardize_function(
                config,
                func,
                crate::functions::standardize::AddStandardizeFunctionParams {
                    connect_str: Some(connect),
                    description: desc,
                    language,
                },
            )
            .map(|(cfg, _)| cfg)
        }

        // ===== Function Commands - Comparison =====
        "removeComparisonFunction" | "deleteComparisonFunction" => {
            let func = get_str_param(params, "function")?;
            crate::functions::comparison::delete_comparison_function(config, func)
                .map(|(cfg, _)| cfg)
        }

        "addComparisonFunction" => {
            let func = get_str_param(params, "function")?;
            let connect = get_str_param(params, "connectStr")?;
            let anon = get_opt_str_param(params, "anonSupport");
            let desc = get_opt_str_param(params, "description");

            crate::functions::comparison::add_comparison_function(
                config,
                func,
                crate::functions::comparison::AddComparisonFunctionParams {
                    connect_str: Some(connect),
                    description: desc,
                    language: None,
                    anon_support: anon,
                },
            )
            .map(|(cfg, _)| cfg)
        }

        // ===== Function Commands - Expression =====
        "addExpressionFunction" => {
            let func = get_str_param(params, "function")?;
            let connect = get_str_param(params, "connectStr")?;
            let desc = get_opt_str_param(params, "description");
            let language = get_opt_str_param(params, "language");

            crate::functions::expression::add_expression_function(
                config,
                func,
                crate::functions::expression::AddExpressionFunctionParams {
                    connect_str: Some(connect),
                    description: desc,
                    language,
                },
            )
            .map(|(cfg, _)| cfg)
        }

        // ===== Threshold Commands =====
        "addComparisonThreshold" => {
            let func = get_str_param(params, "function")?;
            let feature = get_str_param(params, "feature")?;
            let score_name = get_str_param(params, "scoreName")?;
            let same = params.get("sameScore").and_then(|v| v.as_i64());
            let close = params.get("closeScore").and_then(|v| v.as_i64());
            let likely = params.get("likelyScore").and_then(|v| v.as_i64());
            let plausible = params.get("plausibleScore").and_then(|v| v.as_i64());
            let unlikely = params.get("unlikelyScore").and_then(|v| v.as_i64());

            crate::thresholds::add_comparison_threshold(
                config,
                crate::thresholds::AddComparisonThresholdParams {
                    cfunc_code: Some(func),
                    ftype_code: if feature.eq_ignore_ascii_case("ALL") {
                        None
                    } else {
                        Some(feature)
                    },
                    cfunc_rtnval: Some(score_name),
                    exec_order: None,
                    same_score: same,
                    close_score: close,
                    likely_score: likely,
                    plausible_score: plausible,
                    un_likely_score: unlikely,
                },
            )
        }

        "addGenericThreshold" => {
            let threshold_params = crate::thresholds::AddGenericThresholdParams::try_from(params)?;
            crate::thresholds::add_generic_threshold(config, threshold_params)
        }

        // ===== Call Commands - Expression =====
        "addExpressionCall" => {
            let feature = get_str_param(params, "feature")?;
            let function = get_str_param(params, "function")?;
            let exec_order = params.get("execOrder").and_then(|v| v.as_i64());
            let expr_feature = get_opt_str_param(params, "expressionFeature");
            let virtual_flag = get_opt_str_param(params, "virtual").unwrap_or("No");
            let element_list_json = params
                .get("elementList")
                .ok_or_else(|| SzConfigError::MissingField("elementList".to_string()))?;

            // Parse elementList: [{"element": "NAME", "required": "Yes", "feature": "NAME"}, ...]
            let element_list = parse_element_list(element_list_json)?;

            let call_params = crate::calls::expression::AddExpressionCallParams {
                efunc_code: function,
                element_list,
                ftype_code: Some(feature),
                felem_code: None,
                exec_order,
                expression_feature: expr_feature,
                is_virtual: virtual_flag,
            };

            let (new_config, _) =
                crate::calls::expression::add_expression_call(config, call_params)?;

            Ok(new_config)
        }

        // ===== Call Commands - Comparison =====
        "deleteComparisonCallElement" => {
            // The SDK now owns the feature->call resolution and exec_order
            // derivation (#40); the tool is a thin pass-through of the codes.
            let feature = get_str_param(params, "feature")?;
            let element = get_str_param(params, "element")?;
            // Optional: the element's feature disambiguates when one call carries
            // the same element under multiple features (#40 follow-up).
            let element_feature = params.get("elementFeature").and_then(|v| v.as_str());

            crate::calls::comparison::delete_comparison_call_element(
                config,
                crate::calls::CallSelector::Feature(feature),
                element,
                element_feature,
            )
        }

        "addComparisonCallElement" => {
            let feature = get_str_param(params, "feature")?;
            let element = get_str_param(params, "element")?;

            // Lookup IDs
            let ftype_id = crate::helpers::lookup_feature_id(config, feature)?;
            let felem_id = crate::helpers::lookup_element_id(config, element)?;

            // Find the cfcall_id for this feature
            let config_val: Value = serde_json::from_str(config)?;
            let cfcall_array = config_val["G2_CONFIG"]["CFG_CFCALL"]
                .as_array()
                .ok_or_else(|| SzConfigError::MissingSection("CFG_CFCALL".to_string()))?;

            let cfcall = cfcall_array
                .iter()
                .find(|call| call["FTYPE_ID"].as_i64() == Some(ftype_id))
                .ok_or_else(|| {
                    SzConfigError::NotFound(format!(
                        "No comparison call found for feature {feature}"
                    ))
                })?;

            let cfcall_id = cfcall["CFCALL_ID"]
                .as_i64()
                .ok_or_else(|| SzConfigError::InvalidStructure("CFCALL_ID missing".to_string()))?;

            // exec_order: None -> the SDK auto-allocates per CFCALL_ID. This drops
            // the old manual calc, which scoped by (call, ftype) and so restarted
            // the count per element-feature instead of numbering the whole call.
            crate::calls::comparison::add_comparison_call_element(
                config,
                crate::calls::comparison::AddComparisonCallElementParams {
                    cfcall_id,
                    ftype_id,
                    felem_id,
                    exec_order: None,
                },
            )
            .map(|(cfg, _)| cfg)
        }

        // ===== Call Commands - Distinct =====
        "deleteDistinctCallElement" => {
            // Thin pass-through: the SDK resolves the feature->call id and
            // derives the BOM exec_order internally (#40).
            let feature = get_str_param(params, "feature")?;
            let element = get_str_param(params, "element")?;
            // Optional: the element's feature disambiguates when one call carries
            // the same element under multiple features (#40 follow-up).
            let element_feature = params.get("elementFeature").and_then(|v| v.as_str());

            crate::calls::distinct::delete_distinct_call_element(
                config,
                crate::calls::CallSelector::Feature(feature),
                element,
                element_feature,
            )
        }

        // ===== No-op Commands =====
        "save" => Ok(config.to_string()),

        // ===== Unknown Command =====
        _ => Err(SzConfigError::InvalidInput(format!(
            "Unknown command: '{cmd}'"
        ))),
    }
}

// ===== Helper Functions =====

/// Get required string parameter
fn get_str_param<'a>(params: &'a Value, key: &str) -> Result<&'a str> {
    params[key]
        .as_str()
        .ok_or_else(|| SzConfigError::MissingField(key.to_string()))
}

/// Get optional string parameter
fn get_opt_str_param<'a>(params: &'a Value, key: &str) -> Option<&'a str> {
    params.get(key).and_then(|v| v.as_str())
}

/// Parse elementList from JSON into Vec<(element, required, feature)>
fn parse_element_list(list: &Value) -> Result<Vec<(String, String, Option<String>)>> {
    let arr = list
        .as_array()
        .ok_or_else(|| SzConfigError::InvalidInput("elementList must be array".to_string()))?;

    arr.iter()
        .map(|item| {
            let element = item["element"]
                .as_str()
                .ok_or_else(|| SzConfigError::MissingField("element".to_string()))?
                .to_string();
            let required = item
                .get("required")
                .and_then(|v| v.as_str())
                .unwrap_or("No")
                .to_string();
            let feature = item
                .get("feature")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            Ok((element, required, feature))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CONFIG: &str = r#"{
  "G2_CONFIG": {
    "CFG_DSRC": [],
    "CFG_ATTR": [],
    "CFG_FTYPE": [],
    "CFG_FELEM": [],
    "CFG_FCLASS": [
      {"FCLASS_ID": 1, "FCLASS_CODE": "OTHER"}
    ],
    "CFG_FBOVR": [],
    "CFG_ERFRAG": [],
    "CONFIG_BASE_VERSION": {
      "VERSION": "4.0.0",
      "BUILD_VERSION": "4.0.0.0",
      "BUILD_DATE": "2024-01-01",
      "COMPATIBILITY_VERSION": {
        "CONFIG_VERSION": "10"
      }
    }
  }
}"#;

    #[test]
    fn test_parse_command_line() {
        let (cmd, params) =
            parse_command_line(r#"addAttribute {"attribute": "TEST", "class": "OTHER"}"#)
                .expect("Failed to parse");

        assert_eq!(cmd, "addAttribute");
        assert_eq!(params["attribute"], "TEST");
        assert_eq!(params["class"], "OTHER");
    }

    #[test]
    fn test_parse_command_line_no_params() {
        let (cmd, params) = parse_command_line("save").expect("Failed to parse");

        assert_eq!(cmd, "save");
        assert!(params.is_null());
    }

    #[test]
    fn test_command_processor_simple_script() {
        let script = r#"
verifyCompatibilityVersion {"expectedVersion": "10"}
updateCompatibilityVersion {"fromVersion": "10", "toVersion": "11"}
save
"#;

        let mut processor = CommandProcessor::new(TEST_CONFIG.to_string());
        let result = processor.process_script(script);

        assert!(result.is_ok());
        assert_eq!(processor.commands_executed.len(), 2); // save is ignored

        let config: Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(
            config["G2_CONFIG"]["CONFIG_BASE_VERSION"]["COMPATIBILITY_VERSION"]["CONFIG_VERSION"],
            "11"
        );
    }

    #[test]
    fn test_command_processor_dry_run() {
        let script = r#"updateCompatibilityVersion {"fromVersion": "10", "toVersion": "11"}"#;

        let mut processor = CommandProcessor::new(TEST_CONFIG.to_string()).dry_run(true);
        let result = processor.process_script(script);

        assert!(result.is_ok());
        assert_eq!(processor.commands_executed.len(), 1);

        // Config should be unchanged in dry-run
        let config: Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(
            config["G2_CONFIG"]["CONFIG_BASE_VERSION"]["COMPATIBILITY_VERSION"]["CONFIG_VERSION"],
            "10"
        );
    }

    #[test]
    fn test_command_processor_invalid_command() {
        let script = r#"unknownCommand {"param": "value"}"#;

        let mut processor = CommandProcessor::new(TEST_CONFIG.to_string());
        let result = processor.process_script(script);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unknown command"));
    }

    #[test]
    fn test_command_processor_invalid_json() {
        let script = r#"addAttribute {invalid json}"#;

        let mut processor = CommandProcessor::new(TEST_CONFIG.to_string());
        let result = processor.process_script(script);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid JSON"));
    }

    #[test]
    fn test_execute_add_behavior_override() {
        let config_with_feature = r#"{
  "G2_CONFIG": {
    "CFG_FTYPE": [
      {"FTYPE_ID": 1, "FTYPE_CODE": "TEST"}
    ],
    "CFG_FBOVR": []
  }
}"#;

        let params = serde_json::json!({
            "feature": "TEST",
            "usageType": "BUSINESS",
            "behavior": "F1E"
        });

        let result = execute_command(config_with_feature, "addBehaviorOverride", &params);
        assert!(result.is_ok());

        let config: Value = serde_json::from_str(&result.unwrap()).unwrap();
        let overrides = &config["G2_CONFIG"]["CFG_FBOVR"];
        assert_eq!(overrides.as_array().unwrap().len(), 1);
        assert_eq!(overrides[0]["UTYPE_CODE"], "BUSINESS");
    }

    // ===== Coverage: every command arm, success and error paths =====

    use crate::error::SzErrorKind;
    use serde_json::json;

    /// The real Senzing g2config template (the conformance fixture).
    const TEMPLATE: &str = include_str!("../tests/fixtures/g2config_template.json");

    fn run(cmd: &str, params: Value) -> Result<String> {
        execute_command(TEMPLATE, cmd, &params)
    }

    fn parsed(config: &str) -> Value {
        serde_json::from_str(config).unwrap()
    }

    fn rows<'a>(config: &'a Value, section: &str) -> &'a Vec<Value> {
        config["G2_CONFIG"][section].as_array().unwrap()
    }

    fn has_row(config: &Value, section: &str, key: &str, value: &str) -> bool {
        rows(config, section)
            .iter()
            .any(|r| r[key].as_str() == Some(value))
    }

    /// Every required string parameter, when absent, fails with
    /// `MissingField(<param>)` before the underlying SDK call runs.
    #[test]
    fn test_missing_required_params_report_missing_field() {
        let cases: Vec<(&str, Value, &str)> = vec![
            ("verifyCompatibilityVersion", json!({}), "expectedVersion"),
            ("updateCompatibilityVersion", json!({}), "toVersion"),
            ("removeConfigSection", json!({}), "section"),
            ("removeConfigSectionField", json!({}), "section"),
            (
                "removeConfigSectionField",
                json!({"section": "CFG_ATTR"}),
                "field",
            ),
            ("addConfigSection", json!({}), "section"),
            ("addConfigSectionField", json!({}), "section"),
            (
                "addConfigSectionField",
                json!({"section": "CFG_ATTR"}),
                "field",
            ),
            ("addAttribute", json!({}), "attribute"),
            ("addAttribute", json!({"attribute": "A"}), "class"),
            (
                "addAttribute",
                json!({"attribute": "A", "class": "OTHER"}),
                "feature",
            ),
            (
                "addAttribute",
                json!({"attribute": "A", "class": "OTHER", "feature": "NAME"}),
                "element",
            ),
            ("deleteAttribute", json!({}), "attribute"),
            ("setAttribute", json!({}), "attribute"),
            ("addElement", json!({}), "element"),
            ("setFeatureElement", json!({}), "feature"),
            ("setFeatureElement", json!({"feature": "NAME"}), "element"),
            ("addFeature", json!({}), "feature"),
            ("addFeature", json!({"feature": "NEW_F"}), "elementList"),
            ("setFeature", json!({}), "feature"),
            ("addBehaviorOverride", json!({}), "feature"),
            (
                "addBehaviorOverride",
                json!({"feature": "NAME"}),
                "usageType",
            ),
            (
                "addBehaviorOverride",
                json!({"feature": "NAME", "usageType": "BUSINESS"}),
                "behavior",
            ),
            ("deleteFragment", json!({}), "fragment"),
            ("deleteFragment", Value::Null, "fragment"),
            ("setFragment", json!({}), "fragment"),
            ("setFragment", json!({"fragment": "SAME_NAME"}), "source"),
            ("addFragment", json!({}), "ERFRAG_CODE"),
            ("addRule", json!({}), "ERRULE_CODE"),
            ("setRule", json!({}), "code or rule"),
            ("setSetting", json!({}), "name"),
            ("removeStandardizeFunction", json!({}), "function"),
            ("deleteStandardizeFunction", json!({}), "function"),
            ("addStandardizeFunction", json!({}), "function"),
            (
                "addStandardizeFunction",
                json!({"function": "F"}),
                "connectStr",
            ),
            ("removeComparisonFunction", json!({}), "function"),
            ("deleteComparisonFunction", json!({}), "function"),
            ("addComparisonFunction", json!({}), "function"),
            (
                "addComparisonFunction",
                json!({"function": "F"}),
                "connectStr",
            ),
            ("addExpressionFunction", json!({}), "function"),
            (
                "addExpressionFunction",
                json!({"function": "F"}),
                "connectStr",
            ),
            ("addComparisonThreshold", json!({}), "function"),
            (
                "addComparisonThreshold",
                json!({"function": "STR_COMP"}),
                "feature",
            ),
            (
                "addComparisonThreshold",
                json!({"function": "STR_COMP", "feature": "ALL"}),
                "scoreName",
            ),
            ("addExpressionCall", json!({}), "feature"),
            ("addExpressionCall", json!({"feature": "NAME"}), "function"),
            (
                "addExpressionCall",
                json!({"feature": "NAME", "function": "NAME_HASHER"}),
                "elementList",
            ),
            (
                "addExpressionCall",
                json!({"feature": "NAME", "function": "NAME_HASHER", "elementList": [{}]}),
                "element",
            ),
            ("deleteComparisonCallElement", json!({}), "feature"),
            (
                "deleteComparisonCallElement",
                json!({"feature": "NAME"}),
                "element",
            ),
            ("addComparisonCallElement", json!({}), "feature"),
            (
                "addComparisonCallElement",
                json!({"feature": "NAME"}),
                "element",
            ),
            ("deleteDistinctCallElement", json!({}), "feature"),
            (
                "deleteDistinctCallElement",
                json!({"feature": "NAME"}),
                "element",
            ),
        ];

        for (cmd, params, field) in cases {
            let err = run(cmd, params).unwrap_err();
            assert_eq!(err.kind(), SzErrorKind::MissingField, "{cmd}: {err}");
            assert_eq!(
                err.to_string(),
                format!("Missing required field: {field}"),
                "{cmd}"
            );
        }
    }

    /// Errors raised by the underlying SDK call (after parameter extraction)
    /// propagate with their own variant.
    #[test]
    fn test_downstream_errors_propagate_variant() {
        let no_version = r#"{"G2_CONFIG": {}}"#;
        let err = execute_command(
            no_version,
            "verifyCompatibilityVersion",
            &json!({"expectedVersion": "11"}),
        )
        .unwrap_err();
        assert_eq!(err.kind(), SzErrorKind::NotFound);
        assert_eq!(err.to_string(), "CONFIG_VERSION not found");

        let err = run("addGenericThreshold", json!({"scoringCap": "not-a-number"})).unwrap_err();
        assert_eq!(err.kind(), SzErrorKind::InvalidInput);

        let err = run(
            "addExpressionCall",
            json!({"feature": "NAME", "function": "NAME_HASHER", "elementList": "FULL_NAME"}),
        )
        .unwrap_err();
        assert_eq!(err.kind(), SzErrorKind::InvalidInput);
        assert_eq!(err.to_string(), "Invalid input: elementList must be array");

        let err = run(
            "addExpressionCall",
            json!({
                "feature": "NAME",
                "function": "NO_SUCH_EFUNC",
                "elementList": [{"element": "FULL_NAME"}]
            }),
        )
        .unwrap_err();
        assert_eq!(err.kind(), SzErrorKind::NotFound);

        let err = run(
            "setFeatureElement",
            json!({"feature": "NAME", "element": "FULL_NAME"}),
        )
        .unwrap_err();
        assert_eq!(err.kind(), SzErrorKind::InvalidInput);
        assert_eq!(
            err.to_string(),
            "Invalid input: setFeatureElement requires 'derived' or 'displayLevel'"
        );
    }

    #[test]
    fn test_add_comparison_call_element_error_paths() {
        let params = json!({"feature": "F1", "element": "E1"});

        let err = run(
            "addComparisonCallElement",
            json!({"feature": "NO_SUCH_FEATURE", "element": "FULL_NAME"}),
        )
        .unwrap_err();
        assert_eq!(err.kind(), SzErrorKind::NotFound);
        assert_eq!(err.to_string(), "Feature 'NO_SUCH_FEATURE' not found");

        let err = run(
            "addComparisonCallElement",
            json!({"feature": "NAME", "element": "NO_SUCH_ELEMENT"}),
        )
        .unwrap_err();
        assert_eq!(err.kind(), SzErrorKind::NotFound);
        assert_eq!(err.to_string(), "Element 'NO_SUCH_ELEMENT' not found");

        let base = r#""CFG_FTYPE": [{"FTYPE_ID": 7, "FTYPE_CODE": "F1"}],
                      "CFG_FELEM": [{"FELEM_ID": 8, "FELEM_CODE": "E1"}]"#;

        let no_section = format!(r#"{{"G2_CONFIG": {{{base}}}}}"#);
        let err = execute_command(&no_section, "addComparisonCallElement", &params).unwrap_err();
        assert_eq!(err.kind(), SzErrorKind::MissingSection);
        assert_eq!(err.to_string(), "Missing config section: CFG_CFCALL");

        let no_call = format!(
            r#"{{"G2_CONFIG": {{{base}, "CFG_CFCALL": [{{"CFCALL_ID": 1, "FTYPE_ID": 99}}]}}}}"#
        );
        let err = execute_command(&no_call, "addComparisonCallElement", &params).unwrap_err();
        assert_eq!(err.kind(), SzErrorKind::NotFound);
        assert_eq!(err.to_string(), "No comparison call found for feature F1");

        let no_call_id =
            format!(r#"{{"G2_CONFIG": {{{base}, "CFG_CFCALL": [{{"FTYPE_ID": 7}}]}}}}"#);
        let err = execute_command(&no_call_id, "addComparisonCallElement", &params).unwrap_err();
        assert_eq!(err.kind(), SzErrorKind::InvalidStructure);
        assert_eq!(
            err.to_string(),
            "Invalid config structure: CFCALL_ID missing"
        );
    }

    #[test]
    fn test_config_section_commands_succeed() {
        let out = parsed(
            &run(
                "removeConfigSection",
                json!({"section": "CFG_DSRC_INTEREST"}),
            )
            .unwrap(),
        );
        assert!(out["G2_CONFIG"].get("CFG_DSRC_INTEREST").is_none());

        let out = parsed(
            &run(
                "removeConfigSectionField",
                json!({"section": "CFG_ATTR", "field": "INTERNAL"}),
            )
            .unwrap(),
        );
        assert!(
            rows(&out, "CFG_ATTR")
                .iter()
                .all(|r| r.get("INTERNAL").is_none())
        );

        let out = parsed(&run("addConfigSection", json!({"section": "CFG_NEW"})).unwrap());
        assert_eq!(out["G2_CONFIG"]["CFG_NEW"], json!([]));

        let out = parsed(
            &run(
                "addConfigSectionField",
                json!({"section": "CFG_ATTR", "field": "NEW_FIELD", "value": "x"}),
            )
            .unwrap(),
        );
        assert!(rows(&out, "CFG_ATTR").iter().all(|r| r["NEW_FIELD"] == "x"));
    }

    #[test]
    fn test_attribute_and_element_commands_succeed() {
        let added = run(
            "addAttribute",
            json!({
                "attribute": "MY_ATTR",
                "class": "OTHER",
                "feature": "NAME",
                "element": "FULL_NAME",
                "required": "No",
                "internal": "No",
                "default": "dflt",
                "id": 9001
            }),
        )
        .unwrap();
        let out = parsed(&added);
        let attr = rows(&out, "CFG_ATTR")
            .iter()
            .find(|r| r["ATTR_CODE"] == "MY_ATTR")
            .unwrap();
        assert_eq!(attr["ATTR_ID"], 9001);
        assert_eq!(attr["DEFAULT_VALUE"], "dflt");

        let set = execute_command(
            &added,
            "setAttribute",
            &json!({"attribute": "MY_ATTR", "internal": "Yes"}),
        )
        .unwrap();
        let out = parsed(&set);
        let attr = rows(&out, "CFG_ATTR")
            .iter()
            .find(|r| r["ATTR_CODE"] == "MY_ATTR")
            .unwrap();
        assert_eq!(attr["INTERNAL"], "Yes");

        let deleted =
            execute_command(&set, "deleteAttribute", &json!({"attribute": "MY_ATTR"})).unwrap();
        assert!(!has_row(
            &parsed(&deleted),
            "CFG_ATTR",
            "ATTR_CODE",
            "MY_ATTR"
        ));

        let out = parsed(
            &run(
                "addElement",
                json!({"element": "MY_ELEM", "datatype": "string", "id": 9002}),
            )
            .unwrap(),
        );
        let elem = rows(&out, "CFG_FELEM")
            .iter()
            .find(|r| r["FELEM_CODE"] == "MY_ELEM")
            .unwrap();
        assert_eq!(elem["FELEM_ID"], 9002);
    }

    fn fbom_row(config: &Value, ftype_id: i64, felem_id: i64) -> Value {
        rows(config, "CFG_FBOM")
            .iter()
            .find(|r| r["FTYPE_ID"] == ftype_id && r["FELEM_ID"] == felem_id)
            .cloned()
            .unwrap()
    }

    #[test]
    fn test_set_feature_element_derived_and_display_level() {
        // NAME is FTYPE_ID 1 and FULL_NAME is FELEM_ID 2 in the template.
        let out = parsed(
            &run(
                "setFeatureElement",
                json!({"feature": "NAME", "element": "FULL_NAME", "derived": "Yes"}),
            )
            .unwrap(),
        );
        assert_eq!(fbom_row(&out, 1, 2)["DERIVED"], "Yes");

        let out = parsed(
            &run(
                "setFeatureElement",
                json!({"feature": "NAME", "element": "FULL_NAME", "displayLevel": 0}),
            )
            .unwrap(),
        );
        assert_eq!(fbom_row(&out, 1, 2)["DISPLAY_LEVEL"], 0);
    }

    #[test]
    fn test_feature_commands_succeed() {
        let out = parsed(
            &run(
                "addFeature",
                json!({
                    "feature": "MY_FEAT",
                    "elementList": [{"element": "MY_FEAT_ELEM"}],
                    "class": "OTHER",
                    "behavior": "FM",
                    "candidates": "No",
                    "anonymize": "No",
                    "derived": "No",
                    "history": "Yes",
                    "matchKey": "Yes",
                    "standardize": "",
                    "expression": "",
                    "comparison": "",
                    "version": 2,
                    "rtypeId": 0,
                    "id": 9003
                }),
            )
            .unwrap(),
        );
        let feat = rows(&out, "CFG_FTYPE")
            .iter()
            .find(|r| r["FTYPE_CODE"] == "MY_FEAT")
            .unwrap();
        assert_eq!(feat["FTYPE_ID"], 9003);
        assert_eq!(feat["VERSION"], 2);

        let out = parsed(
            &run(
                "setFeature",
                json!({
                    "feature": "NAME",
                    "candidates": "No",
                    "anonymize": "No",
                    "derived": "No",
                    "history": "Yes",
                    "matchKey": "Yes",
                    "behavior": "NAME",
                    "class": "NAME",
                    "version": 3,
                    "rtypeId": 0
                }),
            )
            .unwrap(),
        );
        let name = rows(&out, "CFG_FTYPE")
            .iter()
            .find(|r| r["FTYPE_CODE"] == "NAME")
            .unwrap();
        assert_eq!(name["USED_FOR_CAND"], "No");
        assert_eq!(name["VERSION"], 3);
    }

    #[test]
    fn test_fragment_and_rule_commands_succeed() {
        let out = parsed(&run("deleteFragment", json!({"fragment": "SAME_NAME"})).unwrap());
        assert!(!has_row(&out, "CFG_ERFRAG", "ERFRAG_CODE", "SAME_NAME"));

        // Legacy form: the parameter is a bare JSON string.
        let out = parsed(&run("deleteFragment", json!("CLOSE_NAME")).unwrap());
        assert!(!has_row(&out, "CFG_ERFRAG", "ERFRAG_CODE", "CLOSE_NAME"));

        let out = parsed(
            &run(
                "setFragment",
                json!({"fragment": "SAME_NAME", "source": "./FRAGMENT[./GNR_CLOSE_NAME>0]"}),
            )
            .unwrap(),
        );
        let frag = rows(&out, "CFG_ERFRAG")
            .iter()
            .find(|r| r["ERFRAG_CODE"] == "SAME_NAME")
            .unwrap();
        assert_eq!(frag["ERFRAG_SOURCE"], "./FRAGMENT[./GNR_CLOSE_NAME>0]");

        let out = parsed(
            &run(
                "addFragment",
                json!({"ERFRAG_CODE": "MY_FRAG", "ERFRAG_SOURCE": "./FRAGMENT[./SAME_NAME>0]"}),
            )
            .unwrap(),
        );
        assert!(has_row(&out, "CFG_ERFRAG", "ERFRAG_CODE", "MY_FRAG"));

        let rule = json!({
            "ERRULE_CODE": "MY_RULE",
            "RESOLVE": "No",
            "RELATE": "Yes",
            "RTYPE_ID": 2,
            "QUAL_ERFRAG_CODE": "SAME_NAME"
        });
        let out = parsed(&run("addRule", rule.clone()).unwrap());
        let added = rows(&out, "CFG_ERRULE")
            .iter()
            .find(|r| r["ERRULE_CODE"] == "MY_RULE")
            .unwrap();
        assert!(added["ERRULE_ID"].as_i64().unwrap() >= 1000);

        let mut with_id = rule;
        with_id["ERRULE_ID"] = json!(4242);
        let out = parsed(&run("addRule", with_id).unwrap());
        let added = rows(&out, "CFG_ERRULE")
            .iter()
            .find(|r| r["ERRULE_CODE"] == "MY_RULE")
            .unwrap();
        assert_eq!(added["ERRULE_ID"], 4242);

        let out = parsed(&run("setRule", json!({"code": "SAME_A1", "tier": 11})).unwrap());
        let rule = rows(&out, "CFG_ERRULE")
            .iter()
            .find(|r| r["ERRULE_CODE"] == "SAME_A1")
            .unwrap();
        assert_eq!(rule["ERRULE_TIER"], 11);
    }

    #[test]
    fn test_set_setting_command_succeeds() {
        let out = parsed(
            &run(
                "setSetting",
                json!({"name": "relationshipsBreakMatches", "value": "Yes"}),
            )
            .unwrap(),
        );
        let disclosed = rows(&out, "CFG_RTYPE")
            .iter()
            .find(|r| r["RCLASS_ID"] == 2)
            .unwrap();
        assert_eq!(disclosed["BREAK_RES"], "Yes");
    }

    #[test]
    fn test_function_commands_succeed() {
        let add_std = json!({
            "function": "MY_SFUNC",
            "connectStr": "g2MySfunc",
            "description": "mine",
            "language": "en"
        });
        let added = run("addStandardizeFunction", add_std).unwrap();
        assert!(has_row(
            &parsed(&added),
            "CFG_SFUNC",
            "SFUNC_CODE",
            "MY_SFUNC"
        ));
        for cmd in ["removeStandardizeFunction", "deleteStandardizeFunction"] {
            let out = execute_command(&added, cmd, &json!({"function": "MY_SFUNC"})).unwrap();
            assert!(!has_row(
                &parsed(&out),
                "CFG_SFUNC",
                "SFUNC_CODE",
                "MY_SFUNC"
            ));
        }

        let add_cmp = json!({
            "function": "MY_CFUNC",
            "connectStr": "g2MyCfunc",
            "anonSupport": "Yes",
            "description": "mine"
        });
        let added = run("addComparisonFunction", add_cmp).unwrap();
        assert!(has_row(
            &parsed(&added),
            "CFG_CFUNC",
            "CFUNC_CODE",
            "MY_CFUNC"
        ));
        for cmd in ["removeComparisonFunction", "deleteComparisonFunction"] {
            let out = execute_command(&added, cmd, &json!({"function": "MY_CFUNC"})).unwrap();
            assert!(!has_row(
                &parsed(&out),
                "CFG_CFUNC",
                "CFUNC_CODE",
                "MY_CFUNC"
            ));
        }

        let out = parsed(
            &run(
                "addExpressionFunction",
                json!({
                    "function": "MY_EFUNC",
                    "connectStr": "g2MyEfunc",
                    "description": "mine",
                    "language": "en"
                }),
            )
            .unwrap(),
        );
        assert!(has_row(&out, "CFG_EFUNC", "EFUNC_CODE", "MY_EFUNC"));
    }

    #[test]
    fn test_threshold_commands_succeed() {
        let threshold = |feature: &str| {
            json!({
                "function": "STR_COMP",
                "feature": feature,
                "scoreName": "MY_SCORE",
                "sameScore": 100,
                "closeScore": 90,
                "likelyScore": 80,
                "plausibleScore": 70,
                "unlikelyScore": 60
            })
        };

        // NAME is FTYPE_ID 1 in the template.
        let out = parsed(&run("addComparisonThreshold", threshold("NAME")).unwrap());
        let row = rows(&out, "CFG_CFRTN")
            .iter()
            .find(|r| r["CFUNC_RTNVAL"] == "MY_SCORE")
            .unwrap();
        assert_eq!(row["FTYPE_ID"], 1);
        assert_eq!(row["UN_LIKELY_SCORE"], 60);

        // Current behavior: the processor maps "ALL" to ftype_code=None, which
        // the SDK rejects (the SDK itself resolves "all" when passed through).
        let err = run("addComparisonThreshold", threshold("ALL")).unwrap_err();
        assert_eq!(err.kind(), SzErrorKind::MissingField);
        assert_eq!(err.to_string(), "Missing required field: ftype_code");

        let before = rows(&parsed(TEMPLATE), "CFG_GENERIC_THRESHOLD").len();
        let out = parsed(
            &run(
                "addGenericThreshold",
                json!({
                    "plan": "SEARCH",
                    "behavior": "A1E",
                    "feature": "ALL",
                    "candidateCap": 3,
                    "scoringCap": 4,
                    "sendToRedo": "No"
                }),
            )
            .unwrap(),
        );
        assert_eq!(rows(&out, "CFG_GENERIC_THRESHOLD").len(), before + 1);
    }

    #[test]
    fn test_add_expression_call_command_succeeds() {
        let before = rows(&parsed(TEMPLATE), "CFG_EFCALL").len();
        let out = parsed(
            &run(
                "addExpressionCall",
                json!({
                    "feature": "NAME",
                    "function": "NAME_HASHER",
                    "execOrder": 900,
                    "expressionFeature": "NAME_KEY",
                    "virtual": "No",
                    "elementList": [
                        {"element": "FULL_NAME", "required": "Yes", "feature": "NAME"},
                        {"element": "SUR_NAME"}
                    ]
                }),
            )
            .unwrap(),
        );
        assert_eq!(rows(&out, "CFG_EFCALL").len(), before + 1);
    }

    /// The optional `elementFeature` disambiguator is passed through.
    #[test]
    fn test_delete_call_element_with_element_feature() {
        // NAME (FTYPE_ID 1) calls carry FULL_NAME (FELEM_ID 2) under NAME.
        for (cmd, bom, call_key) in [
            ("deleteComparisonCallElement", "CFG_CFBOM", "CFCALL_ID"),
            ("deleteDistinctCallElement", "CFG_DFBOM", "DFCALL_ID"),
        ] {
            let out = parsed(
                &run(
                    cmd,
                    json!({"feature": "NAME", "element": "FULL_NAME", "elementFeature": "NAME"}),
                )
                .unwrap(),
            );
            assert!(
                !rows(&out, bom)
                    .iter()
                    .any(|r| r[call_key] == 1 && r["FTYPE_ID"] == 1 && r["FELEM_ID"] == 2),
                "{cmd}"
            );
        }
    }

    #[test]
    fn test_save_with_params_is_a_no_op() {
        assert_eq!(run("save", json!({})).unwrap(), TEMPLATE);
    }

    #[test]
    fn test_parse_element_list_defaults() {
        let list = parse_element_list(&json!([
            {"element": "A", "required": "Yes", "feature": "F"},
            {"element": "B"}
        ]))
        .unwrap();
        assert_eq!(
            list,
            vec![
                ("A".to_string(), "Yes".to_string(), Some("F".to_string())),
                ("B".to_string(), "No".to_string(), None),
            ]
        );
    }
}
