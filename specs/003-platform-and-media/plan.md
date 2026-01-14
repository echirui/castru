# Implementation Plan: Platform Controller Expansion & Media Controller

**Branch**: `003-platform-and-media` | **Date**: 2026-01-13 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/003-platform-and-media/spec.md`

## Summary

Expand the library with robust session management (Platform Controller) and full media playback controls (Media Controller). Refactor the client to support these distinct responsibilities and improve connection resilience with exponential backoff.

## Technical Context

**Language/Version**: Rust 1.75+
**Primary Dependencies**: `serde`, `tokio`.
**Storage**: N/A
**Testing**: Manual examples + unit tests for parsing.
**Target Platform**: Cross-platform.
**Project Type**: Library

## Constitution Check

- [x] **Dependency Minimalism**: No new dependencies.
- [x] **Library-First**: Refactoring into Controllers improves library usability.
- [x] **Async I/O**: All new commands are async.

## Project Structure

```text
src/
├── controllers/         # NEW: High-level controllers
│   ├── mod.rs
│   ├── receiver.rs      # ReceiverController
│   └── media.rs         # MediaController
├── protocol/
│   ├── receiver.rs      # UPDATE: Add STOP, Application struct
│   └── media.rs         # UPDATE: Add PLAY, PAUSE, LOAD
├── client.rs            # UPDATE: Use controllers, Backoff
└── lib.rs               # UPDATE: Export controllers
```

## Phases

### Phase 1: Platform Controller (Milestone 1)
- [ ] **Task 1.1**: Update `ReceiverRequest` with `STOP` message.
- [ ] **Task 1.2**: Define `Application` struct for status parsing.
- [ ] **Task 1.3**: Implement `ReceiverController` with `get_status`, `launch_app`, `stop_app`, `join_session`.

### Phase 2: Media Controller (Milestone 2)
- [ ] **Task 2.1**: Update `MediaRequest` with `PLAY`, `PAUSE`, `STOP`, `LOAD`.
- [ ] **Task 2.2**: Implement `MediaController` with `load_media`, `play`, `pause`, `seek`, `stop`.
- [ ] **Task 2.3**: Implement `DefaultMediaReceiver` wrapper using the controllers.

### Phase 3: Resilience (Milestone 3)
- [ ] **Task 3.1**: Implement exponential backoff in `CastClient::connect`.

### Phase 4: Polish (Milestone 4)
- [ ] **Task 4.1**: Refactor `CastClient` to expose `receiver()` and `media(transport_id)`.
- [ ] **Task 4.2**: Verify `examples/` still work and add `full_lifecycle.rs`.

## Reference Documents
- [Research Findings](./research.md)
- [Data Model](./data-model.md)