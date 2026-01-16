# Implementation Plan: Dependency Minimization and Refinement

**Branch**: `019-reduce-dependencies` | **Date**: 2026-01-15 | **Spec**: [specs/019-reduce-dependencies/spec.md](spec.md)
**Input**: Feature specification from `/specs/019-reduce-dependencies/spec.md`

## Summary

This feature focuses on reducing the project's external dependency footprint by replacing small utility crates (`thiserror`, `uuid`, `bstr`) with standard library implementations or simple internal logic. The goal is to maintain 100% functional parity while simplifying the dependency graph.

## Technical Context

**Language/Version**: Rust 2021  
**Primary Dependencies**: `tokio`, `prost`, `rustls`, `serde`, `crossterm`, `librqbit` (Remaining after reduction).  
**Storage**: N/A  
**Testing**: `cargo test`, manual verification of transcode and torrent flows.  
**Target Platform**: Universal (Linux/macOS/Windows).
**Project Type**: Single project.  
**Performance Goals**: Maintain or slightly improve binary size and compile times.  
**Constraints**: Pure Rust stack, Async I/O.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] **Dependency Minimalism**: Explicitly removing crates.
- [x] **Library-First Architecture**: Refactoring library modules to use standard traits.
- [x] **Async I/O**: Unaffected by these changes.
- [x] **Secure Transport**: Unaffected by these changes.

## Project Structure

### Documentation (this feature)

```text
specs/019-reduce-dependencies/
├── plan.md              # This file
├── research.md          # Analysis of target crates and replacements
├── data-model.md        # Manual Error and ID generation patterns
└── quickstart.md        # Verification steps
```

### Source Code (repository root)

```text
src/
├── error.rs             # Update: Replace thiserror with manual impls
├── server.rs            # Update: Replace uuid usage for transcode temp files
├── torrent/
│   ├── mod.rs           # Update: Replace thiserror and uuid
│   └── manager.rs       # Update: Replace bstr
└── main.rs              # No functional changes, verification only
```

## Phase 0: Outline & Research

1.  **Analyze current usage**: Grepped all instances of target crates.
2.  **Determine replacements**: Standard traits for Errors, `SystemTime` for IDs, `from_utf8_lossy` for `bstr`.
3.  **Validate `bytes`**: Decided to KEEP `bytes` due to tight integration with core libraries.

## Phase 1: Design & Contracts

1.  **Error Refactoring**: Use `std::fmt::Display` and `std::error::Error` manually.
2.  **ID Generation**: Use a simple hex-encoded nanosecond timestamp for temp files.
3.  **Agent Context**: Update context to reflect reduced dependency list.

## Phase 2: Implementation Strategy

- **Step 1**: Implement manual Errors in `src/error.rs` and `src/torrent/mod.rs`.
- **Step 2**: Replace `uuid` in `src/server.rs` and `src/torrent/mod.rs`.
- **Step 3**: Replace `bstr` in `src/torrent/manager.rs`.
- **Step 4**: Update `Cargo.toml` and verify.