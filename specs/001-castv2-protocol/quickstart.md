# Quickstart: CastV2 Protocol Implementation

## Prerequisites
- Rust 1.75+
- `protoc` compiler installed (required for `prost-build`)

## Setup

1. Add dependencies to `Cargo.toml`:
   ```toml
   [dependencies]
   tokio = { version = "1", features = ["full"] }
   prost = "0.12"
   rustls = "0.21" # Check for latest compatible version
   bytes = "1"
   # serde, serde_json (if decided)
   
   [build-dependencies]
   prost-build = "0.12"
   ```

2. Place `.proto` files in `proto/` directory:
   - `cast_channel.proto` (Required)

3. Configure `build.rs` to compile protos:
   ```rust
   fn main() {
       prost_build::compile_protos(&["proto/cast_channel.proto"], &["proto/"]).unwrap();
   }
   ```

## Usage Example

```rust
use castru::CastClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Connect
    let mut client = CastClient::connect("192.168.1.50", 8009).await?;
    
    // 2. Launch App
    client.launch_app("CC1AD845").await?; // Default Media Receiver
    
    // 3. Listen for status
    while let Some(event) = client.events().recv().await {
        println!("Received event: {:?}", event);
    }
    
    Ok(())
}
```
