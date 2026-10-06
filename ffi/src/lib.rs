//! C FFI bindings for sz_configtool_lib
//!
//! This crate (built as libSzConfigTool) provides a C-compatible interface to the
//! Rust sz_configtool_lib library. Last-error state is per thread, every export
//! is panic-guarded (see `ffi_guard`), and the header is
//! `include/libSzConfigTool.h`.
//! Functions follow the pattern established by Senzing's SzLang_helpers.h.
//!
//! # Safety
//!
//! All functions in this module that accept raw pointers require:
//! - Non-null pointers (null pointers will return error codes)
//! - Valid UTF-8 strings for string parameters
//! - Memory allocated by this library must be freed with `SzConfigTool_free`

// The package's library is named `SzConfigTool` to produce Senzing-style
// artifact names (libSzConfigTool.so / SzConfigTool.dll).
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::panic::{AssertUnwindSafe, catch_unwind};

use sz_configtool_lib::error::SzConfigError;

/// The last error recorded on the CURRENT thread.
///
/// Every slot is stored NUL-terminated (`CString`) so the borrowed pointers
/// handed to C are valid C strings. A pointer returned by one of the
/// `SzConfigTool_getLastError*` accessors stays valid until the next
/// `SzConfigTool_*` call on the same thread (which may replace or clear it).
#[derive(Default)]
struct LastError {
    message: Option<CString>,
    code: i64,
    /// Stable `reason_code()` of the last error (e.g. "VALIDATION_ERRORS"), so
    /// a C caller can discriminate the error KIND without string-sniffing.
    reason: Option<CString>,
    /// Versioned, namespaced JSON details when the last error is a
    /// `ValidationErrors`. See [`sz_configtool_api::validation_details_json`].
    details: Option<CString>,
}

thread_local! {
    static LAST_ERROR: RefCell<LastError> = RefCell::new(LastError::default());
}

/// Convert to a C string, escaping any interior NUL so conversion cannot fail.
fn to_cstring_lossy(s: String) -> CString {
    let cleaned = if s.contains('\0') {
        s.replace('\0', "\\0")
    } else {
        s
    };
    CString::new(cleaned).unwrap_or_default()
}

///
/// Every access uses `try_with`: during thread teardown (an FFI call from
/// another thread-local's destructor) the slot may already be destroyed, and
/// `with` would panic — and the panic guard recording THAT panic would panic
/// again outside `catch_unwind`, aborting the process. Without a slot the
/// error is simply not recorded (the return code still reports it), and the
/// getters behave as if no error was recorded (NULL / 0).
fn store_error(error: LastError) {
    let _ = LAST_ERROR.try_with(|slot| *slot.borrow_mut() = error);
}

/// Pointer to one optional C-string slot of the current thread's last error
/// (NULL when none is recorded or the slot is gone; see [`store_error`]).
fn last_error_ptr(select: fn(&LastError) -> Option<&CString>) -> *const c_char {
    LAST_ERROR
        .try_with(|slot| select(&slot.borrow()).map_or(std::ptr::null(), |c| c.as_ptr()))
        .unwrap_or(std::ptr::null())
}

/// ABI version of the C interface. Bumped only on an incompatible change to an
/// existing declaration (removal, signature or semantics change); additions
/// keep the same value.
/// Single definition: [`sz_configtool_api::ABI_VERSION`].
pub const SZCONFIGTOOL_ABI_VERSION: i32 = sz_configtool_api::ABI_VERSION;

/// NUL-terminated library version; single definition:
/// [`sz_configtool_api::LIBRARY_VERSION`] (the workspace version).
const LIBRARY_VERSION: &str = sz_configtool_api::LIBRARY_VERSION_NUL;

// ============================================================================
// Panic guard
// ============================================================================

/// The value an exported function returns when its body panics.
trait PanicReturn {
    fn panic_value() -> Self;
}

impl PanicReturn for SzConfigTool_result {
    fn panic_value() -> Self {
        SzConfigTool_result {
            response: std::ptr::null_mut(),
            returnCode: -2,
        }
    }
}

impl PanicReturn for *const c_char {
    fn panic_value() -> Self {
        std::ptr::null()
    }
}

impl PanicReturn for i64 {
    fn panic_value() -> Self {
        -2
    }
}

impl PanicReturn for i32 {
    fn panic_value() -> Self {
        -2
    }
}

impl PanicReturn for () {
    fn panic_value() -> Self {}
}

fn panic_message(payload: &(dyn std::any::Any + Send)) -> &str {
    if let Some(s) = payload.downcast_ref::<&str>() {
        s
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.as_str()
    } else {
        "non-string panic payload"
    }
}

/// Run an exported function body, converting a Rust panic into the function's
/// error return (-2 / NULL) with an "internal panic" last error, so a panic
/// never unwinds across the C boundary (which would be undefined behaviour or
/// an abort). Every `extern "C"` function in this crate wraps its body in this.
fn ffi_guard<R: PanicReturn>(name: &str, body: impl FnOnce() -> R) -> R {
    match catch_unwind(AssertUnwindSafe(body)) {
        Ok(value) => value,
        Err(payload) => {
            set_error(
                format!(
                    "internal panic in {name}: {}",
                    panic_message(payload.as_ref())
                ),
                -2,
            );
            R::panic_value()
        }
    }
}

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
    ffi_guard("SzConfigTool_free", || {
        if !ptr.is_null() {
            unsafe {
                drop(CString::from_raw(ptr));
            }
        }
    })
}

/// Get the last error message recorded on the calling thread
///
/// # Returns
/// Pointer to a NUL-terminated error string (do not free), or null if no
/// error. Valid until the next `SzConfigTool_*` call on the same thread.
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getLastError() -> *const c_char {
    ffi_guard("SzConfigTool_getLastError", || {
        last_error_ptr(|e| e.message.as_ref())
    })
}

/// Get the last error code recorded on the calling thread
///
/// # Returns
/// Error code (0 = no error, negative = error)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getLastErrorCode() -> i64 {
    ffi_guard("SzConfigTool_getLastErrorCode", || {
        LAST_ERROR.try_with(|slot| slot.borrow().code).unwrap_or(0)
    })
}

/// Get the last error's stable reason code (e.g. "VALIDATION_ERRORS")
///
/// # Returns
/// Pointer to the reason-code string (do not free), or null if no error / no
/// classified reason. Callers discriminate on THIS before fetching details.
/// Valid until the next `SzConfigTool_*` call on the same thread.
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getLastErrorReasonCode() -> *const c_char {
    ffi_guard("SzConfigTool_getLastErrorReasonCode", || {
        last_error_ptr(|e| e.reason.as_ref())
    })
}

/// Get versioned, namespaced JSON details for the last error.
///
/// Populated only when the last error is a `ValidationErrors` (reason code
/// "VALIDATION_ERRORS"); returns null otherwise. The payload is:
/// `{"schema":"sz-configtool.validation-errors/v1","failures":[{"field":...,
/// "reasonCode":...,"offendingValue":...}]}`. The `schema` string is the FFI
/// stability contract — a future field addition bumps it to /v2.
///
/// # Returns
/// Pointer to the JSON string (do not free), or null if the last error carries
/// no structured details. Valid until the next `SzConfigTool_*` call on the
/// same thread.
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getLastErrorDetails() -> *const c_char {
    ffi_guard("SzConfigTool_getLastErrorDetails", || {
        last_error_ptr(|e| e.details.as_ref())
    })
}

/// Clear the last error recorded on the calling thread
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_clearLastError() {
    ffi_guard("SzConfigTool_clearLastError", clear_error)
}

/// Library version string (e.g. "4.4.0-1"), NUL-terminated, static storage —
/// never free it.
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getLibraryVersion() -> *const c_char {
    ffi_guard("SzConfigTool_getLibraryVersion", || {
        LIBRARY_VERSION.as_ptr().cast()
    })
}

/// ABI version of the C interface (see `SZCONFIGTOOL_ABI_VERSION` in the
/// header); a caller compiled against a different value must not use this
/// library.
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getAbiVersion() -> i32 {
    ffi_guard("SzConfigTool_getAbiVersion", || SZCONFIGTOOL_ABI_VERSION)
}

// ============================================================================
// Helper Macros for Error Handling
// ============================================================================

macro_rules! handle_result {
    ($result:expr) => {
        match $result {
            Ok(json) => {
                clear_error();
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
                set_error_from(&e);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                }
            }
        }
    };
}

/// Record a boundary error (null pointer, bad UTF-8, panic). These carry no
/// classified reason or structured details.
fn set_error(msg: String, code: i64) {
    store_error(LastError {
        message: Some(to_cstring_lossy(msg)),
        code,
        reason: None,
        details: None,
    });
}

/// Record a library `SzConfigError`: the flattened `Display` message (code -2,
/// unchanged), plus its stable `reason_code()` for boundary discrimination and,
/// when it is a `ValidationErrors`, the versioned structured details JSON.
fn set_error_from(e: &SzConfigError) {
    store_error(LastError {
        message: Some(to_cstring_lossy(e.to_string())),
        code: -2,
        reason: Some(to_cstring_lossy(e.reason_code().to_string())),
        details: e
            .validation_failures()
            .map(|f| to_cstring_lossy(sz_configtool_api::validation_details_json(f))),
    });
}

fn clear_error() {
    store_error(LastError::default());
}

// ============================================================================
// Dynamic invoke (manifest-driven; see api/manifest/schema.md)
// ============================================================================

/// Borrow a required C string argument, recording a -1 error when it is NULL
/// or not UTF-8.
fn required_c_str<'a>(ptr: *const c_char, what: &str) -> Option<&'a str> {
    if ptr.is_null() {
        set_error(format!("{what} is null"), -1);
        return None;
    }
    match unsafe { CStr::from_ptr(ptr) }.to_str() {
        Ok(s) => Some(s),
        Err(e) => {
            set_error(format!("Invalid UTF-8 in {what}: {e}"), -1);
            None
        }
    }
}

/// Record an `invoke` failure: library errors keep their reason code and
/// validation details; internal failures report reason "INTERNAL".
fn set_error_from_api(e: &sz_configtool_api::ApiError) {
    match e.config_error() {
        Some(lib) => set_error_from(lib),
        None => store_error(LastError {
            message: Some(to_cstring_lossy(e.to_string())),
            code: -2,
            reason: Some(to_cstring_lossy(e.reason_code().to_string())),
            details: None,
        }),
    }
}

fn error_result(code: i64) -> SzConfigTool_result {
    SzConfigTool_result {
        response: std::ptr::null_mut(),
        returnCode: code,
    }
}

/// Call any manifest function by name (`sz_configtool_api::invoke`).
///
/// `args_json` is a JSON object keyed by the manifest arg names (absent =
/// leave/none, `null` = clear for tri-state args, value = set); NULL means no
/// arguments. On success the response is the JSON envelope
/// `{"kind":...,"config"?:"<config as a JSON string>","result"?:...}`.
///
/// # Safety
/// `name` and `config_json` must be valid NUL-terminated C strings;
/// `args_json` must be one or NULL.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_invoke(
    name: *const c_char,
    config_json: *const c_char,
    args_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_invoke", || {
        let Some(name) = required_c_str(name, "name") else {
            return error_result(-1);
        };
        let Some(config) = required_c_str(config_json, "config_json") else {
            return error_result(-1);
        };
        let args = if args_json.is_null() {
            "{}"
        } else {
            match required_c_str(args_json, "args_json") {
                Some(s) => s,
                None => return error_result(-1),
            }
        };
        match sz_configtool_api::invoke(name, config, args) {
            Ok(out) => handle_result!(Ok::<String, SzConfigError>(out.to_envelope())),
            Err(e) => {
                set_error_from_api(&e);
                error_result(-2)
            }
        }
    })
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
    ffi_guard("SzConfigTool_addDataSource", || {
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
                set_error(format!("Invalid UTF-8 in configJson: {e}"), -1);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -1,
                };
            }
        };

        let ds_code = match unsafe { CStr::from_ptr(data_source_code) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in dataSourceCode: {e}"), -1);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -1,
                };
            }
        };

        // Use default values for optional parameters (matches Python defaults)
        let result = sz_configtool_lib::datasources::add_data_source(
            config,
            sz_configtool_lib::datasources::AddDataSourceParams {
                code: ds_code,
                ..Default::default()
            },
        );
        handle_result!(result)
    })
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
    ffi_guard("SzConfigTool_deleteDataSource", || {
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
                set_error(format!("Invalid UTF-8 in configJson: {e}"), -1);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -1,
                };
            }
        };

        let ds_code = match unsafe { CStr::from_ptr(data_source_code) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in dataSourceCode: {e}"), -1);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -1,
                };
            }
        };

        let result = sz_configtool_lib::datasources::delete_data_source(config, ds_code);
        handle_result!(result)
    })
}

/// List all data sources in the configuration (returns JSON array string)
///
/// # Safety
/// configJson must be a valid null-terminated C string
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_listDataSources(
    config_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_listDataSources", || {
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
                set_error(format!("Invalid UTF-8 in configJson: {e}"), -1);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -1,
                };
            }
        };

        let result = sz_configtool_lib::datasources::list_data_sources(config).and_then(|vec| {
            serde_json::to_string(&vec).map_err(|e| SzConfigError::JsonParse(e.to_string()))
        });
        handle_result!(result)
    })
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
    ffi_guard("SzConfigTool_addAttribute", || {
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
                set_error(format!("Invalid UTF-8 in configJson: {e}"), -1);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -1,
                };
            }
        };

        let attr_code = match unsafe { CStr::from_ptr(attribute_code) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in attributeCode: {e}"), -1);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -1,
                };
            }
        };

        let feat_code = match unsafe { CStr::from_ptr(feature_code) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in featureCode: {e}"), -1);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -1,
                };
            }
        };

        let elem_code = match unsafe { CStr::from_ptr(element_code) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in elementCode: {e}"), -1);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -1,
                };
            }
        };

        let class = match unsafe { CStr::from_ptr(attr_class) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in attrClass: {e}"), -1);
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
                    set_error(format!("Invalid UTF-8 in defaultValue: {e}"), -1);
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
                    set_error(format!("Invalid UTF-8 in internal: {e}"), -1);
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
                    set_error(format!("Invalid UTF-8 in required: {e}"), -1);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -1,
                    };
                }
            }
        };

        let result = sz_configtool_lib::attributes::add_attribute(
            config,
            sz_configtool_lib::attributes::AddAttributeParams {
                attribute: attr_code,
                feature: feat_code,
                element: elem_code,
                class,
                default_value: def_val,
                internal: int_val,
                required: req_val,
                // ATTR_ID is not exposed over this fixed C signature; always
                // auto-assign (#37). Rust/JSON callers can request a specific id.
                id: None,
            },
        )
        .map(|(json, _item)| json);

        handle_result!(result)
    })
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
    ffi_guard("SzConfigTool_deleteAttribute", || {
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
                set_error(format!("Invalid UTF-8 in configJson: {e}"), -1);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -1,
                };
            }
        };

        let attr_code = match unsafe { CStr::from_ptr(attribute_code) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in attributeCode: {e}"), -1);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -1,
                };
            }
        };

        let result = sz_configtool_lib::attributes::delete_attribute(config, attr_code);
        handle_result!(result)
    })
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
    ffi_guard("SzConfigTool_getAttribute", || {
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
                set_error(format!("Invalid UTF-8 in configJson: {e}"), -1);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -1,
                };
            }
        };

        let attr_code = match unsafe { CStr::from_ptr(attribute_code) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in attributeCode: {e}"), -1);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -1,
                };
            }
        };

        let result =
            sz_configtool_lib::attributes::get_attribute(config, attr_code).and_then(|val| {
                serde_json::to_string(&val).map_err(|e| SzConfigError::JsonParse(e.to_string()))
            });
        handle_result!(result)
    })
}

