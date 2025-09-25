use crate::knowledge_manager::KnowledgeManager;
use crate::personality::Personality;
use anyhow::Result;

/// Structure to handle dynamic prompts for the AI agent
pub struct PromptHandler {
    base_prompt: String,
    system_prompt: String,
}

impl PromptHandler {
    /// Create a new PromptHandler with default prompts
    pub fn new() -> Self {
        Self {
            base_prompt: String::new(),
            system_prompt: String::new(),
        }
    }
    
    /// Initialize the prompt handler with personality information
    pub fn initialize(&mut self, personality: &Personality) -> Result<()> {
        // Create the base prompt from personality information
        self.base_prompt = format!(
            "You are {}, a {}. Your communication style is {} and {}.\n\n",
            personality.name,
            personality.role,
            personality.style.tone,
            personality.style.formality
        );
        
        // Add domain focus
        self.base_prompt.push_str("Your expertise is focused on: ");
        for (i, domain) in personality.style.domain_focus.iter().enumerate() {
            if i > 0 {
                self.base_prompt.push_str(", ");
            }
            self.base_prompt.push_str(domain);
        }
        self.base_prompt.push_str(".\n\n");
        
        // Add rules
        self.base_prompt.push_str("Follow these rules in all interactions:\n");
        for rule in &personality.rules {
            self.base_prompt.push_str(&format!("- {}\n", rule));
        }
        self.base_prompt.push_str("\n");
        
        // Add focused protocols
        let focused_protocols = personality.get_focused_protocols();
        if !focused_protocols.is_empty() {
            self.base_prompt.push_str("You have special expertise in these protocols:\n");
            for protocol in focused_protocols {
                self.base_prompt.push_str(&format!(
                    "- {} on {}: {}\n",
                    protocol.name,
                    protocol.chain,
                    protocol.description
                ));
            }
            self.base_prompt.push_str("\n");
        }
        
        // Add available strategies
        if let Some(yield_strategies) = personality.get_strategies("yield") {
            self.base_prompt.push_str("Yield strategies you can recommend:\n");
            for strategy in yield_strategies {
                self.base_prompt.push_str(&format!("- {}\n", strategy));
            }
            self.base_prompt.push_str("\n");
        }
        
        if let Some(trading_strategies) = personality.get_strategies("trading") {
            self.base_prompt.push_str("Trading strategies you can recommend:\n");
            for strategy in trading_strategies {
                self.base_prompt.push_str(&format!("- {}\n", strategy));
            }
            self.base_prompt.push_str("\n");
        }
        
        if let Some(risk_strategies) = personality.get_strategies("risk_management") {
            self.base_prompt.push_str("Risk management approaches you can implement:\n");
            for strategy in risk_strategies {
                self.base_prompt.push_str(&format!("- {}\n", strategy));
            }
            self.base_prompt.push_str("\n");
        }
        
        // Add available APIs
        let apis = personality.get_apis();
        if !apis.is_empty() {
            self.base_prompt.push_str("You have access to these APIs:\n");
            for api in apis {
                self.base_prompt.push_str(&format!(
                    "- {}: {} (endpoints: {})\n",
                    api.name,
                    api.description,
                    api.endpoints.join(", ")
                ));
            }
            self.base_prompt.push_str("\n");
        }
        
        // System prompt for controlling behavior
        self.system_prompt = "
You are a helpful DeFi trading assistant. Always provide thoughtful, accurate advice based on the knowledge you have.
When you don't know something, admit it rather than making up information.
When analyzing trading opportunities, always consider:
1. Risk vs reward ratio
2. Market conditions and trends
3. Gas costs and transaction fees
4. Impermanent loss for liquidity positions
5. Protocol security and audits

For any trade recommendation:
- Explain your reasoning clearly
- Provide specific entry and exit points when possible
- Discuss potential risks
- Consider tax implications

Use markdown formatting for better readability.
".to_string();
        
        Ok(())
    }
    
    /// Get the complete prompt with knowledge context
    pub fn get_complete_prompt(&self, knowledge_manager: &KnowledgeManager, user_query: &str) -> String {
        let mut complete_prompt = self.base_prompt.clone();
        
        // Add system prompt
        complete_prompt.push_str(&self.system_prompt);
        complete_prompt.push_str("\n\n");
        
        // Add relevant knowledge context
        complete_prompt.push_str("KNOWLEDGE CONTEXT:\n");
        
        // Add database knowledge
        let database_context = knowledge_manager.get_entries_by_type_as_context("database");
        if !database_context.is_empty() {
            complete_prompt.push_str(&database_context);
        }
        
        // Add prompt knowledge
        let prompt_context = knowledge_manager.get_entries_by_type_as_context("prompt");
        if !prompt_context.is_empty() {
            complete_prompt.push_str(&prompt_context);
        }
        
        // Add user query
        complete_prompt.push_str("\nUSER QUERY: ");
        complete_prompt.push_str(user_query);
        
        complete_prompt
    }
    
    /// Add a custom instruction to the system prompt
    pub fn add_system_instruction(&mut self, instruction: &str) {
        self.system_prompt.push_str("\n");
        self.system_prompt.push_str(instruction);
    }
}
