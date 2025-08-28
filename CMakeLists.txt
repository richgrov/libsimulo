cmake_minimum_required(VERSION 3.13)
project(simulo LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)

add_library(simulo simulo.cc)

target_include_directories(simulo PUBLIC glm)
target_include_directories(simulo PUBLIC include)