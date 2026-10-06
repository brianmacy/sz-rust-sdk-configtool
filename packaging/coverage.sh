#!/usr/bin/env bash
# Coverage of every component, measured from ONE instrumented test pass, then
# gated by packaging/lib/coverage_gate.py against coverage/policy.yaml
# (100% of lines, branches and functions; the only allowed gaps are the
# reviewed, justified entries in that file).
#
#   rust    cargo-llvm-cov (pinned, install-tools.sh): `cargo test --workspace`
#           with -C instrument-coverage (this IS the Rust test run), then the
#           C ABI, JNI, napi and pyo3 cdylibs are built instrumented
#           (debug) for the host suites below, so the Rust seams are measured
#           through the languages that call them.
#   python  coverage.py (hash-pinned, requirements-test.txt): pytest against
#           a wheel of the instrumented pyo3 extension
#   node    node --test --experimental-test-coverage (built in): bindings/node
#           and bindings/node/trpc against the instrumented .node
#   java    JaCoCo (Maven profile `coverage`): mvn test against the
#           instrumented JNI library
#   dotnet  coverlet.msbuild: dotnet test against the instrumented C ABI
#   cpp     clang source-based coverage (-DSZCONFIGTOOL_ENABLE_COVERAGE=ON)
#           of the header-only binding, together with ASan + UBSan, against a
#           NON-instrumented release C ABI (two LLVM profile runtimes cannot
#           share one process); the C ABI's Rust code is measured by the
#           Rust, dotnet and C-ABI runs.
#   gate    merge the Rust profiles, write every lcov/XML report and apply
#           the gate (run last; needs the others).
#
# Reports: ${CARGO_TARGET_DIR}/coverage/ (git-ignored). Instrumented cargo
# output: ${CARGO_TARGET_DIR}/llvm-cov-target (never the release target dir).
#
# Usage: packaging/coverage.sh [rust|python|node|java|dotnet|cpp|gate ...]   (default: all, in that order)
# shellcheck source=lib/common.sh
source "$(dirname "$0")/lib/common.sh"
# shellcheck source=lib/tools-env.sh
source "${PACKAGING_DIR}/lib/tools-env.sh"

OS="$(host_os)"
COV_OUT="${CARGO_TARGET_DIR}/coverage"
COV_TARGET="${CARGO_TARGET_DIR}/llvm-cov-target"
POLICY="${REPO_ROOT}/coverage/policy.yaml"
GATE=("$(python_bin)" "${PACKAGING_DIR}/lib/coverage_gate.py" "${POLICY}")
NPROC="$(getconf _NPROCESSORS_ONLN 2>/dev/null || echo 4)"
mkdir -p "${COV_OUT}"

# Instrumented cargo environment (RUSTC_WRAPPER, LLVM_PROFILE_FILE, ...),
# exported only inside the functions that build or run instrumented code.
cov_env() {
    command -v cargo-llvm-cov >/dev/null || die "cargo-llvm-cov missing; run packaging/install-tools.sh <target> cargo-tools"
    local env_sh
    env_sh="$(cd "${REPO_ROOT}" && CARGO_TARGET_DIR="${COV_TARGET}" cargo llvm-cov show-env --sh)"
    eval "${env_sh}"
    export CARGO_TARGET_DIR="${COV_TARGET}"
}

# Debug cdylib file of a crate, as cargo names it on this host.
cov_lib() { echo "${COV_TARGET}/debug/$(shared_lib_name "${OS}" "$1")"; }

cov_rust() {
    (
        cov_env
        cd "${REPO_ROOT}"
        cargo llvm-cov clean --workspace
        log "instrumented Rust tests (workspace)"
        cargo test --workspace
        log "instrumented C ABI / JNI / napi cdylibs"
        cargo build -p sz-configtool-ffi -p sz-configtool-jni -p sz-configtool-node
        log "instrumented pyo3 wheel"
        local args=(build --manifest-path Cargo.toml --out "${COV_TARGET}/wheels")
        [[ "${OS}" == linux ]] && args+=(--compatibility linux)
        rm -rf "${COV_TARGET}/wheels"
        (cd bindings/python && "$(tools_venv_python)" -m maturin "${args[@]}")
    )
}

