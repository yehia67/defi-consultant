use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::config::Config;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

/// Generate a response using the Anthropic API
pub async fn generate_response(messages: &[Message]) -> Result<String> {
    // Try to get the API key from the config first
    let api_key = match Config::get_instance() {
        Ok(config) => config.anthropic_api_key.clone(),
        Err(_) => std::env::var("ANTHROPIC_API_KEY")?,
    };
    
    let client = Client::new();
    
    // Format messages for the Anthropic API
    let formatted_messages: Vec<serde_json::Value> = messages.iter()
        .map(|msg| {
            serde_json::json!({
                "role": msg.role,
                "content": msg.content
            })
        })
        .collect();
    
    let request_body = serde_json::json!({
        "model": "claude-3-opus-20240229",
        "max_tokens": 1024,
        "messages": formatted_messages,
        "system": "You are a helpful AI assistant."
    });
    
    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&request_body)
        .send()
        .await?;
        
    if !response.status().is_success() {
        return Err(anyhow::anyhow!("API request failed with status: {}", response.status()));
    }
    
    let response_json: serde_json::Value = response.json().await?;
    
    // Extract the response text
    let response_text = response_json["content"][0]["text"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Failed to extract response text"))?;
        
    Ok(response_text.to_string())
}
