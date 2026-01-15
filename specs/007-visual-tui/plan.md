# Plan: Visual TUI (btop-style)

## Summary
Transform the current single-line TUI into a full-screen, responsive interface using `crossterm`'s Alternate Screen features. Fix the playback toggle logic.

## Technical Context
- **Rust**: 2021 Edition
- **Crate**: `crossterm` 0.25+ (Already in use)
- **Architecture**: `main.rs` loop controlling `TuiController` which handles rendering.

## Project Structure Changes
- `src/controllers/tui.rs`:
    - Add `EnterAlternateScreen`/`LeaveAlternateScreen` to `start`/`stop`.
    - Refactor `draw` to render multiple lines/widgets.
    - Update input mapping for Toggle.
- `src/main.rs`:
    - Handle `TuiCommand::TogglePlay`.

## Implementation Strategy
1.  **Input Fix**: Add `TogglePlay` command and handle it in `main.rs`.
2.  **Screen Management**: Enable alternate screen in TUI setup.
3.  **Drawing Engine**: Rewrite `draw` to use a layout approach (e.g., center box or rows).
