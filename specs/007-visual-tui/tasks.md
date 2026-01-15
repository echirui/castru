# Tasks: Visual TUI (btop-style)

**Feature**: Visual TUI
**Branch**: `007-visual-tui`
**Spec**: [specs/007-visual-tui/spec.md](spec.md)

## Phase 1: Logic & Input Fixes

**Purpose**: Fix the broken "Pause" behavior and prepare command structure.

- [x] T001 [Logic] Add `TogglePlay` to `TuiCommand` enum in `src/controllers/tui.rs`
- [x] T002 [Input] Map `Space` key to `TuiCommand::TogglePlay` in `src/controllers/tui.rs`
- [x] T003 [Main] Implement `TogglePlay` handling in `src/main.rs` (checking current status to Pause or Play)

## Phase 2: Full Screen Infrastructure

**Purpose**: Switch to Alternate Screen Buffer for full-screen UI.

- [x] T004 [TUI] Update `TuiController::start` to execute `EnterAlternateScreen`
- [x] T005 [TUI] Update `TuiController::stop` (and Drop) to execute `LeaveAlternateScreen`
- [x] T006 [TUI] Verify Panic Hook (revisit from previous task if not done) to ensure `LeaveAlternateScreen` is called on crash

## Phase 3: Visual Design (btop style)

**Purpose**: Implement the new multi-line/box rendering.

- [x] T007 [Render] Refactor `TuiController::draw` to redraw full screen (e.g. `Clear(All)` or optimized redraw)
- [x] T008 [Render] Design layout:
    - Header: Title / App Name
    - Content: Status (Big text or icon?), Media Title
    - Footer: Seekbar (Full width) + Volume + Hints (Keys)
- [x] T009 [Render] Implement `draw_seekbar` with higher fidelity (maybe use block characters like `█` for 'btop' feel)

## Phase 4: Integration & Polish

- [x] T010 [Verify] Verify `q` / `Ctrl+C` exit cleanly from Alternate Screen.
- [x] T011 [Verify] Verify Play/Pause toggle works.
