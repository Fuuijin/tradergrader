//! Caching module for TraderGrader
//!
//! Provides zero-cost in-memory caching with ESI header respect and optional Redis backend.
//! The caching system is designed to reduce ESI API calls while respecting EVE Online's
//! caching guidelines.

use crate::error::Result;
use async_trait::async_trait;
use reqwest::header::{HeaderMap, CACHE_CONTROL};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::fmt::Debug;
use std::time::Duration;

/// Cache key for organizing different types of cached data
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CacheKey {
    /// The type of data being cached (orders, history, summary, analysis)
    pub data_type: String,
    /// Region ID for EVE Online regions
    pub region_id: i32,
    /// Optional type ID for specific items
    pub type_id: Option<i32>,
    /// Additional parameters for complex cache keys
    pub params: Option<String>,
}

impl CacheKey {
    /// Create a new cache key for market orders
    pub fn market_orders(region_id: i32, type_id: Option<i32>) -> Self {
        Self {
            data_type: "orders".to_string(),
            region_id,
            type_id,
            params: None,
        }
    }

    /// Create a new cache key for market history
    pub fn market_history(region_id: i32, type_id: i32) -> Self {
        Self {
            data_type: "history".to_string(),
            region_id,
            type_id: Some(type_id),
            params: None,
        }
    }

    /// Create a new cache key for market summary
    pub fn market_summary(region_id: i32, type_id: i32) -> Self {
        Self {
            data_type: "summary".to_string(),
            region_id,
            type_id: Some(type_id),
            params: None,
        }
    }

    /// Create a new cache key for price analysis
    pub fn price_analysis(region_id: i32, type_id: i32) -> Self {
        Self {
            data_type: "analysis".to_string(),
            region_id,
            type_id: Some(type_id),
            params: None,
        }
    }

    /// Convert cache key to string representation
    pub fn to_string(&self) -> String {
        match (self.type_id, &self.params) {
            (Some(type_id), Some(params)) => {
                format!("tradergrader:{}:{}:{}:{}", self.data_type, self.region_id, type_id, params)
            }
            (Some(type_id), None) => {
                format!("tradergrader:{}:{}:{}", self.data_type, self.region_id, type_id)
            }
            (None, Some(params)) => {
                format!("tradergrader:{}:{}:{}", self.data_type, self.region_id, params)
            }
            (None, None) => {
                format!("tradergrader:{}:{}", self.data_type, self.region_id)
            }
        }
    }
}

/// Cached item with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheItem<T> {
    /// The cached data
    pub data: T,
    /// When this item was cached
    pub cached_at: chrono::DateTime<chrono::Utc>,
    /// How long this item should be cached (from ESI headers)
    pub ttl: Duration,
}

impl<T> CacheItem<T> {
    /// Create a new cache item
    pub fn new(data: T, ttl: Duration) -> Self {
        Self {
            data,
            cached_at: chrono::Utc::now(),
            ttl,
        }
    }

    /// Check if this cache item is still valid
    pub fn is_valid(&self) -> bool {
        let now = chrono::Utc::now();
        let expires_at = self.cached_at + chrono::Duration::from_std(self.ttl).unwrap_or_default();
        now < expires_at
    }

    /// Get remaining TTL for this item
    pub fn remaining_ttl(&self) -> Option<Duration> {
        if self.is_valid() {
            let now = chrono::Utc::now();
            let expires_at = self.cached_at + chrono::Duration::from_std(self.ttl).unwrap_or_default();
            let remaining = expires_at - now;
            remaining.to_std().ok()
        } else {
            None
        }
    }
}

/// Trait for cache backend implementations
#[async_trait]
pub trait CacheBackend: Send + Sync + Debug {
    /// Get raw bytes from the cache
    async fn get_bytes(&self, key: &str) -> Result<Option<Vec<u8>>>;

    /// Set raw bytes in the cache with TTL
    async fn set_bytes(&self, key: &str, data: Vec<u8>, ttl: Duration) -> Result<()>;

    /// Remove an item from the cache
    async fn remove(&self, key: &CacheKey) -> Result<()>;

    /// Clear all items from the cache
    async fn clear(&self) -> Result<()>;

    /// Get cache statistics (hits, misses, size, etc.)
    async fn stats(&self) -> Result<CacheStats>;

    /// Check if the cache backend is healthy
    async fn health_check(&self) -> Result<()>;
}

