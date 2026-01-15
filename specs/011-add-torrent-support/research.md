# Research: Torrent Streaming Support

**Date**: 2026-01-15
**Feature**: `011-add-torrent-support`

## 1. Torrent Engine Selection

### Problem
We need a BitTorrent client implementation in Rust that supports:
1.  **Sequential Downloading**: Prioritizing pieces in order for streaming.
2.  **Async I/O**: Integration with our `tokio` runtime.
3.  **Metadata Resolution**: Fast fetching of magnet link metadata.
4.  **Minimalism**: Balancing feature set with dependency weight (Constitution principle).

### Candidates

1.  **librqbit**
    - **Pros**: Explicitly designed for streaming/sequential usage. Built on `tokio`. Actively maintained. High performance.
    - **Cons**: Medium dependency weight.
2.  **cratetorrent**
    - **Pros**: Tokio-based.
    - **Cons**: Less emphasis on streaming/seeking optimization in documentation.
3.  **Custom Implementation**
    - **Pros**: Zero external dependencies (only std + tokio).
    - **Cons**: Extremely high complexity (DHT, Peer wire protocol, Trackers). High maintenance risk.

### Decision
**Use `librqbit`**.

### Rationale
Implementing a BitTorrent client from scratch violates the spirit of "efficiency" and "safety" (bugs in protocol impl). `librqbit` aligns perfectly with the requirement to "stream like castnow" and uses our existing `tokio` stack. The complexity cost of the dependency is outweighed by the functionality provided.

## 2. Streaming Architecture

### Flow
1.  **Input**: Magnet URI or .torrent path.
2.  **Resolution**: `librqbit::Session` adds the torrent.
3.  **Selection**: Identify largest video file.
4.  **Buffering**: `librqbit` configured to download sequentially.
5.  **Output**: Expose a `Stream` or `AsyncRead` that yields bytes as they arrive.
    - *Challenge*: HTTP range requests from Cast device.
    - *Solution*: The `StreamServer` (from feature 010) needs to be adapted to read from this `AsyncRead` source instead of just `tokio::fs::File`. Or, `librqbit` might stream to a file, and we read from that file while it grows ("growing file" pattern).
    - *Refinement*: `librqbit` writes to disk. We can open that file in `read-only` mode and follow the write cursor, or use `librqbit`'s API if it exposes an in-memory stream.
    - *Decision*: Stream from the file on disk. This persists progress and allows standard file seek operations (with waits if data missing).

### Storage
- Use system temporary directory by default.
- Create a unique subdirectory per session.
- cleanup on exit (RAII or explicit signal).

## 3. Integration with `StreamServer`

The existing `StreamServer` expects a file path.
- **Approach**: Pass the path of the *downloading* file to `StreamServer`.
- **Requirement**: `StreamServer`'s buffering logic (producer task) must handle "EOF vs Not Downloaded Yet".
- **Modification**: We might need a "Live File" reader that knows the *expected* total size but blocks/waits if it hits the current local EOF but the torrent isn't finished.

*Correction*: `010-tune-buffering` refactored `StreamServer` to use a `producer_task`. We can pass a custom `AsyncRead` implementation to this producer that wraps the growing file and waits for pieces.

### Final Plan for Integration
1.  Initialize `librqbit` session.
2.  Start download to temp path.
3.  Wait for metadata and start of data.
4.  Construct a `GrowingFileStream` (or similar) that reads from the temp path.
5.  Pass this stream to `StreamServer`.
