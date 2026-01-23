# Implementation Plan: Show Seek and Download Bars

**Branch**: `024-show-seek-and-download-bars` | **Date**: 2026-01-18 | **Spec**: [specs/024-show-seek-and-download-bars/spec.md](spec.md)
**Input**: Feature specification from `specs/024-show-seek-and-download-bars/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

This feature updates the TUI to simultaneously display both the playback seek bar and the download progress bar when streaming a torrent. This provides users with better visibility into the buffering state and playback position relative to the downloaded content.

## Technical Context

**Language/Version**: Rust 2021 Edition
**Primary Dependencies**: `crossterm` (TUI), `tokio` (async).
**Storage**: N/A (State is in-memory `TuiState` and `AppState`).
**Testing**: `cargo test` for unit tests. Manual verification for TUI layout.
**Target Platform**: Cross-platform (Linux/macOS primary dev environment).
**Project Type**: Rust Library + CLI.
**Performance Goals**: TUI rendering must remain flicker-free and responsive (<16ms frame time).
**Constraints**: Limited terminal height (must fit within standard 24 rows, though adaptive is better).
**Scale/Scope**: Impacts `src/controllers/tui.rs` rendering logic.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] **Dependency Minimalism**: No new dependencies required.
- [x] **Library-First Architecture**: TUI logic is separated in `controllers/tui.rs`.
- [x] **Test-First Development**: Can test helper functions for bar rendering strings.
- [x] **Async I/O**: TUI loop is already async-friendly (runs in thread, communicates via channel).
- [x] **Secure Transport**: N/A for TUI rendering.

## Project Structure

### Documentation (this feature)

```text
specs/024-show-seek-and-download-bars/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
└── tasks.md             # Phase 2 output
```

### Source Code (repository root)

```text
src/
└── controllers/
    └── tui.rs           # Rendering logic updates
```

**Structure Decision**: Modify existing TUI controller.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| N/A | | |