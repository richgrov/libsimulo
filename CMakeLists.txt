cmake_minimum_required(VERSION 3.13)
project(SimuloWasm LANGUAGES CXX)

set(CMAKE_CXX_FLAGS "")
set(CMAKE_CXX_FLAGS_DEBUG "")
set(CMAKE_CXX_FLAGS_RELEASE "")
set(CMAKE_CXX_FLAGS_RELWITHDEBINFO "")
set(CMAKE_CXX_FLAGS_MINSIZEREL "")
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_executable(main main.cc)

target_include_directories(main PUBLIC glm)
target_include_directories(main PRIVATE include)

target_link_options(main PRIVATE
    "--no-entry"
    "-sSTANDALONE_WASM=1"
)

set_target_properties(main PROPERTIES
    OUTPUT_NAME "main"
    SUFFIX ".wasm"
)
