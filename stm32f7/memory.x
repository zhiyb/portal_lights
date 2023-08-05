MEMORY
{
  ITCM(xrw)	: ORIGIN = 0x00000000, LENGTH = 16k
  DTCM(xrw)	: ORIGIN = 0x20000000, LENGTH = 64k
  RAM(xrw)	: ORIGIN = 0x20010000, LENGTH = 176k
  DMARAM(xrw)	: ORIGIN = 0x2003c000, LENGTH = 16k

  FLASH(xr)	: ORIGIN = 0x08000000, LENGTH = 512k
  BOOT(xr)	: ORIGIN = 0x08000000, LENGTH = 16k
  CONF(xr)	: ORIGIN = 0x08004000, LENGTH = 48k
  LOADER(xr)	: ORIGIN = 0x08010000, LENGTH = 64k
  PROGI(xr)	: ORIGIN = 0x00220000, LENGTH = 384k
  PROG(xr)	: ORIGIN = 0x08020000, LENGTH = 384k
}

/*
REGION_ALIAS("FLASH", PROG);
REGION_ALIAS("RAM", RAM);
*/

/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* You may want to use this variable to locate the call stack and static
   variables in different memory regions. Below is shown the default value */
/* _stack_start = ORIGIN(RAM) + LENGTH(RAM); */

/* You can use this symbol to customize the location of the .text section */
/* If omitted the .text section will be placed right after the .vector_table
   section */
/* This is required only on microcontrollers that store some configuration right
   after the vector table */
/* _stext = ORIGIN(FLASH) + 0x400; */

/* Example of putting non-initialized variables into custom RAM locations. */
/* This assumes you have defined a region RAM2 above, and in the Rust
   sources added the attribute `#[link_section = ".ram2bss"]` to the data
   you want to place there. */
/* Note that the section will not be zero-initialized by the runtime! */
/* SECTIONS {
     .ram2bss (NOLOAD) : ALIGN(4) {
       *(.ram2bss);
       . = ALIGN(4);
     } > RAM2
   } INSERT AFTER .bss;
*/
