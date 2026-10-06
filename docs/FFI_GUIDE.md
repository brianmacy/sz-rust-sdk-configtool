# C FFI Interface Guide

Complete guide to using sz_configtool_lib from C, C++, Python (ctypes), and other languages via the Foreign Function Interface (FFI).

## Overview

The sz_configtool_lib provides a C-compatible FFI layer that allows the library to be used from any language that can call C functions. This includes:

- C and C++
- Python (via ctypes or cffi)
- Go (via cgo)
- Java (via JNA or JNI)
- Node.js (via node-ffi or N-API)
- Ruby (via fiddle or ffi)
- And many more

## Building the Shared Library

### Build Commands

```bash
# Build the C library (workspace crate ffi/, package sz-configtool-ffi)
cargo build -p sz-configtool-ffi --release

# Outputs:
# Linux:   target/release/libSzConfigTool.so   (+ libSzConfigTool.a)
# macOS:   target/release/libSzConfigTool.dylib (+ libSzConfigTool.a)
# Windows: target/release/SzConfigTool.dll     (+ SzConfigTool.dll.lib import library)
```

### Installation

Copy the shared library to a system location or use it directly:

```bash
# Linux
sudo cp target/release/libSzConfigTool.so /usr/local/lib/
sudo ldconfig

# macOS
sudo cp target/release/libSzConfigTool.dylib /usr/local/lib/

# Or embed an rpath when linking (see below), or use LD_LIBRARY_PATH/DYLD_LIBRARY_PATH
```

## C/C++ Usage

### Header File

The C header is [`ffi/include/libSzConfigTool.h`](../ffi/include/libSzConfigTool.h)
and is the authoritative list of functions and signatures:

```c
#include "libSzConfigTool.h"
```

Every function is marked `SZCONFIGTOOL_API`. When linking the static archive,
`#define SZCONFIGTOOL_STATIC` before including the header (on Windows this
avoids `__declspec(dllimport)`).

### Core Types

#### SzConfigTool_result

All functions that return configuration or data return this structure:

```c
typedef struct SzConfigTool_result {
    char *response;      // Response string (owned by the library; free with SzConfigTool_free)
    int64_t returnCode;  // 0 = success, negative = error
} SzConfigTool_result;
```

**Important**: Always check `returnCode` before using `response`. Always free `response` using `SzConfigTool_free()` when done.

### Memory Management

**Critical Rules**:

1. All strings in `response` are allocated by the library
2. You MUST call `SzConfigTool_free()` on every non-NULL `response`
3. Never call `free()` or `delete` on them - use `SzConfigTool_free()`
4. Strings from `SzConfigTool_getLastError*()` and `SzConfigTool_getLibraryVersion()` are
   owned by the library: never free them

#### Free Function

```c
void SzConfigTool_free(char *ptr);
```

### Error Handling

Each call records its outcome in a **per-thread** last-error slot (cleared on success):

```c
const char *SzConfigTool_getLastError(void);           // message, or NULL
int64_t     SzConfigTool_getLastErrorCode(void);       // 0 or negative
const char *SzConfigTool_getLastErrorReasonCode(void); // e.g. "VALIDATION_ERRORS", or NULL
const char *SzConfigTool_getLastErrorDetails(void);    // versioned JSON, or NULL
```

Returned strings are NUL-terminated and stay valid until the next `SzConfigTool_*`
call on the same thread. A Rust panic is never propagated into C: it is reported
as returnCode `-2` with an `internal panic in <function>: ...` message.

**Return codes.** `SzConfigTool_invoke` returns only `0`, `-1` (NULL or invalid
UTF-8 argument) and `-2` (failure; library errors always carry a reason code).
The typed exports do not share one numbering: depending on the function a
library error is `-2` (with a reason code), `-2` without one, or `-5` (message
only); invalid UTF-8 is `-1` or `-2`; argument-JSON failures are `-3`; and
`SzConfigTool_setGenericThreshold` returns `-4` for an unknown plan id. The
exact per-function lists are in the header's "Return codes" section (checked
against the source by `ffi/tests/return_codes.rs`). Treat any non-zero code as
failure and read `SzConfigTool_getLastErrorReasonCode()` when you need to
classify it.

