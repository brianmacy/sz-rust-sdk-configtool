#!/usr/bin/env bash
# Print the GitHub Release notes of a tag: the body of its CHANGELOG.md
# section, i.e. the lines after `## [<version>]` (or `## [<version>] - <date>`)
# up to the next `## ` heading or the link-reference footer, with leading and
# trailing blank lines and trailing `---` rules removed. <version> is the tag
# without its leading `v` (v4.4.0-1 -> 4.4.0-1).
#
# Fails when the section is missing, duplicated, has no content or exceeds
# GitHub's 125000-character release-body limit, so a tag whose CHANGELOG
# entry was forgotten cannot be published.
# Self-test: packaging/gates/test-release-notes.sh.
#
# Usage: packaging/release-notes.sh <tag> [<changelog>]   (default: <repo>/CHANGELOG.md)
# shellcheck source=lib/common.sh
source "$(dirname "$0")/lib/common.sh"

TAG="${1:-}"
[[ -n "${TAG}" ]] || die "usage: release-notes.sh <tag> [<changelog>]"
CHANGELOG="${2:-${REPO_ROOT}/CHANGELOG.md}"
[[ -f "${CHANGELOG}" ]] || die "no such file ${CHANGELOG}"
VERSION="${TAG#v}"
[[ -n "${VERSION}" ]] || die "empty version in tag '${TAG}'"

# Exit status: 0 = notes printed, 3 = no section, 4 = several sections,
# 5 = section without content.
status=0
notes="$(tr -d '\r' <"${CHANGELOG}" | awk -v ver="${VERSION}" '
    function is_heading(line) {
        # "## [<ver>]" exactly, optionally followed by " ..." (the date).
        head = "## [" ver "]"
        return line == head || index(line, head " ") == 1
    }
    is_heading($0) { found++; on = (found == 1); next }
    on && (/^## / || /^\[[^]]+\]: /) { on = 0 }
    on { body[++n] = $0 }
    END {
        if (found == 0) exit 3
        if (found > 1) exit 4
        first = 1; last = n
        while (first <= last && body[first] ~ /^[[:space:]]*$/) first++
        while (last >= first && (body[last] ~ /^[[:space:]]*$/ || body[last] ~ /^-{3,}[[:space:]]*$/)) last--
        if (first > last) exit 5
        for (i = first; i <= last; i++) print body[i]
    }
')" || status=$?
case "${status}" in
    0) ;;
    3) die "${CHANGELOG}: no '## [${VERSION}]' section for tag ${TAG}" ;;
    4) die "${CHANGELOG}: more than one '## [${VERSION}]' section" ;;
    5) die "${CHANGELOG}: the '## [${VERSION}]' section is empty" ;;
    *) die "release-notes.sh: awk failed (${status})" ;;
esac
# GitHub rejects a release body over 125000 characters.
[[ ${#notes} -le 125000 ]] || die "the '## [${VERSION}]' section has ${#notes} characters; GitHub allows 125000"
printf '%s\n' "${notes}"
