# Data Model: Optimize Codebase

**Feature**: `021-optimize-codebase`

> **Note**: This feature focuses on code quality. The "Data Model" here refers to the structures used for testing and benchmarking, as well as new internal abstractions resulting from refactoring.

## Refactoring Abstractions

### `CastNowApp` (New)
*Proposed abstraction to reduce `src/main.rs` complexity.*

- **Purpose**: Encapsulates the application state and lifecycle, separating it from CLI argument parsing.
- **Fields**:
  - `config`: `Config` (Parsed CLI args)
  - `runtime`: `tokio::runtime::Runtime`
  - `discovery`: `DiscoveryService`
  - `renderer`: `TuiRenderer` (Optional)
- **Methods**:
  - `new(config: Config) -> Self`
  - `run(&self) -> Result<()>`

## Metric Entities

### `BenchmarkResult` (Conceptual)
*Represents the output of a `criterion` run.*

- **Fields**:
  - `function_name`: String
  - `mean_execution_time`: Duration
  - `throughput`: Bytes/sec (for transcoding)
  - `outliers`: Count

### `CoverageReport` (Conceptual)
*Represents the output of `cargo-llvm-cov`.*

- **Fields**:
  - `file_path`: String
  - `line_coverage_percent`: Float
  - `function_coverage_percent`: Float
  - `branch_coverage_percent`: Float
