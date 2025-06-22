//! Rate limiting module for ESI API compliance
//!
//! Implements rate limiting to respect EVE Online's ESI API limits:
//! - 100 requests per second global limit
//! - Exponential backoff for rate limit errors
//! - ESI header parsing for remaining quota tracking

use crate::error::{Result, TraderGraderError};
use governor::{Quota, RateLimiter};
use reqwest::{header::HeaderMap, Response, StatusCode};
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

/// ESI API rate limiter configuration
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Requests per second limit (ESI default: 100)
    pub requests_per_second: u32,
    /// Maximum retry attempts for rate limited requests
    pub max_retries: u32,
    /// Base delay for exponential backoff (milliseconds)
    pub base_delay_ms: u64,
    /// Maximum delay between retries (seconds)
    pub max_delay_seconds: u64,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_second: 100, // ESI limit
            max_retries: 3,
            base_delay_ms: 100,
            max_delay_seconds: 30,
        }
    }
}

impl RateLimitConfig {
    /// Create a conservative rate limit configuration
    pub fn conservative() -> Self {
        Self {
            requests_per_second: 50, // Half of ESI limit for safety
            max_retries: 5,
            base_delay_ms: 200,
            max_delay_seconds: 60,
        }
    }

    /// Create a configuration for testing (higher limits)
    pub fn testing() -> Self {
        Self {
            requests_per_second: 1000, // No real limiting for tests
            max_retries: 1,
            base_delay_ms: 10,
            max_delay_seconds: 1,
        }
    }
}

/// ESI rate limiter that respects API quotas and handles errors
#[derive(Debug)]
pub struct EsiRateLimiter {
    limiter: Arc<RateLimiter<governor::state::direct::NotKeyed, governor::state::InMemoryState, governor::clock::DefaultClock>>,
    config: RateLimitConfig,
}

impl EsiRateLimiter {
    /// Create a new ESI rate limiter with configuration
    pub fn new(config: RateLimitConfig) -> Result<Self> {
        let quota = Quota::per_second(
            NonZeroU32::new(config.requests_per_second)
                .ok_or_else(|| TraderGraderError::InternalError(
                    "Rate limit must be greater than 0".to_string()
                ))?
        );
        
        let limiter = RateLimiter::direct(quota);
        
        Ok(Self {
            limiter: Arc::new(limiter),
            config,
        })
    }

    /// Create a default ESI rate limiter
    pub fn default() -> Result<Self> {
        Self::new(RateLimitConfig::default())
    }

    /// Wait for rate limit permission before making a request
    pub async fn acquire(&self) -> Result<()> {
        self.limiter.until_ready().await;
        Ok(())
    }

    /// Get the rate limit configuration
    pub fn config(&self) -> &RateLimitConfig {
        &self.config
    }

    /// Check if we should retry a request based on response status and headers
    pub fn should_retry(&self, status: StatusCode, attempt: u32) -> bool {
        if attempt >= self.config.max_retries {
            return false;
        }

        match status {
            StatusCode::TOO_MANY_REQUESTS => true,
            StatusCode::SERVICE_UNAVAILABLE => true,
            StatusCode::BAD_GATEWAY => true,
            StatusCode::GATEWAY_TIMEOUT => true,
            _ => false,
        }
    }

    /// Calculate delay for exponential backoff
    pub fn calculate_backoff_delay(&self, attempt: u32) -> Duration {
        let delay_ms = self.config.base_delay_ms.saturating_mul(2_u64.saturating_pow(attempt));
        let max_delay_ms = self.config.max_delay_seconds * 1000;
        
        Duration::from_millis(delay_ms.min(max_delay_ms))
    }

    /// Parse ESI rate limit headers from response
    pub fn parse_rate_limit_headers(&self, headers: &HeaderMap) -> EsiRateLimitInfo {
        let remaining = headers
            .get("x-esi-error-limit-remain")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u32>().ok());

        let reset_time = headers
            .get("x-esi-error-limit-reset")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok())
            .map(Duration::from_secs);

        let retry_after = headers
            .get("retry-after")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok())
            .map(Duration::from_secs);

        EsiRateLimitInfo {
            remaining,
            reset_time,
            retry_after,
        }
    }

    /// Execute a request with automatic retry and rate limiting
    pub async fn execute_with_retry<F, Fut>(&self, request_fn: F) -> Result<Response>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<Response>>,
    {
        let mut attempt = 0;

        loop {
            // Wait for rate limit permission
            self.acquire().await?;

            // Execute the request
            let response = request_fn().await?;
            let status = response.status();

            // If successful, return response
            if status.is_success() {
                return Ok(response);
            }

            // Check if we should retry
            if !self.should_retry(status, attempt) {
                return Ok(response); // Return the error response for caller to handle
            }

            // Parse rate limit info for smarter backoff
            let rate_limit_info = self.parse_rate_limit_headers(response.headers());
            
            // Calculate delay (prefer retry-after header if present)
            let delay = if let Some(retry_after) = rate_limit_info.retry_after {
                retry_after
            } else {
                self.calculate_backoff_delay(attempt)
            };

            // Log retry attempt (in production, you'd use proper logging)
            eprintln!(
                "ESI request failed with status {}, retrying in {:?} (attempt {})",
                status, delay, attempt + 1
            );

            // Wait before retry
            sleep(delay).await;
            attempt += 1;
        }
    }
}

