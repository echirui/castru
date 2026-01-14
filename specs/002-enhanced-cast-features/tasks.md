# Tasks: Enhanced Cast Features

**Feature**: Enhanced Cast Features
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
- **Incremental**: Add mDNS discovery first, then media control, then reconnection logic.
- **Testing**: Manual testing via `examples/` due to network dependencies (mDNS).

---

## Phase 1: Setup
**Goal**: Add dependencies and update project structure.

- [x] T001 Add `mdns-sd` (or `mdns-rust`) to `Cargo.toml`
- [x] T002 [P] Create `src/discovery.rs` and `src/protocol/media.rs` module structure
- [x] T003 [P] Export new modules in `src/lib.rs` and `src/protocol/mod.rs`

## Phase 2: Foundation
**Goal**: Define data structures for Discovery and Media.

- [x] T004 Define `CastDevice` struct in `src/discovery.rs` (ip, port, friendly_name, model, uuid)
- [x] T005 [P] Define `MediaRequest` (SEEK, GET_STATUS) enum in `src/protocol/media.rs`
- [x] T006 [P] Define `MediaStatus` struct in `src/protocol/media.rs`
- [x] T007 [P] Define `ReceiverVolumeRequest` in `src/protocol/receiver.rs`

## Phase 3: Device Discovery (User Story 1 - P1)
**Goal**: Automatically discover Cast devices.
**Independent Test**: Run `examples/discover.rs` and see local devices.

- [x] T008 [US1] Implement `discover_devices` function in `src/discovery.rs` using mDNS
- [x] T009 [US1] Implement TXT record parsing to extract friendly name and model
- [x] T010 [US1] Create `examples/discover.rs` to demonstrate discovery
- [x] T011 [US1] Expose discovery via public API in `src/lib.rs`

## Phase 4: Extended Media Control (User Story 2 - P2)
**Goal**: Control volume, seek, and get metadata.
**Independent Test**: Connect to a playing device and change volume/seek.

- [x] T012 [US2] Implement `media_seek(destination_id, time)` in `src/client.rs`
- [x] T013 [US2] Implement `media_get_status(destination_id)` in `src/client.rs`
- [x] T014 [US2] Implement `set_volume(level)` in `src/client.rs` (Receiver namespace)
- [x] T015 [US2] Create `examples/media_control.rs` to test volume and seek

## Phase 5: Automatic Reconnection (User Story 3 - P3)
**Goal**: Handle network drops gracefully.
**Independent Test**: Disconnect network while running example, verify resume.

- [x] T016 [US3] Refactor `CastClient` to support internal reconnection state
- [x] T017 [US3] Implement retry logic in the background command loop in `src/client.rs`
- [x] T018 [US3] Add `connection_status` channel/event to notify user of reconnects (Logged to stdout for MVP)

## Phase 6: Public Crate Preparation (User Story 4 - P4)
**Goal**: Polish for public release.
**Independent Test**: `cargo doc` produces clean docs, `README.md` is helpful.

- [x] T019 [US4] Add rustdoc comments to `CastDevice`, `CastClient`, and discovery functions
- [x] T020 [US4] Update `README.md` with "Getting Started", "Discovery", and "Media Control" sections
- [x] T021 [US4] Add metadata (description, license, keywords) to `Cargo.toml`
- [x] T022 [US4] Ensure all examples compile and run

## Phase 7: Polish
**Goal**: Final cleanup.

- [x] T023 Run `cargo clippy` and fix lints
- [x] T024 Verify dependency tree is minimal (`cargo tree`)
