#include <stdlib.h>
#include <string.h>

#include <volk.h>

#include <memonitor.h>

#define ARRAY_LEN(array) sizeof(array) / sizeof(array[0])

#ifdef USE_VALIDATION_LAYERS
const char *layer_names[1] = {"VK_LAYER_KHRONOS_validation"};
#else
const char *layer_names[0] = {};
#endif // USE_VALIDATION_LAYERS

const char *extension_names[1] = {"VK_KHR_get_physical_device_properties2"};

/**
* Check if the required layers are supported locally.
*
* @return `VK_SUCCESS` if they are supported, `VK_RESULT_MAX_ENUM` if they are not and anything else if an error
* occurred.
*/
int layer_support() {
    uint32_t count = 0;
    VkResult res = vkEnumerateInstanceLayerProperties(&count, NULL);
    if (res != VK_SUCCESS) {
        return res;
    }
    VkLayerProperties *properties = malloc(sizeof(VkLayerProperties) * count);
    res = vkEnumerateInstanceLayerProperties(&count, properties);
    if (res != VK_SUCCESS) {
        free(properties);
        return res;
    }

    for (uint32_t l = 0; l < ARRAY_LEN(layer_names); l++) {
        int layer_found = 0;

        for (uint32_t i = 0; i < count; i++) {
            if (!strcmp(layer_names[l], properties[i].layerName)) {
                layer_found = 1;
                break;
            }
        }

        if (!layer_found) {
            free(properties);
            return VK_RESULT_MAX_ENUM;
        }
    }

    free(properties);
    return VK_SUCCESS;
}

/**
 * Check if the required extensions are supported locally.
 *
 * @return `VK_SUCCESS` if they are supported, `VK_RESULT_MAX_ENUM` if they are not and anything else if an error
 * occurred.
 */
int extension_support() {
    uint32_t count = 0;
    VkResult res = vkEnumerateInstanceExtensionProperties(NULL, &count, NULL);
    if (res != VK_SUCCESS) {
        return res;
    }
    VkExtensionProperties *properties = malloc(sizeof(VkExtensionProperties) * count);
    res = vkEnumerateInstanceExtensionProperties(NULL, &count, properties);
    if (res != VK_SUCCESS) {
        free(properties);
        return res;
    }

    for (uint32_t e = 0; e < ARRAY_LEN(extension_names); e++) {
        int extension_found = 0;

        for (uint32_t i = 0; i < count; i++) {
            if (!strcmp(extension_names[e], properties[i].extensionName)) {
                extension_found = 1;
                break;
            }
        }

        if (!extension_found) {
            free(properties);
            return VK_RESULT_MAX_ENUM;
        }
    }

    free(properties);
    return VK_SUCCESS;
}

int vk_init() {
    VkResult res = volkInitialize();
    if (res != VK_SUCCESS) {
        return res;
    }

    res = layer_support();
    if (res != VK_SUCCESS) {
        volkFinalize();
        return res;
    }
    res = extension_support();
    if (res != VK_SUCCESS) {
        volkFinalize();
        return res;
    }

    VkApplicationInfo app_info = {
            VK_STRUCTURE_TYPE_APPLICATION_INFO,
            NULL,
            "memonitor",
            1,
            NULL,
            0,
            VK_API_VERSION_1_1
    };

    VkInstanceCreateInfo instance_info = {
            VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            NULL,
            0,
            &app_info,
            ARRAY_LEN(layer_names),
            layer_names,
            ARRAY_LEN(extension_names),
            extension_names
    };

    VkInstance instance;
    res = vkCreateInstance(&instance_info, NULL, &instance);
    if (res != VK_SUCCESS) {
        volkFinalize();
        return res;
    }

    volkLoadInstanceOnly(instance);
    return VK_SUCCESS;
}

void vk_term() {
    volkFinalize();
}

struct vk_Devices vk_list_devices() {
    const struct vk_Devices invalid_devices = {NULL, 0};

    VkInstance instance = volkGetLoadedInstance();
    if (instance == VK_NULL_HANDLE) {
        return invalid_devices;
    }

