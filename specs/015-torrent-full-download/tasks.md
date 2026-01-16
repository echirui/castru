# Tasks: Torrent Full Download and Playback

**Input**: Design documents from `/specs/015-torrent-full-download/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, quickstart.md

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [ ] T001 [P] Verify `librqbit` stats API availability for progress tracking in `src/torrent/manager.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T002 Add `torrent_progress: Option<f32>` and `torrent_file_name: Option<String>` to `AppState` in `src/main.rs`
- [ ] T003 Add `torrent_progress: Option<f32>` to `TuiState` struct in `src/controllers/tui.rs`

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Monitor Download Progress (Priority: P1) 🎯 MVP

**Goal**: Display download percentage in the TUI during the torrent fetching phase.

**Independent Test**: Start casting a magnet link; verify the TUI shows "DOWNLOADING: 0.0%" and updates as bytes arrive.

### Implementation for User Story 1

- [ ] T004 [P] [US1] Update `TuiController::draw` in `src/controllers/tui.rs` to render a download status line when `torrent_progress` is `Some`.
- [ ] T005 [US1] Implement percentage calculation logic in the buffering loop within `src/main.rs`.

**Checkpoint**: User Story 1 functional - progress is visible.

---

## Phase 4: User Story 2 - Automated Playback After Completion (Priority: P1)

**Goal**: Block media loading until the torrent is 100% downloaded locally.

**Independent Test**: Start casting a torrent; verify the Chromecast stays on the home/idle screen until the local TUI shows 100%, then playback begins automatically.

### Implementation for User Story 2

- [ ] T006 [US2] Modify the buffering loop in `load_media` (within `src/main.rs`) to check for 100% completion instead of 3%.
- [ ] T007 [US2] Ensure `AppState` updates and `tui.draw()` calls occur inside the `load_media` download loop in `src/main.rs`.
- [ ] T008 [US2] Guard the `app.load()` call in `src/main.rs` to execute only after the download loop terminates successfully.

**Checkpoint**: User Story 2 functional - playback only starts after full download.

---

## Phase 5: User Story 3 - Error Handling during Download (Priority: P2)

**Goal**: Detect stalled downloads and inform the user.

**Independent Test**: Disconnect internet during download; verify the TUI eventually shows an error or "Stalled" status.

### Implementation for User Story 3

- [ ] T009 [US3] Add a timeout/stall detection mechanism in the `load_media` download loop in `src/main.rs`.

---

## Phase N: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T010 [P] Refactor `load_media` loop for better readability and performance in `src/main.rs`.
- [ ] T011 [P] Validate all scenarios defined in `specs/015-torrent-full-download/quickstart.md`.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies.
- **Foundational (Phase 2)**: Depends on Phase 1. BLOCKS US1, US2.
- **User Story 1 (US1)**: Depends on Phase 2.
- **User Story 2 (US2)**: Depends on US1 (for progress info) and Phase 2.
- **User Story 3 (US3)**: Depends on US2 loop implementation.
- **Polish**: Depends on all stories.

---

## Parallel Example: User Story 1 & 2

```bash
# T004 can be implemented in parallel with T005/T006
Task: "[P] [US1] Update TuiController::draw in src/controllers/tui.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 & 2)

1. Complete Setup and Foundation.
2. Implement TUI progress rendering (US1).
3. Implement the 100% wait logic in the load loop (US2).
4. **VALIDATE**: Ensure smooth transition from 100% download to playback.

### Incremental Delivery

1. Foundation ready.
2. US1 adds visible feedback.
3. US2 enables full-download playback.
4. US3 adds robustness.