/// Information extracted from ESI rate limit headers
#[derive(Debug, Clone)]
pub struct EsiRateLimitInfo {
    /// Remaining requests in current window
    pub remaining: Option<u32>,
    /// Time when rate limit window resets
    pub reset_time: Option<Duration>,
    /// Suggested retry delay from server
    pub retry_after: Option<Duration>,
}

impl Default for EsiRateLimitInfo {
    fn default() -> Self {
        Self {
            remaining: None,
            reset_time: None,
            retry_after: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    async fn test_rate_limiter_creation() {
        let config = RateLimitConfig::default();
        let limiter = EsiRateLimiter::new(config).expect("Should create rate limiter");
        assert_eq!(limiter.config.requests_per_second, 100);
    }

    #[tokio::test]
    async fn test_rate_limiter_acquire() {
        let config = RateLimitConfig::testing(); // High limit for fast test
        let limiter = EsiRateLimiter::new(config).expect("Should create rate limiter");
        
        // Should be able to acquire immediately
        let start = Instant::now();
        limiter.acquire().await.expect("Should acquire");
        let elapsed = start.elapsed();
        
        // Should be very fast for first request
        assert!(elapsed < Duration::from_millis(10));
    }

    #[test]
    fn test_should_retry() {
        let config = RateLimitConfig::default();
        let limiter = EsiRateLimiter::new(config).expect("Should create rate limiter");
        
        // Should retry on rate limit
        assert!(limiter.should_retry(StatusCode::TOO_MANY_REQUESTS, 0));
        assert!(limiter.should_retry(StatusCode::SERVICE_UNAVAILABLE, 1));
        
        // Should not retry on success
        assert!(!limiter.should_retry(StatusCode::OK, 0));
        assert!(!limiter.should_retry(StatusCode::NOT_FOUND, 0));
        
        // Should not retry after max attempts
        assert!(!limiter.should_retry(StatusCode::TOO_MANY_REQUESTS, 5));
    }

    #[test]
    fn test_backoff_delay_calculation() {
        let config = RateLimitConfig {
            base_delay_ms: 100,
            max_delay_seconds: 5,
            ..RateLimitConfig::default()
        };
        let limiter = EsiRateLimiter::new(config).expect("Should create rate limiter");
        
        // Test exponential backoff
        assert_eq!(limiter.calculate_backoff_delay(0), Duration::from_millis(100));
        assert_eq!(limiter.calculate_backoff_delay(1), Duration::from_millis(200));
        assert_eq!(limiter.calculate_backoff_delay(2), Duration::from_millis(400));
        
        // Test max delay cap
        let long_delay = limiter.calculate_backoff_delay(10);
        assert!(long_delay <= Duration::from_secs(5));
    }

    #[test]
    fn test_rate_limit_header_parsing() {
        let config = RateLimitConfig::default();
        let limiter = EsiRateLimiter::new(config).expect("Should create rate limiter");
        
        let mut headers = HeaderMap::new();
        headers.insert("x-esi-error-limit-remain", "45".parse().unwrap());
        headers.insert("x-esi-error-limit-reset", "60".parse().unwrap());
        headers.insert("retry-after", "30".parse().unwrap());
        
        let info = limiter.parse_rate_limit_headers(&headers);
        
        assert_eq!(info.remaining, Some(45));
        assert_eq!(info.reset_time, Some(Duration::from_secs(60)));
        assert_eq!(info.retry_after, Some(Duration::from_secs(30)));
    }

    #[test]
    fn test_conservative_config() {
        let config = RateLimitConfig::conservative();
        assert_eq!(config.requests_per_second, 50);
        assert_eq!(config.max_retries, 5);
    }

    #[test]
    fn test_testing_config() {
        let config = RateLimitConfig::testing();
        assert_eq!(config.requests_per_second, 1000);
        assert_eq!(config.max_retries, 1);
    }

    #[test]
    fn test_backoff_delay_overflow_protection() {
        let config = RateLimitConfig {
            base_delay_ms: 1000,
            max_delay_seconds: 5,
            ..RateLimitConfig::default()
        };
        let limiter = EsiRateLimiter::new(config).expect("Should create rate limiter");
        
        // Test with very high attempt count that would overflow without saturating_pow
        let large_attempt = 100;
        let delay = limiter.calculate_backoff_delay(large_attempt);
        
        // Should not panic and should be capped at max_delay
        assert!(delay <= Duration::from_secs(5));
        assert!(delay >= Duration::from_millis(1000)); // At least base delay
    }
}