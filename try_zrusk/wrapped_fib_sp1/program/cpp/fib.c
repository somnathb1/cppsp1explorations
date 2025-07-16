// fib.c  – implementation ----------------------------------------------
#include "fib.h"

unsigned int fib_c(unsigned int n) {
    if (n <= 1) return n;
    unsigned int a = 0, b = 1;
    for (unsigned int i = 2; i <= n; ++i) {
        unsigned int next = a + b;
        a = b;
        b = next;
    }
    return b;
}
