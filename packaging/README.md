# Release packaging

Everything a release does is a script in this directory; the workflows
(`.github/workflows/release.yml`, `ci.yml`) only call them. Releases go to
**GitHub Releases only** (no PyPI / Maven Central / NuGet / npm / crates.io).
The version is the Cargo workspace version (`[workspace.package] version`);
the tag must be `v<version>` (first release: `4.4.0-1`, tag `v4.4.0-1`).

## Versions

`X.Y.Z-N`, mirroring Senzing's package versions (`4.4.2-26272`): `X.Y` is the
Senzing line whose config template the release is tested against, `Z` and the
release counter `-N` (N >= 1) are this project's; the Rust API is additive
only within `4.x`. Accepted forms, and how each ecosystem spells them
(`lib/common.sh` `pep440_version`; self-test `gates/test-version-spellings.sh`):

| Workspace version | Cargo / npm / Maven / NuGet / archives | Python wheel (PEP 440) |
|---|---|---|
| `4.4.0` | `4.4.0` | `4.4.0` |
| `4.4.0-1` | `4.4.0-1` | `4.4.0.post1` |
| `4.5.0-rc.1` / `-alpha.N` / `-beta.N` | verbatim | `4.5.0rc1` / `aN` / `bN` |

Every other form (`-dev.N`, `-post.N`, `-pre.N`, bare `-rc`, build metadata,
leading zeros) fails `gates/check-versions.sh`: PEP 440 and Maven order
dev/post/pre spellings differently from SemVer (some above the release).

**Ordering caveat.** SemVer ecosystems (Cargo, npm, NuGet) treat `4.4.0-1` as
a prerelease that sorts BEFORE `4.4.0`; PEP 440 (`4.4.0.post1`) and Maven
sort it AFTER. Assets go to GitHub Releases only and are consumed with exact
pins, so this matters only to range resolvers (npm `^4.4.0` does not match
`4.4.0-1`): pin exact versions.

The macOS dylib `current_version` takes only the numeric `X.Y.Z` (ld64
rejects a suffix; `ffi/build.rs`), and the C++ CMake package version is the
numeric `X.Y.Z` (`find_package(szconfigtool 4.4.0)`) while `LibraryVersion()`
returns the full string.

## Supported platforms

Linux x86_64 / arm64 with glibc >= 2.34 (RHEL 9+, Amazon Linux 2023, Ubuntu
22.04+, Debian 12+), macOS 15+ arm64, Windows x64. The glibc floor matches
G2's anylinux builds; **RHEL 8 (glibc 2.28) is not supported** by these
binaries: RHEL 8 users build from source with the Rust library.

## Matrix

| Target | Runner | Build | Floor |
|---|---|---|---|
| `linux-x64` | `ubuntu-24.04` | `cargo-zigbuild --target x86_64-unknown-linux-gnu.2.34` (generic x86-64, no `target-cpu`) | glibc 2.34 |
| `linux-arm64` | `ubuntu-24.04-arm` (native) | `cargo-zigbuild --target aarch64-unknown-linux-gnu.2.34` | glibc 2.34 |
| `macos-arm64` | `macos-26` | plain cargo, `MACOSX_DEPLOYMENT_TARGET=15.0` | macOS 15.0 |
| `windows-x64` | `windows-2025` | plain cargo, `x86_64-pc-windows-msvc`, explicit MSVC `link.exe` | Windows x64 |

No musl. Targets, tool pins and policy live in [`config.yaml`](config.yaml).

## Release assets (per tag)

