use std::ffi::CStr;
use std::ptr::addr_of_mut;

use memonitor_sys::vulkan;

use crate::{BackendHandle, DeviceHandle, DeviceKind, GPUKind, MemoryStats, VULKAN_NAME};

pub(super) struct Vulkan {
    handle: vulkan::Devices,
}

unsafe impl Send for Vulkan {}

unsafe impl Sync for Vulkan {}

impl Vulkan {
    pub(super) fn init() -> Option<(Self, Vec<VulkanDevice>)> {
        let res = unsafe { vulkan::init() };
        if res == 0 {
            let mut c_devices = unsafe { vulkan::list_devices() };

            if c_devices.handle.is_null() {
                unsafe { vulkan::term() };
                return None;
            }

            let mut devices = Vec::with_capacity(c_devices.count as usize);
            for i in 0..c_devices.count {
                let c_device = unsafe { vulkan::get_device(addr_of_mut!(c_devices), i) };
                if c_device.handle.is_null() {
                    unsafe {
                        vulkan::destroy_devices(addr_of_mut!(c_devices));
                        vulkan::term();
                    };
                    return None;
                }

                let properties = unsafe { vulkan::device_properties(c_device) };
                if properties.name[0] == 0 {
                    unsafe {
                        vulkan::destroy_devices(addr_of_mut!(c_devices));
                        vulkan::term();
                    };
                    return None;
                }

                if properties.total_memory == 0 {
                    continue;
                }

                let name = unsafe { CStr::from_ptr(properties.name.as_ptr()) };
                let kind = match properties.kind {
                    vulkan::DeviceKind::IntegratedGPU => DeviceKind::GPU(GPUKind::Integrated),
                    vulkan::DeviceKind::DiscreteGPU => DeviceKind::GPU(GPUKind::Discrete),
                    vulkan::DeviceKind::VirtualGPU => DeviceKind::GPU(GPUKind::Virtual),
                    vulkan::DeviceKind::CPU => DeviceKind::CPU,
                    vulkan::DeviceKind::Other => DeviceKind::Other,
                    _ => DeviceKind::Other,
                };

                let device = VulkanDevice {
                    handle: c_device,
                    name: name.to_string_lossy().to_string(),
                    kind,
                    memory: properties.total_memory,
                };
                devices.push(device);
            }

            let backend = Vulkan { handle: c_devices };

            Some((backend, devices))
        } else {
            None
        }
    }
}

impl BackendHandle for Vulkan {
    fn name(&self) -> &str {
        VULKAN_NAME
    }
}

impl Drop for Vulkan {
    fn drop(&mut self) {
        unsafe {
            vulkan::destroy_devices(addr_of_mut!(self.handle));
            vulkan::term();
        }
    }
}

pub(super) struct VulkanDevice {
    handle: vulkan::DeviceRef,
    name: String,
    kind: DeviceKind,
    memory: usize,
}

unsafe impl Send for VulkanDevice {}

unsafe impl Sync for VulkanDevice {}

impl DeviceHandle for VulkanDevice {
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
        let c_stats = unsafe { vulkan::device_memory_properties(self.handle) };
        // The Vulkan extension returns the maximum budget at this time, including the memory this process is already
        // currently using, so this is done to get the true value for currently free memory
        let available_memory = c_stats.budget - c_stats.used;
        MemoryStats {
            total: self.memory,
            available: available_memory,
            used: self.memory - available_memory,
        }
    }
}
