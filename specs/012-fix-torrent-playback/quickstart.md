# Quickstart: Verification of Torrent Fix

## Prerequisites

- A Chromecast device (or emulator).
- A valid Magnet link (e.g., Big Buck Bunny).

## Steps

1.  **Build**:
    ```bash
    cargo build
    ```

2.  **Run**:
    ```bash
    ./target/debug/castru cast --name "My TV" "magnet:?xt=urn:btih:..."
    ```

3.  **Observe CLI**:
    - Should see "Resolving magnet link..."
    - **New**: Should see "Buffering: 0% ... 5% ... 10%" (or similar).
    - Then "Playing...".

4.  **Observe TV**:
    - Screen should NOT stay black.
    - Video should start playing immediately after CLI says "Playing".
