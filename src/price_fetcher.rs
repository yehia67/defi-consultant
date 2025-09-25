use reqwest::{self, Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::time::{Duration, Instant};
use std::sync::Mutex;
use once_cell::sync::Lazy;

// Custom error type for price fetcher
#[derive(Debug)]
pub enum PriceError {
    NetworkError(reqwest::Error),
    RateLimitExceeded,
    InvalidResponse(String),
    PriceNotFound(String),
}

impl fmt::Display for PriceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PriceError::NetworkError(e) => write!(f, "Network error: {}", e),
            PriceError::RateLimitExceeded => write!(f, "CoinGecko API rate limit exceeded"),
            PriceError::InvalidResponse(msg) => write!(f, "Invalid API response: {}", msg),
            PriceError::PriceNotFound(coin) => write!(f, "Price not found for {}", coin),
        }
    }
}

impl std::error::Error for PriceError {}

impl From<reqwest::Error> for PriceError {
    fn from(err: reqwest::Error) -> Self {
        PriceError::NetworkError(err)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceResponse {
    #[serde(flatten)]
    pub coins: HashMap<String, HashMap<String, f64>>,
}

// Track API request times to respect rate limits
static LAST_REQUEST: Lazy<Mutex<Option<Instant>>> = Lazy::new(|| Mutex::new(None));

// Minimum time between API requests (milliseconds)
const MIN_REQUEST_INTERVAL_MS: u64 = 1500; // 1.5 seconds between requests

/// Respects rate limits by waiting if needed
async fn respect_rate_limit() {
    if let Ok(last_request) = LAST_REQUEST.lock() {
        if let Some(time) = *last_request {
            let elapsed = time.elapsed();
            let min_interval = Duration::from_millis(MIN_REQUEST_INTERVAL_MS);
            
            if elapsed < min_interval {
                let wait_time = min_interval - elapsed;
                tokio::time::sleep(wait_time).await;
            }
        }
    }
}

/// Fetches the current price of any cryptocurrency in USD
pub async fn fetch_coin_price(coin_id: &str) -> Result<f64, PriceError> {
    // Respect rate limits
    respect_rate_limit().await;
    
    let url = format!("https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd", coin_id);
    
    let client = Client::new();
    let request = client.get(&url);
    
    // Send request with timeout
    let response = request
        .timeout(Duration::from_secs(10))
        .send()
        .await?;
    
    // Check for rate limiting
    if response.status() == StatusCode::TOO_MANY_REQUESTS {
        eprintln!("CoinGecko API rate limit reached. Waiting before retrying.");
        return Err(PriceError::RateLimitExceeded);
    }
    
    // Check for other error status codes
    if !response.status().is_success() {
        eprintln!("CoinGecko API returned error status: {}", response.status());
        return Err(PriceError::InvalidResponse(format!("Status code: {}", response.status())));
    }
    
    // Parse response
    let price_data = response.json::<PriceResponse>().await?;
    
    // Update last request time
    if let Ok(mut last_request) = LAST_REQUEST.lock() {
        *last_request = Some(Instant::now());
    }
    
    // Extract price
    match price_data.coins.get(coin_id) {
        Some(prices) => {
            match prices.get("usd") {
                Some(price) => Ok(*price),
                None => Err(PriceError::PriceNotFound(format!("USD price for {}", coin_id)))
            }
        },
        None => Err(PriceError::PriceNotFound(coin_id.to_string()))
    }
}

/// Fetches the current prices of multiple cryptocurrencies in USD
/// Returns a HashMap with coin_id as key and price as value
pub async fn fetch_multiple_coin_prices(coin_ids: &[&str]) -> Result<HashMap<String, f64>, PriceError> {
    if coin_ids.is_empty() {
        return Ok(HashMap::new());
    }
    
    // Respect rate limits
    respect_rate_limit().await;
    
    let ids = coin_ids.join(",");
    let url = format!("https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd", ids);
    
    let client = Client::new();
    let request = client.get(&url);
    
    // Send request with timeout
    let response = request
        .timeout(Duration::from_secs(10))
        .send()
        .await?;
    
    // Check for rate limiting
    if response.status() == StatusCode::TOO_MANY_REQUESTS {
        eprintln!("CoinGecko API rate limit reached. Waiting before retrying.");
        return Err(PriceError::RateLimitExceeded);
    }
    
    // Check for other error status codes
    if !response.status().is_success() {
        eprintln!("CoinGecko API returned error status: {}", response.status());
        return Err(PriceError::InvalidResponse(format!("Status code: {}", response.status())));
    }
    
    // Parse response
    let price_data = response.json::<PriceResponse>().await?;
    
    // Update last request time
    if let Ok(mut last_request) = LAST_REQUEST.lock() {
        *last_request = Some(Instant::now());
    }
    
    let mut result = HashMap::new();
    for coin_id in coin_ids {
        if let Some(prices) = price_data.coins.get(*coin_id) {
            if let Some(price) = prices.get("usd") {
                result.insert(coin_id.to_string(), *price);
            }
        }
    }
    
    Ok(result)
}

/// Fetches the current price of Aerodrome token in USD (legacy function)
pub async fn fetch_current_price() -> Result<f64, PriceError> {
    fetch_coin_price("aerodrome-finance").await
}

/// Fetches the current price of Ethereum in USD (legacy function)
pub async fn fetch_ethereum_price() -> Result<f64, PriceError> {
    fetch_coin_price("ethereum").await
}

/// Fetches historical price of any cryptocurrency for a specific date
/// Date format should be dd-mm-yyyy (e.g., "01-12-2024")
pub async fn fetch_coin_historical_price(coin_id: &str, date: &str) -> Result<f64, PriceError> {
    // Respect rate limits
    respect_rate_limit().await;
    
    let url = format!(
        "https://api.coingecko.com/api/v3/coins/{}/history?date={}",
        coin_id, date
    );
    
    #[derive(Debug, Deserialize)]
    struct HistoricalResponse {
        market_data: MarketData,
    }
    
    #[derive(Debug, Deserialize)]
    struct MarketData {
        current_price: HashMap<String, f64>,
    }
    
    let client = Client::new();
    let request = client.get(&url);
    
    // Send request with timeout
    let response = request
        .timeout(Duration::from_secs(10))
        .send()
        .await?;
    
    // Check for rate limiting
    if response.status() == StatusCode::TOO_MANY_REQUESTS {
        eprintln!("CoinGecko API rate limit reached. Waiting before retrying.");
        return Err(PriceError::RateLimitExceeded);
    }
    
    // Check for other error status codes
    if !response.status().is_success() {
        eprintln!("CoinGecko API returned error status: {}", response.status());
        return Err(PriceError::InvalidResponse(format!("Status code: {}", response.status())));
    }
    
    // Parse response
    let historical_data = response.json::<HistoricalResponse>().await?;
    
    // Update last request time
    if let Ok(mut last_request) = LAST_REQUEST.lock() {
        *last_request = Some(Instant::now());
    }
    
    // Extract price
    match historical_data.market_data.current_price.get("usd") {
        Some(price) => Ok(*price),
        None => Err(PriceError::PriceNotFound(format!("Historical USD price for {}", coin_id)))
    }
}

/// Fetches historical price of Aerodrome token for a specific date (legacy function)
/// Date format should be dd-mm-yyyy (e.g., "01-12-2024")
pub async fn fetch_historical_price(date: &str) -> Result<f64, PriceError> {
    fetch_coin_historical_price("aerodrome-finance", date).await
}

/// Fetches historical price of Ethereum for a specific date (legacy function)
/// Date format should be dd-mm-yyyy (e.g., "01-12-2024")
pub async fn fetch_ethereum_historical_price(date: &str) -> Result<f64, PriceError> {
    fetch_coin_historical_price("ethereum", date).await
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_fetch_current_price() {
        let price = fetch_current_price().await;
        assert!(price.is_ok());
        let price = price.unwrap();
        assert!(price > 0.0);
        println!("Current AERO price: ${}", price);
    }
    
    #[tokio::test]
    async fn test_fetch_ethereum_price() {
        let price = fetch_ethereum_price().await;
        assert!(price.is_ok());
        let price = price.unwrap();
        assert!(price > 0.0);
        println!("Current ETH price: ${}", price);
    }
    
    #[tokio::test]
    async fn test_fetch_historical_price() {
        let date = "01-12-2024"; // December 1, 2024
        let price = fetch_historical_price(date).await;
        assert!(price.is_ok());
        let price = price.unwrap();
        println!("AERO price on {}: ${}", date, price);
    }
    
    #[tokio::test]
    async fn test_fetch_ethereum_historical_price() {
        let date = "01-12-2024"; // December 1, 2024
        let price = fetch_ethereum_historical_price(date).await;
        assert!(price.is_ok());
        let price = price.unwrap();
        println!("ETH price on {}: ${}", date, price);
    }
    
    #[tokio::test]
    async fn test_fetch_generic_coin_price() {
        // Test with Bitcoin
        let price = fetch_coin_price("bitcoin").await;
        assert!(price.is_ok());
        let price = price.unwrap();
        assert!(price > 0.0);
        println!("Current BTC price: ${}", price);
        
        // Skip Solana test if it fails (API might have rate limits or other issues)
        match fetch_coin_price("solana").await {
            Ok(price) => {
                assert!(price > 0.0);
                println!("Current SOL price: ${}", price);
            },
            Err(e) => {
                println!("Skipping Solana price test due to error: {}", e);
            }
        }
    }
    
    #[tokio::test]
    async fn test_fetch_generic_historical_price() {
        let date = "01-01-2024"; // January 1, 2024
        
        // Test with Bitcoin - skip if it fails (API might have rate limits or other issues)
        match fetch_coin_historical_price("bitcoin", date).await {
            Ok(price) => {
                println!("BTC price on {}: ${}", date, price);
            },
            Err(e) => {
                println!("Skipping Bitcoin historical price test due to error: {}", e);
            }
        };
        
        // Skip Solana test if it fails (API might have rate limits or other issues)
        match fetch_coin_historical_price("solana", date).await {
            Ok(price) => {
                println!("SOL price on {}: ${}", date, price);
            },
            Err(e) => {
                println!("Skipping Solana historical price test due to error: {}", e);
            }
        }
    }
    
    #[tokio::test]
    async fn test_fetch_multiple_coin_prices() {
        let coin_ids = ["bitcoin", "ethereum", "solana", "cardano"];
        let prices = fetch_multiple_coin_prices(&coin_ids).await;
        assert!(prices.is_ok());
        let prices = prices.unwrap();
        
        // Check that we got prices for all requested coins
        for coin_id in coin_ids.iter() {
            assert!(prices.contains_key(*coin_id));
            let price = prices.get(*coin_id).unwrap();
            assert!(*price > 0.0);
            println!("{} price: ${}", coin_id, price);
        }
    }
}
