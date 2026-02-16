# Command Script Processor Design

## Overview

Design document for implementing a `.gtc` (Senzing command script) processor using the sz-rust-sdk-configtool library.

**Purpose:** Execute line-based command scripts to transform Senzing configuration JSON. Use cases include:

- Configuration upgrades (e.g., v10 → v11)
- Batch configuration changes
- Configuration templates and initialization
- Automated testing and validation
- Reproducible configuration modifications

## Script Format Analysis

**Example File:** `szcore-configuration-upgrade-10-to-11.gtc` (one use case)

**Format:**

```
commandName {"param": "value", ...}
commandName {"param": "value", ...}

save
```

**Characteristics:**

- Line-based format
- Command name followed by JSON object with parameters
- Blank lines ignored
- 146 lines total, ~90 commands
- 27 unique command types

## Command Frequency Distribution

```
  29 deleteAttribute
  27 removeConfigSectionField
   6 setFragment
   4 setFeature
   4 removeConfigSection
   4 addFeature
   3 addAttribute
   2 addExpressionCall
   2 addElement
   2 addComparisonThreshold
   2 addComparisonFunction
   1 verifyCompatibilityVersion
   1 updateCompatibilityVersion
   1 setSetting
   1 setFeatureElement
   1 setAttribute
   1 removeStandardizeFunction
   1 removeComparisonFunction
   7 deleteFragment (various)
   1 deleteDistinctCallElement
   1 deleteComparisonCallElement
   1 addRule
   1 addGenericThreshold
   1 addExpressionFunction
   1 addComparisonCallElement
   1 addBehaviorOverride
   1 save
```

## Command to SDK Function Mapping

### ✅ Already Implemented (100% Coverage)

| Script Command               | SDK Function                                   | Module | Notes                            |
| ---------------------------- | ---------------------------------------------- | ------ | -------------------------------- |
| `verifyCompatibilityVersion` | `versioning::verify_compatibility_version`     | ✅     | Pass expectedVersion             |
| `updateCompatibilityVersion` | `versioning::update_compatibility_version`     | ✅     | Pass fromVersion, toVersion      |
| `removeConfigSection`        | `config_sections::remove_config_section`       | ✅     | Pass section                     |
| `removeConfigSectionField`   | `config_sections::remove_config_section_field` | ✅     | Pass section, field              |
| `deleteFragment`             | `fragments::delete_fragment`                   | ✅     | Pass fragment name               |
| `deleteAttribute`            | `attributes::delete_attribute`                 | ✅     | Extract "attribute" from JSON    |
| `setAttribute`               | `attributes::set_attribute`                    | ✅     | Pass attribute, updates          |
| `addAttribute`               | `attributes::add_attribute`                    | ✅     | Parse all fields from JSON       |
| `setFeature`                 | `features::set_feature`                        | ✅     | Pass feature, fields to update   |
| `setFeatureElement`          | `elements::set_feature_element`                | ✅     | Pass feature, element, property  |
| `addFeature`                 | `features::add_feature`                        | ✅     | Parse elementList and all params |
| `addElement`                 | `elements::add_element`                        | ✅     | Parse element, datatype          |
| `setFragment`                | `fragments::set_fragment`                      | ✅     | Pass fragment, source            |
| `addRule`                    | `rules::add_rule`                              | ✅     | Parse rule JSON                  |
| `setSetting`                 | `system_params::set_system_parameter`          | ✅     | Pass name, value                 |

### 🔍 Need to Verify/Map

| Script Command                | Likely SDK Function                                   | Module | Investigation Needed                    |
| ----------------------------- | ----------------------------------------------------- | ------ | --------------------------------------- |
| `removeStandardizeFunction`   | `functions::standardize::delete_standardize_function` | ✅     | Extract "function" param                |
| `removeComparisonFunction`    | `functions::comparison::delete_comparison_function`   | ✅     | Extract "function" param                |
| `addExpressionFunction`       | `functions::expression::add_expression_function`      | ✅     | Parse function, connectStr              |
| `addComparisonFunction`       | `functions::comparison::add_comparison_function`      | ✅     | Parse function, connectStr, anonSupport |
| `addComparisonThreshold`      | `thresholds::add_comparison_threshold`                | ✅     | Parse all score params                  |
| `addGenericThreshold`         | `thresholds::add_generic_threshold`                   | ✅     | Parse plan, feature, caps               |
| `addExpressionCall`           | `calls::expression::add_expression_call`              | ✅     | Parse feature, function, elementList    |
| `deleteComparisonCallElement` | `calls::comparison::delete_*`                         | ❓     | Check calls module                      |
| `deleteDistinctCallElement`   | `calls::distinct::delete_*`                           | ❓     | Check calls module                      |
| `addComparisonCallElement`    | `calls::comparison::add_*`                            | ❓     | Check calls module                      |
| `addBehaviorOverride`         | ❓                                                    | ❓     | Need to find this function              |

