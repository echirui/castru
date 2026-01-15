# Implementation Plan: Accurate Seek and Playback Synchronization

**Branch**: `014-fix-decode-seek-sync` | **Date**: 2026-01-15 | **Spec**: [specs/014-fix-decode-seek-sync/spec.md](spec.md)
**Input**: decodeが発生した場合、seekが機能しません。あと、seekした際に再生時間が誤ったものになります。大抵は0になります。別のcodecに変換した場合、connectionの状態を確認し必要があれば、現状の再生時間から接続をレジュームするようにして欲しい。

## Summary

This feature addresses critical issues in the transcoding pipeline:
1.  **Seek Accuracy**: Fixes the 0-time reset by properly tracking and applying a `seek_offset` when FFmpeg is restarted with `-ss`.
2.  **Connection Resilience**: Implements a connection watchdog that detects playback interruptions and automatically resumes from the last known good timestamp.
3.  **Synchronization**: Ensures the TUI correctly calculates absolute time as `reported_time + seek_offset`.

## Technical Context

**Language/Version**: Rust 2021  
**Primary Dependencies**: `tokio`, `ffmpeg` (external), `prost`, `serde_json`  
**Storage**: N/A (Memory-based state)  
**Testing**: `cargo test`, Manual verification with various codecs  
**Target Platform**: Linux/macOS/Windows (Anywhere `ffmpeg` is available)
**Project Type**: Single project  
**Performance Goals**: Seek resume < 3s, Time drift < 500ms  
**Constraints**: Must use existing `StreamServer` and `transcode.rs` logic.  
**Scale/Scope**: Core playback logic update.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] **Dependency Minimalism**: No new dependencies added.
- [x] **Library-First**: Core logic resides in `src/transcode.rs` and `src/server.rs`.
- [x] **Async I/O**: Uses `tokio::process` and `tokio::select!`.
- [x] **Secure Transport**: TLS via `rustls` (unchanged).

## Project Structure

### Documentation (this feature)

```text
specs/014-fix-decode-seek-sync/
├── plan.md              # This file
├── research.md          # Research on seek_offset and ffmpeg
├── data-model.md        # AppState and TranscodeConfig updates
├── quickstart.md        # Verification steps
└── checklists/
    └── requirements.md  # Spec quality checklist
```

### Source Code (repository root)

```text
src/
├── transcode.rs         # Update: Improve ffmpeg spawning and probe accuracy
├── server.rs            # Update: Ensure transcode cleanup closes old connections
├── main.rs              # Update: Implement watchdog and seek_offset logic
└── controllers/
    └── tui.rs           # Update: Ensure TuiState handles large offsets gracefully
```

## Phase 0: Outline & Research

1.  **Research ffmpeg Timestamping**: Confirmed that `-ss` before `-i` resets output timestamps to 0. Solution: `seek_offset`.
2.  **Research Connection Monitoring**: Use `MEDIA_STATUS` idle reasons (e.g., `ERROR`) and periodic heartbeat checks to detect failures.
3.  **Research Stream Termination**: Closing the `ChildStdout` in `StreamServer` should trigger an EOF on the socket, forcing the receiver to reconnect or report an error.

## Phase 1: Design & Contracts

1.  **Data Model**: Defined `AppState` extensions in `data-model.md`.
2.  **Internal API**: `load_media` already returns `applied_seek_offset`. We will refine its usage in `main.rs`.
3.  **Watchdog Logic**:
    - Track `last_update_time` and `last_current_time`.
    - If `status` becomes `IDLE` with reason `ERROR`, or if no updates are received for > 5s while expecting `PLAYING`, trigger resume.

## Phase 2: Implementation Strategy

- **Step 1**: Refine `seek_offset` calculation in `main.rs`.
- **Step 2**: Update `StreamServer` to ensure clean handovers between transcode sessions.
- **Step 3**: Implement the Resume Watchdog in the `main.rs` event loop.
- **Step 4**: Verify with edge cases (seeking to end, rapid seeks).