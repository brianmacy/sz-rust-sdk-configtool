# cmake -P: install the build tree to WORK_DIR/prefix, configure + build
# CONSUMER_SRC against it with find_package(szconfigtool), run it on FIXTURE.
file(REMOVE_RECURSE "${WORK_DIR}")
function(run)
    execute_process(COMMAND ${ARGN} RESULT_VARIABLE rc)
    if(NOT rc EQUAL 0)
        message(FATAL_ERROR "failed (${rc}): ${ARGN}")
    endif()
endfunction()
# CONFIG is the parent build's configuration ($<CONFIG>); multi-config
# generators (Visual Studio) need it for --install/--build and put the
# executable under build/<CONFIG>/.
if(NOT CONFIG)
    set(CONFIG Release)
endif()
run("${CMAKE_COMMAND}" --install "${BUILD_DIR}" --config "${CONFIG}" --prefix "${WORK_DIR}/prefix")
run("${CMAKE_COMMAND}" -S "${CONSUMER_SRC}" -B "${WORK_DIR}/build"
    "-DCMAKE_PREFIX_PATH=${WORK_DIR}/prefix" "-DCMAKE_CXX_COMPILER=${CXX}"
    "-DCMAKE_BUILD_TYPE=${CONFIG}" "-DSZCONFIGTOOL_USE_STATIC=${USE_STATIC}")
run("${CMAKE_COMMAND}" --build "${WORK_DIR}/build" --config "${CONFIG}")
file(GLOB_RECURSE exe LIST_DIRECTORIES false
     "${WORK_DIR}/build/szconfigtool_quickstart" "${WORK_DIR}/build/szconfigtool_quickstart.exe")
list(FILTER exe EXCLUDE REGEX "/CMakeFiles/")
list(LENGTH exe n)
if(NOT n EQUAL 1)
    message(FATAL_ERROR "expected one built szconfigtool_quickstart, found: ${exe}")
endif()
if(WIN32 AND NOT USE_STATIC)
    # The shared consumer needs the installed DLL next to it (or on PATH).
    get_filename_component(exe_dir "${exe}" DIRECTORY)
    file(COPY "${WORK_DIR}/prefix/bin/SzConfigTool.dll" DESTINATION "${exe_dir}")
endif()
run("${exe}" "${FIXTURE}")
