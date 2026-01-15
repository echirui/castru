# Data Model: Castnow-like TUI

## TuiState

Represents the full state required to render the UI.

```rust
pub struct TuiState {
    pub status: PlaybackStatus,
    pub current_time: f32,
    pub total_duration: Option<f32>,
    pub volume_level: Option<f32>, // 0.0 to 1.0
    pub is_muted: bool,
    pub media_title: Option<String>,
}
```

## InputEvent

Mapped to `TuiCommand` enum (existing).

```rust
pub enum TuiCommand {
    Play,
    Pause,
    Stop,
    Next,
    Previous,
    SeekForward(u64),
    SeekBackward(u64),
    VolumeUp,   // +5%
    VolumeDown, // -5%
    ToggleMute,
    Quit,
}
```

## State Transitions

- **Idle** -> **Loading** -> **Playing**
- **Playing** <-> **Paused**
- **Playing** -> **Buffering** -> **Playing**
