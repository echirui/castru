# Quickstart: Torrent Streaming

## Prerequisites
- A large magnet link (at least 100MB) to observe the buffering.
- Chromecast device.

## Steps

1.  **Run cast command**:
    ```bash
    cargo run -- cast "magnet:?xt=urn:btih:..."
    ```
2.  **Observe TUI (Initial Phase)**:
    - Status: `DOWNLOADING`.
    - Percentage should start from 0.0%.
3.  **Playback Start**:
    - Once it hits ~3% or 10MB, the status should change to `PLAYING`.
    - The video should appear on the Chromecast.
4.  **Observe TUI (Playback Phase)**:
    - You should see the seekbar and playback time.
    - The status line should show something like `PLAYING (DL: 15.2%)`.
5.  **Seek (Optional)**:
    - Try seeking forward. Note that seeking to a non-downloaded part may cause buffering.
