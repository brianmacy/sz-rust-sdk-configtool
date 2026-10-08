#!/usr/bin/env bash
# The ONE native archive of a target -- C ABI and C++ binding together:
#   ${SZ_DIST_DIR}/<target>/out/sz-configtool-<version>-<os>-<arch>.{tar.gz|zip}
#
#   include/libSzConfigTool.h            C header
#   include/szconfigtool/                C++20 header-only binding
#   lib/                                 shared library (Windows: import library
#                                        SzConfigTool.lib) + static library
#                                        (Windows: SzConfigTool_static.lib),
#                                        native-static-libs.txt
#   lib/cmake/szconfigtool/              find_package(szconfigtool) package
#   bin/                                 SzConfigTool.dll + .pdb (Windows)
#   sbom/sz-configtool-c.cdx.json, LICENSE, README.md, VERSION
#
# Assembly = `cmake --install` of bindings/cpp over the STAGED natives
# (headers, libraries, CMake package), then every staged native/c file over
# it, so the libraries shipped are the gated ones byte for byte. The archive is
# then validated AS SHIPPED (gates/check-native-archive.sh: extracted, C tests,
# the C++ test suite and find_package consumers against it). Plain optimized
# build: sanitizers/coverage are refused (the ASan/UBSan run lives in ci.yml).
#
# Usage: packaging/package-c.sh <target>   (after build-native.sh <target>)
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

name="sz-configtool-${VERSION}-${OS}-$(tcfg "${TARGET}" arch)"
root="${STAGE}/pkg-c/${name}"
build="${CARGO_TARGET_DIR}/sz-pkg-c-${TARGET}"
rm -rf "${STAGE}/pkg-c" "${build}"
mkdir -p "${root}" "${OUT}"

# 1. Headers, libraries and the CMake package: install-only configure (no
#    tests/examples). bindings/cpp looks for the libraries in one directory;
#    on Windows the DLL lives in bin/, so it is passed explicitly.
cpp_native_args=(-DSZCONFIGTOOL_NATIVE_DIR="$(native_path "${native_c}/lib")"
    -DSZCONFIGTOOL_C_INCLUDE_DIR="$(native_path "${native_c}/include")")
[[ "${OS}" == windows ]] &&
    cpp_native_args+=(-DSZCONFIGTOOL_SHARED_LIBRARY="$(native_path "${native_c}/bin/SzConfigTool.dll")")
cmake -S "$(native_path "${REPO_ROOT}/bindings/cpp")" -B "$(native_path "${build}")" \
    -DCMAKE_BUILD_TYPE=Release -DSZCONFIGTOOL_BUILD_TESTS=OFF -DSZCONFIGTOOL_BUILD_EXAMPLES=OFF \
    "${cpp_native_args[@]}"
cmake --install "$(native_path "${build}")" --config Release --prefix "$(native_path "${root}")"

# 2. Every staged C ABI file (pdb, native-static-libs.txt, ...), replacing
#    the installed copy: the archive ships exactly the gated bytes and modes.
#    (macOS: the install rule's `install_name_tool -id @rpath/...` re-signs
#    the dylib even though its id already is @rpath/..., changing 3 bytes of
#    LC_CODE_SIGNATURE, and install(FILES) drops the executable bit.)
while IFS= read -r rel; do
    if [[ -e "${root}/${rel}" ]] && ! cmp -s "${native_c}/${rel}" "${root}/${rel}"; then
        log "shipping the staged ${rel} (cmake --install wrote a different copy)"
    fi
    mkdir -p "$(dirname "${root}/${rel}")"
    rm -f "${root}/${rel}"
    cp "${native_c}/${rel}" "${root}/${rel}"
done < <(cd "${native_c}" && find . -type f | sed 's|^\./||' | LC_ALL=C sort)

mkdir -p "${root}/sbom"
cp "${STAGE}/sbom/sz-configtool-c.cdx.json" "${root}/sbom/"
cp "${REPO_ROOT}/LICENSE" "${root}/"
printf '%s\n' "${VERSION}" >"${root}/VERSION"
# @SERIES@ = X.Y, what find_package(szconfigtool X.Y) pins (SameMinorVersion).
series="$(cut -d. -f1-2 <<<"${VERSION}")"
sed -e "s/@VERSION@/${VERSION}/g" -e "s/@TARGET@/${TARGET}/g" -e "s/@SERIES@/${series}/g" \
    "${PACKAGING_DIR}/templates/c-archive-README.md" >"${root}/README.md"

archive="${OUT}/${name}.$(tcfg "${TARGET}" archive)"
SOURCE_DATE_EPOCH="${SOURCE_DATE_EPOCH:-$(source_date_epoch)}" \
    "$(python_bin)" "${PACKAGING_DIR}/lib/archive.py" "${root}" "${archive}"
log "wrote ${archive}"

# 3. Test the archive that ships.
"${PACKAGING_DIR}/gates/check-native-archive.sh" "${TARGET}" "${archive}"
