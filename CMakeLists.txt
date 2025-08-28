cmake_minimum_required(VERSION 3.13)
project(libsimulo LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)

add_library(libsimulo simulo.cc)

target_include_directories(libsimulo PUBLIC glm)
target_include_directories(libsimulo PUBLIC include)