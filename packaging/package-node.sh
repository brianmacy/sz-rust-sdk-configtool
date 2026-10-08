#!/usr/bin/env bash
# Node binding artifacts.
#
#   package-node.sh <target>        per-target npm tarball holding dist/ and that
#                                   target's staged .node (the .node is no
#                                   separate release asset):
#                                     out/sz-configtool-node-<version>-<os>-<arch>.tgz
#   package-node.sh --trpc <target> the platform-independent tRPC router tarball, tested
#                                   against <target>'s staged .node (host must run it):
#                                     ${SZ_DIST_DIR}/universal/out/sz-configtool-trpc-<version>.tgz
#
# The TypeScript is compiled from bindings/node with the pinned Node/npm
# (`npm ci`, lockfile integrity-checked); packing happens in a staging copy so
# the source tree never receives a .node file.
# shellcheck source=lib/common.sh
source "$(dirname "$0")/lib/common.sh"
# shellcheck source=lib/tools-env.sh
source "${PACKAGING_DIR}/lib/tools-env.sh"

TRPC=0
if [[ "${1:-}" == --trpc ]]; then
    TRPC=1
    shift
fi
TARGET="${1:-}"
require_target "${TARGET}"
VERSION="$(workspace_version)"
STAGE="$(target_stage "${TARGET}")"
node_file="${STAGE}/native/node/sz-configtool.$(tcfg "${TARGET}" napi_tag).node"
[[ -f "${node_file}" ]] || die "missing ${node_file}; run build-native.sh ${TARGET}"
NODE_DIR="${REPO_ROOT}/bindings/node"

log "compiling the TypeScript (npm ci + build:ts)"
(cd "${NODE_DIR}" && npm ci --no-audit --no-fund && npm run build:ts)

if [[ ${TRPC} -eq 1 ]]; then
    out="${SZ_DIST_DIR}/universal/out"
    mkdir -p "${out}"
    log "building + testing the tRPC router against $(basename "${node_file}")"
    (cd "${NODE_DIR}/trpc" && npm ci --no-audit --no-fund && npm run build &&
        SZ_CONFIGTOOL_NATIVE_PATH="$(native_path "${node_file}")" npm test)
    tgz="$(cd "${NODE_DIR}/trpc" && npm pack --silent --pack-destination "$(native_path "${out}")" | tail -1)"
    # npm already names the tarball <name>-<version>.tgz; GNU mv fails on same-file moves.
    [[ "${tgz}" == "sz-configtool-trpc-${VERSION}.tgz" ]] || mv "${out}/${tgz}" "${out}/sz-configtool-trpc-${VERSION}.tgz"
    log "wrote ${out}/sz-configtool-trpc-${VERSION}.tgz"
    exit 0
fi

out="$(target_out "${TARGET}")"
pkg="${STAGE}/pkg-node/package"
rm -rf "${STAGE}/pkg-node" && mkdir -p "${pkg}" "${out}"
cp "${NODE_DIR}/package.json" "${NODE_DIR}/README.md" "${pkg}/"
cp -R "${NODE_DIR}/dist" "${pkg}/dist"
cp "${node_file}" "${pkg}/"
tgz="$(cd "${pkg}" && npm pack --silent --pack-destination "$(native_path "${out}")" | tail -1)"
final="sz-configtool-node-${VERSION}-$(tcfg "${TARGET}" os)-$(tcfg "${TARGET}" arch).tgz"
[[ "${tgz}" == "${final}" ]] || mv "${out}/${tgz}" "${out}/${final}"
log "wrote ${out}/${final}"
