cmake_minimum_required(VERSION 3.10)

project(pathtracing)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED True)

FILE(GLOB sources ${CMAKE_SOURCE_DIR}/src/*.cpp)

add_executable(pathtracing ${sources})

set(CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake_modules" ${CMAKE_MODULE_PATH})

# Detect and add SFML
find_package(SFML 2 REQUIRED system window graphics)
include_directories(${SFML_INCLUDE_DIR})
target_link_libraries(pathtracing ${SFML_LIBRARIES} ${SFML_DEPENDENCIES})
