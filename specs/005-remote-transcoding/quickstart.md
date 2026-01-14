# Quickstart: Remote Control and Transcoding

## Prerequisites
- `ffmpeg` and `ffprobe` must be installed and in your PATH.

## Usage

### 1. Start Casting
Run the cast server as usual. It will automatically probe the file.
```bash
cargo run -- cast movie.mkv
```

### 2. Interactive Controls
Once playback starts, use the following keys in your terminal:
- **Space**: Toggle Play/Pause
- **Right Arrow**: Seek forward 30 seconds
- **Left Arrow**: Seek backward 10 seconds
- **'q' or Ctrl+C**: Stop and exit

### 3. Automatic Transcoding
If the file is an MKV with HEVC video or DTS audio, you will see a message:
`[INFO] Unsupported codec detected. Starting transcoding pipeline...`
The playback will start after a short delay (usually < 2 seconds).

### 4. Optional Remote Control (Web)
Enable the API (if implemented):
```bash
cargo run -- cast movie.mkv --enable-remote-api
```
You can then send commands via `curl`:
```bash
curl -X POST http://localhost:8080/api/v1/playback/toggle
```
