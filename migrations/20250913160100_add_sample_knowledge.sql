-- Add sample knowledge entries for crypto trading
-- Note: These inserts will only work after a user with ID 1 exists

-- Bitcoin Knowledge
INSERT INTO knowledge (user_id, source_id, content, tags, created_at, updated_at)
VALUES (
    1,
    'bitcoin_fundamentals',
    'Bitcoin is a decentralized digital currency created in 2009 by an unknown person or group using the pseudonym Satoshi Nakamoto. It operates on a peer-to-peer network without central authority or banks managing transactions. Bitcoin is created through a process called mining, where powerful computers solve complex mathematical problems. The total supply is capped at 21 million coins, making it a deflationary asset. Bitcoin uses blockchain technology to record all transactions in a public ledger, ensuring transparency and security. Key characteristics include: limited supply, decentralization, pseudonymity, immutability of transactions, and divisibility (each bitcoin can be divided into 100 million satoshis).',
    ARRAY['Bitcoin', 'Fundamentals', 'Cryptocurrency'],
    now(),
    now()
);

-- Ethereum Knowledge
INSERT INTO knowledge (user_id, source_id, content, tags, created_at, updated_at)
VALUES (
    1,
    'ethereum_fundamentals',
    'Ethereum is a decentralized, open-source blockchain platform that enables the creation of smart contracts and decentralized applications (dApps). Created by Vitalik Buterin in 2015, Ethereum extends beyond a simple cryptocurrency by providing a platform for developers to build and deploy decentralized applications. Ether (ETH) is the native cryptocurrency of the Ethereum blockchain, used to pay for transaction fees and computational services. Ethereum implemented a major upgrade called "The Merge" in September 2022, transitioning from Proof of Work to Proof of Stake consensus mechanism, significantly reducing its energy consumption. Ethereum supports the ERC-20 token standard, which allows for the creation of fungible tokens, and the ERC-721 standard for non-fungible tokens (NFTs).',
    ARRAY['Ethereum', 'Smart Contracts', 'Cryptocurrency'],
    now(),
    now()
);

-- DeFi Knowledge
INSERT INTO knowledge (user_id, source_id, content, tags, created_at, updated_at)
VALUES (
    1,
    'defi_fundamentals',
    'Decentralized Finance (DeFi) refers to financial applications built on blockchain technology that aim to recreate and improve upon traditional financial systems without centralized intermediaries. Key components of DeFi include: 1) Decentralized Exchanges (DEXs) like Uniswap and SushiSwap that allow direct peer-to-peer trading without centralized order books; 2) Lending protocols such as Aave and Compound where users can lend assets to earn interest or borrow against collateral; 3) Stablecoins like USDC and DAI that maintain a stable value relative to fiat currencies; 4) Yield farming, which involves providing liquidity to protocols to earn rewards; 5) Automated Market Makers (AMMs) that use mathematical formulas to price assets; 6) Liquidity pools where users deposit pairs of tokens to facilitate trading. DeFi applications are composable, meaning they can be combined like "money legos" to create more complex financial products.',
    ARRAY['DeFi', 'Decentralized Finance', 'Yield Farming', 'Liquidity'],
    now(),
    now()
);

-- Technical Analysis Knowledge
INSERT INTO knowledge (user_id, source_id, content, tags, created_at, updated_at)
VALUES (
    1,
    'crypto_technical_analysis',
    'Technical analysis in cryptocurrency trading involves studying price charts and using indicators to identify patterns and predict future price movements. Key technical indicators include: 1) Moving Averages (MA) - show the average price over a specific time period, with common periods being 50, 100, and 200 days; 2) Relative Strength Index (RSI) - measures the speed and change of price movements on a scale of 0-100, with values above 70 indicating overbought conditions and below 30 indicating oversold conditions; 3) Moving Average Convergence Divergence (MACD) - shows the relationship between two moving averages of a price; 4) Bollinger Bands - consist of a middle band (20-day SMA) and two outer bands that represent standard deviations, helping identify volatility and potential reversal points; 5) Support and Resistance levels - price points where a cryptocurrency has historically had difficulty falling below (support) or rising above (resistance). Chart patterns like head and shoulders, double tops/bottoms, and triangles can also signal potential trend reversals or continuations.',
    ARRAY['Technical Analysis', 'Trading', 'Indicators', 'Charts'],
    now(),
    now()
);