/// Extension trait for typed cache operations
#[async_trait]
pub trait CacheBackendExt: CacheBackend {
    /// Get an item from the cache with deserialization
    async fn get<T>(&self, key: &CacheKey) -> Result<Option<CacheItem<T>>>
    where
        T: for<'de> Deserialize<'de> + Send,
    {
        let key_str = key.to_string();
        
        if let Some(cached_bytes) = self.get_bytes(&key_str).await? {
            match bincode::deserialize::<CacheItem<T>>(&cached_bytes) {
                Ok(item) => {
                    // Check if item is still valid
                    if item.is_valid() {
                        Ok(Some(item))
                    } else {
                        // Item expired, remove it
                        self.remove(key).await?;
                        Ok(None)
                    }
                }
                Err(_) => {
                    // Deserialization error, remove corrupted item
                    self.remove(key).await?;
                    Ok(None)
                }
            }
        } else {
            Ok(None)
        }
    }

    /// Set an item in the cache with serialization
    async fn set<T>(&self, key: &CacheKey, item: CacheItem<T>) -> Result<()>
    where
        T: Serialize + Send,
    {
        let key_str = key.to_string();
        
        match bincode::serialize(&item) {
            Ok(serialized_bytes) => {
                self.set_bytes(&key_str, serialized_bytes, item.ttl).await
            }
            Err(e) => Err(crate::error::TraderGraderError::CacheError {
                message: format!("Failed to serialize cache item: {}", e)
            }),
        }
    }
}

// Implement the extension trait for all cache backends
impl<T: CacheBackend + ?Sized> CacheBackendExt for T {}

/// Cache statistics for monitoring and debugging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    /// Number of cache hits
    pub hits: u64,
    /// Number of cache misses
    pub misses: u64,
    /// Number of items currently in cache
    pub item_count: u64,
    /// Cache hit ratio (0.0 to 1.0)
    pub hit_ratio: f64,
    /// Backend-specific information
    pub backend_info: String,
}

impl Default for CacheStats {
    fn default() -> Self {
        Self {
            hits: 0,
            misses: 0,
            item_count: 0,
            hit_ratio: 0.0,
            backend_info: "unknown".to_string(),
        }
    }
}

/// ESI header parser for extracting cache directives
pub struct EsiHeaderParser;

impl EsiHeaderParser {
    /// Parse Cache-Control header from ESI response and return TTL
    /// 
    /// ESI typically returns headers like:
    /// - `Cache-Control: public, max-age=300` (cache for 5 minutes)
    /// - `Cache-Control: no-cache` (don't cache)
    /// - Missing header (use fallback TTL)
    pub fn parse_cache_control(headers: &HeaderMap) -> Duration {
        if let Some(cache_control) = headers.get(CACHE_CONTROL) {
            if let Ok(cache_control_str) = cache_control.to_str() {
                return Self::parse_cache_control_string(cache_control_str);
            }
        }
        
        // Fallback TTL when no valid Cache-Control header is found
        Self::default_ttl_for_missing_header()
    }

    /// Parse Cache-Control string and extract max-age value
    fn parse_cache_control_string(cache_control: &str) -> Duration {
        // Split by commas and look for max-age directive
        for directive in cache_control.split(',') {
            let directive = directive.trim();
            
            // Check for no-cache or no-store directives
            if directive == "no-cache" || directive == "no-store" {
                return Duration::from_secs(0); // Don't cache
            }
            
            // Look for max-age=value
            if directive.starts_with("max-age=") {
                if let Some(max_age_str) = directive.strip_prefix("max-age=") {
                    if let Ok(max_age_seconds) = max_age_str.parse::<u64>() {
                        return Duration::from_secs(max_age_seconds);
                    }
                }
            }
        }
        
        // If no max-age found but header exists, use conservative default
        Duration::from_secs(60) // 1 minute conservative default
    }

    /// Default TTL values for different types of ESI data when headers are missing
    pub fn default_ttl_for_missing_header() -> Duration {
        Duration::from_secs(300) // 5 minutes conservative default
    }

