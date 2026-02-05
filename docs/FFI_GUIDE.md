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
# Build release version with optimizations
cargo build --lib --release

# The shared library will be at:
# Linux:   target/release/libsz_configtool_lib.so
# macOS:   target/release/libsz_configtool_lib.dylib
# Windows: target/release/sz_configtool_lib.dll
```

### Installation

Copy the shared library to a system location or use it directly:

```bash
# Linux
sudo cp target/release/libsz_configtool_lib.so /usr/local/lib/
sudo ldconfig

# macOS
sudo cp target/release/libsz_configtool_lib.dylib /usr/local/lib/

# Or use LD_LIBRARY_PATH/DYLD_LIBRARY_PATH
export LD_LIBRARY_PATH=/path/to/target/release:$LD_LIBRARY_PATH
```

## C/C++ Usage

### Header File

The C header file is located at `include/libSzConfigTool.h`. Include it in your C/C++ projects:

```c
#include "libSzConfigTool.h"
```

### Core Types

#### SzConfigTool_result

All FFI functions return this structure:

```c
typedef struct {
    int32_t return_code;  // 0 = success, 1 = error
    char *response;       // Response string (owned by Rust, must be freed)
} SzConfigTool_result;
```

**Important**: Always check `return_code` before using `response`. Always free `response` using `SzConfigTool_free()` when done.

### Memory Management

**Critical Rules**:

1. All strings returned by FFI functions are owned by Rust
2. You MUST call `SzConfigTool_free()` on every `response` string
3. Never call `free()` or `delete` on FFI-returned strings - use `SzConfigTool_free()`
4. Copy strings if you need to keep them beyond the FFI call scope

#### Free Function

```c
void SzConfigTool_free(char *ptr);
```

### Error Handling

Errors are stored in thread-local storage and retrieved with:

```c
const char *SzConfigTool_getLastError(void);
```

**Pattern**:

```c
SzConfigTool_result result = SzConfigTool_someFunction(params);

if (result.return_code == 0) {
    // Success - use result.response
    printf("Success: %s\n", result.response);
    SzConfigTool_free(result.response);  // REQUIRED!
} else {
    // Error - get error message
    const char *error = SzConfigTool_getLastError();
    fprintf(stderr, "Error: %s\n", error);
}
```

### Complete C Example

```c
#include "libSzConfigTool.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
    // Load configuration from file
    FILE *f = fopen("g2config.json", "r");
    if (!f) {
        perror("Failed to open config file");
        return 1;
    }

    fseek(f, 0, SEEK_END);
    long len = ftell(f);
    fseek(f, 0, SEEK_SET);

    char *config = malloc(len + 1);
    fread(config, 1, len, f);
    config[len] = '\0';
    fclose(f);

    // Add a data source
    SzConfigTool_result result = SzConfigTool_addDataSource(
        config,
        "MY_SOURCE",  // dsrc_code
        NULL,         // dsrc_id (optional)
        NULL,         // dsrc_desc (optional)
        NULL          // retention_level (optional)
    );

    if (result.return_code == 0) {
        printf("Data source added successfully!\n");

        // Use the modified configuration
        free(config);
        config = result.response;  // Take ownership

        // List all data sources
        SzConfigTool_result list_result = SzConfigTool_listDataSources(
            config,
            "JSON"  // output_format
        );

        if (list_result.return_code == 0) {
            printf("Data sources:\n%s\n", list_result.response);
            SzConfigTool_free(list_result.response);
        } else {
            const char *error = SzConfigTool_getLastError();
            fprintf(stderr, "List failed: %s\n", error);
        }

    } else {
        const char *error = SzConfigTool_getLastError();
        fprintf(stderr, "Add failed: %s\n", error);
        free(config);
        return 1;
    }

    free(config);
    return 0;
}
```

### Compiling C Programs

```bash
# Compile
gcc -o myapp myapp.c -I./include -L./target/release -lsz_configtool_lib

# Run with library path
LD_LIBRARY_PATH=./target/release ./myapp  # Linux
DYLD_LIBRARY_PATH=./target/release ./myapp  # macOS
```

### C++ Example

```cpp
#include "libSzConfigTool.h"
#include <iostream>
#include <fstream>
#include <sstream>
#include <memory>

// RAII wrapper for FFI strings
struct SzString {
    char *ptr;
    SzString(char *p) : ptr(p) {}
    ~SzString() { if (ptr) SzConfigTool_free(ptr); }
    operator const char*() const { return ptr; }
};

