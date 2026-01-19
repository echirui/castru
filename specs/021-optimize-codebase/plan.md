# Implementation Plan: Optimize Codebase

**Branch**: `021-optimize-codebase` | **Date**: 2026-01-18 | **Spec**: [specs/021-optimize-codebase/spec.md](spec.md)
**Input**: Feature specification from `specs/021-optimize-codebase/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

This feature aims to improve the overall quality of the codebase through targeted test coverage increases, refactoring of complex modules, and performance tuning of the transcoding pipeline. The goal is to raise test coverage by 10% (or reach 70% overall), reduce cyclomatic complexity in the top 5 most complex files, and lower CPU usage during transcoding by at least 5%.

## Technical Context

**Language/Version**: Rust 2021 Edition
**Primary Dependencies**: `tokio` (async runtime), `prost` (protobuf), `rustls` (TLS). *Constraint*: No new dependencies allowed without strong justification.
**Storage**: N/A (Feature focuses on code quality)
**Testing**: `cargo test` for unit/integration tests, `cargo-llvm-cov` or `tarpaulin` for coverage (NEEDS CLARIFICATION: Which coverage tool is available/preferred?), `criterion` for benchmarking (NEEDS CLARIFICATION: Is `criterion` already in use or permissible?).
**Target Platform**: Cross-platform (Linux/macOS primary dev environment).
**Project Type**: Rust Library + CLI
**Performance Goals**: Reduce CPU usage during transcoding by >5%.
**Constraints**: Must adhere to strict dependency minimalism.
**Scale/Scope**: Refactoring top 5 complex files; increasing test coverage for core modules.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] **Dependency Minimalism**: No new dependencies proposed yet. *Warning*: Adding `criterion` or coverage tools needs to be scoped to `dev-dependencies`.
- [x] **Library-First Architecture**: Refactoring improves library internal structure.
- [x] **Test-First Development**: Feature explicitly mandates adding tests before refactoring.
- [x] **Async I/O**: Performance tuning targets `tokio` based pipelines.
- [x] **Secure Transport**: No changes to TLS logic proposed, only potential refactoring.

## Project Structure

### Documentation (this feature)

```text
specs/021-optimize-codebase/
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
├── client.rs            # Target for coverage/refactoring
├── server.rs            # Target for coverage/refactoring
├── torrent/             # Target for coverage/refactoring
└── transcode.rs         # Target for performance tuning

tests/                   # Integration tests location
```

**Structure Decision**: Standard Rust project layout. No structural changes, just internal improvements.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| `criterion` (dev-dep) | To scientifically measure CPU reduction (SC-003) | Ad-hoc `Instant::now()` logging is unreliable for micro-benchmarks. |