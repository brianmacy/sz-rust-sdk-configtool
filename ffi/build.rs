//! Link settings for the shared library (cdylib) only.
//!
//! * Linux: SONAME `libSzConfigTool.so` (rustc sets none by default, so a
//!   consumer would record the build-time path/filename instead).
//! * macOS: install name `@rpath/libSzConfigTool.dylib` (rustc bakes the
//!   absolute `target/<profile>/deps/...` path otherwise) and the dylib
//!   current version from the crate version's numeric `X.Y.Z` (ld64 accepts
//!   only `xxxx.yy.zz`; a suffix such as `4.4.0-1` or `4.5.0-rc.1` would fail the
//!   link, so the prerelease/build suffix is dropped).
//!
//! Verified by `packaging/gates/check-linkage.sh` (readelf / otool).

use std::env;

const LIB_NAME: &str = "SzConfigTool";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    match target_os.as_str() {
        "linux" => {
            println!("cargo:rustc-cdylib-link-arg=-Wl,-soname,lib{LIB_NAME}.so");
        }
        "macos" => {
            let version = numeric_version();
            println!("cargo:rustc-cdylib-link-arg=-Wl,-install_name,@rpath/lib{LIB_NAME}.dylib");
            println!("cargo:rustc-cdylib-link-arg=-Wl,-current_version,{version}");
        }
        _ => {}
    }
}

/// `MAJOR.MINOR.PATCH` of the crate version, without any SemVer prerelease or
/// build metadata (cargo exposes the numeric parts separately).
fn numeric_version() -> String {
    let part = |name: &str| env::var(name).unwrap_or_else(|_| panic!("{name} is set by cargo"));
    format!(
        "{}.{}.{}",
        part("CARGO_PKG_VERSION_MAJOR"),
        part("CARGO_PKG_VERSION_MINOR"),
        part("CARGO_PKG_VERSION_PATCH")
    )
}
