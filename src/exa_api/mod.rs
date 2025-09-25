mod error;
mod query_builder;
mod models;

pub use error::ExaApiError;
pub use query_builder::QueryBuilder;
pub use models::{ExaSearchResult, ExaSearchResponse};

use crate::config::Config;
use reqwest::Client;
use std::collections::HashSet;
use std::sync::OnceLock;

const EXA_API_BASE_URL: &str = "https://api.exa.ai/api/search";

/// Client for interacting with the Exa API
pub struct ExaApiClient {
    client: Client,
    api_key: String,
}

impl ExaApiClient {
    /// Create a new ExaApiClient using the application config
    /// If the API key is not found, it will use a mock API key
    pub fn new() -> Result<Self, ExaApiError> {
        let api_key = match Config::get_instance() {
            Ok(config) => {
                if config.exa_api_key.is_empty() {
                    "mock_api_key_for_development".to_string()
                } else {
                    config.exa_api_key.clone()
                }
            },
            Err(_) => "mock_api_key_for_development".to_string(),
        };
        
        Ok(Self {
            client: Client::new(),
            api_key,
        })
    }
    
    /// Create a new ExaApiClient with a specific API key
    pub fn with_api_key(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }
    
    /// Search for crypto project information
    pub async fn search_crypto_project(&self, project_name: &str, num_results: usize) -> Result<ExaSearchResponse, ExaApiError> {
        let query = QueryBuilder::new(project_name)
            .add_aspects(&["details", "tokenomics", "technology"])
            .build();
            
        match self.search(&query, num_results, None).await {
            Ok(response) => Ok(response),
            Err(e) => {
                // Log the error
                eprintln!("Exa API error: {}", e);
                
                // Create a basic empty response instead of failing
                Ok(ExaSearchResponse {
                    results: Vec::new(),
                    next_page_id: None,
                })
            }
        }
    }
    
    /// Get technical details about a crypto project
    pub async fn get_technical_details(&self, project_name: &str, num_results: usize) -> Result<ExaSearchResponse, ExaApiError> {
        let query = QueryBuilder::new(project_name)
            .add_aspects(&["blockchain", "technology", "technical", "details", "architecture"])
            .build();
            
        match self.search(&query, num_results, None).await {
            Ok(response) => Ok(response),
            Err(e) => {
                eprintln!("Exa API error getting technical details: {}", e);
                Ok(ExaSearchResponse {
                    results: Vec::new(),
                    next_page_id: None,
                })
            }
        }
    }
    
    /// Get tokenomics information about a crypto project
    pub async fn get_tokenomics(&self, project_name: &str, num_results: usize) -> Result<ExaSearchResponse, ExaApiError> {
        let query = QueryBuilder::new(project_name)
            .add_aspects(&["tokenomics", "supply", "distribution", "inflation", "schedule"])
            .build();
            
        match self.search(&query, num_results, None).await {
            Ok(response) => Ok(response),
            Err(e) => {
                eprintln!("Exa API error getting tokenomics: {}", e);
                Ok(ExaSearchResponse {
                    results: Vec::new(),
                    next_page_id: None,
                })
            }
        }
    }
    
    /// Get team information about a crypto project
    pub async fn get_team_info(&self, project_name: &str, num_results: usize) -> Result<ExaSearchResponse, ExaApiError> {
        let query = QueryBuilder::new(project_name)
            .add_aspects(&["team", "founders", "developers", "background", "experience"])
            .build();
            
        match self.search(&query, num_results, None).await {
            Ok(response) => Ok(response),
            Err(e) => {
                eprintln!("Exa API error getting team info: {}", e);
                Ok(ExaSearchResponse {
                    results: Vec::new(),
                    next_page_id: None,
                })
            }
        }
    }
    
