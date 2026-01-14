# Research: CastV2 Protocol Implementation

## Unknowns & Clarifications

### Protocol Buffers Source
**Question**: Where to obtain the official `.proto` definitions for `CastMessage`?
**Research**:
- `CastMessage` is the core envelope for CastV2.
- The authoritative source is the Chromium source code.
- `cast_channel.proto` defines `CastMessage`, `AuthChallenge`, `AuthResponse`, `AuthError`, `DeviceAuthMessage`.
- `logging_events.proto` and `authority_keys.proto` are mentioned in the user prompt, possibly for `AuthChallenge` or specific logging extensions, but `cast_channel.proto` is the primary requirement for `CastMessage`.

**Decision**:
- We will download `cast_channel.proto` from the Chromium repository mirror or a reliable source.
- We will also check if `logging_events.proto` or `authority_keys.proto` are strictly required for the basic connection and heartbeat (MVP). Usually, `cast_channel.proto` is sufficient for the basic framing and `CastMessage`.
- **Update**: The user prompt explicitly requested `logging_events.proto` and `authority_keys.proto` in Phase 1. This might be a mistake in the user's prompt (confusing it with `cast_channel.proto`) OR they specifically want those too. However, `CastMessage` is in `cast_channel.proto`.
- **Resolution**: We will include `cast_channel.proto` as it is MANDATORY for `CastMessage`. We will also include `authority_keys.proto` and `logging_events.proto` if we find they are dependencies of `cast_channel.proto` or required for the specific "Authority Keys" features, but `cast_channel.proto` is the priority.

### TLS Implementation Details
**Question**: specific settings for `rustls` to talk to Chromecast?
**Research**:
- Chromecasts use self-signed certificates or certificates signed by a Google root that might not be in the standard store for local development without verification.
- Common practice for `castv2` clients is to optionally disable verification or pin the specific Google Cast root CA.
- `rustls` is strict by default. We need a `ServerCertVerifier` implementation that can either be promiscuous (for development/easy use) or strictly verify against the Cast root.

**Decision**:
- Implement a `NoCertificateVerification` struct (or similar) for the MVP to ensure we can connect to local devices without complexity.
- Later, add proper verification using the Cast root CA.

## Technology Decisions

### Async Runtime
- **Decision**: `tokio`
- **Rationale**: User constraint. Industry standard.

### Serialization
- **Decision**: `prost`
- **Rationale**: User constraint. Efficient, generates Rust types from `.proto`.

### TLS
- **Decision**: `rustls`
- **Rationale**: User constraint. Secure, modern, pure Rust.

## Implementation Details

### Message Framing
- 4-byte big-endian length prefix.
- Followed by protobuf wire format of `CastMessage`.

### Heartbeat
- Namespace: `urn:x-cast:com.google.cast.tp.heartbeat`
- Payload: `{"type": "PING"}` (JSON string serialized into the `payload_utf8` field of `CastMessage`? Or is it a binary proto?)
- **Research**: Most CastV2 namespaces (heartbeat, receiver, media) use **JSON** payloads inside the `payload_utf8` field of `CastMessage`. The `CastMessage` itself is protobuf, but the business logic inside is often JSON.
- **Decision**: We need `serde` and `serde_json` to handle the payload contents efficiently, even if `prost` handles the outer envelope.
- **Constraint Check**: User said "minimize dependencies". `serde`/`serde_json` are extremely common but technically "extra".
- **Refinement**: Can we do simple string matching for PING/PONG? Yes. `{"type":"PING"}` is simple. But for complex media/receiver messages, JSON parsing is hard to avoid.
- **User Constraint**: "以下の crate 以外の追加は、慎重な検討を要するものとする。" (Additions other than `tokio`, `prost`, `rustls` require careful consideration).
- **Proposal**: We will *try* to use `serde_json` because manual JSON parsing is error-prone and insecure. If strictly forbidden, we might use a lighter parser or string templating for simple messages, but `serde` is standard. *However*, strictly reading the prompt: "Additions... require careful consideration".
- **Decision**: For PING/PONG, raw strings are fine. For Receiver/Media status, `serde_json` is the responsible choice for security and maintainability. We will propose adding `serde` + `serde_json` as a *necessary* addition for the "Implementation Phase".
