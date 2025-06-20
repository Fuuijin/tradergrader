# TraderGrader MCP Server Usage

## What You've Built

TraderGrader is now a fully functional MCP (Model Context Protocol) server that provides EVE Online market data tools to AI assistants like Claude.

## Available Tools

### 1. `health_check`
- **Description**: Check if the server is running
- **Parameters**: None
- **Usage**: Basic connectivity test

### 2. `get_market_orders`
- **Description**: Get market order count for a region/item
- **Parameters**: 
  - `region_id` (required): EVE region ID (e.g., 10000002 for The Forge)
  - `type_id` (optional): Item type ID to filter
- **Usage**: Check market activity levels

### 3. `get_market_summary`
- **Description**: Detailed market analysis with buy/sell spreads
- **Parameters**:
  - `region_id` (required): EVE region ID 
  - `type_id` (required): Item type ID to analyze
- **Usage**: Price analysis and trading opportunities

## Common EVE Online IDs

### Popular Regions
- **10000002**: The Forge (Jita - main trade hub)
- **10000043**: Domain (Amarr)
- **10000032**: Sinq Laison (Dodixie)
- **10000030**: Heimatar (Rens)

### Popular Items
- **34**: Tritanium (basic mineral)
- **35**: Pyerite (basic mineral)
- **36**: Mexallon (basic mineral)
- **29668**: PLEX (game time token)
- **44992**: Skill Injector

## Installation Complete!

Your MCP server is now installed and ready to use with Claude Desktop. 

**Next Steps:**
1. Restart Claude Desktop
2. The TraderGrader tools will be available in new conversations
3. You can ask Claude to analyze EVE market data using your server

## Example Queries for Claude

Once connected, you can ask Claude things like:
- "Check the market for Tritanium in Jita"
- "What's the price spread for PLEX in The Forge?"
- "Show me market activity for skill injectors"
- "Compare buy and sell orders for Pyerite"

Your EVE market data is now accessible to Claude through your custom MCP server!