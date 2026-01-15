# Research: Fix Transcode Seek Synchronization

**Date**: 2026-01-15
**Feature**: `013-fix-transcode-seek-sync`

## 1. Root Cause Analysis

### Playback Time Reporting
- When a file is transcoded, `ffmpeg` starts the output stream from the requested offset (`-ss`).
- The Google Cast device treats this as a new stream starting at timestamp `0.0`.
- The `MEDIA_STATUS` events returned by the device report `current_time` relative to the start of this specific stream.
- The TUI currently overwrites its local `current_time` with the value from `MEDIA_STATUS` without considering that the stream itself is offset from the start of the file.

### Cumulative Drift
- Every time a seek is performed, the internal "start" of the stream shifts.
- If multiple seeks occur, the `current_time` seen by the user resets to 0 repeatedly.

## 2. Proposed Solution

### Maintain `seek_offset`
- The `AppState` in `main.rs` must include a `seek_offset: f64` field.
- When `load_media` is called for a transcoded stream with a non-zero `start_time`, this `start_time` becomes the new `seek_offset`.

### Display Adjustment
- When receiving `MEDIA_STATUS`:
  - `display_time = reported_time + seek_offset`
- When rendering the progress bar:
  - Use `display_time / total_duration`.

### Transcoding Pipeline
- `ffmpeg -ss` is already used in `src/transcode.rs`. This is correct for the data stream.
- The `StreamServer` correctly pipes the new stream.

## 3. Implementation Details

### Changes in `src/main.rs`
- Add `seek_offset` to `AppState`.
- Update `load_media` to accept and handle `seek_offset`.
- Update the event loop to use `seek_offset` when updating `app_state.current_time`.
- Ensure `SeekForward` and `SeekBackward` update the `seek_offset` appropriately.

### Precision
- `ffmpeg` seeking with `-ss` before `-i` is fast (input seeking) but may not be frame-accurate depending on keyframes.
- Output seeking (after `-i`) is accurate but slow.
- Given "ultrafast" preset and the nature of live streaming, input seeking is preferred for responsiveness. We will stick with current `-ss` placement but be aware of minor drift (usually < 1s).
