#!/usr/bin/env bash
# Write SHA256SUMS (sha256sum format, sorted by name) for every file in a
# release directory, then verify it.
#
# Usage: packaging/make-sums.sh <release-dir>
# shellcheck source=lib/common.sh
source "$(dirname "$0")/lib/common.sh"

DIR="${1:?usage: make-sums.sh <release-dir>}"
[[ -d "${DIR}" ]] || die "no such dir ${DIR}"
cd "${DIR}" || exit 1
rm -f SHA256SUMS
files=()
while IFS= read -r f; do files+=("${f}"); done < <(find . -maxdepth 1 -type f ! -name SHA256SUMS | sed 's|^\./||' | LC_ALL=C sort)
[[ ${#files[@]} -gt 0 ]] || die "no files in ${DIR}"
for f in "${files[@]}"; do
    printf '%s  %s\n' "$(sha256_file "${f}")" "${f}"
done >SHA256SUMS
if command -v sha256sum >/dev/null 2>&1; then sha256sum -c --quiet SHA256SUMS; else shasum -a 256 -c --quiet SHA256SUMS; fi
log "SHA256SUMS: ${#files[@]} files"
