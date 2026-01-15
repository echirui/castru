# Data Model: TUI Enhancements

## TuiState Updates

Reflected in `src/controllers/tui.rs`.

```rust
pub struct TuiState {
    pub status: String,
    pub current_time: f32,
    pub total_duration: Option<f32>,
    pub volume_level: Option<f32>,
    pub is_muted: bool,
    pub media_title: Option<String>,
    
    // New Fields
    pub video_codec: Option<String>,
    pub audio_codec: Option<String>,
    pub device_name: String,
    pub animation_frame: usize, // 0..N
}
```

## Internal Animation Logic

- `animation_frame` increments every tick (approx 150ms).
- `draw` function selects character based on `animation_frame % 4`.
