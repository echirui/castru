# Implementation Plan: Reconnect Action

**Branch**: `016-reconnect-action` | **Date**: 2026-01-15 | **Spec**: [specs/016-reconnect-action/spec.md](spec.md)
**Input**: Feature specification from `/specs/016-reconnect-action/spec.md`

## Summary

This feature adds a manual reconnection trigger to `castru`. Users can press 'r' in the TUI to force a connection reset and re-initialization. This is achieved by dropping the existing `CastClient`, creating a new one, and re-running the connection handshake.

## Technical Context

**Language/Version**: Rust 2021  
**Primary Dependencies**: `tokio`, `rustls`, `crossterm`  
**Storage**: N/A  
**Testing**: `cargo test`, Manual verification with Chromecast device.  
**Target Platform**: Linux/macOS/Windows  
**Project Type**: Single project  
**Performance Goals**: Reconnect within 2 seconds.  
**Constraints**: Must preserve `AppState` (playlist position, current time) in `main.rs`.  
**Scale/Scope**: Impacts `src/controllers/tui.rs` and `src/main.rs`.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] **Dependency Minimalism**: No new dependencies added.
- [x] **Library-First**: Connection logic remains in `CastClient`.
- [x] **Async I/O**: Uses `tokio` for new connection.
- [x] **Secure Transport**: Uses existing TLS setup.

## Project Structure

### Documentation (this feature)

```text
specs/016-reconnect-action/
├── plan.md              # This file
├── research.md          # Connection reset strategies
├── data-model.md        # TuiCommand updates
├── quickstart.md        # Testing instructions
└── checklists/
    └── requirements.md  
```

### Source Code (repository root)

```text
src/
├── main.rs              # Update: Handle TuiCommand::Reconnect in event loop
└── controllers/
    └── tui.rs           # Update: Add TuiCommand::Reconnect variant and key binding
```

## Phase 0: Outline & Research

1.  **Research Reconnection**: Confirmed that re-creating `CastClient` is the most robust way to clear stale state.
2.  **TUI Feedback**: Verified where to inject "RECONNECTING" status in the TUI loop.

## Phase 1: Design & Contracts

1.  **TuiCommand**: Add `Reconnect` variant.
2.  **TUI Binding**: Map `KeyCode::Char('r')` to `TuiCommand::Reconnect`.
3.  **Main Loop Logic**:
    - Match `TuiCommand::Reconnect`.
    - Set status to "RECONNECTING".
    - Call `CastClient::connect`.
    - Update `events` receiver using `new_client.events()`.
    - Run `new_client.connect_receiver()`.

## Phase 2: Implementation Strategy

- **Step 1**: Update `TuiCommand` and binding in `tui.rs`.
- **Step 2**: Implement reconnect block in `main.rs` loop.
- **Step 3**: Verify manual trigger resets connection and restores status updates.
