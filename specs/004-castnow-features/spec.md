# Feature Specification: Castnow Feature Integration

**Feature Branch**: `004-castnow-features`  
**Created**: 2026-01-14  
**Status**: Draft  
**Input**: User description: "castnow (https://github.com/xat/castnow) の主要機能を castru の仕様に統合してください。 以下の要件を既存の spec.md に追加し、Rust での実装方針を具体化してください： 1. ローカルファイルのストリーミング: - 指定したローカル動画ファイルをストリーミングするための最小限の HTTP サーバー機能。 - 依存を抑えるため、既存の tokio を活用したシンプルな実装を目指す。 2. インタラクティブ CLI: - 実行中にキーボード入力（スペースで一時停止、矢印でシーク等）を受け付ける TUI 要素。 3. デバイス自動発見: - mDNS を用いてネットワーク上の Chromecast をスキャンする機能。 4. メディア・プレイリスト管理: - 複数ファイルまたは URL の連続再生機能。 技術選定の制約「極力外部 crate を使わない」を維持しつつ、ストリーミングに必要な MIME タイプの判定や HTTP ヘッダー処理の最小構成を定義してください。"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Stream Local Media (Priority: P1)

As a user, I want to cast a video file stored on my computer to a Chromecast device so that I can watch it on a larger screen.

**Why this priority**: This is the core functionality that bridges local content with the Chromecast ecosystem, representing the primary value proposition of the integration.

**Independent Test**: Can be tested by providing a path to a local MP4 file and verifying that it starts playing on a specified Chromecast device.

**Acceptance Scenarios**:

1. **Given** a valid local video file and a target Chromecast IP, **When** the cast command is executed, **Then** a local HTTP server starts and the Chromecast begins playback of the file.
2. **Given** a non-existent file path, **When** the cast command is executed, **Then** the system provides a clear error message and exits gracefully.

---

### User Story 2 - Interactive Playback Control (Priority: P2)

As a user, I want to control the playback (pause, resume, seek) using my keyboard while the media is playing, so that I don't have to use another device or restart the stream to make adjustments.

**Why this priority**: Enhances the user experience by providing immediate control, making the CLI feel like a real media player.

**Independent Test**: During an active casting session, pressing the Space key should toggle playback state on the TV.

**Acceptance Scenarios**:

1. **Given** an active playback session, **When** the Space key is pressed, **Then** the playback pauses if playing, or resumes if paused.
2. **Given** an active playback session, **When** the Right Arrow key is pressed, **Then** the playback position on the Chromecast advances by a set interval (e.g., 30 seconds).

---

### User Story 3 - Automatic Device Discovery (Priority: P1)

As a user, I want the system to automatically find available Chromecast devices on my network so that I don't have to manually look up and type in IP addresses.

**Why this priority**: Crucial for ease of use and "zero-config" experience.

**Independent Test**: Running the discovery command should list the names and IP addresses of all Chromecasts on the local network.

**Acceptance Scenarios**:

1. **Given** one or more Chromecasts are active on the local network, **When** the discovery process is initiated, **Then** a list of found devices (Friendly Name and IP) is displayed.
2. **Given** no Chromecasts are found, **When** the discovery process times out, **Then** the user is informed that no devices were detected.

---

### User Story 4 - Playlist Management (Priority: P3)

As a user, I want to provide multiple files or URLs to be played in sequence so that I can watch a series of videos without manual intervention between each one.

**Why this priority**: Useful for binge-watching or presentations, but less critical than basic playback.

**Independent Test**: Providing two small video files should result in the second one starting automatically after the first one finishes.

**Acceptance Scenarios**:

1. **Given** a list of two valid video files, **When** the first file reaches the "FINISHED" state, **Then** the system automatically begins casting the second file.

### Edge Cases

- **Network Disruption**: How does the system handle the local HTTP server becoming unreachable? (Assumed: Notify user and attempt reconnection or exit).
- **Unsupported Formats**: What happens if the Chromecast cannot decode the streamed local file? (Assumed: Chromecast will report an error via the media channel, which should be displayed in the TUI).
- **Port Conflicts**: What if the default port for the local HTTP server is already in use? (Assumed: System should attempt to find an available port automatically).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST discover Chromecast devices on the local network using mDNS (DNS-SD) without requiring manual IP entry.
- **FR-002**: System MUST host a minimal HTTP/1.1 server to serve local files.
- **FR-003**: System MUST support standard mechanisms for seeking functionality on the media player (e.g. byte-range requests).
- **FR-004**: System MUST automatically identify the media format of local files.
- **FR-005**: System MUST provide a responsive interface that allows user control during playback without interrupting the stream.
- **FR-006**: System MUST maintain a playback queue and automatically transition to the next item upon completion of the current one.
- **FR-007**: System MUST provide visual feedback to the user regarding current playback status and progress.

### Key Entities *(include if feature involves data)*

- **CastDevice**: Represents a discovered physical device, containing its identifying information and network location.
- **MediaSource**: Represents a single item to be played (Local File Path or Remote URL).
- **StreamServer**: The internal service responsible for making a `MediaSource` available to the device.
- **PlaybackSession**: Orchestrates the interaction between the media source, the target device, and the user's input.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Device discovery identifies all active target devices on a standard subnet within 5 seconds in 95% of attempts.
- **SC-002**: Time from user command to first frame appearing on the device is under 4 seconds for local files (on a standard Wi-Fi network).
- **SC-003**: Interface response to user control (e.g., Pause) is transmitted to the device in under 200ms.
- **SC-004**: System successfully serves local files of any common size (up to 4GB+) reliably and efficiently.

## Assumptions

- **A-001**: The user's computer and the target device are on the same local network and can communicate freely.
- **A-002**: The target device supports the format of the local files being streamed; no real-time transcoding is required.
- **A-003**: The local network has sufficient bandwidth to stream high-definition video from the user's computer to the device.
- **A-004**: The user has the necessary permissions to read the local files and bind to a network port for the streaming server.