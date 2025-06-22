//! Real ESI API integration tests
//!
//! Tests TraderGrader against the actual EVE Online ESI API to validate
//! caching, rate limiting, and error handling in real-world conditions.
//!
//! Note: These tests make real API calls and may be slower. They can be
//! disabled by setting SKIP_ESI_TESTS=1 environment variable.

use std::env;
use std::time::{Duration, Instant};
use tradergrader::{MarketClient, CacheConfig, RateLimitConfig};

/// Check if ESI tests should be skipped
fn should_skip_esi_tests() -> bool {
    env::var("SKIP_ESI_TESTS").unwrap_or_default() == "1"
}

/// Common test regions and types for consistent testing
struct TestData {
    /// The Forge (Jita) - most active trading region
    forge_region_id: i32,
    /// Tritanium - most traded mineral
    tritanium_type_id: i32,
    /// PLEX - valuable item with active trading
    plex_type_id: i32,
}

impl TestData {
    fn new() -> Self {
        Self {
            forge_region_id: 10000002, // The Forge
            tritanium_type_id: 34,     // Tritanium
            plex_type_id: 44992,       // PLEX
        }
    }
}

#[tokio::test]
async fn test_real_esi_market_orders_fetch() {
    if should_skip_esi_tests() {
        println!("Skipping ESI test (SKIP_ESI_TESTS=1)");
        return;
    }

    let client = MarketClient::new();
    let test_data = TestData::new();

    // Test fetching market orders for Tritanium in The Forge
    let result = client
        .fetch_market_orders(test_data.forge_region_id, Some(test_data.tritanium_type_id))
        .await;

    match result {
        Ok(orders) => {
            assert!(!orders.is_empty(), "Should have market orders for Tritanium");
            
            // Validate order structure
            let first_order = &orders[0];
            assert!(first_order.price > 0.0, "Order price should be positive");
            assert!(first_order.volume_remain > 0, "Order should have remaining volume");
            assert_eq!(first_order.type_id, test_data.tritanium_type_id, "Order should be for Tritanium");
            
            println!("✅ Successfully fetched {} market orders for Tritanium", orders.len());
        }
        Err(e) => {
            panic!("Failed to fetch market orders: {}", e);
        }
    }
}

#[tokio::test]
async fn test_real_esi_market_history_fetch() {
    if should_skip_esi_tests() {
        println!("Skipping ESI test (SKIP_ESI_TESTS=1)");
        return;
    }

    let client = MarketClient::new();
    let test_data = TestData::new();

    // Test fetching market history for PLEX in The Forge
    let result = client
        .fetch_market_history(test_data.forge_region_id, test_data.plex_type_id)
        .await;

    match result {
        Ok(history) => {
            assert!(!history.is_empty(), "Should have market history for PLEX");
            
            // Validate history structure
            let recent_day = &history[0];
            assert!(recent_day.average > 0.0, "Average price should be positive");
            assert!(recent_day.highest >= recent_day.average, "Highest should be >= average");
            assert!(recent_day.lowest <= recent_day.average, "Lowest should be <= average");
            assert!(recent_day.volume > 0, "Volume should be positive");
            assert!(!recent_day.date.is_empty(), "Date should not be empty");
            
            println!("✅ Successfully fetched {} days of market history for PLEX", history.len());
        }
        Err(e) => {
            panic!("Failed to fetch market history: {}", e);
        }
    }
}

#[tokio::test]
async fn test_real_esi_caching_behavior() {
    if should_skip_esi_tests() {
        println!("Skipping ESI test (SKIP_ESI_TESTS=1)");
        return;
    }

    // Create client with short-lived cache for testing
    let cache_config = CacheConfig::in_memory(100, Duration::from_secs(300)); // 5 min TTL
    let client = MarketClient::with_cache_config(cache_config)
        .expect("Should create client with cache");
    
    let test_data = TestData::new();

    // First request - should hit ESI API
    let start_time = Instant::now();
    let result1 = client
        .fetch_market_orders(test_data.forge_region_id, Some(test_data.tritanium_type_id))
        .await;
    let first_duration = start_time.elapsed();

    assert!(result1.is_ok(), "First request should succeed");
    let orders1 = result1.unwrap();

    // Second request - should hit cache and be faster
    let start_time = Instant::now();
    let result2 = client
        .fetch_market_orders(test_data.forge_region_id, Some(test_data.tritanium_type_id))
        .await;
    let second_duration = start_time.elapsed();

    assert!(result2.is_ok(), "Second request should succeed");
    let orders2 = result2.unwrap();

    // Validate cache hit
    assert_eq!(orders1.len(), orders2.len(), "Cached response should match original");
    assert!(
        second_duration < first_duration / 2,
        "Cached request should be significantly faster: {:?} vs {:?}",
        second_duration,
        first_duration
    );

    println!(
        "✅ Cache working: API call took {:?}, cache hit took {:?}",
        first_duration, second_duration
    );
}

