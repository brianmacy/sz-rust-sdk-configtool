#!/usr/bin/env bash
# Self-test of the release-version policy (lib/common.sh): which workspace
# versions are accepted, and the PEP 440 spelling each maps to for the wheel.
# Accepted: X.Y.Z, X.Y.Z-N (release counter N on a fixed Senzing line ->
# X.Y.Z.postN), X.Y.Z-rc.N / -alpha.N / -beta.N (-> rcN / aN / bN).
# Everything else (dev/post/pre spellings, bare labels, build metadata,
# leading zeros) is rejected.
#
# Usage: packaging/gates/test-version-spellings.sh
# shellcheck source=../lib/common.sh
source "$(dirname "$0")/../lib/common.sh"

FAIL=0
expect_ok() {
    local got
    if got="$(pep440_version "$1" 2>/dev/null)" && [[ "${got}" == "$2" ]]; then
        echo "ok: $1 -> $2"
    else
        echo "FAIL: $1 -> '${got:-<rejected>}', expected $2"
        FAIL=1
    fi
}
expect_rejected() {
    local got
    if got="$(pep440_version "$1" 2>/dev/null)"; then
        echo "FAIL: $1 accepted as '${got}', expected rejection"
        FAIL=1
    else
        echo "ok: $1 rejected"
    fi
}

expect_ok 4.4.0 4.4.0
expect_ok 4.4.0-1 4.4.0.post1
expect_ok 4.4.0-27 4.4.0.post27
expect_ok 4.4.0-rc.1 4.4.0rc1
expect_ok 4.4.0-alpha.2 4.4.0a2
expect_ok 4.4.0-beta.3 4.4.0b3
expect_ok 10.20.30-4 10.20.30.post4

for bad in 4.4.0-0 4.4.0-01 4.4.0-dev.1 4.4.0-post.1 4.4.0-pre.1 4.4.0-preview.1 \
    4.4.0-rc 4.4.0-beta 4.4.0-rc1 4.4.0-c.1 4.4.0-a.1 4.4.0-1.1 4.4.0-1-rc.1 \
    4.4.0+abc 4.4.0-1+abc 4.4 4.4.0.1 04.4.0 v4.4.0 ""; do
    expect_rejected "${bad}"
done

[[ ${FAIL} -eq 0 ]] || die "version spelling policy"
echo "PASS: version spellings"
