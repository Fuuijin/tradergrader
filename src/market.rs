use crate::error::Result;
use crate::types::{MarketHistory, MarketOrder, PriceAnalysis};
use reqwest::Client;

/// Market data client for EVE Online ESI API
#[derive(Debug)]
pub struct MarketClient {
    http_client: Client,
}

impl MarketClient {
    /// Create a new market client with proper User-Agent
    pub fn new() -> Self {
        Self {
            http_client: Client::builder()
                .user_agent("TraderGrader/0.1.0 (https://github.com/fuuijin/tradergrader)")
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    /// Fetch current market orders for a region and optional item type
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

    /// Fetch historical market data for a specific item in a region
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

    /// Generate a market summary with buy/sell analysis
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

    /// Analyze price trends from historical data
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

    /// Get a formatted price history summary
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