    /// Get recommended TTL for specific EVE market data types
    /// These are fallbacks based on known ESI update patterns
    pub fn recommended_ttl_for_data_type(data_type: &str) -> Duration {
        match data_type {
            "orders" => Duration::from_secs(300),    // 5 minutes (ESI updates every ~5min)
            "history" => Duration::from_secs(3600),  // 1 hour (daily updates)
            "summary" => Duration::from_secs(180),   // 3 minutes (derived from orders)
            "analysis" => Duration::from_secs(1800), // 30 minutes (expensive calculations)
            _ => Duration::from_secs(300),           // 5 minutes default
        }
    }

    /// Create a cache item from ESI response with proper TTL calculation
    pub fn create_cache_item_from_response<T>(
        data: T,
        headers: &HeaderMap,
        data_type: &str,
    ) -> CacheItem<T> {
        // Try to get TTL from headers first
        let header_ttl = Self::parse_cache_control(headers);
        
        // If header says don't cache (0 seconds), respect that
        if header_ttl.is_zero() {
            return CacheItem::new(data, Duration::from_secs(0));
        }
        
        // Use header TTL, but apply reasonable bounds
        let recommended_ttl = Self::recommended_ttl_for_data_type(data_type);
        let final_ttl = Self::apply_ttl_bounds(header_ttl, recommended_ttl);
        
        CacheItem::new(data, final_ttl)
    }

    /// Apply reasonable bounds to TTL values to prevent extreme caching
    fn apply_ttl_bounds(header_ttl: Duration, recommended_ttl: Duration) -> Duration {
        let min_ttl = Duration::from_secs(30);  // Never cache for less than 30 seconds
        let max_ttl = Duration::from_secs(3600 * 6); // Never cache for more than 6 hours
        
        // If header TTL is too low, use minimum
        if header_ttl < min_ttl {
            return min_ttl;
        }
        
        // If header TTL is too high, prefer recommended TTL (capped at max)
        if header_ttl > max_ttl {
            return std::cmp::min(recommended_ttl, max_ttl);
        }
        
        // Header TTL is within reasonable bounds, use it
        header_ttl
    }
}

/// Configuration for cache backends
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Whether caching is enabled
    pub enabled: bool,
    /// Maximum number of items to cache
    pub max_capacity: u64,
    /// Default TTL for items without specific TTL
    pub default_ttl: Duration,
    /// Cache backend type
    pub backend_type: CacheBackendType,
}

/// Types of cache backends available
#[derive(Debug, Clone)]
pub enum CacheBackendType {
    /// In-memory cache using moka
    InMemory,
    /// Redis cache (when redis feature is enabled)
    #[cfg(feature = "redis-cache")]
    Redis { connection_string: String },
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_capacity: 1000,
            default_ttl: Duration::from_secs(3600), // 1 hour
            backend_type: CacheBackendType::InMemory,
        }
    }
}

impl CacheConfig {
    /// Create a new cache configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Disable caching
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Self::default()
        }
    }

    /// Configure in-memory cache with custom settings
    pub fn in_memory(max_capacity: u64, default_ttl: Duration) -> Self {
        Self {
            enabled: true,
            max_capacity,
            default_ttl,
            backend_type: CacheBackendType::InMemory,
        }
    }

    /// Configure Redis cache (requires redis feature)
    #[cfg(feature = "redis-cache")]
    pub fn redis(connection_string: String, max_capacity: u64, default_ttl: Duration) -> Self {
        Self {
            enabled: true,
            max_capacity,
            default_ttl,
            backend_type: CacheBackendType::Redis { connection_string },
        }
    }

    /// Create a cache backend from this configuration
    pub fn create_backend(&self) -> Result<Option<Arc<dyn CacheBackend>>> {
        if !self.enabled {
            return Ok(None);
        }

        match &self.backend_type {
            CacheBackendType::InMemory => {
                let backend = InMemoryCacheBackend::new(self.max_capacity, Some(self.default_ttl));
                Ok(Some(Arc::new(backend)))
            }
            #[cfg(feature = "redis-cache")]
            CacheBackendType::Redis { connection_string: _ } => {
                // TODO: Implement Redis backend in Step 7
                Err(crate::error::TraderGraderError::CacheError {
                    message: "Redis backend not implemented yet".to_string()
                })
            }
        }
    }
}

/// In-memory cache backend using moka
#[derive(Debug)]
pub struct InMemoryCacheBackend {
    cache: moka::future::Cache<String, Vec<u8>>,
    stats: std::sync::Arc<std::sync::Mutex<CacheStats>>,
}