int main() {
    // Load config
    std::ifstream file("g2config.json");
    std::stringstream buffer;
    buffer << file.rdbuf();
    std::string config = buffer.str();

    // Add data source
    auto result = SzConfigTool_addDataSource(
        config.c_str(),
        "MY_SOURCE",
        nullptr,
        nullptr,
        nullptr
    );

    if (result.return_code == 0) {
        SzString response(result.response);
        std::cout << "Success! New config:\n" << response << std::endl;
    } else {
        std::cerr << "Error: " << SzConfigTool_getLastError() << std::endl;
        return 1;
    }

    return 0;
}
```

## Python Usage (ctypes)

### Python Wrapper Example

```python
import ctypes
import json
from pathlib import Path

# Load shared library
lib_path = Path("target/release/libsz_configtool_lib.so")  # or .dylib/.dll
lib = ctypes.CDLL(str(lib_path))

# Define result structure
class SzResult(ctypes.Structure):
    _fields_ = [
        ("return_code", ctypes.c_int32),
        ("response", ctypes.c_char_p),
    ]

# Configure function signatures
lib.SzConfigTool_addDataSource.argtypes = [
    ctypes.c_char_p,  # config_json
    ctypes.c_char_p,  # dsrc_code
    ctypes.c_char_p,  # dsrc_id
    ctypes.c_char_p,  # dsrc_desc
    ctypes.c_char_p,  # retention_level
]
lib.SzConfigTool_addDataSource.restype = SzResult

lib.SzConfigTool_getLastError.restype = ctypes.c_char_p
lib.SzConfigTool_free.argtypes = [ctypes.c_char_p]

# Helper function
def call_ffi(func, *args):
    """Call FFI function and handle errors."""
    result = func(*args)
    if result.return_code == 0:
        response = result.response.decode('utf-8')
        lib.SzConfigTool_free(result.response)
        return response
    else:
        error = lib.SzConfigTool_getLastError().decode('utf-8')
        raise RuntimeError(f"FFI error: {error}")

# Usage
with open("g2config.json", "r") as f:
    config = f.read()

# Add data source
config = call_ffi(
    lib.SzConfigTool_addDataSource,
    config.encode('utf-8'),
    b"MY_SOURCE",
    None,  # Optional parameters
    None,
    None,
)

print("Data source added!")
print(config)
```

### Python Wrapper Class

```python
class SzConfigTool:
    def __init__(self, lib_path):
        self.lib = ctypes.CDLL(str(lib_path))
        self._setup_functions()

    def _setup_functions(self):
        # Setup all function signatures
        self.lib.SzConfigTool_addDataSource.argtypes = [
            ctypes.c_char_p, ctypes.c_char_p, ctypes.c_char_p,
            ctypes.c_char_p, ctypes.c_char_p
        ]
        self.lib.SzConfigTool_addDataSource.restype = SzResult
        # ... setup other functions

    def add_data_source(self, config, dsrc_code, dsrc_id=None,
                       dsrc_desc=None, retention_level=None):
        result = self.lib.SzConfigTool_addDataSource(
            config.encode('utf-8'),
            dsrc_code.encode('utf-8'),
            dsrc_id.encode('utf-8') if dsrc_id else None,
            dsrc_desc.encode('utf-8') if dsrc_desc else None,
            retention_level.encode('utf-8') if retention_level else None,
        )

        if result.return_code == 0:
            response = result.response.decode('utf-8')
            self.lib.SzConfigTool_free(result.response)
            return response
        else:
            error = self.lib.SzConfigTool_getLastError().decode('utf-8')
            raise RuntimeError(error)

# Usage
tool = SzConfigTool("target/release/libsz_configtool_lib.so")
config = tool.add_data_source(config, "MY_SOURCE")
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

    if (result.return_code == 0) {
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

The FFI provides 98 functions covering all library operations. See `include/libSzConfigTool.h` for complete declarations.

### Function Categories

- **Data Sources**: 7 functions (add, delete, get, list, set, setId, getById)
- **Attributes**: 8 functions (add, delete, get, list, set, clone, setId, getById)
- **Features**: 24 functions (CRUD, elements, comparisons, distinct calls)
- **Elements**: 8 functions (add, delete, get, list, set, clone, setId, getById)
- **Thresholds**: 6 functions (comparison and generic thresholds)
- **Functions**: 28 functions (standardize, expression, comparison, distinct)
- **Calls**: 32 functions (all function types with BOM variants)
- **System**: Multiple functions (config sections, parameters, versioning)

## Best Practices

1. **Always Check Return Codes**: Never use `response` without checking `return_code`
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
- [C Header File](../include/libSzConfigTool.h) - Complete FFI declarations
