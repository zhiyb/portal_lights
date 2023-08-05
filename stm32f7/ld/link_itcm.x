/* Provides information about the memory layout of the device */
/* This will be provided by the user (see `memory.x`) or by a Board Support Crate */
INCLUDE memory.x

/* Choose AXIM interface for loading application */
REGION_ALIAS("LOAD", PROGA);
/* Choose ITCM interface for running application */
REGION_ALIAS("EXEC", PROGI);

INCLUDE link_common.x
