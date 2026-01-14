# Quickstart: Castnow Features

## Automatic Device Discovery

To list all available Chromecast devices on your network:

```bash
cargo run -- scan
```

## Streaming a Local File

To cast a local video file to the first discovered Chromecast:

```bash
cargo run -- cast path/to/video.mp4
```

To cast to a specific device by name:

```bash
cargo run -- cast path/to/video.mp4 --device "Living Room TV"
```

## Interactive Controls

Once playback starts, use the following keys:

- `Space`: Pause / Resume
- `Left Arrow`: Seek backward 30s
- `Right Arrow`: Seek forward 30s
- `Q` or `Esc`: Stop and Quit
- `N`: Next item (if playlist)
- `P`: Previous item (if playlist)

## Playlist Management

Provide multiple files to create a playlist:

```bash
cargo run -- cast video1.mp4 video2.mkv video3.mp4
```
