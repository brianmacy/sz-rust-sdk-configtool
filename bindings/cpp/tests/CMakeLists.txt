# googletest: FetchContent (pinned, as in brianmacy/sz-cpp-sdk) unless
# SZCONFIGTOOL_USE_SYSTEM_GTEST selects an installed GTest package.
option(SZCONFIGTOOL_USE_SYSTEM_GTEST "Use find_package(GTest) instead of FetchContent" OFF)
if(SZCONFIGTOOL_USE_SYSTEM_GTEST)
    find_package(GTest REQUIRED)
else()
    include(FetchContent)
    FetchContent_Declare(googletest
        GIT_REPOSITORY https://github.com/google/googletest.git
        GIT_TAG v1.15.2)
    set(INSTALL_GTEST OFF CACHE BOOL "" FORCE)
    set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
    FetchContent_MakeAvailable(googletest)
endif()
include(GoogleTest)

set(SZCONFIGTOOL_TEST_SOURCES
    test_core.cpp
    test_api.cpp
    test_threads.cpp
    test_conformance.cpp)

# ASan's leak checker is not supported on macOS/arm64; UBSan aborts on error
# (-fno-sanitize-recover=all), so any report fails the test.
set(SZCONFIGTOOL_TEST_ENV
    "ASAN_OPTIONS=detect_stack_use_after_return=1:strict_string_checks=1:abort_on_error=1"
    "UBSAN_OPTIONS=print_stacktrace=1:halt_on_error=1")

function(szconfigtool_test_exe name iface)
    add_executable(${name} ${SZCONFIGTOOL_TEST_SOURCES})
    szconfigtool_harden(${name})
    # Windows has no rpath: a test linked to the DLL must find it next to the exe.
    # This post-build step is registered BEFORE gtest_discover_tests, which runs
    # the executable at build time (exit 0xc0000135 = DLL not found otherwise).
    if(WIN32 AND ARGC GREATER 2 AND ARGV2 STREQUAL "SHARED")
        add_custom_command(TARGET ${name} POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy_if_different
                    "${SZCONFIGTOOL_SHARED_LIBRARY}" "$<TARGET_FILE_DIR:${name}>")
    endif()
    target_include_directories(${name} PRIVATE "${CMAKE_CURRENT_SOURCE_DIR}")
    target_compile_definitions(${name} PRIVATE
        SZCONFIGTOOL_TEST_WORKSPACE_ROOT="${SZCONFIGTOOL_REPO_ROOT}"
        SZCONFIGTOOL_TEST_PROJECT_VERSION="${SZCONFIGTOOL_FULL_VERSION}")
    target_link_libraries(${name} PRIVATE ${iface} GTest::gtest_main)
    gtest_discover_tests(${name} TEST_PREFIX "${name}." DISCOVERY_TIMEOUT 120
        PROPERTIES ENVIRONMENT "${SZCONFIGTOOL_TEST_ENV}")
endfunction()

if(SZCONFIGTOOL_SHARED_LIBRARY)
    szconfigtool_interface(szconfigtool_shared_for_tests szconfigtool_native_shared)
    szconfigtool_test_exe(szconfigtool_tests szconfigtool_shared_for_tests SHARED)
endif()
if(SZCONFIGTOOL_STATIC_LIBRARY)
    szconfigtool_interface(szconfigtool_static_for_tests szconfigtool_native_static)
    szconfigtool_test_exe(szconfigtool_tests_static szconfigtool_static_for_tests)
endif()

set(_szct_fixture_rel "")
file(READ "${SZCONFIGTOOL_REPO_ROOT}/api/manifest/project.yaml" _szct_project)
string(REGEX MATCH "\n  fixture: ([^\n]+)" _szct_m "${_szct_project}")
set(_szct_fixture "${SZCONFIGTOOL_REPO_ROOT}/${CMAKE_MATCH_1}")

if(SZCONFIGTOOL_BUILD_EXAMPLES)
    add_test(NAME example.quickstart
             COMMAND szconfigtool_quickstart "${_szct_fixture}")
    set_tests_properties(example.quickstart PROPERTIES ENVIRONMENT "${SZCONFIGTOOL_TEST_ENV}")
endif()

# Install into a scratch prefix and build examples/quickstart as an external
# consumer via find_package(szconfigtool) -- shared, then static.
foreach(_mode shared static)
    if(_mode STREQUAL "static" AND NOT SZCONFIGTOOL_STATIC_LIBRARY)
        continue()
    endif()
    if(_mode STREQUAL "shared" AND NOT SZCONFIGTOOL_SHARED_LIBRARY)
        continue()
    endif()
    add_test(NAME install.consumer_${_mode}
        COMMAND "${CMAKE_COMMAND}"
            -DBUILD_DIR=${PROJECT_BINARY_DIR}
            -DWORK_DIR=${CMAKE_CURRENT_BINARY_DIR}/consumer_${_mode}
            -DCONSUMER_SRC=${PROJECT_SOURCE_DIR}/examples/quickstart
            -DFIXTURE=${_szct_fixture}
            -DUSE_STATIC=$<IF:$<STREQUAL:${_mode},static>,ON,OFF>
            -DCXX=${CMAKE_CXX_COMPILER}
            -DCONFIG=$<CONFIG>
            -P "${CMAKE_CURRENT_SOURCE_DIR}/install_consumer.cmake")
endforeach()
