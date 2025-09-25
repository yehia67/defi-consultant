use agent_friend::price_fetcher;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up logging
    println!("Testing CoinGecko API...");
    
    // Test Ethereum price
    match price_fetcher::fetch_coin_price("ethereum").await {
        Ok(price) => {
            println!("Current Ethereum price: ${:.2}", price);
        },
        Err(e) => {
            println!("Error fetching Ethereum price: {}", e);
            
            // Try with API key if available
            if let Ok(_api_key) = env::var("COINGECKO_API_KEY") {
                println!("API key found, trying with API key...");
            } else {
                println!("No API key found. Consider setting COINGECKO_API_KEY environment variable.");
                println!("CoinGecko now requires an API key for most endpoints.");
            }
        }
    }
    
    // Test Bitcoin price as a comparison
    match price_fetcher::fetch_coin_price("bitcoin").await {
        Ok(price) => {
            println!("Current Bitcoin price: ${:.2}", price);
        },
        Err(e) => {
            println!("Error fetching Bitcoin price: {}", e);
        }
    }
    
    // Test multiple coins
    let coins = ["ethereum", "bitcoin", "solana"];
    match price_fetcher::fetch_multiple_coin_prices(&coins).await {
        Ok(prices) => {
            println!("Multiple coin prices:");
            for (coin, price) in prices {
                println!("  {}: ${:.2}", coin, price);
            }
        },
        Err(e) => {
            println!("Error fetching multiple prices: {}", e);
        }
    }
    
    Ok(())
}
