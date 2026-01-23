# Data Model - Replace PAUSED with Auto-Retry Waiting State

## Entity Updates

### `src/controllers/media.rs`

#### `PlaybackStatus` (Enum)

**Changes**:
- **Remove**: `Paused` variant.
- **Keep**: `Idle`, `Buffering`, `Playing`, `Finished`, `Reconnecting`, `Waiting`.

### `src/app.rs`

#### `AppState` (Struct)

**Changes**:
- **Remove**: `user_paused` field (bool).
- **Rename**: `last_system_pause_time` -> `pause_start_time` (Option<Instant>).
  - *Rationale*: This field now tracks ALL pause durations, not just system ones.

## Logic Updates

### State Transitions

| Trigger | Old State | New State | Side Effects |
|:---|:---|:---|:---|
| User presses 'Pause' | `Playing` | `Waiting` | Set `pause_start_time = Now` |
| Receiver sends 'PAUSED' | `Playing` | `Waiting` | Set `pause_start_time = Now` (if not set) |
| `Waiting` + 10s elapsed | `Waiting` | `Buffering` | Send `Play` / `Load` command |
| User presses 'Play' | `Waiting` | `Buffering` | Send `Play` command immediately |

### Watchdog Logic

- **Condition**: `current_status == PlaybackStatus::Waiting`
- **Check**: `pause_start_time.elapsed() >= 10s`
- **Action**: Trigger playback resume (same as current auto-recovery).
