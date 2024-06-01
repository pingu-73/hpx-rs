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

inline std::int32_t start() { return hpx::start(nullptr, 0, nullptr); }

inline std::int32_t stop() {
  hpx::post([]() { hpx::finalize(); });
  return hpx::stop();
}

extern "C" std::int32_t hpx_init();

extern "C" int hpx_main_rust(int argc, char* argv[]);

template <typename T> T fibonacci(T n);

std::uint64_t fibonacci_hpx(std::uint64_t n);

extern "C" void copy_hpx(const int* src, const int* src_end, int* dest);