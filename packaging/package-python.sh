#!/usr/bin/env bash
# Build the Python wheel (abi3-py310) of one Linux target with maturin:
#   `maturin build --zig --compatibility manylinux_2_34` (glibc floor).
# Only targets with config.yaml `python_wheel: "true"` (the Linux legs) ship a
# wheel: Senzing's Python SDK is Linux only, so macOS / Windows are refused.
# Output: ${SZ_DIST_DIR}/<target>/out/sz_configtool-<pep440-version>-cp310-abi3-<platform>.whl
# (<pep440-version>: lib/common.sh pep440_version, e.g. 4.4.0.post1 for 4.4.0-1, 4.5.0rc1 for 4.5.0-rc.1)
# No SBOM is embedded (bindings/python/pyproject.toml `[tool.maturin.sbom] rust = false`);
# the crate's CycloneDX SBOM ships separately (build-native.sh). check-no-build-paths.sh
# scans the wheel.
# The extension inside the wheel is also extracted to native/python/ so the
# export / glibc / build-path gates can inspect it.
#
# Usage: packaging/package-python.sh <target>
# shellcheck source=lib/common.sh
source "$(dirname "$0")/lib/common.sh"
# shellcheck source=lib/tools-env.sh
source "${PACKAGING_DIR}/lib/tools-env.sh"
# shellcheck source=lib/build-env.sh
source "${PACKAGING_DIR}/lib/build-env.sh"

TARGET="${1:-}"
require_target "${TARGET}"
target_has_python "${TARGET}" ||
    die "no Python wheel for ${TARGET}: wheels are Linux only, like Senzing's Python SDK (config.yaml python_wheel)"
setup_build_env "${TARGET}"
VERSION="$(workspace_version)"
STAGE="$(target_stage "${TARGET}")"
OUT="$(target_out "${TARGET}")"
wheels="${STAGE}/wheels"
rm -rf "${wheels}" "${STAGE}/native/python" && mkdir -p "${wheels}" "${OUT}" "${STAGE}/native/python"

args=(build --release --manifest-path "$(native_path "${REPO_ROOT}/bindings/python/Cargo.toml")"
    --target "${RUST_TARGET}" --out "$(native_path "${wheels}")")
if [[ "${BUILDER}" == zigbuild ]]; then
    args+=(--zig --compatibility "$(tcfg "${TARGET}" python_compat)")
fi
log "maturin ${args[*]}"
(cd "${REPO_ROOT}/bindings/python" && "$(tools_venv_python)" -m maturin "${args[@]}")

# maturin names the wheel with the PEP 440 form of the Cargo version
# (4.4.0-1 -> sz_configtool-4.4.0.post1-...).
PY_VERSION="$(pep440_version "${VERSION}")"
shopt -s nullglob
built=("${wheels}"/sz_configtool-"${PY_VERSION}"-*.whl)
[[ ${#built[@]} -eq 1 ]] || die "expected one wheel for version ${PY_VERSION} (${VERSION}) in ${wheels}, found ${#built[@]}"
cp "${built[0]}" "${OUT}/"
"$(python_bin)" - "${built[0]}" "${STAGE}/native/python" <<'PY'
import sys, zipfile
whl, dest = sys.argv[1:]
with zipfile.ZipFile(whl) as zf:
    names = [n for n in zf.namelist() if n.split("/")[-1].startswith("_native") and n.endswith((".so", ".pyd"))]
    if len(names) != 1:
        sys.exit(f"expected one native extension in {whl}, found {names}")
    open(f"{dest}/{names[0].split('/')[-1]}", "wb").write(zf.read(names[0]))
PY
log "wrote ${OUT}/$(basename "${built[0]}")"
