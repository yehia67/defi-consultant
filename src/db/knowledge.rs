use sqlx::PgPool;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Knowledge {
    pub id: i32,
    pub user_id: i32,
    pub source_id: String,
    pub content: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create a new knowledge entry
pub async fn create_knowledge(
    pool: &PgPool,
    user_id: i32,
    source_id: &str,
    content: &str,
    tags: &[String],
) -> Result<Knowledge> {
    let knowledge = sqlx::query_as!(
        Knowledge,
        r#"
        INSERT INTO knowledge (user_id, source_id, content, tags)
        VALUES ($1, $2, $3, $4)
        RETURNING id, user_id, source_id, content, tags, created_at, updated_at
        "#,
        user_id,
        source_id,
        content,
        tags as _
    )
    .fetch_one(pool)
    .await?;

    Ok(knowledge)
}

/// Get knowledge entries by tag
pub async fn get_knowledge_by_tag(
    pool: &PgPool,
    user_id: i32,
    tag: &str,
) -> Result<Vec<Knowledge>> {
    let knowledge = sqlx::query_as!(
        Knowledge,
        r#"
        SELECT id, user_id, source_id, content, tags, created_at, updated_at
        FROM knowledge
        WHERE user_id = $1 AND $2 = ANY(tags)
        ORDER BY updated_at DESC
        LIMIT 5
        "#,
        user_id,
        tag
    )
    .fetch_all(pool)
    .await?;

    Ok(knowledge)
}

/// Get knowledge entries by multiple tags (optimized version)
pub async fn get_knowledge_by_tags(
    pool: &PgPool,
    user_id: i32,
    tags: &[String],
) -> Result<Vec<Knowledge>> {
    if tags.is_empty() {
        return Ok(Vec::new());
    }
    
    // This query uses array overlap operator && to find knowledge entries
    // that have any tag in common with the provided tags array
    let knowledge = sqlx::query_as!(
        Knowledge,
        r#"
        SELECT id, user_id, source_id, content, tags, created_at, updated_at
        FROM knowledge
        WHERE user_id = $1 AND tags && $2
        ORDER BY updated_at DESC
        LIMIT 10
        "#,
        user_id,
        tags as _
    )
    .fetch_all(pool)
    .await?;

    Ok(knowledge)
}
