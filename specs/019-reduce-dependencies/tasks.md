# Tasks: Dependency Minimization and Refinement

**Input**: Design documents from `/specs/019-reduce-dependencies/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, quickstart.md

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [ ] T001 [P] Verify existing project build and tests with `cargo test`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T002 [P] Implement `generate_temp_id` as a private helper in `src/server.rs` using `std::time::SystemTime`.

---

## Phase 3: User Story 1 - Remove Utility Crates (Priority: P1) 🎯 MVP

**Goal**: Remove `thiserror` and `uuid` dependencies by implementing standard alternatives.

**Independent Test**: The project compiles and passes tests without `thiserror` and `uuid` listed in `Cargo.toml`.

### Implementation for User Story 1

- [ ] T003 [P] [US1] Implement manual `Display` and `Error` traits for `CastError` in `src/error.rs`, removing `thiserror` dependency.
- [ ] T004 [P] [US1] Implement manual `Display` and `Error` traits for `TorrentError` in `src/torrent/mod.rs`.
- [ ] T005 [P] [US1] Update `TorrentSession` struct in `src/torrent/mod.rs` to change `session_id` type from `uuid::Uuid` to `String`.
- [ ] T006 [P] [US1] Replace `uuid::Uuid::new_v4()` with manual timestamp-based ID in `src/server.rs`.
- [ ] T007 [P] [US1] Remove `thiserror` and `uuid` from `[dependencies]` in `Cargo.toml`.

**Checkpoint**: User Story 1 complete - core utilities reduced.

---

## Phase 4: User Story 2 - Minimize Internal Dependencies (Priority: P2)

**Goal**: Remove `bstr` dependency by using standard string/byte utilities.

**Independent Test**: `cargo test` passes after removing `bstr` from `Cargo.toml`.

### Implementation for User Story 2

- [ ] T008 [P] [US2] Refactor `src/torrent/manager.rs` to replace `bstr::ByteSlice` with standard library methods (e.g., `from_utf8_lossy`).
- [ ] T009 [P] [US2] Remove `bstr` from `[dependencies]` in `Cargo.toml`.

**Checkpoint**: User Story 2 complete - internal dependencies minimized.

---

## Phase N: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T010 [P] Run `cargo tree` to verify the dependency graph is reduced.
- [ ] T011 [P] Run all regression tests and manual verification per `quickstart.md`.
- [ ] T011.5 [P] Compare release binary size (`cargo build --release`) before and after changes to satisfy SC-002.
- [ ] T012 Run `cargo fmt` and `cargo clippy` to ensure code quality after refactoring.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies.
- **Foundational (Phase 2)**: Depends on Setup.
- **User Stories (Phase 3 & 4)**: Can proceed in parallel after Phase 2 if team capacity allows, but US1 is prioritized.
- **Polish**: Depends on all stories.

### Parallel Opportunities

- T003, T004, T005, T006 can run in parallel as they affect different files.
- T008 can run in parallel with US1 tasks.

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Setup and Foundation.
2. Complete US1 refactoring (Error and ID replacements).
3. **STOP and VALIDATE**: Verify build without `thiserror` and `uuid`.

### Incremental Delivery

1. Foundation ready.
2. Add US1 (thiserror/uuid removal) → Test independently.
3. Add US2 (bstr removal) → Test independently.
4. Final cleanup and verification.
