#!/usr/bin/env bash
# NuGet package with runtimes/<rid>/native/<lib> for every target
# (dotnet_rid in config.yaml), packed with SzRequireAllNatives=true:
#   ${SZ_DIST_DIR}/universal/out/Sz.ConfigTool.<version>.nupkg
#
# Requires ${SZ_DIST_DIR}/<t>/native/c for EVERY target in config.yaml.
#
# Usage: packaging/package-dotnet.sh [--allow-partial]
#   --allow-partial  local use only: pack just the RIDs that are staged
# shellcheck source=lib/common.sh
source "$(dirname "$0")/lib/common.sh"
# shellcheck source=lib/tools-env.sh
source "${PACKAGING_DIR}/lib/tools-env.sh"

REQUIRE_ALL=true
[[ "${1:-}" == --allow-partial ]] && REQUIRE_ALL=false

VERSION="$(workspace_version)"
natives="${SZ_DIST_DIR}/universal/dotnet-natives"
out="${SZ_DIST_DIR}/universal/out"
rm -rf "${natives}" && mkdir -p "${out}"

while IFS= read -r t; do
    os="$(tcfg "${t}" os)"
    rid="$(tcfg "${t}" dotnet_rid)"
    lib="$(shared_lib_name "${os}" SzConfigTool)"
    sub=lib
    [[ "${os}" == windows ]] && sub=bin
    src="$(target_stage "${t}")/native/c/${sub}/${lib}"
    if [[ ! -f "${src}" ]]; then
        [[ "${REQUIRE_ALL}" == false ]] && { log "WARNING: partial nupkg, no ${t}"; continue; }
        die "missing ${src} (download the ${t} build first)"
    fi
    mkdir -p "${natives}/${rid}"
    cp "${src}" "${natives}/${rid}/"
done < <(cfg_keys targets)

# Release properties: deterministic, source paths mapped, PDB embedded.
# --no-incremental: an earlier local/test build must not be reused.
props=(-c Release -p:Version="${VERSION}" -p:ContinuousIntegrationBuild=true -p:Deterministic=true
    -p:PathMap="$(native_path "${REPO_ROOT}")=$(cfg policy.remap_source_prefix)" -p:DebugType=embedded)
project="$(native_path "${REPO_ROOT}/bindings/csharp/src/Sz.ConfigTool")"
dotnet build "${project}" --no-incremental "${props[@]}"
dotnet pack "${project}" --no-build "${props[@]}" -o "$(native_path "${out}")" \
    -p:SzRequireAllNatives="${REQUIRE_ALL}" -p:SzNativesDir="$(native_path "${natives}")/"
nupkg="${out}/Sz.ConfigTool.${VERSION}.nupkg"
[[ -f "${nupkg}" ]] || die "dotnet pack did not produce ${nupkg}"
"$(python_bin)" - "${nupkg}" "${natives}" <<'PY'
import os, sys, zipfile
nupkg, natives = sys.argv[1:]
names = set(zipfile.ZipFile(nupkg).namelist())
want = [f"runtimes/{rid}/native/{f}" for rid in sorted(os.listdir(natives)) for f in os.listdir(os.path.join(natives, rid))]
missing = [w for w in want if w not in names]
if missing:
    sys.exit(f"{nupkg} lacks {missing}")
print(f"nupkg carries {len(want)} runtimes")
PY
log "wrote ${nupkg}"