/// List all attributes in the configuration (returns JSON array string)
///
/// # Safety
/// configJson must be a valid null-terminated C string
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_listAttributes(
    config_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_listAttributes", || {
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
                set_error(format!("Invalid UTF-8 in configJson: {e}"), -1);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -1,
                };
            }
        };

        let result = sz_configtool_lib::attributes::list_attributes(config).and_then(|vec| {
            serde_json::to_string(&vec).map_err(|e| SzConfigError::JsonParse(e.to_string()))
        });
        handle_result!(result)
    })
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
    ffi_guard("SzConfigTool_setAttribute", || {
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
                set_error(format!("Invalid UTF-8 in configJson: {e}"), -1);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -1,
                };
            }
        };

        let attr_code = match unsafe { CStr::from_ptr(attribute_code) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in attributeCode: {e}"), -1);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -1,
                };
            }
        };

        let updates_str = match unsafe { CStr::from_ptr(updates_json) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in updatesJson: {e}"), -1);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -1,
                };
            }
        };

        let updates: serde_json::Value = match serde_json::from_str(updates_str) {
            Ok(v) => v,
            Err(e) => {
                set_error(format!("Invalid JSON in updatesJson: {e}"), -1);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -1,
                };
            }
        };

        // Build params with attribute from parameter and optional updates from JSON
        let params = sz_configtool_lib::attributes::SetAttributeParams {
            attribute: attr_code,
            internal: updates.get("internal").and_then(|v| v.as_str()),
            required: updates.get("required").and_then(|v| v.as_str()),
            default_value: updates.get("default").and_then(|v| v.as_str()),
        };

        handle_result!(sz_configtool_lib::attributes::set_attribute(config, params))
    })
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
    ffi_guard("SzConfigTool_getFeature", || {
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
                set_error(format!("Invalid UTF-8 in configJson: {e}"), -1);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -1,
                };
            }
        };

        let feat_code = match unsafe { CStr::from_ptr(feature_code) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in featureCode: {e}"), -1);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -1,
                };
            }
        };

        let result = sz_configtool_lib::features::get_feature(config, feat_code).and_then(|val| {
            serde_json::to_string(&val).map_err(|e| SzConfigError::JsonParse(e.to_string()))
        });
        handle_result!(result)
    })
}

/// List all features in the configuration (returns JSON array string)
///
/// # Safety
/// configJson must be a valid null-terminated C string
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_listFeatures(
    config_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_listFeatures", || {
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
                set_error(format!("Invalid UTF-8 in configJson: {e}"), -1);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -1,
                };
            }
        };

        let result = sz_configtool_lib::features::list_features(config).and_then(|vec| {
            serde_json::to_string(&vec).map_err(|e| SzConfigError::JsonParse(e.to_string()))
        });
        handle_result!(result)
    })
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
    ffi_guard("SzConfigTool_getElement", || {
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
                set_error(format!("Invalid UTF-8 in configJson: {e}"), -1);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -1,
                };
            }
        };

        let elem_code = match unsafe { CStr::from_ptr(element_code) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in elementCode: {e}"), -1);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -1,
                };
            }
        };

        let result = sz_configtool_lib::elements::get_element(config, elem_code).and_then(|val| {
            serde_json::to_string(&val).map_err(|e| SzConfigError::JsonParse(e.to_string()))
        });
        handle_result!(result)
    })
}

/// List all elements in the configuration (returns JSON array string)
///
/// # Safety
/// configJson must be a valid null-terminated C string
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_listElements(
    config_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_listElements", || {
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
                set_error(format!("Invalid UTF-8 in configJson: {e}"), -1);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -1,
                };
            }
        };

        let result = sz_configtool_lib::elements::list_elements(config).and_then(|vec| {
            serde_json::to_string(&vec).map_err(|e| SzConfigError::JsonParse(e.to_string()))
        });
        handle_result!(result)
    })
}

/// Set/update a fragment with JSON parameters
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_setFragmentWithJson(
    config_json: *const c_char,
    fragment_code: *const c_char,
    fragment_config_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_setFragmentWithJson", || {
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
                set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        };

        let code = match unsafe { CStr::from_ptr(fragment_code) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in fragment_code: {e}"), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        };

        let fragment_str = match unsafe { CStr::from_ptr(fragment_config_json) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in fragment_config_json: {e}"), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        };

        let fragment_config: serde_json::Value = match serde_json::from_str(fragment_str) {
            Ok(v) => v,
            Err(e) => {
                set_error(format!("Failed to parse fragment_config_json: {e}"), -3);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                };
            }
        };

        // JSON-based FFI: build a tri-state SetFragmentParams. An absent key ->
        // Leave, an explicit null -> Clear (writes null), a value -> Set.
        let params =
            match sz_configtool_lib::fragments::SetFragmentParams::try_from(&fragment_config) {
                Ok(p) => p,
                Err(e) => {
                    set_error(e.to_string(), -3);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -3,
                    };
                }
            };

        let result = sz_configtool_lib::fragments::set_fragment(config, code, params);
        handle_result!(result)
    })
}

/// Clone a generic plan
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_cloneGenericPlan(
    config_json: *const c_char,
    source_gplan_code: *const c_char,
    new_gplan_code: *const c_char,
    new_gplan_desc: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_cloneGenericPlan", || {
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
                set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        };

        let source_code = match unsafe { CStr::from_ptr(source_gplan_code) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in source_gplan_code: {e}"), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        };

        let new_code = match unsafe { CStr::from_ptr(new_gplan_code) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in new_gplan_code: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in new_gplan_desc: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::generic_plans::clone_generic_plan(
            config,
            source_code,
            new_code,
            new_desc,
        ) {
            Ok((modified_config, _gplan_id)) => match CString::new(modified_config) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to convert result: {e}"), -4);
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
    })
}

/// Set/update a generic plan
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_setGenericPlan(
    config_json: *const c_char,
    gplan_code: *const c_char,
    gplan_desc: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_setGenericPlan", || {
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
                set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        };

        let code = match unsafe { CStr::from_ptr(gplan_code) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in gplan_code: {e}"), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        };

        let desc = match unsafe { CStr::from_ptr(gplan_desc) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in gplan_desc: {e}"), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        };

        match sz_configtool_lib::generic_plans::set_generic_plan(config, code, desc) {
            Ok((modified_config, _gplan_id, _was_created)) => match CString::new(modified_config) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to convert result: {e}"), -4);
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
    })
}

/// List generic plans
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_listGenericPlans(
    config_json: *const c_char,
    filter: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_listGenericPlans", || {
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
                set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in filter: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::generic_plans::list_generic_plans(config, filter_opt) {
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
                        set_error(format!("Failed to convert result: {e}"), -4);
                        SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -4,
                        }
                    }
                },
                Err(e) => {
                    set_error(format!("Failed to serialize result: {e}"), -3);
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
    })
}

/// Get a threshold by ID
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_getThreshold(
    config_json: *const c_char,
    threshold_id: i64,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_getThreshold", || {
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
                set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        };

        match sz_configtool_lib::thresholds::get_threshold(config, threshold_id) {
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
                        set_error(format!("Failed to convert result: {e}"), -4);
                        SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -4,
                        }
                    }
                },
                Err(e) => {
                    set_error(format!("Failed to serialize result: {e}"), -3);
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
    })
}

/// List system parameters
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_listSystemParameters(
    config_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_listSystemParameters", || {
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
                set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        };

        match sz_configtool_lib::system_params::list_system_parameters(config) {
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
                        set_error(format!("Failed to convert result: {e}"), -4);
                        SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -4,
                        }
                    }
                },
                Err(e) => {
                    set_error(format!("Failed to serialize result: {e}"), -3);
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
    })
}

/// Set a system parameter with JSON value
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_setSystemParameterWithJson(
    config_json: *const c_char,
    parameter_name: *const c_char,
    parameter_value_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_setSystemParameterWithJson", || {
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
                set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        };

        let param_name = match unsafe { CStr::from_ptr(parameter_name) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in parameter_name: {e}"), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        };

        let value_str = match unsafe { CStr::from_ptr(parameter_value_json) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in parameter_value_json: {e}"), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        };

        let param_value: serde_json::Value = match serde_json::from_str(value_str) {
            Ok(v) => v,
            Err(e) => {
                set_error(format!("Failed to parse parameter_value_json: {e}"), -3);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                };
            }
        };

        let result = sz_configtool_lib::system_params::set_system_parameter(
            config,
            param_name,
            &param_value,
        );
        handle_result!(result)
    })
}

/// Get the configuration version
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_getVersion(
    config_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_getVersion", || {
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
                set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        };

        match sz_configtool_lib::versioning::get_version(config) {
            Ok(version) => match CString::new(version) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to convert result: {e}"), -4);
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
    })
}

/// Get the compatibility version
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_getCompatibilityVersion(
    config_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_getCompatibilityVersion", || {
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
                set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        };

        match sz_configtool_lib::versioning::get_compatibility_version(config) {
            Ok(version) => match CString::new(version) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to convert result: {e}"), -4);
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
    })
}

/// Update the compatibility version
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_updateCompatibilityVersion(
    config_json: *const c_char,
    new_version: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_updateCompatibilityVersion", || {
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
                set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        };

        let version = match unsafe { CStr::from_ptr(new_version) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in new_version: {e}"), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        };

        let result = sz_configtool_lib::versioning::update_compatibility_version(config, version);
        handle_result!(result)
    })
}

/// Update the feature version
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_updateFeatureVersion(
    config_json: *const c_char,
    version: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_updateFeatureVersion", || {
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
                set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        };

        let version_str = match unsafe { CStr::from_ptr(version) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in version: {e}"), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        };

        let result = sz_configtool_lib::features::update_feature_version(config, version_str);
        handle_result!(result)
    })
}

/// Verify compatibility version
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_verifyCompatibilityVersion(
    config_json: *const c_char,
    expected_version: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_verifyCompatibilityVersion", || {
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
                set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        };

        let version = match unsafe { CStr::from_ptr(expected_version) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in expected_version: {e}"), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        };

        match sz_configtool_lib::versioning::verify_compatibility_version(config, version) {
            Ok((message, _matches)) => match CString::new(message) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to convert result: {e}"), -4);
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
    })
}

/// Add a config section
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_addConfigSection(
    config_json: *const c_char,
    section_name: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_addConfigSection", || {
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
                set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        };

        let section = match unsafe { CStr::from_ptr(section_name) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in section_name: {e}"), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        };

        let result = sz_configtool_lib::config_sections::add_config_section(config, section);
        handle_result!(result)
    })
}

/// Remove a config section
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_removeConfigSection(
    config_json: *const c_char,
    section_name: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_removeConfigSection", || {
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
                set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        };

        let section = match unsafe { CStr::from_ptr(section_name) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in section_name: {e}"), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        };

        let result = sz_configtool_lib::config_sections::remove_config_section(config, section);
        handle_result!(result)
    })
}

/// Get a config section
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_getConfigSection(
    config_json: *const c_char,
    section_name: *const c_char,
    filter: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_getConfigSection", || {
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
                set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        };

        let section = match unsafe { CStr::from_ptr(section_name) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in section_name: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in filter: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::config_sections::get_config_section(config, section, filter_opt) {
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
                        set_error(format!("Failed to convert result: {e}"), -4);
                        SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -4,
                        }
                    }
                },
                Err(e) => {
                    set_error(format!("Failed to serialize result: {e}"), -3);
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
    })
}

/// Report whether a config section is empty (null or `[]`).
///
/// Companion to `SzConfigTool_getConfigSection` that lets a caller distinguish an
/// empty section from a filter that matched nothing. On success the response is
/// the JSON boolean `"true"` or `"false"`; a missing section returns an error.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_configSectionIsEmpty(
    config_json: *const c_char,
    section_name: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_configSectionIsEmpty", || {
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
                set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        };

        let section = match unsafe { CStr::from_ptr(section_name) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in section_name: {e}"), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        };

        match sz_configtool_lib::config_sections::config_section_is_empty(config, section) {
            Ok(is_empty) => match CString::new(if is_empty { "true" } else { "false" }) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to convert result: {e}"), -4);
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
    })
}

/// List all config sections
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_listConfigSections(
    config_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_listConfigSections", || {
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
                set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        };

        match sz_configtool_lib::config_sections::list_config_sections(config) {
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
                        set_error(format!("Failed to convert result: {e}"), -4);
                        SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -4,
                        }
                    }
                },
                Err(e) => {
                    set_error(format!("Failed to serialize result: {e}"), -3);
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
    })
}

/// Add a field to a config section (returns tuple)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_addConfigSectionField(
    config_json: *const c_char,
    section_name: *const c_char,
    field_name: *const c_char,
    field_value_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_addConfigSectionField", || {
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
                set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        };

        let section = match unsafe { CStr::from_ptr(section_name) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in section_name: {e}"), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        };

        let field = match unsafe { CStr::from_ptr(field_name) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in field_name: {e}"), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        };

        let value_str = match unsafe { CStr::from_ptr(field_value_json) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in field_value_json: {e}"), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        };

        let field_value: serde_json::Value = match serde_json::from_str(value_str) {
            Ok(v) => v,
            Err(e) => {
                set_error(format!("Failed to parse field_value_json: {e}"), -3);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                };
            }
        };

        match sz_configtool_lib::config_sections::add_config_section_field(
            config,
            section,
            field,
            &field_value,
        ) {
            Ok((modified_config, _counts)) => match CString::new(modified_config) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to convert result: {e}"), -4);
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
    })
}

/// Remove a field from a config section (returns tuple)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SzConfigTool_removeConfigSectionField(
    config_json: *const c_char,
    section_name: *const c_char,
    field_name: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_removeConfigSectionField", || {
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
                set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        };

        let section = match unsafe { CStr::from_ptr(section_name) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in section_name: {e}"), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        };

        let field = match unsafe { CStr::from_ptr(field_name) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in field_name: {e}"), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        };

        match sz_configtool_lib::config_sections::remove_config_section_field(
            config, section, field,
        ) {
            Ok((modified_config, _count)) => match CString::new(modified_config) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to convert result: {e}"), -4);
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
    })
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
    ffi_guard("SzConfigTool_addRule", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in rule_json: {e}"), -2);
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
                set_error(format!("Invalid JSON in rule_json: {e}"), -3);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                };
            }
        };

        let id = rule_value
            .get("ERRULE_ID")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);
        match sz_configtool_lib::rules::add_rule(config, id, &rule_value) {
            Ok((modified_config, _rule_id)) => match CString::new(modified_config) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to convert result: {e}"), -4);
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
    })
}

