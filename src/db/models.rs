use serde::{Deserialize, Serialize};
use sqlx::{types::JsonValue, FromRow, types::chrono::{NaiveDateTime}};

/// User model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub wallet_address: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Strategy model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Strategy {
    pub id: i32,
    pub user_id: i32,
    pub strategy_id: String,
    pub name: String,
    pub category: String,
    pub description: String,
    pub risk_level: String,
    pub tags: Vec<String>,
    pub steps: Vec<String>,
    pub requirements: Vec<String>,
    pub expected_returns: JsonValue,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub author: String,
    pub version: String,
}

/// Knowledge model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Knowledge {
    pub id: i32,
    pub user_id: i32,
    pub source_id: String,
    pub content: String,
    pub tags: Vec<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Data source model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DataSource {
    pub id: i32,
    pub user_id: i32,
    pub source_id: String,
    pub name: String,
    pub description: String,
    pub source_type: String,
    pub refresh_interval_minutes: i32,
    pub config: JsonValue,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub last_refresh: Option<NaiveDateTime>,
}

/// Message model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Message {
    pub id: i32,
    pub role: String,
    pub content: String,
    pub created_at: NaiveDateTime,
}
