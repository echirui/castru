# Implementation Plan: Castnow-like TUI

**Branch**: `006-castnow-tui` | **Date**: 2026-01-15 | **Spec**: [specs/006-castnow-tui/spec.md](spec.md)
**Input**: Feature specification from `/specs/006-castnow-tui/spec.md`

## Summary

This feature implements a polished, "castnow-like" Terminal User Interface (TUI) for `castru`. It replaces the basic debug logs with a single, dynamic status line that displays playback state, timestamps, a visual progress bar, and volume. It also enhances the keyboard control system to support seeking (Arrow keys), volume control (Up/Down), and quitting (q/Esc), providing a seamless and minimal user interaction experience consistent with command-line media players.

## Technical Context

**Language/Version**: Rust 2021
**Primary Dependencies**: `tokio` (full), `crossterm` (existing), `mdns-sd`.
**Storage**: N/A
**Testing**: `cargo test` (unit tests for TUI formatting logic).
**Target Platform**: Desktop (macOS/Linux).
**Project Type**: Single (Library + CLI).
**Performance Goals**: UI update latency <100ms.
**Constraints**: Must use `crossterm` for raw mode and rendering. Must not block the main Tokio runtime.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Dependency Minimalism**: Reusing `crossterm` which is already a dependency. No new UI crates (like `ratatui`) to keep it simple and "castnow-like" (minimalist).
- **Library-First**: TUI logic remains in `src/controllers/tui.rs`, decoupling rendering from the main event loop where possible.
- **Async I/O**: Input handling uses `tokio::sync::mpsc` and non-blocking polling, consistent with Constitution IV.
- **Secure Transport**: N/A (UI only).

## Project Structure

### Documentation (this feature)

```text
specs/006-castnow-tui/
├── plan.md              # This file
├── research.md          # Visual layout, ANSI codes, Input mapping
├── data-model.md        # TuiState, InputEvent
├── quickstart.md        # Updated CLI controls
└── tasks.md             # Implementation tasks
```

### Source Code (repository root)

```text
src/
├── controllers/
│   ├── tui.rs           # Update: Enhanced rendering and input handling
│   └── media.rs         # No change expected, but maybe volume control helpers
└── main.rs              # Update: Event loop integration for volume/seek/status
```

**Structure Decision**: enhance existing `tui.rs` module.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| N/A | | |
