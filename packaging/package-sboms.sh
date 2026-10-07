#!/usr/bin/env bash
# Publish the CycloneDX SBOMs of one target (written by build-native.sh to
# sbom/) as release assets, one per shipped artifact family, named so the
# four targets never collide in the flat GitHub Release:
#
#   out/sz-configtool-<v>-<os>-<arch>.cdx.json         C ABI (C archive, C++ package, NuGet runtime)
#   out/sz-configtool-jni-<v>-<os>-<arch>.cdx.json     JNI library (bundled in the jar)
#   out/sz-configtool-node-<v>-<os>-<arch>.cdx.json    napi .node (npm tarball)
#   out/sz-configtool-python-<v>-<os>-<arch>.cdx.json  pyo3 extension (wheel)
#
# release.yml copies out/ into the release, so each SBOM is listed in
# SHA256SUMS and covered by the build-provenance attestation.
#
# Usage: packaging/package-sboms.sh <target>   (after build-native.sh <target>)
# shellcheck source=lib/common.sh
source "$(dirname "$0")/lib/common.sh"

TARGET="${1:-}"
require_target "${TARGET}"
VERSION="$(workspace_version)"
STAGE="$(target_stage "${TARGET}")"
OUT="$(target_out "${TARGET}")"
suffix="${VERSION}-$(tcfg "${TARGET}" os)-$(tcfg "${TARGET}" arch).cdx.json"
mkdir -p "${OUT}"

for crate in c jni node python; do
    src="${STAGE}/sbom/sz-configtool-${crate}.cdx.json"
    [[ -f "${src}" ]] || die "missing ${src}; run build-native.sh ${TARGET}"
    if [[ "${crate}" == c ]]; then
        dst="${OUT}/sz-configtool-${suffix}"
    else
        dst="${OUT}/sz-configtool-${crate}-${suffix}"
    fi
    cp "${src}" "${dst}"
    log "wrote ${dst}"
done
