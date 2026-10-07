#!/usr/bin/env bash
# Install the pinned build tools for one release target into ${SZ_TOOLS_DIR}
# (default ${CARGO_TARGET_DIR}/sz-tools). Versions/hashes: packaging/config.yaml
# (aligned with G2; see packaging/README.md "Toolchain alignment").
# Every download is verified:
#   * Rust: rust-toolchain.toml via rustup (rustup verifies dist hashes)
#   * zig, JDK, Node: sha256 from config.yaml; Maven: sha512 from config.yaml
#   * cargo-zigbuild / cargo-cyclonedx / cargo-llvm-cov: `cargo install --locked`
#     (crates.io checksums); llvm-tools: rustup component of the pinned toolchain
#   * maturin / pytest / coverage.py: pip --require-hashes (packaging/requirements-*.txt)
#
# Usage: packaging/install-tools.sh <target> [component...]
#   components: rust zig cargo-tools python jdk maven node   (default: all that apply)
#               coverage   cargo-llvm-cov + llvm-tools (packaging/coverage.sh; never by default)
# Afterwards every packaging script finds the tools via lib/tools-env.sh;
# in GitHub Actions the PATH/JAVA_HOME are also exported to later steps.
# shellcheck source=lib/common.sh
source "$(dirname "$0")/lib/common.sh"
# shellcheck source=lib/tools-env.sh
source "${PACKAGING_DIR}/lib/tools-env.sh"

TARGET="${1:-}"
require_target "${TARGET}"
shift
BUILDER="$(tcfg "${TARGET}" builder)"
RUST_TARGET="$(tcfg "${TARGET}" rust_target)"
HOST="$(host_arch)-$(host_os)"
mkdir -p "${SZ_TOOLS_DIR}"

# fetch_verified <url> <file> <algo:256|512> <expected-hex>
fetch_verified() {
    log "downloading $1"
    curl --proto '=https' --tlsv1.2 -fsSL --retry 5 --retry-delay 10 "$1" -o "$2"
    local actual
    if [[ "$3" == 512 ]]; then
        actual="$(sha512_file "$2")"
    else
        actual="$(sha256_file "$2")"
    fi
    [[ "${actual}" == "$4" ]] || die "sha$3 mismatch for $(basename "$2"): expected $4, got ${actual}"
    log "sha$3 OK: $(basename "$2")"
}

