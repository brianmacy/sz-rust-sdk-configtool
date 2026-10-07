#!/usr/bin/env bash
# Build and run the C test + C example against the STAGED C ABI of a target
# (shared and static link) via packaging/ctest/CMakeLists.txt. Runs on the
# target's own runner (it executes the binaries).
#
# Usage: packaging/gates/run-c-tests.sh <target>
# shellcheck source=../lib/common.sh
source "$(dirname "$0")/../lib/common.sh"

TARGET="${1:-}"
require_target "${TARGET}"
native_c="$(target_stage "${TARGET}")/native/c"
[[ -d "${native_c}" ]] || die "no staged C ABI in ${native_c}; run build-native.sh ${TARGET}"
build="${CARGO_TARGET_DIR}/sz-ctest-${TARGET}"
rm -rf "${build}"

# Empty-array expansion uses ${a[@]+...}: bash 3.2 (macOS /bin/bash) treats
# "${a[@]}" of an empty array as unbound under `set -u`.
generator=()
if [[ "$(host_os)" != windows ]] && command -v ninja >/dev/null 2>&1; then
    generator=(-G Ninja)
fi
cmake -S "$(native_path "${PACKAGING_DIR}/ctest")" -B "$(native_path "${build}")" ${generator[@]+"${generator[@]}"} \
    -DCMAKE_BUILD_TYPE=Release \
    -DSZ_NATIVE_C_DIR="$(native_path "${native_c}")" -DSZ_REPO_ROOT="$(native_path "${REPO_ROOT}")"
cmake --build "$(native_path "${build}")" --config Release
ctest --test-dir "$(native_path "${build}")" -C Release --output-on-failure
echo "PASS: C ABI tests for ${TARGET}"
