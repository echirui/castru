# Research: Optimize Codebase

**Feature**: `021-optimize-codebase`
**Date**: 2026-01-18

## 1. Complexity Analysis & Refactoring Targets

**Decision**: Target the following 5 files for refactoring, prioritized by size and structural importance:
1.  `src/main.rs` (49KB) - **CRITICAL**. Contains excessive logic. needs to move logic to `src/lib.rs` or `src/controllers/` to adhere to "Library-First" principle.
2.  `src/server.rs` (19KB) - Large module, candidate for splitting.
3.  `src/controllers/tui.rs` (16KB) - UI logic can likely be decoupled further.
4.  `src/client.rs` (11KB) - Core logic, needs robust testing.
5.  `src/torrent/manager.rs` (6KB) - Torrent logic is complex by nature, needs encapsulation verification.

**Rationale**: File size is a strong proxy for complexity in this codebase. `src/main.rs` is an outlier, suggesting CLI logic is mixed with core application logic.

## 2. Test Coverage Strategy

**Decision**: Use `cargo-llvm-cov` for generating coverage reports.
**Rationale**: It provides accurate, source-based coverage for Rust and generates standard lcov reports. It is easier to set up than `tarpaulin` for local development in many cases and supports modern Rust features well.

**Unknown Resolution**: `cargo-llvm-cov` is preferred over `tarpaulin` for this specific iteration due to its accuracy with `async` code (essential for `tokio`).

## 3. Performance Benchmarking

**Decision**: Add `criterion` as a `dev-dependency`.
**Rationale**: `std::time::Instant` is insufficient for statistical significance when measuring active transcoding loops or small hot paths. `criterion` handles warmup and statistical analysis automatically.

**Alternatives Considered**:
- *Manual logging*: Rejected due to noise and lack of repeatability.
- *OS Profilers (perf/Instruments)*: Good for deep dives, but `criterion` is better for regression testing (CI-friendly).

## 4. Performance Tuning Targets

**Decision**: Focus on `src/transcode.rs` and `src/protocol/media.rs`.
**Rationale**: Although `src/transcode.rs` is not the largest file, it handles the `ffmpeg` process spawning and piping. This is the CPU bottleneck. Optimizing buffer sizes and async stream handling here will yield the required 5% improvement.

## 5. Dependency Impact

**Decision**: 
- Add `criterion` (dev-only).
- Use `cargo-llvm-cov` (tooling, not dependency).

**Constitution Compliance**: `dev-dependencies` do not violate the minimal dependency principle for the production build artifact.