/// Delete a rule
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteRule(
    config_json: *const c_char,
    rule_code: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_deleteRule", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in rule_code: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        handle_result!(sz_configtool_lib::rules::delete_rule(config, code))
    })
}

/// Get a rule by code or ID
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getRule(
    config_json: *const c_char,
    code_or_id: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_getRule", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in code_or_id: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::rules::get_rule(config, identifier) {
            Ok(rule_json_value) => {
                let rule_str = serde_json::to_string(&rule_json_value)
                    .unwrap_or_else(|e| format!("{{\"error\": \"Failed to serialize: {e}\"}}"));
                match CString::new(rule_str) {
                    Ok(c_str) => {
                        clear_error();
                        SzConfigTool_result {
                            response: c_str.into_raw(),
                            returnCode: 0,
                        }
                    }
                    Err(e) => {
                        set_error(format!("Failed to convert result: {e}"), -4);
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
    })
}

/// List all rules
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_listRules(config_json: *const c_char) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_listRules", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::rules::list_rules(config) {
            Ok(rules_vec) => {
                let rules_str = serde_json::to_string(&rules_vec)
                    .unwrap_or_else(|e| format!("{{\"error\": \"Failed to serialize: {e}\"}}"));
                match CString::new(rules_str) {
                    Ok(c_str) => {
                        clear_error();
                        SzConfigTool_result {
                            response: c_str.into_raw(),
                            returnCode: 0,
                        }
                    }
                    Err(e) => {
                        set_error(format!("Failed to convert result: {e}"), -4);
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
    })
}

/// Set/update a rule with JSON configuration
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_setRule(
    config_json: *const c_char,
    rule_code: *const c_char,
    rule_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_setRule", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in rule_code: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in rule_json: {e}"), -2);
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
                set_error(format!("Invalid JSON in rule_json: {e}"), -3);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                };
            }
        };

        // Build params from code and JSON config
        let params = sz_configtool_lib::rules::SetRuleParams {
            code,
            resolve: rule_value
                .get("resolve")
                .and_then(|v| v.as_str())
                .or_else(|| rule_value.get("RESOLVE").and_then(|v| v.as_str())),
            relate: rule_value
                .get("relate")
                .and_then(|v| v.as_str())
                .or_else(|| rule_value.get("RELATE").and_then(|v| v.as_str())),
            rtype_id: rule_value
                .get("rtypeId")
                .and_then(|v| v.as_i64())
                .or_else(|| rule_value.get("RTYPE_ID").and_then(|v| v.as_i64())),
            // JSON-based FFI can express Clear: an absent key -> Leave, an explicit
            // null -> Clear (writes null), a value -> Set.
            fragment: sz_configtool_lib::helpers::field_update_str(
                &rule_value,
                &["fragment", "FRAGMENT"],
            ),
            disqualifier: sz_configtool_lib::helpers::field_update_str(
                &rule_value,
                &["disqualifier", "DISQUALIFIER"],
            ),
            tier: sz_configtool_lib::helpers::field_update_i64(&rule_value, &["tier", "TIER"]),
        };

        handle_result!(sz_configtool_lib::rules::set_rule(config, params))
    })
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
    ffi_guard("SzConfigTool_addStandardizeFunction", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in sfunc_code: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        // Direct-arg add FFI: a NULL connect_str stores JSON null; a non-null
        // pointer (including an empty string) stores that value.
        let conn_opt = if connect_str.is_null() {
            None
        } else {
            unsafe {
                match CStr::from_ptr(connect_str).to_str() {
                    Ok(s) => Some(s),
                    Err(e) => {
                        set_error(format!("Invalid UTF-8 in connect_str: {e}"), -2);
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
                        set_error(format!("Invalid UTF-8 in sfunc_desc: {e}"), -2);
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
                        set_error(format!("Invalid UTF-8 in language: {e}"), -2);
                        return SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -2,
                        };
                    }
                }
            }
        };

        match sz_configtool_lib::functions::standardize::add_standardize_function(
            config,
            code,
            sz_configtool_lib::functions::standardize::AddStandardizeFunctionParams {
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
                    set_error(format!("Failed to convert result: {e}"), -4);
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
    })
}

/// Delete a standardize function
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteStandardizeFunction(
    config_json: *const c_char,
    sfunc_code: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_deleteStandardizeFunction", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in sfunc_code: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::functions::standardize::delete_standardize_function(config, code) {
            Ok((modified_config, _record)) => match CString::new(modified_config) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to convert result: {e}"), -4);
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
    })
}

/// Get a standardize function
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getStandardizeFunction(
    config_json: *const c_char,
    sfunc_code: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_getStandardizeFunction", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in sfunc_code: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::functions::standardize::get_standardize_function(config, code) {
            Ok(value) => {
                let json_str = serde_json::to_string(&value)
                    .unwrap_or_else(|e| format!("{{\"error\": \"Failed to serialize: {e}\"}}"));
                match CString::new(json_str) {
                    Ok(c_str) => {
                        clear_error();
                        SzConfigTool_result {
                            response: c_str.into_raw(),
                            returnCode: 0,
                        }
                    }
                    Err(e) => {
                        set_error(format!("Failed to convert result: {e}"), -4);
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
    })
}

/// List all standardize functions
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_listStandardizeFunctions(
    config_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_listStandardizeFunctions", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::functions::standardize::list_standardize_functions(config) {
            Ok(vec) => {
                let json_str = serde_json::to_string(&vec)
                    .unwrap_or_else(|e| format!("{{\"error\": \"Failed to serialize: {e}\"}}"));
                match CString::new(json_str) {
                    Ok(c_str) => {
                        clear_error();
                        SzConfigTool_result {
                            response: c_str.into_raw(),
                            returnCode: 0,
                        }
                    }
                    Err(e) => {
                        set_error(format!("Failed to convert result: {e}"), -4);
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
    })
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
    ffi_guard("SzConfigTool_setStandardizeFunction", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in sfunc_code: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        // Direct-arg set FFI: a plain pointer can encode only Leave/Set (D12), so a
        // NULL connect_str leaves the stored value untouched and a non-null pointer
        // (including an empty string) sets it. Clearing a value to null is only
        // expressible via the JSON-based set APIs.
        let conn_update = if connect_str.is_null() {
            sz_configtool_lib::helpers::FieldUpdate::Leave
        } else {
            unsafe {
                match CStr::from_ptr(connect_str).to_str() {
                    Ok(s) => sz_configtool_lib::helpers::FieldUpdate::Set(s),
                    Err(e) => {
                        set_error(format!("Invalid UTF-8 in connect_str: {e}"), -2);
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
                        set_error(format!("Invalid UTF-8 in sfunc_desc: {e}"), -2);
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
                        set_error(format!("Invalid UTF-8 in language: {e}"), -2);
                        return SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -2,
                        };
                    }
                }
            }
        };

        match sz_configtool_lib::functions::standardize::set_standardize_function(
            config,
            code,
            sz_configtool_lib::functions::standardize::SetStandardizeFunctionParams {
                connect_str: conn_update,
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
                    set_error(format!("Failed to convert result: {e}"), -4);
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
    })
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
    ffi_guard("SzConfigTool_addExpressionFunction", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in efunc_code: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        // Direct-arg add FFI: a NULL connect_str stores JSON null; a non-null
        // pointer (including an empty string) stores that value.
        let conn_opt = if connect_str.is_null() {
            None
        } else {
            unsafe {
                match CStr::from_ptr(connect_str).to_str() {
                    Ok(s) => Some(s),
                    Err(e) => {
                        set_error(format!("Invalid UTF-8 in connect_str: {e}"), -2);
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
                        set_error(format!("Invalid UTF-8 in efunc_desc: {e}"), -2);
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
                        set_error(format!("Invalid UTF-8 in language: {e}"), -2);
                        return SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -2,
                        };
                    }
                }
            }
        };

        match sz_configtool_lib::functions::expression::add_expression_function(
            config,
            code,
            sz_configtool_lib::functions::expression::AddExpressionFunctionParams {
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
                    set_error(format!("Failed to convert result: {e}"), -4);
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
    })
}

/// Delete an expression function
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteExpressionFunction(
    config_json: *const c_char,
    efunc_code: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_deleteExpressionFunction", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in efunc_code: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::functions::expression::delete_expression_function(config, code) {
            Ok((modified_config, _)) => match CString::new(modified_config) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to convert result: {e}"), -4);
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
    })
}

/// Get an expression function
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getExpressionFunction(
    config_json: *const c_char,
    efunc_code: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_getExpressionFunction", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in efunc_code: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::functions::expression::get_expression_function(config, code) {
            Ok(value) => {
                let json_str = serde_json::to_string(&value)
                    .unwrap_or_else(|e| format!("{{\"error\": \"Failed to serialize: {e}\"}}"));
                match CString::new(json_str) {
                    Ok(c_str) => {
                        clear_error();
                        SzConfigTool_result {
                            response: c_str.into_raw(),
                            returnCode: 0,
                        }
                    }
                    Err(e) => {
                        set_error(format!("Failed to convert result: {e}"), -4);
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
    })
}

/// List all expression functions
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_listExpressionFunctions(
    config_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_listExpressionFunctions", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::functions::expression::list_expression_functions(config) {
            Ok(vec) => {
                let json_str = serde_json::to_string(&vec)
                    .unwrap_or_else(|e| format!("{{\"error\": \"Failed to serialize: {e}\"}}"));
                match CString::new(json_str) {
                    Ok(c_str) => {
                        clear_error();
                        SzConfigTool_result {
                            response: c_str.into_raw(),
                            returnCode: 0,
                        }
                    }
                    Err(e) => {
                        set_error(format!("Failed to convert result: {e}"), -4);
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
    })
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
    ffi_guard("SzConfigTool_setExpressionFunction", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in efunc_code: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        // Direct-arg set FFI: a plain pointer can encode only Leave/Set (D12), so a
        // NULL connect_str leaves the stored value untouched and a non-null pointer
        // (including an empty string) sets it. Clearing a value to null is only
        // expressible via the JSON-based set APIs.
        let conn_update = if connect_str.is_null() {
            sz_configtool_lib::helpers::FieldUpdate::Leave
        } else {
            unsafe {
                match CStr::from_ptr(connect_str).to_str() {
                    Ok(s) => sz_configtool_lib::helpers::FieldUpdate::Set(s),
                    Err(e) => {
                        set_error(format!("Invalid UTF-8 in connect_str: {e}"), -2);
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
                        set_error(format!("Invalid UTF-8 in efunc_desc: {e}"), -2);
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
                        set_error(format!("Invalid UTF-8 in language: {e}"), -2);
                        return SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -2,
                        };
                    }
                }
            }
        };

        match sz_configtool_lib::functions::expression::set_expression_function(
            config,
            code,
            sz_configtool_lib::functions::expression::SetExpressionFunctionParams {
                connect_str: conn_update,
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
                    set_error(format!("Failed to convert result: {e}"), -4);
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
    })
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
    ffi_guard("SzConfigTool_addComparisonFunction", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in cfunc_code: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        // Direct-arg add FFI: a NULL connect_str stores JSON null; a non-null
        // pointer (including an empty string) stores that value.
        let conn_opt = if connect_str.is_null() {
            None
        } else {
            unsafe {
                match CStr::from_ptr(connect_str).to_str() {
                    Ok(s) => Some(s),
                    Err(e) => {
                        set_error(format!("Invalid UTF-8 in connect_str: {e}"), -2);
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
                        set_error(format!("Invalid UTF-8 in cfunc_desc: {e}"), -2);
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
                        set_error(format!("Invalid UTF-8 in language: {e}"), -2);
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
                        set_error(format!("Invalid UTF-8 in anon_support: {e}"), -2);
                        return SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -2,
                        };
                    }
                }
            }
        };

        match sz_configtool_lib::functions::comparison::add_comparison_function(
            config,
            code,
            sz_configtool_lib::functions::comparison::AddComparisonFunctionParams {
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
                    set_error(format!("Failed to convert result: {e}"), -4);
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
    })
}

/// Delete a comparison function
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteComparisonFunction(
    config_json: *const c_char,
    cfunc_code: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_deleteComparisonFunction", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in cfunc_code: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::functions::comparison::delete_comparison_function(config, code) {
            Ok((modified_config, _)) => match CString::new(modified_config) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to convert result: {e}"), -4);
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
    })
}

/// Get a comparison function
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getComparisonFunction(
    config_json: *const c_char,
    cfunc_code: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_getComparisonFunction", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in cfunc_code: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::functions::comparison::get_comparison_function(config, code) {
            Ok(value) => {
                let json_str = serde_json::to_string(&value)
                    .unwrap_or_else(|e| format!("{{\"error\": \"Failed to serialize: {e}\"}}"));
                match CString::new(json_str) {
                    Ok(c_str) => {
                        clear_error();
                        SzConfigTool_result {
                            response: c_str.into_raw(),
                            returnCode: 0,
                        }
                    }
                    Err(e) => {
                        set_error(format!("Failed to convert result: {e}"), -4);
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
    })
}

/// List all comparison functions
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_listComparisonFunctions(
    config_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_listComparisonFunctions", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::functions::comparison::list_comparison_functions(config) {
            Ok(vec) => {
                let json_str = serde_json::to_string(&vec)
                    .unwrap_or_else(|e| format!("{{\"error\": \"Failed to serialize: {e}\"}}"));
                match CString::new(json_str) {
                    Ok(c_str) => {
                        clear_error();
                        SzConfigTool_result {
                            response: c_str.into_raw(),
                            returnCode: 0,
                        }
                    }
                    Err(e) => {
                        set_error(format!("Failed to convert result: {e}"), -4);
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
    })
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
    ffi_guard("SzConfigTool_setComparisonFunction", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in cfunc_code: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        // Direct-arg set FFI: a plain pointer can encode only Leave/Set (D12), so a
        // NULL connect_str leaves the stored value untouched and a non-null pointer
        // (including an empty string) sets it. Clearing a value to null is only
        // expressible via the JSON-based set APIs.
        let conn_update = if connect_str.is_null() {
            sz_configtool_lib::helpers::FieldUpdate::Leave
        } else {
            unsafe {
                match CStr::from_ptr(connect_str).to_str() {
                    Ok(s) => sz_configtool_lib::helpers::FieldUpdate::Set(s),
                    Err(e) => {
                        set_error(format!("Invalid UTF-8 in connect_str: {e}"), -2);
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
                        set_error(format!("Invalid UTF-8 in cfunc_desc: {e}"), -2);
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
                        set_error(format!("Invalid UTF-8 in language: {e}"), -2);
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
                        set_error(format!("Invalid UTF-8 in anon_support: {e}"), -2);
                        return SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -2,
                        };
                    }
                }
            }
        };

        match sz_configtool_lib::functions::comparison::set_comparison_function(
            config,
            code,
            sz_configtool_lib::functions::comparison::SetComparisonFunctionParams {
                connect_str: conn_update,
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
                    set_error(format!("Failed to convert result: {e}"), -4);
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
    })
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
    ffi_guard("SzConfigTool_addStandardizeCall", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                        set_error(format!("Invalid UTF-8 in ftype_code: {e}"), -2);
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
                        set_error(format!("Invalid UTF-8 in felem_code: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in sfunc_code: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        let params = sz_configtool_lib::calls::standardize::AddStandardizeCallParams {
            ftype_code: ftype_opt,
            felem_code: felem_opt,
            exec_order: exec_opt,
            sfunc_code: sfunc,
        };

        match sz_configtool_lib::calls::standardize::add_standardize_call(config, params) {
            Ok((modified_config, _)) => match CString::new(modified_config) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to convert result: {e}"), -4);
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
    })
}

/// Delete a standardize call
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteStandardizeCall(
    config_json: *const c_char,
    sfcall_id: i64,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_deleteStandardizeCall", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        handle_result!(
            sz_configtool_lib::calls::standardize::delete_standardize_call(config, sfcall_id)
        )
    })
}

