use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::str::FromStr;
use dotenv::dotenv;
use ethers::{
    prelude::*,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, U256, Chain},
    middleware::SignerMiddleware,
};
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use tokio::sync::Mutex;

// ABI for a simple ERC20 token interface
abigen!(
    IERC20,
    r#"[
        function balanceOf(address account) external view returns (uint256)
        function transfer(address recipient, uint256 amount) external returns (bool)
        function approve(address spender, uint256 amount) external returns (bool)
        function allowance(address owner, address spender) external view returns (uint256)
    ]"#
);

// Enum to represent the type of limit order
#[derive(Debug, Clone, PartialEq)]
pub enum OrderType {
    Buy,
    Sell,
}

// Enum to represent the status of a limit order
#[derive(Debug, Clone, PartialEq)]
pub enum OrderStatus {
    Open,
    Filled,
    Cancelled,
}

// Struct to represent a limit order
#[derive(Debug, Clone)]
pub struct LimitOrder {
    pub id: String,
    pub order_type: OrderType,
    pub token_address: String,
    pub amount: f64,
    pub price: f64,
    pub created_at: DateTime<Utc>,
    pub status: OrderStatus,
}

// In-memory storage for limit orders
lazy_static::lazy_static! {
    static ref LIMIT_ORDERS: Mutex<Vec<LimitOrder>> = Mutex::new(Vec::new());
}

// Token structure for 1inch API responses
#[derive(Debug, Deserialize, Clone)]
pub struct Token {
    pub address: String,
    pub decimals: u32,
    pub symbol: String,
    pub name: String,
    pub logo_uri: Option<String>,
}

// Tokens response from 1inch API
#[derive(Debug, Deserialize)]
pub struct TokensResponse {
    pub tokens: HashMap<String, Token>,
}

// Quote response from 1inch API
#[derive(Debug, Deserialize)]
pub struct QuoteResponse {
    #[serde(rename = "fromToken")]
    pub from_token: Token,
    #[serde(rename = "toToken")]
    pub to_token: Token,
    #[serde(rename = "fromTokenAmount")]
    pub from_amount: String,
    #[serde(rename = "toTokenAmount")]
    pub to_amount: String,
    pub protocols: Vec<Vec<Vec<ProtocolRoute>>>,
    #[serde(rename = "estimatedGas")]
    pub estimated_gas: u64,
}

// Protocol route information
#[derive(Debug, Deserialize)]
pub struct ProtocolRoute {
    pub name: String,
    pub part: u32,
    #[serde(rename = "fromTokenAddress")]
    pub from_token_address: String,
    #[serde(rename = "toTokenAddress")]
    pub to_token_address: String,
}

// Swap response from 1inch API
#[derive(Debug, Deserialize)]
pub struct SwapResponse {
    #[serde(rename = "fromToken")]
    pub from_token: Token,
    #[serde(rename = "toToken")]
    pub to_token: Token,
    #[serde(rename = "fromTokenAmount")]
    pub from_amount: String,
    #[serde(rename = "toTokenAmount")]
    pub to_amount: String,
    pub tx: TransactionData,
}

// Transaction data for swap
#[derive(Debug, Deserialize)]
pub struct TransactionData {
    pub from: String,
    pub to: String,
    pub data: String,
    pub value: String,
    #[serde(rename = "gasPrice")]
    pub gas_price: String,
    pub gas: u64,
}

// 1inch API client
pub struct OneInchClient {
    client: Client,
    base_url: String,
    api_key: Option<String>,
    chain_id: u32,
}

impl OneInchClient {
    pub fn new(chain_id: u32, api_key: Option<String>) -> Self {
        // Chain ID 84532 is Base Sepolia
        let base_url = format!("https://api.1inch.dev/swap/v5.2/{}", chain_id);
        
        Self {
            client: Client::new(),
            base_url,
            api_key,
            chain_id,
        }
    }
    
    /// Fetches all supported tokens on the specified chain
    pub async fn get_tokens(&self) -> Result<TokensResponse, Box<dyn std::error::Error>> {
        let url = format!("{}/tokens", self.base_url);
        
        let mut request = self.client.get(&url);
        if let Some(ref key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }
        
        let response = request.send().await?.json::<TokensResponse>().await?;
        Ok(response)
    }
    
