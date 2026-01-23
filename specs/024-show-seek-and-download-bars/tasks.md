# Tasks: Show Seek and Download Bars

**Feature Branch**: `024-show-seek-and-download-bars`
**Status**: Pending

## Phase 1: Setup
*Goal: Prepare for TUI changes.*

- [x] T001 Verify current TUI layout by running with a mock torrent state (Mental check or temporary code, no permanent task needed).

## Phase 2: Core Implementation (US1 & US2)
*Goal: Display both bars simultaneously with visual distinction.*

- [x] T002 [US1] Modify `src/controllers/tui.rs`: Update `draw` function layout calculation to support dynamic vertical spacing.
- [x] T003 [US1] Modify `src/controllers/tui.rs`: Remove mutual exclusion between playback seek bar and download progress bar.
- [x] T004 [US2] Modify `src/controllers/tui.rs`: Implement dual-bar rendering logic:
    - Playback Bar at `base_y` (White).
    - Download Bar at `base_y + 1` (Yellow/Cyan) if torrent active.
    - Add labels "Play:" and "Load:" (or similar) prefix to bars for distinction.
- [x] T005 [US1] Modify `src/controllers/tui.rs`: Shift Codec and Volume information lines down by 1 row when download bar is visible.

## Phase 3: Polish
*Goal: Ensure layout stability.*

- [x] T006 Ensure footer and other elements don't overlap if terminal height is small (clamp Y coordinates).

## Dependencies

- All tasks are sequential within `src/controllers/tui.rs`.

## Parallel Execution Opportunities

- None, single file modification.
