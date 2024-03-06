#include <stdlib.h>
#include <string.h>

#include <volk.h>

#include <memonitor.h>

#define ARRAY_LEN(array) sizeof(array) / sizeof(array[0])

#ifdef USE_VALIDATION_LAYERS
const char* layer_names[1] = {"VK_LAYER_KHRONOS_validation"};
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
    VkLayerProperties *properties = malloc(sizeof(VkLayerProperties *) * count);
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
    VkExtensionProperties *properties = malloc(sizeof(VkExtensionProperties *) * count);
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

int init_vk() {
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

void term_vk() {
    volkFinalize();
}

struct Devices list_devices() {
    const struct Devices invalid_devices = {NULL, 0};

    VkInstance instance = volkGetLoadedInstance();
    if (instance == VK_NULL_HANDLE) {
        return invalid_devices;
    }

    uint32_t count = 0;
    VkResult res = vkEnumeratePhysicalDevices(instance, &count, NULL);
    if (res != VK_SUCCESS) {
        return invalid_devices;
    }
    VkPhysicalDevice *device_handles = malloc(sizeof(VkPhysicalDevice *) * count);
    res = vkEnumeratePhysicalDevices(instance, &count, device_handles);
    if (res != VK_SUCCESS) {
        free(device_handles);
        return invalid_devices;
    }

    struct Devices devices = {device_handles, count};
    return devices;
}

void destroy_devices(struct Devices *devices) {
    if (devices && devices->handle && devices->count) {
        free(devices->handle);
    }
}

struct DeviceRef get_device(struct Devices *devices, uint32_t index) {
    const struct DeviceRef invalid_ref = {NULL};
    if (!devices || !devices->handle || !devices->count || devices->count <= index) {
        return invalid_ref;
    }

    VkPhysicalDevice *cast_devices = devices->handle;
    struct DeviceRef ref = {cast_devices[index]};
    return ref;
}