    /// Gets a price quote without executing a trade
    pub async fn get_quote(
        &self,
        src: &str,
        dst: &str,
        amount: &str,
        from: &str
    ) -> Result<QuoteResponse, Box<dyn std::error::Error>> {
        let url = format!("{}/quote", self.base_url);
        
        let mut request = self.client.get(&url)
            .query(&[
                ("src", src),
                ("dst", dst),
                ("amount", amount),
                ("from", from),
            ]);
            
        if let Some(ref key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }
        
        let response = request.send().await?.json::<QuoteResponse>().await?;
        Ok(response)
    }
    
    /// Gets transaction data ready for blockchain submission
    pub async fn get_swap(
        &self,
        src: &str,
        dst: &str,
        amount: &str,
        from: &str,
        slippage: f32,
        disable_estimate: bool
    ) -> Result<SwapResponse, Box<dyn std::error::Error>> {
        let url = format!("{}/swap", self.base_url);
        
        let mut request = self.client.get(&url)
            .query(&[
                ("src", src),
                ("dst", dst),
                ("amount", amount),
                ("from", from),
                ("slippage", slippage.to_string()),
                ("disableEstimate", disable_estimate.to_string()),
            ]);
            
        if let Some(ref key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }
        
        let response = request.send().await?.json::<SwapResponse>().await?;
        Ok(response)
    }
    
    /// Helper function to convert human-readable amounts to blockchain format (wei)
    pub fn to_wei(amount: f64, decimals: u32) -> String {
        let multiplier = 10_u64.pow(decimals) as f64;
        let wei_amount = (amount * multiplier) as u64;
        wei_amount.to_string()
    }
}

// Main trading client that uses 1inch API
pub struct TradingClient {
    wallet: LocalWallet,
    provider: Arc<Provider<Http>>,
    client: Arc<SignerMiddleware<Arc<Provider<Http>>, LocalWallet>>,
    pub one_inch: OneInchClient,
    usdc_address: String,
    weth_address: String,
}

impl TradingClient {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();
        
        let rpc_url = env::var("BASE_SEPOLIA_RPC_URL")?;
        let private_key = env::var("PRIVATE_KEY")?;
        let api_key = env::var("1INCH_API_KEY").ok();
        
        let provider = Provider::<Http>::try_from(rpc_url)?;
        let provider = Arc::new(provider);
        
        let wallet = private_key.parse::<LocalWallet>()?;
        let wallet = wallet.with_chain_id(Chain::BaseSepolia);
        
        // Create a client with the wallet and provider
        let client = Arc::new(SignerMiddleware::new(provider.clone(), wallet.clone()));
        
        // Create 1inch client for Base Sepolia (chain ID 84532)
        let one_inch = OneInchClient::new(84532, api_key);
        
        // Token addresses on Base Sepolia
        // USDC on Base Sepolia
        let usdc_address = "0xf175520c52418dfe19c8098071a252da48cd1c19".to_string();
        // WETH on Base Sepolia
        let weth_address = "0x4200000000000000000000000000000000000006".to_string();
        
