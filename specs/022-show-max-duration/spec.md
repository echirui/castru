# Feature Specification: Show Max Duration

**Feature Branch**: `022-show-max-duration`
**Created**: 2026-01-18
**Status**: Draft
**Input**: User description: "magnet でダウンロードした後、動画の最大再生時間が表示されるようにしてください。"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - View Total Duration for Torrent Media (Priority: P1)

As a user streaming a video via magnet link, I want to see the total duration of the video once metadata is available, so that I know how long the content is and can seek accurately.

**Why this priority**: Currently, torrent streams might show unknown duration until fully downloaded or probed, which degrades the user experience.

**Independent Test**: Stream a magnet link. Once playback starts or buffering reaches a sufficient level, the TUI should display the correct total duration (e.g., "00:10:30") instead of "Unknown" or "00:00:00".

**Acceptance Scenarios**:

1. **Given** a magnet link for a video file, **When** the application resolves the metadata and starts buffering, **Then** the total duration is extracted and displayed in the TUI.
2. **Given** a multi-file torrent, **When** the main video file is identified, **Then** the duration corresponds to that specific video file.

---

### Edge Cases

- What happens when `ffprobe` fails to determine duration? (Fallback to "Unknown" or estimate based on size/bitrate if possible, but "Unknown" is acceptable MVP).
- How does system handle very large files where probing might take time? (Should be non-blocking).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST probe the media file for duration once enough data is downloaded (e.g., header).
- **FR-002**: The `TorrentManager` or `MediaController` MUST update the `AppState` with the discovered total duration.
- **FR-003**: The TUI MUST render the total duration formatted as `HH:MM:SS`.
- **FR-004**: Probing MUST be performed asynchronously to avoid blocking the download or playback loops.

### Key Entities

- **MediaProbeResult**: Existing entity, needs to be utilized effectively for torrent sources.
- **AppState**: Needs to store and update `total_duration` dynamically.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Total duration is displayed within 10 seconds of the first bytes being downloaded for 90% of standard video torrents.
- **SC-002**: Duration accuracy is within 1 second of the actual file duration.
- **SC-003**: No UI freezes occur during the probing process.