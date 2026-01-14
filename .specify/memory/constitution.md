<!--
Sync Impact Report:
- Version change: Template -> 1.0.0
- Modified principles: Defined Dependency Minimalism, Library-First, Test-First, Async I/O, Secure Transport
- Added sections: Defined Technology Constraints and Workflow
- Removed sections: None
- Templates requiring updates: ✅ None (Generic templates are compatible)
- Follow-up TODOs: None
-->
# castru Constitution

## Core Principles

### I. Dependency Minimalism
The project strictly limits external dependencies to ensure long-term maintainability and security. The core stack is restricted to `tokio`, `prost`, and `rustls`. Any additional dependency (e.g., `serde`) requires explicit justification and "careful consideration" as noted in the project mandate. We prefer standard library solutions where feasible.

### II. Library-First Architecture
The primary artifact is a Rust library crate (`lib.rs`) that exposes a clean, idiomatic, and documented API. Any CLI or example binary (`main.rs`) consumes this library via its public API, ensuring the core functionality is reusable and embedded-friendly. The library must not depend on CLI-specific logic.

### III. Test-First Development
We adhere to a Test-First methodology. For protocol parsing and state machines, unit tests MUST be written before implementation (Red-Green-Refactor). For network interactions, integration tests against mock servers or defined contracts MUST be defined prior to connecting to real devices.

### IV. Async I/O
All I/O operations must be non-blocking and built upon the `tokio` runtime. Blocking I/O in the core event loop is strictly prohibited. The system must handle concurrent operations (e.g., heartbeats and user commands) using `tokio::select!` or similar patterns without managing raw threads.

### V. Secure Transport
Security is a foundational requirement. All connections to Cast devices must use TLS 1.2 or 1.3 via `rustls`. The implementation must handle certificate validation scenarios (including self-signed or specific roots) explicitly and securely, defaulting to safe behaviors where possible.

## Technology Constraints

- **Language**: Rust (Edition 2021 or later, stable channel).
- **Runtime**: `tokio` with `net`, `time`, `sync` features.
- **Serialization**: Protocol Buffers via `prost` and `prost-build`.
- **Encryption**: `rustls` for TLS; `openssl` is explicitly avoided to maintain a pure Rust stack.
- **Framing**: CastV2 framing (4-byte Big Endian length header).

## Development Workflow

1.  **Specify**: Define the feature in `spec.md` with clear User Stories and Success Criteria.
2.  **Plan**: Map technical implementation in `plan.md`, ensuring Constitution compliance.
3.  **Tasks**: Generate granular, testable tasks in `tasks.md`, grouped by User Story.
4.  **Implement**: Execute tasks in order, prioritizing the foundational phase and then User Stories by priority (P1 -> P2...).
5.  **Verify**: Run `cargo test` and `cargo clippy` to ensure quality.

## Governance

This Constitution supersedes all other practices. Amendments require a version bump and explicit documentation of the rationale.

**Rules**:
- All Pull Requests must be checked against these principles.
- Complexity increases must be justified by user value.
- "Works on my machine" is not acceptable; CI/Automated tests are the standard.

**Version**: 1.0.0 | **Ratified**: 2026-01-13 | **Last Amended**: 2026-01-13