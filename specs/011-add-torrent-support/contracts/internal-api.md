# Internal API: Torrent Streaming

## TorrentManager

Orchestrates the lifecycle of a torrent stream.

```rust
pub struct TorrentManager {
    // fields...
}

impl TorrentManager {
    /// Starts a new torrent session from a URI or file.
    pub async fn start(source: &str, config: TorrentConfig) -> Result<Self, Error>;

    /// Returns a handle to the streamable file.
    /// This handle implements AsyncRead + AsyncSeek and handles "wait for data".
    pub async fn get_stream(&self) -> Result<Box<dyn AsyncReadSeek>, Error>;

    /// Stops the session and cleans up.
    pub async fn shutdown(self) -> Result<(), Error>;
}
```

## GrowingFile

A wrapper around `tokio::fs::File` (or `librqbit` internal stream) that coordinates with the download progress.

```rust
impl AsyncRead for GrowingFile {
    fn poll_read(...) -> Poll<Result<()>>; // Blocks (pending) if data missing
}

impl AsyncSeek for GrowingFile {
    fn start_seek(...) -> Poll<Result<()>>; // Prioritizes pieces at new location
}
```
