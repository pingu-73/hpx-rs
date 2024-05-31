#pragma once

#include <hpx/algorithm.hpp>
#include <hpx/hpx_init.hpp>
#include <hpx/execution.hpp>
#include <hpx/chrono.hpp>
#include <hpx/format.hpp>
#include <hpx/future.hpp>
#include <hpx/hpx_start.hpp>
#include <vector>
#include <cstdint>
#include <hpx/iostream.hpp>
#include <iostream>

template <typename T> T fibonacci(T n);

std::uint64_t fibonacci_hpx(std::uint64_t n);

extern "C" std::int32_t hpx_init();
extern "C" void copy_hpx(const int* src, const int* src_end, int* dest);

extern "C" int hpx_main_rust(int argc, char* argv[]);
