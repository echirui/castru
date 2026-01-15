# Data Model: Accurate Seek and Playback Synchronization

## AppState (in main.rs)

The `AppState` struct manages the overall state of the application and TUI.

| Field | Type | Description |
|-------|------|-------------|
| `is_transcoding` | `bool` | True if the current stream is being transcoded via FFmpeg. |
| `seek_offset` | `f64` | The absolute start time (in seconds) of the current stream fragment. |
| `current_time` | `f64` | The absolute playback time (Reported Time + Seek Offset). |
| `total_duration` | `Option<f64>` | Total duration of the media in seconds. |
| `media_session_id` | `Option<i32>` | The active media session ID on the Chromecast. |
| `last_reported_time` | `f64` | The raw `currentTime` from the last `MEDIA_STATUS` update. |

## TranscodingPipeline (in transcode.rs)

| Field | Type | Description |
|-------|------|-------------|
| `process` | `Child` | The running FFmpeg process. |
| `stdout` | `ChildStdout` | The pipe providing the transcoded data. |

## State Transitions

1.  **Idle -> Loading**: `load_media` called. `seek_offset` initialized to 0.0 (or target seek time).
2.  **Loading -> Playing**: `MEDIA_STATUS` received with `PLAYING` state. `media_session_id` captured.
3.  **Playing -> Seeking (Transcoded)**:
    - User presses Arrow key.
    - `new_time` calculated.
    - Existing `ffmpeg` killed.
    - New `ffmpeg` spawned with `-ss new_time`.
    - `seek_offset` updated to `new_time`.
    - `app.load` called with new URL.
4.  **Playing -> Connection Lost**:
    - Watchdog detects disconnect.
    - App attempts to re-load from `last_known_time`.
    - `load_media` called with `start_time = last_known_time`.
