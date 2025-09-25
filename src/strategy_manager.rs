use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};
use anyhow::Result;
use crate::personality::Personality;

/// Structure to represent a trading or yield strategy
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Strategy {
    pub id: String,
    pub name: String,
    pub category: String,
    pub description: String,
    pub risk_level: RiskLevel,
    pub tags: Vec<String>,
    pub steps: Vec<String>,
    pub requirements: Vec<String>,
    pub expected_returns: Option<ExpectedReturns>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub author: Option<String>,
    pub version: String,
}

/// Risk level for a strategy
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Experimental,
}

/// Expected returns for a strategy
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExpectedReturns {
    pub min: f64,
    pub max: f64,
    pub timeframe: String, // e.g., "daily", "weekly", "monthly", "yearly"
}

/// Strategy Manager to handle dynamic strategy loading and management
pub struct StrategyManager {
    storage_dir: PathBuf,
    strategies: HashMap<String, Strategy>,
}

impl StrategyManager {
    /// Create a new StrategyManager with the specified storage directory
    pub fn new(storage_dir: &Path) -> Result<Self> {
        // Create the storage directory if it doesn't exist
        if !storage_dir.exists() {
            fs::create_dir_all(storage_dir)?;
        }
        
        // Initialize the strategies map
        let mut strategies = HashMap::new();
        
        // Load existing strategies from the storage directory
        for entry in fs::read_dir(storage_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                let mut file = File::open(&path)?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                
                let strategy: Strategy = serde_json::from_str(&contents)?;
                strategies.insert(strategy.id.clone(), strategy);
            }
        }
        
        Ok(Self {
            storage_dir: storage_dir.to_path_buf(),
            strategies,
        })
    }
    
    /// Add a new strategy
    pub fn add_strategy(&mut self, strategy: Strategy) -> Result<()> {
        // Save the strategy to disk
        self.save_strategy(&strategy)?;
        
        // Add to in-memory map
        self.strategies.insert(strategy.id.clone(), strategy);
        
        Ok(())
    }
    
    /// Update an existing strategy
    pub fn update_strategy(&mut self, strategy: Strategy) -> Result<()> {
        if self.strategies.contains_key(&strategy.id) {
            // Save the updated strategy to disk
            self.save_strategy(&strategy)?;
            
            // Update in-memory map
            self.strategies.insert(strategy.id.clone(), strategy);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Strategy not found: {}", strategy.id))
        }
    }
    
    /// Get a strategy by ID
    pub fn get_strategy(&self, id: &str) -> Option<&Strategy> {
        self.strategies.get(id)
    }
    
    /// Get all strategies
    pub fn get_all_strategies(&self) -> Vec<&Strategy> {
        self.strategies.values().collect()
    }
    
    /// Get strategies by category
    pub fn get_strategies_by_category(&self, category: &str) -> Vec<&Strategy> {
        self.strategies.values()
            .filter(|s| s.category == category)
            .collect()
    }
    
    /// Get strategies by risk level
    pub fn get_strategies_by_risk(&self, risk_level: RiskLevel) -> Vec<&Strategy> {
        self.strategies.values()
            .filter(|s| s.risk_level == risk_level)
            .collect()
    }
    
    /// Get strategies by tags
    pub fn get_strategies_by_tags(&self, tags: &[String]) -> Vec<&Strategy> {
        self.strategies.values()
            .filter(|s| tags.iter().any(|tag| s.tags.contains(tag)))
            .collect()
    }
    
    /// Delete a strategy
    pub fn delete_strategy(&mut self, id: &str) -> Result<()> {
        if self.strategies.remove(id).is_some() {
            let file_path = self.get_file_path(id);
            if file_path.exists() {
                fs::remove_file(file_path)?;
            }
            Ok(())
        } else {
            Err(anyhow::anyhow!("Strategy not found: {}", id))
        }
    }
    
    /// Save a strategy to disk
    fn save_strategy(&self, strategy: &Strategy) -> Result<()> {
        let file_path = self.get_file_path(&strategy.id);
        let json = serde_json::to_string_pretty(strategy)?;
        
        let mut file = File::create(file_path)?;
        file.write_all(json.as_bytes())?;
        
        Ok(())
    }
    
    /// Get the file path for a strategy
    fn get_file_path(&self, id: &str) -> PathBuf {
        self.storage_dir.join(format!("{}.json", id))
    }
    
    /// Import strategies to personality
    pub fn import_strategies_to_personality(&self, personality: &mut Personality) -> Result<usize> {
        let mut count = 0;
        
        for strategy in self.strategies.values() {
            // Add the strategy to the personality
            personality.add_strategy(&strategy.category, strategy.name.clone());
            count += 1;
        }
        
        Ok(count)
    }
    
    /// Create a strategy from template
    pub fn create_strategy_from_template(
        id: &str,
        name: &str,
        category: &str,
        description: &str,
        risk_level: RiskLevel,
        tags: Vec<String>,
        steps: Vec<String>,
        requirements: Vec<String>,
        expected_returns: Option<ExpectedReturns>,
        author: Option<String>,
    ) -> Strategy {
        let now = Utc::now();
        
        Strategy {
            id: id.to_string(),
            name: name.to_string(),
            category: category.to_string(),
            description: description.to_string(),
            risk_level,
            tags,
            steps,
            requirements,
            expected_returns,
            created_at: now,
            updated_at: now,
            author,
            version: "1.0.0".to_string(),
        }
    }
}

