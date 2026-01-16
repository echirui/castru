# Research: castnow Feature Parity and Torrent Refinement

**Feature**: `018-castnow-feature-parity`

## 1. castnow CLI Options Analysis

Reference: [castnow GitHub](https://github.com/xat/castnow)

| Option | Description | Implementation Status in castru |
|--------|-------------|---------------------------------|
| `--myip` | Specify local interface IP | Missing. Needed for multi-interface setups. |
| `--port` | Streaming server port | Missing. Currently random. |
| `--subtitles` | Sidecar subtitle file | Missing. |
| `--volume` | Initial volume (0.0 - 1.0) | Missing. |
| `--loop` | Loop playlist | Missing. |
| `--quiet` | Suppress TUI/Logs | Missing. |
| `--exit` | Exit when done | Missing. |

## 2. Subtitle Support on Chromecast

- **Format**: Chromecast natively supports WebVTT (`.vtt`).
- **Mechanism**: Sidecar subtitles must be served via HTTP and included in the `tracks` field of the `MediaInformation` object during the `LOAD` command.
- **Conversion**: If the user provides `.srt`, we should ideally convert it to `.vtt` on-the-fly or warn the user.

## 3. Torrent Refinement (Sequential Downloading)

- **Peer-flix Pattern**: `peer-flix` (used by `castnow`) prioritizes the first and last pieces, then downloads sequentially.
- **librqbit**: It supports setting piece priorities. To ensure smooth streaming, we should:
  1.  Set the entire torrent or the target file to "sequential" mode using `AddTorrentOptions::initial_mode` or `ManagedTorrent::set_sequential(true)`.
  2.  If not directly supported, the `GrowingFile`'s sequential reads will naturally trigger sequential piece requests.
- **Refinement**: We will ensure `AddTorrentOptions` are optimized for streaming (e.g., initial buffering pieces priority).

## 4. Decision Log

| Decision | Rationale |
|----------|-----------|
| Manual CLI Parsing Extension | Maintain dependency minimalism. |
| WebVTT focus for Subtitles | Native Chromecast compatibility. |
| Add `port` and `myip` to `StreamServer` | Required for complex networking. |
| Sequential Priority in Torrent | Essential for high-bitrate streaming stability. |
