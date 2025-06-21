#!/bin/bash
# TraderGrader MCP Server - One-liner installer
# Usage: curl -sSL https://raw.githubusercontent.com/your-username/tradergrader/main/install.sh | bash

set -e

echo "🚀 Installing TraderGrader MCP Server..."

# Check if git is installed
if ! command -v git &>/dev/null; then
	echo "❌ Git is not installed. Please install git and try again."
	exit 1
fi

# Check if cargo is installed
if ! command -v cargo &>/dev/null; then
	echo "❌ Rust/Cargo is not installed. Please install Rust from https://rustup.rs/ and try again."
	exit 1
fi

# Default installation directory
INSTALL_DIR="$HOME/.local/share/tradergrader"
REPO_URL="https://github.com/fuuijin/tradergrader.git"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
	case $1 in
	--dir)
		INSTALL_DIR="$2"
		shift 2
		;;
	--repo)
		REPO_URL="$2"
		shift 2
		;;
	-h | --help)
		echo "TraderGrader MCP Server Installer"
		echo ""
		echo "Usage: $0 [options]"
		echo ""
		echo "Options:"
		echo "  --dir DIR     Installation directory (default: $HOME/.local/share/tradergrader)"
		echo "  --repo URL    Git repository URL"
		echo "  -h, --help    Show this help message"
		echo ""
		echo "Examples:"
		echo "  # Install to default location"
		echo "  curl -sSL https://raw.githubusercontent.com/your-username/tradergrader/main/install.sh | bash"
		echo ""
		echo "  # Install to custom directory"
		echo "  curl -sSL https://raw.githubusercontent.com/your-username/tradergrader/main/install.sh | bash -s -- --dir /opt/tradergrader"
		exit 0
		;;
	*)
		echo "Unknown option: $1"
		exit 1
		;;
	esac
done

echo "📂 Installing to: $INSTALL_DIR"

# Create installation directory
mkdir -p "$(dirname "$INSTALL_DIR")"

# Clone or update repository
if [ -d "$INSTALL_DIR" ]; then
	echo "🔄 Updating existing installation..."
	cd "$INSTALL_DIR"
	git pull
else
	echo "📥 Cloning repository..."
	git clone "$REPO_URL" "$INSTALL_DIR"
	cd "$INSTALL_DIR"
fi

# Build the project
echo "🔨 Building TraderGrader..."
cargo build --release

# Get the absolute path to the binary
BINARY_PATH="$INSTALL_DIR/target/release/tradergrader"

# Claude Desktop config setup
CLAUDE_CONFIG_DIR="$HOME/.config/claude-desktop"
CONFIG_FILE="$CLAUDE_CONFIG_DIR/claude_desktop_config.json"

echo "⚙️  Configuring Claude Desktop integration..."

# Create config directory if it doesn't exist
mkdir -p "$CLAUDE_CONFIG_DIR"

# Update or create Claude Desktop config
if [ -f "$CONFIG_FILE" ]; then
	# Backup existing config
	cp "$CONFIG_FILE" "$CONFIG_FILE.backup.$(date +%s)"

	# Check if tradergrader is already configured
	if grep -q '"tradergrader"' "$CONFIG_FILE"; then
		echo "🔄 Updating existing TraderGrader configuration..."
		# Use jq if available, otherwise manual replacement
		if command -v jq &>/dev/null; then
			jq ".mcpServers.tradergrader.command = \"$BINARY_PATH\"" "$CONFIG_FILE" >"$CONFIG_FILE.tmp" && mv "$CONFIG_FILE.tmp" "$CONFIG_FILE"
		else
			# Manual replacement as fallback
			sed -i.bak "s|\"command\": \".*tradergrader[^\"]*\"|\"command\": \"$BINARY_PATH\"|g" "$CONFIG_FILE"
		fi
	else
		echo "➕ Adding TraderGrader to existing configuration..."
		# Add tradergrader to existing config
		if command -v jq &>/dev/null; then
			jq ".mcpServers.tradergrader = {\"command\": \"$BINARY_PATH\", \"args\": []}" "$CONFIG_FILE" >"$CONFIG_FILE.tmp" && mv "$CONFIG_FILE.tmp" "$CONFIG_FILE"
		else
			# Manual addition as fallback (more complex, create a simple version)
			echo "⚠️  Please manually add the following to your Claude Desktop config:"
			echo "\"tradergrader\": {"
			echo "  \"command\": \"$BINARY_PATH\","
			echo "  \"args\": []"
			echo "}"
		fi
	fi
else
	echo "📝 Creating new Claude Desktop configuration..."
	cat >"$CONFIG_FILE" <<EOF
{
  "mcpServers": {
    "tradergrader": {
      "command": "$BINARY_PATH",
      "args": []
    }
  }
}
EOF
fi

# Create convenient scripts
echo "📜 Creating convenience scripts..."

# Create a simple CLI wrapper
cat >"$INSTALL_DIR/tradergrader-cli" <<EOF
#!/bin/bash
# TraderGrader CLI wrapper
cd "$INSTALL_DIR"
exec "./market_query.sh" "\$@"
EOF
chmod +x "$INSTALL_DIR/tradergrader-cli"

# Add to PATH if possible
if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]] && [ -d "$HOME/.local/bin" ]; then
	ln -sf "$INSTALL_DIR/tradergrader-cli" "$HOME/.local/bin/tradergrader"
	echo "🔗 Created symlink in $HOME/.local/bin/tradergrader"
fi

echo ""
echo "✅ TraderGrader MCP Server installed successfully!"
echo ""
echo "🎯 Installation summary:"
echo "   • Installed to: $INSTALL_DIR"
echo "   • Binary: $BINARY_PATH"
echo "   • Config: $CONFIG_FILE"
echo ""
echo "🚀 Next steps:"
echo "   1. Restart Claude Desktop to load the new MCP server"
echo "   2. Start a new conversation to access TraderGrader tools"
echo ""
echo "🛠️  CLI usage:"
echo "   • Test: cd $INSTALL_DIR && ./market_query.sh --help"
if [ -L "$HOME/.local/bin/tradergrader" ]; then
	echo "   • Quick access: tradergrader --help"
fi
echo ""
echo "🔧 Tools available in Claude:"
echo "   • health_check - Test server connectivity"
echo "   • get_market_orders - Get current market orders"
echo "   • get_market_summary - Market analysis with spreads"
echo "   • get_market_history - Historical price data"
echo "   • get_price_analysis - Price trends and volatility"
echo ""
echo "📖 Documentation: $INSTALL_DIR/README_MCP.md"
echo ""
echo "Happy trading! 🎉"
