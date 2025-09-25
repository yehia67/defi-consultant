# Agent Friend - Customizable DeFi Trading AI Agent

A customizable AI agent for DeFi trading and strategy management, built with Rust, Tokio, and Anthropic's Claude API. This agent helps users execute trading strategies, manage yield positions, and analyze on-chain data based on personalized knowledge and strategies.

## Features

- 🤖 Intelligent chat interface with Claude AI
- 🧠 Customizable agent personality, knowledge, and strategies
- 📈 Real-time token price tracking via multiple data sources
- 📊 Historical price analysis for informed trading decisions
- 💹 Customizable trading strategies for any token pair
- 💰 Portfolio management and wallet integration
- 🔍 Market trend analysis and trading recommendations
- 💾 PostgreSQL database integration for message, strategy, and knowledge storage
- ⛓️ Multi-chain support for trading operations
- 🔎 Deep research capabilities with EXA API integration
- 📝 Planning mode for step-by-step strategy execution

## Architecture

The project is structured into several key modules:

```
agent-friend/
├── src/
│   ├── main.rs              # Entry point and main loop
│   ├── lib.rs               # Library exports and module organization
│   ├── anthropic.rs         # Claude API integration
│   ├── agent_customizer.rs  # Agent customization functionality
│   ├── data_source.rs       # Data source integrations
│   ├── db.rs                # Database operations
│   └── bin/                 # Additional binaries
│       └── agent_customizer_cli.rs # CLI for agent customization
├── assets/
│   ├── personality.json     # Agent personality configuration
│   ├── data_sources/        # Data source configurations
│   └── knowledge/           # Knowledge base files (migrating to DB)
├── migrations/
│   ├── 20250816175200_create_messages.sql    # Messages table
│   ├── 20250913131600_create_strategies_table.sql # Strategies table
│   ├── 20250913131700_create_knowledge_table.sql  # Knowledge table
│   └── 20250913131800_create_users_table.sql      # Users table
├── examples/
│   └── agent_customization.json # Example customization file
├── .env.example         # Example environment variables
└── Cargo.toml           # Project dependencies
```

### Core Components

1. **Main Loop** (`main.rs`): Handles user input/output, command processing, and orchestrates the agent's components
2. **Library Exports** (`lib.rs`): Organizes module exports and provides a clean API for the project
3. **Anthropic Integration** (`anthropic.rs`): Manages communication with Claude API
4. **Agent Customizer** (`agent_customizer.rs`): Provides functionality for customizing agent personality, knowledge, and strategies
5. **Data Source Integration** (`data_source.rs`): Manages connections to external data sources like CoinGecko and 1inch
6. **Database Layer** (`db.rs`): Stores conversation history, user data, strategies, and knowledge in PostgreSQL
7. **CLI Tool** (`bin/agent_customizer_cli.rs`): Command-line interface for managing agent customizations

## Prerequisites

- Rust and Cargo
- PostgreSQL database
- Anthropic API key
- Base Sepolia RPC URL (for blockchain interactions)
- Private key for a wallet with Base Sepolia ETH and USDC
- CoinGecko API access (free tier works for basic usage)

## Setup Instructions

### 1. Clone the repository

```bash
git clone https://github.com/yehia67/onchain-agent-template.git
cd onchain-agent-template
```

### 2. Set up environment variables

Copy the example environment file and add your credentials:

```bash
cp .env.example .env
```

Edit the `.env` file to include:
- Your Anthropic API key (get it from [Anthropic Console](https://console.anthropic.com/settings/keys))
- PostgreSQL database connection string
- Base Sepolia RPC URL (e.g., https://sepolia.base.org)
- Your wallet's private key (for trading operations)
- 1inch API key (for DEX aggregation)
- EXA API key (for deep research capabilities)

### 3. Set up the database

Create a PostgreSQL database and user:

```bash
# Example commands - adjust as needed for your PostgreSQL setup
createdb agentdb
createuser -P agent  # Set password to 'agent' when prompted
```

Run database migrations:

```bash
# If you have sqlx-cli installed
sqlx migrate run

# Alternatively, the migrations will run automatically on first startup
```

### 4. Build and run the project

```bash
cargo build
cargo run
```

## Usage

Once running, you can interact with the Aero trading agent via the command line:

### General Interaction
- Type messages and press Enter to send them to the agent
- The agent will respond based on its trading-focused personality
- Type 'exit' to quit

### Price Commands
Use these commands to check Aerodrome token prices:

```
/price                      - Get current AERO price
/price history              - Get yesterday's AERO price
/price history DD-MM-YYYY   - Get AERO price on specific date
```

### Trading Commands
Use these commands to manage your AERO/USDC trading:

```
/trade help                 - Show available trading commands
/trade balance              - Check your AERO and USDC balances
/trade buy [amount]         - Buy AERO with specified USDC amount
/trade sell [amount]        - Sell specified amount of AERO for USDC
/trade analyze              - Get trading recommendations based on price analysis
```

## Aerodrome Trading Features

The Aero agent provides these specialized trading capabilities:

### Price Tracking
- Real-time price monitoring of Aerodrome token
- Historical price data retrieval and analysis
- Price trend identification for optimal entry/exit points

### Trading Strategy
- Automated market analysis based on price movements
- Recommendations for buying when prices are low
- Suggestions for taking profits when prices are high
- Dollar-cost averaging strategy implementation

### Portfolio Management
- AERO and USDC balance tracking
- Position sizing recommendations
- Risk management through partial profit-taking
- Performance tracking over time

## Agent Customization Tool

The project includes a powerful agent customization tool that allows you to personalize your DeFi trading assistant with custom strategies and knowledge.

### Using the Agent Customizer CLI

```bash
# Create or update an agent with strategies and knowledge from a JSON file
cargo run --bin agent_customizer_cli customize --input-file examples/agent_customization.json

# Get an agent's profile
cargo run --bin agent_customizer_cli get-profile --username defi_trader --output profile.json

# Search for strategies and knowledge
cargo run --bin agent_customizer_cli search --username defi_trader --query "arbitrage" --output search_results.json

# Import strategies and knowledge from JSON files in directories
cargo run --bin agent_customizer_cli import --username defi_trader --wallet "0x123..." --strategies-dir ./strategies --knowledge-dir ./knowledge
```

### Customization JSON Format

The agent customization JSON file has the following structure:

```json
{
  "username": "your_username",
  "wallet_address": "optional_wallet_address",
  "strategies": [
    {
      "strategy_id": "unique_strategy_id",
      "name": "Strategy Name",
      "category": "trading|yield|risk_management",
      "description": "Strategy description",
      "risk_level": "Low|Medium|High",
      "tags": ["tag1", "tag2"],
      "steps": ["Step 1", "Step 2", "Step 3"],
      "requirements": ["Requirement 1", "Requirement 2"],
      "expected_returns": {
        "min": 0.5,
        "max": 2.0,
        "timeframe": "daily|monthly|yearly|per_trade"
      },
      "author": "Author Name",
      "version": "1.0.0"
    }
  ],
  "knowledge": [
    {
      "source_id": "unique_knowledge_id",
      "content": "Markdown content with knowledge information",
      "tags": ["tag1", "tag2"]
    }
  ]
}
```

See the `examples/agent_customization.json` file for a complete example.

## Extending the Agent Friend

You can extend this agent by:
- Adding more data sources in `data_source.rs`
- Creating new trading strategies in the database
- Implementing additional knowledge sources
- Enhancing the agent personality with more capabilities
- Creating a web dashboard for visualizing trading performance
- Implementing automated trading based on predefined rules
- Adding notifications for price alerts and trade executions

## License

MIT
