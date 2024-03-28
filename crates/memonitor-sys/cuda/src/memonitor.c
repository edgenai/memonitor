#include <stdlib.h>
#include <string.h>

#include <windows.h>

#include <memonitor.h>

const char *LIB_NAME = "nvcuda.dll";

typedef int CUresult;
struct CUctx_st;
typedef struct CUctx_st *CUcontext;
struct CUdevice_v1;
typedef struct CUdevice_v1 *CUdevice;

typedef CUresult(*cuInit_type)(unsigned int);

typedef CUresult (*cuCtxCreate_type)(CUcontext *, unsigned int, CUdevice);

typedef CUresult (*cuCtxDestroy_type)(CUcontext);

typedef CUresult (*cuCtxSetCurrent_type)(CUcontext);

typedef CUresult (*cuDeviceGetCount_type)(int *);

typedef CUresult (*cuDeviceGet_type)(CUdevice *, int);

typedef CUresult (*cuDeviceGetName_type)(char *, int, CUdevice);

typedef CUresult (*cuDeviceTotalMem_type)(size_t *, CUdevice);

typedef CUresult (*cuMemGetInfo_type)(size_t *, size_t *);

HMODULE module = NULL;

cuInit_type cuInit = NULL;
cuCtxCreate_type cuCtxCreate = NULL;
cuCtxDestroy_type cuCtxDestroy = NULL;
cuCtxSetCurrent_type cuCtxSetCurrent = NULL;
cuDeviceGetCount_type cuDeviceGetCount = NULL;
cuDeviceGet_type cuDeviceGet = NULL;
cuDeviceGetName_type cuDeviceGetName = NULL;
cuDeviceTotalMem_type cuDeviceTotalMem = NULL;
cuMemGetInfo_type cuMemGetInfo = NULL;


int cu_init() {
    module = LoadLibraryA(LIB_NAME);
    if (!module) {
        return -1;
    }

    cuInit = (cuInit_type) GetProcAddress(module, "cuInit");
    cuCtxCreate = (cuCtxCreate_type) GetProcAddress(module, "cuCtxCreate");
    cuCtxDestroy = (cuCtxDestroy_type) GetProcAddress(module, "cuCtxDestroy");
    cuCtxSetCurrent = (cuCtxSetCurrent_type) GetProcAddress(module, "cuCtxSetCurrent");
    cuDeviceGetCount = (cuDeviceGetCount_type) GetProcAddress(module, "cuDeviceGetCount");
    cuDeviceGet = (cuDeviceGet_type) GetProcAddress(module, "cuDeviceGet");
    cuDeviceGetName = (cuDeviceGetName_type) GetProcAddress(module, "cuDeviceGetName");
    cuDeviceTotalMem = (cuDeviceTotalMem_type) GetProcAddress(module, "cuDeviceTotalMem");
    cuMemGetInfo = (cuMemGetInfo_type) GetProcAddress(module, "cuMemGetInfo");

    CUresult res = cuInit(0);
    if (res != 0) {
        return -2;
    }

    return 0;
}

void cu_term() {
    FreeLibrary(module);
    module = NULL;

    cuInit = NULL;
    cuCtxCreate = NULL;
    cuCtxDestroy = NULL;
    cuCtxSetCurrent = NULL;
    cuDeviceGetCount = NULL;
    cuDeviceGet = NULL;
    cuDeviceGetName = NULL;
    cuDeviceTotalMem = NULL;
    cuMemGetInfo = NULL;
}

struct cu_Devices cu_list_devices() {
    const struct cu_Devices invalid_devices = {
            NULL,
            NULL,
            0,
    };

    int count = 0;
    CUresult res = cuDeviceGetCount(&count);
    if (count <= 0 || res != 0) {
        return invalid_devices;
    }

    CUdevice *device_handles = malloc(sizeof(CUdevice) * count);
    CUcontext *ctx_handles = malloc(sizeof(CUcontext) * count);
    for (int d = 0; d < count; d++) {
        res = cuDeviceGet(&device_handles[d], d);
        if (res != 0) {
            free(device_handles);
            free(ctx_handles);
            return invalid_devices;
        }

        res = cuCtxCreate(&ctx_handles[d], 0, device_handles[d]);
        if (res != 0) {
            free(device_handles);
            free(ctx_handles);
            return invalid_devices;
        }
    }

    struct cu_Devices devices = {
            device_handles,
            ctx_handles,
            count,
    };
    return devices;
}

void cu_destroy_devices(struct cu_Devices *devices) {
    if (devices && devices->devices_handle && devices->ctx_handle && devices->count) {
        CUcontext *ctx_handles = devices->ctx_handle;
        for (int d = 0; d < devices->count; d++) {
            cuCtxDestroy(ctx_handles[d]);
        }
        free(devices->devices_handle);
        free(devices->ctx_handle);
        devices->devices_handle = NULL;
        devices->ctx_handle = NULL;
        devices->count = 0;
    }
}

struct cu_DeviceRef cu_get_device(struct cu_Devices *devices, uint32_t index) {
    const struct cu_DeviceRef invalid_ref = {NULL};
    if (!devices || !devices->devices_handle || !devices->ctx_handle || !devices->count || devices->count <= index) {
        return invalid_ref;
    }

    CUcontext *cast_devices = devices->devices_handle;
    CUcontext *cast_ctxs = devices->ctx_handle;
    struct cu_DeviceRef ref = {cast_devices[index], cast_ctxs[index]};
    return ref;
}

struct cu_DeviceProperties cu_device_properties(struct cu_DeviceRef device) {
    const struct cu_DeviceProperties invalid_properties = {{0}, Other, 0};
    if (!device.handle) {
        return invalid_properties;
    }

    CUdevice cast_device = device.handle;
    struct cu_DeviceProperties props = {0};
    props.kind = DiscreteGPU;

    CUresult res = cuDeviceGetName(props.name, sizeof(props.name), cast_device);
    if (res != 0) {
        return invalid_properties;
    }

    res = cuDeviceTotalMem(&props.total_memory, cast_device);
    if (res != 0) {
        return invalid_properties;
    }

    return props;
}

struct cu_DeviceMemoryProperties cu_device_memory_properties(struct cu_DeviceRef device) {
    const struct cu_DeviceMemoryProperties invalid_properties = {0};
    if (!device.handle) {
        return invalid_properties;
    }

    CUcontext cast_ctx = device.ctx_handle;
    CUresult res = cuCtxSetCurrent(cast_ctx);
    if (res != 0) {
        return invalid_properties;
    }

    size_t free_memory = 0;
    size_t total_memory = 0;

    res = cuMemGetInfo(&free_memory, &total_memory);
    if (res != 0) {
        return invalid_properties;
    }

    struct cu_DeviceMemoryProperties props = {
            free_memory,
            total_memory - free_memory,
    };
    return props;
}
