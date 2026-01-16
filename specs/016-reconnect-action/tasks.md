# Tasks: Reconnect Action

**Input**: Design documents from `/specs/016-reconnect-action/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, quickstart.md

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [ ] T001 [P] Verify existing project build with `cargo build`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

- [ ] T002 Update `PlaybackStatus` enum in `src/controllers/media.rs` to include `Reconnecting` variant
- [ ] T003 Update `TuiCommand` enum in `src/controllers/tui.rs` to include `Reconnect` variant

---

## Phase 3: User Story 1 - Manual Reconnect (Priority: P1) 🎯 MVP

**Goal**: Implement the manual trigger and reconnection logic.

**Independent Test**: Press 'r' in the TUI and verify that the application attempts to reconnect to the device.

### Implementation for User Story 1

- [ ] T004 [P] [US1] Map `KeyCode::Char('r')` to `TuiCommand::Reconnect` in `src/controllers/tui.rs`
- [ ] T005 [US1] Implement `TuiCommand::Reconnect` handling in `src/main.rs` to recreate `CastClient`
- [ ] T006 [US1] Update `events` receiver in `src/main.rs` after client recreation
- [ ] T007 [US1] Trigger `connect_receiver` and session restoration in the reconnect handler in `src/main.rs`

**Checkpoint**: Manual reconnection triggered by 'r' key should now work.

---

## Phase 4: User Story 2 - Reconnect Status Feedback (Priority: P2)

**Goal**: Provide visual feedback during the reconnection process.

**Independent Test**: Press 'r' and verify that the TUI status shows `RECONNECTING`.

### Implementation for User Story 2

- [ ] T008 [US2] Update `current_status` to `PlaybackStatus::Reconnecting` when starting the reconnect process in `src/main.rs`
- [ ] T009 [US2] Update `TuiController::draw` in `src/controllers/tui.rs` to handle and display the `RECONNECTING` status with appropriate color

---

## Phase 5: Polish & Cross-Cutting Concerns

**Purpose**: Final touches and verification.

- [ ] T010 [P] Validate all scenarios in `specs/016-reconnect-action/quickstart.md`
- [ ] T011 [P] Run `cargo fmt` and `cargo clippy` to ensure code quality

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies.
- **Foundational (Phase 2)**: Depends on Phase 1.
- **User Story 1 (Phase 3)**: Depends on Phase 2.
- **User Story 2 (Phase 4)**: Depends on Phase 3.
- **Polish (Phase 5)**: Depends on all user stories.

---

## Parallel Opportunities

- T004 can be done in parallel with T005 if handled carefully.
- T010 and T011 can run in parallel.

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Foundational changes (T002, T003).
2. Implement TUI trigger (T004).
3. Implement core reconnection logic in `main.rs` (T005, T006, T007).

### Incremental Delivery

1. Foundation ready.
2. US1 adds the "Reconnect" capability.
3. US2 adds the necessary UI feedback for the new capability.