impl InMemoryCacheBackend {
    /// Create a new in-memory cache backend
    pub fn new(max_capacity: u64, ttl: Option<Duration>) -> Self {
        let mut builder = moka::future::Cache::builder()
            .max_capacity(max_capacity);
            
        if let Some(ttl) = ttl {
            builder = builder.time_to_live(ttl);
        }
        
        Self {
            cache: builder.build(),
            stats: std::sync::Arc::new(std::sync::Mutex::new(CacheStats {
                hits: 0,
                misses: 0,
                item_count: 0,
                hit_ratio: 0.0,
                backend_info: "in-memory".to_string(),
            })),
        }
    }
    
    /// Create a default in-memory cache with reasonable settings
    pub fn default() -> Self {
        Self::new(
            1000,                           // Max 1000 items
            Some(Duration::from_secs(3600)) // 1 hour default TTL
        )
    }
    
    /// Update cache statistics
    fn update_stats(&self, hit: bool) {
        if let Ok(mut stats) = self.stats.lock() {
            if hit {
                stats.hits += 1;
            } else {
                stats.misses += 1;
            }
            
            stats.item_count = self.cache.entry_count();
            
            let total = stats.hits + stats.misses;
            if total > 0 {
                stats.hit_ratio = stats.hits as f64 / total as f64;
            }
        }
    }
}

#[async_trait]
impl CacheBackend for InMemoryCacheBackend {
    async fn get_bytes(&self, key: &str) -> Result<Option<Vec<u8>>> {
        if let Some(cached_bytes) = self.cache.get(key).await {
            self.update_stats(true);
            Ok(Some(cached_bytes))
        } else {
            self.update_stats(false);
            Ok(None)
        }
    }

    async fn set_bytes(&self, key: &str, data: Vec<u8>, _ttl: Duration) -> Result<()> {
        self.cache.insert(key.to_string(), data).await;
        Ok(())
    }

    async fn remove(&self, key: &CacheKey) -> Result<()> {
        let key_str = key.to_string();
        self.cache.remove(&key_str).await;
        Ok(())
    }

    async fn clear(&self) -> Result<()> {
        self.cache.invalidate_all();
        // Wait for invalidation to complete
        self.cache.run_pending_tasks().await;
        Ok(())
    }

    async fn stats(&self) -> Result<CacheStats> {
        if let Ok(stats) = self.stats.lock() {
            Ok(stats.clone())
        } else {
            Ok(CacheStats::default())
        }
    }

