# Contract: Internal Probe Events

**Purpose**: define the mechanism for asynchronous probe results to reach the main event loop.

## Enum Definition

```rust
pub enum AppEvent {
    // Existing sources (Conceptual)
    CastEvent(castru::client::CastEvent),
    TuiEvent(castru::controllers::tui::TuiCommand),
    
    // New
    ProbeCompleted {
        duration: Option<f64>,
        video_codec: Option<String>,
        audio_codec: Option<String>,
    },
}
```

**Usage**:
- `wait_for_torrent_download` will spawn a tokio task.
- This task waits for a threshold (e.g., 2MB downloaded).
- It runs `probe_media`.
- It sends `ProbeCompleted` via an `mpsc::Sender`.
