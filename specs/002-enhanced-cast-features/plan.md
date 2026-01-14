# Implementation Plan: Enhanced Cast Features

**Branch**: `002-enhanced-cast-features` | **Date**: 2026-01-13 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/002-enhanced-cast-features/spec.md`

## Summary

Implement mDNS discovery, extended media control (seek/volume/metadata), automatic reconnection logic, and polish the public API documentation to make `castru` a production-ready crate.

## Technical Context

**Language/Version**: Rust 1.75+
**Primary Dependencies**: `mdns-sd` (or `mdns`), `serde`, `tokio`.
**Storage**: N/A
**Testing**: Integration tests for discovery (if possible in CI) and media logic.
**Target Platform**: Cross-platform (wherever `tokio` and mDNS work).
**Project Type**: Library

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] **Dependency Minimalism**: `mdns-sd` is chosen carefully. If it's too heavy, we will re-evaluate.
- [x] **Library-First**: All features exposed via `lib.rs`.
- [x] **Async I/O**: Discovery and Reconnection use `tokio`.

## Project Structure

```text
src/
├── discovery.rs         # NEW: mDNS discovery logic
├── protocol/
│   ├── media.rs         # NEW: Media namespace messages
│   └── receiver.rs      # UPDATE: Volume control often here too
├── client.rs            # UPDATE: Reconnection logic, Media methods
├── lib.rs               # UPDATE: Export new modules
└── error.rs             # UPDATE: Discovery errors

examples/
├── discover.rs          # NEW: Discovery example
└── full_control.rs      # NEW: Media control example
```

## Phases

### Phase 1: Discovery (Milestone 1)
- [ ] **Task 1.1**: Add `mdns-sd` dependency.
- [ ] **Task 1.2**: Implement `DiscoveryService` to listen for `_googlecast._tcp.local`.
- [ ] **Task 1.3**: Parse TXT records for friendly name/model.

### Phase 2: Media Control (Milestone 2)
- [ ] **Task 2.1**: Define `Media` namespace messages (GET_STATUS, SEEK, LOAD).
- [ ] **Task 2.2**: Implement `CastClient::media_seek`, `media_get_status`.
- [ ] **Task 2.3**: Implement `CastClient::set_volume` (Receiver namespace).

### Phase 3: Reconnection (Milestone 3)
- [ ] **Task 3.1**: Refactor `CastClient` to handle connection drops.
- [ ] **Task 3.2**: Implement automatic retry loop in background task.

### Phase 4: Polish & Docs (Milestone 4)
- [ ] **Task 4.1**: Update `README.md`.
- [ ] **Task 4.2**: Add Rustdoc examples to public methods.
- [ ] **Task 4.3**: Prepare `Cargo.toml` metadata (description, license) for publishing.

## Reference Documents
- [Research Findings](./research.md)
- [Data Model](./data-model.md)