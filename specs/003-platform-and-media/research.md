# Research: Platform and Media Controllers

## Receiver Controller (Platform)
**Namespace**: `urn:x-cast:com.google.cast.receiver`

**Key Messages**:
- **STOP**: `{"type": "STOP", "requestId": 1, "sessionId": "..."}`. Stops an application.
- **GET_STATUS**: Already partially implemented. Need to parse the `status.applications` array fully.
- **LAUNCH**: Already implemented.
- **JOIN**: There is no explicit "JOIN" message. Joining means connecting to the `transportId` of an existing session (which we get from `GET_STATUS`) and then interacting with it. The connection flow is: `CONNECT` (to transportId) -> `GET_STATUS` (Media/Receiver).

**Decision**:
- `ReceiverController` will handle `GET_STATUS`, `LAUNCH`, `STOP`.
- `join_session` logic involves:
    1. `GET_STATUS` (Receiver) to find app.
    2. Extract `transportId`.
    3. `CONNECT` to that `transportId` (creating a new virtual channel).
    4. Then send Media commands.

## Media Controller
**Namespace**: `urn:x-cast:com.google.cast.media`

**Key Messages**:
- **LOAD**: `{"type": "LOAD", ...}`.
- **PLAY**: `{"type": "PLAY", "requestId": 1, "mediaSessionId": ...}`.
- **PAUSE**: `{"type": "PAUSE", ...}`.
- **SEEK**: Already partially researched.
- **STOP**: `{"type": "STOP", ...}`. Stops media playback.

**Status Parsing**:
- `MEDIA_STATUS` contains `status` array. We need to track `mediaSessionId` from this status to use in subsequent commands.
- `MediaController` needs state (current `mediaSessionId`).

## Architecture Refactoring
**Current**: `CastClient` has monolithic methods.
**New**:
- `CastClient` holds the connection.
- `ReceiverController` wraps `CastClient` (or channel) to send Receiver messages.
- `MediaController` wraps `CastClient` to send Media messages.
- Or `CastClient` exposes `receiver()` and `media()` accessors.

**Decision**:
- Keep `CastClient` as the main entry point.
- Implement `ReceiverController` and `MediaController` structs that take a reference to `CastClient` (or share the command channel).
- This allows logical separation without heavy refactoring of the connection loop.

## Resilience (Exponential Backoff)
**Algorithm**:
- Initial delay: 1s
- Multiplier: 2.0
- Max delay: 30s
- Jitter: Optional (good for thundering herd, but maybe overkill here).
- Reset on successful connection (after X seconds of stability).

**Implementation**:
- Modify the `connect` loop in `client.rs` to use these parameters.
