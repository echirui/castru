# Feature Specification: Castnow-like TUI

**Feature Branch**: `006-castnow-tui`
**Created**: 2026-01-15
**Status**: Draft
**Input**: User description: "castnow と同じようなTUIを実装したいです。"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Real-time Playback Status (Priority: P1)

As a user, I want to see the current playback progress, time, and status of the media so that I know exactly where I am in the video/audio.

**Why this priority**: Essential for any media player experience; users need to know if the system is working (playing vs buffering) and how much time is left.

**Independent Test**: Play a media file and observe the terminal output updating strictly in-place (not scrolling) with time and progress bar.

**Acceptance Scenarios**:

1. **Given** media is playing, **When** time progresses, **Then** the current time and progress bar update at least once per second.
2. **Given** media is paused, **When** no action is taken, **Then** the status display indicates "Paused" and does not advance.
3. **Given** media is buffering, **When** waiting for network, **Then** the status display indicates "Buffering".

---

### User Story 2 - Keyboard Playback Controls (Priority: P1)

As a user, I want to control playback using standard keyboard shortcuts (Space, Arrows) so that I can easily pause, seek, or adjust volume without typing complex commands.

**Why this priority**: "Castnow-like" implies a keyboard-driven interface. This is the primary interaction method.

**Independent Test**: Pressing keys during playback triggers the corresponding action on the Chromecast and updates the UI.

**Acceptance Scenarios**:

1. **Given** media is playing, **When** Space is pressed, **Then** media pauses and UI updates to "Paused".
2. **Given** media is paused, **When** Space is pressed, **Then** media resumes and UI updates to "Playing".
3. **Given** media is playing, **When** Right Arrow is pressed, **Then** media seeks forward (e.g., +30s) and UI reflects the new time.
4. **Given** media is playing, **When** Up/Down Arrow is pressed, **Then** volume increases/decreases.
5. **Given** any state, **When** 'q' or 'Esc' is pressed, **Then** the application stops playback and exits cleanly.

---

### User Story 3 - Clean Terminal Interface (Priority: P2)

As a user, I want the interface to be minimal and not clutter my terminal history, utilizing a single active status line (or few lines) that rewrites itself.

**Why this priority**: Defines the "castnow" aesthetic—unobtrusive and clean.

**Independent Test**: Run the application and ensure it captures the cursor/terminal mode, preventing stray input from echoing, and restores the terminal on exit.

**Acceptance Scenarios**:

1. **Given** the application starts, **When** rendering the UI, **Then** it overwrites the previous status line rather than appending new lines.
2. **Given** the application exits, **When** returning to the shell, **Then** the cursor is visible and text input works normally (terminal state restored).

### Edge Cases

- **Terminal Resize**: How does the progress bar handle narrow terminals? (Should truncate or adjust bar width).
- **Disconnect**: What happens if the Chromecast disconnects? (Should show "Disconnected" and potentially exit or wait).
- **Unknown Duration**: What if the media is a live stream or duration is unknown? (Should show elapsed time but no total/progress bar).

## Assumptions

- **A-001**: The user is operating in a standard terminal environment that supports ANSI escape codes and raw mode.
- **A-002**: The backend media controller can provide real-time updates on playback status (current time, duration, state).
- **A-003**: The terminal window has sufficient width (at least 40 columns) to display a meaningful progress bar and time.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST render a text-based interface that updates in-place (no scrolling logs during normal playback).
- **FR-002**: System MUST display Playback State (Playing, Paused, Buffering, Idle).
- **FR-003**: System MUST display Current Time and Total Duration (format MM:SS or HH:MM:SS).
- **FR-004**: System MUST display a visual progress bar indicating relative position.
- **FR-005**: System MUST capture keyboard input without requiring "Enter" (Raw Mode).
- **FR-006**: System MUST map 'Space' to Play/Pause toggle.
- **FR-007**: System MUST map 'Right Arrow' to Seek Forward and 'Left Arrow' to Seek Backward.
- **FR-008**: System MUST map 'Up Arrow' to Volume Up and 'Down Arrow' to Volume Down.
- **FR-009**: System MUST map 'm' to Mute/Unmute.
- **FR-010**: System MUST restore terminal settings (canonical mode, cursor visibility) upon exit or crash (best effort).

### Key Entities

- **TuiState**: Holds current playback status, volume, and metadata to be rendered.
- **InputEvent**: Represents a user keypress to be processed.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: UI updates frequency is at least 1Hz (1 update per second).
- **SC-002**: Input latency (keypress to UI acknowledgement) is under 100ms (optimistic update or fast feedback).
- **SC-003**: Application cleans up terminal state 100% of the time on graceful exit.
- **SC-004**: Progress bar scales correctly to terminal width, utilizing available space without wrapping.