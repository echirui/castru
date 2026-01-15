# Tasks: Fix Torrent Playback

**Feature Branch**: `012-fix-torrent-playback`
**Status**: Ready
**Total Tasks**: 9

## Dependencies

- **Phase 1 (Setup)**: Prerequisites
- **Phase 2 (Foundational)**: Core structures and Safe Reader
- **Phase 3 (Manager Refactor)**: API updates
- **Phase 4 (Integration)**: CLI Buffering Logic
- **Phase 5 (Polish)**: Verification

## Phase 1: Setup

- [x] T001 Verify development environment

## Phase 2: Foundational

- [x] T002 Update `TorrentState` enum in `src/torrent/mod.rs` to include `Buffering { progress: f32 }`
- [x] T003 Define `TorrentStreamInfo` struct in `src/torrent/mod.rs` (holds handle, path, offsets)
- [ ] T004 [P] Update `GrowingFile` in `src/torrent/stream.rs` to check `handle.chunks().is_present()` in `poll_read` before reading

## Phase 3: Manager Refactor

- [ ] T005 Refactor `TorrentManager::start_magnet` and `start_torrent_file` in `src/torrent/manager.rs` to return `TorrentStreamInfo` instead of opening `GrowingFile` directly

## Phase 4: Integration (CLI & Server)

- [ ] T006 Update `StreamSource` enum in `src/server.rs` to include `handle`, `file_offset`, `piece_length` in `Growing` variant
- [ ] T007 Update `StreamSource::open` in `src/server.rs` to instantiate `GrowingFile` with the new fields
- [x] T008 Implement buffering loop in `src/main.rs`: wait for ~3% download progress before calling `server.set_source` and `app.load`

## Phase 5: Polish

- [x] T009 Run `cargo fmt` and `cargo clippy` to ensure code quality

## Parallel Execution Opportunities

- T002, T003, and T004 are mostly independent edits to different files/structs.
- T006/T007 can be done while T005 is being implemented.

## Implementation Strategy

1.  **Safety First**: We fix `GrowingFile` (T004) first to ensure that *if* we read, we never read invalid zeroes.
2.  **API Change**: We update `TorrentManager` (T005) to expose the handle needed for monitoring.
3.  **UX**: We update `main.rs` (T008) to provide the visual feedback (Buffering...) and the delay logic.
