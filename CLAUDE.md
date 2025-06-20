# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

TraderGrader is a Rust-based MCP (Model Context Protocol) server designed to interact with EVE Online market data. The project is in early development stage with a basic Rust structure.

## Development Commands

**Build the project:**
```bash
cargo build
```

**Run the project:**
```bash
cargo run
```

**Check for compilation errors:**
```bash
cargo check
```

**Run tests:**
```bash
cargo test
```

**Format code:**
```bash
cargo fmt
```

**Run linter:**
```bash
cargo clippy
```

## Architecture

- **Language**: Rust (2024 edition)
- **Project Type**: MCP server for EVE Online market data interaction
- **Structure**: Standard Cargo project layout with `src/main.rs` as entry point
- **Current State**: Basic "Hello, world!" implementation - project is in initial setup phase

## Key Files

- `src/main.rs`: Main entry point (currently minimal implementation)
- `Cargo.toml`: Project configuration and dependencies
- `.gitignore`: Git ignore rules (excludes `/target` directory)