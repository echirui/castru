# Tasks: Buffering Tuning and Refactoring

**Feature Branch**: `010-tune-buffering`
**Status**: Ready
**Total Tasks**: 11

## Dependencies

- **Phase 1 (Setup)**: Prerequisites
- **Phase 2 (Foundational)**: Core buffering structures
- **Phase 3 (User Story 1)**: Implementation and Tuning
- **Phase 4 (Polish)**: Verification and cleanup

## Phase 1: Setup

- [x] T001 Verify development environment and backup `src/server.rs` state

## Phase 2: Foundational

- [x] T002 Define `StreamConfig` struct and constants (CHUNK_SIZE, BUFFER_CAPACITY) in `src/server.rs`

## Phase 3: User Story 1 - Smooth Playback Optimization

**Goal**: Eliminate stuttering by decoupling file read from network write.
**Priority**: P1
**Independent Test**: Play a high-bitrate file and observe zero stuttering.

- [x] T003 [US1] Create unit tests for `producer_task` in `src/server.rs` (verify chunking and channel behavior)
- [x] T004 [US1] Implement `producer_task` function in `src/server.rs` that reads from file and sends to channel (detached logic)
- [x] T005 [US1] Implement `stream_file_buffered` helper in `src/server.rs` to spawn producer and run consumer loop
- [x] T006 [US1] Refactor `handle_connection` in `src/server.rs` to use `stream_file_buffered` instead of sync loop
- [x] T007 [US1] Tune `CHUNK_SIZE` (e.g., 512KB) and `CHANNEL_CAPACITY` (e.g., 8) for optimal performance based on manual testing

## Phase 4: Polish & Cross-Cutting Concerns

- [x] T008 Verify fix using `specs/010-tune-buffering/quickstart.md` with a high-bitrate video file (SC-001)
- [x] T009 Verify no regression in startup time/stability with standard (low/medium bitrate) files (SC-002)
- [x] T010 Verify stability under simulated system load (CPU/Disk activity) (AC2)
- [x] T011 Run `cargo fmt` and `cargo clippy` to ensure code quality

## Parallel Execution Opportunities

- T003 (Tests) and T004 (Implementation) are tightly coupled but can be iterated on.
- T008, T009, T010 can be performed in parallel by different testers or automated scripts if available.

## Implementation Strategy

1.  **Safety**: We will implement the new logic as a separate helper function (`stream_file_buffered`) first, keeping the old logic until the switch (T006).
2.  **TDD**: Unit tests for the producer (T003) ensure the reading logic is correct before hooking it up to the network.
3.  **Tuning**: Final values for buffer sizes will be set in T007 after integration.