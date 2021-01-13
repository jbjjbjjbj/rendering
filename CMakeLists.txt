cmake_minimum_required(VERSION 3.13)

project(pathtracing)

# cmake options
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_AUTOMOC ON)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Compiler options
set(CMAKE_CXX_FLAGS "-Wall -Wextra")
set(CMAKE_CXX_FLAGS_DEBUG "-g")
set(CMAKE_CXX_FLAGS_RELEASE "-O3")

FILE(GLOB app_sources ${CMAKE_SOURCE_DIR}/app/*.cpp)
SET(sources
    src/core/ray.cpp
    src/core/vector.cpp
    src/core/spectrum.cpp
    src/object.cpp
    src/render.cpp
    src/scene.cpp
    )
list(TRANSFORM sources PREPEND ${CMAKE_SOURCE_DIR}/)
FILE(GLOB test_sources ${CMAKE_SOURCE_DIR}/test/*.cpp)

add_executable(pathtracing ${sources} ${app_sources})
target_include_directories(pathtracing PUBLIC src)

# Catch2 unittests
add_custom_target(test)
add_executable(run_test EXCLUDE_FROM_ALL ${sources} ${test_sources})
target_include_directories(run_test PUBLIC src)
add_dependencies(test run_test)

# Qt
find_package(Qt5 COMPONENTS Widgets REQUIRED)
target_link_libraries(pathtracing Qt5::Widgets)
