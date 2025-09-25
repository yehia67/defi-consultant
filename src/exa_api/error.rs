use thiserror::Error;

/// Custom error types for the Exa API client
#[derive(Debug, Error)]
pub enum ExaApiError {
    #[error("API key not found")]
    ApiKeyNotFound,
    
    #[error("API request failed: {0}")]
    RequestFailed(String),
    
    #[error("Failed to parse response: {0}")]
    ParseError(#[from] serde_json::Error),
    
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

// No need for a custom From implementation since thiserror derives std::error::Error,
// which allows anyhow to automatically convert it
