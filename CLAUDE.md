# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A Rust MCP (Model Context Protocol) server that wraps the `academic-paper-interpreter` library to analyze academic papers. It accepts paper metadata (title, URL, PDF URL, arXivId) and returns analysis results in JSON format.

## Build Commands

This project uses `cargo-make` for task automation:

```bash
# Development build
cargo make dev-build

# Release build (optimized)
cargo make release-build

# Run all tests (uses cargo-nextest, single-threaded)
cargo make nextest-run

# Format and lint all code
cargo make format-all
```

Run a single test:
```bash
cargo nextest run <test_name> --test-threads=1
```

## Architecture

**Workspace Structure:**
- `app` (root) - Main binary crate at `src/bin/app.rs`
- `shared` - Shared library with common utilities

**Shared Library Modules** (`shared/src/`):
- `errors.rs` - `AppError` enum and `AppResult<T>` type alias using `thiserror`
- `logger.rs` - `init_logger()` function using `tracing-subscriber`
- `utils.rs` - Utility functions (e.g., progress bar generation with `indicatif`)

**Key Dependencies:**
- `academic-paper-interpreter` - Git dependency for paper analysis
- `tracing`/`tracing-subscriber` - Logging infrastructure
- `thiserror`/`anyhow` - Error handling
- `serde`/`serde_json` - JSON serialization

## Conventions

- Uses Rust 2024 edition
- Tests use `#[test_log::test]` macro for logging in tests
- Workspace dependencies defined in root `Cargo.toml` under `[workspace.dependencies]`
