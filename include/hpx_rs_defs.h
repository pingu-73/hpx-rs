#include <hpx/chrono.hpp>
#include <hpx/format.hpp>
#include <hpx/future.hpp>
#include <hpx/init.hpp>

#include <cstdint>
#include <iostream>

template <typename T> T fibonacci(T n);

std::uint64_t fibonacci_hpx(std::uint64_t n);
