#!/bin/bash
echo "Testing TraderGrader binary..."
echo "Binary path: $(pwd)/target/release/tradergrader"
echo "Binary exists: $(test -f target/release/tradergrader && echo "YES" || echo "NO")"
echo "Binary executable: $(test -x target/release/tradergrader && echo "YES" || echo "NO")"
echo "Running basic test..."
echo '{"jsonrpc": "2.0", "id": 1, "method": "initialize"}' | ./target/release/tradergrader | head -2