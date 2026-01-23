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
- Rust 2021 + `tokio`, `prost`, `rustls`, `serde`, `crossterm`, `librqbit` (Remaining after reduction). (019-reduce-dependencies)
- Rust 2021 + `librqbit`, `tokio` (020-refine-torrent-streaming)
- N/A (Temporary buffering on disk) (020-refine-torrent-streaming)
- Rust 2021 Edition + `tokio` (async runtime), `prost` (protobuf), `rustls` (TLS). *Constraint*: No new dependencies allowed without strong justification. (021-optimize-codebase)
- N/A (Feature focuses on code quality) (021-optimize-codebase)
- Rust 2021 Edition + `tokio` (async runtime), `librqbit` (torrent), `ffmpeg` (via `std::process::Command` for probing). (022-show-max-duration)
- N/A (Temporary file storage handled by `librqbit`). (022-show-max-duration)

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
- 022-show-max-duration: Added Rust 2021 Edition + `tokio` (async runtime), `librqbit` (torrent), `ffmpeg` (via `std::process::Command` for probing).
- 021-optimize-codebase: Added Rust 2021 Edition + `tokio` (async runtime), `prost` (protobuf), `rustls` (TLS). *Constraint*: No new dependencies allowed without strong justification.
- 020-refine-torrent-streaming: Added Rust 2021 + `librqbit`, `tokio`


<!-- MANUAL ADDITIONS START -->
- 018-castnow-feature-parity: Implemented advanced CLI options, subtitles, torrent stability, and playlist loop.
- 006-castnow-tui: Added panic hooks, title truncation, and TUI cleanup.
- 022-show-max-duration: Added background media probing for duration display during torrent buffering.
- 023-auto-resume-buffering: Implemented auto-pause/resume logic based on buffer thresholds.
- 024-show-seek-and-download-bars: Implemented dual seek/download bar in TUI with dynamic layout.
- 007-visual-tui: Implemented full visual TUI with ascii projector animation.
<!-- MANUAL ADDITIONS END -->
