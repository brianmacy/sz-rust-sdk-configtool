/*
 * C FFI example for libSzConfigTool.
 *
 * Build the library, then compile and run (from the repository root):
 *
 *   cargo build -p sz-configtool-ffi --release
 *   cc -o target/c_ffi_example ffi/examples/c_ffi_example.c \
 *       -Iffi/include -Ltarget/release -lSzConfigTool \
 *       -Wl,-rpath,"$PWD/target/release"
 *   ./target/c_ffi_example
 *
 * `cargo test -p sz-configtool-ffi --test c_abi` builds and runs this example
 * automatically and fails if it exits non-zero.
 *
 * Memory: every non-NULL `response` is allocated by the library and must be
 * released with SzConfigTool_free (never free()).
 */

#include "libSzConfigTool.h"
#include <stdio.h>
#include <string.h>

/* Replace *config with result.response on success; return 0 on success. */
static int take_config(const char *operation, SzConfigTool_result result, char **config) {
    if (result.returnCode != 0) {
        fprintf(stderr, "  FAIL %s: %s\n", operation, SzConfigTool_getLastError());
        return 1;
    }
    SzConfigTool_free(*config);
    *config = result.response;
    printf("  ok   %s\n", operation);
    return 0;
}

/* Print and free a read-only result; return 0 on success. */
static int show(const char *operation, SzConfigTool_result result) {
    if (result.returnCode != 0) {
        fprintf(stderr, "  FAIL %s: %s\n", operation, SzConfigTool_getLastError());
        return 1;
    }
    printf("  ok   %s: %s\n", operation, result.response);
    SzConfigTool_free(result.response);
    return 0;
}

int main(void) {
    printf("=== libSzConfigTool %s (ABI %d) ===\n", SzConfigTool_getLibraryVersion(),
           SzConfigTool_getAbiVersion());
    if (SzConfigTool_getAbiVersion() != SZCONFIGTOOL_ABI_VERSION) {
        fprintf(stderr, "ABI mismatch: header %d, library %d\n", SZCONFIGTOOL_ABI_VERSION,
                SzConfigTool_getAbiVersion());
        return 1;
    }

    /* `config` always holds a library-allocated string (or NULL). */
    char *config = NULL;
    const char *initial = "{\"G2_CONFIG\":{\"CFG_DSRC\":[]}}";
    int failures = 0;

    failures += take_config("add CUSTOMERS", SzConfigTool_addDataSource(initial, "CUSTOMERS"),
                            &config);
    if (config == NULL) {
        return 1;
    }
    failures += take_config("add VENDORS", SzConfigTool_addDataSource(config, "VENDORS"), &config);
    failures += show("list", SzConfigTool_listDataSources(config));
    failures += show("get CUSTOMERS", SzConfigTool_getDataSource(config, "CUSTOMERS"));
    failures += take_config("set CUSTOMERS retention",
                            SzConfigTool_setDataSource(config, "CUSTOMERS",
                                                       "{\"retentionLevel\":\"Remember\"}"),
                            &config);
    failures += take_config("delete VENDORS", SzConfigTool_deleteDataSource(config, "VENDORS"),
                            &config);
    failures += show("final list", SzConfigTool_listDataSources(config));

    /* Error handling: getting a deleted data source must fail. */
    SzConfigTool_result missing = SzConfigTool_getDataSource(config, "VENDORS");
    if (missing.returnCode == 0) {
        fprintf(stderr, "  FAIL get VENDORS unexpectedly succeeded\n");
        SzConfigTool_free(missing.response);
        failures++;
    } else {
        const char *reason = SzConfigTool_getLastErrorReasonCode();
        printf("  ok   expected error: %s (reason %s)\n", SzConfigTool_getLastError(),
               reason ? reason : "none");
    }

    SzConfigTool_free(config);
    printf("=== Example %s ===\n", failures == 0 ? "complete" : "FAILED");
    return failures == 0 ? 0 : 1;
}
