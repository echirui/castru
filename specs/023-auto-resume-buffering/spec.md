# Feature Specification: Auto Resume Buffering

**Feature Branch**: `023-auto-resume-buffering`
**Created**: 2026-01-18
**Status**: Draft
**Input**: User description: "ダウンロードやエンコーディングが間に合わなくなるとPAUSEにするのではなく、BUFFERINGとかにして、自動で復帰するようにしてください。PAUSEはスペースキーを押した時飲み遷移するようにしてください。"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Automatic Buffering Recovery (Priority: P1)

As a user streaming media, I want the player to enter a "BUFFERING" state automatically if download or encoding lags behind playback, and automatically resume "PLAYING" once sufficient data is available, so that I don't have to manually unpause playback.

**Why this priority**: Currently, lag causes the player to pause indefinitely or requires manual intervention, disrupting the viewing experience.

**Independent Test**: Simulate a network slowdown or high CPU load during playback. The player status should change to `BUFFERING` and then back to `PLAYING` without user input.

**Acceptance Scenarios**:

1. **Given** a playing stream, **When** the download buffer falls below a critical threshold, **Then** the player status changes to `BUFFERING`.
2. **Given** a player in `BUFFERING` state, **When** the buffer fills up to a safe threshold, **Then** the player status automatically changes to `PLAYING`.
3. **Given** a playing stream, **When** the encoder cannot keep up, **Then** the player enters `BUFFERING` until encoding catches up.

### User Story 2 - Explicit User Pause (Priority: P2)

As a user, I want the "PAUSED" state to be reserved only for when I explicitly press the spacebar (or other pause controls), so that I can distinguish between intentional pauses and network/performance issues.

**Why this priority**: Avoids confusion where a user thinks they paused the video but it was actually a buffer underrun, or vice versa.

**Independent Test**: Press spacebar during playback. Status becomes `PAUSED`. Wait. Status remains `PAUSED` regardless of buffer state until spacebar is pressed again.

**Acceptance Scenarios**:

1. **Given** playing media, **When** the user presses the spacebar, **Then** the status changes to `PAUSED`.
2. **Given** a `PAUSED` state initiated by the user, **When** the buffer fills completely, **Then** the status remains `PAUSED`.
3. **Given** a `BUFFERING` state, **When** the user presses the spacebar, **Then** the status changes to `PAUSED` (user overrides buffering).

---

### Edge Cases

- **Recovering from long buffering**: If buffering takes too long (> 30s), should it timeout? (Currently, keep buffering until data arrives).
- **End of File**: Ensure EOF doesn't trigger infinite buffering.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST detect when playback position approaches the end of the buffered content (underrun).
- **FR-002**: Upon detecting underrun, the system MUST automatically transition to a `BUFFERING` state.
- **FR-003**: While in `BUFFERING` state, the system MUST monitor buffer growth.
- **FR-004**: When the buffer reaches a safe resumption threshold (e.g., 5-10 seconds of content), the system MUST automatically transition to `PLAYING` state.
- **FR-005**: The system MUST NOT transition to `PAUSED` state automatically due to data starvation.
- **FR-006**: The `PAUSED` state MUST only be entered via explicit user command (TUI or API).

### Key Entities

- **PlaybackStatus**: Enum needs to clearly distinguish `Buffering` from `Paused`.
- **AppState**: Needs to track if the current "pause" is user-initiated or system-initiated (if underlying player logic uses "pause" for buffering).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Playback resumes automatically 100% of the time after a buffer underrun event resolves.
- **SC-002**: User-initiated `PAUSED` state never automatically transitions to `PLAYING`.
- **SC-003**: `BUFFERING` status is displayed in the TUI during underrun events.