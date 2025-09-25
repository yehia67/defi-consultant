use crate::investment_chat::InvestmentChatError;
use reqwest::Client;
use serde_json::json;
use std::time::Duration;
use tracing::{debug, error};

/// Get AI response using Anthropic API
pub async fn get_ai_response(prompt: &str, api_key: &str) -> Result<String, InvestmentChatError> {
    debug!("Preparing AI request with prompt length: {}", prompt.len());
    
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| InvestmentChatError::Internal(format!("Failed to build HTTP client: {}", e)))?;
    
    // Create system prompt that enables the AI to handle all functionality
    let system_prompt = "You are Nova, a crypto investment advisor with expertise in blockchain, DeFi, NFTs, and crypto markets. \
        You can research projects, analyze market trends, provide investment advice, and explain complex crypto concepts. \
        When asked about specific projects, provide detailed information about their technology, tokenomics, team, \
        recent developments, and investment potential. Include both strengths and risks in your analysis. \
        If the user asks about prices, trading, or portfolio management, provide thoughtful advice while being clear \
        about market uncertainties. Always be helpful, concise, and focused on providing value to the user.";
    
    let request_body = json!({
        "model": "claude-3-opus-20240229",
        "max_tokens": 2048,
        "system": system_prompt,
        "messages": [
            {
                "role": "user",
                "content": prompt
            }
        ]
    });
    
    debug!("Sending request to Anthropic API");
    let response_result = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&request_body)
        .send()
        .await;
        
    // Enhanced error handling for network/request errors
    let response = match response_result {
        Ok(resp) => resp,
        Err(e) => {
            let detailed_error = if e.is_timeout() {
                format!("API request timed out: {}", e)
            } else if e.is_connect() {
                format!("Connection error: {}. Please check your internet connection and API endpoint.", e)
            } else if e.is_status() {
                format!("HTTP status error: {}", e)
            } else {
                format!("API request failed: {}", e)
            };
            
            error!("Anthropic API error: {}", detailed_error);
            return Err(InvestmentChatError::AnthropicApi(detailed_error));
        }
    };
        
    // Enhanced error handling for HTTP status errors
    if !response.status().is_success() {
        let status = response.status();
        
        // Try to get error details from response body
        let error_body = match response.text().await {
            Ok(body) => {
                // Try to parse as JSON for better error details
                match serde_json::from_str::<serde_json::Value>(&body) {
                    Ok(json_body) => {
                        if let Some(error_msg) = json_body.get("error").and_then(|e| e.get("message")).and_then(|m| m.as_str()) {
                            format!("API error message: {}", error_msg)
                        } else {
                            format!("Response body: {}", body)
                        }
                    },
                    Err(_) => format!("Response body: {}", body)
                }
            },
            Err(_) => "Could not read response body".to_string()
        };
        
        let error_msg = match status.as_u16() {
            401 => format!("Authentication error (401): Invalid API key. Please check your ANTHROPIC_API_KEY. {}", error_body),
            403 => format!("Authorization error (403): Your API key doesn't have permission. {}", error_body),
            429 => format!("Rate limit exceeded (429): Too many requests. Please try again later. {}", error_body),
            500..=599 => format!("Server error ({}): Anthropic API is experiencing issues. Please try again later. {}", status.as_u16(), error_body),
            _ => format!("API returned error status: {} - {}", status, error_body)
        };
        
        error!("Anthropic API error: {}", error_msg);
        return Err(InvestmentChatError::AnthropicApi(error_msg));
    }
    
    debug!("Parsing Anthropic API response");
    let response_json_result = response.json::<serde_json::Value>().await;
    
    // Enhanced error handling for JSON parsing
    let response_json = match response_json_result {
        Ok(json) => json,
        Err(e) => {
            let error_msg = format!("Failed to parse API response: {}. This may indicate an issue with the API or a change in response format.", e);
            error!("Anthropic API error: {}", error_msg);
            return Err(InvestmentChatError::AnthropicApi(error_msg));
        }
    };
    
    // Extract the response text with better error handling
    let response_text = match response_json.get("content")
        .and_then(|content| content.get(0))
        .and_then(|first| first.get("text"))
        .and_then(|text| text.as_str()) {
            Some(text) => text,
            None => {
                let error_msg = format!(
                    "Failed to extract response text. Unexpected response structure: {}",
                    response_json.to_string()
                );
                error!("Anthropic API error: {}", error_msg);
                return Err(InvestmentChatError::AnthropicApi(error_msg));
            }
        };
    
    debug!("Successfully received AI response with length: {}", response_text.len());
    Ok(response_text.to_string())
}
