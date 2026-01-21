# Tasks: Auto Resume Buffering

**Feature Branch**: `023-auto-resume-buffering`
**Status**: Pending

## Phase 1: Foundational
*Goal: Establish state tracking for buffering and user overrides.*

- [x] T001 [US2] Add `user_paused` boolean field to `AppState` in `src/app.rs` to distinguish intentional pause.
- [x] T002 [US2] Update `TuiCommand::TogglePlay`, `Pause`, `Play` handlers in `src/app.rs` to set `user_paused` accordingly.

## Phase 2: Core Logic (Auto-Resume)
*Goal: Implement automatic buffering and resumption logic.*

- [x] T003 [US1] Modify `src/app.rs` main loop (animation/watchdog tick) to check download progress vs playback position.
- [x] T004 [US1] Implement logic: If `downloaded_bytes` is too close to `playback_pos` (converted to bytes approx or time threshold), transition to `BUFFERING`.
- [x] T005 [US1] Implement logic: If in `BUFFERING` and `downloaded_bytes` > `safe_threshold`, and `!user_paused`, auto-resume playback (call `app.play()`).
- [x] T006 [US1] Ensure `BUFFERING` state doesn't trigger if `user_paused` is true (remain `PAUSED`).

## Phase 3: UI Updates
*Goal: Reflect buffering status in TUI.*

- [x] T007 [P] [US1] Verify `src/controllers/tui.rs` handles `PlaybackStatus::Buffering` correctly (display "BUFFERING"). *Note: Existing code likely does this, verify and tweak if needed.*

## Phase 4: Polish
*Goal: Tuning thresholds.*

- [x] T008 Tune buffer thresholds (e.g., 5MB ahead for resume, 1MB ahead for pause) in constants.

## Dependencies

- T004, T005 depend on T001.

## Parallel Execution Opportunities

- T007 can be checked while core logic is being written.