# Implementation Plan: Torrent Streaming Support

**Branch**: `011-add-torrent-support` | **Date**: 2026-01-15 | **Spec**: [specs/011-add-torrent-support/spec.md](spec.md)
**Input**: Feature specification from `specs/011-add-torrent-support/spec.md`

## Summary

Integrate `librqbit` to enable streaming of video content directly from Magnet URIs and `.torrent` files. The system will download pieces sequentially, buffering them to a temporary file which is served via HTTP to the Cast device.

## Technical Context

**Language/Version**: Rust 2021
**Primary Dependencies**: `librqbit` (for BitTorrent), `tokio` (for async runtime).
**Storage**: System temporary directory for download buffering.
**Testing**: Integration tests using public magnet links.
**Target Platform**: Cross-platform (Linux/macOS/Windows)
**Project Type**: Library + CLI
**Performance Goals**: Start playback < 30s. Seek resume < 20s.
**Constraints**: Must handle slow peers gracefully (buffer).
**Scale/Scope**: Single active torrent stream.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] **Dependency Minimalism**: `librqbit` is a significant dependency but is required for the feature (complexity of BitTorrent). It is preferred over `cratetorrent` for its streaming focus.
- [x] **Library-First Architecture**: Torrent logic encapsulated in `src/torrent/` module, accessible via library API.
- [x] **Test-First Development**: Will write integration tests for the torrent manager.
- [x] **Async I/O**: `librqbit` is built on `tokio`.
- [x] **Secure Transport**: N/A for P2P, but local serving is HTTP.

## Project Structure

### Documentation (this feature)

```text
specs/011-add-torrent-support/
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
├── lib.rs
├── main.rs              # Update CLI to accept magnet/torrent args
├── torrent/             # New module
│   ├── mod.rs           # Public API
│   ├── manager.rs       # Session management
│   └── stream.rs        # AsyncRead adapter (GrowingFile)
└── server.rs            # Update to accept Generic AsyncRead source
```

**Structure Decision**: Add `src/torrent` module. Update `StreamServer` to be generic over the input source (not just `File`, but `AsyncRead + AsyncSeek`).

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| New Dependency: `librqbit` | BitTorrent protocol is too complex to implement from scratch safely. | Custom impl is high risk/maintenance. |