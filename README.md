# TraderGrader

⚠️ **WORK IN PROGRESS - USE WITH CAUTION** ⚠️

This is an **experimental** Rust-based MCP (Model Context Protocol) server that provides AI tools with access to EVE Online market data, historical trends, and trading analysis through the ESI API. While functional with Claude Code and Claude Desktop, this project is still in active development and should be used with care, if at all.

## 🎯 What is TraderGrader?

TraderGrader bridges the gap between AI assistants and EVE Online's complex market ecosystem. By implementing the Model Context Protocol, it allows AI tools to query real-time market data, analyze historical price trends, identify trading opportunities, and provide insights into New Eden's economy.

✅ **Basic functionality working with Claude Code** - The MCP server connects and responds to basic queries, but comprehensive testing is ongoing.

**Model Context Protocol (MCP)** is an [open standard](https://modelcontextprotocol.io/) that enables AI applications to connect securely with external data sources and tools. TraderGrader implements an experimental MCP server that exposes EVE Online market data as callable tools for AI assistants.

## ⚡ Quick Start

### One-Liner Installation

```bash
curl -sSL https://raw.githubusercontent.com/fuuijin/tradergrader/main/scripts/install.sh | bash
```

This automatically:
- Installs TraderGrader to `~/.local/share/tradergrader`
- Builds the optimized release binary
- Configures Claude Desktop integration
- Creates convenient CLI tools

### Manual Installation

```bash
git clone https://github.com/fuuijin/tradergrader.git
cd tradergrader
cargo build --release
./scripts/install_mcp.sh  # Configure for Claude Desktop
```

### 🐳 Docker Installation

**Quick Docker Start:**
```bash
# Build and run with Docker
git clone https://github.com/fuuijin/tradergrader.git
cd tradergrader
./scripts/docker-build.sh
./scripts/docker-run.sh --interactive
```

**Using docker-compose:**
```bash
# Production setup
docker-compose up

# Development with Redis
docker-compose --profile with-redis up

# Development mode with hot reload
docker-compose --profile dev up
```

**Docker Commands:**
```bash
# Build optimized production image
./scripts/docker-build.sh

# Build development image (with Rust toolchain)
./scripts/docker-build.sh --dev

# Run interactively
./scripts/docker-run.sh --interactive

# Run health check
./scripts/docker-run.sh --health

# Test Docker setup
./scripts/docker-test.sh
```

## 🛠️ Available Tools

TraderGrader provides 5 MCP tools for comprehensive market analysis:

### Core Market Data
- **`health_check`** - Test server connectivity and status
- **`get_market_orders`** - Current buy/sell order counts and activity
- **`get_market_summary`** - Real-time price analysis with spreads

### Historical Analysis 📈
- **`get_market_history`** - Historical price data (~400 days)
- **`get_price_analysis`** - Advanced trend analysis with volatility

## 📊 Features

### Real-Time Market Data
- Current market orders and pricing
- Buy/sell spreads and profit margins
- Multi-region arbitrage opportunities
- Order volume and market activity

### Historical Analysis
- Price trend analysis (daily/weekly/monthly changes)
- Volatility calculations using standard deviation
- Trend classification (Strong Upward/Downward, Stable)
- Historical data spanning ~400 days per item

### Trading Intelligence
- Automatic arbitrage opportunity detection
- Price spread analysis for profit margins
- Market activity monitoring
- Regional price comparison

## 🚀 Usage

### With Claude Desktop

After installation, restart Claude Desktop and use TraderGrader tools in conversations:

- *"Analyze the Tritanium market in Jita"*
- *"Check price trends for PLEX over the last month"*
- *"Find profitable trading opportunities for Skill Injectors"*
- *"Compare Pyerite prices between Jita and Amarr"*

### CLI Tools

```bash
# Market summary (default)
./market_query.sh -t 34 -r 10000002

# Price trend analysis
./market_query.sh --analysis -t 44992

# Historical data
./market_query.sh --history -t 35

# Current orders count
./market_query.sh --orders -t 36

# Help and options
./market_query.sh --help
```

### Direct MCP Integration

```bash
# Start MCP server
cargo run

# Send JSON-RPC commands
echo '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"get_price_analysis","arguments":{"region_id":10000002,"type_id":44992}}}' | cargo run
```

## 🏗️ Architecture

### Technology Stack
- **Language**: Rust (2024 edition)
- **Protocol**: Model Context Protocol (MCP)
- **API**: EVE Online ESI (EVE Swagger Interface)
- **Transport**: JSON-RPC over stdio
- **Runtime**: Tokio async
- **HTTP Client**: Reqwest with rate limiting

### Core Components
- **MCP Server**: Protocol communication and tool dispatch
- **ESI Client**: HTTP client with proper User-Agent and error handling
- **Market Analysis**: Real-time and historical data analysis
- **CLI Tools**: Command-line utilities for direct usage

### Key Dependencies
- `tokio` - Async runtime
- `reqwest` - HTTP client for ESI API
- `serde` - JSON serialization/deserialization
- `chrono` - Date/time handling for historical analysis

## 📈 Example Analysis

### Price Trend Analysis
```
Price Analysis for Type 44992 in Region 10000002:
Current Price: 6063000.00 ISK

Changes:
Daily: 0.00 ISK (+0.00%)
Weekly: -83000.00 ISK (-1.35%)
Monthly: -118000.00 ISK (-1.91%)

Volatility: 79716.97 ISK
Trend: Stable
```

### Trading Opportunities
```
Market Summary for Type 34 in Region 10000002:
Total Orders: 215
Buy Orders: 71
Sell Orders: 144
Highest Buy: 4.27 ISK
Lowest Sell: 2.70 ISK
Spread: -1.57 ISK
```

## 🗺️ Common EVE Online IDs

### Major Trade Hubs
- **10000002** - The Forge (Jita) - Primary trade hub
- **10000043** - Domain (Amarr) - Major trade hub
- **10000032** - Sinq Laison (Dodixie) - Regional hub
- **10000030** - Heimatar (Rens) - Regional hub

### Popular Trading Items
- **34** - Tritanium (basic mineral)
- **35** - Pyerite (basic mineral)
- **44992** - Skill Injector (high-value item)
- **29668** - PLEX (game time token)
- **11399** - Morphite (rare mineral)

## ⚠️ Technical Considerations

### Rate Limiting
- ESI history endpoint: 300 requests per IP per minute
- Proper error handling and backoff strategies
- User-Agent identification as required by CCP

### Data Quality
- Real-time data from official EVE ESI API
- ~400 days of historical data per item
- Automatic handling of API downtime and errors

## 🧪 Testing

```bash
# Run unit tests
cargo test

# Run integration tests (makes real API calls)
cargo test -- --ignored

# Test specific functionality
cargo test test_price_analysis -- --ignored
```

## 📁 Project Structure

```
tradergrader/
├── src/
│   ├── main.rs          # Application entry point  
│   ├── lib.rs           # Main application & public API (123 lines)
│   ├── types.rs         # Data structures & types (50 lines)
│   ├── market.rs        # Market logic & ESI API (199 lines)
│   └── mcp.rs           # MCP protocol handling (411 lines)
├── tests/               # Integration tests
├── install.sh           # One-liner installer script
├── market_query.sh      # CLI utility for market queries
├── ROADMAP.md           # Development roadmap
├── HISTORICAL_DATA.md   # Historical features documentation
└── README_MCP.md        # MCP-specific documentation
```

## 🛣️ Development Roadmap

### ✅ Completed (v0.1.0)
- [x] Production MCP server with stdio transport
- [x] Real-time market data fetching
- [x] Historical price data and trend analysis
- [x] Price volatility and change calculations
- [x] Trading opportunity analysis
- [x] Comprehensive CLI tools
- [x] One-liner installation script
- [x] Claude Desktop integration

### 🚧 Next Phase (v0.2.0)
- [ ] Redis caching for improved performance
- [ ] Multi-item bulk operations
- [ ] Corporation and alliance market tools
- [ ] Manufacturing cost calculations
- [ ] Advanced portfolio tracking

### 🔮 Future Phases
- [ ] Web dashboard interface
- [ ] Real-time market alerts and notifications
- [ ] Machine learning price prediction
- [ ] Integration with EVE character APIs
- [ ] Docker containerization

## 📝 License

[License TBD]

## 🤝 Contributing

Contributions welcome! Areas of interest:
- EVE Online market mechanics and trading strategies
- Rust async programming and optimization
- MCP protocol extensions and improvements
- Market analysis algorithms and statistics

### Development Guidelines
- Follow Rust idioms and conventions
- Add comprehensive tests for new features
- Respect ESI rate limits and best practices
- Maintain MCP protocol compliance

## 📚 Resources

### EVE Online & Market Data
- [ESI Documentation](https://esi.evetech.net/ui) - Interactive API docs
- [Market Data Guide](https://wiki.eveuniversity.org/API_access_to_market_data) - Comprehensive market API guide
- [EVE University Trading](https://wiki.eveuniversity.org/Trading) - Trading mechanics and strategies

### Model Context Protocol
- [MCP Specification](https://modelcontextprotocol.io/specification/2025-03-26) - Official protocol docs
- [MCP Introduction](https://modelcontextprotocol.io/introduction) - Getting started guide

### Rust Development
- [Tokio Async Runtime](https://tokio.rs/) - Async programming
- [Reqwest HTTP Client](https://docs.rs/reqwest/) - HTTP requests
- [Serde JSON](https://serde.rs/) - Serialization

---

_Fly safe, trade smart, profit wisely._ 🚀

**Status**: Experimental v0.1.0 - Use with Caution | **Last Updated**: June 2025