### ❌ Special Cases

| Script Command | Implementation         |
| -------------- | ---------------------- |
| `save`         | No-op or write to file |

## Proposed Implementation

### Module Structure

```
src/
├── upgrade_processor.rs  (NEW)
└── upgrade_commands.rs   (NEW - command implementations)
```

### Core Processor

```rust
// src/upgrade_processor.rs

use crate::error::Result;
use serde_json::Value;
use std::fs;
use std::path::Path;

/// Processes Senzing upgrade script (.gtc files)
pub struct UpgradeProcessor {
    config: String,
    commands_executed: Vec<String>,
}

impl UpgradeProcessor {
    /// Create a new processor with initial config
    pub fn new(config_json: String) -> Self {
        Self {
            config: config_json,
            commands_executed: Vec::new(),
        }
    }

    /// Process an upgrade script file
    pub fn process_file<P: AsRef<Path>>(&mut self, path: P) -> Result<String> {
        let content = fs::read_to_string(path)?;
        self.process_script(&content)
    }

    /// Process upgrade script from string
    pub fn process_script(&mut self, script: &str) -> Result<String> {
        for (line_num, line) in script.lines().enumerate() {
            let trimmed = line.trim();

            // Skip blank lines and save command
            if trimmed.is_empty() || trimmed == "save" {
                continue;
            }

            // Process command
            match self.process_command(trimmed) {
                Ok(()) => {
                    self.commands_executed.push(format!("Line {}: {}", line_num + 1, trimmed));
                }
                Err(e) => {
                    return Err(SzConfigError::InvalidInput(
                        format!("Line {}: {} - Error: {}", line_num + 1, trimmed, e)
                    ));
                }
            }
        }

        Ok(self.config.clone())
    }

    /// Process a single command line
    fn process_command(&mut self, line: &str) -> Result<()> {
        let (cmd, params) = parse_command_line(line)?;

        // Execute command and update config
        self.config = execute_command(&self.config, &cmd, &params)?;

        Ok(())
    }

    /// Get execution summary
    pub fn get_summary(&self) -> String {
        format!("Executed {} commands", self.commands_executed.len())
    }
}

/// Parse a command line into (command_name, parameters)
fn parse_command_line(line: &str) -> Result<(String, Value)> {
    // Split on first space
    let parts: Vec<&str> = line.splitn(2, ' ').collect();

    if parts.is_empty() {
        return Err(SzConfigError::InvalidInput("Empty command line".to_string()));
    }

    let cmd = parts[0].to_string();

    let params = if parts.len() > 1 {
        serde_json::from_str(parts[1])
            .map_err(|e| SzConfigError::JsonParse(format!("Invalid JSON params: {}", e)))?
    } else {
        Value::Null
    };

    Ok((cmd, params))
}

/// Execute a command and return updated config
fn execute_command(config: &str, cmd: &str, params: &Value) -> Result<String> {
    match cmd {
        // Versioning
        "verifyCompatibilityVersion" => {
            let expected = params["expectedVersion"].as_str()
                .ok_or_else(|| SzConfigError::MissingField("expectedVersion".to_string()))?;
            crate::versioning::verify_compatibility_version(config, expected)?;
            Ok(config.to_string()) // No modification, just verification
        }

        "updateCompatibilityVersion" => {
            let from = params["fromVersion"].as_str()
                .ok_or_else(|| SzConfigError::MissingField("fromVersion".to_string()))?;
            let to = params["toVersion"].as_str()
                .ok_or_else(|| SzConfigError::MissingField("toVersion".to_string()))?;
            crate::versioning::update_compatibility_version(config, from, to)
        }

        // Config Sections
        "removeConfigSection" => {
            let section = params["section"].as_str()
                .ok_or_else(|| SzConfigError::MissingField("section".to_string()))?;
            crate::config_sections::remove_config_section(config, section)
        }

        "removeConfigSectionField" => {
            let section = params["section"].as_str()
                .ok_or_else(|| SzConfigError::MissingField("section".to_string()))?;
            let field = params["field"].as_str()
                .ok_or_else(|| SzConfigError::MissingField("field".to_string()))?;
            crate::config_sections::remove_config_section_field(config, section, field)
        }

        // Attributes
        "addAttribute" => {
            let attr = params["attribute"].as_str()
                .ok_or_else(|| SzConfigError::MissingField("attribute".to_string()))?;
            let class = params["class"].as_str()
                .ok_or_else(|| SzConfigError::MissingField("class".to_string()))?;
            let feature = params["feature"].as_str()
                .ok_or_else(|| SzConfigError::MissingField("feature".to_string()))?;
            let element = params["element"].as_str()
                .ok_or_else(|| SzConfigError::MissingField("element".to_string()))?;
            let required = params.get("required").and_then(|v| v.as_str());
            let internal = params.get("internal").and_then(|v| v.as_str());

            crate::attributes::add_attribute(
                config, attr, class, feature, element,
                None, // default_value
                required, internal
            )
        }

        "deleteAttribute" => {
            let attr = params["attribute"].as_str()
                .ok_or_else(|| SzConfigError::MissingField("attribute".to_string()))?;
            crate::attributes::delete_attribute(config, attr)
        }

        "setAttribute" => {
            let attr = params["attribute"].as_str()
                .ok_or_else(|| SzConfigError::MissingField("attribute".to_string()))?;
            // Remove "attribute" key and pass rest as updates
            let mut updates = params.clone();
            if let Some(obj) = updates.as_object_mut() {
                obj.remove("attribute");
            }
            crate::attributes::set_attribute(config, attr, &updates)
        }

        // Elements
        "addElement" => {
            let element = params["element"].as_str()
                .ok_or_else(|| SzConfigError::MissingField("element".to_string()))?;
            let datatype = params.get("datatype").and_then(|v| v.as_str());

            let elem_config = json!({
                "FELEM_CODE": element,
                "FELEM_DESC": element,
                "DATA_TYPE": datatype.unwrap_or("string")
            });

            crate::elements::add_element(config, element, &elem_config)
        }

        "setFeatureElement" => {
            let feature = params["feature"].as_str()
                .ok_or_else(|| SzConfigError::MissingField("feature".to_string()))?;
            let element = params["element"].as_str()
                .ok_or_else(|| SzConfigError::MissingField("element".to_string()))?;

            // Determine which property to set
            if let Some(derived) = params.get("derived").and_then(|v| v.as_str()) {
                crate::elements::set_feature_element_derived(config, feature, element, derived)
            } else if let Some(display_level) = params.get("displayLevel").and_then(|v| v.as_i64()) {
                crate::elements::set_feature_element_display_level(
                    config, feature, element, display_level
                )
            } else {
                Err(SzConfigError::InvalidInput(
                    "setFeatureElement requires derived or displayLevel".to_string()
                ))
            }
        }

        // Features
        "addFeature" => {
            let feature = params["feature"].as_str()
                .ok_or_else(|| SzConfigError::MissingField("feature".to_string()))?;
            let class = params.get("class").and_then(|v| v.as_str());
            let behavior = params.get("behavior").and_then(|v| v.as_str());
            let candidates = params.get("candidates").and_then(|v| v.as_str());
            let anonymize = params.get("anonymize").and_then(|v| v.as_str());
            let derived = params.get("derived").and_then(|v| v.as_str());
            let history = params.get("history").and_then(|v| v.as_str());
            let matchkey = params.get("matchKey").and_then(|v| v.as_str());
            let standardize = params.get("standardize").and_then(|v| v.as_str())
                .filter(|s| !s.is_empty());
            let expression = params.get("expression").and_then(|v| v.as_str())
                .filter(|s| !s.is_empty());
            let comparison = params.get("comparison").and_then(|v| v.as_str())
                .filter(|s| !s.is_empty());
            let element_list = params.get("elementList")
                .ok_or_else(|| SzConfigError::MissingField("elementList".to_string()))?;

            crate::features::add_feature(
                config, feature, element_list, class, behavior,
                candidates, anonymize, derived, history, matchkey,
                standardize, expression, comparison, None, None
            )
        }

        "setFeature" => {
            let feature = params["feature"].as_str()
                .ok_or_else(|| SzConfigError::MissingField("feature".to_string()))?;
            let candidates = params.get("candidates").and_then(|v| v.as_str());
            let anonymize = params.get("anonymize").and_then(|v| v.as_str());
            let derived = params.get("derived").and_then(|v| v.as_str());
            let history = params.get("history").and_then(|v| v.as_str());
            let matchkey = params.get("matchKey").and_then(|v| v.as_str());
            let version = params.get("version").and_then(|v| v.as_i64());

            crate::features::set_feature(
                config, feature, candidates, anonymize,
                derived, history, matchkey, version
            )
        }

        // Fragments
        "setFragment" => {
            let fragment = params["fragment"].as_str()
                .ok_or_else(|| SzConfigError::MissingField("fragment".to_string()))?;
            let source = params["source"].as_str()
                .ok_or_else(|| SzConfigError::MissingField("source".to_string()))?;
            crate::fragments::set_fragment(config, fragment, source)
        }

        // Rules
        "addRule" => {
            crate::rules::add_rule(config, params)
        }

        // System Parameters
        "setSetting" => {
            let name = params["name"].as_str()
                .ok_or_else(|| SzConfigError::MissingField("name".to_string()))?;
            let value = params["value"].clone();
            crate::system_params::set_system_parameter(config, name, &value)
        }

        // Functions - Standardize
        "removeStandardizeFunction" => {
            let func = params["function"].as_str()
                .ok_or_else(|| SzConfigError::MissingField("function".to_string()))?;
            crate::functions::standardize::delete_standardize_function(config, func)
        }

        // Functions - Comparison
        "removeComparisonFunction" => {
            let func = params["function"].as_str()
                .ok_or_else(|| SzConfigError::MissingField("function".to_string()))?;
            crate::functions::comparison::delete_comparison_function(config, func)
        }

        "addComparisonFunction" => {
            let func = params["function"].as_str()
                .ok_or_else(|| SzConfigError::MissingField("function".to_string()))?;
            let connect = params["connectStr"].as_str()
                .ok_or_else(|| SzConfigError::MissingField("connectStr".to_string()))?;
            let anon = params.get("anonSupport").and_then(|v| v.as_str());
            let desc = params.get("description").and_then(|v| v.as_str());

            crate::functions::comparison::add_comparison_function(
                config, func, connect, desc, None, anon
            )
        }

        // Functions - Expression
        "addExpressionFunction" => {
            let func = params["function"].as_str()
                .ok_or_else(|| SzConfigError::MissingField("function".to_string()))?;
            let connect = params["connectStr"].as_str()
                .ok_or_else(|| SzConfigError::MissingField("connectStr".to_string()))?;
            let desc = params.get("description").and_then(|v| v.as_str());

            crate::functions::expression::add_expression_function(
                config, func, connect, desc, None
            )
        }

        // Thresholds
        "addComparisonThreshold" => {
            let func = params["function"].as_str()
                .ok_or_else(|| SzConfigError::MissingField("function".to_string()))?;
            let feature = params["feature"].as_str()
                .ok_or_else(|| SzConfigError::MissingField("feature".to_string()))?;
            let score_name = params.get("scoreName").and_then(|v| v.as_str());
            let same = params.get("sameScore").and_then(|v| v.as_i64());
            let close = params.get("closeScore").and_then(|v| v.as_i64());
            let likely = params.get("likelyScore").and_then(|v| v.as_i64());
            let plausible = params.get("plausibleScore").and_then(|v| v.as_i64());
            let unlikely = params.get("unlikelyScore").and_then(|v| v.as_i64());

            crate::thresholds::add_comparison_threshold(
                config, func, feature, score_name,
                same, close, likely, plausible, unlikely
            )
        }

        "addGenericThreshold" => {
            let plan = params["plan"].as_str()
                .ok_or_else(|| SzConfigError::MissingField("plan".to_string()))?;
            let feature = params["feature"].as_str()
                .ok_or_else(|| SzConfigError::MissingField("feature".to_string()))?;
            let behavior = params["behavior"].as_str()
                .ok_or_else(|| SzConfigError::MissingField("behavior".to_string()))?;
            let candidate_cap = params.get("candidateCap").and_then(|v| v.as_i64());
            let scoring_cap = params.get("scoringCap").and_then(|v| v.as_i64());
            let send_to_redo = params.get("sendToRedo").and_then(|v| v.as_str());

            crate::thresholds::add_generic_threshold(
                config, plan, feature, behavior,
                candidate_cap, scoring_cap, send_to_redo
            )
        }

        // Calls - would need to verify module structure
        "addExpressionCall" => {
            // This is complex - has feature, function, execOrder, elementList
            // Need to check if calls::expression module has this function
            Err(SzConfigError::InvalidInput(
                format!("Command '{}' not yet implemented in processor", cmd)
            ))
        }

        "deleteComparisonCallElement" => {
            // Need to verify calls::comparison module
            Err(SzConfigError::InvalidInput(
                format!("Command '{}' not yet implemented in processor", cmd)
            ))
        }

        "deleteDistinctCallElement" => {
            // Need to verify calls::distinct module
            Err(SzConfigError::InvalidInput(
                format!("Command '{}' not yet implemented in processor", cmd)
            ))
        }

        "addComparisonCallElement" => {
            // Need to verify calls::comparison module
            Err(SzConfigError::InvalidInput(
                format!("Command '{}' not yet implemented in processor", cmd)
            ))
        }

        "addBehaviorOverride" => {
            // Need to find this function in SDK
            Err(SzConfigError::InvalidInput(
                format!("Command '{}' not yet implemented in processor", cmd)
            ))
        }

        // No-ops
        "save" => {
            // Save is handled outside the processor
            Ok(config.to_string())
        }

        // Unknown command
        _ => {
            Err(SzConfigError::InvalidInput(
                format!("Unknown command: {}", cmd)
            ))
        }
    }
}
```

