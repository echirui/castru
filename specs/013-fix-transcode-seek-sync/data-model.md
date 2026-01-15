# Data Model: Fix Transcode Seek Synchronization

## AppState (Modified)

Internal state of the CLI application.

| Field | Type | Description |
|-------|------|-------------|
| `is_transcoding` | `bool` | Whether the current stream is being transcoded. |
| `seek_offset` | `f64` | The timestamp (in seconds) where the current transcoding stream started. Default: `0.0`. |
| `current_time` | `f64` | The absolute playback time (including `seek_offset`). |
| `total_duration` | `Option<f64>` | Total duration of the media file. |

## TranscodeConfig (Existing)

Config used to spawn `ffmpeg`.

| Field | Type | Description |
|-------|------|-------------|
| `start_time` | `f64` | The offset passed to `-ss`. |
| `input_path` | `PathBuf` | Path to the source file. |
| `target_video_codec` | `String` | Output video codec (e.g., `libx264`). |
| `target_audio_codec` | `String` | Output audio codec (e.g., `aac`). |
