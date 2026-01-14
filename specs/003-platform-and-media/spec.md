# Feature Specification: Platform Controller Expansion & Media Controller

**Feature Branch**: `003-platform-and-media`
**Created**: 2026-01-13
**Status**: Draft
**Input**: User description: (See original prompt)

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Platform Session Management (Priority: P1)

As a developer, I want to inspect running applications, stop them, and join existing sessions so that I can manage the device state effectively.

**Why this priority**: Essential for multi-app management and robust session handling.

**Independent Test**: Can list running apps, join an existing session, and stop it via a test script.

**Acceptance Scenarios**:

1. **Given** a device with running apps, **When** `GET_STATUS` is sent, **Then** the system parses and returns a list of `Application` structs including `sessionId` and `transportId`.
2. **Given** an active session ID, **When** `stop_app` is called, **Then** the device terminates the application.
3. **Given** an existing session, **When** `join_session` is called with the correct `sessionId`, **Then** a virtual connection is established, allowing control.

---

### User Story 2 - Media Playback Control (Priority: P2)

As a developer, I want to load media content and control playback (play, pause, seek, volume) so that I can build a functional media casting application.

**Why this priority**: Core value proposition of the Cast protocol.

**Independent Test**: Load a video URL and verify playback state changes (Playing -> Paused -> Playing) and time updates.

**Acceptance Scenarios**:

1. **Given** a connected media receiver, **When** `load_media` is called with a URL, **Then** the device begins playback.
2. **Given** playing media, **When** `pause` is called, **Then** the state changes to `PAUSED`.
3. **Given** playing media, **When** `seek` is called, **Then** playback position updates.
4. **Given** playing media, **When** `MEDIA_STATUS` messages arrive, **Then** the system parses and exposes current time and player state.

---

### User Story 3 - High-Level Abstraction & Resilience (Priority: P3)

As a developer, I want a robust and easy-to-use API that handles connection drops and provides simple wrappers for common tasks so that I can focus on app logic.

**Why this priority**: Improves developer experience (DX) and application stability.

**Independent Test**: Simulate network drop and verify automatic reconnection; verify simplified API usage.

**Acceptance Scenarios**:

1. **Given** a simplified `DefaultMediaReceiver` wrapper, **When** instantiated, **Then** it exposes intuitive methods like `play_video(url)`.
2. **Given** a network interruption, **When** connection is lost, **Then** the system attempts reconnection using exponential backoff.
3. **Given** the internal architecture, **When** reviewed, **Then** logic is separated into `ReceiverController` and `MediaController` structs.

### Edge Cases

- **Platform**: What if `join_session` is called for a non-existent session? -> Should return a distinct error.
- **Media**: What if `load_media` fails (invalid URL)? -> Should parse `LOAD_FAILED` or error status.
- **Resilience**: What if reconnection fails indefinitely? -> Should eventually timeout and notify the user.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST parse `GET_STATUS` responses to extract `Application` details (appId, displayName, sessionId, transportId).
- **FR-002**: System MUST support sending `STOP` messages to the Receiver namespace to end sessions.
- **FR-003**: System MUST support joining existing sessions by connecting to their `transportId`.
- **FR-004**: System MUST implement `MediaController` supporting `LOAD`, `PLAY`, `PAUSE`, `SEEK`, `STOP`, `SET_VOLUME` commands.
- **FR-005**: System MUST parse `MEDIA_STATUS` messages to track `playerState`, `currentTime`, and `media` metadata.
- **FR-006**: System MUST implement a `DefaultMediaReceiver` wrapper that simplifies launching and loading media.
- **FR-007**: System MUST implement automatic reconnection with exponential backoff logic.
- **FR-008**: System MUST refactor monolithic client logic into distinct `ReceiverController` and `MediaController` components.

### Key Entities

- **Application**: Struct with `app_id`, `display_name`, `session_id`, `transport_id`.
- **MediaStatus**: Detailed struct for media state (extended from previous basic version).
- **ReceiverController**: Handles platform-level commands (Launch, Stop, GetStatus, Join).
- **MediaController**: Handles media-level commands (Load, Play, Pause, Seek).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Can successfully join an existing session and take control without re-launching.
- **SC-002**: `load_media` successfully starts playback of a standard MP4 URL on a Chromecast.
- **SC-003**: System recovers from a simulated 5-second network drop and resumes heartbeat/control within 10 seconds.
- **SC-004**: API surface is refactored into distinct controllers, reducing `CastClient` complexity.