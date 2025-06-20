#!/bin/bash
# Interactive test for TraderGrader MCP server

echo "Starting TraderGrader MCP Server..."
echo "You can send JSON-RPC messages. Try these examples:"
echo ""
echo "1. Initialize:"
echo '{"jsonrpc":"2.0","id":1,"method":"initialize"}'
echo ""
echo "2. List tools:"
echo '{"jsonrpc":"2.0","id":2,"method":"tools/list"}'
echo ""
echo "3. Health check:"
echo '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"health_check","arguments":{}}}'
echo ""
echo "4. Market summary (Tritanium in The Forge):"
echo '{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"get_market_summary","arguments":{"region_id":10000002,"type_id":34}}}'
echo ""
echo "5. Market orders count:"
echo '{"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"get_market_orders","arguments":{"region_id":10000002,"type_id":34}}}'
echo ""
echo "Press Ctrl+C to exit"
echo "=================="

cargo run