# Quickstart - Auto-Resume Wait

**Feature**: Replace PAUSED with Auto-Retry Waiting State (`026-replace-paused-with-sleep`)

## Behavior Changes

- **Manual Pause**: Pressing `Space` (Pause) no longer permanently stops playback. It enters a `Waiting` state.
- **Auto-Resume**: After 10 seconds in `Waiting`, the player automatically attempts to resume playback.
- **System Pause**: If the Chromecast pauses due to buffering or errors, it also enters `Waiting` and retries after 10 seconds.

## Usage

1. **Pause**: Press `Space` or `k`.
   - **Result**: Status changes to `WAITING` (Magenta).
   - **After 10s**: Status changes to `BUFFERING`/`PLAYING` automatically.
2. **Stop**: Press `q` or `Esc` (depending on binding) to strictly Stop.
   - **Result**: Status `FINISHED`/`IDLE`. No auto-resume.
3. **Resume Early**: While `WAITING`, press `Space` or `k`.
   - **Result**: Resumes immediately.
