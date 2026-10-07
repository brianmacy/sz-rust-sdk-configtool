# shellcheck shell=bash
# Puts the tools installed by packaging/install-tools.sh on PATH (and sets
# JAVA_HOME when the pinned JDK is installed). Sourced by every packaging
# script after lib/common.sh; safe to source more than once.

SZ_TOOLS_DIR="${SZ_TOOLS_DIR:-${CARGO_TARGET_DIR}/sz-tools}"
export SZ_TOOLS_DIR

tools_venv_bin() {
    if [[ -d "${SZ_TOOLS_DIR}/venv/Scripts" ]]; then
        echo "${SZ_TOOLS_DIR}/venv/Scripts"
    else
        echo "${SZ_TOOLS_DIR}/venv/bin"
    fi
}

tools_venv_python() {
    local bin
    bin="$(tools_venv_bin)"
    if [[ -x "${bin}/python.exe" ]]; then echo "${bin}/python.exe"; else echo "${bin}/python"; fi
}

tools_jdk_dir() { echo "${SZ_TOOLS_DIR}/jdk-$(cfg tools.jdk.version)"; }

# macOS JDK bundles keep the home under Contents/Home.
tools_java_home() {
    local dir
    dir="$(tools_jdk_dir)"
    if [[ -d "${dir}/Contents/Home" ]]; then echo "${dir}/Contents/Home"; else echo "${dir}"; fi
}

# One PATH entry per line (also the $GITHUB_PATH format); only existing dirs.
tools_path_entries() {
    local node_dir
    node_dir="${SZ_TOOLS_DIR}/node-$(cfg tools.node.version)"
    local candidates=(
        "${SZ_TOOLS_DIR}/zig-$(cfg tools.zig.version)"
        "${SZ_TOOLS_DIR}/cargo/bin"
        "$(tools_venv_bin)"
        "$(tools_java_home)/bin"
        "${SZ_TOOLS_DIR}/maven-$(cfg tools.maven.version)/bin"
        "${node_dir}/bin"
        "${node_dir}"
    )
    local dir
    for dir in "${candidates[@]}"; do
        # Windows Node keeps node.exe at the archive root; elsewhere it is bin/.
        [[ "${dir}" == "${node_dir}" && -d "${node_dir}/bin" ]] && continue
        [[ -d "${dir}" ]] && echo "${dir}"
    done
    return 0
}

# Under Git Bash SZ_TOOLS_DIR derives from CARGO_TARGET_DIR in C:/... form
# (lib/common.sh); the drive colon would split a PATH entry, so PATH gets the
# POSIX form (/c/...). Other uses keep the mixed form native tools accept.
while IFS= read -r _entry; do
    if command -v cygpath >/dev/null 2>&1; then
        _entry="$(cygpath -u "${_entry}")"
    fi
    case ":${PATH}:" in
        *":${_entry}:"*) ;;
        *) PATH="${_entry}:${PATH}" ;;
    esac
done < <(tools_path_entries)
unset _entry
export PATH
if [[ -d "$(tools_java_home)/bin" ]]; then
    JAVA_HOME="$(tools_java_home)"
    export JAVA_HOME
fi
