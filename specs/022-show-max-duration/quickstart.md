# Quickstart: Testing Max Duration

**Feature**: `022-show-max-duration`

## Prerequisites

- A valid magnet link for a video file (safe/legal content).
- `ffprobe` installed (`ffmpeg` package).

## Steps

1.  **Run with Magnet Link**:
    ```bash
    cargo run -- cast "magnet:?xt=urn:btih:..."
    ```

2.  **Observe TUI**:
    - Initially, Status: `METADATA FETCHING`
    - Then, Status: `BUFFERING (TORRENT)`
    - Watch the `Duration` field in the TUI header or status line.
    - **Expected**: It should change from `Unknown` (or 00:00:00) to a valid timestamp (e.g., `00:10:30`) within a few seconds of buffering starting.

3.  **Verify Playback**:
    - Once buffering finishes and playback starts, ensure the duration remains correct.
