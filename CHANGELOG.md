# Changelog

All notable changes to TraderGrader will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added (Docker Support)
- **Comprehensive Docker Support** - Multi-stage Dockerfile with optimized 88.4MB production builds
- **ENTRYPOINT Architecture** - Clean CLI argument handling (`docker run tradergrader --health`)
- **docker-compose.yml** - Production, development, and Redis integration profiles
- **Docker Scripts** - Automated build, run, and testing utilities
- **Health Checks** - Proper container health monitoring and lifecycle management
- **Security** - Non-root user configuration and minimal attack surface
- **Development Mode** - Docker setup with Rust toolchain for development workflows

## [0.1.0] - 2025-06-22

### Added
- Initial MCP (Model Context Protocol) server implementation
- EVE Online ESI API integration for market data
- Five core MCP tools:
  - `health_check` - Server status verification
  - `get_market_orders` - Real-time buy/sell orders
  - `get_market_summary` - Market analysis with spreads
  - `get_market_history` - Historical price data (~400 days)
  - `get_price_analysis` - Advanced trend analysis with volatility
- Comprehensive error handling with custom error types
- Standalone MCP server with connection lifecycle management
- Automated installation scripts for easy deployment
- Claude Desktop integration configuration
- Comprehensive test suite (23 unit tests)
- Full API documentation with examples
- Multiple testing and development utilities

### Features
- **Real-time Market Data**: Live EVE Online market orders and pricing
- **Historical Analysis**: Up to 13 months of daily market history
- **Price Trend Analysis**: Daily/weekly/monthly changes with volatility metrics
- **Market Summaries**: Human-readable market analysis reports
- **MCP Protocol Compliance**: Full JSON-RPC 2.0 and MCP standard support
- **Claude Code Compatible**: Verified working with Claude Code CLI
- **Claude Desktop Integration**: Automatic configuration setup

### Technical
- **Language**: Rust 2021 edition
- **Architecture**: Modular design with clean separation of concerns
- **Dependencies**: Modern Rust ecosystem (tokio, reqwest, serde, thiserror)
- **Error Handling**: Comprehensive error types with JSON-RPC code mapping
- **Testing**: 23 unit tests covering all core functionality
- **Documentation**: Full docstrings and examples for all public APIs
- **Code Quality**: Zero clippy warnings, idiomatic Rust patterns

### Infrastructure
- **Installation**: One-liner curl installer
- **Scripts**: Organized development and testing utilities
- **CI/CD Ready**: Clean project structure for future automation
- **Documentation**: Comprehensive README and inline documentation

### Security
- **No Secrets**: Clean codebase with no hardcoded credentials
- **Safe Dependencies**: Vetted Rust crates with security focus
- **Input Validation**: Proper validation of all external inputs

### Development
- **Project Structure**: Standard Rust project layout
- **Testing Scripts**: Multiple test utilities for different scenarios
- **Development Tools**: MCP debugging and analysis scripts
- **Documentation**: Extensive inline documentation and examples

## Project Status

⚠️ **EXPERIMENTAL** - This project is in active development and should be used with caution. While functional, it's not recommended for production trading decisions.

### Known Limitations
- No caching implementation (all API calls are real-time)
- Limited to EVE Online ESI API rate limits
- No authentication for private market data
- Experimental status with ongoing development

### Future Plans
- Redis caching implementation
- Rate limiting and optimization
- Extended market analysis features
- Production hardening and stability improvements

---

**Legend:**
- 🚀 Major features
- ✨ New features  
- 🐛 Bug fixes
- 📚 Documentation
- 🔧 Technical improvements
- ⚠️ Breaking changes