# castru Development Guidelines

Auto-generated from all feature plans. Last updated: 2026-01-13

## Active Technologies
- Rust 2021 + `tokio`, `prost`, `rustls`. *Needs Research*: Minimal mDNS and TUI crates vs. raw implementation. (004-castnow-features)
- N/A (Memory-based playlist/queue) (004-castnow-features)
- Rust 2021 + `tokio`, `prost`, `rustls`, `mdns-sd`, `crossterm`. (004-castnow-features)
- N/A (Memory-based pipeline) (005-remote-transcoding)
- Rust 2021 + `tokio` (full), `crossterm` (existing), `mdns-sd`. (006-castnow-tui)
- Rust 2021 Edition + `crossterm` (existing) (007-visual-tui)
- Rust 2021 + `crossterm` (existing), `tokio` (existing) (009-projector-animation)
- N/A (Stateless) (009-projector-animation)
- Rust 2021 + `tokio` (existing). No new dependencies. (010-tune-buffering)
- N/A (Streaming from filesystem) (010-tune-buffering)
- Rust 2021 + `librqbit` (for BitTorrent), `tokio` (for async runtime). (011-add-torrent-support)
- System temporary directory for download buffering. (011-add-torrent-support)
- Rust 2021 + `librqbit` (existing), `tokio`. (012-fix-torrent-playback)
- Rust 2021 + `tokio`, `ffmpeg` (external). (013-fix-transcode-seek-sync)

- (001-castv2-protocol)

## Project Structure

```text
backend/
frontend/
tests/
```

## Commands

# Add commands for 

## Code Style

: Follow standard conventions

## Recent Changes
- 015-torrent-full-download: Implemented full download before playback with TUI progress tracking (percentage) and stall detection.
- 013-fix-transcode-seek-sync: Fixed seek/sync issues by capturing dynamic `media_session_id` and implementing time interpolation.
- 009-projector-animation: Upgraded TUI to a full-screen high-fidelity 3D animation (later transitioned to Projector ASCII per spec 009).


<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
