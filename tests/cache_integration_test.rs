//! Integration tests for caching functionality
//!
//! Tests the complete caching workflow including cache hits, misses,
//! TTL expiration, and ESI header respect.

use std::time::Duration;
use tradergrader::{CacheConfig, MarketClient, CacheBackend, CacheBackendExt};

#[tokio::test]
async fn test_cache_integration_workflow() {
    // Create client with short TTL for testing
    let config = CacheConfig::in_memory(100, Duration::from_secs(2));
    let client = MarketClient::with_cache_config(config)
        .expect("Should create client with cache");

    // Note: These are integration tests that would normally call the real ESI API
    // In a real environment, you would mock the HTTP client or use a test server
    // For now, we test the cache structure and configuration
    
    // Verify cache is enabled and configured
    assert!(client.has_cache());
    
    // Test client creation with different configs
    let disabled_client = MarketClient::with_cache_config(CacheConfig::disabled())
        .expect("Should create client without cache");
    assert!(!disabled_client.has_cache());
    
    let default_client = MarketClient::new();
    assert!(default_client.has_cache());
}

#[tokio::test] 
async fn test_cache_backend_functionality() {
    use tradergrader::{CacheKey, CacheItem, CacheBackendExt, InMemoryCacheBackend};
    
    let cache = InMemoryCacheBackend::new(100, Some(Duration::from_secs(60)));
    
    // Test cache operations
    let key = CacheKey::market_orders(10000002, Some(34));
    let test_data = vec!["test_item".to_string()];
    let item = CacheItem::new(test_data.clone(), Duration::from_secs(30));
    
    // Test set and get
    cache.set(&key, item).await.expect("Should cache item");
    let retrieved = cache.get::<Vec<String>>(&key).await.expect("Should retrieve item");
    
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().data, test_data);
    
    // Test cache statistics
    let stats = cache.stats().await.expect("Should get stats");
    assert_eq!(stats.hits, 1);
    assert_eq!(stats.misses, 0);
    assert_eq!(stats.backend_info, "in-memory");
    
    // Test health check
    cache.health_check().await.expect("Health check should pass");
}

#[tokio::test]
async fn test_cache_expiration_behavior() {
    use tradergrader::{CacheKey, CacheItem, CacheBackendExt, InMemoryCacheBackend};
    
    let cache = InMemoryCacheBackend::new(100, None);
    
    let key = CacheKey::market_history(10000002, 34);
    let test_data = "expired_data".to_string();
    
    // Create item that expires immediately
    let expired_item = CacheItem {
        data: test_data,
        cached_at: chrono::Utc::now() - chrono::Duration::seconds(60),
        ttl: Duration::from_secs(30), // Expired 30 seconds ago
    };
    
    cache.set(&key, expired_item).await.expect("Should set expired item");
    
    // Should return None for expired item and remove it
    let retrieved = cache.get::<String>(&key).await.expect("Should handle expiration");
    assert!(retrieved.is_none());
    
    // Verify stats show hit from finding the data, even though it was expired
    // (The cache backend first finds the data, then the extension trait checks expiration)
    let stats = cache.stats().await.expect("Should get stats");
    assert_eq!(stats.hits, 1); // Found the cached bytes, even though expired
    assert_eq!(stats.misses, 0);
}

#[test]
fn test_cache_key_generation() {
    use tradergrader::CacheKey;
    
    // Test different cache key types
    let orders_key = CacheKey::market_orders(10000002, Some(34));
    let history_key = CacheKey::market_history(10000002, 34);
    let summary_key = CacheKey::market_summary(10000002, 34);
    let analysis_key = CacheKey::price_analysis(10000002, 34);
    
    // Keys should be unique for different data types
    assert_ne!(orders_key.to_string(), history_key.to_string());
    assert_ne!(summary_key.to_string(), analysis_key.to_string());
    
    // Keys should be consistent for same parameters
    let orders_key2 = CacheKey::market_orders(10000002, Some(34));
    assert_eq!(orders_key.to_string(), orders_key2.to_string());
    
    // Keys should differ for different parameters
    let orders_key_diff = CacheKey::market_orders(10000003, Some(34));
    assert_ne!(orders_key.to_string(), orders_key_diff.to_string());
}

#[test] 
fn test_esi_header_parsing() {
    use reqwest::header::{HeaderMap, HeaderValue, CACHE_CONTROL};
    use tradergrader::EsiHeaderParser;
    use std::time::Duration;
    
    // Test various Cache-Control scenarios
    let mut headers = HeaderMap::new();
    
    // Test max-age
    headers.insert(CACHE_CONTROL, HeaderValue::from_static("max-age=300"));
    let ttl = EsiHeaderParser::parse_cache_control(&headers);
    assert_eq!(ttl, Duration::from_secs(300));
    
    // Test no-cache
    headers.insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));
    let ttl = EsiHeaderParser::parse_cache_control(&headers);
    assert_eq!(ttl, Duration::from_secs(0));
    
    // Test missing header
    headers.clear();
    let ttl = EsiHeaderParser::parse_cache_control(&headers);
    assert_eq!(ttl, Duration::from_secs(300)); // Default for missing header
    
    // Test create_cache_item_from_response which applies TTL bounds internally
    let item = EsiHeaderParser::create_cache_item_from_response(
        "test_data".to_string(),
        &headers,
        "orders"
    );
    // Should use default TTL for missing header
    assert_eq!(item.ttl, Duration::from_secs(300));
}