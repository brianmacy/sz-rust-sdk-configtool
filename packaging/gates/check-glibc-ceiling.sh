#!/usr/bin/env bash
# Verify that every shipped Linux shared object (.so, .node, Python extension)
# references no glibc symbol version newer than the ceiling.
#
# Port of G2 dev/scripts/verify-glibc-ceiling.sh (same method: `objdump -T`
# dynamic symbol table, newest GLIBC_x.y via `sort -V`). Differences: searches
# the whole directory tree (our layout has no lib/ + bin/ split), recognises
# ELF files by magic instead of name, and fails when it finds no ELF at all.
# Static archives (.a) have no dynamic symbol table and are skipped.
#
# Usage: packaging/gates/check-glibc-ceiling.sh <dir> [max-glibc-version]
#        (default max: policy.glibc_ceiling in packaging/config.yaml)
# shellcheck source=../lib/common.sh
source "$(dirname "$0")/../lib/common.sh"

DIST="${1:?usage: check-glibc-ceiling.sh <dir> [max-glibc-version]}"
MAX="${2:-$(cfg policy.glibc_ceiling)}"
[[ -d "${DIST}" ]] || die "dir '${DIST}' not found"

OBJDUMP="$(command -v objdump || command -v llvm-objdump || true)"
[[ -n "${OBJDUMP}" ]] || die "objdump (binutils) or llvm-objdump is required"

is_elf() { [[ "$(head -c 4 "$1" | od -An -c | tr -d ' ')" == '177ELF' ]]; }

TARGETS=()
while IFS= read -r -d '' f; do
    is_elf "${f}" && TARGETS+=("${f}")
done < <(find "${DIST}" -type f \( -name '*.so' -o -name '*.so.*' -o -name '*.node' \) -print0 | sort -z)
[[ ${#TARGETS[@]} -gt 0 ]] || die "no ELF shared objects under ${DIST}"

FAIL=0
printf '%-70s %s\n' "BINARY" "MAX GLIBC"
for bin in "${TARGETS[@]}"; do
    # Fail closed: an objdump error (or output without a dynamic symbol table)
    # must never be reported as "(none)" and pass.
    dyn=$("${OBJDUMP}" -T "${bin}") || die "${OBJDUMP} -T failed on ${bin}"
    grep -q 'DYNAMIC SYMBOL TABLE' <<<"${dyn}" || die "no dynamic symbol table in ${OBJDUMP} -T output for ${bin}"
    found=$(grep -oE 'GLIBC_[0-9]+\.[0-9]+(\.[0-9]+)?' <<<"${dyn}" | sort -uV | tail -1 || true)
    if [[ -z "${found}" ]]; then
        printf '%-70s %s\n' "${bin#"${DIST}/"}" "(none)"
        continue
    fi
    larger=$(printf '%s\n%s\n' "${found#GLIBC_}" "${MAX}" | sort -V | tail -1)
    if [[ "${larger}" == "${MAX}" ]]; then
        printf '%-70s %s\n' "${bin#"${DIST}/"}" "${found} <= GLIBC_${MAX}"
    else
        printf '%-70s %s  <-- FAIL\n' "${bin#"${DIST}/"}" "${found} > GLIBC_${MAX}"
        FAIL=1
    fi
done

if [[ ${FAIL} -ne 0 ]]; then
    echo "FAILED: one or more binaries reference glibc symbols newer than GLIBC_${MAX}." >&2
    exit 1
fi
echo "OK: all ${#TARGETS[@]} ELF objects stay at or below GLIBC_${MAX}."
