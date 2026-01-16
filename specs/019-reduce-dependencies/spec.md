# Feature Specification: Dependency Minimization and Refinement

**Feature Branch**: `019-reduce-dependencies`  
**Created**: 2026-01-15  
**Status**: Draft  
**Input**: User description: "外部crateを減らしたいです。"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Remove Utility Crates (Priority: P1)

As a maintainer, I want to reduce the number of small utility crates like `thiserror` and `uuid` by using standard library features or simple manual implementations, so that the project is more lightweight and has fewer supply chain risks.

**Why this priority**: Directly addresses the user's request to reduce external crates.

**Independent Test**: The project compiles and runs all existing functionality (cast, tui, torrent) without the removed crates in `Cargo.toml`.

**Acceptance Scenarios**:

1. **Given** the current codebase uses `thiserror` for error enums, **When** I replace it with manual `std::fmt::Display` and `std::error::Error` implementations, **Then** all error handling should continue to work identically.
2. **Given** the code uses `uuid` for temporary filename generation, **When** I replace it with a simple random string or timestamp-based approach, **Then** temporary files should still be created correctly without collisions.

---

### User Story 2 - Minimize Internal Dependencies (Priority: P2)

As a developer, I want to evaluate if utility crates like `bstr` or `bytes` can be replaced with standard `String` or `Vec<u8>` usage in non-critical paths, reducing the overall dependency count.

**Why this priority**: Further reduces external reliance and simplifies the build process.

**Independent Test**: `cargo test` passes and `cast` functionality remains intact after refactoring.

**Acceptance Scenarios**:

1. **Given** usage of `bstr` for byte-to-string conversions, **When** I refactor to use standard Rust string methods, **Then** functionality should remain identical.

---

### Edge Cases

- **Error chain breakage**: Ensure that removing `thiserror` doesn't break the `source()` link in error reporting.
- **Filename collisions**: Replacing `uuid` with a simpler generator must not cause collisions in high-concurrency or rapid-restart scenarios.
- **Performance impact**: Replacing `bytes` or specialized crates must not significantly degrade performance in the streaming pipeline.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST remove `thiserror` and replace it with standard library error traits.
- **FR-002**: System MUST remove `uuid` and implement a minimalist unique identifier generator (e.g. via `std::time`).
- **FR-003**: System MUST remove `bstr` and use standard library string/byte utilities.
- **FR-004**: System MUST maintain 100% feature parity with all existing CLI commands and TUI features.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Total dependency count (excluding dev-dependencies) reduced by at least 3 crates.
- **SC-002**: Binary size (release build) reduced or maintained.
- **SC-003**: 100% pass rate on existing regression tests.
- **SC-004**: No regressions in torrent metadata resolution or streaming stability.