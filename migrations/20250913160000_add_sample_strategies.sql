-- Add sample crypto trading strategies
-- Note: These inserts will only work after a user with ID 1 exists

-- DCA Strategy
INSERT INTO strategies (user_id, strategy_id, name, category, description, risk_level, tags, steps, requirements, expected_returns, author, version, created_at, updated_at)
VALUES (
    1,
    'dca_btc_eth',
    'Dollar Cost Averaging BTC/ETH',
    'Accumulation',
    'A simple dollar cost averaging strategy for Bitcoin and Ethereum that reduces market timing risk by spreading purchases over regular intervals.',
    'Low',
    ARRAY['DCA', 'Bitcoin', 'Ethereum', 'Long-term'],
    ARRAY[
        'Set aside a fixed amount of capital for regular purchases',
        'Divide capital into equal portions for each purchase period',
        'Execute purchases on a fixed schedule (daily, weekly, or monthly)',
        'Hold assets for long-term appreciation'
    ],
    ARRAY['Stable funding source', 'Long-term investment horizon (1+ years)', 'Access to spot markets'],
    '{"annual_expected_return": {"min": 0.15, "target": 0.25, "max": 0.40}, "risk_adjusted_return": "Medium", "time_horizon": "Long-term", "notes": "Historical performance varies significantly by market cycle"}'::jsonb,
    'System',
    '1.0.0',
    now(),
    now()
);

-- Grid Trading Strategy
INSERT INTO strategies (user_id, strategy_id, name, category, description, risk_level, tags, steps, requirements, expected_returns, author, version, created_at, updated_at)
VALUES (
    1,
    'grid_trading_volatile',
    'Grid Trading for Volatile Markets',
    'Market Neutral',
    'A grid trading strategy that places buy and sell orders at regular price intervals to profit from price volatility in sideways markets.',
    'Medium',
    ARRAY['Grid Trading', 'Volatility', 'Automated', 'Market Neutral'],
    ARRAY[
        'Define upper and lower price boundaries for the grid',
        'Divide the price range into equal grid levels',
        'Place buy orders at each level below current price',
        'Place sell orders at each level above current price',
        'When orders execute, place new orders in the opposite direction'
    ],
    ARRAY['API trading access', 'Sufficient capital for grid density', 'Sideways or ranging market conditions'],
    '{"annual_expected_return": {"min": 0.20, "target": 0.35, "max": 0.50}, "risk_adjusted_return": "High", "time_horizon": "Medium-term", "notes": "Performance depends on volatility and proper grid setup"}'::jsonb,
    'System',
    '1.0.0',
    now(),
    now()
);

-- Yield Farming Strategy
INSERT INTO strategies (user_id, strategy_id, name, category, description, risk_level, tags, steps, requirements, expected_returns, author, version, created_at, updated_at)
VALUES (
    1,
    'aerodrome_yield_farming',
    'Aerodrome Liquidity Provision Strategy',
    'Yield',
    'A yield farming strategy focused on providing liquidity to Aerodrome pools on Base to earn trading fees and ARB rewards.',
    'High',
    ARRAY['DeFi', 'Yield Farming', 'Aerodrome', 'Base', 'Liquidity'],
    ARRAY[
        'Acquire equal values of both tokens in the target liquidity pool',
        'Deposit tokens into Aerodrome liquidity pool',
        'Stake LP tokens to earn additional AERO rewards',
        'Compound rewards periodically to maximize returns',
        'Monitor impermanent loss and adjust position as needed'
    ],
    ARRAY['Base network access', 'Gas funds for transactions', 'Understanding of impermanent loss'],
    '{"annual_expected_return": {"min": 0.30, "target": 0.50, "max": 0.80}, "risk_adjusted_return": "Medium", "time_horizon": "Short to Medium-term", "notes": "Returns include trading fees, token rewards, and potential token appreciation"}'::jsonb,
    'System',
    '1.0.0',
    now(),
    now()
);

-- Momentum Trading Strategy
INSERT INTO strategies (user_id, strategy_id, name, category, description, risk_level, tags, steps, requirements, expected_returns, author, version, created_at, updated_at)
VALUES (
    1,
    'momentum_trading',
    'Crypto Momentum Trading',
    'Active Trading',
    'A momentum-based trading strategy that identifies and trades with the prevailing market trend using technical indicators.',
    'High',
    ARRAY['Momentum', 'Technical Analysis', 'Trend Following', 'Active Trading'],
    ARRAY[
        'Identify assets showing strong momentum using indicators (RSI, MACD, Moving Averages)',
        'Enter positions in the direction of the trend when momentum indicators confirm',
        'Set stop-loss orders to limit downside risk',
        'Take profits at predetermined price targets',
        'Exit positions when momentum indicators show reversal signals'
    ],
    ARRAY['Technical analysis skills', 'Active market monitoring', 'Ability to execute trades quickly'],
    '{"annual_expected_return": {"min": 0.25, "target": 0.60, "max": 1.20}, "risk_adjusted_return": "Medium", "time_horizon": "Short-term", "notes": "High variance in returns; requires disciplined execution of entry/exit rules"}'::jsonb,
    'System',
    '1.0.0',
    now(),
    now()
);

-- Arbitrage Strategy
INSERT INTO strategies (user_id, strategy_id, name, category, description, risk_level, tags, steps, requirements, expected_returns, author, version, created_at, updated_at)
VALUES (
    1,
    'dex_arbitrage',
    'DEX Arbitrage Strategy',
    'Arbitrage',
    'A strategy that exploits price differences for the same asset across different decentralized exchanges to generate risk-free profits.',
    'Medium',
    ARRAY['Arbitrage', 'DEX', 'Flash Loans', 'MEV'],
    ARRAY[
        'Monitor price differences for assets across multiple DEXes',
        'Identify arbitrage opportunities when price discrepancies exceed transaction costs',
        'Execute trades to buy on the lower-priced exchange',
        'Simultaneously sell on the higher-priced exchange',
        'Optionally use flash loans to increase capital efficiency'
    ],
    ARRAY['Fast execution capabilities', 'Access to multiple DEXes', 'Gas optimization knowledge', 'MEV protection'],
    '{"annual_expected_return": {"min": 0.15, "target": 0.30, "max": 0.50}, "risk_adjusted_return": "High", "time_horizon": "Short-term", "notes": "Returns diminish as markets become more efficient; competition from MEV bots"}'::jsonb,
    'System',
    '1.0.0',
    now(),
    now()
);
