# Implementation Plan: Castnow Feature Integration

**Branch**: `004-castnow-features` | **Date**: 2026-01-14 | **Spec**: [specs/004-castnow-features/spec.md](spec.md)
**Input**: Feature specification from `/specs/004-castnow-features/spec.md`

## Summary

Integrate core "castnow" capabilities into `castru`, including local media streaming via a minimal internal HTTP server, mDNS device discovery, an interactive CLI (TUI) for playback control, and playlist management. The implementation will prioritize the project's "Dependency Minimalism" principle by leveraging `tokio` and avoiding heavy external crates where possible.

## Technical Context

**Language/Version**: Rust 2021  
**Primary Dependencies**: `tokio`, `prost`, `rustls`, `mdns-sd`, `crossterm`.  
**Storage**: N/A (Memory-based playlist/queue)  
**Testing**: `cargo test` (unit/integration)  
**Target Platform**: CLI (Linux, macOS, Windows)
**Project Type**: Library + CLI  
**Performance Goals**: <200ms control latency, stable 4GB+ file streaming.  
**Constraints**: Pure Rust (no `openssl`), no blocking I/O, minimal dependency footprint.  
**Scale/Scope**: Local network media orchestration.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Dependency Minimalism**: PASSED. `mdns-sd` is already a dependency. Adding `crossterm` for TUI is justified for cross-platform reliability. No other new crates required.
- **Library-First**: PASSED. Discovery, Streaming, and Playlist logic are in `src/` and exposed via `lib.rs`.
- **Async I/O**: PASSED. `tokio` handles all I/O (TCP, FS, Stdin).
- **Secure Transport**: PASSED. Protocol continues to use `rustls`.

## Project Structure

### Documentation (this feature)

```text
specs/004-castnow-features/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
└── tasks.md             # Phase 2 output (generated later)
```

### Source Code (repository root)

```text
src/
├── discovery.rs         # Add mDNS discovery logic
├── server.rs            # New: Minimal HTTP server for local files
├── controllers/
│   ├── media.rs         # Update: Enhanced media controls and playlist management
│   └── tui.rs           # New: Keyboard input and status display
├── lib.rs               # Export new modules
└── main.rs              # CLI entry point to orchestrate features
```

**Structure Decision**: Single project with modular controllers. New `server.rs` and `tui.rs` to handle specific "castnow" responsibilities while keeping `lib.rs` as the primary interface.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| New module: `server.rs` | Local file streaming requires hosting content. | External server requirement complicates user experience. |
| New module: `tui.rs` | Interactive control (space to pause) requires terminal raw mode. | Basic CLI arguments cannot handle real-time interactions. |
