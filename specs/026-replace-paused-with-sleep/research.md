# Research Findings - Replace PAUSED with Auto-Retry Waiting State

**Feature Branch**: `026-replace-paused-with-sleep`
**Created**: 2026-01-23

## Decisions

### Decision 1: Remove PlaybackStatus::Paused
- **Rationale**: The requirement is "Remove all PAUSED". Eliminating the enum variant guarantees this at the compiler level and forces logic updates in all state handlers.
- **Impact**: 
  - `src/controllers/media.rs`: Remove `Paused` from `PlaybackStatus` enum.
  - `src/app.rs`: Replace usage with `Waiting` + Timer.
  - `src/controllers/tui.rs`: Remove rendering logic for `Paused` status.

### Decision 2: Generalize Waiting State
- **Rationale**: Use the existing `Waiting` state (from 025) for everything.
- **Logic**:
  - `last_system_pause_time` in `AppState` should be renamed to `pause_start_time` (since it's now for ALL pauses).
  - When `Pause` command is received -> set `Waiting`, set `pause_start_time`.
  - When `PAUSED` event received -> set `Waiting`, set `pause_start_time` (if not already set).
  - Watchdog checks `Waiting` state:
    - If `now - pause_start_time > 10s` -> `load_media` / `play`.

### Decision 3: Remove `user_paused` distinction
- **Rationale**: Since user manual pause now behaves identically to a system pause (wait 10s then resume), we no longer need to track `user_paused` to prevent auto-recovery.
- **Impact**: Simplify `AppState` and logic in `src/app.rs`.

## Technical Unknowns Resolved

- **PlaybackStatus::Paused removal**: Yes, removing it entirely is feasible and safer.
- **Timer visualization**: Will update TUI to show `Waiting` for all paused states.

## Alternatives Considered

- **Keeping `Paused` but changing logic**: Might lead to confusion or accidental "forever pause" regressions. Removing the variant is safer for compliance.
- **Separate `UserWaiting` state**: No benefit if behavior is identical (10s wait). Simpler to unify.
