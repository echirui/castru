# Implementation Plan - CastV2 Protocol Implementation

**Feature**: CastV2 Protocol Implementation
**Status**: Planned
**Spec**: [spec.md](./spec.md)

## Technical Context

### Architecture Constraints
- **Runtime**: `tokio` (Async I/O)
- **Serialization**: `prost` (Protocol Buffers)
- **Security**: `rustls` (TLS 1.2/1.3)
- **Dependencies**: Minimal. `serde`/`serde_json` allowed for payload parsing if strictly necessary for maintenance.

### Key Components
1. **Transport Layer**: TCP + TLS wrapper over `tokio::net::TcpStream`.
2. **Framer**: Reads/Writes 4-byte length prefix + Protobuf payload.
3. **Session Manager**: Maintains Heartbeat, Routing, and Request/Response correlation.
4. **Message Dispatcher**: Routes messages to appropriate handlers based on namespace.

### Data Flow
1. **Outbound**: Struct -> `CastMessage` (Proto) -> Length Prefix -> TLS Stream.
2. **Inbound**: TLS Stream -> Length Prefix -> `CastMessage` (Proto) -> Dispatcher -> Handler.

## Constitution Check

### Compliance
- [x] **Library-First**: Designed as a library crate.
- [x] **No unnecessary dependencies**: Strict adherence to `tokio`/`prost`/`rustls`.
- [x] **Test-First**: Unit tests for framing and logic; Integration tests against mock/real device.

## Phases

### Phase 1: Foundation & Serialization (Milestone 1)
**Goal**: Establish the build system and basic data structures.
- [ ] **Task 1.1**: Initialize `Cargo.toml` with `prost`, `prost-build`, `tokio`, `rustls`.
- [ ] **Task 1.2**: Acquire `cast_channel.proto` and configure `build.rs`.
- [ ] **Task 1.3**: Implement `CastMessage` wrapper (encode/decode with length prefix).

### Phase 2: Secure Transport (Milestone 2)
**Goal**: Connect to port 8009 with TLS.
- [ ] **Task 2.1**: Implement `TlsConnector` using `rustls`.
- [ ] **Task 2.2**: Implement `CastStream` for reading/writing framed messages over TLS.
- [ ] **Task 2.3**: Verify TLS handshake with a mock server or actual device.

### Phase 3: Core Protocol & Heartbeat (Milestone 3)
**Goal**: Maintain a persistent connection.
- [ ] **Task 3.1**: Implement Heartbeat loop (PING every 5s).
- [ ] **Task 3.2**: Implement `CONNECT` message for virtual connection.
- [ ] **Task 3.3**: Implement basic Dispatcher/Event Loop (`tokio::select!`).

### Phase 4: Application Control (Milestone 4)
**Goal**: Launch apps and control media.
- [ ] **Task 4.1**: Implement Receiver namespace (LAUNCH).
- [ ] **Task 4.2**: Implement Media namespace (PLAY/PAUSE).
- [ ] **Task 4.3**: Implement Status Update parsing.

### Phase 5: Polish & Tools (Milestone 5)
**Goal**: Usability and robustness.
- [ ] **Task 5.1**: Create a minimal CLI example.
- [ ] **Task 5.2**: Refine Error Handling (custom `CastError`).

## Reference Documents
- [Research Findings](./research.md)
- [Data Model](./data-model.md)
- [API Contracts](./contracts/cast_client_trait.rs)
- [Quickstart](./quickstart.md)