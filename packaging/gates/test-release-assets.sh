#!/usr/bin/env bash
# Self-test of packaging/gates/check-release-assets.sh against mock release
# directories: the expected set (spelled out here independently of the gate,
# for the four config.yaml targets) must PASS; a standalone SBOM
# (*.cdx.json), an attestation bundle (*.intoto.jsonl) — even when listed in
# SHA256SUMS —, a stray file, a subdirectory, a missing asset, a macOS wheel
# and a stale SHA256SUMS must each FAIL.
#
# Usage: packaging/gates/test-release-assets.sh
# shellcheck source=../lib/common.sh
source "$(dirname "$0")/../lib/common.sh"

GATE="${PACKAGING_DIR}/gates/check-release-assets.sh"
SUMS="${PACKAGING_DIR}/make-sums.sh"
TMP="$(mktemp -d)"
trap 'rm -rf "${TMP}"' EXIT
V="$(workspace_version)"
PY_V="$(pep440_version "${V}")"

ASSETS=(
    "sz-configtool-${V}-linux-x64.tar.gz" "sz-configtool-${V}-linux-arm64.tar.gz"
    "sz-configtool-${V}-macos-arm64.tar.gz" "sz-configtool-${V}-windows-x64.zip"
    "sz-configtool-cpp-${V}-linux-x64.tar.gz" "sz-configtool-cpp-${V}-linux-arm64.tar.gz"
    "sz-configtool-cpp-${V}-macos-arm64.tar.gz" "sz-configtool-cpp-${V}-windows-x64.zip"
    "sz-configtool-node-${V}-linux-x64.tgz" "sz-configtool-node-${V}-linux-arm64.tgz"
    "sz-configtool-node-${V}-macos-arm64.tgz" "sz-configtool-node-${V}-windows-x64.tgz"
    "sz-configtool.linux-x64-gnu.node" "sz-configtool.linux-arm64-gnu.node"
    "sz-configtool.darwin-arm64.node" "sz-configtool.win32-x64-msvc.node"
    "sz_configtool-${PY_V}-cp310-abi3-manylinux_2_34_x86_64.whl"
    "sz_configtool-${PY_V}-cp310-abi3-manylinux_2_34_aarch64.whl"
    "sz-configtool-${V}.jar" "Sz.ConfigTool.${V}.nupkg" "sz-configtool-trpc-${V}.tgz"
)

# mock <name>: a fresh release dir holding every expected asset; prints it.
mock() {
    local d="${TMP}/$1" a
    mkdir -p "${d}"
    for a in "${ASSETS[@]}"; do printf '%s\n' "${a}" >"${d}/${a}"; done
    echo "${d}"
}

# sums <dir>: write SHA256SUMS quietly (its failure stops the test).
sums() { "${SUMS}" "$1" >/dev/null 2>&1 || die "make-sums.sh $1 failed"; }

FAIL=0
# expect <pass|fail> <case> <dir> [grep pattern the gate output must contain]
expect() {
    local out rc=0 ok=false
    out="$("${GATE}" "$3" 2>&1)" || rc=$?
    case "$1" in
        pass) [[ ${rc} -eq 0 ]] && ok=true ;;
        fail) [[ ${rc} -ne 0 ]] && grep -qF -- "${4:-}" <<<"${out}" && ok=true ;;
    esac
    if [[ "${ok}" == true ]]; then
        echo "ok: $2"
    else
        echo "FAIL: $2 (expected $1, gate exit ${rc}):"
        echo "${out}"
        FAIL=1
    fi
}

d="$(mock good)"
sums "${d}"
expect pass "expected set (${#ASSETS[@]} assets + SHA256SUMS)" "${d}"

d="$(mock sbom)"
echo '{}' >"${d}/sz-configtool-${V}-linux-x64.cdx.json"
sums "${d}"
expect fail "standalone SBOM (in SHA256SUMS)" "${d}" "never be release assets"

d="$(mock bundle)"
echo '{}' >"${d}/sz-configtool-${V}.intoto.jsonl"
sums "${d}"
expect fail "attestation bundle (in SHA256SUMS)" "${d}" "never be release assets"

d="$(mock nested)"
mkdir -p "${d}/sbom" && echo '{}' >"${d}/sbom/sz-configtool-c.cdx.json"
sums "${d}"
expect fail "SBOM in a subdirectory" "${d}" "never be release assets"

d="$(mock stray)"
echo x >"${d}/notes.md"
sums "${d}"
expect fail "unexpected file" "${d}" "> notes.md"

d="$(mock missing)"
rm "${d}/Sz.ConfigTool.${V}.nupkg"
sums "${d}"
expect fail "missing asset" "${d}" "< Sz.ConfigTool.${V}.nupkg"

d="$(mock macwheel)"
echo x >"${d}/sz_configtool-${PY_V}-cp310-abi3-macosx_11_0_arm64.whl"
sums "${d}"
expect fail "macOS wheel" "${d}" "macosx_11_0_arm64.whl"

d="$(mock stale)"
sums "${d}"
echo '{}' >"${d}/sz-configtool-${V}.intoto.jsonl"
expect fail "bundle added after SHA256SUMS" "${d}" "never be release assets"

[[ ${FAIL} -eq 0 ]] || die "check-release-assets.sh self-test"
echo "PASS: check-release-assets.sh self-test"
