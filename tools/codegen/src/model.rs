//! Typed model of the hand-maintained YAML inputs (see api/manifest/schema.md).
//!
//! Every struct denies unknown fields so a typo in a manifest is an error, not
//! a silently ignored key.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// `api/manifest/project.yaml`.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Project {
    pub root_crate: String,
    pub paths: Paths,
    pub reason_codes: Vec<String>,
    /// Per-language generated output files (never hardcoded in generators).
    pub bindings: Bindings,
}

/// `project.yaml` `bindings:` — every per-language generated file, by role.
/// All paths are workspace-relative FILES.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bindings {
    pub python: PythonOut,
    pub java: JavaOut,
    pub csharp: CsharpOut,
    pub cpp: CppOut,
    pub node: NodeOut,
}

/// Python generated files.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PythonOut {
    /// Typed module (its stem is the import name used by `init`).
    pub module: String,
    /// Type stub of `module`.
    pub stub: String,
    /// Package `__init__.py` (re-exports).
    pub init: String,
    /// Workspace paths read by the pytest suite.
    pub test_paths: String,
}

/// Java generated files.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct JavaOut {
    pub api: String,
    pub error_kinds: String,
    /// Test-only reflective dispatcher for the conformance runner.
    pub test_dispatch: String,
}

/// C# generated files.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CsharpOut {
    pub api: String,
    pub error_kinds: String,
}

/// C++ generated files.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CppOut {
    pub api: String,
    pub error_kinds: String,
    /// Test-only typed dispatcher for the conformance runner.
    pub test_dispatch: String,
}

/// Node/TypeScript generated files.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NodeOut {
    /// The npm package directory (test paths are relative to it).
    pub package_dir: String,
    pub functions: String,
    pub reason_codes: String,
    pub trpc_schemas: String,
    pub trpc_router: String,
    pub test_paths: String,
}

/// Workspace-relative paths named by `project.yaml`.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Paths {
    pub root_src: String,
    pub c_header: String,
    pub fixture: String,
    pub manifest_dir: String,
    pub conformance_dir: String,
    pub excluded: String,
    pub dispatch_out: String,
    pub manifest_json_out: String,
    pub conformance_json_out: String,
}

/// One `api/manifest/<group>.yaml` file.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GroupFile {
    pub group: String,
    pub functions: Vec<Function>,
    /// Exclusions owned by this group (merged into excluded.yaml's entries),
    /// so parallel group authors never edit the shared file.
    #[serde(default)]
    pub excluded: Vec<ExcludedEntry>,
}

/// Wire type of an argument.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ArgType {
    Str,
    Int,
    Bool,
    Json,
    StrList,
    /// A JSON integer OR a JSON string (e.g. a call selector: id or feature
    /// code). Typed bindings expose a natural union/overloads; needs
    /// `rust_convert` (there is no plain Rust mapping).
    IntOrStr,
}

impl ArgType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Str => "str",
            Self::Int => "int",
            Self::Bool => "bool",
            Self::Json => "json",
            Self::StrList => "str_list",
            Self::IntOrStr => "int_or_str",
        }
    }
}

/// Wire shape of a function's successful result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Returns {
    Config,
    Json,
    ConfigAndJson,
    Int,
    Unit,
}

impl Returns {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Config => "config",
            Self::Json => "json",
            Self::ConfigAndJson => "config_and_json",
            Self::Int => "int",
            Self::Unit => "unit",
        }
    }
}

/// One function argument (after `config`, which is implicit).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Arg {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: ArgType,
    #[serde(default)]
    pub optional: bool,
    #[serde(default)]
    pub tristate: bool,
    /// `optional` arg (Rust `Option<T>`) that the LIBRARY requires (absent =
    /// MISSING_FIELD): typed bindings must make it a required parameter.
    /// `invoke` is unchanged (absent still reaches the library as `None`).
    #[serde(default)]
    pub required: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semantics: Option<String>,
    /// Params-struct field this arg fills (defaults to `name`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// With `rust_params_struct`: pass this arg positionally instead.
    #[serde(default)]
    pub positional: bool,
    /// `json` only: pass an owned `serde_json::Value` instead of `&Value`.
    #[serde(default)]
    pub owned: bool,
    /// Converter fn in `sz_configtool_api::convert` producing the Rust value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rust_convert: Option<String>,
    /// `json` only (and required there): the structural type of the value
    /// (`any` for free-form JSON). See [`JsonType`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub json_type: Option<JsonType>,
}

