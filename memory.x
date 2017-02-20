MEMORY
{
  /* TODO You must correct these values */
  FLASH : ORIGIN = 0xBAAAAAAD, LENGTH = 0
  RAM : ORIGIN = 0xBAAAAAAD, LENGTH = 0
}

SECTIONS
{
  .text ORIGIN(FLASH) :
  {
    /* Vector table */
    _VECTOR_TABLE = .;
    LONG(ORIGIN(RAM) + LENGTH(RAM));
    LONG(__reset + 1);

    KEEP(*(.rodata._EXCEPTIONS));
    __exceptions = .;

    KEEP(*(.rodata._INTERRUPTS));
    __interrupts = .;

    /* Entry point: the reset handler */
    __reset = .;
    *(.text.start);

    *(.text.*);

    *(.rodata.*);
    __pre_init_array_start = ALIGN(4);
    KEEP(*(.pre_init_array));
    __pre_init_array_end = ALIGN(4);
    __init_array_start = ALIGN(4);
    KEEP(*(.init_array));
    __init_array_end = ALIGN(4);
  } > FLASH

  .bss : ALIGN(4)
  {
    _sbss = .;
    *(.bss.*);
    _ebss = ALIGN(4);
  } > RAM

  .data : ALIGN(4)
  {
    _sdata = .;
    *(.data.*);
    _edata = ALIGN(4);
  } > RAM AT > FLASH

  _sidata = LOADADDR(.data);

  /DISCARD/ :
  {
    /* Unused unwinding stuff */
    *(.ARM.exidx.*)
  }
}

ASSERT(__exceptions - ORIGIN(FLASH) == 0x40,
       "you must define the _EXCEPTIONS symbol");

ASSERT(__interrupts - __exceptions > 0,
       "you must define the _INTERRUPTS symbol");
