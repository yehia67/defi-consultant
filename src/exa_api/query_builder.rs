/// QueryBuilder for constructing Exa API search queries
#[derive(Debug, Clone)]
pub struct QueryBuilder {
    base_terms: Vec<String>,
    project_name: String,
    aspects: Vec<String>,
}

impl QueryBuilder {
    /// Create a new QueryBuilder for a specific crypto project
    pub fn new(project_name: &str) -> Self {
        Self {
            base_terms: vec!["cryptocurrency".to_string()],
            project_name: project_name.to_string(),
            aspects: Vec::new(),
        }
    }
    
    /// Add an aspect to the query (e.g., "technology", "tokenomics")
    pub fn add_aspect(mut self, aspect: &str) -> Self {
        self.aspects.push(aspect.to_string());
        self
    }
    
    /// Add multiple aspects to the query
    pub fn add_aspects(mut self, aspects: &[&str]) -> Self {
        for aspect in aspects {
            self.aspects.push(aspect.to_string());
        }
        self
    }
    
    /// Build the final query string
    pub fn build(&self) -> String {
        let mut terms = self.base_terms.clone();
        terms.push(self.project_name.clone());
        terms.extend(self.aspects.clone());
        terms.join(" ")
    }
}
