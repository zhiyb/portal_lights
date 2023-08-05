/* Provides information about the memory layout of the device */
/* This will be provided by the user (see `memory.x`) or by a Board Support Crate */
INCLUDE memory.x

/* Choose AXIM interface for loading bootloader */
REGION_ALIAS("LOAD", BOOTA);
/* Choose AXIM interface for running bootloader */
REGION_ALIAS("EXEC", BOOTA);

INCLUDE link_common.x