/// Get a standardize call
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getStandardizeCall(
    config_json: *const c_char,
    sfcall_id: i64,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_getStandardizeCall", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::calls::standardize::get_standardize_call(
            config,
            sz_configtool_lib::calls::CallSelector::Id(sfcall_id),
        ) {
            Ok(value) => {
                let json_str = serde_json::to_string(&value)
                    .unwrap_or_else(|e| format!("{{\"error\": \"Failed to serialize: {e}\"}}"));
                match CString::new(json_str) {
                    Ok(c_str) => {
                        clear_error();
                        SzConfigTool_result {
                            response: c_str.into_raw(),
                            returnCode: 0,
                        }
                    }
                    Err(e) => {
                        set_error(format!("Failed to convert result: {e}"), -4);
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
    })
}

/// List all standardize calls
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_listStandardizeCalls(
    config_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_listStandardizeCalls", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::calls::standardize::list_standardize_calls(config) {
            Ok(vec) => {
                let json_str = serde_json::to_string(&vec)
                    .unwrap_or_else(|e| format!("{{\"error\": \"Failed to serialize: {e}\"}}"));
                match CString::new(json_str) {
                    Ok(c_str) => {
                        clear_error();
                        SzConfigTool_result {
                            response: c_str.into_raw(),
                            returnCode: 0,
                        }
                    }
                    Err(e) => {
                        set_error(format!("Failed to convert result: {e}"), -4);
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
    })
}

/// Set (update) a standardize call
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_setStandardizeCall(
    config_json: *const c_char,
    sfcall_id: i64,
    updates_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_setStandardizeCall", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in updates_json: {e}"), -2);
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
                set_error(format!("Invalid JSON in updates_json: {e}"), -3);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                };
            }
        };

        let params = sz_configtool_lib::calls::standardize::SetStandardizeCallParams {
            sfcall_id,
            exec_order: updates_value.get("execOrder").and_then(|v| v.as_i64()),
        };

        handle_result!(sz_configtool_lib::calls::standardize::set_standardize_call(
            config, params
        ))
    })
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
    ffi_guard("SzConfigTool_addComparisonThreshold", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in cfunc_rtnval: {e}"), -2);
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

        handle_result!(
            sz_configtool_lib::thresholds::add_comparison_threshold_by_id(
                config,
                cfunc_id,
                ftype_opt,
                rtnval,
                exec_opt,
                same_opt,
                close_opt,
                likely_opt,
                plausible_opt,
                unlikely_opt,
            )
        )
    })
}

/// Delete a comparison threshold
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteComparisonThreshold(
    config_json: *const c_char,
    cfrtn_id: i64,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_deleteComparisonThreshold", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        handle_result!(
            sz_configtool_lib::thresholds::delete_comparison_threshold_by_id(config, cfrtn_id)
        )
    })
}

/// Set (update) a comparison threshold
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_setComparisonThreshold(
    config_json: *const c_char,
    cfrtn_id: i64,
    updates_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_setComparisonThreshold", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in updates_json: {e}"), -2);
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
                set_error(format!("Invalid JSON in updates_json: {e}"), -3);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                };
            }
        };

        handle_result!(
            sz_configtool_lib::thresholds::set_comparison_threshold_by_id(
                config,
                cfrtn_id,
                updates_value.get("sameScore").and_then(|v| v.as_i64()),
                updates_value.get("closeScore").and_then(|v| v.as_i64()),
                updates_value.get("likelyScore").and_then(|v| v.as_i64()),
                updates_value.get("plausibleScore").and_then(|v| v.as_i64()),
                updates_value.get("unlikelyScore").and_then(|v| v.as_i64()),
            )
        )
    })
}

/// List all comparison thresholds
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_listComparisonThresholds(
    config_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_listComparisonThresholds", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::thresholds::list_comparison_thresholds(config) {
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
                        set_error(format!("Failed to create C string: {e}"), -4);
                        SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -4,
                        }
                    }
                },
                Err(e) => {
                    set_error(format!("Failed to serialize list: {e}"), -3);
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
    })
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
    ffi_guard("SzConfigTool_addGenericThreshold", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in plan: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in behavior: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in send_to_redo: {e}"), -2);
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
                        set_error(format!("Invalid UTF-8 in feature: {e}"), -2);
                        return SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -2,
                        };
                    }
                }
            }
        };

        handle_result!(sz_configtool_lib::thresholds::add_generic_threshold(
            config,
            sz_configtool_lib::thresholds::AddGenericThresholdParams {
                plan: Some(plan_str),
                behavior: Some(behavior_str),
                scoring_cap: Some(scoring_cap),
                candidate_cap: Some(candidate_cap),
                send_to_redo: Some(redo_str),
                feature: feature_opt,
            },
        ))
    })
}

/// Validate a generic-threshold ADD without mutating the config.
///
/// Returns the staged [`GenericThresholdCheck`](sz_configtool_lib::thresholds::GenericThresholdCheck)
/// as a versioned JSON payload (`schema` = "sz-configtool.generic-threshold-check/v1")
/// in `response` with `returnCode` 0. `returnCode` is negative only for a
/// boundary/internal error (null/invalid-UTF-8 argument or unparseable config).
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_validateGenericThreshold(
    config_json: *const c_char,
    plan: *const c_char,
    behavior: *const c_char,
    send_to_redo: *const c_char,
    feature: *const c_char, // Null = "ALL"
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_validateGenericThreshold", || {
        macro_rules! required_str {
            ($ptr:expr, $name:expr) => {
                unsafe {
                    if $ptr.is_null() {
                        set_error(format!("{} is null", $name), -1);
                        return SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -1,
                        };
                    }
                    match CStr::from_ptr($ptr).to_str() {
                        Ok(s) => s,
                        Err(e) => {
                            set_error(format!("Invalid UTF-8 in {}: {e}", $name), -2);
                            return SzConfigTool_result {
                                response: std::ptr::null_mut(),
                                returnCode: -2,
                            };
                        }
                    }
                }
            };
        }

        let config = required_str!(config_json, "config_json");
        let plan_str = required_str!(plan, "plan");
        let behavior_str = required_str!(behavior, "behavior");
        let redo_str = required_str!(send_to_redo, "send_to_redo");
        let feature_opt = if feature.is_null() {
            None
        } else {
            unsafe {
                match CStr::from_ptr(feature).to_str() {
                    Ok(s) => Some(s),
                    Err(e) => {
                        set_error(format!("Invalid UTF-8 in feature: {e}"), -2);
                        return SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -2,
                        };
                    }
                }
            }
        };

        handle_result!(
            sz_configtool_lib::thresholds::validate_generic_threshold(
                config,
                plan_str,
                behavior_str,
                redo_str,
                feature_opt,
            )
            .and_then(|check| {
                // One serializer for every surface: the root library's
                // `Serialize for GenericThresholdCheck` (schema v1).
                serde_json::to_string(&check).map_err(sz_configtool_lib::SzConfigError::from)
            })
        )
    })
}

/// Delete a generic threshold
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteGenericThreshold(
    config_json: *const c_char,
    plan: *const c_char,
    behavior: *const c_char,
    feature: *const c_char, // Null = "ALL"
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_deleteGenericThreshold", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in plan: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in behavior: {e}"), -2);
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
                        set_error(format!("Invalid UTF-8 in feature: {e}"), -2);
                        return SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -2,
                        };
                    }
                }
            }
        };

        handle_result!(sz_configtool_lib::thresholds::delete_generic_threshold(
            config,
            sz_configtool_lib::thresholds::DeleteGenericThresholdParams {
                plan: Some(plan_str),
                behavior: Some(behavior_str),
                feature: feature_opt,
            },
        ))
    })
}

/// Set (update) a generic threshold
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_setGenericThreshold(
    config_json: *const c_char,
    gplan_id: i64,
    behavior: *const c_char,
    updates_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_setGenericThreshold", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in behavior: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in updates_json: {e}"), -2);
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
                set_error(format!("Invalid JSON in updates_json: {e}"), -3);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                };
            }
        };

        // Lookup plan code from ID
        let plan_code_str = match sz_configtool_lib::helpers::lookup_gplan_code(config, gplan_id) {
            Ok(code) => code,
            Err(e) => {
                set_error(e.to_string(), -4);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -4,
                };
            }
        };

        // Parse the cap fields via the params' TryFrom so the FFI path applies the
        // SAME strict integer typing as the Rust API: a present-but-wrong-type cap
        // (e.g. {"candidateCap": "500"}) is rejected as InvalidInput rather than
        // silently coerced to None and no-op'd. The FFI supplies plan/behavior out
        // of band (gplan_id + behavior C-strings), so overwrite those two after.
        handle_result!((|| -> sz_configtool_lib::error::Result<String> {
            let mut params =
                sz_configtool_lib::thresholds::SetGenericThresholdParams::try_from(&updates_value)?;
            params.plan = Some(&plan_code_str);
            params.behavior = Some(behavior_str);
            sz_configtool_lib::thresholds::set_generic_threshold(config, params)
        })())
    })
}

/// List all generic thresholds
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_listGenericThresholds(
    config_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_listGenericThresholds", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::thresholds::list_generic_thresholds(config) {
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
                        set_error(format!("Failed to create C string: {e}"), -4);
                        SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -4,
                        }
                    }
                },
                Err(e) => {
                    set_error(format!("Failed to serialize list: {e}"), -3);
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
    })
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
    ffi_guard("SzConfigTool_getFragment", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in code_or_id: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::fragments::get_fragment(config, code) {
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
                        set_error(format!("Failed to create C string: {e}"), -4);
                        SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -4,
                        }
                    }
                },
                Err(e) => {
                    set_error(format!("Failed to serialize fragment: {e}"), -3);
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
    })
}

/// List all fragments
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_listFragments(config_json: *const c_char) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_listFragments", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::fragments::list_fragments(config) {
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
                        set_error(format!("Failed to create C string: {e}"), -4);
                        SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -4,
                        }
                    }
                },
                Err(e) => {
                    set_error(format!("Failed to serialize list: {e}"), -3);
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
    })
}

/// Add a fragment
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_addFragment(
    config_json: *const c_char,
    fragment_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_addFragment", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in fragment_json: {e}"), -2);
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
                set_error(format!("Invalid JSON in fragment_json: {e}"), -3);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                };
            }
        };

        match sz_configtool_lib::fragments::add_fragment(config, &fragment_value) {
            Ok((modified_config, _frag_id)) => match CString::new(modified_config) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to create C string: {e}"), -4);
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
    })
}

/// Delete a fragment
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteFragment(
    config_json: *const c_char,
    fragment_code: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_deleteFragment", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in fragment_code: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        handle_result!(sz_configtool_lib::fragments::delete_fragment(config, code))
    })
}

// ===== Data Source Operations =====

/// Get a data source by code
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getDataSource(
    config_json: *const c_char,
    code: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_getDataSource", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in code: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::datasources::get_data_source(config, ds_code) {
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
                        set_error(format!("Failed to create C string: {e}"), -4);
                        SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -4,
                        }
                    }
                },
                Err(e) => {
                    set_error(format!("Failed to serialize data source: {e}"), -3);
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
    })
}

/// Set (update) a data source
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_setDataSource(
    config_json: *const c_char,
    code: *const c_char,
    updates_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_setDataSource", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in code: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in updates_json: {e}"), -2);
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
                set_error(format!("Invalid JSON in updates_json: {e}"), -3);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                };
            }
        };

        // Build params with code from parameter and optional updates from JSON
        let params = sz_configtool_lib::datasources::SetDataSourceParams {
            code: ds_code,
            retention_level: updates_value.get("retentionLevel").and_then(|v| v.as_str()),
        };

        handle_result!(sz_configtool_lib::datasources::set_data_source(
            config, params
        ))
    })
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
    ffi_guard("SzConfigTool_addFeature", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in feature_code: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in feature_json: {e}"), -2);
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
                set_error(format!("Invalid JSON in feature_json: {e}"), -3);
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

        match sz_configtool_lib::features::add_feature(
            config,
            sz_configtool_lib::features::AddFeatureParams {
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
                // #37: honour a caller-supplied FTYPE_ID from the feature JSON.
                id: feature_config.get("id").and_then(|v| v.as_i64()),
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
                    set_error(format!("Failed to create C string: {e}"), -4);
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
    })
}

/// Delete a feature by code or ID
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteFeature(
    config_json: *const c_char,
    feature_code_or_id: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_deleteFeature", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in feature_code_or_id: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        handle_result!(sz_configtool_lib::features::delete_feature(
            config, code_or_id
        ))
    })
}

/// Set (update) feature properties with JSON
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_setFeature(
    config_json: *const c_char,
    feature_code_or_id: *const c_char,
    updates_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_setFeature", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in feature_code_or_id: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in updates_json: {e}"), -2);
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
                set_error(format!("Invalid JSON in updates_json: {e}"), -3);
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

        handle_result!(sz_configtool_lib::features::set_feature(
            config,
            sz_configtool_lib::features::SetFeatureParams {
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
    })
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
    ffi_guard("SzConfigTool_addBehaviorOverride", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in feature_code: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in usage_type: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in behavior: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        handle_result!(
            sz_configtool_lib::behavior_overrides::add_behavior_override(
                config,
                sz_configtool_lib::behavior_overrides::AddBehaviorOverrideParams::new(
                    feature, utype, bhvr
                )
            )
        )
    })
}

/// Delete a behavior override
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteBehaviorOverride(
    config_json: *const c_char,
    feature_code: *const c_char,
    usage_type: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_deleteBehaviorOverride", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in feature_code: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in usage_type: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        handle_result!(
            sz_configtool_lib::behavior_overrides::delete_behavior_override(config, feature, utype)
        )
    })
}

/// List all behavior overrides
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_listBehaviorOverrides(
    config_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_listBehaviorOverrides", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        let result = sz_configtool_lib::behavior_overrides::list_behavior_overrides(config)
            .and_then(|vec| {
                serde_json::to_string(&vec).map_err(|e| SzConfigError::JsonParse(e.to_string()))
            });
        handle_result!(result)
    })
}

/// List behavior overrides in the resolved display shape
///
/// Returns a JSON array of `{ "feature", "usageType", "behavior" }` objects,
/// sorted by `(FTYPE_ID, UTYPE_CODE)`.
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_listBehaviorOverridesResolved(
    config_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_listBehaviorOverridesResolved", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        let result =
            sz_configtool_lib::behavior_overrides::list_behavior_overrides_resolved(config)
                .and_then(|vec| {
                    serde_json::to_string(&vec).map_err(|e| SzConfigError::JsonParse(e.to_string()))
                });
        handle_result!(result)
    })
}

