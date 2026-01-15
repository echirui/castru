# Research: Castnow-like TUI

## 1. Visual Design

Objective: Minimalist, single-line display.

**Format**:
```text
[State] Current / Total [Progress Bar] Volume%
```

**Example**:
```text
[Playing] 01:23 / 04:50 [==============>---------------------] Vol: 100%
```

**Components**:
- **State**: `Playing` (Green), `Paused` (Yellow), `Buffering` (Blue), `Idle` (Grey).
- **Time**: `MM:SS` format.
- **Bar**: `[` + `=` (completed) + `>` (head) + `-` (remaining) + `]`. Auto-scales to window width.
- **Volume**: Percentage or `(Muted)`.

## 2. Technical Implementation (Crossterm)

**Redrawing Strategy**:
To avoid flickering and scrolling:
1. `Queue(MoveToColumn(0))`
2. `Queue(Clear(ClearType::UntilNewLine))` or `CurrentLine`.
3. `Queue(Print(formatted_string))`
4. `Flush()`

**Input Handling**:
- Use `crossterm::event::EventStream` (if async) or the existing `std::thread` approach sending mpsc messages.
- Existing `TuiController` spawns a thread. We will stick to that as it works well with `tokio::sync::mpsc`.

**Key Mapping**:
- `Space`: Toggle Pause
- `Left`: Seek -15s
- `Right`: Seek +30s
- `Up`: Volume +5%
- `Down`: Volume -5%
- `m`: Toggle Mute
- `q` / `Esc`: Quit

## 3. Data Flow

`main.rs` loop:
- Receives `MediaStatus` from Chromecast.
- Updates local `AppState`.
- Calls `tui.draw(state)`.
- Receives `TuiCommand` from `tui_rx`.
- Executes command (send to CastDevice).

## 4. Dependencies

No new dependencies required. `crossterm` is sufficient.
