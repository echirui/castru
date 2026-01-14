# Research: Enhanced Cast Features

## mDNS Discovery
**Goal**: Discover Cast devices (`_googlecast._tcp.local`) without heavy dependencies.

**Options**:
1. **`mdns-sd`**: Pure Rust, supports discovery.
2. **`mdns`**: Another pure Rust option.
3. **`tokio` + `simple-dns`**: Implement raw UDP multicast listener.
4. **`zeroconf`**: Bindings to system libraries (Bonjour/Avahi).

**Constraint Check**: "Dependency Minimalism" prefers pure Rust solutions to avoid system lib dependencies (like Avahi dev headers) which complicate builds.
`mdns-sd` is pure Rust.
**Decision**: Use `mdns-sd` (or similar pure Rust crate like `mdns-rust` if better maintained) IF it doesn't pull in too many deps.
*Correction*: The user explicitly suggested `mdns-sd`. I will investigate its suitability.
*Alternative*: Since we already have `tokio`, building a simple mDNS query sender/receiver using `tokio::net::UdpSocket` might be the *most* minimal if we only need to send one query and parse the response. However, mDNS is complex (caching, TTL, etc.).
**Conclusion**: We will start with a specialized crate like `mdns-sd` for robustness. If it's too heavy, we fall back to a minimal custom implementation using `socket2` or `tokio` UDP.

## Media Control Messages
**Namespace**: `urn:x-cast:com.google.cast.media`

**Key Messages**:
- **GET_STATUS**: `{"type": "GET_STATUS", "requestId": 1}`. Response: `MEDIA_STATUS`.
- **LOAD**: Load content. `{"type": "LOAD", ...}`.
- **PLAY/PAUSE**: `{"type": "PLAY", ...}` / `{"type": "PAUSE", ...}`.
- **SEEK**: `{"type": "SEEK", "currentTime": <float sec>, "resumeState": "PLAYBACK_START"}`.
- **SET_VOLUME**: `{"type": "SET_VOLUME", "volume": {"level": 0.5}}`. Note: Volume can be set at Receiver level (`urn:x-cast:com.google.cast.receiver`) or Media level? Usually Receiver level for global device volume. Media level for stream volume.
*Clarification*: User asked for "音量調整" (Volume adjustment). Usually this means device volume.
**Decision**: Implement `Receiver` namespace volume control as primary (device volume). Implement `Media` namespace volume if needed for specific stream.

## Reconnection Logic
**Strategy**:
- **Connection State**: Track `Connected`, `Connecting`, `Disconnected`.
- **Keep-Alive**: Heartbeat (already implemented). If heartbeat fails, transition to `Disconnected`.
- **Retry Loop**: In `connect` (or a supervisor), if connection drops, wait `X` seconds (exponential backoff) and retry `connect()`.
- **Session Restoration**: After reconnection, we might need to re-launch the app or at least query status. The `source_id` / `destination_id` might change if the receiver restart? No, `sender-0` / `receiver-0` are constant. But app sessions have unique IDs.
**Decision**: For MVP, just re-establish TLS and `CONNECT` to `receiver-0`. Application session restoration is advanced.

## Documentation
**Standard**: `cargo doc` comments + `README.md`.
**Example**: A full example in `examples/` that does Discovery -> Connect -> Launch -> Play Media.
