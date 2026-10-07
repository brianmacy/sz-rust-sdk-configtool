cmake_minimum_required(VERSION 3.20)

# Version = the workspace version (Cargo.toml [workspace.package]) when built
# in the source tree, so the binding and libSzConfigTool always agree.
# SZCONFIGTOOL_FULL_VERSION is the SemVer string (e.g. 4.4.0-1 or 4.5.0-rc.1, what
# LibraryVersion() returns); project() takes only its numeric X.Y.Z.
set(SZCONFIGTOOL_REPO_ROOT "${CMAKE_CURRENT_LIST_DIR}/../.." CACHE PATH
    "sz-rust-sdk-configtool workspace root (in-tree builds)")
set(_szct_version "0.0.0")
set(SZCONFIGTOOL_FULL_VERSION "0.0.0")
if(EXISTS "${SZCONFIGTOOL_REPO_ROOT}/Cargo.toml")
    file(STRINGS "${SZCONFIGTOOL_REPO_ROOT}/Cargo.toml" _szct_vline
         REGEX "^version = \"[0-9]+\\.[0-9]+\\.[0-9]+([-+][0-9A-Za-z.+-]*)?\"" LIMIT_COUNT 1)
    string(REGEX MATCH "\"([^\"]+)\"" _szct_m "${_szct_vline}")
    set(SZCONFIGTOOL_FULL_VERSION "${CMAKE_MATCH_1}")
    string(REGEX MATCH "^[0-9]+\\.[0-9]+\\.[0-9]+" _szct_version "${SZCONFIGTOOL_FULL_VERSION}")
endif()

