use crate::personality::{KnowledgeSource, Personality};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use anyhow::Result;

/// Structure to represent a knowledge entry with content and metadata
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KnowledgeEntry {
    pub source_id: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: Vec<String>,
}

/// Knowledge Manager to handle dynamic prompts and data sources
pub struct KnowledgeManager {
    storage_dir: PathBuf,
    pub entries: HashMap<String, KnowledgeEntry>,
}

impl KnowledgeManager {
    /// Create a new KnowledgeManager with the specified storage directory
    pub fn new(storage_dir: &Path) -> Result<Self> {
        // Create the storage directory if it doesn't exist
        if !storage_dir.exists() {
            fs::create_dir_all(storage_dir)?;
        }
        
        // Initialize the entries map
        let mut entries = HashMap::new();
        
        // Load existing entries from the storage directory
        for entry in fs::read_dir(storage_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                let mut file = File::open(&path)?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                
                let knowledge_entry: KnowledgeEntry = serde_json::from_str(&contents)?;
                entries.insert(knowledge_entry.source_id.clone(), knowledge_entry);
            }
        }
        
        Ok(Self {
            storage_dir: storage_dir.to_path_buf(),
            entries,
        })
    }
    
    /// Add a new knowledge entry
    pub fn add_entry(&mut self, source_id: &str, content: &str, tags: Vec<String>) -> Result<()> {
        let now = Utc::now();
        
        let entry = KnowledgeEntry {
            source_id: source_id.to_string(),
            content: content.to_string(),
            created_at: now,
            updated_at: now,
            tags,
        };
        
        // Save the entry to disk
        self.save_entry(&entry)?;
        
        // Add to in-memory map
        self.entries.insert(source_id.to_string(), entry);
        
        Ok(())
    }
    
    /// Update an existing knowledge entry
    pub fn update_entry(&mut self, source_id: &str, content: &str) -> Result<()> {
        if let Some(mut entry) = self.entries.get(source_id).cloned() {
            entry.content = content.to_string();
            entry.updated_at = Utc::now();
            
            // Save the updated entry to disk
            self.save_entry(&entry)?;
            
            // Update in-memory map
            self.entries.insert(source_id.to_string(), entry);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Entry not found: {}", source_id))
        }
    }
    
    /// Get a knowledge entry by ID
    pub fn get_entry(&self, source_id: &str) -> Option<&KnowledgeEntry> {
        self.entries.get(source_id)
    }
    
    /// Get all knowledge entries with specific tags
    pub fn get_entries_by_tags(&self, tags: &[String]) -> Vec<&KnowledgeEntry> {
        self.entries.values()
            .filter(|entry| tags.iter().any(|tag| entry.tags.contains(tag)))
            .collect()
    }
    
    /// Delete a knowledge entry
    pub fn delete_entry(&mut self, source_id: &str) -> Result<()> {
        if self.entries.remove(source_id).is_some() {
            let file_path = self.get_file_path(source_id);
            if file_path.exists() {
                fs::remove_file(file_path)?;
            }
            Ok(())
        } else {
            Err(anyhow::anyhow!("Entry not found: {}", source_id))
        }
    }
    
    /// Save an entry to disk
    fn save_entry(&self, entry: &KnowledgeEntry) -> Result<()> {
        let file_path = self.get_file_path(&entry.source_id);
        let json = serde_json::to_string_pretty(entry)?;
        
        let mut file = File::create(file_path)?;
        file.write_all(json.as_bytes())?;
        
        Ok(())
    }
    
    /// Get the file path for a knowledge entry
    fn get_file_path(&self, source_id: &str) -> PathBuf {
        self.storage_dir.join(format!("{}.json", source_id))
    }
    
    /// Add knowledge sources from a personality to the knowledge manager
    pub fn add_sources_from_personality(&mut self, personality: &Personality) -> Result<()> {
        for source in &personality.knowledge_sources {
            // Only add database and prompt type sources
            if source.source_type == "database" || source.source_type == "prompt" {
                // Check if we already have this source
                if !self.entries.contains_key(&source.name) {
                    // Create an empty entry for this source
                    self.add_entry(
                        &source.name,
                        &format!("# {}\n\n{}", source.name, source.description),
                        vec![source.source_type.clone()]
                    )?;
                }
            }
        }
        
        Ok(())
    }
    
    /// Get all entries as a formatted string for context injection
    pub fn get_all_entries_as_context(&self) -> String {
        let mut context = String::new();
        
        for entry in self.entries.values() {
            context.push_str(&format!("--- BEGIN KNOWLEDGE: {} ---\n", entry.source_id));
            context.push_str(&entry.content);
            context.push_str(&format!("\n--- END KNOWLEDGE: {} ---\n\n", entry.source_id));
        }
        
        context
    }
    
    /// Get entries by source type as a formatted string for context injection
    pub fn get_entries_by_type_as_context(&self, source_type: &str) -> String {
        let mut context = String::new();
        
        for entry in self.entries.values() {
            if entry.tags.contains(&source_type.to_string()) {
                context.push_str(&format!("--- BEGIN {}: {} ---\n", source_type.to_uppercase(), entry.source_id));
                context.push_str(&entry.content);
                context.push_str(&format!("\n--- END {}: {} ---\n\n", source_type.to_uppercase(), entry.source_id));
            }
        }
        
        context
    }
}

/// Interactive prompt to add a new knowledge entry
pub async fn interactive_add_knowledge(knowledge_manager: &mut KnowledgeManager) -> Result<()> {
    println!("Adding new knowledge entry");
    
    // Get source ID
    print!("Enter source ID: ");
    io::stdout().flush()?;
    let mut source_id = String::new();
    io::stdin().read_line(&mut source_id)?;
    let source_id = source_id.trim();
    
    // Get content
    print!("Enter content (end with a line containing only 'END'): ");
    io::stdout().flush()?;
    let mut content = String::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        if line.trim() == "END" {
            break;
        }
        content.push_str(&line);
    }
    
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
    
    // Add the entry
    knowledge_manager.add_entry(source_id, &content, tags)?;
    println!("Knowledge entry added successfully!");
    
    Ok(())
}
