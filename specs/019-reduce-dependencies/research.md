# Research: Dependency Minimization and Refinement

**Feature**: `019-reduce-dependencies`

## 1. Target Crates for Removal

### thiserror
- **Usage**: Automatically generates `Display` and `From` implementations for error enums.
- **Replacement**: Manual implementation of `std::fmt::Display` and `std::error::Error`.
- **Affected Files**: `src/error.rs`, `src/torrent/mod.rs`.

### uuid
- **Usage**: Generates unique IDs for `TorrentSession` and temporary transcode filenames.
- **Replacement**: 
  - For `TorrentSession`: Use a simple counter or a timestamp if perfect uniqueness isn't globally required (local only).
  - For transcode filenames: Use `std::time::SystemTime` + a simple random suffix or just a counter.
- **Affected Files**: `src/server.rs`, `src/torrent/mod.rs`.

### bstr
- **Usage**: Used in `src/torrent/manager.rs` for `ByteSlice`.
- **Replacement**: Use standard `String::from_utf8_lossy` or standard slice methods.
- **Affected Files**: `src/torrent/manager.rs`.

## 2. Evaluation of bytes and others

- **bytes**: Highly integrated into `tokio` and `prost`. Removal would require significant re-engineering of the protocol codec. **KEEP**.
- **simplelog**: Used for basic logging. **KEEP** until a custom logger is specifically requested.
- **serde/serde_json**: Required for protocol JSON parsing. **KEEP**.

## 3. Decision Log

| Decision | Rationale |
|----------|-----------|
| Remove `thiserror` | Simple to implement manually; reduces one macro-heavy crate. |
| Remove `uuid` | Only used locally; timestamp + random is sufficient. |
| Remove `bstr` | Minimal usage; standard library is enough. |
| Keep `bytes` | Standard library `Vec<u8>` is less efficient for the specific buffer manipulation required by the protocol. |
