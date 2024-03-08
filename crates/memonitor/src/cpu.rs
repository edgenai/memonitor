use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

use crate::{BackendHandle, DeviceHandle, DeviceKind, MemoryStats, CPU_NAME};

pub(super) struct Host {
    _system: Arc<RwLock<System>>,
}

impl Host {
    pub(super) fn init() -> (Self, Vec<Cpu>) {
        let mut system = System::new_with_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::default())
                .with_memory(MemoryRefreshKind::default().with_ram()),
        );
        system.refresh_cpu();

        let mut cpu_names = HashSet::new();
        // This loops over CPU cores, not physical CPUs, could probably change to be not redundant
        for handle in system.cpus() {
            // Brand seems to return the actual name of the CPU
            cpu_names.insert(handle.brand().to_string());
        }

        system.refresh_memory();
        let total_memory = system.total_memory();

        let system = Arc::new(RwLock::new(system));
        let cpus = cpu_names
            .drain()
            .map(|name| Cpu {
                system: system.clone(),
                name,
                memory: total_memory as usize,
            })
            .collect();

        let backend = Self { _system: system };
        (backend, cpus)
    }
}

impl BackendHandle for Host {
    fn name(&self) -> &str {
        CPU_NAME
    }
}

pub(super) struct Cpu {
    system: Arc<RwLock<System>>,
    name: String,
    memory: usize,
}

impl DeviceHandle for Cpu {
    fn name(&self) -> &str {
        &self.name
    }

    fn kind(&self) -> DeviceKind {
        DeviceKind::CPU
    }

    fn backend_name(&self) -> &str {
        CPU_NAME
    }

    fn current_memory_stats(&self) -> MemoryStats {
        let mut guard = self.system.write().unwrap();
        guard.refresh_memory();

        MemoryStats {
            total: self.memory,
            available: guard.available_memory() as usize,
            used: guard.used_memory() as usize,
        }
    }
}
