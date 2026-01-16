# Research: Torrent Full Download and Playback

**Feature**: `015-torrent-full-download`

## 1. librqbit Progress Tracking

The `librqbit::ManagedTorrent` struct provides a `stats()` method that returns a `Stats` object.
Key fields for progress calculation:
- `progress_bytes`: Number of bytes downloaded.
- `total_bytes`: Total size of the torrent (if metadata is resolved).

Calculation: `progress = (progress_bytes as f32 / total_bytes as f32) * 100.0`

## 2. TUI Integration

The TUI currently displays "No Media" or metadata during playback.
When a torrent is being downloaded *before* playback, we need a special state in the TUI to show:
- Status: "DOWNLOADING"
- Progress: "XX.X%"
- Filename: The target video file name.

## 3. Playback Trigger

Existing `load_media` logic:
- Starts magnet/torrent.
- Buffering loop (3% or 10MB).
- Calls `app.load()`.

New Logic:
- Starts magnet/torrent.
- Download loop (waits for 100%).
- Updates `AppState` and re-draws TUI inside the loop.
- After 100%, calls `app.load()`.

## 4. Decision Log

| Decision | Rationale |
|----------|-----------|
| Wait for 100% | Requested by user to ensure smooth playback without buffering issues. |
| Use existing TUI loop structure | Consistency with current rendering logic. |
| Keep `GrowingFile` for now | While the file is "complete", the `GrowingFile` abstraction still works and avoids extra logic to switch to `Static`. |
