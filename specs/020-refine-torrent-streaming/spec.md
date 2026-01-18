# Feature Specification: Refined Torrent Streaming Strategy

**Feature Branch**: `020-refine-torrent-streaming`  
**Created**: 2026-01-15  
**Status**: Draft  
**Input**: User description: "https://github.com/mafintosh/peerflixを参考にして、torrent を指定した時のダウンロード優先順位やstreamingの方法を再検討してください。"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Fast Start Playback (Priority: P1)

As a user, I want to start watching a torrent video as quickly as possible, so that I don't have to wait for minutes of buffering.

**Why this priority**: Essential for a good streaming experience. Peerflix achieves this by prioritizing pieces required for file headers and initial playback.

**Independent Test**: Can be fully tested by playing a large torrent and measuring the time from "Start" to "Playback Begins".

**Acceptance Scenarios**:

1. **Given** a new torrent is added, **When** the download begins, **Then** the first and last pieces of the largest media file should be requested first.
2. **Given** the initial pieces are downloaded, **When** the buffer reaches the minimum threshold, **Then** the Chromecast `LOAD` command should be triggered immediately.

---

### User Story 2 - Smooth Sequential Playback (Priority: P1)

As a user, I want to watch the video without stuttering or "Loading" interruptions, provided my internet speed is sufficient.

**Why this priority**: Core functionality of a streaming client. Sequential downloading ensures the data is available just in time for the playback head.

**Independent Test**: Play a 10-minute video and verify that pieces are downloaded in roughly increasing order of their index.

**Acceptance Scenarios**:

1. **Given** media is playing, **When** the background downloader picks next pieces, **Then** it should prioritize pieces immediately following the current playback position.

---

### User Story 3 - Responsive Seeking (Priority: P2)

As a user, I want to jump to a different part of the video and have it resume playing quickly, even if that part wasn't downloaded yet.

**Why this priority**: Critical for usability. Standard BitTorrent (rarest-first) is terrible for seeking; we need to re-prioritize pieces dynamically.

**Independent Test**: Seek to 50% progress in a large torrent and verify that the download priority shifts to that timestamp's byte offset.

**Acceptance Scenarios**:

1. **Given** playback is at 1:00, **When** the user seeks to 10:00, **Then** the torrent engine should immediately cancel low-priority requests and start requesting pieces at 10:00.

---

### Edge Cases

- **Poorly seeded torrents**: How does the system handle pieces that are unavailable? (Fallback to rarest-first or stall with a clear message).
- **Multiple files in torrent**: Ensuring the priority logic applies to the *selected* file and not just the whole torrent.
- **Rapid seeking**: Ensuring the priority queue doesn't get overwhelmed if a user clicks around the seekbar.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST prioritize the first and last pieces of the selected media file to enable metadata parsing and format identification.
- **FR-002**: System MUST implement a sequential downloading strategy that requests pieces in the order they appear in the file.
- **FR-003**: System MUST dynamically update piece priorities based on the current read position (playback head).
- **FR-004**: System MUST support "urgent" requests for pieces immediately needed for playback, overriding standard sequential order.
- **FR-005**: System MUST maintain a small "buffer window" of sequential pieces ahead of the playback head.

### Key Entities *(include if feature involves data)*

- **Piece Selector**: Logic that decides which piece to request next from peers.
- **Priority Queue**: A structure that ranks pieces by urgency (1: Seeking Head, 2: Initial/Last pieces, 3: Sequential Buffer, 4: Rest).
- **Stream Server**: Reports the current byte offset being read by the receiver to the Torrent Engine.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Initial playback starts in < 15 seconds for torrents with > 10 healthy seeds.
- **SC-002**: Piece download order matches file sequence > 90% of the time during active playback.
- **SC-003**: Time to resume playback after seeking to a non-downloaded section is < 8 seconds (assuming 50Mbps+ connection).
- **SC-004**: Memory usage for the piece priority queue remains below 50MB even for very large torrents (> 50GB).