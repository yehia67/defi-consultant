use thiserror::Error;

/// Database error types
#[derive(Debug, Error)]
pub enum DbError {
    #[error("Database configuration error: {0}")]
    Configuration(String),
    
    #[error("Database connection error: {0}")]
    Connection(String),
    
    #[error("Database query error: {0}")]
    Query(String),
    
    #[error("Database transaction error: {0}")]
    Transaction(String),
    
    #[error("Database pool error: {0}")]
    Pool(String),
    
    #[error("Database record not found: {0}")]
    NotFound(String),
    
    #[error("Database constraint violation: {0}")]
    Constraint(String),
    
    #[error("Database serialization error: {0}")]
    Serialization(String),
}
