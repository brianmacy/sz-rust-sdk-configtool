# libSzConfigTool @VERSION@ (@TARGET@)

C ABI of `sz_configtool_lib`, for editing Senzing configuration JSON
(`g2config.json`).

> **Unofficial.** This is an unofficial library: Senzing does not publicly
> document most configuration functions and parameters. Use it only with
> Senzing-provided guidance on what to change and when.

| Path | Content |
|---|---|
| `include/libSzConfigTool.h` | C header (all functions return `SzConfigTool_result`; free responses with `SzConfigTool_free`) |
| `lib/` | shared library and static archive (see below) |
| `lib/native-static-libs.txt` | system libraries to add when linking the static archive |
| `sbom/sz-configtool-c.cdx.json` | CycloneDX SBOM of the Rust dependencies |
| `VERSION` | library version (= `SzConfigTool_getLibraryVersion()`) |

Supported platforms: Linux x86_64 / arm64 with glibc >= 2.34 (RHEL 9+,
Amazon Linux 2023, Ubuntu 22.04+, Debian 12+), macOS 15+ arm64, Windows x64.
RHEL 8 (glibc 2.28) is **not** supported by these binaries; build from source
with the Rust library (https://github.com/brianmacy/sz-rust-sdk-configtool).

Return codes: see "Return codes" in `include/libSzConfigTool.h` (the typed
functions do not share one numbering; `SzConfigTool_invoke` uses only 0 / -1 /
-2 and always sets a reason code for library errors).

Platform files:

| Platform | Shared | Static |
|---|---|---|
| Linux (glibc >= 2.34) | `lib/libSzConfigTool.so` (SONAME `libSzConfigTool.so`) | `lib/libSzConfigTool.a` |
| macOS arm64 (>= 15.0) | `lib/libSzConfigTool.dylib` (install name `@rpath/libSzConfigTool.dylib`) | `lib/libSzConfigTool.a` |
| Windows x64 (MSVC) | `bin/SzConfigTool.dll` + `bin/SzConfigTool.pdb`, import library `lib/SzConfigTool.lib` | `lib/SzConfigTool_static.lib` |

Link (Linux/macOS):

```sh
cc app.c -Iinclude -Llib -lSzConfigTool -Wl,-rpath,'$ORIGIN/../lib'      # shared
cc app.c -Iinclude -DSZCONFIGTOOL_STATIC lib/libSzConfigTool.a $(cat lib/native-static-libs.txt) # static
```

Static linking: define `SZCONFIGTOOL_STATIC` (`-DSZCONFIGTOOL_STATIC`,
`/DSZCONFIGTOOL_STATIC`) for every file that includes the header. On Windows
it is required: without it the header declares the functions
`__declspec(dllimport)` and linking `SzConfigTool_static.lib` fails with
unresolved `__imp_SzConfigTool_*` symbols.

Windows: `SzConfigTool.dll` uses the static MSVC runtime, so it needs no VC++
Redistributable (it imports only Windows system DLLs). `SzConfigTool_static.lib`
is built for the dynamic CRT (`/MD`), like most C/C++ projects.

Verify this archive against the release's `SHA256SUMS` and build provenance:
`gh attestation verify <archive> --repo brianmacy/sz-rust-sdk-configtool`.

The binaries are not code-signed (no Authenticode, no macOS codesign or
notarization). On macOS, if the archive was downloaded with a browser,
Gatekeeper may refuse the quarantined dylib; after verifying it, run
`xattr -dr com.apple.quarantine <this directory>`.