/// Structural type of a `json` arg (`json_type`; api/manifest/schema.md):
/// a scalar name (`any`, `string`, `int`, `bool`) or a one-key map
/// (`enum: [..]`, `array: T`, `object: {field: T, "opt?": T}`,
/// `one_of: [T, ..]`). Structural rules (non-empty enum, distinct one_of
/// kinds, ...) are checked by `validate`, not here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsonType {
    Any,
    String,
    Int,
    Bool,
    /// A string, exactly one of these values.
    Enum(Vec<String>),
    Array(Box<JsonType>),
    /// An object with exactly these fields (unknown keys are not allowed).
    Object(Vec<JsonField>),
    /// One of the alternatives (each a different JSON kind).
    OneOf(Vec<JsonType>),
}

/// One field of a [`JsonType::Object`] (YAML key `name`, or `name?` when optional).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonField {
    pub name: String,
    pub optional: bool,
    pub ty: JsonType,
}

/// JSON kind a [`JsonType`] accepts (used to keep `one_of` unambiguous).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum JsonKind {
    String,
    Int,
    Bool,
    Array,
    Object,
}

impl JsonType {
    /// Parse the descriptor syntax (see the type docs).
    pub fn from_value(v: &Value) -> Result<Self, String> {
        match v {
            Value::String(s) => match s.as_str() {
                "any" => Ok(Self::Any),
                "string" => Ok(Self::String),
                "int" => Ok(Self::Int),
                "bool" => Ok(Self::Bool),
                other => Err(format!(
                    "json_type: unknown type '{other}' (any, string, int, bool, or a one-key \
                     map of enum, array, object, one_of)"
                )),
            },
            Value::Object(map) if map.len() == 1 => {
                let (tag, body) = map.iter().next().expect("one entry");
                Self::tagged(tag, body)
            }
            _ => Err(format!(
                "json_type: expected a type name or a one-key map, got {v}"
            )),
        }
    }

    fn tagged(tag: &str, body: &Value) -> Result<Self, String> {
        let list = |what: &str| {
            body.as_array()
                .ok_or_else(|| format!("json_type: {what} needs a list"))
        };
        match tag {
            "enum" => list("enum")?
                .iter()
                .map(|s| {
                    s.as_str()
                        .map(str::to_string)
                        .ok_or_else(|| format!("json_type: enum values must be strings, got {s}"))
                })
                .collect::<Result<_, _>>()
                .map(Self::Enum),
            "array" => Ok(Self::Array(Box::new(Self::from_value(body)?))),
            "one_of" => list("one_of")?
                .iter()
                .map(Self::from_value)
                .collect::<Result<_, _>>()
                .map(Self::OneOf),
            "object" => body
                .as_object()
                .ok_or_else(|| "json_type: object needs a map of fields".to_string())?
                .iter()
                .map(|(key, ty)| {
                    let (name, optional) = match key.strip_suffix('?') {
                        Some(name) => (name.to_string(), true),
                        None => (key.clone(), false),
                    };
                    Ok(JsonField {
                        name,
                        optional,
                        ty: Self::from_value(ty)?,
                    })
                })
                .collect::<Result<_, String>>()
                .map(Self::Object),
            other => Err(format!(
                "json_type: unknown tag '{other}' (enum, array, object, one_of)"
            )),
        }
    }

    /// The descriptor syntax (inverse of [`JsonType::from_value`]).
    pub fn to_value(&self) -> Value {
        match self {
            Self::Any => Value::from("any"),
            Self::String => Value::from("string"),
            Self::Int => Value::from("int"),
            Self::Bool => Value::from("bool"),
            Self::Enum(values) => serde_json::json!({ "enum": values }),
            Self::Array(item) => serde_json::json!({ "array": item.to_value() }),
            Self::OneOf(alts) => {
                let alts: Vec<Value> = alts.iter().map(Self::to_value).collect();
                serde_json::json!({ "one_of": alts })
            }
            Self::Object(fields) => {
                let map: serde_json::Map<String, Value> =
                    fields.iter().map(|f| (f.key(), f.ty.to_value())).collect();
                serde_json::json!({ "object": map })
            }
        }
    }

