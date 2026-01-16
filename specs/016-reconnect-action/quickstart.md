# Quickstart: Testing Reconnect Action

## Prerequisites
- A Google Cast device.
- `castru` built and running.

## Steps

1.  **Start Playback**:
    ```bash
    cargo run -- cast "path/to/media.mp4"
    ```
2.  **Verify Normal Operation**: Ensure media is playing and status is updated.
3.  **Simulate Disconnect (Optional)**: Briefly unplug the network cable or disable Wi-Fi. Observe "Disconnected" logs if possible.
4.  **Trigger Reconnect**:
    - Press the `r` key on your keyboard.
5.  **Observe TUI**:
    - Status should change to `RECONNECTING`.
    - Within a few seconds, it should change back to `PLAYING` or `PAUSED` (based on actual device state).
6.  **Verify Control**: Ensure you can still pause/seek after reconnection.
