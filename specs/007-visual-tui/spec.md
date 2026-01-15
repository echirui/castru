# Specification: Visual TUI (btop-style)

## 1. Overview
Refactor the Terminal User Interface to use a full-screen, "btop-like" visual style instead of a single-line output. This aims to improve visibility, provide a proper seekbar, and fix usability issues with playback controls.

## 2. Problem Statement
- **Pause/Resume Issue**: Currently, the Space key only sends `Pause`, making it impossible to resume playback if the user toggles it.
- **Visibility**: The single-line display is hard to read and lacks a clear visual indication of progress (seekbar) compared to modern CLI tools like `btop`.
- **UX**: Screen updates can leave artifacts; user requested "Clear screen once" and full refresh.

## 3. Goals
- **Full Screen UI**: Use the terminal's Alternate Screen Buffer to control the entire window.
- **Toggle Playback**: Implement logic to toggle between Play and Pause states using a single key (Space).
- **Visual Seekbar**: Render a distinct, full-width or panel-based progress bar.
- **Controls**: Ensure `q` and `Ctrl+C` reliably exit the application and restore the terminal.

## 4. User Stories

### US.1 Full Screen Interface
- **As a** user,
- **I want** the application to take over the terminal screen,
- **So that** the interface looks clean and `btop`-like without mixing with shell history.

### US.2 Toggle Playback
- **As a** user,
- **I want** pressing Space to pause if playing, and play if paused,
- **So that** I have intuitive control over media flow.

### US.3 Visual Progress Bar
- **As a** user,
- **I want** a clear seekbar showing elapsed and total time,
- **So that** I can easily scan my position in the media.

## 5. Technical Requirements
- **Library**: Continue using `crossterm`.
- **Screen Buffer**: Use `EnterAlternateScreen` / `LeaveAlternateScreen`.
- **Input**:
    - Update `TuiCommand` to include `TogglePlay`.
    - Map Space to `TogglePlay`.
- **Rendering**:
    - Clear screen (or region) on redraw.
    - Draw a centralized interface or panels (Status, Progress, Metadata, Playlist).
- **State**:
    - Track `PlaybackStatus` accurately in `main.rs` to handle Toggle logic.

## 6. Success Criteria
- [ ] Application opens in full screen mode.
- [ ] Spacebar successfully toggles Play/Pause.
- [ ] A progress bar is clearly visible.
- [ ] `q` or `Ctrl+C` exits cleanly, restoring previous terminal content.
