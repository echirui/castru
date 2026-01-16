# Quickstart: Verifying Torrent Full Download

## Prerequisites
- Internet connection for BitTorrent traffic.
- Google Cast device.

## Verification Steps

1.  **Run cast command with a magnet link**:
    ```bash
    cargo run -- cast "magnet:?xt=urn:btih:..."
    ```
2.  **Observe TUI**:
    - The status should show `DOWNLOADING`.
    - A percentage `XX.X%` should be visible and increasing.
    - The projector animation should be static or shown as idle.
3.  **Wait for completion**:
    - Once it reaches 100.0%, the status should change to `PLAYING` (or `BUFFERING` initially).
    - The video should start playing on the Chromecast.
4.  **Confirm smooth playback**:
    - Verify that there are no stutters caused by network speed, since the file is local.
