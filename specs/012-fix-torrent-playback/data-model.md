# Data Model: Fix Torrent Playback

## TorrentState

Extended state enum.

```rust
pub enum TorrentState {
    Resolving,
    DownloadingMetadata,
    Buffering { progress: f32 }, // New: Indicates pre-roll buffering
    ReadyToPlay,
    Playing,
    Finished,
}
```

## TorrentInfo

Struct returned by `TorrentManager` to allow external control.

| Field | Type | Description |
|-------|------|-------------|
| `handle` | `Arc<ManagedTorrent>` | The active torrent handle. |
| `file_path` | `PathBuf` | Absolute path to the downloading file. |
| `file_index` | `usize` | Index of the main video file. |
| `total_size` | `u64` | Total size of the main video file. |