    async fn health_check(&self) -> Result<()> {
        // Simple health check: ensure cache can store and retrieve a test item
        let test_key = CacheKey {
            data_type: "health_check".to_string(),
            region_id: 0,
            type_id: None,
            params: None,
        };
        
        let test_item = CacheItem::new("health_check_data".to_string(), Duration::from_secs(10));
        
        // Try to set and get the test item
        self.set(&test_key, test_item.clone()).await?;
        
        if let Some(retrieved_item) = self.get::<String>(&test_key).await? {
            if retrieved_item.data == test_item.data {
                // Clean up test item
                self.remove(&test_key).await?;
                Ok(())
            } else {
                Err(crate::error::TraderGraderError::CacheError {
                    message: "Health check failed: retrieved data doesn't match".to_string()
                })
            }
        } else {
            Err(crate::error::TraderGraderError::CacheError {
                message: "Health check failed: could not retrieve test item".to_string()
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::{HeaderMap, HeaderValue};

    #[test]
    fn test_cache_key_creation() {
        let orders_key = CacheKey::market_orders(10000002, Some(34));
        assert_eq!(orders_key.data_type, "orders");
        assert_eq!(orders_key.region_id, 10000002);
        assert_eq!(orders_key.type_id, Some(34));

        let history_key = CacheKey::market_history(10000002, 34);
        assert_eq!(history_key.data_type, "history");
        assert_eq!(history_key.type_id, Some(34));
    }

    #[test]
    fn test_cache_key_to_string() {
        let key = CacheKey::market_orders(10000002, Some(34));
        assert_eq!(key.to_string(), "tradergrader:orders:10000002:34");

        let key_no_type = CacheKey::market_orders(10000002, None);
        assert_eq!(key_no_type.to_string(), "tradergrader:orders:10000002");
    }

    #[test]
    fn test_cache_item_validity() {
        let item = CacheItem::new("test_data".to_string(), Duration::from_secs(10));
        assert!(item.is_valid());

        let expired_item = CacheItem {
            data: "test_data".to_string(),
            cached_at: chrono::Utc::now() - chrono::Duration::seconds(20),
            ttl: Duration::from_secs(10),
        };
        assert!(!expired_item.is_valid());
    }

    #[test]
    fn test_esi_header_parser_max_age() {
        let ttl = EsiHeaderParser::parse_cache_control_string("public, max-age=300");
        assert_eq!(ttl, Duration::from_secs(300));

        let ttl = EsiHeaderParser::parse_cache_control_string("max-age=600, public");
        assert_eq!(ttl, Duration::from_secs(600));
    }

    #[test]
    fn test_esi_header_parser_no_cache() {
        let ttl = EsiHeaderParser::parse_cache_control_string("no-cache");
        assert_eq!(ttl, Duration::from_secs(0));

        let ttl = EsiHeaderParser::parse_cache_control_string("public, no-store");
        assert_eq!(ttl, Duration::from_secs(0));
    }

    #[test]
    fn test_esi_header_parser_missing_max_age() {
        let ttl = EsiHeaderParser::parse_cache_control_string("public");
        assert_eq!(ttl, Duration::from_secs(60)); // Conservative default
    }

    #[test]
    fn test_esi_header_parser_with_headers() {
        let mut headers = HeaderMap::new();
        headers.insert(CACHE_CONTROL, HeaderValue::from_static("max-age=300"));
        
        let ttl = EsiHeaderParser::parse_cache_control(&headers);
        assert_eq!(ttl, Duration::from_secs(300));
    }

    #[test]
    fn test_esi_header_parser_missing_headers() {
        let headers = HeaderMap::new();
        let ttl = EsiHeaderParser::parse_cache_control(&headers);
        assert_eq!(ttl, Duration::from_secs(300)); // Default fallback
    }

    #[test]
    fn test_recommended_ttl_for_data_types() {
        assert_eq!(
            EsiHeaderParser::recommended_ttl_for_data_type("orders"),
            Duration::from_secs(300)
        );
        assert_eq!(
            EsiHeaderParser::recommended_ttl_for_data_type("history"),
            Duration::from_secs(3600)
        );
        assert_eq!(
            EsiHeaderParser::recommended_ttl_for_data_type("unknown"),
            Duration::from_secs(300)
        );
    }

    #[test]
    fn test_ttl_bounds() {
        // Test normal case
        let bounded = EsiHeaderParser::apply_ttl_bounds(
            Duration::from_secs(300),
            Duration::from_secs(300),
        );
        assert_eq!(bounded, Duration::from_secs(300));

        // Test too low
        let bounded = EsiHeaderParser::apply_ttl_bounds(
            Duration::from_secs(10),
            Duration::from_secs(300),
        );
        assert_eq!(bounded, Duration::from_secs(30)); // Minimum

        // Test too high - should be capped
        let bounded = EsiHeaderParser::apply_ttl_bounds(
            Duration::from_secs(25000), // > 6 hours (21600)
            Duration::from_secs(300),
        );
        assert_eq!(bounded, Duration::from_secs(300)); // Use recommended
        
        // Test within bounds but high
        let bounded = EsiHeaderParser::apply_ttl_bounds(
            Duration::from_secs(10000), // < 6 hours, so within bounds
            Duration::from_secs(300),
        );
        assert_eq!(bounded, Duration::from_secs(10000)); // Use header TTL
    }

    #[test]
    fn test_create_cache_item_from_response() {
        let mut headers = HeaderMap::new();
        headers.insert(CACHE_CONTROL, HeaderValue::from_static("max-age=300"));
        
        let item = EsiHeaderParser::create_cache_item_from_response(
            "test_data".to_string(),
            &headers,
            "orders",
        );
        
        assert_eq!(item.data, "test_data");
        assert_eq!(item.ttl, Duration::from_secs(300));
        assert!(item.is_valid());
    }

    #[test]
    fn test_create_cache_item_no_cache() {
        let mut headers = HeaderMap::new();
        headers.insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));
        
        let item = EsiHeaderParser::create_cache_item_from_response(
            "test_data".to_string(),
            &headers,
            "orders",
        );
        
        assert_eq!(item.ttl, Duration::from_secs(0));
        assert!(!item.is_valid()); // Should be invalid (don't cache)
    }

    #[tokio::test]
    async fn test_in_memory_cache_backend() {
        let cache = InMemoryCacheBackend::new(100, Some(Duration::from_secs(60)));
        
        // Test key and item
        let key = CacheKey::market_orders(10000002, Some(34));
        let test_data = vec!["test".to_string(), "data".to_string()];
        let item = CacheItem::new(test_data.clone(), Duration::from_secs(30));
        
        // Test set and get
        cache.set(&key, item).await.expect("Should set item");
        
        let retrieved = cache.get::<Vec<String>>(&key).await.expect("Should retrieve item");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().data, test_data);
        
        // Test stats
        let stats = cache.stats().await.expect("Should get stats");
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 0);
        assert!(stats.hit_ratio > 0.0);
        
        // Test remove
        cache.remove(&key).await.expect("Should remove item");
        let retrieved = cache.get::<Vec<String>>(&key).await.expect("Should handle missing item");
        assert!(retrieved.is_none());
        
        // Test miss stats
        let stats = cache.stats().await.expect("Should get stats");
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
    }

