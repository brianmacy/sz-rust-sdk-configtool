//! # sz-configtool-jni
//!
//! The JNI seam of the Java binding (`bindings/java`): ONE native call,
//! `io.github.brianmacy.szconfigtool.NativeBridge.invoke(String name,
//! String config, String argsJson)`, over [`sz_configtool_api::invoke`], plus
//! the `libraryVersion()` / `abiVersion()` accessors (single source:
//! `sz_configtool_api::{LIBRARY_VERSION, ABI_VERSION}`).
//! The typed Java wrappers are generated (`tools/codegen/src/lang/java.rs`).
//!
//! * Success returns `String[3]` = `{kind, config | null, result | null}`:
//!   `config` is the library's exact bytes (opaque), `result` is JSON text.
//! * Failure throws `SzConfigToolException(kind, reasonCode, message, details)`
//!   constructed here; `kind` is the reason code (see
//!   `api/manifest/schema.md` → Errors), `details` the validation-failures/v1
//!   JSON or `null`.
//! * Every entry runs inside `catch_unwind(AssertUnwindSafe(..))` (the `jni`
//!   crate does not convert panics); a panic becomes an `INTERNAL` exception.
//!
//! No `SzConfigTool_*` C symbol is exported (this crate does not depend on the
//! C ABI crate); the Java tests assert the export list with `nm`.

use std::any::Any;
use std::panic::{AssertUnwindSafe, catch_unwind};

use jni::objects::{JClass, JObject, JObjectArray, JString, JThrowable, JValue};
use jni::strings::JNIString;
use jni::sys::jobjectArray;
use jni::{AttachGuard, Env, EnvUnowned};
use sz_configtool_api::{ApiError, Output};

/// Reason code for boundary faults (marshalling, panics).
const INTERNAL: &str = "INTERNAL";
/// Reason code for unusable arguments at the JNI boundary.
const INVALID_INPUT: &str = "INVALID_INPUT";

/// A failure to be thrown as `SzConfigToolException`.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Failure {
    reason_code: String,
    message: String,
    details: Option<String>,
}

impl Failure {
    fn new(reason_code: &str, message: impl Into<String>) -> Self {
        Self {
            reason_code: reason_code.to_string(),
            message: message.into(),
            details: None,
        }
    }

    /// Map an `invoke` error: reason code, Display message, validation details.
    fn from_api(e: &ApiError) -> Self {
        Self {
            reason_code: e.reason_code().to_string(),
            message: e.to_string(),
            details: e.details_json(),
        }
    }

    /// Map a caught panic payload to `INTERNAL`.
    fn from_panic(payload: &(dyn Any + Send)) -> Self {
        let text = payload
            .downcast_ref::<&str>()
            .map(|s| s.to_string())
            .or_else(|| payload.downcast_ref::<String>().cloned())
            .unwrap_or_else(|| "non-string panic payload".to_string());
        Self::new(INTERNAL, format!("panic in JNI binding: {text}"))
    }
}

/// Run `call`, turning a panic into an `INTERNAL` failure (the `jni` crate
/// does not catch panics, and one must never unwind into the JVM).
fn guarded<T>(call: impl FnOnce() -> Result<T, Failure>) -> Result<T, Failure> {
    catch_unwind(AssertUnwindSafe(call))
        .unwrap_or_else(|payload| Err(Failure::from_panic(payload.as_ref())))
}

/// The `{kind, config, result}` triple returned to Java.
fn output_parts(out: &Output) -> [Option<String>; 3] {
    [
        Some(out.kind().to_string()),
        out.config().map(str::to_string),
        out.result().map(|v| v.to_string()),
    ]
}

