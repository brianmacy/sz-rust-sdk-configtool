#!/usr/bin/env bash
# Build every native library of one release target (plain optimized release
# profile; never sanitizers) and stage them under ${SZ_DIST_DIR}/<target>/:
#
#   native/c/include/libSzConfigTool.h
#   native/c/lib/   libSzConfigTool.{so,dylib} + libSzConfigTool.a          (Linux/macOS)
#                   SzConfigTool.lib (DLL import lib) + SzConfigTool_static.lib (MSVC)
#   native/c/bin/   SzConfigTool.dll + SzConfigTool.pdb                     (Windows)
#   native/c/native-static-libs.txt   system libs the static archive needs
#   native/jni/     JNI library (Java binding)
#   native/node/    sz-configtool.<napi_tag>.node
#   sbom/           CycloneDX SBOMs (one per shipped crate)
#
# Linux: cargo-zigbuild against glibc 2.34 (generic x86-64 / aarch64; no
# target-cpu). macOS: plain cargo, MACOSX_DEPLOYMENT_TARGET from config.yaml.
# Windows: plain cargo with an explicit MSVC link.exe; DLL/.node/.pyd use the
# static MSVC runtime (+crt-static), the static archive the dynamic one (/MD).
#
# Usage: packaging/build-native.sh <target>
# shellcheck source=lib/common.sh
source "$(dirname "$0")/lib/common.sh"
# shellcheck source=lib/tools-env.sh
source "${PACKAGING_DIR}/lib/tools-env.sh"
# shellcheck source=lib/build-env.sh
source "${PACKAGING_DIR}/lib/build-env.sh"

TARGET="${1:-}"
require_target "${TARGET}"
setup_build_env "${TARGET}"

STAGE="$(target_stage "${TARGET}")"
OUT_DIR="${CARGO_TARGET_DIR}/${RUST_TARGET}/release"
# A build starts a fresh stage: leftovers of an earlier run (out/ of another
# version, wheels, smoke-venv, pkg-*) would otherwise be scanned by the gates
# and could be shipped.
rm -rf "${STAGE}"
mkdir -p "${STAGE}/native/c/include" "${STAGE}/native/c/lib" "${STAGE}/native/jni" "${STAGE}/native/node" "${STAGE}/sbom"

cd "${REPO_ROOT}" || exit 1

# 1. C ABI. Relink sz-configtool-ffi so rustc prints native-static-libs.
#    Windows: the DLL is built with the static CRT (lib/build-env.sh); the
#    static archive is then rebuilt alone with the default dynamic CRT (/MD),
#    so its native-static-libs.txt names the CRT /MD consumers link anyway.
log "building sz-configtool-ffi for ${BUILD_TARGET} (${BUILDER})"
cargo clean -p sz-configtool-ffi --release --target "${RUST_TARGET}" 2>/dev/null || true
log_file="${STAGE}/native/ffi-build.log"
ffi_static_build=(rustc -p sz-configtool-ffi --lib --release --target "${BUILD_TARGET}")
if [[ "${OS}" == windows ]]; then
    cargo_cmd build -p sz-configtool-ffi --lib --release --target "${BUILD_TARGET}"
    mkdir -p "${STAGE}/native/c/bin"
    cp "${OUT_DIR}/SzConfigTool.dll" "${OUT_DIR}/SzConfigTool.pdb" "${STAGE}/native/c/bin/"
    # rustc: import lib = SzConfigTool.dll.lib, staticlib = SzConfigTool.lib.
    cp "${OUT_DIR}/SzConfigTool.dll.lib" "${STAGE}/native/c/lib/SzConfigTool.lib"
    CARGO_ENCODED_RUSTFLAGS="${SZ_RUSTFLAGS_DYNAMIC_CRT}" \
        cargo_cmd "${ffi_static_build[@]}" --crate-type staticlib -- --print=native-static-libs 2>&1 |
        tee "${log_file}" >&2
else
    cargo_cmd "${ffi_static_build[@]}" -- --print=native-static-libs 2>&1 | tee "${log_file}" >&2
fi
static_libs="$(sed -n 's/.*native-static-libs: //p' "${log_file}" | tail -1 | tr -d '\r')"
[[ -n "${static_libs}" ]] || die "rustc did not report native-static-libs"
printf '%s\n' "${static_libs}" >"${STAGE}/native/c/native-static-libs.txt"
rm -f "${log_file}"

cp "${REPO_ROOT}/ffi/include/libSzConfigTool.h" "${STAGE}/native/c/include/"
case "${OS}" in
    linux | macos)
        cp "${OUT_DIR}/$(shared_lib_name "${OS}" SzConfigTool)" "${STAGE}/native/c/lib/"
        cp "${OUT_DIR}/libSzConfigTool.a" "${STAGE}/native/c/lib/"
        ;;
    windows)
        cp "${OUT_DIR}/SzConfigTool.lib" "${STAGE}/native/c/lib/SzConfigTool_static.lib"
        ;;
esac

# 2. Rust-native binding seams (JNI, napi). Python is built by maturin (package-python.sh).
#    Their macOS install names (@rpath/...) are set by bindings/{jni,node}/build.rs.
for crate in jni node; do
    log "building sz-configtool-${crate} for ${BUILD_TARGET}"
    cargo_cmd build -p "sz-configtool-${crate}" --lib --release --target "${BUILD_TARGET}"
done
cp "${OUT_DIR}/$(shared_lib_name "${OS}" szconfigtool_jni)" "${STAGE}/native/jni/"
cp "${OUT_DIR}/$(shared_lib_name "${OS}" szconfigtool_node)" \
    "${STAGE}/native/node/sz-configtool.$(tcfg "${TARGET}" napi_tag).node"

# 3. SBOMs (CycloneDX 1.5, dependencies resolved for this target). The tool
#    writes next to every workspace Cargo.toml; collect and remove them, and
#    replace the absolute workspace path (every host spelling) with the remap prefix.
log "generating CycloneDX SBOMs"
sbom_name="sbom-${TARGET}-$$"
cargo cyclonedx --manifest-path "${REPO_ROOT}/Cargo.toml" -f json --spec-version 1.5 \
    --target "${RUST_TARGET}" --no-build-deps --override-filename "${sbom_name}" -q
for crate_dir in ffi bindings/jni bindings/node bindings/python; do
    crate="$(basename "${crate_dir}")"
    [[ "${crate_dir}" == ffi ]] && crate=c
    # Both spellings of the root: Git Bash's /d/... and the native D:/... (the
    # script also covers D:\... and its JSON-escaped form; lib/sbom_paths.py).
    "$(python_bin)" "${PACKAGING_DIR}/lib/sbom_paths.py" \
        "${REPO_ROOT}/${crate_dir}/${sbom_name}.json" "${STAGE}/sbom/sz-configtool-${crate}.cdx.json" \
        "${REMAP_PREFIX}" "${REPO_ROOT}" "$(native_path "${REPO_ROOT}")"
done
find "${REPO_ROOT}" -name "${sbom_name}.json" -not -path "${CARGO_TARGET_DIR}/*" -delete

log "staged natives for ${TARGET} in ${STAGE}/native"
