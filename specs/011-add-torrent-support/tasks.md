# Tasks: Torrent Streaming Support

**Feature Branch**: `011-add-torrent-support`
**Status**: Ready
**Total Tasks**: 10

## Dependencies

- **Phase 1 (Setup)**: Dependencies
- **Phase 2 (Foundational)**: Core structures and Streaming Adapter
- **Phase 3 (User Story 1)**: Magnet Link Support & Integration
- **Phase 4 (User Story 2)**: Torrent File Support
- **Phase 5 (Polish)**: Cleanup and Quality

## Phase 1: Setup

- [x] T001 Verify development environment and backup `Cargo.toml`
- [x] T002 Add `librqbit` and `tokio-util` (if needed) dependencies to `Cargo.toml`

## Phase 2: Foundational

- [x] T003 Define `TorrentConfig`, `TorrentSession` structs and `TorrentError` in `src/torrent/mod.rs`
- [x] T004 Implement `GrowingFile` struct in `src/torrent/stream.rs` implementing `AsyncRead + AsyncSeek` (wraps file, handles EOF wait)
- [x] T005 Implement `TorrentManager` in `src/torrent/manager.rs` (init session, add torrent, sequential logic, find largest file)

## Phase 3: User Story 1 - Stream from Magnet Link

**Goal**: Stream video from a magnet link.
**Priority**: P1
**Independent Test**: Run `castru cast "magnet:?..."` and verify playback.

- [x] T006 [US1] Refactor `StreamServer` in `src/server.rs` to support abstract `StreamSource` (File vs Torrent) instead of hardcoded `PathBuf`
- [x] T007 [US1] Implement `TorrentManager::start_magnet` to resolve metadata and return a `GrowingFile` handle
- [x] T008 [US1] Update `src/main.rs` to detect magnet links, instantiate `TorrentManager`, and pass stream to `StreamServer`

## Phase 4: User Story 2 - Stream from Torrent File

**Goal**: Stream video from a local .torrent file.
**Priority**: P2
**Independent Test**: Run `castru cast ./video.torrent` and verify playback.

- [x] T009 [US2] Update `TorrentManager` and `main.rs` to handle `.torrent` file paths

## Phase 4: Polish & Cross-Cutting Concerns

- [x] T010 Implement `Drop` or shutdown logic for `TorrentManager` to delete temp files (unless configured otherwise) and run `cargo fmt/clippy`

## Parallel Execution Opportunities

- T004 (GrowingFile) and T005 (Manager) can be implemented in parallel.
- T006 (Server Refactor) can be done while Torrent logic is being built.

## Implementation Strategy

1.  **Adapter Pattern**: We will wrap the complex torrent download logic behind a `GrowingFile` that looks just like a standard file to the `StreamServer`.
2.  **Refactor First**: We will make `StreamServer` generic/abstract (T006) before hooking up the actual torrent logic to ensure no regression for normal files.
3.  **MVP**: Get Magnet links working (T007/T008) as they are the primary use case.