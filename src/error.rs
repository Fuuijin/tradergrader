//! Error types for TraderGrader MCP server

use thiserror::Error;

/// TraderGrader specific errors
#[derive(Error, Debug)]
pub enum TraderGraderError {
    #[error("EVE ESI API error: {message}")]
    EsiApiError { message: String },
    
    #[error("Invalid region ID: {region_id}")]
    InvalidRegionId { region_id: i32 },
    
    #[error("Invalid type ID: {type_id}")]
    InvalidTypeId { type_id: i32 },
    
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("Cache error: {message}")]
    CacheError { message: String },
    
    #[error("Rate limit exceeded: {0}")]
    RateLimitError(String),
    
    #[error("Authentication error: {0}")]
    AuthenticationError(String),
    
    #[error("Internal server error: {0}")]
    InternalError(String),
}

impl From<String> for TraderGraderError {
    fn from(message: String) -> Self {
        Self::InternalError(message)
    }
}

impl From<&str> for TraderGraderError {
    fn from(message: &str) -> Self {
        Self::InternalError(message.to_string())
    }
}

/// Result type alias for TraderGrader operations
pub type Result<T> = std::result::Result<T, TraderGraderError>;

impl TraderGraderError {
    /// Convert to JSON-RPC error code
    pub fn to_rpc_code(&self) -> i32 {
        match self {
            Self::EsiApiError { .. } => -32603, // Internal error
            Self::InvalidRegionId { .. } => -32602, // Invalid params
            Self::InvalidTypeId { .. } => -32602, // Invalid params
            Self::NetworkError(_) => -32603, // Internal error
            Self::JsonError(_) => -32700, // Parse error
            Self::CacheError { .. } => -32603, // Internal error
            Self::RateLimitError(_) => -32000, // Server error (custom)
            Self::AuthenticationError(_) => -32001, // Server error (custom)
            Self::InternalError(_) => -32603, // Internal error
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_conversions() {
        let error = TraderGraderError::InvalidRegionId { region_id: 999 };
        assert_eq!(error.to_rpc_code(), -32602);
        assert_eq!(error.to_string(), "Invalid region ID: 999");

        let error2 = TraderGraderError::from("Test error");
        assert_eq!(error2.to_rpc_code(), -32603);
    }

    #[test]
    fn test_string_conversion() {
        let error = TraderGraderError::from("Custom error message".to_string());
        assert!(error.to_string().contains("Custom error message"));
        
        let error2 = TraderGraderError::from("String slice error");
        assert!(error2.to_string().contains("String slice error"));
    }

    #[test]
    fn test_esi_api_error() {
        let error = TraderGraderError::EsiApiError {
            message: "Service unavailable".to_string(),
        };
        assert_eq!(error.to_rpc_code(), -32603);
        assert!(error.to_string().contains("EVE ESI API error"));
    }

    #[test] 
    fn test_rate_limit_error() {
        let error = TraderGraderError::RateLimitError("Too many requests".to_string());
        assert_eq!(error.to_rpc_code(), -32000);
        assert!(error.to_string().contains("Rate limit exceeded"));
    }
}