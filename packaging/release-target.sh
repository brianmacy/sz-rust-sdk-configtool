#!/usr/bin/env bash
# The per-target release pipeline, in one place. CI calls one stage per step;
# locally run `all`.
#
#   build    build-native.sh                                   (natives + SBOMs)
#   gates    exports, linkage, glibc ceiling (Linux), no build paths, C tests
#            — over everything staged so far (run after build AND after package)
#   package  C archive, Python wheel, Node tarball, C++ package (runs its ctest),
#            per-artifact SBOMs
#   smoke    each binding's test suite against the staged natives
#   all      build gates package gates smoke
#
# Usage: packaging/release-target.sh <target> <stage>...
# shellcheck source=lib/common.sh
source "$(dirname "$0")/lib/common.sh"

TARGET="${1:-}"
require_target "${TARGET}"
shift
[[ $# -gt 0 ]] || die "usage: release-target.sh <target> build|gates|package|smoke|all ..."
OS="$(tcfg "${TARGET}" os)"
STAGE="$(target_stage "${TARGET}")"
P="${PACKAGING_DIR}"

stage_build() { "${P}/build-native.sh" "${TARGET}"; }

stage_gates() {
    "${P}/gates/check-exports.sh" "${STAGE}"
    "${P}/gates/check-linkage.sh" "${TARGET}"
    if [[ "${OS}" == linux ]]; then
        "${P}/gates/check-glibc-ceiling.sh" "${STAGE}"
    fi
    "${P}/gates/check-no-build-paths.sh" "${STAGE}"
    "${P}/gates/run-c-tests.sh" "${TARGET}"
}

stage_package() {
    "${P}/package-c.sh" "${TARGET}"
    "${P}/package-python.sh" "${TARGET}"
    "${P}/package-node.sh" "${TARGET}"
    "${P}/package-cpp.sh" "${TARGET}"
    "${P}/package-sboms.sh" "${TARGET}"
}

stage_smoke() { "${P}/smoke-bindings.sh" "${TARGET}"; }

for stage in "$@"; do
    case "${stage}" in
        build) stage_build ;;
        gates) stage_gates ;;
        package) stage_package ;;
        smoke) stage_smoke ;;
        # One command per line: in an `a && b` list bash suspends `set -e`
        # inside the functions, so a failing gate would be ignored.
        all)
            stage_build
            stage_gates
            stage_package
            stage_gates
            stage_smoke
            ;;
        *) die "unknown stage '${stage}'" ;;
    esac
done
log "release-target ${TARGET}: $* OK"
