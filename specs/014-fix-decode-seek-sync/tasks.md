# Tasks: Accurate Seek and Playback Synchronization

**Input**: Design documents from `/specs/014-fix-decode-seek-sync/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, quickstart.md

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [x] T001 [P] Verify `ffmpeg` and `ffprobe` are in system path (Manual check)
- [x] T002 [P] Create a test non-mp4 media file (e.g., using ffmpeg) for local verification

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T003 Add `last_update_instant: std::time::Instant` and `last_known_time: f64` to `AppState` in `src/main.rs`
- [x] T004 Add `WatchdogConfig` constants (e.g., `WATCHDOG_TIMEOUT_SEC = 5`) to `src/main.rs`
- [x] T005 [P] Implement `clear_transcode` improvements in `src/server.rs` to ensure clean handovers

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Seek during Transcoding (Priority: P1) 🎯 MVP

**Goal**: Support seeking in transcoded streams by restarting FFmpeg at the target offset.

**Independent Test**: Play a transcoded file and perform a seek using Arrow keys. The stream should resume from the new position.

### Implementation for User Story 1

- [x] T006 [P] [US1] Update `TranscodeConfig` in `src/transcode.rs` to ensure `start_time` is correctly typed (f64)
- [x] T007 [US1] Refactor `load_media` in `src/main.rs` to return `applied_seek_offset` and update `app_state.seek_offset`
- [x] T008 [US1] Update `TuiCommand::SeekForward` and `SeekBackward` in `src/main.rs` to call `load_media` when `is_transcoding` is true
- [x] T009 [US1] Implement FFmpeg process cleanup in `src/server.rs` to prevent zombie processes during rapid seeks
- [x] T010 [US1] Add log events for seek operations in `src/main.rs` for easier debugging

**Checkpoint**: At this point, seeking in transcoded streams should be functional.

---

## Phase 4: User Story 2 - Accurate Playback Time after Seek (Priority: P1)

**Goal**: Ensure TUI displays the absolute playback time by adding the seek offset.

**Independent Test**: Seek to 1:00 in a video and verify that the TUI shows "01:00" instead of "00:00".

### Implementation for User Story 2

- [x] T011 [US2] Update `MediaResponse::MediaStatus` handler in `src/main.rs` to calculate `current_time = reported_time + app_state.seek_offset`
- [x] T012 [US2] Update animation tick interval in `src/main.rs` to increment `current_time` relative to the current offset
- [x] T013 [P] [US2] Ensure `TuiState` in `src/controllers/tui.rs` correctly handles large time values for formatting
- [x] T014 [US2] Add logic to reset `seek_offset` to 0.0 when loading a new file from the playlist in `src/main.rs`

**Checkpoint**: At this point, the playback time should be accurate across seeks.

---

## Phase 5: Connection Resilience & Resume

**Goal**: Automatically resume playback if the connection fails or gets stuck.

**Independent Test**: Simulate a network interruption; the app should automatically attempt to resume from the last known time.

### Implementation for Connection Resilience

- [x] T015 Implement the watchdog loop in the `main.rs` event loop to monitor `current_time` progress
- [x] T016 Implement `resume_playback` logic in `src/main.rs` that triggers a reload if the watchdog times out
- [x] T017 Handle `IDLE` with reason `ERROR` in `MediaStatus` handler in `src/main.rs` to trigger immediate resume

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [x] T018 [P] Refactor `load_media` to reduce code duplication for different `MediaSource` types in `src/main.rs`
- [x] T019 Performance optimization for FFmpeg startup (e.g., verifying `-preset ultrafast` is used) in `src/transcode.rs`
- [x] T020 [P] Run `quickstart.md` validation scenarios to ensure all features work as expected

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Can start immediately.
- **Foundational (Phase 2)**: BLOCKS all user stories.
- **User Stories (Phase 3 & 4)**: Can proceed in parallel after Phase 2.
- **Resilience (Phase 5)**: Depends on User Story 1 & 2 logic being stable.

### Parallel Opportunities

- T001, T002, T005, T006, T013, T018, T020 are all parallelizable.
- User Story 1 and User Story 2 can be developed somewhat in parallel once `load_media` is refactored.

---

## Implementation Strategy

### MVP First (User Story 1 & 2)

1. Setup and Foundation.
2. Implement US1 (Seek functionality).
3. Implement US2 (Time synchronization).
4. **VALIDATE**: Ensure seeking works and time is correct.

### Incremental Delivery

1. Basic Seeking + Sync (MVP).
2. Connection Watchdog (Resilience).
3. Polish and Refactoring.
