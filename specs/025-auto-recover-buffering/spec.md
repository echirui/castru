# Feature Specification: Auto Recover Buffering

**Feature Branch**: `025-auto-recover-buffering`
**Created**: 2026-01-18
**Status**: Draft
**Input**: User description: "castを実行中に自動的にPAUSEになる場合あります。mp4意外のファイルをエンコーディングしている時に起きたりしていると思います。その場合PAUSEにはせずに、自動復旧する例えば、10秒SLEEPする、WAITING満たない別のステータスにして自動普及するようにしてください。"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Auto-Recovery from System Pause (Priority: P1)

As a user streaming content (especially non-mp4 files requiring transcoding), I want the system to automatically attempt recovery when playback unexpectedly pauses due to transcoding or network lag, so that I don't have to manually intervene.

**Why this priority**: Unintentional pauses disrupt the viewing experience and currently require manual resumption, which is annoying.

**Independent Test**: Simulate a transcoding lag or network delay that triggers a system-side pause. The player should enter a `WAITING` or `BUFFERING` state and automatically resume `PLAYING` after a short delay (e.g., 10 seconds).

**Acceptance Scenarios**:

1. **Given** a playing stream (transcoding), **When** the receiver reports a `PAUSED` state NOT initiated by the user, **Then** the application status changes to `WAITING` (or `BUFFERING`).
2. **Given** a playing stream ends prematurely (reports `FINISHED` but time < duration), **Then** the application status changes to `WAITING` instead of advancing to next track.
3. **Given** the application is in `WAITING` state, **When** a 10-second timer elapses, **Then** the application automatically reloads the media at the current position to resume.
4. **Given** the user explicitly paused the stream, **When** the receiver reports `PAUSED`, **Then** the application remains in `PAUSED` state and does NOT auto-recover.

---

### User Story 2 - Waiting Status Indication (Priority: P2)

As a user, I want to see a clear "WAITING" or "RECOVERING" status in the TUI when the system is attempting to auto-recover, so that I know the playback hasn't just stopped permanently.

**Why this priority**: Feedback is crucial to prevent the user from thinking the app has crashed.

**Independent Test**: Trigger the auto-recovery condition. The TUI status line should read "WAITING" or "RECOVERING" during the sleep period.

**Acceptance Scenarios**:

1. **Given** auto-recovery is active, **When** I look at the TUI, **Then** the status displays "WAITING" (or similar).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST detect when the Cast receiver transitions to `PAUSED` state.
- **FR-002**: The system MUST distinguish between a user-initiated pause (tracked via `user_paused` flag) and a system-initiated pause.
- **FR-003**: If a system-initiated pause is detected, the application state MUST transition to `WAITING`.
- **FR-004**: While in `WAITING` state, the system MUST wait for a predefined duration (e.g., 10 seconds).
- **FR-005**: After the wait duration, the system MUST automatically send a `PLAY` command to the receiver.
- **FR-006**: If the user presses pause during `WAITING`, the state MUST transition to `PAUSED` and cancel auto-recovery.

### Key Entities

- **PlaybackStatus**: Update/Verify enum supports `Waiting` or reusing `Buffering` with a sub-state/reason.
- **AppState**: Track `last_system_pause_time` or similar to manage the backoff/wait timer.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: System automatically sends a PLAY command within 12 seconds (10s wait + processing) of an unintended pause.
- **SC-002**: 100% of user-initiated pauses remain paused (zero false positives).
- **SC-003**: TUI displays the recovery status during the wait period.