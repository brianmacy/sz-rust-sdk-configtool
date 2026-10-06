# shellcheck shell=bash
# Shared helpers for packaging/*.sh. Source it; do not execute.
#
# Environment (all optional):
#   CARGO_TARGET_DIR  cargo output root           (default <repo>/target)
#   SZ_DIST_DIR       staging/output root          (default ${CARGO_TARGET_DIR}/dist, git-ignored)

set -euo pipefail

PACKAGING_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPO_ROOT="$(cd "${PACKAGING_DIR}/.." && pwd -P)"
CONFIG_FILE="${PACKAGING_DIR}/config.yaml"
CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-${REPO_ROOT}/target}"
SZ_DIST_DIR="${SZ_DIST_DIR:-${CARGO_TARGET_DIR}/dist}"
export CARGO_TARGET_DIR SZ_DIST_DIR

log() { printf '==> %s\n' "$*" >&2; }
die() { printf 'error: %s\n' "$*" >&2; exit 1; }

python_bin() {
    local py
    for py in python3 python; do
        if command -v "${py}" >/dev/null 2>&1 && "${py}" -c 'import sys; sys.exit(sys.version_info < (3, 10))' 2>/dev/null; then
            echo "${py}"
            return
        fi
    done
    die "python >= 3.10 is required"
}

# cfg <dotted.key>: scalar from packaging/config.yaml.
cfg() { "$(python_bin)" "${PACKAGING_DIR}/lib/cfg.py" "${CONFIG_FILE}" "$1"; }
# cfg_keys <dotted.key>: child keys of a map.
cfg_keys() { "$(python_bin)" "${PACKAGING_DIR}/lib/cfg.py" "${CONFIG_FILE}" --keys "$1"; }
# tcfg <target> <field>: a field of targets.<target>.
tcfg() { cfg "targets.$1.$2"; }

require_target() {
    local t="${1:-}"
    [[ -n "${t}" ]] || die "missing <target>; one of: $(cfg_keys targets | tr '\n' ' ')"
    cfg_keys targets | grep -qxF "${t}" || die "unknown target '${t}'; one of: $(cfg_keys targets | tr '\n' ' ')"
}

# Release version = the Cargo workspace version (single source of truth).
workspace_version() {
    (cd "${REPO_ROOT}" && cargo metadata --no-deps --format-version 1) |
        "$(python_bin)" -c 'import json,sys; print(next(p["version"] for p in json.load(sys.stdin)["packages"] if p["name"] == "sz-configtool-ffi"))'
}

# pep440_version <version>: the PEP 440 spelling maturin gives the workspace
# version (wheel file names, pip). Release-version policy (the ONLY accepted
# forms; packaging/gates/test-version-spellings.sh is the self-test):
#   X.Y.Z                       -> X.Y.Z
#   X.Y.Z-N   (N >= 1, release counter on a fixed Senzing line) -> X.Y.Z.postN
#   X.Y.Z-rc.N / -alpha.N / -beta.N                             -> X.Y.ZrcN / aN / bN
# Rejected: dev/post/pre/preview spellings (PEP 440 and Maven order them
# differently from SemVer, some ABOVE the release), bare labels, build
# metadata, leading zeros.
pep440_version() {
    "$(python_bin)" - "$1" <<'PY'
import re, sys
v = sys.argv[1]
num = r"(?:0|[1-9]\d*)"
m = re.fullmatch(rf"({num}\.{num}\.{num})(?:-([1-9]\d*)|-(rc|alpha|beta)\.({num}))?", v)
if not m:
    sys.exit(
        f"version '{v}' is not X.Y.Z, X.Y.Z-N (N >= 1) or X.Y.Z-rc.N/-alpha.N/-beta.N"
    )
release, counter, label, n = m.groups()
if counter:
    print(f"{release}.post{counter}")
elif label:
    short = {"alpha": "a", "beta": "b", "rc": "rc"}[label]
    print(f"{release}{short}{n}")
else:
    print(release)
PY
}

# Commit time of HEAD (reproducible archive mtimes); 0 outside a git checkout.
source_date_epoch() {
    git -C "${REPO_ROOT}" log -1 --format=%ct 2>/dev/null || echo 0
}

