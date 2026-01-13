use agentropic_core::AgentId;
use serde::{Deserialize, Serialize};

/// Type of knowledge source
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KnowledgeSourceType {
    /// Sensor data
    Sensor,
    /// Reasoning module
    Reasoning,
    /// Planning module
    Planning,
    /// Learning module
    Learning,
}

/// Knowledge source that contributes to blackboard
#[derive(Debug, Clone)]
pub struct KnowledgeSource {
    id: AgentId,
    source_type: KnowledgeSourceType,
    priority: u32,
}

impl KnowledgeSource {
    /// Create a new knowledge source
    pub fn new(id: AgentId, source_type: KnowledgeSourceType) -> Self {
        Self {
            id,
            source_type,
            priority: 0,
        }
    }

    /// Set priority
    pub fn with_priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }

    /// Get ID
    pub fn id(&self) -> &AgentId {
        &self.id
    }

    /// Get source type
    pub fn source_type(&self) -> KnowledgeSourceType {
        self.source_type
    }

    /// Get priority
    pub fn priority(&self) -> u32 {
        self.priority
    }
}
