use serde::{Deserialize, Serialize};

/// Represents a market order from the EVE ESI API
/// 
/// Contains all information about a buy or sell order in EVE Online's market system,
/// including price, volume, location, and timing details.
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
/// 
/// Contains basic information about a tradeable item type including its unique
/// identifier, name, and optional description.
#[derive(Debug, Deserialize, Serialize)]
pub struct MarketType {
    pub type_id: i32,
    pub name: String,
    pub description: Option<String>,
}

/// Represents a single day of historical market data
/// 
/// Contains daily aggregated market statistics including price ranges,
/// average price, total volume traded, and number of orders.
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
/// 
/// Contains calculated metrics for price movement analysis including
/// short-term and long-term changes, volatility measures, and trend direction.
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_market_order_serialization() {
        let order = MarketOrder {
            duration: 90,
            is_buy_order: true,
            issued: "2025-06-22T10:00:00Z".to_string(),
            location_id: 60003760,
            min_volume: 1,
            order_id: 123456789,
            price: 100.50,
            range: "region".to_string(),
            system_id: 30000142,
            type_id: 34,
            volume_remain: 1000,
            volume_total: 1000,
        };

        // Test serialization
        let json = serde_json::to_string(&order).unwrap();
        assert!(json.contains("is_buy_order"));
        assert!(json.contains("100.5"));

        // Test deserialization
        let deserialized: MarketOrder = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.price, 100.50);
        assert!(deserialized.is_buy_order);
    }

    #[test]
    fn test_market_history_serialization() {
        let history = MarketHistory {
            average: 95.75,
            date: "2025-06-22".to_string(),
            highest: 105.00,
            lowest: 90.00,
            order_count: 150,
            volume: 50000,
        };

        let json = serde_json::to_string(&history).unwrap();
        let deserialized: MarketHistory = serde_json::from_str(&json).unwrap();
        
        assert_eq!(deserialized.average, 95.75);
        assert_eq!(deserialized.order_count, 150);
        assert_eq!(deserialized.date, "2025-06-22");
    }

    #[test]
    fn test_price_analysis_creation() {
        let analysis = PriceAnalysis {
            current_price: 100.0,
            day_change: 5.0,
            day_change_percent: 5.26,
            week_change: -2.0,
            week_change_percent: -1.96,
            month_change: 15.0,
            month_change_percent: 17.65,
            volatility: 12.5,
            trend: "bullish".to_string(),
        };

        assert_eq!(analysis.current_price, 100.0);
        assert_eq!(analysis.trend, "bullish");
        assert!(analysis.day_change > 0.0);
        assert!(analysis.week_change < 0.0);
    }

    #[test]
    fn test_market_type_validation() {
        let market_type = MarketType {
            type_id: 34,
            name: "Tritanium".to_string(),
            description: Some("The most common type of asteroid ore.".to_string()),
        };

        assert_eq!(market_type.type_id, 34);
        assert_eq!(market_type.name, "Tritanium");
        assert!(market_type.description.is_some());
    }
}