# Historical Market Data Features

TraderGrader now includes comprehensive historical market data analysis powered by EVE ESI APIs.

## 🎯 New Features

### 1. Historical Data Fetching
- **Endpoint**: `GET /markets/{region_id}/history/?type_id={type_id}`
- **Data Points**: ~400 days of historical data per item
- **Rate Limit**: 300 requests per IP per minute

### 2. Price Trend Analysis
- **Daily/Weekly/Monthly Changes**: Price movements and percentage changes
- **Volatility Analysis**: Standard deviation of recent prices
- **Trend Classification**: Strong Upward, Upward, Stable, Downward, Strong Downward

### 3. MCP Tools Added

#### `get_market_history`
Fetch raw historical market data for analysis.

**Parameters:**
- `region_id` (required): EVE region ID
- `type_id` (required): Item type ID

**Response:** Last 10 days of market data with average, high, low, and volume.

#### `get_price_analysis` 
Advanced price trend analysis with volatility and change calculations.

**Parameters:**
- `region_id` (required): EVE region ID  
- `type_id` (required): Item type ID

**Response:** Comprehensive price analysis including:
- Current price from historical data
- Daily/weekly/monthly price changes
- Volatility (standard deviation)
- Trend direction classification

## 📊 Data Structure

### MarketHistory
```rust
pub struct MarketHistory {
    pub average: f64,      // Average price for the day
    pub date: String,      // Date (YYYY-MM-DD format) 
    pub highest: f64,      // Highest price traded
    pub lowest: f64,       // Lowest price traded
    pub order_count: i64,  // Number of orders
    pub volume: i64,       // Total volume traded
}
```

### PriceAnalysis
```rust
pub struct PriceAnalysis {
    pub current_price: f64,
    pub day_change: f64,
    pub day_change_percent: f64,
    pub week_change: f64,
    pub week_change_percent: f64,
    pub month_change: f64,
    pub month_change_percent: f64,
    pub volatility: f64,
    pub trend: String,
}
```

## 🚀 CLI Usage

The `market_query.sh` script now supports historical commands:

```bash
# Price trend analysis
./market_query.sh --analysis -t 44992

# Historical data
./market_query.sh --history -t 34 -r 10000002

# Order count (existing)
./market_query.sh --orders -t 35

# Market summary (default)
./market_query.sh -t 36
```

## 📈 Example Analysis Output

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

## 🔧 One-Liner Installation

Install TraderGrader with historical data support:

```bash
curl -sSL https://raw.githubusercontent.com/your-username/tradergrader/main/install.sh | bash
```

The installer automatically:
- Clones the repository
- Builds the release binary
- Configures Claude Desktop integration
- Creates CLI shortcuts
- Sets up all 5 MCP tools (including historical)

## ⚠️ Rate Limiting

The ESI `/markets/{region_id}/history/` endpoint has a rate limit of **300 requests per IP per minute**. TraderGrader handles this gracefully with proper error reporting.

## 🧪 Testing

Historical functionality is tested with:
```bash
# Test historical data fetching
cargo test test_market_history -- --ignored

# Test price analysis
cargo test test_price_analysis -- --ignored
```

## 🎉 Integration

The historical features integrate seamlessly with existing TraderGrader functionality:
- Same MCP protocol compliance
- Consistent error handling  
- Compatible with Claude Desktop and Claude Code
- CLI tools work alongside existing market summary tools

Historical data enables more sophisticated trading analysis including trend identification, volatility assessment, and temporal price pattern recognition.