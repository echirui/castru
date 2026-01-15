# Data Model: Projector Animation

## Entities

### Animation Frame

Represents a single static state of the projector animation.

| Field | Type | Description |
|-------|------|-------------|
| lines | `[&'static str; N]` | Array of strings representing the rows of the ASCII art. |

### Animation Sequence

Represents the collection of frames that form the loop.

| Field | Type | Description |
|-------|------|-------------|
| frames | `[Frame; 4]` | Fixed array of 4 frames. |

## State Management

The `TuiState` struct in `src/controllers/tui.rs` already maintains:

```rust
pub struct TuiState {
    // ...
    pub animation_frame: usize, // Incremented by the event loop
    // ...
}
```

This existing state is sufficient. The renderer will interpret `animation_frame` as `animation_frame % 4`.
