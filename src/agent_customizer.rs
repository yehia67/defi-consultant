use crate::db::{
    self, Strategy, User, Knowledge,
    create_strategy, create_knowledge,
    get_strategies_by_user_id, get_knowledge_by_user_id,
    search_strategies_by_text, search_knowledge_by_text
};
use sqlx::{Pool, Postgres};
use serde::{Deserialize, Serialize};
use sqlx::types::JsonValue;

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentCustomizationRequest {
    pub username: String,
    pub wallet_address: Option<String>,
    pub strategies: Option<Vec<StrategyInput>>,
    pub knowledge: Option<Vec<KnowledgeInput>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StrategyInput {
    pub strategy_id: String,
    pub name: String,
    pub category: String,
    pub description: String,
    pub risk_level: String,
    pub tags: Vec<String>,
    pub steps: Vec<String>,
    pub requirements: Vec<String>,
    pub expected_returns: JsonValue,
    pub author: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KnowledgeInput {
    pub source_id: String,
    pub content: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentProfile {
    pub user: User,
    pub strategies: Vec<Strategy>,
    pub knowledge: Vec<Knowledge>,
}

/// Customizes an agent for a user by adding strategies and knowledge
pub async fn customize_agent(
    pool: &Pool<Postgres>,
    request: AgentCustomizationRequest,
) -> Result<AgentProfile, String> {
    // Get or create user
    let user = match db::get_user_by_username(pool, &request.username).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            // Create new user
            match db::create_user(pool, &request.username, request.wallet_address.as_deref()).await {
                Ok(user) => user,
                Err(e) => return Err(format!("Failed to create user: {}", e)),
            }
        },
        Err(e) => return Err(format!("Database error: {}", e)),
    };
    
    // Add strategies if provided
    if let Some(strategies) = request.strategies {
        for strategy in strategies {
            if let Err(e) = create_strategy(
                pool,
                user.id,
                &strategy.strategy_id,
                &strategy.name,
                &strategy.category,
                &strategy.description,
                &strategy.risk_level,
                &strategy.tags,
                &strategy.steps,
                &strategy.requirements,
                strategy.expected_returns,
                &strategy.author,
                &strategy.version,
            ).await {
                return Err(format!("Failed to create strategy {}: {}", strategy.name, e));
            }
        }
    }
    
    // Add knowledge if provided
    if let Some(knowledge_items) = request.knowledge {
        for knowledge in knowledge_items {
            if let Err(e) = create_knowledge(
                pool,
                user.id,
                &knowledge.source_id,
                &knowledge.content,
                &knowledge.tags,
            ).await {
                return Err(format!("Failed to create knowledge {}: {}", knowledge.source_id, e));
            }
        }
    }
    
    // Retrieve all strategies and knowledge for the user
    let strategies = match get_strategies_by_user_id(pool, user.id).await {
        Ok(strategies) => strategies,
        Err(e) => return Err(format!("Failed to retrieve strategies: {}", e)),
    };

    let knowledge = match get_knowledge_by_user_id(pool, user.id).await {
        Ok(knowledge) => knowledge,
        Err(e) => return Err(format!("Failed to retrieve knowledge: {}", e)),
    };

    Ok(AgentProfile {
        user,
        strategies,
        knowledge,
    })
}

/// Searches for strategies and knowledge based on text query
pub async fn search_agent_data(
    pool: &Pool<Postgres>,
    user_id: i32,
    query: &str,
) -> Result<(Vec<Strategy>, Vec<Knowledge>), String> {
    let strategies = match search_strategies_by_text(pool, user_id, query).await {
        Ok(strategies) => strategies,
        Err(e) => return Err(format!("Failed to search strategies: {}", e)),
    };

    let knowledge = match search_knowledge_by_text(pool, user_id, query).await {
        Ok(knowledge) => knowledge,
        Err(e) => return Err(format!("Failed to search knowledge: {}", e)),
    };
    
    Ok((strategies, knowledge))
}

/// Retrieves a complete agent profile for a user
pub async fn get_agent_profile(
    pool: &Pool<Postgres>,
    username: &str,
) -> Result<Option<AgentProfile>, String> {
    // Get user
    let user = match db::get_user_by_username(pool, username).await {
        Ok(Some(user)) => user,
        Ok(None) => return Ok(None),
        Err(e) => return Err(format!("Database error: {}", e)),
    };

    // Retrieve all strategies and knowledge for the user
    let strategies = match get_strategies_by_user_id(pool, user.id).await {
        Ok(strategies) => strategies,
        Err(e) => return Err(format!("Failed to retrieve strategies: {}", e)),
    };

    let knowledge = match get_knowledge_by_user_id(pool, user.id).await {
        Ok(knowledge) => knowledge,
        Err(e) => return Err(format!("Failed to retrieve knowledge: {}", e)),
    };

    Ok(Some(AgentProfile {
        user,
        strategies,
        knowledge,
    }))
}
