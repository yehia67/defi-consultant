pub mod db;
pub mod anthropic;
pub mod data_source;
pub mod agent_customizer;
pub mod exa_api;
pub mod investment_chat;
pub mod config;
pub mod logging;
pub mod price_fetcher;

// Re-export commonly used types
pub use db::{
    User, Strategy, Knowledge, DataSource,
    get_db_pool, save_message, get_messages, init_db_pool,
};

pub use investment_chat::{
    InvestmentChatAgent, InvestmentChatError,
};

pub use exa_api::{
    ExaApiClient, ExaSearchResult, ExaSearchResponse,
};

pub use config::Config;