#[tokio::test]
async fn test_real_esi_rate_limiting() {
    if should_skip_esi_tests() {
        println!("Skipping ESI test (SKIP_ESI_TESTS=1)");
        return;
    }

    // Create client with conservative rate limiting
    let cache_config = CacheConfig::disabled(); // Disable cache to force API calls
    let rate_config = RateLimitConfig::conservative(); // 50 req/sec
    
    let client = MarketClient::with_configs(cache_config, rate_config)
        .expect("Should create client");
    
    let test_data = TestData::new();

    // Make multiple rapid requests to test rate limiting
    let start_time = Instant::now();
    let mut successful_requests = 0;
    
    for i in 0..5 {
        let result = client
            .fetch_market_orders(test_data.forge_region_id, Some(test_data.tritanium_type_id))
            .await;
        
        if result.is_ok() {
            successful_requests += 1;
        }
        
        println!("Request {}: {:?}", i + 1, result.is_ok());
    }
    
    let total_duration = start_time.elapsed();
    
    assert!(successful_requests > 0, "At least some requests should succeed");
    assert!(
        total_duration >= Duration::from_millis(80), // Conservative rate limiting should add delay
        "Rate limiting should add measurable delay: {:?}",
        total_duration
    );

    println!(
        "✅ Rate limiting working: {} successful requests in {:?}",
        successful_requests, total_duration
    );
}

#[tokio::test]
async fn test_real_esi_market_summary() {
    if should_skip_esi_tests() {
        println!("Skipping ESI test (SKIP_ESI_TESTS=1)");
        return;
    }

    let client = MarketClient::new();
    let test_data = TestData::new();

    // Test market summary generation with real data
    let result = client
        .get_market_summary(test_data.forge_region_id, test_data.tritanium_type_id)
        .await;

    match result {
        Ok(summary) => {
            assert!(!summary.is_empty(), "Summary should not be empty");
            assert!(summary.contains("Market Summary"), "Should contain summary header");
            assert!(summary.contains("Total Orders"), "Should contain order count");
            assert!(summary.contains("Buy Orders"), "Should contain buy order count");
            assert!(summary.contains("Sell Orders"), "Should contain sell order count");
            assert!(summary.contains("ISK"), "Should contain prices in ISK");
            
            println!("✅ Generated market summary:\n{}", summary);
        }
        Err(e) => {
            panic!("Failed to generate market summary: {}", e);
        }
    }
}

#[tokio::test]
async fn test_real_esi_price_analysis() {
    if should_skip_esi_tests() {
        println!("Skipping ESI test (SKIP_ESI_TESTS=1)");
        return;
    }

    let client = MarketClient::new();
    let test_data = TestData::new();

    // Test price analysis with real historical data
    let result = client
        .analyze_price_trends(test_data.forge_region_id, test_data.plex_type_id)
        .await;

    match result {
        Ok(analysis) => {
            assert!(analysis.current_price > 0.0, "Current price should be positive");
            assert!(analysis.volatility >= 0.0, "Volatility should be non-negative");
            assert!(!analysis.trend.is_empty(), "Trend should not be empty");
            
            // Validate trend categories
            let valid_trends = ["Strong Upward", "Upward", "Stable", "Downward", "Strong Downward"];
            assert!(
                valid_trends.contains(&analysis.trend.as_str()),
                "Trend should be one of the valid categories: {}",
                analysis.trend
            );
            
            println!("✅ Price analysis for PLEX:");
            println!("   Current Price: {:.2} ISK", analysis.current_price);
            println!("   Daily Change: {:.2}% ({:.2} ISK)", analysis.day_change_percent, analysis.day_change);
            println!("   Weekly Change: {:.2}% ({:.2} ISK)", analysis.week_change_percent, analysis.week_change);
            println!("   Monthly Change: {:.2}% ({:.2} ISK)", analysis.month_change_percent, analysis.month_change);
            println!("   Volatility: {:.2} ISK", analysis.volatility);
            println!("   Trend: {}", analysis.trend);
        }
        Err(e) => {
            panic!("Failed to analyze price trends: {}", e);
        }
    }
}

#[tokio::test]
async fn test_real_esi_error_handling() {
    if should_skip_esi_tests() {
        println!("Skipping ESI test (SKIP_ESI_TESTS=1)");
        return;
    }

    let client = MarketClient::new();

    // Test with invalid region ID
    let result = client.fetch_market_orders(999999, Some(34)).await;
    
    // ESI should return an error for invalid region
    // Our client should handle this gracefully
    match result {
        Ok(_) => {
            println!("⚠️ ESI accepted invalid region ID (API behavior changed?)");
        }
        Err(e) => {
            println!("✅ Properly handled invalid region error: {}", e);
        }
    }

    // Test with invalid type ID
    let result = client.fetch_market_history(10000002, 999999).await;
    
    match result {
        Ok(history) => {
            if history.is_empty() {
                println!("✅ ESI returned empty history for invalid type (expected)");
            } else {
                println!("⚠️ ESI returned data for invalid type ID (unexpected)");
            }
        }
        Err(e) => {
            println!("✅ Properly handled invalid type error: {}", e);
        }
    }
}