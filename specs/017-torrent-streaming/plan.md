# Implementation Plan: Torrent Streaming while Downloading

**Branch**: `017-torrent-streaming` | **Date**: 2026-01-15 | **Spec**: [specs/017-torrent-streaming/spec.md](spec.md)
**Input**: torrentでダウンロード中からcastするように修正してください。100%行かなくても再生可能であればstreamingしてください。

## Summary

This feature reverts the "wait for 100%" policy and implements a buffering threshold (3% or 10MB) to allow early playback of torrents. It also enables background progress tracking and TUI updates during playback.

## Technical Context

**Language/Version**: Rust 2021  
**Primary Dependencies**: `librqbit`, `tokio`, `crossterm`  
**Storage**: N/A (Memory state + temp filesystem)  
**Testing**: `cargo test`, Manual magnet verification  
**Target Platform**: All platforms supporting `ffmpeg` and `crossterm`.
**Project Type**: Single project  
**Performance Goals**: Start playback < 30s for well-seeded torrents.  
**Constraints**: Must not interfere with the main event loop.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] **Dependency Minimalism**: No new dependencies.
- [x] **Library-First**: Core logic remains in `main.rs` and library modules.
- [x] **Async I/O**: Uses `tokio` for background updates.
- [x] **Secure Transport**: Unchanged.

## Project Structure

### Documentation (this feature)

```text
specs/017-torrent-streaming/
├── plan.md              # This file
├── research.md          # Threshold and tracking research
├── data-model.md        # State transitions
├── quickstart.md        # Verification steps
└── checklists/
    └── requirements.md  
```

### Source Code (repository root)

```text
src/
├── main.rs              # Update: AppState, load_media threshold, tick loop
└── controllers/
    └── tui.rs           # Update: TUI rendering for concurrent progress
```

## Phase 0: Outline & Research

1.  **Threshold Analysis**: Determined a hybrid threshold (3% or 10MB) is optimal.
2.  **Tracking Strategy**: Store `ManagedTorrent` handle in `AppState` for periodic polling in the tick loop.

## Phase 1: Design & Contracts

1.  **AppState**: Add `torrent_handle: Option<Arc<librqbit::ManagedTorrent>>`.
2.  **Wait Logic**: Refactor `wait_for_torrent_download` to exit early.
3.  **TUI rendering**: Enhance `TuiController::draw` to show `(DL: XX.X%)` during playback.

## Phase 2: Implementation Strategy

- **Step 1**: Update `AppState` and import `librqbit::ManagedTorrent`.
- **Step 2**: Modify `wait_for_torrent_download` to support early exit.
- **Step 3**: Update the animation tick loop to query torrent stats.
- **Step 4**: Refine TUI rendering to show download progress during playback.