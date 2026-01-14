# Tasks: Remote Control and Transcoding Pipeline

**Input**: Design documents from `/specs/005-remote-transcoding/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: This feature follows the "Test-First Development" principle. Unit tests for the transcoding pipeline and API endpoints are included.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel
- **[Story]**: Which user story this task belongs to

## Phase 1: Setup

**Purpose**: Initialize new modules and dependencies.

- [x] T001 [Setup] Add `tokio::process` usage (included in tokio full) and ensure `ffmpeg`/`ffprobe` are available in env
- [x] T002 [P] [Setup] Create skeleton for `src/transcode.rs`
- [x] T003 [Setup] Register `mod transcode` in `src/lib.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core logic for interacting with external media tools.

- [x] T004 [P] [Foundational] Define `MediaProbeResult` and `TranscodeConfig` structs in `src/transcode.rs`
- [x] T005 [Foundational] Implement `probe_media(path)` function in `src/transcode.rs` using `ffprobe`
- [x] T006 [Foundational] Implement `needs_transcoding(&MediaProbeResult)` logic
- [x] T007 [Foundational] Add error variants for `Transcoding` and `Probe` in `src/error.rs`

**Checkpoint**: Application can inspect local files and decide if transcoding is needed.

---

## Phase 3: User Story 2 - Automatic Transcoding (Priority: P1)

**Goal**: Automatically transcode unsupported media files to H.264/AAC.

**Independent Test**: Casting a non-compatible file (e.g., MKV) should trigger ffmpeg and play successfully.

### Implementation for User Story 2

- [x] T008 [US2] Implement `TranscodingPipeline` struct in `src/transcode.rs` to manage `Child` process
- [x] T009 [US2] Implement `spawn_ffmpeg(&TranscodeConfig)` method returning `Stdout` pipe
- [x] T010 [US2] Update `StreamServer` in `src/server.rs` to accept an optional `child_stdout` stream
- [x] T011 [US2] Update `StreamServer` to serve from `child_stdout` chunk-by-chunk if present
- [x] T012 [US2] Update `load_media` in `src/main.rs` to use `probe_media` and configure server accordingly

---

## Phase 4: User Story 3 - Seeking during Transcoding (Priority: P2)

**Goal**: Enable seeking in transcoded streams by restarting the pipeline.

**Independent Test**: Seeking in a transcoded stream resumes playback from the new timestamp.

### Implementation for User Story 3

- [x] T013 [P] [US3] Parse HTTP `Range` header in `src/server.rs` to extract start time for transcoding (Replaced by Reload Strategy)
- [x] T014 [US3] Update `StreamServer` request handler to restart `TranscodingPipeline` with new `-ss` offset on seek (Replaced by Reload Strategy)
- [x] T015 [US3] Ensure `StreamServer` correctly kills the previous ffmpeg process before starting a new one
- [x] T016 [US3] Update `TuiController` seek logic to reload media with new timestamp if transcoding (?)
      *(Note: Implemented via Reload logic in main.rs)*

---

## Phase 5: User Story 4 - External Remote Control API (Priority: P3)

**Goal**: Control playback via HTTP requests.

**Independent Test**: `curl -X POST /api/v1/playback/toggle` toggles pause on the device.

### Implementation for User Story 4

- [ ] T017 [P] [US4] Define API route matching logic in `src/server.rs` (basic path parsing)
- [ ] T018 [US4] Implement `handle_api_request` in `src/server.rs` for `toggle`, `seek`, `stop`
- [ ] T019 [US4] Bridge `StreamServer` API handler to `CastClient` (This requires shared state or a command channel to `main.rs`)
- [ ] T020 [US4] Add `API_PORT` configuration or reuse streaming port (likely separate or same?)
      *(Decision: Reuse same port, check path prefix `/api/v1/`)*

---

## Phase 6: Polish & Cross-Cutting Concerns

- [x] T021 [Polish] Add graceful shutdown for all ffmpeg processes on app exit (Implicit via Drop)
- [x] T022 [Polish] Add descriptive error messages if `ffmpeg` is missing
- [ ] T023 [P] [Polish] Add `ffmpeg` progress logging (parse stderr?)
- [x] T024 [Doc] Update `README.md` with Transcoding details and API docs
- [ ] T025 [Test] Final integration test with a known "difficult" video file

---

## Implementation Strategy

1. **Probe & Decide**: First, get `ffprobe` working to decide *when* to transcode.
2. **Pipe**: Get `ffmpeg` stdout piping to the HTTP response. This is the hardest part (async IO).
3. **Seek**: Add the restart logic.
4. **Control**: Add the API last as a convenience layer.

## Note on User Story 1 (TUI)

User Story 1 (Interactive Keyboard Control) was largely implemented in the previous feature (`004-castnow-features`). 
Any refinements to TUI for seeking transcoded streams (if needing special handling) are covered in **Phase 4**.
