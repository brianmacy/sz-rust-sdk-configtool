#!/usr/bin/env bash
# Multi-platform Java jar: bundles every target's staged JNI library under
# natives/<os>-<arch>/ (java_platform in config.yaml) and runs the Java test
# suite against <host-target>'s library while packaging.
#   ${SZ_DIST_DIR}/universal/out/sz-configtool-<version>.jar
#
# Requires ${SZ_DIST_DIR}/<t>/native/jni for EVERY target in config.yaml.
# bindings/java is built from a staging copy so its natives/ tree stays empty.
#
# Usage: packaging/package-java.sh [--allow-partial] <host-target>
#   --allow-partial  local use only: bundle just the targets that are staged
# shellcheck source=lib/common.sh
source "$(dirname "$0")/lib/common.sh"
# shellcheck source=lib/tools-env.sh
source "${PACKAGING_DIR}/lib/tools-env.sh"

PARTIAL=0
[[ "${1:-}" == --allow-partial ]] && { PARTIAL=1; shift; }
HOST_TARGET="${1:-}"
require_target "${HOST_TARGET}"
VERSION="$(workspace_version)"
work="${SZ_DIST_DIR}/universal/java-build"
out="${SZ_DIST_DIR}/universal/out"
rm -rf "${work}" && mkdir -p "${work}/natives" "${out}"
cp -R "${REPO_ROOT}/bindings/java/pom.xml" "${REPO_ROOT}/bindings/java/src" "${work}/"

expected=()
while IFS= read -r t; do
    platform="$(tcfg "${t}" java_platform)"
    lib="$(shared_lib_name "$(tcfg "${t}" os)" szconfigtool_jni)"
    src="$(target_stage "${t}")/native/jni/${lib}"
    if [[ ! -f "${src}" ]]; then
        [[ ${PARTIAL} -eq 1 ]] && { log "WARNING: partial jar, no ${t}"; continue; }
        die "missing ${src} (download the ${t} build first)"
    fi
    mkdir -p "${work}/natives/${platform}"
    cp "${src}" "${work}/natives/${platform}/"
    expected+=("natives/${platform}/${lib}")
done < <(cfg_keys targets)

(cd "${work}" && mvn -B -ntp package \
    -Dworkspace.dir="$(native_path "${REPO_ROOT}")" \
    -Dnative.lib.dir="$(native_path "$(target_stage "${HOST_TARGET}")/native/jni")")

jar="${work}/target/sz-configtool-${VERSION}.jar"
[[ -f "${jar}" ]] || die "maven did not produce ${jar} (pom version must equal ${VERSION})"
"$(python_bin)" - "${jar}" "${expected[@]}" <<'PY'
import sys, zipfile
jar, *expected = sys.argv[1:]
names = set(zipfile.ZipFile(jar).namelist())
missing = [e for e in expected if e not in names]
if missing:
    sys.exit(f"{jar} lacks bundled natives: {missing}")
print(f"jar bundles {len(expected)} natives")
PY
cp "${jar}" "${out}/"
log "wrote ${out}/$(basename "${jar}")"
