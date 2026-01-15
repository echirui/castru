# Implementation Plan: Fix Torrent Playback

**Branch**: `012-fix-torrent-playback` | **Date**: 2026-01-15 | **Spec**: [specs/012-fix-torrent-playback/spec.md](spec.md)
**Input**: Feature specification from `specs/012-fix-torrent-playback/spec.md`

## Summary

Fix the issue where torrent playback fails (black screen/stuck) by ensuring data is available before streaming. This involves two key changes:
1.  **Pre-buffering**: The CLI will wait for a safe threshold (e.g., 3% or first few chunks) before sending the load command to the Chromecast.
2.  **Safety Check**: The `GrowingFile` adapter will explicitly check `librqbit`'s piece status before reading, preventing the delivery of zero-filled sparse data.

## Technical Context

**Language/Version**: Rust 2021
**Primary Dependencies**: `librqbit` (existing), `tokio`.
**Storage**: N/A
**Testing**: Manual verification with Magnet link.
**Target Platform**: Cross-platform.
**Project Type**: Library + CLI.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] **Dependency Minimalism**: No new dependencies.
- [x] **Library-First Architecture**: `GrowingFile` logic remains in `src/torrent/`.
- [x] **Async I/O**: `GrowingFile` uses `Poll::Pending` to block non-destructively.

## Project Structure

### Documentation (this feature)

```text
specs/012-fix-torrent-playback/
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
├── main.rs              # Update: Add buffering loop
└── torrent/
    ├── manager.rs       # Update: Return handle for monitoring
    └── stream.rs        # Update: Add is_present check
```

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| Polling in Reader | `librqbit` doesn't expose async piece notification easily. | Creating a full async notification system is overkill. |