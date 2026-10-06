#!/usr/bin/env bash
# Compare the dynamic export surface of every shipped shared library under
# <dir> with the committed baselines in ffi/expected-exports/:
#
#   SzConfigTool.exports        libSzConfigTool.{so,dylib} / SzConfigTool.dll  (SzConfigTool_* only)
#   szconfigtool_jni.exports    JNI library        (Java_* entry points only)
#   szconfigtool_node.exports   napi .node         (napi module registration only)
#   szconfigtool_py.exports     pyo3 extension     (PyInit_* only)
#
# A per-OS baseline <name>.<linux|macos|windows>.exports wins when present.
# Same approach as G2 dev/scripts/verify-export-surface.sh (nm / dumpbin vs a
# committed list; the lists are NOT fed to the linker), but stricter: every
# defined dynamic symbol counts (not only text), and a library without a
# baseline fails instead of being skipped.
#
# Usage: packaging/gates/check-exports.sh <dir>
# shellcheck source=../lib/common.sh
source "$(dirname "$0")/../lib/common.sh"

DIST="${1:?usage: check-exports.sh <dir>}"
BASELINES="${REPO_ROOT}/ffi/expected-exports"
[[ -d "${BASELINES}" ]] || die "baselines dir ${BASELINES} not found"

# baseline_name <file>: which baseline governs a library file.
baseline_name() {
    case "$(basename "$1")" in
        libSzConfigTool.so | libSzConfigTool.dylib | SzConfigTool.dll) echo SzConfigTool ;;
        libszconfigtool_jni.so | libszconfigtool_jni.dylib | szconfigtool_jni.dll) echo szconfigtool_jni ;;
        sz-configtool.*.node) echo szconfigtool_node ;;
        _native*.so | _native*.pyd) echo szconfigtool_py ;;
        *) echo "" ;;
    esac
}

file_format() {
    local magic
    magic="$(head -c 4 "$1" | od -An -tx1 | tr -d ' \n')"
    case "${magic}" in
        7f454c46) echo elf ;;
        cffaedfe | cefaedfe | cafebabe) echo macho ;;
        4d5a*) echo pe ;;
        *) echo unknown ;;
    esac
}

exports_of() {
    case "$(file_format "$1")" in
        elf)
            # Defined dynamic symbols, excluding the version-definition pseudo symbols.
            nm -D --defined-only "$1" | awk 'NF >= 3 && $2 != "A" { print $3 }' | sed 's/@.*//'
            ;;
        macho)
            nm -gU "$1" | awk 'NF >= 3 { sub(/^_/, "", $3); print $3 }'
            ;;
        pe)
            "$(msvc_tool dumpbin.exe)" //NOLOGO //EXPORTS "$(native_path "$1")" | tr -d '\r' |
                awk '/ordinal +hint +RVA +name/ { on = 1; next } on && NF >= 4 && $1 ~ /^[0-9]+$/ { print $4 } on && /Summary/ { exit }'
            ;;
        *) die "unrecognised binary format: $1" ;;
    esac
}

FAIL=0
CHECKED=0
while IFS= read -r -d '' lib; do
    name="$(baseline_name "${lib}")"
    [[ -n "${name}" ]] || continue
    case "$(file_format "${lib}")" in elf) os=linux ;; macho) os=macos ;; *) os=windows ;; esac
    baseline="${BASELINES}/${name}.${os}.exports"
    [[ -f "${baseline}" ]] || baseline="${BASELINES}/${name}.exports"
    if [[ ! -f "${baseline}" ]]; then
        echo "FAIL: ${lib#"${DIST}/"} has no baseline (${name}.exports)"
        FAIL=1
        continue
    fi
    # tr -d '\r': a CRLF checkout (core.autocrlf on Windows) must not change the list.
    expected="$(tr -d '\r' <"${baseline}" | grep -v '^#' | grep . | LC_ALL=C sort -u)"
    actual="$(exports_of "${lib}" | LC_ALL=C sort -u)"
    if [[ "${actual}" == "${expected}" ]]; then
        echo "PASS: ${lib#"${DIST}/"} — $(echo "${expected}" | wc -l | tr -d ' ') exports match $(basename "${baseline}")"
    else
        echo "FAIL: ${lib#"${DIST}/"} — export surface differs from $(basename "${baseline}") (< expected, > actual)"
        diff <(echo "${expected}") <(echo "${actual}") | head -40 || true
        FAIL=1
    fi
    CHECKED=$((CHECKED + 1))
done < <(find "${DIST}" -type f \( -name '*.so' -o -name '*.dylib' -o -name '*.dll' -o -name '*.node' -o -name '*.pyd' \) -print0)

[[ ${CHECKED} -gt 0 || ${FAIL} -ne 0 ]] || die "no shared libraries found under ${DIST}"
if [[ ${FAIL} -ne 0 ]]; then
    echo "FAIL: export surface mismatch — update ffi/expected-exports/ only if intentional" >&2
    exit 1
fi
echo "PASS: ${CHECKED} libraries match their export baselines"
