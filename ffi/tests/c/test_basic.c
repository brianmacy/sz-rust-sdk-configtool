/**
 * Basic C test for libSzConfigTool
 *
 * Tests:
 * 1. Library linkage
 * 2. Result struct field access (returnCode matching SzHelpers)
 * 3. Memory management (free)
 * 4. Basic operations (add data source, list, delete)
 * 5. Library/ABI version entry points
 * 6. Last error is NUL-terminated and thread-local
 * 7. Dynamic SzConfigTool_invoke (envelope + reason codes)
 */

/* strdup and pthreads are POSIX, not ISO C11. Windows uses the Win32 thread
 * API and _strdup instead (MSVC flags the POSIX name as deprecated, C4996). */
#ifndef _WIN32
#define _POSIX_C_SOURCE 200809L
#endif

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#ifdef _WIN32
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#define strdup _strdup
#else
#include <pthread.h>
#endif
#include "../../include/libSzConfigTool.h"

#define ASSERT(condition, message) \
    if (!(condition)) { \
        fprintf(stderr, "FAIL: %s\n", message); \
        return 1; \
    }

/* Runs on a second thread: it must not see the main thread's error. */
static int thread_sees_no_error(void) {
    return SzConfigTool_getLastError() == NULL && SzConfigTool_getLastErrorCode() == 0;
}

/* Runs thread_sees_no_error() on a new thread and joins it.
 * Returns 1 (clean), 0 (saw an error) or -1 (thread create/join failed). */
#ifdef _WIN32
static DWORD WINAPI other_thread(LPVOID arg) {
    (void)arg;
    return thread_sees_no_error() ? 1u : 0u;
}

static int run_other_thread(void) {
    HANDLE thread = CreateThread(NULL, 0, other_thread, NULL, 0, NULL);
    if (thread == NULL) {
        return -1;
    }
    DWORD exit_code = 0;
    int joined = WaitForSingleObject(thread, INFINITE) == WAIT_OBJECT_0 &&
                 GetExitCodeThread(thread, &exit_code);
    CloseHandle(thread);
    return joined ? (int)exit_code : -1;
}
#else
static void *other_thread(void *arg) {
    (void)arg;
    return thread_sees_no_error() ? (void *)1 : NULL;
}

static int run_other_thread(void) {
    pthread_t thread;
    void *thread_clean = NULL;
    if (pthread_create(&thread, NULL, other_thread, NULL) != 0 ||
        pthread_join(thread, &thread_clean) != 0) {
        return -1;
    }
    return thread_clean != NULL;
}
#endif

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

    // Test 7: Version entry points
    printf("\n6. Checking library and ABI version...\n");
    const char *version = SzConfigTool_getLibraryVersion();
    ASSERT(version != NULL && strlen(version) > 0, "getLibraryVersion should return a string");
    ASSERT(SzConfigTool_getAbiVersion() == SZCONFIGTOOL_ABI_VERSION,
           "getAbiVersion should match SZCONFIGTOOL_ABI_VERSION");
    printf("   ✓ version %s, ABI %d\n", version, SzConfigTool_getAbiVersion());

    // Test 8: NUL-terminated, thread-local last error
    printf("\n7. Checking last-error termination and thread isolation...\n");
    struct SzConfigTool_result result5 = SzConfigTool_addDataSource(NULL, "X");
    ASSERT(result5.returnCode == -1, "NULL config should return -1");
    ASSERT(strcmp(SzConfigTool_getLastError(), "Null pointer provided") == 0,
           "last error should be exactly the NUL-terminated message");

    int other_clean = run_other_thread();
    ASSERT(other_clean >= 0, "create/join a second thread");
    ASSERT(other_clean == 1, "another thread must not see this thread's error");
    ASSERT(SzConfigTool_getLastErrorCode() == -1, "this thread's error must survive");
    printf("   ✓ error is NUL-terminated and per-thread\n");

    // Test 9: dynamic invoke
    printf("\n8. Testing SzConfigTool_invoke...\n");
    struct SzConfigTool_result inv = SzConfigTool_invoke(
        "add_data_source", initial_config, "{\"code\":\"INV_DS\",\"id\":1500}");
    ASSERT(inv.returnCode == 0, "invoke add_data_source should return 0");
    ASSERT(strstr(inv.response, "\"kind\":\"config\"") != NULL, "envelope kind is config");
    ASSERT(strstr(inv.response, "INV_DS") != NULL, "envelope carries the new config");
    SzConfigTool_free(inv.response);

    struct SzConfigTool_result bad = SzConfigTool_invoke("no_such_function", initial_config, NULL);
    ASSERT(bad.returnCode == -2 && bad.response == NULL, "unknown function should return -2");
    ASSERT(strcmp(SzConfigTool_getLastErrorReasonCode(), "INVALID_INPUT") == 0,
           "unknown function reason code is INVALID_INPUT");

    bad = SzConfigTool_invoke("list_data_sources", initial_config, "{oops");
    ASSERT(bad.returnCode == -2, "bad args JSON should return -2");
    ASSERT(strcmp(SzConfigTool_getLastErrorReasonCode(), "INVALID_INPUT") == 0,
           "bad args JSON reason code is INVALID_INPUT");
    printf("   ✓ invoke envelope and error reason codes\n");

    // Test 10: deleteGenericThreshold honours its plan argument
    printf("\n9. Testing SzConfigTool_deleteGenericThreshold plan selection...\n");
    const char *gt_config =
        "{\"G2_CONFIG\":{"
        "\"CFG_GPLAN\":[{\"GPLAN_ID\":1,\"GPLAN_CODE\":\"INGEST\"},"
        "{\"GPLAN_ID\":2,\"GPLAN_CODE\":\"SEARCH\"}],"
        "\"CFG_FTYPE\":[],"
        "\"CFG_GENERIC_THRESHOLD\":["
        "{\"GPLAN_ID\":1,\"BEHAVIOR\":\"NAME\",\"FTYPE_ID\":0},"
        "{\"GPLAN_ID\":2,\"BEHAVIOR\":\"NAME\",\"FTYPE_ID\":0}]}}";
    struct SzConfigTool_result gt =
        SzConfigTool_deleteGenericThreshold(gt_config, "SEARCH", "NAME", NULL);
    ASSERT(gt.returnCode == 0, "deleteGenericThreshold(SEARCH) should return 0");
    ASSERT(strstr(gt.response, "{\"GPLAN_ID\":1,\"BEHAVIOR\":\"NAME\"") != NULL,
           "deleteGenericThreshold(SEARCH) must keep the INGEST row");
    ASSERT(strstr(gt.response, "{\"GPLAN_ID\":2,\"BEHAVIOR\":\"NAME\"") == NULL,
           "deleteGenericThreshold(SEARCH) must remove the SEARCH row");
    SzConfigTool_free(gt.response);

    gt = SzConfigTool_deleteGenericThreshold(gt_config, "NO_SUCH_PLAN", "NAME", NULL);
    ASSERT(gt.returnCode != 0 && gt.response == NULL,
           "deleteGenericThreshold with an unknown plan must fail");
    printf("   ✓ plan argument selects the plan\n");

    printf("\n=== All tests passed! ===\n");
    return 0;
}