| Asset | From |
|---|---|
| `sz-configtool-<v>-<os>-<arch>.tar.gz` (`.zip` on Windows) | C ABI: `include/libSzConfigTool.h`, shared lib (`.so` SONAME `libSzConfigTool.so` / `.dylib` id `@rpath/libSzConfigTool.dylib` / `SzConfigTool.dll` + `.pdb` + import lib `SzConfigTool.lib`), static lib (`libSzConfigTool.a` / `SzConfigTool_static.lib`), `lib/native-static-libs.txt`, CycloneDX SBOM, `LICENSE`, `README.md`, `VERSION` |
| `sz-configtool-cpp-<v>-<os>-<arch>.tar.gz`/`.zip` | C++ header-only binding + `find_package(szconfigtool)` prefix (`cmake --install` of `bindings/cpp`) |
| `sz_configtool-<pep440-v>-cp310-abi3-<platform>.whl` | Python distribution `sz-configtool`, import `sz_configtool` (maturin, abi3-py310): `manylinux_2_34_{x86_64,aarch64}`, `macosx_15_0_arm64`, `win_amd64`. `<pep440-v>` is the PEP 440 spelling maturin gives the version (`4.4.0-1` -> `4.4.0.post1`, `4.5.0-rc.1` -> `4.5.0rc1`; see Versions). No embedded SBOM (`[tool.maturin.sbom] rust = false`); see the SBOM row |
| `sz-configtool-node-<v>-<os>-<arch>.tgz`, `sz-configtool.<napi-tag>.node` | Node (napi-rs): npm tarball with `dist/` + that platform's `.node`, and the bare `.node` |
| `sz-configtool-trpc-<v>.tgz` | tRPC router (platform independent) |
| `sz-configtool-<v>.jar` | Java, natives bundled under `natives/<os>-<arch>/` for all four targets |
| `Sz.ConfigTool.<v>.nupkg` | .NET, `runtimes/{linux-x64,linux-arm64,osx-arm64,win-x64}/native/` |
| `sz-configtool-<v>-<os>-<arch>.cdx.json` | CycloneDX 1.5 SBOM of the C ABI (C archive, C++ package, NuGet runtime of that platform) |
| `sz-configtool-{jni,node,python}-<v>-<os>-<arch>.cdx.json` | CycloneDX SBOMs of the JNI library (jar), the `.node` (npm tarball) and the pyo3 extension (wheel) of that platform (`package-sboms.sh`) |
| `SHA256SUMS` | sha256 of every asset above (SBOMs included) |
| `sz-configtool-<v>.intoto.jsonl` | Sigstore bundle of the GitHub build-provenance attestation over `SHA256SUMS` subjects (every asset above, SBOMs included) |


## Pipeline

`release-target.sh <target> <stage>` defines the per-target order; CI runs one
stage per step.