/// Strictly decode JNI "modified UTF-8" (`GetStringUTFChars` bytes) into a
/// Rust `String`.
///
/// Java strings are UTF-16; modified UTF-8 encodes each UTF-16 code unit in
/// 1-3 bytes (NUL as `C0 80`, a supplementary character as two 3-byte
/// surrogates). Only the shortest form is accepted (the one overlong form is
/// NUL as `C0 80`). The bytes are decoded back to code units and then through
/// the STRICT [`String::from_utf16`], so a lone surrogate is an error. (`jni`
/// 0.22's `JString::try_to_string` falls back to `from_utf8_lossy`, which
/// silently replaces it with U+FFFD and changes the value.)
fn decode_mutf8_strict(bytes: &[u8]) -> Result<String, String> {
    let cont = |i: usize| match bytes.get(i) {
        Some(b) if b & 0xC0 == 0x80 => Ok(u16::from(b & 0x3F)),
        _ => Err(format!("malformed modified UTF-8 at byte {i}")),
    };
    let mut units = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while let Some(&b0) = bytes.get(i) {
        let (unit, width) = match b0 {
            0x00..=0x7F => (u16::from(b0), 1),
            0xC0..=0xDF => ((u16::from(b0 & 0x1F) << 6) | cont(i + 1)?, 2),
            0xE0..=0xEF => (
                (u16::from(b0 & 0x0F) << 12) | (cont(i + 1)? << 6) | cont(i + 2)?,
                3,
            ),
            _ => return Err(format!("malformed modified UTF-8 at byte {i}")),
        };
        // Shortest form only: the sole permitted overlong spelling is C0 80
        // (NUL). Any other 2-byte unit < 0x80 or 3-byte unit < 0x800 is an
        // overlong encoding and malformed.
        let overlong = match width {
            2 => unit < 0x80 && unit != 0,
            3 => unit < 0x800,
            _ => false,
        };
        if overlong {
            return Err(format!("overlong modified UTF-8 at byte {i}"));
        }
        units.push(unit);
        i += width;
    }
    String::from_utf16(&units).map_err(|_| "contains a lone UTF-16 surrogate".to_string())
}

/// A Java string argument; `None` when the reference is null. Malformed text
/// (a lone surrogate) is `INVALID_INPUT`, never silently replaced.
fn read_string(env: &Env, s: &JString, what: &str) -> Result<Option<String>, Failure> {
    if s.is_null() {
        return Ok(None);
    }
    // GetStringUTFChars fails only when the JVM cannot allocate; that and a
    // malformed string are the same INVALID_INPUT for this argument.
    s.mutf8_chars(env)
        .map_err(|e| e.to_string())
        .and_then(|chars| decode_mutf8_strict(chars.to_bytes()))
        .map(Some)
        .map_err(|e| Failure::new(INVALID_INPUT, format!("invalid {what}: {e}")))
}

/// A required (non-null) Java string argument.
fn require_string(env: &Env, s: &JString, what: &str) -> Result<String, Failure> {
    read_string(env, s, what)?.ok_or_else(|| Failure::new(INVALID_INPUT, format!("{what} is null")))
}

/// Build the Java `String[3]` for a successful output.
fn new_result_array<'local>(
    env: &mut Env<'local>,
    parts: &[Option<String>; 3],
) -> Result<JObjectArray<'local, JString<'local>>, Failure> {
    // Every step fails only when the JVM cannot allocate (NewObjectArray,
    // NewStringUTF) or on a String stored into a fresh String[3]; one
    // INTERNAL "building result" failure covers them all.
    JObjectArray::<JString>::new(env, parts.len(), JString::null())
        .and_then(|array| {
            parts
                .iter()
                .enumerate()
                .filter_map(|(i, part)| part.as_deref().map(|text| (i, text)))
                .try_for_each(|(i, text)| {
                    env.new_string(text)
                        .and_then(|js| array.set_element(env, i, &js))
                })
                .map(|()| array)
        })
        .map_err(marshal_failure)
}

/// A JNI call failure while building the result (JVM allocation failure).
fn marshal_failure(e: jni::errors::Error) -> Failure {
    Failure::new(INTERNAL, format!("building result: {e}"))
}

/// The whole native call: read args, invoke, marshal the result.
fn run<'local>(
    env: &mut Env<'local>,
    name: &JString,
    config: &JString,
    args: &JString,
) -> Result<JObjectArray<'local, JString<'local>>, Failure> {
    let name = require_string(env, name, "name")?;
    let config = require_string(env, config, "config")?;
    let args = read_string(env, args, "argsJson")?.unwrap_or_else(|| "{}".to_string());
    let out =
        sz_configtool_api::invoke(&name, &config, &args).map_err(|e| Failure::from_api(&e))?;
    new_result_array(env, &output_parts(&out))
}

