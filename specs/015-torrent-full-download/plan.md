# Implementation Plan: Torrent Full Download and Playback

**Branch**: `015-torrent-full-download` | **Date**: 2026-01-15 | **Spec**: [specs/015-torrent-full-download/spec.md](spec.md)
**Input**: Feature specification from `/specs/015-torrent-full-download/spec.md`

## Summary

This feature implements a "download everything first" policy for torrent playback. We will enhance the TUI to show download progress as a percentage and block media casting until the torrent is 100% complete.

## Technical Context

**Language/Version**: Rust 2021  
**Primary Dependencies**: `librqbit`, `tokio`, `crossterm`  
**Storage**: N/A (Memory state + temp filesystem)  
**Testing**: `cargo test`, Manual magnet verification  
**Target Platform**: Any supporting `ffmpeg` and `crossterm`
**Project Type**: Single project  
**Performance Goals**: Accurate progress updates (<1s frequency)  
**Constraints**: Must not freeze the UI while downloading.  
**Scale/Scope**: Impacts `main.rs` load logic and `tui.rs` rendering.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] **Dependency Minimalism**: No new dependencies.
- [x] **Library-First**: Logic remains in library modules where possible.
- [x] **Async I/O**: Uses `tokio` for the download loop.
- [x] **Secure Transport**: N/A for torrent (standard protocol).

## Project Structure

### Documentation (this feature)

```text
specs/015-torrent-full-download/
├── plan.md              # This file
├── research.md          # librqbit stats research
├── data-model.md        # AppState updates
├── quickstart.md        # Verification
└── checklists/
    └── requirements.md  
```

### Source Code (repository root)

```text
src/
├── main.rs              # Update: Implement 100% wait loop in load_media
├── controllers/
│   └── tui.rs           # Update: Add progress display to renderer
└── torrent/
    └── mod.rs           # Update: Ensure TorrentStreamInfo has enough metadata
```

## Phase 0: Outline & Research

1.  **Extract stats**: Verify `ManagedTorrent::stats().progress_bytes` and `total_bytes`.
2.  **TUI Design**: Decide where to place the "Downloading: XX%" text (re-use status line).

## Phase 1: Design & Contracts

1.  **AppState**: Add `torrent_progress: Option<f32>`.
2.  **TuiState**: Add `torrent_progress: Option<f32>`.
3.  **Refactor `load_media`**: Change buffering logic to 100% completion.

## Phase 2: Implementation Strategy

- **Step 1**: Update `TuiState` and `TuiController::draw`.
- **Step 2**: Implement the wait loop in `load_media` with TUI updates.
- **Step 3**: Verify transition from `DOWNLOADING` to `PLAYING`.