//! TraderGrader - EVE Online Market Data MCP Server
//! 
//! An experimental MCP server that provides AI tools with access to EVE Online 
//! market data, historical trends, and trading analysis via the ESI API.
//!
//! # Features
//! 
//! - Real-time EVE Online market data via ESI API
//! - Historical price analysis and trend detection
//! - Market opportunity identification
//! - Caching for optimal performance
//! - Full MCP (Model Context Protocol) compliance

use serde_json::Value;
use std::io::{self, BufRead, Write};

// Module declarations
pub mod error;
pub mod types;
pub mod market;
pub mod mcp;
pub mod server;
pub mod cache;

// Re-export commonly used types
pub use error::{TraderGraderError, Result};
pub use types::{MarketOrder, MarketHistory, MarketType, PriceAnalysis};
pub use market::MarketClient;
pub use mcp::McpHandler;
pub use server::StandaloneMcpServer;
pub use cache::{CacheKey, CacheItem, CacheBackend, CacheBackendExt, CacheConfig, CacheBackendType, CacheStats, EsiHeaderParser, InMemoryCacheBackend};

/// Main TraderGrader application
#[derive(Debug)]
pub struct TraderGraderApplication {
    mcp_handler: McpHandler,
}

impl TraderGraderApplication {
    /// Create a new TraderGrader application
    pub fn new() -> Self {
        Self {
            mcp_handler: McpHandler::new(
                "TraderGrader".to_string(),
                "0.1.0".to_string(),
            ),
        }
    }

    /// Run the MCP server main loop
    pub async fn run(&self) -> anyhow::Result<()> {
        // Silent startup for MCP protocol compliance

        // Simple MCP server loop - reads JSON-RPC from stdin, responds on stdout
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        for line in stdin.lock().lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }

            // Parse incoming JSON-RPC message
            match serde_json::from_str::<Value>(&line) {
                Ok(message) => {
                    let response = self.mcp_handler.handle_message(message).await;
                    
                    // Only send response if it's not null (notifications return null)
                    if !response.is_null() {
                        let response_str = serde_json::to_string(&response)?;
                        writeln!(stdout, "{response_str}")?;
                        stdout.flush()?;
                    }
                }
                Err(e) => {
                    eprintln!("Failed to parse message: {e}");
                }
            }
        }

        Ok(())
    }

    /// Get direct access to the market client for advanced usage
    pub fn market_client(&self) -> &MarketClient {
        &self.mcp_handler.market_client
    }

    /// Convenience method for fetching market orders
    pub async fn fetch_market_orders(&self, region_id: i32, type_id: Option<i32>) -> Result<Vec<MarketOrder>> {
        self.market_client().fetch_market_orders(region_id, type_id).await
    }

    /// Convenience method for getting market summary
    pub async fn get_market_summary(&self, region_id: i32, type_id: i32) -> Result<String> {
        self.market_client().get_market_summary(region_id, type_id).await
    }

    /// Convenience method for fetching market history
    pub async fn fetch_market_history(&self, region_id: i32, type_id: i32) -> Result<Vec<MarketHistory>> {
        self.market_client().fetch_market_history(region_id, type_id).await
    }

    /// Convenience method for price trend analysis
    pub async fn analyze_price_trends(&self, region_id: i32, type_id: i32) -> Result<PriceAnalysis> {
        self.market_client().analyze_price_trends(region_id, type_id).await
    }

    /// Convenience method for price history summary
    pub async fn get_price_history_summary(&self, region_id: i32, type_id: i32) -> Result<String> {
        self.market_client().get_price_history_summary(region_id, type_id).await
    }
}

impl Default for TraderGraderApplication {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_application_creation() {
        let app = TraderGraderApplication::new();
        // Test that the application can be created successfully
        let _ = app;
    }

    #[tokio::test]
    async fn test_convenience_methods() {
        let app = TraderGraderApplication::new();
        
        // Test that convenience methods exist and can be called
        // These won't make real API calls in unit tests
        let _market_client = app.market_client();
    }
}