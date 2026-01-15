# Tasks: Castnow-like TUI

**Feature**: Castnow-like TUI
**Branch**: `006-castnow-tui`
**Spec**: [specs/006-castnow-tui/spec.md](spec.md)

## Phase 1: Setup

**Purpose**: Initialize TUI module structures and update state management.

- [x] T001 [Setup] Update `TuiCommand` enum in `src/controllers/tui.rs` to include new commands (VolumeUp, VolumeDown, ToggleMute)
- [x] T002 [Setup] Create `TuiState` struct in `src/controllers/tui.rs` to hold all rendering data (status, time, volume, title)
- [x] T003 [Setup] Update `TuiController::draw_status` signature to accept `&TuiState` instead of individual arguments

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Ensure backend can supply necessary data to TUI.

- [x] T004 [Foundational] Ensure `MediaStatus` parsing in `src/main.rs` extracts Volume level and Mute state
- [x] T005 [Foundational] Update `AppState` in `src/main.rs` to track volume and mute status alongside playback info

---

## Phase 3: User Story 1 - Real-time Playback Status (Priority: P1)

**Goal**: Display a dynamic, single-line status bar with progress.

**Independent Test**: Running `cast` shows a progress bar that updates in-place without scrolling.

### Implementation for User Story 1

- [x] T006 [US1] Implement `format_duration` helper in `src/controllers/tui.rs` for MM:SS display
- [x] T007 [US1] Implement `render_progress_bar` helper in `src/controllers/tui.rs` that scales to terminal width
- [x] T008 [US1] Implement `TuiController::draw` method using `crossterm` to clear line and print formatted state string
- [x] T009 [US1] Update `src/main.rs` loop to construct `TuiState` and call `tui.draw()` on status updates

---

## Phase 4: User Story 2 - Keyboard Playback Controls (Priority: P1)

**Goal**: Control playback and volume via keyboard.

**Independent Test**: Pressing Arrow keys adjusts volume/seek; Space toggles pause.

### Implementation for User Story 2

- [x] T010 [US2] Update `start` input loop in `src/controllers/tui.rs` to map Up/Down arrows to `VolumeUp`/`VolumeDown`
- [x] T011 [US2] Update `start` input loop in `src/controllers/tui.rs` to map 'm' to `ToggleMute`
- [x] T012 [US2] Update `src/main.rs` to handle `VolumeUp`/`VolumeDown` commands by calling `media.set_volume` (needs implementation on MediaController?)
      *(Note: Volume is usually on ReceiverController or MediaController depending on level. `SET_VOLUME` is often a Receiver command `urn:x-cast:com.google.cast.receiver`, but Media also has volume. Let's send to Receiver for global control or check best practice. Spec says "standard keyboard shortcuts". Castv2 usually controls receiver volume.)*
- [x] T013 [US2] Update `src/main.rs` to handle `ToggleMute` command
- [x] T014 [US2] Verify `SeekForward`/`SeekBackward` are already mapped and working from previous feature (if not, add/fix)

---

## Phase 5: User Story 3 - Clean Terminal Interface (Priority: P2)

**Goal**: Ensure the terminal remains clean and restores state on exit.

**Independent Test**: App exit leaves cursor visible and no stray characters.

### Implementation for User Story 3

- [x] T015 [US3] Verify `Drop` implementation in `src/controllers/tui.rs` restores cursor and raw mode
- [ ] T016 [US3] Add `crossterm` panic hook or signal handler in `src/main.rs` to ensure terminal restoration on crash (Best Effort)

---

## Phase 6: Polish & Cross-Cutting Concerns

- [x] T017 [Polish] Add color coding to status state (Green=Playing, Yellow=Paused, etc.) in `src/controllers/tui.rs`
- [ ] T018 [Polish] Handle long titles by truncating in `render_status_line`
- [x] T019 [Doc] Update `README.md` with new keyboard shortcuts table

## Implementation Strategy

1.  **Refactor TuiState**: First consolidate all UI data into a struct.
2.  **Visuals**: Implement the single-line renderer.
3.  **Input**: Add the remaining volume/mute mappings.
4.  **Integration**: Connect the new commands in `main.rs`.

