#!/usr/bin/env bash
# Self-test of packaging/release-notes.sh against a temporary CHANGELOG
# fixture: a `-N` version, the newest and the oldest section, a heading
# without a date, a missing version, an empty section, a duplicated section,
# a version that is only a prefix of another one, an oversized section, and
# the real CHANGELOG.md (its newest released section must yield notes).
#
# Usage: packaging/gates/test-release-notes.sh
# shellcheck source=../lib/common.sh
source "$(dirname "$0")/../lib/common.sh"

NOTES="${PACKAGING_DIR}/release-notes.sh"
TMP="$(mktemp -d)"
trap 'rm -rf "${TMP}"' EXIT
FIXTURE="${TMP}/CHANGELOG.md"
cat >"${FIXTURE}" <<'MD'
# Changelog

Intro text that is not part of any release.

## [Unreleased]

## [4.4.0-10] - 2026-11-01

Ten: must not leak into 4.4.0-1.

## [4.4.0-2] - 2026-10-08

## [4.4.0-1] - 2026-10-06

First Senzing-aligned release.

### Added

- Linux-only Python wheels.

## [0.10.0]
- no date on this heading

## [0.9.0] - 2026-08-31

## [0.9.0] - 2026-08-30

Duplicate heading.

## [0.1.0] - 2025-01-01

Oldest entry.

---

[0.1.0]: https://example.invalid/releases/tag/v0.1.0
MD

FAIL=0
# expect_notes <tag> <expected-output>
expect_notes() {
    local got
    if got="$("${NOTES}" "$1" "${FIXTURE}" 2>"${TMP}/err")" && [[ "${got}" == "$2" ]]; then
        echo "ok: $1"
    else
        echo "FAIL: $1 -> '${got:-}' ($(cat "${TMP}/err")), expected '$2'"
        FAIL=1
    fi
}
# expect_fail <tag> <error-substring>
expect_fail() {
    local got
    if got="$("${NOTES}" "$1" "${FIXTURE}" 2>"${TMP}/err")"; then
        echo "FAIL: $1 accepted ('${got}'), expected failure"
        FAIL=1
    elif grep -qF "$2" "${TMP}/err"; then
        echo "ok: $1 rejected ($2)"
    else
        echo "FAIL: $1 rejected with '$(cat "${TMP}/err")', expected '$2'"
        FAIL=1
    fi
}

expect_notes v4.4.0-1 "First Senzing-aligned release.

### Added

- Linux-only Python wheels."
expect_notes 4.4.0-1 "First Senzing-aligned release.

### Added

- Linux-only Python wheels."
expect_notes v4.4.0-10 "Ten: must not leak into 4.4.0-1."
expect_notes v0.10.0 "- no date on this heading"
expect_notes v0.1.0 "Oldest entry."
expect_fail v4.4.0-3 "no '## [4.4.0-3]' section"
expect_fail v4.4.0 "no '## [4.4.0]' section"
expect_fail v4.4.0-2 "section is empty"
expect_fail vUnreleased "section is empty"
expect_fail v0.9.0 "more than one"
expect_fail "" "usage"

# Over GitHub's 125000-character release-body limit.
{
    echo "## [9.9.9-1] - 2030-01-01"
    for _ in $(seq 1 1300); do printf '%0100d\n' 0; done
} >>"${FIXTURE}"
expect_fail v9.9.9-1 "GitHub allows 125000"

# The real CHANGELOG: the newest released section must produce notes.
latest="$(sed -n 's/^## \[\([0-9][^]]*\)\].*/\1/p' "${REPO_ROOT}/CHANGELOG.md" | head -1)"
if [[ -n "${latest}" ]] && "${NOTES}" "v${latest}" >/dev/null 2>"${TMP}/err"; then
    echo "ok: CHANGELOG.md v${latest}"
else
    echo "FAIL: CHANGELOG.md v${latest:-<none>}: $(cat "${TMP}/err")"
    FAIL=1
fi

[[ ${FAIL} -eq 0 ]] || die "release notes"
echo "PASS: release notes"
