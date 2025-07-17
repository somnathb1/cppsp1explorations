// fib.c  – implementation ----------------------------------------------
// #include "fib.h"

// unsigned int fib_c(unsigned int n) {
//     if (n <= 1) return n;
//     unsigned int a = 0, b = 1;
//     for (unsigned int i = 2; i <= n; ++i) {
//         unsigned int next = a + b;
//         a = b;
//         b = next;
//     }
//     return b;
// }


// src/fib.cpp
#include "fib.h"

std::uint64_t fib_cxx(std::uint32_t n) {
    if (n <= 1) return n;
    std::uint64_t a = 0, b = 1;
    for (std::uint32_t i = 2; i <= n; ++i) {
        std::uint64_t next = a + b;
        a = b;
        b = next;
    }
    return b;
}