    /// Get recent news about a crypto project
    pub async fn get_recent_news(&self, project_name: &str, num_results: usize) -> Result<ExaSearchResponse, ExaApiError> {
        let query = QueryBuilder::new(project_name)
            .add_aspects(&["recent", "news", "updates", "developments"])
            .build();
            
        match self.search(&query, num_results, None).await {
            Ok(response) => Ok(response),
            Err(e) => {
                eprintln!("Exa API error getting recent news: {}", e);
                Ok(ExaSearchResponse {
                    results: Vec::new(),
                    next_page_id: None,
                })
            }
        }
    }
    
    /// Get investment analysis about a crypto project
    pub async fn get_investment_analysis(&self, project_name: &str, num_results: usize) -> Result<ExaSearchResponse, ExaApiError> {
        let query = QueryBuilder::new(project_name)
            .add_aspects(&["investment", "analysis", "potential", "risks", "opportunities"])
            .build();
            
        match self.search(&query, num_results, None).await {
            Ok(response) => Ok(response),
            Err(e) => {
                eprintln!("Exa API error getting investment analysis: {}", e);
                Ok(ExaSearchResponse {
                    results: Vec::new(),
                    next_page_id: None,
                })
            }
        }
    }
    
    /// Perform a search using the Exa API
    pub async fn search(&self, query: &str, num_results: usize, next_page_id: Option<&str>) -> Result<ExaSearchResponse, ExaApiError> {
        let mut url = format!("{}?query={}&num_results={}", 
            EXA_API_BASE_URL, 
            urlencoding::encode(query), 
            num_results
        );
        
        if let Some(page_id) = next_page_id {
            url.push_str(&format!("&next_page_id={}", page_id));
        }
        
        let response = self.client
            .get(&url)
            .header("x-api-key", &self.api_key)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(ExaApiError::HttpError)?;
        
        if !response.status().is_success() {
            return Err(ExaApiError::RequestFailed(format!("API request failed with status: {}", response.status())));
        }
        
        // Use a direct match to handle the error conversion properly
        let search_response = match response.json::<ExaSearchResponse>().await {
            Ok(response) => response,
            Err(e) => {
                // Convert the reqwest::Error to a string first
                return Err(ExaApiError::RequestFailed(format!("Failed to parse JSON: {}", e)));
            }
        };
            
        Ok(search_response)
    }
    
    /// Extract key insights from search results
    pub fn extract_insights(&self, results: &[ExaSearchResult]) -> Vec<String> {
        let mut insights = Vec::new();
        
        // Use a HashSet for more efficient keyword matching
        static KEYWORDS: OnceLock<HashSet<&'static str>> = OnceLock::new();
        let keywords = KEYWORDS.get_or_init(|| {
            let mut set = HashSet::new();
            set.insert("market cap");
            set.insert("technology");
            set.insert("blockchain");
            set.insert("token");
            set.insert("supply");
            set.insert("founder");
            set.insert("launch");
            set.insert("partnership");
            set
        });
        
        for result in results {
            // Extract sentences that might contain valuable insights
            let content = &result.content;
            let sentences: Vec<&str> = content.split(['.', '!', '?']).collect();
            
            for sentence in sentences {
                let sentence = sentence.trim();
                if sentence.is_empty() {
                    continue;
                }
                
                // Look for sentences with key information using the keyword set
                if keywords.iter().any(|&keyword| sentence.contains(keyword)) {
                    insights.push(format!("{}", sentence));
                }
            }
        }
        
        insights
    }
    
    /// Summarize project information from search results
    pub fn summarize_project(&self, results: &[ExaSearchResult]) -> String {
        if results.is_empty() {
            return "No information found.".to_string();
        }
        
        let mut summary = String::new();
        let insights = self.extract_insights(results);
        
        // Combine insights into a summary
        if !insights.is_empty() {
            summary.push_str("Project Insights:\n\n");
            for (i, insight) in insights.iter().enumerate().take(10) {
                summary.push_str(&format!("{}. {}\n", i + 1, insight));
            }
        } else {
            // If no specific insights were found, use the first result
            summary.push_str(&format!("Summary from {}: {}\n", 
                results[0].url, 
                results[0].content.chars().take(500).collect::<String>()));
        }
        
        summary
    }
}
