/* crt0.S  - first code the CPU sees
 *
 *  - sets sp to the top of RAM (symbol _estack)
 *  - jumps straight to your real C++ entry
 *
 *  Build with the same cross-compiler you use for the rest:
 *      riscv32-unknown-elf-gcc -c -march=rv32im -mabi=ilp32 crt0.S
 */

    .section .text._start        /* keep it first in the image             */
    .globl  _start
_start:
    la  sp, _estack              /* stack grows downward from _estack      */
    call main     /* never returns; main() must ebreak/exit */
