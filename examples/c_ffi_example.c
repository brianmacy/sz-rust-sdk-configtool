/*
 * C FFI Example for sz_configtool_lib
 *
 * This example demonstrates using sz_configtool_lib from C code.
 *
 * Compile:
 *   gcc -o c_ffi_example c_ffi_example.c \
 *       -I../include \
 *       -L../target/release \
 *       -lsz_configtool_lib
 *
 * Run (Linux):
 *   LD_LIBRARY_PATH=../target/release ./c_ffi_example
 *
 * Run (macOS):
 *   DYLD_LIBRARY_PATH=../target/release ./c_ffi_example
 */

#include "libSzConfigTool.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* Helper function to print results */
void print_result(const char *operation, SzConfigTool_result result) {
    if (result.return_code == 0) {
        printf("✓ %s succeeded\n", operation);
        if (result.response) {
            printf("  Response: %s\n", result.response);
        }
    } else {
        const char *error = SzConfigTool_getLastError();
        printf("✗ %s failed: %s\n", operation, error);
    }
}

int main() {
    printf("=== C FFI Example for sz_configtool_lib ===\n\n");

    /* Initial minimal configuration */
    const char *initial_config =
        "{\n"
        "  \"G2_CONFIG\": {\n"
        "    \"CFG_DSRC\": []\n"
        "  }\n"
        "}";

    printf("Initial configuration:\n%s\n\n", initial_config);

    /* Keep track of current config (ownership transfers between calls) */
    char *config = strdup(initial_config);

    /* 1. Add a data source */
    printf("1. Adding data source 'CUSTOMERS'...\n");
    SzConfigTool_result result = SzConfigTool_addDataSource(
        config,
        "CUSTOMERS",
        NULL,  /* dsrc_id - auto-assign */
        "Customer records from CRM",
        NULL   /* retention_level - use default */
    );

    if (result.return_code == 0) {
        printf("  ✓ Data source added\n");
        free(config);
        config = result.response;  /* Take ownership of modified config */
    } else {
        const char *error = SzConfigTool_getLastError();
        printf("  ✗ Failed: %s\n", error);
        free(config);
        return 1;
    }

    /* 2. Add another data source */
    printf("\n2. Adding data source 'VENDORS'...\n");
    result = SzConfigTool_addDataSource(
        config,
        "VENDORS",
        NULL,
        "Vendor records from ERP",
        NULL
    );

    if (result.return_code == 0) {
        printf("  ✓ Data source added\n");
        free(config);
        config = result.response;
    } else {
        const char *error = SzConfigTool_getLastError();
        printf("  ✗ Failed: %s\n", error);
        free(config);
        return 1;
    }

    /* 3. List all data sources */
    printf("\n3. Listing all data sources...\n");
    result = SzConfigTool_listDataSources(config, "JSON");

    if (result.return_code == 0) {
        printf("  Data sources:\n%s\n", result.response);
        SzConfigTool_free(result.response);  /* Free response, keep config */
    } else {
        const char *error = SzConfigTool_getLastError();
        printf("  ✗ Failed: %s\n", error);
    }

    /* 4. Get specific data source */
    printf("\n4. Getting details for 'CUSTOMERS'...\n");
    result = SzConfigTool_getDataSource(config, "CUSTOMERS", "JSON");

    if (result.return_code == 0) {
        printf("  Customer data source:\n%s\n", result.response);
        SzConfigTool_free(result.response);
    } else {
        const char *error = SzConfigTool_getLastError();
        printf("  ✗ Failed: %s\n", error);
    }

    /* 5. Update data source description */
    printf("\n5. Updating CUSTOMERS description...\n");
    result = SzConfigTool_setDataSource(
        config,
        "CUSTOMERS",
        "Updated: Customer records from Salesforce",
        NULL  /* Keep retention level */
    );

    if (result.return_code == 0) {
        printf("  ✓ Data source updated\n");
        free(config);
        config = result.response;
    } else {
        const char *error = SzConfigTool_getLastError();
        printf("  ✗ Failed: %s\n", error);
    }

    /* 6. Delete a data source */
    printf("\n6. Deleting 'VENDORS' data source...\n");
    result = SzConfigTool_deleteDataSource(config, "VENDORS");

    if (result.return_code == 0) {
        printf("  ✓ Data source deleted\n");
        free(config);
        config = result.response;
    } else {
        const char *error = SzConfigTool_getLastError();
        printf("  ✗ Failed: %s\n", error);
    }

    /* 7. Verify final state */
    printf("\n7. Final data source list:\n");
    result = SzConfigTool_listDataSources(config, "JSON");

    if (result.return_code == 0) {
        printf("  Remaining data sources:\n%s\n", result.response);
        SzConfigTool_free(result.response);
    } else {
        const char *error = SzConfigTool_getLastError();
        printf("  ✗ Failed: %s\n", error);
    }

    /* 8. Demonstrate error handling - try to get deleted data source */
    printf("\n8. Attempting to get deleted 'VENDORS' (should fail)...\n");
    result = SzConfigTool_getDataSource(config, "VENDORS", "JSON");

    if (result.return_code == 0) {
        printf("  ✗ Unexpected success!\n");
        SzConfigTool_free(result.response);
    } else {
        const char *error = SzConfigTool_getLastError();
        printf("  ✓ Expected error: %s\n", error);
    }

    /* Clean up */
    free(config);

    printf("\n=== Example Complete ===\n");
    return 0;
}
