# Data Model: castnow Feature Parity and Torrent Refinement

## CastOptions (Updated)

| Field | Type | Description |
|-------|------|-------------|
| `myip` | `Option<String>` | Manual local interface override. |
| `port` | `Option<u16>` | Manual HTTP server port override. |
| `subtitles` | `Option<String>` | Path to sidecar subtitle file. |
| `initial_volume` | `Option<f32>` | Volume level at start. |
| `loop_playlist` | `bool` | Whether to restart after last item. |
| `quiet` | `bool` | Suppress non-critical output. |

## Subtitle Track

Representation for the Cast Media Namespace.

| Field | Type | Description |
|-------|------|-------------|
| `trackId` | `i32` | Unique ID. |
| `type` | `String` | "TEXT". |
| `trackContentId` | `String` | URL to the subtitle file on the `StreamServer`. |
| `trackContentType` | `String` | "text/vtt". |
| `name` | `String` | Display name. |
| `language` | `String` | "en-US" (default). |
| `subtype` | `String` | "SUBTITLES". |

## Torrent Refinement State

- `sequential`: `bool` (Set to true in `AddTorrentOptions`).