    /// The JSON kind this type accepts (`None` for `any` and `one_of`).
    pub fn kind(&self) -> Option<JsonKind> {
        match self {
            Self::String | Self::Enum(_) => Some(JsonKind::String),
            Self::Int => Some(JsonKind::Int),
            Self::Bool => Some(JsonKind::Bool),
            Self::Array(_) => Some(JsonKind::Array),
            Self::Object(_) => Some(JsonKind::Object),
            Self::Any | Self::OneOf(_) => None,
        }
    }

    /// Whether `v` has this shape (unknown object keys do not fit).
    pub fn matches(&self, v: &Value) -> bool {
        match self {
            Self::Any => true,
            Self::String => v.is_string(),
            Self::Int => v.as_i64().is_some(),
            Self::Bool => v.is_boolean(),
            Self::Enum(values) => v.as_str().is_some_and(|s| values.iter().any(|e| e == s)),
            Self::Array(item) => v
                .as_array()
                .is_some_and(|items| items.iter().all(|i| item.matches(i))),
            Self::OneOf(alts) => alts.iter().any(|a| a.matches(v)),
            Self::Object(fields) => v.as_object().is_some_and(|obj| {
                obj.keys().all(|k| fields.iter().any(|f| &f.name == k))
                    && fields.iter().all(|f| match obj.get(&f.name) {
                        None => f.optional,
                        Some(value) => f.ty.matches(value),
                    })
            }),
        }
    }

    /// A compact, language-neutral rendering for doc comments, e.g.
    /// `[{feature: string, flag: "Yes"|"No"}]`.
    pub fn describe(&self) -> String {
        match self {
            Self::Any => "any".into(),
            Self::String => "string".into(),
            Self::Int => "int".into(),
            Self::Bool => "bool".into(),
            Self::Enum(values) => values
                .iter()
                .map(|v| Value::from(v.as_str()).to_string())
                .collect::<Vec<_>>()
                .join("|"),
            Self::Array(item) => format!("[{}]", item.describe()),
            Self::OneOf(alts) => alts
                .iter()
                .map(Self::describe)
                .collect::<Vec<_>>()
                .join(" | "),
            Self::Object(fields) => {
                let fields: Vec<String> = fields
                    .iter()
                    .map(|f| format!("{}: {}", f.key(), f.ty.describe()))
                    .collect();
                format!("{{{}}}", fields.join(", "))
            }
        }
    }
}

impl JsonField {
    /// The descriptor key: `name`, or `name?` when optional.
    pub fn key(&self) -> String {
        if self.optional {
            format!("{}?", self.name)
        } else {
            self.name.clone()
        }
    }
}

impl<'de> Deserialize<'de> for JsonType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let v = Value::deserialize(d)?;
        Self::from_value(&v).map_err(serde::de::Error::custom)
    }
}

impl Serialize for JsonType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.to_value().serialize(s)
    }
}

impl Arg {
    /// Absent is allowed (tri-state args are implicitly optional).
    pub fn is_optional(&self) -> bool {
        self.optional || self.tristate
    }

    /// [`JsonType::describe`] of a structured (non-`any`) `json_type`, for
    /// binding docs (`Shape: ...`).
    pub fn shape(&self) -> Option<String> {
        self.json_type
            .as_ref()
            .filter(|t| **t != JsonType::Any)
            .map(JsonType::describe)
    }
}

/// Implementation status of a function.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    /// A real operation (the default).
    #[default]
    Implemented,
    /// A placeholder that always fails with NOT_IMPLEMENTED: still dispatched
    /// by `invoke`, but typed language generators skip it.
    NotImplemented,
}

