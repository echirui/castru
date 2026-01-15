# Quickstart: Torrent Streaming

## Prerequisites

- A valid Magnet URI for a public domain video (e.g., Big Buck Bunny).
- `castru` built with the `torrent` feature (if feature-gated) or standard build.
- A Cast device on the network.

## Streaming from Magnet

1.  **Command**:
    ```bash
    castru cast --name "My TV" "magnet:?xt=urn:btih:..."
    ```

2.  **Expected Output**:
    ```text
    Connected to My TV.
    Resolving magnet link...
    Metadata received: Big Buck Bunny (1080p).
    Found video: big_buck_bunny_1080p_surround.avi (800MB)
    Buffering...
    Streaming to My TV...
    ```

3.  **Verification**:
    - Video starts playing on TV.
    - CLI shows download progress/speed.

## Streaming from File

1.  **Command**:
    ```bash
    castru cast --name "My TV" ./my-video.torrent
    ```

2.  **Expected Output**:
    - Similar to magnet, but skips "Resolving" phase if metadata is in file.
