# Data Model: Buffering Tuning

## Internal Structures

### StreamConfig

Configuration for the streaming behavior.

| Field | Type | Description |
|-------|------|-------------|
| `chunk_size` | `usize` | Size of a single read operation (e.g., 256KB). |
| `buffer_capacity` | `usize` | Number of chunks to hold in memory (channel size). |

### StreamerState

Internal state of the streaming process.

| Field | Type | Description |
|-------|------|-------------|
| `tx` | `mpsc::Sender` | Channel to send chunks from reader to writer. |
| `handle` | `JoinHandle` | Handle to the background reader task. |

## Flow

1. **Connection**: Client connects to `StreamServer`.
2. **Setup**: Server determines `chunk_size` (default or tuned).
3. **Spawn**: Server spawns a `ReaderTask` that loops:
   - Read `chunk_size` from file.
   - Send `Bytes` to channel.
   - Pause if channel full (backpressure).
4. **Loop**: Main task loops:
   - Receive `Bytes` from channel.
   - Write to TCP socket.