/// One manifest function.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Function {
    pub name: String,
    /// Taken from the file's `group:`; may be repeated per function but must match.
    #[serde(default)]
    pub group: String,
    pub doc: String,
    pub rust: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rust_params_struct: Option<String>,
    #[serde(default)]
    pub c_symbol: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub c_aliases: Vec<String>,
    #[serde(default)]
    pub args: Vec<Arg>,
    pub returns: Returns,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tuple_names: Vec<String>,
    #[serde(default)]
    pub errors: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// C-ABI-specific deltas (typed `SzConfigTool_*` exports, return codes).
    /// Manifest-only reference data: emitted in manifest.json, never in a
    /// language binding's docs (`notes` is language-neutral).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub c_notes: Option<String>,
    #[serde(default)]
    pub status: Status,
    /// The library rejects a call that passes none of the optional arguments
    /// (e.g. "exactly one of ftype_code / felem_code"), so a binding that
    /// offers a no-options convenience overload (Java) omits it.
    #[serde(default, skip_serializing_if = "is_false")]
    pub requires_options: bool,
}

impl Function {
    pub fn arg(&self, name: &str) -> Option<&Arg> {
        self.args.iter().find(|a| a.name == name)
    }

    /// Snake name of the typed COMPANION of a `config_and_json` function
    /// (`<name>_result`), or `None`. Typed bindings split such a function in
    /// two: the primary method returns only the new config text (so every
    /// config-changing method chains the same way) and the companion, taking
    /// the same arguments, returns the non-config part (the record JSON text,
    /// or the `tuple_names` record). The `invoke` wire is unchanged.
    pub fn companion(&self) -> Option<String> {
        (self.returns == Returns::ConfigAndJson).then(|| format!("{}_result", self.name))
    }
}

/// `api/manifest/excluded.yaml`.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Excluded {
    pub enforce_complete: bool,
    #[serde(default)]
    pub entries: Vec<ExcludedEntry>,
}

/// One exclusion: exactly one of `rust` (a fn) or `module` (a whole module).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExcludedEntry {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rust: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,
    pub reason: String,
}

/// One `api/manifest/conformance/<group>.yaml` file.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConformanceFile {
    pub group: String,
    pub cases: Vec<CaseInput>,
}

/// A case as written: either a single step inline (`fn`/`args`/...) or `steps`.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CaseInput {
    pub name: String,
    #[serde(default)]
    pub doc: Option<String>,
    #[serde(default, rename = "fn")]
    pub func: Option<String>,
    #[serde(default)]
    pub args: Option<serde_json::Map<String, Value>>,
    #[serde(default)]
    pub config_literal: Option<String>,
    #[serde(default)]
    pub expect: Option<Expect>,
    #[serde(default)]
    pub steps: Option<Vec<Step>>,
}

/// One invocation inside a case.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Step {
    #[serde(rename = "fn")]
    pub func: String,
    #[serde(default)]
    pub args: serde_json::Map<String, Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_literal: Option<String>,
    #[serde(default)]
    pub expect: Expect,
    /// Computed (never written by authors): the step omits a `required` arg
    /// to test MISSING_FIELD, which a typed binding cannot express. Language
    /// runners execute such steps through `invoke` (or skip them).
    #[serde(default, skip_deserializing, skip_serializing_if = "is_false")]
    pub wire_only: bool,
}

fn is_false(b: &bool) -> bool {
    !*b
}

/// Expected outcome of a step (all fields optional; empty = "must succeed").
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Expect {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<Returns>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contains: Option<Vec<Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub excludes: Option<Vec<Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub len: Option<usize>,
}

impl Expect {
    pub fn has_success_checks(&self) -> bool {
        self.kind.is_some()
            || self.result.is_some()
            || self.contains.is_some()
            || self.excludes.is_some()
            || self.len.is_some()
    }
}

/// A normalized case (always a list of steps), as written to conformance.json.
#[derive(Debug, Clone, Serialize)]
pub struct Case {
    pub group: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
    pub steps: Vec<Step>,
}

#[cfg(test)]
impl Bindings {
    /// The real `bindings:` of the checked-in `project.yaml` (for unit tests
    /// that build a synthetic [`Project`]).
    pub fn from_real_project() -> Self {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .expect("workspace root");
        let text =
            std::fs::read_to_string(root.join(crate::DEFAULT_PROJECT_FILE)).expect("project.yaml");
        serde_yaml_ng::from_str::<Project>(&text)
            .expect("project.yaml parses")
            .bindings
    }
}

#[cfg(test)]
mod json_type_tests {
    use super::*;
    use serde_json::json;

    fn parse(v: Value) -> JsonType {
        JsonType::from_value(&v).expect("valid descriptor")
    }

