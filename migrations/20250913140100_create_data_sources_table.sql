-- Create data_sources table
CREATE TABLE data_sources (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    source_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    source_type TEXT NOT NULL,
    refresh_interval_minutes INTEGER NOT NULL,
    config JSONB NOT NULL,
    created_at TIMESTAMP DEFAULT now(),
    updated_at TIMESTAMP DEFAULT now(),
    last_refresh TIMESTAMP,
    UNIQUE(user_id, source_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Create index on user_id for faster lookups
CREATE INDEX idx_data_sources_user_id ON data_sources(user_id);
-- Create index on source_type for filtering
CREATE INDEX idx_data_sources_source_type ON data_sources(source_type);

-- Insert sample data sources
-- Note: These inserts will only work after a user with ID 1 exists
-- You can adjust the user_id as needed or remove these inserts if you prefer to add data through the application
INSERT INTO data_sources (user_id, source_id, name, description, source_type, refresh_interval_minutes, config, created_at, updated_at)
VALUES 
(1, 'coingecko_aero_price', 'CoinGecko Aerodrome Price Feed', 'Real-time price data for Aerodrome token from CoinGecko API', 'rest_api', 15, 
'{"url": "https://api.coingecko.com/api/v3/simple/price?ids=aerodrome-finance&vs_currencies=usd,eth&include_24hr_change=true", "method": "GET", "headers": {"Accept": "application/json"}}', now(), now()),

(1, 'oneinch_swap_api', '1inch Swap API', 'API for getting swap quotes and executing trades via 1inch', 'rest_api', 5, 
'{"url": "https://api.1inch.io/v5.0/1/swap", "method": "GET", "headers": {"Accept": "application/json", "Authorization": "Bearer $1INCH_API_KEY"}}', now(), now()),

(1, 'mellow_finance_api', 'Mellow Finance API', 'API for Mellow Finance yield strategies', 'rest_api', 60, 
'{"url": "https://api.mellow.finance/v1/strategies", "method": "GET", "headers": {"Accept": "application/json"}}', now(), now()),

(1, 'binance_btc_price', 'Binance BTC Price Feed', 'Real-time BTC/USDT price data from Binance API', 'rest_api', 1, 
'{"url": "https://api.binance.com/api/v3/ticker/price?symbol=BTCUSDT", "method": "GET", "headers": {"Accept": "application/json"}}', now(), now()),

(1, 'binance_eth_price', 'Binance ETH Price Feed', 'Real-time ETH/USDT price data from Binance API', 'rest_api', 1, 
'{"url": "https://api.binance.com/api/v3/ticker/price?symbol=ETHUSDT", "method": "GET", "headers": {"Accept": "application/json"}}', now(), now()),

(1, 'dexscreener_aero_pool', 'DexScreener Aerodrome Pool Data', 'Liquidity and volume data for Aerodrome pools', 'rest_api', 30, 
'{"url": "https://api.dexscreener.com/latest/dex/pairs/base/0x2223f9fe624f69dc9a22b53a9d5ba4a817eb7c08", "method": "GET", "headers": {"Accept": "application/json"}}', now(), now()),

(1, 'defillama_tvl', 'DefiLlama TVL Data', 'Total Value Locked data for major DeFi protocols', 'rest_api', 60, 
'{"url": "https://api.llama.fi/protocols", "method": "GET", "headers": {"Accept": "application/json"}}', now(), now()),

(1, 'gas_price_feed', 'Ethereum Gas Price Feed', 'Current gas prices on Ethereum', 'rest_api', 5, 
'{"url": "https://api.etherscan.io/api?module=gastracker&action=gasoracle", "method": "GET", "headers": {"Accept": "application/json"}}', now(), now());
