#!/usr/bin/env bash
# Verify that an assembled release directory holds EXACTLY the expected asset
# set (nothing missing, nothing extra) and that SHA256SUMS lists exactly those
# files. Expected, for every target in config.yaml (or the targets given):
#
#   sz-configtool-<v>-<os>-<arch>.<archive>          native archive (C ABI +
#                                                    C++ binding + CMake package)
#   sz-configtool-node-<v>-<os>-<arch>.tgz           npm tarball (holds the .node)
#   python_wheel targets only (Linux; Senzing's Python SDK is Linux only):
#     sz_configtool-<pep440-v>-cp310-abi3-<python_compat>_<cpu>.whl
#
# plus the universal sz-configtool-<v>.jar, Sz.ConfigTool.<v>.nupkg and
# sz-configtool-trpc-<v>.tgz. A macOS / Windows wheel is therefore an error.
# Never release assets (each is an explicit FAIL, besides being "extra"):
# a standalone SBOM (*.cdx.json; the native archive embeds its own under
# sbom/), an attestation bundle (*.intoto.jsonl; attestations are verified
# online with `gh attestation verify`), a separate C++ package
# (sz-configtool-cpp-*; the native archive holds the C++ binding) and a raw
# Node addon (*.node; it ships only inside the Node tarball). Nor any
# subdirectory or non-regular file.
# Self-test: packaging/gates/test-release-assets.sh.
#
# Usage: packaging/gates/check-release-assets.sh <release-dir> [target...]
# shellcheck source=../lib/common.sh
source "$(dirname "$0")/../lib/common.sh"

DIR="${1:?usage: check-release-assets.sh <release-dir> [target...]}"
shift
[[ -d "${DIR}" ]] || die "no such dir ${DIR}"
[[ -f "${DIR}/SHA256SUMS" ]] || die "${DIR}/SHA256SUMS missing; run make-sums.sh"
TARGETS=("$@")
if [[ ${#TARGETS[@]} -eq 0 ]]; then
    while IFS= read -r t; do TARGETS+=("${t}"); done < <(cfg_keys targets)
fi
V="$(workspace_version)"
PY_V="$(pep440_version "${V}")"

expected=("sz-configtool-${V}.jar" "Sz.ConfigTool.${V}.nupkg" "sz-configtool-trpc-${V}.tgz")
for t in "${TARGETS[@]}"; do
    require_target "${t}"
    plat="$(tcfg "${t}" os)-$(tcfg "${t}" arch)"
    archive="$(tcfg "${t}" archive)"
    expected+=("sz-configtool-${V}-${plat}.${archive}" "sz-configtool-node-${V}-${plat}.tgz")
    if target_has_python "${t}"; then
        cpu="$(tcfg "${t}" rust_target)"
        expected+=("sz_configtool-${PY_V}-cp310-abi3-$(tcfg "${t}" python_compat)_${cpu%%-*}.whl")
    fi
done

want="$(printf '%s\n' "${expected[@]}" | LC_ALL=C sort)"
have="$(find "${DIR}" -maxdepth 1 -type f ! -name SHA256SUMS -exec basename {} \; | LC_ALL=C sort)"
summed="$(tr -d '\r' <"${DIR}/SHA256SUMS" | sed 's/^[0-9a-f]\{64\} [ *]//' | LC_ALL=C sort)"
FAIL=0
forbidden="$(find "${DIR}" -mindepth 1 \( -name '*.cdx.json' -o -name '*.intoto.jsonl' \) -exec basename {} \; | LC_ALL=C sort)"
if [[ -n "${forbidden}" ]]; then
    echo "FAIL: SBOM / attestation bundle files must never be release assets:"
    echo "${forbidden}"
    FAIL=1
fi
# forbid <find -name pattern> <message>: explicit FAIL for files that must never ship.
forbid() {
    local found
    found="$(find "${DIR}" -mindepth 1 -name "$1" -exec basename {} \; | LC_ALL=C sort)"
    [[ -z "${found}" ]] && return 0
    echo "FAIL: $2:"
    echo "${found}"
    FAIL=1
}
forbid 'sz-configtool-cpp-*' "the C++ binding ships inside the native archive sz-configtool-<v>-<os>-<arch>, not as a separate package"
forbid '*.node' "the Node addon ships only inside the Node tarball sz-configtool-node-<v>-<os>-<arch>.tgz"
irregular="$(find "${DIR}" -mindepth 1 -maxdepth 1 ! -type f -exec basename {} \; | LC_ALL=C sort)"
if [[ -n "${irregular}" ]]; then
    echo "FAIL: not a regular file (subdirectory, link, ...):"
    echo "${irregular}"
    FAIL=1
fi
if [[ "${have}" != "${want}" ]]; then
    echo "FAIL: release assets differ from the expected set (< expected, > present):"
    diff <(echo "${want}") <(echo "${have}") | grep '^[<>]' || true
    FAIL=1
fi
if [[ "${summed}" != "${have}" ]]; then
    echo "FAIL: SHA256SUMS does not list exactly the release files (< SHA256SUMS, > present):"
    diff <(echo "${summed}") <(echo "${have}") | grep '^[<>]' || true
    FAIL=1
fi
[[ ${FAIL} -eq 0 ]] || die "release asset set"
echo "PASS: ${#expected[@]} release assets (${TARGETS[*]}) match the expected set and SHA256SUMS"
