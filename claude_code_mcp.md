# Using TraderGrader with Claude Code

## Method 1: Direct MCP Connection

Claude Code can connect to MCP servers using the `--mcp-server` flag:

```bash
# Start your TraderGrader MCP server in one terminal
cd /home/fuuijin/repos/tradergrader
cargo run

# In another terminal, connect Claude Code to it
claude --mcp-server "stdio::/home/fuuijin/repos/tradergrader/target/release/tradergrader"
```

## Method 2: Add to CLAUDE.md Configuration

Add MCP server configuration to your project's CLAUDE.md file so Claude Code knows about it automatically.

## Method 3: Use as a Background Service

Run the MCP server as a background service that Claude Code can connect to when needed.

## Testing the Connection

Once connected, Claude Code will have access to these tools:
- `health_check` - Test server connectivity
- `get_market_orders` - Get order counts for regions/items  
- `get_market_summary` - Get detailed price analysis

You can then ask me to use these tools to analyze EVE market data!