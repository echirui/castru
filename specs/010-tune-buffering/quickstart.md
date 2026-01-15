# Quickstart: Verification of Buffering Fix

## Prerequisites

- A high-bitrate video file (e.g., 4K or 1080p high-bitrate MP4).
- `castru` built in debug or release mode.

## Steps

1. **Build**:
   ```bash
   cargo build --release
   ```

2. **Run Cast**:
   ```bash
   # Replace with your local high-bitrate file
   ./target/release/castru cast --name "My TV" ./my_high_bitrate_movie.mp4
   ```

3. **Observe**:
   - Playback should start reasonably quickly (< 2 seconds).
   - Watch for at least 30 seconds.
   - **Success**: No stuttering, freezing, or "Buffering..." messages on the TV.
   - **Failure**: Video freezes periodically while audio continues, or visual buffering artifacts appear.

## Tuning (Optional)

If stuttering persists, you can try manually adjusting the buffer constants in `src/server.rs` (if exposed via flags later, use those):
- `CHUNK_SIZE`
- `CHANNEL_CAPACITY`
