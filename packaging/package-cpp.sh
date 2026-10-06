#!/usr/bin/env bash
# C++ binding package of one target: configure bindings/cpp against the STAGED
# libSzConfigTool, build and run its test suite (plain optimized build — the
# ASan/UBSan run lives only in .github/workflows/ci.yml), install to a prefix
# and archive it:
#   out/sz-configtool-cpp-<version>-<os>-<arch>.{tar.gz|zip}
# (headers, libSzConfigTool.h, shared + static library, lib/cmake/szconfigtool).
#
# Usage: packaging/package-cpp.sh <target>
# shellcheck source=lib/common.sh
source "$(dirname "$0")/lib/common.sh"

TARGET="${1:-}"
require_target "${TARGET}"
VERSION="$(workspace_version)"
OS="$(tcfg "${TARGET}" os)"
STAGE="$(target_stage "${TARGET}")"
OUT="$(target_out "${TARGET}")"
native_c="${STAGE}/native/c"
[[ -d "${native_c}" ]] || die "no staged C ABI; run build-native.sh ${TARGET}"
case "${CFLAGS:-} ${CXXFLAGS:-} ${LDFLAGS:-}" in
    *-fsanitize*) die "release packaging must not use sanitizer flags" ;;
    *-fprofile-instr-generate* | *--coverage*) die "release packaging must not use coverage flags" ;;
esac

name="sz-configtool-cpp-${VERSION}-${OS}-$(tcfg "${TARGET}" arch)"
build="${CARGO_TARGET_DIR}/sz-cpp-${TARGET}"
prefix="${STAGE}/pkg-cpp/${name}"
rm -rf "${build}" "${STAGE}/pkg-cpp"
mkdir -p "${OUT}"

# bindings/cpp looks for the shared and static library in one directory.
natives="${STAGE}/cpp-natives"
rm -rf "${natives}" && mkdir -p "${natives}"
cp "${native_c}"/lib/* "${natives}/"
[[ "${OS}" == windows ]] && cp "${native_c}"/bin/* "${natives}/"

# Empty-array expansion uses ${a[@]+...}: bash 3.2 (macOS /bin/bash) treats
# "${a[@]}" of an empty array as unbound under `set -u`.
generator=()
if [[ "${OS}" != windows ]] && command -v ninja >/dev/null 2>&1; then
    generator=(-G Ninja)
fi
cmake -S "$(native_path "${REPO_ROOT}/bindings/cpp")" -B "$(native_path "${build}")" ${generator[@]+"${generator[@]}"} \
    -DCMAKE_BUILD_TYPE=Release \
    -DSZCONFIGTOOL_ENABLE_SANITIZERS=OFF \
    -DSZCONFIGTOOL_ENABLE_COVERAGE=OFF \
    -DSZCONFIGTOOL_NATIVE_DIR="$(native_path "${natives}")" \
    -DSZCONFIGTOOL_C_INCLUDE_DIR="$(native_path "${native_c}/include")"
cmake --build "$(native_path "${build}")" --config Release --parallel
(
    [[ "${OS}" == windows ]] && export PATH="${natives}:${PATH}"
    ctest --test-dir "$(native_path "${build}")" -C Release --output-on-failure -j "$(getconf _NPROCESSORS_ONLN 2>/dev/null || echo 4)"
)
cmake --install "$(native_path "${build}")" --config Release --prefix "$(native_path "${prefix}")"
cp "${REPO_ROOT}/LICENSE" "${prefix}/"
printf '%s\n' "${VERSION}" >"${prefix}/VERSION"

archive="${OUT}/${name}.$(tcfg "${TARGET}" archive)"
SOURCE_DATE_EPOCH="${SOURCE_DATE_EPOCH:-$(source_date_epoch)}" \
    "$(python_bin)" "${PACKAGING_DIR}/lib/archive.py" "${prefix}" "${archive}"
log "wrote ${archive}"
