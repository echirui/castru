use crate::error::CastError;
use mdns_sd::{ServiceDaemon, ServiceEvent};
use std::net::IpAddr;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;

/// Represents a discovered Cast device on the network.
#[derive(Debug, Clone)]
pub struct CastDevice {
    /// IP address of the device.
    pub ip: IpAddr,
    /// Port number (usually 8009).
    pub port: u16,
    /// Friendly name (e.g., "Living Room TV").
    pub friendly_name: String,
    /// Model name (e.g., "Chromecast Ultra").
    pub model_name: String,
    /// Unique UUID of the device.
    pub uuid: String,
}

/// Discovers Cast devices on the local network using mDNS.
///
/// This function blocks for the specified `timeout` duration and returns a list of found devices.
///
/// # Arguments
///
/// * `timeout` - The duration to wait for mDNS responses.
///
/// # Example
///
/// ```no_run
/// use castru::discovery::discover_devices;
/// use std::time::Duration;
///
/// let devices = discover_devices(Duration::from_secs(5)).unwrap();
/// for device in devices {
///     println!("Found: {}", device.friendly_name);
/// }
/// ```
pub fn discover_devices(timeout: Duration) -> Result<Vec<CastDevice>, CastError> {
    let mdns = ServiceDaemon::new().map_err(|e| CastError::Protocol(e.to_string()))?;
    let service_type = "_googlecast._tcp.local.";
    let receiver = mdns
        .browse(service_type)
        .map_err(|e| CastError::Protocol(e.to_string()))?;

    let mut devices = Vec::new();
    let start = Instant::now();

    loop {
        let remaining = timeout.checked_sub(start.elapsed());
        if remaining.is_none() {
            break;
        }

        if let Ok(event) = receiver.recv_timeout(remaining.unwrap()) {
            if let ServiceEvent::ServiceResolved(info) = event {
                let friendly_name = info
                    .get_properties()
                    .get_property_val_str("fn")
                    .unwrap_or("Unknown")
                    .to_string();
                let model_name = info
                    .get_properties()
                    .get_property_val_str("md")
                    .unwrap_or("Unknown")
                    .to_string();
                let uuid = info
                    .get_properties()
                    .get_property_val_str("id")
                    .unwrap_or("Unknown")
                    .to_string();

                if let Some(ip) = info.get_addresses().iter().next() {
                    devices.push(CastDevice {
                        ip: *ip,
                        port: info.get_port(),
                        friendly_name,
                        model_name,
                        uuid,
                    });
                }
            }
        } else {
            break;
        }
    }

    Ok(devices)
}

/// Asynchronously discovers Cast devices on the local network.
///
/// Returns a channel receiver that streams discovered devices.
/// The discovery runs until the user stops receiving or for the default mDNS duration.
pub fn discover_devices_async() -> Result<mpsc::Receiver<CastDevice>, CastError> {
    let mdns = ServiceDaemon::new().map_err(|e| CastError::Protocol(e.to_string()))?;
    let service_type = "_googlecast._tcp.local.";
    let receiver = mdns
        .browse(service_type)
        .map_err(|e| CastError::Protocol(e.to_string()))?;

    let (tx, rx) = mpsc::channel(32);

    tokio::task::spawn_blocking(move || {
        while let Ok(event) = receiver.recv() {
            if let ServiceEvent::ServiceResolved(info) = event {
                let friendly_name = info
                    .get_properties()
                    .get_property_val_str("fn")
                    .unwrap_or("Unknown")
                    .to_string();
                let model_name = info
                    .get_properties()
                    .get_property_val_str("md")
                    .unwrap_or("Unknown")
                    .to_string();
                let uuid = info
                    .get_properties()
                    .get_property_val_str("id")
                    .unwrap_or("Unknown")
                    .to_string();

                if let Some(ip) = info.get_addresses().iter().next() {
                    let device = CastDevice {
                        ip: *ip,
                        port: info.get_port(),
                        friendly_name,
                        model_name,
                        uuid,
                    };
                    if tx.blocking_send(device).is_err() {
                        break;
                    }
                }
            }
        }
    });

    Ok(rx)
}
