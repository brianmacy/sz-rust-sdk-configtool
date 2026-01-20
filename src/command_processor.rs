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
            SzConfigError::InvalidConfig(format!("Failed to read script file: {}", e))
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
            .map_err(|e| SzConfigError::JsonParse(format!("Invalid JSON in '{}': {}", cmd, e)))?
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

            // Function signature: (attribute, feature, element, class, default, internal, required)
            crate::attributes::add_attribute(
                config,
                attr,
                feature,
                element,
                class,
                default_value,
                internal,
                required,
            )
            .map(|(cfg, _)| cfg)
        }

        "deleteAttribute" => {
            let attr = get_str_param(params, "attribute")?;
            crate::attributes::delete_attribute(config, attr)
        }

        "setAttribute" => {
            let attr = get_str_param(params, "attribute")?;
            // Remove "attribute" key, pass rest as updates
            let mut updates = params.clone();
            if let Some(obj) = updates.as_object_mut() {
                obj.remove("attribute");
            }
            crate::attributes::set_attribute(config, attr, &updates)
        }

        // ===== Element Commands =====
        "addElement" => {
            let element = get_str_param(params, "element")?;
            let datatype = get_opt_str_param(params, "datatype").unwrap_or("string");

            let elem_config = serde_json::json!({
                "FELEM_CODE": element,
                "FELEM_DESC": element,
                "DATA_TYPE": datatype
            });

            crate::elements::add_element(config, element, &elem_config)
        }

        "setFeatureElement" => {
            let feature = get_str_param(params, "feature")?;
            let element = get_str_param(params, "element")?;

            // Lookup IDs
            let ftype_id = crate::helpers::lookup_feature_id(config, feature)?;
            let felem_id = crate::helpers::lookup_element_id(config, element)?;

            // Check which property to set
            if let Some(derived) = get_opt_str_param(params, "derived") {
                crate::elements::set_feature_element_derived(config, ftype_id, felem_id, derived)
            } else if let Some(display_level) = params.get("displayLevel").and_then(|v| v.as_i64())
            {
                crate::elements::set_feature_element_display_level(
                    config,
                    ftype_id,
                    felem_id,
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
            let class = get_opt_str_param(params, "class");
            let behavior = get_opt_str_param(params, "behavior");
            let candidates = get_opt_str_param(params, "candidates");
            let anonymize = get_opt_str_param(params, "anonymize");
            let derived = get_opt_str_param(params, "derived");
            let history = get_opt_str_param(params, "history");
            let matchkey = get_opt_str_param(params, "matchKey");
            let standardize = get_opt_str_param(params, "standardize").filter(|s| !s.is_empty());
            let expression = get_opt_str_param(params, "expression").filter(|s| !s.is_empty());
            let comparison = get_opt_str_param(params, "comparison").filter(|s| !s.is_empty());
            let element_list = params
                .get("elementList")
                .ok_or_else(|| SzConfigError::MissingField("elementList".to_string()))?;

            crate::features::add_feature(
                config,
                feature,
                element_list,
                class,
                behavior,
                candidates,
                anonymize,
                derived,
                history,
                matchkey,
                standardize,
                expression,
                comparison,
                None,
                None,
            )
        }

        "setFeature" => {
            let feature = get_str_param(params, "feature")?;
            let candidates = get_opt_str_param(params, "candidates");
            let anonymize = get_opt_str_param(params, "anonymize");
            let derived = get_opt_str_param(params, "derived");
            let history = get_opt_str_param(params, "history");
            let matchkey = get_opt_str_param(params, "matchKey");
            let behavior = get_opt_str_param(params, "behavior");
            let class = get_opt_str_param(params, "class");
            let version = params.get("version").and_then(|v| v.as_i64());
            let rtype_id = params.get("rtypeId").and_then(|v| v.as_i64());

            crate::features::set_feature(
                config, feature, candidates, anonymize, derived, history, matchkey, behavior,
                class, version, rtype_id,
            )
        }

        // ===== Behavior Override Commands =====
        "addBehaviorOverride" => {
            let feature = get_str_param(params, "feature")?;
            let usage_type = get_str_param(params, "usageType")?;
            let behavior = get_str_param(params, "behavior")?;

            crate::behavior_overrides::add_behavior_override(config, feature, usage_type, behavior)
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

            // Construct fragment config with ERFRAG_SOURCE
            let fragment_config = serde_json::json!({
                "ERFRAG_SOURCE": source
            });

            crate::fragments::set_fragment(config, fragment, &fragment_config)
        }

        "addFragment" => {
            // add_fragment takes a full fragment config as Value
            crate::fragments::add_fragment(config, params).map(|(cfg, _)| cfg)
        }

        // ===== Rule Commands =====
        "addRule" => crate::rules::add_rule(config, params).map(|(cfg, _)| cfg),

        "setRule" => {
            let rule_code = get_str_param(params, "rule")?;
            crate::rules::set_rule(config, rule_code, params)
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
                config, func, connect, desc, language,
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
                config, func, connect, desc, None, anon,
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
                config, func, connect, desc, language,
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

            // Lookup function ID and feature ID
            let cfunc_id = crate::helpers::lookup_cfunc_id(config, func)?;
            let ftype_id = if feature.eq_ignore_ascii_case("ALL") {
                Some(0)
            } else {
                Some(crate::helpers::lookup_feature_id(config, feature)?)
            };

            crate::thresholds::add_comparison_threshold(
                config, cfunc_id, score_name, ftype_id, None, // exec_order
                same, close, likely, plausible, unlikely,
            )
        }

        "addGenericThreshold" => {
            let plan = get_str_param(params, "plan")?;
            let feature = get_opt_str_param(params, "feature");
            let behavior = get_str_param(params, "behavior")?;
            let candidate_cap = params
                .get("candidateCap")
                .and_then(|v| v.as_i64())
                .unwrap_or(0);
            let scoring_cap = params
                .get("scoringCap")
                .and_then(|v| v.as_i64())
                .unwrap_or(0);
            let send_to_redo = get_opt_str_param(params, "sendToRedo").unwrap_or("No");

            crate::thresholds::add_generic_threshold(
                config,
                plan,
                behavior,
                scoring_cap,
                candidate_cap,
                send_to_redo,
                feature,
            )
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

            let (new_config, _) = crate::calls::expression::add_expression_call(
                config,
                Some(feature),
                None,
                exec_order,
                function,
                element_list,
                expr_feature,
                virtual_flag,
            )?;

            Ok(new_config)
        }

        // ===== Call Commands - Comparison =====
        "deleteComparisonCallElement" => {
            // TODO: Complex command - requires finding call ID from feature
            // The SDK function needs: cfcall_id, ftype_id, felem_id, exec_order
            // The script only provides: feature, element
            // Need to find the call record first, then delete from it
            Err(SzConfigError::NotImplemented(
                "deleteComparisonCallElement requires call lookup - use SDK directly".to_string(),
            ))
        }

        "addComparisonCallElement" => {
            // TODO: Complex command - requires finding call ID from feature
            Err(SzConfigError::NotImplemented(
                "addComparisonCallElement requires call lookup - use SDK directly".to_string(),
            ))
        }

        // ===== Call Commands - Distinct =====
        "deleteDistinctCallElement" => {
            // TODO: Complex command - requires finding call ID from feature
            Err(SzConfigError::NotImplemented(
                "deleteDistinctCallElement requires call lookup - use SDK directly".to_string(),
            ))
        }

        // ===== No-op Commands =====
        "save" => Ok(config.to_string()),

        // ===== Unknown Command =====
        _ => Err(SzConfigError::InvalidInput(format!(
            "Unknown command: '{}'",
            cmd
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
}
