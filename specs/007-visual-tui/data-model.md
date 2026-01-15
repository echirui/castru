# Data Model: Visual TUI

## TuiState (In-Memory)

Passes data from `main.rs` to `tui.rs`.

```rust
pub struct TuiState {
    pub status: String,
    pub current_time: f32,
    pub total_duration: Option<f32>,
    pub volume_level: Option<f32>,
    pub is_muted: bool,
    pub media_title: Option<String>,
}
```

## TuiCommand (Enum)

Commands from TUI to Controller.

```rust
pub enum TuiCommand {
    Play,
    Pause,
    TogglePlay, // New
    Stop,
    Next,
    Previous,
    SeekForward(u64),
    SeekBackward(u64),
    VolumeUp,
    VolumeDown,
    ToggleMute,
    Quit,
}
```
