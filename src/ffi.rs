//! C FFI bindings for sz_configtool_lib
//!
//! This module provides a C-compatible interface to the Rust sz_configtool_lib library.
//! Functions follow the pattern established by Senzing's SzLang_helpers.h.
//!
//! # Safety
//!
//! All functions in this module that accept raw pointers require:
//! - Non-null pointers (null pointers will return error codes)
//! - Valid UTF-8 strings for string parameters
//! - Memory allocated by this library must be freed with `SzConfigTool_free`

#![allow(clippy::missing_safety_doc)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Mutex;

use crate::error::SzConfigError;

// Thread-safe error storage
static LAST_ERROR: Mutex<Option<String>> = Mutex::new(None);
static LAST_ERROR_CODE: Mutex<i64> = Mutex::new(0);

/// Platform-specific export macro
#[cfg(target_os = "windows")]
const _DLEXPORT: &str = "__declspec(dllexport)";

#[cfg(not(target_os = "windows"))]
const _DLEXPORT: &str = "";

// ============================================================================
// Result Structures (matching SzLang_helpers.h pattern)
// ============================================================================

/// Result structure for operations that return modified configuration JSON
#[repr(C)]
#[allow(non_snake_case)] // Match C convention from SzHelpers
pub struct SzConfigTool_result {
    /// Modified configuration JSON (caller must free with SzConfigTool_free)
    pub response: *mut c_char,
    /// Return code: 0 = success, negative = error (matches SzHelpers convention)
    pub returnCode: i64,
}

// ============================================================================
// Infrastructure Functions
// ============================================================================

/// Free memory allocated by this library
///
/// # Safety
/// ptr must be a valid pointer previously returned by this library, or null
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_free(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            drop(CString::from_raw(ptr));
        }
    }
}

/// Get the last error message
///
/// # Returns
/// Pointer to error string (do not free), or null if no error
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getLastError() -> *const c_char {
    let error = LAST_ERROR.lock().unwrap();
    match error.as_ref() {
        Some(err) => err.as_ptr() as *const c_char,
        None => std::ptr::null(),
    }
}

/// Get the last error code
///
/// # Returns
/// Error code (0 = no error, negative = error)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getLastErrorCode() -> i64 {
    *LAST_ERROR_CODE.lock().unwrap()
}

/// Clear the last error
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_clearLastError() {
    *LAST_ERROR.lock().unwrap() = None;
    *LAST_ERROR_CODE.lock().unwrap() = 0;
}

// ============================================================================
// Helper Macros for Error Handling
// ============================================================================

macro_rules! handle_result {
    ($result:expr) => {
        match $result {
            Ok(json) => {
                SzConfigTool_clearLastError();
                match CString::new(json) {
                    Ok(c_str) => SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    },
                    Err(e) => {
                        set_error(format!("Failed to convert result to C string: {}", e), -1);
                        SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -1,
                        }
                    }
                }
            }
            Err(e) => {
                set_error(format!("{}", e), -2);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                }
            }
        }
    };
}

fn set_error(msg: String, code: i64) {
    *LAST_ERROR.lock().unwrap() = Some(msg);
    *LAST_ERROR_CODE.lock().unwrap() = code;
}

fn clear_error() {
    *LAST_ERROR.lock().unwrap() = None;
    *LAST_ERROR_CODE.lock().unwrap() = 0;
}

// ============================================================================
// Data Source Functions
// ============================================================================

