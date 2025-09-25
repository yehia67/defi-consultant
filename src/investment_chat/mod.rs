mod constants;
mod error;
mod service;

pub use constants::*;
pub use error::*;
pub use service::*;

use crate::db;
use crate::exa_api::ExaApiClient;
use crate::config::Config;
use crate::price_fetcher;
use crate::price_fetcher::PriceError;

use std::sync::Arc;
use sqlx::Pool;
use sqlx::Postgres;
use tokio::sync::Mutex;
use chrono::{Utc, Datelike};
use regex::Regex;

/// Investment Chat Agent that provides conversational interface for crypto investment decisions
pub struct InvestmentChatAgent {
    user_id: i32,
    username: String,
    pool: Arc<Pool<Postgres>>,
    exa_client: Arc<Mutex<ExaApiClient>>,
}

impl InvestmentChatAgent {
    /// Create a new InvestmentChatAgent
    pub async fn new(username: &str) -> Result<Self, InvestmentChatError> {
        // Get database pool
        let pool = db::get_db_pool()
            .await
            .map_err(|e| InvestmentChatError::Database(e))?;
        
        // Get or create user
        let user = match db::get_user_by_username(pool, username).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                db::create_user(pool, username, None)
                    .await
                    .map_err(|e| InvestmentChatError::Database(e))?
            }
            Err(e) => return Err(InvestmentChatError::Database(e)),
        };
        
        // Create Exa API client
        let exa_client = ExaApiClient::new()
            .map_err(|e| InvestmentChatError::ExaApi(format!("{}", e)))?;
        
        Ok(Self {
            user_id: user.id,
            username: username.to_string(),
            pool: Arc::new(pool.clone()),
            exa_client: Arc::new(Mutex::new(exa_client)),
        })
    }
    
    /// Process a user message and generate a response
    pub async fn process_message(&self, user_message: &str) -> Result<String, InvestmentChatError> {
        // Save user message to database
        db::save_message(&self.pool, "user", user_message)
            .await
            .map_err(|e| InvestmentChatError::Database(e))?;
        
        // Check if this is a price query
        if let Some(price_info) = self.handle_price_query(user_message).await? {
            // Save assistant response to database
            db::save_message(&self.pool, "assistant", &price_info)
                .await
                .map_err(|e| InvestmentChatError::Database(e))?;
            
            return Ok(price_info);
        }
        
        // Check if this is a strategy creation request
        if let Some(strategy_response) = self.handle_strategy_creation(user_message).await? {
            // Save assistant response to database
            db::save_message(&self.pool, "assistant", &strategy_response)
                .await
                .map_err(|e| InvestmentChatError::Database(e))?;
            
            return Ok(strategy_response);
        }
        
        // Retrieve recent conversation history (last 10 messages)
        let recent_messages = match self.get_conversation_history(10).await {
            Ok(messages) => messages,
            Err(e) => {
                eprintln!("Error retrieving conversation history: {}", e);
                Vec::new()
            }
        };
        
        // Build context for the AI
        let mut context = String::new();
        
        // Skip research for strategy creation messages
        let message_lower = user_message.to_lowercase();
        let is_strategy_request = 
            (message_lower.contains("save") && message_lower.contains("strategy")) ||
            (message_lower.contains("add") && message_lower.contains("strategy")) ||
            (message_lower.contains("create") && message_lower.contains("strategy")) ||
            (message_lower.contains("store") && message_lower.contains("strategy")) ||
            (message_lower.contains("save") && message_lower.contains("database"));
            
        // Only attempt research if not a strategy request
        if !is_strategy_request {
            // Try to extract project name but don't fail if research fails
            if let Some(project_name) = self.extract_project_name(user_message) {
                // Use existing knowledge if available, don't call API
                let existing_knowledge = match self.get_knowledge_by_tag(&project_name).await {
                    Ok(knowledge) => knowledge,
                    Err(_) => String::new()
                };
                
                if !existing_knowledge.is_empty() {
                    context.push_str(&format!("Research about {}:\n\n{}\n\n", project_name, existing_knowledge));
                }
                // Skip external API calls completely - rely on AI's built-in knowledge
            }
        }
        
        // Get relevant knowledge from database
        let keywords = self.extract_keywords(user_message);
        if !keywords.is_empty() {
            let knowledge = self.get_knowledge_by_keywords(&keywords).await?;
            if !knowledge.is_empty() {
                context.push_str(&format!("Relevant knowledge:\n\n{}\n\n", knowledge));
            }
        }
        
        // Check if this is a planning mode request
        let is_planning_request = message_lower.contains("plan") && 
            (message_lower.contains("investment") || message_lower.contains("strategy") || 
             message_lower.contains("portfolio"));
            
        // Format conversation history for context
        let mut conversation_context = String::new();
        if !recent_messages.is_empty() {
            conversation_context.push_str("RECENT CONVERSATION HISTORY:\n");
            for message in recent_messages {
                conversation_context.push_str(&format!("{}:\n{}", message.role.to_uppercase(), message.content));
                conversation_context.push_str("\n\n");
            }
        }
        
        // Always include planning steps in the response format
        let planning_instructions = "IMPORTANT: Before answering ANY question, you MUST first outline your approach as a numbered list of steps. \n\
        For example:\n\
        PLANNING STEPS:\n\
        1. Research [specific topic] to understand current market conditions\n\
        2. Analyze [specific factors] that might impact the investment\n\
        3. Formulate a strategy based on [specific criteria]\n\n\
        Only AFTER listing these planning steps should you provide your full response.\n\n";
        
        // Construct prompt with context, conversation history, and mode
        let prompt = if is_planning_request {
            format!(
                "You are Nova, a crypto investment advisor in PLANNING MODE. Create a detailed investment plan or strategy based on the user's request.\n\n\
                {}\n\
                When in planning mode, structure your response as follows:\n\
                1. OBJECTIVE: Clearly state the investment goal\n\
                2. STRATEGY OVERVIEW: Provide a high-level summary of the recommended approach\n\
                3. ASSET ALLOCATION: Suggest specific percentage allocations\n\
                4. ENTRY STRATEGY: When and how to enter positions\n\
                5. RISK MANAGEMENT: Stop-losses, position sizing, and risk mitigation\n\
                6. EXIT STRATEGY: When and how to take profits or cut losses\n\
                7. TIMELINE: Expected timeframe for the strategy\n\
                8. MONITORING: Key indicators to watch\n\n\
                {}\n\
                CONTEXT INFORMATION:\n{}\n\nUSER QUERY: {}", 
                planning_instructions,
                conversation_context,
                context, 
                user_message
            )
        } else {
            format!(
                "You are Nova, a crypto investment advisor. Help the user with their investment decisions.\n\n\
                {}\n\
                {}\n\
                CONTEXT INFORMATION:\n{}\n\nUSER QUERY: {}", 
                planning_instructions,
                conversation_context,
                context, 
                user_message
            )
        };
        
        // Use the config to get the API key
        let config = Config::get_instance()
            .map_err(|e| InvestmentChatError::Configuration(format!("{}", e)))?;
        
        // Get AI response
        let response = self.get_ai_response(&prompt, &config.anthropic_api_key).await?;
        
        // Save assistant response to database
        db::save_message(&self.pool, "assistant", &response)
            .await
            .map_err(|e| InvestmentChatError::Database(e))?;
        
        Ok(response)
    }
    
    /// Extract potential crypto project name from user message
    fn extract_project_name(&self, message: &str) -> Option<String> {
        let message_lower = message.to_lowercase();
        
        // Use the constant set of crypto projects
        for &project in constants::crypto_projects().iter() {
            if message_lower.contains(project) {
                return Some(project.to_string());
            }
        }
        
        None
    }
    
    /// Extract keywords from user message for knowledge retrieval
    fn extract_keywords(&self, message: &str) -> Vec<String> {
        let message_lower = message.to_lowercase();
        let mut keywords = Vec::new();
        
        // Use the constant set of investment keywords
        for &keyword in constants::investment_keywords().iter() {
            if message_lower.contains(keyword) {
                keywords.push(keyword.to_string());
            }
        }
        
        keywords
    }
    
    /// Research a crypto project using Exa API
    #[allow(dead_code)]
    async fn research_project(&self, project_name: &str) -> Result<String, InvestmentChatError> {
        // Check if we already have knowledge about this project
        let existing_knowledge = self.get_knowledge_by_tag(project_name).await?;
        if !existing_knowledge.is_empty() {
            return Ok(existing_knowledge);
        }
        
        // Research the project using Exa API
        let exa_client = self.exa_client.lock().await;
        
        // Try to get information from Exa API with error handling
        let response = match exa_client.search_crypto_project(project_name, 5).await {
            Ok(response) => response,
            Err(e) => {
                // Log the error but return a fallback message instead of propagating the error
                eprintln!("Exa API error when researching {}: {}", project_name, e);
                return Ok(format!("I don't have specific research data about {} at the moment. \
                                 Let me provide some general information based on my knowledge.", project_name));
            }
        };
            
        let summary = exa_client.summarize_project(&response.results);
        
        // Only save to database if we got meaningful results
        if summary != "No information found." {
            // Save research to database
            let source_id = format!("{}_research_{}", project_name.to_lowercase().replace(" ", "_"), Utc::now().timestamp());
            let tags = vec![project_name.to_lowercase(), "research".to_string(), "exa_api".to_string()];
            
            // Try to save to database but don't fail if it doesn't work
            if let Err(e) = db::create_knowledge(
                &self.pool,
                self.user_id,
                &source_id,
                &summary,
                &tags,
            ).await {
                eprintln!("Error saving knowledge to database: {}", e);
            }
        }
        
        Ok(summary)
    }
    
    /// Get knowledge from database by tag
    async fn get_knowledge_by_tag(&self, tag: &str) -> Result<String, InvestmentChatError> {
        let tag_lower = tag.to_lowercase();
        let entries = db::get_knowledge_by_tag(&self.pool, self.user_id, &tag_lower)
            .await
            .map_err(|e| InvestmentChatError::Database(e))?;
        
        if entries.is_empty() {
            return Ok(String::new());
        }
        
        // Combine knowledge entries
        let mut combined_knowledge = String::new();
        for (i, entry) in entries.iter().enumerate().take(2) {
            combined_knowledge.push_str(&format!("Knowledge {}: {}\n\n", i + 1, entry.content));
        }
        
        Ok(combined_knowledge)
    }
    
    /// Get knowledge from database by keywords
    async fn get_knowledge_by_keywords(&self, keywords: &[String]) -> Result<String, InvestmentChatError> {
        if keywords.is_empty() {
            return Ok(String::new());
        }
        
        // Use the optimized query that fetches all matching entries in a single database call
        let entries = db::get_knowledge_by_tags(&self.pool, self.user_id, keywords)
            .await
            .map_err(|e| InvestmentChatError::Database(e))?;
        
        if entries.is_empty() {
            return Ok(String::new());
        }
        
        // Format the entries (already sorted and deduplicated by get_knowledge_by_tags)
        let mut combined_knowledge = String::new();
        for (i, entry) in entries.iter().enumerate().take(2) {
            combined_knowledge.push_str(&format!("Knowledge {}: {}\n\n", i + 1, entry.content));
        }
        
        Ok(combined_knowledge)
    }
    
    /// Get AI response using Anthropic API
    async fn get_ai_response(&self, prompt: &str, api_key: &str) -> Result<String, InvestmentChatError> {
        service::get_ai_response(prompt, api_key).await
    }
    
    /// Get recent conversation history from the database
    async fn get_conversation_history(&self, limit: i64) -> Result<Vec<db::Message>, InvestmentChatError> {
        db::get_messages(&self.pool, limit)
            .await
            .map_err(|e| InvestmentChatError::Database(e))
    }
    
    /// Handle strategy creation requests
    async fn handle_strategy_creation(&self, message: &str) -> Result<Option<String>, InvestmentChatError> {
        // Check if the message is a strategy creation request
        let message_lower = message.to_lowercase();
        
        // Check for various ways a user might request to save a strategy
        let is_strategy_request = 
            (message_lower.contains("save") && message_lower.contains("strategy")) ||
            (message_lower.contains("add") && message_lower.contains("strategy")) ||
            (message_lower.contains("create") && message_lower.contains("strategy")) ||
            (message_lower.contains("store") && message_lower.contains("strategy")) ||
            (message_lower.contains("save") && message_lower.contains("database")) ||
            (message_lower.contains("please save this"));
            
        if !is_strategy_request {
            return Ok(None); // Not a strategy creation request
        }
        
        // Extract strategy fields from the message
        let name = self.extract_field(message, "name:");
        let category = self.extract_field(message, "category:");
        let description = self.extract_field(message, "description:");
        let risk_level = self.extract_field(message, "risk level:");
        let author = self.extract_field(message, "author:").unwrap_or_else(|| "User".to_string());
        let version = self.extract_field(message, "version:").unwrap_or_else(|| "1.0".to_string());
        
        // Extract array fields
        let tags = self.extract_array_field(message, "tags:");
        let steps = self.extract_array_field(message, "steps:");
        let requirements = self.extract_array_field(message, "requirements:");
        
        // Extract expected returns as JSON
        let expected_returns = self.extract_json_field(message, "expected returns:");
        
        // Validate required fields
        if name.is_none() || category.is_none() || description.is_none() || risk_level.is_none() {
            return Ok(Some("To add a strategy, please provide at least the following information:\n\nName: [strategy name]\nCategory: [category]\nDescription: [description]\nRisk Level: [low/medium/high]\n\nOptional fields:\nTags: [comma-separated tags]\nSteps: [numbered steps]\nRequirements: [numbered requirements]\nExpected Returns: [JSON object with timeframes]\nAuthor: [author name]\nVersion: [version number]".to_string()));
        }
        
        // Generate a unique strategy ID
        let strategy_id = format!("{}_{}_{}", 
            name.as_ref().unwrap().to_lowercase().replace(" ", "_"),
            self.username.to_lowercase(),
            chrono::Utc::now().timestamp()
        );
        
        // Create JSON for expected returns
        let expected_returns_json = match expected_returns {
            Some(json_str) => {
                match serde_json::from_str::<serde_json::Value>(&json_str) {
                    Ok(json) => json,
                    Err(_) => serde_json::json!({"note": "Not specified"})
                }
            },
            None => serde_json::json!({"note": "Not specified"})
        };
        
        // Create the strategy in the database
        match db::create_strategy(
            &self.pool,
            self.user_id,
            &strategy_id,
            name.as_ref().unwrap(),
            category.as_ref().unwrap(),
            description.as_ref().unwrap(),
            risk_level.as_ref().unwrap(),
            &tags.unwrap_or_else(|| vec!["investment".to_string()]),
            &steps.unwrap_or_default(),
            &requirements.unwrap_or_default(),
            sqlx::types::JsonValue::from(expected_returns_json),
            &author,
            &version,
        ).await {
            Ok(_) => Ok(Some(format!("Strategy '{}' has been successfully added to your investment strategies. You can refer to it in future conversations.", name.unwrap()))),
            Err(e) => Err(InvestmentChatError::Database(e))
        }
    }
    
    /// Extract a field from a message
    fn extract_field(&self, message: &str, field_name: &str) -> Option<String> {
        let field_regex = Regex::new(&format!(r"(?i){}\s*([^\n]+)(?:\n|$)", regex::escape(field_name))).unwrap();
        
        if let Some(caps) = field_regex.captures(message) {
            if let Some(value_match) = caps.get(1) {
                return Some(value_match.as_str().trim().to_string());
            }
        }
        
        None
    }
    
    /// Extract an array field from a message
    fn extract_array_field(&self, message: &str, field_name: &str) -> Option<Vec<String>> {
        let field_value = self.extract_field(message, field_name)?;
        
        // Check if it's a comma-separated list
        if field_value.contains(',') {
            return Some(field_value.split(',').map(|s| s.trim().to_string()).collect());
        }
        
        // Otherwise, look for numbered items in subsequent lines
        let items_regex = Regex::new(&format!(r"(?i){}\s*[^\n]*\n((?:\s*\d+\.\s*[^\n]+\n?)+)", regex::escape(field_name))).unwrap();
        
        if let Some(caps) = items_regex.captures(message) {
            if let Some(items_block) = caps.get(1) {
                let item_regex = Regex::new(r"\s*\d+\.\s*([^\n]+)").unwrap();
                let mut items = Vec::new();
                
                for cap in item_regex.captures_iter(items_block.as_str()) {
                    if let Some(item) = cap.get(1) {
                        items.push(item.as_str().trim().to_string());
                    }
                }
                
                if !items.is_empty() {
                    return Some(items);
                }
            }
        }
        
        // Return the single value as a one-element vector
        Some(vec![field_value])
    }
    
    /// Extract a JSON field from a message
    fn extract_json_field(&self, message: &str, field_name: &str) -> Option<String> {
        let field_value = self.extract_field(message, field_name)?;
        
        // Check if it's already in JSON format
        if field_value.starts_with('{') && field_value.ends_with('}') {
            return Some(field_value);
        }
        
        // Try to parse it as key-value pairs
        let pairs_regex = Regex::new(r#""?([^"]+)"?\s*:\s*"?([^,"]+)"?"#).unwrap();
        let mut json_obj = std::collections::HashMap::new();
        
        for cap in pairs_regex.captures_iter(&field_value) {
            if let (Some(key), Some(value)) = (cap.get(1), cap.get(2)) {
                json_obj.insert(key.as_str().trim(), value.as_str().trim());
            }
        }
        
        if !json_obj.is_empty() {
            if let Ok(json_str) = serde_json::to_string(&json_obj) {
                return Some(json_str);
            }
        }
        
        // Return the raw value
        Some(format!("{{\"value\": \"{}\"}}", field_value))
    }
    
    /// Handle price queries for cryptocurrencies
    async fn handle_price_query(&self, message: &str) -> Result<Option<String>, InvestmentChatError> {
        // Check for price queries using regex - improved pattern to catch more variations
        let price_regex = Regex::new(r"(?i)(?:what(?:'s| is)(?: the)? (?:current |latest |recent )?(?:price|value) (?:of |for )?|how much is|price of|what(?:'s| is)|ethereum price|eth price|btc price|bitcoin price) ?([a-z]+)?(?: now| today| currently|\?|$)").unwrap();
        
        // Also check for direct queries like "ethereum price" or just "what is ethereum"
        let direct_regex = Regex::new(r"(?i)^([a-z]+)(?:\s+(?:price|current price))?$").unwrap();
        
        // Special case for bitcoin and other common cryptos
        let common_crypto_regex = Regex::new(r"(?i)(?:what is|what's)(?: the)? (bitcoin|btc|ethereum|eth|solana|sol|cardano|ada)(?: price| current price)?\??").unwrap();
        
        // Additional pattern for "entering points" or "entry points" queries - generalized for any crypto
        let entry_points_regex = Regex::new(r"(?i)(?:what (?:is|are)|price)(?: the)? (?:entry|entering) points(?: for)? ([a-z]+)(?:\s|from|$)|(?:entry|entering) points(?: for)? ([a-z]+)(?:\s|from|$)|(?:price|prices)(?: for| of)? ([a-z]+)(?: entry| entering| entry points| entering points)(?:\s|from|$)").unwrap();
        
        // Check for historical price queries
        let historical_regex = Regex::new(r"(?i)(?:what was|historical|history|past|previous|what is the historical) (?:the )?(?:price|value) (?:of |for )?([a-z]+) (?:on|at|in) ([0-9]{1,2}[-/][0-9]{1,2}[-/][0-9]{2,4})").unwrap();
        
        // Additional pattern for "historical price" queries without a date
        let historical_general_regex = Regex::new(r"(?i)(?:what is|what's)(?: the)? historical (?:price|value)(?: of| for)? ([a-z]+)").unwrap();
        
        // First check if it's a historical price query with a specific date
        if let Some(caps) = historical_regex.captures(message) {
            if let (Some(crypto_match), Some(date_match)) = (caps.get(1), caps.get(2)) {
                let crypto = crypto_match.as_str().to_lowercase();
                let date_str = date_match.as_str();
                
                // Convert date to the format expected by the API (dd-mm-yyyy)
                let formatted_date = self.format_date_for_api(date_str)?;
                
                // Map common ticker symbols to their full names
                let coin_id = self.map_crypto_name_to_id(&crypto);
                
                // Fetch historical price using the generic function
                match price_fetcher::fetch_coin_historical_price(coin_id, &formatted_date).await {
                    Ok(price) => {
                        // Get current price for comparison
                        let current_price = match price_fetcher::fetch_coin_price(coin_id).await {
                            Ok(p) => p,
                            Err(_) => 0.0,
                        };
                        
                        let price_change = if current_price > 0.0 {
                            let change_pct = ((current_price - price) / price) * 100.0;
                            format!("Since then, the price has changed by {:.2}% to the current price of ${:.2}.", 
                                   change_pct, current_price)
                        } else {
                            "".to_string()
                        };
                        
                        // Determine if this is a major or smaller cryptocurrency for formatting
                        let _is_major = matches!(crypto.as_str(), "bitcoin" | "btc" | "ethereum" | "eth");
                        
                        // Generate insights based on the cryptocurrency
                        let insights = if matches!(crypto.as_str(), "bitcoin" | "btc") {
                            "- Bitcoin has historically shown lower volatility than other cryptocurrencies\n\
                            - Major support levels tend to form at previous cycle lows\n\
                            - Consider dollar-cost averaging rather than lump-sum investments\n\
                            - Historical data suggests accumulating during 30%+ drawdowns from all-time highs"
                        } else if matches!(crypto.as_str(), "ethereum" | "eth") {
                            "- Ethereum has shown moderate volatility compared to smaller cryptocurrencies\n\
                            - Major support levels tend to form at previous cycle lows\n\
                            - Consider dollar-cost averaging rather than lump-sum investments\n\
                            - Historical data suggests accumulating during 30%+ drawdowns from all-time highs"
                        } else {
                            "- Smaller cryptocurrencies typically show higher volatility than Bitcoin or Ethereum\n\
                            - Consider smaller position sizes due to higher risk\n\
                            - Set wider stop losses (15-20%) to account for volatility\n\
                            - Look for accumulation opportunities during market-wide corrections"
                        };
                        
                        let display_name = self.get_display_name(&crypto);
                        let response = format!(
                            "The price of {} on {} was ${:.2}. {}\n\n\
                            Based on historical data, here are some insights:\n\
                            {}",
                            display_name, date_str, price, price_change, insights
                        );
                        return Ok(Some(response));
                    },
                    Err(e) => {
                        // Try to use the Exa API as a fallback for cryptocurrencies not supported by CoinGecko
                        let query = format!("historical price of {} cryptocurrency on {}", crypto, date_str);
                        let exa_client = self.exa_client.lock().await;
                        match exa_client.search(&query, 3, None).await {
                            Ok(response) => {
                                let summary = exa_client.summarize_project(&response.results);
                                return Ok(Some(format!("Based on my research: {}\n\nNote: This information might not be from real-time price data. For more accurate historical data, I recommend checking specialized crypto data providers.", summary)));
                            },
                            Err(_) => {
                                return Err(InvestmentChatError::PriceApi(
                                    format!("Error fetching historical price for {}: {}. Make sure the date format is correct (DD-MM-YYYY) and that the cryptocurrency is supported.", crypto, e)
                                ));
                            }
                        }
                    }
                }
            }
        }
        
        // Check for general historical price queries without a specific date
        if let Some(caps) = historical_general_regex.captures(message) {
            if let Some(crypto_match) = caps.get(1) {
                let crypto = crypto_match.as_str().to_lowercase();
                let coin_id = self.map_crypto_name_to_id(&crypto);
                
                // Use a default date (30 days ago) for general historical queries
                let today = chrono::Utc::now();
                let thirty_days_ago = today - chrono::Duration::days(30);
                let formatted_date = format!("{:02}-{:02}-{}", 
                    thirty_days_ago.day(), thirty_days_ago.month(), thirty_days_ago.year());
                
                // Fetch historical price
                match price_fetcher::fetch_coin_historical_price(coin_id, &formatted_date).await {
                    Ok(price) => {
                        // Get current price for comparison
                        let current_price = match price_fetcher::fetch_coin_price(coin_id).await {
                            Ok(p) => p,
                            Err(_) => 0.0,
                        };
                        
                        let price_change = if current_price > 0.0 && price > 0.0 {
                            let change_pct = ((current_price - price) / price) * 100.0;
                            format!("Since then, the price has changed by {:.2}% to the current price of ${:.2}.", 
                                   change_pct, current_price)
                        } else {
                            "".to_string()
                        };
                        
                        let display_name = self.get_display_name(&crypto);
                        let date_str = format!("{:02}-{:02}-{}", thirty_days_ago.day(), thirty_days_ago.month(), thirty_days_ago.year());
                        
                        let response = format!(
                            "The price of {} one month ago ({}) was ${:.2}. {}\n\n\
                            Historical price data can help identify trends and potential support/resistance levels.",
                            display_name, date_str, price, price_change
                        );
                        return Ok(Some(response));
                    },
                    Err(e) => {
                        eprintln!("Error fetching historical price for {}: {}", crypto, e);
                        // Continue to other price queries
                    }
                }
            }
        }
        
        let mut crypto = String::new();
        
        // Try the main regex first
        if let Some(caps) = price_regex.captures(message) {
            if let Some(crypto_match) = caps.get(1) {
                crypto = crypto_match.as_str().to_lowercase();
            }
        } 
        // Try the common crypto regex
        else if let Some(caps) = common_crypto_regex.captures(message) {
            if let Some(crypto_match) = caps.get(1) {
                crypto = crypto_match.as_str().to_lowercase();
            }
        }
        // Try the entry points regex
        else if let Some(caps) = entry_points_regex.captures(message) {
            if let Some(crypto_match) = caps.get(1) {
                crypto = crypto_match.as_str().to_lowercase();
            } else if let Some(crypto_match) = caps.get(2) {
                crypto = crypto_match.as_str().to_lowercase();
            } else if let Some(crypto_match) = caps.get(3) {
                crypto = crypto_match.as_str().to_lowercase();
            }
        }
        // If no match, try the direct regex
        else if let Some(caps) = direct_regex.captures(message) {
            if let Some(crypto_match) = caps.get(1) {
                crypto = crypto_match.as_str().to_lowercase();
            }
        }
        
        // If we found a crypto name, process it
        if !crypto.is_empty() {
            // Map common ticker symbols to their full names
            let coin_id = self.map_crypto_name_to_id(&crypto);
            
            // Fetch current price using the generic function
            match price_fetcher::fetch_coin_price(coin_id).await {
                Ok(price) => {
                    // Determine if this is a major or smaller cryptocurrency for formatting and volatility settings
                    let is_major = matches!(crypto.as_str(), "bitcoin" | "btc" | "ethereum" | "eth");
                    
                    // Calculate key price levels based on current price and volatility
                    let (support_factor, resistance_factor) = if is_major {
                        (0.92, 1.08) // Less volatile
                    } else {
                        (0.85, 1.15) // More volatile
                    };
                    
                    let strong_support = price * (support_factor - 0.07);
                    let support = price * support_factor;
                    let resistance = price * resistance_factor;
                    let strong_resistance = price * (resistance_factor + 0.07);
                    
                    // Determine stop loss recommendation based on volatility
                    let stop_loss_recommendation = if is_major {
                        "Setting stop losses 5-8% below your entry price"
                    } else {
                        "Setting stop losses 10-15% below your entry price for this more volatile asset"
                    };
                    
                    let display_name = self.get_display_name(&crypto);
                    let price_str = if is_major { format!("${:.2}", price) } else { format!("${:.4}", price) };
                    let support_str = if is_major { format!("${:.2}", support) } else { format!("${:.4}", support) };
                    let strong_support_str = if is_major { format!("${:.2}", strong_support) } else { format!("${:.4}", strong_support) };
                    let resistance_str = if is_major { format!("${:.2}", resistance) } else { format!("${:.4}", resistance) };
                    let strong_resistance_str = if is_major { format!("${:.2}", strong_resistance) } else { format!("${:.4}", strong_resistance) };
                    
                    // Check if this is an entry points query with a more comprehensive check
                    let message_lower = message.to_lowercase();
                    let is_entry_points_query = message_lower.contains("entry points") || 
                                               message_lower.contains("entering points") ||
                                               message_lower.contains("entry point") ||
                                               message_lower.contains("entering point") ||
                                               (message_lower.contains("entry") && message_lower.contains("price")) ||
                                               (message_lower.contains("enter") && message_lower.contains("price")) ||
                                               (message_lower.contains("buy") && message_lower.contains("level")) ||
                                               (message_lower.contains("when") && message_lower.contains("buy")) ||
                                               (message_lower.contains("good") && message_lower.contains("entry"));
                    
                    let response = if is_entry_points_query {
                        // Add additional insights based on the cryptocurrency
                        let entry_insights = if is_major {
                            // For major cryptocurrencies like BTC and ETH
                            format!("MARKET CONTEXT:\n\
                            - {} is a major cryptocurrency with relatively lower volatility compared to smaller altcoins\n\
                            - Major cryptocurrencies tend to lead market trends and have higher liquidity\n\
                            - Historical data shows {} often finds support at previous resistance levels", 
                                display_name, display_name)
                        } else {
                            // For smaller cryptocurrencies
                            format!("MARKET CONTEXT:\n\
                            - {} is a smaller cryptocurrency that may experience higher volatility than Bitcoin or Ethereum\n\
                            - Smaller cryptocurrencies often follow the general trend of Bitcoin but with amplified movements\n\
                            - Consider using smaller position sizes due to potentially higher risk",
                                display_name)
                        };
                        
                        // Calculate additional price levels for more granular entry points
                        let mid_support = (support + strong_support) / 2.0;
                        let mid_support_str = if is_major { format!("${:.2}", mid_support) } else { format!("${:.4}", mid_support) };
                        
                        let mid_resistance = (resistance + strong_resistance) / 2.0;
                        let mid_resistance_str = if is_major { format!("${:.2}", mid_resistance) } else { format!("${:.4}", mid_resistance) };
                        
                        format!(
                            "ENTRY POINTS ANALYSIS FOR {}:\n\n\
                            Current Price: {}\n\n\
                            SUPPORT LEVELS (Potential Entry Points):\n\
                            - Strong support: {} (Excellent entry, high probability of bounce)\n\
                            - Mid support: {} (Very good entry opportunity)\n\
                            - Support: {} (Good entry, moderate probability of bounce)\n\n\
                            RESISTANCE LEVELS (Potential Exit Points):\n\
                            - Resistance: {} (Consider taking partial profits - 25-33%)\n\
                            - Mid resistance: {} (Consider taking additional profits - 25-33%)\n\
                            - Strong resistance: {} (Consider taking significant profits - remaining position)\n\n\
                            {}\n\n\
                            ENTRY STRATEGY RECOMMENDATIONS:\n\
                            1. Dollar-Cost Average (DCA): Split your investment into 4-5 equal parts and buy at regular intervals\n\
                            2. Scaled Entry: Allocate 20% at current price, 30% at {}, and 50% at {}\n\
                            3. Limit Orders: Set buy orders at {}, {}, and {} to automatically purchase on dips\n\n\
                            EXIT STRATEGY RECOMMENDATIONS:\n\
                            1. Scaled Exit: Sell 25% at {}, 25% at {}, and remaining 50% at {}\n\
                            2. Trailing Stop: Set a trailing stop 7-10% below price after breaking {}\n\
                            3. Risk Management: {}\n\n\
                            TIME HORIZON CONSIDERATIONS:\n\
                            - Short-term traders: Focus on tighter ranges between {} and {}\n\
                            - Medium-term investors: Accumulate between {} and {}, sell between {} and {}\n\
                            - Long-term investors: Focus on accumulation at or below {}, consider holding through volatility\n\n\
                            Remember that these are technical levels only. Always consider fundamental factors, on-chain metrics, and overall market conditions before making investment decisions.",
                            display_name.to_uppercase(), price_str,
                            strong_support_str,
                            mid_support_str,
                            support_str,
                            resistance_str,
                            mid_resistance_str,
                            strong_resistance_str,
                            entry_insights,
                            support_str,
                            strong_support_str,
                            support_str,
                            mid_support_str,
                            strong_support_str,
                            resistance_str,
                            mid_resistance_str,
                            strong_resistance_str,
                            resistance_str,
                            stop_loss_recommendation,
                            support_str,
                            resistance_str,
                            mid_support_str,
                            strong_support_str,
                            resistance_str,
                            strong_resistance_str,
                            support_str
                        )
                    } else {
                        format!(
                            "The current price of {} is {}\n\n\
                            Key price levels for {}:\n\
                            - Strong support: {}\n\
                            - Support: {}\n\
                            - Current price: {}\n\
                            - Resistance: {}\n\
                            - Strong resistance: {}\n\n\
                            Based on these levels, consider:\n\
                            - Accumulating at support levels ({} - {})\n\
                            - Taking partial profits at resistance ({} - {})\n\
                            - {}",
                            display_name, price_str,
                            display_name, 
                            strong_support_str,
                            support_str,
                            price_str,
                            resistance_str,
                            strong_resistance_str,
                            support_str,
                            strong_support_str,
                            resistance_str,
                            strong_resistance_str,
                            stop_loss_recommendation
                        )
                    };
                    return Ok(Some(response));
                },
                Err(e) => {
                    // Handle different error types
                    let error_message = match e {
                        PriceError::RateLimitExceeded => {
                            "The CoinGecko API rate limit has been reached. Please try again in a minute.".to_string()
                        },
                        PriceError::PriceNotFound(_) => {
                            format!("Could not find price information for {}. Please check that the cryptocurrency name or ticker is correct.", crypto)
                        },
                        PriceError::InvalidResponse(msg) => {
                            format!("Error from CoinGecko API: {}", msg)
                        },
                        _ => {
                            // Try to use the Exa API as a fallback for cryptocurrencies not supported by CoinGecko
                            let query = format!("current price of {} cryptocurrency", crypto);
                            let exa_client = self.exa_client.lock().await;
                            match exa_client.search(&query, 3, None).await {
                                Ok(response) => {
                                    let summary = exa_client.summarize_project(&response.results);
                                    return Ok(Some(format!("Based on my research: {}\n\nNote: This information might not be real-time. For specific trading levels and recommendations, I recommend checking specialized crypto data providers.", summary)));
                                },
                                Err(_) => {
                                    // Log the error but provide a fallback message
                                    eprintln!("Error fetching price for {}: {}", crypto, e);
                                    format!("I couldn't find real-time price information for {}. Please check that the cryptocurrency name or ticker is correct and try again.", crypto)
                                }
                            }
                        }
                    };
                    
                    return Ok(Some(error_message));
                }
            }
        }
        
        Ok(None) // Not a price query
    }
    
    /// Map common cryptocurrency names and tickers to their CoinGecko IDs
    fn map_crypto_name_to_id<'a>(&self, name: &'a str) -> &'a str {
        match name.to_lowercase().as_str() {
            "btc" | "bitcoin" => "bitcoin",
            "eth" | "ethereum" => "ethereum",
            "sol" | "solana" => "solana",
            "ada" | "cardano" => "cardano",
            "dot" | "polkadot" => "polkadot",
            "doge" | "dogecoin" => "dogecoin",
            "xrp" | "ripple" => "ripple",
            "ltc" | "litecoin" => "litecoin",
            "link" | "chainlink" => "chainlink",
            "uni" | "uniswap" => "uniswap",
            "aave" => "aave",
            "matic" | "polygon" => "matic-network",
            "avax" | "avalanche" => "avalanche-2",
            "aero" | "aerodrome" => "aerodrome-finance",
            _ => name,  // Return as-is for unknown cryptocurrencies
        }
    }
    
    /// Get a display name for a cryptocurrency
    fn get_display_name(&self, name: &str) -> String {
        match name.to_lowercase().as_str() {
            "btc" => "Bitcoin (BTC)".to_string(),
            "bitcoin" => "Bitcoin".to_string(),
            "eth" => "Ethereum (ETH)".to_string(),
            "ethereum" => "Ethereum".to_string(),
            "sol" => "Solana (SOL)".to_string(),
            "solana" => "Solana".to_string(),
            "ada" => "Cardano (ADA)".to_string(),
            "cardano" => "Cardano".to_string(),
            "dot" => "Polkadot (DOT)".to_string(),
            "polkadot" => "Polkadot".to_string(),
            "doge" => "Dogecoin (DOGE)".to_string(),
            "dogecoin" => "Dogecoin".to_string(),
            "xrp" => "XRP".to_string(),
            "ripple" => "XRP (Ripple)".to_string(),
            "ltc" => "Litecoin (LTC)".to_string(),
            "litecoin" => "Litecoin".to_string(),
            "link" => "Chainlink (LINK)".to_string(),
            "chainlink" => "Chainlink".to_string(),
            "uni" => "Uniswap (UNI)".to_string(),
            "uniswap" => "Uniswap".to_string(),
            "aave" => "Aave".to_string(),
            "matic" => "Polygon (MATIC)".to_string(),
            "polygon" => "Polygon".to_string(),
            "avax" => "Avalanche (AVAX)".to_string(),
            "avalanche" => "Avalanche".to_string(),
            "aero" => "Aerodrome (AERO)".to_string(),
            "aerodrome" => "Aerodrome".to_string(),
            _ => name.to_string().chars().next().unwrap().to_uppercase().collect::<String>() + &name[1..],  // Capitalize first letter for unknown cryptocurrencies
        }
    }
    
    /// Format date string to the format expected by the API (dd-mm-yyyy)
    fn format_date_for_api(&self, date_str: &str) -> Result<String, InvestmentChatError> {
        // Try to parse different date formats
        let date_parts: Vec<&str> = date_str.split(|c| c == '-' || c == '/' || c == '.').collect();
        
        if date_parts.len() != 3 {
            return Err(InvestmentChatError::InvalidInput(
                format!("Invalid date format: {}. Please use DD-MM-YYYY format.", date_str)
            ));
        }
        
        // Extract day, month, and year and convert to strings directly
        let (day_str, month_str, year_str) = if date_parts[0].len() == 4 {
            // YYYY-MM-DD format
            let d = date_parts[2].to_string();
            let m = date_parts[1].to_string();
            let y = date_parts[0].to_string();
            (d, m, y)
        } else if date_parts[2].len() == 4 {
            // DD-MM-YYYY format
            let d = date_parts[0].to_string();
            let m = date_parts[1].to_string();
            let y = date_parts[2].to_string();
            (d, m, y)
        } else if date_parts[2].len() == 2 {
            // DD-MM-YY format - assume 20XX for simplicity
            let d = date_parts[0].to_string();
            let m = date_parts[1].to_string();
            let y = format!("20{}", date_parts[2]);
            (d, m, y)
        } else {
            return Err(InvestmentChatError::InvalidInput(
                format!("Invalid date format: {}. Please use DD-MM-YYYY format.", date_str)
            ));
        };
        
        // Pad day and month with leading zeros if needed
        let day_padded = if day_str.len() == 1 { format!("0{}", day_str) } else { day_str };
        let month_padded = if month_str.len() == 1 { format!("0{}", month_str) } else { month_str };
        
        Ok(format!("{}-{}-{}", day_padded, month_padded, year_str))
    }
}