# Tasks: Optimize Codebase

**Feature Branch**: `021-optimize-codebase`
**Status**: Pending

## Phase 1: Setup & Tools
*Goal: Prepare the environment for coverage analysis and benchmarking.*

- [x] T001 Install `cargo-llvm-cov` tool locally (Manual Step)
- [x] T002 Add `criterion` as a dev-dependency in `Cargo.toml`
- [x] T003 Create benchmarks directory structure at `benches/`

## Phase 2: Foundational
*Goal: Establish baseline metrics before making changes.*

- [x] T004 [P] Create baseline coverage report using `cargo llvm-cov` and save summary to `coverage_baseline.txt`
- [x] T005 [P] Create initial dummy benchmark in `benches/baseline.rs` to verify criterion setup

## Phase 3: User Story 1 - Developer Code Confidence (P1)
*Goal: Increase test coverage by >5% and cover critical paths.*
*Independent Test: Run `cargo llvm-cov` and verify increased percentage.*

### Unit Tests
- [x] T006 [P] [US1] Add unit tests for `Client` connection logic in `src/client.rs`
- [x] T007 [P] [US1] Add unit tests for `Server` message handling in `src/server.rs`
- [x] T008 [P] [US1] Add unit tests for `TorrentManager` state logic in `src/torrent/manager.rs`

### Integration Tests
- [x] T009 [US1] Create `tests/integration_cast.rs` to test the casting lifecycle (mocked device)
- [x] T010 [US1] Implement mock CastV2 device for integration testing in `tests/common/mock_device.rs`

## Phase 4: User Story 2 - Codebase Maintainability (P2)
*Goal: Refactor `src/main.rs` to reduce complexity and adhere to library-first architecture.*
*Independent Test: `src/main.rs` size < 20KB and tests pass.*

### Logic Extraction
- [x] T011 [US2] Define `Config` struct for CLI arguments in `src/config.rs` (create new file)
- [x] T012 [US2] Implement `CastNowCore` struct in `src/lib.rs` to encapsulate app state
- [x] T013 [US2] Move discovery logic from `src/main.rs` to `CastNowCore` in `src/lib.rs`
- [x] T014 [US2] Move media casting logic from `src/main.rs` to `CastNowCore` in `src/lib.rs`
- [x] T015 [US2] Move TUI initialization and loop from `src/main.rs` to `src/controllers/tui.rs` (or dedicated runner in lib)

### CLI Cleanup
- [x] T016 [US2] Refactor `src/main.rs` to only parse args and call `CastNowCore::run`
- [x] T017 [US2] Verify no regressions by running integration tests from Phase 3

## Phase 5: User Story 3 - Application Performance (P3)
*Goal: Reduce CPU usage during transcoding by >5%.*
*Independent Test: Run `cargo bench` and observe improvement.*

### Benchmarking
- [x] T018 [US3] Create transcoding benchmark in `benches/transcode_throughput.rs`
- [x] T019 [US3] Run benchmark to establish performance baseline (record in `research.md` or locally)

### Optimization
- [x] T020 [P] [US3] Optimize buffer sizes for ffmpeg pipes in `src/transcode.rs`
- [x] T021 [P] [US3] Implement async stream handling for transcoding to avoid blocking calls in `src/transcode.rs`
- [x] T022 [US3] Optimize media protocol loop in `src/protocol/media.rs` to reduce allocations

### Verification
- [x] T023 [US3] Run `cargo bench` again to verify performance improvement

## Phase 6: Polish
*Goal: Final cleanup and standard checks.*

- [x] T024 Run `cargo clippy` and fix any new warnings in `src/` and `tests/`
- [x] T025 Update `README.md` with new testing and benchmarking commands
- [x] T026 Generate final coverage report and verify SC-001 (Success Criteria)

## Dependencies

- Phase 2 depends on Phase 1
- Phase 3, 4, 5 are theoretically parallelizable, but:
  - Phase 4 (Refactoring) modifies code tested in Phase 3. **Recommendation**: Do Phase 3 (Tests) -> Phase 4 (Refactor) -> Phase 5 (Perf).
  - Writing tests first (Phase 3) ensures the refactoring (Phase 4) is safe.

## Parallel Execution Opportunities

- **Phase 3**: T006, T007, T008 can be done in parallel by different developers.
- **Phase 5**: T020 and T022 can be investigated in parallel.
