# Implementation Plan: Fix Transcode Seek Synchronization

**Branch**: `013-fix-transcode-seek-sync` | **Date**: 2026-01-15 | **Spec**: [spec.md](spec.md)
**Input**: Fix playback time inconsistency when seeking in non-mp4 (transcoded) files.

## Summary

The core issue is that `ffmpeg` output starts at `0.0` even if the source is seeked. We will resolve this by tracking a `seek_offset` in the application state and adding it to the relative timestamps reported by the Chromecast device.

## Technical Context

**Language/Version**: Rust 2021
**Primary Dependencies**: `tokio`, `ffmpeg` (external).
**Storage**: N/A
**Testing**: Manual verification using `.mkv` files and seeking.
**Target Platform**: Cross-platform.
**Project Type**: CLI.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] **Dependency Minimalism**: No new crates added.
- [x] **Library-First Architecture**: Changes localized to state management in `main.rs`.
- [x] **Async I/O**: Maintains non-blocking event loop.

## Project Structure

### Documentation (this feature)

```text
specs/013-fix-transcode-seek-sync/
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
├── main.rs              # Modified: Update AppState and event loop logic.
├── transcode.rs         # Verify: Ensure -ss is correctly applied (already exists).
```

**Structure Decision**: Minimal changes to `main.rs` to track and apply temporal offsets.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| N/A | | |