-- Risk Management Knowledge
INSERT INTO knowledge (user_id, source_id, content, tags, created_at, updated_at)
VALUES (
    1,
    'crypto_risk_management',
    'Effective risk management is crucial in cryptocurrency trading due to the market''s high volatility. Key risk management principles include: 1) Position Sizing - never risk more than 1-2% of your total portfolio on a single trade; 2) Stop-Loss Orders - always set stop-losses to limit potential losses on each trade; 3) Take-Profit Levels - establish clear price targets for taking profits; 4) Diversification - spread investments across different cryptocurrencies, sectors, and risk levels; 5) Risk-Reward Ratio - aim for trades with a minimum 1:2 risk-reward ratio, meaning the potential profit is at least twice the potential loss; 6) Correlation Analysis - understand how different assets move in relation to each other to avoid overexposure to correlated risks; 7) Volatility Adjustment - adjust position sizes based on an asset''s volatility; 8) Drawdown Management - have a plan for handling extended periods of losses; 9) Psychological Discipline - stick to your trading plan and avoid emotional decisions. Implementing these practices can help preserve capital during market downturns and improve long-term performance.',
    ARRAY['Risk Management', 'Trading', 'Position Sizing', 'Stop-Loss'],
    now(),
    now()
);

-- Base Chain Knowledge
INSERT INTO knowledge (user_id, source_id, content, tags, created_at, updated_at)
VALUES (
    1,
    'base_chain_overview',
    'Base is an Ethereum Layer 2 (L2) scaling solution developed by Coinbase. It uses Optimistic Rollup technology to provide faster and cheaper transactions while inheriting Ethereum''s security. Key features of Base include: 1) Ethereum Compatibility - supports Ethereum''s programming language Solidity and is compatible with Ethereum Virtual Machine (EVM); 2) Lower Fees - transactions cost significantly less than on Ethereum mainnet; 3) Faster Transactions - provides quicker confirmation times compared to Ethereum; 4) Security - derives its security from Ethereum through the rollup mechanism; 5) Ecosystem - growing ecosystem of DeFi protocols, including Aerodrome (a fork of Velodrome), BaseSwap, and Degen Chain; 6) Bridge - users can bridge assets between Ethereum and Base using the official Base Bridge. Base is particularly suitable for DeFi applications, NFT projects, and games that require lower transaction costs while maintaining connection to Ethereum''s liquidity and security.',
    ARRAY['Base', 'Layer 2', 'Scaling', 'Coinbase', 'Blockchain'],
    now(),
    now()
);

-- Aerodrome Finance Knowledge
INSERT INTO knowledge (user_id, source_id, content, tags, created_at, updated_at)
VALUES (
    1,
    'aerodrome_finance_overview',
    'Aerodrome Finance is a decentralized exchange (DEX) built on the Base blockchain. It is a fork of Velodrome Finance from the Optimism ecosystem, designed to enhance liquidity and trading on Base. Key features include: 1) Liquidity Pools - users can provide liquidity to earn trading fees and rewards; 2) Vote-Escrowed Tokenomics - AERO token holders can lock their tokens to receive veAERO, which grants voting rights and boosted rewards; 3) Bribes System - protocols can incentivize veAERO holders to direct emissions to their pools; 4) Concentrated Liquidity - supports both traditional constant product pools and concentrated liquidity positions; 5) Gauge Voting - veAERO holders vote weekly on which pools receive AERO emissions; 6) Fee Distribution - trading fees are distributed to liquidity providers and veAERO holders. Aerodrome has become one of the largest DEXes on Base, capturing significant market share and total value locked (TVL) on the network.',
    ARRAY['Aerodrome', 'DEX', 'Base', 'Liquidity', 'DeFi'],
    now(),
    now()
);

