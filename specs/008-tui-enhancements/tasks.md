# Tasks: TUI Enhancements & Animation

**Feature**: TUI Enhancements & Animation
**Branch**: `008-tui-enhancements`
**Spec**: [specs/008-tui-enhancements/spec.md](spec.md)

## Phase 1: Logic & State Updates

**Purpose**: Update state structures to hold new metadata and animation counters.

- [x] T001 [Setup] Update `TuiState` struct in `src/controllers/tui.rs` to include `video_codec`, `audio_codec`, `device_name`, and `animation_frame`.
- [x] T002 [Logic] Update `AppState` in `src/main.rs` to track codec info (from `MediaProbeResult`) and Device Name (from `CastDevice`).
- [x] T003 [Logic] Add logic in `src/main.rs` to pipe `video_codec`, `audio_codec`, and `device_name` into `TuiState` during `draw`.

## Phase 2: Animation Infrastructure

**Purpose**: Drive the animation loop in the main application.

- [x] T004 [Logic] Initialize a `tokio::time::interval` in `src/main.rs` (e.g., 200ms) for animation ticking.
- [x] T005 [Logic] Implement the animation loop in `src/main.rs`: on every tick, increment an `animation_counter` and trigger a TUI redraw if status is `PLAYING`.

## Phase 3: Visual Implementation

**Purpose**: Render the new fields and animation characters.

- [x] T006 [Render] Implement `get_animation_char(frame: usize) -> char` in `src/controllers/tui.rs` using Unicode discs (`◐ ◓ ◑ ◒`).
- [x] T007 [Render] Update `TuiController::draw` to render:
    - **Header**: `State.device_name`
    - **Center**: Animation character (only if Playing).
    - **Details**: `Video: <codec> | Audio: <codec>` below the media title.

## Phase 4: Integration & Verify

**Purpose**: Ensure smooth operation and correctness.

- [x] T008 [Verify] Verify Codec info is displayed correctly for local files (from `load_media` probe).
- [x] T009 [Verify] Verify Device Name is displayed.
- [x] T010 [Verify] Verify Animation spins when playing and stops (or disappears/pauses) when paused.
