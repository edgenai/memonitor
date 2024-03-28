#pragma once

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Initializes the global context.
 *
 * Must be called before all other functions. Must be called again after `cu_term` is called before using other
 * functions.
 *
 * @return 0 on success, otherwise the error code.
 */
int cu_init();

/**
 * Destroys the global context and frees all allocations.
 *
 * It isn't necessary to call this before program exit, but it is if the context will be created again.
 */
void cu_term();

/**
 * A list of physical devices.
 */
struct cu_Devices {
    /// Internal handle to the device list.
    void *devices_handle;
    /// Internal handle to the context list.
    void *ctx_handle;
    /// The number of devices within this list.
    uint32_t count;
};

/**
 * Creates a list containing all physical devices found within the Vulkan context.
 *
 * @return The list of devices found.
 */
struct cu_Devices cu_list_devices();

/**
 * Destroys a `cu_Devices` object and frees its used memory
 *
 * @param devices the object to destroy
 */
void cu_destroy_devices(struct cu_Devices *devices);

/// A reference to a physical device retrieved from a `cu_Devices` object.
struct cu_DeviceRef {
    /// The internal handle to the physical device.
    void *handle;
    /// The internal handle to the device context.
    void *ctx_handle;
};

/**
 * Acquire a reference to a device from within a `cu_Devices` object.
 *
 * @param devices a `cu_Devices` handle from which to get the reference
 * @param index the index of the device within `cu_Devices`
 * @return A reference to the device at the index. If any of the arguments is invalid/null, return an null reference.
 */
struct cu_DeviceRef cu_get_device(struct cu_Devices *devices, uint32_t index);

/**
 * The hardware type of a device.
 */
enum cu_DeviceKind {
    /// A Graphics Card physically integrated into the CPU (probably sharing the same memory).
    IntegratedGPU,
    /// A discrete Graphics Card, probably connected through PCIE.
    DiscreteGPU,
    /// Some other, unknown type.
    Other,
};

/**
 * Properties of a device.
 */
struct cu_DeviceProperties {
    /// The name of this device.
    char name[256U];
    /// The hardware type of this device.
    enum cu_DeviceKind kind;
    /// The total amount of local memory for this device.
    size_t total_memory;
};

/**
 * Get the properties of the provided device.
 *
 * @param device the device to get properties from
 * @return The device properties.
 */
struct cu_DeviceProperties cu_device_properties(struct cu_DeviceRef device);

/**
 * Memory information of a device at one point in time.
 */
struct cu_DeviceMemoryProperties {
    /// How much memory is available to this device at a given moment.
    size_t budget;
    /// How much memory is currently getting used from the device.
    size_t used;
};

/**
 * Query local memory information of the provided device at this point in time.
 *
 * @param device the device to be queried
 * @return The device's memory information.
 */
struct cu_DeviceMemoryProperties cu_device_memory_properties(struct cu_DeviceRef device);

#ifdef __cplusplus
}
#endif // __cplusplus
