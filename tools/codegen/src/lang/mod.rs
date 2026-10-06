//! Per-language wrapper generators. Each language owns exactly one file here
//! and the output tree `bindings/<lang>/`; see `bindings/CONTRACT.md`.

pub mod cpp;
pub mod csharp;
pub mod java;
pub mod node;
pub mod python;

use crate::Generated;
use crate::load::Inputs;

/// Every language's generated files (workspace-relative paths).
pub fn generate_all(inputs: &Inputs) -> Vec<Generated> {
    let mut out = Vec::new();
    out.extend(python::generate(inputs));
    out.extend(java::generate(inputs));
    out.extend(csharp::generate(inputs));
    out.extend(cpp::generate(inputs));
    out.extend(node::generate(inputs));
    out
}
