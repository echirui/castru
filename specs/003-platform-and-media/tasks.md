# Tasks: Platform Controller Expansion & Media Controller

**Feature**: Platform Controller Expansion & Media Controller
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
- **Refactoring First**: Establish the Controller pattern first to avoid building on the old monolithic structure.
- **Incremental**: Platform Control -> Media Control -> Resilience.

---

## Phase 1: Setup
**Goal**: Refactor project structure to support Controllers.

- [x] T001 Create `src/controllers/` directory and module structure (mod.rs, receiver.rs, media.rs)
- [x] T002 [P] Export new controller modules in `src/lib.rs`
- [x] T003 [P] Move existing `ReceiverRequest` / `MediaRequest` definitions to `src/protocol/` if not already there (verify/update)

## Phase 2: Foundation
**Goal**: Define new messages and data structures.

- [x] T004 [US1] Update `src/protocol/receiver.rs` to include `STOP` message
- [x] T005 [US1] Define `Application` struct in `src/protocol/receiver.rs` for `GET_STATUS` parsing
- [x] T006 [US2] Update `src/protocol/media.rs` to include `PLAY`, `PAUSE`, `STOP` messages
- [x] T007 [US2] Define `MediaInformation` and `MediaMetadata` in `src/protocol/media.rs`

## Phase 3: Platform Controller (User Story 1 - P1)
**Goal**: Manage applications and sessions.
**Independent Test**: Connect, list apps, and stop an app.

- [x] T008 [US1] Implement `ReceiverController` struct in `src/controllers/receiver.rs` wrapping `CastClient`
- [x] T009 [US1] Implement `launch_app`, `stop_app`, `get_status` in `ReceiverController`
- [x] T010 [US1] Implement `join_session` logic (find transportId -> connect) in `ReceiverController` (or helper)
- [x] T011 [US1] Create `examples/platform_control.rs` to test listing and stopping apps

## Phase 4: Media Controller (User Story 2 - P2)
**Goal**: Full playback control.
**Independent Test**: Load a video and control it.

- [x] T012 [US2] Implement `MediaController` struct in `src/controllers/media.rs` wrapping `CastClient`
- [x] T013 [US2] Implement `load`, `play`, `pause`, `seek`, `stop` in `MediaController`
- [x] T014 [US2] Implement `get_status` parsing in `MediaController` to track `mediaSessionId` (User managed)
- [x] T015 [US2] Create `examples/full_media.rs` to demonstrate loading and controlling video

## Phase 5: High-Level Abstraction & Resilience (User Story 3 - P3)
**Goal**: Robustness and Ease of Use.
**Independent Test**: Disconnect network and observe reconnection log.

- [x] T016 [US3] Implement `DefaultMediaReceiver` wrapper struct in `src/controllers/mod.rs` (or similar)
- [x] T017 [US3] Implement exponential backoff in `src/client.rs` connection loop
- [x] T018 [US3] Refactor `CastClient` to expose `receiver()` and `media()` methods returning controllers

## Phase 6: Polish
**Goal**: Documentation and Cleanup.

- [x] T019 Update `README.md` with new Controller usage examples
- [x] T020 Add rustdoc to all new Controller methods
- [x] T021 Run `cargo clippy` and fix any lints
