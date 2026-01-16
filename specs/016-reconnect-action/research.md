# Research: Reconnect Action

**Feature**: `016-reconnect-action`

## 1. Current Connection Logic

### CastClient (`src/client.rs`)
- `connect(host, port)` spawns a background task with two nested loops.
- **Outer Loop**: Handles connection attempts (TCP + TLS). Retries every 5s on failure.
- **Inner Loop**: Handles active connection (Heartbeats, sending commands, reading events).
- If the inner loop fails (e.g., socket error), it breaks back to the outer loop for reconnection.

### Main Loop (`src/main.rs`)
- Calls `CastClient::connect()` once at startup.
- Obtains `events()` receiver.
- Performs `connect_receiver()` and `app.launch()`.

## 2. Problem: Stale Session
When the `CastClient` reconnects TCP/TLS automatically, the higher-level protocols (Receiver, Media) are NOT automatically re-initialized. The Chromecast expects a `CONNECT` message on the connection namespace for each new TCP session.

## 3. Proposed "Reconnect Action"

### Trigger
- Add `KeyCode::Char('r')` to `TuiController` in `src/controllers/tui.rs` mapping to `TuiCommand::Reconnect`.

### Implementation Strategy
1.  **Main Event Loop Integration**:
    - When `TuiCommand::Reconnect` is received:
    - Option A: Force-close the existing `CastClient` and create a new one. (Safest, ensures fresh state).
    - Option B: Signal the existing `CastClient` to restart its inner loop. (More complex to implement).
    
    Decision: **Option A** is preferred for reliability. However, we must ensure that the `events()` receiver in the main loop is updated or that we use a proxy channel.
    
    Actually, since `CastClient`'s `event_tx` is a broadcast channel, if we create a new `CastClient`, we get a new receiver. The main loop must replace its `events` receiver.

2.  **State Preservation**:
    - The `AppState` in `main.rs` (playback time, volume, playlist) must be preserved.
    - After reconnection, we call `client.connect_receiver()`.
    - We might NOT want to call `app.launch()` if an app is already running, but we should probably call `app.join()` or similar if we want to resume control.
    - For simplicity, a "Reconnect" might just re-run the initial setup: `connect_receiver` -> `launch` (or `join`).

## 4. Decision Log

| Decision | Rationale |
|----------|-----------|
| Use 'r' key | Standard for "reload" or "reconnect" in many CLI apps. |
| Re-create CastClient | Ensures all internal buffers and state are cleared. Simplifies logic compared to signalling a spawned task. |
| Show "RECONNECTING" in TUI | Essential user feedback. |
