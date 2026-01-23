# Tasks - Replace PAUSED with Auto-Retry Waiting State

**Feature**: Replace PAUSED with Auto-Retry Waiting State
**Branch**: `026-replace-paused-with-sleep`
**Specification**: [specs/026-replace-paused-with-sleep/spec.md](spec.md)

## Implementation Strategy

We will implement this by strictly removing the `PlaybackStatus::Paused` state and `user_paused` flag, forcing all pauses (manual or system) to use the `Waiting` state with a 10-second timer. This will be done in two main phases: removing the old types to break the build, then fixing the logic to use the new `Waiting` flow.

MVP Scope: Complete replacement of PAUSED state (User Story 1 & 2 combined as they modify the same core logic).

## Phase 1: Setup

- [x] T001 Verify clean state by running tests before changes

## Phase 2: Foundational (Type Changes)

**Goal**: Update data models to remove `Paused` state, effectively forcing all logic to be updated.

- [x] T002 [P] Remove `PlaybackStatus::Paused` enum variant in `src/controllers/media.rs`
- [x] T003 Remove `user_paused` field from `AppState` struct in `src/app.rs`
- [x] T004 Rename `last_system_pause_time` to `pause_start_time` in `AppState` struct in `src/app.rs`

## Phase 3: Eliminate Static Pause (User Story 1 & 2)

**Goal**: Unify all pause logic to use `Waiting` state with 10s auto-resume.

**Independent Test**:
1. Play media -> Press Pause -> Verify `Waiting` state -> Wait 10s -> Verify Auto-Resume.
2. Play media -> Press Pause -> Press Play -> Verify Immediate Resume.

**Implementation Tasks**:

- [x] T005 [US1] Update `TuiCommand::Pause` handler in `src/app.rs` to set `current_status = Waiting` and `pause_start_time = Instant::now()`
- [x] T006 [US1] Update `TuiCommand::TogglePlay` handler in `src/app.rs` to toggle between `Playing` and `Waiting` (instead of `Paused`)
- [x] T007 [US1] Update `MediaStatus` event handler (PAUSED case) in `src/app.rs` to always transition to `Waiting` and set timer (remove `user_paused` checks)
- [x] T008 [US1] Update `Watchdog` logic in `src/app.rs` to check `pause_start_time` for `Waiting` status and trigger `load_media` or `play` after 10s
- [x] T009 [US2] Update `TuiCommand::Play` handler in `src/app.rs` to handle `Waiting` state by immediately resuming (clearing timer)
- [x] T010 [US1] Update `src/controllers/tui.rs` to remove `Paused` rendering logic and ensure `Waiting` displays correctly (optional: add timer countdown if feasible, otherwise static "WAITING")
- [x] T011 [US1] Clean up any remaining references to `user_paused` in `src/app.rs` (e.g., in `Next`/`Previous` handlers)

## Phase 4: Polish & Verification

- [x] T012 Run `cargo check` to ensure all `PlaybackStatus::Paused` usages are gone
- [x] T013 Run `cargo clippy` for linting
- [x] T014 Manual Verification: Test manual pause auto-resume (10s)
- [x] T015 Manual Verification: Test manual immediate resume from waiting

## Dependencies

- Phase 2 must strictly precede Phase 3 as it intentionally breaks the build to identify all call sites.
- Phase 3 implements the new logic.