/// Validate the top-level structure of a config document
///
/// Returns `"OK"` with return code 0 when the document is structurally a config
/// document; otherwise returns an error result (see `SzConfigTool_getLastError`).
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_validateConfig(config_json: *const c_char) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_validateConfig", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        let result =
            sz_configtool_lib::validation::validate_config(config).map(|()| "OK".to_string());
        handle_result!(result)
    })
}

/// Render a config document to its canonical export form
///
/// Recursively sorts all object keys and pretty-prints at `indent` spaces per
/// level. `indent` is required; a negative value is treated as 0.
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_renderConfig(
    config_json: *const c_char,
    indent: i64,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_renderConfig", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        let indent = if indent < 0 { 0usize } else { indent as usize };
        let result = sz_configtool_lib::export::render_config(config, indent);
        handle_result!(result)
    })
}

// ===== Element Operations =====

/// Add an element with JSON configuration
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_addElement(
    config_json: *const c_char,
    element_code: *const c_char,
    element_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_addElement", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in element_code: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in element_json: {e}"), -2);
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
                set_error(format!("Invalid JSON in element_json: {e}"), -3);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                };
            }
        };

        // Build params from code and JSON config
        let params = sz_configtool_lib::elements::AddElementParams {
            code,
            description: element_config
                .get("description")
                .and_then(|v| v.as_str())
                .or_else(|| element_config.get("FELEM_DESC").and_then(|v| v.as_str())),
            data_type: element_config
                .get("dataType")
                .and_then(|v| v.as_str())
                .or_else(|| element_config.get("DATA_TYPE").and_then(|v| v.as_str())),
            // #37: honour a caller-supplied FELEM_ID from the element JSON.
            id: element_config.get("id").and_then(|v| v.as_i64()),
        };

        handle_result!(sz_configtool_lib::elements::add_element(config, params))
    })
}

/// Delete an element by code
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteElement(
    config_json: *const c_char,
    element_code: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_deleteElement", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in element_code: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        handle_result!(sz_configtool_lib::elements::delete_element(config, code))
    })
}

/// Set (update) element properties with JSON
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_setElement(
    config_json: *const c_char,
    element_code: *const c_char,
    updates_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_setElement", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in element_code: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in updates_json: {e}"), -2);
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
                set_error(format!("Invalid JSON in updates_json: {e}"), -3);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                };
            }
        };

        // Build params from code and JSON updates
        let params = sz_configtool_lib::elements::SetElementParams {
            code,
            description: updates_config
                .get("description")
                .and_then(|v| v.as_str())
                .or_else(|| updates_config.get("FELEM_DESC").and_then(|v| v.as_str())),
            data_type: updates_config
                .get("dataType")
                .and_then(|v| v.as_str())
                .or_else(|| updates_config.get("DATA_TYPE").and_then(|v| v.as_str())),
        };

        handle_result!(sz_configtool_lib::elements::set_element(config, params))
    })
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
    ffi_guard("SzConfigTool_addExpressionCall", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                        set_error(format!("Invalid UTF-8 in ftype_code: {e}"), -2);
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
                        set_error(format!("Invalid UTF-8 in felem_code: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in efunc_code: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in element_list_json: {e}"), -2);
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
                        set_error(format!("Invalid UTF-8 in expression_feature: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in is_virtual: {e}"), -2);
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
                set_error(format!("Invalid JSON in element_list_json: {e}"), -3);
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

        let call_params = sz_configtool_lib::calls::expression::AddExpressionCallParams {
            efunc_code: efunc,
            element_list,
            ftype_code: ftype_opt,
            felem_code: felem_opt,
            exec_order: exec_opt,
            expression_feature: expr_feat_opt,
            is_virtual: virtual_str,
        };

        match sz_configtool_lib::calls::expression::add_expression_call(config, call_params) {
            Ok((modified_config, _record)) => match CString::new(modified_config) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to create C string: {e}"), -4);
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
    })
}

/// Delete an expression call by ID
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteExpressionCall(
    config_json: *const c_char,
    efcall_id: i64,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_deleteExpressionCall", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        handle_result!(
            sz_configtool_lib::calls::expression::delete_expression_call(config, efcall_id)
        )
    })
}

/// Get an expression call by ID
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getExpressionCall(
    config_json: *const c_char,
    efcall_id: i64,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_getExpressionCall", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::calls::expression::get_expression_call(
            config,
            sz_configtool_lib::calls::CallSelector::Id(efcall_id),
        ) {
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
                        set_error(format!("Failed to create C string: {e}"), -4);
                        SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -4,
                        }
                    }
                },
                Err(e) => {
                    set_error(format!("Failed to serialize record: {e}"), -3);
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
    })
}

/// List all expression calls
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_listExpressionCalls(
    config_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_listExpressionCalls", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::calls::expression::list_expression_calls(config) {
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
                        set_error(format!("Failed to create C string: {e}"), -4);
                        SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -4,
                        }
                    }
                },
                Err(e) => {
                    set_error(format!("Failed to serialize list: {e}"), -3);
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
    })
}

/// Set (update) an expression call
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_setExpressionCall(
    config_json: *const c_char,
    efcall_id: i64,
    updates_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_setExpressionCall", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in updates_json: {e}"), -2);
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
                set_error(format!("Invalid JSON in updates_json: {e}"), -3);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                };
            }
        };

        let params = sz_configtool_lib::calls::expression::SetExpressionCallParams {
            efcall_id,
            exec_order: updates_value.get("execOrder").and_then(|v| v.as_i64()),
        };

        handle_result!(sz_configtool_lib::calls::expression::set_expression_call(
            config, params
        ))
    })
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
    ffi_guard("SzConfigTool_addComparisonCall", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in ftype_code: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in cfunc_code: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in element_list_json: {e}"), -2);
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
                set_error(format!("Invalid JSON in element_list_json: {e}"), -3);
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

        match sz_configtool_lib::calls::comparison::add_comparison_call(
            config,
            sz_configtool_lib::calls::comparison::AddComparisonCallParams {
                ftype_code: ftype.to_string(),
                cfunc_code: cfunc.to_string(),
                element_list,
                // CFCALL_ID is not exposed over this fixed C signature; always
                // auto-assign (#37).
                id: None,
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
                    set_error(format!("Failed to create C string: {e}"), -4);
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
    })
}

/// Delete a comparison call by ID
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteComparisonCall(
    config_json: *const c_char,
    cfcall_id: i64,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_deleteComparisonCall", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        handle_result!(
            sz_configtool_lib::calls::comparison::delete_comparison_call(config, cfcall_id)
        )
    })
}

/// Get a comparison call by ID
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getComparisonCall(
    config_json: *const c_char,
    cfcall_id: i64,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_getComparisonCall", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::calls::comparison::get_comparison_call(
            config,
            sz_configtool_lib::calls::CallSelector::Id(cfcall_id),
        ) {
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
                        set_error(format!("Failed to create C string: {e}"), -4);
                        SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -4,
                        }
                    }
                },
                Err(e) => {
                    set_error(format!("Failed to serialize record: {e}"), -3);
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
    })
}

/// List all comparison calls
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_listComparisonCalls(
    config_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_listComparisonCalls", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::calls::comparison::list_comparison_calls(config) {
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
                        set_error(format!("Failed to create C string: {e}"), -4);
                        SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -4,
                        }
                    }
                },
                Err(e) => {
                    set_error(format!("Failed to serialize list: {e}"), -3);
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
    })
}

/// Set (update) a comparison call
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_setComparisonCall(
    config_json: *const c_char,
    cfcall_id: i64,
    updates_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_setComparisonCall", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in updates_json: {e}"), -2);
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
                set_error(format!("Invalid JSON in updates_json: {e}"), -3);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                };
            }
        };

        let params = sz_configtool_lib::calls::comparison::SetComparisonCallParams {
            cfcall_id,
            exec_order: updates_value.get("execOrder").and_then(|v| v.as_i64()),
        };

        handle_result!(sz_configtool_lib::calls::comparison::set_comparison_call(
            config, params
        ))
    })
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
    ffi_guard("SzConfigTool_addDistinctCall", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in ftype_code: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in dfunc_code: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in element_list_json: {e}"), -2);
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
                set_error(format!("Failed to parse element_list_json: {e}"), -3);
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

        match sz_configtool_lib::calls::distinct::add_distinct_call(
            config,
            sz_configtool_lib::calls::distinct::AddDistinctCallParams {
                ftype_code: ftype.to_string(),
                dfunc_code: dfunc.to_string(),
                element_list,
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
                    set_error(format!("Failed to create C string: {e}"), -4);
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
    })
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
    ffi_guard("SzConfigTool_deleteDistinctCall", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::calls::distinct::delete_distinct_call(config, dfcall_id) {
            Ok(modified_config) => match CString::new(modified_config) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to create C string: {e}"), -4);
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
    })
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
    ffi_guard("SzConfigTool_getDistinctCall", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::calls::distinct::get_distinct_call(
            config,
            sz_configtool_lib::calls::CallSelector::Id(dfcall_id),
        ) {
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
                        set_error(format!("Failed to create C string: {e}"), -4);
                        SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -4,
                        }
                    }
                },
                Err(e) => {
                    set_error(format!("Failed to serialize record: {e}"), -3);
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
    })
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
    ffi_guard("SzConfigTool_listDistinctCalls", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::calls::distinct::list_distinct_calls(config) {
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
                        set_error(format!("Failed to create C string: {e}"), -4);
                        SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -4,
                        }
                    }
                },
                Err(e) => {
                    set_error(format!("Failed to serialize list: {e}"), -3);
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
    })
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
    ffi_guard("SzConfigTool_setDistinctCall", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in updates_json: {e}"), -2);
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
                set_error(format!("Failed to parse updates_json: {e}"), -3);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -3,
                };
            }
        };

        let params = sz_configtool_lib::calls::distinct::SetDistinctCallParams {
            dfcall_id,
            exec_order: updates_value.get("execOrder").and_then(|v| v.as_i64()),
        };

        match sz_configtool_lib::calls::distinct::set_distinct_call(config, params) {
            Ok(modified_config) => match CString::new(modified_config) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to create C string: {e}"), -4);
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
    })
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
    ffi_guard("SzConfigTool_addMatchingFunction", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in rtype_code: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in matching_func: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::functions::matching::add_matching_function(config, rtype, matching)
        {
            Ok((modified_config, _record)) => match CString::new(modified_config) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to create C string: {e}"), -4);
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
    })
}

/// Delete a matching function (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteMatchingFunction(
    config_json: *const c_char,
    rtype_code: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_deleteMatchingFunction", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in rtype_code: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::functions::matching::delete_matching_function(config, rtype) {
            Ok((modified_config, _record)) => match CString::new(modified_config) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to create C string: {e}"), -4);
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
    })
}

/// Get a matching function (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getMatchingFunction(
    config_json: *const c_char,
    rtype_code: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_getMatchingFunction", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in rtype_code: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::functions::matching::get_matching_function(config, rtype) {
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
                        set_error(format!("Failed to create C string: {e}"), -4);
                        SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -4,
                        }
                    }
                },
                Err(e) => {
                    set_error(format!("Failed to serialize record: {e}"), -3);
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
    })
}

/// List all matching functions (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_listMatchingFunctions(
    config_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_listMatchingFunctions", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::functions::matching::list_matching_functions(config) {
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
                        set_error(format!("Failed to create C string: {e}"), -4);
                        SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -4,
                        }
                    }
                },
                Err(e) => {
                    set_error(format!("Failed to serialize list: {e}"), -3);
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
    })
}

/// Set (update) a matching function (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_setMatchingFunction(
    config_json: *const c_char,
    rtype_code: *const c_char,
    matching_func: *const c_char, // NULL allowed
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_setMatchingFunction", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in rtype_code: {e}"), -2);
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
                        set_error(format!("Invalid UTF-8 in matching_func: {e}"), -2);
                        return SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -2,
                        };
                    }
                }
            }
        };

        match sz_configtool_lib::functions::matching::set_matching_function(
            config,
            rtype,
            matching_opt,
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
                    set_error(format!("Failed to create C string: {e}"), -4);
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
    })
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
    ffi_guard("SzConfigTool_addDistinctFunction", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in dfunc_code: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        // Direct-arg add FFI: a NULL connect_str stores JSON null; a non-null
        // pointer (including an empty string) stores that value.
        let connect = if connect_str.is_null() {
            None
        } else {
            unsafe {
                match CStr::from_ptr(connect_str).to_str() {
                    Ok(s) => Some(s),
                    Err(e) => {
                        set_error(format!("Invalid UTF-8 in connect_str: {e}"), -2);
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
                        set_error(format!("Invalid UTF-8 in dfunc_desc: {e}"), -2);
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
                        set_error(format!("Invalid UTF-8 in language: {e}"), -2);
                        return SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -2,
                        };
                    }
                }
            }
        };

        match sz_configtool_lib::functions::distinct::add_distinct_function(
            config,
            dfunc,
            sz_configtool_lib::functions::distinct::AddDistinctFunctionParams {
                connect_str: connect,
                description: desc_opt,
                language: lang_opt,
                anon_support: None,
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
                    set_error(format!("Failed to create C string: {e}"), -4);
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
    })
}

/// Delete a distinct function by code
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteDistinctFunction(
    config_json: *const c_char,
    dfunc_code: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_deleteDistinctFunction", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in dfunc_code: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::functions::distinct::delete_distinct_function(config, dfunc) {
            Ok((modified_config, _record)) => match CString::new(modified_config) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to create C string: {e}"), -4);
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
    })
}

/// Get a distinct function by code
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getDistinctFunction(
    config_json: *const c_char,
    dfunc_code: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_getDistinctFunction", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in dfunc_code: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::functions::distinct::get_distinct_function(config, dfunc) {
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
                        set_error(format!("Failed to create C string: {e}"), -4);
                        SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -4,
                        }
                    }
                },
                Err(e) => {
                    set_error(format!("Failed to serialize record: {e}"), -3);
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
    })
}

/// List all distinct functions
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_listDistinctFunctions(
    config_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_listDistinctFunctions", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::functions::distinct::list_distinct_functions(config) {
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
                        set_error(format!("Failed to create C string: {e}"), -4);
                        SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -4,
                        }
                    }
                },
                Err(e) => {
                    set_error(format!("Failed to serialize list: {e}"), -3);
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
    })
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
    ffi_guard("SzConfigTool_setDistinctFunction", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in dfunc_code: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        // Direct-arg set FFI: a plain pointer can encode only Leave/Set (D12), so a
        // NULL connect_str leaves the stored value untouched and a non-null pointer
        // (including an empty string) sets it. Clearing a value to null is only
        // expressible via the JSON-based set APIs.
        let connect_update = if connect_str.is_null() {
            sz_configtool_lib::helpers::FieldUpdate::Leave
        } else {
            unsafe {
                match CStr::from_ptr(connect_str).to_str() {
                    Ok(s) => sz_configtool_lib::helpers::FieldUpdate::Set(s),
                    Err(e) => {
                        set_error(format!("Invalid UTF-8 in connect_str: {e}"), -2);
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
                        set_error(format!("Invalid UTF-8 in dfunc_desc: {e}"), -2);
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
                        set_error(format!("Invalid UTF-8 in language: {e}"), -2);
                        return SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -2,
                        };
                    }
                }
            }
        };

        match sz_configtool_lib::functions::distinct::set_distinct_function(
            config,
            dfunc,
            sz_configtool_lib::functions::distinct::SetDistinctFunctionParams {
                connect_str: connect_update,
                description: desc_opt,
                language: lang_opt,
                anon_support: None,
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
                    set_error(format!("Failed to create C string: {e}"), -4);
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
    })
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
    ffi_guard("SzConfigTool_addCandidateFunction", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in rtype_code: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in candidate_func: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::functions::candidate::add_candidate_function(
            config, rtype, candidate,
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
                    set_error(format!("Failed to create C string: {e}"), -4);
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
    })
}

