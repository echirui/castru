# Implementation Plan: Refined Torrent Streaming Strategy

**Branch**: `020-refine-torrent-streaming` | **Date**: 2026-01-15 | **Spec**: [specs/020-refine-torrent-streaming/spec.md](spec.md)
**Input**: Feature specification from `specs/020-refine-torrent-streaming/spec.md`.

## Summary

This feature implements a high-performance torrent streaming strategy inspired by `peerflix`. It uses sequential piece requests, header/footer prioritization, and dynamic priority sliding windows to minimize start-up time and maximize seek responsiveness.

## Technical Context

**Language/Version**: Rust 2021  
**Primary Dependencies**: `librqbit`, `tokio`  
**Storage**: N/A (Temporary buffering on disk)  
**Testing**: `cargo test`, Manual verification with public magnets.  
**Target Platform**: Linux/macOS/Windows.
**Project Type**: Single project.  
**Performance Goals**: Start < 15s, Seek < 8s.  
**Constraints**: Avoid complex manual piece bookkeeping; leverage `librqbit`'s internal engine.  
**Scale/Scope**: Core torrent module refinement.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] **Dependency Minimalism**: No new crates added.
- [x] **Library-First Architecture**: Refinements are internal to the `torrent` module.
- [x] **Async I/O**: Leverages `tokio` and non-blocking `librqbit` APIs.
- [x] **Secure Transport**: N/A for BitTorrent.

## Project Structure

### Documentation (this feature)

```text
specs/020-refine-torrent-streaming/
├── plan.md              # This file
├── research.md          # Strategy analysis
├── data-model.md        # Priority tiers
└── quickstart.md        # Verification steps
```

### Source Code (repository root)

```text
src/
└── torrent/
    ├── manager.rs       # Update: Enable sequential mode and Tier 0 priority
    └── stream.rs        # Update: Implement sliding window logic in GrowingFile
```

## Phase 0: Outline & Research

1.  **Peerflix Analysis**: Researched sliding window and header/footer strategies.
2.  **librqbit API**: Verified `set_sequential` and piece prioritization hooks.

## Phase 1: Design & Contracts

1.  **Priority Tiers**: Defined 4 tiers of urgency for pieces.
2.  **Sliding Window**: Designed the logic for `GrowingFile` to trigger priority updates based on byte offsets.

## Phase 2: Implementation Strategy

- **Step 1**: Update `TorrentManager` to trigger `set_sequential(true)` and prioritize first/last pieces.
- **Step 2**: Refactor `GrowingFile` to track the "sliding window" head.
- **Step 3**: Connect `GrowingFile` reads to `librqbit` piece priorities.
- **Step 4**: Verify with the "Big Buck Bunny" stress test.