#pragma once

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Initializes the global context.
 *
 * Must be called before all other functions. Must be called again after `vk_term` is called before using other
 * functions.
 *
 * @return 0 on success, otherwise the error code.
 */
int vk_init();

/**
 * Destroys the global context and frees all allocations.
 *
 * It isn't necessary to call this before program exit, but it is if the context will be created again.
 */
void vk_term();

/**
 * A list of physical devices.
 */
struct vk_Devices {
    /// Internal handle to this list.
    void *handle;
    /// A vector containing the index of the device's local heap.
    uint32_t *local_heaps;
    /// The number of devices within this list.
    uint32_t count;
};

/**
 * Creates a list containing all physical devices found within the Vulkan context.
 *
 * @return The list of devices found.
 */
struct vk_Devices vk_list_devices();

/**
 * Destroys a `vk_Devices` object and frees its used memory
 *
 * @param devices the object to destroy
 */
void vk_destroy_devices(struct vk_Devices *devices);

/// A reference to a physical device retrieved from a `vk_Devices` object.
struct vk_DeviceRef {
    /// The internal handle to the physical device.
    void *handle;
    /// The index of this device's local heap.
    uint32_t local_heap;
};

/**
 * Acquire a reference to a device from within a `vk_Devices` object.
 *
 * @param devices a `vk_Devices` handle from which to get the reference
 * @param index the index of the device within `vk_Devices`
 * @return A reference to the device at the index. If any of the arguments is invalid/null, return an null reference.
 */
struct vk_DeviceRef vk_get_device(struct vk_Devices *devices, uint32_t index);

/**
 * The hardware type of a device.
 */
enum vk_DeviceKind {
    IntegratedGPU,
    DiscreteGPU,
    VirtualGPU,
    CPU,
    Other,
};

/**
 * Properties of a device.
 */
struct vk_DeviceProperties {
    /// The name of this device.
    char name[256U];
    /// The hardware type of this device.
    enum vk_DeviceKind kind;
};

/**
 * Get the properties of the provided device.
 *
 * @param device the device to get properties from
 * @return The device properties.
 */
struct vk_DeviceProperties vk_device_properties(struct vk_DeviceRef device);

/**
 * Memory information of a device at one point in time.
 */
struct vk_DeviceMemoryProperties {
    /// How much memory is available to this device at any given moment (includes used memory).
    size_t budget;
    /// How much memory this process is currently using from the device.
    size_t used;
};

/**
 * Query local memory information of the provided device at this point in time.
 *
 * @param device the device to be queried
 * @return The device's memory information.
 */
struct vk_DeviceMemoryProperties vk_device_memory_properties(struct vk_DeviceRef device);

#ifdef __cplusplus
}
#endif // __cplusplus