/// Delete a candidate function (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteCandidateFunction(
    config_json: *const c_char,
    rtype_code: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_deleteCandidateFunction", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in rtype_code: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::functions::candidate::delete_candidate_function(config, rtype) {
            Ok((modified_config, _record)) => match CString::new(modified_config) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to create C string: {e}"), -4);
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
    })
}

/// Get a candidate function (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getCandidateFunction(
    config_json: *const c_char,
    rtype_code: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_getCandidateFunction", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in rtype_code: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::functions::candidate::get_candidate_function(config, rtype) {
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
                        set_error(format!("Failed to create C string: {e}"), -4);
                        SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -4,
                        }
                    }
                },
                Err(e) => {
                    set_error(format!("Failed to serialize record: {e}"), -3);
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
    })
}

/// List all candidate functions (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_listCandidateFunctions(
    config_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_listCandidateFunctions", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::functions::candidate::list_candidate_functions(config) {
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
                        set_error(format!("Failed to create C string: {e}"), -4);
                        SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -4,
                        }
                    }
                },
                Err(e) => {
                    set_error(format!("Failed to serialize list: {e}"), -3);
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
    })
}

/// Set (update) a candidate function (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_setCandidateFunction(
    config_json: *const c_char,
    rtype_code: *const c_char,
    candidate_func: *const c_char, // NULL allowed
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_setCandidateFunction", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in rtype_code: {e}"), -2);
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
                        set_error(format!("Invalid UTF-8 in candidate_func: {e}"), -2);
                        return SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -2,
                        };
                    }
                }
            }
        };

        match sz_configtool_lib::functions::candidate::set_candidate_function(
            config,
            rtype,
            candidate_opt,
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
                    set_error(format!("Failed to create C string: {e}"), -4);
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
    })
}

// --- VALIDATION FUNCTIONS ---

/// Add a validation function (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_addValidationFunction(
    config_json: *const c_char,
    attr_code: *const c_char,
    validation_func: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_addValidationFunction", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in attr_code: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in validation_func: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::functions::validation::add_validation_function(
            config, attr, validation,
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
                    set_error(format!("Failed to create C string: {e}"), -4);
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
    })
}

/// Delete a validation function (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteValidationFunction(
    config_json: *const c_char,
    attr_code: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_deleteValidationFunction", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in attr_code: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::functions::validation::delete_validation_function(config, attr) {
            Ok((modified_config, _record)) => match CString::new(modified_config) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to create C string: {e}"), -4);
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
    })
}

/// Get a validation function (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getValidationFunction(
    config_json: *const c_char,
    attr_code: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_getValidationFunction", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in attr_code: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::functions::validation::get_validation_function(config, attr) {
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
                        set_error(format!("Failed to create C string: {e}"), -4);
                        SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -4,
                        }
                    }
                },
                Err(e) => {
                    set_error(format!("Failed to serialize record: {e}"), -3);
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
    })
}

/// List all validation functions (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_listValidationFunctions(
    config_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_listValidationFunctions", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::functions::validation::list_validation_functions(config) {
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
                        set_error(format!("Failed to create C string: {e}"), -4);
                        SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -4,
                        }
                    }
                },
                Err(e) => {
                    set_error(format!("Failed to serialize list: {e}"), -3);
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
    })
}

/// Set (update) a validation function (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_setValidationFunction(
    config_json: *const c_char,
    attr_code: *const c_char,
    validation_func: *const c_char, // NULL allowed
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_setValidationFunction", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in attr_code: {e}"), -2);
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
                        set_error(format!("Invalid UTF-8 in validation_func: {e}"), -2);
                        return SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -2,
                        };
                    }
                }
            }
        };

        match sz_configtool_lib::functions::validation::set_validation_function(
            config,
            attr,
            validation_opt,
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
                    set_error(format!("Failed to create C string: {e}"), -4);
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
    })
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
    ffi_guard("SzConfigTool_addScoringFunction", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in rtype_code: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in scoring_func: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::functions::scoring::add_scoring_function(config, rtype, scoring) {
            Ok((modified_config, _record)) => match CString::new(modified_config) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to create C string: {e}"), -4);
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
    })
}

/// Delete a scoring function (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteScoringFunction(
    config_json: *const c_char,
    rtype_code: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_deleteScoringFunction", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in rtype_code: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::functions::scoring::delete_scoring_function(config, rtype) {
            Ok((modified_config, _record)) => match CString::new(modified_config) {
                Ok(c_str) => {
                    clear_error();
                    SzConfigTool_result {
                        response: c_str.into_raw(),
                        returnCode: 0,
                    }
                }
                Err(e) => {
                    set_error(format!("Failed to create C string: {e}"), -4);
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
    })
}

/// Get a scoring function (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getScoringFunction(
    config_json: *const c_char,
    rtype_code: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_getScoringFunction", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in rtype_code: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::functions::scoring::get_scoring_function(config, rtype) {
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
                        set_error(format!("Failed to create C string: {e}"), -4);
                        SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -4,
                        }
                    }
                },
                Err(e) => {
                    set_error(format!("Failed to serialize record: {e}"), -3);
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
    })
}

/// List all scoring functions (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_listScoringFunctions(
    config_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_listScoringFunctions", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -2,
                    };
                }
            }
        };

        match sz_configtool_lib::functions::scoring::list_scoring_functions(config) {
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
                        set_error(format!("Failed to create C string: {e}"), -4);
                        SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -4,
                        }
                    }
                },
                Err(e) => {
                    set_error(format!("Failed to serialize list: {e}"), -3);
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
    })
}

/// Set (update) a scoring function (placeholder - not implemented)
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_setScoringFunction(
    config_json: *const c_char,
    rtype_code: *const c_char,
    scoring_func: *const c_char, // NULL allowed
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_setScoringFunction", || {
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
                    set_error(format!("Invalid UTF-8 in config_json: {e}"), -2);
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
                    set_error(format!("Invalid UTF-8 in rtype_code: {e}"), -2);
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
                        set_error(format!("Invalid UTF-8 in scoring_func: {e}"), -2);
                        return SzConfigTool_result {
                            response: std::ptr::null_mut(),
                            returnCode: -2,
                        };
                    }
                }
            }
        };

        match sz_configtool_lib::functions::scoring::set_scoring_function(
            config,
            rtype,
            scoring_opt,
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
                    set_error(format!("Failed to create C string: {e}"), -4);
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
    })
}

// ============================================================================
// Wave 4A additions (#38): feature-element mutators, settings, cascade deletes
// ============================================================================

/// Read a required C string arg, returning an error `SzConfigTool_result` from
/// the enclosing function on null or invalid UTF-8.
macro_rules! ffi_required_str {
    ($ptr:expr, $name:expr) => {{
        if $ptr.is_null() {
            set_error(format!("{} is null", $name), -1);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -1,
            };
        }
        match unsafe { CStr::from_ptr($ptr) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_error(format!("Invalid UTF-8 in {}: {e}", $name), -2);
                return SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                };
            }
        }
    }};
}

/// Add an element to a feature (append a CFG_FBOM row).
///
/// `options_json` may be null; when present it is a JSON object that may carry
/// `displayLevel` (integer), `displayDelim` (string), and `derived`
/// (`"Yes"`/`"No"`).
///
/// # Safety
/// `config_json`, `feature_code`, and `element_code` must be valid
/// null-terminated C strings; `options_json` may be null or a valid C string.
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_addElementToFeature(
    config_json: *const c_char,
    feature_code: *const c_char,
    element_code: *const c_char,
    options_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_addElementToFeature", || {
        let config = ffi_required_str!(config_json, "config_json");
        let feature = ffi_required_str!(feature_code, "feature_code");
        let element = ffi_required_str!(element_code, "element_code");

        let options: serde_json::Value = if options_json.is_null() {
            serde_json::Value::Null
        } else {
            let s = ffi_required_str!(options_json, "options_json");
            match serde_json::from_str(s) {
                Ok(v) => v,
                Err(e) => {
                    set_error(format!("Invalid JSON in options_json: {e}"), -3);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -3,
                    };
                }
            }
        };

        let params = sz_configtool_lib::elements::AddElementToFeatureParams {
            feature_code: feature,
            element_code: element,
            display_level: options.get("displayLevel").and_then(|v| v.as_i64()),
            display_delim: options.get("displayDelim").and_then(|v| v.as_str()),
            derived: options.get("derived").and_then(|v| v.as_str()),
        };

        handle_result!(sz_configtool_lib::elements::add_element_to_feature(
            config, params
        ))
    })
}

/// Delete a single feature-element mapping (one CFG_FBOM row).
///
/// # Safety
/// All pointers must be valid null-terminated C strings.
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteElementFromFeature(
    config_json: *const c_char,
    feature_code: *const c_char,
    element_code: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_deleteElementFromFeature", || {
        let config = ffi_required_str!(config_json, "config_json");
        let feature = ffi_required_str!(feature_code, "feature_code");
        let element = ffi_required_str!(element_code, "element_code");
        handle_result!(sz_configtool_lib::elements::delete_element_from_feature(
            config, feature, element
        ))
    })
}

/// Set (create or overwrite) a named configuration setting.
///
/// The `value` parameter is a JSON-encoded value: it is parsed with
/// `serde_json` and stored verbatim as its typed value (an integer stays an
/// integer, a JSON string stays a string). A bare string must therefore be
/// quoted JSON (e.g. `"\"hello\""`). Invalid JSON returns an error — there is
/// no string fallback.
///
/// # Safety
/// All pointers must be valid null-terminated C strings.
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_setSetting(
    config_json: *const c_char,
    name: *const c_char,
    value: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_setSetting", || {
        let config = ffi_required_str!(config_json, "config_json");
        let name = ffi_required_str!(name, "name");
        let value = ffi_required_str!(value, "value");
        handle_result!((|| {
            let value: serde_json::Value = serde_json::from_str(value).map_err(|e| {
                sz_configtool_lib::error::SzConfigError::InvalidInput(format!(
                    "value is not valid JSON: {e}"
                ))
            })?;
            sz_configtool_lib::settings::set_setting(config, name, value)
        })())
    })
}

/// Delete a comparison function and all dependent rows (cascade).
///
/// # Safety
/// All pointers must be valid null-terminated C strings.
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteComparisonFunctionCascade(
    config_json: *const c_char,
    cfunc_code: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_deleteComparisonFunctionCascade", || {
        let config = ffi_required_str!(config_json, "config_json");
        let code = ffi_required_str!(cfunc_code, "cfunc_code");
        handle_result!(
            sz_configtool_lib::functions::comparison::delete_comparison_function_cascade(
                config, code
            )
            .map(|(json, _)| json)
        )
    })
}

/// Delete an expression function and all dependent rows (cascade).
///
/// # Safety
/// All pointers must be valid null-terminated C strings.
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteExpressionFunctionCascade(
    config_json: *const c_char,
    efunc_code: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_deleteExpressionFunctionCascade", || {
        let config = ffi_required_str!(config_json, "config_json");
        let code = ffi_required_str!(efunc_code, "efunc_code");
        handle_result!(
            sz_configtool_lib::functions::expression::delete_expression_function_cascade(
                config, code
            )
            .map(|(json, _)| json)
        )
    })
}

/// Delete a standardize function and all dependent rows (cascade).
///
/// # Safety
/// All pointers must be valid null-terminated C strings.
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteStandardizeFunctionCascade(
    config_json: *const c_char,
    sfunc_code: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_deleteStandardizeFunctionCascade", || {
        let config = ffi_required_str!(config_json, "config_json");
        let code = ffi_required_str!(sfunc_code, "sfunc_code");
        handle_result!(
            sz_configtool_lib::functions::standardize::delete_standardize_function_cascade(
                config, code
            )
            .map(|(json, _)| json)
        )
    })
}

// ============================================================================
// Wave 4B additions (#40): by-feature call gets and code-addressed call-element
// deletes.
//
// The `*CallByFeature` gets resolve a feature code to the call bound to it
// (scanning CFG_*CALL by FTYPE_ID), fixing the historical bug where the feature
// id was used directly as a call id. The `delete*CallElement` wrappers address
// the element by (feature code + element code) and derive the BOM EXEC_ORDER
// internally, so no pre-resolved exec_order is passed over the ABI. These
// call-element deletes are new FFI surface (there were no prior wrappers).
// ============================================================================

/// Serialize a single call `Value` result into an FFI result.
macro_rules! ffi_json_value {
    ($result:expr) => {
        match $result {
            Ok(value) => match serde_json::to_string(&value) {
                Ok(s) => handle_result!(Ok::<String, sz_configtool_lib::error::SzConfigError>(s)),
                Err(e) => {
                    set_error(format!("Failed to serialize result: {e}"), -3);
                    SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -3,
                    }
                }
            },
            Err(e) => {
                set_error(format!("{e}"), -2);
                SzConfigTool_result {
                    response: std::ptr::null_mut(),
                    returnCode: -2,
                }
            }
        }
    };
}

/// Get the comparison call bound to a feature (by feature code).
///
/// # Safety
/// All pointers must be valid null-terminated C strings.
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getComparisonCallByFeature(
    config_json: *const c_char,
    feature_code: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_getComparisonCallByFeature", || {
        let config = ffi_required_str!(config_json, "config_json");
        let feature = ffi_required_str!(feature_code, "feature_code");
        ffi_json_value!(sz_configtool_lib::calls::comparison::get_comparison_call(
            config,
            sz_configtool_lib::calls::CallSelector::Feature(feature)
        ))
    })
}

/// Get the distinct call bound to a feature (by feature code).
///
/// # Safety
/// All pointers must be valid null-terminated C strings.
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getDistinctCallByFeature(
    config_json: *const c_char,
    feature_code: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_getDistinctCallByFeature", || {
        let config = ffi_required_str!(config_json, "config_json");
        let feature = ffi_required_str!(feature_code, "feature_code");
        ffi_json_value!(sz_configtool_lib::calls::distinct::get_distinct_call(
            config,
            sz_configtool_lib::calls::CallSelector::Feature(feature)
        ))
    })
}

