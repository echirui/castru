# Research: Accurate Seek and Playback Synchronization

**Feature**: `014-fix-decode-seek-sync`

## 1. Problem Analysis

### Seek Failure during Transcoding
When `ffmpeg` is used for transcoding, the output stream is often non-seekable from the Chromecast's perspective (HTTP `Transfer-Encoding: chunked`). Currently, when a user seeks, the `main.rs` event loop calls `load_media` again with a new `start_time`.
The issue arises because:
1.  The `StreamServer` might not be correctly resetting its internal state when a new transcoding pipeline is set.
2.  Chromecast might be confused by the sudden change in the stream if the previous connection isn't properly terminated.

### Incorrect Playback Time
`ffmpeg` output timestamps always start from `0.0` regardless of the `-ss` offset used for input seeking.
- Current logic in `main.rs` tries to add a `seek_offset` to the `reported_time`.
- However, if the `seek_offset` isn't updated correctly or if the timing of the update is off, the TUI shows `0:00` or an incorrect value.

## 2. Proposed Solution

### Resume Logic for Connection Errors
The user requested: "別のcodecに変換した場合、connectionの状態を確認し必要があれば、現状の再生時間から接続をレジュームするようにして欲しい" (When converting to a different codec, check the connection status and, if necessary, resume the connection from the current playback time).

This implies:
1.  **Monitoring Connection**: Detect when the Chromecast disconnects or fails to load.
2.  **State Persistence**: Keep track of `current_time` even if the app crashes or the connection drops.
3.  **Automatic Resume**: If a connection is lost during playback, attempt to reconnect and call `load_media` with the last known `current_time`.

### Improved `seek_offset` Management
1.  Ensure `load_media` returns the *actual* offset used.
2.  Update `AppState` atomically when a new media is loaded.
3.  Handle `MEDIA_STATUS` updates more robustly by verifying the `media_session_id`.

## 3. Technical Implementation Details

### ffmpeg Seeking
Input seeking (`ffmpeg -ss ... -i ...`) is fast because it jumps to the nearest keyframe. This is what we currently use.
To improve accuracy, we should ensure that the `seek_offset` matches exactly what was passed to `-ss`.

### StreamServer Reset
`StreamServer::set_transcode_output` currently calls `clear_transcode`, which kills the process and clears the `stdout` pipe. This should be sufficient, but we must ensure that any existing HTTP connections are closed so the receiver is forced to reconnect.

## 4. Decision Log

| Decision | Rationale |
|----------|-----------|
| Use `seek_offset` tracking | Necessary because `ffmpeg` resets timestamps to 0. |
| Use "Pseudo-seek" (Restart ffmpeg) | Real-time transcoding cannot be easily seeked within a single process/stream. |
| Implement Connection Watchdog | To support the "resume from current time" requirement when connections fail. |