        Ok(Self {
            wallet,
            provider,
            client,
            one_inch,
            usdc_address,
            weth_address,
        })
    }
    
    pub async fn get_wallet_address(&self) -> String {
        self.wallet.address().to_string()
    }
    
    /// Get USDC balance
    pub async fn get_usdc_balance(&self) -> Result<f64, Box<dyn std::error::Error>> {
        // Get tokens to find USDC decimals
        let tokens = self.one_inch.get_tokens().await?;
        let usdc_token = tokens.tokens.values().find(|t| t.address.to_lowercase() == self.usdc_address.to_lowercase())
            .ok_or("USDC token not found")?;
        
        // Create ERC20 contract instance
        let address = Address::from_str(&self.usdc_address)?;
        let contract = IERC20::new(address, Arc::clone(&self.client));
        let wallet_address = self.wallet.address();
        
        // Get balance
        let balance = contract.balance_of(wallet_address).call().await?;
        
        // Convert to human-readable format
        let balance_float = balance.as_u128() as f64 / 10_f64.powi(usdc_token.decimals as i32);
        
        Ok(balance_float)
    }
    
    /// Get WETH balance
    pub async fn get_weth_balance(&self) -> Result<f64, Box<dyn std::error::Error>> {
        // Get tokens to find WETH decimals
        let tokens = self.one_inch.get_tokens().await?;
        let weth_token = tokens.tokens.values().find(|t| t.address.to_lowercase() == self.weth_address.to_lowercase())
            .ok_or("WETH token not found")?;
        
        // Create ERC20 contract instance
        let address = Address::from_str(&self.weth_address)?;
        let contract = IERC20::new(address, Arc::clone(&self.client));
        let wallet_address = self.wallet.address();
        
        // Get balance
        let balance = contract.balance_of(wallet_address).call().await?;
        
        // Convert to human-readable format
        let balance_float = balance.as_u128() as f64 / 10_f64.powi(weth_token.decimals as i32);
        
        Ok(balance_float)
    }
    
    /// Create a limit order
    pub async fn create_limit_order(
        &self,
        order_type: OrderType,
        amount: f64,
        price: f64
    ) -> Result<String, Box<dyn std::error::Error>> {
        let id = Uuid::new_v4().to_string();
        let token_address = match order_type {
            OrderType::Buy => self.usdc_address.clone(),
            OrderType::Sell => self.weth_address.clone(),
        };
        
        let order = LimitOrder {
            id: id.clone(),
            order_type,
            token_address,
            amount,
            price,
            created_at: Utc::now(),
            status: OrderStatus::Open,
        };
        
        // Store the order
        let mut orders = LIMIT_ORDERS.lock().await;
        orders.push(order);
        
        let order_type_str = match order_type {
            OrderType::Buy => "buy",
            OrderType::Sell => "sell",
        };
        
        Ok(format!("Created {} limit order for {} tokens at ${} (ID: {})", 
                  order_type_str, amount, price, id))
    }
    
    /// Get all open limit orders
    pub async fn get_open_limit_orders(&self) -> Result<Vec<LimitOrder>, Box<dyn std::error::Error>> {
        let orders = LIMIT_ORDERS.lock().await;
        let open_orders = orders.iter()
            .filter(|order| order.status == OrderStatus::Open)
            .cloned()
            .collect();
        
        Ok(open_orders)
    }
    
    /// Cancel a limit order
    pub async fn cancel_limit_order(&self, order_id: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut orders = LIMIT_ORDERS.lock().await;
        
        if let Some(order) = orders.iter_mut().find(|o| o.id == order_id && o.status == OrderStatus::Open) {
            order.status = OrderStatus::Cancelled;
            Ok(format!("Cancelled order {}", order_id))
        } else {
            Err("Order not found or not open".into())
        }
    }
    
    /// Check and execute limit orders based on current price
    pub async fn check_and_execute_limit_orders(&self) -> Result<String, Box<dyn std::error::Error>> {
        // In a real implementation, this would check current market prices and execute orders
        // For this example, we'll just return a placeholder message
        Ok("Checked limit orders. None ready to execute.".to_string())
    }
    
    /// Analyze trading data and suggest strategies
    pub async fn analyze_and_suggest_strategy(&self) -> Result<String, Box<dyn std::error::Error>> {
        // In a real implementation, this would analyze market data and suggest strategies
        // For this example, we'll just return a placeholder message
        Ok("Market analysis: Consider setting limit orders at key support/resistance levels.".to_string())
    }
    
    /// Execute a trade using 1inch API
    pub async fn execute_trade_strategy(
        &self,
        from_token: &str,
        to_token: &str,
        amount_in_tokens: f64,
        decimals: u32,
        max_slippage: f32,
    ) -> Result<SwapResponse, Box<dyn std::error::Error>> {
        // Convert amount to wei format
        let amount = OneInchClient::to_wei(amount_in_tokens, decimals);
        
        // Get wallet address
        let wallet_address = self.wallet.address().to_string();
        
        // Get swap data
        let swap = self.one_inch.get_swap(
            from_token,
            to_token,
            &amount,
            &wallet_address,
            max_slippage,
            false
        ).await?;
        
        // In a real implementation, we would now sign and submit this transaction
        // For this example, we'll just return the swap response
        
        Ok(swap)
    }
}
