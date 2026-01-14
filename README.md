# castru

A minimalist, pure-Rust implementation of the Google Cast (CastV2) protocol.

## Features

- **Discovery**: Automatically find Chromecast devices on your local network using mDNS.
- **Connection**: Secure TLS connection with automatic reconnection logic (Exponential Backoff).
- **Controllers**: High-level `ReceiverController` and `MediaController` for easy interaction.
- **Async**: Built on `tokio` for efficient asynchronous I/O.
- **Minimal Dependencies**: Carefully selected dependencies (`tokio`, `prost`, `rustls`, `mdns-sd`).

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
castru = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

## Getting Started

### 1. Discover and Connect

```rust
use castru::discovery::discover_devices;
use castru::CastClient;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Discover
    let devices = discover_devices(Duration::from_secs(5))?;
    let device = devices.first().ok_or("No device found")?;
    
    // 2. Connect
    let client = CastClient::connect(&device.ip.to_string(), device.port).await?;
    client.connect_receiver().await?;
    
    Ok(())
}
```

### 2. Platform Control (Launch App)

```rust
use castru::controllers::receiver::ReceiverController;

let receiver = client.receiver();
receiver.launch_app("CC1AD845").await?; // Launch Default Media Receiver
```

### 3. Media Control

```rust
use castru::controllers::media::MediaController;
use castru::protocol::media::MediaInformation;

// Assume we have the transport ID for the launched app
let media = client.media("transport-id-from-status");

let info = MediaInformation {
    content_id: "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4".to_string(),
    stream_type: "BUFFERED".to_string(),
    content_type: "video/mp4".to_string(),
    metadata: None,
};

// Load and play
media.load(info, true, 0.0).await?;

// Pause
media.pause(1).await?; // Requires mediaSessionId from status updates
```

### 4. Simplified Media Control (Default Media Receiver)

For the common use case of using the Default Media Receiver, use the high-level wrapper:

```rust
use castru::controllers::default_media_receiver::DefaultMediaReceiver;

let mut app = DefaultMediaReceiver::new(&client);
app.launch().await?; // Auto-launches and connects

// Load content directly
app.load(info, true, 0.0).await?;
```

## Examples

Check the `examples/` directory for full working examples:

- `cargo run --example discover`
- `cargo run --example platform_control <IP>`
- `cargo run --example full_media <IP> <MEDIA_URL>`

## CLI Usage

Castru also provides a command-line interface for common tasks:

- **Scan**: Find devices
  ```bash
  cargo run -- scan
  ```

- **Cast**: Stream a local file or URL (Supports playlists)
  ```bash
  cargo run -- cast ./myvideo.mp4 https://example.com/video.mp4
  ```
  *(Controls: Space to Pause/Play, N for Next, P for Previous, Q to Quit)*

  **Options:**
  - `--ip <IP>`: Connect directly to a specific IP address.
  - `--name <NAME>`: Connect to a device with a specific friendly name (e.g., "Living Room TV").

  ```bash
  cargo run -- cast --ip 192.168.1.100 video.mp4
  cargo run -- cast --name "Living Room TV" video.mp4
  ```

## License

MIT