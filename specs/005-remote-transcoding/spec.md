# Feature Specification: Remote Control and Transcoding Pipeline

**Feature Branch**: `005-remote-transcoding`  
**Created**: 2026-01-14  
**Status**: Draft  
**Input**: User description: "## 6. サーバサイド・コントロール (Remote Control) - **対話型インターフェース**: `tokio::io::stdin` を用い、サーバ実行中のキーボード入力（Space, 左右矢印など）を即座に CastMessage として送信する。 - **外部 API (オプション)**: 必要に応じて、制御用の簡易的な HTTP エンドポイントを公開し、ブラウザや他端末からの操作を可能にする。 ## 7. トランスコーディング・パイプライン - **ffmpeg 統合**: 標準ライブラリの `std::process::Command` を使用し、外部の `ffmpeg` プロセスを起動する。 - **オンデマンド変換**: - 入力ファイルのコーデックを `ffprobe` で事前判定。 - 必要に応じて、H.264 8bit (yuv420p) / AAC 44.1kHz の MP4 ストリームにリアルタイム変換し、HTTP レスポンスとしてパイプ出力する。 - **制約**: トランスコード中はシーク（Seek）機能を制限するか、ffmpeg の `-ss` オプションを用いた擬似シークを実装する。"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Interactive Keyboard Control (Priority: P1)

As a user running the cast server, I want to control playback directly from my terminal using the keyboard so that I can quickly pause or seek without switching to another device.

**Why this priority**: Core functionality for a CLI-based casting tool to provide immediate feedback and control.

**Independent Test**: Can be tested by running the server, playing a video, and pressing 'Space' to toggle pause. The cast device should respond immediately.

**Acceptance Scenarios**:

1. **Given** a media file is currently casting, **When** the user presses 'Space', **Then** the playback on the target device toggles between play and pause.
2. **Given** a media file is currently casting, **When** the user presses the 'Left' or 'Right' arrow keys, **Then** the playback position on the target device jumps backward or forward by a fixed interval (e.g., 10 seconds).

---

### User Story 2 - Automatic Transcoding for Compatibility (Priority: P1)

As a user, I want to cast media files even if their format is not natively supported by the receiver so that I don't have to manually convert files before casting.

**Why this priority**: Essential for ensuring a seamless user experience across a wide variety of media formats.

**Independent Test**: Attempt to cast a file with an unsupported codec (e.g., a high-bitrate MKV or a file with AC3 audio if the receiver doesn't support it). The system should automatically start transcoding and the video should play on the receiver.

**Acceptance Scenarios**:

1. **Given** a media file with an unsupported codec, **When** the user starts casting, **Then** the system detects the incompatibility using `ffprobe` and initiates an `ffmpeg` transcoding process.
2. **Given** a transcoding process is active, **When** the receiver requests the media stream, **Then** the system pipes the transcoded H.264/AAC MP4 data to the receiver in real-time.

---

### User Story 3 - Seeking during Transcoding (Priority: P2)

As a user watching a transcoded video, I want to be able to seek to different parts of the video so that I can skip content I've already seen or re-watch a specific part.

**Why this priority**: Seeking is a basic expectation for media playback, though technically challenging for live-transcoded streams.

**Independent Test**: While a transcoded video is playing, use the keyboard or external control to seek forward. The stream should restart from the new position after a short buffering period.

**Acceptance Scenarios**:

1. **Given** a video is being transcoded and streamed, **When** the user requests a seek to a specific timestamp, **Then** the system restarts the `ffmpeg` process using the `-ss` option to begin transcoding from the requested offset.

---

### User Story 4 - External Remote Control API (Priority: P3)

As a user, I want to control the cast session from a web browser or another device on my network so that I am not tied to the terminal where the server is running.

**Why this priority**: Optional enhancement for convenience and multi-device workflows.

**Independent Test**: Send an HTTP GET/POST request to a control endpoint (e.g., `/api/pause`) while a video is playing. The playback should pause.

**Acceptance Scenarios**:

1. **Given** the server is running with the optional API enabled, **When** an external HTTP request is received on a control endpoint, **Then** the server translates the request into the corresponding `CastMessage` and sends it to the receiver.

### Edge Cases

- **Incompatible Format but Transcoding Fails**: What happens if `ffmpeg` cannot process the source file? (System should notify the user and stop the cast).
- **Seek in Near-End of Stream**: How does the system handle a seek to the very end of a transcoded stream?
- **Missing Dependencies**: What if `ffmpeg` or `ffprobe` is not installed on the user's system? (System should detect this at startup and provide a clear error message).
- **Format already compatible**: If the source file is already H.264/AAC, the system should bypass transcoding and stream the file directly.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST capture keyboard input from `stdin` asynchronously without blocking the main event loop.
- **FR-002**: System MUST identify video and audio codecs of the input file using `ffprobe` before starting the cast session.
- **FR-003**: System MUST trigger `ffmpeg` transcoding if the source codecs do not match the target profile (H.264 8bit yuv420p / AAC 44.1kHz).
- **FR-004**: System MUST stream transcoded output via HTTP as a non-seekable (or pseudo-seekable) MP4 stream.
- **FR-005**: System MUST support "pseudo-seek" by killing and restarting the transcoding process with the `-ss` flag at the target timestamp.
- **FR-006**: System MAY provide a REST-like HTTP API for remote control actions (Play, Pause, Seek, Stop).

### Assumptions & Dependencies

- **External Tools**: The system assumes `ffmpeg` and `ffprobe` are available in the system's PATH.
- **Network Stability**: Remote control responsiveness depends on a stable local network connection between the server and the Cast device.
- **Hardware Performance**: Transcoding performance (real-time conversion) depends on the host machine's CPU capabilities.

### Key Entities *(include if feature involves data)*

- **Transcoding Pipeline**: Manages the lifecycle of the `ffmpeg` subprocess, including stdin/stdout piping and error handling.
- **Media Controller**: Interprets user inputs (keyboard or API) and dispatches appropriate commands to the Cast device.
- **Probe Result**: Data structure representing the technical characteristics of the source media file (codecs, bitrate, duration).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Latency between a local keypress (Space) and the execution of the command on the Cast device MUST be less than 250ms.
- **SC-002**: Transcoding initialization (from file selection to first byte of stream) MUST complete in under 2 seconds for local files.
- **SC-003**: Pseudo-seek operations in transcoded streams MUST resume playback at the target position within 3 seconds.
- **SC-004**: System MUST successfully transcode at least 95% of common video formats (MKV, AVI, MOV) into the target H.264/AAC profile.