    const EVERY_KIND: &str = r#"{"one_of": ["string", "int", "bool", {"array": "any"}, {"object": {"a": "string", "b?": {"enum": ["X", "Y"]}}}]}"#;

    #[test]
    fn test_round_trips_and_kinds() {
        let v: Value = serde_json::from_str(EVERY_KIND).unwrap();
        let t = parse(v.clone());
        assert_eq!(t.to_value(), v);
        let alts: Vec<JsonType> = v["one_of"]
            .as_array()
            .unwrap()
            .iter()
            .map(|a| parse(a.clone()))
            .collect();
        let kinds: Vec<_> = alts.iter().map(JsonType::kind).collect();
        assert_eq!(
            kinds,
            [
                Some(JsonKind::String),
                Some(JsonKind::Int),
                Some(JsonKind::Bool),
                Some(JsonKind::Array),
                Some(JsonKind::Object),
            ]
        );
        assert_eq!(t.kind(), None);
        assert_eq!(JsonType::Any.kind(), None);
        assert_eq!(parse(json!({"enum": ["A"]})).kind(), Some(JsonKind::String));
        assert_eq!(
            t.describe(),
            "string | int | bool | [any] | {a: string, b?: \"X\"|\"Y\"}"
        );
    }

    #[test]
    fn test_matches_every_variant() {
        let t = parse(serde_json::from_str(EVERY_KIND).unwrap());
        for ok in [
            json!("s"),
            json!(-3),
            json!(true),
            json!([1, "x", null]),
            json!({"a": "s"}),
            json!({"a": "s", "b": "Y"}),
        ] {
            assert!(t.matches(&ok), "{ok}");
        }
        for bad in [
            json!(1.5),
            json!(null),
            json!({"b": "X"}),
            json!({"a": "s", "b": "Z"}),
            json!({"a": "s", "c": 1}),
            json!({"a": 1}),
        ] {
            assert!(!t.matches(&bad), "{bad}");
        }
        assert!(!parse(json!({"array": "int"})).matches(&json!([1, "x"])));
        assert!(!parse(json!({"array": "int"})).matches(&json!({})));
    }

    #[test]
    fn test_syntax_errors_name_json_type() {
        for (bad, want) in [
            (json!(1), "expected a type name or a one-key map"),
            (json!({"enum": ["A"], "array": "int"}), "one-key map"),
            (json!("float"), "unknown type 'float'"),
            (json!({"tuple": []}), "unknown tag 'tuple'"),
            (json!({"enum": "A"}), "enum needs a list"),
            (json!({"enum": [1]}), "enum values must be strings"),
            (json!({"one_of": "string"}), "one_of needs a list"),
            (json!({"object": ["a"]}), "object needs a map of fields"),
            (json!({"array": "float"}), "unknown type 'float'"),
            (json!({"object": {"a": "float"}}), "unknown type 'float'"),
            (json!({"one_of": ["float"]}), "unknown type 'float'"),
        ] {
            let err = JsonType::from_value(&bad).expect_err("rejected");
            assert!(err.starts_with("json_type: "), "{err}");
            assert!(err.contains(want), "{bad}: {err}");
        }
    }

    #[test]
    fn test_shape_doc_skips_any_and_absent() {
        let mut a: Arg = serde_json::from_value(json!({"name": "v", "type": "json"})).unwrap();
        assert_eq!(a.shape(), None);
        a.json_type = Some(JsonType::Any);
        assert_eq!(a.shape(), None);
        a.json_type = Some(parse(json!({"array": "string"})));
        assert_eq!(a.shape().as_deref(), Some("[string]"));
        let bad = serde_json::from_value::<Arg>(
            json!({"name": "v", "type": "json", "json_type": "float"}),
        );
        assert!(
            bad.unwrap_err()
                .to_string()
                .contains("unknown type 'float'")
        );
        // A syntax error inside the json_type value surfaces from the deserializer.
        let broken =
            serde_json::from_str::<Arg>(r#"{"name": "v", "type": "json", "json_type": [1,]}"#);
        assert!(broken.is_err());
        let back = serde_json::to_value(&a).unwrap();
        assert_eq!(back["json_type"], json!({"array": "string"}));
    }
}
