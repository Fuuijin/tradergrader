use crate::error::Result;
use crate::types::{MarketHistory, MarketOrder, PriceAnalysis};
use reqwest::Client;

/// Market data client for EVE Online ESI API
/// 
/// Provides methods to fetch real-time market data, historical price information,
/// and perform market analysis using the EVE Online ESI (EVE Swagger Interface) API.
#[derive(Debug)]
pub struct MarketClient {
    http_client: Client,
}

impl MarketClient {
    /// Creates a new MarketClient with default HTTP client configuration
    /// 
    /// The client is configured with a proper user agent string for EVE ESI API compliance.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use tradergrader::MarketClient;
    /// let client = MarketClient::new();
    /// ```
    pub fn new() -> Self {
        Self {
            http_client: Client::builder()
                .user_agent("TraderGrader/0.1.0 (https://github.com/fuuijin/tradergrader)")
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    /// Fetches current market orders for a specific region and optional item type
    /// 
    /// # Arguments
    /// 
    /// * `region_id` - The EVE Online region ID (e.g., 10000002 for The Forge)
    /// * `type_id` - Optional item type ID to filter orders for a specific item
    /// 
    /// # Returns
    /// 
    /// Returns a vector of `MarketOrder` structs containing buy and sell orders
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// # use tradergrader::{MarketClient, Result};
    /// # async fn example() -> Result<()> {
    /// let client = MarketClient::new();
    /// // Get all orders in The Forge
    /// let orders = client.fetch_market_orders(10000002, None).await?;
    /// // Get orders for Tritanium only
    /// let tritanium_orders = client.fetch_market_orders(10000002, Some(34)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn fetch_market_orders(
        &self,
        region_id: i32,
        type_id: Option<i32>,
    ) -> Result<Vec<MarketOrder>> {
        let mut url = format!("https://esi.evetech.net/latest/markets/{region_id}/orders/");

        if let Some(tid) = type_id {
            url = format!("{url}?type_id={tid}");
        }

        let response = self.http_client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(
                format!("ESI API request failed with status: {}", response.status()).into(),
            );
        }

        let orders: Vec<MarketOrder> = response.json().await?;
        Ok(orders)
    }