/// Add a data source to the configuration
///
/// # Safety
/// configJson and dataSourceCode must be valid null-terminated C strings
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_addDataSource(
    config_json: *const c_char,
    data_source_code: *const c_char,
) -> SzConfigTool_result {
    if config_json.is_null() || data_source_code.is_null() {
        set_error("Null pointer provided".to_string(), -1);
        return SzConfigTool_result {
            response: std::ptr::null_mut(),
            returnCode: -1,
        };
    }

    let config = match unsafe { CStr::from_ptr(config_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in configJson: {}", e), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
    };

    let ds_code = match unsafe { CStr::from_ptr(data_source_code) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in dataSourceCode: {}", e), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
    };

    // Use default values for optional parameters (matches Python defaults)
    let result = crate::datasources::add_data_source(
        config,
        crate::datasources::AddDataSourceParams {
            code: ds_code,
            ..Default::default()
        },
    );
    handle_result!(result)
}

/// Delete a data source from the configuration
///
/// # Safety
/// configJson and dataSourceCode must be valid null-terminated C strings
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_deleteDataSource(
    config_json: *const c_char,
    data_source_code: *const c_char,
) -> SzConfigTool_result {
    if config_json.is_null() || data_source_code.is_null() {
        set_error("Null pointer provided".to_string(), -1);
        return SzConfigTool_result {
            response: std::ptr::null_mut(),
            returnCode: -1,
        };
    }

    let config = match unsafe { CStr::from_ptr(config_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in configJson: {}", e), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
    };

    let ds_code = match unsafe { CStr::from_ptr(data_source_code) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in dataSourceCode: {}", e), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
    };

    let result = crate::datasources::delete_data_source(config, ds_code);
    handle_result!(result)
}

/// List all data sources in the configuration (returns JSON array string)
///
/// # Safety
/// configJson must be a valid null-terminated C string
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_listDataSources(
    config_json: *const c_char,
) -> SzConfigTool_result {
    if config_json.is_null() {
        set_error("Null pointer provided".to_string(), -1);
        return SzConfigTool_result {
            response: std::ptr::null_mut(),
            returnCode: -1,
        };
    }

    let config = match unsafe { CStr::from_ptr(config_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in configJson: {}", e), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
    };

    let result = crate::datasources::list_data_sources(config).and_then(|vec| {
        serde_json::to_string(&vec).map_err(|e| SzConfigError::JsonParse(e.to_string()))
    });
    handle_result!(result)
}

// ============================================================================
// Attribute Functions
// ============================================================================

/// Add an attribute to the configuration
///
/// # Safety
/// All string parameters must be valid null-terminated C strings
/// Optional parameters can be null
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_addAttribute(
    config_json: *const c_char,
    attribute_code: *const c_char,
    feature_code: *const c_char,
    element_code: *const c_char,
    attr_class: *const c_char,
    default_value: *const c_char,
    internal: *const c_char,
    required: *const c_char,
) -> SzConfigTool_result {
    if config_json.is_null()
        || attribute_code.is_null()
        || feature_code.is_null()
        || element_code.is_null()
        || attr_class.is_null()
    {
        set_error("Required parameter is null".to_string(), -1);
        return SzConfigTool_result {
            response: std::ptr::null_mut(),
            returnCode: -1,
        };
    }

    let config = match unsafe { CStr::from_ptr(config_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in configJson: {}", e), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
    };

    let attr_code = match unsafe { CStr::from_ptr(attribute_code) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in attributeCode: {}", e), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
    };

    let feat_code = match unsafe { CStr::from_ptr(feature_code) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in featureCode: {}", e), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
    };

    let elem_code = match unsafe { CStr::from_ptr(element_code) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in elementCode: {}", e), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
    };

    let class = match unsafe { CStr::from_ptr(attr_class) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in attrClass: {}", e), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
    };

    let def_val = if default_value.is_null() {
        None
    } else {
        match unsafe { CStr::from_ptr(default_value) }.to_str() {
            Ok(s) => Some(s),
            Err(e) => {
                set_error(format!("Invalid UTF-8 in defaultValue: {}", e), -1);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -1,
                };
            }
        }
    };

    let int_val = if internal.is_null() {
        None
    } else {
        match unsafe { CStr::from_ptr(internal) }.to_str() {
            Ok(s) => Some(s),
            Err(e) => {
                set_error(format!("Invalid UTF-8 in internal: {}", e), -1);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -1,
                };
            }
        }
    };

    let req_val = if required.is_null() {
        None
    } else {
        match unsafe { CStr::from_ptr(required) }.to_str() {
            Ok(s) => Some(s),
            Err(e) => {
                set_error(format!("Invalid UTF-8 in required: {}", e), -1);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -1,
                };
            }
        }
    };

    let result = crate::attributes::add_attribute(config, crate::attributes::AddAttributeParams {
                    attribute: attr_code,
            feature: feat_code,
            element: elem_code,
            class,
            default_value: def_val,
            internal: int_val,
            required: req_val,
        },
    )
    .map(|(json, _item)| json);

    handle_result!(result)
}

/// Delete an attribute from the configuration
///
/// # Safety
/// configJson and attributeCode must be valid null-terminated C strings
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_deleteAttribute(
    config_json: *const c_char,
    attribute_code: *const c_char,
) -> SzConfigTool_result {
    if config_json.is_null() || attribute_code.is_null() {
        set_error("Null pointer provided".to_string(), -1);
        return SzConfigTool_result {
            response: std::ptr::null_mut(),
            returnCode: -1,
        };
    }

    let config = match unsafe { CStr::from_ptr(config_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in configJson: {}", e), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
    };

    let attr_code = match unsafe { CStr::from_ptr(attribute_code) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in attributeCode: {}", e), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
    };

    let result = crate::attributes::delete_attribute(config, attr_code);
    handle_result!(result)
}

/// Get an attribute from the configuration (returns JSON object string)
///
/// # Safety
/// configJson and attributeCode must be valid null-terminated C strings
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_getAttribute(
    config_json: *const c_char,
    attribute_code: *const c_char,
) -> SzConfigTool_result {
    if config_json.is_null() || attribute_code.is_null() {
        set_error("Null pointer provided".to_string(), -1);
        return SzConfigTool_result {
            response: std::ptr::null_mut(),
            returnCode: -1,
        };
    }

    let config = match unsafe { CStr::from_ptr(config_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in configJson: {}", e), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
    };

    let attr_code = match unsafe { CStr::from_ptr(attribute_code) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in attributeCode: {}", e), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
    };

    let result = crate::attributes::get_attribute(config, attr_code).and_then(|val| {
        serde_json::to_string(&val).map_err(|e| SzConfigError::JsonParse(e.to_string()))
    });
    handle_result!(result)
}

/// List all attributes in the configuration (returns JSON array string)
///
/// # Safety
/// configJson must be a valid null-terminated C string
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_listAttributes(
    config_json: *const c_char,
) -> SzConfigTool_result {
    if config_json.is_null() {
        set_error("Null pointer provided".to_string(), -1);
        return SzConfigTool_result {
            response: std::ptr::null_mut(),
            returnCode: -1,
        };
    }

    let config = match unsafe { CStr::from_ptr(config_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in configJson: {}", e), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
    };

    let result = crate::attributes::list_attributes(config).and_then(|vec| {
        serde_json::to_string(&vec).map_err(|e| SzConfigError::JsonParse(e.to_string()))
    });
    handle_result!(result)
}

/// Set (update) an attribute's properties
///
/// # Safety
/// configJson, attributeCode, and updatesJson must be valid null-terminated C strings
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_setAttribute(
    config_json: *const c_char,
    attribute_code: *const c_char,
    updates_json: *const c_char,
) -> SzConfigTool_result {
    if config_json.is_null() || attribute_code.is_null() || updates_json.is_null() {
        set_error("Null pointer provided".to_string(), -1);
        return SzConfigTool_result {
            response: std::ptr::null_mut(),
            returnCode: -1,
        };
    }

    let config = match unsafe { CStr::from_ptr(config_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in configJson: {}", e), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
    };

    let attr_code = match unsafe { CStr::from_ptr(attribute_code) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in attributeCode: {}", e), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
    };

    let updates_str = match unsafe { CStr::from_ptr(updates_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in updatesJson: {}", e), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
    };

    let updates: serde_json::Value = match serde_json::from_str(updates_str) {
        Ok(v) => v,
        Err(e) => {
            set_error(format!("Invalid JSON in updatesJson: {}", e), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
    };

    // Build params with attribute from parameter and optional updates from JSON
    let params = crate::attributes::SetAttributeParams {
        attribute: attr_code,
        internal: updates.get("internal").and_then(|v| v.as_str()),
        required: updates.get("required").and_then(|v| v.as_str()),
        default_value: updates.get("default").and_then(|v| v.as_str()),
    };

    handle_result!(crate::attributes::set_attribute(config, params))
}

// ============================================================================
// Feature Functions (Phase 1: read-only)
// ============================================================================
// Note: add_feature/delete_feature require 15 parameters and JSON objects.
// These will be added in Phase 2.

/// Get a feature from the configuration (returns JSON object string)
///
/// # Safety
/// configJson and featureCode must be valid null-terminated C strings
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_getFeature(
    config_json: *const c_char,
    feature_code: *const c_char,
) -> SzConfigTool_result {
    if config_json.is_null() || feature_code.is_null() {
        set_error("Null pointer provided".to_string(), -1);
        return SzConfigTool_result {
            response: std::ptr::null_mut(),
            returnCode: -1,
        };
    }

    let config = match unsafe { CStr::from_ptr(config_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in configJson: {}", e), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
    };

    let feat_code = match unsafe { CStr::from_ptr(feature_code) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in featureCode: {}", e), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
    };

    let result = crate::features::get_feature(config, feat_code).and_then(|val| {
        serde_json::to_string(&val).map_err(|e| SzConfigError::JsonParse(e.to_string()))
    });
    handle_result!(result)
}

/// List all features in the configuration (returns JSON array string)
///
/// # Safety
/// configJson must be a valid null-terminated C string
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_listFeatures(
    config_json: *const c_char,
) -> SzConfigTool_result {
    if config_json.is_null() {
        set_error("Null pointer provided".to_string(), -1);
        return SzConfigTool_result {
            response: std::ptr::null_mut(),
            returnCode: -1,
        };
    }

    let config = match unsafe { CStr::from_ptr(config_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in configJson: {}", e), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
    };

    let result = crate::features::list_features(config).and_then(|vec| {
        serde_json::to_string(&vec).map_err(|e| SzConfigError::JsonParse(e.to_string()))
    });
    handle_result!(result)
}

// ============================================================================
// Element Functions (Phase 1: read-only)
// ============================================================================
// Note: add_element/delete_element require JSON object parameters.
// These will be added in Phase 2.

/// Get an element from the configuration (returns JSON object string)
///
/// # Safety
/// configJson and elementCode must be valid null-terminated C strings
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_getElement(
    config_json: *const c_char,
    element_code: *const c_char,
) -> SzConfigTool_result {
    if config_json.is_null() || element_code.is_null() {
        set_error("Null pointer provided".to_string(), -1);
        return SzConfigTool_result {
            response: std::ptr::null_mut(),
            returnCode: -1,
        };
    }

    let config = match unsafe { CStr::from_ptr(config_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in configJson: {}", e), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
    };

    let elem_code = match unsafe { CStr::from_ptr(element_code) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in elementCode: {}", e), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
    };

    let result = crate::elements::get_element(config, elem_code).and_then(|val| {
        serde_json::to_string(&val).map_err(|e| SzConfigError::JsonParse(e.to_string()))
    });
    handle_result!(result)
}

/// List all elements in the configuration (returns JSON array string)
///
/// # Safety
/// configJson must be a valid null-terminated C string
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_listElements(
    config_json: *const c_char,
) -> SzConfigTool_result {
    if config_json.is_null() {
        set_error("Null pointer provided".to_string(), -1);
        return SzConfigTool_result {
            response: std::ptr::null_mut(),
            returnCode: -1,
        };
    }

    let config = match unsafe { CStr::from_ptr(config_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in configJson: {}", e), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
    };

    let result = crate::elements::list_elements(config).and_then(|vec| {
        serde_json::to_string(&vec).map_err(|e| SzConfigError::JsonParse(e.to_string()))
    });
    handle_result!(result)
}

/// Set/update a fragment with JSON parameters
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_setFragmentWithJson(
    config_json: *const c_char,
    fragment_code: *const c_char,
    fragment_config_json: *const c_char,
) -> SzConfigTool_result {
    if config_json.is_null() || fragment_code.is_null() || fragment_config_json.is_null() {
        set_error("Required parameter is null".to_string(), -1);
        return SzConfigTool_result {
            response: std::ptr::null_mut(),
            returnCode: -1,
        };
    }

    let config = match unsafe { CStr::from_ptr(config_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    let code = match unsafe { CStr::from_ptr(fragment_code) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in fragment_code: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    let fragment_str = match unsafe { CStr::from_ptr(fragment_config_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in fragment_config_json: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    let fragment_config: serde_json::Value = match serde_json::from_str(fragment_str) {
        Ok(v) => v,
        Err(e) => {
            set_error(format!("Failed to parse fragment_config_json: {}", e), -3);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -3,
            };
        }
    };

    let result = crate::fragments::set_fragment(config, code, &fragment_config);
    handle_result!(result)
}

/// Clone a generic plan
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_cloneGenericPlan(
    config_json: *const c_char,
    source_gplan_code: *const c_char,
    new_gplan_code: *const c_char,
    new_gplan_desc: *const c_char,
) -> SzConfigTool_result {
    if config_json.is_null() || source_gplan_code.is_null() || new_gplan_code.is_null() {
        set_error("Required parameter is null".to_string(), -1);
        return SzConfigTool_result {
            response: std::ptr::null_mut(),
            returnCode: -1,
        };
    }

    let config = match unsafe { CStr::from_ptr(config_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    let source_code = match unsafe { CStr::from_ptr(source_gplan_code) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in source_gplan_code: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    let new_code = match unsafe { CStr::from_ptr(new_gplan_code) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in new_gplan_code: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    let new_desc = if new_gplan_desc.is_null() {
        None
    } else {
        match unsafe { CStr::from_ptr(new_gplan_desc) }.to_str() {
            Ok(s) => Some(s),
            Err(e) => {
                set_error(format!("Invalid UTF-8 in new_gplan_desc: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::generic_plans::clone_generic_plan(config, source_code, new_code, new_desc) {
        Ok((modified_config, _gplan_id)) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to convert result: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Set/update a generic plan
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_setGenericPlan(
    config_json: *const c_char,
    gplan_code: *const c_char,
    gplan_desc: *const c_char,
) -> SzConfigTool_result {
    if config_json.is_null() || gplan_code.is_null() || gplan_desc.is_null() {
        set_error("Required parameter is null".to_string(), -1);
        return SzConfigTool_result {
            response: std::ptr::null_mut(),
            returnCode: -1,
        };
    }

    let config = match unsafe { CStr::from_ptr(config_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    let code = match unsafe { CStr::from_ptr(gplan_code) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in gplan_code: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    let desc = match unsafe { CStr::from_ptr(gplan_desc) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in gplan_desc: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    match crate::generic_plans::set_generic_plan(config, code, desc) {
        Ok((modified_config, _gplan_id, _was_created)) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to convert result: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// List generic plans
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_listGenericPlans(
    config_json: *const c_char,
    filter: *const c_char,
) -> SzConfigTool_result {
    if config_json.is_null() {
        set_error("Required parameter is null".to_string(), -1);
        return SzConfigTool_result {
            response: std::ptr::null_mut(),
            returnCode: -1,
        };
    }

    let config = match unsafe { CStr::from_ptr(config_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    let filter_opt = if filter.is_null() {
        None
    } else {
        match unsafe { CStr::from_ptr(filter) }.to_str() {
            Ok(s) => Some(s),
            Err(e) => {
                set_error(format!("Invalid UTF-8 in filter: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::generic_plans::list_generic_plans(config, filter_opt) {
        Ok(list) => match serde_json::to_string(&list) {
            Ok(json_str) => match CString::new(json_str) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to convert result: {}", e), -4);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -4,
                    }
                }
            },
            Err(e) => {
                set_error(format!("Failed to serialize result: {}", e), -3);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Add a name to the SSN_LAST4 hash
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_addToSsnLast4Hash(
    config_json: *const c_char,
    name: *const c_char,
) -> SzConfigTool_result {
    if config_json.is_null() || name.is_null() {
        set_error("Required parameter is null".to_string(), -1);
        return SzConfigTool_result {
            response: std::ptr::null_mut(),
            returnCode: -1,
        };
    }

    let config = match unsafe { CStr::from_ptr(config_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    let name_str = match unsafe { CStr::from_ptr(name) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in name: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    let result = crate::hashes::add_to_ssn_last4_hash(config, name_str);
    handle_result!(result)
}

/// Delete a name from the SSN_LAST4 hash
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_deleteFromSsnLast4Hash(
    config_json: *const c_char,
    name: *const c_char,
) -> SzConfigTool_result {
    if config_json.is_null() || name.is_null() {
        set_error("Required parameter is null".to_string(), -1);
        return SzConfigTool_result {
            response: std::ptr::null_mut(),
            returnCode: -1,
        };
    }

    let config = match unsafe { CStr::from_ptr(config_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    let name_str = match unsafe { CStr::from_ptr(name) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in name: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    let result = crate::hashes::delete_from_ssn_last4_hash(config, name_str);
    handle_result!(result)
}

/// Get a threshold by ID
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_getThreshold(
    config_json: *const c_char,
    threshold_id: i64,
) -> SzConfigTool_result {
    if config_json.is_null() {
        set_error("Required parameter is null".to_string(), -1);
        return SzConfigTool_result {
            response: std::ptr::null_mut(),
            returnCode: -1,
        };
    }

    let config = match unsafe { CStr::from_ptr(config_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    match crate::thresholds::get_threshold(config, threshold_id) {
        Ok(record) => match serde_json::to_string(&record) {
            Ok(json_str) => match CString::new(json_str) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to convert result: {}", e), -4);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -4,
                    }
                }
            },
            Err(e) => {
                set_error(format!("Failed to serialize result: {}", e), -3);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// List system parameters
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_listSystemParameters(
    config_json: *const c_char,
) -> SzConfigTool_result {
    if config_json.is_null() {
        set_error("Required parameter is null".to_string(), -1);
        return SzConfigTool_result {
            response: std::ptr::null_mut(),
            returnCode: -1,
        };
    }

    let config = match unsafe { CStr::from_ptr(config_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    match crate::system_params::list_system_parameters(config) {
        Ok(params) => match serde_json::to_string(&params) {
            Ok(json_str) => match CString::new(json_str) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to convert result: {}", e), -4);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -4,
                    }
                }
            },
            Err(e) => {
                set_error(format!("Failed to serialize result: {}", e), -3);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Set a system parameter with JSON value
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_setSystemParameterWithJson(
    config_json: *const c_char,
    parameter_name: *const c_char,
    parameter_value_json: *const c_char,
) -> SzConfigTool_result {
    if config_json.is_null() || parameter_name.is_null() || parameter_value_json.is_null() {
        set_error("Required parameter is null".to_string(), -1);
        return SzConfigTool_result {
            response: std::ptr::null_mut(),
            returnCode: -1,
        };
    }

    let config = match unsafe { CStr::from_ptr(config_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    let param_name = match unsafe { CStr::from_ptr(parameter_name) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in parameter_name: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    let value_str = match unsafe { CStr::from_ptr(parameter_value_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in parameter_value_json: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    let param_value: serde_json::Value = match serde_json::from_str(value_str) {
        Ok(v) => v,
        Err(e) => {
            set_error(format!("Failed to parse parameter_value_json: {}", e), -3);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -3,
            };
        }
    };

    let result = crate::system_params::set_system_parameter(config, param_name, &param_value);
    handle_result!(result)
}

/// Get the configuration version
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_getVersion(
    config_json: *const c_char,
) -> SzConfigTool_result {
    if config_json.is_null() {
        set_error("Required parameter is null".to_string(), -1);
        return SzConfigTool_result {
            response: std::ptr::null_mut(),
            returnCode: -1,
        };
    }

    let config = match unsafe { CStr::from_ptr(config_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    match crate::versioning::get_version(config) {
        Ok(version) => match CString::new(version) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to convert result: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Get the compatibility version
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_getCompatibilityVersion(
    config_json: *const c_char,
) -> SzConfigTool_result {
    if config_json.is_null() {
        set_error("Required parameter is null".to_string(), -1);
        return SzConfigTool_result {
            response: std::ptr::null_mut(),
            returnCode: -1,
        };
    }

    let config = match unsafe { CStr::from_ptr(config_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    match crate::versioning::get_compatibility_version(config) {
        Ok(version) => match CString::new(version) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to convert result: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Update the compatibility version
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_updateCompatibilityVersion(
    config_json: *const c_char,
    new_version: *const c_char,
) -> SzConfigTool_result {
    if config_json.is_null() || new_version.is_null() {
        set_error("Required parameter is null".to_string(), -1);
        return SzConfigTool_result {
            response: std::ptr::null_mut(),
            returnCode: -1,
        };
    }

    let config = match unsafe { CStr::from_ptr(config_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    let version = match unsafe { CStr::from_ptr(new_version) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in new_version: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    let result = crate::versioning::update_compatibility_version(config, version);
    handle_result!(result)
}

/// Update the feature version
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_updateFeatureVersion(
    config_json: *const c_char,
    version: *const c_char,
) -> SzConfigTool_result {
    if config_json.is_null() || version.is_null() {
        set_error("Required parameter is null".to_string(), -1);
        return SzConfigTool_result {
            response: std::ptr::null_mut(),
            returnCode: -1,
        };
    }

    let config = match unsafe { CStr::from_ptr(config_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    let version_str = match unsafe { CStr::from_ptr(version) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in version: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    let result = crate::features::update_feature_version(config, version_str);
    handle_result!(result)
}

/// Verify compatibility version
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_verifyCompatibilityVersion(
    config_json: *const c_char,
    expected_version: *const c_char,
) -> SzConfigTool_result {
    if config_json.is_null() || expected_version.is_null() {
        set_error("Required parameter is null".to_string(), -1);
        return SzConfigTool_result {
            response: std::ptr::null_mut(),
            returnCode: -1,
        };
    }

    let config = match unsafe { CStr::from_ptr(config_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    let version = match unsafe { CStr::from_ptr(expected_version) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in expected_version: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    match crate::versioning::verify_compatibility_version(config, version) {
        Ok((message, _matches)) => match CString::new(message) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to convert result: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Add a config section
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_addConfigSection(
    config_json: *const c_char,
    section_name: *const c_char,
) -> SzConfigTool_result {
    if config_json.is_null() || section_name.is_null() {
        set_error("Required parameter is null".to_string(), -1);
        return SzConfigTool_result {
            response: std::ptr::null_mut(),
            returnCode: -1,
        };
    }

    let config = match unsafe { CStr::from_ptr(config_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    let section = match unsafe { CStr::from_ptr(section_name) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in section_name: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    let result = crate::config_sections::add_config_section(config, section);
    handle_result!(result)
}

/// Remove a config section
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_removeConfigSection(
    config_json: *const c_char,
    section_name: *const c_char,
) -> SzConfigTool_result {
    if config_json.is_null() || section_name.is_null() {
        set_error("Required parameter is null".to_string(), -1);
        return SzConfigTool_result {
            response: std::ptr::null_mut(),
            returnCode: -1,
        };
    }

    let config = match unsafe { CStr::from_ptr(config_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    let section = match unsafe { CStr::from_ptr(section_name) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in section_name: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    let result = crate::config_sections::remove_config_section(config, section);
    handle_result!(result)
}

/// Get a config section
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_getConfigSection(
    config_json: *const c_char,
    section_name: *const c_char,
    filter: *const c_char,
) -> SzConfigTool_result {
    if config_json.is_null() || section_name.is_null() {
        set_error("Required parameter is null".to_string(), -1);
        return SzConfigTool_result {
            response: std::ptr::null_mut(),
            returnCode: -1,
        };
    }

    let config = match unsafe { CStr::from_ptr(config_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    let section = match unsafe { CStr::from_ptr(section_name) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in section_name: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    let filter_opt = if filter.is_null() {
        None
    } else {
        match unsafe { CStr::from_ptr(filter) }.to_str() {
            Ok(s) => Some(s),
            Err(e) => {
                set_error(format!("Invalid UTF-8 in filter: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::config_sections::get_config_section(config, section, filter_opt) {
        Ok(section_data) => match serde_json::to_string(&section_data) {
            Ok(json_str) => match CString::new(json_str) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to convert result: {}", e), -4);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -4,
                    }
                }
            },
            Err(e) => {
                set_error(format!("Failed to serialize result: {}", e), -3);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// List all config sections
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_listConfigSections(
    config_json: *const c_char,
) -> SzConfigTool_result {
    if config_json.is_null() {
        set_error("Required parameter is null".to_string(), -1);
        return SzConfigTool_result {
            response: std::ptr::null_mut(),
            returnCode: -1,
        };
    }

    let config = match unsafe { CStr::from_ptr(config_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    match crate::config_sections::list_config_sections(config) {
        Ok(sections) => match serde_json::to_string(&sections) {
            Ok(json_str) => match CString::new(json_str) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to convert result: {}", e), -4);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -4,
                    }
                }
            },
            Err(e) => {
                set_error(format!("Failed to serialize result: {}", e), -3);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Add a field to a config section (returns tuple)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_addConfigSectionField(
    config_json: *const c_char,
    section_name: *const c_char,
    field_name: *const c_char,
    field_value_json: *const c_char,
) -> SzConfigTool_result {
    if config_json.is_null()
        || section_name.is_null()
        || field_name.is_null()
        || field_value_json.is_null()
    {
        set_error("Required parameter is null".to_string(), -1);
        return SzConfigTool_result {
            response: std::ptr::null_mut(),
            returnCode: -1,
        };
    }

    let config = match unsafe { CStr::from_ptr(config_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    let section = match unsafe { CStr::from_ptr(section_name) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in section_name: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    let field = match unsafe { CStr::from_ptr(field_name) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in field_name: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    let value_str = match unsafe { CStr::from_ptr(field_value_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in field_value_json: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    let field_value: serde_json::Value = match serde_json::from_str(value_str) {
        Ok(v) => v,
        Err(e) => {
            set_error(format!("Failed to parse field_value_json: {}", e), -3);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -3,
            };
        }
    };

    match crate::config_sections::add_config_section_field(config, section, field, &field_value) {
        Ok((modified_config, _count)) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to convert result: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Remove a field from a config section (returns tuple)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_removeConfigSectionField(
    config_json: *const c_char,
    section_name: *const c_char,
    field_name: *const c_char,
) -> SzConfigTool_result {
    if config_json.is_null() || section_name.is_null() || field_name.is_null() {
        set_error("Required parameter is null".to_string(), -1);
        return SzConfigTool_result {
            response: std::ptr::null_mut(),
            returnCode: -1,
        };
    }

    let config = match unsafe { CStr::from_ptr(config_json) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    let section = match unsafe { CStr::from_ptr(section_name) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in section_name: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    let field = match unsafe { CStr::from_ptr(field_name) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error(format!("Invalid UTF-8 in field_name: {}", e), -2);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -2,
            };
        }
    };

    match crate::config_sections::remove_config_section_field(config, section, field) {
        Ok((modified_config, _count)) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to convert result: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/* ============================================================================
 * Rule Functions (Batch 4)
 * ============================================================================ */

/// Add a rule with JSON configuration
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_addRule(
    config_json: *const c_char,
    rule_json: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let rule_config = unsafe {
        if rule_json.is_null() {
            set_error("rule_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(rule_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in rule_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let rule_value: serde_json::Value = match serde_json::from_str(rule_config) {
        Ok(v) => v,
        Err(e) => {
            set_error(format!("Invalid JSON in rule_json: {}", e), -3);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -3,
            };
        }
    };

    match crate::rules::add_rule(config, &rule_value) {
        Ok((modified_config, _rule_id)) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to convert result: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Delete a rule
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteRule(
    config_json: *const c_char,
    rule_code: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let code = unsafe {
        if rule_code.is_null() {
            set_error("rule_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(rule_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in rule_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    handle_result!(crate::rules::delete_rule(config, code))
}

/// Get a rule by code or ID
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getRule(
    config_json: *const c_char,
    code_or_id: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let identifier = unsafe {
        if code_or_id.is_null() {
            set_error("code_or_id is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(code_or_id).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in code_or_id: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::rules::get_rule(config, identifier) {
        Ok(rule_json_value) => {
            let rule_str = serde_json::to_string(&rule_json_value)
                .unwrap_or_else(|e| format!("{{\"error\": \"Failed to serialize: {}\"}}", e));
            match CString::new(rule_str) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to convert result: {}", e), -4);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -4,
                    }
                }
            }
        }
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// List all rules
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_listRules(config_json: *const c_char) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::rules::list_rules(config) {
        Ok(rules_vec) => {
            let rules_str = serde_json::to_string(&rules_vec)
                .unwrap_or_else(|e| format!("{{\"error\": \"Failed to serialize: {}\"}}", e));
            match CString::new(rules_str) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to convert result: {}", e), -4);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -4,
                    }
                }
            }
        }
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Set/update a rule with JSON configuration
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_setRule(
    config_json: *const c_char,
    rule_code: *const c_char,
    rule_json: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let code = unsafe {
        if rule_code.is_null() {
            set_error("rule_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(rule_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in rule_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let rule_config = unsafe {
        if rule_json.is_null() {
            set_error("rule_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(rule_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in rule_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let rule_value: serde_json::Value = match serde_json::from_str(rule_config) {
        Ok(v) => v,
        Err(e) => {
            set_error(format!("Invalid JSON in rule_json: {}", e), -3);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -3,
            };
        }
    };

    // Build params from code and JSON config
    let params = crate::rules::SetRuleParams {
        code,
        resolve: rule_value.get("resolve").and_then(|v| v.as_str())
            .or_else(|| rule_value.get("RESOLVE").and_then(|v| v.as_str())),
        relate: rule_value.get("relate").and_then(|v| v.as_str())
            .or_else(|| rule_value.get("RELATE").and_then(|v| v.as_str())),
        rtype_id: rule_value.get("rtypeId").and_then(|v| v.as_i64())
            .or_else(|| rule_value.get("RTYPE_ID").and_then(|v| v.as_i64())),
    };

    handle_result!(crate::rules::set_rule(config, params))
}

/* ============================================================================
 * Standardize Function Operations (Batch 5a)
 * ============================================================================ */

/// Add a standardize function
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_addStandardizeFunction(
    config_json: *const c_char,
    sfunc_code: *const c_char,
    connect_str: *const c_char,
    sfunc_desc: *const c_char,
    language: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let code = unsafe {
        if sfunc_code.is_null() {
            set_error("sfunc_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(sfunc_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in sfunc_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let conn = unsafe {
        if connect_str.is_null() {
            set_error("connect_str is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(connect_str).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in connect_str: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let desc_opt = if sfunc_desc.is_null() {
        None
    } else {
        unsafe {
            match CStr::from_ptr(sfunc_desc).to_str() {
                Ok(s) if !s.is_empty() => Some(s),
                Ok(_) => None,
                Err(e) => {
                    set_error(format!("Invalid UTF-8 in sfunc_desc: {}", e), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        }
    };

    let lang_opt = if language.is_null() {
        None
    } else {
        unsafe {
            match CStr::from_ptr(language).to_str() {
                Ok(s) if !s.is_empty() => Some(s),
                Ok(_) => None,
                Err(e) => {
                    set_error(format!("Invalid UTF-8 in language: {}", e), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        }
    };

    match crate::functions::standardize::add_standardize_function(
        config,
        code,
        crate::functions::standardize::AddStandardizeFunctionParams {
            connect_str: conn,
            description: desc_opt,
            language: lang_opt,
        },
    ) {
        Ok((modified_config, _record)) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to convert result: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Delete a standardize function
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteStandardizeFunction(
    config_json: *const c_char,
    sfunc_code: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let code = unsafe {
        if sfunc_code.is_null() {
            set_error("sfunc_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(sfunc_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in sfunc_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::functions::standardize::delete_standardize_function(config, code) {
        Ok((modified_config, _record)) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to convert result: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Get a standardize function
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getStandardizeFunction(
    config_json: *const c_char,
    sfunc_code: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let code = unsafe {
        if sfunc_code.is_null() {
            set_error("sfunc_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(sfunc_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in sfunc_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::functions::standardize::get_standardize_function(config, code) {
        Ok(value) => {
            let json_str = serde_json::to_string(&value)
                .unwrap_or_else(|e| format!("{{\"error\": \"Failed to serialize: {}\"}}", e));
            match CString::new(json_str) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to convert result: {}", e), -4);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -4,
                    }
                }
            }
        }
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// List all standardize functions
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_listStandardizeFunctions(
    config_json: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::functions::standardize::list_standardize_functions(config) {
        Ok(vec) => {
            let json_str = serde_json::to_string(&vec)
                .unwrap_or_else(|e| format!("{{\"error\": \"Failed to serialize: {}\"}}", e));
            match CString::new(json_str) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to convert result: {}", e), -4);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -4,
                    }
                }
            }
        }
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Set (update) a standardize function
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_setStandardizeFunction(
    config_json: *const c_char,
    sfunc_code: *const c_char,
    connect_str: *const c_char,
    sfunc_desc: *const c_char,
    language: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let code = unsafe {
        if sfunc_code.is_null() {
            set_error("sfunc_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(sfunc_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in sfunc_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let conn_opt = if connect_str.is_null() {
        None
    } else {
        unsafe {
            match CStr::from_ptr(connect_str).to_str() {
                Ok(s) if !s.is_empty() => Some(s),
                Ok(_) => None,
                Err(e) => {
                    set_error(format!("Invalid UTF-8 in connect_str: {}", e), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        }
    };

    let desc_opt = if sfunc_desc.is_null() {
        None
    } else {
        unsafe {
            match CStr::from_ptr(sfunc_desc).to_str() {
                Ok(s) if !s.is_empty() => Some(s),
                Ok(_) => None,
                Err(e) => {
                    set_error(format!("Invalid UTF-8 in sfunc_desc: {}", e), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        }
    };

    let lang_opt = if language.is_null() {
        None
    } else {
        unsafe {
            match CStr::from_ptr(language).to_str() {
                Ok(s) if !s.is_empty() => Some(s),
                Ok(_) => None,
                Err(e) => {
                    set_error(format!("Invalid UTF-8 in language: {}", e), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        }
    };

    match crate::functions::standardize::set_standardize_function(
        config,
        code,
        crate::functions::standardize::SetStandardizeFunctionParams {
            connect_str: conn_opt,
            description: desc_opt,
            language: lang_opt,
        },
    ) {
        Ok((modified_config, _record)) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to convert result: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/* ============================================================================
 * Expression Function Operations (Batch 5b)
 * ============================================================================ */

/// Add an expression function
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_addExpressionFunction(
    config_json: *const c_char,
    efunc_code: *const c_char,
    connect_str: *const c_char,
    efunc_desc: *const c_char,
    language: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let code = unsafe {
        if efunc_code.is_null() {
            set_error("efunc_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(efunc_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in efunc_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let conn = unsafe {
        if connect_str.is_null() {
            set_error("connect_str is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(connect_str).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in connect_str: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let desc_opt = if efunc_desc.is_null() {
        None
    } else {
        unsafe {
            match CStr::from_ptr(efunc_desc).to_str() {
                Ok(s) if !s.is_empty() => Some(s),
                Ok(_) => None,
                Err(e) => {
                    set_error(format!("Invalid UTF-8 in efunc_desc: {}", e), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        }
    };

    let lang_opt = if language.is_null() {
        None
    } else {
        unsafe {
            match CStr::from_ptr(language).to_str() {
                Ok(s) if !s.is_empty() => Some(s),
                Ok(_) => None,
                Err(e) => {
                    set_error(format!("Invalid UTF-8 in language: {}", e), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        }
    };

    match crate::functions::expression::add_expression_function(
        config,
        code,
        crate::functions::expression::AddExpressionFunctionParams {
            connect_str: conn,
            description: desc_opt,
            language: lang_opt,
        },
    ) {
        Ok((modified_config, _)) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to convert result: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Delete an expression function
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteExpressionFunction(
    config_json: *const c_char,
    efunc_code: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let code = unsafe {
        if efunc_code.is_null() {
            set_error("efunc_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(efunc_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in efunc_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::functions::expression::delete_expression_function(config, code) {
        Ok((modified_config, _)) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to convert result: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Get an expression function
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getExpressionFunction(
    config_json: *const c_char,
    efunc_code: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let code = unsafe {
        if efunc_code.is_null() {
            set_error("efunc_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(efunc_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in efunc_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::functions::expression::get_expression_function(config, code) {
        Ok(value) => {
            let json_str = serde_json::to_string(&value)
                .unwrap_or_else(|e| format!("{{\"error\": \"Failed to serialize: {}\"}}", e));
            match CString::new(json_str) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to convert result: {}", e), -4);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -4,
                    }
                }
            }
        }
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// List all expression functions
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_listExpressionFunctions(
    config_json: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::functions::expression::list_expression_functions(config) {
        Ok(vec) => {
            let json_str = serde_json::to_string(&vec)
                .unwrap_or_else(|e| format!("{{\"error\": \"Failed to serialize: {}\"}}", e));
            match CString::new(json_str) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to convert result: {}", e), -4);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -4,
                    }
                }
            }
        }
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Set (update) an expression function
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_setExpressionFunction(
    config_json: *const c_char,
    efunc_code: *const c_char,
    connect_str: *const c_char,
    efunc_desc: *const c_char,
    language: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let code = unsafe {
        if efunc_code.is_null() {
            set_error("efunc_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(efunc_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in efunc_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let conn_opt = if connect_str.is_null() {
        None
    } else {
        unsafe {
            match CStr::from_ptr(connect_str).to_str() {
                Ok(s) if !s.is_empty() => Some(s),
                Ok(_) => None,
                Err(e) => {
                    set_error(format!("Invalid UTF-8 in connect_str: {}", e), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        }
    };

    let desc_opt = if efunc_desc.is_null() {
        None
    } else {
        unsafe {
            match CStr::from_ptr(efunc_desc).to_str() {
                Ok(s) if !s.is_empty() => Some(s),
                Ok(_) => None,
                Err(e) => {
                    set_error(format!("Invalid UTF-8 in efunc_desc: {}", e), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        }
    };

    let lang_opt = if language.is_null() {
        None
    } else {
        unsafe {
            match CStr::from_ptr(language).to_str() {
                Ok(s) if !s.is_empty() => Some(s),
                Ok(_) => None,
                Err(e) => {
                    set_error(format!("Invalid UTF-8 in language: {}", e), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        }
    };

    match crate::functions::expression::set_expression_function(
        config,
        code,
        crate::functions::expression::SetExpressionFunctionParams {
            connect_str: conn_opt,
            description: desc_opt,
            language: lang_opt,
        },
    ) {
        Ok((modified_config, _)) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to convert result: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/* ============================================================================
 * Comparison Function Operations (Batch 5c)
 * ============================================================================ */

/// Add a comparison function
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_addComparisonFunction(
    config_json: *const c_char,
    cfunc_code: *const c_char,
    connect_str: *const c_char,
    cfunc_desc: *const c_char,
    language: *const c_char,
    anon_support: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let code = unsafe {
        if cfunc_code.is_null() {
            set_error("cfunc_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(cfunc_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in cfunc_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let conn = unsafe {
        if connect_str.is_null() {
            set_error("connect_str is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(connect_str).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in connect_str: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let desc_opt = if cfunc_desc.is_null() {
        None
    } else {
        unsafe {
            match CStr::from_ptr(cfunc_desc).to_str() {
                Ok(s) if !s.is_empty() => Some(s),
                Ok(_) => None,
                Err(e) => {
                    set_error(format!("Invalid UTF-8 in cfunc_desc: {}", e), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        }
    };

    let lang_opt = if language.is_null() {
        None
    } else {
        unsafe {
            match CStr::from_ptr(language).to_str() {
                Ok(s) if !s.is_empty() => Some(s),
                Ok(_) => None,
                Err(e) => {
                    set_error(format!("Invalid UTF-8 in language: {}", e), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        }
    };

    let anon_opt = if anon_support.is_null() {
        None
    } else {
        unsafe {
            match CStr::from_ptr(anon_support).to_str() {
                Ok(s) if !s.is_empty() => Some(s),
                Ok(_) => None,
                Err(e) => {
                    set_error(format!("Invalid UTF-8 in anon_support: {}", e), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        }
    };

    match crate::functions::comparison::add_comparison_function(
        config,
        code,
        crate::functions::comparison::AddComparisonFunctionParams {
            connect_str: conn,
            description: desc_opt,
            language: lang_opt,
            anon_support: anon_opt,
        },
    ) {
        Ok((modified_config, _)) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to convert result: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Delete a comparison function
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteComparisonFunction(
    config_json: *const c_char,
    cfunc_code: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let code = unsafe {
        if cfunc_code.is_null() {
            set_error("cfunc_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(cfunc_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in cfunc_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::functions::comparison::delete_comparison_function(config, code) {
        Ok((modified_config, _)) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to convert result: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Get a comparison function
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getComparisonFunction(
    config_json: *const c_char,
    cfunc_code: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let code = unsafe {
        if cfunc_code.is_null() {
            set_error("cfunc_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(cfunc_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in cfunc_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::functions::comparison::get_comparison_function(config, code) {
        Ok(value) => {
            let json_str = serde_json::to_string(&value)
                .unwrap_or_else(|e| format!("{{\"error\": \"Failed to serialize: {}\"}}", e));
            match CString::new(json_str) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to convert result: {}", e), -4);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -4,
                    }
                }
            }
        }
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// List all comparison functions
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_listComparisonFunctions(
    config_json: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::functions::comparison::list_comparison_functions(config) {
        Ok(vec) => {
            let json_str = serde_json::to_string(&vec)
                .unwrap_or_else(|e| format!("{{\"error\": \"Failed to serialize: {}\"}}", e));
            match CString::new(json_str) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to convert result: {}", e), -4);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -4,
                    }
                }
            }
        }
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Set (update) a comparison function
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_setComparisonFunction(
    config_json: *const c_char,
    cfunc_code: *const c_char,
    connect_str: *const c_char,
    cfunc_desc: *const c_char,
    language: *const c_char,
    anon_support: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let code = unsafe {
        if cfunc_code.is_null() {
            set_error("cfunc_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(cfunc_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in cfunc_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let conn_opt = if connect_str.is_null() {
        None
    } else {
        unsafe {
            match CStr::from_ptr(connect_str).to_str() {
                Ok(s) if !s.is_empty() => Some(s),
                Ok(_) => None,
                Err(e) => {
                    set_error(format!("Invalid UTF-8 in connect_str: {}", e), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        }
    };

    let desc_opt = if cfunc_desc.is_null() {
        None
    } else {
        unsafe {
            match CStr::from_ptr(cfunc_desc).to_str() {
                Ok(s) if !s.is_empty() => Some(s),
                Ok(_) => None,
                Err(e) => {
                    set_error(format!("Invalid UTF-8 in cfunc_desc: {}", e), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        }
    };

    let lang_opt = if language.is_null() {
        None
    } else {
        unsafe {
            match CStr::from_ptr(language).to_str() {
                Ok(s) if !s.is_empty() => Some(s),
                Ok(_) => None,
                Err(e) => {
                    set_error(format!("Invalid UTF-8 in language: {}", e), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        }
    };

    let anon_opt = if anon_support.is_null() {
        None
    } else {
        unsafe {
            match CStr::from_ptr(anon_support).to_str() {
                Ok(s) if !s.is_empty() => Some(s),
                Ok(_) => None,
                Err(e) => {
                    set_error(format!("Invalid UTF-8 in anon_support: {}", e), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        }
    };

    match crate::functions::comparison::set_comparison_function(
        config,
        code,
        crate::functions::comparison::SetComparisonFunctionParams {
            connect_str: conn_opt,
            description: desc_opt,
            language: lang_opt,
            anon_support: anon_opt,
        },
    ) {
        Ok((modified_config, _)) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to convert result: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/* ============================================================================
 * Standardize Call Operations (Batch 6a)
 * ============================================================================ */

/// Add a standardize call
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_addStandardizeCall(
    config_json: *const c_char,
    ftype_code: *const c_char,
    felem_code: *const c_char,
    exec_order: i64,
    sfunc_code: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let ftype_opt = if ftype_code.is_null() {
        None
    } else {
        unsafe {
            match CStr::from_ptr(ftype_code).to_str() {
                Ok(s) if !s.is_empty() => Some(s),
                Ok(_) => None,
                Err(e) => {
                    set_error(format!("Invalid UTF-8 in ftype_code: {}", e), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        }
    };

    let felem_opt = if felem_code.is_null() {
        None
    } else {
        unsafe {
            match CStr::from_ptr(felem_code).to_str() {
                Ok(s) if !s.is_empty() => Some(s),
                Ok(_) => None,
                Err(e) => {
                    set_error(format!("Invalid UTF-8 in felem_code: {}", e), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        }
    };

    let exec_opt = if exec_order < 0 {
        None
    } else {
        Some(exec_order)
    };

    let sfunc = unsafe {
        if sfunc_code.is_null() {
            set_error("sfunc_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(sfunc_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in sfunc_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let params = crate::calls::standardize::AddStandardizeCallParams {
        ftype_code: ftype_opt,
        felem_code: felem_opt,
        exec_order: exec_opt,
        sfunc_code: sfunc,
    };

    match crate::calls::standardize::add_standardize_call(config, params) {
        Ok((modified_config, _)) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to convert result: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Delete a standardize call
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteStandardizeCall(
    config_json: *const c_char,
    sfcall_id: i64,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    handle_result!(crate::calls::standardize::delete_standardize_call(
        config, sfcall_id
    ))
}

/// Get a standardize call
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getStandardizeCall(
    config_json: *const c_char,
    sfcall_id: i64,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::calls::standardize::get_standardize_call(config, sfcall_id) {
        Ok(value) => {
            let json_str = serde_json::to_string(&value)
                .unwrap_or_else(|e| format!("{{\"error\": \"Failed to serialize: {}\"}}", e));
            match CString::new(json_str) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to convert result: {}", e), -4);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -4,
                    }
                }
            }
        }
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// List all standardize calls
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_listStandardizeCalls(
    config_json: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::calls::standardize::list_standardize_calls(config) {
        Ok(vec) => {
            let json_str = serde_json::to_string(&vec)
                .unwrap_or_else(|e| format!("{{\"error\": \"Failed to serialize: {}\"}}", e));
            match CString::new(json_str) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to convert result: {}", e), -4);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -4,
                    }
                }
            }
        }
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Set (update) a standardize call
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_setStandardizeCall(
    config_json: *const c_char,
    sfcall_id: i64,
    updates_json: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let updates = unsafe {
        if updates_json.is_null() {
            set_error("updates_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(updates_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in updates_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let updates_value: serde_json::Value = match serde_json::from_str(updates) {
        Ok(v) => v,
        Err(e) => {
            set_error(format!("Invalid JSON in updates_json: {}", e), -3);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -3,
            };
        }
    };

    let params = crate::calls::standardize::SetStandardizeCallParams {
        sfcall_id,
        exec_order: updates_value.get("execOrder").and_then(|v| v.as_i64()),
    };

    handle_result!(crate::calls::standardize::set_standardize_call(config, params))
}

/* ============================================================================
 * Threshold Operations (Batch 7)
 * ============================================================================ */

// ===== Comparison Thresholds =====

/// Add a comparison threshold
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_addComparisonThreshold(
    config_json: *const c_char,
    cfunc_id: i64,
    cfunc_rtnval: *const c_char,
    ftype_id: i64,        // Negative = None
    exec_order: i64,      // Negative = None
    same_score: i64,      // Negative = None
    close_score: i64,     // Negative = None
    likely_score: i64,    // Negative = None
    plausible_score: i64, // Negative = None
    un_likely_score: i64, // Negative = None
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let rtnval = unsafe {
        if cfunc_rtnval.is_null() {
            set_error("cfunc_rtnval is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(cfunc_rtnval).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in cfunc_rtnval: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    // Convert negative values to None
    let ftype_opt = if ftype_id < 0 { None } else { Some(ftype_id) };
    let exec_opt = if exec_order < 0 {
        None
    } else {
        Some(exec_order)
    };
    let same_opt = if same_score < 0 {
        None
    } else {
        Some(same_score)
    };
    let close_opt = if close_score < 0 {
        None
    } else {
        Some(close_score)
    };
    let likely_opt = if likely_score < 0 {
        None
    } else {
        Some(likely_score)
    };
    let plausible_opt = if plausible_score < 0 {
        None
    } else {
        Some(plausible_score)
    };
    let unlikely_opt = if un_likely_score < 0 {
        None
    } else {
        Some(un_likely_score)
    };

    handle_result!(crate::thresholds::add_comparison_threshold(
        config,
        cfunc_id,
        rtnval,
        crate::thresholds::AddComparisonThresholdParams {
            ftype_id: ftype_opt,
            exec_order: exec_opt,
            same_score: same_opt,
            close_score: close_opt,
            likely_score: likely_opt,
            plausible_score: plausible_opt,
            un_likely_score: unlikely_opt,
        }
    ))
}

/// Delete a comparison threshold
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteComparisonThreshold(
    config_json: *const c_char,
    cfrtn_id: i64,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    handle_result!(crate::thresholds::delete_comparison_threshold(
        config, cfrtn_id
    ))
}

/// Set (update) a comparison threshold
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_setComparisonThreshold(
    config_json: *const c_char,
    cfrtn_id: i64,
    updates_json: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let updates = unsafe {
        if updates_json.is_null() {
            set_error("updates_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(updates_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in updates_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let updates_value: serde_json::Value = match serde_json::from_str(updates) {
        Ok(v) => v,
        Err(e) => {
            set_error(format!("Invalid JSON in updates_json: {}", e), -3);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -3,
            };
        }
    };

    handle_result!(crate::thresholds::set_comparison_threshold(
        config,
        cfrtn_id,
        &updates_value
    ))
}

/// List all comparison thresholds
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_listComparisonThresholds(
    config_json: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::thresholds::list_comparison_thresholds(config) {
        Ok(list) => match serde_json::to_string(&list) {
            Ok(json_str) => match CString::new(json_str) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to create C string: {}", e), -4);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -4,
                    }
                }
            },
            Err(e) => {
                set_error(format!("Failed to serialize list: {}", e), -3);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

// ===== Generic Thresholds =====

/// Add a generic threshold
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_addGenericThreshold(
    config_json: *const c_char,
    plan: *const c_char,
    behavior: *const c_char,
    scoring_cap: i64,
    candidate_cap: i64,
    send_to_redo: *const c_char,
    feature: *const c_char, // Null = "ALL"
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let plan_str = unsafe {
        if plan.is_null() {
            set_error("plan is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(plan).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in plan: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let behavior_str = unsafe {
        if behavior.is_null() {
            set_error("behavior is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(behavior).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in behavior: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let redo_str = unsafe {
        if send_to_redo.is_null() {
            set_error("send_to_redo is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(send_to_redo).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in send_to_redo: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let feature_opt = if feature.is_null() {
        None
    } else {
        unsafe {
            match CStr::from_ptr(feature).to_str() {
                Ok(s) => Some(s),
                Err(e) => {
                    set_error(format!("Invalid UTF-8 in feature: {}", e), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        }
    };

    handle_result!(crate::thresholds::add_generic_threshold(
        config,
        plan_str,
        crate::thresholds::AddGenericThresholdParams {
            behavior: behavior_str,
            scoring_cap,
            candidate_cap,
            send_to_redo: redo_str,
            feature: feature_opt,
        }
    ))
}

/// Delete a generic threshold
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteGenericThreshold(
    config_json: *const c_char,
    plan: *const c_char,
    behavior: *const c_char,
    feature: *const c_char, // Null = "ALL"
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let plan_str = unsafe {
        if plan.is_null() {
            set_error("plan is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(plan).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in plan: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let behavior_str = unsafe {
        if behavior.is_null() {
            set_error("behavior is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(behavior).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in behavior: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let feature_opt = if feature.is_null() {
        None
    } else {
        unsafe {
            match CStr::from_ptr(feature).to_str() {
                Ok(s) => Some(s),
                Err(e) => {
                    set_error(format!("Invalid UTF-8 in feature: {}", e), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        }
    };

    handle_result!(crate::thresholds::delete_generic_threshold(
        config,
        plan_str,
        behavior_str,
        feature_opt
    ))
}

/// Set (update) a generic threshold
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_setGenericThreshold(
    config_json: *const c_char,
    gplan_id: i64,
    behavior: *const c_char,
    updates_json: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let behavior_str = unsafe {
        if behavior.is_null() {
            set_error("behavior is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(behavior).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in behavior: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let updates = unsafe {
        if updates_json.is_null() {
            set_error("updates_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(updates_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in updates_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let updates_value: serde_json::Value = match serde_json::from_str(updates) {
        Ok(v) => v,
        Err(e) => {
            set_error(format!("Invalid JSON in updates_json: {}", e), -3);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -3,
            };
        }
    };

    handle_result!(crate::thresholds::set_generic_threshold(
        config,
        gplan_id,
        behavior_str,
        &updates_value
    ))
}

/// List all generic thresholds
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_listGenericThresholds(
    config_json: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::thresholds::list_generic_thresholds(config) {
        Ok(list) => match serde_json::to_string(&list) {
            Ok(json_str) => match CString::new(json_str) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to create C string: {}", e), -4);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -4,
                    }
                }
            },
            Err(e) => {
                set_error(format!("Failed to serialize list: {}", e), -3);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/* ============================================================================
 * Fragment & Data Source Operations (Batch 8)
 * ============================================================================ */

// ===== Fragment Operations =====

/// Get a fragment by code or ID
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getFragment(
    config_json: *const c_char,
    code_or_id: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let code = unsafe {
        if code_or_id.is_null() {
            set_error("code_or_id is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(code_or_id).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in code_or_id: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::fragments::get_fragment(config, code) {
        Ok(fragment) => match serde_json::to_string(&fragment) {
            Ok(json_str) => match CString::new(json_str) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to create C string: {}", e), -4);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -4,
                    }
                }
            },
            Err(e) => {
                set_error(format!("Failed to serialize fragment: {}", e), -3);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// List all fragments
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_listFragments(config_json: *const c_char) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::fragments::list_fragments(config) {
        Ok(list) => match serde_json::to_string(&list) {
            Ok(json_str) => match CString::new(json_str) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to create C string: {}", e), -4);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -4,
                    }
                }
            },
            Err(e) => {
                set_error(format!("Failed to serialize list: {}", e), -3);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Add a fragment
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_addFragment(
    config_json: *const c_char,
    fragment_json: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let fragment_str = unsafe {
        if fragment_json.is_null() {
            set_error("fragment_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(fragment_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in fragment_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let fragment_value: serde_json::Value = match serde_json::from_str(fragment_str) {
        Ok(v) => v,
        Err(e) => {
            set_error(format!("Invalid JSON in fragment_json: {}", e), -3);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -3,
            };
        }
    };

    match crate::fragments::add_fragment(config, &fragment_value) {
        Ok((modified_config, _frag_id)) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to create C string: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Delete a fragment
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteFragment(
    config_json: *const c_char,
    fragment_code: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let code = unsafe {
        if fragment_code.is_null() {
            set_error("fragment_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(fragment_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in fragment_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    handle_result!(crate::fragments::delete_fragment(config, code))
}

// ===== Data Source Operations =====

/// Get a data source by code
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getDataSource(
    config_json: *const c_char,
    code: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let ds_code = unsafe {
        if code.is_null() {
            set_error("code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::datasources::get_data_source(config, ds_code) {
        Ok(data_source) => match serde_json::to_string(&data_source) {
            Ok(json_str) => match CString::new(json_str) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to create C string: {}", e), -4);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -4,
                    }
                }
            },
            Err(e) => {
                set_error(format!("Failed to serialize data source: {}", e), -3);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Set (update) a data source
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_setDataSource(
    config_json: *const c_char,
    code: *const c_char,
    updates_json: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let ds_code = unsafe {
        if code.is_null() {
            set_error("code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let updates = unsafe {
        if updates_json.is_null() {
            set_error("updates_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(updates_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in updates_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let updates_value: serde_json::Value = match serde_json::from_str(updates) {
        Ok(v) => v,
        Err(e) => {
            set_error(format!("Invalid JSON in updates_json: {}", e), -3);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -3,
            };
        }
    };

    // Build params with code from parameter and optional updates from JSON
    let params = crate::datasources::SetDataSourceParams {
        code: ds_code,
        retention_level: updates_value.get("retentionLevel").and_then(|v| v.as_str()),
        conversational: updates_value.get("conversational").and_then(|v| v.as_str()),
        reliability: updates_value.get("reliability").and_then(|v| v.as_i64()),
    };

    handle_result!(crate::datasources::set_data_source(config, params))
}

/* ============================================================================
 * Feature & Element Operations (Batch 9)
 * ============================================================================ */

// ===== Feature Operations =====

/// Add a feature with JSON configuration
///
/// # Safety
/// config_json and feature_json must be valid null-terminated C strings
///
/// # Parameters
/// - config_json: Current configuration
/// - feature_code: Feature code (will be uppercased)
/// - feature_json: JSON object with feature configuration including:
///   - elementList: Array of element definitions (required)
///   - class, behavior, candidates, anonymize, derived, history, matchkey (optional)
///   - standardize, expression, comparison: Function codes (optional)
///   - version, rtype_id (optional)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_addFeature(
    config_json: *const c_char,
    feature_code: *const c_char,
    feature_json: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let code = unsafe {
        if feature_code.is_null() {
            set_error("feature_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(feature_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in feature_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let feature_str = unsafe {
        if feature_json.is_null() {
            set_error("feature_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(feature_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in feature_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    // Parse feature configuration
    let feature_config: serde_json::Value = match serde_json::from_str(feature_str) {
        Ok(v) => v,
        Err(e) => {
            set_error(format!("Invalid JSON in feature_json: {}", e), -3);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -3,
            };
        }
    };

    // Extract parameters from JSON
    let element_list = match feature_config
        .get("elementList")
        .or_else(|| feature_config.get("element_list"))
    {
        Some(v) => v,
        None => {
            set_error("Missing required field: elementList".to_string(), -3);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -3,
            };
        }
    };

    let class = feature_config.get("class").and_then(|v| v.as_str());
    let behavior = feature_config.get("behavior").and_then(|v| v.as_str());
    let candidates = feature_config.get("candidates").and_then(|v| v.as_str());
    let anonymize = feature_config.get("anonymize").and_then(|v| v.as_str());
    let derived = feature_config.get("derived").and_then(|v| v.as_str());
    let history = feature_config.get("history").and_then(|v| v.as_str());
    let matchkey = feature_config
        .get("matchkey")
        .or_else(|| feature_config.get("matchKey"))
        .and_then(|v| v.as_str());
    let standardize = feature_config.get("standardize").and_then(|v| v.as_str());
    let expression = feature_config.get("expression").and_then(|v| v.as_str());
    let comparison = feature_config.get("comparison").and_then(|v| v.as_str());
    let version = feature_config.get("version").and_then(|v| v.as_i64());
    let rtype_id = feature_config
        .get("rtype_id")
        .or_else(|| feature_config.get("rtypeId"))
        .and_then(|v| v.as_i64());

    match crate::features::add_feature(config, crate::features::AddFeatureParams {
                    feature: code,
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
            version,
            rtype_id,
        },
    ) {
        Ok(modified_config) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to create C string: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Delete a feature by code or ID
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteFeature(
    config_json: *const c_char,
    feature_code_or_id: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let code_or_id = unsafe {
        if feature_code_or_id.is_null() {
            set_error("feature_code_or_id is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(feature_code_or_id).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in feature_code_or_id: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    handle_result!(crate::features::delete_feature(config, code_or_id))
}

/// Set (update) feature properties with JSON
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_setFeature(
    config_json: *const c_char,
    feature_code_or_id: *const c_char,
    updates_json: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let code_or_id = unsafe {
        if feature_code_or_id.is_null() {
            set_error("feature_code_or_id is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(feature_code_or_id).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in feature_code_or_id: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let updates = unsafe {
        if updates_json.is_null() {
            set_error("updates_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(updates_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in updates_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    // Parse updates JSON
    let updates_config: serde_json::Value = match serde_json::from_str(updates) {
        Ok(v) => v,
        Err(e) => {
            set_error(format!("Invalid JSON in updates_json: {}", e), -3);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -3,
            };
        }
    };

    // Extract optional parameters
    let candidates = updates_config.get("candidates").and_then(|v| v.as_str());
    let anonymize = updates_config.get("anonymize").and_then(|v| v.as_str());
    let derived = updates_config.get("derived").and_then(|v| v.as_str());
    let history = updates_config.get("history").and_then(|v| v.as_str());
    let matchkey = updates_config
        .get("matchkey")
        .or_else(|| updates_config.get("matchKey"))
        .and_then(|v| v.as_str());
    let behavior = updates_config.get("behavior").and_then(|v| v.as_str());
    let class = updates_config.get("class").and_then(|v| v.as_str());
    let version = updates_config.get("version").and_then(|v| v.as_i64());
    let rtype_id = updates_config
        .get("rtypeId")
        .or_else(|| updates_config.get("RTYPE_ID"))
        .and_then(|v| v.as_i64());

    handle_result!(crate::features::set_feature(config, crate::features::SetFeatureParams {
                    feature: code_or_id,
            candidates,
            anonymize,
            derived,
            history,
            matchkey,
            behavior,
            class,
            version,
            rtype_id,
        }
    ))
}

// ===== Behavior Override Operations =====

/// Add a behavior override for a feature based on usage type
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_addBehaviorOverride(
    config_json: *const c_char,
    feature_code: *const c_char,
    usage_type: *const c_char,
    behavior: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let feature = unsafe {
        if feature_code.is_null() {
            set_error("feature_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(feature_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in feature_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let utype = unsafe {
        if usage_type.is_null() {
            set_error("usage_type is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(usage_type).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in usage_type: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let bhvr = unsafe {
        if behavior.is_null() {
            set_error("behavior is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(behavior).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in behavior: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    handle_result!(crate::behavior_overrides::add_behavior_override(
        config,
        crate::behavior_overrides::AddBehaviorOverrideParams::new(feature, utype, bhvr)
    ))
}

/// Delete a behavior override
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteBehaviorOverride(
    config_json: *const c_char,
    feature_code: *const c_char,
    usage_type: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let feature = unsafe {
        if feature_code.is_null() {
            set_error("feature_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(feature_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in feature_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let utype = unsafe {
        if usage_type.is_null() {
            set_error("usage_type is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(usage_type).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in usage_type: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    handle_result!(crate::behavior_overrides::delete_behavior_override(
        config, feature, utype
    ))
}

/// List all behavior overrides
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_listBehaviorOverrides(
    config_json: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let result = crate::behavior_overrides::list_behavior_overrides(config).and_then(|vec| {
        serde_json::to_string(&vec).map_err(|e| SzConfigError::JsonParse(e.to_string()))
    });
    handle_result!(result)
}

// ===== Element Operations =====

/// Add an element with JSON configuration
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_addElement(
    config_json: *const c_char,
    element_code: *const c_char,
    element_json: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let code = unsafe {
        if element_code.is_null() {
            set_error("element_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(element_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in element_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let element_str = unsafe {
        if element_json.is_null() {
            set_error("element_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(element_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in element_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let element_config: serde_json::Value = match serde_json::from_str(element_str) {
        Ok(v) => v,
        Err(e) => {
            set_error(format!("Invalid JSON in element_json: {}", e), -3);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -3,
            };
        }
    };

    // Build params from code and JSON config
    let params = crate::elements::AddElementParams {
        code,
        description: element_config.get("description").and_then(|v| v.as_str())
            .or_else(|| element_config.get("FELEM_DESC").and_then(|v| v.as_str())),
        data_type: element_config.get("dataType").and_then(|v| v.as_str())
            .or_else(|| element_config.get("DATA_TYPE").and_then(|v| v.as_str())),
        tokenized: element_config.get("tokenized").and_then(|v| v.as_str())
            .or_else(|| element_config.get("TOKENIZED").and_then(|v| v.as_str())),
    };

    handle_result!(crate::elements::add_element(config, params))
}

/// Delete an element by code
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteElement(
    config_json: *const c_char,
    element_code: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let code = unsafe {
        if element_code.is_null() {
            set_error("element_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(element_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in element_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    handle_result!(crate::elements::delete_element(config, code))
}

/// Set (update) element properties with JSON
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_setElement(
    config_json: *const c_char,
    element_code: *const c_char,
    updates_json: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let code = unsafe {
        if element_code.is_null() {
            set_error("element_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(element_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in element_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let updates = unsafe {
        if updates_json.is_null() {
            set_error("updates_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(updates_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in updates_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let updates_config: serde_json::Value = match serde_json::from_str(updates) {
        Ok(v) => v,
        Err(e) => {
            set_error(format!("Invalid JSON in updates_json: {}", e), -3);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -3,
            };
        }
    };

    // Build params from code and JSON updates
    let params = crate::elements::SetElementParams {
        code,
        description: updates_config.get("description").and_then(|v| v.as_str())
            .or_else(|| updates_config.get("FELEM_DESC").and_then(|v| v.as_str())),
        data_type: updates_config.get("dataType").and_then(|v| v.as_str())
            .or_else(|| updates_config.get("DATA_TYPE").and_then(|v| v.as_str())),
        tokenized: updates_config.get("tokenized").and_then(|v| v.as_str())
            .or_else(|| updates_config.get("TOKENIZED").and_then(|v| v.as_str())),
    };

    handle_result!(crate::elements::set_element(config, params))
}

/* ============================================================================
 * Call Operations - Expression, Comparison, Distinct (Batch 10)
 * ============================================================================ */

// ===== Expression Call Operations =====

/// Add an expression call with JSON element list
///
/// # Parameters
/// - element_list_json: JSON array of element definitions, e.g.:
///   [{"element": "NAME_LAST", "required": "Yes", "feature": "NAME"}]
///   or simplified: [["NAME", "NAME_LAST", "NAME"], ["NAME", "NAME_FIRST", null]]
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_addExpressionCall(
    config_json: *const c_char,
    ftype_code: *const c_char, // NULL or "ALL" for all features
    felem_code: *const c_char, // NULL or "N/A" for no element
    exec_order: i64,           // Negative = auto-assign
    efunc_code: *const c_char,
    element_list_json: *const c_char,  // JSON array
    expression_feature: *const c_char, // NULL or "N/A" for none
    is_virtual: *const c_char,         // "Yes", "No", "Any", "Desired"
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    // Handle optional ftype_code
    let ftype_opt = if ftype_code.is_null() {
        None
    } else {
        unsafe {
            match CStr::from_ptr(ftype_code).to_str() {
                Ok(s) => Some(s),
                Err(e) => {
                    set_error(format!("Invalid UTF-8 in ftype_code: {}", e), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        }
    };

    // Handle optional felem_code
    let felem_opt = if felem_code.is_null() {
        None
    } else {
        unsafe {
            match CStr::from_ptr(felem_code).to_str() {
                Ok(s) => Some(s),
                Err(e) => {
                    set_error(format!("Invalid UTF-8 in felem_code: {}", e), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        }
    };

    let efunc = unsafe {
        if efunc_code.is_null() {
            set_error("efunc_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(efunc_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in efunc_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let elem_list_str = unsafe {
        if element_list_json.is_null() {
            set_error("element_list_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(element_list_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in element_list_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    // Handle optional expression_feature
    let expr_feat_opt = if expression_feature.is_null() {
        None
    } else {
        unsafe {
            match CStr::from_ptr(expression_feature).to_str() {
                Ok(s) => Some(s),
                Err(e) => {
                    set_error(format!("Invalid UTF-8 in expression_feature: {}", e), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        }
    };

    let virtual_str = unsafe {
        if is_virtual.is_null() {
            set_error("is_virtual is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(is_virtual).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in is_virtual: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    // Parse element list JSON
    let elem_list_value: serde_json::Value = match serde_json::from_str(elem_list_str) {
        Ok(v) => v,
        Err(e) => {
            set_error(format!("Invalid JSON in element_list_json: {}", e), -3);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -3,
            };
        }
    };

    // Convert JSON array to Vec<(String, String, Option<String>)>
    let element_list: Vec<(String, String, Option<String>)> = match elem_list_value.as_array() {
        Some(arr) => {
            let mut result = Vec::new();
            for item in arr {
                // Support both array format: ["element", "required", "feature"]
                // and object format: {"element": "...", "required": "...", "feature": "..."}
                if let Some(arr_item) = item.as_array() {
                    if arr_item.len() >= 2 {
                        let element = arr_item[0].as_str().unwrap_or("").to_string();
                        let required = arr_item[1].as_str().unwrap_or("Yes").to_string();
                        let feature = if arr_item.len() > 2 {
                            arr_item[2].as_str().map(|s| s.to_string())
                        } else {
                            None
                        };
                        result.push((element, required, feature));
                    }
                } else if let Some(obj) = item.as_object() {
                    let element = obj
                        .get("element")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    let required = obj
                        .get("required")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Yes")
                        .to_string();
                    let feature = obj
                        .get("feature")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    result.push((element, required, feature));
                }
            }
            result
        }
        None => {
            set_error("element_list_json must be a JSON array".to_string(), -3);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -3,
            };
        }
    };

    let exec_opt = if exec_order < 0 {
        None
    } else {
        Some(exec_order)
    };

    let call_params = crate::calls::expression::AddExpressionCallParams {
        efunc_code: efunc,
        element_list,
        ftype_code: ftype_opt,
        felem_code: felem_opt,
        exec_order: exec_opt,
        expression_feature: expr_feat_opt,
        is_virtual: virtual_str,
    };

    match crate::calls::expression::add_expression_call(
        config,
        call_params,
    ) {
        Ok((modified_config, _record)) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to create C string: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Delete an expression call by ID
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteExpressionCall(
    config_json: *const c_char,
    efcall_id: i64,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    handle_result!(crate::calls::expression::delete_expression_call(
        config, efcall_id
    ))
}

/// Get an expression call by ID
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getExpressionCall(
    config_json: *const c_char,
    efcall_id: i64,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::calls::expression::get_expression_call(config, efcall_id) {
        Ok(record) => match serde_json::to_string(&record) {
            Ok(json_str) => match CString::new(json_str) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to create C string: {}", e), -4);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -4,
                    }
                }
            },
            Err(e) => {
                set_error(format!("Failed to serialize record: {}", e), -3);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// List all expression calls
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_listExpressionCalls(
    config_json: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::calls::expression::list_expression_calls(config) {
        Ok(list) => match serde_json::to_string(&list) {
            Ok(json_str) => match CString::new(json_str) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to create C string: {}", e), -4);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -4,
                    }
                }
            },
            Err(e) => {
                set_error(format!("Failed to serialize list: {}", e), -3);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Set (update) an expression call
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_setExpressionCall(
    config_json: *const c_char,
    efcall_id: i64,
    updates_json: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let updates = unsafe {
        if updates_json.is_null() {
            set_error("updates_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(updates_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in updates_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let updates_value: serde_json::Value = match serde_json::from_str(updates) {
        Ok(v) => v,
        Err(e) => {
            set_error(format!("Invalid JSON in updates_json: {}", e), -3);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -3,
            };
        }
    };

    let params = crate::calls::expression::SetExpressionCallParams {
        efcall_id,
        exec_order: updates_value.get("execOrder").and_then(|v| v.as_i64()),
    };

    handle_result!(crate::calls::expression::set_expression_call(config, params))
}

// ===== Comparison Call Operations =====

/// Add a comparison call with JSON element list
///
/// # Parameters
/// - element_list_json: JSON array of element codes, e.g.: ["NAME_LAST", "NAME_FIRST"]
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_addComparisonCall(
    config_json: *const c_char,
    ftype_code: *const c_char,
    cfunc_code: *const c_char,
    element_list_json: *const c_char, // JSON array of strings
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let ftype = unsafe {
        if ftype_code.is_null() {
            set_error("ftype_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(ftype_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in ftype_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let cfunc = unsafe {
        if cfunc_code.is_null() {
            set_error("cfunc_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(cfunc_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in cfunc_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let elem_list_str = unsafe {
        if element_list_json.is_null() {
            set_error("element_list_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(element_list_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in element_list_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    // Parse element list JSON
    let elem_list_value: serde_json::Value = match serde_json::from_str(elem_list_str) {
        Ok(v) => v,
        Err(e) => {
            set_error(format!("Invalid JSON in element_list_json: {}", e), -3);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -3,
            };
        }
    };

    // Convert JSON array to Vec<String>
    let element_list: Vec<String> = match elem_list_value.as_array() {
        Some(arr) => arr
            .iter()
            .filter_map(|v| v.as_str())
            .map(|s| s.to_string())
            .collect(),
        None => {
            set_error("element_list_json must be a JSON array".to_string(), -3);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -3,
            };
        }
    };

    match crate::calls::comparison::add_comparison_call(config, ftype, cfunc, element_list) {
        Ok((modified_config, _record)) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to create C string: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Delete a comparison call by ID
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteComparisonCall(
    config_json: *const c_char,
    cfcall_id: i64,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    handle_result!(crate::calls::comparison::delete_comparison_call(
        config, cfcall_id
    ))
}

/// Get a comparison call by ID
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getComparisonCall(
    config_json: *const c_char,
    cfcall_id: i64,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::calls::comparison::get_comparison_call(config, cfcall_id) {
        Ok(record) => match serde_json::to_string(&record) {
            Ok(json_str) => match CString::new(json_str) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to create C string: {}", e), -4);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -4,
                    }
                }
            },
            Err(e) => {
                set_error(format!("Failed to serialize record: {}", e), -3);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// List all comparison calls
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_listComparisonCalls(
    config_json: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::calls::comparison::list_comparison_calls(config) {
        Ok(list) => match serde_json::to_string(&list) {
            Ok(json_str) => match CString::new(json_str) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to create C string: {}", e), -4);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -4,
                    }
                }
            },
            Err(e) => {
                set_error(format!("Failed to serialize list: {}", e), -3);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Set (update) a comparison call
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_setComparisonCall(
    config_json: *const c_char,
    cfcall_id: i64,
    updates_json: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let updates = unsafe {
        if updates_json.is_null() {
            set_error("updates_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(updates_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in updates_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let updates_value: serde_json::Value = match serde_json::from_str(updates) {
        Ok(v) => v,
        Err(e) => {
            set_error(format!("Invalid JSON in updates_json: {}", e), -3);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -3,
            };
        }
    };

    let params = crate::calls::comparison::SetComparisonCallParams {
        cfcall_id,
        exec_order: updates_value.get("execOrder").and_then(|v| v.as_i64()),
    };

    handle_result!(crate::calls::comparison::set_comparison_call(config, params))
}

// ============================================================================
// BATCH 11: DISTINCT CALL OPERATIONS
// ============================================================================

/// Add a distinct call with JSON element list
///
/// Creates a distinct call linking a function to a feature with element list.
/// Note: Only one distinct call is allowed per feature.
///
/// # Parameters
/// - `config_json`: Configuration JSON string
/// - `ftype_code`: Feature type code
/// - `dfunc_code`: Distinct function code
/// - `element_list_json`: JSON array of element codes, e.g. ["NAME_LAST", "NAME_FIRST"]
///
/// # Returns
/// SzConfigTool_result with modified config or error
///
/// # Memory
/// Caller must free result.response with SzConfigTool_free()
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_addDistinctCall(
    config_json: *const c_char,
    ftype_code: *const c_char,
    dfunc_code: *const c_char,
    element_list_json: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let ftype = unsafe {
        if ftype_code.is_null() {
            set_error("ftype_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(ftype_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in ftype_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let dfunc = unsafe {
        if dfunc_code.is_null() {
            set_error("dfunc_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(dfunc_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in dfunc_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let elem_list_json = unsafe {
        if element_list_json.is_null() {
            set_error("element_list_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(element_list_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in element_list_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    // Parse JSON array of element codes
    let elem_list_value: serde_json::Value = match serde_json::from_str(elem_list_json) {
        Ok(v) => v,
        Err(e) => {
            set_error(format!("Failed to parse element_list_json: {}", e), -3);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -3,
            };
        }
    };

    let element_list: Vec<String> = match elem_list_value.as_array() {
        Some(arr) => arr
            .iter()
            .filter_map(|v| v.as_str())
            .map(|s| s.to_string())
            .collect(),
        None => {
            set_error("element_list_json must be a JSON array".to_string(), -3);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -3,
            };
        }
    };

    match crate::calls::distinct::add_distinct_call(config, ftype, dfunc, element_list) {
        Ok((modified_config, _record)) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to create C string: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Delete a distinct call by ID
///
/// # Parameters
/// - `config_json`: Configuration JSON string
/// - `dfcall_id`: Distinct call ID to delete
///
/// # Returns
/// SzConfigTool_result with modified config or error
///
/// # Memory
/// Caller must free result.response with SzConfigTool_free()
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteDistinctCall(
    config_json: *const c_char,
    dfcall_id: i64,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::calls::distinct::delete_distinct_call(config, dfcall_id) {
        Ok(modified_config) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to create C string: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Get a distinct call by ID
///
/// Returns JSON representing the distinct call record.
///
/// # Parameters
/// - `config_json`: Configuration JSON string
/// - `dfcall_id`: Distinct call ID
///
/// # Returns
/// SzConfigTool_result with JSON record or error
///
/// # Memory
/// Caller must free result.response with SzConfigTool_free()
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getDistinctCall(
    config_json: *const c_char,
    dfcall_id: i64,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::calls::distinct::get_distinct_call(config, dfcall_id) {
        Ok(record) => match serde_json::to_string(&record) {
            Ok(json_str) => match CString::new(json_str) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to create C string: {}", e), -4);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -4,
                    }
                }
            },
            Err(e) => {
                set_error(format!("Failed to serialize record: {}", e), -3);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// List all distinct calls
///
/// Returns JSON array of distinct calls with resolved names.
///
/// # Parameters
/// - `config_json`: Configuration JSON string
///
/// # Returns
/// SzConfigTool_result with JSON array or error
///
/// # Memory
/// Caller must free result.response with SzConfigTool_free()
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_listDistinctCalls(
    config_json: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::calls::distinct::list_distinct_calls(config) {
        Ok(list) => match serde_json::to_string(&list) {
            Ok(json_str) => match CString::new(json_str) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to create C string: {}", e), -4);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -4,
                    }
                }
            },
            Err(e) => {
                set_error(format!("Failed to serialize list: {}", e), -3);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Set (update) a distinct call
///
/// Note: This is a stub function - not implemented in Python version.
///
/// # Parameters
/// - `config_json`: Configuration JSON string
/// - `dfcall_id`: Distinct call ID to update
/// - `updates_json`: JSON object with fields to update
///
/// # Returns
/// SzConfigTool_result with modified config or error
///
/// # Memory
/// Caller must free result.response with SzConfigTool_free()
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_setDistinctCall(
    config_json: *const c_char,
    dfcall_id: i64,
    updates_json: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let updates = unsafe {
        if updates_json.is_null() {
            set_error("updates_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(updates_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in updates_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let updates_value: serde_json::Value = match serde_json::from_str(updates) {
        Ok(v) => v,
        Err(e) => {
            set_error(format!("Failed to parse updates_json: {}", e), -3);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -3,
            };
        }
    };

    let params = crate::calls::distinct::SetDistinctCallParams {
        dfcall_id,
        exec_order: updates_value.get("execOrder").and_then(|v| v.as_i64()),
    };

    match crate::calls::distinct::set_distinct_call(config, params) {
        Ok(modified_config) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to create C string: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

// ============================================================================
// BATCH 12: MATCHING & DISTINCT FUNCTION OPERATIONS
// ============================================================================

// --- MATCHING FUNCTIONS (Placeholders) ---

/// Add a matching function (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_addMatchingFunction(
    config_json: *const c_char,
    rtype_code: *const c_char,
    matching_func: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let rtype = unsafe {
        if rtype_code.is_null() {
            set_error("rtype_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(rtype_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in rtype_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let matching = unsafe {
        if matching_func.is_null() {
            set_error("matching_func is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(matching_func).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in matching_func: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::functions::matching::add_matching_function(config, rtype, matching) {
        Ok((modified_config, _record)) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to create C string: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Delete a matching function (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteMatchingFunction(
    config_json: *const c_char,
    rtype_code: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let rtype = unsafe {
        if rtype_code.is_null() {
            set_error("rtype_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(rtype_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in rtype_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::functions::matching::delete_matching_function(config, rtype) {
        Ok((modified_config, _record)) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to create C string: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Get a matching function (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getMatchingFunction(
    config_json: *const c_char,
    rtype_code: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let rtype = unsafe {
        if rtype_code.is_null() {
            set_error("rtype_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(rtype_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in rtype_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::functions::matching::get_matching_function(config, rtype) {
        Ok(record) => match serde_json::to_string(&record) {
            Ok(json_str) => match CString::new(json_str) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to create C string: {}", e), -4);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -4,
                    }
                }
            },
            Err(e) => {
                set_error(format!("Failed to serialize record: {}", e), -3);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// List all matching functions (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_listMatchingFunctions(
    config_json: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::functions::matching::list_matching_functions(config) {
        Ok(list) => match serde_json::to_string(&list) {
            Ok(json_str) => match CString::new(json_str) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to create C string: {}", e), -4);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -4,
                    }
                }
            },
            Err(e) => {
                set_error(format!("Failed to serialize list: {}", e), -3);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Set (update) a matching function (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_setMatchingFunction(
    config_json: *const c_char,
    rtype_code: *const c_char,
    matching_func: *const c_char, // NULL allowed
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let rtype = unsafe {
        if rtype_code.is_null() {
            set_error("rtype_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(rtype_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in rtype_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let matching_opt = if matching_func.is_null() {
        None
    } else {
        unsafe {
            match CStr::from_ptr(matching_func).to_str() {
                Ok(s) => Some(s),
                Err(e) => {
                    set_error(format!("Invalid UTF-8 in matching_func: {}", e), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        }
    };

    match crate::functions::matching::set_matching_function(config, rtype, matching_opt) {
        Ok((modified_config, _record)) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to create C string: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

// --- DISTINCT FUNCTIONS (Fully Implemented) ---

/// Add a distinct function
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_addDistinctFunction(
    config_json: *const c_char,
    dfunc_code: *const c_char,
    connect_str: *const c_char,
    dfunc_desc: *const c_char, // NULL allowed
    language: *const c_char,   // NULL allowed
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let dfunc = unsafe {
        if dfunc_code.is_null() {
            set_error("dfunc_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(dfunc_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in dfunc_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let connect = unsafe {
        if connect_str.is_null() {
            set_error("connect_str is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(connect_str).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in connect_str: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let desc_opt = if dfunc_desc.is_null() {
        None
    } else {
        unsafe {
            match CStr::from_ptr(dfunc_desc).to_str() {
                Ok(s) => Some(s),
                Err(e) => {
                    set_error(format!("Invalid UTF-8 in dfunc_desc: {}", e), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        }
    };

    let lang_opt = if language.is_null() {
        None
    } else {
        unsafe {
            match CStr::from_ptr(language).to_str() {
                Ok(s) => Some(s),
                Err(e) => {
                    set_error(format!("Invalid UTF-8 in language: {}", e), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        }
    };

    match crate::functions::distinct::add_distinct_function(
        config,
        dfunc,
        crate::functions::distinct::AddDistinctFunctionParams {
            connect_str: connect,
            description: desc_opt,
            language: lang_opt,
        },
    ) {
        Ok((modified_config, _record)) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to create C string: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Delete a distinct function by code
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteDistinctFunction(
    config_json: *const c_char,
    dfunc_code: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let dfunc = unsafe {
        if dfunc_code.is_null() {
            set_error("dfunc_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(dfunc_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in dfunc_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::functions::distinct::delete_distinct_function(config, dfunc) {
        Ok((modified_config, _record)) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to create C string: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Get a distinct function by code
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getDistinctFunction(
    config_json: *const c_char,
    dfunc_code: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let dfunc = unsafe {
        if dfunc_code.is_null() {
            set_error("dfunc_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(dfunc_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in dfunc_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::functions::distinct::get_distinct_function(config, dfunc) {
        Ok(record) => match serde_json::to_string(&record) {
            Ok(json_str) => match CString::new(json_str) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to create C string: {}", e), -4);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -4,
                    }
                }
            },
            Err(e) => {
                set_error(format!("Failed to serialize record: {}", e), -3);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// List all distinct functions
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_listDistinctFunctions(
    config_json: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::functions::distinct::list_distinct_functions(config) {
        Ok(list) => match serde_json::to_string(&list) {
            Ok(json_str) => match CString::new(json_str) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to create C string: {}", e), -4);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -4,
                    }
                }
            },
            Err(e) => {
                set_error(format!("Failed to serialize list: {}", e), -3);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Set (update) a distinct function
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_setDistinctFunction(
    config_json: *const c_char,
    dfunc_code: *const c_char,
    connect_str: *const c_char, // NULL allowed
    dfunc_desc: *const c_char,  // NULL allowed
    language: *const c_char,    // NULL allowed
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let dfunc = unsafe {
        if dfunc_code.is_null() {
            set_error("dfunc_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(dfunc_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in dfunc_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let connect_opt = if connect_str.is_null() {
        None
    } else {
        unsafe {
            match CStr::from_ptr(connect_str).to_str() {
                Ok(s) => Some(s),
                Err(e) => {
                    set_error(format!("Invalid UTF-8 in connect_str: {}", e), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        }
    };

    let desc_opt = if dfunc_desc.is_null() {
        None
    } else {
        unsafe {
            match CStr::from_ptr(dfunc_desc).to_str() {
                Ok(s) => Some(s),
                Err(e) => {
                    set_error(format!("Invalid UTF-8 in dfunc_desc: {}", e), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        }
    };

    let lang_opt = if language.is_null() {
        None
    } else {
        unsafe {
            match CStr::from_ptr(language).to_str() {
                Ok(s) => Some(s),
                Err(e) => {
                    set_error(format!("Invalid UTF-8 in language: {}", e), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        }
    };

    match crate::functions::distinct::set_distinct_function(
        config,
        dfunc,
        crate::functions::distinct::SetDistinctFunctionParams {
            connect_str: connect_opt,
            description: desc_opt,
            language: lang_opt,
        },
    ) {
        Ok((modified_config, _record)) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to create C string: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

// ============================================================================
// BATCH 13: CANDIDATE & VALIDATION FUNCTION OPERATIONS (Placeholders)
// ============================================================================

// --- CANDIDATE FUNCTIONS ---

/// Add a candidate function (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_addCandidateFunction(
    config_json: *const c_char,
    rtype_code: *const c_char,
    candidate_func: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let rtype = unsafe {
        if rtype_code.is_null() {
            set_error("rtype_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(rtype_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in rtype_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let candidate = unsafe {
        if candidate_func.is_null() {
            set_error("candidate_func is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(candidate_func).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in candidate_func: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::functions::candidate::add_candidate_function(config, rtype, candidate) {
        Ok((modified_config, _record)) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to create C string: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Delete a candidate function (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteCandidateFunction(
    config_json: *const c_char,
    rtype_code: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let rtype = unsafe {
        if rtype_code.is_null() {
            set_error("rtype_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(rtype_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in rtype_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::functions::candidate::delete_candidate_function(config, rtype) {
        Ok((modified_config, _record)) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to create C string: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Get a candidate function (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getCandidateFunction(
    config_json: *const c_char,
    rtype_code: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let rtype = unsafe {
        if rtype_code.is_null() {
            set_error("rtype_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(rtype_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in rtype_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::functions::candidate::get_candidate_function(config, rtype) {
        Ok(record) => match serde_json::to_string(&record) {
            Ok(json_str) => match CString::new(json_str) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to create C string: {}", e), -4);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -4,
                    }
                }
            },
            Err(e) => {
                set_error(format!("Failed to serialize record: {}", e), -3);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// List all candidate functions (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_listCandidateFunctions(
    config_json: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::functions::candidate::list_candidate_functions(config) {
        Ok(list) => match serde_json::to_string(&list) {
            Ok(json_str) => match CString::new(json_str) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to create C string: {}", e), -4);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -4,
                    }
                }
            },
            Err(e) => {
                set_error(format!("Failed to serialize list: {}", e), -3);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Set (update) a candidate function (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_setCandidateFunction(
    config_json: *const c_char,
    rtype_code: *const c_char,
    candidate_func: *const c_char, // NULL allowed
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let rtype = unsafe {
        if rtype_code.is_null() {
            set_error("rtype_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(rtype_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in rtype_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let candidate_opt = if candidate_func.is_null() {
        None
    } else {
        unsafe {
            match CStr::from_ptr(candidate_func).to_str() {
                Ok(s) => Some(s),
                Err(e) => {
                    set_error(format!("Invalid UTF-8 in candidate_func: {}", e), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        }
    };

    match crate::functions::candidate::set_candidate_function(config, rtype, candidate_opt) {
        Ok((modified_config, _record)) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to create C string: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

// --- VALIDATION FUNCTIONS ---

/// Add a validation function (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_addValidationFunction(
    config_json: *const c_char,
    attr_code: *const c_char,
    validation_func: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let attr = unsafe {
        if attr_code.is_null() {
            set_error("attr_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(attr_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in attr_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let validation = unsafe {
        if validation_func.is_null() {
            set_error("validation_func is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(validation_func).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in validation_func: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::functions::validation::add_validation_function(config, attr, validation) {
        Ok((modified_config, _record)) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to create C string: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Delete a validation function (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteValidationFunction(
    config_json: *const c_char,
    attr_code: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let attr = unsafe {
        if attr_code.is_null() {
            set_error("attr_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(attr_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in attr_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::functions::validation::delete_validation_function(config, attr) {
        Ok((modified_config, _record)) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to create C string: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Get a validation function (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getValidationFunction(
    config_json: *const c_char,
    attr_code: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let attr = unsafe {
        if attr_code.is_null() {
            set_error("attr_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(attr_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in attr_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::functions::validation::get_validation_function(config, attr) {
        Ok(record) => match serde_json::to_string(&record) {
            Ok(json_str) => match CString::new(json_str) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to create C string: {}", e), -4);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -4,
                    }
                }
            },
            Err(e) => {
                set_error(format!("Failed to serialize record: {}", e), -3);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// List all validation functions (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_listValidationFunctions(
    config_json: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::functions::validation::list_validation_functions(config) {
        Ok(list) => match serde_json::to_string(&list) {
            Ok(json_str) => match CString::new(json_str) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to create C string: {}", e), -4);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -4,
                    }
                }
            },
            Err(e) => {
                set_error(format!("Failed to serialize list: {}", e), -3);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Set (update) a validation function (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_setValidationFunction(
    config_json: *const c_char,
    attr_code: *const c_char,
    validation_func: *const c_char, // NULL allowed
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let attr = unsafe {
        if attr_code.is_null() {
            set_error("attr_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(attr_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in attr_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let validation_opt = if validation_func.is_null() {
        None
    } else {
        unsafe {
            match CStr::from_ptr(validation_func).to_str() {
                Ok(s) => Some(s),
                Err(e) => {
                    set_error(format!("Invalid UTF-8 in validation_func: {}", e), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        }
    };

    match crate::functions::validation::set_validation_function(config, attr, validation_opt) {
        Ok((modified_config, _record)) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to create C string: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

// ============================================================================
// BATCH 14: SCORING FUNCTION OPERATIONS (Placeholders) - FINAL BATCH!
// ============================================================================

/// Add a scoring function (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_addScoringFunction(
    config_json: *const c_char,
    rtype_code: *const c_char,
    scoring_func: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let rtype = unsafe {
        if rtype_code.is_null() {
            set_error("rtype_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(rtype_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in rtype_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let scoring = unsafe {
        if scoring_func.is_null() {
            set_error("scoring_func is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(scoring_func).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in scoring_func: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::functions::scoring::add_scoring_function(config, rtype, scoring) {
        Ok((modified_config, _record)) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to create C string: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Delete a scoring function (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteScoringFunction(
    config_json: *const c_char,
    rtype_code: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let rtype = unsafe {
        if rtype_code.is_null() {
            set_error("rtype_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(rtype_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in rtype_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::functions::scoring::delete_scoring_function(config, rtype) {
        Ok((modified_config, _record)) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to create C string: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Get a scoring function (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getScoringFunction(
    config_json: *const c_char,
    rtype_code: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let rtype = unsafe {
        if rtype_code.is_null() {
            set_error("rtype_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(rtype_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in rtype_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::functions::scoring::get_scoring_function(config, rtype) {
        Ok(record) => match serde_json::to_string(&record) {
            Ok(json_str) => match CString::new(json_str) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to create C string: {}", e), -4);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -4,
                    }
                }
            },
            Err(e) => {
                set_error(format!("Failed to serialize record: {}", e), -3);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// List all scoring functions (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_listScoringFunctions(
    config_json: *const c_char,
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    match crate::functions::scoring::list_scoring_functions(config) {
        Ok(list) => match serde_json::to_string(&list) {
            Ok(json_str) => match CString::new(json_str) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to create C string: {}", e), -4);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -4,
                    }
                }
            },
            Err(e) => {
                set_error(format!("Failed to serialize list: {}", e), -3);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}

/// Set (update) a scoring function (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_setScoringFunction(
    config_json: *const c_char,
    rtype_code: *const c_char,
    scoring_func: *const c_char, // NULL allowed
) -> SzConfigTool_result {
    let config = unsafe {
        if config_json.is_null() {
            set_error("config_json is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(config_json).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in config_json: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let rtype = unsafe {
        if rtype_code.is_null() {
            set_error("rtype_code is null".to_string(), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match CStr::from_ptr(rtype_code).to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in rtype_code: {}", e), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    };

    let scoring_opt = if scoring_func.is_null() {
        None
    } else {
        unsafe {
            match CStr::from_ptr(scoring_func).to_str() {
                Ok(s) => Some(s),
                Err(e) => {
                    set_error(format!("Invalid UTF-8 in scoring_func: {}", e), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        }
    };

    match crate::functions::scoring::set_scoring_function(config, rtype, scoring_opt) {
        Ok((modified_config, _record)) => match CString::new(modified_config) {
            Ok(c_str) => {
                clear_error();
                SzConfigTool_result {
                    response: c_str.into_raw(),
                    returnCode: 0,
                }
            }
            Err(e) => {
                set_error(format!("Failed to create C string: {}", e), -4);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                }
            }
        },
        Err(e) => {
            set_error(e.to_string(), -5);
            SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -5,
            }
        }
    }
}
