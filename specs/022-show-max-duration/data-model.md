# Data Model: Show Max Duration

**Feature**: `022-show-max-duration`

## App State Updates

### `AppState` (in `src/app.rs`)

No structural changes to `AppState` struct itself, but the lifecycle of `total_duration` changes.

- **Current**: `total_duration` populated only on initial load if file exists.
- **New**: `total_duration` populated asynchronously during torrent buffering.

### `ProbeResult` (Internal Message)

A new enum/struct to communicate probing results back to the main loop.

```rust
enum AppEvent {
    // ... existing events
    ProbeComplete {
        duration: f64,
        video_codec: Option<String>,
        audio_codec: Option<String>,
    },
}
```
