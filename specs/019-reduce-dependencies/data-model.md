# Data Model: Dependency Minimization

## Error Handling Pattern (Manual)

Instead of `#[derive(Error)]`, we will use:

```rust
impl std::fmt::Display for CastError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "IO Error: {}", e),
            // ... other variants
        }
    }
}

impl std::error::Error for CastError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}
```

## Unique ID Generation

Replacement for `uuid::Uuid::new_v4()` for transcode filenames:

```rust
fn generate_temp_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos();
    // Combined with a process-local counter if needed
    format!("{:x}", now)
}
```

## Entity Changes

| Entity | Change |
|--------|--------|
| `TorrentSession` | Change `session_id` from `uuid::Uuid` to `String` or `u64`. |
| `CastDevice` | No change (already uses `String` for UUID). |
