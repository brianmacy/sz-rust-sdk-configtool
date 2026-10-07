#!/usr/bin/env bash
# Fail if any file under the given paths embeds an absolute build-host path
# (the source tree, the cargo home, the rustup home, the cargo target dir or
# $HOME), i.e. if
# --remap-path-prefix (lib/build-env.sh) missed something. Zip-based outputs
# (.whl, .jar, .nupkg) are inspected after extraction; gzip tarballs too.
# Windows .pdb files are exempt: a PDB is a debug artifact that records the
# object-file paths of the link by design (the DLL itself references it only
# by file name, rustc's /PDBALTPATH:%_PDB%).
#
# Usage: packaging/gates/check-no-build-paths.sh <path>...
# shellcheck source=../lib/common.sh
source "$(dirname "$0")/../lib/common.sh"
[[ $# -gt 0 ]] || die "usage: check-no-build-paths.sh <path>..."

needles=("${REPO_ROOT}" "${CARGO_HOME:-${HOME}/.cargo}" "${RUSTUP_HOME:-${HOME}/.rustup}" "${CARGO_TARGET_DIR}" "${HOME}")
if [[ "$(host_os)" == windows ]]; then
    for n in "${needles[@]}"; do
        w="$(cygpath -w "${n}")"
        # D:\a\..., D:/a/..., and the JSON-escaped D:\\a\\... (SBOMs, JSON metadata).
        needles+=("${w}" "$(cygpath -m "${n}")" "${w//\\/\\\\}")
    done
fi
needle_file="$(mktemp)"
work="$(mktemp -d)"
trap 'rm -rf "${needle_file}" "${work}"' EXIT
printf '%s\n' "${needles[@]}" | sort -u >"${needle_file}"

expand() {
    local f="$1" dest
    dest="${work}/$(basename "${f}").d"
    mkdir -p "${dest}"
    case "${f}" in
        *.whl | *.jar | *.nupkg | *.zip) "$(python_bin)" -m zipfile -e "${f}" "${dest}" ;;
        *.tar.gz | *.tgz)
            "$(python_bin)" -c 'import sys, tarfile; tarfile.open(sys.argv[1]).extractall(sys.argv[2], filter="data")' \
                "$(native_path "${f}")" "$(native_path "${dest}")"
            ;;
    esac
    echo "${dest}"
}

FAIL=0
scan() {
    local hits
    hits="$(grep -r -a -l -F --exclude='*.pdb' -f "${needle_file}" "$1" 2>/dev/null || true)"
    if [[ -n "${hits}" ]]; then
        while IFS= read -r h; do
            echo "FAIL: build path in ${h#"${work}/"}: $(grep -a -o -F -f "${needle_file}" "${h}" | sort -u | head -3 | tr '\n' ' ')"
        done <<<"${hits}"
        FAIL=1
    fi
}

for p in "$@"; do
    [[ -e "${p}" ]] || die "no such path: ${p}"
    scan "${p}"
    while IFS= read -r -d '' a; do
        scan "$(expand "${a}")"
    done < <(find "${p}" -type f \( -name '*.whl' -o -name '*.jar' -o -name '*.nupkg' -o -name '*.zip' -o -name '*.tar.gz' -o -name '*.tgz' \) -print0)
done

[[ ${FAIL} -eq 0 ]] || die "absolute build paths found (check --remap-path-prefix in packaging/lib/build-env.sh)"
echo "PASS: no build-host paths in $*"
