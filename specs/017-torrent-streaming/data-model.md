# Data Model: Torrent Streaming while Downloading

## AppState (in main.rs)

Updated to store the active torrent handle for background tracking.

| Field | Type | Description |
|-------|------|-------------|
| `torrent_handle` | `Option<Arc<librqbit::ManagedTorrent>>` | The active torrent session handle. |
| `torrent_progress` | `Option<f32>` | Current download percentage (0.0 to 100.0). |

## State Transitions

1.  **Idle -> Downloading**: Torrent started. `wait_for_torrent_download` loop begins.
2.  **Downloading -> Playing**: Progress >= 3% OR downloaded >= 10MB.
    - `load_media` continues.
    - `app.load()` sent to receiver.
    - `torrent_handle` remains in `AppState`.
3.  **Playing (with BG Download)**:
    - Main event loop updates `torrent_progress` via `torrent_handle.stats()`.
    - TUI renders both playback time and download progress.
4.  **Download Finished**: `torrent_progress` reaches 100.0%.
    - `torrent_handle` can be cleared or kept for statistics.

## Constants

- `TORRENT_BUFFER_PCT_THRESHOLD: f32 = 3.0`
- `TORRENT_BUFFER_SIZE_THRESHOLD: u64 = 10 * 1024 * 1024` (10MB)
