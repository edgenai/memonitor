use memonitor::{init, list_all_devices, list_backends};

#[test]
fn all() {
    init();

    for backend in list_backends().iter() {
        println!("Backend found: {}", backend.name());
    }

    {
        for device in list_all_devices().iter() {
            let stats = device.current_memory_stats();
            println!(
                "Device found: {} ({}) - Memory stats: {} bytes used out of {}, {} are free",
                device.name(),
                device.kind(),
                stats.used,
                stats.total,
                stats.free
            );
        }
    }
}
