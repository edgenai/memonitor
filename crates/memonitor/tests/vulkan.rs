use memonitor::{init, list_backends};

#[test]
fn vulkan() {
    init();

    if let Some(guard) = list_backends() {
        let vulkan = guard.iter().find(|b| b.name() == "Vulkan").unwrap();
        for device in vulkan.list_devices().unwrap().iter() {
            let stats = device.current_memory_stats();
            println!(
                "Device found: {} - Memory stats: {} bytes used out of {}",
                device.name(),
                stats.usage,
                stats.budget
            );
        }
    }
}
