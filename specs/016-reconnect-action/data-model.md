# Data Model: Reconnect Action

## TuiCommand (Updated)

| Variant | Description |
|---------|-------------|
| `Reconnect` | Triggers a full connection reset and re-initialization. |

## AppState (No changes expected, but status might be used)

| Field | Type | Description |
|-------|------|-------------|
| `status` | `String` | Will show "RECONNECTING" during the process. |

## Sequence Diagram (Conceptual)

1.  **User** presses 'r'.
2.  **TUI** sends `TuiCommand::Reconnect`.
3.  **Main Loop** receives command.
4.  **Main Loop** updates status to "RECONNECTING".
5.  **Main Loop** drops old `CastClient` (or it's dropped by replacement).
6.  **Main Loop** calls `CastClient::connect()`.
7.  **Main Loop** replaces its `events` receiver with `new_client.events()`.
8.  **Main Loop** calls `new_client.connect_receiver()`.
9.  **Main Loop** updates status to "CONNECTED" (or relies on status events from device).
