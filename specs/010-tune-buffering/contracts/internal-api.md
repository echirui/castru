# Internal API Contract: Buffering

**Note**: This feature modifies internal logic in `src/server.rs`. No external public API changes.

## StreamServer

### Modified Methods

#### `handle_connection`

**Signature Change**: None (internal logic change only).

**Behavior Change**:
- Old: Synchronous read-write loop.
- New: Spawns a background `tokio::task` for reading. Consumes from a channel.

### New Helper

#### `stream_file_buffered`

```rust
async fn stream_file_buffered(
    socket: &mut TcpStream,
    file: File,
    config: StreamConfig
) -> Result<(), std::io::Error>
```

- **Input**:
  - `socket`: The active TCP connection.
  - `file`: Opened `tokio::fs::File`.
  - `config`: Buffer settings.
- **Output**: `Result` indicating success or I/O error.
