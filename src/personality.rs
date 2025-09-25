use serde::{Deserialize, Serialize};
use std::fs;
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Personality {
    pub name: String,
    pub role: String,
    pub style: Style,
    pub rules: Vec<String>,
    #[serde(default)]
    pub integrations: Integrations,
    #[serde(default)]
    pub knowledge_sources: Vec<KnowledgeSource>,
    #[serde(default)]
    pub strategies: HashMap<String, Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Style {
    pub tone: String,
    pub formality: String,
    pub domain_focus: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Integrations {
    #[serde(default)]
    pub apis: Vec<ApiIntegration>,
    #[serde(default)]
    pub protocols: Vec<Protocol>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ApiIntegration {
    pub name: String,
    pub description: String,
    pub endpoints: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Protocol {
    pub name: String,
    pub chain: String,
    pub description: String,
    #[serde(default)]
    pub focus: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct KnowledgeSource {
    #[serde(rename = "type")]
    pub source_type: String,
    pub name: String,
    pub description: String,
}

impl Personality {
    /// Get all focused protocols
    pub fn get_focused_protocols(&self) -> Vec<&Protocol> {
        self.integrations.protocols.iter()
            .filter(|p| p.focus)
            .collect()
    }
    
    /// Get all available APIs
    pub fn get_apis(&self) -> Vec<&ApiIntegration> {
        self.integrations.apis.iter().collect()
    }
    
    /// Get strategies by category
    pub fn get_strategies(&self, category: &str) -> Option<&Vec<String>> {
        self.strategies.get(category)
    }
    
    /// Get knowledge sources by type
    pub fn get_knowledge_sources_by_type(&self, source_type: &str) -> Vec<&KnowledgeSource> {
        self.knowledge_sources.iter()
            .filter(|ks| ks.source_type == source_type)
            .collect()
    }
    
    /// Add a new knowledge source
    pub fn add_knowledge_source(&mut self, source: KnowledgeSource) {
        self.knowledge_sources.push(source);
    }
    
    /// Add a new protocol
    pub fn add_protocol(&mut self, protocol: Protocol) {
        self.integrations.protocols.push(protocol);
    }
    
    /// Add a new API integration
    pub fn add_api(&mut self, api: ApiIntegration) {
        self.integrations.apis.push(api);
    }
    
    /// Add a new strategy to a category
    pub fn add_strategy(&mut self, category: &str, strategy: String) {
        self.strategies.entry(category.to_string())
            .or_insert_with(Vec::new)
            .push(strategy);
    }
}

pub fn load_personality(path: &str) -> anyhow::Result<Personality> {
    let data = fs::read_to_string(path)?;
    let persona: Personality = serde_json::from_str(&data)?;
    Ok(persona)
}

pub fn save_personality(path: &str, personality: &Personality) -> anyhow::Result<()> {
    let data = serde_json::to_string_pretty(personality)?;
    fs::write(path, data)?;
    Ok(())
}
