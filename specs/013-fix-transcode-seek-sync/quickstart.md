# Quickstart: Verifying Transcode Seek Fix

## Prerequisites

-   A video file requiring transcoding (e.g., `.mkv` or `.avi` with non-h264 codecs).
-   `ffmpeg` and `ffprobe` installed.

## Verification Steps

1.  **Run castru**:
    ```bash
    cargo run -- cast "path/to/video.mkv"
    ```

2.  **Verify Initial Time**:
    -   Playback starts.
    -   TUI shows `00:00`.

3.  **Perform Seek**:
    -   Press `l` (Forward 30s) or `L` (Forward 60s).
    -   Wait for buffering.
    -   **Expected**: TUI time display shows `00:30` (or `01:00`) and continues incrementing from there.
    -   **Actual (Before Fix)**: TUI time display would reset to `00:00`.

4.  **Perform Multiple Seeks**:
    -   Seek forward multiple times.
    -   **Expected**: TUI time display correctly accumulates all seek offsets.
