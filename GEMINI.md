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
- 009-projector-animation: Added Rust 2021 + `crossterm` (existing), `tokio` (existing)
- 008-tui-enhancements: Added Rust 2021 Edition + `crossterm` (existing)
- 007-visual-tui: Added Rust 2021 Edition + `crossterm` (existing)


<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
