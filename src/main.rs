use agent_friend::{
    db, 
    investment_chat::InvestmentChatAgent, 
    config::Config, 
    logging,
    exa_api::ExaApiClient
};
use std::io::{self, Write};
use std::path::Path;
use tracing::{info, error};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    let log_dir = Path::new("./logs");
    if let Err(e) = logging::init_logging(log_dir) {
        eprintln!("Warning: Failed to initialize logging: {}", e);
    }
    
    // Load environment variables
    dotenv::dotenv().ok();
    info!("Starting Crypto Investment Agent");
    
    // Initialize database
    info!("Initializing database connection");
    match db::init_db_pool().await {
        Ok(_) => info!("Database connection established"),
        Err(e) => {
            error!("Database connection failed: {}", e);
            println!("Warning: Database connection failed. The agent will work without database features.");
        }
    }
    
    // Create agent
    let username = "default_user";
    info!("Creating investment chat agent for user: {}", username);
    let agent = match InvestmentChatAgent::new(username).await {
        Ok(agent) => {
            info!("Agent created successfully");
            agent
        },
        Err(e) => {
            error!("Failed to create investment chat agent: {}", e);
            return Err(anyhow::anyhow!("Failed to initialize agent: {}", e));
        }
    };
    
    // Welcome message
    println!("\n=== Nova - Your Crypto Investment Advisor ===");
    println!("Chat with Nova about crypto investments, market trends, and trading strategies.");
    println!("Nova can research projects in real-time and provide personalized investment advice.");
    println!("Type 'exit' or 'quit' to end the conversation.\n");
    
    // Initial greeting
    let greeting = "Hi! I'm Nova, your crypto investment advisor. I can help you research projects, analyze market trends, and make informed investment decisions. What would you like to discuss today?";
    println!("Nova: {}", greeting);
    
    // Main chat loop
    loop {
        print!("\nYou: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input.to_lowercase() == "exit" || input.to_lowercase() == "quit" {
            println!("\nNova: Thanks for chatting! Feel free to come back anytime you need investment advice.");
            break;
        }
        
        // Skip empty inputs
        if input.is_empty() {
            continue;
        }
        
        // Process the message
        print!("\nNova is thinking...");
        io::stdout().flush()?;
        
        match agent.process_message(input).await {
            Ok(response) => {
                print!("\r"); // Clear the "thinking" message
                println!("\nNova: {}", response);
            },
            Err(e) => {
                print!("\r"); // Clear the "thinking" message
                error!("Error processing message: {}", e);
                
                // Provide more specific error messages based on error type
                let user_message = match e {
                    agent_friend::investment_chat::InvestmentChatError::AnthropicApi(ref msg) => {
                        if msg.contains("Authentication error") || msg.contains("Invalid API key") {
                            "Sorry, I'm having trouble with my API authentication. Please check that your Anthropic API key is valid in the .env file."
                        } else if msg.contains("Connection error") || msg.contains("timed out") {
                            "Sorry, I'm having trouble connecting to my AI service. Please check your internet connection and try again."
                        } else if msg.contains("Rate limit") {
                            "Sorry, I've reached my usage limit with the AI service. Please try again in a few minutes."
                        } else if msg.contains("Server error") {
                            "Sorry, the AI service is currently experiencing issues. Please try again later."
                        } else {
                            "Sorry, I encountered an error while processing your request. There might be an issue with the Anthropic API service."
                        }
                    },
                    agent_friend::investment_chat::InvestmentChatError::Configuration(ref msg) => {
                        "Sorry, there's a configuration issue. Please check your .env file and ensure all required API keys are set correctly."
                    },
                    agent_friend::investment_chat::InvestmentChatError::PriceApi(ref msg) => {
                        "Sorry, I couldn't fetch the cryptocurrency price data. The price API might be experiencing issues or the cryptocurrency symbol might not be supported."
                    },
                    agent_friend::investment_chat::InvestmentChatError::ExternalApi(ref msg) => {
                        if msg.contains("price") {
                            "Sorry, I couldn't fetch the latest cryptocurrency price data. The price API might be experiencing issues."
                        } else {
                            "Sorry, I encountered an issue with an external API. Please try again later."
                        }
                    },
                    _ => "Sorry, I encountered an error while processing your request. Please try again."
                };
                
                println!("\nNova: {}", user_message);
            }
        }
    }
    
    Ok(())
}