/// Interactive prompt to add a new strategy
pub async fn interactive_add_strategy(strategy_manager: &mut StrategyManager) -> Result<()> {
    println!("Adding new strategy");
    
    // Get strategy ID
    print!("Enter strategy ID: ");
    io::stdout().flush()?;
    let mut id = String::new();
    io::stdin().read_line(&mut id)?;
    let id = id.trim();
    
    // Get strategy name
    print!("Enter strategy name: ");
    io::stdout().flush()?;
    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    let name = name.trim();
    
    // Get category
    print!("Enter category (e.g., yield, trading, risk_management): ");
    io::stdout().flush()?;
    let mut category = String::new();
    io::stdin().read_line(&mut category)?;
    let category = category.trim();
    
    // Get description
    print!("Enter description: ");
    io::stdout().flush()?;
    let mut description = String::new();
    io::stdin().read_line(&mut description)?;
    let description = description.trim();
    
    // Get risk level
    print!("Enter risk level (low, medium, high, experimental): ");
    io::stdout().flush()?;
    let mut risk_level_str = String::new();
    io::stdin().read_line(&mut risk_level_str)?;
    let risk_level = match risk_level_str.trim().to_lowercase().as_str() {
        "low" => RiskLevel::Low,
        "medium" => RiskLevel::Medium,
        "high" => RiskLevel::High,
        "experimental" => RiskLevel::Experimental,
        _ => RiskLevel::Medium,
    };
    
    // Get tags
    print!("Enter tags (comma separated): ");
    io::stdout().flush()?;
    let mut tags_input = String::new();
    io::stdin().read_line(&mut tags_input)?;
    let tags: Vec<String> = tags_input
        .trim()
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    
    // Get steps
    println!("Enter steps (one per line, end with a line containing only 'END'):");
    let mut steps = Vec::new();
    loop {
        let mut step = String::new();
        io::stdin().read_line(&mut step)?;
        if step.trim() == "END" {
            break;
        }
        steps.push(step.trim().to_string());
    }
    
    // Get requirements
    println!("Enter requirements (one per line, end with a line containing only 'END'):");
    let mut requirements = Vec::new();
    loop {
        let mut req = String::new();
        io::stdin().read_line(&mut req)?;
        if req.trim() == "END" {
            break;
        }
        requirements.push(req.trim().to_string());
    }
    
    // Get expected returns
    print!("Include expected returns? (y/n): ");
    io::stdout().flush()?;
    let mut include_returns = String::new();
    io::stdin().read_line(&mut include_returns)?;
    
    let expected_returns = if include_returns.trim().to_lowercase() == "y" {
        print!("Enter minimum return percentage: ");
        io::stdout().flush()?;
        let mut min = String::new();
        io::stdin().read_line(&mut min)?;
        let min: f64 = min.trim().parse().unwrap_or(0.0);
        
        print!("Enter maximum return percentage: ");
        io::stdout().flush()?;
        let mut max = String::new();
        io::stdin().read_line(&mut max)?;
        let max: f64 = max.trim().parse().unwrap_or(0.0);
        
        print!("Enter timeframe (daily, weekly, monthly, yearly): ");
        io::stdout().flush()?;
        let mut timeframe = String::new();
        io::stdin().read_line(&mut timeframe)?;
        let timeframe = timeframe.trim().to_string();
        
        Some(ExpectedReturns {
            min,
            max,
            timeframe,
        })
    } else {
        None
    };
    
    // Get author
    print!("Enter author (optional): ");
    io::stdout().flush()?;
    let mut author = String::new();
    io::stdin().read_line(&mut author)?;
    let author = if author.trim().is_empty() {
        None
    } else {
        Some(author.trim().to_string())
    };
    
    // Create the strategy
    let strategy = StrategyManager::create_strategy_from_template(
        id,
        name,
        category,
        description,
        risk_level,
        tags,
        steps,
        requirements,
        expected_returns,
        author,
    );
    
    // Add the strategy
    strategy_manager.add_strategy(strategy)?;
    println!("Strategy added successfully!");
    
    Ok(())
}
