# Tasks: Auto Recover Buffering

**Feature Branch**: `025-auto-recover-buffering`
**Status**: Pending

## Phase 1: Foundational
*Goal: Update state definitions to support recovery state.*

- [x] T001 Modify `PlaybackStatus` enum in `src/controllers/media.rs` to include `Waiting` variant.
- [x] T002 Add `last_system_pause_time` field (Option<Instant>) to `AppState` struct in `src/app.rs`.

## Phase 2: Core Logic (Auto-Recovery)
*Goal: Implement detection and recovery timer.*

- [x] T003 [US1] Update `src/app.rs` event loop: When `PAUSED` status is received from receiver, check `user_paused`.
- [x] T004 [US1] Implement logic: If `PAUSED` and `!user_paused`, set internal status to `PlaybackStatus::Waiting` and set `last_system_pause_time`.
- [x] T005 [US1] Update `src/app.rs` loop (watchdog/tick): If status is `Waiting` and 10 seconds have elapsed, send `app.play()`.
- [x] T006 [US1] Update `src/app.rs` TUI command handler: If `Pause` command received while `Waiting`, transition to `Paused` (set `user_paused=true`).

## Phase 3: UI Updates
*Goal: Feedback to user.*

- [x] T007 [P] [US2] Update `src/controllers/tui.rs` `draw` function to handle `PlaybackStatus::Waiting` (display "WAITING" or "RECOVERING" in Magenta/Cyan).

## Phase 4: Polish
*Goal: Cleanup.*

- [x] T008 Verify no infinite loops (e.g. play -> immediate pause -> wait -> play). (Manual verification).

## Dependencies

- T003, T004, T005 depend on T001/T002.
- T007 depends on T001.

## Parallel Execution Opportunities

- T007 can be done in parallel with T003-T006.
