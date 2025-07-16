.section .init
.global _start
_start:
  la  sp, _stack_top     # initialise stack
  call main
  j   .                 # sit here if main returns