/// Get the standardize call bound to a feature (by feature code).
///
/// Errors if the feature has more than one standardize call (address by id).
///
/// # Safety
/// All pointers must be valid null-terminated C strings.
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getStandardizeCallByFeature(
    config_json: *const c_char,
    feature_code: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_getStandardizeCallByFeature", || {
        let config = ffi_required_str!(config_json, "config_json");
        let feature = ffi_required_str!(feature_code, "feature_code");
        ffi_json_value!(sz_configtool_lib::calls::standardize::get_standardize_call(
            config,
            sz_configtool_lib::calls::CallSelector::Feature(feature)
        ))
    })
}

/// Get the expression call bound to a feature (by feature code).
///
/// Errors if the feature has more than one expression call (address by id).
///
/// # Safety
/// All pointers must be valid null-terminated C strings.
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_getExpressionCallByFeature(
    config_json: *const c_char,
    feature_code: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_getExpressionCallByFeature", || {
        let config = ffi_required_str!(config_json, "config_json");
        let feature = ffi_required_str!(feature_code, "feature_code");
        ffi_json_value!(sz_configtool_lib::calls::expression::get_expression_call(
            config,
            sz_configtool_lib::calls::CallSelector::Feature(feature)
        ))
    })
}

/// Delete a comparison call element by (feature code + element code).
///
/// The BOM `EXEC_ORDER` is derived internally.
///
/// # Safety
/// All pointers must be valid null-terminated C strings.
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteComparisonCallElement(
    config_json: *const c_char,
    feature_code: *const c_char,
    element_code: *const c_char,
    element_feature: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_deleteComparisonCallElement", || {
        let config = ffi_required_str!(config_json, "config_json");
        let feature = ffi_required_str!(feature_code, "feature_code");
        let element = ffi_required_str!(element_code, "element_code");
        let elem_feature = if element_feature.is_null() {
            None
        } else {
            match unsafe { CStr::from_ptr(element_feature) }.to_str() {
                Ok(s) => Some(s),
                Err(e) => {
                    set_error(format!("Invalid UTF-8 in element_feature: {e}"), -1);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -1,
                    };
                }
            }
        };
        handle_result!(
            sz_configtool_lib::calls::comparison::delete_comparison_call_element(
                config,
                sz_configtool_lib::calls::CallSelector::Feature(feature),
                element,
                elem_feature,
            )
        )
    })
}

/// Delete a distinct call element by (feature code + element code).
///
/// The BOM `EXEC_ORDER` is derived internally.
///
/// # Safety
/// All pointers must be valid null-terminated C strings.
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteDistinctCallElement(
    config_json: *const c_char,
    feature_code: *const c_char,
    element_code: *const c_char,
    element_feature: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_deleteDistinctCallElement", || {
        let config = ffi_required_str!(config_json, "config_json");
        let feature = ffi_required_str!(feature_code, "feature_code");
        let element = ffi_required_str!(element_code, "element_code");
        let elem_feature = if element_feature.is_null() {
            None
        } else {
            match unsafe { CStr::from_ptr(element_feature) }.to_str() {
                Ok(s) => Some(s),
                Err(e) => {
                    set_error(format!("Invalid UTF-8 in element_feature: {e}"), -1);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -1,
                    };
                }
            }
        };
        handle_result!(
            sz_configtool_lib::calls::distinct::delete_distinct_call_element(
                config,
                sz_configtool_lib::calls::CallSelector::Feature(feature),
                element,
                elem_feature,
            )
        )
    })
}

/// Delete an expression call element by (call id + element code).
///
/// Expression calls are many-per-feature, so this wrapper addresses the call by
/// its `EFCALL_ID`; the BOM `EXEC_ORDER` is derived internally.
///
/// # Safety
/// All pointers must be valid null-terminated C strings.
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_deleteExpressionCallElement(
    config_json: *const c_char,
    efcall_id: i64,
    element_code: *const c_char,
    element_feature: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_deleteExpressionCallElement", || {
        let config = ffi_required_str!(config_json, "config_json");
        let element = ffi_required_str!(element_code, "element_code");
        let elem_feature = if element_feature.is_null() {
            None
        } else {
            match unsafe { CStr::from_ptr(element_feature) }.to_str() {
                Ok(s) => Some(s),
                Err(e) => {
                    set_error(format!("Invalid UTF-8 in element_feature: {e}"), -1);
                    return SzConfigTool_result {
                        response: std::ptr::null_mut(),
                        returnCode: -1,
                    };
                }
            }
        };
        handle_result!(
            sz_configtool_lib::calls::expression::delete_expression_call_element(
                config,
                sz_configtool_lib::calls::CallSelector::Id(efcall_id),
                element,
                elem_feature,
            )
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};

    /// Read (and free) the response string of an FFI result, asserting success.
    fn take_response(result: SzConfigTool_result) -> String {
        assert_eq!(result.returnCode, 0, "FFI call returned an error code");
        assert!(!result.response.is_null(), "response was null");
        let s = unsafe { CStr::from_ptr(result.response) }
            .to_str()
            .unwrap()
            .to_string();
        unsafe { SzConfigTool_free(result.response) };
        s
    }

    /// Count CFG_GENERIC_THRESHOLD rows for (GPLAN_ID, BEHAVIOR, FTYPE_ID).
    fn count_generic_thresholds(config: &str, gplan_id: i64, behavior: &str, ftype: i64) -> usize {
        let v: Value = serde_json::from_str(config).unwrap();
        v["G2_CONFIG"]["CFG_GENERIC_THRESHOLD"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|r| {
                r["GPLAN_ID"] == json!(gplan_id)
                    && r["BEHAVIOR"] == json!(behavior)
                    && r["FTYPE_ID"] == json!(ftype)
            })
            .count()
    }

    /// deleteGenericThreshold must delete from the plan it is given, not
    /// always from INGEST (the `plan` argument used to be parsed and dropped).
    #[test]
    fn test_ffi_delete_generic_threshold_honours_plan() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../tests/fixtures/g2config_template.json"
        );
        let fixture = std::fs::read_to_string(path).unwrap();
        // Fixture: GPLAN 1 = INGEST, 2 = SEARCH; both carry (NAME, FTYPE 0).
        assert_eq!(count_generic_thresholds(&fixture, 1, "NAME", 0), 1);
        assert_eq!(count_generic_thresholds(&fixture, 2, "NAME", 0), 1);

        let config = CString::new(fixture).unwrap();
        let plan = CString::new("search").unwrap();
        let behavior = CString::new("NAME").unwrap();
        let out = take_response(SzConfigTool_deleteGenericThreshold(
            config.as_ptr(),
            plan.as_ptr(),
            behavior.as_ptr(),
            std::ptr::null(),
        ));
        assert_eq!(
            count_generic_thresholds(&out, 2, "NAME", 0),
            0,
            "SEARCH row"
        );
        assert_eq!(
            count_generic_thresholds(&out, 1, "NAME", 0),
            1,
            "INGEST row"
        );

        let unknown = CString::new("NO_SUCH_PLAN").unwrap();
        let result = SzConfigTool_deleteGenericThreshold(
            config.as_ptr(),
            unknown.as_ptr(),
            behavior.as_ptr(),
            std::ptr::null(),
        );
        assert_ne!(result.returnCode, 0, "unknown plan must fail");
        assert!(result.response.is_null());
    }

    /// #52: setSetting FFI parses the value as JSON — an integer is stored as a
    /// JSON number, not a quoted string — and uppercases the name.
    #[test]
    fn test_ffi_set_setting() {
        let config = CString::new(r#"{"G2_CONFIG": {}}"#).unwrap();
        let name = CString::new("my_setting").unwrap();
        let value = CString::new("42").unwrap();
        let result = SzConfigTool_setSetting(config.as_ptr(), name.as_ptr(), value.as_ptr());
        let modified = take_response(result);
        let v: Value = serde_json::from_str(&modified).unwrap();
        assert_eq!(v["G2_CONFIG"]["SETTINGS"]["MY_SETTING"], json!(42));
        assert_ne!(v["G2_CONFIG"]["SETTINGS"]["MY_SETTING"], json!("42"));
    }

    /// #52: a quoted JSON string value is stored as a JSON string.
    #[test]
    fn test_ffi_set_setting_quoted_string() {
        let config = CString::new(r#"{"G2_CONFIG": {}}"#).unwrap();
        let name = CString::new("greeting").unwrap();
        let value = CString::new(r#""hello""#).unwrap();
        let result = SzConfigTool_setSetting(config.as_ptr(), name.as_ptr(), value.as_ptr());
        let modified = take_response(result);
        let v: Value = serde_json::from_str(&modified).unwrap();
        assert_eq!(v["G2_CONFIG"]["SETTINGS"]["GREETING"], json!("hello"));
    }

    /// #52: an unparseable value returns an error (no string fallback).
    #[test]
    fn test_ffi_set_setting_invalid_json_errors() {
        let config = CString::new(r#"{"G2_CONFIG": {}}"#).unwrap();
        let name = CString::new("bad").unwrap();
        // Bare unquoted text is not valid JSON.
        let value = CString::new("not json").unwrap();
        let result = SzConfigTool_setSetting(config.as_ptr(), name.as_ptr(), value.as_ptr());
        assert_ne!(result.returnCode, 0);
        assert!(result.response.is_null());
    }

    /// #38.1/#38.2: addElementToFeature then deleteElementFromFeature round-trip.
    #[test]
    fn test_ffi_add_delete_element_to_feature() {
        let base = r#"{"G2_CONFIG": {
            "CFG_FTYPE": [{"FTYPE_ID": 1, "FTYPE_CODE": "NAME"}],
            "CFG_FELEM": [{"FELEM_ID": 2, "FELEM_CODE": "FULL_NAME"}],
            "CFG_FBOM": []
        }}"#;
        let config = CString::new(base).unwrap();
        let feature = CString::new("NAME").unwrap();
        let element = CString::new("FULL_NAME").unwrap();

        let added = take_response(SzConfigTool_addElementToFeature(
            config.as_ptr(),
            feature.as_ptr(),
            element.as_ptr(),
            std::ptr::null(),
        ));
        let v: Value = serde_json::from_str(&added).unwrap();
        assert_eq!(v["G2_CONFIG"]["CFG_FBOM"].as_array().unwrap().len(), 1);

        let added_c = CString::new(added).unwrap();
        let removed = take_response(SzConfigTool_deleteElementFromFeature(
            added_c.as_ptr(),
            feature.as_ptr(),
            element.as_ptr(),
        ));
        let v: Value = serde_json::from_str(&removed).unwrap();
        assert_eq!(v["G2_CONFIG"]["CFG_FBOM"].as_array().unwrap().len(), 0);
    }

    /// #38.4: deleteComparisonFunctionCascade FFI empties dependents.
    #[test]
    fn test_ffi_delete_comparison_function_cascade() {
        let base = r#"{"G2_CONFIG": {
            "CFG_FTYPE": [{"FTYPE_ID": 3, "FTYPE_CODE": "NAME"}],
            "CFG_CFUNC": [{"CFUNC_ID": 1, "CFUNC_CODE": "CMP_X"}],
            "CFG_CFCALL": [{"CFCALL_ID": 10, "FTYPE_ID": 3, "CFUNC_ID": 1}],
            "CFG_CFBOM": [{"CFCALL_ID": 10, "FTYPE_ID": 3, "FELEM_ID": 5, "EXEC_ORDER": 1}],
            "CFG_CFRTN": [{"CFRTN_ID": 100, "CFUNC_ID": 1, "FTYPE_ID": 3, "CFUNC_RTNVAL": "SAME"}]
        }}"#;
        let config = CString::new(base).unwrap();
        let code = CString::new("CMP_X").unwrap();
        let modified = take_response(SzConfigTool_deleteComparisonFunctionCascade(
            config.as_ptr(),
            code.as_ptr(),
        ));
        let v: Value = serde_json::from_str(&modified).unwrap();
        let g2 = &v["G2_CONFIG"];
        assert_eq!(g2["CFG_CFUNC"].as_array().unwrap().len(), 0);
        assert_eq!(g2["CFG_CFCALL"].as_array().unwrap().len(), 0);
        assert_eq!(g2["CFG_CFBOM"].as_array().unwrap().len(), 0);
        assert_eq!(g2["CFG_CFRTN"].as_array().unwrap().len(), 0);
    }

    /// D12: a NULL connect_str on the direct-arg add FFI stores JSON null, with
    /// the key still present.
    #[test]
    fn test_ffi_add_standardize_function_null_connect_str_stores_null() {
        let config = CString::new(r#"{"G2_CONFIG": {"CFG_SFUNC": []}}"#).unwrap();
        let code = CString::new("CUSTOM_PARSE").unwrap();

        let result = SzConfigTool_addStandardizeFunction(
            config.as_ptr(),
            code.as_ptr(),
            std::ptr::null(), // connect_str NULL -> stored null
            std::ptr::null(),
            std::ptr::null(),
        );
        let modified = take_response(result);
        let v: Value = serde_json::from_str(&modified).unwrap();
        let row = v["G2_CONFIG"]["CFG_SFUNC"]
            .as_array()
            .unwrap()
            .last()
            .unwrap();
        assert!(row.as_object().unwrap().contains_key("CONNECT_STR"));
        assert_eq!(row["CONNECT_STR"], Value::Null);
    }

    /// A NULL connect_str on the distinct add FFI likewise stores JSON null.
    #[test]
    fn test_ffi_add_distinct_function_null_connect_str_stores_null() {
        let config = CString::new(r#"{"G2_CONFIG": {"CFG_DFUNC": []}}"#).unwrap();
        let code = CString::new("CUSTOM_DIST").unwrap();

        let result = SzConfigTool_addDistinctFunction(
            config.as_ptr(),
            code.as_ptr(),
            std::ptr::null(),
            std::ptr::null(),
            std::ptr::null(),
        );
        let modified = take_response(result);
        let v: Value = serde_json::from_str(&modified).unwrap();
        let row = v["G2_CONFIG"]["CFG_DFUNC"]
            .as_array()
            .unwrap()
            .last()
            .unwrap();
        assert!(row.as_object().unwrap().contains_key("CONNECT_STR"));
        assert_eq!(row["CONNECT_STR"], Value::Null);
    }

    /// The JSON-based setRule FFI can clear a column: an explicit null for a
    /// disqualifier writes JSON null.
    #[test]
    fn test_ffi_set_rule_json_explicit_null_clears_column() {
        let config = CString::new(
            r#"{"G2_CONFIG": {"CFG_ERRULE": [
                {"ERRULE_ID": 100, "ERRULE_CODE": "R1", "RESOLVE": "Yes",
                 "RELATE": "No", "RTYPE_ID": 1, "QUAL_ERFRAG_CODE": "R1",
                 "DISQ_ERFRAG_CODE": "SOMETHING", "ERRULE_TIER": 10}
            ], "CFG_ERFRAG": []}}"#,
        )
        .unwrap();
        let code = CString::new("R1").unwrap();
        // Explicit JSON null on disqualifier -> Clear.
        let rule_json = CString::new(r#"{"disqualifier": null}"#).unwrap();

        let result = SzConfigTool_setRule(config.as_ptr(), code.as_ptr(), rule_json.as_ptr());
        let modified = take_response(result);
        let v: Value = serde_json::from_str(&modified).unwrap();
        let rule = &v["G2_CONFIG"]["CFG_ERRULE"][0];
        assert!(rule.as_object().unwrap().contains_key("DISQ_ERFRAG_CODE"));
        assert_eq!(rule["DISQ_ERFRAG_CODE"], Value::Null);
        // A field not mentioned is left untouched.
        assert_eq!(rule["QUAL_ERFRAG_CODE"], serde_json::json!("R1"));
    }

    /// The JSON-based setFragment FFI can clear ERFRAG_SOURCE (and, per D11, its
    /// ERFRAG_DEPENDS) via an explicit null.
    #[test]
    fn test_ffi_set_fragment_json_explicit_null_clears_source() {
        let config = CString::new(
            r#"{"G2_CONFIG": {"CFG_ERFRAG": [
                {"ERFRAG_ID": 3, "ERFRAG_CODE": "F1", "ERFRAG_DESC": "F1",
                 "ERFRAG_SOURCE": "./FRAGMENT[./SAME_NAME>0]", "ERFRAG_DEPENDS": "1,2"}
            ]}}"#,
        )
        .unwrap();
        let code = CString::new("F1").unwrap();
        let frag_json = CString::new(r#"{"ERFRAG_SOURCE": null}"#).unwrap();

        let result = unsafe {
            SzConfigTool_setFragmentWithJson(config.as_ptr(), code.as_ptr(), frag_json.as_ptr())
        };
        let modified = take_response(result);
        let v: Value = serde_json::from_str(&modified).unwrap();
        let frag = &v["G2_CONFIG"]["CFG_ERFRAG"][0];
        assert_eq!(frag["ERFRAG_SOURCE"], Value::Null);
        assert_eq!(frag["ERFRAG_DEPENDS"], Value::Null);
    }

    /// #40: deleteComparisonCallElement FFI addresses the row by (feature,
    /// element) codes and derives EXEC_ORDER internally — no exec_order arg.
    #[test]
    fn test_ffi_delete_comparison_call_element_by_feature() {
        let base = r#"{"G2_CONFIG": {
            "CFG_FTYPE": [{"FTYPE_ID": 3, "FTYPE_CODE": "NAME"}],
            "CFG_FELEM": [
                {"FELEM_ID": 11, "FELEM_CODE": "FIRST_NAME"},
                {"FELEM_ID": 12, "FELEM_CODE": "LAST_NAME"}
            ],
            "CFG_CFCALL": [{"CFCALL_ID": 7, "FTYPE_ID": 3, "CFUNC_ID": 1}],
            "CFG_CFBOM": [
                {"CFCALL_ID": 7, "FTYPE_ID": 3, "FELEM_ID": 11, "EXEC_ORDER": 1},
                {"CFCALL_ID": 7, "FTYPE_ID": 3, "FELEM_ID": 12, "EXEC_ORDER": 2}
            ]
        }}"#;
        let config = CString::new(base).unwrap();
        let feature = CString::new("NAME").unwrap();
        let element = CString::new("FIRST_NAME").unwrap();

        let modified = take_response(SzConfigTool_deleteComparisonCallElement(
            config.as_ptr(),
            feature.as_ptr(),
            element.as_ptr(),
            std::ptr::null(), // element_feature: not needed (unambiguous)
        ));
        let v: Value = serde_json::from_str(&modified).unwrap();
        let cfbom = v["G2_CONFIG"]["CFG_CFBOM"].as_array().unwrap();
        assert_eq!(cfbom.len(), 1);
        assert_eq!(cfbom[0]["FELEM_ID"], 12);
    }

    /// #40: getStandardizeCallByFeature returns the feature's call — the same
    /// row an id lookup returns — proving the FTYPE_ID-as-call-id bug is gone.
    #[test]
    fn test_ffi_get_standardize_call_by_feature() {
        // FTYPE_ID (3) deliberately differs from SFCALL_ID (5): the old code
        // used the feature id as the call id and returned the wrong/no row.
        let base = r#"{"G2_CONFIG": {
            "CFG_FTYPE": [{"FTYPE_ID": 3, "FTYPE_CODE": "NAME"}],
            "CFG_SFCALL": [
                {"SFCALL_ID": 5, "FTYPE_ID": 3, "FELEM_ID": -1, "SFUNC_ID": 1, "EXEC_ORDER": 1}
            ]
        }}"#;
        let config = CString::new(base).unwrap();
        let feature = CString::new("NAME").unwrap();

        let by_feature = take_response(SzConfigTool_getStandardizeCallByFeature(
            config.as_ptr(),
            feature.as_ptr(),
        ));
        let v: Value = serde_json::from_str(&by_feature).unwrap();
        assert_eq!(v["SFCALL_ID"], 5);
        assert_eq!(v["FTYPE_ID"], 3);
    }
}

