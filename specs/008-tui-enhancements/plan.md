# Implementation Plan: TUI Enhancements & Animation

**Branch**: `008-tui-enhancements` | **Date**: 2026-01-15 | **Spec**: [specs/008-tui-enhancements/spec.md](spec.md)
**Input**: Feature specification from `/specs/008-tui-enhancements/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Enhance the TUI to display rich metadata (Codec, Device Name) and implement a "spinning DVD" ASCII animation. The animation will be driven by a `tokio::interval` in the main event loop to ensure consistent frame updates without blocking.

## Technical Context

**Language/Version**: Rust 2021 Edition
**Primary Dependencies**: `crossterm` (existing)
**Storage**: N/A
**Testing**: Manual verification
**Target Platform**: Mac/Linux/Windows Terminal
**Project Type**: Single binary (`castru`)
**Performance Goals**: Low CPU usage for animation (100-200ms interval)
**Constraints**: Minimize redraw artifacts (use `crossterm` functionality efficiently)
**Scale/Scope**: UI Refactor

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] **Dependency Minimalism**: No new dependencies required.
- [x] **Library-First Architecture**: `TuiController` in `src/controllers/tui.rs` remains a stateless renderer (functionally), receiving `TuiState` from `main.rs`.
- [x] **Test-First Development**: Visual logic is hard to unit test, but state updates can be verified.
- [x] **Async I/O**: Animation loop uses `tokio::interval`, avoiding thread blocking.
- [x] **Secure Transport**: N/A.

## Project Structure

### Documentation (this feature)

```text
specs/008-tui-enhancements/
├── plan.md
├── spec.md
├── tasks.md
```

### Source Code (repository root)

```text
src/
├── controllers/
│   └── tui.rs         # Render logic updates (ASCII art, new fields)
├── main.rs            # Event loop updates (Animation tick, metadata piping)
```

**Structure Decision**: Modify existing files.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| None | N/A | N/A |
