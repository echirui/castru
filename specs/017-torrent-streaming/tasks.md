# Tasks: Torrent Streaming while Downloading

**Input**: Design documents from `/specs/017-torrent-streaming/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, quickstart.md

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [ ] T001 [P] Import `librqbit::ManagedTorrent` and other types in `src/main.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T002 Add `torrent_handle: Option<Arc<librqbit::ManagedTorrent>>` to `AppState` struct in `src/main.rs`
- [ ] T003 Define hybrid threshold constants (`TORRENT_BUFFER_PCT_THRESHOLD`, `TORRENT_BUFFER_SIZE_THRESHOLD`) in `src/main.rs`

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Stream Torrent during Download (Priority: P1) 🎯 MVP

**Goal**: Trigger media casting as soon as the buffering threshold is reached.

**Independent Test**: Start a large magnet link; verify `app.load()` is called within seconds (well before 100%).

### Implementation for User Story 1

- [ ] T004 Update `wait_for_torrent_download` in `src/main.rs` to return `Result<Arc<ManagedTorrent>, ...>` and implement early exit logic based on thresholds.
- [ ] T005 Update `load_media` in `src/main.rs` to capture the `ManagedTorrent` handle and store it in `AppState`.
- [ ] T006 Ensure `app.load()` in `src/main.rs` executes immediately after the early exit from the download loop.

**Checkpoint**: User Story 1 functional - early streaming enabled.

---

## Phase 4: User Story 2 - Monitor Progress during Streaming (Priority: P1)

**Goal**: Update and display background download progress while media is playing.

**Independent Test**: While playing a torrent, verify the status line shows `(DL: XX.X%)` and updates correctly.

### Implementation for User Story 2

- [ ] T007 Update the animation tick loop in `src/main.rs` to poll `app_state.torrent_handle` for stats and update `app_state.torrent_progress`.
- [ ] T008 Update `TuiController::draw` in `src/controllers/tui.rs` to show the download progress percentage in the status line if `torrent_progress < 100.0`.

**Checkpoint**: User Story 2 functional - background progress visible.

---

## Phase 5: User Story 3 - Buffer Underrun Handling (Priority: P2)

**Goal**: Prevent playback corruption when catching up to the download head.

**Independent Test**: Verify that the `GrowingFile` blocks reading when requested bytes are unavailable.

### Implementation for User Story 3

- [ ] T009 [P] Verify and refine `GrowingFile` implementation in `src/torrent/stream.rs` to ensure robust blocking behavior (already exists but should be checked for this flow).

---

## Phase N: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T010 [P] Implement `Drop` or cleanup logic to clear `torrent_handle` when media is stopped in `src/main.rs`.
- [ ] T011 [P] Run `specs/017-torrent-streaming/quickstart.md` validation.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies.
- **Foundational (Phase 2)**: Depends on Phase 1. BLOCKS US1, US2.
- **User Story 1 (US1)**: Depends on Phase 2.
- **User Story 2 (US2)**: Depends on US1 (handle availability) and Phase 2.
- **Polish**: Depends on all stories.

---

## Parallel Example: User Story 1 & 2

```bash
# T008 can be worked on in parallel with T004/T005
Task: "[P] [US2] Update TuiController::draw in src/controllers/tui.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1)

1. Complete Setup and Foundation.
2. Refactor `wait_for_torrent_download` for early exit (US1).
3. **VALIDATE**: Ensure streaming starts quickly.

### Incremental Delivery

1. Foundation ready.
2. US1 enables early playback.
3. US2 provides visibility into background download.
4. US3 ensures stability.
