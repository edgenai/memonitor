use std::ffi::CStr;
use std::ptr::addr_of_mut;

use memonitor_sys::cuda;

use crate::{
    BackendHandle, DeviceHandle, DeviceKind, GPUKind, MemoryStats, CUDA_NAME, VULKAN_NAME,
};

pub(super) struct Cuda {
    handle: cuda::Devices,
}

unsafe impl Send for Cuda {}

unsafe impl Sync for Cuda {}

impl Cuda {
    pub(super) fn init() -> Option<(Self, Vec<CudaDevice>)> {
        let res = unsafe { cuda::init() };
        if res == 0 {
            let mut c_devices = unsafe { cuda::list_devices() };

            if c_devices.count == 0 {
                unsafe {
                    cuda::destroy_devices(addr_of_mut!(c_devices));
                    cuda::term();
                }
                return None;
            }

            let mut devices = Vec::with_capacity(c_devices.count as usize);
            for i in 0..c_devices.count {
                let c_device = unsafe { cuda::get_device(addr_of_mut!(c_devices), i) };
                if c_device.handle.is_null() {
                    unsafe {
                        cuda::destroy_devices(addr_of_mut!(c_devices));
                        cuda::term();
                    };
                    return None;
                }

                let properties = unsafe { cuda::device_properties(c_device) };
                if properties.name[0] == 0 {
                    unsafe {
                        cuda::destroy_devices(addr_of_mut!(c_devices));
                        cuda::term();
                    };
                    return None;
                }

                let name = unsafe { CStr::from_ptr(properties.name.as_ptr()) };
                let kind = match properties.kind {
                    cuda::DeviceKind::IntegratedGPU => DeviceKind::GPU(GPUKind::Integrated),
                    cuda::DeviceKind::DiscreteGPU => DeviceKind::GPU(GPUKind::Discrete),
                    cuda::DeviceKind::Other => DeviceKind::Other,
                    _ => DeviceKind::Other,
                };

                if properties.total_memory == 0 {
                    continue;
                }

                let device = CudaDevice {
                    handle: c_device,
                    name: name.to_string_lossy().to_string(),
                    kind,
                    memory: properties.total_memory,
                };
                devices.push(device);
            }

            let backend = Cuda { handle: c_devices };

            Some((backend, devices))
        } else {
            None
        }
    }
}

impl BackendHandle for Cuda {
    fn name(&self) -> &str {
        CUDA_NAME
    }
}

impl Drop for Cuda {
    fn drop(&mut self) {
        unsafe {
            cuda::destroy_devices(addr_of_mut!(self.handle));
            cuda::term();
        }
    }
}

pub(super) struct CudaDevice {
    handle: cuda::DeviceRef,
    name: String,
    kind: DeviceKind,
    memory: usize,
}

unsafe impl Send for CudaDevice {}

unsafe impl Sync for CudaDevice {}

impl DeviceHandle for CudaDevice {
    fn name(&self) -> &str {
        &self.name
    }

    fn kind(&self) -> DeviceKind {
        self.kind
    }

    fn backend_name(&self) -> &str {
        VULKAN_NAME
    }

    fn current_memory_stats(&self) -> MemoryStats {
        let c_stats = unsafe { cuda::device_memory_properties(self.handle) };
        MemoryStats {
            total: self.memory,
            available: c_stats.budget,
            used: c_stats.used,
        }
    }
}
