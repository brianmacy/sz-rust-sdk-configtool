#!/usr/bin/env bash
# Validate a target's native release archive AS SHIPPED: extract it to a
# scratch dir and, against the extracted tree only,
#   1. the C test + C example, shared and static (gates/run-c-tests.sh);
#   2. the bindings/cpp googletest suite, its in-tree example and its
#      install-then-find_package consumer tests (plain optimized build; the
#      ASan/UBSan run lives in .github/workflows/ci.yml);
#   3. bindings/cpp/examples/quickstart as an external consumer of the
#      archive's own CMake package (CMAKE_PREFIX_PATH = extracted root),
#      shared and static, run on the conformance fixture.
# Called by package-c.sh; runs on the target's own runner (it executes the
# binaries).
#
# Usage: packaging/gates/check-native-archive.sh <target> <archive>
# shellcheck source=../lib/common.sh
source "$(dirname "$0")/../lib/common.sh"

TARGET="${1:-}"
require_target "${TARGET}"
ARCHIVE="${2:?usage: check-native-archive.sh <target> <archive>}"
[[ -f "${ARCHIVE}" ]] || die "no such archive ${ARCHIVE}"
OS="$(tcfg "${TARGET}" os)"
work="${CARGO_TARGET_DIR}/sz-archive-check-${TARGET}"
rm -rf "${work}" && mkdir -p "${work}/x"

case "${ARCHIVE}" in
    *.zip) "$(python_bin)" -m zipfile -e "$(native_path "${ARCHIVE}")" "$(native_path "${work}/x")" ;;
    *.tar.gz) "$(python_bin)" -m tarfile --filter data -e "$(native_path "${ARCHIVE}")" "$(native_path "${work}/x")" ;;
    *) die "unknown archive type ${ARCHIVE}" ;;
esac
# archive.py: one top-level directory, named like the archive.
root="${work}/x/$(basename "${ARCHIVE}")"
root="${root%.zip}"
root="${root%.tar.gz}"
[[ -d "${root}" ]] || die "${ARCHIVE} has no top-level directory $(basename "${root}")"
fixture="${REPO_ROOT}/$(sed -n 's/^  fixture: //p' "${REPO_ROOT}/api/manifest/project.yaml")"
[[ -f "${fixture}" ]] || die "conformance fixture not found (api/manifest/project.yaml paths.fixture)"

# Empty-array expansion uses ${a[@]+...}: bash 3.2 (macOS /bin/bash) treats
# "${a[@]}" of an empty array as unbound under `set -u`.
generator=()
if [[ "${OS}" != windows ]] && command -v ninja >/dev/null 2>&1; then
    generator=(-G Ninja)
fi

# 1. C ABI.
"${PACKAGING_DIR}/gates/run-c-tests.sh" "${TARGET}" "${root}"

# 2. C++ test suite against the archive's libraries and C header (the DLL
#    lives in bin/, so on Windows it is passed explicitly).
cpp_native_args=(-DSZCONFIGTOOL_NATIVE_DIR="$(native_path "${root}/lib")"
    -DSZCONFIGTOOL_C_INCLUDE_DIR="$(native_path "${root}/include")")
[[ "${OS}" == windows ]] &&
    cpp_native_args+=(-DSZCONFIGTOOL_SHARED_LIBRARY="$(native_path "${root}/bin/SzConfigTool.dll")")
cmake -S "$(native_path "${REPO_ROOT}/bindings/cpp")" -B "$(native_path "${work}/cpp")" \
    ${generator[@]+"${generator[@]}"} -DCMAKE_BUILD_TYPE=Release \
    -DSZCONFIGTOOL_ENABLE_SANITIZERS=OFF -DSZCONFIGTOOL_ENABLE_COVERAGE=OFF "${cpp_native_args[@]}"
cmake --build "$(native_path "${work}/cpp")" --config Release --parallel
ctest --test-dir "$(native_path "${work}/cpp")" -C Release --output-on-failure \
    -j "$(getconf _NPROCESSORS_ONLN 2>/dev/null || echo 4)"

# 3. find_package(szconfigtool) from the extracted archive.
for mode in shared static; do
    use_static=OFF
    [[ "${mode}" == static ]] && use_static=ON
    b="${work}/consumer-${mode}"
    cmake -S "$(native_path "${REPO_ROOT}/bindings/cpp/examples/quickstart")" -B "$(native_path "${b}")" \
        ${generator[@]+"${generator[@]}"} -DCMAKE_BUILD_TYPE=Release \
        -DCMAKE_PREFIX_PATH="$(native_path "${root}")" -DSZCONFIGTOOL_USE_STATIC="${use_static}"
    cmake --build "$(native_path "${b}")" --config Release
    # Single-config generators put the executable in the build dir, Visual
    # Studio under Release/.
    exe="$(find "${b}" -path '*/CMakeFiles' -prune -o -type f \
        \( -name szconfigtool_quickstart -o -name szconfigtool_quickstart.exe \) -print)"
    [[ -n "${exe}" && "$(wc -l <<<"${exe}")" -eq 1 ]] || die "expected one built szconfigtool_quickstart in ${b}: ${exe}"
    # Windows has no rpath: the shared consumer needs the archive's DLL next
    # to it (the C tests and the C++ suite copy it post-build). Not via PATH:
    # under Git Bash ${root} is C:/... and the drive colon would split PATH.
    [[ "${OS}" == windows && "${mode}" == shared ]] && cp "${root}/bin/SzConfigTool.dll" "$(dirname "${exe}")/"
    "${exe}" "$(native_path "${fixture}")"
    log "find_package consumer (${mode}) of $(basename "${ARCHIVE}") OK"
done
echo "PASS: native archive $(basename "${ARCHIVE}") (C tests, C++ suite, find_package shared + static)"
