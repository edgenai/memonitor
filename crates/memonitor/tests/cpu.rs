use memonitor::{init, list_all_devices, list_backends};

#[test]
fn cpu() {
    init();

    let device_ids = {
        let backends = list_backends();
        backends
            .iter()
            .find(|b| b.name() == "Host")
            .unwrap()
            .device_ids()
            .to_vec()
    };

    {
        let devices = list_all_devices();
        for id in device_ids {
            let stats = devices[id].current_memory_stats();
            println!(
                "Device found: {} ({}) - Memory stats: {} bytes used out of {}, {} are free",
                devices[id].name(),
                devices[id].kind(),
                stats.used,
                stats.total,
                stats.free
            );
        }
    }
}
