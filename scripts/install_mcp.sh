#!/bin/bash
# Install TraderGrader as an MCP server for Claude Desktop

echo "Installing TraderGrader MCP Server..."

# Build the release version
cargo build --release

# Get the absolute path to the binary
BINARY_PATH=$(pwd)/target/release/tradergrader

# Claude Desktop config directory
CLAUDE_CONFIG_DIR="$HOME/.config/claude-desktop"
CONFIG_FILE="$CLAUDE_CONFIG_DIR/claude_desktop_config.json"

# Create config directory if it doesn't exist
mkdir -p "$CLAUDE_CONFIG_DIR"

# Check if config file exists
if [ ! -f "$CONFIG_FILE" ]; then
    echo "Creating new Claude Desktop config..."
    cat > "$CONFIG_FILE" << EOF
{
  "mcpServers": {
    "tradergrader": {
      "command": "$BINARY_PATH",
      "args": []
    }
  }
}
EOF
else
    echo "Claude Desktop config exists. You'll need to manually add this to $CONFIG_FILE:"
    echo ""
    echo "Add this to the mcpServers section:"
    echo '"tradergrader": {'
    echo "  \"command\": \"$BINARY_PATH\","
    echo '  "args": []'
    echo '}'
    echo ""
fi

echo "Installation complete!"
echo "Binary location: $BINARY_PATH"
echo "Config file: $CONFIG_FILE"
echo ""
echo "Restart Claude Desktop to use the TraderGrader MCP server."