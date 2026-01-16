# Quickstart: Verifying Dependency Reduction

## Steps

1.  **Check current dependencies**:
    ```bash
    cargo tree
    ```
2.  **Apply refactoring**:
    - Remove `thiserror`, `uuid`, `bstr` from `Cargo.toml`.
3.  **Verify compilation**:
    ```bash
    cargo build
    ```
4.  **Run regression tests**:
    ```bash
    cargo test
    ```
5.  **Manual functional check**:
    - Run a transcode-heavy file:
      ```bash
      cargo run -- cast "path/to/mkv_file.mkv"
      ```
    - Run a torrent playback:
      ```bash
      cargo run -- cast "magnet:?..."
      ```
    - Observe TUI and server behavior.
