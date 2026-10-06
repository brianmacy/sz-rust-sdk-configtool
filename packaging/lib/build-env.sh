# shellcheck shell=bash
# Release build environment for one target. Source after lib/common.sh and
# lib/tools-env.sh, then call `setup_build_env <target>`. Sets:
#   OS ARCH BUILDER RUST_TARGET BUILD_TARGET REMAP_PREFIX
#   CARGO_ENCODED_RUSTFLAGS   --remap-path-prefix for source, cargo home, rustup home
#                             (std sources, if rust-src is installed), target dir
#                             (REPLACES any caller RUSTFLAGS/CARGO_ENCODED_RUSTFLAGS;
#                             RUSTFLAGS is unset)
#   CARGO_PROFILE_RELEASE_*   strip (ELF/Mach-O) or PDB line tables (MSVC)
#   MACOSX_DEPLOYMENT_TARGET  (macOS)
#   CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER (Windows; explicit MSVC link.exe)
#   SZ_RUSTFLAGS_DYNAMIC_CRT  (Windows) CARGO_ENCODED_RUSTFLAGS without +crt-static,
#                             for the C ABI static archive (build-native.sh)
# and provides `cargo_cmd <build|rustc> ...` which uses cargo-zigbuild on zig targets.

# remap_pair <from> <to>: --remap-path-prefix in the form rustc sees paths
# (native Windows form under Git Bash).
remap_pair() {
    local from="$1"
    if [[ "$(host_os)" == windows ]]; then
        from="$(cygpath -w "$1")"
    fi
    printf -- '--remap-path-prefix=%s=%s' "${from}" "$2"
}

# shellcheck disable=SC2034 # OS/ARCH/RUST_TARGET/BUILD_TARGET are read by the sourcing script
setup_build_env() {
    local target="$1" sep=$'\x1f' cargo_home rustup_home
    OS="$(tcfg "${target}" os)"
    ARCH="$(tcfg "${target}" arch)"
    BUILDER="$(tcfg "${target}" builder)"
    RUST_TARGET="$(tcfg "${target}" rust_target)"
    BUILD_TARGET="$(tcfg "${target}" build_target)"
    REMAP_PREFIX="$(cfg policy.remap_source_prefix)"
    cargo_home="${CARGO_HOME:-${HOME}/.cargo}"
    rustup_home="${RUSTUP_HOME:-${HOME}/.rustup}"

    # Release builds are plain optimized builds: refuse sanitizer/instrumentation flags.
    case "${RUSTFLAGS:-} ${CARGO_ENCODED_RUSTFLAGS:-}" in
        *sanitizer* | *instrument-coverage*) die "release builds must not use sanitizer/coverage RUSTFLAGS" ;;
    esac
    # Intentional: a caller's RUSTFLAGS / CARGO_ENCODED_RUSTFLAGS are DISCARDED
    # (after the sanitizer check above). Release flags are exactly the set built
    # here, so a developer's local flags (target-cpu=native, -D warnings, ...)
    # can never leak into a shipped binary or change reproducibility.
    unset RUSTFLAGS
    CARGO_ENCODED_RUSTFLAGS="$(remap_pair "${REPO_ROOT}" "${REMAP_PREFIX}")${sep}$(remap_pair "${cargo_home}" /cargo)${sep}$(remap_pair "${rustup_home}" /rustup)${sep}$(remap_pair "${CARGO_TARGET_DIR}" /target)"
    export CARGO_ENCODED_RUSTFLAGS

    case "${OS}" in
        linux)
            export CARGO_PROFILE_RELEASE_STRIP=symbols
            export CARGO_PROFILE_RELEASE_BUILD_OVERRIDE_STRIP=false
            ;;
        macos)
            # No post-link strip(1) on macOS: with Xcode 27 it produced dylibs that
            # ld/dyld reject ("mis-aligned LINKEDIT string pool"), depending on
            # content. Let ld drop local symbols (-x) and debug notes (-S) instead.
            export CARGO_PROFILE_RELEASE_STRIP=none
            export CARGO_PROFILE_RELEASE_BUILD_OVERRIDE_STRIP=false
            CARGO_ENCODED_RUSTFLAGS+="${sep}-Clink-arg=-Wl,-x${sep}-Clink-arg=-Wl,-S"
            ;;
        windows)
            # Keep a useful PDB (line tables); the DLL itself carries no symbols.
            export CARGO_PROFILE_RELEASE_STRIP=none
            export CARGO_PROFILE_RELEASE_DEBUG=line-tables-only
            # Two steps: in `$(cygpath -w "$(msvc_tool ...)")` the inner failure is
            # masked by cygpath's exit status; a plain assignment propagates it.
            local msvc_link
            msvc_link="$(msvc_tool link.exe)"
            CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER="$(cygpath -w "${msvc_link}")"
            export CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER
            # Static MSVC runtime (/MT) for every shipped DLL / .node / .pyd, so
            # they import no vcruntime140*.dll / api-ms-win-crt-* (no VC++
            # Redistributable needed; gates/check-linkage.sh enforces it). The C
            # ABI static archive keeps the default dynamic CRT (/MD, what CMake and
            # most C/C++ consumers use): build-native.sh builds it with
            # SZ_RUSTFLAGS_DYNAMIC_CRT. Set here, not in .cargo/config.toml: cargo
            # ignores target.<triple>.rustflags whenever CARGO_ENCODED_RUSTFLAGS is set.
            SZ_RUSTFLAGS_DYNAMIC_CRT="${CARGO_ENCODED_RUSTFLAGS}"
            export SZ_RUSTFLAGS_DYNAMIC_CRT
            CARGO_ENCODED_RUSTFLAGS+="${sep}-Ctarget-feature=+crt-static"
            ;;
    esac
    if [[ "${OS}" == macos ]]; then
        MACOSX_DEPLOYMENT_TARGET="$(cfg policy.macos_deployment_target)"
        export MACOSX_DEPLOYMENT_TARGET
    fi
    if [[ "${BUILDER}" == zigbuild ]]; then
        command -v zig >/dev/null || die "zig not on PATH; run packaging/install-tools.sh ${target}"
        command -v cargo-zigbuild >/dev/null || die "cargo-zigbuild not on PATH; run packaging/install-tools.sh ${target}"
    fi
    export SOURCE_DATE_EPOCH="${SOURCE_DATE_EPOCH:-$(source_date_epoch)}"
}

# cargo_cmd <build|rustc> <args...>
cargo_cmd() {
    local sub="$1"
    shift
    if [[ "${BUILDER}" == zigbuild ]]; then
        [[ "${sub}" == build ]] && sub=zigbuild
        cargo-zigbuild "${sub}" "$@"
    else
        cargo "${sub}" "$@"
    fi
}