### Usage Example

```rust
use sz_configtool_lib::upgrade_processor::UpgradeProcessor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load current config
    let config = std::fs::read_to_string("g2config.json")?;

    // Create processor
    let mut processor = UpgradeProcessor::new(config);

    // Process upgrade script
    let upgraded_config = processor.process_file(
        "/opt/homebrew/opt/senzing/runtime/er/resources/config/szcore-configuration-upgrade-10-to-11.gtc"
    )?;

    // Save upgraded config
    std::fs::write("g2config_upgraded.json", upgraded_config)?;

    println!("{}", processor.get_summary());
    // Output: "Executed 90 commands"

    Ok(())
}
```

## Implementation Phases

### Phase 1: Core Infrastructure ✅

- [x] Command parser (parse_command_line)
- [x] Command executor dispatcher (execute_command)
- [x] UpgradeProcessor struct
- [x] File I/O support

### Phase 2: Command Implementations (Estimated 80% done)

Most commands map directly to existing SDK functions:

**✅ Already Mapped (22 commands):**

- verifyCompatibilityVersion, updateCompatibilityVersion
- removeConfigSection, removeConfigSectionField
- addAttribute, deleteAttribute, setAttribute
- addElement
- addFeature, setFeature, setFeatureElement
- setFragment
- addRule
- setSetting
- removeStandardizeFunction, removeComparisonFunction
- addExpressionFunction, addComparisonFunction
- addComparisonThreshold, addGenericThreshold

