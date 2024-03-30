#include "hpx_rs_defs.h"

template <> std::uint64_t fibonacci(std::uint64_t n) {
  if (n < 2)
    return n;

  std::uint64_t n1 = fibonacci(n - 1);
  std::uint64_t n2 = fibonacci(n - 2);

  return n1 + n2;
}

std::uint64_t fibonacci_hpx(std::uint64_t n) {
  if (n < 2)
    return n;

  hpx::future<std::uint64_t> n1 = hpx::async(fibonacci_hpx, n - 1);
  hpx::future<std::uint64_t> n2 = hpx::async(fibonacci_hpx, n - 2);

  return n1.get() + n2.get(); // wait for the Futures to return their values
}