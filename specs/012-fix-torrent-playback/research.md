# Research: Fix Torrent Playback

**Date**: 2026-01-15
**Feature**: `012-fix-torrent-playback`

## 1. Buffering Strategy

### Problem
The current implementation starts streaming immediately. If the file is sparse and data isn't ready, the Chromecast receives zeroes (from sparse file read) or connection drops, leading to playback failure.

### Solution
Implement a "Pre-buffering" phase.
1.  **Start Torrent**: Initialize session and add torrent.
2.  **Wait**: Monitor download progress until a threshold is met (e.g., 5% or 10MB).
3.  **Serve**: Only then start the HTTP server (or just provide the URL to Chromecast).

### Librqbit Capabilities
- `ManagedTorrent::stats()` returns `Stats` struct.
- `Stats` has `bytes_downloaded`, `total_bytes`, `progress` (0.0 - 1.0).
- `chunks().is_present(idx)` allows checking specific pieces.

### Decision
- **Threshold**: Wait for 3% of total size or 10MB (whichever is smaller) AND ensure the *first* piece is downloaded (crucial for headers).
- **UX**: Show "Downloading... X%" in the CLI/TUI.
- **Implementation**: `TorrentManager::prepare` method that loops and checks status.

## 2. Preventing "Zero Read"
Even after initial buffering, we might catch up to the download cursor.
- **Mechanism**: The `GrowingFile` adapter must be aware of piece availability.
- **Logic**: Before `file.read()`, calculate the piece index for the current offset. Check `handle.chunks().is_present(idx)`. If false, `return Poll::Pending` and schedule a wakeup.
- **Improvement**: `librqbit` doesn't have a "notify on piece complete" easily accessible in 8.x for external consumers without polling or callbacks. Polling (with backoff) inside `poll_read` is acceptable for a local stream.

## 3. Architecture Change
- **Current**: `start_magnet` returns `GrowingFile` immediately.
- **New**: `main.rs` should orchestrate the wait. `start_magnet` returns the `TorrentHandle`. `main.rs` loops on `handle.stats()` until ready. Then creates `GrowingFile` and starts `StreamServer`.

### Refined Plan
1.  `TorrentManager::start_magnet` returns `Arc<ManagedTorrent>`.
2.  `main.rs` (or a helper) monitors `handle`.
3.  Once ready, `main.rs` calls `StreamServer::set_source`.
