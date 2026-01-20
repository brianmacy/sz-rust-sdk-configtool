/**
 * Basic C test for libSzConfigTool
 *
 * Tests:
 * 1. Library linkage
 * 2. Result struct field access (returnCode matching SzHelpers)
 * 3. Memory management (free)
 * 4. Basic operations (add data source, list, delete)
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "../../include/libSzConfigTool.h"

#define ASSERT(condition, message) \
    if (!(condition)) { \
        fprintf(stderr, "FAIL: %s\n", message); \
        return 1; \
    }

int main(void) {
    printf("=== libSzConfigTool C Test ===\n\n");

    // Test 1: Initial empty config
    const char *initial_config = "{\"G2_CONFIG\":{\"CFG_DSRC\":[]}}";

    // Test 2: Add a data source
    printf("1. Adding data source 'TEST_DS'...\n");
    struct SzConfigTool_result result1 = SzConfigTool_addDataSource(
        initial_config,
        "TEST_DS"
    );

    // Check result using returnCode field (SzHelpers convention)
    ASSERT(result1.returnCode == 0, "addDataSource should return 0");
    ASSERT(result1.response != NULL, "addDataSource should return non-null response");

    printf("   ✓ Data source added\n");
    printf("   returnCode: %lld\n", (long long)result1.returnCode);

    // Save config for next operation
    char *config_with_ds = strdup(result1.response);
    SzConfigTool_free(result1.response);

    // Test 3: List data sources
    printf("\n2. Listing data sources...\n");
    struct SzConfigTool_result result2 = SzConfigTool_listDataSources(config_with_ds);

    ASSERT(result2.returnCode == 0, "listDataSources should return 0");
    ASSERT(result2.response != NULL, "listDataSources should return non-null response");
    ASSERT(strstr(result2.response, "TEST_DS") != NULL,
           "listDataSources should include TEST_DS");

    printf("   ✓ Data sources listed\n");
    printf("   Response: %s\n", result2.response);

    SzConfigTool_free(result2.response);

    // Test 4: Delete data source
    printf("\n3. Deleting data source 'TEST_DS'...\n");
    struct SzConfigTool_result result3 = SzConfigTool_deleteDataSource(
        config_with_ds,
        "TEST_DS"
    );

    ASSERT(result3.returnCode == 0, "deleteDataSource should return 0");
    ASSERT(result3.response != NULL, "deleteDataSource should return non-null response");

    printf("   ✓ Data source deleted\n");

    SzConfigTool_free(result3.response);
    free(config_with_ds);

    // Test 5: Error handling
    printf("\n4. Testing error handling (delete non-existent)...\n");
    struct SzConfigTool_result result4 = SzConfigTool_deleteDataSource(
        initial_config,
        "NONEXISTENT"
    );

    ASSERT(result4.returnCode != 0, "deleteDataSource should return error code");
    ASSERT(result4.response == NULL, "deleteDataSource error should return null response");

    // Check last error
    const char *last_error = SzConfigTool_getLastError();
    int64_t last_error_code = SzConfigTool_getLastErrorCode();

    printf("   ✓ Error detected\n");
    printf("   Last error: %s\n", last_error ? last_error : "(null)");
    printf("   Last error code: %lld\n", (long long)last_error_code);

    ASSERT(last_error != NULL, "getLastError should return error message");
    ASSERT(last_error_code != 0, "getLastErrorCode should return non-zero");

    // Test 6: Clear error
    printf("\n5. Clearing error...\n");
    SzConfigTool_clearLastError();

    ASSERT(SzConfigTool_getLastError() == NULL, "Error should be cleared");
    ASSERT(SzConfigTool_getLastErrorCode() == 0, "Error code should be cleared");

    printf("   ✓ Error cleared\n");

    printf("\n=== All tests passed! ===\n");
    return 0;
}
