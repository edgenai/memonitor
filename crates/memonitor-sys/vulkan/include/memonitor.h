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

/**
 * A list of physical devices.
 */
struct Devices {
    /// Internal handle to this list.
    void *handle;
    /// The number of devices within this list.
    uint32_t count;
};

/**
 * Creates a list containing all physical devices found within the Vulkan context.
 *
 * @return The list of devices found.
 */
struct Devices list_devices();

/**
 * Destroys a `Devices` object and frees its used memory
 *
 * @param devices the object to destroy
 */
void destroy_devices(struct Devices *devices);

/// A reference to a physical device retrieved from a `Devices` object.
struct DeviceRef {
    /// The internal handle to the physical device.
    void *handle;
};

/**
 * Acquire a reference to a device from within a `Devices` object.
 *
 * @param devices a `Devices` handle from which to get the reference
 * @param index the index of the device within `Devices`
 * @return A reference to the device at the index. If any of the arguments is invalid/null, return an null reference.
 */
struct DeviceRef get_device(struct Devices *devices, uint32_t index);