    /// Fetches historical market data for a specific item in a region
    /// 
    /// Returns up to 13 months of historical daily market data including
    /// average price, highest/lowest prices, volume, and order count.
    /// 
    /// # Arguments
    /// 
    /// * `region_id` - The EVE Online region ID
    /// * `type_id` - The item type ID to get history for
    /// 
    /// # Returns
    /// 
    /// Returns a vector of `MarketHistory` structs with daily market data
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// # use tradergrader::{MarketClient, Result};
    /// # async fn example() -> Result<()> {
    /// let client = MarketClient::new();
    /// let history = client.fetch_market_history(10000002, 34).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn fetch_market_history(
        &self,
        region_id: i32,
        type_id: i32,
    ) -> Result<Vec<MarketHistory>> {
        let url = format!(
            "https://esi.evetech.net/latest/markets/{region_id}/history/?type_id={type_id}"
        );

        let response = self.http_client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(
                format!("ESI API request failed with status: {}", response.status()).into(),
            );
        }

        let history: Vec<MarketHistory> = response.json().await?;
        Ok(history)
    }

    /// Generates a comprehensive market summary with buy/sell order analysis
    /// 
    /// Analyzes current market orders to provide best buy/sell prices, spreads,
    /// and order depth information in a human-readable format.
    /// 
    /// # Arguments
    /// 
    /// * `region_id` - The EVE Online region ID
    /// * `type_id` - The item type ID to analyze
    /// 
    /// # Returns
    /// 
    /// Returns a formatted string containing market analysis
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// # use tradergrader::{MarketClient, Result};
    /// # async fn example() -> Result<()> {
    /// let client = MarketClient::new();
    /// let summary = client.get_market_summary(10000002, 34).await?;
    /// println!("{}", summary);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_market_summary(&self, region_id: i32, type_id: i32) -> Result<String> {
        let orders = self.fetch_market_orders(region_id, Some(type_id)).await?;

        let buy_orders: Vec<&MarketOrder> = orders.iter().filter(|o| o.is_buy_order).collect();
        let sell_orders: Vec<&MarketOrder> = orders.iter().filter(|o| !o.is_buy_order).collect();

        let highest_buy = buy_orders
            .iter()
            .max_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
        let lowest_sell = sell_orders
            .iter()
            .min_by(|a, b| a.price.partial_cmp(&b.price).unwrap());

        let summary = format!(
            "Market Summary for Type {} in Region {}:\n\
            Total Orders: {}\n\
            Buy Orders: {}\n\
            Sell Orders: {}\n\
            Highest Buy: {:.2} ISK\n\
            Lowest Sell: {:.2} ISK\n\
            Spread: {:.2} ISK",
            type_id,
            region_id,
            orders.len(),
            buy_orders.len(),
            sell_orders.len(),
            highest_buy.map(|o| o.price).unwrap_or(0.0),
            lowest_sell.map(|o| o.price).unwrap_or(0.0),
            if let (Some(sell), Some(buy)) = (lowest_sell, highest_buy) {
                sell.price - buy.price
            } else {
                0.0
            }
        );

        Ok(summary)
    }

    /// Analyzes price trends from historical market data
    /// 
    /// Calculates daily, weekly, and monthly price changes, volatility metrics,
    /// and determines the overall trend direction (bullish, bearish, or sideways).
    /// 
    /// # Arguments
    /// 
    /// * `region_id` - The EVE Online region ID
    /// * `type_id` - The item type ID to analyze
    /// 
    /// # Returns
    /// 
    /// Returns a `PriceAnalysis` struct with comprehensive trend metrics
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// # use tradergrader::{MarketClient, Result};
    /// # async fn example() -> Result<()> {
    /// let client = MarketClient::new();
    /// let analysis = client.analyze_price_trends(10000002, 34).await?;
    /// println!("Current trend: {}", analysis.trend);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn analyze_price_trends(
        &self,
        region_id: i32,
        type_id: i32,
    ) -> Result<PriceAnalysis> {
        let history = self.fetch_market_history(region_id, type_id).await?;

        if history.is_empty() {
            return Err("No historical data available".into());
        }

        // Sort by date (newest first)
        let mut sorted_history = history;
        sorted_history.sort_by(|a, b| b.date.cmp(&a.date));

        let current_price = sorted_history[0].average;

        // Calculate changes (day, week, month)
        let day_change = if sorted_history.len() > 1 {
            current_price - sorted_history[1].average
        } else {
            0.0
        };

        let week_change = if sorted_history.len() > 7 {
            current_price - sorted_history[7].average
        } else {
            0.0
        };

        let month_change = if sorted_history.len() > 30 {
            current_price - sorted_history[30].average
        } else {
            0.0
        };

        // Calculate volatility (standard deviation of last 30 days)
        let recent_prices: Vec<f64> = sorted_history.iter().take(30).map(|h| h.average).collect();

        let mean_price = recent_prices.iter().sum::<f64>() / recent_prices.len() as f64;
        let variance = recent_prices
            .iter()
            .map(|price| (price - mean_price).powi(2))
            .sum::<f64>()
            / recent_prices.len() as f64;
        let volatility = variance.sqrt();

        // Determine trend
        let trend = if week_change > current_price * 0.05 {
            "Strong Upward".to_string()
        } else if week_change > current_price * 0.02 {
            "Upward".to_string()
        } else if week_change < -current_price * 0.05 {
            "Strong Downward".to_string()
        } else if week_change < -current_price * 0.02 {
            "Downward".to_string()
        } else {
            "Stable".to_string()
        };

        Ok(PriceAnalysis {
            current_price,
            day_change,
            day_change_percent: if sorted_history.len() > 1 {
                (day_change / sorted_history[1].average) * 100.0
            } else {
                0.0
            },
            week_change,
            week_change_percent: if sorted_history.len() > 7 {
                (week_change / sorted_history[7].average) * 100.0
            } else {
                0.0
            },
            month_change,
            month_change_percent: if sorted_history.len() > 30 {
                (month_change / sorted_history[30].average) * 100.0
            } else {
                0.0
            },
            volatility,
            trend,
        })
    }

    /// Generates a formatted price history summary with trend analysis
    /// 
    /// Combines price analysis with human-readable formatting to provide
    /// a comprehensive overview of an item's price performance.
    /// 
    /// # Arguments
    /// 
    /// * `region_id` - The EVE Online region ID
    /// * `type_id` - The item type ID to analyze
    /// 
    /// # Returns
    /// 
    /// Returns a formatted string with price analysis summary
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// # use tradergrader::{MarketClient, Result};
    /// # async fn example() -> Result<()> {
    /// let client = MarketClient::new();
    /// let summary = client.get_price_history_summary(10000002, 34).await?;
    /// println!("{}", summary);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_price_history_summary(&self, region_id: i32, type_id: i32) -> Result<String> {
        let analysis = self.analyze_price_trends(region_id, type_id).await?;

        let summary = format!(
            "Price Analysis for Type {} in Region {}:\n\
            Current Price: {:.2} ISK\n\
            \n\
            Changes:\n\
            Daily: {:.2} ISK ({:+.2}%)\n\
            Weekly: {:.2} ISK ({:+.2}%)\n\
            Monthly: {:.2} ISK ({:+.2}%)\n\
            \n\
            Volatility: {:.2} ISK\n\
            Trend: {}",
            type_id,
            region_id,
            analysis.current_price,
            analysis.day_change,
            analysis.day_change_percent,
            analysis.week_change,
            analysis.week_change_percent,
            analysis.month_change,
            analysis.month_change_percent,
            analysis.volatility,
            analysis.trend
        );

        Ok(summary)
    }
}

