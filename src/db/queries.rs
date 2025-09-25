use super::{DbError, User, Strategy, Knowledge, Message};
use sqlx::{Pool, Postgres, query, query_as};

// User queries
pub async fn get_user_by_username(pool: &Pool<Postgres>, username: &str) -> Result<Option<User>, DbError> {
    query_as::<_, User>("SELECT id, username, wallet_address, created_at, updated_at FROM users WHERE username = $1")
        .bind(username)
        .fetch_optional(pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))
}

pub async fn get_user_by_id(pool: &Pool<Postgres>, user_id: i32) -> Result<Option<User>, DbError> {
    query_as::<_, User>("SELECT id, username, wallet_address, created_at, updated_at FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))
}

pub async fn create_user(pool: &Pool<Postgres>, username: &str, wallet_address: Option<&str>) -> Result<User, DbError> {
    query_as::<_, User>("INSERT INTO users (username, wallet_address) VALUES ($1, $2) RETURNING id, username, wallet_address, created_at, updated_at")
        .bind(username)
        .bind(wallet_address)
        .fetch_one(pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))
}

// Message queries
pub async fn save_message(pool: &Pool<Postgres>, role: &str, content: &str) -> Result<(), DbError> {
    query("INSERT INTO messages (role, content) VALUES ($1, $2)")
        .bind(role)
        .bind(content)
        .execute(pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;
    
    Ok(())
}

pub async fn get_messages(pool: &Pool<Postgres>, limit: i64) -> Result<Vec<Message>, DbError> {
    query_as::<_, Message>("SELECT id, role, content, created_at FROM messages ORDER BY created_at DESC LIMIT $1")
        .bind(limit)
        .fetch_all(pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))
}

// Knowledge queries
pub async fn create_knowledge(
    pool: &Pool<Postgres>,
    user_id: i32,
    source_id: &str,
    content: &str,
    tags: &[String],
) -> Result<Knowledge, DbError> {
    query_as::<_, Knowledge>("INSERT INTO knowledge (user_id, source_id, content, tags) VALUES ($1, $2, $3, $4) RETURNING id, user_id, source_id, content, tags, created_at, updated_at")
        .bind(user_id)
        .bind(source_id)
        .bind(content)
        .bind(tags)
        .fetch_one(pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))
}

pub async fn get_knowledge_by_user_id(pool: &Pool<Postgres>, user_id: i32) -> Result<Vec<Knowledge>, DbError> {
    query_as::<_, Knowledge>("SELECT id, user_id, source_id, content, tags, created_at, updated_at FROM knowledge WHERE user_id = $1")
        .bind(user_id)
        .fetch_all(pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))
}

pub async fn get_knowledge_by_tag(
    pool: &Pool<Postgres>,
    user_id: i32,
    tag: &str,
) -> Result<Vec<Knowledge>, DbError> {
    query_as::<_, Knowledge>("SELECT id, user_id, source_id, content, tags, created_at, updated_at FROM knowledge WHERE user_id = $1 AND $2 = ANY(tags)")
        .bind(user_id)
        .bind(tag)
        .fetch_all(pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))
}

pub async fn get_knowledge_by_tags(
    pool: &Pool<Postgres>,
    user_id: i32,
    tags: &[String],
) -> Result<Vec<Knowledge>, DbError> {
    if tags.is_empty() {
        return Ok(Vec::new());
    }
    
    // Using the array overlap operator
    query_as::<_, Knowledge>("SELECT id, user_id, source_id, content, tags, created_at, updated_at FROM knowledge WHERE user_id = $1 AND tags && $2 ORDER BY updated_at DESC")
        .bind(user_id)
        .bind(tags)
        .fetch_all(pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))
}

// Strategy queries
pub async fn get_strategies_by_user_id(pool: &Pool<Postgres>, user_id: i32) -> Result<Vec<Strategy>, DbError> {
    query_as::<_, Strategy>("SELECT id, user_id, strategy_id, name, category, description, risk_level, tags, steps, requirements, expected_returns, created_at, updated_at, author, version FROM strategies WHERE user_id = $1")
        .bind(user_id)
        .fetch_all(pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))
}

pub async fn create_strategy(
    pool: &Pool<Postgres>,
    user_id: i32,
    strategy_id: &str,
    name: &str,
    category: &str,
    description: &str,
    risk_level: &str,
    tags: &[String],
    steps: &[String],
    requirements: &[String],
    expected_returns: sqlx::types::JsonValue,
    author: &str,
    version: &str,
) -> Result<Strategy, DbError> {
    query_as::<_, Strategy>("INSERT INTO strategies (user_id, strategy_id, name, category, description, risk_level, tags, steps, requirements, expected_returns, author, version) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) RETURNING id, user_id, strategy_id, name, category, description, risk_level, tags, steps, requirements, expected_returns, created_at, updated_at, author, version")
        .bind(user_id)
        .bind(strategy_id)
        .bind(name)
        .bind(category)
        .bind(description)
        .bind(risk_level)
        .bind(tags)
        .bind(steps)
        .bind(requirements)
        .bind(expected_returns)
        .bind(author)
        .bind(version)
        .fetch_one(pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))
}

// Search functions
pub async fn search_strategies_by_text(
    pool: &Pool<Postgres>,
    user_id: i32,
    search_text: &str,
) -> Result<Vec<Strategy>, DbError> {
    let search_pattern = format!("%{}%", search_text);
    
    query_as::<_, Strategy>("SELECT id, user_id, strategy_id, name, category, description, risk_level, tags, steps, requirements, expected_returns, created_at, updated_at, author, version FROM strategies WHERE user_id = $1 AND (name ILIKE $2 OR description ILIKE $2)")
        .bind(user_id)
        .bind(search_pattern)
        .fetch_all(pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))
}

pub async fn search_knowledge_by_text(
    pool: &Pool<Postgres>,
    user_id: i32,
    search_text: &str,
) -> Result<Vec<Knowledge>, DbError> {
    let search_pattern = format!("%{}%", search_text);
    
    query_as::<_, Knowledge>("SELECT id, user_id, source_id, content, tags, created_at, updated_at FROM knowledge WHERE user_id = $1 AND (source_id ILIKE $2 OR content ILIKE $2)")
        .bind(user_id)
        .bind(search_pattern)
        .fetch_all(pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))
}
