// // Put this file somewhere you already `-I`, e.g. src/include/stub_gthread_cond.hpp
// #pragma once
// extern "C" {

// // Dummy types
// typedef int __gthread_cond_t;
// typedef long long __gthread_time_t;
// typedef int __gthread_mutex_t;
// typedef int __gthread_recursive_mutex_t;


// // Dummy functions – all succeed immediately
// static inline int __gthread_cond_init(__gthread_cond_t*, const void*) { return 0; }
// static inline int __gthread_cond_destroy(__gthread_cond_t*)            { return 0; }
// static inline int __gthread_cond_wait(__gthread_cond_t*, void*)        { return 0; }
// static inline int __gthread_cond_signal(__gthread_cond_t*)             { return 0; }
// static inline int __gthread_cond_broadcast(__gthread_cond_t*)          { return 0; }
// static inline int __gthread_cond_timedwait(__gthread_cond_t*,
//                                            void*, const __gthread_time_t*) { return 0; }
// #define __GTHREAD_COND_INIT          0
// #define __GTHREAD_COND_INIT_FUNCTION(cond_ptr)  ((void)0)




// static inline int
// __gthread_mutex_timedlock(__gthread_mutex_t*, const __gthread_time_t*)
// { return 0; }

// static inline int
// __gthread_recursive_mutex_timedlock(__gthread_recursive_mutex_t*,
//                                      const __gthread_time_t*)
// { return 0; }

// } // extern "C"