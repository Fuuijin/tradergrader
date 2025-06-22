# Scripts Directory

This directory contains all shell scripts and utilities for TraderGrader development and deployment.

## Installation Scripts

- **`install.sh`** - One-liner installer for production use
- **`install_mcp.sh`** - Claude Desktop MCP configuration

## Development & Testing Scripts

- **`market_query.sh`** - Quick market data testing
- **`mcp-wrapper.sh`** - MCP protocol wrapper for debugging
- **`test_from_claude_code.sh`** - Claude Code compatibility tests
- **`test_interactive.sh`** - Interactive testing interface
- **`test_jita_detailed.sh`** - Detailed Jita market testing
- **`test_jita_orders.sh`** - Jita order book testing
- **`test_mcp.py`** - Python-based MCP protocol testing

## Usage

All scripts are executable and can be run from the project root:

```bash
# Install TraderGrader
./scripts/install.sh

# Run market tests
./scripts/test_jita_orders.sh

# Test MCP integration
./scripts/test_from_claude_code.sh
```

For development, most scripts include built-in help when run without arguments.