# Feature Specification: CastV2 Protocol Implementation

**Feature Branch**: `001-castv2-protocol`
**Created**: 2026-01-13
**Status**: Draft
**Input**: User description: (See original prompt)

## User Scenarios & Testing *(mandatory)*

<!--
  IMPORTANT: User stories should be PRIORITIZED as user journeys ordered by importance.
  Each user story/journey must be INDEPENDENTLY TESTABLE - meaning if you implement just ONE of them,
  you should still have a viable MVP (Minimum Viable Product) that delivers value.
-->

### User Story 1 - Secure Device Connection (Priority: P1)

As a developer using the library, I want to establish a secure TLS connection to a Google Cast device on port 8009 so that I can begin exchanging control messages.

**Why this priority**: Without a connection, no other functionality is possible.

**Independent Test**: Can successfully complete a TLS handshake with a mock or real Cast device.

**Acceptance Scenarios**:

1. **Given** a valid IP address of a Cast device, **When** the client initiates a connection, **Then** a TCP connection is established on port 8009.
2. **Given** a TCP connection, **When** the TLS handshake is initiated, **Then** it completes successfully using TLS 1.2 or 1.3.
3. **Given** a connection attempt, **When** the device is unreachable, **Then** the system reports a connection error.

---

### User Story 2 - Heartbeat Maintenance (Priority: P1)

As a developer, I want the system to automatically handle heartbeat messages so that the connection remains active without manual intervention.

**Why this priority**: Cast devices disconnect clients that do not send periodic heartbeats (keep-alive).

**Independent Test**: Connect to a device and observe the connection remaining open for longer than the device's timeout period (typically >10 seconds).

**Acceptance Scenarios**:

1. **Given** an active connection, **When** 5 seconds elapse, **Then** the system automatically sends a `PING` message in the `urn:x-cast:com.google.cast.tp.heartbeat` namespace.
2. **Given** a `PONG` message is received from the device, **When** processed, **Then** the system logs or acknowledges the keep-alive.

---

### User Story 3 - Connection Management (Priority: P2)

As a developer, I want to establish a virtual connection (CONNECT) to the device receiver so that I can send subsequent commands.

**Why this priority**: Required protocol step before launching apps or controlling media.

**Independent Test**: Send a CONNECT message and verify no error is returned/connection is not closed.

**Acceptance Scenarios**:

1. **Given** a TLS connection, **When** the client sends a `CONNECT` message to `urn:x-cast:com.google.cast.tp.connection`, **Then** the device accepts the virtual connection.

---

### User Story 4 - Receiver Application Control (Priority: P3)

As a developer, I want to launch receiver applications (like YouTube or Netflix) so that I can start a casting session.

**Why this priority**: Core functionality of the Cast protocol.

**Independent Test**: Send a launch command and observe the application starting on the device.

**Acceptance Scenarios**:

1. **Given** a connected session, **When** a `LAUNCH` command is sent for a specific App ID to `urn:x-cast:com.google.cast.receiver`, **Then** the device launches the application and returns a `RECEIVER_STATUS` message.

### Edge Cases

- What happens when the network connection drops unexpectedly? System should detect and report the error.
- How does system handle malformed Cast packets? System should discard them and optionally log a warning, but not crash.
- What happens if the TLS certificate is invalid? System should fail the connection (or allow bypass if configured for testing).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST connect to Cast devices via TCP port 8009.
- **FR-002**: System MUST secure all communications using TLS 1.2 or 1.3.
- **FR-003**: System MUST implement the CastV2 framing protocol (4-byte Big Endian payload length header).
- **FR-004**: System MUST serialize and deserialize payloads using Protocol Buffers (`CastMessage`).
- **FR-005**: System MUST support the `urn:x-cast:com.google.cast.tp.heartbeat` namespace for PING/PONG.
- **FR-006**: System MUST send heartbeat PINGs at a configurable interval (default 0.2 Hz / 5 seconds).
- **FR-007**: System MUST support the `urn:x-cast:com.google.cast.tp.connection` namespace for session management.
- **FR-008**: System MUST support the `urn:x-cast:com.google.cast.receiver` namespace for application control.
- **FR-009**: System MUST support the `urn:x-cast:com.google.cast.media` namespace for media playback control.
- **FR-010**: System MUST handle asynchronous notifications (Status Updates) concurrently with outgoing requests.

### Technical Constraints

- **TC-001**: System MUST use `tokio` (net, time, sync) as the asynchronous runtime.
- **TC-002**: System MUST use `prost` for Protocol Buffers serialization.
- **TC-003**: System MUST use `rustls` for TLS encryption.
- **TC-004**: Dependencies MUST be minimized to ensure maintainability and security.

### Key Entities *(include if feature involves data)*

- **CastMessage**: The fundamental data unit, containing protocol version, source/destination IDs, namespace, and payload type (string/binary).
- **CastHeader**: The 4-byte prefix indicating payload length.
- **CastSession**: Represents the state of the connection, including the socket, message queue, and keep-alive timer.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Successfully performs TLS handshake with a Cast device or compliant emulator.
- **SC-002**: Connection is maintained for at least 60 seconds via automated heartbeats without error.
- **SC-003**: Can launch a specified receiver application (e.g., Default Media Receiver) and receive a status update.
- **SC-004**: Dependency graph is restricted to only the minimal set required for Async I/O, TLS, and Protobuf serialization, adhering strictly to the defined Technical Constraints.