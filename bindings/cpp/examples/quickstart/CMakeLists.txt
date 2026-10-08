# Standalone consumer of an installed szconfigtool package:
#   cmake -S . -B build -DCMAKE_PREFIX_PATH=<install prefix> [-DSZCONFIGTOOL_USE_STATIC=ON]
cmake_minimum_required(VERSION 3.20)
project(szconfigtool_quickstart LANGUAGES CXX)

# No version here: this example is built by the in-tree install test of every
# version. A real project pins the minor it was written against, e.g.
# find_package(szconfigtool 4.4 REQUIRED) (SameMinorVersion compatibility).
find_package(szconfigtool REQUIRED)

add_executable(szconfigtool_quickstart quickstart.cpp)
target_link_libraries(szconfigtool_quickstart PRIVATE SzConfigTool::szconfigtool)
