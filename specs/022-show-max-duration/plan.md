# Implementation Plan: Show Max Duration

**Branch**: `022-show-max-duration` | **Date**: 2026-01-18 | **Spec**: [specs/022-show-max-duration/spec.md](spec.md)
**Input**: Feature specification from `specs/022-show-max-duration/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

This feature implements automatic duration discovery for media streamed via torrents (magnet links). By asynchronously probing the media file once sufficient data (header) is downloaded, the application will display the total duration in the TUI, improving the user experience for streaming content.

## Technical Context

**Language/Version**: Rust 2021 Edition
**Primary Dependencies**: `tokio` (async runtime), `librqbit` (torrent), `ffmpeg` (via `std::process::Command` for probing).
**Storage**: N/A (Temporary file storage handled by `librqbit`).
**Testing**: `cargo test` for unit tests. Integration tests for probing logic.
**Target Platform**: Cross-platform (Linux/macOS primary dev environment).
**Project Type**: Rust Library + CLI.
**Performance Goals**: Duration displayed within 10s of download start. No blocking of main loop.
**Constraints**: Must not introduce new heavy dependencies. `ffprobe` assumed available (existing constraint).
**Scale/Scope**: Impacts `TorrentManager`, `MediaController` (loading logic), and `TuiController` (display).

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] **Dependency Minimalism**: No new crates. Uses existing `ffmpeg` dependency (via CLI).
- [x] **Library-First Architecture**: Logic resides in `src/transcode.rs` and `src/app.rs` (or `src/lib.rs` if refactored).
- [x] **Test-First Development**: Unit tests for probe parsing required.
- [x] **Async I/O**: Probing uses `tokio::process`.
- [x] **Secure Transport**: N/A for local file probing.

## Project Structure

### Documentation (this feature)

```text
specs/022-show-max-duration/
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
├── app.rs               # Orchestration of probing and state update
├── transcode.rs         # Enhanced probe_media function
├── torrent/
│   └── manager.rs       # Torrent info retrieval
└── controllers/
    └── tui.rs           # Rendering updates
```

**Structure Decision**: enhance existing modules.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| N/A | | |