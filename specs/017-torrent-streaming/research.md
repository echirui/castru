# Research: Torrent Streaming while Downloading

**Feature**: `017-torrent-streaming`

## 1. Buffering Threshold for Playback

Instead of waiting for 100% download, we can trigger playback once a minimum amount of data is buffered. 
Common practices suggest:
- Percentage-based: 3% to 5% of total size.
- Size-based: 10MB to 50MB.

Decision: Use a hybrid approach. Wait for 3% OR 10MB, whichever comes first. This ensures small torrents start quickly and large torrents have enough buffer.

## 2. Background Progress Tracking

`librqbit` handles the download in a background task. To display progress in the TUI during playback, the main event loop needs access to the torrent's statistics.

Decision: Store `Arc<librqbit::ManagedTorrent>` in `AppState`. This allows the `animation_interval.tick()` block to query `stats()` and update `app_state.torrent_progress` without blocking.

## 3. TUI Display Logic

Currently, the TUI shows a dedicated "DOWNLOADING" status which blocks the seekbar.
When streaming starts, we should transition to the normal playback UI (showing seekbar and time), but include the download percentage as an overlay or in the status line.

Decision: If `torrent_progress < 100.0`, append `(DL: XX.X%)` to the status text or display it near the codec info.

## 4. Decision Log

| Decision | Rationale |
|----------|-----------|
| Hybrid Threshold (3% or 10MB) | Balancing wait time and playback stability. |
| Store Torrent Handle in AppState | Enables non-blocking progress updates during playback. |
| Non-blocking wait loop | Re-use `wait_for_torrent_download` but exit early. |
