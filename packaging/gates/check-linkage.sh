#!/usr/bin/env bash
# Linkage gate for the staged natives of one target:
#   Linux  : SONAME libSzConfigTool.so; no RPATH/RUNPATH; NEEDED only glibc/libgcc_s.
#   macOS  : install names @rpath/... (libSzConfigTool: @rpath/libSzConfigTool.dylib);
#            deps only /usr/lib + /System; no LC_RPATH;
#            minos <= policy.macos_deployment_target.
#   Windows: imports only Windows system DLLs and api-ms-win-core-* API sets.
#            No MSVC runtime (vcruntime140*.dll) and no UCRT (api-ms-win-crt-*):
#            the shipped DLL/.node/.pyd use the static CRT (+crt-static,
#            lib/build-env.sh), so no VC++ Redistributable is required.
#            Every imported module (.dll AND .exe) is checked; a .node may
#            import node.exe (the napi host) and a .pyd python3.dll (abi3);
#            nothing else may.
# Applies to every shared library in native/ (C ABI, JNI, node, python).
#
# Usage: packaging/gates/check-linkage.sh <target>
# shellcheck source=../lib/common.sh
source "$(dirname "$0")/../lib/common.sh"

TARGET="${1:-}"
require_target "${TARGET}"
OS="$(tcfg "${TARGET}" os)"
NATIVE="$(target_stage "${TARGET}")/native"
[[ -d "${NATIVE}" ]] || die "no staged natives in ${NATIVE}; run build-native.sh ${TARGET}"

FAIL=0
fail() { echo "FAIL: $*"; FAIL=1; }

READELF="$(command -v readelf || command -v llvm-readelf || true)"
OTOOL="$(command -v otool || command -v llvm-otool || true)"
[[ "${OS}" != linux || -n "${READELF}" ]] || die "readelf (binutils) or llvm-readelf is required"
[[ "${OS}" != macos || -n "${OTOOL}" ]] || die "otool or llvm-otool is required"

check_elf() {
    local lib="$1" dyn
    dyn="$("${READELF}" -d "${lib}")"
    if [[ "$(basename "${lib}")" == libSzConfigTool.so ]]; then
        grep -q 'Library soname: \[libSzConfigTool.so\]' <<<"${dyn}" || fail "${lib}: SONAME is not libSzConfigTool.so"
    fi
    grep -qE '\((RPATH|RUNPATH)\)' <<<"${dyn}" && fail "${lib}: has RPATH/RUNPATH"
    local needed
    needed="$(sed -n 's/.*Shared library: \[\(.*\)\]/\1/p' <<<"${dyn}")"
    while IFS= read -r dep; do
        [[ -z "${dep}" ]] && continue
        case "${dep}" in
            libc.so.6 | libm.so.6 | libdl.so.2 | libpthread.so.0 | librt.so.1 | libutil.so.1 | libgcc_s.so.1 | ld-linux-*.so.*) ;;
            *) fail "${lib}: unexpected NEEDED ${dep}" ;;
        esac
    done <<<"${needed}"
    echo "ok: $(basename "${lib}") NEEDED: $(echo "${needed}" | tr '\n' ' ')"
}

check_macho() {
    local lib="$1" max minos id
    id="$("${OTOOL}" -D "${lib}" | tail -n +2 | tail -1)"
    [[ "${id}" == @rpath/* ]] || fail "${lib}: install name '${id}' is not @rpath-relative"
    if [[ "$(basename "${lib}")" == libSzConfigTool.dylib && "${id}" != "@rpath/libSzConfigTool.dylib" ]]; then
        fail "${lib}: install name is ${id}"
    fi
    while IFS= read -r dep; do
        [[ "${dep}" == "${id}" ]] && continue
        case "${dep}" in
            /usr/lib/* | /System/*) ;;
            *) fail "${lib}: unexpected dependency ${dep}" ;;
        esac
    done < <("${OTOOL}" -L "${lib}" | tail -n +2 | awk '{print $1}')
    "${OTOOL}" -l "${lib}" | grep -q LC_RPATH && fail "${lib}: has LC_RPATH"
    max="$(cfg policy.macos_deployment_target)"
    minos="$("${OTOOL}" -l "${lib}" | awk '/LC_BUILD_VERSION/ { f = 1 } f && $1 == "minos" { print $2; exit }')"
    [[ -n "${minos}" ]] || fail "${lib}: no LC_BUILD_VERSION minos"
    [[ "$(printf '%s\n%s\n' "${minos}" "${max}" | sort -V | tail -1)" == "${max}" ]] ||
        fail "${lib}: minos ${minos} > ${max}"
    echo "ok: $(basename "${lib}") id ${id} minos ${minos}"
}

check_pe() {
    local lib="$1" deps
    deps="$("$(msvc_tool dumpbin.exe)" //NOLOGO //DEPENDENTS "$(native_path "${lib}")" | tr -d '\r' |
        awk '/Image has the following dependencies/ { on = 1; next } on && /Summary/ { exit }
             on && tolower($1) ~ /\.(dll|exe)$/ { print $1 }')"
    while IFS= read -r dep; do
        [[ -z "${dep}" ]] && continue
        case "${dep,,}" in
            kernel32.dll | ntdll.dll | advapi32.dll | bcrypt.dll | ws2_32.dll | userenv.dll | \
                api-ms-win-core-*.dll | bcryptprimitives.dll) ;;
            node.exe)
                [[ "${lib}" == *.node ]] || fail "${lib}: imports node.exe but is not a .node addon"
                ;;
            python3.dll)
                [[ "${lib}" == *.pyd ]] || fail "${lib}: imports python3.dll but is not a Python extension"
                ;;
            *) fail "${lib}: unexpected dependency ${dep}" ;;
        esac
    done <<<"${deps}"
    echo "ok: $(basename "${lib}") imports: $(echo "${deps}" | tr '\n' ' ')"
}

while IFS= read -r -d '' lib; do
    case "${OS}" in
        linux) check_elf "${lib}" ;;
        macos) check_macho "${lib}" ;;
        windows) check_pe "${lib}" ;;
    esac
done < <(find "${NATIVE}" -type f \( -name '*.so' -o -name '*.dylib' -o -name '*.dll' -o -name '*.node' -o -name '*.pyd' \) -print0)

[[ ${FAIL} -eq 0 ]] || die "linkage gate failed for ${TARGET}"
echo "PASS: linkage for ${TARGET}"
