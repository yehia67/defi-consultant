use thiserror::Error;
use crate::db::DbError;
use crate::price_fetcher::PriceError;

/// Investment chat error types
#[derive(Debug, Error)]
pub enum InvestmentChatError {
    #[error("Database error: {0}")]
    Database(#[from] DbError),
    
    #[error("Exa API error: {0}")]
    ExaApi(String),
    
    #[error("Price fetcher error: {0}")]
    PriceFetcher(#[from] PriceError),
    
    #[error("Anthropic API error: {0}")]
    AnthropicApi(String),
    
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
    
    #[error("External API error: {0}")]
    ExternalApi(String),
    
    #[error("Price API error: {0}")]
    PriceApi(String),
}
