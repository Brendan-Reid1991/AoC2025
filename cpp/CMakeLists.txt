cmake_minimum_required(VERSION 3.15)
project(aoc2025 VERSION 1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

include_directories(include)

add_executable(day1 src/day1.cpp)
add_executable(day3 src/day3.cpp)
