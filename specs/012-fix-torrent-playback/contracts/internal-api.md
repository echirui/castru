# Internal API: Fix Torrent Playback

## TorrentManager

### `start_magnet`

```rust
pub async fn start_magnet(&self, uri: &str) -> Result<TorrentStreamInfo, TorrentError>;
```

**Output**:
```rust
pub struct TorrentStreamInfo {
    pub handle: Arc<ManagedTorrent>,
    pub path: PathBuf,
    pub total_size: u64,
    pub file_offset: u64,
    pub piece_length: u64,
}
```

## GrowingFile

### `poll_read`

Updated behavior:
- Calculates `piece_idx` from current `position`.
- Checks `handle.chunks().is_present(piece_idx)`.
- If missing: returns `Poll::Pending` and wakes later.
- If present: reads from `File`.