**Pattern**:

```c
SzConfigTool_result result = SzConfigTool_addDataSource(config, "MY_SOURCE");

if (result.returnCode == 0) {
    // Use result.response, then:
    SzConfigTool_free(result.response);  // REQUIRED!
} else {
    fprintf(stderr, "Error: %s\n", SzConfigTool_getLastError());
}
```

### C Example

A complete, compiled-and-run-in-CI example is
[`ffi/examples/c_ffi_example.c`](../ffi/examples/c_ffi_example.c). Core loop:

```c
#include "libSzConfigTool.h"
#include <stdio.h>

int main(void) {
    const char *initial = "{\"G2_CONFIG\":{\"CFG_DSRC\":[]}}";

    SzConfigTool_result added = SzConfigTool_addDataSource(initial, "MY_SOURCE");
    if (added.returnCode != 0) {
        fprintf(stderr, "Add failed: %s\n", SzConfigTool_getLastError());
        return 1;
    }
    char *config = added.response;  /* library-owned; free with SzConfigTool_free */

    SzConfigTool_result list = SzConfigTool_listDataSources(config);
    if (list.returnCode == 0) {
        printf("Data sources:\n%s\n", list.response);
        SzConfigTool_free(list.response);
    }

    SzConfigTool_free(config);
    return 0;
}
```

### Compiling C Programs

```bash
cc -o myapp myapp.c -Iffi/include -Ltarget/release -lSzConfigTool \
   -Wl,-rpath,"$PWD/target/release"
./myapp
```

### Versioning

```c
printf("libSzConfigTool %s\n", SzConfigTool_getLibraryVersion());
if (SzConfigTool_getAbiVersion() != SZCONFIGTOOL_ABI_VERSION) { /* incompatible */ }
```

### C++ Example

```cpp
#include "libSzConfigTool.h"
#include <fstream>
#include <iostream>
#include <memory>
#include <sstream>

// RAII owner for library-allocated strings
using SzString = std::unique_ptr<char, decltype(&SzConfigTool_free)>;

int main() {
    std::ifstream file("g2config.json");
    std::stringstream buffer;
    buffer << file.rdbuf();
    std::string config = buffer.str();

    auto result = SzConfigTool_addDataSource(config.c_str(), "MY_SOURCE");
    if (result.returnCode != 0) {
        std::cerr << "Error: " << SzConfigTool_getLastError() << std::endl;
        return 1;
    }
    SzString response(result.response, &SzConfigTool_free);
    std::cout << "Success! New config:\n" << response.get() << std::endl;
    return 0;
}
```

## Python Usage (ctypes)

### Python Wrapper Example

```python
import ctypes
from pathlib import Path

lib = ctypes.CDLL(str(Path("target/release/libSzConfigTool.so")))  # or .dylib / SzConfigTool.dll

# Field order and types must match the C struct exactly.
class SzResult(ctypes.Structure):
    _fields_ = [
        ("response", ctypes.c_void_p),  # void* so the original pointer can be freed
        ("returnCode", ctypes.c_int64),
    ]

lib.SzConfigTool_addDataSource.argtypes = [ctypes.c_char_p, ctypes.c_char_p]
lib.SzConfigTool_addDataSource.restype = SzResult
lib.SzConfigTool_getLastError.restype = ctypes.c_char_p
lib.SzConfigTool_free.argtypes = [ctypes.c_void_p]

def call_ffi(func, *args):
    """Call an SzConfigTool function; return the response str or raise."""
    result = func(*args)
    if result.returnCode != 0:
        raise RuntimeError(lib.SzConfigTool_getLastError().decode("utf-8"))
    try:
        return ctypes.string_at(result.response).decode("utf-8")
    finally:
        lib.SzConfigTool_free(result.response)

with open("g2config.json", "r") as f:
    config = f.read()

config = call_ffi(lib.SzConfigTool_addDataSource, config.encode("utf-8"), b"MY_SOURCE")
print(config)
```

## JSON Parameter Marshalling

Complex parameters (like maps or arrays) are passed as JSON strings:

### C Example with JSON Parameters

