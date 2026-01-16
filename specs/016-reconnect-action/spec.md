# Feature Specification: Reconnect Action

**Feature Branch**: `016-reconnect-action`  
**Created**: 2026-01-15  
**Status**: Draft  
**Input**: User description: "アプリケーション起動中に再接続アクションができるように実装する"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Manual Reconnect (Priority: P1)

As a user, I want to be able to trigger a reconnection to the Chromecast device without restarting the entire application, so that I can quickly recover from minor network glitches or temporary device unavailability.

**Why this priority**: Core functionality requested. It improves usability by avoiding full application restarts.

**Independent Test**: Can be fully tested by pressing a specific key (e.g., 'r') and observing the logs/TUI to see the connection being re-established.

**Acceptance Scenarios**:

1. **Given** the application is connected to a device, **When** the user presses the 'r' key, **Then** the current connection should be closed and a new connection should be established to the same device.
2. **Given** the connection has been lost, **When** the user triggers a reconnect action, **Then** the application should attempt to restore the session and resume status monitoring.

---

### User Story 2 - Reconnect Status Feedback (Priority: P2)

As a user, I want to see the status of the reconnection attempt in the TUI, so that I know whether the system is currently trying to reconnect, succeeded, or failed.

**Why this priority**: Essential for user experience. Without feedback, the user doesn't know if the 'r' key press did anything.

**Independent Test**: Trigger a reconnect and verify that the TUI status line or a dedicated indicator shows "Reconnecting...", "Connected", or "Reconnect Failed".

**Acceptance Scenarios**:

1. **Given** a reconnect is triggered, **When** the process starts, **Then** the TUI status should temporarily show "RECONNECTING".
2. **Given** a reconnect failed, **When** the attempt finishes, **Then** an error message should be briefly visible or the status should reflect the failure.

---

### Edge Cases

- **Reconnecting during active playback**: Does the session remain valid? (Ideally, it should resume playback tracking).
- **Target device changed IP**: Does the reconnect use the original IP or perform a quick mDNS lookup? (Assumption: use original IP first, then lookup if it fails).
- **Rapid successive reconnect triggers**: How does the system handle multiple 'r' key presses in 1 second?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST provide a manual trigger (e.g., keyboard shortcut 'r') to initiate a reconnection.
- **FR-002**: System MUST gracefully close the existing TCP/TLS connection before opening a new one.
- **FR-003**: System MUST re-authenticate/re-connect to the Receiver and Media namespaces after reconnection.
- **FR-004**: System MUST update the TUI to reflect the reconnection state.
- **FR-005**: System MUST NOT lose the current playlist or application state during a manual reconnection.

### Key Entities *(include if feature involves data)*

- **Connection State**: Tracks whether the socket is active, connecting, or disconnected.
- **Session Context**: Holds the current device IP, port, and active app/media IDs to facilitate resumption.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Manual reconnection completes in under 2 seconds (assuming network stability).
- **SC-002**: 100% of successful reconnections restore the TUI to its previous functional state (correct volume, playback time, etc.).
- **SC-003**: TUI reflects the "RECONNECTING" state within 100ms of the trigger.