# unpack <archive> <dest-dir>: extract and drop the single top-level directory.
unpack() {
    local tmp="$2.unpack"
    rm -rf "$2" "${tmp}" && mkdir -p "${tmp}"
    case "$1" in
        *.zip) "$(python_bin)" -m zipfile -e "$1" "${tmp}" ;;
        *) tar -xf "$1" -C "${tmp}" ;;
    esac
    local top=("${tmp}"/*)
    [[ ${#top[@]} -eq 1 && -d "${top[0]}" ]] || die "unexpected layout in $(basename "$1")"
    mv "${top[0]}" "$2"
    rm -rf "${tmp}" "$1"
}

install_rust() {
    log "Rust toolchain (rust-toolchain.toml) + target ${RUST_TARGET}"
    (cd "${REPO_ROOT}" && rustup toolchain install --no-self-update)
    (cd "${REPO_ROOT}" && rustup target add "${RUST_TARGET}")
    # macOS: llvm-strip strips the static archive's debug info (build-native.sh).
    if [[ "$(tcfg "${TARGET}" os)" == macos ]]; then
        (cd "${REPO_ROOT}" && rustup component add llvm-tools)
    fi
    (cd "${REPO_ROOT}" && rustc -V)
}

install_zig() {
    local version dir sha archive
    version="$(cfg tools.zig.version)"
    dir="${SZ_TOOLS_DIR}/zig-${version}"
    if [[ -x "${dir}/zig" ]] && "${dir}/zig" version | grep -qxF "${version}"; then
        log "zig ${version} present"
        return
    fi
    sha="$(cfg "tools.zig.sha256.${HOST}")" || die "no pinned zig sha256 for host ${HOST}"
    archive="${SZ_TOOLS_DIR}/zig.tar.xz"
    fetch_verified "https://ziglang.org/download/${version}/zig-${HOST}-${version}.tar.xz" "${archive}" 256 "${sha}"
    unpack "${archive}" "${dir}"
    "${dir}/zig" version | grep -qxF "${version}" || die "zig version mismatch"
}

install_cargo_tools() {
    local crate
    for crate in cargo-zigbuild cargo-cyclonedx; do
        [[ "${crate}" == cargo-zigbuild && "${BUILDER}" != zigbuild ]] && continue
        # Idempotent: cargo skips an identical installed version.
        cargo install --locked --version "$(cfg "tools.${crate//-/_}")" --root "${SZ_TOOLS_DIR}/cargo" "${crate}"
    done
}

# Coverage tooling (CI `linux` job, local runs): not part of a release build.
install_coverage_tools() {
    (cd "${REPO_ROOT}" && rustup component add llvm-tools)
    cargo install --locked --version "$(cfg tools.cargo_llvm_cov)" --root "${SZ_TOOLS_DIR}/cargo" cargo-llvm-cov
}

install_python_tools() {
    # Reuse a (cached) venv only if its interpreter still runs: when the host
    # Python moves (e.g. actions/setup-python picks a new patch release) the
    # venv's python symlink dangles; recreate it then.
    if [[ -d "${SZ_TOOLS_DIR}/venv" ]] && ! "$(tools_venv_python)" -c pass 2>/dev/null; then
        log "tools venv interpreter is broken (host Python changed?); recreating"
        rm -rf "${SZ_TOOLS_DIR}/venv"
    fi
    [[ -d "${SZ_TOOLS_DIR}/venv" ]] || "$(python_bin)" -m venv "${SZ_TOOLS_DIR}/venv"
    "$(tools_venv_python)" -m pip install --disable-pip-version-check -q --only-binary :all: --require-hashes \
        -r "${PACKAGING_DIR}/requirements-build.txt" -r "${PACKAGING_DIR}/requirements-test.txt"
    "$(tools_venv_python)" -m maturin --version
}

install_jdk() {
    local dir ext
    dir="$(tools_jdk_dir)"
    if [[ -x "$(tools_java_home)/bin/java" || -x "$(tools_java_home)/bin/java.exe" ]]; then
        log "JDK $(cfg tools.jdk.version) present"
        return
    fi
    ext=tar.gz
    [[ "$(host_os)" == windows ]] && ext=zip
    fetch_verified "$(cfg "tools.jdk.url.${HOST}")" "${SZ_TOOLS_DIR}/jdk.${ext}" 256 "$(cfg "tools.jdk.sha256.${HOST}")"
    unpack "${SZ_TOOLS_DIR}/jdk.${ext}" "${dir}"
    "$(tools_java_home)/bin/java" -version
}

install_maven() {
    local version dir ext key
    version="$(cfg tools.maven.version)"
    dir="${SZ_TOOLS_DIR}/maven-${version}"
    [[ -d "${dir}/bin" ]] && { log "Maven ${version} present"; return; }
    ext=tar.gz key=tar_gz
    [[ "$(host_os)" == windows ]] && ext=zip key=zip
    fetch_verified "https://archive.apache.org/dist/maven/maven-3/${version}/binaries/apache-maven-${version}-bin.${ext}" \
        "${SZ_TOOLS_DIR}/maven.${ext}" 512 "$(cfg "tools.maven.sha512.${key}")"
    unpack "${SZ_TOOLS_DIR}/maven.${ext}" "${dir}"
}

install_node() {
    local version dir plat ext
    version="$(cfg tools.node.version)"
    dir="${SZ_TOOLS_DIR}/node-${version}"
    [[ -d "${dir}" ]] && { log "Node ${version} present"; return; }
    case "${HOST}" in
        x86_64-linux) plat=linux-x64 ext=tar.xz ;;
        aarch64-linux) plat=linux-arm64 ext=tar.xz ;;
        aarch64-macos) plat=darwin-arm64 ext=tar.xz ;;
        x86_64-windows) plat=win-x64 ext=zip ;;
        *) die "no pinned Node for host ${HOST}" ;;
    esac
    fetch_verified "https://nodejs.org/dist/v${version}/node-v${version}-${plat}.${ext}" \
        "${SZ_TOOLS_DIR}/node.${ext}" 256 "$(cfg "tools.node.sha256.${HOST}")"
    unpack "${SZ_TOOLS_DIR}/node.${ext}" "${dir}"
}

COMPONENTS=("$@")
[[ ${#COMPONENTS[@]} -gt 0 ]] || COMPONENTS=(rust zig cargo-tools python jdk maven node)
for component in "${COMPONENTS[@]}"; do
    case "${component}" in
        rust) install_rust ;;
        zig) if [[ "${BUILDER}" == zigbuild ]]; then install_zig; fi ;;
        cargo-tools) install_cargo_tools ;;
        python) install_python_tools ;;
        jdk) install_jdk ;;
        maven) install_maven ;;
        node) install_node ;;
        coverage) install_coverage_tools ;;
        *) die "unknown component '${component}'" ;;
    esac
done

if [[ -n "${GITHUB_PATH:-}" ]]; then
    while IFS= read -r entry; do native_path "${entry}"; done < <(tools_path_entries) >>"${GITHUB_PATH}"
    if [[ -d "$(tools_java_home)" ]]; then
        echo "JAVA_HOME=$(native_path "$(tools_java_home)")" >>"${GITHUB_ENV}"
    fi
fi
log "tools ready in ${SZ_TOOLS_DIR}"
