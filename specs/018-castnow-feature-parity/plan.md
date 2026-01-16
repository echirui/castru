# Implementation Plan: castnow Feature Parity and Torrent Refinement

**Branch**: `018-castnow-feature-parity` | **Date**: 2026-01-15 | **Spec**: [specs/018-castnow-feature-parity/spec.md](spec.md)
**Input**: Feature specification from `/specs/018-castnow-feature-parity/spec.md`

## Summary

This feature achieves high functional parity with the `castnow` reference tool. We will implement essential CLI flags (`--myip`, `--port`, `--subtitles`, `--volume`, `--loop`, `--quiet`) and refine the torrent streaming engine to ensure sequential piece prioritization and improved stability.

## Technical Context

**Language/Version**: Rust 2021  
**Primary Dependencies**: `tokio`, `librqbit`, `crossterm`, `rustls`  
**Storage**: N/A  
**Testing**: `cargo test`, Manual CLI validation with various flags.  
**Target Platform**: Any system with `ffmpeg` and network access.
**Project Type**: Single project  
**Performance Goals**: <5s torrent buffering, <100ms CLI response.  
**Constraints**: Dependency minimalism (manual parsing).  
**Scale/Scope**: Impacts `main.rs`, `server.rs`, and `torrent/manager.rs`.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] **Dependency Minimalism**: Extended manual parsing instead of adding `clap`.
- [x] **Library-First**: Subtitle and network logic implemented in library modules.
- [x] **Async I/O**: Server and torrent engine remain asynchronous.
- [x] **Secure Transport**: Unchanged.

## Project Structure

### Documentation (this feature)

```text
specs/018-castnow-feature-parity/
├── plan.md              # This file
├── research.md          # castnow flag analysis
├── data-model.md        # CLI Option structures
├── quickstart.md        # Testing instructions
└── checklists/
    └── requirements.md  
```

### Source Code (repository root)

```text
src/
├── main.rs              # Update: CLI parsing, loop logic, volume init
├── server.rs            # Update: Support custom IP/Port and sidecar files
└── torrent/
    └── manager.rs       # Update: Sequential download settings
```

## Phase 0: Outline & Research

1.  **Flag mapping**: Completed in `research.md`.
2.  **Subtitle Serving**: Chromecast requires VTT via URL.
3.  **librqbit refinement**: sequential piece requests.

## Phase 1: Design & Contracts

1.  **Data Model**: Defined `CastOptions` expansion.
2.  **Server API**: `StreamServer::start` will accept an optional port.
3.  **Media Protocol**: `MediaInformation` will be updated to support the `tracks` field.

## Phase 2: Implementation Strategy

- **Step 1**: Expand `CastOptions` and manual parsing in `main.rs`.
- **Step 2**: Update `StreamServer` to bind to specific IP/Port.
- **Step 3**: Implement subtitle serving and protocol integration.
- **Step 4**: Configure `librqbit` for optimized streaming.
- **Step 5**: Implement `--loop` and `--quiet` logic in the main loop.