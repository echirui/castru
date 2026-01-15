# Implementation Plan: Projector Animation

**Branch**: `009-projector-animation` | **Date**: 2026-01-15 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `specs/009-projector-animation/spec.md`

## Summary

Replace the existing procedural 3D Cube animation in the TUI with a specific 4-frame ASCII art animation of a film projector. This involves replacing the rendering logic in `src/controllers/tui.rs` to cycle through static string arrays instead of calculating 3D geometry.

## Technical Context

**Language/Version**: Rust 2021
**Primary Dependencies**: `crossterm` (existing), `tokio` (existing)
**Storage**: N/A (Stateless)
**Testing**: `cargo test` (Unit tests for frame generation)
**Target Platform**: macOS/Linux (CLI TUI)
**Project Type**: Library/CLI
**Performance Goals**: < 16ms render time (60fps equivalent), minimal CPU usage.
**Constraints**: Terminal size must accommodate the ASCII art (~40x15).
**Scale/Scope**: Single controller file modification.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] **Dependency Minimalism**: No new dependencies added.
- [x] **Library-First Architecture**: Changes are confined to `TuiController` logic within the existing structure.
- [x] **Test-First Development**: Will implement unit tests for frame index logic and output dimensions.
- [x] **Async I/O**: TUI event loop remains non-blocking (existing pattern).
- [x] **Secure Transport**: N/A for visual rendering.

## Project Structure

### Documentation (this feature)

```text
specs/009-projector-animation/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
└── checklists/
    └── requirements.md
```

### Source Code (repository root)

```text
src/
└── controllers/
    └── tui.rs           # Modified: Replace render_cube_frame with render_projector_frame
```

**Structure Decision**: Modify existing `src/controllers/tui.rs` as it encapsulates all TUI logic. No new files required for source code.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| N/A | | |