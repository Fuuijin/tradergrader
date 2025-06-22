#!/bin/bash
# Easy market data queries for Claude Code

# Default values
REGION_ID=10000002  # The Forge (Jita)
TYPE_ID=34          # Tritanium
COMMAND="summary"   # Default command

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -r|--region)
            REGION_ID="$2"
            shift 2
            ;;
        -t|--type)
            TYPE_ID="$2"
            shift 2
            ;;
        -c|--command)
            COMMAND="$2"
            shift 2
            ;;
        --history)
            COMMAND="history"
            shift
            ;;
        --analysis)
            COMMAND="analysis"
            shift
            ;;
        --orders)
            COMMAND="orders"
            shift
            ;;
        -h|--help)
            echo "TraderGrader CLI - EVE Online Market Data Tool"
            echo ""
            echo "Usage: $0 [options]"
            echo ""
            echo "Options:"
            echo "  -r, --region ID    Region ID (default: 10000002 - The Forge)"
            echo "  -t, --type ID      Type ID (default: 34 - Tritanium)"
            echo "  -c, --command CMD  Command to run (summary|history|analysis|orders)"
            echo ""
            echo "Quick commands:"
            echo "  --history          Get historical price data"
            echo "  --analysis         Get price trend analysis"
            echo "  --orders           Get current market orders count"
            echo ""
            echo "Common regions:"
            echo "  10000002 - The Forge (Jita)"
            echo "  10000043 - Domain (Amarr)"
            echo "  10000032 - Sinq Laison (Dodixie)"
            echo "  10000030 - Heimatar (Rens)"
            echo ""
            echo "Common items:"
            echo "  34 - Tritanium        35 - Pyerite"
            echo "  36 - Mexallon         37 - Isogen"
            echo "  39 - Megacyte         40 - Zydrine"
            echo "  11399 - Morphite      29668 - PLEX"
            echo "  44992 - Skill Injector"
            echo ""
            echo "Examples:"
            echo "  $0                              # Default: Tritanium summary in Jita"
            echo "  $0 --analysis -t 44992          # Skill Injector price analysis"
            echo "  $0 --history -r 10000043 -t 35  # Pyerite history in Amarr"
            echo "  $0 --orders -t 29668            # PLEX order count"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

echo "Querying market data for Type $TYPE_ID in Region $REGION_ID..."
echo "Command: $COMMAND"
echo ""

# Build and query
cargo build --release >/dev/null 2>&1

# Determine which tool to call based on command
case $COMMAND in
    "summary")
        TOOL_NAME="get_market_summary"
        ;;
    "history")
        TOOL_NAME="get_market_history"
        ;;
    "analysis")
        TOOL_NAME="get_price_analysis"
        ;;
    "orders")
        TOOL_NAME="get_market_orders"
        ;;
    *)
        echo "Unknown command: $COMMAND"
        echo "Valid commands: summary, history, analysis, orders"
        exit 1
        ;;
esac

# Build the JSON-RPC message
MESSAGE="{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"tools/call\",\"params\":{\"name\":\"$TOOL_NAME\",\"arguments\":{\"region_id\":$REGION_ID,\"type_id\":$TYPE_ID}}}"

# Execute query and parse response
echo "$MESSAGE" | ./target/release/tradergrader 2>/dev/null | tail -1 | jq -r '.result.content[0].text'