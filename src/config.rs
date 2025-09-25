use anyhow::{Result, anyhow};
use std::env;

/// Central configuration for the application
/// Loads all environment variables at startup and provides access to them
#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub anthropic_api_key: String,
    pub base_sepolia_rpc_url: String,
    pub private_key: Option<String>,
    pub oneinch_api_key: Option<String>,
    pub exa_api_key: String,
}

impl Config {
    /// Load configuration from environment variables
    /// Uses default values for missing environment variables
    pub fn from_env() -> Result<Self> {
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/postgres".to_string());
            
        let anthropic_api_key = env::var("ANTHROPIC_API_KEY")
            .unwrap_or_else(|_| "mock_anthropic_api_key_for_development".to_string());
            
        let base_sepolia_rpc_url = env::var("BASE_SEPOLIA_RPC_URL")
            .unwrap_or_else(|_| "https://sepolia.base.org".to_string());
            
        let private_key = env::var("PRIVATE_KEY").ok();
        
        let oneinch_api_key = env::var("1INCH_API_KEY").ok();
        
        let exa_api_key = env::var("EXA_API_KEY")
            .unwrap_or_else(|_| "mock_exa_api_key_for_development".to_string());
        
        Ok(Self {
            database_url,
            anthropic_api_key,
            base_sepolia_rpc_url,
            private_key,
            oneinch_api_key,
            exa_api_key,
        })
    }
    
    /// Get a shared instance of the config
    pub fn get_instance() -> Result<&'static Self> {
        use std::sync::OnceLock;
        
        static CONFIG: OnceLock<Config> = OnceLock::new();
        
        let config = CONFIG.get_or_init(|| {
            match Config::from_env() {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("Failed to initialize config: {}", e);
                    // Provide a default config to avoid panicking
                    Config {
                        database_url: String::new(),
                        anthropic_api_key: String::new(),
                        base_sepolia_rpc_url: String::new(),
                        private_key: None,
                        oneinch_api_key: None,
                        exa_api_key: String::new(),
                    }
                }
            }
        });
        
        if config.database_url.is_empty() {
            Err(anyhow!("Failed to initialize config"))
        } else {
            Ok(config)
        }
    }
}
