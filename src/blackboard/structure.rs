use super::KnowledgeSource;
use std::collections::HashMap;

/// Blackboard for shared knowledge coordination
#[derive(Debug, Clone)]
pub struct Blackboard {
    name: String,
    knowledge: HashMap<String, String>,
    sources: Vec<KnowledgeSource>,
}

impl Blackboard {
    /// Create a new blackboard
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            knowledge: HashMap::new(),
            sources: Vec::new(),
        }
    }

    /// Write knowledge to blackboard
    pub fn write(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.knowledge.insert(key.into(), value.into());
    }

    /// Read knowledge from blackboard
    pub fn read(&self, key: &str) -> Option<&String> {
        self.knowledge.get(key)
    }

    /// Remove knowledge
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.knowledge.remove(key)
    }

    /// Add knowledge source
    pub fn add_source(&mut self, source: KnowledgeSource) {
        self.sources.push(source);
    }

    /// Get name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get all knowledge
    pub fn knowledge(&self) -> &HashMap<String, String> {
        &self.knowledge
    }

    /// Get all sources
    pub fn sources(&self) -> &[KnowledgeSource] {
        &self.sources
    }

    /// Clear all knowledge
    pub fn clear(&mut self) {
        self.knowledge.clear();
    }

    /// Get knowledge count
    pub fn size(&self) -> usize {
        self.knowledge.len()
    }
}
