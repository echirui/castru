# Research: Remote Control and Transcoding Pipeline

## 1. Terminal Control (Asynchronous Keyboard Input)

### Decision: Use `crossterm` (minimal features)
**Rationale**: 
- Standard `tokio::io::stdin` is line-buffered by the OS. Reading single keypresses (like Space or Arrows) requires putting the terminal into "raw mode".
- While `termios` (Unix) or `winapi` (Windows) could be used directly, `crossterm` is a well-vetted, cross-platform, and relatively lean crate that handles this complexity safely.
- It integrates with `tokio` via the `crossterm::event::EventStream`.
- **Justification for Constitution**: Avoids re-implementing complex platform-specific terminal handling and provides a cleaner async API than raw syscalls.

**Alternatives considered**: 
- `termion`: Good, but Unix-only.
- Raw `termios`: Hard to maintain and cross-platform.

---

## 2. FFmpeg Pipeline for Cast Streaming

### Decision: Use fragmented MP4 (fMP4)
**Rationale**:
- Chromecast requires MP4 to have the "moov" atom at the beginning for non-fragmented files. For a live transcode, we don't know the full duration or "moov" content ahead of time.
- **Flags**: `ffmpeg -i [input] -c:v libx264 -b:v 3M -profile:v high -level 4.1 -pix_fmt yuv420p -c:a aac -ac 2 -ar 44100 -f mp4 -movflags frag_keyframe+empty_moov+default_base_moof pipe:1`
- This produces a stream of fragments that the receiver can start playing immediately.

---

## 3. Pseudo-seek Implementation

### Decision: Restart FFmpeg with `-ss` and notify Receiver
**Rationale**:
- When a user seeks, we kill the existing `ffmpeg` process.
- We start a new `ffmpeg` process with `-ss [timestamp]` *before* the `-i` flag (input seeking is faster).
- **Challenge**: The HTTP connection from the Chromecast might need to be reset, or the server needs to handle the stream continuity.
- **Approach**: The most reliable way is to tell the Chromecast to "Load" the URL again with a query parameter (e.g., `?start=[timestamp]`). This forces the receiver to establish a new connection, and the server starts `ffmpeg` at that point.

---

## 4. Codec Detection

### Decision: Use `ffprobe` JSON output
**Rationale**:
- `ffprobe -v error -show_entries stream=codec_name,codec_type -of json [file]`
- Parse the JSON to check if `v:codec == h264` and `a:codec == aac`.
- If both match, we can stream the file directly (no transcode).
