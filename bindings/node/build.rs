//! napi-rs link setup, plus (macOS) a relocatable install name
//! `@rpath/libszconfigtool_node.dylib` for the `.node` cdylib; rustc otherwise
//! bakes the absolute `target/<profile>/deps/...` path, which leaks the build
//! directory.
//!
//! Verified by `packaging/gates/check-linkage.sh` (otool) and
//! `packaging/gates/check-no-build-paths.sh`.

use std::env;

fn main() {
    napi_build::setup();
    println!("cargo:rerun-if-changed=build.rs");
    if env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("macos") {
        println!("cargo:rustc-cdylib-link-arg=-Wl,-install_name,@rpath/libszconfigtool_node.dylib");
    }
}
