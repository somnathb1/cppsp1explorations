// fib.h  – public header -------------------------------------------------
#pragma once

#ifdef __cplusplus
extern "C" {            // tell the C++ compiler to use a C ABI
#endif

/// Fast iterative Fibonacci (u32 → u64)
unsigned long fib_c(unsigned int n);

#ifdef __cplusplus
}  // extern "C"
#endif
