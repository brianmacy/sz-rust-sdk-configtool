# C ABI release gate: compiles ffi/tests/c/test_basic.c and
# ffi/examples/c_ffi_example.c against a STAGED libSzConfigTool (the exact
# files that get archived), linking the shared library and the static archive,
# and runs them with CTest. Driven by packaging/gates/run-c-tests.sh.
#
#   -DSZ_NATIVE_C_DIR=<dist>/<target>/native/c   (include/, lib/, bin/, native-static-libs.txt)
#   -DSZ_REPO_ROOT=<repo>
cmake_minimum_required(VERSION 3.20)
project(sz_configtool_release_ctest LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
if(NOT SZ_NATIVE_C_DIR OR NOT SZ_REPO_ROOT)
    message(FATAL_ERROR "SZ_NATIVE_C_DIR and SZ_REPO_ROOT are required")
endif()

set(_inc "${SZ_NATIVE_C_DIR}/include")
if(WIN32)
    set(_shared_imp "${SZ_NATIVE_C_DIR}/lib/SzConfigTool.lib")
    set(_shared_dll "${SZ_NATIVE_C_DIR}/bin/SzConfigTool.dll")
    set(_static "${SZ_NATIVE_C_DIR}/lib/SzConfigTool_static.lib")
else()
    set(_static "${SZ_NATIVE_C_DIR}/lib/libSzConfigTool.a")
    if(APPLE)
        set(_shared "${SZ_NATIVE_C_DIR}/lib/libSzConfigTool.dylib")
    else()
        set(_shared "${SZ_NATIVE_C_DIR}/lib/libSzConfigTool.so")
    endif()
endif()

# System libraries the Rust staticlib needs, as rustc reported them
# (-lfoo / foo.lib / -framework X; /defaultlib: entries are implied by MSVC).
file(READ "${SZ_NATIVE_C_DIR}/native-static-libs.txt" _raw)
string(STRIP "${_raw}" _raw)
separate_arguments(_tokens UNIX_COMMAND "${_raw}")
set(_static_deps "")
set(_framework OFF)
foreach(tok IN LISTS _tokens)
    if(_framework)
        list(APPEND _static_deps "-framework ${tok}")
        set(_framework OFF)
    elseif(tok STREQUAL "-framework")
        set(_framework ON)
    elseif(tok MATCHES "^-l(.+)$")
        list(APPEND _static_deps "${CMAKE_MATCH_1}")
    elseif(tok MATCHES "^/defaultlib:")
    elseif(tok MATCHES "\\.lib$")
        list(APPEND _static_deps "${tok}")
    endif()
endforeach()
message(STATUS "static deps: ${_static_deps}")

find_package(Threads REQUIRED)
enable_testing()

function(sz_c_test name source link_kind)
    add_executable(${name} "${SZ_REPO_ROOT}/${source}")
    target_include_directories(${name} PRIVATE "${_inc}")
    if(MSVC)
        # /utf-8: the sources print UTF-8 check marks (no C4819). /W4 without
        # /WX: MSVC warnings are not validated on a Windows host yet.
        target_compile_options(${name} PRIVATE /W4 /utf-8)
    else()
        target_compile_options(${name} PRIVATE -Wall -Wextra -Werror)
    endif()
    target_link_libraries(${name} PRIVATE Threads::Threads)
    if(link_kind STREQUAL "static")
        # The header's static opt-out: without it, Windows compiles the calls
        # as __declspec(dllimport) (__imp_SzConfigTool_*), which the static
        # archive does not define (LNK2019).
        target_compile_definitions(${name} PRIVATE SZCONFIGTOOL_STATIC)
        target_link_libraries(${name} PRIVATE "${_static}" ${_static_deps})
    elseif(WIN32)
        target_link_libraries(${name} PRIVATE "${_shared_imp}")
        add_custom_command(TARGET ${name} POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy_if_different "${_shared_dll}" "$<TARGET_FILE_DIR:${name}>")
    else()
        target_link_libraries(${name} PRIVATE "${_shared}")
        set_target_properties(${name} PROPERTIES BUILD_RPATH "${SZ_NATIVE_C_DIR}/lib")
    endif()
    add_test(NAME ${name} COMMAND ${name})
endfunction()

sz_c_test(example_shared ffi/examples/c_ffi_example.c shared)
sz_c_test(example_static ffi/examples/c_ffi_example.c static)
# test_basic.c: pthreads on Unix, Win32 threads (CreateThread) on Windows.
sz_c_test(test_basic_shared ffi/tests/c/test_basic.c shared)
sz_c_test(test_basic_static ffi/tests/c/test_basic.c static)
