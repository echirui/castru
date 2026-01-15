# Internal API: Fix Transcode Seek Synchronization

## main.rs

### `load_media`

Updates the signature to better support state synchronization.

```rust
async fn load_media(
    app: &DefaultMediaReceiver,
    server: &StreamServer,
    source: &MediaSource,
    server_base: &str,
    start_time: f64,
    torrent_manager: &TorrentManager,
) -> Result<(bool, MediaProbeResult, f64), Box<dyn Error>>
```

**Returns**:
- `bool`: `is_transcoding`
- `MediaProbeResult`: Metadata probe
- `f64`: `applied_seek_offset` (The actual start time used for transcoding)

### Event Loop Logic

When processing `MediaResponse::MediaStatus`:

```rust
// Logic update
let reported_time = s.current_time as f64;
if app_state.is_transcoding {
    app_state.current_time = reported_time + app_state.seek_offset;
} else {
    app_state.current_time = reported_time;
}
```