```c
#include "libSzConfigTool.h"
#include <stdio.h>

int main() {
    const char *config = /* ... */;

    // Complex update with multiple fields
    const char *updates = "{"
        "\"CONNECT_STR\": \"new_connection\","
        "\"SFUNC_DESC\": \"Updated description\","
        "\"LANGUAGE\": \"eng\""
    "}";

    SzConfigTool_result result = SzConfigTool_setStandardizeFunctionWithJson(
        config,
        "PARSE",
        updates
    );

    if (result.returnCode == 0) {
        printf("Function updated!\n");
        SzConfigTool_free(result.response);
    } else {
        fprintf(stderr, "Error: %s\n", SzConfigTool_getLastError());
    }

    return 0;
}
```

### Python Example with JSON Parameters

```python
updates = {
    "CONNECT_STR": "new_connection",
    "SFUNC_DESC": "Updated description",
    "LANGUAGE": "eng"
}

config = tool.set_standardize_function(
    config,
    "PARSE",
    json.dumps(updates)
)
```

## Thread Safety

- **FFI Functions**: Thread-safe (can be called from multiple threads)
- **Error Storage**: Thread-local (each thread has separate error state)
- **Configuration JSON**: Immutable (operations return new modified config)

## Available FFI Functions

The C library exports 149 functions. See [`ffi/include/libSzConfigTool.h`](../ffi/include/libSzConfigTool.h) for complete declarations.

### Function Categories

| Category | Exports |
|---|---|
| Infrastructure (`free`, last-error accessors, versions, `invoke`) | 9 |
| Data sources | 5 |
| Attributes | 5 |
| Elements | 7 |
| Features | 6 |
| Behavior overrides | 4 |
| Thresholds (comparison + generic) | 10 |
| Functions (standardize 7, expression 7, comparison 7, distinct 6, candidate 5, matching 5, scoring 5, validation 5) | 47 |
| Calls (standardize 6, expression 7, comparison 7, distinct 7) | 27 |
| Config sections 7, rules 5, fragments 5, generic plans 3, system parameters 2, versioning 4, settings 1, export 1, validation 1 | 29 |
| **Total** | **149** |

## Best Practices

1. **Always Check Return Codes**: Never use `response` without checking `returnCode`
2. **Always Free Strings**: Memory leaks occur if you don't free FFI strings
3. **Copy Strings if Needed**: If storing strings, copy them before freeing
4. **Handle Errors Gracefully**: Use `SzConfigTool_getLastError()` for debugging
5. **Use RAII in C++**: Create wrapper classes to manage memory automatically
6. **JSON Validation**: Validate JSON before passing to FFI functions
7. **Thread Safety**: Safe to call from multiple threads, but manage config strings

## Performance Considerations

- **JSON Parsing**: Occurs once per operation, keep configs reasonably sized
- **String Copying**: FFI involves some string copying overhead
- **Memory Allocation**: Rust allocates, C frees - no significant overhead
- **Function Call Overhead**: Minimal, similar to any shared library call

## Troubleshoug

### Library Not Found

```bash
# Linux
export LD_LIBRARY_PATH=/path/to/library:$LD_LIBRARY_PATH

# macOS
export DYLD_LIBRARY_PATH=/path/to/library:$DYLD_LIBRARY_PATH

# Or use absolute path in dlopen/LoadLibrary
```

### Memory Leaks

- Ensure every `result.response` is freed with `SzConfigTool_free()`
- Use memory profilers (Valgrind, AddressSanitizer) to detect leaks
- In C++, use RAII wrappers to ensure automatic cleanup

### Segmentation Faults

- Always check for NULL pointers before dereferencing
- Ensure strings are NULL-terminated
- Don't free FFI strings with standard `free()`, use `SzConfigTool_free()`

### Encoding Issues

- All strings should be UTF-8 encoded
- Check locale settings if seeing garbled text
- In Python, explicitly encode/decode with UTF-8

## See Also

- [API Documentation](API.md) - Complete API reference
- [README](../README.md) - Quick start guide
- [Contributing](CONTRIBUTING.md) - Contribution guidelines
- [C Header File](../ffi/include/libSzConfigTool.h) - Complete FFI declarations
