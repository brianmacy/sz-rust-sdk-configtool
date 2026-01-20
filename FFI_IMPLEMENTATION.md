# C Shared Library FFI Implementation

This document summarizes the implementation of the C shared library FFI for sz-rust-sdk-configtool, following Senzing SDK patterns.

## Recent Updates (2026-01-20)

**Phase 2 Completion:**
- ✅ Added `SzConfigTool_setAttribute` function (completes attribute CRUD operations)
- ✅ Verified all datasource, element, feature, and threshold functions are implemented
- ✅ Cleaned up duplicate function definitions
- ✅ Total FFI functions: 121 (down from 131 due to duplicate removal)
- ✅ All tests passing: Rust (47 tests), C (1 test with CTest)

## Implementation Summary

### Phase 1: Infrastructure Fixes ✅

#### 1. Result Struct Field Naming
**Changed:** `return_code` → `returnCode`
**Location:** `src/ffi.rs:43`, `include/libSzConfigTool.h:27`
**Reason:** Match SzHelpers C convention (camelCase)

```c
typedef struct SzConfigTool_result {
    char *response;
    int64_t returnCode;  // ✓ Matches SzHelpers convention
} SzConfigTool_result;
```

#### 2. C++ Compatibility Guards
**Added:** `extern "C"` guards to header
**Location:** `include/libSzConfigTool.h:11-13, 572-574`
**Benefit:** Allows C++ code to link against C library

```c
#ifdef __cplusplus
extern "C" {
#endif
// ... function declarations ...
#ifdef __cplusplus
}
#endif
```

#### 3. Function Naming Consistency
**Removed:** `_helper` suffix from 11 functions
**Location:** `src/ffi.rs` (throughout)
**Result:** Consistent naming across all 131 FFI functions

**Functions Updated:**
- `SzConfigTool_addDataSource_helper` → `SzConfigTool_addDataSource`
- `SzConfigTool_deleteDataSource_helper` → `SzConfigTool_deleteDataSource`
- `SzConfigTool_listDataSources_helper` → `SzConfigTool_listDataSources`
- `SzConfigTool_addAttribute_helper` → `SzConfigTool_addAttribute`
- `SzConfigTool_deleteAttribute_helper` → `SzConfigTool_deleteAttribute`
- `SzConfigTool_getAttribute_helper` → `SzConfigTool_getAttribute`
- `SzConfigTool_listAttributes_helper` → `SzConfigTool_listAttributes`
- `SzConfigTool_getFeature_helper` → `SzConfigTool_getFeature`
- `SzConfigTool_listFeatures_helper` → `SzConfigTool_listFeatures`
- `SzConfigTool_getElement_helper` → `SzConfigTool_getElement`
- `SzConfigTool_listElements_helper` → `SzConfigTool_listElements`

#### 4. Removed Duplicate Wrapper Functions
**Deleted:** Lines 9080-9188 in `src/ffi.rs`
**Reason:** Redundant wrappers after removing `_helper` suffix
**Result:** Cleaner codebase, no function duplication

### Phase 3: Library Building & Testing ✅

#### 1. Library Configuration
**File:** `Cargo.toml`
**Changed:** Removed `staticlib` from `crate-type` (per user request)

```toml
[lib]
crate-type = ["lib", "cdylib"]
```

**Output Files:**
- **Rust library:** `libsz_configtool_lib.rlib` (for Rust projects)
- **C shared library:** `libsz_configtool_lib.dylib` (macOS) / `.so` (Linux)

#### 2. Senzing-Style Naming
**Script:** `create_senzing_lib.sh`
**Purpose:** Create `libSzConfigTool.dylib` symlink from `libsz_configtool_lib.dylib`
**Usage:** `./create_senzing_lib.sh release`

**Result:**
```bash
target/release/
├── libsz_configtool_lib.dylib  # Built by Cargo
└── libSzConfigTool.dylib       # Symlink (Senzing naming)
```

#### 3. C Test Suite
**Location:** `tests/c/`
**Build System:** CMake (as requested)
**Test Coverage:**
- Library linkage verification
- Result struct field access (`returnCode`)
- Memory management (`SzConfigTool_free`)
- Basic operations (add, list, delete data sources)
- Error handling (`getLastError`, `clearLastError`)

**Files:**
- `tests/c/test_basic.c` - Test implementation (100 lines)
- `tests/c/CMakeLists.txt` - CMake configuration
- `tests/c/Makefile` - Legacy Makefile (deprecated, use CMake)

**Running Tests:**
```bash
cd tests/c
mkdir build && cd build
cmake ..
cmake --build .
ctest --output-on-failure
```

**Test Output:**
```
=== libSzConfigTool C Test ===

1. Adding data source 'TEST_DS'...
   ✓ Data source added
   returnCode: 0

2. Listing data sources...
   ✓ Data sources listed
   Response: [{"id":1,"dataSource":"TEST_DS"}]

3. Deleting data source 'TEST_DS'...
   ✓ Data source deleted

4. Testing error handling (delete non-existent)...
   ✓ Error detected
   Last error: Data source not found: NONEXISTENT
   Last error code: -2

5. Clearing error...
   ✓ Error cleared

=== All tests passed! ===
```

## FFI Function Inventory

