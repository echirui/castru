# Implementation Plan: Remote Control and Transcoding Pipeline

**Branch**: `005-remote-transcoding` | **Date**: 2026-01-14 | **Spec**: [specs/005-remote-transcoding/spec.md](spec.md)
**Input**: Feature specification from `/specs/005-remote-transcoding/spec.md`

## Summary

This feature adds interactive playback control and an on-demand transcoding pipeline to `castru`. We will implement an asynchronous keyboard listener using `tokio::io::stdin` and a media controller to send `CastMessage` commands. For media compatibility, we'll integrate `ffprobe` to detect unsupported codecs and `ffmpeg` to transcode streams into H.264/AAC MP4 in real-time, served via the internal HTTP server.

## Technical Context

**Language/Version**: Rust 2021
**Primary Dependencies**: `tokio` (full), `prost`, `rustls`, `ffmpeg` (external tool), `ffprobe` (external tool). [NEEDS CLARIFICATION: Choice of minimal crate for raw terminal mode (e.g., `crossterm`) vs. manual `termios` for reading single keypresses].
**Storage**: N/A (Memory-based pipeline)
**Testing**: `cargo test` with mocked `CastClient` and `ffmpeg` outputs.
**Target Platform**: Desktop (macOS/Linux/Windows) with `ffmpeg` in PATH.
**Project Type**: single (Library + CLI)
**Performance Goals**: <250ms control latency, <2s transcoding start, <3s seek resume.
**Constraints**: Non-blocking I/O mandatory. Must not block the Tokio reactor with `ffmpeg` process management.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Dependency Minimalism**: [NEEDS CLARIFICATION: justify `crossterm` or similar for terminal control]. Project prefers standard library or existing stack.
- **Library-First**: Transcoding logic and controllers will be implemented as library modules (`src/controllers/`, `src/transcode.rs`).
- **Async I/O**: `tokio::process` will be used for all external tool invocations. `tokio::io::stdin` for keyboard.
- **Secure Transport**: Commands are sent over the existing TLS-encrypted Cast channel.

## Project Structure

### Documentation (this feature)

```text
specs/005-remote-transcoding/
├── plan.md              # This file
├── research.md          # Terminal control, FFmpeg flags, Pseudo-seek
├── data-model.md        # MediaProbe, TranscodeConfig, PlaybackCommand
├── quickstart.md        # CLI usage and keyboard shortcuts
├── contracts/
│   └── control-api.md   # HTTP REST API for remote control
└── checklists/
    └── requirements.md  # Spec quality checklist
```

### Source Code (repository root)

```text
src/
├── controllers/
│   ├── media.rs         # Update to handle remote commands
│   ├── receiver.rs      # Update to handle transport control
│   └── tui.rs           # NEW: Keyboard listener (crossterm)
├── server.rs            # Update to integrate transcoding pipeline
└── transcode.rs         # NEW: FFmpeg/FFprobe integration logic
```

**Structure Decision**: Single project structure. Added `tui.rs` for terminal interaction and `transcode.rs` for the media pipeline.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
