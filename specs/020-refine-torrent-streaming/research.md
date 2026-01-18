# Research: Refined Torrent Streaming Strategy

**Feature**: `020-refine-torrent-streaming`

## 1. Peerflix Strategy Analysis

`peerflix` implements several key strategies for streaming torrents:
- **Header/Footer Prioritization**: Requests the first and last pieces of the file immediately. This is crucial for media players to parse container metadata (e.g., MP4 atoms, MKV clusters).
- **Sequential Download**: Unlike standard BitTorrent (rarest-first), it requests pieces in linear order.
- **Sliding Window**: Requests a window of pieces (e.g., 10-20MB) ahead of the current playback head.
- **Seek Handling**: When a seek occurs, the sliding window is immediately shifted to the new offset.

## 2. librqbit Capabilities

`librqbit` (v8.1.1) provides:
- `ManagedTorrent::set_sequential(bool)`: A high-level toggle for sequential piece selection.
- `ManagedTorrent::stats()`: Provides detailed download statistics.
- `ManagedTorrent::list_files()`: To identify file offsets and lengths.

## 3. Proposed Refinements for castru

### Initial Buffering
In `TorrentManager::get_info`, after identifying the target file:
1.  Enable sequential mode: `handle.set_sequential(true)`.
2.  Identify the piece range for the target file.
3.  Set high priority for the first 2-3 pieces and the last 2-3 pieces of the file to satisfy media header requirements.

### Playback-Head Tracking
`GrowingFile` should inform the torrent engine about the current read position:
1.  As `poll_read` progresses, it should ensure pieces in the next ~50MB are prioritized.
2.  On `start_seek`, it should immediately update the "head" and reset priorities.

## 4. Decision Log

| Decision | Rationale |
|----------|-----------|
| Use `set_sequential(true)` | Built-in support in `librqbit` is more efficient than manual piece management. |
| Hybrid Priority Window | Combining sequential mode with "urgent" pieces (header/footer) matches Peerflix's success. |
| Trigger updates from `GrowingFile` | It's the only component that knows the *actual* read position of the receiver. |
