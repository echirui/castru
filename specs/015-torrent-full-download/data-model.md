# Data Model: Torrent Full Download and Playback

## AppState (in main.rs)

Updated to track torrent download progress.

| Field | Type | Description |
|-------|------|-------------|
| `torrent_progress` | `Option<f32>` | Current download percentage (0.0 to 100.0). |
| `torrent_file_name` | `Option<String>` | Name of the file being downloaded. |

## TuiState (in tui.rs)

Updated to carry progress to the renderer.

| Field | Type | Description |
|-------|------|-------------|
| `torrent_progress` | `Option<f32>` | If Some, TUI shows downloading progress instead of playback time. |

## State Transitions

1.  **Idle -> Resolving**: Magnet link added, waiting for metadata.
2.  **Resolving -> Downloading**: Metadata acquired, pieces being fetched.
3.  **Downloading -> Ready**: Progress reaches 100%.
4.  **Ready -> Playing**: `LOAD` command sent to Chromecast.
