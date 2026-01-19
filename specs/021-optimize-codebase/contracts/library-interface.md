# Contract: Library Interface Refactoring

**Purpose**: Define the API surface for the core library logic extracted from `src/main.rs`.

## `src/lib.rs` Public API

### Application Lifecycle

```rust
/// Main entry point for the CastNow application logic.
/// This struct allows embedding the cast logic into other applications (CLI, GUI, etc).
pub struct CastNowCore {
    // ... internal state
}

impl CastNowCore {
    /// Initialize the core with configuration.
    pub fn new(config: Config) -> Self;

    /// Start the main event loop.
    /// This is async and non-blocking.
    pub async fn run(&self) -> Result<(), AppError>;
    
    /// Trigger a specific action programmatically.
    pub async fn cast_media(&self, media: MediaSource) -> Result<()>;
}
```

### Configuration

```rust
pub struct Config {
    pub device_ip: Option<String>,
    pub port: u16,
    pub media_path: Option<String>,
    // ... other CLI args
}
```