| Stage | Scripts |
|---|---|
| tools | `install-tools.sh <target>` — pinned Rust (rustup), zig + JDK + Node (sha256), Maven (sha512), cargo-zigbuild/cargo-cyclonedx (`cargo install --locked`), maturin/pytest (`pip --require-hashes`) into `target/sz-tools` |
| build | `build-native.sh` — C ABI (cdylib + staticlib), JNI and napi cdylibs, SBOMs; `--remap-path-prefix` for source, cargo home and target dir; strip (Linux: `strip=symbols`; macOS: linker `-x -S`; Windows: PDB with line tables) |
| gates | `gates/check-exports.sh` (nm / dumpbin vs `ffi/expected-exports/*.exports`), `gates/check-linkage.sh` (SONAME / install name / deps / minos; Windows: no `vcruntime140*.dll` / `api-ms-win-crt-*` imports), `gates/check-glibc-ceiling.sh` (Linux, `objdump -T` <= 2.34), `gates/check-no-build-paths.sh`, `gates/run-c-tests.sh` (`ffi/tests/c` + `ffi/examples` linked shared and static against the staged files) |
| package | `package-c.sh`, `package-python.sh`, `package-node.sh`, `package-cpp.sh` (runs the C++ ctest suite, plain optimized build), `package-sboms.sh` (per-artifact SBOM assets) |
| smoke | `smoke-bindings.sh` — pytest (installed wheel), `npm test`, `mvn test`, `dotnet test`, all against the staged natives |
| universal | `package-java.sh linux-x64`, `package-dotnet.sh`, `package-node.sh --trpc linux-x64` (need every target's natives) |
| assemble | collect every target's `out/` + `universal/out` and `make-sums.sh`; uploaded as the `release-assets` workflow artifact (this is also the dry run) |
| publish | tag **push** only: `actions/attest-build-provenance`, `gh release create` |

Export baselines (`ffi/expected-exports/`): `SzConfigTool.exports` must equal
the functions declared in `ffi/include/libSzConfigTool.h`; the JNI, napi and
pyo3 libraries may export only their entry symbol (no `SzConfigTool_*`). The
lists are compared, never passed to the linker. A per-OS
`<name>.<linux|macos|windows>.exports` overrides when a platform differs.

The C++ ASan + UBSan run is CI-only (`ci.yml`, job `linux`); no release job
or packaging script builds with sanitizers (`build-env.sh` and
`package-cpp.sh` refuse sanitizer flags).

## Cutting a release

1. Bump `[workspace.package] version` (then `cargo update -w`) and the binding
   manifests (`bindings/node/package.json`, `bindings/node/trpc/package.json`
   incl. `peerDependencies.sz-configtool`, `bindings/java/pom.xml`,
   `bindings/csharp/Directory.Build.props`), refresh both npm lockfiles
   (`npm install --package-lock-only` in `bindings/node` and
   `bindings/node/trpc`); `packaging/gates/check-versions.sh v<version>` must
   pass (it also checks the lockfiles).
2. Optional dry run: *Actions → Release → Run workflow*, on any branch or tag
   (builds, gates, assembles every asset + `SHA256SUMS` into the
   `release-assets` workflow artifact; never attests or publishes — the
   `release` job requires `github.event_name == 'push'`, so a dispatch on a
   tag does not publish either).
3. `git tag -a v<version> -m "Release v<version>" && git push origin v<version>`.
   Only that tag push publishes; a `-rc.N` / `-alpha.N` / `-beta.N` tag is
   marked a GitHub prerelease (`-N` is a full release).

## Code signing, Gatekeeper and the VC++ runtime

* **Unsigned binaries.** No release binary is code-signed: no Authenticode
  signature on the Windows DLLs, no macOS `codesign` / notarization of the
  dylibs, `.node` or wheel extension. Integrity and origin come from
  `SHA256SUMS` + the build-provenance attestation (below). G2 signs both
  (Authenticode, and macOS codesign + notarization); doing the same here is
  future work (it needs signing identities/secrets in the release job).
* **macOS Gatekeeper.** A browser download carries the
  `com.apple.quarantine` attribute, and Gatekeeper may refuse to load an
  unsigned, quarantined library. After verifying the download, remove it:
  `xattr -d com.apple.quarantine libSzConfigTool.dylib` (or recursively on
  the extracted directory: `xattr -dr com.apple.quarantine <dir>`). `curl`,
  `pip`, `npm` and Maven downloads are not quarantined.
* **VC++ runtime (Windows).** The shipped `SzConfigTool.dll`,
  `szconfigtool_jni.dll`, `.node` and `.pyd` are linked with the static MSVC
  runtime (`-C target-feature=+crt-static`, set in `lib/build-env.sh`): they
  import only Windows system DLLs (`check-linkage.sh` rejects
  `vcruntime140*.dll` and `api-ms-win-crt-*`), so **no VC++ Redistributable
  is required**. The static archive `SzConfigTool_static.lib` is the
  exception: it is built with the default dynamic CRT (`/MD`, what CMake and
  most C/C++ projects use), so the consuming application links the CRT it
  already uses; `lib/native-static-libs.txt` lists it.

## Verifying a download

```bash
sha256sum -c --ignore-missing SHA256SUMS            # macOS: shasum -a 256 -c
gh attestation verify sz-configtool-<v>-linux-x64.tar.gz --repo brianmacy/sz-rust-sdk-configtool
# offline, with the bundle attached to the release:
gh attestation verify sz-configtool-<v>-linux-x64.tar.gz --repo brianmacy/sz-rust-sdk-configtool \
  --bundle sz-configtool-<v>.intoto.jsonl
```

## Local use

```bash
export CARGO_TARGET_DIR=$PWD/target
packaging/install-tools.sh macos-arm64
packaging/release-target.sh macos-arm64 all            # output in $CARGO_TARGET_DIR/dist/macos-arm64/out
packaging/install-tools.sh linux-x64 rust zig cargo-tools
packaging/build-native.sh linux-x64                    # cross build from macOS works (zig)
```

The scripts need bash, python >= 3.10 (3.11+ for maturin), curl, tar, and on
Linux binutils (`objdump`, `readelf`, `nm`) or their llvm equivalents.
`package-java.sh` / `package-dotnet.sh` accept `--allow-partial` for local
single-platform packages; release CI never passes it.

## Toolchain alignment with G2

Every pin matches G2 (`~/dev/v4/G2`) where G2 pins the tool. Runner labels are
the standard GitHub-hosted images of G2's larger/self-hosted runners.

| Tool | Here | G2 source |
|---|---|---|
| Rust | `1.97.0` (`rust-toolchain.toml`); MSRV 1.88 still declared + checked in CI | `rust-toolchain.toml:26` |
| zig | `0.16.0`, sha256 linux x86_64 / aarch64 identical | `.github/workflows/build_anylinux_amd.yml:62,65`, `build_anylinux_arm.yml:54,57` |
| cargo-zigbuild | `0.22.3` | `build_anylinux_amd.yml:66`, `build_anylinux_arm.yml:58` |
| glibc floor | `2.34` via `*-unknown-linux-gnu.2.34` | `dev/scripts/verify-glibc-ceiling.sh:25` |
| glibc gate | `gates/check-glibc-ceiling.sh` — port (same `objdump -T` + `sort -V`) | `dev/scripts/verify-glibc-ceiling.sh:51` |
| export gate | `gates/check-exports.sh` — same nm/dumpbin-vs-baseline method, stricter | `dev/scripts/verify-export-surface.sh:59` |
| macOS floor | `MACOSX_DEPLOYMENT_TARGET=15.0` | `dev/CMakeLists.txt:69` |
| MSVC linker | explicit `CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER` (vswhere) | `.claude/faqs/building/main-product-build.md:163-167` |
| JDK | Temurin `21.0.12.1+1`, sha256-verified (all 4 hosts) | `build_macos.yml:505-508` (Linux: `openjdk-21-jdk`, `build_anylinux_amd.yml:291`) |
| Maven | `3.9.16`, sha512-verified | `build_anylinux_amd.yml:344-345` |
| .NET SDK | `8.0.x` (`actions/setup-dotnet`) | `build_anylinux_amd.yml:397` |
| Python | `3.12` (`actions/setup-python`) | `build_win.yml:560` |
| Linux x64 runner | `ubuntu-24.04` | `Ubuntu-24.04-16Core`, `build_anylinux_amd.yml:145` |
| Linux arm64 runner | `ubuntu-24.04-arm` | `Ubuntu-24.04-16Core-ARM`, `build_anylinux_arm.yml:102` |
| macOS runner | `macos-26` | `macos-26-xlarge`, `build_macos.yml:129` |
| Windows runner | `windows-2025` | `Windows-2025-16Core`, `build_win.yml:123` |
| maturin | `1.9.0` — **G2 pins none**; lowest allowed by `bindings/python/pyproject.toml` (`>=1.9`), verified | — |
| Node / npm | `22.18.0` (npm 10.9.3 bundled) — **G2 pins none**; lowest the Node tests support | — |
| CMake / Ninja | runner image — **G2 pins none** (apt `cmake ninja-build`, `docker-anylinux/Dockerfile:23`); `bindings/cpp` needs CMake >= 3.20 | — |
| cargo-cyclonedx | `0.5.9` — **G2 pins none** | — |
| cargo-deny | `0.20.2` prebuilt `x86_64-unknown-linux-musl`, sha256-verified (`security.yml`, every PR) — **G2 pins none** | — |
| cargo-audit / cargo-vet | `cargo install --locked`, unpinned; weekly + tags + manual only (`security.yml`) — **G2 pins none** (`security-scan.yml:112`, unpinned) | — |

JDK, Maven and Node are downloaded by `install-tools.sh` and checked against
the sha256/sha512 in `config.yaml` instead of using `actions/setup-java` /
`setup-node`: that is how G2 provisions the JDK (`build_macos.yml:505-508`)
and Maven (`build_anylinux_amd.yml:344-345`) — fetched and hash-checked —
and Node follows the same rule. It keeps one pinned, verified version identical on
all four hosts and in local runs (the scripts work outside GitHub Actions),
and no action's floating "latest patch" can change a release build.
