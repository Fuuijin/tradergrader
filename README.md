# TraderGrader

A Rust-based MCP (Model Context Protocol) server that provides AI tools with seamless access to EVE Online market data through the ESI API.

## What is TraderGrader?

TraderGrader bridges the gap between AI assistants and EVE Online's complex market ecosystem. By implementing the Model Context Protocol, it allows AI tools to query market data, analyze trading opportunities, and provide insights into New Eden's economy.

**Model Context Protocol (MCP)** is an open standard that enables AI applications to connect securely with external data sources and tools. TraderGrader implements an MCP server that exposes EVE Online market data as callable tools for AI assistants.

## Architecture

### Technology Stack
- **Language**: Rust (2024 edition)
- **Protocol**: Model Context Protocol (MCP) 
- **API**: EVE Online ESI (EVE Swagger Interface)
- **Transport**: JSON-RPC over stdio/HTTP
- **Runtime**: Tokio async

### Core Components
- **MCP Server**: Handles protocol communication and tool dispatch
- **ESI Client**: HTTP client for EVE Online's ESI API with rate limiting
- **Data Models**: Rust structs for market data, orders, and history
- **Caching Layer**: Response caching to respect API limits  
- **Market Analysis**: Tools for finding trading opportunities

### Key Dependencies
- `tokio` - Async runtime
- `reqwest` - HTTP client for ESI API  
- `serde` - JSON serialization/deserialization
- `mcp-rust-sdk` or `rust-mcp` - MCP protocol implementation
- `chrono` - Date/time handling
- `redis` or `moka` - Caching (TBD)

## Planned Features

### MCP Tools
- **get_market_orders** - Retrieve current buy/sell orders for items in specific regions
- **get_market_history** - Access historical price data and trends
- **get_market_prices** - Compare current prices across multiple regions  
- **search_market_opportunities** - Identify arbitrage and trading opportunities
- **get_item_info** - Look up item details, names, and type IDs

### EVE ESI Endpoints
- `/markets/{region_id}/orders/` - Current market orders
- `/markets/{region_id}/history/` - Historical market data
- `/universe/types/{type_id}/` - Item information  
- `/universe/regions/` - Region data

## Development Roadmap

### Phase 1: Basic MCP Server Setup ⏳
- [x] Project initialization and structure
- [ ] MCP server with stdio transport
- [ ] Basic message handling and tool dispatch
- [ ] Health check tool implementation

### Phase 2: ESI Integration 📋
- [ ] ESI HTTP client with proper rate limiting  
- [ ] Market data models and JSON deserialization
- [ ] Basic market order fetching tool
- [ ] Error handling for API failures

### Phase 3: Core Market Tools 📋
- [ ] Market history tool implementation
- [ ] Multi-region price comparison
- [ ] Response caching system
- [ ] Rate limit compliance (300 req/min for history endpoint)

### Phase 4: Advanced Features 📋  
- [ ] Market opportunity analysis algorithms
- [ ] Bulk data operations
- [ ] Advanced error handling and retry logic
- [ ] Performance optimizations

### Phase 5: Production Ready 📋
- [ ] Comprehensive logging and monitoring
- [ ] Documentation and usage examples
- [ ] Integration tests
- [ ] Performance benchmarking

## Technical Considerations

### Rate Limiting & Caching
- ESI market history endpoint: 300 requests per IP per minute
- Cache market data for 5-15 minutes (matches ESI update frequency)
- Implement exponential backoff for failed requests
- Graceful handling of ESI downtime

### Data Strategy
- Focus on major trade hubs (The Forge, Domain, Heimatar, etc.)
- Support bulk operations to minimize API calls
- Handle invalid item/region IDs gracefully
- Provide meaningful error messages to AI tools

## Getting Started

### Prerequisites
- Rust 2024 edition or later
- Internet connection for ESI API access

### Build and Run
```bash
# Clone the repository
git clone https://github.com/yourusername/tradergrader.git
cd tradergrader

# Build the project
cargo build

# Run the server
cargo run

# Run tests
cargo test

# Check code quality
cargo clippy
cargo fmt
```

### Usage with AI Tools
TraderGrader implements the MCP protocol, allowing it to work with any MCP-compatible AI client:

```bash
# Run as MCP server (stdio transport)
cargo run

# The server will communicate via JSON-RPC messages on stdin/stdout
```

## Project Status

**Current Status**: Phase 1 - Project Setup  
**Last Updated**: June 2025

This project is in active development. The basic Rust structure is in place, and we're currently implementing the core MCP server functionality.

## Contributing

This project welcomes contributions! Whether you're interested in:
- EVE Online market mechanics and trading
- Rust development and async programming  
- Model Context Protocol implementation
- API integration and rate limiting strategies

### Development Guidelines
- Follow Rust idiomatic patterns and conventions
- Add tests for new functionality
- Ensure proper error handling for ESI API interactions
- Respect ESI rate limits and best practices
- Keep MCP tool interfaces simple and well-documented

## Resources

- [Model Context Protocol Specification](https://modelcontextprotocol.io/)
- [EVE Online ESI Documentation](https://esi.evetech.net/ui)
- [EVE University API Guide](https://wiki.eveuniversity.org/API_access_to_market_data)
- [Rust MCP SDK](https://github.com/modelcontextprotocol/rust-sdk)

## License

[License TBD]

---

*Fly safe, trade smart.* 🚀