# Tasks: Show Max Duration

**Feature Branch**: `022-show-max-duration`
**Status**: Pending

## Phase 1: Setup
*Goal: Ensure environment is ready.*

- [x] T001 Verify `ffprobe` is installed and accessible in the system path (Manual Check)

## Phase 2: Foundational
*Goal: Establish internal event structures for async communication.*

- [x] T002 [P] Define `InternalEvent` enum in `src/app.rs` to handle `ProbeCompleted` messages containing duration and codec info

## Phase 3: User Story 1 - View Total Duration for Torrent Media (P1)
*Goal: Display total duration for magnet links as soon as metadata is available.*
*Independent Test: Stream a magnet link and verify duration appears in TUI within 10 seconds.*

### Tests
- [x] T003 [P] [US1] Add unit test in `src/transcode.rs` to verify `probe_media` parsing logic with mock FFprobe output

### Implementation
- [x] T004 [US1] Initialize `mpsc` channel for `InternalEvent` in `src/app.rs` (`probe_tx`, `probe_rx`)
- [x] T005 [US1] Modify `wait_for_torrent_download` in `src/app.rs` to spawn a background task that triggers `probe_media` once ~2MB is downloaded
- [x] T006 [US1] Update the main event loop in `src/app.rs` to handle `InternalEvent::ProbeCompleted` and update `AppState`
- [x] T007 [P] [US1] Verify and ensure `src/controllers/tui.rs` correctly formats and displays `app_state.total_duration` (HH:MM:SS)

## Phase 4: Polish
*Goal: Robustness and cleanup.*

- [x] T008 Implement logic to prevent repeated probe attempts if the first one fails or if duration is already found
- [x] T009 Run `cargo clippy` and fix any new warnings

## Dependencies

- Phase 3 depends on Phase 2 (Event structure).
- T006 depends on T004.

## Parallel Execution Opportunities

- T003 (Test) and T007 (TUI verification) can be done in parallel with T004/T005.
