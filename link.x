MEMORY
{
  /* NOTE K = KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x08000000, LENGTH = 128K 
  RAM : ORIGIN = 0x20000000, LENGTH = 32K
}

_stack_start = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS
{
    . = ORIGIN(FLASH);
    .text :
    {
      . = ALIGN(4);
      _stext = .;
      KEEP(*(.isr_vector));
      . = ALIGN(4);
      *(.text)
      *(.text.*)
      *(.rodata)
      *(.rodata.*)
      . = ALIGN(4);
      _etext = .;
    } >FLASH
    .data :{
        . = ALIGN(4);
        _sdata = .;
        *(.data)
        *(.data.*)
        . = ALIGN(4);
        _edata = .;
    } >RAM AT >FLASH
}