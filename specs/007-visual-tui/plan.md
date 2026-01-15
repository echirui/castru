# Implementation Plan: Visual TUI (btop-style)

**Branch**: `007-visual-tui` | **Date**: 2026-01-15 | **Spec**: [specs/007-visual-tui/spec.md](spec.md)
**Input**: Feature specification from `/specs/007-visual-tui/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Refactor the existing single-line TUI into a full-screen, visually rich interface using `crossterm`'s Alternate Screen buffer. Include a robust playback toggle (Space) and a high-fidelity visual seekbar akin to `btop`.

## Technical Context

**Language/Version**: Rust 2021 Edition
**Primary Dependencies**: `crossterm` (existing)
**Storage**: N/A
**Testing**: Manual verification (TUI is hard to unit test)
**Target Platform**: Mac/Linux/Windows Terminal
**Project Type**: Single binary (`castru`)
**Performance Goals**: Responsive (60fps not needed, but reactive to state changes)
**Constraints**: Minimize dependencies (stick to `crossterm`, no `ratatui` unless necessary)
**Scale/Scope**: Small UI refactor (<200 LOC changes)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] **Dependency Minimalism**: Reuses `crossterm` which is already in `Cargo.toml`. No new crates.
- [x] **Library-First Architecture**: TUI logic remains encapsulated in `src/controllers/tui.rs`. `main.rs` only integrates it via channels.
- [x] **Test-First Development**: Logic logic (toggle) is testable, rendering is verified visually.
- [x] **Async I/O**: TUI input runs in a separate thread communicating via `mpsc`, compatible with `tokio` main loop.
- [x] **Secure Transport**: N/A for TUI.

## Project Structure

### Documentation (this feature)

```text
specs/007-visual-tui/
├── plan.md
├── spec.md
├── tasks.md
```

### Source Code (repository root)

```text
src/
├── controllers/
│   └── tui.rs         # TUI Controller logic and rendering
├── main.rs            # Integration and Panic Hooks
```

**Structure Decision**: Modify existing files. No new modules required.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| None | N/A | N/A |
