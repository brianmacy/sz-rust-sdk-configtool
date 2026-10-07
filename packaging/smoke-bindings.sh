#!/usr/bin/env bash
# Run each binding's own test suite against the STAGED release natives of a
# target (on that target's runner). Nothing here rebuilds a native library.
#
#   python  fresh venv, `pip install` the built wheel, pytest bindings/python/tests
#           (only targets that ship a wheel: config.yaml python_wheel, Linux)
#   node    npm test with SZ_CONFIGTOOL_NATIVE_PATH = staged .node
#   java    mvn test with the staged JNI library (-Dnative.lib.dir)
#   dotnet  dotnet test with SZCONFIGTOOL_NATIVE_DIR = staged C ABI
# (C++ is tested by package-cpp.sh, the C ABI by gates/run-c-tests.sh.)
#
# Usage: packaging/smoke-bindings.sh <target> [python|node|java|dotnet ...]
#        (default: all that the target ships)
# shellcheck source=lib/common.sh
source "$(dirname "$0")/lib/common.sh"
# shellcheck source=lib/tools-env.sh
source "${PACKAGING_DIR}/lib/tools-env.sh"

TARGET="${1:-}"
require_target "${TARGET}"
shift
OS="$(tcfg "${TARGET}" os)"
STAGE="$(target_stage "${TARGET}")"
OUT="$(target_out "${TARGET}")"

smoke_python() {
    target_has_python "${TARGET}" || die "no Python wheel for ${TARGET} (Linux targets only; config.yaml python_wheel)"
    local venv="${STAGE}/smoke-venv" py wheel
    shopt -s nullglob
    local wheels=("${OUT}"/sz_configtool-*.whl)
    shopt -u nullglob
    [[ ${#wheels[@]} -eq 1 ]] || die "expected one wheel in ${OUT}; run package-python.sh ${TARGET}"
    wheel="${wheels[0]}"
    rm -rf "${venv}"
    "$(python_bin)" -m venv "${venv}"
    py="${venv}/bin/python"
    [[ -x "${venv}/Scripts/python.exe" ]] && py="${venv}/Scripts/python.exe"
    "${py}" -m pip install --disable-pip-version-check -q --only-binary :all: --require-hashes \
        -r "${PACKAGING_DIR}/requirements-test.txt"
    "${py}" -m pip install --disable-pip-version-check -q --no-deps "$(native_path "${wheel}")"
    (cd "${REPO_ROOT}/bindings/python" && "${py}" -m pytest -q -p no:cacheprovider)
}

smoke_node() {
    local node_file
    node_file="${STAGE}/native/node/sz-configtool.$(tcfg "${TARGET}" napi_tag).node"
    [[ -f "${node_file}" ]] || die "missing ${node_file}"
    (cd "${REPO_ROOT}/bindings/node" && npm ci --no-audit --no-fund && npm run build:ts &&
        SZ_CONFIGTOOL_NATIVE_PATH="$(native_path "${node_file}")" npm test)
}

smoke_java() {
    (cd "${REPO_ROOT}/bindings/java" &&
        mvn -B -ntp test -Dnative.lib.dir="$(native_path "${STAGE}/native/jni")")
}

smoke_dotnet() {
    local dir="${STAGE}/native/c/lib"
    [[ "${OS}" == windows ]] && dir="${STAGE}/native/c/bin"
    SZCONFIGTOOL_NATIVE_DIR="$(native_path "${dir}")" \
        dotnet test "$(native_path "${REPO_ROOT}/bindings/csharp/Sz.ConfigTool.sln")" -c Release
}

SUITES=("$@")
if [[ ${#SUITES[@]} -eq 0 ]]; then
    SUITES=(node java dotnet)
    if target_has_python "${TARGET}"; then SUITES=(python "${SUITES[@]}"); fi
fi
for suite in "${SUITES[@]}"; do
    log "smoke: ${suite} (${TARGET})"
    case "${suite}" in
        python) smoke_python ;;
        node) smoke_node ;;
        java) smoke_java ;;
        dotnet) smoke_dotnet ;;
        *) die "unknown suite '${suite}'" ;;
    esac
done
log "smoke tests passed for ${TARGET}: ${SUITES[*]}"