-- Market Cycles Knowledge
INSERT INTO knowledge (user_id, source_id, content, tags, created_at, updated_at)
VALUES (
    1,
    'crypto_market_cycles',
    'Cryptocurrency markets typically move in cycles that can be broadly categorized into four phases: 1) Accumulation Phase - occurs after a market bottom when sentiment is negative but smart money begins accumulating at low prices; 2) Markup Phase (Bull Market) - characterized by rising prices, increasing trading volume, and growing public interest, often culminating in euphoria; 3) Distribution Phase - occurs near market tops when early investors begin selling to late entrants, often accompanied by excessive optimism and FOMO (Fear Of Missing Out); 4) Markdown Phase (Bear Market) - features declining prices, decreasing volume, and negative sentiment, often ending in capitulation. Bitcoin has historically followed approximately four-year cycles influenced by its halving events, which reduce mining rewards by 50% roughly every four years. These halvings create supply shocks that have preceded previous bull markets. Understanding these cycles can help investors make more informed decisions about entry and exit points, though past patterns don''t guarantee future results.',
    ARRAY['Market Cycles', 'Bull Market', 'Bear Market', 'Halving', 'Bitcoin'],
    now(),
    now()
);

-- Impermanent Loss Knowledge
INSERT INTO knowledge (user_id, source_id, content, tags, created_at, updated_at)
VALUES (
    1,
    'impermanent_loss_explained',
    'Impermanent Loss (IL) is a phenomenon that affects liquidity providers in automated market maker (AMM) protocols when the price of the deposited assets changes relative to when they were deposited. It represents the difference in value between holding assets in an AMM pool versus holding them in a wallet. Key points about impermanent loss: 1) Cause - occurs due to the constant product formula (x*y=k) used by many AMMs, which requires the pool to maintain a balanced ratio of assets; 2) Calculation - the greater the price change of the assets from the time of deposit, the greater the impermanent loss; 3) Permanence - becomes "permanent" only when liquidity is withdrawn after a price change; 4) Mitigation - trading fees and liquidity mining rewards often aim to offset impermanent loss; 5) Risk Factors - highly volatile or uncorrelated pairs experience greater impermanent loss; 6) Alternatives - concentrated liquidity positions (as in Uniswap v3) or stablecoin pairs can reduce impermanent loss risk. Understanding impermanent loss is crucial for liquidity providers to accurately assess the profitability of their positions.',
    ARRAY['Impermanent Loss', 'AMM', 'Liquidity Providing', 'DeFi', 'Risk'],
    now(),
    now()
);

-- MEV Knowledge
INSERT INTO knowledge (user_id, source_id, content, tags, created_at, updated_at)
VALUES (
    1,
    'mev_explained',
    'Maximal Extractable Value (MEV), formerly known as Miner Extractable Value, refers to the profit that can be extracted from blockchain users by controlling transaction ordering within blocks. Key aspects of MEV include: 1) Types - common MEV strategies include frontrunning, backrunning, sandwich attacks, liquidations, and arbitrage; 2) Frontrunning - occurs when bots observe pending transactions and insert their own transactions before them with higher gas prices; 3) Sandwich Attacks - involve placing one transaction before and one after a target transaction to profit from the price impact; 4) Flashbots - a research organization that developed an auction mechanism allowing traders to submit transactions directly to miners/validators, reducing negative MEV effects; 5) MEV-Boost - a solution implemented post-Ethereum Merge that separates block proposing from block building; 6) Protection - users can protect themselves using private transaction pools, setting appropriate slippage tolerance, and using MEV-aware protocols. MEV has significant implications for market efficiency, user experience, and the overall fairness of blockchain networks.',
    ARRAY['MEV', 'Frontrunning', 'Blockchain', 'DeFi', 'Trading'],
    now(),
    now()
);
