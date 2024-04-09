#include <hpx/chrono.hpp>
#include <hpx/format.hpp>
#include <hpx/future.hpp>
#include <hpx/hpx_start.hpp>

#include <cstdint>
#include <hpx/hpx_init.hpp>
#include <hpx/iostream.hpp>
#include <iostream>

#include "rust/cxx.h"

template <typename T> T fibonacci(T n);

std::uint64_t fibonacci_hpx(std::uint64_t n);

inline std::int32_t start(rust::Fn<int(int, char **)> rust_fn) {
  int argc = 1;
  char *argv[] = {"--hpx:threads=4"};
  return hpx::init([&](int argc, char **argv) { return rust_fn(argc, argv); },
                   argc, argv);
}

inline std::int32_t stop() { return hpx::finalize(); }