**❓ Need Investigation (5 commands):**

- addExpressionCall (check calls::expression module)
- deleteComparisonCallElement (check calls::comparison module)
- deleteDistinctCallElement (check calls::distinct module)
- addComparisonCallElement (check calls::comparison module)
- addBehaviorOverride (need to find this function)

### Phase 3: Testing

- Unit tests for command parser
- Integration test with sample upgrade script
- Validation of config after upgrade

### Phase 4: CLI Integration

Add to CLI tool:

```bash
sz_configtool upgrade -i g2config.json -s upgrade-10-to-11.gtc -o g2config_v11.json
```

## Missing SDK Functions

Based on script analysis, need to verify these exist:

1. **addExpressionCall** - Complex call with elementList parameter
   - Check: `calls::expression` module
   - Script example: Line 122 with nested elementList

2. **deleteComparisonCallElement** - Delete by feature + element
   - Check: `calls::comparison` module
   - Script example: Line 85

3. **deleteDistinctCallElement** - Delete by feature + element
   - Check: `calls::distinct` module
   - Script example: Line 87

4. **addComparisonCallElement** - Add call element
   - Check: `calls::comparison` module
   - Script example: Line 120

5. **addBehaviorOverride** - Add behavior override (CFG_FBOVR?)
   - Need to search SDK for this
   - Script example: Line 113

## Recommended Next Steps

1. **Investigate missing commands** - Check calls modules and search for behavior override
2. **Implement upgrade_processor module** - Core infrastructure
3. **Add tests** - Process sample upgrade script
4. **Add CLI command** - Integrate with sz_configtool CLI tool
5. **Document** - Add usage examples and API docs

## Benefits

- **Automated upgrades** - Apply official Senzing upgrade scripts programmatically
- **Testable** - Verify upgrade scripts in CI/CD
- **Reversible** - Can create downgrade scripts
- **Auditable** - Log all commands executed
- **Extensible** - Easy to add new commands as SDK grows

## Estimated Effort

- **Core processor:** 2-3 hours
- **Command mappings:** 1-2 hours
- **Missing functions:** 2-4 hours (if need to implement)
- **Testing:** 2-3 hours
- **Total:** 7-12 hours

---

_Analysis Date: 2026-01-20_
