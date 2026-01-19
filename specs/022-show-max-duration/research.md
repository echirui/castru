# Research: Show Max Duration

**Feature**: `022-show-max-duration`
**Date**: 2026-01-18

## 1. Duration Probing Strategy

**Decision**: Use `ffprobe` asynchronously on the growing file.
**Rationale**: `ffprobe` can extract duration from the `moov` atom in MP4/MOV files or the header in MKV/AVI files even if the file is incomplete, provided the header bytes are downloaded.

**Mechanism**:
- In `src/app.rs`, trigger a background task once `torrent_progress` > 0.1% (or a fixed byte threshold like 2MB).
- This task runs `probe_media` on the target file path.
- On success, it updates `AppState` via a channel or shared Arc/Mutex (simplified: `AppState` is local to main loop, so we might need a channel or poll result).

## 2. Librqbit Prioritization

**Decision**: Rely on default sequential download or implicit prioritization.
**Rationale**: `librqbit` usually downloads strictly sequentially for streaming if configured (which `GrowingFile` usage implies). We assume header bytes come first.

## 3. Implementation Details

- **Concurrency**: `spawn_ffmpeg` is async but `probe_media` waits for output. This fits well within a `tokio::spawn`.
- **State Update**: The main loop in `app.rs` manages `AppState`. We can spawn the probe task and send the result back via an `mpsc` channel (e.g., `probe_tx`).
- **UI Update**: The TUI loop already redraws on events. Updating `app_state.total_duration` will be reflected in the next draw.

## 4. Risks

- **False Positives**: `ffprobe` might report wrong duration if metadata is incomplete.
- **File Locking**: Reading the file while writing might be an issue on Windows, but on Unix-likes it's fine. `GrowingFile` shares the file handle/path.

## 5. Dependency Impact

**Decision**: No new dependencies.
**Rationale**: `tokio`, `std::process`, and `serde_json` are all available.
