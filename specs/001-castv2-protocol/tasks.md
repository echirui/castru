# Tasks: CastV2 Protocol Implementation

**Feature**: CastV2 Protocol Implementation
**Status**: Ready for Implementation
**Spec**: [spec.md](./spec.md)
**Plan**: [plan.md](./plan.md)

## Dependencies
- **Phase 1 (Setup)**: No dependencies
- **Phase 2 (Foundation)**: Depends on Phase 1
- **Phase 3 (US1)**: Depends on Phase 2
- **Phase 4 (US2)**: Depends on Phase 3
- **Phase 5 (US3)**: Depends on Phase 3 (Can run parallel with Phase 4)
- **Phase 6 (US4)**: Depends on Phase 5
- **Phase 7 (Polish)**: Depends on Phase 6

## Implementation Strategy
- **MVP**: Phases 1-3 constitute the minimal connectivity MVP.
- **Incremental**: Add Heartbeat (Ph4) and Virtual Connection (Ph5) to stabilize.
- **Full Feature**: Add App Launching (Ph6) to complete the user requirements.
- **Testing**: Unit tests for framing (Ph2). Integration tests for connection (Ph3+).

---

## Phase 1: Setup
**Goal**: Initialize project and build system.

- [x] T001 Define dependencies in `Cargo.toml` (tokio, prost, rustls, bytes, serde, serde_json)
- [x] T002 Create `proto/` directory and download `cast_channel.proto` (skipped others as not found/essential)
- [x] T003 Create `build.rs` to configure `prost-build` for proto compilation
- [x] T004 [P] Create initial `src/lib.rs` and `src/error.rs` module structure

## Phase 2: Foundation
**Goal**: Protocol Buffers generation and Message Framing.

- [x] T005 [P] Generate Rust code from protos and re-export in `src/proto/mod.rs`
- [x] T006 Define `CastHeader` struct for 4-byte big-endian length prefix in `src/codec.rs`
- [x] T007 Implement `CastMessage` encoder (Header + Proto) in `src/codec.rs`
- [x] T008 Implement `CastMessage` decoder (Header + Proto) in `src/codec.rs`
- [x] T009 [P] Write unit tests for `encode` and `decode` in `src/codec.rs`

## Phase 3: Secure Device Connection (User Story 1 - P1)
**Goal**: Establish TLS connection to port 8009.
**Independent Test**: Connect to device and complete handshake.

- [x] T010 [US1] Create `NoCertificateVerification` struct in `src/tls.rs` for development
- [x] T011 [US1] Implement `TlsConnector` setup using `rustls` in `src/tls.rs`
- [x] T012 [US1] Define `CastClient` struct in `src/client.rs` holding the TLS stream
- [x] T013 [US1] Implement `CastClient::connect(host, port)` in `src/client.rs`
- [x] T014 [US1] Create integration test `tests/connection.rs` to verify handshake

## Phase 4: Heartbeat Maintenance (User Story 2 - P1)
**Goal**: Keep connection alive with PING/PONG.
**Independent Test**: Connection remains open >10s.

- [x] T015 [US2] Define `Heartbeat` struct/enum (PING/PONG) using serde in `src/protocol/heartbeat.rs`
- [x] T016 [US2] Implement `send_message` helper in `src/client.rs` for general sending
- [x] T017 [US2] Implement background heartbeat loop in `src/client.rs`
- [x] T018 [US2] Handle incoming `PONG` messages in message dispatcher in `src/client.rs`
- [x] T019 [US2] Update integration test to wait 15 seconds and verify connection persists

## Phase 5: Connection Management (User Story 3 - P2)
**Goal**: Establish virtual connection for receiver control.
**Independent Test**: Send CONNECT and receive no error.

- [x] T020 [US3] Define `Connection` namespace messages (CONNECT/CLOSE) in `src/protocol/connection.rs`
- [x] T021 [US3] Implement `connect_receiver` method in `src/client.rs` to send CONNECT to `receiver-0`
- [x] T022 [US3] Add `source_id` and `destination_id` management in `src/client.rs`
- [x] T023 [US3] Add test case for virtual connection establishment

## Phase 6: Receiver Application Control (User Story 4 - P3)
**Goal**: Launch applications and handle status updates.
**Independent Test**: Launch Default Media Receiver.

- [x] T024 [US4] Define `Receiver` namespace messages (LAUNCH, GET_STATUS) in `src/protocol/receiver.rs`
- [x] T025 [US4] Implement `launch_app(app_id)` in `src/client.rs`
- [x] T026 [US4] [P] Implement `events()` channel exposure in `src/client.rs`
- [x] T027 [US4] Parse `RECEIVER_STATUS` messages and dispatch to event channel in `src/client.rs`
- [x] T028 [US4] Create example/test `examples/launch_app.rs` to demonstrate full flow

## Phase 7: Polish & Tools
**Goal**: Usability and robustness.

- [x] T029 Refine `CastError` in `src/error.rs` to cover all failure modes
- [x] T030 Create `src/main.rs` (CLI) to allow `castru <IP> <APP_ID>` execution
- [x] T031 Add documentation comments to public API in `src/lib.rs` and `src/client.rs`
