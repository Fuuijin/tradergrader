#!/bin/bash

# Test script to get detailed market information for Jita
# This tests multiple tools to get comprehensive market data

echo "Testing TraderGrader MCP server - comprehensive Jita market analysis..."

# Build the project first
cargo build

# Create test messages for multiple tools
cat > /tmp/mcp_detailed_test.json << 'EOF'
{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocolVersion": "2024-11-05", "capabilities": {}, "clientInfo": {"name": "test-client", "version": "1.0.0"}}}
{"jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": {"name": "health_check", "arguments": {}}}
{"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {"name": "get_market_orders", "arguments": {"region_id": 10000002}}}
{"jsonrpc": "2.0", "id": 4, "method": "tools/call", "params": {"name": "get_market_orders", "arguments": {"region_id": 10000002, "type_id": 34}}}
EOF

# Run the test
echo "Running comprehensive Jita market test..."
cat /tmp/mcp_detailed_test.json | cargo run

# Clean up
rm -f /tmp/mcp_detailed_test.json