cmake_minimum_required(VERSION 3.19)
project(hpx-rs CXX)

find_package(HPX REQUIRED)

add_library(hpx-rs src/hpx_rs_main.cc)
target_include_directories(hpx-rs PRIVATE include/hpx_rs_defs.h)
target_link_libraries(hpx-rs HPX::hpx HPX::wrap_main HPX::iostreams_component)