# Data Model: Torrent Support

## Entities

### TorrentSource

Represents the source of the media content.

| Field | Type | Description |
|-------|------|-------------|
| `source_type` | `Enum` | `Magnet` or `File`. |
| `path` | `String` | Magnet URI or file path. |

### TorrentSession

Manages the active download and streaming state.

| Field | Type | Description |
|-------|------|-------------|
| `session_id` | `UUID` | Unique identifier for the streaming session. |
| `save_path` | `PathBuf` | Local directory where files are downloaded. |
| `target_file_index` | `usize` | Index of the video file being streamed. |
| `state` | `State` | `Resolving`, `Buffering`, `Playing`, `Finished`. |

## Configuration

### TorrentConfig

| Field | Type | Description |
|-------|------|-------------|
| `download_dir` | `Option<PathBuf>` | Custom download location (default: temp). |
| `keep_files` | `bool` | Whether to keep files after exit (default: false). |
| `listen_port` | `u16` | Port for peer connections (default: random). |

## Internal Structures

### GrowingFile

An `AsyncRead` + `AsyncSeek` implementation that reads from a file being downloaded.

| Behavior | Description |
|----------|-------------|
| `read` | Reads available bytes. If EOF reached but torrent incomplete, waits for more data (or specific piece). |
| `seek` | Moves the cursor. Triggers prioritization of the new piece in the torrent engine. |
