# Crypto Research Tool

The Crypto Research Tool is a powerful utility that leverages the Exa API to gather comprehensive information about cryptocurrency projects. This tool helps you make informed investment decisions by providing detailed insights about various aspects of crypto projects.

## Features

- **General Project Information**: Get an overview of any cryptocurrency project
- **Technical Details**: Research the technical architecture and implementation
- **Tokenomics Analysis**: Understand token distribution, supply, and economics
- **Team Information**: Learn about the founders and development team
- **Recent News**: Stay updated with the latest developments
- **Investment Analysis**: Get insights on potential risks and opportunities
- **Database Integration**: Save all research directly to your knowledge database

## Setup

1. Make sure you have an Exa API key
2. Add your API key to the `.env` file:
   ```
   EXA_API_KEY=your_exa_api_key_here
   ```
3. Build the project:
   ```
   cargo build --bin crypto_research
   ```

## Usage

### Command Line Interface

The tool provides several commands to research different aspects of a crypto project:

#### General Search

```bash
cargo run --bin crypto_research -- search --project "Bitcoin" --results 5 --save
```

#### Technical Details

```bash
cargo run --bin crypto_research -- technical --project "Ethereum" --results 5 --save
```

#### Tokenomics Information

```bash
cargo run --bin crypto_research -- tokenomics --project "Solana" --results 5 --save
```

#### Team Information

```bash
cargo run --bin crypto_research -- team --project "Cardano" --results 5 --save
```

#### Recent News

```bash
cargo run --bin crypto_research -- news --project "Polkadot" --results 5 --save
```

#### Investment Analysis

```bash
cargo run --bin crypto_research -- investment --project "Chainlink" --results 5 --save
```

### Interactive Mode

For a more user-friendly experience, use the interactive mode:

```bash
cargo run --bin crypto_research -- interactive
```

This will guide you through the research process with step-by-step prompts.

## Command Options

All commands support the following options:

- `--project` or `-p`: The name of the crypto project to research (required)
- `--results` or `-r`: Number of results to return (default: 5)
- `--username` or `-u`: Username to associate with the search (default: "default_user")
- `--save` or `-s`: Save results to database as knowledge (flag)

## Database Integration

When you use the `--save` flag, the research results are automatically saved to your database as knowledge entries. These entries can then be used by your AI agent to make informed recommendations.

Each knowledge entry includes:
- Source ID: A unique identifier for the research
- Content: The summarized research findings
- Tags: Project name, category, and "exa_research" tag

## Examples

### Researching a New DeFi Protocol

```bash
cargo run --bin crypto_research -- interactive
# Enter "Uniswap" as the project name
# Select option 7 (All of the above)
# Enter "10" for number of results
# Enter "y" to save to database
```

### Quick Technical Analysis

```bash
cargo run --bin crypto_research -- technical --project "Avalanche" --results 10 --save
```

### Investment Research Before Trading

```bash
cargo run --bin crypto_research -- investment --project "Arbitrum" --save
```

## Advanced Usage

### Programmatic Access

You can also use the Exa API client in your own code:

```rust
use agent_friend::exa_api::ExaApiClient;

async fn research_project() -> Result<(), Box<dyn std::error::Error>> {
    let exa_client = ExaApiClient::new()?;
    let response = exa_client.search_crypto_project("Bitcoin", 5).await?;
    println!("{}", exa_client.summarize_project(&response.results));
    Ok(())
}
```

### Custom Queries

The tool is designed to be flexible. If you need more specific information, you can modify the queries in the `exa_api.rs` file to target exactly what you're looking for.

## Troubleshooting

### API Key Issues

If you encounter errors related to the API key:
1. Check that your `.env` file contains the correct `EXA_API_KEY` value
2. Ensure the API key is active and has sufficient permissions
3. Verify that the `.env` file is in the correct location (project root)

### No Results Found

If the tool doesn't return any results:
1. Try a more well-known project name
2. Increase the number of results
3. Check your internet connection
4. Verify that the Exa API service is operational
