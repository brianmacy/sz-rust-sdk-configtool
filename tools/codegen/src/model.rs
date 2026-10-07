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
}

impl Arg {
    /// Absent is allowed (tri-state args are implicitly optional).
    pub fn is_optional(&self) -> bool {
        self.optional || self.tristate
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
