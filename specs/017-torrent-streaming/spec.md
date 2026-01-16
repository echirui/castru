# Feature Specification: Torrent Streaming while Downloading

**Feature Branch**: `017-torrent-streaming`  
**Created**: 2026-01-15  
**Status**: Draft  
**Input**: User description: "torrentでダウンロード中からcastするように修正してください。100%行かなくても再生可能であればstreamingしてください。"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Stream Torrent during Download (Priority: P1)

As a user, I want to start watching a movie from a torrent as soon as enough data is buffered, so that I don't have to wait for the entire file to download.

**Why this priority**: High. This provides immediate value and improves the user experience by reducing wait time.

**Independent Test**: Can be tested by playing a large torrent and verifying that playback starts well before the download reaches 100%.

**Acceptance Scenarios**:

1. **Given** a magnet link is provided, **When** the download progress reaches a minimum threshold (e.g., 3% or 10MB), **Then** the application should initiate the `LOAD` command to the Chromecast.
2. **Given** playback has started, **When** the download continues in the background, **Then** the stream should remain stable as long as the download speed exceeds the playback bitrate.

---

### User Story 2 - Monitor Progress during Streaming (Priority: P1)

As a user, I want to see the current download percentage even while the video is playing, so that I can monitor the health of the download and know if I might hit a buffering wall.

**Why this priority**: High. Essential feedback for the user to understand the state of the background download.

**Independent Test**: While a torrent is streaming, observe the TUI to ensure the download percentage is visible alongside playback time.

**Acceptance Scenarios**:

1. **Given** a torrent is being streamed, **When** the TUI is active, **Then** it should display both the playback progress and the background download progress.

---

### User Story 3 - Buffer Underrun Handling (Priority: P2)

As a user, I want the system to handle situations where playback catches up to the download progress, so that the stream doesn't crash or show corrupted data.

**Why this priority**: Medium. Important for robustness when network speed is inconsistent.

**Independent Test**: Limit the download speed so that playback catches up. The stream should pause or buffer until more data is available.

**Acceptance Scenarios**:

1. **Given** playback reaches the current end of downloaded data, **When** more data is still being fetched, **Then** the `GrowingFile` implementation should block/wait until the requested bytes are available.

---

### Edge Cases

- **Seeking to non-downloaded parts**: If the user seeks to a part of the video that hasn't been downloaded yet, the system should prioritize those pieces and pause the stream until they arrive.
- **Slow download speed**: If the download is consistently slower than playback, the user should be warned or the stream should stay in a "Buffering" state.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST define a "Playable" threshold (e.g., 3% or 10MB) to trigger playback before 100% completion.
- **FR-002**: System MUST continue downloading the remainder of the torrent in the background after playback starts.
- **FR-003**: System MUST update and display the download percentage in the TUI status line or metadata area during playback.
- **FR-004**: System MUST ensure the `GrowingFile` adapter blocks reading when requested data is not yet available on disk.
- **FR-005**: System MUST prioritize pieces requested by the playback head (sequential downloading).

### Key Entities *(include if feature involves data)*

- **Torrent Handle**: The internal reference to the active download session.
- **Download Progress**: Calculated as (bytes_downloaded / total_bytes) * 100.
- **Playback Position**: The current byte offset being read by the HTTP server.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Playback initiates within 30 seconds for well-seeded torrents.
- **SC-002**: TUI displays both playback time and download percentage concurrently.
- **SC-003**: 0% stream corruption occurrences when playback hits the download edge (it should block/wait instead).
- **SC-004**: Successful playback of a 1GB file with only 50MB downloaded initially.