project(szconfigtool VERSION ${_szct_version} LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# ---- Embedding detection (same policy as brianmacy/sz-cpp-sdk) ----
if(DEFINED PROJECT_IS_TOP_LEVEL)
    set(SZCONFIGTOOL_TOP_LEVEL ${PROJECT_IS_TOP_LEVEL})
elseif(CMAKE_SOURCE_DIR STREQUAL CMAKE_CURRENT_SOURCE_DIR)
    set(SZCONFIGTOOL_TOP_LEVEL ON)
else()
    set(SZCONFIGTOOL_TOP_LEVEL OFF)
endif()

option(SZCONFIGTOOL_BUILD_TESTS "Build the googletest suite" ${SZCONFIGTOOL_TOP_LEVEL})
option(SZCONFIGTOOL_BUILD_EXAMPLES "Build the example program" ${SZCONFIGTOOL_TOP_LEVEL})
option(SZCONFIGTOOL_ENABLE_SANITIZERS
       "Build tests/examples with AddressSanitizer + UndefinedBehaviorSanitizer" OFF)
option(SZCONFIGTOOL_ENABLE_COVERAGE
       "Build tests/examples with clang source-based coverage (packaging/coverage.sh)" OFF)
if(SZCONFIGTOOL_ENABLE_COVERAGE AND NOT CMAKE_CXX_COMPILER_ID MATCHES "Clang")
    message(FATAL_ERROR "SZCONFIGTOOL_ENABLE_COVERAGE needs clang (got ${CMAKE_CXX_COMPILER_ID})")
endif()
option(SZCONFIGTOOL_LINK_STATIC
       "SzConfigTool::szconfigtool links libSzConfigTool.a instead of the shared library" OFF)

# ---- libSzConfigTool (built by `cargo build -p sz-configtool-ffi --release`) ----
if(DEFINED ENV{CARGO_TARGET_DIR})
    set(_szct_target_dir "$ENV{CARGO_TARGET_DIR}")
else()
    set(_szct_target_dir "${SZCONFIGTOOL_REPO_ROOT}/target")
endif()
set(SZCONFIGTOOL_NATIVE_DIR "${_szct_target_dir}/release" CACHE PATH
    "Directory containing libSzConfigTool")
set(SZCONFIGTOOL_C_INCLUDE_DIR "${SZCONFIGTOOL_REPO_ROOT}/ffi/include" CACHE PATH
    "Directory containing libSzConfigTool.h")
if(WIN32)
    # Two layouts share the name SzConfigTool.lib with different meanings:
    #  * cargo target dir: rustc names the DLL import library SzConfigTool.dll.lib
    #    and the staticlib SzConfigTool.lib;
    #  * release archive / install prefix (packaging/build-native.sh): import
    #    library SzConfigTool.lib, staticlib SzConfigTool_static.lib.
    if(EXISTS "${SZCONFIGTOOL_NATIVE_DIR}/SzConfigTool.dll.lib")
        set(_szct_implib_name "SzConfigTool.dll.lib")
        set(_szct_static_name "SzConfigTool.lib")
    else()
        set(_szct_implib_name "SzConfigTool.lib")
        set(_szct_static_name "SzConfigTool_static.lib")
    endif()
    find_file(SZCONFIGTOOL_SHARED_LIBRARY NAMES SzConfigTool.dll
              PATHS "${SZCONFIGTOOL_NATIVE_DIR}" NO_DEFAULT_PATH)
    find_file(SZCONFIGTOOL_IMPORT_LIBRARY NAMES "${_szct_implib_name}"
              PATHS "${SZCONFIGTOOL_NATIVE_DIR}" NO_DEFAULT_PATH)
    find_file(SZCONFIGTOOL_STATIC_LIBRARY NAMES "${_szct_static_name}"
              PATHS "${SZCONFIGTOOL_NATIVE_DIR}" NO_DEFAULT_PATH)
    if(NOT SZCONFIGTOOL_IMPORT_LIBRARY)
        # A DLL without its import library cannot be linked.
        set(SZCONFIGTOOL_SHARED_LIBRARY "SZCONFIGTOOL_SHARED_LIBRARY-NOTFOUND")
    endif()
else()
    find_library(SZCONFIGTOOL_SHARED_LIBRARY NAMES SzConfigTool
                 PATHS "${SZCONFIGTOOL_NATIVE_DIR}" NO_DEFAULT_PATH)
    find_library(SZCONFIGTOOL_STATIC_LIBRARY
                 NAMES "${CMAKE_STATIC_LIBRARY_PREFIX}SzConfigTool${CMAKE_STATIC_LIBRARY_SUFFIX}"
                 PATHS "${SZCONFIGTOOL_NATIVE_DIR}" NO_DEFAULT_PATH)
endif()
# System libraries a Rust staticlib needs (`rustc --print native-static-libs`).
if(APPLE)
    set(_szct_static_deps "")
elseif(WIN32)
    set(_szct_static_deps "ws2_32;userenv;ntdll;bcrypt;advapi32;kernel32")
else()
    set(_szct_static_deps "pthread;dl;m;rt")
endif()
set(SZCONFIGTOOL_STATIC_DEPS "${_szct_static_deps}" CACHE STRING
    "Extra link libraries for libSzConfigTool.a")

if(NOT EXISTS "${SZCONFIGTOOL_C_INCLUDE_DIR}/libSzConfigTool.h")
    message(FATAL_ERROR "libSzConfigTool.h not found in ${SZCONFIGTOOL_C_INCLUDE_DIR}")
endif()
if(SZCONFIGTOOL_LINK_STATIC AND NOT SZCONFIGTOOL_STATIC_LIBRARY)
    message(FATAL_ERROR "libSzConfigTool static archive not found in ${SZCONFIGTOOL_NATIVE_DIR}; "
                        "run: cargo build -p sz-configtool-ffi --release")
endif()
if(NOT SZCONFIGTOOL_LINK_STATIC AND NOT SZCONFIGTOOL_SHARED_LIBRARY)
    message(FATAL_ERROR "libSzConfigTool shared library not found in ${SZCONFIGTOOL_NATIVE_DIR}; "
                        "run: cargo build -p sz-configtool-ffi --release")
endif()

include(GNUInstallDirs)
set(SZCONFIGTOOL_INCLUDE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/include")

if(SZCONFIGTOOL_SHARED_LIBRARY)
    add_library(szconfigtool_native_shared SHARED IMPORTED GLOBAL)
    set_target_properties(szconfigtool_native_shared PROPERTIES
        IMPORTED_LOCATION "${SZCONFIGTOOL_SHARED_LIBRARY}")
    if(WIN32)
        set_target_properties(szconfigtool_native_shared PROPERTIES
            IMPORTED_IMPLIB "${SZCONFIGTOOL_IMPORT_LIBRARY}")
    endif()
endif()
if(SZCONFIGTOOL_STATIC_LIBRARY)
    add_library(szconfigtool_native_static STATIC IMPORTED GLOBAL)
    set_target_properties(szconfigtool_native_static PROPERTIES
        IMPORTED_LOCATION "${SZCONFIGTOOL_STATIC_LIBRARY}"
        INTERFACE_LINK_LIBRARIES "${SZCONFIGTOOL_STATIC_DEPS}"
        INTERFACE_COMPILE_DEFINITIONS SZCONFIGTOOL_STATIC)
endif()

# Header-only interface over one native flavour.
function(szconfigtool_interface name native)
    add_library(${name} INTERFACE)
    target_compile_features(${name} INTERFACE cxx_std_20)
    target_include_directories(${name} INTERFACE
        "$<BUILD_INTERFACE:${SZCONFIGTOOL_INCLUDE_DIR}>"
        "$<BUILD_INTERFACE:${SZCONFIGTOOL_C_INCLUDE_DIR}>"
        "$<INSTALL_INTERFACE:${CMAKE_INSTALL_INCLUDEDIR}>")
    target_link_libraries(${name} INTERFACE ${native})
endfunction()

if(SZCONFIGTOOL_LINK_STATIC)
    szconfigtool_interface(szconfigtool szconfigtool_native_static)
else()
    szconfigtool_interface(szconfigtool szconfigtool_native_shared)
endif()
add_library(SzConfigTool::szconfigtool ALIAS szconfigtool)

# ---- Install / find_package(szconfigtool) ----
include(CMakePackageConfigHelpers)
set(SZCONFIGTOOL_INSTALL_CMAKEDIR "${CMAKE_INSTALL_LIBDIR}/cmake/szconfigtool")
install(DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/include/szconfigtool"
        DESTINATION "${CMAKE_INSTALL_INCLUDEDIR}")
install(FILES "${SZCONFIGTOOL_C_INCLUDE_DIR}/libSzConfigTool.h"
        DESTINATION "${CMAKE_INSTALL_INCLUDEDIR}")
# Installed names (substituted into szconfigtool-config.cmake). Windows always
# installs the release-archive names: bin/SzConfigTool.dll, lib/SzConfigTool.lib
# (import library) and lib/SzConfigTool_static.lib.
set(SZCONFIGTOOL_SHARED_NAME "")
set(SZCONFIGTOOL_IMPLIB_NAME "")
set(SZCONFIGTOOL_STATIC_NAME "")
if(SZCONFIGTOOL_SHARED_LIBRARY AND WIN32)
    set(SZCONFIGTOOL_SHARED_NAME "SzConfigTool.dll")
    set(SZCONFIGTOOL_IMPLIB_NAME "SzConfigTool.lib")
    install(FILES "${SZCONFIGTOOL_SHARED_LIBRARY}" DESTINATION "${CMAKE_INSTALL_BINDIR}")
    install(FILES "${SZCONFIGTOOL_IMPORT_LIBRARY}" DESTINATION "${CMAKE_INSTALL_LIBDIR}"
            RENAME "${SZCONFIGTOOL_IMPLIB_NAME}")
elseif(SZCONFIGTOOL_SHARED_LIBRARY)
    get_filename_component(SZCONFIGTOOL_SHARED_NAME "${SZCONFIGTOOL_SHARED_LIBRARY}" NAME)
    install(FILES "${SZCONFIGTOOL_SHARED_LIBRARY}" DESTINATION "${CMAKE_INSTALL_LIBDIR}")
    if(APPLE)
        # Cargo bakes an absolute build-tree install name; make the installed
        # copy relocatable.
        install(CODE "execute_process(COMMAND install_name_tool -id
            \"@rpath/${SZCONFIGTOOL_SHARED_NAME}\"
            \"\$ENV{DESTDIR}\${CMAKE_INSTALL_PREFIX}/${CMAKE_INSTALL_LIBDIR}/${SZCONFIGTOOL_SHARED_NAME}\"
            COMMAND_ERROR_IS_FATAL ANY)")
    endif()
endif()
if(SZCONFIGTOOL_STATIC_LIBRARY AND WIN32)
    set(SZCONFIGTOOL_STATIC_NAME "SzConfigTool_static.lib")
    install(FILES "${SZCONFIGTOOL_STATIC_LIBRARY}" DESTINATION "${CMAKE_INSTALL_LIBDIR}"
            RENAME "${SZCONFIGTOOL_STATIC_NAME}")
elseif(SZCONFIGTOOL_STATIC_LIBRARY)
    get_filename_component(SZCONFIGTOOL_STATIC_NAME "${SZCONFIGTOOL_STATIC_LIBRARY}" NAME)
    install(FILES "${SZCONFIGTOOL_STATIC_LIBRARY}" DESTINATION "${CMAKE_INSTALL_LIBDIR}")
endif()
configure_package_config_file(
    "${CMAKE_CURRENT_SOURCE_DIR}/cmake/szconfigtool-config.cmake.in"
    "${CMAKE_CURRENT_BINARY_DIR}/szconfigtool-config.cmake"
    INSTALL_DESTINATION "${SZCONFIGTOOL_INSTALL_CMAKEDIR}"
    PATH_VARS CMAKE_INSTALL_INCLUDEDIR CMAKE_INSTALL_LIBDIR CMAKE_INSTALL_BINDIR)
write_basic_package_version_file(
    "${CMAKE_CURRENT_BINARY_DIR}/szconfigtool-config-version.cmake"
    VERSION "${PROJECT_VERSION}"
    COMPATIBILITY SameMinorVersion
    ARCH_INDEPENDENT)
install(FILES "${CMAKE_CURRENT_BINARY_DIR}/szconfigtool-config.cmake"
              "${CMAKE_CURRENT_BINARY_DIR}/szconfigtool-config-version.cmake"
        DESTINATION "${SZCONFIGTOOL_INSTALL_CMAKEDIR}")

# ---- Sanitizers / warnings for our own executables ----
function(szconfigtool_harden target)
    if(MSVC)
        # /W4 without /WX: MSVC warnings are not validated on a Windows host
        # yet, so they are reported but do not fail the build.
        target_compile_options(${target} PRIVATE /W4 /permissive- /utf-8)
    else()
        target_compile_options(${target} PRIVATE -Wall -Wextra -Wpedantic -Werror)
    endif()
    if(SZCONFIGTOOL_ENABLE_SANITIZERS)
        target_compile_options(${target} PRIVATE
            -fsanitize=address,undefined -fno-sanitize-recover=all -fno-omit-frame-pointer)
        target_link_options(${target} PRIVATE -fsanitize=address,undefined)
    endif()
    if(SZCONFIGTOOL_ENABLE_COVERAGE)
        target_compile_options(${target} PRIVATE -fprofile-instr-generate -fcoverage-mapping)
        target_link_options(${target} PRIVATE -fprofile-instr-generate)
    endif()
endfunction()

if(SZCONFIGTOOL_BUILD_EXAMPLES)
    add_executable(szconfigtool_quickstart examples/quickstart/quickstart.cpp)
    szconfigtool_harden(szconfigtool_quickstart)
    target_link_libraries(szconfigtool_quickstart PRIVATE SzConfigTool::szconfigtool)
    if(WIN32 AND SZCONFIGTOOL_SHARED_LIBRARY AND NOT SZCONFIGTOOL_LINK_STATIC)
        add_custom_command(TARGET szconfigtool_quickstart POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy_if_different
                    "${SZCONFIGTOOL_SHARED_LIBRARY}" "$<TARGET_FILE_DIR:szconfigtool_quickstart>")
    endif()
endif()

if(SZCONFIGTOOL_BUILD_TESTS)
    enable_testing()
    add_subdirectory(tests)
endif()
