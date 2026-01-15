# Feature Specification: Torrent Streaming Support

**Feature Branch**: `011-add-torrent-support`
**Created**: 2026-01-15
**Status**: Draft
**Input**: "https://github.com/xat/castnow/blob/master/plugins/torrent.js のようにtorrentファイルに対応してください" (Support torrent files like castnow's torrent plugin)

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Stream from Magnet Link (Priority: P1)

As a user, I want to stream video content directly from a Magnet URI to my Cast device so that I don't have to wait for the entire file to download before watching.

**Why this priority**: Magnet links are the most common way to share large media files via BitTorrent.

**Independent Test**:
1.  Run the CLI with a known public domain movie magnet link (e.g., Big Buck Bunny).
2.  Verify playback starts on the Cast device within a reasonable buffering time.

**Acceptance Scenarios**:

1.  **Given** a valid Magnet URI containing a video file, **When** I execute the run command with the URI, **Then** the application resolves the metadata, identifies the video, and begins streaming to the device.
2.  **Given** the stream is playing, **When** I seek forward to a non-downloaded section, **Then** the application prioritizes downloading that section and resumes playback after buffering.

### User Story 2 - Stream from Torrent File (Priority: P2)

As a user, I want to stream video content from a local `.torrent` file.

**Why this priority**: Users may have archived torrent files or download them from private trackers.

**Independent Test**:
1.  Run the CLI with a path to a valid `.torrent` file.
2.  Verify playback starts on the Cast device.

**Acceptance Scenarios**:

1.  **Given** a local `.torrent` file path, **When** I execute the run command, **Then** the application parses the file and begins streaming the main video content.

### Edge Cases

- **No Video Found**: If the torrent contains no playable video files, the application should exit with a clear error message.
- **Multiple Video Files**: If multiple video files exist, the application should default to the largest one (consistent with typical "main movie" behavior).
- **Stalled Download**: If peers are insufficient to sustain playback, the application should pause/buffer, potentially notifying the user if progress halts completely.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST accept Magnet URIs (starting with `magnet:?`) as a valid media input.
- **FR-002**: The system MUST accept file paths to `.torrent` files as a valid media input.
- **FR-003**: The system MUST implement a BitTorrent client capability that supports sequential downloading (streaming) of pieces.
- **FR-004**: The system MUST automatically select the largest video file in the torrent for playback by default.
- **FR-005**: The system MUST support standard HTTP streaming of the downloading content to the Cast device (allowing the Cast device to request ranges).
- **FR-006**: The system MUST prioritize downloading pieces requested by the playback position (seeking support).

### Key Entities

- **Torrent Engine**: Manages peer connections, metadata resolution, and piece downloading.
- **Torrent Stream**: Adapts the torrent download process into a readable stream/HTTP endpoint for the Cast device.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Playback initiation (time from metadata resolution to first frame) is under 30 seconds for a well-seeded public torrent.
- **SC-002**: Users can seek to the middle of a 1-hour video and resume playback within 20 seconds (assuming adequate bandwidth/peers).
- **SC-003**: The application successfully cleans up temporary download data (or offers to keep it) upon termination. *Assumption: Default behavior is ephemeral or specified by a flag, but for MVP we ensure it doesn't leave orphaned large files without user knowledge.*

## Assumptions

- The user has an internet connection that allows BitTorrent traffic.
- The Cast device is on the same local network.
- The "main" content is defined as the largest file with a video extension.