# Research: Castnow Feature Integration

## Minimal HTTP Server for Local Streaming

**Goal**: Host local files for Chromecast playback using only `tokio` and standard library.

### Key Requirements
- **Protocol**: HTTP/1.1 (minimal implementation).
- **Headers**: 
  - `Content-Type`: Map file extensions to MIME types (mp4, m4v, mp3, webm, etc.).
  - `Content-Length`: Total size of the file.
  - `Accept-Ranges: bytes`: Signal support for seeking.
  - `Content-Range`: Response header for partial content.
- **Seeking**: Handle `Range: bytes=start-end` requests. If `end` is missing, serve until EOF.

### Implementation Strategy
1. Use `tokio::net::TcpListener` to accept connections.
2. For each connection, read the request line (e.g., `GET /media HTTP/1.1`).
3. Parse the `Range` header if present.
4. Open the file with `tokio::fs::File`.
5. Seek to the start position.
6. Use `tokio::io::copy_with_buffer` or a manual loop to stream the requested byte range.
7. Return `200 OK` for full requests or `206 Partial Content` for range requests.

### Rationale
Avoids heavy dependencies like `axum` or `hyper`, keeping the binary size small and fulfilling the "Dependency Minimalism" principle.

---

## Interactive CLI (TUI) with Keystroke Handling

**Goal**: Non-blocking keyboard input during playback.

### Options
1. **Raw Terminal Mode + `crossterm`**: Best cross-platform support. `crossterm` is the industry standard for Rust TUIs.
2. **`tokio::io::stdin()`**: Difficult to handle raw mode (needed for single keystrokes without Enter) without platform-specific code.

### Decision
Use `crossterm` for terminal manipulation and event handling. 
- *Rationale*: Implementing cross-platform raw mode manually is complex and error-prone. `crossterm` is lightweight enough and highly reliable.

---

## mDNS Discovery Enhancements

**Goal**: Integration with async event loop.

### Findings
- `mdns-sd` is already in use.
- Current implementation is blocking (`recv_timeout` loop).
- **Update Strategy**: Wrap discovery in a `tokio::sync::mpsc` channel or use `tokio::task::spawn_blocking` to prevent blocking the main TUI/Protocol loop.

---

## MIME Type Mapping

**Goal**: Minimal mapping for common media files.

### Mapping Table
| Extension | MIME Type |
|-----------|-----------|
| .mp4      | video/mp4 |
| .m4v      | video/mp4 |
| .mkv      | video/x-matroska |
| .webm     | video/webm |
| .mp3      | audio/mpeg |
| .ogg      | audio/ogg |
| .wav      | audio/wav |

*Default*: `application/octet-stream` if unknown.