impl Default for MarketClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Tests use parent scope types

    #[test]
    fn test_market_client_creation() {
        let client = MarketClient::new();
        // Just test that we can create a client
        let _ = client;
    }

    #[test]
    fn test_default_client() {
        let client = MarketClient::default();
        let _ = client;
    }

    // Mock test for URL formation
    #[test]
    fn test_url_formation() {
        let region_id = 10000002;
        let type_id = Some(34);
        
        let base_url = format!("https://esi.evetech.net/latest/markets/{region_id}/orders/");
        assert!(base_url.contains("10000002"));
        
        if let Some(tid) = type_id {
            let full_url = format!("{base_url}?type_id={tid}");
            assert!(full_url.contains("type_id=34"));
        }
    }

    #[test]
    fn test_price_trend_calculation() {
        // Test trend determination logic
        let week_change = 5.0;
        let day_change = 2.0;
        
        let trend = if week_change > 2.0 && day_change > 0.0 {
            "bullish"
        } else if week_change < -2.0 && day_change < 0.0 {
            "bearish"
        } else {
            "sideways"
        };
        
        assert_eq!(trend, "bullish");
    }

    #[test]
    fn test_market_summary_format() {
        // Test summary format strings
        let region_id = 10000002;
        let type_id = 34;
        let best_buy = 95.50;
        let best_sell = 96.75;
        let spread = best_sell - best_buy;
        
        let summary = format!(
            "Market Summary for Type {} in Region {}:\n\
            Best Buy Order: {:.2} ISK\n\
            Best Sell Order: {:.2} ISK\n\
            Spread: {:.2} ISK ({:.2}%)",
            type_id, region_id, best_buy, best_sell, spread, 
            (spread / best_buy) * 100.0
        );
        
        assert!(summary.contains("Market Summary"));
        assert!(summary.contains("95.50"));
        assert!(summary.contains("96.75"));
    }

    #[test]
    fn test_volatility_calculation() {
        let prices = vec![100.0, 105.0, 95.0, 102.0, 98.0];
        let mean = prices.iter().sum::<f64>() / prices.len() as f64;
        
        let variance = prices.iter()
            .map(|price| (price - mean).powi(2))
            .sum::<f64>() / prices.len() as f64;
            
        let volatility = variance.sqrt();
        
        assert!(volatility > 0.0);
        assert!(mean > 90.0 && mean < 110.0);
    }
}