// ============================================================================
// JSON-based function setters (standardize / expression / comparison /
// distinct). CONNECT_STR is tri-state here — absent = leave, JSON null = clear,
// string = set — which the direct-arg set*Function forms cannot express.
// ============================================================================

/// Parsed `updates_json` for a `set*FunctionWithJson` call.
struct FunctionUpdates<'a> {
    connect_str: sz_configtool_lib::helpers::FieldUpdate<&'a str>,
    description: Option<&'a str>,
    language: Option<&'a str>,
    anon_support: Option<&'a str>,
}

/// Read one updates field given by its column key OR its camelCase alias:
/// absent = Leave, JSON null = Clear, string = Set. Both spellings present
/// (even if one is null) or a non-string, non-null value is `INVALID_INPUT`
/// — never silently ignored.
fn update_field<'a>(
    obj: &'a serde_json::Map<String, serde_json::Value>,
    key: &str,
    alias: &str,
) -> sz_configtool_lib::Result<sz_configtool_lib::helpers::FieldUpdate<&'a str>> {
    use sz_configtool_lib::helpers::FieldUpdate;
    let (name, value) = match (obj.get(key), obj.get(alias)) {
        (None, None) => return Ok(FieldUpdate::Leave),
        (Some(_), Some(_)) => {
            return Err(SzConfigError::InvalidInput(format!(
                "updates_json: give only one of \"{key}\" / \"{alias}\""
            )));
        }
        (Some(v), None) => (key, v),
        (None, Some(v)) => (alias, v),
    };
    match value {
        serde_json::Value::Null => Ok(FieldUpdate::Clear),
        serde_json::Value::String(s) => Ok(FieldUpdate::Set(s.as_str())),
        other => Err(SzConfigError::InvalidInput(format!(
            "updates_json: \"{name}\" must be a string or null, got {other}"
        ))),
    }
}

/// Parse a function-updates object.
///
/// Keys (each with a camelCase alias; give one spelling per field):
/// `CONNECT_STR`|`connectStr` (tri-state: null clears), `<desc_key>`|
/// `description` (e.g. `SFUNC_DESC`), `LANGUAGE`|`language`, and — only when
/// `anon_supported` (comparison / distinct) — `ANON_SUPPORT`|`anonSupport`.
/// For every field but CONNECT_STR a null or absent value leaves the stored
/// value untouched. A non-object, an unknown key (including `ANON_SUPPORT`
/// where unsupported), a non-string value, or both spellings of one field is
/// `INVALID_INPUT`.
fn parse_function_updates<'a>(
    json: &'a serde_json::Value,
    desc_key: &str,
    anon_supported: bool,
) -> sz_configtool_lib::Result<FunctionUpdates<'a>> {
    let obj = json.as_object().ok_or_else(|| {
        SzConfigError::InvalidInput("updates_json must be a JSON object".to_string())
    })?;
    let mut fields = vec![
        ("CONNECT_STR", "connectStr"),
        (desc_key, "description"),
        ("LANGUAGE", "language"),
    ];
    if anon_supported {
        fields.push(("ANON_SUPPORT", "anonSupport"));
    }
    if let Some(k) = obj
        .keys()
        .find(|k| !fields.iter().any(|(key, alias)| k == key || k == alias))
    {
        return Err(SzConfigError::InvalidInput(format!(
            "updates_json: unknown key \"{k}\" (accepted: {})",
            fields
                .iter()
                .map(|(key, alias)| format!("{key}|{alias}"))
                .collect::<Vec<_>>()
                .join(", ")
        )));
    }
    let leave_on_null = |u: sz_configtool_lib::helpers::FieldUpdate<&'a str>| match u {
        sz_configtool_lib::helpers::FieldUpdate::Set(s) => Some(s),
        _ => None,
    };
    Ok(FunctionUpdates {
        connect_str: update_field(obj, "CONNECT_STR", "connectStr")?,
        description: leave_on_null(update_field(obj, desc_key, "description")?),
        language: leave_on_null(update_field(obj, "LANGUAGE", "language")?),
        anon_support: leave_on_null(update_field(obj, "ANON_SUPPORT", "anonSupport")?),
    })
}

/// Shared body of the `set*FunctionWithJson` wrappers: read the C arguments,
/// parse `updates_json`, apply `set`, and return the modified config.
fn set_function_with_json<F>(
    config_json: *const c_char,
    code: *const c_char,
    updates_json: *const c_char,
    desc_key: &str,
    anon_supported: bool,
    set: F,
) -> SzConfigTool_result
where
    F: for<'a> FnOnce(
        &'a str,
        &'a str,
        FunctionUpdates<'a>,
    ) -> sz_configtool_lib::Result<(String, serde_json::Value)>,
{
    if config_json.is_null() || code.is_null() || updates_json.is_null() {
        set_error("Required parameter is null".to_string(), -1);
        return SzConfigTool_result {
            response: std::ptr::null_mut(),
            returnCode: -1,
        };
    }
    let config = ffi_required_str!(config_json, "config_json");
    let code = ffi_required_str!(code, "code");
    let updates_str = ffi_required_str!(updates_json, "updates_json");
    let updates_value: serde_json::Value = match serde_json::from_str(updates_str) {
        Ok(v) => v,
        Err(e) => {
            set_error(format!("Failed to parse updates_json: {e}"), -3);
            return SzConfigTool_result {
                response: std::ptr::null_mut(),
                returnCode: -3,
            };
        }
    };
    let result = parse_function_updates(&updates_value, desc_key, anon_supported)
        .and_then(|updates| set(config, code, updates))
        .map(|(modified, _)| modified);
    handle_result!(result)
}

/// Set/update a standardize function from JSON (`CONNECT_STR`, `SFUNC_DESC`,
/// `LANGUAGE`; CONNECT_STR null clears it).
///
/// # Safety
/// All pointers must be valid null-terminated C strings.
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_setStandardizeFunctionWithJson(
    config_json: *const c_char,
    sfunc_code: *const c_char,
    updates_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_setStandardizeFunctionWithJson", || {
        set_function_with_json(
            config_json,
            sfunc_code,
            updates_json,
            "SFUNC_DESC",
            false,
            |c, k, u| {
                use sz_configtool_lib::functions::standardize as f;
                let params = f::SetStandardizeFunctionParams {
                    connect_str: u.connect_str,
                    description: u.description,
                    language: u.language,
                };
                f::set_standardize_function(c, k, params)
            },
        )
    })
}

/// Set/update an expression function from JSON (`CONNECT_STR`, `EFUNC_DESC`,
/// `LANGUAGE`; CONNECT_STR null clears it).
///
/// # Safety
/// All pointers must be valid null-terminated C strings.
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_setExpressionFunctionWithJson(
    config_json: *const c_char,
    efunc_code: *const c_char,
    updates_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_setExpressionFunctionWithJson", || {
        set_function_with_json(
            config_json,
            efunc_code,
            updates_json,
            "EFUNC_DESC",
            false,
            |c, k, u| {
                use sz_configtool_lib::functions::expression as f;
                let params = f::SetExpressionFunctionParams {
                    connect_str: u.connect_str,
                    description: u.description,
                    language: u.language,
                };
                f::set_expression_function(c, k, params)
            },
        )
    })
}

/// Set/update a comparison function from JSON (`CONNECT_STR`, `CFUNC_DESC`,
/// `LANGUAGE`, `ANON_SUPPORT`; CONNECT_STR null clears it).
///
/// # Safety
/// All pointers must be valid null-terminated C strings.
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_setComparisonFunctionWithJson(
    config_json: *const c_char,
    cfunc_code: *const c_char,
    updates_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_setComparisonFunctionWithJson", || {
        set_function_with_json(
            config_json,
            cfunc_code,
            updates_json,
            "CFUNC_DESC",
            true,
            |c, k, u| {
                use sz_configtool_lib::functions::comparison as f;
                let params = f::SetComparisonFunctionParams {
                    connect_str: u.connect_str,
                    description: u.description,
                    language: u.language,
                    anon_support: u.anon_support,
                };
                f::set_comparison_function(c, k, params)
            },
        )
    })
}

/// Set/update a distinct function from JSON (`CONNECT_STR`, `DFUNC_DESC`,
/// `LANGUAGE`, `ANON_SUPPORT`; CONNECT_STR null clears it).
///
/// # Safety
/// All pointers must be valid null-terminated C strings.
#[unsafe(no_mangle)]
pub extern "C" fn SzConfigTool_setDistinctFunctionWithJson(
    config_json: *const c_char,
    dfunc_code: *const c_char,
    updates_json: *const c_char,
) -> SzConfigTool_result {
    ffi_guard("SzConfigTool_setDistinctFunctionWithJson", || {
        set_function_with_json(
            config_json,
            dfunc_code,
            updates_json,
            "DFUNC_DESC",
            true,
            |c, k, u| {
                use sz_configtool_lib::functions::distinct as f;
                let params = f::SetDistinctFunctionParams {
                    connect_str: u.connect_str,
                    description: u.description,
                    language: u.language,
                    anon_support: u.anon_support,
                };
                f::set_distinct_function(c, k, params)
            },
        )
    })
}

/// Test-only allocator that over-allocates and fills every block with a
/// non-NUL poison byte, so reading a string past its end (a missing NUL
/// terminator) is caught deterministically rather than masked by zeroed
/// memory. Real allocations still go through the system allocator.
#[cfg(test)]
mod poison_alloc {
    use std::alloc::{GlobalAlloc, Layout, System};

    const PAD: usize = 64;

    struct PoisonAllocator;

    unsafe impl GlobalAlloc for PoisonAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            let Ok(padded) = Layout::from_size_align(layout.size() + PAD, layout.align()) else {
                return std::ptr::null_mut();
            };
            let ptr = unsafe { System.alloc(padded) };
            if !ptr.is_null() {
                unsafe { ptr.write_bytes(0xA5, padded.size()) };
            }
            ptr
        }

        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            // Infallible: the same padded layout was valid in `alloc`.
            if let Ok(padded) = Layout::from_size_align(layout.size() + PAD, layout.align()) {
                unsafe { System.dealloc(ptr, padded) };
            }
        }
    }

    #[global_allocator]
    static GLOBAL: PoisonAllocator = PoisonAllocator;
}

#[cfg(test)]
mod tests_validation_errors;

#[cfg(test)]
mod tests_hardening;

#[cfg(test)]
mod tests_tls_teardown;

#[cfg(test)]
mod tests_set_function_json;

#[cfg(test)]
mod tests_invoke;

#[cfg(test)]
mod tests_cov_a;

#[cfg(test)]
mod tests_cov_b;

#[cfg(test)]
mod tests_cov_c;

#[cfg(test)]
mod tests_cov_d;
