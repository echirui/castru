# Tasks: Transcoded Seek Sync Fix

**Feature Branch**: `013-fix-transcode-seek-sync`
**Status**: Ready
**Total Tasks**: 8

## Dependencies

- **Phase 1 (Setup)**: Prerequisites
- **Phase 2 (Foundational)**: State Management
- **Phase 3 (User Story 1)**: Seek Synchronization Logic
- **Phase 4 (Polish)**: Verification and Cleanup

## Phase 1: Setup

- [x] T001 Verify development environment and backup `src/main.rs`

## Phase 2: Foundational

- [x] T002 Add `seek_offset: f64` field to `AppState` struct in `src/main.rs`

## Phase 3: User Story 1 - Accurate Seek Synchronization

**Goal**: Ensure TUI displays absolute time by adding seek offset to relative stream time.
**Priority**: P1
**Independent Test**: Seek in an MKV file and verify the TUI time display matches the visual content.

- [x] T003 [US1] Refactor `load_media` in `src/main.rs` to return the `applied_seek_offset` as part of the Result tuple
- [x] T004 [US1] Update `MediaResponse::MediaStatus` handling in `src/main.rs` to calculate `app_state.current_time` as `reported_time + app_state.seek_offset` when transcoding
- [x] T005 [US1] Update `TuiCommand::SeekForward` logic in `src/main.rs` to update `app_state.seek_offset` with the `new_time` when reloading a transcoded stream
- [x] T006 [US1] Update `TuiCommand::SeekBackward` logic in `src/main.rs` to update `app_state.seek_offset` with the `new_time` when reloading a transcoded stream
- [x] T007 [US1] Ensure `app_state.seek_offset` is reset to `0.0` during initial load and `TuiCommand::Next`/`Previous` in `src/main.rs`

## Phase 4: Polish & Cross-Cutting Concerns

- [x] T008 Run `cargo fmt` and `cargo clippy` to ensure code quality in `src/main.rs`

## Parallel Execution Opportunities

- T004, T005, and T006 are sequential modifications to the event loop logic in `src/main.rs`.
- This feature is mostly linear due to being contained within a single file's state machine.

## Implementation Strategy

1.  **State first**: Update the `AppState` struct so the field is available for logic.
2.  **API second**: Update `load_media` so it correctly communicates the intended offset to the rest of the application.
3.  **Loop third**: Connect the `MEDIA_STATUS` reporting to the `seek_offset` to fix the display issue.
4.  **Interaction fourth**: Fix the seek handlers to maintain the `seek_offset` across user actions.
