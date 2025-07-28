#include <stdint.h>
#include <stddef.h>
// extern void __sp1_host_write(uint32_t fd, const char* buf, uint32_t len);

// int _write(int fd, const char* buf, int len) {
//     __sp1_host_write(fd, buf, (uint32_t)len);
//     return len;
// }

void *_sbrk(ptrdiff_t incr) {
    extern char   _heap_base, _heap_end;
    static char*  brk = &_heap_base;
    char*         old = brk;
    if (brk + incr > &_heap_end) return (void*)-1;
    brk += incr;
    return old;
}
