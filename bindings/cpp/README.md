# szconfigtool — C++20 binding

Header-only C++20 binding of `libSzConfigTool` (the C ABI of
`sz_configtool_lib`): stateless functions that take a Senzing configuration
JSON string and return the modified configuration (or JSON text).

> **Unofficial.** This is an unofficial library: Senzing does not publicly
> document most configuration functions and parameters. Use it only with
> Senzing-provided guidance on what to change and when.

```cpp
#include <szconfigtool/szconfigtool.hpp>
namespace sz = szconfigtool;

std::string cfg = sz::AddDataSource(template_json, "CUSTOMERS", {.id = 5001});
std::string ds  = sz::GetDataSource(cfg, "CUSTOMERS");          // JSON text
cfg = sz::SetFragment(cfg, "SNAME_SSTAB",
                      {.description = sz::FieldUpdate<std::string>::Clear()});
```

## API

Every function is generated from `api/manifest/*.yaml` by
`tools/codegen/src/lang/cpp.rs` into
`include/szconfigtool/generated/api.hpp` (do not edit; Doxygen comments
come from the manifest). Contract: [`bindings/CONTRACT.md`](../CONTRACT.md).

| Manifest | C++ |
|---|---|
| name `add_data_source` | `szconfigtool::AddDataSource` (PascalCase, split on `_`) |
| implicit config | `const std::string& config_json` — opaque, never parsed by the binding |
| required arg (incl. `required: true`) | positional parameter, snake_case; C++ keywords get `_` (`class_`) |
| `optional` arg | `std::optional<T>` field of `<Name>Options` (designated initializers) |
| `tristate` arg | `FieldUpdate<T>` field: Leave (default) / `Clear()` / `Set(v)` (a value converts to Set) |
| `str` / `json` / `int` / `bool` / `str_list` | `std::string_view` / JSON text `std::string_view` / `std::int64_t` / `bool` / `std::vector<std::string>` |
| `int_or_str` (call selector) | two overloads: `std::int64_t` (id) / `std::string_view` (feature code), e.g. `GetComparisonCall(cfg, 34)` or `GetComparisonCall(cfg, "TAX_ID")`; an option field is `std::variant<std::int64_t, std::string>` |
| returns `config` / `json` | `std::string` (config / JSON text) |
| returns `config_and_json` | `ConfigAndJson{config, json}` |
| `tuple_names` (`json` or `config_and_json`) | `struct <Name>Result` with `config` (for `config_and_json`) and one `std::string` per name holding that member's JSON text (`3`, `true`, `"11"` with quotes): `auto [config, plan_id, was_created] = SetGenericPlan(...)` |
| returns `int` / `unit` | `std::int64_t` / `void` |
| `status: not_implemented` | not generated; reachable via `szconfigtool::Invoke(name, config, args_json)` |

Errors throw `SzConfigToolException : std::runtime_error`. The error kind IS
the reason code: `ReasonCode()` is the wire string and `Kind()` its
`ErrorKind` constant (generated from `project.yaml reason_codes`), plus
`Details()` (validation-errors/v1 JSON for `VALIDATION_ERRORS`) and
`ReturnCode()` (raw C code). A string with an embedded NUL is rejected as
`INVALID_INPUT` before the call. The C library's last error is thread-local;
the binding reads it on the calling thread immediately, so threads never see
each other's errors.

All calls go through `SzConfigTool_invoke`; response strings are owned by an
RAII holder that calls `SzConfigTool_free`. The envelope is read by a small
dependency-free JSON reader: `config` is decoded from its JSON string to the
library's exact bytes, `result` is returned as its verbatim JSON text.

## Build and test (in this repository)

Requires CMake >= 3.20, a C++20 compiler and Rust (to build the native
library). googletest v1.15.2 is fetched with FetchContent; pass
`-DSZCONFIGTOOL_USE_SYSTEM_GTEST=ON` to use an installed GTest instead.

```bash
cargo build -p sz-configtool-ffi --release \
  && cmake -S bindings/cpp -B bindings/cpp/build -G Ninja \
  && cmake --build bindings/cpp/build \
  && ctest --test-dir bindings/cpp/build -j8
```

The suite runs the generated `api/manifest/generated/conformance.json`
against the real library through the typed functions (shared and static
link), plus naming, argument, error-mapping, opacity, thread-isolation and
install/`find_package` consumer tests. ASan + UBSan:

```bash
cmake -S bindings/cpp -B bindings/cpp/build-asan -G Ninja -DSZCONFIGTOOL_ENABLE_SANITIZERS=ON \
  && cmake --build bindings/cpp/build-asan && ctest --test-dir bindings/cpp/build-asan -j8
```

Regenerate the headers after a manifest change with
`cargo run -p sz-configtool-codegen` (`-- --check` fails when stale).

| CMake option | Default | Meaning |
|---|---|---|
| `SZCONFIGTOOL_NATIVE_DIR` | `$CARGO_TARGET_DIR/release` or `<repo>/target/release` | where `libSzConfigTool` is |
| `SZCONFIGTOOL_C_INCLUDE_DIR` | `<repo>/ffi/include` | where `libSzConfigTool.h` is |
| `SZCONFIGTOOL_LINK_STATIC` | `OFF` | `SzConfigTool::szconfigtool` links `libSzConfigTool.a` (Windows: the staticlib) |
| `SZCONFIGTOOL_STATIC_DEPS` | per platform | system libs the Rust staticlib needs |
| `SZCONFIGTOOL_BUILD_TESTS` / `_EXAMPLES` | `ON` top-level | |
| `SZCONFIGTOOL_ENABLE_SANITIZERS` | `OFF` | ASan + UBSan for tests/examples |

## Install and consume

```bash
cmake --install bindings/cpp/build --prefix /opt/szconfigtool
```

installs the headers, `libSzConfigTool.h`, the shared and static libraries
and `lib/cmake/szconfigtool/szconfigtool-config.cmake`. In a consumer:

```cmake
find_package(szconfigtool 0.10 REQUIRED)   # set SZCONFIGTOOL_USE_STATIC=ON first for the .a
target_link_libraries(app PRIVATE SzConfigTool::szconfigtool)
```

A GitHub Release archive of the binding has the same layout as the install
prefix: unpack it and point `CMAKE_PREFIX_PATH` at it.

## Runnable example

[`examples/quickstart`](examples/quickstart/quickstart.cpp) adds a data
source, clears a fragment description and handles `NOT_FOUND`. It is built
in-tree (`szconfigtool_quickstart`) and as a standalone consumer:

```bash
cmake -S bindings/cpp/examples/quickstart -B /tmp/qs -DCMAKE_PREFIX_PATH=/opt/szconfigtool \
  && cmake --build /tmp/qs && /tmp/qs/szconfigtool_quickstart tests/fixtures/g2config_template.json
```

## Platform status

Verified on macOS arm64 (Apple clang 21, CMake 4.4) and Linux arm64 (Debian
trixie, GCC, against the zig-built glibc 2.34 release natives), shared and
static. Windows is configured but not yet verified: `SZCONFIGTOOL_NATIVE_DIR`
may be a cargo target dir (`SzConfigTool.dll` + import library
`SzConfigTool.dll.lib`, staticlib `SzConfigTool.lib`) or the release layout
(import library `SzConfigTool.lib`, staticlib `SzConfigTool_static.lib`); the
install prefix always uses the release names (`bin/SzConfigTool.dll`,
`lib/SzConfigTool.lib`, `lib/SzConfigTool_static.lib`).
