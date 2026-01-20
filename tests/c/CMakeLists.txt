# CMakeLists.txt for libSzConfigTool C tests
# Follows Senzing SDK conventions

cmake_minimum_required(VERSION 3.15)
project(sz_configtool_c_tests VERSION 0.1.0 LANGUAGES C)

# C standard
set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)

# Compiler warnings
if(CMAKE_C_COMPILER_ID MATCHES "GNU|Clang")
    add_compile_options(-Wall -Wextra -Wpedantic)
endif()

# Library paths (relative to this CMakeLists.txt)
set(LIB_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../../target/release")
set(INCLUDE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../../include")

# Find the Senzing-style library (libSzConfigTool)
find_library(SZCONFIGTOOL_LIB
    NAMES SzConfigTool
    PATHS ${LIB_DIR}
    REQUIRED
    NO_DEFAULT_PATH
)

if(NOT SZCONFIGTOOL_LIB)
    message(FATAL_ERROR "libSzConfigTool not found in ${LIB_DIR}")
else()
    message(STATUS "Found libSzConfigTool: ${SZCONFIGTOOL_LIB}")
endif()

# Test executable
add_executable(test_basic
    test_basic.c
)

target_include_directories(test_basic PRIVATE
    ${INCLUDE_DIR}
)

target_link_libraries(test_basic PRIVATE
    ${SZCONFIGTOOL_LIB}
)

# Set RPATH for macOS/Linux so executable can find dylib/so at runtime
if(APPLE)
    set_target_properties(test_basic PROPERTIES
        BUILD_RPATH "${LIB_DIR}"
        INSTALL_RPATH "${LIB_DIR}"
    )
elseif(UNIX)
    set_target_properties(test_basic PROPERTIES
        BUILD_RPATH "${LIB_DIR}"
        INSTALL_RPATH "${LIB_DIR}"
    )
endif()

# Add test to CTest
enable_testing()
add_test(NAME test_basic COMMAND test_basic)

# Custom target to run tests
add_custom_target(run_tests
    COMMAND ${CMAKE_CTEST_COMMAND} --output-on-failure
    DEPENDS test_basic
    COMMENT "Running C tests..."
)

# Print configuration
message(STATUS "Configuration:")
message(STATUS "  Library directory: ${LIB_DIR}")
message(STATUS "  Include directory: ${INCLUDE_DIR}")
message(STATUS "  C standard: C${CMAKE_C_STANDARD}")
