cmake_minimum_required(VERSION 3.22)

project(cachetest CXX)

configure_file(header.h.in header.h)

add_executable(cachetest header.h main.cpp)

target_include_directories(cachetest PRIVATE ${CMAKE_CURRENT_BINARY_DIR})
