# Research: Buffering Tuning and Refactoring

**Feature**: Buffering Tuning (010-tune-buffering)
**Status**: Complete
**Date**: 2026-01-15

## Technical Decisions

### 1. Buffering Strategy

**Decision**: Implement a **Decoupled Producer-Consumer** pattern using `tokio` channels (mpsc).

**Rationale**:
- **Problem**: The current implementation uses a synchronous-style loop (`read` -> `await` -> `write` -> `await`). If the network write blocks (backpressure), disk I/O pauses. If disk I/O is slow (seek latency), network transmission halts, causing buffer underrun on the receiver.
- **Solution**: A separate "Reader Task" continuously reads from the file into chunks and sends them to a channel. The main "Writer Loop" pulls from the channel and writes to the socket.
- **Benefit**: Disk I/O happens proactively, filling the channel buffer. Network writes consume this pre-buffered data. This smooths out jitter from both disk and network.

**Alternatives Considered**:
- **Just increasing buffer size**: E.g., reading 1MB at a time.
    - *Pros*: Simplest change.
    - *Cons*: Still coupled. Large reads might block the loop longer. Doesn't solve the "pause while writing" issue.
- **`tokio::io::BufReader`**:
    - *Pros*: Standard library wrapper.
    - *Cons*: Primarily helps with many small reads (e.g., parsing lines). Less effective for large streaming chunks where we want *prefetching*.

### 2. Buffer Tuning Parameters

**Decision**:
- **Chunk Size**: Increase from **64KB** to **256KB** or **512KB**.
- **Channel Capacity**: Set to **4-8 chunks** (approx 2MB - 4MB total pre-load).

**Rationale**:
- High bitrate video (e.g., 4K 60fps) can be 50-100 Mbps (~6-12 MB/s).
- 64KB is exhausted in ~5ms at 100Mbps.
- 512KB allows ~40-80ms of playback data per chunk.
- A 4MB ring buffer (channel) provides ~0.5 - 1 second of buffer, which is robust against minor disk/scheduler latency.

### 3. Architecture Change

**Decision**: Introduce a `BufferedStreamer` struct (or internal helper in `server.rs`).

**Rationale**:
- Keeps `server.rs` clean.
- Encapsulates the task spawning and channel management.
- Allows easy unit testing of the "producer" logic (does it keep reading? does it stop when full?).

## Open Questions

- **Resolved**: No external crates needed. `tokio::sync::mpsc` and `tokio::spawn` are sufficient.