/// Construct and throw `SzConfigToolException`; if that is impossible, throw
/// a plain `RuntimeException` so the failure is never silently dropped.
fn throw_failure(env: &mut Env, failure: &Failure) {
    env.exception_clear();
    // `Env::throw` reports a successful throw as `Err(JavaException)`.
    let thrown = build_exception(env, failure)
        .and_then(|obj| env.cast_local::<JThrowable>(obj))
        .map(|t| env.throw(t));
    if matches!(thrown, Ok(Err(jni::errors::Error::JavaException))) && env.exception_check() {
        return;
    }
    env.exception_clear();
    let _ = env.throw_new(
        jni::jni_str!("java/lang/RuntimeException"),
        JNIString::from(format!("{}: {}", failure.reason_code, failure.message)),
    );
}

fn build_exception<'local>(
    env: &mut Env<'local>,
    failure: &Failure,
) -> jni::errors::Result<JObject<'local>> {
    // The constructor's four String arguments: kind, reasonCode, message and
    // details (null when absent). Any allocation failure, or a missing
    // exception class, is the Err that throw_failure falls back on.
    let texts = [
        Some(failure.reason_code.as_str()),
        Some(failure.reason_code.as_str()),
        Some(failure.message.as_str()),
        failure.details.as_deref(),
    ];
    texts
        .into_iter()
        .map(|text| {
            text.map_or(Ok(JObject::null()), |t| {
                env.new_string(t).map(JObject::from)
            })
        })
        .collect::<jni::errors::Result<Vec<JObject>>>()
        .and_then(|args| {
            // SzConfigToolException(String kind, String reasonCode, String message, String details)
            env.new_object(
                jni::jni_str!("io/github/brianmacy/szconfigtool/SzConfigToolException"),
                jni::jni_sig!(
                    "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V"
                ),
                &[
                    JValue::Object(&args[0]),
                    JValue::Object(&args[1]),
                    JValue::Object(&args[2]),
                    JValue::Object(&args[3]),
                ],
            )
        })
}

/// `NativeBridge.invoke(String name, String config, String argsJson)`.
///
/// Returns `{kind, config | null, result | null}` or throws
/// `SzConfigToolException` (and returns null).
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_brianmacy_szconfigtool_NativeBridge_invoke<'local>(
    env: EnvUnowned<'local>,
    _class: JClass<'local>,
    name: JString<'local>,
    config: JString<'local>,
    args_json: JString<'local>,
) -> jobjectArray {
    // SAFETY: `env` is the JNIEnv the JVM passed to this native method, valid
    // for the duration of the call on this thread.
    let mut guard: AttachGuard<'local> = unsafe { AttachGuard::from_unowned(env.into_raw()) };
    let env = guard.borrow_env_mut();
    let failure = match guarded(|| run(&mut *env, &name, &config, &args_json)) {
        Ok(array) => return array.into_raw(),
        Err(failure) => failure,
    };
    // Throwing must not unwind into the JVM either.
    let _ = catch_unwind(AssertUnwindSafe(|| throw_failure(&mut *env, &failure)));
    std::ptr::null_mut()
}

/// `NativeBridge.libraryVersion()`: [`sz_configtool_api::LIBRARY_VERSION`].
/// Returns null (with a pending Java exception) only if the JVM cannot
/// allocate the string.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_brianmacy_szconfigtool_NativeBridge_libraryVersion<'local>(
    env: EnvUnowned<'local>,
    _class: JClass<'local>,
) -> jni::sys::jstring {
    // SAFETY: `env` is the JNIEnv the JVM passed to this native method, valid
    // for the duration of the call on this thread.
    let mut guard: AttachGuard<'local> = unsafe { AttachGuard::from_unowned(env.into_raw()) };
    let env = guard.borrow_env_mut();
    catch_unwind(AssertUnwindSafe(|| {
        env.new_string(sz_configtool_api::LIBRARY_VERSION)
            .map_or(std::ptr::null_mut(), |s| s.into_raw())
    }))
    .unwrap_or(std::ptr::null_mut())
}