sha256_file() {
    if command -v sha256sum >/dev/null 2>&1; then
        sha256sum "$1" | awk '{print $1}'
    else
        shasum -a 256 "$1" | awk '{print $1}'
    fi
}

sha512_file() {
    if command -v sha512sum >/dev/null 2>&1; then
        sha512sum "$1" | awk '{print $1}'
    else
        shasum -a 512 "$1" | awk '{print $1}'
    fi
}

host_os() {
    case "$(uname -s)" in
        Linux) echo linux ;;
        Darwin) echo macos ;;
        MINGW* | MSYS* | CYGWIN*) echo windows ;;
        *) die "unsupported host OS $(uname -s)" ;;
    esac
}

host_arch() {
    case "$(uname -m)" in
        x86_64 | amd64) echo x86_64 ;;
        arm64 | aarch64) echo aarch64 ;;
        *) die "unsupported host arch $(uname -m)" ;;
    esac
}

# Native file names for a target OS.
# shared_lib_name <os> <stem>: libX.so / libX.dylib / X.dll
shared_lib_name() {
    case "$1" in
        linux) echo "lib$2.so" ;;
        macos) echo "lib$2.dylib" ;;
        windows) echo "$2.dll" ;;
    esac
}

# Per-target staging layout under ${SZ_DIST_DIR}/<target>/:
#   native/c/{lib,bin,include}   C ABI (shared + static + header)
#   native/jni/                  JNI library
#   native/node/                 sz-configtool.<napi_tag>.node
#   sbom/                        CycloneDX SBOMs
#   out/                         release assets for this target
target_stage() { echo "${SZ_DIST_DIR}/$1"; }
target_out() { echo "${SZ_DIST_DIR}/$1/out"; }

# Convert a path for native Windows tools when running under Git Bash.
native_path() {
    if [[ "$(host_os)" == windows ]] && command -v cygpath >/dev/null 2>&1; then
        cygpath -m "$1"
    else
        echo "$1"
    fi
}

# msvc_tool <exe>: absolute path of an MSVC x64 host/target tool (link.exe,
# dumpbin.exe, lib.exe) located with vswhere. Used instead of PATH lookup
# because Git Bash's /usr/bin/link.exe shadows MSVC's linker (G2 pitfall:
# .claude/faqs/building/main-product-build.md "Rust linker").
msvc_tool() {
    local vswhere="C:/Program Files (x86)/Microsoft Visual Studio/Installer/vswhere.exe"
    [[ -x "${vswhere}" ]] || die "vswhere.exe not found; MSVC build tools are required"
    local found
    found="$("${vswhere}" -latest -products '*' -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 \
        -find "VC/Tools/MSVC/**/bin/Hostx64/x64/$1" | tr -d '\r' | head -1)"
    [[ -n "${found}" ]] || die "MSVC $1 not found via vswhere"
    cygpath -u "${found}"
}

# Under Git Bash hand cargo/MSBuild a Windows-form target dir (C:/...), which
# bash also accepts, instead of relying on MSYS env-var path conversion.
if [[ "$(host_os)" == windows ]]; then
    CARGO_TARGET_DIR="$(cygpath -m "${CARGO_TARGET_DIR}")"
    SZ_DIST_DIR="$(cygpath -m "${SZ_DIST_DIR}")"
    export CARGO_TARGET_DIR SZ_DIST_DIR
fi

# rust_llvm_tool <name>: path of an LLVM tool of the pinned Rust toolchain
# (rustup component llvm-tools, installed by install-tools.sh).
rust_llvm_tool() {
    local sysroot host tool
    sysroot="$(cd "${REPO_ROOT}" && rustc --print sysroot)"
    host="$(cd "${REPO_ROOT}" && rustc -vV | sed -n 's/^host: //p')"
    tool="${sysroot}/lib/rustlib/${host}/bin/$1"
    [[ -x "${tool}" || -x "${tool}.exe" ]] || die "$1 not found in ${sysroot}; run packaging/install-tools.sh <target> rust (rustup component llvm-tools)"
    echo "${tool}"
}
