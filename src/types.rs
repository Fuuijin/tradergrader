use serde::{Deserialize, Serialize};

/// Represents a market order from the EVE ESI API
#[derive(Debug, Deserialize, Serialize)]
pub struct MarketOrder {
    pub duration: i32,
    pub is_buy_order: bool,
    pub issued: String,
    pub location_id: i64,
    pub min_volume: i32,
    pub order_id: i64,
    pub price: f64,
    pub range: String,
    pub system_id: i32,
    pub type_id: i32,
    pub volume_remain: i32,
    pub volume_total: i32,
}

/// Represents an item type in EVE Online
#[derive(Debug, Deserialize, Serialize)]
pub struct MarketType {
    pub type_id: i32,
    pub name: String,
    pub description: Option<String>,
}

/// Represents a single day of historical market data
#[derive(Debug, Deserialize, Serialize)]
pub struct MarketHistory {
    pub average: f64,
    pub date: String,
    pub highest: f64,
    pub lowest: f64,
    pub order_count: i64,
    pub volume: i64,
}

/// Comprehensive price analysis including trends and volatility
#[derive(Debug, Serialize)]
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