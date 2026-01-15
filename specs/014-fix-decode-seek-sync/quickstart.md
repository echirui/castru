# Quickstart: Verifying Accurate Seek and Resume

## Prerequisites
- `ffmpeg` and `ffprobe` installed and in PATH.
- A non-native media file (e.g., `.mkv`, `.avi`, or high-bitrate `.mp4` that triggers transcoding).
- A Google Cast device on the same network.

## Testing Seek Accuracy

1.  **Start Playback**:
    ```bash
    cargo run -- cast "path/to/transcoded_file.mkv"
    ```
2.  **Observe Initial Time**: The TUI should start from `0:00`.
3.  **Perform Seek**: Press `Right Arrow` (30s).
    - **Expected**: The video reloads quickly. The TUI time display jumps to `0:30` and continues from there.
4.  **Perform Backward Seek**: Press `Left Arrow` (15s).
    - **Expected**: The TUI time display shows `0:15` (assuming it was at `0:30`).

## Testing Resume on Disconnect (Simulated)

1.  **Interrupt Stream**: Temporarily disconnect your network or restart the Chromecast during playback.
2.  **Wait for Watchdog**: (This feature will be implemented in the plan).
3.  **Expected**: The application detects the playback failure and attempts to reload the media from the last known `current_time`.
