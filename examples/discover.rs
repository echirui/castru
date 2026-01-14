use castru::discovery::discover_devices;
use std::time::Duration;

fn main() {
    println!("Discovering Cast devices for 5 seconds...");
    match discover_devices(Duration::from_secs(5)) {
        Ok(devices) => {
            if devices.is_empty() {
                println!("No devices found.");
            } else {
                for device in devices {
                    println!("Found device:");
                    println!("  Name: {}", device.friendly_name);
                    println!("  Model: {}", device.model_name);
                    println!("  IP: {}:{}", device.ip, device.port);
                    println!("  UUID: {}", device.uuid);
                    println!("--------------------------------");
                }
            }
        }
        Err(e) => eprintln!("Discovery failed: {}", e),
    }
}