### Infrastructure (4 functions)
- `SzConfigTool_free` - Memory deallocation
- `SzConfigTool_getLastError` - Error message retrieval
- `SzConfigTool_getLastErrorCode` - Error code retrieval
- `SzConfigTool_clearLastError` - Error state reset

### Data Sources (3 functions)
- `SzConfigTool_addDataSource`
- `SzConfigTool_deleteDataSource`
- `SzConfigTool_listDataSources`

### Attributes (5 functions)
- `SzConfigTool_addAttribute`
- `SzConfigTool_deleteAttribute`
- `SzConfigTool_getAttribute`
- `SzConfigTool_listAttributes`
- `SzConfigTool_setAttribute` ⭐ NEW

### Features (2 functions)
- `SzConfigTool_getFeature`
- `SzConfigTool_listFeatures`

### Elements (2 functions)
- `SzConfigTool_getElement`
- `SzConfigTool_listElements`

### Additional Functions (116+ functions)
- Config sections, fragments, rules, thresholds
- Standardize/expression/comparison/distinct functions
- Call management (CFG_SFCALL, CFG_EFCALL, etc.)
- Generic plans, hashes, system parameters
- Versioning and compatibility

**Total: 121 FFI functions** (updated after removing duplicates and adding setAttribute)

## Build Process

### Development Build
```bash
cargo build --lib
./create_senzing_lib.sh debug
```

### Release Build
```bash
cargo build --lib --release
./create_senzing_lib.sh release
```

### Testing
```bash
# Rust tests
cargo test

# C tests (CMake)
cd tests/c && mkdir build && cd build
cmake .. && cmake --build . && ctest
```

## Platform Support

| Platform | Shared Library | Tested |
|----------|---------------|--------|
| macOS    | `libSzConfigTool.dylib` | ✅ |
| Linux    | `libSzConfigTool.so` | ⚠️ Not tested |
| Windows  | `SzConfigTool.dll` | ❌ Not implemented |

## Comparison with SzHelpers

| Aspect | SzHelpers | This Implementation | Status |
|--------|-----------|---------------------|--------|
| **Result Struct** | `returnCode` (camelCase) | `returnCode` | ✅ Match |
| **C++ Guards** | `extern "C"` | `extern "C"` | ✅ Match |
| **Export Macro** | `_DLEXPORT` macro | Not used | ⚠️ Different |
| **Function Naming** | All use `_helper` | None use `_helper` | ⚠️ Different |
| **Error Storage** | Thread-local | Thread-local (Mutex) | ✅ Compatible |
| **Memory Management** | Caller frees | Caller frees | ✅ Match |
| **Field Order** | response, returnCode | response, returnCode | ✅ Match |

### Naming Convention Decision

**SzHelpers Pattern:** Functions named `SzModule_function_helper()` wrap SDK calls.

**Our Pattern:** Functions named `SzConfigTool_function()` **are** the implementation (not wrappers).

**Rationale:** Since this library IS the implementation (not wrapping another SDK), the `_helper` suffix is not semantically appropriate. The library provides direct C bindings to Rust functions.

## Integration Example

```c
#include "libSzConfigTool.h"

int main(void) {
    const char *config = "{\"G2_CONFIG\":{\"CFG_DSRC\":[]}}";

    // Add data source
    struct SzConfigTool_result result =
        SzConfigTool_addDataSource(config, "MY_SOURCE");

    if (result.returnCode == 0) {
        printf("Success: %s\n", result.response);
        SzConfigTool_free(result.response);
    } else {
        fprintf(stderr, "Error: %s\n",
                SzConfigTool_getLastError());
    }

    return 0;
}
```

## Future Enhancements

### Completed ✅
- [x] Result struct field naming (returnCode)
- [x] C++ compatibility guards
- [x] Consistent function naming
- [x] Senzing-style library naming
- [x] C test suite with CMake
- [x] Memory management verification
- [x] Error handling validation

### Potential ❓
- [ ] Windows DLL support
- [ ] Linux testing and validation
- [ ] Python bindings (ctypes or PyO3)
- [ ] Additional language bindings
- [ ] Performance benchmarking
- [ ] Thread safety testing
- [ ] Fuzzing for C API

## Documentation

- **C Header:** `include/libSzConfigTool.h` (576 lines)
- **Rust FFI:** `src/ffi.rs` (9079 lines, 131 functions)
- **This Document:** `FFI_IMPLEMENTATION.md`

## Validation Status

| Check | Status |
|-------|--------|
| **Cargo build** | ✅ Pass (no warnings) |
| **Cargo test** | ✅ Pass (47 tests) |
| **Cargo clippy** | ✅ Pass (strict mode) |
| **C compilation** | ✅ Pass (gcc/clang) |
| **C test execution** | ✅ Pass (all assertions) |
| **Memory management** | ✅ Verified (no leaks) |
| **CMake build** | ✅ Pass |
| **CTest** | ✅ Pass (1/1 tests) |

## Contact

For questions or issues related to the C FFI implementation:
- **Repository:** https://github.com/brianmacy/sz-rust-sdk-configtool
- **Author:** Brian Macy <bmacy@senzing.com>

---

\*Last Updated: 2026-01-20 (Phase 2 Complete)\*
*Implementation Version: 0.1.0*
