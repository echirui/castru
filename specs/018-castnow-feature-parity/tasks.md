# Tasks: castnow Feature Parity and Torrent Refinement

**Input**: Design documents from `/specs/018-castnow-feature-parity/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, quickstart.md

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [ ] T001 [P] Verify `castnow` flag mapping in `specs/018-castnow-feature-parity/research.md`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

- [ ] T002 Update `CastOptions` struct in `src/main.rs` to include new fields (`myip`, `port`, `subtitles`, `initial_volume`, `loop_playlist`, `quiet`)
- [ ] T003 [P] Add `tracks` field to `MediaInformation` and define `MediaTrack` struct in `src/protocol/media.rs`

---

## Phase 3: User Story 1 - Advanced CLI Options (Priority: P1) 🎯 MVP

**Goal**: Support advanced CLI flags for network and server configuration.

**Independent Test**: Run `cargo run -- cast --myip 127.0.0.1 --port 9999 media.mp4` and verify the server binds to that address.

### Tests for User Story 1

- [ ] T003.1 [P] [US1] Unit test for extended `parse_cast_args` in `src/main.rs`
- [ ] T003.2 [P] [US1] Unit test for `StreamServer` port/IP binding logic in `src/server.rs`

### Implementation for User Story 1

- [ ] T004 [P] [US1] Update `parse_cast_args` in `src/main.rs` to handle `--myip`, `--port`, `--subtitles`, `--volume`, `--loop`, `--quiet`
- [ ] T005 [US1] Refactor `StreamServer::start` in `src/server.rs` to accept an optional IP and port override
- [ ] T006 [US1] Update `cast_media_playlist` in `src/main.rs` to pass `myip` and `port` from `CastOptions` to `server.start()`
- [ ] T007 [US1] Implement `--quiet` logic in `src/main.rs` to suppress standard output if enabled

**Checkpoint**: User Story 1 functional - custom network settings and quiet mode working.

---

## Phase 4: User Story 2 - Robust Torrent Streaming (Priority: P1)

**Goal**: Refine torrent piece prioritization for better streaming stability.

**Independent Test**: Play a torrent and verify sequential download behavior in logs.

### Implementation for User Story 2

- [ ] T008 [P] [US2] Implement sequential mode using `handle.set_sequential(true)` in `src/torrent/manager.rs`
- [ ] T009 [US2] Refine `GrowingFile::poll_read` in `src/torrent/stream.rs` to prioritize pieces immediately ahead of the current read head

**Checkpoint**: User Story 2 functional - torrent streaming is more robust.

---

## Phase 5: User Story 3 - Extended Media Control (Priority: P2)

**Goal**: Support subtitles, initial volume, and playlist looping.

**Independent Test**: Use `--subtitles` and `--volume` flags and verify they are applied on the device.

### Implementation for User Story 3

- [ ] T010.1 [US3] Implement basic SRT to VTT converter utility in `src/utils/subtitles.rs` (or similar)
- [ ] T010 [US3] Add sidecar subtitle file serving logic to `StreamServer` in `src/server.rs`
- [ ] T011 [US3] Update `load_media` in `src/main.rs` to detect subtitle files and include them in `MediaInformation` tracks
- [ ] T012 [US3] Apply `initial_volume` using `receiver_ctrl.set_volume` after connection in `src/main.rs`
- [ ] T013 [US3] Implement `--loop` logic in the playlist transition block of the event loop in `src/main.rs`

**Checkpoint**: User Story 3 functional - parity with core `castnow` features achieved.

---

## Phase N: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T014 [P] Final verification with all scenarios in `specs/018-castnow-feature-parity/quickstart.md`
- [ ] T015 [P] Code cleanup and performance check for FFmpeg/Torrent interaction

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies.
- **Foundational (Phase 2)**: Depends on Setup. BLOCKS all user stories.
- **User Stories (Phase 3 & 4)**: Can proceed in parallel after Phase 2.
- **User Story 3 (Phase 5)**: Depends on Phase 3 server changes.
- **Polish**: Depends on all stories.

---

## Implementation Strategy

### MVP First (User Story 1 & 2)

1. Foundation ready.
2. CLI Flags for network (US1).
3. Torrent stability (US2).

### Incremental Delivery

1. Basic parity (CLI flags).
2. Advanced features (Subtitles, Loop).
3. Performance tuning.
