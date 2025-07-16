// fib.cpp  – implementation ----------------------------------------------
#include "fib.h"

extern "C"            // (redundant but harmless here)
unsigned long fib_c(unsigned int n) {
    if (n <= 1) return n;
    unsigned long a = 0, b = 1;
    for (unsigned int i = 2; i <= n; ++i) {
        unsigned long next = a + b;
        a = b;
        b = next;
    }
    return b;
}
