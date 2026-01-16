# Quickstart: castnow Feature Parity

## New CLI Options

1.  **Specify local IP and Port**:
    ```bash
    cargo run -- cast media.mp4 --myip 192.168.1.5 --port 8888
    ```
2.  **Load Subtitles**:
    ```bash
    cargo run -- cast video.mkv --subtitles caption.vtt
    ```
3.  **Set Initial Volume**:
    ```bash
    cargo run -- cast media.mp4 --volume 0.3
    ```
4.  **Loop Playlist**:
    ```bash
    cargo run -- cast item1.mp4 item2.mp4 --loop
    ```

## Torrent Stability Check

1.  **Start a Torrent**:
    ```bash
    cargo run -- cast "magnet:?xt=urn:btih:..."
    ```
2.  **Observe**: Verify that playback starts smoothly and sequential downloading is active (check logs if `--log` is used).
