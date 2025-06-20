#!/bin/bash
# Easy market data queries for Claude Code

# Default values
REGION_ID=10000002  # The Forge (Jita)
TYPE_ID=34          # Tritanium

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
        -h|--help)
            echo "Usage: $0 [options]"
            echo "Options:"
            echo "  -r, --region ID    Region ID (default: 10000002 - The Forge)"
            echo "  -t, --type ID      Type ID (default: 34 - Tritanium)"
            echo ""
            echo "Common regions:"
            echo "  10000002 - The Forge (Jita)"
            echo "  10000043 - Domain (Amarr)"
            echo "  10000032 - Sinq Laison (Dodixie)"
            echo ""
            echo "Common items:"
            echo "  34 - Tritanium"
            echo "  35 - Pyerite"
            echo "  29668 - PLEX"
            echo "  44992 - Skill Injector"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

echo "Querying market data for Type $TYPE_ID in Region $REGION_ID..."
echo ""

# Build and query
cargo build --release >/dev/null 2>&1

# Get market summary
MESSAGE="{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"tools/call\",\"params\":{\"name\":\"get_market_summary\",\"arguments\":{\"region_id\":$REGION_ID,\"type_id\":$TYPE_ID}}}"

echo "$MESSAGE" | ./target/release/tradergrader 2>/dev/null | tail -1 | jq -r '.result.content[0].text'