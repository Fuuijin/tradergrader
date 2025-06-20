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
- **Current State**: Project has evolved from basic setup to use a `TraderGraderApplication` struct

## Key Files

- `src/main.rs`: Main entry point that runs `TraderGraderApplication`
- `src/lib.rs`: Library crate entry point (contains `TraderGraderApplication`)
- `Cargo.toml`: Project configuration, dependencies, and build settings
- `.gitignore`: Git ignore rules (excludes `/target` directory)

## Rust Project Structure Reference

Based on standard Rust project anatomy:
- `src/main.rs`: Binary crate entry point
- `src/lib.rs`: Library crate entry point  
- `src/bin/`: Additional binary targets
- `src/`: Module files (can be `mod.rs` or standalone `.rs` files)
- `tests/`: Integration tests
- `examples/`: Example code
- `benches/`: Benchmarks

For more details on Rust project structure: https://cheats.rs/#project-anatomy