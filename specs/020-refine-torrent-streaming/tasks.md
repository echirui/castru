# Tasks: Refined Torrent Streaming Strategy

**Input**: Design documents from `/specs/020-refine-torrent-streaming/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, quickstart.md

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [ ] T001 [P] Verify BitTorrent protocol constants and thresholds in `src/torrent/manager.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T002 Update `TorrentStreamInfo` struct in `src/torrent/mod.rs` to ensure piece metadata is accessible
- [ ] T003 [P] Add `last_window_head` field to `GrowingFile` in `src/torrent/stream.rs` to track sliding window progress

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Fast Start Playback (Priority: P1) 🎯 MVP

**Goal**: Minimize time to first frame by prioritizing file headers and footers.

**Independent Test**: Measure time from magnet resolution to playback start; verify it's under 15 seconds.

### Implementation for User Story 1

- [ ] T004 [US1] Implement `set_sequential(true)` call in `TorrentManager::get_info` in `src/torrent/manager.rs`
- [ ] T005 [US1] Implement header/footer piece prioritization (Tier 0) in `TorrentManager::get_info` in `src/torrent/manager.rs`
- [ ] T006 [US1] Update `wait_for_torrent_download` in `src/main.rs` to trigger playback immediately upon Tier 0 completion

**Checkpoint**: User Story 1 functional - fast start enabled.

---

## Phase 4: User Story 2 - Smooth Sequential Playback (Priority: P1)

**Goal**: Ensure stutter-free playback by dynamically prioritizing pieces ahead of the read head.

**Independent Test**: Play a full 10-minute video; verify no buffering interruptions occur.

### Implementation for User Story 2

- [ ] T007 [US2] Implement Tier 1 & 2 sliding window logic in `GrowingFile::poll_read` in `src/torrent/stream.rs`
- [ ] T008 [US2] Create internal helper in `src/torrent/stream.rs` to update piece priorities via `librqbit` handle

**Checkpoint**: User Story 2 functional - sequential streaming is robust.

---

## Phase 5: User Story 3 - Responsive Seeking (Priority: P2)

**Goal**: Quickly resume playback after seeking by shifting the download priority window.

**Independent Test**: Seek to a 50% offset; verify playback resumes in under 8 seconds.

### Implementation for User Story 3

- [ ] T009 [US3] Override `GrowingFile::start_seek` in `src/torrent/stream.rs` to immediately reset and shift the sliding window
- [ ] T010 [US3] Implement priority cancellation for old pieces in `src/torrent/stream.rs` upon seeking

---

## Phase N: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T011 [P] Perform stress test with "Big Buck Bunny" magnet per `quickstart.md`
- [ ] T012 [P] Validate memory usage of the priority queue for large torrents

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies.
- **Foundational (Phase 2)**: Depends on Phase 1. BLOCKS all user stories.
- **User Story 1 (US1)**: Depends on Phase 2.
- **User Story 2 (US2)**: Depends on US1 logic.
- **User Story 3 (US3)**: Depends on US2 logic.
- **Polish**: Depends on all stories.

---

## Implementation Strategy

### MVP First (User Story 1)

1. Complete Setup and Foundation.
2. Implement header/footer priority (US1).
3. **VALIDATE**: Verify initial start time reduction.

### Incremental Delivery

1. Foundation ready.
2. US1 enables fast start.
3. US2 enables smooth linear playback.
4. US3 enables fast seeking.
