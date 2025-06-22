#!/bin/bash
# MCP Server wrapper for Claude Code compatibility
# This ensures proper startup and shutdown behavior

set -e

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BINARY_PATH="$SCRIPT_DIR/target/release/tradergrader"

# Ensure binary exists
if [ ! -f "$BINARY_PATH" ]; then
    echo "Error: TraderGrader binary not found at $BINARY_PATH" >&2
    exit 1
fi

# Ensure binary is executable
if [ ! -x "$BINARY_PATH" ]; then
    chmod +x "$BINARY_PATH"
fi

# Set up signal handling for clean shutdown
trap 'kill $(jobs -p) 2>/dev/null || true; exit 0' TERM INT

# Execute the binary with proper stdio handling
exec "$BINARY_PATH" "$@"