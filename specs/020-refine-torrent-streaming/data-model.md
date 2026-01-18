# Data Model: Refined Torrent Streaming Strategy

## Priority Tiers

We will categorize torrent pieces into tiers to guide the engine:

| Tier | Urgency | Pieces | Trigger |
|------|---------|--------|---------|
| **Tier 0** | Immediate | First & Last pieces of target file | Initial Load |
| **Tier 1** | High | Current read head + 10MB | `poll_read` / `start_seek` |
| **Tier 2** | Normal | Current read head + 50MB | Sequential Window |
| **Tier 3** | Low | Remainder of the file | Default Sequential |

## Entity Refinements

### TorrentStreamInfo (src/torrent/mod.rs)

No changes to fields, but we will use `handle` more actively.

### GrowingFile (src/torrent/stream.rs)

| Method | Internal Logic Change |
|--------|----------------------|
| `poll_read` | Periodically checks if the "priority window" needs to slide forward. |
| `start_seek` | Immediately resets the "priority window" to the new byte offset. |

## State Transitions

1.  **Torrent Added**:
    - Metadata resolved.
    - Largest file identified.
    - `set_sequential(true)` called.
    - Tier 0 pieces requested.
2.  **Streaming Begins**:
    - Receiver requests bytes via HTTP.
    - `GrowingFile` enters Tier 1 buffering.
3.  **User Seeks**:
    - `start_seek` called with new offset.
    - `GrowingFile` shifts Tier 1 and Tier 2 windows.
    - Peers informed of new priority.
