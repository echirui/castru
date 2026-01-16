# Feature Specification: castnow Feature Parity and Torrent Refinement

**Feature Branch**: `018-castnow-feature-parity`  
**Created**: 2026-01-15  
**Status**: Draft  
**Input**: User description: "castnow https://github.com/xat/castnow を参考にして未実装のオプションを実装してください。また、torrentの精度がまだなので、castnowの実装を調査してください。"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Advanced CLI Options (Priority: P1)

As a power user, I want to use advanced command-line options similar to `castnow`, such as specifying a local IP address or custom ports, so that I can cast media in complex network environments.

**Why this priority**: High. Essential for parity with the reference tool and for professional use cases.

**Independent Test**: Run `castru` with new flags (e.g., `--myip`, `--port`) and verify that the server binds to the correct interface/port.

**Acceptance Scenarios**:

1. **Given** a multi-interface machine, **When** `--myip` is specified, **Then** the streaming server MUST bind to that specific IP.
2. **Given** a firewall restriction, **When** `--port` is specified, **Then** the server MUST listen on that specific port.

---

### User Story 2 - Robust Torrent Streaming (Priority: P1)

As a user, I want torrent streaming to be as reliable as it is in `castnow`, with better piece prioritization and buffering, so that I can watch high-quality videos without interruptions.

**Why this priority**: High. The current torrent implementation is basic and needs logic improvements based on `peer-flix` patterns used in `castnow`.

**Independent Test**: Stream a poorly seeded torrent and compare stability with the current implementation.

**Acceptance Scenarios**:

1. **Given** a torrent with distributed pieces, **When** streaming starts, **Then** the engine SHOULD prioritize pieces needed for immediate playback more aggressively.
2. **Given** inconsistent network speed, **When** the buffer is low, **Then** the system SHOULD manage peer connections to maximize throughput for the current playback head.

---

### User Story 3 - Extended Media Control (Priority: P2)

As a user, I want to use additional flags like `--subtitles` or `--volume` directly from the command line, so that I can pre-configure my viewing experience.

**Why this priority**: Medium. Improves convenience but not strictly required for basic functionality.

**Independent Test**: Run `castru cast media.mp4 --volume 0.5` and verify the initial volume on the device.

---

### Edge Cases

- **Conflicting IP/Port**: What happens if the specified `--myip` is not available on the system?
- **Torrent metadata timeout**: How does the system handle magnets that take a very long time to resolve metadata?
- **FFmpeg version mismatch**: If advanced transcode flags are passed but the local FFmpeg doesn't support them.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST support `--myip <IP>` to specify the local interface for the streaming server.
- **FR-002**: System MUST support `--port <PORT>` to specify the port for the internal HTTP server.
- **FR-003**: System MUST implement sequential piece prioritization for torrents (Refinement).
- **FR-004**: System MUST support `--subtitles <FILE>` to load sidecar subtitle files (.vtt). System MAY provide basic regex-based conversion for SubRip (.srt) to WebVTT to ensure compatibility.
- **FR-005**: System MUST support `--quiet` to suppress non-error TUI/Log output.
- **FR-006**: System MUST support `--loop` to repeat the current playlist.

### Key Entities *(include if feature involves data)*

- **CLI Config**: The internal representation of all command-line arguments.
- **Torrent Strategy**: The logic governing piece selection and peer management.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Successful binding to user-specified IP/Port 100% of the time (if valid).
- **SC-002**: Torrent playback start time parity with `castnow` (within +/- 5 seconds).
- **SC-003**: Support for at least 5 new CLI flags from the `castnow` reference.
- **SC-004**: Zero process leaks when using `--exit` flag after playback finishes.

## Assumptions

- Subtitle support may require on-the-fly VTT conversion if the receiver only supports VTT.
- Torrent refinement will use `librqbit` features or configuration tweaks to match `peer-flix` behavior.