/* Provides information about the memory layout of the device */
/* This will be provided by the user (see `memory.x`) or by a Board Support Crate */
INCLUDE memory.x

/* Choose AXIM interface for loading application */
REGION_ALIAS("LOAD", PROGA);
/* Choose AXIM interface for running application */
REGION_ALIAS("EXEC", PROGA);

INCLUDE link_common.x
