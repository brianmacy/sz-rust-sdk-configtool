#!/usr/bin/env bash
# Version consistency gate. The Cargo workspace version is the release
# version; every binding manifest must carry the same number, the tag (when
# given) must be v<version>, and the maturin pin must agree between
# config.yaml and requirements-build.txt.
#
# Accepted version forms (lib/common.sh pep440_version): X.Y.Z, X.Y.Z-N,
# X.Y.Z-rc.N / -alpha.N / -beta.N; anything else fails.
#
# Usage: packaging/gates/check-versions.sh [tag]     e.g. v4.4.0-1
# shellcheck source=../lib/common.sh
source "$(dirname "$0")/../lib/common.sh"

VERSION="$(workspace_version)"
FAIL=0
check() {
    if [[ "$2" == "${VERSION}" ]]; then
        echo "ok: $1 = $2"
    else
        echo "FAIL: $1 = '$2', workspace = ${VERSION}"
        FAIL=1
    fi
}

json_version() { "$(python_bin)" -c 'import json,sys; print(json.load(open(sys.argv[1]))["version"])' "$1"; }
# json_at <file> <key>...: the value at that key path (missing key -> "<missing>").
json_at() {
    "$(python_bin)" -c '
import json, sys
v = json.load(open(sys.argv[1]))
for k in sys.argv[2:]:
    v = v.get(k, {}) if isinstance(v, dict) else {}
print(v if isinstance(v, str) else "<missing>")' "$@"
}
xml_first() { sed -n "s:.*<$2>\([^<]*\)</$2>.*:\1:p" "$1" | head -1; }

check "bindings/node/package.json" "$(json_version "${REPO_ROOT}/bindings/node/package.json")"
check "bindings/node/trpc/package.json" "$(json_version "${REPO_ROOT}/bindings/node/trpc/package.json")"
# The router is released with the binding of the same version (peer dependency, exact).
check "bindings/node/trpc/package.json peerDependencies.sz-configtool" "$("$(python_bin)" -c 'import json,sys; print(json.load(open(sys.argv[1]))["peerDependencies"]["sz-configtool"])' "${REPO_ROOT}/bindings/node/trpc/package.json")"
# Lockfiles carry the version too (npm ci / npm pack read them): root
# "version" and packages[""].version; the router's lock also records the
# peer pin and the linked binding (packages[".."]).
node_lock="${REPO_ROOT}/bindings/node/package-lock.json"
trpc_lock="${REPO_ROOT}/bindings/node/trpc/package-lock.json"
check "bindings/node/package-lock.json version" "$(json_at "${node_lock}" version)"
check "bindings/node/package-lock.json packages[\"\"].version" "$(json_at "${node_lock}" packages "" version)"
check "bindings/node/trpc/package-lock.json version" "$(json_at "${trpc_lock}" version)"
check "bindings/node/trpc/package-lock.json packages[\"\"].version" "$(json_at "${trpc_lock}" packages "" version)"
check "bindings/node/trpc/package-lock.json packages[\"\"].peerDependencies.sz-configtool" "$(json_at "${trpc_lock}" packages "" peerDependencies sz-configtool)"
check "bindings/node/trpc/package-lock.json packages[\"..\"].version" "$(json_at "${trpc_lock}" packages .. version)"
# The project <version> follows <artifactId>sz-configtool</artifactId>.
check "bindings/java/pom.xml" "$(awk '/<artifactId>sz-configtool<\/artifactId>/ { f = 1; next } f && /<version>/ { gsub(/.*<version>|<\/version>.*/, ""); print; exit }' "${REPO_ROOT}/bindings/java/pom.xml")"
check "bindings/csharp/Directory.Build.props" "$(xml_first "${REPO_ROOT}/bindings/csharp/Directory.Build.props" Version)"

# Version spellings: Cargo/npm/Maven/NuGet take the version verbatim
# (4.4.0-1, 4.5.0-rc.1); the Python wheel uses its PEP 440 form (4.4.0.post1,
# 4.5.0rc1). pep440_version rejects every other form (dev/post/pre, bare
# labels, build metadata), so this also enforces the version policy.
if py_version="$(pep440_version "${VERSION}")"; then
    echo "ok: Python (PEP 440) version ${py_version}"
else
    echo "FAIL: workspace version ${VERSION} is not an accepted release version form"
    FAIL=1
fi

if [[ -n "${1:-}" ]]; then
    if [[ "$1" == "v${VERSION}" ]]; then echo "ok: tag $1"; else echo "FAIL: tag $1 != v${VERSION}"; FAIL=1; fi
fi

maturin_cfg="$(cfg tools.maturin)"
maturin_req="$(sed -n 's/^maturin==\([^ ]*\).*/\1/p' "${PACKAGING_DIR}/requirements-build.txt")"
if [[ "${maturin_cfg}" == "${maturin_req}" ]]; then
    echo "ok: maturin pin ${maturin_cfg}"
else
    echo "FAIL: maturin config.yaml ${maturin_cfg} != requirements-build.txt ${maturin_req}"
    FAIL=1
fi

[[ ${FAIL} -eq 0 ]] || die "version mismatch"
echo "PASS: versions consistent (${VERSION})"
