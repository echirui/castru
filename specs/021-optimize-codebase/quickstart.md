# Quickstart: Testing & Benchmarking

**Feature**: `021-optimize-codebase`

This guide explains how to run the new test suite, generate coverage reports, and execute performance benchmarks.

## Prerequisites

1.  **Install `cargo-llvm-cov`**:
    ```bash
    cargo install cargo-llvm-cov
    ```
2.  **Install `criterion`** (handled automatically by cargo when running benches).

## Running Tests

Run the full test suite, including new unit and integration tests:

```bash
cargo test
```

## Generating Coverage Reports

To check code coverage:

```bash
# Print summary to stdout
cargo llvm-cov

# Generate HTML report (open target/llvm-cov/html/index.html)
cargo llvm-cov --html
```

## Running Benchmarks

To measure performance regression/improvement:

```bash
# Run all benchmarks
cargo bench

# Run specific transcoding benchmark
cargo bench --bench transcode_throughput
```

## Verifying Refactoring

To verify that `src/main.rs` has been simplified:

1. Check file size (should be significantly < 49KB).
2. Ensure logic resides in `src/lib.rs` or `src/controllers/`.