/// `NativeBridge.abiVersion()`: [`sz_configtool_api::ABI_VERSION`].
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_brianmacy_szconfigtool_NativeBridge_abiVersion<'local>(
    _env: EnvUnowned<'local>,
    _class: JClass<'local>,
) -> jni::sys::jint {
    sz_configtool_api::ABI_VERSION
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};
    use sz_configtool_api::SzConfigError;

    const CFG: &str = r#"{"G2_CONFIG": {"CFG_DSRC": []}}"#;

    #[test]
    fn test_output_parts_per_kind() {
        let cfg = Output::Config("c".into());
        assert_eq!(
            output_parts(&cfg),
            [Some("config".into()), Some("c".into()), None]
        );
        let both = Output::ConfigAndJson {
            config: "c".into(),
            record: json!({"ID": 1}),
        };
        assert_eq!(
            output_parts(&both),
            [
                Some("config_and_json".into()),
                Some("c".into()),
                Some(r#"{"ID":1}"#.into())
            ]
        );
        assert_eq!(
            output_parts(&Output::Json(json!([1, "a"]))),
            [Some("json".into()), None, Some(r#"[1,"a"]"#.into())]
        );
        assert_eq!(
            output_parts(&Output::Int(7)),
            [Some("int".into()), None, Some("7".into())]
        );
        assert_eq!(
            output_parts(&Output::Unit),
            [Some("unit".into()), None, None]
        );
    }

    #[test]
    fn test_guarded_turns_a_panic_into_internal() {
        let f = guarded::<()>(|| panic!("seam bug")).unwrap_err();
        assert_eq!(f.reason_code, INTERNAL);
        assert_eq!(f.message, "panic in JNI binding: seam bug");
        assert_eq!(guarded(|| Ok::<i32, Failure>(3)), Ok(3));
    }

    #[test]
    fn test_marshal_failure_is_internal() {
        let f = marshal_failure(jni::errors::Error::NullPtr("array"));
        assert_eq!(f.reason_code, INTERNAL);
        assert!(f.message.starts_with("building result: "));
    }

    #[test]
    fn test_config_bytes_are_passed_through() {
        let out = sz_configtool_api::invoke("add_data_source", CFG, r#"{"code":"crm"}"#).unwrap();
        let parts = output_parts(&out);
        assert_eq!(parts[1].as_deref(), out.config());
    }

    #[test]
    fn test_failure_from_api_keeps_reason_and_message() {
        let err = sz_configtool_api::invoke("get_data_source", CFG, r#"{"code":"X"}"#).unwrap_err();
        let f = Failure::from_api(&err);
        assert_eq!(f.reason_code, "NOT_FOUND");
        assert_eq!(f.message, err.to_string());
        assert_eq!(f.details, None);
        let internal = Failure::from_api(&ApiError::Internal("boom".into()));
        assert_eq!(internal.reason_code, INTERNAL);
        assert_eq!(internal.message, "internal error: boom");
    }

    #[test]
    fn test_validation_details_shape() {
        let failure = Failure::from_api(&real_validation_error());
        assert_eq!(failure.reason_code, "VALIDATION_ERRORS");
        let details: Value = serde_json::from_str(&failure.details.unwrap()).unwrap();
        assert_eq!(
            details["schema"],
            sz_configtool_api::VALIDATION_DETAILS_SCHEMA
        );
        assert!(
            details["failures"]
                .as_array()
                .is_some_and(|a| !a.is_empty())
        );
        let first = &details["failures"][0];
        for key in ["field", "reasonCode", "offendingValue"] {
            assert!(first.get(key).is_some(), "missing {key}");
        }
        assert_eq!(
            Failure::from_api(&ApiError::Config(SzConfigError::NotFound("x".into()))).details,
            None
        );
    }

    /// A real VALIDATION_ERRORS from the library (no hand-built failure).
    fn real_validation_error() -> ApiError {
        // The fixture path comes from the manifest (project.yaml paths.fixture).
        let manifest: Value = serde_json::from_str(sz_configtool_api::MANIFEST_JSON).unwrap();
        let fixture = manifest["paths"]["fixture"].as_str().unwrap();
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let template = std::fs::read_to_string(root.join(fixture)).unwrap();
        let err = sz_configtool_api::invoke(
            "set_generic_threshold",
            &template,
            r#"{"plan":"INGEST","behavior":"NAME","send_to_redo":"sometimes"}"#,
        )
        .unwrap_err();
        assert_eq!(err.reason_code(), "VALIDATION_ERRORS", "{err}");
        err
    }

    #[test]
    fn test_decode_mutf8_strict() {
        assert_eq!(decode_mutf8_strict(b"abc").unwrap(), "abc");
        assert_eq!(decode_mutf8_strict(b"").unwrap(), "");
        // NUL is C0 80 in modified UTF-8.
        assert_eq!(decode_mutf8_strict(&[b'a', 0xC0, 0x80]).unwrap(), "a\0");
        // U+00E9 (2 bytes), U+20AC (3 bytes).
        assert_eq!(decode_mutf8_strict("é€".as_bytes()).unwrap(), "é€");
        // U+1F600 as a CESU-8 surrogate pair (D83D DE00).
        let pair = [0xED, 0xA0, 0xBD, 0xED, 0xB8, 0x80];
        assert_eq!(decode_mutf8_strict(&pair).unwrap(), "\u{1F600}");
        // Lone high, lone low, and reversed surrogates are rejected.
        for bad in [
            &[0xED, 0xA0, 0xBD][..],
            &[b'x', 0xED, 0xB8, 0x80],
            &[0xED, 0xB8, 0x80, 0xED, 0xA0, 0xBD],
        ] {
            assert!(decode_mutf8_strict(bad).is_err(), "{bad:02X?}");
        }
        // Truncated / invalid lead or continuation bytes.
        for bad in [
            &[0xC3][..],
            &[0xE2, 0x82],
            &[0xE2, 0x41, 0x80],
            &[0xF0, 0x9F, 0x98, 0x80],
            &[0x80],
        ] {
            assert!(decode_mutf8_strict(bad).is_err(), "{bad:02X?}");
        }
    }

    #[test]
    fn test_decode_mutf8_strict_rejects_overlong_forms() {
        // Only C0 80 (NUL) may be overlong; every other overlong spelling of a
        // code unit is malformed (it would decode to a different, shorter form).
        for bad in [
            &[0xC0, 0x81][..],   // U+0001 in 2 bytes
            &[0xC0, 0xBF],       // U+003F
            &[0xC1, 0x80],       // U+0040
            &[0xC1, 0xBF],       // U+007F
            &[b'a', 0xC1, 0xA1], // 'a' after a valid byte
            &[0xE0, 0x80, 0x80], // NUL in 3 bytes
            &[0xE0, 0x80, 0x81], // U+0001 in 3 bytes
            &[0xE0, 0x81, 0xBF], // U+007F in 3 bytes
            &[0xE0, 0x9F, 0xBF], // U+07FF in 3 bytes (largest overlong)
            &[0xF0, 0x80, 0x80, 0x80],
            &[0xF0, 0x8F, 0xBF, 0xBF],
        ] {
            assert!(decode_mutf8_strict(bad).is_err(), "{bad:02X?}");
        }
        // Shortest forms at the boundaries still decode.
        assert_eq!(decode_mutf8_strict(&[0xC2, 0x80]).unwrap(), "\u{80}");
        assert_eq!(decode_mutf8_strict(&[0xDF, 0xBF]).unwrap(), "\u{7FF}");
        assert_eq!(decode_mutf8_strict(&[0xE0, 0xA0, 0x80]).unwrap(), "\u{800}");
        assert_eq!(
            decode_mutf8_strict(&[0xEF, 0xBF, 0xBF]).unwrap(),
            "\u{FFFF}"
        );
    }

    #[test]
    fn test_panic_payloads_map_to_internal() {
        let caught = catch_unwind(|| panic!("kaboom")).unwrap_err();
        let f = Failure::from_panic(caught.as_ref());
        assert_eq!(f.reason_code, INTERNAL);
        assert!(f.message.contains("kaboom"), "{}", f.message);
        let owned = catch_unwind(|| std::panic::panic_any(String::from("owned"))).unwrap_err();
        assert!(
            Failure::from_panic(owned.as_ref())
                .message
                .contains("owned")
        );
        let other = catch_unwind(|| std::panic::panic_any(5_i32)).unwrap_err();
        assert!(
            Failure::from_panic(other.as_ref())
                .message
                .contains("non-string panic payload")
        );
    }
}
