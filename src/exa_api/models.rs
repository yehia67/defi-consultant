use serde::{Deserialize, Serialize};

/// Represents a single search result from the Exa API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExaSearchResult {
    pub id: String,
    pub url: String,
    pub title: String,
    pub content: String,
    pub score: f64,
    pub published_date: Option<String>,
    pub author: Option<String>,
}

/// Represents the response from the Exa API search endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExaSearchResponse {
    pub results: Vec<ExaSearchResult>,
    pub next_page_id: Option<String>,
}
