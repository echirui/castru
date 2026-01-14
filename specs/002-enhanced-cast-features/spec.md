# Feature Specification: Enhanced Cast Features

**Feature Branch**: `002-enhanced-cast-features`
**Created**: 2026-01-13
**Status**: Draft
**Input**: User description: (See original prompt)

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Device Discovery (Priority: P1)

As a developer, I want to automatically discover Chromecast devices on the local network so that I don't need to manually lookup and hardcode IP addresses.

**Why this priority**: Manual IP entry is tedious and error-prone; discovery is fundamental for consumer-friendly apps.

**Independent Test**: Run a CLI command that lists all available Cast devices on the network with their friendly names and IPs.

**Acceptance Scenarios**:

1. **Given** a Chromecast device is on the same Wi-Fi network, **When** the discovery process is initiated, **Then** the system detects the device and reports its IP and friendly name within 5 seconds.
2. **Given** multiple devices are present, **When** discovery runs, **Then** all reachable devices are listed.
3. **Given** no devices are present, **When** discovery runs, **Then** the system gracefully reports no devices found after a timeout.

---

### User Story 2 - Extended Media Control (Priority: P2)

As a user, I want to control media playback (volume, seek, metadata status) so that I can have a full casting experience beyond just launching apps.

**Why this priority**: Core functionality for a media casting library.

**Independent Test**: Connect to a device playing media and successfully toggle pause/play, change volume, and seek to a specific time.

**Acceptance Scenarios**:

1. **Given** media is playing, **When** a "Get Status" command is sent, **Then** the system returns current metadata (title, current time, duration, volume).
2. **Given** media is playing, **When** a "Seek" command is sent, **Then** the playback position updates to the target time.
3. **Given** a connected session, **When** a "Set Volume" command is sent, **Then** the device volume changes to the specified level.

---

### User Story 3 - Automatic Reconnection (Priority: P3)

As a developer, I want the library to automatically handle temporary network disruptions so that my application doesn't crash or require manual restart when Wi-Fi flickers.

**Why this priority**: Reliability is key for long-running sessions.

**Independent Test**: Physically disconnect/reconnect network (or simulate via firewall) and verify the session resumes without manual intervention.

**Acceptance Scenarios**:

1. **Given** an active connection, **When** the network is interrupted for < 30 seconds, **Then** the system attempts to reconnect automatically.
2. **Given** reconnection succeeds, **When** the session resumes, **Then** previous state (like heartbeat loop) continues functioning.
3. **Given** the device is permanently gone, **When** reconnection fails repeatedly, **Then** the system eventually raises a fatal disconnection error.

---

### User Story 4 - Public Crate Preparation (Priority: P4)

As a Rust developer, I want a well-documented and easy-to-use library interface so that I can easily integrate `castru` into my own projects.

**Why this priority**: Facilitates adoption and open-source contribution.

**Independent Test**: A new user can follow the README to install the crate and run a "Hello World" example without errors.

**Acceptance Scenarios**:

1. **Given** the crate source, **When** `cargo doc` is run, **Then** public APIs have clear documentation examples.
2. **Given** the repository root, **When** a user reads `README.md`, **Then** it clearly explains installation, basic usage, and example code.

### Edge Cases

- **Discovery**: What if the network blocks mDNS (multicast)? -> System should timeout and allow manual IP fallback.
- **Media Control**: What if the device has no media session active? -> Commands like "Seek" should return a sensible error/ignore.
- **Reconnection**: What if the device IP changes during disconnection (DHCP)? -> Re-discovery might be needed (advanced scope, basic reconnection attempts last known IP).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST support mDNS-based discovery to locate Google Cast devices (service type `_googlecast._tcp.local`).
- **FR-002**: System MUST parse mDNS TXT records to extract friendly names and device models.
- **FR-003**: System MUST support the `urn:x-cast:com.google.cast.media` namespace for advanced control (Seek, SetVolume, GetStatus).
- **FR-004**: System MUST parse media status updates to provide structural data (current time, duration, metadata).
- **FR-005**: System MUST automatically attempt reconnection if the TCP/TLS connection is dropped unexpectedly.
- **FR-006**: System MUST implement exponential backoff or simple retry logic for reconnection attempts.
- **FR-007**: System MUST provide comprehensive documentation (Rustdoc) for all public structs and methods.
- **FR-008**: System MUST have a `README.md` with installation instructions and a "Getting Started" guide.

### Key Entities

- **CastDevice**: Represents a discovered device with `ip`, `port`, `friendly_name`, `model_name`.
- **MediaStatus**: Struct containing `player_state`, `current_time`, `volume`, `media_metadata`.
- **DiscoveryService**: Background service that listens for mDNS packets and updates the device list.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Discovery finds a local device within 5 seconds in 95% of standard home network environments.
- **SC-002**: Media commands (Volume/Seek) are reflected on the device within 500ms.
- **SC-003**: Reconnection logic recovers a session after a 10-second network interruption 100% of the time.
- **SC-004**: `README.md` contains a working, copy-pasteable example that compiles against the released version.