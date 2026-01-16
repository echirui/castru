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

- [x] T001 [P] Verify `librqbit` stats API availability for progress tracking in `src/torrent/manager.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T002 Add `torrent_progress: Option<f32>` and `torrent_file_name: Option<String>` to `AppState` in `src/main.rs`
- [x] T003 Add `torrent_progress: Option<f32>` to `TuiState` struct in `src/controllers/tui.rs`

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Monitor Download Progress (Priority: P1) 🎯 MVP

**Goal**: Display download percentage in the TUI during the torrent fetching phase.

**Independent Test**: Start casting a magnet link; verify the TUI shows "DOWNLOADING: 0.0%" and updates as bytes arrive.

### Implementation for User Story 1

- [x] T004 [P] [US1] Update `TuiController::draw` in `src/controllers/tui.rs` to render a download status line when `torrent_progress` is `Some`.
- [x] T005 [US1] Implement percentage calculation logic in the buffering loop within `src/main.rs`.

**Checkpoint**: User Story 1 functional - progress is visible.

---

## Phase 4: User Story 2 - Automated Playback After Completion (Priority: P1)

**Goal**: Block media loading until the torrent is 100% downloaded locally.

**Independent Test**: Start casting a torrent; verify the Chromecast stays on the home/idle screen until the local TUI shows 100%, then playback begins automatically.

### Implementation for User Story 2

- [x] T006 [US2] Modify the buffering loop in `load_media` (within `src/main.rs`) to check for 100% completion instead of 3%.
- [x] T007 [US2] Ensure `AppState` updates and `tui.draw()` calls occur inside the `load_media` download loop in `src/main.rs`.
- [x] T008 [US2] Guard the `app.load()` call in `src/main.rs` to execute only after the download loop terminates successfully.

**Checkpoint**: User Story 2 functional - playback only starts after full download.

---

## Phase 5: User Story 3 - Error Handling during Download (Priority: P2)

**Goal**: Detect stalled downloads and inform the user.

**Independent Test**: Disconnect internet during download; verify the TUI eventually shows an error or "Stalled" status.

### Implementation for User Story 3

- [x] T009 [US3] Add a timeout/stall detection mechanism in the `load_media` download loop in `src/main.rs`.

---

## Phase N: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [x] T010 [P] Refactor `load_media` loop for better readability and performance in `src/main.rs`.
- [x] T011 [P] Validate all scenarios defined in `specs/015-torrent-full-download/quickstart.md`.