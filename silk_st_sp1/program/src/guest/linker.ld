MEMORY
{
  RAM (rwx) : ORIGIN = 0x00200000, LENGTH = 4M
}

_estack    = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS
{
  .text : {
      KEEP(*(.text._start))
      *(.text*) *(.rodata*)
  } > RAM

  .data : { *(.data*) } > RAM

  .bss  : { *(.bss*)  } > RAM
  PROVIDE(_end = .);
  PROVIDE(end  = .);
}


ENTRY(_start)
