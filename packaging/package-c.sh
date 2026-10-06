#!/usr/bin/env bash
# Archive the staged C ABI of one target:
#   ${SZ_DIST_DIR}/<target>/out/sz-configtool-<version>-<os>-<arch>.{tar.gz|zip}
#
# Usage: packaging/package-c.sh <target>   (after build-native.sh <target>)
# shellcheck source=lib/common.sh
source "$(dirname "$0")/lib/common.sh"

TARGET="${1:-}"
require_target "${TARGET}"
VERSION="$(workspace_version)"
STAGE="$(target_stage "${TARGET}")"
OUT="$(target_out "${TARGET}")"
name="sz-configtool-${VERSION}-$(tcfg "${TARGET}" os)-$(tcfg "${TARGET}" arch)"
root="${STAGE}/pkg-c/${name}"
[[ -d "${STAGE}/native/c" ]] || die "no staged C ABI; run build-native.sh ${TARGET}"

rm -rf "${STAGE}/pkg-c" && mkdir -p "${root}/sbom" "${OUT}"
cp -R "${STAGE}/native/c/." "${root}/"
mv "${root}/native-static-libs.txt" "${root}/lib/"
cp "${STAGE}/sbom/sz-configtool-c.cdx.json" "${root}/sbom/"
cp "${REPO_ROOT}/LICENSE" "${root}/"
printf '%s\n' "${VERSION}" >"${root}/VERSION"
sed -e "s/@VERSION@/${VERSION}/g" -e "s/@TARGET@/${TARGET}/g" \
    "${PACKAGING_DIR}/templates/c-archive-README.md" >"${root}/README.md"

archive="${OUT}/${name}.$(tcfg "${TARGET}" archive)"
SOURCE_DATE_EPOCH="${SOURCE_DATE_EPOCH:-$(source_date_epoch)}" \
    "$(python_bin)" "${PACKAGING_DIR}/lib/archive.py" "${root}" "${archive}"
log "wrote ${archive}"
