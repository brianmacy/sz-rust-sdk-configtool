# libSzConfigTool @VERSION@ (@TARGET@)

C ABI of `sz_configtool_lib`, for editing Senzing configuration JSON
(`g2config.json`), plus its header-only C++20 binding and CMake package.

> **Unofficial.** This is an unofficial library: Senzing does not publicly
> document most configuration functions and parameters. Use it only with
> Senzing-provided guidance on what to change and when.

| Path | Content |
|---|---|
| `include/libSzConfigTool.h` | C header (all functions return `SzConfigTool_result`; free responses with `SzConfigTool_free`) |
| `include/szconfigtool/` | C++20 header-only binding (`#include <szconfigtool/szconfigtool.hpp>`) |
| `lib/` | shared library and static archive (see below) |
| `lib/native-static-libs.txt` | system libraries to add when linking the static archive |
| `lib/cmake/szconfigtool/` | CMake package: `find_package(szconfigtool)` → `SzConfigTool::szconfigtool` |
| `bin/` | Windows only: `SzConfigTool.dll` + `SzConfigTool.pdb` |
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

## C

Link (Linux/macOS):

```sh
cc app.c -Iinclude -Llib -lSzConfigTool -Wl,-rpath,'$ORIGIN/../lib'      # shared
cc app.c -Iinclude -DSZCONFIGTOOL_STATIC lib/libSzConfigTool.a $(cat lib/native-static-libs.txt) # static
```

Static linking: define `SZCONFIGTOOL_STATIC` (`-DSZCONFIGTOOL_STATIC`,
`/DSZCONFIGTOOL_STATIC`) for every file that includes the header. On Windows
it is required: without it the header declares the functions
`__declspec(dllimport)` and linking `SzConfigTool_static.lib` fails with
unresolved `__imp_SzConfigTool_*` symbols. (The CMake package defines it for
you when `SZCONFIGTOOL_USE_STATIC=ON`.)

## C++ (CMake)

C++20, CMake >= 3.20. This directory is an install prefix: point
`CMAKE_PREFIX_PATH` at it.

```cmake
# set(SZCONFIGTOOL_USE_STATIC ON) before find_package to link the static archive
find_package(szconfigtool @SERIES@ REQUIRED)   # SameMinorVersion compatibility
target_link_libraries(app PRIVATE SzConfigTool::szconfigtool)
```

```sh
cmake -S . -B build -DCMAKE_PREFIX_PATH=/path/to/this/directory   # add -DSZCONFIGTOOL_USE_STATIC=ON for static
cmake --build build --config Release
```

```cpp
#include <szconfigtool/szconfigtool.hpp>
namespace sz = szconfigtool;
std::string cfg = sz::AddDataSource(template_json, "CUSTOMERS", {.id = 5001});
```

Errors throw `szconfigtool::SzConfigToolException`. Shared on Linux/macOS:
the consumer's build tree gets an rpath to `lib/`; for a deployed binary
ship the library and set an rpath (or `LD_LIBRARY_PATH`). Shared on Windows:
copy `bin/SzConfigTool.dll` next to the executable (or add `bin/` to `PATH`).
API reference: `bindings/cpp/README.md` in the source repository.

## Windows runtime

`SzConfigTool.dll` uses the static MSVC runtime, so it needs no VC++
Redistributable (it imports only Windows system DLLs). `SzConfigTool_static.lib`
is built for the dynamic CRT (`/MD`), like most C/C++ projects.

## Verification

Verify this archive against the release's `SHA256SUMS` and build provenance:
`sha256sum -c --ignore-missing SHA256SUMS` and
`gh attestation verify <archive> --repo brianmacy/sz-rust-sdk-configtool`.

The binaries are not code-signed (no Authenticode, no macOS codesign or
notarization). On macOS, if the archive was downloaded with a browser,
Gatekeeper may refuse the quarantined dylib; after verifying it, run
`xattr -dr com.apple.quarantine <this directory>`.
