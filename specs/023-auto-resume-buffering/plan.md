# Implementation Plan: Auto Resume Buffering

**Branch**: `023-auto-resume-buffering` | **Date**: 2026-01-18 | **Spec**: [specs/023-auto-resume-buffering/spec.md](spec.md)
**Input**: Feature specification from `specs/023-auto-resume-buffering/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

This feature enhances the media playback experience by implementing an automatic buffering mechanism. Instead of pausing indefinitely when download or encoding lags, the player will transition to a `BUFFERING` state and automatically resume `PLAYING` once sufficient data is available. The `PAUSED` state will be strictly reserved for explicit user actions.

## Technical Context

**Language/Version**: Rust 2021 Edition
**Primary Dependencies**: `tokio` (async runtime), `librqbit` (torrent), `crossterm` (TUI).
**Storage**: N/A (State is in-memory `AppState`).
**Testing**: `cargo test` for unit tests. Manual verification for timing-dependent behavior.
**Target Platform**: Cross-platform (Linux/macOS primary dev environment).
**Project Type**: Rust Library + CLI.
**Performance Goals**: Seamless transition between buffering and playing without user intervention.
**Constraints**: Must not introduce new dependencies. Must not block the main event loop.
**Scale/Scope**: Impacts `src/app.rs` (main loop state machine) and `src/controllers/tui.rs` (display).

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] **Dependency Minimalism**: No new dependencies required.
- [x] **Library-First Architecture**: Logic resides in `src/app.rs` (cli logic) but should ideally be in `src/lib.rs` if possible, though `AppState` is currently local to `app.rs`. *Refinement*: Logic will be kept in `app.rs` as it orchestrates the TUI and playback loop, consistent with current architecture for CLI-specific behavior.
- [x] **Test-First Development**: Unit tests for state transitions (if extractable) or integration tests.
- [x] **Async I/O**: Buffering checks are non-blocking.
- [x] **Secure Transport**: N/A for internal state logic.

## Project Structure

### Documentation (this feature)

```text
specs/023-auto-resume-buffering/
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
├── app.rs               # Main event loop and state machine updates
├── controllers/
│   ├── media.rs         # PlaybackStatus enum update
│   └── tui.rs           # Rendering update for BUFFERING state
```

**Structure Decision**: enhance existing modules.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| N/A | | |