# Quickstart: Verifying Refined Torrent Streaming

## Setup
1.  Ensure you have a reliable internet connection.
2.  Have a known healthy magnet link (e.g., Big Buck Bunny).

## Test Case 1: Rapid Metadata Resolution
1.  Run `cargo run -- cast "magnet:?xt=urn:btih:..."`
2.  Observe the time until "Default Media Receiver launched".
3.  **Success**: Status changes from `DOWNLOADING` to `PLAYING` in under 15 seconds.

## Test Case 2: Smooth Playback
1.  Start playback of a large torrent.
2.  Watch for 2-3 minutes.
3.  **Success**: No "BUFFERING" interruptions occur if download speed > video bitrate.

## Test Case 3: Responsive Seeking
1.  While playing, seek forward 10 minutes using `Right Arrow`.
2.  Observe the buffering time.
3.  **Success**: Playback resumes in under 8 seconds.
