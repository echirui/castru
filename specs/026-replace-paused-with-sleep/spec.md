# Feature Specification: Replace PAUSED with Auto-Retry Waiting State

**Feature Branch**: `026-replace-paused-with-sleep`  
**Created**: 2026-01-23  
**Status**: Draft  
**Input**: User description: "PAUSEDをすべて、削除しsleep 10 を実施するステータスに変更してください。" (Remove all PAUSED and change to a status that executes sleep 10.)

## User Scenarios & Testing

### User Story 1 - Eliminate Static Pause State (Priority: P1)

As a user, I want the player to never remain in a static `PAUSED` state indefinitely, regardless of whether the pause was triggered by the system or myself, so that playback always attempts to resume automatically after a short delay.

**Why this priority**: The user explicitly requested to remove "all" PAUSED states and replace them with a "sleep 10" status, indicating a desire for a self-healing or always-active playback loop.

**Independent Test**: Trigger a pause (system or manual) and verify the system enters a `WAITING` state and resumes after 10 seconds.

**Acceptance Scenarios**:

1. **Given** the player is playing, **When** a system event (e.g., buffering, interruption) causes a pause, **Then** the status transitions to `WAITING` (not `PAUSED`), waits for 10 seconds, and then attempts to resume playback.
2. **Given** the player is playing, **When** the user triggers the "Pause" command, **Then** the status transitions to `WAITING` (not `PAUSED`), waits for 10 seconds, and then attempts to resume playback automatically.
3. **Given** the player is in `WAITING` state, **When** the 10-second timer expires, **Then** the player triggers a reload or play command to resume content.

### User Story 2 - Immediate Resume from Waiting (Priority: P2)

As a user, I want to be able to manually resume playback immediately if the player is in the `WAITING` state, so I don't have to wait for the full 10 seconds if I'm ready.

**Why this priority**: Improves UX by allowing manual override of the wait timer.

**Independent Test**: Enter `WAITING` state, press Play, verify immediate resumption.

**Acceptance Scenarios**:

1. **Given** the player is in `WAITING` state (counting down 10s), **When** the user presses the "Play" or "Toggle Play" key, **Then** the player immediately attempts to resume playback and transitions to `PLAYING` (or `BUFFERING`) without waiting for the timer.

### Edge Cases

- **Connection Failure during Retry**: If the resume attempt fails after 10s, does it go back to `WAITING`? (Assumption: Yes, it loops).
- **Stop Command**: Does `Stop` still strictly stop the player? (Assumption: Yes, `Stop` should behave as a hard stop/exit, not a pause).

## Requirements

### Functional Requirements

- **FR-001**: The system MUST NOT use a static `PAUSED` status that requires manual intervention to resume indefinitely.
- **FR-002**: The system MUST replace all transitions to `PAUSED` with a transition to a `WAITING` (or similarly named) status.
- **FR-003**: The `WAITING` status MUST implement a timer logic that waits for 10 seconds.
- **FR-004**: Upon expiration of the 10-second timer in `WAITING` status, the system MUST automatically attempt to resume playback (via `play` or `load`).
- **FR-005**: The system MUST allow the user to interrupt the 10-second wait and resume immediately by triggering the Play command.
- **FR-006**: The "User Paused" distinction MUST be removed or ignored for the purpose of preventing auto-recovery; manual user pause must also follow the auto-retry logic. Pressing the Pause key triggers the `WAITING` state (10s countdown to auto-resume). Users must use `Stop` for permanent cessation.

### Key Entities

- **PlaybackStatus**: Updated enum to remove `Paused` or repurpose it.
- **AppState**: Logic for `last_system_pause_time` applied to all pauses.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Tracing logs show zero occurrences of the player remaining in a non-playing state (other than `IDLE` or `FINISHED` from `Stop`) for longer than 10 seconds without a retry attempt.
- **SC-002**: Manually pressing "Pause" results in a resumption of playback within 10 seconds (plus buffering time).
- **SC-003**: System recovers from interruptions (previously `PAUSED`) by entering `WAITING` and resuming.