    uint32_t count = 0;
    VkResult res = vkEnumeratePhysicalDevices(instance, &count, NULL);
    if (res != VK_SUCCESS) {
        return invalid_devices;
    }
    VkPhysicalDevice *device_handles = malloc(sizeof(VkPhysicalDevice) * count);
    res = vkEnumeratePhysicalDevices(instance, &count, device_handles);
    if (res != VK_SUCCESS) {
        free(device_handles);
        return invalid_devices;
    }

    uint32_t *heap_indexes = calloc(count, sizeof(uint32_t));
    for (uint32_t d = 0; d < count; d++) {
        VkPhysicalDeviceMemoryProperties2 memory_properties = {};
        memory_properties.sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2;
        vkGetPhysicalDeviceMemoryProperties2(device_handles[d], &memory_properties);
        for (uint32_t i = 0; i < memory_properties.memoryProperties.memoryHeapCount; i++) {
            VkMemoryHeapFlags flags = memory_properties.memoryProperties.memoryHeaps[i].flags;
            if (flags == VK_MEMORY_HEAP_DEVICE_LOCAL_BIT) {
                heap_indexes[d] = i;
                break;
            }
        }
    }

    struct vk_Devices devices = {device_handles, heap_indexes, count};
    return devices;
}

void vk_destroy_devices(struct vk_Devices *devices) {
    if (devices && devices->handle && devices->count) {
        free(devices->handle);
        devices->handle = NULL;
        free(devices->local_heaps);
        devices->local_heaps = NULL;
        devices->count = 0;
    }
}

struct vk_DeviceRef vk_get_device(struct vk_Devices *devices, uint32_t index) {
    const struct vk_DeviceRef invalid_ref = {NULL};
    if (!devices || !devices->handle || !devices->count || devices->count <= index) {
        return invalid_ref;
    }

    VkPhysicalDevice *cast_devices = devices->handle;
    struct vk_DeviceRef ref = {cast_devices[index], devices->local_heaps[index]};
    return ref;
}

struct vk_DeviceProperties vk_device_properties(struct vk_DeviceRef device) {
    const struct vk_DeviceProperties invalid_properties = {{}, Other};
    if (!device.handle) {
        return invalid_properties;
    }

    VkPhysicalDevice cast_device = device.handle;
    VkPhysicalDeviceProperties2 properties = {};
    properties.sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2;
    vkGetPhysicalDeviceProperties2(cast_device, &properties);
    enum vk_DeviceKind kind = Other;
    switch (properties.properties.deviceType) {
        case VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU:
            kind = IntegratedGPU;
            break;
        case VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU:
            kind = DiscreteGPU;
            break;
        case VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU:
            kind = VirtualGPU;
            break;
        case VK_PHYSICAL_DEVICE_TYPE_CPU:
            kind = CPU;
            break;
        case VK_PHYSICAL_DEVICE_TYPE_OTHER:
        case VK_PHYSICAL_DEVICE_TYPE_MAX_ENUM:
            kind = Other;
            break;
    }
    struct vk_DeviceProperties ret_props = {};
    strncpy(ret_props.name, properties.properties.deviceName, 256U);
    ret_props.kind = kind;
    return ret_props;
}

struct vk_DeviceMemoryProperties vk_device_memory_properties(struct vk_DeviceRef device) {
    const struct vk_DeviceMemoryProperties invalid_properties = {};
    if (!device.handle) {
        return invalid_properties;
    }

    VkPhysicalDevice cast_device = device.handle;
    VkPhysicalDeviceMemoryProperties2 properties = {};
    properties.sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2;
    VkPhysicalDeviceMemoryBudgetPropertiesEXT memory_stats = {};
    memory_stats.sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT;
    properties.pNext = &memory_stats;
    vkGetPhysicalDeviceMemoryProperties2(cast_device, &properties);

    if (!properties.pNext) {
        return invalid_properties;
    }
    struct vk_DeviceMemoryProperties mem_props = {memory_stats.heapBudget[device.local_heap],
                                                  memory_stats.heapUsage[device.local_heap]};
    return mem_props;
}
