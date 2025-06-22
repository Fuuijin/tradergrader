#!/bin/bash
# Test TraderGrader from within Claude Code session

echo "=== TraderGrader MCP Server Test ==="
echo ""

# Function to test a single MCP call
test_mcp_call() {
    local test_name="$1"
    local json_message="$2"
    
    echo "Testing: $test_name"
    echo "Message: $json_message"
    echo "Response:"
    
    # Start server and send message
    echo "$json_message" | cargo run 2>/dev/null | tail -1 | jq .
    echo ""
}

# Test 1: Health Check
test_mcp_call "Health Check" \
'{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"health_check","arguments":{}}}'

# Test 2: Market Summary for Tritanium in The Forge
test_mcp_call "Market Summary (Tritanium in Jita)" \
'{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"get_market_summary","arguments":{"region_id":10000002,"type_id":34}}}'

# Test 3: Market Orders Count
test_mcp_call "Market Orders Count (Tritanium in Jita)" \
'{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"get_market_orders","arguments":{"region_id":10000002,"type_id":34}}}'

echo "=== Test Complete ==="