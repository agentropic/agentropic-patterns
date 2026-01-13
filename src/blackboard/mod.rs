//! Blackboard architecture patterns

/// Blackboard structure
pub mod blackboard;
/// Knowledge source
pub mod knowledge_source;

pub use blackboard::Blackboard;
pub use knowledge_source::{KnowledgeSource, KnowledgeSourceType};
