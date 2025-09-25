mod models;
mod queries;
mod error;

pub use models::*;
pub use queries::*;
pub use error::*;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;
use std::sync::OnceLock;
use std::time::Duration;

// Global database pool
static DB_POOL: OnceLock<Pool<Postgres>> = OnceLock::new();

/// Initialize the database connection pool
pub async fn init_db_pool() -> Result<&'static Pool<Postgres>, DbError> {
    if DB_POOL.get().is_some() {
        return Ok(DB_POOL.get().unwrap());
    }

    let database_url = env::var("DATABASE_URL").map_err(|_| {
        DbError::Configuration("DATABASE_URL environment variable not set".to_string())
    })?;

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
        .map_err(|e| DbError::Connection(e.to_string()))?;

    // Test the connection
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

    DB_POOL.get_or_init(|| pool);
    Ok(DB_POOL.get().unwrap())
}

/// Get the database pool
pub async fn get_db_pool() -> Result<&'static Pool<Postgres>, DbError> {
    match DB_POOL.get() {
        Some(pool) => Ok(pool),
        None => init_db_pool().await,
    }
}

/// Check if the database is connected
pub async fn is_db_connected() -> bool {
    if let Some(pool) = DB_POOL.get() {
        match sqlx::query("SELECT 1").execute(pool).await {
            Ok(_) => true,
            Err(_) => false,
        }
    } else {
        false
    }
}

/// Close the database connection pool
pub async fn close_db_pool() {
    if let Some(pool) = DB_POOL.get() {
        pool.close().await;
    }
}
