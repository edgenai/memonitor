#pragma once

/**
 * Initializes the global context.
 *
 * Must be called before all other functions. Must be called again after `term_vk` is called before using other
 * functions.
 *
 * @return 0 on success, otherwise the error code.
 */
int init_vk();

/**
 * Destroys the global context and frees all allocations.
 *
 * It isn't necessary to call this before program exit, but it is if the context will be created again.
 */
void term_vk();

