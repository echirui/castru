# Feature Specification: Accurate Seek and Playback Synchronization

**Feature Branch**: `014-fix-decode-seek-sync`  
**Created**: 2026-01-15  
**Status**: Draft  
**Input**: User description: "decodeが発生した場合、seekが機能しません。あと、seekした際に再生時間が誤ったものになります。大抵は0になります。"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Seek during Transcoding (Priority: P1)

As a user, I want to be able to skip forward or backward in a video even if the system is currently transcoding the stream, so that I can find the specific part I want to watch without waiting for the entire file to be processed.

**Why this priority**: Core functionality. Seeking is a fundamental media control, and transcoding is a common state in this application.

**Independent Test**: Can be tested by playing a file that triggers transcoding and attempting multiple seek operations (short and long jumps).

**Acceptance Scenarios**:

1. **Given** a video is playing and being transcoded, **When** the user sends a seek command to a new position, **Then** the video should jump to that position and continue playing.
2. **Given** a video is playing and being transcoded, **When** a seek command is issued, **Then** the stream should not freeze or crash.

---

### User Story 2 - Accurate Playback Time after Seek (Priority: P1)

As a user, I want the playback time display to show the correct time immediately after I seek to a new position, so that I know exactly where I am in the video and how much is remaining.

**Why this priority**: Essential for user orientation. Incorrect time (like "0") breaks the TUI and user experience.

**Independent Test**: Seek to various positions (e.g., 50%, 90%, 10%) and verify that the reported playback time matches the seek target.

**Acceptance Scenarios**:

1. **Given** a video is playing, **When** the user seeks to 10:00 minutes, **Then** the playback time should show approximately 10:00 (allowing for minor buffering delay) instead of 0:00.
2. **Given** a video has been sought multiple times, **When** the playback continues, **Then** the time should advance correctly from the new seek position.

---

### Edge Cases

- **Seeking near the end of the file**: Does the system handle seeking to a position very close to the end during transcoding?
- **Rapid successive seeks**: How does the system handle multiple seek commands sent in quick succession while transcoding is active?
- **Seeking before transcoding starts**: What happens if a seek is requested during the initial buffering/transcoding setup phase?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST support seeking in media streams that are undergoing real-time transcoding/decoding.
- **FR-002**: System MUST accurately synchronize and report the new playback time to the control interface after a seek operation completes.
- **FR-003**: System MUST NOT reset the playback time to zero unless a seek to the beginning was explicitly requested.
- **FR-004**: System MUST maintain audio and video synchronization after a seek operation in a transcoded stream.
- **FR-005**: System MUST provide feedback to the user if a seek operation is temporarily delayed due to transcoding buffer updates.

### Key Entities *(include if feature involves data)*

- **Media Stream**: The data being played, which may be in a raw or transcoded format.
- **Playback State**: Includes the current position, duration, and whether the stream is being transcoded.
- **Seek Command**: A request containing the target timestamp for the new playback position.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Seek operations succeed 100% of the time when initiated during an active transcoding session.
- **SC-002**: The reported playback time is within +/- 1 second of the actual media timestamp within 500ms of the seek operation completing.
- **SC-003**: The media resumes playback at the new position within 3 seconds of the seek command being issued (assuming normal network conditions).
- **SC-004**: 0% occurrence of the playback timer resetting to "0" unexpectedly after a seek to a non-zero position.