    #[tokio::test]
    async fn test_in_memory_cache_expiration() {
        let cache = InMemoryCacheBackend::new(100, None);
        
        let key = CacheKey::market_orders(10000002, Some(34));
        let test_data = "expired_data".to_string();
        
        // Create expired item
        let expired_item = CacheItem {
            data: test_data,
            cached_at: chrono::Utc::now() - chrono::Duration::seconds(60),
            ttl: Duration::from_secs(30), // Expired 30 seconds ago
        };
        
        cache.set(&key, expired_item).await.expect("Should set expired item");
        
        // Should return None for expired item
        let retrieved = cache.get::<String>(&key).await.expect("Should handle expired item");
        assert!(retrieved.is_none());
    }

    #[tokio::test]
    async fn test_in_memory_cache_health_check() {
        let cache = InMemoryCacheBackend::new(100, Some(Duration::from_secs(60)));
        
        // Health check should pass
        cache.health_check().await.expect("Health check should pass");
    }

    #[tokio::test]
    async fn test_in_memory_cache_clear() {
        let cache = InMemoryCacheBackend::new(100, Some(Duration::from_secs(60)));
        
        // Add some items
        let key1 = CacheKey::market_orders(10000002, Some(34));
        let key2 = CacheKey::market_history(10000002, 34);
        
        let item1 = CacheItem::new("data1".to_string(), Duration::from_secs(30));
        let item2 = CacheItem::new("data2".to_string(), Duration::from_secs(30));
        
        cache.set(&key1, item1).await.expect("Should set item1");
        cache.set(&key2, item2).await.expect("Should set item2");
        
        // Clear all items
        cache.clear().await.expect("Should clear cache");
        
        // Items should be gone
        let retrieved1 = cache.get::<String>(&key1).await.expect("Should handle cleared cache");
        let retrieved2 = cache.get::<String>(&key2).await.expect("Should handle cleared cache");
        
        assert!(retrieved1.is_none());
        assert!(retrieved2.is_none());
    }

    #[test]
    fn test_cache_config_default() {
        let config = CacheConfig::default();
        assert!(config.enabled);
        assert_eq!(config.max_capacity, 1000);
        assert_eq!(config.default_ttl, Duration::from_secs(3600));
    }

    #[test]
    fn test_cache_config_disabled() {
        let config = CacheConfig::disabled();
        assert!(!config.enabled);
    }

    #[test]
    fn test_cache_config_in_memory() {
        let config = CacheConfig::in_memory(500, Duration::from_secs(1800));
        assert!(config.enabled);
        assert_eq!(config.max_capacity, 500);
        assert_eq!(config.default_ttl, Duration::from_secs(1800));
    }

    #[tokio::test]
    async fn test_cache_config_create_backend() {
        // Test disabled cache
        let disabled_config = CacheConfig::disabled();
        let backend = disabled_config.create_backend().expect("Should handle disabled cache");
        assert!(backend.is_none());

        // Test in-memory cache
        let in_memory_config = CacheConfig::in_memory(100, Duration::from_secs(300));
        let backend = in_memory_config.create_backend().expect("Should create in-memory backend");
        assert!(backend.is_some());
        
        // Test backend functionality
        if let Some(cache) = backend {
            cache.health_check().await.expect("Health check should pass");
        }
    }
}