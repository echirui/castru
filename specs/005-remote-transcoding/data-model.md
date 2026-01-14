# Data Model: Remote Control and Transcoding

## Entities

### 1. MediaProbeResult
Represents the technical details of a media file.
- `video_codec`: String (e.g., "h264", "hevc")
- `audio_codec`: String (e.g., "aac", "ac3")
- `duration`: Option<f64> (Seconds)
- `needs_transcoding`: Boolean (Calculated: true if codecs != h264/aac)

### 2. TranscodeConfig
Configuration for the FFmpeg process.
- `input_path`: PathBuf
- `start_time`: f64 (Offset in seconds for seeking)
- `target_video_codec`: "libx264"
- `target_audio_codec`: "aac"

### 3. PlaybackCommand (Enum)
Commands sent from the controller to the Cast receiver.
- `Play`
- `Pause`
- `TogglePause`
- `Seek(f64)` (Relative or Absolute offset)
- `Stop`

## State Transitions (Transcoding)

1. **Idle**: No media being processed.
2. **Probing**: Running `ffprobe`.
3. **Streaming (Direct)**: Serving file content directly via HTTP.
4. **Streaming (Transcoding)**: Running `ffmpeg`, piping output to HTTP response.
5. **Seeking**: Killing current `ffmpeg`, restarting at new offset, transitioning back to **Streaming (Transcoding)**.
