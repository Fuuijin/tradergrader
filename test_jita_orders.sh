#!/bin/bash

# Test script to call the get_market_orders tool for Jita (The Forge region)
# Region ID 10000002 corresponds to The Forge region where Jita is located

echo "Testing TraderGrader MCP server - fetching Jita market orders..."

# Build the project first
cargo build

# Create a temporary file with the MCP JSON-RPC messages
cat > /tmp/mcp_test_messages.json << 'EOF'
{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocolVersion": "2024-11-05", "capabilities": {}, "clientInfo": {"name": "test-client", "version": "1.0.0"}}}
{"jsonrpc": "2.0", "id": 2, "method": "tools/list"}
{"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {"name": "get_market_orders", "arguments": {"region_id": 10000002}}}
EOF

# Run the MCP server and pipe the test messages to it
echo "Sending MCP commands to TraderGrader server..."
cat /tmp/mcp_test_messages.json | cargo run

# Clean up
rm -f /tmp/mcp_test_messages.json