cov_python() {
    local venv="${COV_TARGET}/py-venv" py wheel
    shopt -s nullglob
    local wheels=("${COV_TARGET}"/wheels/sz_configtool-*.whl)
    shopt -u nullglob
    [[ ${#wheels[@]} -eq 1 ]] || die "expected one instrumented wheel; run: coverage.sh rust"
    wheel="${wheels[0]}"
    rm -rf "${venv}"
    "$(python_bin)" -m venv "${venv}"
    py="${venv}/bin/python"
    [[ -x "${venv}/Scripts/python.exe" ]] && py="${venv}/Scripts/python.exe"
    "${py}" -m pip install --disable-pip-version-check -q --only-binary :all: --require-hashes \
        -r "${PACKAGING_DIR}/requirements-test.txt"
    "${py}" -m pip install --disable-pip-version-check -q --no-deps "$(native_path "${wheel}")"
    (
        cov_env
        cd "${REPO_ROOT}/bindings/python"
        rm -f "${COV_OUT}/python.coverage"
        COVERAGE_FILE="${COV_OUT}/python.coverage" "${py}" -m coverage run --branch \
            --source=sz_configtool -m pytest -q -p no:cacheprovider
        COVERAGE_FILE="${COV_OUT}/python.coverage" "${py}" -m coverage lcov -q -o "${COV_OUT}/python.lcov"
        COVERAGE_FILE="${COV_OUT}/python.coverage" "${py}" -m coverage report -m
    )
}

# node_cov <dir> <lcov-name>: the package's own tests with Node's built-in
# coverage (source-mapped to the TypeScript), lcov to ${COV_OUT}.
node_cov() {
    (cd "$1" && node --test --experimental-strip-types --enable-source-maps \
        --experimental-test-coverage --test-coverage-exclude='test/**' \
        --test-reporter=spec --test-reporter-destination=stdout \
        --test-reporter=lcov --test-reporter-destination="$(native_path "${COV_OUT}/$2")" \
        test/*.test.ts)
}

cov_node() {
    local node_file="${COV_TARGET}/node/sz-configtool.node"
    mkdir -p "$(dirname "${node_file}")"
    cp "$(cov_lib szconfigtool_node)" "${node_file}"
    (cd "${REPO_ROOT}/bindings/node" && npm ci --no-audit --no-fund && npm run build:ts)
    (cd "${REPO_ROOT}/bindings/node/trpc" && npm ci --no-audit --no-fund && npm run build)
    (
        cov_env
        export SZ_CONFIGTOOL_NATIVE_PATH
        SZ_CONFIGTOOL_NATIVE_PATH="$(native_path "${node_file}")"
        node_cov "${REPO_ROOT}/bindings/node" node.lcov
        node_cov "${REPO_ROOT}/bindings/node/trpc" trpc.lcov
    )
}

cov_java() {
    (
        cov_env
        cd "${REPO_ROOT}/bindings/java"
        mvn -B -ntp -Pcoverage test -Dnative.lib.dir="$(native_path "${COV_TARGET}/debug")"
        cp target/site/jacoco/jacoco.xml "${COV_OUT}/java.jacoco.xml"
    )
}

cov_dotnet() {
    (
        cov_env
        SZCONFIGTOOL_NATIVE_DIR="$(native_path "${COV_TARGET}/debug")" \
            dotnet test "$(native_path "${REPO_ROOT}/bindings/csharp/Sz.ConfigTool.sln")" -c Debug \
            -p:CollectCoverage=true -p:CoverletOutputFormat=lcov \
            -p:CoverletOutput="$(native_path "${COV_OUT}")/dotnet.lcov" \
            -p:Include='[Sz.ConfigTool]*'
    )
}

# llvm_tool <name>: the llvm-profdata / llvm-cov of the C++ compiler's own
# LLVM (raw profiles are only readable by the matching version): Xcode's on
# macOS; elsewhere the one next to the resolved clang++, or <name>-<major>.
llvm_tool() {
    if [[ "${OS}" == macos ]]; then
        xcrun --find "$1"
        return
    fi
    local cxx dir major
    cxx="$(command -v "${CXX:-clang++}")" || die "clang++ not found (C++ coverage needs clang)"
    dir="$(dirname "$(readlink -f "${cxx}")")"
    if [[ -x "${dir}/$1" ]]; then
        echo "${dir}/$1"
        return
    fi
    major="$("${cxx}" -dumpversion | cut -d. -f1)"
    command -v "$1-${major}" || die "$1 for clang ${major} not found (Debian/Ubuntu: apt-get install llvm-${major})"
}

cov_cpp() {
    local build="${COV_TARGET}/cpp" profiles="${COV_TARGET}/cpp-profiles"
    log "release C ABI (not instrumented) for the C++ binding"
    (cd "${REPO_ROOT}" && cargo build -p sz-configtool-ffi --release)
    rm -rf "${build}" "${profiles}"
    local generator=()
    command -v ninja >/dev/null 2>&1 && generator=(-G Ninja)
    CC="${CC:-clang}" CXX="${CXX:-clang++}" cmake -S "${REPO_ROOT}/bindings/cpp" -B "${build}" \
        ${generator[@]+"${generator[@]}"} \
        -DCMAKE_BUILD_TYPE=Debug \
        -DSZCONFIGTOOL_ENABLE_SANITIZERS=ON \
        -DSZCONFIGTOOL_ENABLE_COVERAGE=ON \
        -DSZCONFIGTOOL_NATIVE_DIR="${CARGO_TARGET_DIR}/release"
    cmake --build "${build}" --parallel
    LLVM_PROFILE_FILE="${profiles}/%p-%m.profraw" ctest --test-dir "${build}" -j "${NPROC}" --output-on-failure
    "$(llvm_tool llvm-profdata)" merge -sparse "${profiles}"/*.profraw -o "${build}/cpp.profdata"
    # The shared-library test binary and the example. The static test binary
    # runs the same tests but compiles the headers with SZCONFIGTOOL_STATIC
    # (different function hashes), so it is left out of the report.
    local cov_args=(-instr-profile="${build}/cpp.profdata" "${build}/tests/szconfigtool_tests"
        -object "${build}/szconfigtool_quickstart" "${REPO_ROOT}/bindings/cpp/include")
    "$(llvm_tool llvm-cov)" export -format=lcov "${cov_args[@]}" >"${COV_OUT}/cpp.lcov"
    "$(llvm_tool llvm-cov)" report "${cov_args[@]}"
}

cov_gate() {
    (
        cov_env
        cd "${REPO_ROOT}"
        # `report` also reads the instrumented cdylibs in ${COV_TARGET}/debug,
        # so the seams' coverage from the host suites is included.
        cargo llvm-cov report --workspace --lcov --output-path "${COV_OUT}/rust.lcov"
        cargo llvm-cov report --workspace --json --output-path "${COV_OUT}/rust.json"
        cargo llvm-cov report --workspace --summary-only
    )
    local status=0 component
    for component in rust python node trpc java dotnet cpp; do
        "${GATE[@]}" "${component}" "${COV_OUT}" || status=1
    done
    [[ ${status} -eq 0 ]] || die "coverage gate failed (see the tables above; policy: coverage/policy.yaml)"
    log "coverage gate passed: every component at its coverage/policy.yaml floor"
}

STEPS=("$@")
[[ ${#STEPS[@]} -gt 0 ]] || STEPS=(rust python node java dotnet cpp gate)
for step in "${STEPS[@]}"; do
    log "coverage: ${step}"
    case "${step}" in
        rust) cov_rust ;;
        python) cov_python ;;
        node) cov_node ;;
        java) cov_java ;;
        dotnet) cov_dotnet ;;
        cpp) cov_cpp ;;
        gate) cov_gate ;;
        *) die "unknown step '${step}'" ;;
    esac
done
log "coverage: ${STEPS[*]} OK"
